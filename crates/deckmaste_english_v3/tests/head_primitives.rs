mod common;

use std::collections::BTreeSet;

use common::readings_with_lexicon as readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn slot(relation: Relation, category: &str) -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation,
        category: category.into(),
    })
}

fn signatures() -> Vec<Frame> {
    vec![
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Object, "NounPhrase")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "PredicativeComplement")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "LocativeComplement")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "ManaPhrase")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "Amount")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "MeasurePhrase")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "PowerToughnessAdjustment")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "GrantedAbility")],
        },
        Frame {
            kind: "Auxiliary".into(),
            items: vec![slot(Relation::Complement, "BarePredicate")],
        },
        Frame {
            kind: "Auxiliary".into(),
            items: vec![slot(Relation::Complement, "ParticipialPredicate")],
        },
        Frame {
            kind: "Auxiliary".into(),
            items: vec![slot(Relation::Complement, "PastParticiplePredicate")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![
                slot(Relation::Object, "NounPhrase"),
                slot(Relation::Complement, "Name"),
            ],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "Cardinal")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Complement, "InfinitiveComplement")],
        },
    ]
}

fn lexicon_with_signatures(frames: Vec<Frame>) -> Lexicon {
    let mut head = Lexeme::verb(
        "fixture:head",
        "act",
        Source {
            kind: SourceKind::Core,
            path: "independent head fixture".into(),
            owner: "fixture:head".into(),
        },
    );
    head.properties.frames = frames;
    let card = Lexeme::noun(
        "fixture:card",
        "card",
        vec![Countability::Count],
        Source {
            kind: SourceKind::Core,
            path: "independent object fixture".into(),
            owner: "fixture:card".into(),
        },
    );
    Lexicon::new([head, card]).unwrap()
}

fn lexicon() -> Lexicon {
    lexicon_with_signatures(signatures())
}

fn word(frame: usize, finite: bool) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "fixture:head".into(),
            form: if finite { WordForm::Present } else { WordForm::Plain },
            features: if finite {
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
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: Some(frame),
        countability: None,
    }
}

fn primitive(frame: usize, finite: bool) -> Reading {
    assert!(frame < signatures().len(), "unknown fixture signature");
    Reading::SelectedVerbHead {
        category: if finite {
            Category::FiniteSelectedHead
        } else {
            Category::SecondarySelectedHead
        },
        form: 0,
        head: word(frame, finite),
    }
}

fn cards() -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(Reading::BarePlural {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "fixture:card".into(),
                        form: WordForm::Plural,
                        features: FeatureBundle {
                            number: Some(Number::Plural),
                            ..Default::default()
                        },
                        variant: 0,
                        capitalization: SurfaceCase::Declared,
                    }),
                    frame: None,
                    countability: Some(true),
                },
            }),
        }),
    }
}

#[test]
fn independent_atomic_heads_preserve_every_exact_selected_signature() {
    let lexicon = lexicon();
    for (finite, text, category) in [
        (true, "acts", Category::FiniteSelectedHead),
        (false, "act", Category::SecondarySelectedHead),
    ] {
        let expected: BTreeSet<_> = (0..signatures().len())
            .map(|index| primitive(index, finite))
            .collect();
        for reading in &expected {
            assert_eq!(reading.realize(&lexicon).unwrap(), text);
        }
        assert_eq!(readings(&lexicon, text, category), expected);
    }
}

#[test]
fn independent_atomic_heads_reject_retired_ability_object_signatures() {
    for category in ["KeywordPhrase", "QuotedText"] {
        let lexicon = lexicon_with_signatures(vec![Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Object, category)],
        }]);
        for (finite, text, category) in [
            (true, "acts", Category::FiniteSelectedHead),
            (false, "act", Category::SecondarySelectedHead),
        ] {
            assert!(primitive(0, finite).admit(&lexicon).is_err());
            assert_eq!(readings(&lexicon, text, category), BTreeSet::new());
        }
    }
}

#[test]
fn independent_atomic_heads_reject_wrong_signature_index_and_morphology() {
    let lexicon = lexicon();
    // Selection now belongs to the consuming host, rather than a head variant
    // name. Preserve the exact predicative-frame witness against an
    // object-selecting host.
    let wrong_head = primitive(1, false);
    assert_eq!(
        wrong_head,
        Reading::SelectedVerbHead {
            category: Category::SecondarySelectedHead,
            form: 0,
            head: word(1, false),
        }
    );
    assert!(wrong_head.admit(&lexicon).is_ok());
    let invalid = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word(1, false),
        complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
            Box::new(cards()),
        )],
    };
    assert!(invalid.admit(&lexicon).is_err());
    let invalid = Reading::SelectedVerbHead {
        category: Category::FiniteSelectedHead,
        form: 0,
        head: word(0, false),
    };
    assert!(invalid.admit(&lexicon).is_err());
    let invalid = Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: word(0, true),
    };
    assert!(invalid.admit(&lexicon).is_err());
    let invalid = Reading::SelectedVerbHead {
        category: Category::FiniteSelectedHead,
        form: 0,
        head: word(signatures().len(), true),
    };
    assert!(invalid.admit(&lexicon).is_err());
    assert_eq!(
        readings(&lexicon, "acts", Category::SecondarySelectedHead).len(),
        0
    );
    assert_eq!(
        readings(&lexicon, "acting", Category::FiniteSelectedHead).len(),
        0
    );
}
