mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn verb(owner: &str, frame: usize, form: WordForm) -> Word {
    let mut head = word(
        owner,
        form,
        if matches!(form, WordForm::GerundParticiple | WordForm::PastParticiple) {
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..Default::default()
            }
        } else if form == WordForm::Present {
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                case: None,
            }
        } else {
            FeatureBundle::default()
        },
    );
    head.frame = Some(frame);
    head
}

fn and() -> Word {
    word(
        "vocab:Coordinator/And",
        WordForm::Invariant,
        FeatureBundle::default(),
    )
}

fn object_head(owner: &str) -> Reading {
    Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: verb(owner, 0, WordForm::Plain),
    }
}

fn coordinate(left: Reading, right: Reading) -> Reading {
    Reading::Coordination {
        category: Category::SecondarySelectedHead,
        form: 0,
        left: Box::new(left),
        coordinator: and(),
        right: Box::new(right),
    }
}

fn cards() -> Reading {
    let mut head = word(
        "lexeme:CommonNoun/Card",
        WordForm::Plural,
        FeatureBundle {
            number: Some(Number::Plural),
            ..Default::default()
        },
    );
    head.countability = Some(true);
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(Reading::BarePlural {
            form: 0,
            head: Box::new(Reading::Noun { form: 0, head }),
        }),
    }
}

fn legendary() -> Reading {
    let owner = LEXICON
        .lexemes()
        .values()
        .find(|lexeme| {
            lexeme.category == deckmaste_lexical::Category::Adjective && lexeme.lemma == "legendary"
        })
        .unwrap()
        .id;
    Reading::AdjectivalComplement {
        form: 0,
        phrase: Box::new(Reading::Adjective {
            form: 0,
            head: word(&owner, WordForm::Invariant, FeatureBundle::default()),
        }),
    }
}

fn roundtrip(expected: &Reading, text: &str, category: Category) {
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    assert!(
        readings(text, category).contains(expected),
        "missing independent value for {text}"
    );
}

#[test]
fn independent_selected_head_coordination_preserves_flat_arity_and_layered_scope() {
    let draw = object_head("core-verb:Draw");
    let discard = object_head("lexeme:keyword_action/discard");
    let exile = object_head("lexeme:keyword_action/exile");
    let serial = Reading::SerialCoordination {
        category: Category::SecondarySelectedHead,
        form: 0,
        left: Box::new(draw.clone()),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::SecondarySelectedHeadSeries,
            form: 0,
            left: Box::new(discard.clone()),
            coordinator: and(),
            right: Box::new(exile.clone()),
        }),
    };
    roundtrip(
        &serial,
        "draw, discard, and exile",
        Category::SecondarySelectedHead,
    );
    assert_eq!(
        readings("draw, discard, and exile", Category::SecondarySelectedHead),
        BTreeSet::from([serial])
    );
    let left = coordinate(coordinate(draw.clone(), discard.clone()), exile.clone());
    let right = coordinate(draw, coordinate(discard, exile));
    for expected in [&left, &right] {
        roundtrip(
            expected,
            "draw and discard and exile",
            Category::SecondarySelectedHead,
        );
    }
    assert_eq!(
        readings(
            "draw and discard and exile",
            Category::SecondarySelectedHead
        ),
        BTreeSet::from([left, right])
    );
    assert_eq!(
        readings("draw, discard and exile", Category::SecondarySelectedHead).len(),
        0
    );
    assert_eq!(
        readings("draw, and discard", Category::SecondarySelectedHead).len(),
        0
    );
}

#[test]
fn independent_shared_objects_and_predicatives_keep_selected_functions() {
    let head = coordinate(
        object_head("core-verb:Draw"),
        object_head("lexeme:keyword_action/discard"),
    );
    let expected = Reading::SharedObjectComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(head),
        object: Box::new(cards()),
    };
    roundtrip(
        &expected,
        "draw and discard cards",
        Category::SecondaryVerbPhrase,
    );
    let expected = Reading::SharedPredicativeComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(coordinate(
            Reading::SelectedVerbHead {
                category: Category::SecondarySelectedHead,
                form: 0,
                head: verb("core-verb:Be", 0, WordForm::Plain),
            },
            Reading::SelectedVerbHead {
                category: Category::SecondarySelectedHead,
                form: 0,
                head: verb("core-verb:Become", 0, WordForm::Plain),
            },
        )),
        complement: Box::new(legendary()),
    };
    roundtrip(
        &expected,
        "be and become legendary",
        Category::SecondaryVerbPhrase,
    );
    let invalid = Reading::SharedObjectComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(object_head("core-verb:Draw")),
        object: Box::new(cards()),
    };
    assert!(
        invalid.admit(&LEXICON).is_err(),
        "ordinary noncoordinated head is not a shared-head reading"
    );
    let mixed = coordinate(
        object_head("core-verb:Draw"),
        Reading::SelectedVerbHead {
            category: Category::SecondarySelectedHead,
            form: 0,
            head: verb("core-verb:Become", 0, WordForm::Plain),
        },
    );
    assert!(mixed.admit(&LEXICON).is_err());
    assert_eq!(
        readings("become and draw cards", Category::SecondaryVerbPhrase).len(),
        0
    );
}

