use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, Numeral, SurfaceCase, WordForm,
};

use crate::common::{lexicon, readings};

fn word(owner: &str, number: Option<Number>, variant: usize) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle {
                number,
                ..FeatureBundle::default()
            },
            variant,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn component(value: i32) -> Reading {
    Reading::PositiveScalar {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
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
    }
}

fn adjustment(owner: &str) -> Reading {
    Reading::SlashPair {
        form: 0,
        determiner: Some(word("vocab:Article/Indefinite", Some(Number::Singular), 1)),
        modifier: Some(word(owner, None, 0)),
        left: Box::new(component(1)),
        right: Box::new(component(0)),
    }
}

#[test]
fn witnesses_keep_the_modifier_inside_the_selected_adjustment_complement() {
    // Silver-Inlaid Dagger, Wild Might, Taste for Mayhem, Desert's Due, Tenza.
    for text in [
        "As long as equipped creature is a Human, it gets an additional +1/+0.",
        "That creature gets an additional +4/+4 until end of turn unless any player pays {2}.",
        "Enchanted creature gets an additional +2/+0 as long as you have no cards in hand.",
        "It gets an additional -1/-1 until end of turn for each Desert you control.",
        "As long as it's legendary, it gets an additional +2/+2.",
    ] {
        let values = readings(text, Category::Sentence);
        let control = readings(&text.replace("an additional ", ""), Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        // AD6 / W9-A: the construction supplies one route while keeping
        // the same independent ambiguities as its unmodified control.
        assert_eq!(values.len(), control.len(), "{text}");
        for value in values {
            let mut complements = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        complements: arguments,
                        ..
                    } = node
                    {
                        for argument in arguments {
                            if let FrameValue::Argument(pair) = argument
                                && let Reading::SlashPair {
                                    modifier: Some(_), ..
                                } = pair.as_ref()
                            {
                                complements.push(pair.as_ref().clone());
                            }
                        }
                    }
                })
                .unwrap();
            assert_eq!(complements.len(), 1, "one adjusted Complement per witness");
            let Reading::SlashPair {
                determiner,
                modifier,
                ..
            } = &complements[0]
            else {
                unreachable!()
            };
            assert_eq!(
                *determiner,
                Some(word("vocab:Article/Indefinite", Some(Number::Singular), 1))
            );
            assert_eq!(
                *modifier,
                Some(word("vocab:AttributiveAdjective/Additional", None, 0))
            );
        }
    }
}

#[test]
fn independent_additive_class_members_obey_both_roundtrip_laws() {
    for (owner, surface) in [
        (
            "vocab:AttributiveAdjective/Additional",
            "an additional +1/+0",
        ),
        ("vocab:Adjective/Extra", "an extra +1/+0"),
    ] {
        let pair = adjustment(owner);
        pair.admit(lexicon()).unwrap();
        assert_eq!(pair.realize(lexicon()).unwrap(), surface);
        assert_eq!(
            readings(surface, Category::SlashPair),
            BTreeSet::from([pair])
        );
    }
}

#[test]
fn articles_and_modifiers_are_paired_and_agree_in_onset() {
    let pair = adjustment("vocab:AttributiveAdjective/Additional");
    for text in ["a additional +1/+0", "an +1/+0", "additional +1/+0"] {
        assert!(readings(text, Category::SlashPair).is_empty(), "{text}");
    }
    for case in 0..4 {
        let mut invalid = pair.clone();
        let Reading::SlashPair {
            determiner,
            modifier,
            ..
        } = &mut invalid
        else {
            unreachable!()
        };
        match case {
            0 => *determiner = None,
            1 => *modifier = None,
            2 => *determiner = Some(word("vocab:Article/Indefinite", Some(Number::Singular), 0)),
            3 => *modifier = Some(word("vocab:AttributiveAdjective/Starting", None, 0)),
            _ => unreachable!(),
        }
        assert!(invalid.admit(lexicon()).is_err());
    }
}

#[test]
fn noun_controls_keep_their_nominal_modifier_and_bare_pair() {
    assert!(readings("an additional +1/+0", Category::NounPhrase).is_empty());
    for text in [
        "It deals an additional 2 damage.",
        "It enters with an additional +1/+1 counter on it.",
        "Draw an additional card.",
    ] {
        let values = readings(text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut nominal_modifiers = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::BarrierPremodifiedNominal { modifier, .. } = node
                        && let Reading::Adjective { head, .. } = modifier.as_ref()
                    {
                        nominal_modifiers.push(head.clone());
                    }
                    if let Reading::SlashPair {
                        determiner,
                        modifier,
                        ..
                    } = node
                    {
                        assert_eq!(*determiner, None);
                        assert_eq!(*modifier, None);
                    }
                })
                .unwrap();
            assert!(nominal_modifiers.contains(&word(
                "vocab:AttributiveAdjective/Additional",
                None,
                0
            )));
        }
    }
}
