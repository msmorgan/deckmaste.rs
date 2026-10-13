use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, Numeral, SurfaceCase, WordForm,
};

use crate::common;

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn amount(value: i32, notation: Numeral, comparative: &str, noun: &str) -> Reading {
    let numeral = Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    };
    let left = if notation == Numeral::Cardinal {
        Reading::CardinalQuantityConjunct {
            form: 0,
            value: Box::new(Reading::Cardinal {
                form: 0,
                modifier: None,
                head: numeral,
            }),
        }
    } else {
        Reading::NumericQuantityConjunct {
            form: 0,
            head: numeral,
        }
    };
    let mut head = word(noun);
    let LexicalReading::Word(ref mut lexical) = head.value else { unreachable!() };
    lexical.form = WordForm::Singular;
    lexical.features.number = Some(Number::Singular);
    head.countability = Some(false);
    Reading::MeasuredNounPhrase {
        form: 1,
        quantity: Box::new(Reading::NumeralComparativeCoordination {
            form: 0,
            left: Box::new(left),
            marker: word("vocab:Coordinator/Or"),
            right: word(comparative),
        }),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun { form: 0, head }),
            }),
        }),
    }
}

fn witness(card: &str, text: &str, expected: &Reading) {
    let values = common::readings_on_card(card, text, Category::Sentence);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut quantities = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::MeasuredNounPhrase { quantity, .. } = node
                    && quantity.category() == Category::ComparativeQuantity
                {
                    quantities.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(
            quantities.as_slice(),
            std::slice::from_ref(expected),
            "{card}"
        );
    }
}

#[test]
fn abandoned_campground_measures_life_with_an_upper_comparative_bound() {
    witness(
        "Abandoned Campground",
        "This land enters tapped unless a player has 13 or less life.",
        &amount(
            13,
            Numeral::Arabic(false),
            "vocab:Determinative/Less",
            "lexeme:CommonNoun/Life",
        ),
    );
}

#[test]
fn divinity_of_pride_measures_life_in_a_conditional_complement() {
    witness(
        "Divinity of Pride",
        "This creature gets +4/+4 as long as you have 25 or more life.",
        &amount(
            25,
            Numeral::Arabic(false),
            "vocab:ComparativeQuantifier/More",
            "lexeme:CommonNoun/Life",
        ),
    );
}

#[test]
fn innocent_bystander_measures_the_passive_damage_complement() {
    witness(
        "Innocent Bystander",
        "Whenever this creature is dealt 3 or more damage, investigate.",
        &amount(
            3,
            Numeral::Arabic(false),
            "vocab:ComparativeQuantifier/More",
            "lexeme:CommonNoun/Damage",
        ),
    );
}

#[test]
fn callous_giant_measures_the_active_damage_object() {
    witness(
        "Callous Giant",
        "If a source would deal 3 or less damage to this creature, prevent that damage.",
        &amount(
            3,
            Numeral::Arabic(false),
            "vocab:Determinative/Less",
            "lexeme:CommonNoun/Damage",
        ),
    );
}

#[test]
fn dragonborn_champion_measures_damage_before_a_recipient_complement() {
    witness(
        "Dragonborn Champion",
        "Whenever a source you control deals 5 or more damage to a player, draw a card.",
        &amount(
            5,
            Numeral::Arabic(false),
            "vocab:ComparativeQuantifier/More",
            "lexeme:CommonNoun/Damage",
        ),
    );
}

#[test]
fn independent_bounded_measurements_keep_one_structure_and_both_roundtrip_laws() {
    // Constituents of Abandoned Campground, Innocent Bystander and
    // Tackle Artist. Neither a count NP nor NP Coordination is admitted.
    for (value, text) in [
        (
            amount(
                13,
                Numeral::Arabic(false),
                "vocab:Determinative/Less",
                "lexeme:CommonNoun/Life",
            ),
            "13 or less life",
        ),
        (
            amount(
                3,
                Numeral::Arabic(false),
                "vocab:ComparativeQuantifier/More",
                "lexeme:CommonNoun/Damage",
            ),
            "3 or more damage",
        ),
        (
            amount(
                5,
                Numeral::Cardinal,
                "vocab:ComparativeQuantifier/More",
                "lexeme:CommonNoun/Mana",
            ),
            "five or more mana",
        ),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::from([value])
        );
    }
}

#[test]
fn zubera_damage_subject_keeps_singular_agreement() {
    // Burning-Eye Zubera and Rushing-Tide Zubera's attested conditional Clause.
    let text = "4 or more damage was dealt to it this turn";
    let values = common::readings(text, Category::Clause);
    assert!(!values.is_empty());
    let expected = amount(
        4,
        Numeral::Arabic(false),
        "vocab:ComparativeQuantifier/More",
        "lexeme:CommonNoun/Damage",
    );
    for value in values {
        let mut subjects = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::MeasuredNounPhrase { quantity, .. } = node
                    && quantity.category() == Category::ComparativeQuantity
                {
                    subjects.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(subjects.as_slice(), std::slice::from_ref(&expected));
    }
    assert!(common::readings(&text.replace("was", "were"), Category::Clause).is_empty());
}

#[test]
fn count_heads_plain_measurements_and_postpositive_controls_keep_their_roles() {
    for (text, quantity) in [
        ("5 or more cards", "5 or more"),
        ("4 or more counters", "4 or more"),
    ] {
        for value in common::readings(text, Category::NounPhrase) {
            let Reading::QuantifiedNounPhrase {
                quantity: observed, ..
            } = value
            else {
                panic!("count head must retain quantitative Determiner function")
            };
            assert_eq!(observed.realize(common::lexicon()).unwrap(), quantity);
        }
        common::assert_constituents(
            text,
            Category::NounPhrase,
            &[(Category::QuantitativeDeterminer, quantity)],
        );
    }
    for text in [
        "A player has 13 life.",
        "You have at least 7 life.",
        "You have 5 or more cards in hand.",
    ] {
        assert!(!common::readings(text, Category::Sentence).is_empty());
    }
    common::assert_constituents(
        "mana value 3 or less",
        Category::NounPhrase,
        &[(Category::ComparativeQuantity, "3 or less")],
    );
}
