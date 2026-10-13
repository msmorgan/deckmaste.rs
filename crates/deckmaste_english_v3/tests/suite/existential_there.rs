use crate::common;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::WordForm;

fn witness(text: &str, displaced: &str, location: &str) {
    let values = readings(text, Category::Document);
    assert!(!values.is_empty(), "{text}");
    for value in values {
        let mut clauses = 0;
        value
            .visit(&mut |node| {
                if let Reading::FiniteClause {
                    subject, predicate, ..
                } = node
                    && predicate.category() == Category::ExistentialPredicate
                {
                    let Reading::NominativePronoun { head: subject, .. } = &**subject else {
                        panic!("expected dummy Pronoun")
                    };
                    let LexicalReading::Word(subject_value) = &subject.value else {
                        panic!("dummy Subject must be a declared pronoun")
                    };
                    assert_eq!(subject_value.lexeme.as_str(), "vocab:DummyPronoun/There");
                    assert_eq!(subject_value.features.number, None);
                    assert_eq!(subject_value.features.person, None);
                    assert_eq!(subject.frame, None);
                }
                if let Reading::ExistentialPredicate {
                    head,
                    displaced_subject,
                    extension,
                    ..
                } = node
                {
                    clauses += 1;
                    let LexicalReading::Word(head_value) = &head.value else {
                        panic!("verb must be a declared word")
                    };
                    assert_eq!(head_value.lexeme.as_str(), "core-verb:Be");
                    assert_eq!(
                        lexicon().lexemes()[&head_value.lexeme].properties.frames
                            [head.frame.unwrap()]
                        .kind,
                        "Existential"
                    );
                    assert_eq!(displaced_subject.category(), Category::AccusativePhrase);
                    if let Some(extension) = extension {
                        assert_eq!(displaced_subject.realize(lexicon()).unwrap(), displaced);
                        assert_eq!(
                            extension.realize(lexicon()).unwrap(),
                            format!(" {location}")
                        );
                    } else {
                        // Existing hosts also license a higher PP attachment.
                        let observed = displaced_subject.realize(lexicon()).unwrap();
                        assert!(
                            [displaced.to_owned(), format!("{displaced} {location}")]
                                .contains(&observed),
                            "unexpected displaced Subject: {observed}"
                        );
                    }
                }
            })
            .unwrap();
        assert_eq!(clauses, 1);
    }
}

#[test]
fn sandstone_needle_condition() {
    witness(
        "If there are no depletion counters on this land, sacrifice it.",
        "no depletion counters",
        "on this land",
    );
}

#[test]
fn glacial_crasher_unless() {
    witness(
        "This creature can't attack unless there is a Mountain on the battlefield.",
        "a Mountain",
        "on the battlefield",
    );
}

#[test]
fn kamahls_desire_as_long_as() {
    witness(
        "Threshold — Enchanted creature gets +3/+0 as long as there are seven or more cards in your graveyard.",
        "seven or more cards",
        "in your graveyard",
    );
}

#[test]
fn case_of_the_shifting_visage_main_clause() {
    witness(
        "To solve — There are fifteen or more cards in your graveyard.",
        "fifteen or more cards",
        "in your graveyard",
    );
}

#[test]
fn drop_of_honey_temporal_clause() {
    witness(
        "When there are no creatures on the battlefield, sacrifice this enchantment.",
        "no creatures",
        "on the battlefield",
    );
}

#[test]
fn agreement_mismatches_do_not_read() {
    for text in [
        "there are a Mountain on the battlefield",
        "there is no creatures on the battlefield",
        "there is seven or more cards in your graveyard",
        "there be fifteen or more cards in your graveyard",
        "there am a Mountain on the battlefield",
        "there's no creatures on the battlefield",
        "there were a Mountain on the battlefield",
        "there was three or more card types among the sacrificed permanents",
    ] {
        assert!(readings(text, Category::FiniteClause).is_empty(), "{text}");
    }
}

