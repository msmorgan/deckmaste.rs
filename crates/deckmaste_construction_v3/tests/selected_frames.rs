use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_lexical::{
    Category as Pos, Frame, FrameItem, FrameSlot, Lexeme, LexicalReading, Lexicon, Numeral,
    Relation, Source, SourceKind, SurfaceCase, SurfaceStructure, WordForm,
};

constructions! {
    pub mod grammar {
        category NounPhrase();
        category Amount();
        category Object();
        category Predicate(form);
        frame_category NounPhrase = Object;
        construction Noun: NounPhrase { form [head: lexical(Noun)]; }
        construction Determined: NounPhrase {
            form [determiner: lexical(Determinative), " ", head: lexical(Noun)];
        }
        construction Amount: Amount {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
        }
        construction Object: Object { form [head: NounPhrase]; }
        construction SelectedPredicate: Predicate {
            form [head: lexical(Verb), complements: selected_frame(head, Predicate)];
            require head.form = Plain;
            export form = head.form;
        }
    }
}

fn source(owner: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "selected-frame-proof".into(),
        owner: owner.into(),
    }
}
fn argument(category: &str) -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation: Relation::Object,
        category: category.into(),
    })
}
fn marker(member: &str) -> FrameItem {
    FrameItem::Marker {
        vocabulary: "Preposition".into(),
        member: member.into(),
    }
}
fn predicate(items: Vec<FrameItem>) -> Frame {
    Frame {
        kind: "Predicate".into(),
        items,
    }
}
fn lexicon() -> Lexicon {
    let mut entries = vec![];
    for (id, spelling) in [
        ("Orcs", "Orcs"),
        ("creature-type", "creature type"),
        ("creature", "creature"),
        ("battlefield", "battlefield"),
        ("control", "control"),
    ] {
        let mut lexeme = Lexeme::invariant(id, spelling, Pos::Noun, source(id));
        if spelling.contains(' ') {
            lexeme.surface_structure = SurfaceStructure::Multiword;
        }
        entries.push(lexeme);
    }
    for id in ["a", "that", "the", "your"] {
        entries.push(Lexeme::invariant(id, id, Pos::Determinative, source(id)));
    }
    for (id, spelling, member) in [
        ("with", "with", "With"),
        ("under", "under", "Under"),
        ("to", "to", "To"),
        ("homograph-with", "with", "To"),
    ] {
        let mut lexeme = Lexeme::invariant(id, spelling, Pos::Preposition, source(id));
        lexeme
            .properties
            .features
            .insert("FrameMarker".into(), format!("Preposition/{member}"));
        entries.push(lexeme);
    }
    for (id, pos, reference) in [
        ("face", Pos::Noun, "CommonNoun/Face"),
        ("up", Pos::Adverb, "Adverb/Up"),
    ] {
        let mut lexeme = Lexeme::invariant(id, id, pos, source(id));
        lexeme
            .properties
            .features
            .insert("FrameMarker".into(), reference.into());
        entries.push(lexeme);
    }
    // Authored Amass NP+Amount, Share NP With NP and Enter Object [Under Object]
    // frames. Enter's legacy role category is declared explicitly above;
    // unrelated unreconciled roles remain unsupported below.
    for (id, frames) in [
        (
            "amass",
            vec![predicate(vec![argument("NounPhrase"), argument("Amount")])],
        ),
        (
            "share",
            vec![
                predicate(vec![argument("NounPhrase")]),
                predicate(vec![
                    argument("NounPhrase"),
                    marker("With"),
                    argument("NounPhrase"),
                ]),
            ],
        ),
        (
            "enter",
            vec![predicate(vec![
                argument("Object"),
                FrameItem::Optional(Box::new(FrameItem::Marked {
                    vocabulary: "Preposition".into(),
                    member: "Under".into(),
                    slot: FrameSlot {
                        relation: Relation::Object,
                        category: "Object".into(),
                    },
                })),
            ])],
        ),
        (
            "turn",
            vec![predicate(vec![
                FrameItem::Marker {
                    vocabulary: "CommonNoun".into(),
                    member: "Face".into(),
                },
                FrameItem::Marker {
                    vocabulary: "Adverb".into(),
                    member: "Up".into(),
                },
            ])],
        ),
        (
            "remove",
            vec![
                predicate(vec![argument("FrameComplement")]),
                predicate(vec![FrameItem::Literal("legacy".into())]),
            ],
        ),
    ] {
        let mut lexeme = Lexeme::verb(id, id, source(id));
        lexeme.properties.frames = frames;
        entries.push(lexeme);
    }
    Lexicon::new(entries).unwrap()
}
fn word(lexicon: &Lexicon, id: &str, form: WordForm, frame: Option<usize>) -> grammar::Word {
    grammar::Word {
        value: LexicalReading::Word(
            lexicon
                .values()
                .find(|value| {
                    value.lexeme == id
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame,
        countability: None,
    }
}
fn noun(lexicon: &Lexicon, id: &str) -> grammar::Reading {
    grammar::Reading::Noun {
        form: 0,
        head: word(lexicon, id, WordForm::Invariant, None),
    }
}
fn np(lexicon: &Lexicon, determiner: &str, id: &str) -> grammar::Reading {
    grammar::Reading::Determined {
        form: 0,
        determiner: word(lexicon, determiner, WordForm::Invariant, None),
        head: word(lexicon, id, WordForm::Invariant, None),
    }
}
fn object(reading: grammar::Reading) -> grammar::Reading {
    grammar::Reading::Object {
        form: 0,
        head: Box::new(reading),
    }
}
fn selected(
    lexicon: &Lexicon,
    id: &str,
    frame: usize,
    complements: Vec<grammar::FrameValue>,
) -> grammar::Reading {
    grammar::Reading::SelectedPredicate {
        form: 0,
        head: word(lexicon, id, WordForm::Plain, Some(frame)),
        complements,
    }
}
fn arg(reading: grammar::Reading) -> grammar::FrameValue {
    let reading = if reading.category() == grammar::Category::NounPhrase {
        object(reading)
    } else {
        reading
    };
    grammar::FrameValue::Argument(Box::new(reading))
}
fn amount() -> grammar::Reading {
    grammar::Reading::Amount {
        form: 0,
        head: grammar::Word {
            value: LexicalReading::Numeral {
                value: 1,
                notation: Numeral::Arabic(false),
                capitalization: SurfaceCase::Declared,
            },
            frame: None,
            countability: None,
        },
    }
}
fn parsed(environment: &grammar::GrammarEnvironment<'_>, text: &str) -> BTreeSet<grammar::Reading> {
    let forest = environment
        .parse(text, grammar::Category::Predicate)
        .unwrap();
    environment
        .grammar()
        .readings(&forest)
        .map(Result::unwrap)
        .collect()
}

#[test]
fn one_constructor_consumes_ordered_declared_frames_with_independent_roundtrip_laws() {
    let lexicon = lexicon();
    let environment = grammar::GrammarEnvironment::new(&lexicon);
    let cases = [
        // Dreadhorde Invasion's keyword constituent; Reins of the Vinesteed's
        // selected-with constituent; Enter's ordinary optional control phrase.
        (
            "turn face up",
            selected(
                &lexicon,
                "turn",
                0,
                vec![
                    grammar::FrameValue::Marker(word(&lexicon, "face", WordForm::Invariant, None)),
                    grammar::FrameValue::Marker(word(&lexicon, "up", WordForm::Invariant, None)),
                ],
            ),
        ),
        (
            "amass Orcs 1",
            selected(
                &lexicon,
                "amass",
                0,
                vec![arg(noun(&lexicon, "Orcs")), arg(amount())],
            ),
        ),
        (
            "share a creature type with that creature",
            selected(
                &lexicon,
                "share",
                1,
                vec![
                    arg(np(&lexicon, "a", "creature-type")),
                    grammar::FrameValue::Marker(word(&lexicon, "with", WordForm::Invariant, None)),
                    arg(np(&lexicon, "that", "creature")),
                ],
            ),
        ),
        (
            "enter the battlefield",
            selected(
                &lexicon,
                "enter",
                0,
                vec![
                    arg(object(np(&lexicon, "the", "battlefield"))),
                    grammar::FrameValue::Optional(None),
                ],
            ),
        ),
        (
            "enter the battlefield under your control",
            selected(
                &lexicon,
                "enter",
                0,
                vec![
                    arg(object(np(&lexicon, "the", "battlefield"))),
                    grammar::FrameValue::Optional(Some(Box::new(grammar::FrameValue::Marked {
                        marker: word(&lexicon, "under", WordForm::Invariant, None),
                        argument: Box::new(object(np(&lexicon, "your", "control"))),
                    }))),
                ],
            ),
        ),
    ];
    for (text, expected) in cases {
        let actual = parsed(&environment, text);
        assert_eq!(actual, BTreeSet::from([expected.clone()]), "{text}");
        assert!(environment.admit(&expected).is_ok());
        assert_eq!(environment.realize(&expected).unwrap(), text);
        for reading in actual {
            let surface = environment.realize(&reading).unwrap();
            assert!(parsed(&environment, &surface).contains(&reading));
        }
    }
    let unsupported = environment.unsupported_frames();
    assert_eq!(unsupported.len(), 2);
    assert!(
        unsupported
            .iter()
            .any(|frame| frame.reason.contains("FrameComplement"))
    );
    assert!(
        unsupported
            .iter()
            .any(|frame| frame.reason.contains("unreconciled legacy"))
    );
}

#[test]
fn frame_order_marker_ownership_and_local_choice_are_checked_before_materialization() {
    let lexicon = lexicon();
    let environment = grammar::GrammarEnvironment::new(&lexicon);
    for text in [
        "amass 1 Orcs",
        "amass Orcs",
        "amass Orcs 1 with that creature",
        "share a creature type to that creature",
        "share with that creature a creature type",
        "enter under your control the battlefield",
    ] {
        assert!(parsed(&environment, text).is_empty(), "{text}");
    }
    let make = |marker| {
        selected(
            &lexicon,
            "share",
            1,
            vec![
                arg(np(&lexicon, "a", "creature-type")),
                grammar::FrameValue::Marker(word(&lexicon, marker, WordForm::Invariant, None)),
                arg(np(&lexicon, "that", "creature")),
            ],
        )
    };
    let reading = make("with");
    assert_eq!(
        reading.admit(&lexicon).unwrap(),
        environment.admit(&reading).unwrap()
    );
    assert_eq!(
        reading.realize(&lexicon).unwrap(),
        environment.realize(&reading).unwrap()
    );
    // Identical surface spelling does not authorize a different declared class.
    assert!(environment.admit(&make("homograph-with")).is_err());
    let mut wrong_frame = reading.clone();
    if let grammar::Reading::SelectedPredicate { head, .. } = &mut wrong_frame {
        head.frame = Some(0);
    }
    assert!(environment.admit(&wrong_frame).is_err());
    let mut constructions = vec![];
    environment
        .visit(&reading, &mut |node| {
            constructions.push(node.construction());
        })
        .unwrap();
    assert_eq!(
        constructions,
        [
            "SelectedPredicate",
            "Object",
            "Determined",
            "Object",
            "Determined"
        ]
    );
    let mut words = vec![];
    environment
        .visit_words(&reading, &mut |word| words.push(word.clone()))
        .unwrap();
    assert_eq!(
        words,
        [
            word(&lexicon, "share", WordForm::Plain, Some(1)),
            word(&lexicon, "a", WordForm::Invariant, None),
            word(&lexicon, "creature-type", WordForm::Invariant, None),
            word(&lexicon, "with", WordForm::Invariant, None),
            word(&lexicon, "that", WordForm::Invariant, None),
            word(&lexicon, "creature", WordForm::Invariant, None)
        ]
    );
}

#[test]
fn identical_marker_classes_share_metadata_without_losing_lexical_owners() {
    let original = lexicon();
    let mut entries: Vec<_> = original.lexemes().values().cloned().collect();
    let mut alternative = Lexeme::invariant(
        "alternate-with",
        "with",
        Pos::Preposition,
        source("alternate-with"),
    );
    alternative
        .properties
        .features
        .insert("FrameMarker".into(), "Preposition/With".into());
    entries.push(alternative);
    let lexicon = Lexicon::new(entries).unwrap();
    let environment = grammar::GrammarEnvironment::new(&lexicon);
    let expected: BTreeSet<_> = ["with", "alternate-with"]
        .into_iter()
        .map(|owner| {
            selected(
                &lexicon,
                "share",
                1,
                vec![
                    arg(np(&lexicon, "a", "creature-type")),
                    grammar::FrameValue::Marker(word(&lexicon, owner, WordForm::Invariant, None)),
                    arg(np(&lexicon, "that", "creature")),
                ],
            )
        })
        .collect();
    let text = "share a creature type with that creature";
    assert_eq!(parsed(&environment, text), expected);
    for reading in expected {
        assert_eq!(environment.realize(&reading).unwrap(), text);
    }
}
