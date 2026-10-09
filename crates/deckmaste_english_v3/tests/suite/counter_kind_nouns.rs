use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

fn witness(card: &str, text: &str, kind: &str, occurrences: usize) {
    let readings = common::readings_on_card(card, text, Category::Sentence);
    assert!(!readings.is_empty(), "{card}");
    let owner = format!("lexeme:counter_kind/{kind}Counter/compound-noun");
    for reading in readings {
        let mut counters = Vec::new();
        reading
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value
                    && value.lexeme.as_str() == owner
                {
                    counters.push(common::lexicon().realize(&word.value).unwrap());
                }
            })
            .unwrap();
        assert_eq!(counters.len(), occurrences, "{card}: {reading:?}");
        assert!(counters.iter().all(|surface| {
            surface == &format!("{kind} counter") || surface == &format!("{kind} counters")
        }));
    }
}

#[test]
fn xathrid_gorgon() {
    witness(
        "Xathrid Gorgon",
        "Put a petrification counter on target creature.",
        "petrification",
        1,
    );
}

#[test]
fn flamewar_streetwise_operative() {
    witness(
        "Flamewar, Streetwise Operative",
        "Put an intel counter on each of them.",
        "intel",
        1,
    );
}

#[test]
fn out_of_the_tombs() {
    witness(
        "Out of the Tombs",
        "At the beginning of your upkeep, put two eon counters on this enchantment, then mill cards equal to the number of eon counters on it.",
        "eon",
        2,
    );
}

#[test]
fn the_magic_mirror() {
    witness(
        "The Magic Mirror",
        "At the beginning of your upkeep, put a knowledge counter on The Magic Mirror, then draw a card for each knowledge counter on The Magic Mirror.",
        "knowledge",
        2,
    );
}

#[test]
fn icatian_javelineers() {
    witness(
        "Icatian Javelineers",
        "This creature enters with a javelin counter on it.",
        "javelin",
        1,
    );
}

#[test]
fn independently_constructed_compounds_preserve_identity_and_number() {
    // These constituents occur on the ticket witnesses, Legacy's Allure,
    // Mine Layer, Spell Satchel, Fated Firepower, Aurification,
    // Karn, Scion of Urza, and Dawn of a New Age.
    for kind in [
        "petrification",
        "intel",
        "eon",
        "knowledge",
        "javelin",
        "treasure",
        "mine",
        "book",
        "fire",
        "gold",
        "silver",
        "hope",
    ] {
        for number in [Number::Singular, Number::Plural] {
            let head = Word {
                value: LexicalReading::Word(LexicalValue {
                    lexeme: format!("lexeme:counter_kind/{kind}Counter/compound-noun")
                        .as_str()
                        .into(),
                    form: if number == Number::Singular {
                        WordForm::Singular
                    } else {
                        WordForm::Plural
                    },
                    features: FeatureBundle {
                        number: Some(number),
                        ..Default::default()
                    },
                    variant: 0,
                    capitalization: SurfaceCase::Declared,
                }),
                frame: None,
                countability: Some(true),
            };
            let expected = Reading::Noun { form: 0, head };
            expected.admit(common::lexicon()).unwrap();
            let text = format!(
                "{kind} counter{}",
                if number == Number::Plural { "s" } else { "" }
            );
            assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
            assert_eq!(
                common::readings(&text, Category::Nominal),
                BTreeSet::from([expected])
            );
        }
    }
}
