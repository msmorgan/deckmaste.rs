mod ability;
mod clause;
mod phrase;

pub use ability::*;
pub use clause::*;
pub(crate) use phrase::nominal_constructions;
pub use phrase::*;

pub use crate::constructions::coordination::CoordinatedNominalPhrase;
pub use crate::constructions::coordination::CoordinatedNounPhrase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum RecoveryRole {
    Clause,
    NominalComplement,
    ActivationCost,
    KeywordArgument,
    ModalHeader,
    EmbeddedRules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct RecoveryRef<'a> {
    pub role: RecoveryRole,
    pub text: &'a str,
    pub source_tokens: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum LexicalOpacityKind {
    Noun,
    FlavorHeader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LexicalOpacityRef<'a> {
    pub kind: LexicalOpacityKind,
    pub text: &'a str,
    pub source_tokens: usize,
}

impl OracleText {
    #[must_use]
    pub fn recoveries(&self) -> Vec<RecoveryRef<'_>> {
        let mut walker = RecoveryWalker::default();
        walker.oracle_text(self);
        walker.phrases
    }

    #[must_use]
    pub fn lexical_opacity(&self) -> Vec<LexicalOpacityRef<'_>> {
        let mut walker = RecoveryWalker::default();
        walker.oracle_text(self);
        walker.lexical_opacity
    }

    /// Returns every noun phrase in this syntax tree in depth-first order.
    ///
    /// This semantic traversal is independent of serialization layout. It is
    /// intended for consumers that need to inspect noun-phrase relationships
    /// without maintaining a second exhaustive AST walker.
    #[must_use]
    pub fn noun_phrases(&self) -> Vec<&NounPhrase> {
        let mut walker = RecoveryWalker::default();
        walker.oracle_text(self);
        walker.noun_phrases
    }
}

impl crate::fragment::Fragment {
    /// Every recovered span inside this fragment, the same census
    /// [`OracleText::recoveries`] reports for a whole card.
    ///
    /// A fragment's recoveries are what make
    /// [`crate::FragmentReport::clean`] more than "the parser returned
    /// something": the ability layer is total, so an unparsed run becomes a
    /// `RecoveredText`-bearing node rather than an error, and a quoted
    /// ability's interior is parsed by a nested `Parser` whose diagnostics
    /// never reach the outer report at all.
    ///
    /// Crate-internal: the public reader is
    /// [`crate::FragmentReport::recoveries`], which is where a caller holding
    /// a parse result looks.
    pub(crate) fn recoveries(&self) -> Vec<RecoveryRef<'_>> {
        use crate::fragment::Fragment;

        let mut walker = RecoveryWalker::default();
        match self {
            Fragment::Nominal(noun_phrase) => walker.noun_phrase(noun_phrase, None),
            Fragment::Sentence(sentence) => walker.sentence(sentence, None),
            Fragment::Cost(cost) => walker.cost(cost, Some(RecoveryRole::ActivationCost)),
            Fragment::KeywordLine(list) => walker.keyword_ability_list(list, None),
            Fragment::Ability(ability) => walker.ability(ability, None),
        }
        walker.phrases
    }
}

#[derive(Default, serde::Serialize)]
struct RecoveryWalker<'syntax> {
    phrases: Vec<RecoveryRef<'syntax>>,
    lexical_opacity: Vec<LexicalOpacityRef<'syntax>>,
    noun_phrases: Vec<&'syntax NounPhrase>,
}

impl<'syntax> RecoveryWalker<'syntax> {
    fn oracle_text(&mut self, oracle_text: &'syntax OracleText) {
        for ability in &oracle_text.abilities {
            self.ability(ability, None);
        }
    }

