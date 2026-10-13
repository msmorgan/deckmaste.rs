use deckmaste_english_v3::grammar::Category;

use crate::common::assert_constituents;
use crate::common::readings;

#[test]
fn reduce_in_stature_and_illusory_wrappings_select_the_pair() {
    // Both cards have this identical Oracle sentence.
    assert_constituents(
        "Enchanted creature has base power and toughness 0/2.",
        Category::Document,
        &[
            (Category::NounPhrase, "base power and toughness 0/2"),
            (Category::Nominal, "base power and toughness"),
            (Category::SlashPair, "0/2"),
        ],
    );
}

#[test]
fn godhead_of_awe_selects_the_pair_under_plural_have() {
    assert_constituents(
        "Other creatures have base power and toughness 1/1.",
        Category::Document,
        &[
            (Category::NounPhrase, "base power and toughness 1/1"),
            (Category::Nominal, "base power and toughness"),
            (Category::SlashPair, "1/1"),
        ],
    );
}

#[test]
fn admiral_brass_selects_the_pair_under_pronominal_have() {
    assert_constituents(
        "It has base power and toughness 4/4.",
        Category::Document,
        &[
            (Category::NounPhrase, "base power and toughness 4/4"),
            (Category::Nominal, "base power and toughness"),
            (Category::SlashPair, "4/4"),
        ],
    );
}

#[test]
fn glamer_gifter_keeps_the_property_complement_inside_the_first_predicate() {
    assert_constituents(
        "Until end of turn, that creature has base power and toughness 4/4 and gains all creature types.",
        Category::Document,
        &[
            (Category::NounPhrase, "base power and toughness 4/4"),
            (Category::Nominal, "base power and toughness"),
            (Category::SlashPair, "4/4"),
        ],
    );
}

#[test]
fn property_value_shape_matches_the_ordered_coordinates() {
    for (text, nominal, value, category) in [
        ("base power 0", "base power", "0", Category::ScalarValue),
        (
            "power and toughness 0/2",
            "power and toughness",
            "0/2",
            Category::SlashPair,
        ),
        (
            "base power and toughness X/X",
            "base power and toughness",
            "X/X",
            Category::SlashPair,
        ),
    ] {
        assert_constituents(
            text,
            Category::NounPhrase,
            &[(Category::Nominal, nominal), (category, value)],
        );
    }
    for text in [
        "power 0/2",
        "power and toughness 2",
        "toughness and power 0/2",
        "power or toughness 0/2",
        "mana value and toughness 0/2",
        "power and toughness and power 0/2",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn with_property_values_postmodifies_the_nominal_in_become() {
    // Warden of the First Tree, first activated ability.
    assert_constituents(
        "{1}{W/B}: This creature becomes a Human Warrior with base power and toughness 3/3.",
        Category::Document,
        &[
            (
                Category::Nominal,
                "Human Warrior with base power and toughness 3/3",
            ),
            (Category::NounPhrase, "base power and toughness 3/3"),
            (Category::SlashPair, "3/3"),
        ],
    );
}

#[test]
fn independent_ordered_pair_and_premodification_preserve_both_laws() {
    use deckmaste_english_v3::grammar::Reading;
    use deckmaste_lexical::WordForm;

    use crate::scalar_property_values::assert_laws;
    use crate::scalar_property_values::numeric;
    use crate::scalar_property_values::word;
    let noun = |property: &str| Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(&format!("lexeme:CommonNoun/{property}"), WordForm::Singular),
            }),
        }),
    };
    let head = Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Coordination {
                category: Category::NominalCore,
                form: 0,
                left: Box::new(noun("Power")),
                coordinator: word("vocab:Coordinator/And", WordForm::Invariant),
                right: Box::new(noun("Toughness")),
            }),
        }),
    };
    let modified = Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::BarrierPremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::Adjective {
                    form: 0,
                    head: word("vocab:AttributiveAdjective/Base", WordForm::Invariant),
                }),
                head: Box::new(head.clone()),
            }),
        }),
    };
    let component = |value| Reading::UnsignedScalar {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: numeric(value),
        }),
    };
    let pair = Reading::SlashPair {
        determiner: None,
        modifier: None,
        form: 0,
        left: Box::new(component(0)),
        right: Box::new(component(2)),
    };
    let variable = Reading::UnsignedScalar {
        form: 0,
        value: Box::new(Reading::ScalarVariable {
            category: Category::UnsignedScalar,
            form: 0,
            head: word("vocab:Variable/X", WordForm::Invariant),
        }),
    };
    let variable_pair = Reading::SlashPair {
        determiner: None,
        modifier: None,
        form: 0,
        left: Box::new(variable.clone()),
        right: Box::new(variable),
    };
    assert_laws(&pair, "0/2", Category::SlashPair);
    assert_laws(&variable_pair, "X/X", Category::SlashPair);
    for (head, prefix) in [
        (head, "power and toughness"),
        (modified, "base power and toughness"),
    ] {
        assert_laws(&head, prefix, Category::Nominal);
        for (pair, surface) in [(&pair, "0/2"), (&variable_pair, "X/X")] {
            let quantity = Reading::SlashMeasure {
                category: Category::PropertyValue,
                form: 0,
                pair: Box::new(pair.clone()),
            };
            assert_laws(&quantity, surface, Category::PropertyValue);
            let np = Reading::MeasuredAttribute {
                form: 0,
                head: Box::new(head.clone()),
                quantity: Box::new(quantity.clone()),
            };
            assert_laws(&np, &format!("{prefix} {surface}"), Category::NounPhrase);
            let invalid = Reading::MeasuredAttribute {
                form: 0,
                head: Box::new(noun("Power")),
                quantity: Box::new(quantity),
            };
            assert!(invalid.admit(crate::common::lexicon()).is_err());
            assert!(invalid.realize(crate::common::lexicon()).is_err());
        }
    }
    assert!(readings("Its power is 0/2.", Category::Document).is_empty());
}

#[test]
fn a_property_complement_does_not_supply_a_required_outer_determiner() {
    for text in ["one power 1", "one power and toughness 0/2"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn only_ordinary_adjectival_premodification_preserves_property_values() {
    // Royal Assassin: "Destroy target tapped creature."
    assert_constituents(
        "target tapped creature",
        Category::NounPhrase,
        &[(Category::AdjectivePhrase, "tapped")],
    );
    for text in [
        "power with flying 2",
        "power of that creature 2",
        "power that it has 2",
        "power named Bob 2",
        "power among creatures you control 2",
        "base power of that creature and toughness 0/2",
        "power and toughness of it 0/2",
        "tapped power 2",
        "untapped power 2",
        "blocked power 2",
        "tapped and base power 2",
        "base tapped power 2",
        "tapped base power 2",
        "creature power 2",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}
