use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

constructions! {
    mod selected {
        category Atom();
        category Sentence();
        separator prefix(number) { (Singular) => "", (Plural) => "\"" }
        separator suffix(number, number) {
            (Singular, Singular) => "", (Plural, Plural) => ".\"",
        }
        construction Atom: Atom {
            form [prefix(word.number), word: lexical(Noun), suffix(word.number, word.number)];
        }
        separator terminal(terminal_punctuation) {
            (None) => ".", (QuotedFullStop) => "",
        }
        construction Sentence: Sentence { form [atom: Atom, terminal(atom.terminal_punctuation)]; }
        separator singular(number) { (Singular) => "" }
        construction Singular: Atom {
            form [word: lexical(Noun), singular(word.number)];
        }
    }
}

fn noun(number: Number) -> selected::Word {
    selected::Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "lexeme:CommonNoun/Card".into(),
            form: if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            capitalization: SurfaceCase::Declared,
            variant: 0,
        }),
        countability: Some(true),
        frame: None,
    }
}

#[test]
fn selected_literals_scan_realize_and_admit_with_one_form_index() {
    let lexicon = common::lexicon();
    let grammar = selected::Grammar::default();
    for (number, text) in [(Number::Singular, "card"), (Number::Plural, "\"cards.\"")] {
        let expected = selected::Reading::Atom {
            form: 0,
            word: noun(number),
        };
        expected.admit_with(&grammar, lexicon).unwrap();
        assert_eq!(expected.realize_with(&grammar, lexicon).unwrap(), text);
        let forest = parse(
            &grammar,
            lexicon,
            &lexicon.analyze(text),
            &selected::Category::Atom,
        )
        .unwrap();
        let values: Vec<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(values.iter().filter(|value| **value == expected).count(), 1);
        let sentence = selected::Reading::Sentence {
            form: 0,
            atom: Box::new(expected),
        };
        let sentence_text = if number == Number::Singular { "card." } else { text };
        assert_eq!(
            sentence.realize_with(&grammar, lexicon).unwrap(),
            sentence_text
        );
        let forest = parse(
            &grammar,
            lexicon,
            &lexicon.analyze(sentence_text),
            &selected::Category::Sentence,
        )
        .unwrap();
        assert!(
            grammar
                .readings(&forest)
                .map(Result::unwrap)
                .any(|value| value == sentence)
        );
    }
    for text in ["card.\"", "\"card.\"", "cards", "\"cards"] {
        let forest = parse(
            &grammar,
            lexicon,
            &lexicon.analyze(text),
            &selected::Category::Atom,
        )
        .unwrap();
        assert_eq!(grammar.readings(&forest).count(), 0, "{text}");
    }
    let missing = selected::Reading::Singular {
        form: 0,
        word: noun(Number::Plural),
    };
    assert!(matches!(
        missing.admit_with(&grammar, lexicon),
        Err(selected::Error::Invalid(_, _))
    ));
    assert!(matches!(
        missing.realize_with(&grammar, lexicon),
        Err(selected::Error::Invalid(_, _))
    ));
}

#[test]
fn coordination_and_sentence_punctuation_have_no_independent_form_choice() {
    // Obsessive Stitcher and attested coordinated keyword qualities.
    for (text, category) in [
        (
            "Draw a card, then discard a card",
            Category::SecondaryVerbPhrase,
        ),
        ("red and blue", Category::AdjectivePhrase),
        ("Draw a card.", Category::Document),
        (
            "It has \"Sacrifice this token: Add {C}.\"",
            Category::Document,
        ),
    ] {
        let readings = common::readings(text, category);
        assert!(!readings.is_empty(), "{text}");
        for reading in &readings {
            let mut nodes = Vec::new();
            reading.visit(&mut |node| nodes.push(node.clone())).unwrap();
            for mut node in nodes {
                match &mut node {
                    Reading::Coordination { form, .. } | Reading::Sentence { form, .. } => {
                        assert_eq!(*form, 0);
                        *form = 1;
                        assert!(node.admit(common::lexicon()).is_err());
                    }
                    _ => {}
                }
            }
        }
    }
}

#[test]
fn all_keyword_separator_surfaces_derive_from_the_head() {
    // Rift Bolt, Kodama's Might, Rust Goliath and Goldmaw Champion.
    for text in [
        "Suspend 1—{R}",
        "Splice onto Arcane {G}",
        "Prototype {3}{G}{G} — 3/5",
        "Boast — {1}{W}: Tap target creature.",
    ] {
        let values = common::readings(text, Category::KeywordPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            assert_eq!(value.realize(common::lexicon()).unwrap(), text);
            value
                .visit(&mut |node| {
                    assert!(!node.construction().ends_with("KeywordSeparator"));
                })
                .unwrap();
        }
    }
}
