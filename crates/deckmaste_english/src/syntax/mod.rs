mod ability;
mod clause;
mod phrase;

pub use ability::*;
pub use clause::*;
pub use phrase::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownRole {
    Clause,
    Subject,
    VerbDependent,
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
                self.simple_clause(&triggered.event, context);
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
                    ModalFrame::Triggered { event, .. } => self.simple_clause(event, context),
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
            self.clause(&sentence.clause, context);
        }
    }

    fn clause(&mut self, clause: &'syntax Clause, context: Option<UnknownRole>) {
        match clause {
            Clause::Simple(simple) => self.simple_clause(simple, context),
            Clause::Conditional(conditional) => {
                self.clause(&conditional.condition, context);
                self.clause(&conditional.consequence, context);
            }
            Clause::Coordinated(coordinated) => {
                self.clause(&coordinated.first, context);
                for coordination in &coordinated.rest {
                    self.clause(&coordination.clause, context);
                }
            }
            Clause::Unknown(unknown) => self.push(unknown, UnknownRole::Clause, context),
        }
    }

    fn simple_clause(&mut self, clause: &'syntax SimpleClause, context: Option<UnknownRole>) {
        if let Some(subject) = &clause.subject {
            match subject {
                Subject::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
                Subject::Unknown(unknown) => self.push(unknown, UnknownRole::Subject, context),
            }
        }
        self.verb_phrase(&clause.predicate, context);
    }

    fn verb_phrase(&mut self, phrase: &'syntax VerbPhrase, context: Option<UnknownRole>) {
        for dependent in &phrase.dependents {
            match dependent {
                VerbDependent::DirectObject(noun_phrase)
                | VerbDependent::IndirectObject(noun_phrase) => {
                    self.noun_phrase(noun_phrase, context);
                }
                VerbDependent::PredicateComplement(phrase)
                | VerbDependent::Scalar(phrase)
                | VerbDependent::Statistic(phrase)
                | VerbDependent::Adverbial(phrase) => {
                    self.phrase(phrase, UnknownRole::VerbDependent, context);
                }
                VerbDependent::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition, UnknownRole::VerbDependent, context);
                }
                VerbDependent::Infinitive(infinitive) => {
                    self.verb_phrase(&infinitive.predicate, context);
                }
                VerbDependent::Subordinate(clause) => self.clause(clause, context),
                VerbDependent::Unknown(unknown) => {
                    self.push(unknown, UnknownRole::VerbDependent, context);
                }
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
                for modifier in &nominal.modifiers {
                    if let NominalModifier::Adjective(adjective) = modifier {
                        self.adjective_phrase(adjective, UnknownRole::NominalComplement, context);
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
                            self.clause(&relative.clause, context);
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
                    self.verb_phrase(&infinitive.predicate, context);
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
            Phrase::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase, context),
            Phrase::AdjectivePhrase(adjective) => {
                self.adjective_phrase(adjective, role, context);
            }
            Phrase::PrepositionalPhrase(preposition) => {
                self.prepositional_phrase(preposition, role, context);
            }
            Phrase::QuotedAbility(quoted) => {
                self.ability(&quoted.ability, Some(UnknownRole::EmbeddedRules));
            }
            Phrase::UnknownPhrase(unknown) => self.push(unknown, role, context),
            Phrase::Quantity(_)
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
                paragraph(Clause::Unknown(unknown("clause"))),
                paragraph(Clause::Simple(SimpleClause {
                    subject: Some(Subject::Unknown(unknown("subject"))),
                    predicate: predicate(vec![VerbDependent::Unknown(unknown("dependent"))]),
                })),
                paragraph(Clause::Simple(SimpleClause {
                    subject: Some(Subject::NounPhrase(NounPhrase::Nominal(NominalPhrase {
                        determiner: None,
                        modifiers: vec![],
                        head: NounInstance::Singular(Noun::Word(Vocab::Card)),
                        complements: vec![NominalComplement::Unknown(unknown("nominal"))],
                    }))),
                    predicate: predicate(vec![]),
                })),
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
                        header: paragraph_body(Clause::Unknown(unknown("header"))),
                        header_suffix: ModalHeaderSuffix::None,
                        modes: vec![],
                    }),
                },
                paragraph(Clause::Simple(SimpleClause {
                    subject: None,
                    predicate: predicate(vec![VerbDependent::PredicateComplement(
                        Phrase::QuotedAbility(Box::new(QuotedAbility {
                            ability: Box::new(paragraph(Clause::Unknown(unknown("embedded")))),
                            closed: true,
                        })),
                    )]),
                })),
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
                    role: UnknownRole::Subject,
                    text: "subject"
                },
                UnknownPhraseRef {
                    role: UnknownRole::VerbDependent,
                    text: "dependent"
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

    fn unknown(text: &str) -> UnknownPhrase {
        UnknownPhrase(text.to_owned())
    }

    fn predicate(dependents: Vec<VerbDependent>) -> VerbPhrase {
        VerbPhrase {
            auxiliaries: vec![],
            preverb_modifiers: vec![],
            verb: VerbInstance {
                verb: Verb::Word(Vocab::Draw),
                slot: VerbSlot::Imperative,
            },
            dependents,
        }
    }

    fn paragraph(clause: Clause) -> Ability {
        Ability {
            ability_word: None,
            kind: AbilityKind::Paragraph(paragraph_body(clause)),
        }
    }

    fn paragraph_body(clause: Clause) -> Paragraph {
        Paragraph {
            sentences: vec![Sentence {
                clause,
                ending: SentenceEnding::Period(1),
            }],
        }
    }
}