#[test]
fn independent_auxiliary_sharing_preserves_complement_form_and_voice() {
    let predicate = Reading::BarePredicate {
        form: 0,
        head: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: verb("core-verb:Draw", 0, WordForm::Plain),
            complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                Box::new(cards()),
            )],
        }),
    };
    let expected = Reading::SharedAuxiliaryBareComplement {
        category: Category::FinitePredicate,
        form: 0,
        head: Box::new(Reading::Coordination {
            category: Category::FiniteSelectedHead,
            form: 0,
            left: Box::new(Reading::SelectedVerbHead {
                category: Category::FiniteSelectedHead,
                form: 0,
                head: verb("core-verb:May", 0, WordForm::Present),
            }),
            coordinator: and(),
            right: Box::new(Reading::SelectedVerbHead {
                category: Category::FiniteSelectedHead,
                form: 0,
                head: verb("core-verb:Can", 0, WordForm::Present),
            }),
        }),
        complement: Box::new(Reading::OvertComplement {
            category: Category::BareComplement,
            form: 0,
            predicate: Box::new(predicate),
        }),
    };
    roundtrip(
        &expected,
        "may and can draw cards",
        Category::FinitePredicate,
    );
    let passive = Reading::PassiveComplement {
        form: 0,
        head: Box::new(Reading::PassivePredicate {
            form: 0,
            head: verb("core-verb:Draw", 0, WordForm::PastParticiple),
        }),
    };
    let head = Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: verb("core-verb:Be", 1, WordForm::Plain),
    };
    let expected = Reading::SharedAuxiliaryParticipleComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(coordinate(head.clone(), head)),
        complement: Box::new(Reading::OvertComplement {
            category: Category::ParticipialComplement,
            form: 0,
            predicate: Box::new(passive),
        }),
    };
    roundtrip(&expected, "be and be drawn", Category::SecondaryVerbPhrase);
    assert_eq!(
        readings("may and can drawing cards", Category::FinitePredicate).len(),
        0
    );
    assert_eq!(
        readings("be and may drawn", Category::SecondaryVerbPhrase).len(),
        0
    );
}

#[test]
fn selected_heads_reject_crossed_agreement_and_secondary_forms() {
    let left = Reading::SelectedVerbHead {
        category: Category::FiniteSelectedHead,
        form: 0,
        head: verb("core-verb:Draw", 0, WordForm::Present),
    };
    let mut right = verb("core-verb:Gain", 0, WordForm::Present);
    if let LexicalReading::Word(value) = &mut right.value {
        value.features.number = Some(Number::Plural);
    }
    let invalid = Reading::Coordination {
        category: Category::FiniteSelectedHead,
        form: 0,
        left: Box::new(left),
        coordinator: and(),
        right: Box::new(Reading::SelectedVerbHead {
            category: Category::FiniteSelectedHead,
            form: 0,
            head: right,
        }),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    let invalid = coordinate(
        object_head("core-verb:Draw"),
        Reading::SelectedVerbHead {
            category: Category::SecondarySelectedHead,
            form: 0,
            head: verb("core-verb:Draw", 0, WordForm::GerundParticiple),
        },
    );
    assert!(invalid.admit(&LEXICON).is_err());
    assert_eq!(
        readings("draw and drawing", Category::SecondarySelectedHead).len(),
        0
    );
    assert_eq!(
        readings("draws and discard", Category::FiniteSelectedHead).len(),
        0
    );
}

fn intransitive(owner: &str, finite: bool) -> Reading {
    let head = verb(
        owner,
        0,
        if finite { WordForm::Present } else { WordForm::Plain },
    );
    if finite {
        Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head,
            complements: vec![],
        }
    } else {
        Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head,
            complements: vec![],
        }
    }
}

