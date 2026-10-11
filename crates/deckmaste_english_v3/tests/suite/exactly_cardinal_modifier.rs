use std::collections::BTreeSet;

use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Numeral, SurfaceCase, WordForm,
};

fn modifier() -> Reading {
    Reading::AdverbCardinalModifier {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:Adverb/Exactly".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
    }
}

fn precise(value: i32, notation: Numeral) -> Reading {
    let head = Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    };
    let modifier = Some(Box::new(modifier()));
    if notation == Numeral::Cardinal {
        Reading::Cardinal {
            form: 0,
            modifier,
            head,
        }
    } else {
        Reading::UngroupedScalarNumeral {
            form: 0,
            modifier,
            head,
        }
    }
}

fn assert_role(values: BTreeSet<Reading>, expected: &Reading) {
    assert!(!values.is_empty());
    for value in values {
        let mut numerals = Vec::new();
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if node.category() == Category::CardinalModifier {
                    modifiers.push(node.clone());
                }
                if let Reading::Cardinal {
                    modifier: Some(_), ..
                }
                | Reading::UngroupedScalarNumeral {
                    modifier: Some(_), ..
                } = node
                {
                    numerals.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(modifiers, [modifier()]);
        assert_eq!(numerals.as_slice(), std::slice::from_ref(expected));
    }
}

fn witness(card: &str, text: &str, value: i32, notation: Numeral) {
    assert_role(
        common::readings_on_card(card, text, Category::Document),
        &precise(value, notation),
    );
}

#[test]
fn library_of_alexandria_keeps_precision_inside_the_card_count() {
    witness(
        "Library of Alexandria",
        "{T}: Draw a card. Activate only if you have exactly seven cards in hand.",
        7,
        Numeral::Cardinal,
    );
}

#[test]
fn homicidal_seclusion_keeps_precision_inside_the_determiner() {
    witness(
        "Homicidal Seclusion",
        "As long as you control exactly one creature, that creature gets +3/+1 and has lifelink.",
        1,
        Numeral::Cardinal,
    );
}

#[test]
fn near_death_experience_keeps_precision_inside_the_measured_amount() {
    witness(
        "Near-Death Experience",
        "At the beginning of your upkeep, if you have exactly 1 life, you win the game.",
        1,
        Numeral::Arabic(false),
    );
}

#[test]
fn alluring_suitor_keeps_precision_inside_the_prepositions_complement() {
    witness(
        "Alluring Suitor",
        "When you attack with exactly two creatures, transform this creature.",
        2,
        Numeral::Cardinal,
    );
}

#[test]
fn amalia_keeps_precision_inside_the_scalar_value() {
    witness(
        "Amalia Benavides Aguirre",
        "Then destroy all other creatures if its power is exactly 20.",
        20,
        Numeral::Arabic(false),
    );
}

#[test]
fn attested_nominal_and_partitive_constituents_keep_the_numeral_modifier() {
    // Homicidal Seclusion, Library of Alexandria, Ghyrson Starn, Kelermorph,
    // and General Jarkeld respectively.
    for (text, amount, notation) in [
        ("exactly one creature", 1, Numeral::Cardinal),
        ("exactly seven cards in hand", 7, Numeral::Cardinal),
        ("exactly 1 damage", 1, Numeral::Arabic(false)),
        (
            "exactly one of those attacking creatures",
            1,
            Numeral::Cardinal,
        ),
    ] {
        assert_role(
            common::readings(text, Category::NounPhrase),
            &precise(amount, notation),
        );
    }
}

#[test]
fn independently_constructed_precise_numerals_obey_both_roundtrip_laws() {
    for (value, text, category) in [
        (
            precise(7, Numeral::Cardinal),
            "exactly seven",
            Category::Cardinal,
        ),
        (
            precise(20, Numeral::Arabic(false)),
            "exactly 20",
            Category::MeasurePhrase,
        ),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        assert_eq!(common::readings(text, category), BTreeSet::from([value]));
    }
}

#[test]
fn precision_has_no_np_modifier_or_clause_adjunct_licence() {
    // Determined-NP shapes are deferred by EX2, as are all Adjunct positions.
    for text in [
        "exactly the chosen number of cards",
        "exactly those colors",
        "exactly that many colors",
    ] {
        assert!(common::readings(text, Category::NounPhrase).is_empty());
    }
    for text in [
        "Exactly, draw a card.",
        "Draw a card exactly.",
        "You exactly draw a card.",
        "Its power is exactly.",
    ] {
        assert!(common::readings(text, Category::Document).is_empty());
    }
    let Reading::AdverbCardinalModifier { head, .. } = modifier() else {
        unreachable!()
    };
    let properties = &common::lexicon()
        .lexemes()
        .get(match &head.value {
            LexicalReading::Word(word) => &word.lexeme,
            _ => unreachable!(),
        })
        .unwrap()
        .properties
        .features;
    assert_eq!(properties["CardinalModifierUse"], "Yes");
    assert_eq!(properties["VPFinalAdjunct"], "No");
    assert_eq!(properties["ClauseInitialAdjunct"], "No");
    assert!(!properties.contains_key("CentralAdjunctLicence"));
}
