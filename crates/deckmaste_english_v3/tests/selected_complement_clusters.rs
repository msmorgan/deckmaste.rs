mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn invariant(id: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn verb(id: &str, form: WordForm, frame: usize) -> Word {
    let mut word = invariant(id);
    word.frame = Some(frame);
    let LexicalReading::Word(value) = &mut word.value else { unreachable!() };
    value.form = form;
    value.features = match form {
        WordForm::Present | WordForm::Preterite => FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(if form == WordForm::Present { Tense::Present } else { Tense::Past }),
            finiteness: Some(Finiteness::Finite),
            ..Default::default()
        },
        WordForm::PastParticiple => FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..Default::default()
        },
        _ => FeatureBundle::default(),
    };
    word
}

fn noun_word(id: &str, count: bool) -> Word {
    let mut word = invariant(id);
    word.countability = Some(count);
    let LexicalReading::Word(value) = &mut word.value else { unreachable!() };
    value.form = WordForm::Singular;
    value.features.number = Some(Number::Singular);
    word
}

fn noun(id: &str) -> Reading {
    Reading::Noun {
        form: 0,
        head: noun_word(id, true),
    }
}

fn case(head: Reading, category: Category) -> Reading {
    Reading::CasePhrase {
        category,
        form: 0,
        head: Box::new(head),
    }
}

fn acc(head: Reading) -> Reading {
    case(head, Category::AccusativePhrase)
}

fn damage(value: i32) -> Reading {
    acc(Reading::MeasuredNounPhrase {
        form: 0,
        quantity: Box::new(Reading::UngroupedScalarNumeral {
            form: 0,
            head: Word {
                value: LexicalReading::Numeral {
                    value,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        head: noun_word("lexeme:CommonNoun/Damage", false),
    })
}

fn any_target(other: bool) -> Reading {
    let target = noun("lexeme:CommonNoun/Target");
    acc(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:Determinative/Any"),
        head: Box::new(if other {
            Reading::PremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::IntransitiveAdjective {
                    form: 0,
                    head: {
                        let mut head = invariant("vocab:AttributiveAdjective/Other");
                        head.frame = Some(0);
                        head
                    },
                }),
                head: Box::new(target),
            }
        } else {
            target
        }),
    })
}

fn you() -> Reading {
    let mut head = invariant("vocab:ObjectPronoun/You");
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Accusative),
        ..Default::default()
    };
    acc(Reading::AccusativePronoun { form: 0, head })
}

fn tail(amount: i32, recipient: Reading) -> Reading {
    Reading::ObjectPrepositionTail {
        form: 0,
        object: Box::new(damage(amount)),
        marker: invariant("vocab:Preposition/To"),
        complement: Box::new(recipient),
    }
}

fn clusters(first: i32, second: i32, recipient: Reading) -> Reading {
    Reading::SelectedComplementClustersPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: verb("core-verb:Deal", WordForm::Present, 4),
        tail: Box::new(Reading::Coordination {
            category: Category::SelectedComplementTail,
            form: 0,
            left: Box::new(tail(first, any_target(false))),
            coordinator: invariant("vocab:Coordinator/And"),
            right: Box::new(tail(second, recipient)),
        }),
    }
}

fn proper_name(name: &str) -> Reading {
    Reading::ProperNameNounPhrase {
        form: 0,
        head: Box::new(Reading::CatalogName {
            form: 0,
            head: invariant(&format!("catalog:card-names.txt/{name}")),
        }),
    }
}

fn sentence(subject: Reading, predicate: Reading) -> Reading {
    Reading::Sentence {
        form: 0,
        clause: Box::new(Reading::Declarative {
            form: 0,
            clause: Box::new(Reading::FiniteClause {
                form: 0,
                subject: Box::new(case(subject, Category::NominativePhrase)),
                predicate: Box::new(predicate),
            }),
        }),
    }
}

fn paragraph(sentence: Reading) -> Reading {
    Reading::Paragraph {
        form: 0,
        first: Box::new(Reading::SentenceItem {
            form: 0,
            sentence: Box::new(sentence),
        }),
        rest: vec![],
    }
}