    fn ability(&mut self, ability: &'syntax Ability, context: Option<RecoveryRole>) {
        if let Some(AbilityHeader::Flavor(header)) = ability.header() {
            self.lexical_opacity.push(LexicalOpacityRef {
                kind: LexicalOpacityKind::FlavorHeader,
                text: header.text(),
                source_tokens: header.source_tokens(),
            });
        }
        match ability.kind() {
            AbilityKind::Activated(activated) => {
                self.cost(&activated.cost, Some(RecoveryRole::ActivationCost));
                self.paragraph(&activated.effect, context);
            }
            AbilityKind::ClassLevel(level) => {
                self.cost(&level.cost, Some(RecoveryRole::ActivationCost));
            }
            AbilityKind::Chapter(chapter) => self.paragraph(&chapter.body, context),
            AbilityKind::RollRow(row) => self.paragraph(&row.body, context),
            AbilityKind::LevelBand(band) => {
                for ability in &band.abilities {
                    self.ability(ability, context);
                }
            }
            AbilityKind::StationThreshold(threshold) => {
                self.ability(&threshold.ability, context);
            }
            AbilityKind::Triggered(triggered) => {
                self.trigger_event(&triggered.conditions.first.event, context);
                for coordination in &triggered.conditions.rest {
                    self.trigger_event(&coordination.condition.event, context);
                }
                if let Some(condition) = &triggered.intervening_condition {
                    self.dependent_clause(condition, context);
                }
                self.paragraph(&triggered.effect, context);
            }
            AbilityKind::Loyalty(loyalty) => self.paragraph(&loyalty.effect, context),
            AbilityKind::Modal(modal) => {
                match &modal.frame {
                    ModalFrame::Unframed
                    | ModalFrame::Loyalty(_)
                    | ModalFrame::Chapter(_)
                    | ModalFrame::Keyword(_) => {}
                    ModalFrame::Activated(cost) => {
                        self.cost(cost, Some(RecoveryRole::ActivationCost));
                    }
                    ModalFrame::Triggered(trigger) => {
                        self.trigger_event(&trigger.event, context);
                        if let Some(condition) = &trigger.intervening_condition {
                            self.dependent_clause(condition, context);
                        }
                    }
                }
                self.paragraph(&modal.header, Some(RecoveryRole::ModalHeader));
                for mode in &modal.modes {
                    if let Some(heading) = &mode.heading {
                        self.lexical_opacity.push(LexicalOpacityRef {
                            kind: LexicalOpacityKind::FlavorHeader,
                            text: heading.label.text(),
                            source_tokens: heading.label.source_tokens(),
                        });
                        self.cost(&heading.cost, Some(RecoveryRole::ActivationCost));
                    }
                    self.paragraph(&mode.body, context);
                }
            }
            AbilityKind::Keyword(list) => self.keyword_ability_list(list, context),
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph, context),
        }
    }

    fn keyword_ability_list(
        &mut self,
        list: &'syntax KeywordAbilityList,
        context: Option<RecoveryRole>,
    ) {
        for keyword in list.abilities() {
            self.keyword_argument(&keyword.argument, context);
        }
        if let Some(trailing) = list.trailing() {
            self.paragraph(trailing, context);
        }
    }

    fn keyword_argument(
        &mut self,
        argument: &'syntax KeywordArgument,
        context: Option<RecoveryRole>,
    ) {
        // A keyword argument's recovery is normally its own role, but inside a
        // quoted or embedded ability the inherited context wins: an interior
        // failure stays attributed to that text, never leaking into the outer
        // census.
        let inner = context.or(Some(RecoveryRole::KeywordArgument));
        match argument {
            KeywordArgument::Costed(KeywordCost::Sentence { ability, .. }) => {
                self.ability(ability, inner);
            }
            KeywordArgument::Costed(KeywordCost::Components { cost, .. }) => {
                self.cost(cost, inner);
            }
            KeywordArgument::RestrictedCost {
                restriction, cost, ..
            } => {
                self.noun_phrase(restriction, inner);
                match cost {
                    KeywordCost::Symbols(_) => {}
                    KeywordCost::Sentence { ability, .. } => self.ability(ability, inner),
                    KeywordCost::Components { cost, .. } => self.cost(cost, inner),
                }
            }
            KeywordArgument::Qualified(phrase) => {
                self.phrase(phrase, RecoveryRole::KeywordArgument, inner);
            }
            KeywordArgument::Predicated(predicated) => {
                for quality in &predicated.qualities {
                    self.phrase(&quality.quality, RecoveryRole::KeywordArgument, inner);
                }
            }
            KeywordArgument::Recovered { text, .. } => {
                self.push(text, RecoveryRole::KeywordArgument, inner);
            }
            KeywordArgument::Absent
            | KeywordArgument::Counted(_)
            | KeywordArgument::Costed(KeywordCost::Symbols(_))
            | KeywordArgument::CountedCost { .. }
            | KeywordArgument::Statted { .. }
            | KeywordArgument::Named { .. } => {}
        }
    }

    fn cost(&mut self, cost: &'syntax Cost, context: Option<RecoveryRole>) {
        if let Some(header) = cost.flavor_header() {
            self.lexical_opacity.push(LexicalOpacityRef {
                kind: LexicalOpacityKind::FlavorHeader,
                text: header.text(),
                source_tokens: header.source_tokens(),
            });
        }
        for component in cost.components() {
            self.cost_component(component, context);
        }
    }

    fn cost_component(&mut self, component: &'syntax CostComponent, context: Option<RecoveryRole>) {
        match component {
            CostComponent::Recovered(text) => {
                self.push(text, RecoveryRole::ActivationCost, context);
            }
            CostComponent::Clause(clause) => self.independent_clause(clause, context),
            CostComponent::Noun(noun) => self.noun_phrase(noun, context),
            CostComponent::Alternative(left, right) => {
                self.cost_component(left, context);
                self.cost_component(right, context);
            }
            CostComponent::Symbols(_) => {}
        }
    }

    fn paragraph(&mut self, paragraph: &'syntax Paragraph, context: Option<RecoveryRole>) {
        if let Some(header) = &paragraph.flavor_header {
            self.lexical_opacity.push(LexicalOpacityRef {
                kind: LexicalOpacityKind::FlavorHeader,
                text: header.text(),
                source_tokens: header.source_tokens(),
            });
        }
        for sentence in &paragraph.sentences {
            self.sentence(sentence, context);
        }
    }

    fn sentence(&mut self, sentence: &'syntax Sentence, context: Option<RecoveryRole>) {
        match sentence.body() {
            SentenceBody::Independent(clause) => self.independent_clause(clause, context),
            SentenceBody::Choice(choice) => self.choice_instruction(choice, context),
            SentenceBody::PowerToughness(_) => {}
            SentenceBody::Triggered(triggered) => {
                self.trigger_event(&triggered.trigger.event, context);
                if let Some(condition) = &triggered.trigger.intervening_condition {
                    self.dependent_clause(condition, context);
                }
                self.independent_clause(&triggered.effect, context);
            }
            SentenceBody::Recovered(unknown) => {
                self.push(unknown, RecoveryRole::Clause, context);
            }
        }
    }

    fn choice_instruction(
        &mut self,
        choice: &'syntax ChoiceInstruction,
        context: Option<RecoveryRole>,
    ) {
        if let Some(trigger) = &choice.trigger_prefix {
            self.trigger_event(&trigger.event, context);
            if let Some(condition) = &trigger.intervening_condition {
                self.dependent_clause(condition, context);
            }
        }
        self.predicate(&choice.imperative, context);
    }

    fn clause(&mut self, clause: &'syntax Clause, context: Option<RecoveryRole>) {
        match clause {
            Clause::Independent(clause) => self.independent_clause(clause, context),
            Clause::Dependent(clause) => self.dependent_clause(clause, context),
        }
    }

    fn trigger_event(&mut self, event: &'syntax TriggerEvent, context: Option<RecoveryRole>) {
        match event {
            TriggerEvent::Clause(clause) => self.independent_clause(clause, context),
            TriggerEvent::Temporal(noun_phrase) => self.noun_phrase(noun_phrase, context),
        }
    }

    fn independent_clause(
        &mut self,
        clause: &'syntax IndependentClause,
        context: Option<RecoveryRole>,
    ) {
        match clause {
            IndependentClause::Finite(finite) => {
                if let Some(subject) = finite.subject() {
                    self.subject(subject, context);
                }
                self.predicate_expression(finite.predicate(), context);
            }
            IndependentClause::Existential(existential) => {
                self.noun_phrase(existential.pivot(), context);
            }
            IndependentClause::Complex(complex) => {
                self.independent_clause(complex.host(), context);
                self.clause_attachment(complex.attachment(), context);
            }
            IndependentClause::Coordinated(coordinated) => {
                self.independent_clause(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    match &coordination.member {
                        CoordinatedClauseMember::Independent(clause) => {
                            self.independent_clause(clause, context);
                        }
                    }
                }
            }
        }
    }

    fn clause_attachment(
        &mut self,
        attachment: &'syntax ClauseAttachment,
        context: Option<RecoveryRole>,
    ) {
        match attachment.payload() {
            ClauseAttachmentKind::Dependent(clause) => self.dependent_clause(clause, context),
            ClauseAttachmentKind::Adjunct(adjunct) => self.predicate_adjunct(adjunct, context),
            ClauseAttachmentKind::Exception(rider) => {
                self.independent_clause(rider.first(), context);
                for conjunct in rider.rest() {
                    self.independent_clause(conjunct.clause(), context);
                }
            }
            ClauseAttachmentKind::Restriction(run) => {
                for adjunct in run.first().adjuncts() {
                    self.predicate_adjunct(adjunct, context);
                }
                for member in run.rest() {
                    for adjunct in member.member().adjuncts() {
                        self.predicate_adjunct(adjunct, context);
                    }
                }
            }
            ClauseAttachmentKind::Appositive(clause) => {
                self.independent_clause(clause, context);
            }
        }
    }

    fn dependent_clause(
        &mut self,
        clause: &'syntax DependentClause,
        context: Option<RecoveryRole>,
    ) {
        match clause {
            DependentClause::Subordinate(_, SubordinateBody::Finite(clause)) => {
                self.independent_clause(clause, context);
            }
            DependentClause::Subordinate(_, SubordinateBody::Infinitive(clause)) => {
                self.predicate(clause.predicate(), context);
            }
            DependentClause::Subordinate(_, SubordinateBody::Gerund(clause)) => {
                self.gerund_clause(clause, context);
            }
            DependentClause::Subordinate(
                _,
                SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
            ) => self.adjective_phrase(adjective, RecoveryRole::Clause, context),
            DependentClause::Relative(relative) => self.relative_clause(relative, context),
            DependentClause::Infinitive(infinitive) => {
                self.predicate(infinitive.predicate(), context);
            }
            DependentClause::Gerund(gerund) => self.gerund_clause(gerund, context),
        }
    }

    fn gerund_clause(&mut self, clause: &'syntax GerundClause, context: Option<RecoveryRole>) {
        match clause.kind() {
            GerundClauseKind::Base { predicate } => self.predicate(predicate, context),
            GerundClauseKind::RatherThan {
                matrix,
                alternative,
            } => {
                self.gerund_clause(matrix, context);
                self.gerund_clause(alternative, context);
            }
        }
    }

    fn subject(&mut self, subject: &'syntax Subject, context: Option<RecoveryRole>) {
        self.noun_phrase(&subject.0, context);
    }

    fn predicate_expression(
        &mut self,
        expression: &'syntax PredicateExpression,
        context: Option<RecoveryRole>,
    ) {
        match expression {
            PredicateExpression::Simple(predicate) => self.predicate(predicate, context),
            PredicateExpression::Coordinated(coordination) => {
                for expression in coordination.conjuncts() {
                    self.predicate_expression(expression, context);
                }
            }
        }
    }

    fn predicate(&mut self, predicate: &'syntax Predicate, context: Option<RecoveryRole>) {
        match predicate {
            Predicate::Transitive(predicate) => self.transitive_predicate(predicate, context),
            Predicate::Intransitive(predicate) => {
                Self::predicate_head(predicate.head(), context);
                self.predicate_elements(predicate.elements(), context);
            }
            Predicate::Copular(predicate) => {
                self.copular_complement(predicate.complement(), context);
                self.predicate_adjuncts(predicate.adjuncts(), context);
            }
            Predicate::Passive(predicate) => {
                Self::predicate_head(predicate.head(), context);
                if let Some(retained_object) = predicate.retained_object() {
                    self.predicate_object(retained_object, context);
                }
                self.predicate_elements(predicate.elements(), context);
            }
            Predicate::Proform(_) => {}
            Predicate::Deontic(predicate) => {
                if let Some(inner) = predicate.inner() {
                    self.predicate_expression(inner, context);
                }
            }
            Predicate::Attached(predicate) => {
                self.predicate(predicate.predicate(), context);
                self.clause_attachment(predicate.attachment(), context);
            }
        }
    }

    fn transitive_predicate(
        &mut self,
        predicate: &'syntax TransitivePredicate,
        context: Option<RecoveryRole>,
    ) {
        Self::predicate_head(predicate.head(), context);
        self.predicate_elements(predicate.pre_object_elements(), context);
        self.predicate_object(predicate.object(), context);
        self.predicate_elements(predicate.elements(), context);
    }

    fn predicate_head(head: &'syntax PredicateHead, context: Option<RecoveryRole>) {
        let _ = (head, context);
    }

    fn predicate_object(
        &mut self,
        object: &'syntax PredicateObject,
        context: Option<RecoveryRole>,
    ) {
        match object {
            PredicateObject::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
            PredicateObject::EmbeddedAbility(ability) => self.ability(ability, context),
            PredicateObject::QuotedAbility(quoted) => {
                self.ability(&quoted.ability, Some(RecoveryRole::EmbeddedRules));
            }
            PredicateObject::Ability(ability) => {
                if let Some(argument) = &ability.argument {
                    self.predicate_object(argument, context);
                }
            }
            PredicateObject::Quantity(_)
            | PredicateObject::OracleSymbol(_)
            | PredicateObject::SymbolSequence(_)
            | PredicateObject::PowerToughness(_) => {}
            PredicateObject::Coordinated(coordinated) => {
                self.predicate_object(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    self.predicate_object(&coordination.object, context);
                }
            }
        }
    }

    fn predicate_elements(
        &mut self,
        elements: &'syntax [PredicateElement],
        context: Option<RecoveryRole>,
    ) {
        for element in elements {
            match element {
                PredicateElement::Complement(complement) => match complement {
                    PredicateComplement::IndirectObject(noun_phrase) => {
                        self.noun_phrase(noun_phrase, context);
                    }
                    PredicateComplement::Adjective(adjective) => {
                        self.adjective_phrase(adjective, RecoveryRole::Clause, context);
                    }
                    PredicateComplement::CoordinatedAdjective(coordinated) => {
                        self.coordinated_adjective_phrase(
                            coordinated,
                            RecoveryRole::Clause,
                            context,
                        );
                    }
                    PredicateComplement::Prepositional(preposition) => {
                        self.prepositional_phrase(preposition, context);
                    }
                    PredicateComplement::Infinitive(infinitive) => {
                        self.predicate(infinitive.predicate(), context);
                    }
                },
                PredicateElement::Adjunct(adjunct) => self.predicate_adjunct(adjunct, context),
                // The closed coin-result tail is a fully typed literal, never
                // recovered structure or opaque content — same as a particle.
                PredicateElement::Particle(_) | PredicateElement::CoinResult(_) => {}
            }
        }
    }

    fn predicate_adjuncts(
        &mut self,
        adjuncts: &'syntax [PredicateAdjunct],
        context: Option<RecoveryRole>,
    ) {
        for adjunct in adjuncts {
            self.predicate_adjunct(adjunct, context);
        }
    }

    fn predicate_adjunct(
        &mut self,
        adjunct: &'syntax PredicateAdjunct,
        context: Option<RecoveryRole>,
    ) {
        match adjunct {
            PredicateAdjunct::Temporal(noun_phrase) | PredicateAdjunct::Manner(noun_phrase) => {
                self.noun_phrase(noun_phrase, context);
            }
            PredicateAdjunct::Prepositional(preposition)
            | PredicateAdjunct::Exception(preposition) => {
                self.prepositional_phrase(preposition, context);
            }
            PredicateAdjunct::Dependent(dependent) => {
                self.dependent_clause(dependent, context);
            }
            PredicateAdjunct::AbilityPostmodifier(postmodifier) => {
                self.ability(
                    &postmodifier.ability().ability,
                    Some(RecoveryRole::EmbeddedRules),
                );
            }
            PredicateAdjunct::Adverb(_) | PredicateAdjunct::Frequency(_) => {}
        }
    }

    fn copular_complement(
        &mut self,
        complement: &'syntax CopularComplement,
        context: Option<RecoveryRole>,
    ) {
        match complement {
            CopularComplement::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
            CopularComplement::Adjective(adjective) => {
                self.adjective_phrase(adjective, RecoveryRole::Clause, context);
            }
            CopularComplement::CoordinatedAdjective(coordinated) => {
                self.coordinated_adjective_phrase(coordinated, RecoveryRole::Clause, context);
            }
            CopularComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition, context);
            }
            CopularComplement::PowerToughness(_) | CopularComplement::CatalogAtom(_) => {}
        }
    }

    fn relative_clause(
        &mut self,
        relative: &'syntax RelativeClause,
        context: Option<RecoveryRole>,
    ) {
        match relative.body() {
            RelativeBody::SubjectGap(predicate) => self.predicate(predicate, context),
            RelativeBody::ObjectGap { subject, predicate } => {
                self.subject(subject, context);
                Self::predicate_head(predicate.head(), context);
                self.predicate_elements(predicate.elements(), context);
            }
        }
    }

    fn noun_phrase(&mut self, phrase: &'syntax NounPhrase, context: Option<RecoveryRole>) {
        self.noun_phrases.push(phrase);
        match phrase.kind() {
            NounPhraseKind::Nominal(nominal) => self.nominal_phrase(nominal, context),
            NounPhraseKind::Pronoun { .. }
            | NounPhraseKind::Demonstrative(_)
            | NounPhraseKind::Quantity(_)
            | NounPhraseKind::ThisCard(_)
            | NounPhraseKind::PossessiveThisCard(_) => {}
            NounPhraseKind::Partitive(partitive) => self.noun_phrase(&partitive.whole, context),
            NounPhraseKind::AnyNumberOf(value) => self.noun_phrase(value.complement(), context),
            NounPhraseKind::CoordinatedNominal(coordinated) => {
                if let DeterminerKind::Possessive(possessor) = coordinated.determiner().kind()
                    && let Possessor::NounPhrase(possessor) = possessor
                {
                    self.noun_phrase(possessor, context);
                }
                self.nominal_phrase(coordinated.first(), context);
                for coordination in coordinated.rest() {
                    self.nominal_phrase(&coordination.phrase, context);
                }
                for complement in coordinated.complements() {
                    self.nominal_complement(complement, context);
                }
            }
            NounPhraseKind::Coordinated(coordinated) => {
                self.noun_phrase(coordinated.first(), context);
                for coordination in coordinated.rest() {
                    self.noun_phrase(&coordination.phrase, context);
                }
            }
            NounPhraseKind::SetException(exception) => {
                self.noun_phrase(&exception.included, context);
                self.noun_phrase(&exception.excluded, context);
            }
            NounPhraseKind::Arithmetic(value) => match value {
                ArithmeticValue::Minus { left, right } => {
                    self.noun_phrase(left, context);
                    self.noun_phrase(right, context);
                }
                ArithmeticValue::Half { value, .. } => self.noun_phrase(value, context),
            },
        }
    }

    fn nominal_phrase(&mut self, nominal: &'syntax NominalPhrase, context: Option<RecoveryRole>) {
        if let Some(determiner) = nominal.determiner()
            && let DeterminerKind::Possessive(possessor) = determiner.kind()
            && let Possessor::NounPhrase(possessor) = possessor
        {
            self.noun_phrase(possessor, context);
        }
        if let crate::word::Noun::Opaque(opaque) = nominal.head().noun() {
            self.lexical_opacity.push(LexicalOpacityRef {
                kind: LexicalOpacityKind::Noun,
                text: opaque.spelling(),
                source_tokens: 1,
            });
        }
        for modifier in nominal.modifiers() {
            self.nominal_modifier(modifier, context);
        }
        for complement in nominal.complements() {
            self.nominal_complement(complement, context);
        }
    }

    fn nominal_complement(
        &mut self,
        complement: &'syntax NominalComplement,
        context: Option<RecoveryRole>,
    ) {
        match complement {
            NominalComplement::Adjective(adjective) => {
                self.adjective_phrase(adjective, RecoveryRole::NominalComplement, context);
            }
            NominalComplement::CoordinatedAdjective(coordinated) => {
                self.coordinated_adjective_phrase(
                    coordinated,
                    RecoveryRole::NominalComplement,
                    context,
                );
            }
            NominalComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition, context);
            }
            NominalComplement::Infinitive(infinitive) => {
                self.predicate(infinitive.predicate(), context);
            }
            NominalComplement::Relative(relative) => self.relative_clause(relative, context),
            NominalComplement::ReducedRecipientPassive(predicate) => {
                self.transitive_predicate(predicate, context);
            }
            NominalComplement::EventClause(clause) => {
                self.independent_clause(clause, context);
            }
            NominalComplement::KeywordArgument(argument) => {
                self.keyword_argument(argument, context);
            }
            NominalComplement::Quantity(_)
            | NominalComplement::PowerToughness(_)
            | NominalComplement::Devotion(_) => {}
        }
    }

    fn nominal_modifier(
        &mut self,
        modifier: &'syntax NominalModifier,
        context: Option<RecoveryRole>,
    ) {
        match modifier {
            NominalModifier::Adjective { phrase, .. } => {
                self.adjective_phrase(phrase, RecoveryRole::NominalComplement, context);
            }
            NominalModifier::Coordinated(coordinated) => {
                self.nominal_modifier(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    self.nominal_modifier(&coordination.modifier, context);
                }
            }
            NominalModifier::Noun { noun, .. } => {
                if let crate::word::Noun::Opaque(opaque) = noun.noun() {
                    self.lexical_opacity.push(LexicalOpacityRef {
                        kind: LexicalOpacityKind::Noun,
                        text: opaque.spelling(),
                        source_tokens: 1,
                    });
                }
            }
            NominalModifier::Quantity(_)
            | NominalModifier::PowerToughness(_)
            | NominalModifier::CombatStepName { .. } => {}
        }
    }

    fn adjective_phrase(
        &mut self,
        phrase: &'syntax AdjectivePhrase,
        role: RecoveryRole,
        context: Option<RecoveryRole>,
    ) {
        for complement in phrase.complements() {
            match complement {
                AdjectiveComplement::Comparison(comparison)
                | AdjectiveComplement::PostnominalComparison(comparison) => {
                    self.phrase(comparison.standard(), role, context);
                }
                AdjectiveComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition, context);
                }
                AdjectiveComplement::Infinitive(infinitive) => {
                    self.predicate(infinitive.predicate(), context);
                }
            }
        }
    }

    fn coordinated_adjective_phrase(
        &mut self,
        coordinated: &'syntax CoordinatedAdjectivePhrase,
        role: RecoveryRole,
        context: Option<RecoveryRole>,
    ) {
        self.adjective_phrase(&coordinated.first, role, context);
        for coordination in &coordinated.rest {
            self.adjective_phrase(&coordination.phrase, role, context);
        }
    }

    fn prepositional_phrase(
        &mut self,
        phrase: &'syntax PrepositionalPhrase,
        context: Option<RecoveryRole>,
    ) {
        // Every conjunct, not just the head: recovery inside a later member of
        // `from A, from B, and from C` must still reach the census.
        for member in phrase.members() {
            match member.object.kind() {
                PrepositionalObjectKind::NounPhrase(value) => {
                    self.noun_phrase(value, context);
                }
                PrepositionalObjectKind::PrepositionalPhrase(value) => {
                    self.prepositional_phrase(value, context);
                }
                PrepositionalObjectKind::GerundClause(value) => {
                    self.gerund_clause(value, context);
                }
                PrepositionalObjectKind::Adverb(_) => {}
            }
        }
    }

    fn phrase(
        &mut self,
        phrase: &'syntax Phrase,
        role: RecoveryRole,
        context: Option<RecoveryRole>,
    ) {
        match phrase {
            Phrase::Clause(clause) => self.clause(clause, context),
            Phrase::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
            Phrase::AdjectivePhrase(adjective) => {
                self.adjective_phrase(adjective, role, context);
            }
            Phrase::PrepositionalPhrase(preposition) => {
                self.prepositional_phrase(preposition, context);
            }
            Phrase::EmbeddedAbility(ability) => self.ability(ability, context),
            Phrase::QuotedAbility(quoted) => {
                self.ability(&quoted.ability, Some(RecoveryRole::EmbeddedRules));
            }
            Phrase::Cost(cost) => self.cost(cost, context),
            Phrase::Recovered(unknown) => self.push(unknown, role, context),
            Phrase::Quantity(_)
            | Phrase::Adverb(_)
            | Phrase::CatalogAtom(_)
            | Phrase::ColorWord(_)
            | Phrase::ThisCard(_)
            | Phrase::OracleSymbol(_)
            | Phrase::SymbolSequence(_)
            | Phrase::NumberLiteral(_)
            | Phrase::SignedScalar(_)
            | Phrase::PowerToughness(_) => {}
        }
    }

    fn push(
        &mut self,
        recovery: &'syntax RecoveredText,
        role: RecoveryRole,
        context: Option<RecoveryRole>,
    ) {
        self.phrases.push(RecoveryRef {
            role: context.unwrap_or(role),
            text: recovery.spelling(),
            source_tokens: recovery.source_tokens(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Numeral;
    use crate::catalog::CatalogSlot;
    use crate::catalog::CatalogValue;
    use crate::catalog::Catalogs;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::Pronoun;
    use crate::word::Verb;
    use crate::word::VerbInstance;
    use crate::word::VerbSlot;
    use crate::word::Vocab;

    #[test]
    fn determiners_state_their_required_noun_cardinality() {
        let one = NumberLiteral {
            value: 1,
            numeral: Numeral::Cardinal,
        };
        let three = NumberLiteral {
            value: 3,
            numeral: Numeral::Cardinal,
        };

        assert_eq!(
            crate::determiner::indefinite().noun_cardinality(),
            NounCardinality::SingularCount
        );
        assert_eq!(
            crate::determiner::demonstrative(Demonstrative::Those).noun_cardinality(),
            NounCardinality::PluralCount
        );
        assert_eq!(
            crate::determiner::demonstrative(Demonstrative::That).noun_cardinality(),
            NounCardinality::SingularOrMass
        );
        assert_eq!(
            crate::determiner::demonstrative(Demonstrative::This).noun_cardinality(),
            NounCardinality::SingularOrMass
        );
        assert_eq!(
            crate::determiner::target(Some(Quantity::unchecked_up_to(QuantityValue::Literal(one))))
                .noun_cardinality(),
            NounCardinality::SingularCount
        );
        assert_eq!(
            crate::determiner::target(Some(Quantity::unchecked_up_to(QuantityValue::Literal(
                three
            ))))
            .noun_cardinality(),
            NounCardinality::PluralCount
        );
        assert_eq!(
            crate::determiner::quantity(Quantity::unchecked_that_much()).noun_cardinality(),
            NounCardinality::Mass
        );
        assert_eq!(
            crate::determiner::possessive_pronoun(Pronoun::You)
                .unwrap()
                .noun_cardinality(),
            NounCardinality::Unconstrained
        );
        assert_eq!(
            crate::determiner::quantity(Quantity::unchecked_at_least(QuantityValue::Literal(one)))
                .noun_cardinality(),
            NounCardinality::SingularOrMass
        );
        assert_eq!(
            crate::determiner::quantity(Quantity::unchecked_at_least(QuantityValue::Literal(
                three
            )))
            .noun_cardinality(),
            NounCardinality::PluralOrMass
        );
        assert_eq!(
            crate::determiner::quantity(Quantity::unchecked_exact(three)).noun_cardinality(),
            NounCardinality::PluralOrMass
        );
        assert_eq!(
            crate::determiner::target(Some(Quantity::unchecked_up_to(QuantityValue::Variable)))
                .noun_cardinality(),
            NounCardinality::PluralCount
        );
        assert_eq!(
            Quantity::unchecked_up_to(QuantityValue::Variable).noun_cardinality(),
            NounCardinality::PluralOrMass
        );
        assert_eq!(
            Quantity::unchecked_up_to(QuantityValue::Literal(one)).noun_cardinality(),
            NounCardinality::SingularOrMass
        );
        assert_eq!(
            Quantity::unchecked_at_least(QuantityValue::Variable).noun_cardinality(),
            NounCardinality::PluralOrMass
        );
        assert_eq!(
            Quantity::unchecked_or_comparison(QuantityValue::Variable, ComparativeWord::Less)
                .noun_cardinality(),
            NounCardinality::PluralOrMass
        );
        assert_eq!(
            crate::determiner::any().noun_cardinality(),
            NounCardinality::Unconstrained
        );
        assert_eq!(
            crate::determiner::no().noun_cardinality(),
            NounCardinality::Unconstrained
        );
    }

    #[test]
    fn recovery_walker_reports_outer_roles_separately_from_lexical_opacity() {
        let catalogs = Catalogs::new(
            ["Flying"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        );
        let CatalogValue::Atom(flying) = catalogs
            .matches("flying", CatalogSlot::AbilityItem)
            .remove(0)
            .value
        else {
            panic!("expected flying catalog atom");
        };

        let oracle_text = OracleText {
            abilities: vec![
                paragraph_recovered("clause"),
                paragraph(finite(
                    Some(Subject(NounPhrase::from_nominal_declaration(
                        NominalPhrase::test_from_projection_parts(
                            None,
                            vec![NominalModifier::Noun {
                                polarity: Polarity::Positive,
                                noun: NounInstance::unchecked_singular(Noun::Opaque(
                                    OpaqueLexeme::new("flimflam"),
                                )),
                            }],
                            NounInstance::unchecked_singular(Noun::Opaque(OpaqueLexeme::new(
                                "blorple",
                            ))),
                            vec![],
                        ),
                    ))),
                    Predicate::Intransitive(intransitive(Vocab::Draw)),
                )),
                checked_ability(AbilityKind::Activated(ActivatedAbility {
                    cost: crate::cost::build_cost(
                        None,
                        vec![CostComponent::Recovered(recovered("cost"))],
                    )
                    .expect("nonempty test cost must satisfy the declaration"),
                    effect: valid_test_paragraph(),
                })),
                checked_ability(AbilityKind::Keyword(
                    crate::keyword_line::build_keyword_line(
                        SeparatedNonEmpty::from_first(KeywordAbility {
                            ability: flying,
                            argument: KeywordArgument::Recovered {
                                text: recovered("argument"),
                            },
                        }),
                        None,
                    )
                    .expect("well-formed test keyword line must satisfy the declaration"),
                )),
                checked_ability(AbilityKind::Modal(ModalAbility {
                    frame: ModalFrame::Unframed,
                    header: paragraph_body(SentenceBody::Recovered(recovered("header"))),
                    header_suffix: ModalHeaderSuffix::None,
                    modes: vec![Mode {
                        heading: None,
                        body: valid_test_paragraph(),
                    }],
                })),
                paragraph(finite(
                    None,
                    Predicate::Transitive(TransitivePredicate {
                        head: predicate_head(Vocab::Draw),
                        kind: Transitive {
                            pre_object_elements: vec![],
                            object: PredicateObject::QuotedAbility(Box::new(QuotedAbility {
                                ability: Box::new(paragraph_recovered("embedded")),
                                initial_uppercase: false,
                            })),
                        },
                        elements: vec![],
                    }),
                )),
            ],
        };

        assert_eq!(
            oracle_text.recoveries(),
            vec![
                RecoveryRef {
                    role: RecoveryRole::Clause,
                    text: "clause",
                    source_tokens: 1,
                },
                RecoveryRef {
                    role: RecoveryRole::ActivationCost,
                    text: "cost",
                    source_tokens: 1,
                },
                RecoveryRef {
                    role: RecoveryRole::KeywordArgument,
                    text: "argument",
                    source_tokens: 1,
                },
                RecoveryRef {
                    role: RecoveryRole::ModalHeader,
                    text: "header",
                    source_tokens: 1,
                },
                RecoveryRef {
                    role: RecoveryRole::EmbeddedRules,
                    text: "embedded",
                    source_tokens: 1,
                },
            ]
        );
        assert_eq!(
            oracle_text.lexical_opacity(),
            vec![
                LexicalOpacityRef {
                    kind: LexicalOpacityKind::Noun,
                    text: "blorple",
                    source_tokens: 1,
                },
                LexicalOpacityRef {
                    kind: LexicalOpacityKind::Noun,
                    text: "flimflam",
                    source_tokens: 1,
                },
            ]
        );
    }

    #[test]
    fn distributive_each_predicate_head_still_reports_a_recovered_object() {
        // The `distributive_each` flag has no recovery-bearing child (the
        // discarded `each` lexical node never enters the walker), so this
        // proves the census guard: a flagged predicate's ordinary object is
        // still traversed and its opaque descendant still reported, per the
        // qfloat round's walker regression.
        let mut head = predicate_head(Vocab::Draw);
        head.distributive_each = true;
        let ast = paragraph(finite(
            None,
            Predicate::Transitive(TransitivePredicate {
                head,
                kind: Transitive {
                    pre_object_elements: vec![],
                    object: PredicateObject::NounPhrase(NounPhrase::from_nominal_declaration(
                        NominalPhrase::test_from_projection_parts(
                            None,
                            vec![],
                            NounInstance::unchecked_singular(Noun::Opaque(OpaqueLexeme::new(
                                "blorple",
                            ))),
                            vec![],
                        ),
                    )),
                },
                elements: vec![],
            }),
        ));
        let oracle_text = OracleText {
            abilities: vec![ast],
        };
        assert_eq!(
            oracle_text.lexical_opacity(),
            vec![LexicalOpacityRef {
                kind: LexicalOpacityKind::Noun,
                text: "blorple",
                source_tokens: 1,
            }]
        );
    }

    #[test]
    fn recovery_walker_descends_into_shared_group_complements() {
        let known_nominal = |head| {
            NominalPhrase::test_from_projection_parts(
                None,
                vec![],
                NounInstance::unchecked_singular(Noun::Word(head)),
                vec![],
            )
        };
        let opaque =
            NounPhrase::from_nominal_declaration(NominalPhrase::test_from_projection_parts(
                None,
                vec![],
                NounInstance::unchecked_singular(Noun::Opaque(OpaqueLexeme::new("blorple"))),
                vec![],
            ));
        let mut relative_head = predicate_head(Vocab::Draw);
        relative_head.verb.slot = VerbSlot::Present {
            person: crate::features::Person::Third,
            number: crate::features::Number::Singular,
        };
        let relative = crate::clause::build_relative_subject(
            RelativeMarker::That,
            Predicate::Transitive(TransitivePredicate {
                head: relative_head,
                kind: Transitive {
                    pre_object_elements: vec![],
                    object: PredicateObject::NounPhrase(opaque),
                },
                elements: vec![],
            }),
        )
        .expect("the recovery fixture uses a complete finite subject relative");
        let subject = NounPhrase::from_coordinated_nominal_declaration(
            CoordinatedNominalPhrase::try_new(
                crate::determiner::any(),
                Box::new(known_nominal(Vocab::Card)),
                vec![NominalPhraseCoordination {
                    conjunction: Some(NounPhraseConjunction::Or),
                    phrase: known_nominal(Vocab::Spell),
                }],
                vec![NominalComplement::Relative(relative)],
            )
            .expect("the recovery fixture uses a declared shared-determiner shape"),
        );
        let oracle_text = OracleText {
            abilities: vec![paragraph(finite(
                Some(Subject(subject)),
                Predicate::Intransitive(intransitive(Vocab::Draw)),
            ))],
        };

        assert_eq!(
            oracle_text.lexical_opacity(),
            vec![LexicalOpacityRef {
                kind: LexicalOpacityKind::Noun,
                text: "blorple",
                source_tokens: 1,
            }]
        );
    }

    #[test]
    fn recovery_walker_reports_unsupported_predicates_at_sentence_scope() {
        let ast = crate::parse("You frobnitz a card.").into_ast();

        assert!(ast.recoveries().contains(&RecoveryRef {
            role: RecoveryRole::Clause,
            text: "You frobnitz a card.",
            source_tokens: 5,
        }));
    }

    #[test]
    fn splitting_recovery_does_not_reduce_recovered_source_tokens() {
        let one_span = activated_with_cost(vec![CostComponent::Recovered(RecoveredText::new(
            "alpha beta",
            2,
        ))]);
        let split = activated_with_cost(vec![
            CostComponent::Recovered(recovered("alpha")),
            CostComponent::Recovered(recovered("beta")),
        ]);

        let recovered_tokens = |ast: &OracleText| {
            ast.recoveries()
                .iter()
                .map(|recovery| recovery.source_tokens)
                .sum::<usize>()
        };
        assert_eq!(recovered_tokens(&one_span), 2);
        assert_eq!(recovered_tokens(&split), 2);
    }

    fn recovered(text: &str) -> RecoveredText {
        RecoveredText::new(text, 1)
    }

    fn predicate_head(vocab: Vocab) -> PredicateHead {
        PredicateHead {
            auxiliaries: vec![],
            first_auxiliary_contracted_with_subject: crate::features::Contraction::Full,
            preverb_modifiers: vec![],
            verb: VerbInstance {
                verb: Verb::Word(vocab),
                slot: VerbSlot::Imperative,
            },
            frame: Verb::Word(vocab).predicate_frames()[0],
            distributive_each: false,
        }
    }

    fn intransitive(vocab: Vocab) -> IntransitivePredicate {
        IntransitivePredicate {
            head: predicate_head(vocab),
            kind: Intransitive,
            elements: vec![],
        }
    }

    fn paragraph(clause: IndependentClause) -> Ability {
        checked_ability(AbilityKind::Paragraph(paragraph_body(
            SentenceBody::Independent(clause),
        )))
    }

    fn finite(subject: Option<Subject>, predicate: Predicate) -> IndependentClause {
        IndependentClause::Finite(FiniteClause::from_declaration_parts(
            subject,
            PredicateExpression::Simple(predicate),
        ))
    }

    fn paragraph_recovered(text: &str) -> Ability {
        checked_ability(AbilityKind::Paragraph(paragraph_body(
            SentenceBody::Recovered(recovered(text)),
        )))
    }

    fn activated_with_cost(components: Vec<CostComponent>) -> OracleText {
        OracleText {
            abilities: vec![checked_ability(AbilityKind::Activated(ActivatedAbility {
                cost: crate::cost::build_cost(None, components)
                    .expect("nonempty test cost must satisfy the declaration"),
                effect: valid_test_paragraph(),
            }))],
        }
    }

    fn paragraph_body(body: SentenceBody) -> Paragraph {
        Paragraph {
            flavor_header: None,
            sentences: vec![Sentence::from_body(body)],
        }
    }

    fn valid_test_paragraph() -> Paragraph {
        paragraph_body(SentenceBody::Independent(finite(
            None,
            Predicate::Intransitive(intransitive(Vocab::Draw)),
        )))
    }

    fn checked_ability(kind: AbilityKind) -> Ability {
        crate::ability::build_ability(None, kind).expect("syntax fixture is a valid ability")
    }
}
