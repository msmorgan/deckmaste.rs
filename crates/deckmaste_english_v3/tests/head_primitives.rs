use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
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
            items: vec![slot(Relation::Object, "KeywordPhrase")],
        },
        Frame {
            kind: "Predicate".into(),
            items: vec![slot(Relation::Object, "QuotedText")],
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
            items: vec![
                slot(Relation::Object, "NounPhrase"),
                slot(Relation::Complement, "ScalarEquality"),
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

fn lexicon() -> Lexicon {
    let mut head = Lexeme::verb(
        "fixture:head",
        "act",
        Source {
            kind: SourceKind::Core,
            path: "independent head fixture".into(),
            owner: "fixture:head".into(),
        },
    );
    head.properties.frames = signatures();
    Lexicon::new([head]).unwrap()
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
    let head = word(frame, finite);
    match (frame, finite) {
        (0, true) => Reading::FiniteSelectedObjectHead { form: 0, head },
        (0, false) => Reading::SecondarySelectedObjectHead { form: 0, head },
        (1, true) => Reading::FiniteSelectedPredicativeHead { form: 0, head },
        (1, false) => Reading::SecondarySelectedPredicativeHead { form: 0, head },
        (2, true) => Reading::FiniteSelectedLocativeHead { form: 0, head },
        (2, false) => Reading::SecondarySelectedLocativeHead { form: 0, head },
        (3, true) => Reading::FiniteSelectedManaHead { form: 0, head },
        (3, false) => Reading::SecondarySelectedManaHead { form: 0, head },
        (4, true) => Reading::FiniteSelectedAmountHead { form: 0, head },
        (4, false) => Reading::SecondarySelectedAmountHead { form: 0, head },
        (5, true) => Reading::FiniteSelectedMeasureHead { form: 0, head },
        (5, false) => Reading::SecondarySelectedMeasureHead { form: 0, head },
        (6, true) => Reading::FiniteSelectedSlashMeasureHead { form: 0, head },
        (6, false) => Reading::SecondarySelectedSlashMeasureHead { form: 0, head },
        (7, true) => Reading::FiniteSelectedKeywordHead { form: 0, head },
        (7, false) => Reading::SecondarySelectedKeywordHead { form: 0, head },
        (8, true) => Reading::FiniteSelectedQuotedHead { form: 0, head },
        (8, false) => Reading::SecondarySelectedQuotedHead { form: 0, head },
        (9, true) => Reading::FiniteSelectedAuxiliaryBareHead { form: 0, head },
        (9, false) => Reading::SecondarySelectedAuxiliaryBareHead { form: 0, head },
        (10, true) => Reading::FiniteSelectedAuxiliaryParticipleHead { form: 0, head },
        (10, false) => Reading::SecondarySelectedAuxiliaryParticipleHead { form: 0, head },
        (11, true) => Reading::FiniteSelectedAuxiliaryPerfectHead { form: 0, head },
        (11, false) => Reading::SecondarySelectedAuxiliaryPerfectHead { form: 0, head },
        (12, true) => Reading::FiniteSelectedObjectNameHead { form: 0, head },
        (12, false) => Reading::SecondarySelectedObjectNameHead { form: 0, head },
        (13, true) => Reading::FiniteSelectedObjectEqualityHead { form: 0, head },
        (13, false) => Reading::SecondarySelectedObjectEqualityHead { form: 0, head },

        (14, true) => Reading::FiniteSelectedCardinalHead { form: 0, head },
        (14, false) => Reading::SecondarySelectedCardinalHead { form: 0, head },
        (15, true) => Reading::FiniteSelectedInfinitiveHead { form: 0, head },
        (15, false) => Reading::SecondarySelectedInfinitiveHead { form: 0, head },
        _ => panic!("unknown fixture signature"),
    }
}

fn readings(lexicon: &Lexicon, text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &analyzed, &category).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}

#[test]
fn independent_atomic_heads_preserve_every_exact_selected_signature() {
    let lexicon = lexicon();
    for (finite, text, category) in [
        (true, "acts", Category::FiniteSelectedHead),
        (false, "act", Category::SecondarySelectedHead),
    ] {
        let expected: BTreeSet<_> = (0..16).map(|index| primitive(index, finite)).collect();
        for reading in &expected {
            assert_eq!(reading.realize(&lexicon).unwrap(), text);
        }
        assert_eq!(readings(&lexicon, text, category), expected);
    }
}

#[test]
fn independent_atomic_heads_reject_wrong_signature_index_and_morphology() {
    let lexicon = lexicon();
    let invalid = Reading::SecondarySelectedObjectHead {
        form: 0,
        head: word(1, false),
    };
    assert!(invalid.admit(&lexicon).is_err());
    let invalid = Reading::FiniteSelectedObjectHead {
        form: 0,
        head: word(0, false),
    };
    assert!(invalid.admit(&lexicon).is_err());
    let invalid = Reading::SecondarySelectedObjectHead {
        form: 0,
        head: word(0, true),
    };
    assert!(invalid.admit(&lexicon).is_err());
    let invalid = Reading::FiniteSelectedObjectHead {
        form: 0,
        head: word(16, true),
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