fn roundtrip(text: &str, category: Category, expected: &Reading) -> BTreeSet<Reading> {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let actual = readings(text, category);
    let parsed = actual
        .get(expected)
        .expect("independent expected Reading must be parsed");
    let mut original_nodes = Vec::new();
    let mut parsed_nodes = Vec::new();
    expected
        .visit(&mut |node| original_nodes.push(node.clone()))
        .unwrap();
    parsed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(original_nodes, parsed_nodes);
    let mut original_words = Vec::new();
    let mut parsed_words = Vec::new();
    expected
        .visit_words(&mut |word| original_words.push(word.clone()))
        .unwrap();
    parsed
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(original_words, parsed_words);
    assert_eq!(expected.total_cost().unwrap(), parsed.total_cost().unwrap());
    actual
}

#[test]
fn authentic_recipient_clusters_keep_independent_complete_sentence_structures() {
    // CGEL Ch. 15 §4.3, pp. 1341–1343: right nonce-constituent coordination.
    for (name, first, second, recipient, text) in [
        (
            "Arc Trail",
            2,
            1,
            any_target(true),
            "Arc Trail deals 2 damage to any target and 1 damage to any other target.",
        ),
        (
            "Char",
            4,
            2,
            you(),
            "Char deals 4 damage to any target and 2 damage to you.",
        ),
    ] {
        let expected = sentence(proper_name(name), clusters(first, second, recipient));
        assert_eq!(
            roundtrip(text, Category::Sentence, &expected),
            BTreeSet::from([expected])
        );
    }
    // Fireslinger's complete activated ability, including its actual cost.
    let mut this = invariant("vocab:SingularDemonstrative/This");
    let LexicalReading::Word(value) = &mut this.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
    let expected = Reading::ActivatedAbility {
        form: 0,
        cost: Box::new(Reading::Cost {
            form: 0,
            first: Box::new(Reading::SymbolCost {
                form: 0,
                symbols: Box::new(Reading::CostSymbols {
                    form: 0,
                    first: Box::new(Reading::NamedCostSymbol {
                        form: 0,
                        symbol: invariant("vocab:FixedCostSymbol/Tap"),
                    }),
                    rest: vec![],
                }),
            }),
            rest: vec![],
        }),
        body: Box::new(paragraph(sentence(
            Reading::DeterminedNounPhrase {
                form: 0,
                determiner: this,
                head: Box::new(noun("lexeme:type/creature")),
            },
            clusters(1, 1, you()),
        ))),
    };
    let text = "{T}: This creature deals 1 damage to any target and 1 damage to you.";
    assert_eq!(
        roundtrip(text, Category::Ability, &expected),
        BTreeSet::from([expected])
    );
}

#[test]
fn selected_cluster_hosts_require_coordination_and_parallel_selected_markers() {
    let valid = clusters(2, 1, any_target(true));
    for mixed in [false, true] {
        let mut invalid = valid.clone();
        let Reading::SelectedComplementClustersPredicate { tail, .. } = &mut invalid else {
            unreachable!()
        };
        let Reading::Coordination { left, right, .. } = tail.as_mut() else {
            unreachable!()
        };
        for item in if mixed { vec![right] } else { vec![left, right] } {
            let Reading::ObjectPrepositionTail { marker, .. } = item.as_mut() else {
                unreachable!()
            };
            *marker = invariant("vocab:Preposition/Into");
        }
        assert!(invalid.admit(&LEXICON).is_err());
        assert!(invalid.realize(&LEXICON).is_err());
        let text = if mixed {
            "deals 2 damage to any target and 1 damage into any other target"
        } else {
            "deals 2 damage into any target and 1 damage into any other target"
        };
        assert!(readings(text, Category::FinitePredicate).is_empty());
    }
    let invalid = Reading::SelectedComplementClustersPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: verb("core-verb:Deal", WordForm::Present, 4),
        tail: Box::new(tail(3, any_target(false))),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    assert!(invalid.realize(&LEXICON).is_err());
    // Lightning Bolt retains its ordinary, uncoordinated selected-frame AST.
    let ordinary = Reading::SelectedPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: verb("core-verb:Deal", WordForm::Present, 4),
        complements: vec![
            deckmaste_english_v3::grammar::FrameValue::Argument(Box::new(damage(3))),
            deckmaste_english_v3::grammar::FrameValue::Marker(invariant("vocab:Preposition/To")),
            deckmaste_english_v3::grammar::FrameValue::Argument(Box::new(any_target(false))),
        ],
    };
    assert_eq!(
        roundtrip(
            "deals 3 damage to any target",
            Category::FinitePredicate,
            &ordinary
        ),
        BTreeSet::from([ordinary])
    );
}

