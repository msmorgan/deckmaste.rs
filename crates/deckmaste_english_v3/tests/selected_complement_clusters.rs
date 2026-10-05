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
                modifier: Box::new(Reading::Adjective {
                    form: 0,
                    head: invariant("vocab:AttributiveAdjective/Other"),
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
        head: verb("core-verb:Deal", WordForm::Present, 5),
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
        head: verb("core-verb:Deal", WordForm::Present, 5),
        tail: Box::new(tail(3, any_target(false))),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    assert!(invalid.realize(&LEXICON).is_err());
    // Lightning Bolt retains its ordinary, uncoordinated selected-frame AST.
    let ordinary = Reading::SelectedObjectPrepositionPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: verb("core-verb:Deal", WordForm::Present, 5),
        object: Box::new(damage(3)),
        marker: invariant("vocab:Preposition/To"),
        complement: Box::new(any_target(false)),
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
        head: verb("core-verb:Deal", WordForm::PastParticiple, 5),
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
        head: verb("core-verb:Deal", WordForm::PastParticiple, 4),
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
