use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::WordForm;

use crate::common;
use crate::scalar_property_values::assert_laws;
use crate::scalar_property_values::word;

fn assert_property_value(text: &str, property: &str, comparison: &str) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "{text}");
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
                        panic!("the comparative must enter the existing property-value slot")
                    };
                    assert_eq!(*category, Category::PropertyValue);
                    let Reading::ComparativeScalarValue { value, .. } = value.as_ref() else {
                        panic!("the property value must retain the comparative constituent")
                    };
                    assert_eq!(value.category(), Category::ComparativeAdjectivePhrase);
                    found = true;
                    full_comparison |= quantity.realize(common::lexicon()).unwrap() == comparison;
                }
            })
            .unwrap();
        assert!(
            found,
            "every Reading must retain the comparative as the property value: {reading:?}"
        );
    }
    // Genitive scope can vary; require the ordinary comparison with the entire
    // genitive inside its Complement without discarding other grammatical
    // Readings (english-lexical-analysis roundtrip and ambiguity contract).
    assert!(
        full_comparison,
        "the complete comparison must remain admitted: {comparison}"
    );
}

#[test]
fn howlgeist() {
    assert_property_value(
        "Creatures with power less than this creature's power can't block it.",
        "power",
        "less than this creature's power",
    );
}

#[test]
fn unliving_psychopath() {
    assert_property_value(
        "{B}, {T}: Destroy target creature with power less than this creature's power.",
        "power",
        "less than this creature's power",
    );
}

#[test]
fn ensnaring_bridge() {
    assert_property_value(
        "Creatures with power greater than the number of cards in your hand can't attack.",
        "power",
        "greater than the number of cards in your hand",
    );
}

#[test]
fn immobilizer_eldrazi() {
    assert_property_value(
        "{2}{C}: Each creature with toughness greater than its power can't block this turn.",
        "toughness",
        "greater than its power",
    );
}

#[test]
fn kiora_sovereign_of_the_deep() {
    assert_property_value(
        "You may cast a spell with mana value less than X from among them without paying its mana cost.",
        "mana value",
        "less than X",
    );
}

#[test]
fn shared_comparative_complement_stays_inside_the_value_slot() {
    // Legacy's Allure, activated ability.
    common::assert_constituents(
        "power less than or equal to the number of treasure counters on this enchantment",
        Category::NounPhrase,
        &[
            (
                Category::PropertyValue,
                "less than or equal to the number of treasure counters on this enchantment",
            ),
            (Category::ComparativeGovernorHead, "less than or equal to"),
            (
                Category::ComparativeComplement,
                "the number of treasure counters on this enchantment",
            ),
        ],
    );
    common::assert_constituents(
        "equal to its toughness",
        Category::PropertyValue,
        &[
            (
                Category::ComparativeAdjectivePhrase,
                "equal to its toughness",
            ),
            (Category::ComparativeGovernorHead, "equal to"),
            (Category::ComparativeComplement, "its toughness"),
        ],
    );
}

#[test]
fn existing_routes_keep_their_predicate_readings() {
    assert_eq!(
        common::readings(
            "Destroy target creature with power 3 or less.",
            Category::Document
        )
        .len(),
        2
    );
    assert_eq!(
        common::readings("Its power is less than 3.", Category::Document).len(),
        1
    );
}

#[test]
fn independent_comparative_property_value_preserves_both_laws() {
    // The property constituent of Kiora, Sovereign of the Deep's trigger.
    let comparative = Reading::ComparativeAdjectivePhrase {
        form: 0,
        governor: Box::new(Reading::ComparativeGovernorHead {
            form: 0,
            head: word("vocab:Adjective/Less", WordForm::Invariant),
            marker: word("vocab:Preposition/Than", WordForm::Invariant),
        }),
        complement: Box::new(Reading::ScalarComparativeComplement {
            form: 0,
            value: Box::new(Reading::ScalarMeasurePhrase {
                form: 0,
                head: Box::new(Reading::ScalarVariable {
                    category: Category::MeasurePhrase,
                    form: 0,
                    head: word("vocab:Variable/X", WordForm::Invariant),
                }),
            }),
        }),
    };
    assert_laws(
        &comparative,
        "less than X",
        Category::ComparativeAdjectivePhrase,
    );
    let scalar = Reading::ComparativeScalarValue {
        form: 0,
        value: Box::new(comparative),
    };
    assert_laws(&scalar, "less than X", Category::ScalarValue);
    let quantity = Reading::ScalarMeasureValue {
        category: Category::PropertyValue,
        form: 0,
        value: Box::new(scalar),
    };
    assert_laws(&quantity, "less than X", Category::PropertyValue);
    let value = Reading::MeasuredAttribute {
        form: 0,
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word("lexeme:CommonNoun/ManaValue", WordForm::Singular),
                }),
            }),
        }),
        quantity: Box::new(quantity),
    };
    assert_laws(&value, "mana value less than X", Category::NounPhrase);
}