#[test]
fn bare_and_extended_existentials_preserve_distinct_constituencies() {
    // Drop of Honey's condition: NP-internal Modifier and verb Complement
    // remain distinct grammatical Readings alongside the predicate Adjunct.
    let text = "there are no creatures on the battlefield";
    let values = readings(text, Category::FiniteClause);
    assert_eq!(values.len(), 3);
    let observed: std::collections::BTreeSet<_> = values
        .iter()
        .map(|value| {
            let Reading::FiniteClause { predicate, .. } = value else {
                panic!("expected finite Clause")
            };
            let mut predicate = &**predicate;
            while let Reading::PrepositionPredicate { head, .. } = predicate {
                predicate = head;
            }
            let Reading::ExistentialPredicate {
                displaced_subject,
                extension,
                ..
            } = predicate
            else {
                panic!("expected existential Predicate")
            };
            (
                displaced_subject.realize(lexicon()).unwrap(),
                extension.as_ref().map(|v| v.realize(lexicon()).unwrap()),
            )
        })
        .collect();
    assert_eq!(
        observed,
        std::collections::BTreeSet::from([
            (
                "no creatures".to_owned(),
                Some(" on the battlefield".to_owned())
            ),
            ("no creatures on the battlefield".to_owned(), None),
            ("no creatures".to_owned(), None),
        ])
    );
    common::assert_constituents(
        text,
        Category::FiniteClause,
        &[
            (Category::NounPhrase, "no creatures"),
            (Category::LocativeComplement, "on the battlefield"),
        ],
    );
    common::assert_constituents(
        text,
        Category::FiniteClause,
        &[
            (Category::NounPhrase, "no creatures on the battlefield"),
            (Category::PrepositionPhrase, "on the battlefield"),
        ],
    );
}

#[test]
fn dummy_pronoun_has_no_ordinary_nominal_or_adverb_route() {
    for category in [
        Category::NounPhrase,
        Category::NominativePhrase,
        Category::AccusativePhrase,
        Category::AdverbPhrase,
    ] {
        assert!(readings("there", category).is_empty(), "{category:?}");
    }
    for text in [
        "there attacks",
        "there is tapped",
        "there remains a creature",
    ] {
        assert!(readings(text, Category::FiniteClause).is_empty(), "{text}");
    }
    assert!(
        !lexicon()
            .lexemes()
            .contains_key(&deckmaste_lexical::LexemeId::from("vocab:Adverb/There"))
    );
}

#[test]
fn contracted_ambiguity_and_preterite_lexical_form() {
    // World at War retains the compound-noun and noun-premodification
    // Readings of "combat phase", with one existential frame each.
    assert_eq!(
        readings("there's an additional combat phase", Category::FiniteClause).len(),
        2
    );
    // Baba Lysaga's attested clause now retains all three attachment Readings
    // after the card-noun-premodifier landing retires its vocabulary blocker.
    assert_eq!(
        readings(
            "there were three or more card types among the sacrificed permanents",
            Category::FiniteClause
        )
        .len(),
        3
    );
    // Re-spell the retired blocker assertion against an undeclared Attribute
    // Premodifier head; the authentic clause above remains a positive witness.
    assert_eq!(
        readings(
            "there were three or more card cards among the sacrificed permanents",
            Category::FiniteClause
        )
        .len(),
        0
    );
    let preterite = word("core-verb:Be", WordForm::Preterite, Some(Number::Plural));
    assert_eq!(lexicon().realize(&preterite.value).unwrap(), "were");
}