fn imperative(owner: &str) -> Reading {
    Reading::Imperative {
        form: 0,
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(intransitive(owner, false)),
        }),
    }
}

#[test]
fn independently_constructed_ordinary_serial_values_require_oxford_comma() {
    let expected = Reading::SerialCoordination {
        category: Category::FinitePredicate,
        form: 0,
        left: Box::new(intransitive("core-verb:Attack", true)),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::FinitePredicateSeries,
            form: 0,
            left: Box::new(intransitive("core-verb:Block", true)),
            coordinator: and(),
            right: Box::new(intransitive("core-verb:Enter", true)),
        }),
    };
    roundtrip(
        &expected,
        "attacks, blocks, and enters",
        Category::FinitePredicate,
    );
    let expected = Reading::SerialCoordination {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        left: Box::new(intransitive("core-verb:Attack", false)),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::SecondaryPredicateSeries,
            form: 0,
            left: Box::new(intransitive("core-verb:Block", false)),
            coordinator: and(),
            right: Box::new(intransitive("core-verb:Enter", false)),
        }),
    };
    roundtrip(
        &expected,
        "attack, block, and enter",
        Category::SecondaryVerbPhrase,
    );
    let expected = Reading::SerialCoordination {
        category: Category::Clause,
        form: 0,
        left: Box::new(imperative("core-verb:Attack")),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::ClauseSeries,
            form: 0,
            left: Box::new(imperative("core-verb:Block")),
            coordinator: and(),
            right: Box::new(imperative("core-verb:Enter")),
        }),
    };
    roundtrip(&expected, "attack, block, and enter", Category::Clause);
    let Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head,
        ..
    } = cards()
    else {
        unreachable!()
    };
    let expected = Reading::SerialCoordination {
        category: Category::NounPhrase,
        form: 0,
        left: head.clone(),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::NounPhraseSeries,
            form: 0,
            left: head.clone(),
            coordinator: and(),
            right: head,
        }),
    };
    roundtrip(&expected, "cards, cards, and cards", Category::NounPhrase);
    for (text, category) in [
        ("attacks, blocks and enters", Category::FinitePredicate),
        ("attack, block and enter", Category::SecondaryVerbPhrase),
        ("attack, block and enter", Category::Clause),
        ("cards, cards and cards", Category::NounPhrase),
    ] {
        assert_eq!(readings(text, category).len(), 0, "{text}");
    }
}

fn selected_complement_head(owner: &str, category: &str) -> Word {
    let index = LEXICON.lexemes()[owner].properties.frames.iter().position(|frame| {
        frame.kind == "Predicate" && matches!(frame.items.as_slice(), [deckmaste_lexical::FrameItem::Argument(slot)] if slot.relation == deckmaste_lexical::Relation::Complement && slot.category == category)
    }).unwrap();
    verb(owner, index, WordForm::Plain)
}

#[test]
fn independent_shared_cardinal_and_infinitive_complements_preserve_selected_categories() {
    let head = Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: selected_complement_head("core-verb:Choose", "Cardinal"),
    };
    let expected = Reading::SharedCardinalComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(coordinate(head.clone(), head)),
        complement: Box::new(Reading::Cardinal {
            form: 0,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 1,
                    notation: deckmaste_lexical::Numeral::Cardinal,
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
    };
    roundtrip(
        &expected,
        "choose and choose one",
        Category::SecondaryVerbPhrase,
    );
    let head = Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: selected_complement_head("core-verb:Choose", "InfinitiveComplement"),
    };
    let expected = Reading::SharedInfinitiveComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(coordinate(head.clone(), head)),
        complement: Box::new(Reading::ToInfinitive {
            form: 0,
            marker: word(
                "vocab:InfinitivalMarker/To",
                WordForm::Invariant,
                FeatureBundle::default(),
            ),
            predicate: Box::new(Reading::BarePredicate {
                form: 0,
                head: Box::new(intransitive("core-verb:Attack", false)),
            }),
        }),
    };
    roundtrip(
        &expected,
        "choose and choose to attack",
        Category::SecondaryVerbPhrase,
    );
    let invalid = Reading::SharedCardinalComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(Reading::SelectedVerbHead {
            category: Category::SecondarySelectedHead,
            form: 0,
            head: selected_complement_head("core-verb:Choose", "InfinitiveComplement"),
        }),
        complement: Box::new(cards()),
    };
    assert!(invalid.admit(&LEXICON).is_err());
}
