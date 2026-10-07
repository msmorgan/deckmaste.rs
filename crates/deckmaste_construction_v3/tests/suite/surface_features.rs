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
        category Sentence();
        category Quotation();
        category Matrix();
        category Tail();
        category Quotations();
        category TerminalMatrix();
        category QuotedStop();
        category TerminalWord();
        category PunctuationMatrix();
        construction PunctuationMatrix: PunctuationMatrix {
            form [body: TerminalWord];
            require body.terminal_punctuation = QuotedFullStop;
        }
        construction QuotedStop: QuotedStop { form [".\""]; }
        construction TerminalWord: TerminalWord {
            boundary Initial;
            form [word: Word, stop: QuotedStop];
            require stop.terminal_punctuation = QuotedFullStop;
        }
        construction Word: Word { form [head: lexical(Adjective)]; }
        construction Phrase: Phrase { form [first: Word, " ", rest: repeat(Word, " ")]; }
        construction Line: Line { boundary Initial; form [phrase: Phrase]; }
        construction Fragment: Fragment { boundary Interior; form ["\"", phrase: Phrase, ".\""]; }
        construction Bound: Bound { form [quality: lexical(Noun), suffix: lexical(Keyword)]; }
        construction BoundLine: BoundLine { boundary Initial; form [word: Bound]; }
        construction Sentence: Sentence { boundary Initial; form [word: Word, "."]; }
        construction Quotation: Quotation { form ["\"", sentence: Sentence, "\""]; }
        construction Tail: Tail { form [" ", word: Word]; }
        construction Quotations: Quotations { form [quotes: repeat(Quotation, " ")]; }
        construction TerminalMatrix: TerminalMatrix {
            form [body: Matrix];
            require body.terminal_punctuation = QuotedFullStop;
        }
        construction Matrix: Matrix {
            boundary Initial;
            form [head: Word, " ", quotations: Quotations, tail: optional(Tail)]
                require quotations.terminal_punctuation = QuotedFullStop;
        }
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

#[test]
fn terminal_punctuation_roundtrips_through_repetition_and_optional_empty_tails() {
    let lexicon = lexicon();
    let punctuation_only_child = fixture::Reading::PunctuationMatrix {
        form: 0,
        body: Box::new(fixture::Reading::TerminalWord {
            form: 0,
            word: Box::new(fixture::Reading::Word {
                form: 0,
                head: word("first", SurfaceCase::Initial),
            }),
            stop: Box::new(fixture::Reading::QuotedStop { form: 0 }),
        }),
    };
    assert_eq!(
        punctuation_only_child.realize(&lexicon).unwrap(),
        "White.\""
    );
    assert!(
        readings(&lexicon, "White.\"", fixture::Category::PunctuationMatrix)
            .contains(&punctuation_only_child)
    );

    let quotation = || fixture::Reading::Quotation {
        form: 0,
        sentence: Box::new(fixture::Reading::Sentence {
            form: 0,
            word: Box::new(fixture::Reading::Word {
                form: 0,
                head: word("second", SurfaceCase::Initial),
            }),
        }),
    };
    let value = fixture::Reading::Matrix {
        form: 0,
        head: Box::new(fixture::Reading::Word {
            form: 0,
            head: word("first", SurfaceCase::Initial),
        }),
        quotations: Box::new(fixture::Reading::Quotations {
            form: 0,
            quotes: vec![quotation(), quotation()],
        }),
        tail: None,
    };
    let text = "White \"Blue.\" \"Blue.\"";
    value.admit(&lexicon).unwrap();
    assert_eq!(value.realize(&lexicon).unwrap(), text);
    assert!(readings(&lexicon, text, fixture::Category::Matrix).contains(&value));
    let mut terminal = fixture::Reading::TerminalMatrix {
        form: 0,
        body: Box::new(value),
    };
    assert_eq!(terminal.realize(&lexicon).unwrap(), text);
    assert!(readings(&lexicon, text, fixture::Category::TerminalMatrix).contains(&terminal));
    let fixture::Reading::TerminalMatrix { body, .. } = &mut terminal else {
        unreachable!()
    };
    let fixture::Reading::Matrix { tail, .. } = body.as_mut() else {
        unreachable!()
    };
    *tail = Some(Box::new(fixture::Reading::Tail {
        form: 0,
        word: Box::new(fixture::Reading::Word {
            form: 0,
            head: word("second", SurfaceCase::Declared),
        }),
    }));
    assert!(terminal.admit(&lexicon).is_err());
    assert!(
        readings(
            &lexicon,
            "White \"Blue.\" blue",
            fixture::Category::TerminalMatrix
        )
        .is_empty()
    );
}
