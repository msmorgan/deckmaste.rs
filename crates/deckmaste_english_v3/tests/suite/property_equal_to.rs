use crate::common;
use crate::scalar_property_values::{assert_laws, word};
use deckmaste_english_v3::grammar::{Category, Reading};
use deckmaste_lexical::WordForm;

fn assert_property_value(card: &str, text: &str, property: &str, comparison: &str) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    let mut full_comparison = false;
    for reading in values {
        let mut found = false;
        reading
            .visit(&mut |node| {
                if let Reading::MeasuredAttribute { head, quantity, .. } = node
                    && head.realize(common::lexicon()).unwrap() == property
                {
                    let Reading::ScalarMeasureValue {
                        category, value, ..
                    } = quantity.as_ref()
                    else {
                        panic!("equality must enter the existing property-value slot")
                    };
                    assert_eq!(*category, Category::PropertyValue);
                    let Reading::ComparativeScalarValue { value, .. } = value.as_ref() else {
                        panic!("the value must retain its comparative constituent")
                    };
                    assert_eq!(value.category(), Category::ComparativeAdjectivePhrase);
                    found = true;
                    full_comparison |= quantity.realize(common::lexicon()).unwrap() == comparison;
                }
            })
            .unwrap();
        assert!(
            found,
            "every Reading must keep equality in the property value: {card}"
        );
    }
    assert!(
        full_comparison,
        "the entire comparison must remain admitted: {card}"
    );
}

#[test]
fn master_symmetrist() {
    assert_property_value(
        "Master Symmetrist",
        "Whenever a creature you control with power equal to its toughness attacks, it gains trample until end of turn.",
        "power",
        "equal to its toughness",
    );
}

#[test]
fn chalice_of_the_void() {
    assert_property_value(
        "Chalice of the Void",
        "Whenever a player casts a spell with mana value equal to the number of charge counters on this artifact, counter that spell.",
        "mana value",
        "equal to the number of charge counters on this artifact",
    );
}

#[test]
fn sanctum_prelate() {
    assert_property_value(
        "Sanctum Prelate",
        "Noncreature spells with mana value equal to the chosen number can't be cast.",
        "mana value",
        "equal to the chosen number",
    );
}

#[test]
fn ratchet_bomb() {
    assert_property_value(
        "Ratchet Bomb",
        "{T}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
        "mana value",
        "equal to the number of charge counters on this artifact",
    );
}

#[test]
fn aether_vial() {
    assert_property_value(
        "Aether Vial",
        "{T}: You may put a creature card with mana value equal to the number of charge counters on this artifact from your hand onto the battlefield.",
        "mana value",
        "equal to the number of charge counters on this artifact",
    );
}

#[test]
fn predicative_equality_keeps_its_reading() {
    assert_eq!(
        common::readings("Its power is equal to its toughness.", Category::Document).len(),
        1
    );
    common::assert_constituents(
        "Its power is equal to its toughness.",
        Category::Document,
        &[(Category::AdjectivePhrase, "equal to its toughness")],
    );
}

#[test]
fn nonproperty_damage_keeps_its_postmodifier() {
    let text = "damage equal to its power";
    let values = common::readings(text, Category::NounPhrase);
    assert_eq!(values.len(), 1);
    for value in values {
        value
            .visit(&mut |node| {
                if let Reading::MeasuredAttribute { head, .. } = node {
                    assert_ne!(head.realize(common::lexicon()).unwrap(), "damage");
                }
            })
            .unwrap();
    }
    common::assert_constituents(
        text,
        Category::NounPhrase,
        &[(Category::AdjectivePhrase, "equal to its power")],
    );
}

#[test]
fn ironclaw_curse_keeps_its_coordinated_governor_readings() {
    let text = "Enchanted creature can't block creatures with power equal to or greater than the enchanted creature's toughness.";
    assert_eq!(
        common::readings_on_card("Ironclaw Curse", text, Category::Document).len(),
        6
    );
    common::assert_constituents_on_card(
        "Ironclaw Curse",
        text,
        Category::Document,
        &[
            (
                Category::ComparativeGovernorHead,
                "equal to or greater than",
            ),
            (
                Category::PropertyValue,
                "equal to or greater than the enchanted creature's toughness",
            ),
        ],
    );
}

#[test]
fn soul_separator_coordinates_two_property_values() {
    let text = "Create a black Zombie creature token with power equal to that card's power and toughness equal to that card's toughness.";
    // Witness both property values in one Reading while retaining alternative
    // coordination and genitive scopes under the ambiguity contract.
    common::assert_constituents_on_card(
        "Soul Separator",
        text,
        Category::Document,
        &[
            (
                Category::NounPhrase,
                "power equal to that card's power and toughness equal to that card's toughness",
            ),
            (Category::NounPhrase, "power equal to that card's power"),
            (
                Category::NounPhrase,
                "toughness equal to that card's toughness",
            ),
            (Category::PropertyValue, "equal to that card's power"),
            (Category::PropertyValue, "equal to that card's toughness"),
        ],
    );
}

#[test]
fn landed_arithmetic_composes_with_equality() {
    // Birthing Pod and Neoform share this property constituent.
    let text = "mana value equal to 1 plus the sacrificed creature's mana value";
    common::assert_constituents(
        text,
        Category::NounPhrase,
        &[
            (
                Category::PropertyValue,
                "equal to 1 plus the sacrificed creature's mana value",
            ),
            (
                Category::MeasurePhrase,
                "1 plus the sacrificed creature's mana value",
            ),
            (
                Category::ArithmeticOperand,
                "the sacrificed creature's mana value",
            ),
        ],
    );
}

#[test]
fn independently_composed_equality_property_preserves_both_laws() {
    // Master Symmetrist's property, composed from its complement and lexical heads.
    let complement = common::readings("its toughness", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    let comparative = Reading::ComparativeAdjectivePhrase {
        form: 0,
        governor: Box::new(Reading::ComparativeGovernorHead {
            form: 0,
            head: word("vocab:ScalarDegree/Equal", WordForm::Invariant),
            marker: word("vocab:Preposition/To", WordForm::Invariant),
        }),
        complement: Box::new(Reading::NominalComparativeComplement {
            form: 0,
            value: Box::new(complement),
        }),
    };
    assert_laws(
        &comparative,
        "equal to its toughness",
        Category::ComparativeAdjectivePhrase,
    );
    let scalar = Reading::ComparativeScalarValue {
        form: 0,
        value: Box::new(comparative),
    };
    assert_laws(&scalar, "equal to its toughness", Category::ScalarValue);
    let quantity = Reading::ScalarMeasureValue {
        category: Category::PropertyValue,
        form: 0,
        value: Box::new(scalar),
    };
    assert_laws(&quantity, "equal to its toughness", Category::PropertyValue);
    let property = Reading::MeasuredAttribute {
        form: 0,
        head: Box::new(Reading::Noun {
            form: 0,
            head: word("lexeme:CommonNoun/Power", WordForm::Singular),
        }),
        quantity: Box::new(quantity),
    };
    assert_laws(
        &property,
        "power equal to its toughness",
        Category::NounPhrase,
    );
}
