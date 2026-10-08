use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

use crate::common::lexicon;
use crate::common::readings;

fn assert_activated(text: &str, cost: &str) {
    let values = readings(text, Category::Ability);
    assert!(!values.is_empty(), "no Reading for {text:?}");
    for value in values {
        let Reading::ActivatedAbility { cost: actual, .. } = &value else {
            panic!("wrong ability structure: {value:?}");
        };
        assert_eq!(actual.realize(lexicon()).unwrap(), cost);
        assert_eq!(
            readings(cost, Category::Cost),
            std::collections::BTreeSet::from([*actual.clone()])
        );
    }
}

#[test]
fn ob_nixilis_the_hate_twisted() {
    assert_activated("−2: Destroy target creature.", "−2");
}

#[test]
fn kiora_behemoth_beckoner() {
    assert_activated("−1: Untap target permanent.", "−1");
}

#[test]
fn arlinn_voice_of_the_pack() {
    assert_activated("−2: Create a 2/2 green Wolf creature token.", "−2");
}

#[test]
fn jiang_yanggu_wildcrafter() {
    assert_activated("−1: Put a +1/+1 counter on target creature.", "−1");
}

#[test]
fn teyo_the_shieldmage() {
    assert_activated(
        "−2: Create a 0/3 white Wall creature token with defender.",
        "−2",
    );
}

fn numeral(value: i32) -> deckmaste_english_v3::grammar::Word {
    deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Numeral {
            value,
            notation: deckmaste_lexical::Numeral::Arabic(false),
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn scalar_cost(value: Reading) -> Reading {
    Reading::Cost {
        form: 0,
        first: Box::new(Reading::SignedScalarCost {
            form: 0,
            value: Box::new(value),
        }),
        rest: vec![],
    }
}

#[test]
fn independently_constructed_signed_costs_preserve_signs_and_traversal() {
    use std::collections::BTreeSet;

    let digit = numeral(2);
    let number = Reading::SmallUnsignedScalar {
        form: 0,
        head: digit.clone(),
    };
    let variable_word = deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Word(deckmaste_lexical::LexicalValue {
            lexeme: "vocab:Variable/X".into(),
            form: deckmaste_lexical::WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            variant: 0,
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    };
    let variable = Reading::ScalarVariable {
        category: Category::UnsignedScalar,
        form: 0,
        head: variable_word.clone(),
    };
    for (number, surface, word) in [(number, "2", digit), (variable, "X", variable_word)] {
        for (sign, signed) in [
            (
                "+",
                Reading::PositiveScalar {
                    form: 0,
                    value: Box::new(number.clone()),
                },
            ),
            (
                "-",
                Reading::NegativeScalar {
                    form: 0,
                    value: Box::new(number.clone()),
                },
            ),
            (
                "−",
                Reading::NegativeScalar {
                    form: 1,
                    value: Box::new(number.clone()),
                },
            ),
        ] {
            let expected = scalar_cost(signed.clone());
            expected.admit(lexicon()).unwrap();
            let text = format!("{sign}{surface}");
            assert_eq!(expected.realize(lexicon()).unwrap(), text);
            assert_eq!(
                readings(&text, Category::Cost),
                BTreeSet::from([expected.clone()])
            );
            let mut nodes = vec![];
            expected
                .visit(&mut |node| nodes.push(node.clone()))
                .unwrap();
            assert_eq!(
                nodes,
                [
                    expected,
                    Reading::SignedScalarCost {
                        form: 0,
                        value: Box::new(signed.clone())
                    },
                    signed,
                    number.clone(),
                ]
            );
            let mut words = vec![];
            nodes[0]
                .visit_words(&mut |leaf| words.push(leaf.clone()))
                .unwrap();
            assert_eq!(words.as_slice(), std::slice::from_ref(&word));
        }
    }
}

#[test]
fn zero_cost_is_independently_constructible() {
    let expected = Reading::Cost {
        form: 0,
        first: Box::new(Reading::ZeroScalarCost { form: 0 }),
        rest: vec![],
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "0");
    assert_eq!(
        readings("0", Category::Cost),
        std::collections::BTreeSet::from([expected])
    );
}

#[test]
fn unsigned_nonzero_and_malformed_notation_are_not_costs() {
    for text in [
        "1", "2", "X", "two", "+", "−", "-", "++2", "−-2", "+ 2", "− 2", "+2nd",
    ] {
        assert!(
            readings(text, Category::Cost).is_empty(),
            "invalid Cost: {text}"
        );
    }
    let unsigned = scalar_cost(Reading::UnsignedScalar {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: numeral(2),
        }),
    });
    assert!(unsigned.admit(lexicon()).is_err());
    let negative_leaf = scalar_cost(Reading::PositiveScalar {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: numeral(-2),
        }),
    });
    assert!(negative_leaf.admit(lexicon()).is_err());
}