fn another_creature() -> Reading {
    acc(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:Determinative/Another"),
        head: Box::new(Reading::TargetedNominal {
            form: 0,
            marker: invariant("vocab:TargetingMarker/Target"),
            head: Box::new(noun("lexeme:type/creature")),
        }),
    })
}

fn be(passive: Reading) -> Reading {
    Reading::ParticipialAuxiliaryPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: verb("core-verb:Be", WordForm::Plain, 1),
        complement: Box::new(Reading::OvertComplement {
            category: Category::ParticipialComplement,
            form: 0,
            predicate: Box::new(Reading::PassiveComplement {
                form: 0,
                head: Box::new(passive),
            }),
        }),
    }
}

fn would(predicate: Reading) -> Reading {
    Reading::BareAuxiliaryPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: verb("core-verb:Would", WordForm::Preterite, 0),
        complement: Box::new(Reading::OvertComplement {
            category: Category::BareComplement,
            form: 0,
            predicate: Box::new(Reading::BarePredicate {
                form: 0,
                head: Box::new(predicate),
            }),
        }),
    }
}

#[test]
fn avacyn_recipient_stays_with_the_lexical_passive_below_both_auxiliaries() {
    // Actual constituent of Avacyn, Guardian Angel's first activated ability.
    let selected = Reading::SelectedGapPrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: verb("core-verb:Deal", WordForm::PastParticiple, 4),
        marker: invariant("vocab:Preposition/To"),
        complement: Box::new(another_creature()),
    };
    let expected = would(be(selected));
    let parsed = roundtrip(
        "would be dealt to another target creature",
        Category::FinitePredicate,
        &expected,
    );
    for value in parsed {
        value
            .visit(&mut |node| assert!(!matches!(node, Reading::PrepositionPredicate { .. })))
            .unwrap();
    }
    let bare = Reading::PassivePredicate {
        form: 0,
        head: verb("core-verb:Deal", WordForm::PastParticiple, 3),
    };
    for (category, head) in [
        (Category::SecondaryVerbPhrase, bare.clone()),
        (Category::SecondaryVerbPhrase, be(bare.clone())),
        (Category::FinitePredicate, would(be(bare))),
    ] {
        let invalid = Reading::PrepositionPredicate {
            category,
            form: 0,
            head: Box::new(head),
            modifier: Box::new(Reading::PrepositionPhrase {
                form: 0,
                head: invariant("vocab:Preposition/To"),
                complement: Box::new(another_creature()),
            }),
        };
        assert!(invalid.admit(&LEXICON).is_err());
        assert!(invalid.realize(&LEXICON).is_err());
    }
}

fn initial(mut word: Word) -> Word {
    let LexicalReading::Word(value) = &mut word.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
    word
}

fn this(kind: &str, capitalized: bool) -> Reading {
    let mut determiner = invariant("vocab:SingularDemonstrative/This");
    if capitalized {
        determiner = initial(determiner);
    }
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner,
        head: Box::new(noun(&format!("lexeme:type/{kind}"))),
    }
}

fn pronoun(id: &str, case: Case, person: Person, capitalized: bool) -> Word {
    let mut head = invariant(id);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(person),
        case: Some(case),
        ..Default::default()
    };
    if capitalized { initial(head) } else { head }
}

fn named_symbol(id: &str) -> Reading {
    Reading::NamedCostSymbol {
        form: 0,
        symbol: invariant(id),
    }
}

