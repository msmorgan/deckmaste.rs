mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::ability_readings as readings;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn flavor(label: &str, body: Reading) -> Reading {
    Reading::FlavorWordHead {
        form: 0,
        head: Word {
            value: LexicalReading::FlavorWord {
                label: label.into(),
            },
            frame: None,
            countability: None,
        },
        body: Box::new(body),
    }
}

fn equip() -> Reading {
    Reading::KeywordLine {
        form: 0,
        first: Box::new(Reading::CostKeyword {
            form: 0,
            head: Word {
                value: LexicalReading::Word(LexicalValue {
                    lexeme: "lexeme:keyword_ability/equip".into(),
                    form: WordForm::Invariant,
                    features: FeatureBundle::default(),
                    variant: 0,
                    capitalization: SurfaceCase::Initial,
                }),
                frame: None,
                countability: None,
            },
            cost: Box::new(Reading::CostSymbols {
                form: 0,
                first: Box::new(Reading::NumericCostSymbol {
                    form: 0,
                    number: Word {
                        value: LexicalReading::Numeral {
                            value: 2,
                            notation: Numeral::Arabic(false),
                            capitalization: SurfaceCase::Declared,
                        },
                        frame: None,
                        countability: None,
                    },
                }),
                rest: vec![],
            }),
        }),
        rest: vec![],
    }
}

fn check(text: &str, expected: &Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    assert!(readings(text).contains(expected));
    let mut words = vec![];
    expected
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    assert!(!words.is_empty());
    for word in words {
        LEXICON.realize(&word.value).unwrap();
    }
}

#[test]
fn authentic_open_labels_preserve_independent_keyword_bodies() {
    // Astrologian's Planisphere and Blue Mage's Cane, respectively.
    for label in ["Diana", "Spirit of the Whalaqee"] {
        let text = format!("{label} — Equip {{2}}");
        let body = equip();
        let expected = flavor(label, body.clone());
        check(&text, &expected);
        assert_eq!(readings(&text), BTreeSet::from([expected.clone()]));
        assert_eq!(
            expected.total_cost().unwrap(),
            100 + body.total_cost().unwrap()
        );
        let mut visited = vec![];
        expected
            .visit_words(&mut |word| visited.push(word.clone()))
            .unwrap();
        let mut body_words = vec![];
        body.visit_words(&mut |word| body_words.push(word.clone()))
            .unwrap();
        assert_eq!(visited[1..], body_words);
        assert_eq!(
            visited[0].value,
            LexicalReading::FlavorWord {
                label: label.into()
            }
        );
    }
}

#[test]
fn declared_ability_word_keeps_both_readings_and_favors_its_declared_analysis() {
    // Mnemonic Sphere.
    let text = "Channel — {U}, Discard this card: Draw a card.";
    let actual = readings(text);
    assert!(!actual.is_empty());
    let mut declared = 0;
    let mut fallback = 0;
    for reading in &actual {
        check(text, reading);
        match reading {
            Reading::AbilityWordHead { body, .. } => {
                declared += 1;
                let alternative = flavor("Channel", *body.clone());
                assert!(actual.contains(&alternative));
                assert_eq!(
                    alternative.total_cost().unwrap() - reading.total_cost().unwrap(),
                    99
                );
            }
            Reading::FlavorWordHead { .. } => fallback += 1,
            other => panic!("unexpected label reading: {other:?}"),
        }
    }
    assert_eq!(declared, fallback);
    assert!(declared > 0);
}

#[test]
fn label_requires_its_complete_written_separator_and_a_parsed_body() {
    for text in [
        "Diana— Equip {2}",
        "Diana - Equip {2}",
        "Diana —",
        "Diana — Unparsed prose.",
        "Diana\n — Equip {2}",
    ] {
        assert!(readings(text).is_empty(), "{text}");
    }
}

#[test]
fn independently_constructed_nested_labels_and_invalid_lexical_shapes_are_rejected() {
    let inner = flavor("Diana", equip());
    assert!(
        flavor("Spirit of the Whalaqee", inner)
            .admit(&LEXICON)
            .is_err()
    );
    let mut invalid = flavor("Diana", equip());
    if let Reading::FlavorWordHead { head, .. } = &mut invalid {
        head.frame = Some(0);
    }
    assert!(invalid.admit(&LEXICON).is_err());
    let mut invalid = flavor("Diana", equip());
    if let Reading::FlavorWordHead { head, .. } = &mut invalid {
        head.countability = Some(true);
    }
    assert!(invalid.admit(&LEXICON).is_err());
}
