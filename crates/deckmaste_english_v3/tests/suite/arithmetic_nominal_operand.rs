use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Numeral, SurfaceCase, WordForm,
};
use std::collections::BTreeSet;

const WITNESSES: &[(&str, &str, &str, &str)] = &[
    (
        "Kindle",
        "Kindle deals X damage to any target, where X is 2 plus the number of cards named Kindle in all graveyards.",
        "2 plus the number of cards named Kindle in all graveyards",
        "the number of cards named Kindle in all graveyards",
    ),
    (
        "Ivory Tower",
        "At the beginning of your upkeep, you gain X life, where X is the number of cards in your hand minus 4.",
        "the number of cards in your hand minus 4",
        "the number of cards in your hand",
    ),
    (
        "Allosaurus Rider",
        "Allosaurus Rider's power and toughness are each equal to 1 plus the number of lands you control.",
        "1 plus the number of lands you control",
        "the number of lands you control",
    ),
    (
        "Haunting Apparition",
        "Haunting Apparition's power is equal to 1 plus the number of green creature cards in the chosen player's graveyard.",
        "1 plus the number of green creature cards in the chosen player's graveyard",
        "the number of green creature cards in the chosen player's graveyard",
    ),
    (
        "Food Chain",
        "Exile a creature you control: Add X mana of any one color, where X is 1 plus the exiled creature's mana value.",
        "1 plus the exiled creature's mana value",
        "the exiled creature's mana value",
    ),
];

fn witness(index: usize) {
    let (card, text, expression, operand) = WITNESSES[index];
    common::assert_constituents_on_card(
        card,
        text,
        Category::Document,
        &[
            (Category::MeasurePhrase, expression),
            (Category::ArithmeticOperand, operand),
            (Category::NounPhrase, operand),
        ],
    );
}

#[test]
fn kindle_keeps_the_card_count_as_a_right_operand() {
    witness(0);
}
#[test]
fn ivory_tower_keeps_the_card_count_as_a_left_operand() {
    witness(1);
}
#[test]
fn allosaurus_rider_keeps_the_land_count() {
    witness(2);
}
#[test]
fn haunting_apparition_keeps_the_modified_card_count() {
    witness(3);
}
#[test]
fn food_chain_keeps_the_exiled_creatures_mana_value() {
    witness(4);
}

#[test]
fn war_report_keeps_two_scalar_nominal_operands() {
    let text = "the number of creatures on the battlefield plus the number of artifacts on the battlefield";
    assert_eq!(common::readings(text, Category::MeasurePhrase).len(), 4);
    common::assert_constituents(
        text,
        Category::MeasurePhrase,
        &[
            (
                Category::ArithmeticOperand,
                "the number of creatures on the battlefield",
            ),
            (
                Category::NounPhrase,
                "the number of creatures on the battlefield",
            ),
            (
                Category::ArithmeticOperand,
                "the number of artifacts on the battlefield",
            ),
            (
                Category::NounPhrase,
                "the number of artifacts on the battlefield",
            ),
        ],
    );
}

#[test]
fn torbrans_quantified_damage_is_a_scalar_nominal_operand() {
    // Arithmetic constituent of Torbran, Thane of Red Fell's replacement clause.
    let text = "that much damage plus 2";
    assert_eq!(common::readings(text, Category::MeasurePhrase).len(), 1);
    common::assert_constituents(
        text,
        Category::MeasurePhrase,
        &[
            (Category::ArithmeticOperand, "that much damage"),
            (Category::NounPhrase, "that much damage"),
            (Category::QuantitativeDeterminer, "that much"),
            (Category::Nominal, "damage"),
        ],
    );
}

#[test]
fn unlicensed_operands_and_preposition_functions_stay_excluded() {
    assert_eq!(
        common::readings("2 life plus 2 life", Category::NounPhrase).len(),
        0
    );
    for text in [
        "a creature plus 1",
        "2 plus a creature",
        "that many plus one",
        "that many plus 1",
        "2 plus an additional {1}",
        "mana cost plus exile three other cards",
        "1/1 plus 2",
        "2 plus 1/1",
    ] {
        assert_eq!(
            common::readings(text, Category::MeasurePhrase).len(),
            0,
            "{text}"
        );
    }
    assert_eq!(
        common::readings("plus an additional {1}", Category::PrepositionPhrase).len(),
        0
    );
}