fn symbol_cost(symbols: Vec<Reading>) -> Reading {
    let mut symbols = symbols.into_iter();
    Reading::SymbolCost {
        form: 0,
        symbols: Box::new(Reading::CostSymbols {
            form: 0,
            first: Box::new(symbols.next().unwrap()),
            rest: symbols.collect(),
        }),
    }
}

fn activated(cost: Reading, body: Reading) -> Reading {
    Reading::ActivatedAbility {
        form: 0,
        cost: Box::new(cost),
        body: Box::new(paragraph(body)),
    }
}

fn cost(first: Reading, rest: Vec<Reading>) -> Reading {
    Reading::Cost {
        form: 0,
        first: Box::new(first),
        rest: rest
            .into_iter()
            .map(|component| Reading::CostContinuation {
                form: 0,
                component: Box::new(component),
            })
            .collect(),
    }
}

fn numeral(value: i32) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn numeric_symbol(value: i32) -> Reading {
    Reading::NumericCostSymbol {
        form: 0,
        number: numeral(value),
    }
}

fn enter_trigger(kind: &str, predicate: Reading) -> Reading {
    let subject = Reading::NominativePronoun {
        form: 0,
        head: pronoun(
            "vocab:SubjectPronoun/It",
            Case::Nominative,
            Person::Third,
            false,
        ),
    };
    Reading::Sentence {
        form: 0,
        clause: Box::new(Reading::InitialPreposition {
            form: 0,
            dependent: Box::new(Reading::ClauseComplementPreposition {
                form: 0,
                head: initial(invariant("vocab:TriggerMarker/When")),
                complement: Box::new(Reading::FiniteClause {
                    form: 0,
                    subject: Box::new(case(this(kind, false), Category::NominativePhrase)),
                    predicate: Box::new(Reading::SelectedPredicate {
                        category: Category::FinitePredicate,
                        form: 0,
                        head: verb("core-verb:Enter", WordForm::Present, 0),
                        complements: vec![],
                    }),
                }),
            }),
            clause: Box::new(Reading::Declarative {
                form: 0,
                clause: Box::new(Reading::FiniteClause {
                    form: 0,
                    subject: Box::new(case(subject, Category::NominativePhrase)),
                    predicate: Box::new(predicate),
                }),
            }),
        }),
    }
}

fn ordinary(sentence: Reading) -> Reading {
    Reading::OrdinaryAbility {
        form: 0,
        body: Box::new(paragraph(sentence)),
    }
}

fn document(first: Reading, rest: Vec<Reading>) -> Reading {
    Reading::Document {
        form: 0,
        first: Box::new(first),
        rest: rest
            .into_iter()
            .map(|ability| Reading::AbilityContinuation {
                form: 0,
                ability: Box::new(ability),
            })
            .collect(),
    }
}

fn bounded_roundtrip(text: &str, expected: Reading) {
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(&LEXICON);
    let forest = environment.parse(text, expected.category()).unwrap();
    // These finite authentic fixtures have fewer than 5,000 items at baseline.
    // Leave explicit headroom, while detecting renewed combinatorial growth.
    assert!(forest.metrics().items <= 12_000, "{:?}", forest.metrics());
    assert!(
        forest.metrics().families <= 13_000,
        "{:?}",
        forest.metrics()
    );
    assert_eq!(
        roundtrip(text, expected.category(), &expected),
        BTreeSet::from([expected])
    );
}

fn itself() -> Reading {
    acc(Reading::AccusativePronoun {
        form: 0,
        head: pronoun(
            "vocab:ReflexivePronoun/Itself",
            Case::Accusative,
            Person::Third,
            false,
        ),
    })
}

