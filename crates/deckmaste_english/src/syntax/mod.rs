mod ability;
mod clause;
mod phrase;

pub use ability::*;
pub use clause::*;
pub use phrase::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownRole {
    Clause,
    NominalComplement,
    ActivationCost,
    KeywordArgument,
    ModalHeader,
    EmbeddedRules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownPhraseRef<'a> {
    pub role: UnknownRole,
    pub text: &'a str,
}

impl OracleText {
    #[must_use]
    pub fn unknown_phrases(&self) -> Vec<UnknownPhraseRef<'_>> {
        let mut walker = UnknownWalker::default();
        walker.oracle_text(self);
        walker.phrases
    }
}

#[derive(Default)]
struct UnknownWalker<'syntax> {
    phrases: Vec<UnknownPhraseRef<'syntax>>,
}

impl<'syntax> UnknownWalker<'syntax> {
    fn oracle_text(&mut self, oracle_text: &'syntax OracleText) {
        for ability in &oracle_text.abilities {
            self.ability(ability, None);
        }
    }

    fn ability(&mut self, ability: &'syntax Ability, context: Option<UnknownRole>) {
        match &ability.kind {
            AbilityKind::Activated(activated) => {
                self.cost(&activated.cost, Some(UnknownRole::ActivationCost));
                self.paragraph(&activated.effect, context);
            }
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
                    ModalFrame::Unframed | ModalFrame::Loyalty(_) => {}
                    ModalFrame::Preamble { body, .. } => self.paragraph(body, context),
                    ModalFrame::Activated(cost) => {
                        self.cost(cost, Some(UnknownRole::ActivationCost));
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
                self.paragraph(&modal.header, Some(UnknownRole::ModalHeader));
                for mode in &modal.modes {
                    self.paragraph(&mode.body, context);
                }
            }
            AbilityKind::Keyword(list) => {
                for keyword in &list.abilities {
                    if let Some(argument) = &keyword.argument {
                        self.phrase(
                            argument,
                            UnknownRole::KeywordArgument,
                            Some(UnknownRole::KeywordArgument),
                        );
                    }
                }
            }
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph, context),
        }
    }

    fn cost(&mut self, cost: &'syntax Cost, context: Option<UnknownRole>) {
        for component in &cost.components {
            self.phrase(component, UnknownRole::ActivationCost, context);
        }
    }

    fn paragraph(&mut self, paragraph: &'syntax Paragraph, context: Option<UnknownRole>) {
        for sentence in &paragraph.sentences {
            match &sentence.body {
                SentenceBody::Independent(clause) => self.independent_clause(clause, context),
                SentenceBody::Unknown(unknown) => {
                    self.push(unknown, UnknownRole::Clause, context);
                }
            }
        }
    }

    fn clause(&mut self, clause: &'syntax Clause, context: Option<UnknownRole>) {
        match clause {
            Clause::Independent(clause) => self.independent_clause(clause, context),
            Clause::Dependent(clause) => self.dependent_clause(clause, context),
        }
    }

    fn trigger_event(&mut self, event: &'syntax TriggerEvent, context: Option<UnknownRole>) {
        match event {
            TriggerEvent::Clause(clause) => self.independent_clause(clause, context),
            TriggerEvent::Temporal(noun_phrase) => self.noun_phrase(noun_phrase, context),
        }
    }

    fn independent_clause(
        &mut self,
        clause: &'syntax IndependentClause,
        context: Option<UnknownRole>,
    ) {
        match clause {
            IndependentClause::Transitive(subject, predicate) => {
                self.subject(subject, context);
                self.transitive_predicate(predicate, context);
            }
            IndependentClause::Intransitive(subject, predicate) => {
                self.subject(subject, context);
                self.predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            IndependentClause::Copular(subject, predicate) => {
                self.subject(subject, context);
                self.copular_complement(&predicate.complement, context);
                self.predicate_adjuncts(&predicate.adjuncts, context);
            }
            IndependentClause::Passive(subject, predicate) => {
                self.subject(subject, context);
                self.predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            IndependentClause::Imperative(predicate) => self.predicate(predicate, context),
            IndependentClause::Deontic(subject, _, predicate) => {
                self.subject(subject, context);
                self.predicate(predicate, context);
            }
            IndependentClause::Existential(existential) => {
                self.noun_phrase(&existential.pivot, context);
                self.predicate_adjuncts(&existential.adjuncts, context);
            }
            IndependentClause::Proform(subject, _) => self.subject(subject, context),
            IndependentClause::Complex(complex) => {
                self.independent_clause(&complex.matrix, context);
                for attachment in &complex.attachments {
                    self.dependent_clause(&attachment.clause, context);
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

    fn dependent_clause(&mut self, clause: &'syntax DependentClause, context: Option<UnknownRole>) {
        match clause {
            DependentClause::Subordinate(_, SubordinateBody::Finite(clause)) => {
                self.independent_clause(clause, context);
            }
            DependentClause::Subordinate(
                _,
                SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
            ) => self.adjective_phrase(adjective, UnknownRole::Clause, context),
            DependentClause::Relative(relative) => self.relative_clause(relative, context),
            DependentClause::Infinitive(infinitive) => {
                self.predicate(&infinitive.predicate, context);
            }
        }
    }

    fn subject(&mut self, subject: &'syntax Subject, context: Option<UnknownRole>) {
        self.noun_phrase(&subject.0, context);
    }

    fn predicate(&mut self, predicate: &'syntax Predicate, context: Option<UnknownRole>) {
        match predicate {
            Predicate::Transitive(predicate) => self.transitive_predicate(predicate, context),
            Predicate::Intransitive(predicate) => {
                self.predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            Predicate::Copular(predicate) => {
                self.copular_complement(&predicate.complement, context);
                self.predicate_adjuncts(&predicate.adjuncts, context);
            }
            Predicate::Passive(predicate) => {
                self.predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
            Predicate::Proform(_) => {}
        }
    }

    fn transitive_predicate(
        &mut self,
        predicate: &'syntax TransitivePredicate,
        context: Option<UnknownRole>,
    ) {
        self.predicate_head(&predicate.head, context);
        self.predicate_object(&predicate.object, context);
        self.predicate_elements(&predicate.elements, context);
    }

    fn predicate_head(&mut self, head: &'syntax PredicateHead, context: Option<UnknownRole>) {
        let _ = (head, context);
    }

    fn predicate_object(&mut self, object: &'syntax PredicateObject, context: Option<UnknownRole>) {
        match object {
            PredicateObject::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
            PredicateObject::EmbeddedAbility(ability) => self.ability(ability, context),
            PredicateObject::QuotedAbility(quoted) => {
                self.ability(&quoted.ability, Some(UnknownRole::EmbeddedRules));
            }
            PredicateObject::Ability(ability) => {
                if let Some(argument) = &ability.argument {
                    self.predicate_object(argument, context);
                }
            }
            PredicateObject::Quantity(_)
            | PredicateObject::OracleSymbol(_)
            | PredicateObject::PowerToughness(_) => {}
        }
    }

    fn predicate_elements(
        &mut self,
        elements: &'syntax [PredicateElement],
        context: Option<UnknownRole>,
    ) {
        for element in elements {
            match element {
                PredicateElement::Complement(complement) => match complement {
                    PredicateComplement::IndirectObject(noun_phrase) => {
                        self.noun_phrase(noun_phrase, context);
                    }
                    PredicateComplement::Adjective(adjective) => {
                        self.adjective_phrase(adjective, UnknownRole::Clause, context);
                    }
                    PredicateComplement::Prepositional(preposition) => {
                        self.prepositional_phrase(preposition, UnknownRole::Clause, context)
                    }
                    PredicateComplement::Infinitive(infinitive) => {
                        self.predicate(&infinitive.predicate, context);
                    }
                },
                PredicateElement::Adjunct(adjunct) => self.predicate_adjunct(adjunct, context),
            }
        }
    }

    fn predicate_adjuncts(
        &mut self,
        adjuncts: &'syntax [PredicateAdjunct],
        context: Option<UnknownRole>,
    ) {
        for adjunct in adjuncts {
            self.predicate_adjunct(adjunct, context);
        }
    }

    fn predicate_adjunct(
        &mut self,
        adjunct: &'syntax PredicateAdjunct,
        context: Option<UnknownRole>,
    ) {
        match adjunct {
            PredicateAdjunct::Temporal(noun_phrase) => self.noun_phrase(noun_phrase, context),
            PredicateAdjunct::Prepositional(preposition) => {
                self.prepositional_phrase(preposition, UnknownRole::Clause, context);
            }
            PredicateAdjunct::Dependent(dependent) => {
                self.dependent_clause(dependent, context);
            }
            PredicateAdjunct::Adverb(_) => {}
        }
    }

    fn copular_complement(
        &mut self,
        complement: &'syntax CopularComplement,
        context: Option<UnknownRole>,
    ) {
        match complement {
            CopularComplement::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
            CopularComplement::Adjective(adjective) => {
                self.adjective_phrase(adjective, UnknownRole::Clause, context);
            }
            CopularComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition, UnknownRole::Clause, context)
            }
            CopularComplement::CatalogAtom(_) => {}
        }
    }

    fn relative_clause(&mut self, relative: &'syntax RelativeClause, context: Option<UnknownRole>) {
        match &relative.body {
            RelativeBody::SubjectGap(predicate) => self.predicate(predicate, context),
            RelativeBody::ObjectGap { subject, predicate } => {
                self.subject(subject, context);
                self.predicate_head(&predicate.head, context);
                self.predicate_elements(&predicate.elements, context);
            }
        }
    }

    fn noun_phrase(&mut self, phrase: &'syntax NounPhrase, context: Option<UnknownRole>) {
        match phrase {
            NounPhrase::Nominal(nominal) => {
                if let Some(Determiner::Possessive(Possessor::NounPhrase(possessor))) =
                    &nominal.determiner
                {
                    self.noun_phrase(possessor, context);
                }
                if let crate::word::NounInstance::Singular(crate::word::Noun::Unknown(unknown))
                | crate::word::NounInstance::Plural(crate::word::Noun::Unknown(unknown))
                | crate::word::NounInstance::Mass(crate::word::Noun::Unknown(unknown)) =
                    &nominal.head
                {
                    self.push(unknown, UnknownRole::NominalComplement, context);
                }
                for modifier in &nominal.modifiers {
                    match modifier {
                        NominalModifier::Adjective(adjective) => self.adjective_phrase(
                            adjective,
                            UnknownRole::NominalComplement,
                            context,
                        ),
                        NominalModifier::Unknown(unknown) => {
                            self.push(unknown, UnknownRole::NominalComplement, context);
                        }
                        NominalModifier::Noun(_) | NominalModifier::PowerToughness(_) => {}
                    }
                }
                for complement in &nominal.complements {
                    match complement {
                        NominalComplement::Prepositional(preposition) => {
                            self.prepositional_phrase(
                                preposition,
                                UnknownRole::NominalComplement,
                                context,
                            );
                        }
                        NominalComplement::Relative(relative) => {
                            self.relative_clause(relative, context);
                        }
                        NominalComplement::Unknown(unknown) => {
                            self.push(unknown, UnknownRole::NominalComplement, context);
                        }
                    }
                }
            }
            NounPhrase::Pronoun { .. } | NounPhrase::ThisCard(_) => {}
            NounPhrase::Coordinated(coordinated) => {
                self.noun_phrase(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    self.noun_phrase(&coordination.phrase, context);
                }
            }
        }
    }

    fn adjective_phrase(
        &mut self,
        phrase: &'syntax AdjectivePhrase,
        role: UnknownRole,
        context: Option<UnknownRole>,
    ) {
        for complement in &phrase.complements {
            match complement {
                AdjectiveComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition, role, context);
                }
                AdjectiveComplement::Infinitive(infinitive) => {
                    self.predicate(&infinitive.predicate, context);
                }
                AdjectiveComplement::Unknown(unknown) => self.push(unknown, role, context),
            }
        }
    }

    fn prepositional_phrase(
        &mut self,
        phrase: &'syntax PrepositionalPhrase,
        role: UnknownRole,
        context: Option<UnknownRole>,
    ) {
        self.phrase(&phrase.object, role, context);
    }

    fn phrase(&mut self, phrase: &'syntax Phrase, role: UnknownRole, context: Option<UnknownRole>) {
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
                self.ability(&quoted.ability, Some(UnknownRole::EmbeddedRules));
            }
            Phrase::UnknownPhrase(unknown) => self.push(unknown, role, context),
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
        unknown: &'syntax UnknownPhrase,
        role: UnknownRole,
        context: Option<UnknownRole>,
    ) {
        self.phrases.push(UnknownPhraseRef {
            role: context.unwrap_or(role),
            text: &unknown.0,
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
    }

    #[test]
    fn unknown_walker_reports_the_grammatical_role_of_every_opaque_leaf() {
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
                paragraph_unknown("clause"),
                paragraph(IndependentClause::Intransitive(
                    Subject(NounPhrase::Nominal(NominalPhrase {
                        determiner: None,
                        modifiers: vec![],
                        head: NounInstance::Singular(Noun::Word(Vocab::Card)),
                        complements: vec![NominalComplement::Unknown(unknown("nominal"))],
                    })),
                    intransitive(Vocab::Draw),
                )),
                Ability {
                    ability_word: None,
                    kind: AbilityKind::Activated(ActivatedAbility {
                        cost: Cost {
                            components: vec![Phrase::UnknownPhrase(unknown("cost"))],
                        },
                        effect: Paragraph::default(),
                    }),
                },
                Ability {
                    ability_word: None,
                    kind: AbilityKind::Keyword(KeywordAbilityList {
                        abilities: vec![KeywordAbility {
                            preceding_separator: None,
                            ability: flying,
                            argument_separator: Some(KeywordArgumentSeparator::Space),
                            argument: Some(Phrase::UnknownPhrase(unknown("argument"))),
                        }],
                    }),
                },
                Ability {
                    ability_word: None,
                    kind: AbilityKind::Modal(ModalAbility {
                        frame: ModalFrame::Unframed,
                        header: paragraph_body(SentenceBody::Unknown(unknown("header"))),
                        header_suffix: ModalHeaderSuffix::None,
                        modes: vec![],
                    }),
                },
                paragraph(IndependentClause::Imperative(Predicate::Transitive(
                    TransitivePredicate {
                        head: predicate_head(Vocab::Draw),
                        object: PredicateObject::QuotedAbility(Box::new(QuotedAbility {
                            ability: Box::new(paragraph_unknown("embedded")),
                            closed: true,
                        })),
                        elements: vec![],
                    },
                ))),
            ],
        };

        assert_eq!(
            oracle_text.unknown_phrases(),
            vec![
                UnknownPhraseRef {
                    role: UnknownRole::Clause,
                    text: "clause"
                },
                UnknownPhraseRef {
                    role: UnknownRole::NominalComplement,
                    text: "nominal"
                },
                UnknownPhraseRef {
                    role: UnknownRole::ActivationCost,
                    text: "cost"
                },
                UnknownPhraseRef {
                    role: UnknownRole::KeywordArgument,
                    text: "argument"
                },
                UnknownPhraseRef {
                    role: UnknownRole::ModalHeader,
                    text: "header"
                },
                UnknownPhraseRef {
                    role: UnknownRole::EmbeddedRules,
                    text: "embedded"
                },
            ]
        );
    }

    #[test]
    fn unknown_walker_reports_unsupported_predicates_at_sentence_scope() {
        let ast = crate::parse("You frobnitz a card.").into_ast();

        assert!(ast.unknown_phrases().contains(&UnknownPhraseRef {
            role: UnknownRole::Clause,
            text: "You frobnitz a card",
        }));
    }

    fn unknown(text: &str) -> UnknownPhrase {
        UnknownPhrase(text.to_owned())
    }

    fn predicate_head(vocab: Vocab) -> PredicateHead {
        PredicateHead {
            auxiliaries: vec![],
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

    fn paragraph_unknown(text: &str) -> Ability {
        Ability {
            ability_word: None,
            kind: AbilityKind::Paragraph(paragraph_body(SentenceBody::Unknown(unknown(text)))),
        }
    }

    fn paragraph_body(body: SentenceBody) -> Paragraph {
        Paragraph {
            sentences: vec![Sentence {
                body,
                ending: SentenceEnding::Period(1),
            }],
        }
    }
}