fn word(id: &str, form: WordForm, number: Option<Number>) -> Word {
    let value = lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == id
                && value.form == form
                && value.features.number == number
                && (form != WordForm::Present || value.features.person == Some(Person::Third))
        })
        .unwrap()
        .clone();
    let lexeme = &lexicon().lexemes()[&value.lexeme];
    Word {
        value: LexicalReading::Word(value),
        frame: lexeme
            .properties
            .frames
            .iter()
            .position(|frame| frame.kind == "Existential")
            .or_else(|| (!lexeme.properties.frames.is_empty()).then_some(0)),
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn independent_condition() -> Reading {
    // Independently build Drop of Honey's displaced NP and locative
    // extension, without deriving the value from a parse.
    Reading::FiniteClause {
        form: 1,
        subject: Box::new(Reading::NominativePronoun {
            category: Category::DummyPronoun,
            form: 0,
            head: word("vocab:DummyPronoun/There", WordForm::Invariant, None),
        }),
        predicate: Box::new(Reading::ExistentialPredicate {
            form: 0,
            head: word("core-verb:Be", WordForm::Present, Some(Number::Plural)),
            displaced_subject: Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(Reading::DeterminedNounPhrase {
                    form: 0,
                    determiner: word("vocab:Determinative/No", WordForm::Invariant, None),
                    head: Box::new(Reading::NominalProjection {
                        form: 0,
                        head: Box::new(Reading::NominalCoreProjection {
                            form: 0,
                            head: Box::new(Reading::Noun {
                                form: 0,
                                head: word(
                                    "lexeme:type/creature",
                                    WordForm::Plural,
                                    Some(Number::Plural),
                                ),
                            }),
                        }),
                    }),
                }),
            }),
            extension: Some(Box::new(Reading::ExistentialExtension {
                form: 0,
                complement: Box::new(Reading::LocativeComplement {
                    form: 0,
                    phrase: Box::new(Reading::PrepositionPhrase {
                        form: 0,
                        head: word("vocab:Preposition/On", WordForm::Invariant, None),
                        complement: Box::new(Reading::CasePhrase {
                            category: Category::AccusativePhrase,
                            form: 0,
                            head: Box::new(Reading::DeterminedNounPhrase {
                                form: 0,
                                determiner: word(
                                    "vocab:DefiniteMarker/The",
                                    WordForm::Invariant,
                                    None,
                                ),
                                head: Box::new(Reading::NominalProjection {
                                    form: 0,
                                    head: Box::new(Reading::NominalCoreProjection {
                                        form: 0,
                                        head: Box::new(Reading::Noun {
                                            form: 0,
                                            head: word(
                                                "lexeme:CommonNoun/Battlefield",
                                                WordForm::Singular,
                                                Some(Number::Singular),
                                            ),
                                        }),
                                    }),
                                }),
                            }),
                        }),
                    }),
                }),
            })),
        }),
    }
}

#[test]
fn independently_constructed_existential_retains_both_traversals() {
    let value = independent_condition();
    let text = "there are no creatures on the battlefield";
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let observed = readings(text, Category::FiniteClause)
        .get(&value)
        .unwrap()
        .clone();
    let (mut nodes, mut observed_nodes, mut words, mut observed_words) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    observed
        .visit(&mut |node| observed_nodes.push(node.clone()))
        .unwrap();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    observed
        .visit_words(&mut |word| observed_words.push(word.clone()))
        .unwrap();
    assert_eq!(nodes, observed_nodes);
    assert_eq!(words, observed_words);
}

#[test]
fn independently_constructed_bare_existential_roundtrips() {
    let mut value = independent_condition();
    let Reading::FiniteClause { predicate, .. } = &mut value else {
        unreachable!()
    };
    let Reading::ExistentialPredicate { extension, .. } = &mut **predicate else {
        unreachable!()
    };
    *extension = None;
    let text = "there are no creatures";
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    assert_eq!(
        readings(text, Category::FiniteClause),
        std::collections::BTreeSet::from([value])
    );
}

#[test]
fn independent_construction_rejects_wrong_agreement_and_lexical_permissions() {
    let mut value = independent_condition();
    let Reading::FiniteClause { predicate, .. } = &mut value else {
        unreachable!()
    };
    let Reading::ExistentialPredicate { head, .. } = &mut **predicate else {
        unreachable!()
    };
    *head = word("core-verb:Be", WordForm::Present, Some(Number::Singular));
    assert!(value.admit(lexicon()).is_err());
    assert!(value.realize(lexicon()).is_err());
    let mut value = independent_condition();
    let Reading::FiniteClause { subject, .. } = &mut value else { unreachable!() };
    let Reading::NominativePronoun { head, .. } = &mut **subject else {
        unreachable!()
    };
    *head = word(
        "vocab:SubjectPronoun/They",
        WordForm::Invariant,
        Some(Number::Plural),
    );
    assert!(value.admit(lexicon()).is_err());
    assert!(value.realize(lexicon()).is_err());
    let mut value = independent_condition();
    let Reading::FiniteClause { predicate, .. } = &mut value else {
        unreachable!()
    };
    let Reading::ExistentialPredicate { head, .. } = &mut **predicate else {
        unreachable!()
    };
    *head = word("core-verb:Remain", WordForm::Present, Some(Number::Plural));
    assert!(value.admit(lexicon()).is_err());
    assert!(value.realize(lexicon()).is_err());
}
