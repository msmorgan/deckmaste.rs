use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Binding;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

constructions! {
    pub mod fixture {
        capitalization Positional;
        category Word();
        category Phrase();
        category Line();
        category Fragment();
        category Bound();
        category BoundLine();
        construction Word: Word { form [head: lexical(Adjective)]; }
        construction Phrase: Phrase { form [first: Word, " ", rest: repeat(Word, " ")]; }
        construction Line: Line { boundary Initial; form [phrase: Phrase]; }
        construction Fragment: Fragment { boundary Interior; form ["\"", phrase: Phrase, ".\""]; }
        construction Bound: Bound { form [quality: lexical(Noun), suffix: lexical(Keyword)]; }
        construction BoundLine: BoundLine { boundary Initial; form [word: Bound]; }
    }
}

fn lexicon() -> Lexicon {
    let rows = [
        ("first", "white", Category::Adjective, Binding::Free),
        ("second", "blue", Category::Adjective, Binding::Free),
        ("quality", "island", Category::Noun, Binding::Free),
        ("suffix", "walk", Category::Keyword, Binding::Suffix),
    ]
    .map(|(id, text, category, binding)| {
        let mut row = Lexeme::invariant(
            id,
            text,
            category,
            Source {
                kind: SourceKind::Core,
                path: "surface-fixture".into(),
                owner: id.into(),
            },
        );
        if category == Category::Noun {
            row = Lexeme::noun(
                id,
                text,
                vec![deckmaste_lexical::Countability::Count],
                row.source,
            );
        }
        row.binding = binding;
        row
    });
    Lexicon::new(rows).unwrap()
}

fn word(id: &str, capitalization: SurfaceCase) -> fixture::Word {
    fixture::Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: if id == "quality" { WordForm::Singular } else { WordForm::Invariant },
            features: deckmaste_lexical::FeatureBundle {
                number: (id == "quality").then_some(deckmaste_lexical::Number::Singular),
                ..Default::default()
            },
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: (id == "quality").then_some(true),
    }
}

fn readings(lexicon: &Lexicon, text: &str, root: fixture::Category) -> BTreeSet<fixture::Reading> {
    let grammar = fixture::Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &root).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}

#[test]
fn independent_collections_and_bound_words_check_case_in_both_directions() {
    let lexicon = lexicon();
    for initial in [false, true] {
        for inner in [false, true] {
            let first = if initial { SurfaceCase::Initial } else { SurfaceCase::Declared };
            let rest = if inner { SurfaceCase::Initial } else { SurfaceCase::Declared };
            let phrase = fixture::Reading::Phrase {
                form: 0,
                first: Box::new(fixture::Reading::Word {
                    form: 0,
                    head: word("first", first),
                }),
                rest: vec![fixture::Reading::Word {
                    form: 0,
                    head: word("second", rest),
                }],
            };
            let text = format!(
                "{} {}",
                if initial { "White" } else { "white" },
                if inner { "Blue" } else { "blue" }
            );
            let line = fixture::Reading::Line {
                form: 0,
                phrase: Box::new(phrase.clone()),
            };
            let expected: BTreeSet<_> = if initial && !inner {
                assert_eq!(line.realize(&lexicon).unwrap(), text);
                [line].into()
            } else {
                assert!(line.admit(&lexicon).is_err());
                BTreeSet::new()
            };
            assert_eq!(readings(&lexicon, &text, fixture::Category::Line), expected);
            let fragment = fixture::Reading::Fragment {
                form: 0,
                phrase: Box::new(phrase),
            };
            let quoted = format!("\"{text}.\"");
            let expected: BTreeSet<_> = if !initial && !inner {
                assert_eq!(fragment.realize(&lexicon).unwrap(), quoted);
                [fragment].into()
            } else {
                assert!(fragment.admit(&lexicon).is_err());
                BTreeSet::new()
            };
            assert_eq!(
                readings(&lexicon, &quoted, fixture::Category::Fragment),
                expected
            );
            let line = fixture::Reading::BoundLine {
                form: 0,
                word: Box::new(fixture::Reading::Bound {
                    form: 0,
                    quality: word("quality", first),
                    suffix: word("suffix", rest),
                }),
            };
            let text = format!(
                "{}{}",
                if initial { "Island" } else { "island" },
                if inner { "Walk" } else { "walk" }
            );
            let found = readings(&lexicon, &text, fixture::Category::BoundLine);
            if initial && !inner {
                assert_eq!(line.realize(&lexicon).unwrap(), text);
                assert_eq!(found, [line].into());
            } else {
                assert!(line.admit(&lexicon).is_err());
                assert!(found.is_empty());
            }
        }
    }
}