fn numeral(value: i32) -> Reading {
    Reading::UngroupedScalarNumeral {
        form: 0,
        modifier: None,
        head: Word {
            value: LexicalReading::Numeral {
                value,
                notation: Numeral::Arabic(false),
                capitalization: SurfaceCase::Declared,
            },
            frame: None,
            countability: None,
        },
    }
}
fn measure(value: Reading) -> Reading {
    Reading::ScalarMeasureOperand {
        form: 0,
        value: Box::new(value),
    }
}
fn nominal(value: Reading) -> Reading {
    Reading::ScalarNominalOperand {
        form: 0,
        value: Box::new(value),
    }
}
fn arithmetic(left: Reading, owner: &str, right: Reading) -> Reading {
    Reading::ArithmeticMeasure {
        form: 0,
        left: Box::new(left),
        operator: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        right: Box::new(right),
    }
}
fn exiled_attribute() -> Reading {
    let values = common::readings("the exiled creature's mana value", Category::NounPhrase);
    assert_eq!(values.len(), 1);
    values.into_iter().next().unwrap()
}

#[test]
fn independently_composed_nominal_arithmetic_roundtrips_and_checks_the_licence() {
    // Assemble Food Chain's arithmetic from an admitted NP and declared
    // numeral/operator values, rather than parsing the whole expression.
    let value = arithmetic(
        measure(numeral(1)),
        "vocab:Preposition/Plus",
        nominal(exiled_attribute()),
    );
    let text = "1 plus the exiled creature's mana value";
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, Category::MeasurePhrase).contains(&value));
    let ordinary = common::readings("a creature", Category::NounPhrase)
        .into_iter()
        .next()
        .unwrap();
    let invalid = arithmetic(
        measure(numeral(1)),
        "vocab:Preposition/Plus",
        nominal(ordinary),
    );
    assert!(invalid.admit(common::lexicon()).is_err());
    assert!(invalid.realize(common::lexicon()).is_err());
}

#[test]
fn independently_composed_chains_retain_both_bracketings() {
    // R10c: compose two arithmetic values using Food Chain's attested operand.
    let one = measure(numeral(1));
    let four = measure(numeral(4));
    let attribute = nominal(exiled_attribute());
    let left = arithmetic(
        measure(arithmetic(
            one.clone(),
            "vocab:Preposition/Plus",
            attribute.clone(),
        )),
        "vocab:Preposition/Minus",
        four.clone(),
    );
    let right = arithmetic(
        one,
        "vocab:Preposition/Plus",
        measure(arithmetic(attribute, "vocab:Preposition/Minus", four)),
    );
    let text = "1 plus the exiled creature's mana value minus 4";
    for value in [&left, &right] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
    let values = common::readings(text, Category::MeasurePhrase);
    assert_eq!(values.len(), 2);
    assert_eq!(values, BTreeSet::from([left, right]));
}

#[test]
fn r10e_only_gains_keep_a_scalar_copular_complement() {
    for (card, text, predicate, value) in [
        ("Banefire", "X is 5 or more", "is 5 or more", "5 or more"),
        (
            "Finale of Glory",
            "X is 10 or more",
            "is 10 or more",
            "10 or more",
        ),
        (
            "Roku's Mastery",
            "X is 4 or more",
            "is 4 or more",
            "4 or more",
        ),
        (
            "Silver Scrutiny",
            "X is 3 or less",
            "is 3 or less",
            "3 or less",
        ),
        (
            "Vision Quest",
            "X is 4 or greater",
            "is 4 or greater",
            "4 or greater",
        ),
    ] {
        assert_eq!(
            common::readings_on_card(card, text, Category::FiniteClause).len(),
            1,
            "{card}"
        );
        common::assert_constituents_on_card(
            card,
            text,
            Category::FiniteClause,
            &[
                (Category::NominativePhrase, "X"),
                (Category::ScalarLocationPredicate, predicate),
                (Category::ScalarValue, value),
            ],
        );
    }
}
