use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
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
use deckmaste_lexical::Relation;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

constructions! {
    pub mod frame {
        category Head();
        frame Selected = r#"(
            kind: "selected",
            items: [
                Marker(vocabulary: "marker", member: "first"),
                Marked(vocabulary: "marker", member: "second", slot: (relation: Complement, category: "Phrase")),
                Optional(Argument((relation: Object, category: "Nominal"))),
                Literal("legacy signature"),
            ],
        )"#;
        construction Head: Head {
            form [word: lexical(Verb)];
            require word.form = Plain;
            require word.frame = Selected;
        }
    }
}

#[test]
fn complete_frame_signatures_and_selected_indices_survive_compilation() {
    let signature = Frame {
        kind: "selected".into(),
        items: vec![
            FrameItem::Marker {
                vocabulary: "marker".into(),
                member: "first".into(),
            },
            FrameItem::Marked {
                vocabulary: "marker".into(),
                member: "second".into(),
                slot: FrameSlot {
                    relation: Relation::Complement,
                    category: "Phrase".into(),
                },
            },
            FrameItem::Optional(Box::new(FrameItem::Argument(FrameSlot {
                relation: Relation::Object,
                category: "Nominal".into(),
            }))),
            FrameItem::Literal("legacy signature".into()),
        ],
    };
    let mut other = signature.clone();
    other.items.swap(0, 1);
    let mut lexeme = Lexeme::verb(
        "verb",
        "act",
        Source {
            kind: SourceKind::Core,
            path: "fixture".into(),
            owner: "verb".into(),
        },
    );
    lexeme.properties.frames = vec![other, signature.clone(), signature];
    let lexicon = Lexicon::new([lexeme]).unwrap();
    let grammar = frame::Grammar::default();
    let expected = frame::Reading::Head {
        form: 0,
        word: frame::Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "verb".into(),
                form: WordForm::Plain,
                features: FeatureBundle {
                    finiteness: Some(Finiteness::Nonfinite),
                    ..FeatureBundle::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: Some(1),
            countability: None,
        },
    };
    assert_eq!(expected.realize(&lexicon).unwrap(), "act");
    let forest = parse(
        &grammar,
        &lexicon,
        &lexicon.analyze("act"),
        &frame::Category::Head,
    )
    .unwrap();
    let readings: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
    assert_eq!(readings, BTreeSet::from([expected.clone()]));
    for index in [0, 2, 3] {
        let mut invalid = expected.clone();
        let frame::Reading::Head { word, .. } = &mut invalid;
        word.frame = Some(index);
        assert!(invalid.realize(&lexicon).is_err());
    }
}