#[test]
fn all_nine_inherited_recipient_witnesses_have_exact_independent_document_sets() {
    for (_name, first, second, recipient, symbols, text) in [
        (
            "Brothers of Fire",
            1,
            1,
            you(),
            vec![
                numeric_symbol(1),
                named_symbol("vocab:FixedCostSymbol/Red"),
                named_symbol("vocab:FixedCostSymbol/Red"),
            ],
            "{1}{R}{R}: This creature deals 1 damage to any target and 1 damage to you.",
        ),
        (
            "Fireslinger",
            1,
            1,
            you(),
            vec![named_symbol("vocab:FixedCostSymbol/Tap")],
            "{T}: This creature deals 1 damage to any target and 1 damage to you.",
        ),
        (
            "Goblin Artillery",
            2,
            3,
            you(),
            vec![named_symbol("vocab:FixedCostSymbol/Tap")],
            "{T}: This creature deals 2 damage to any target and 3 damage to you.",
        ),
        (
            "Orcish Artillery",
            2,
            3,
            you(),
            vec![named_symbol("vocab:FixedCostSymbol/Tap")],
            "{T}: This creature deals 2 damage to any target and 3 damage to you.",
        ),
        (
            "Orcish Cannoneers",
            2,
            3,
            you(),
            vec![named_symbol("vocab:FixedCostSymbol/Tap")],
            "{T}: This creature deals 2 damage to any target and 3 damage to you.",
        ),
        (
            "Psionic Entity",
            2,
            3,
            itself(),
            vec![named_symbol("vocab:FixedCostSymbol/Tap")],
            "{T}: This creature deals 2 damage to any target and 3 damage to itself.",
        ),
        (
            "Reckless Embermage",
            1,
            1,
            itself(),
            vec![numeric_symbol(1), named_symbol("vocab:FixedCostSymbol/Red")],
            "{1}{R}: This creature deals 1 damage to any target and 1 damage to itself.",
        ),
    ] {
        let expected = document(
            activated(
                cost(symbol_cost(symbols), vec![]),
                sentence(this("creature", true), clusters(first, second, recipient)),
            ),
            vec![],
        );
        bounded_roundtrip(text, expected);
    }
    let mut forge = clusters(1, 1, you());
    let Reading::SelectedComplementClustersPredicate { tail, .. } = &mut forge else {
        unreachable!()
    };
    let Reading::Coordination { left, .. } = tail.as_mut() else { unreachable!() };
    let Reading::ObjectPrepositionTail { complement, .. } = left.as_mut() else {
        unreachable!()
    };
    **complement = acc(Reading::TargetNounPhrase {
        form: 0,
        marker: invariant("vocab:TargetingMarker/Target"),
        head: Box::new(noun("lexeme:type/creature")),
    });
    bounded_roundtrip(
        "When this creature enters, it deals 1 damage to target creature and 1 damage to you.",
        document(ordinary(enter_trigger("creature", forge)), vec![]),
    );

    let sacrifice = Reading::ActionCost {
        form: 0,
        action: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: initial(verb("lexeme:keyword_action/sacrifice", WordForm::Plain, 0)),
                complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                    Box::new(acc(this("artifact", false))),
                )],
            }),
        }),
    };
    let mut gain = verb("core-verb:Gain", WordForm::Present, 0);
    let LexicalReading::Word(value) = &mut gain.value else { unreachable!() };
    value.features.person = Some(Person::Second);
    let expected = document(
        ordinary(enter_trigger("artifact", clusters(4, 3, you()))),
        vec![activated(
            cost(
                symbol_cost(vec![numeric_symbol(2)]),
                vec![
                    symbol_cost(vec![named_symbol("vocab:FixedCostSymbol/Tap")]),
                    sacrifice,
                ],
            ),
            sentence(
                Reading::NominativePronoun {
                    form: 0,
                    head: pronoun(
                        "vocab:SubjectPronoun/You",
                        Case::Nominative,
                        Person::Second,
                        true,
                    ),
                },
                Reading::SelectedPredicate {
                    category: Category::FinitePredicate,
                    form: 0,
                    head: gain,
                    complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                        Box::new(acc(Reading::MeasuredNounPhrase {
                            form: 0,
                            quantity: Box::new(Reading::UngroupedScalarNumeral {
                                form: 0,
                                head: numeral(3),
                            }),
                            head: noun_word("lexeme:CommonNoun/Life", false),
                        })),
                    )],
                },
            ),
        )],
    );
    bounded_roundtrip(
        "When this artifact enters, it deals 4 damage to any target and 3 damage to you.\n{2}, {T}, Sacrifice this artifact: You gain 3 life.",
        expected,
    );
}
