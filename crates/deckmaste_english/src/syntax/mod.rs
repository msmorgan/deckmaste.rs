mod ability;
mod clause;
mod phrase;

pub use ability::*;
pub use clause::*;
pub use phrase::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryRole {
    Clause,
    NominalComplement,
    ActivationCost,
    KeywordArgument,
    ModalHeader,
    EmbeddedRules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryRef<'a> {
    pub role: RecoveryRole,
    pub text: &'a str,
    pub source_tokens: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexicalOpacityKind {
    Noun,
    FlavorHeader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

#[derive(Default)]
struct RecoveryWalker<'syntax> {
    phrases: Vec<RecoveryRef<'syntax>>,
    lexical_opacity: Vec<LexicalOpacityRef<'syntax>>,
}

impl<'syntax> RecoveryWalker<'syntax> {
    fn oracle_text(&mut self, oracle_text: &'syntax OracleText) {
        for ability in &oracle_text.abilities {
            self.ability(ability, None);
        }
    }

    fn ability(&mut self, ability: &'syntax Ability, context: Option<RecoveryRole>) {
        match &ability.kind {
            AbilityKind::Activated(activated) => {
                self.cost(&activated.cost, Some(RecoveryRole::ActivationCost));
                self.paragraph(&activated.effect, context);
            }
            AbilityKind::ClassLevel(level) => {
                self.cost(&level.cost, Some(RecoveryRole::ActivationCost));
            }
            AbilityKind::Chapter(chapter) => self.paragraph(&chapter.body, context),
            AbilityKind::RollRow(row) => self.paragraph(&row.body, context),
            AbilityKind::Triggered(triggered) => {
                self.trigger_event(&triggered.event, context);
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
                    ModalFrame::Preamble { body, .. } => self.paragraph(body, context),
                    ModalFrame::Activated(cost) => {
                        self.cost(cost, Some(RecoveryRole::ActivationCost));
                    }
                    ModalFrame::Triggered {
                        event,
                        intervening_condition,
                        ..
                    } => {
                        self.trigger_event(event, context);
                        if let Some(condition) = intervening_condition {
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
            AbilityKind::Keyword(list) => {
                for keyword in &list.abilities {
                    self.keyword_argument(&keyword.argument, context);
                }
            }
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph, context),
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
        for component in &cost.components {
            match component {
                // A recovered component is the only thing that recovers at the
                // activation-cost role; a clause or noun component recurses so
                // its interior failures stay attributed to their own roles, and
                // a symbol run never recovers.
                CostComponent::Recovered(text) => {
                    self.push(text, RecoveryRole::ActivationCost, context);
                }
                CostComponent::Clause(clause) => self.independent_clause(clause, context),
                CostComponent::Noun(noun) => self.noun_phrase(noun, context),
                CostComponent::Symbols(_) => {}
            }
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
            match &sentence.body {
                SentenceBody::Independent(clause) => self.independent_clause(clause, context),
                SentenceBody::Choice(choice) => self.choice_instruction(choice, context),
                SentenceBody::Recovered(unknown) => {
                    self.push(unknown, RecoveryRole::Clause, context);
                }
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
            IndependentClause::Transitive(subject, predicate) => {
                self.subject(subject, context);
                self.transitive_predicate(predicate, context);
            }
            IndependentClause::Intransitive(subject, predicate) => {
                self.subject(subject, context);
                Self::predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            IndependentClause::Copular(subject, predicate) => {
                self.subject(subject, context);
                self.copular_complement(&predicate.complement, context);
                self.predicate_adjuncts(&predicate.adjuncts, context);
            }
            IndependentClause::Passive(subject, predicate) => {
                self.subject(subject, context);
                Self::predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            IndependentClause::Imperative(predicate) => self.predicate(predicate, context),
            IndependentClause::Deontic(subject, _, predicate) => {
                self.subject(subject, context);
                if let Some(predicate) = predicate {
                    self.predicate(predicate, context);
                }
            }
            IndependentClause::Existential(existential) => {
                self.noun_phrase(&existential.pivot, context);
                self.predicate_adjuncts(&existential.adjuncts, context);
            }
            IndependentClause::Proform(subject, _) => self.subject(subject, context),
            IndependentClause::Complex(complex) => {
                self.independent_clause(&complex.matrix, context);
                for attachment in &complex.attachments {
                    match &attachment.kind {
                        ClauseAttachmentKind::Dependent(clause) => {
                            self.dependent_clause(clause, context);
                        }
                        ClauseAttachmentKind::Adjunct(adjunct) => {
                            self.predicate_adjunct(adjunct, context);
                        }
                        ClauseAttachmentKind::Exception(rider) => {
                            self.independent_clause(&rider.first, context);
                            for conjunct in &rider.rest {
                                self.independent_clause(&conjunct.clause, context);
                            }
                        }
                    }
                }
            }
            IndependentClause::Coordinated(coordinated) => {
                self.independent_clause(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    match &coordination.member {
                        CoordinatedClauseMember::Independent(clause) => {
                            self.independent_clause(clause, context);
                        }
                        CoordinatedClauseMember::SharedPredicate(predicate) => {
                            self.predicate(predicate, context);
                        }
                    }
                }
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
                self.predicate(&clause.predicate, context);
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
                self.predicate(&infinitive.predicate, context);
            }
            DependentClause::Gerund(gerund) => self.gerund_clause(gerund, context),
        }
    }

    fn gerund_clause(&mut self, clause: &'syntax GerundClause, context: Option<RecoveryRole>) {
        self.predicate(&clause.predicate, context);
        for attachment in &clause.attachments {
            self.dependent_clause(&attachment.clause, context);
        }
    }

    fn subject(&mut self, subject: &'syntax Subject, context: Option<RecoveryRole>) {
        self.noun_phrase(&subject.0, context);
    }

    fn predicate(&mut self, predicate: &'syntax Predicate, context: Option<RecoveryRole>) {
        match predicate {
            Predicate::Transitive(predicate) => self.transitive_predicate(predicate, context),
            Predicate::Intransitive(predicate) => {
                Self::predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            Predicate::Copular(predicate) => {
                self.copular_complement(&predicate.complement, context);
                self.predicate_adjuncts(&predicate.adjuncts, context);
            }
            Predicate::Passive(predicate) => {
                Self::predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            Predicate::Proform(_) => {}
        }
    }

    fn transitive_predicate(
        &mut self,
        predicate: &'syntax TransitivePredicate,
        context: Option<RecoveryRole>,
    ) {
        Self::predicate_head(&predicate.head, context);
        self.predicate_elements(&predicate.pre_object_elements, context);
        self.predicate_object(&predicate.object, context);
        self.predicate_elements(&predicate.elements, context);
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
                    PredicateComplement::Prepositional(preposition) => {
                        self.prepositional_phrase(preposition, RecoveryRole::Clause, context);
                    }
                    PredicateComplement::Infinitive(infinitive) => {
                        self.predicate(&infinitive.predicate, context);
                    }
                },
                PredicateElement::Adjunct(adjunct) => self.predicate_adjunct(adjunct, context),
                PredicateElement::Particle(_) => {}
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
            PredicateAdjunct::Prepositional(preposition) => {
                self.prepositional_phrase(preposition, RecoveryRole::Clause, context);
            }
            PredicateAdjunct::Dependent(dependent) => {
                self.dependent_clause(dependent, context);
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
            CopularComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition, RecoveryRole::Clause, context);
            }
            CopularComplement::PowerToughness(_) | CopularComplement::CatalogAtom(_) => {}
        }
    }

    fn relative_clause(
        &mut self,
        relative: &'syntax RelativeClause,
        context: Option<RecoveryRole>,
    ) {
        match &relative.body {
            RelativeBody::SubjectGap(predicate) => self.predicate(predicate, context),
            RelativeBody::ModalSubjectGap { predicate, .. } => {
                if let Some(predicate) = predicate {
                    self.predicate(predicate, context);
                }
            }
            RelativeBody::ObjectGap { subject, predicate } => {
                self.subject(subject, context);
                Self::predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
        }
    }

    fn noun_phrase(&mut self, phrase: &'syntax NounPhrase, context: Option<RecoveryRole>) {
        match phrase {
            NounPhrase::Nominal(nominal) => {
                if let Some(Determiner::Possessive(Possessor::NounPhrase(possessor))) =
                    &nominal.determiner
                {
                    self.noun_phrase(possessor, context);
                }
                if let crate::word::NounInstance::Singular(crate::word::Noun::Opaque(opaque))
                | crate::word::NounInstance::Plural(crate::word::Noun::Opaque(opaque))
                | crate::word::NounInstance::Mass(crate::word::Noun::Opaque(opaque)) =
                    &nominal.head
                {
                    self.lexical_opacity.push(LexicalOpacityRef {
                        kind: LexicalOpacityKind::Noun,
                        text: opaque.spelling(),
                        source_tokens: 1,
                    });
                }
                for modifier in &nominal.modifiers {
                    match modifier {
                        NominalModifier::Adjective { phrase, .. } => {
                            self.adjective_phrase(phrase, RecoveryRole::NominalComplement, context);
                        }
                        NominalModifier::Noun { .. }
                        | NominalModifier::Quantity(_)
                        | NominalModifier::PowerToughness(_) => {}
                    }
                }
                for complement in &nominal.complements {
                    match complement {
                        NominalComplement::Adjective(adjective) => self.adjective_phrase(
                            adjective,
                            RecoveryRole::NominalComplement,
                            context,
                        ),
                        NominalComplement::Prepositional(preposition) => {
                            self.prepositional_phrase(
                                preposition,
                                RecoveryRole::NominalComplement,
                                context,
                            );
                        }
                        NominalComplement::Infinitive(infinitive) => {
                            self.predicate(&infinitive.predicate, context);
                        }
                        NominalComplement::Relative(relative) => {
                            self.relative_clause(relative, context);
                        }
                        NominalComplement::EventClause(clause) => {
                            self.independent_clause(clause, context);
                        }
                        NominalComplement::Quantity(_) | NominalComplement::Devotion(_) => {}
                    }
                }
            }
            NounPhrase::Pronoun { .. }
            | NounPhrase::Possessive(Possessor::Pronoun(_))
            | NounPhrase::Demonstrative(_)
            | NounPhrase::Quantity(_)
            | NounPhrase::ThisCard(_) => {}
            NounPhrase::Possessive(Possessor::NounPhrase(possessor)) => {
                self.noun_phrase(possessor, context);
            }
            NounPhrase::Partitive(partitive) => self.noun_phrase(&partitive.whole, context),
            NounPhrase::Coordinated(coordinated) => {
                self.noun_phrase(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    self.noun_phrase(&coordination.phrase, context);
                }
            }
            NounPhrase::Arithmetic(value) => match value {
                ArithmeticValue::Minus { left, right } => {
                    self.noun_phrase(left, context);
                    self.noun_phrase(right, context);
                }
                ArithmeticValue::Half { value, .. } => self.noun_phrase(value, context),
            },
        }
    }

    fn adjective_phrase(
        &mut self,
        phrase: &'syntax AdjectivePhrase,
        role: RecoveryRole,
        context: Option<RecoveryRole>,
    ) {
        for complement in &phrase.complements {
            match complement {
                AdjectiveComplement::Comparison(comparison)
                | AdjectiveComplement::PostnominalComparison(comparison) => {
                    self.phrase(&comparison.standard, role, context);
                }
                AdjectiveComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition, role, context);
                }
                AdjectiveComplement::Infinitive(infinitive) => {
                    self.predicate(&infinitive.predicate, context);
                }
            }
        }
    }

    fn prepositional_phrase(
        &mut self,
        phrase: &'syntax PrepositionalPhrase,
        role: RecoveryRole,
        context: Option<RecoveryRole>,
    ) {
        self.phrase(&phrase.object, role, context);
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
                self.prepositional_phrase(preposition, role, context);
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
            Determiner::Indefinite(IndefiniteArticle::An).noun_cardinality(),
            NounCardinality::SingularCount
        );
        assert_eq!(
            Determiner::Demonstrative(Demonstrative::Those).noun_cardinality(),
            NounCardinality::PluralCount
        );
        assert_eq!(
            Determiner::Demonstrative(Demonstrative::That).noun_cardinality(),
            NounCardinality::SingularOrMass
        );
        assert_eq!(
            Determiner::Demonstrative(Demonstrative::This).noun_cardinality(),
            NounCardinality::SingularOrMass
        );
        assert_eq!(
            Determiner::Target(Some(Quantity::UpTo(one))).noun_cardinality(),
            NounCardinality::SingularCount
        );
        assert_eq!(
            Determiner::Target(Some(Quantity::UpTo(three))).noun_cardinality(),
            NounCardinality::PluralCount
        );
        assert_eq!(
            Determiner::Quantity(Quantity::ThatMuch).noun_cardinality(),
            NounCardinality::Mass
        );
        assert_eq!(
            Determiner::Possessive(Possessor::Pronoun(Pronoun::You)).noun_cardinality(),
            NounCardinality::Unconstrained
        );
        assert_eq!(
            Determiner::Quantity(Quantity::AtLeast(one)).noun_cardinality(),
            NounCardinality::PluralCount
        );
        assert_eq!(
            Determiner::Quantity(Quantity::Exact(three)).noun_cardinality(),
            NounCardinality::PluralOrMass
        );
        assert_eq!(
            Determiner::Any.noun_cardinality(),
            NounCardinality::Unconstrained
        );
        assert_eq!(
            Determiner::No.noun_cardinality(),
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
                paragraph(IndependentClause::Intransitive(
                    Subject(NounPhrase::Nominal(NominalPhrase {
                        determiner: None,
                        modifiers: vec![],
                        head: NounInstance::Singular(Noun::Opaque(OpaqueLexeme::new("blorple"))),
                        complements: vec![],
                    })),
                    intransitive(Vocab::Draw),
                )),
                Ability {
                    ability_word: None,
                    kind: AbilityKind::Activated(ActivatedAbility {
                        cost: Cost {
                            components: vec![CostComponent::Recovered(recovered("cost"))],
                        },
                        effect: Paragraph::default(),
                        effect_initial_uppercase: true,
                    }),
                },
                Ability {
                    ability_word: None,
                    kind: AbilityKind::Keyword(KeywordAbilityList {
                        abilities: vec![KeywordAbility {
                            preceding_separator: None,
                            ability: flying,
                            argument: KeywordArgument::Recovered {
                                separator: KeywordArgumentSeparator::Space,
                                text: recovered("argument"),
                            },
                        }],
                    }),
                },
                Ability {
                    ability_word: None,
                    kind: AbilityKind::Modal(ModalAbility {
                        frame: ModalFrame::Unframed,
                        header: paragraph_body(SentenceBody::Recovered(recovered("header"))),
                        header_suffix: ModalHeaderSuffix::None,
                        modes: vec![],
                    }),
                },
                paragraph(IndependentClause::Imperative(Predicate::Transitive(
                    TransitivePredicate {
                        head: predicate_head(Vocab::Draw),
                        pre_object_elements: vec![],
                        object: PredicateObject::QuotedAbility(Box::new(QuotedAbility {
                            ability: Box::new(paragraph_recovered("embedded")),
                            initial_uppercase: false,
                            closed: true,
                            terminal_period: true,
                        })),
                        elements: vec![],
                    },
                ))),
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
            first_auxiliary_contracted_with_subject: false,
            preverb_modifiers: vec![],
            verb: VerbInstance {
                verb: Verb::Word(vocab),
                slot: VerbSlot::Imperative,
            },
        }
    }

    fn intransitive(vocab: Vocab) -> IntransitivePredicate {
        IntransitivePredicate {
            head: predicate_head(vocab),
            elements: vec![],
        }
    }

    fn paragraph(clause: IndependentClause) -> Ability {
        Ability {
            ability_word: None,
            kind: AbilityKind::Paragraph(paragraph_body(SentenceBody::Independent(clause))),
        }
    }

    fn paragraph_recovered(text: &str) -> Ability {
        Ability {
            ability_word: None,
            kind: AbilityKind::Paragraph(paragraph_body(SentenceBody::Recovered(recovered(text)))),
        }
    }

    fn activated_with_cost(components: Vec<CostComponent>) -> OracleText {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Activated(ActivatedAbility {
                    cost: Cost { components },
                    effect: Paragraph::default(),
                    effect_initial_uppercase: true,
                }),
            }],
        }
    }

    fn paragraph_body(body: SentenceBody) -> Paragraph {
        Paragraph {
            flavor_header: None,
            sentences: vec![Sentence {
                initial_uppercase: true,
                body,
            }],
        }
    }
}
