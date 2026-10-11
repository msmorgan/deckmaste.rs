use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Numeral, SurfaceCase, WordForm,
};

use crate::common;

const HARDENED: &str = "If one or more +1/+1 counters would be put on a creature you control, that many plus one +1/+1 counters are put on it instead.";
const CONCLAVE: &str = "If one or more +1/+1 counters would be put on a creature you control, that many plus one +1/+1 counters are put on that creature instead.";

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

fn arithmetic_modifier(owner: &str, value: i32, notation: Numeral) -> Reading {
    let head = Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    };
    let complement = if notation == Numeral::Cardinal {
        Reading::CardinalQuantityConjunct {
            form: 0,
            value: Box::new(Reading::Cardinal {
                form: 0,
                modifier: None,
                head,
            }),
        }
    } else {
        Reading::NumericQuantityConjunct { form: 0, head }
    };
    Reading::QuantitativePrepositionPhrase {
        category: Category::ArithmeticModifier,
        form: 0,
        head: word(owner),
        complement: Box::new(complement),
    }
}

fn degree(modifier: Option<Reading>) -> Reading {
    Reading::DegreeDeterminativePhrase {
        form: 0,
        modifier: word("vocab:SingularDemonstrative/That"),
        head: word("vocab:Determinative/Many"),
        arithmetic_modifier: modifier.map(Box::new),
    }
}

#[test]
fn replacement_clauses_keep_the_pp_inside_the_degree_determiner() {
    let expected = degree(Some(arithmetic_modifier(
        "vocab:Preposition/Plus",
        1,
        Numeral::Cardinal,
    )));
    for (card, text) in [("Hardened Scales", HARDENED), ("Conclave Mentor", CONCLAVE)] {
        let values = common::readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        let control = text.replace(" plus one", "");
        let controls = common::readings_on_card(card, &control, Category::Document);
        // W9-A: preserve the surrounding ambiguity, with one arithmetic route.
        assert_eq!(values.len(), controls.len(), "{card}");
        for value in values {
            let mut degrees = Vec::new();
            let mut arithmetic_measures = Vec::new();
            value
                .visit(&mut |node| match node {
                    Reading::DegreeDeterminativePhrase { .. } => degrees.push(node.clone()),
                    Reading::ArithmeticMeasure { .. } => arithmetic_measures.push(node.clone()),
                    _ => {}
                })
                .unwrap();
            assert_eq!(degrees.as_slice(), std::slice::from_ref(&expected), "{card}");
            assert!(arithmetic_measures.is_empty(), "{card}");
        }
        common::assert_constituent_occurrences_on_card(
            card,
            text,
            Category::Document,
            &[
                (Category::NounPhrase, "that many plus one +1/+1 counters", 0),
                (Category::QuantitativeDeterminer, "that many plus one", 0),
                (Category::ArithmeticModifier, " plus one", 0),
                (Category::Nominal, "+1/+1 counters", 1),
            ],
        );
    }
}

#[test]
fn the_quantified_noun_retains_plural_agreement_and_its_head() {
    let text = "that many plus one +1/+1 counters";
    let values = common::readings(text, Category::NounPhrase);
    assert!(!values.is_empty());
    for value in values {
        let Reading::QuantifiedNounPhrase { quantity, head, .. } = value else {
            panic!("arithmetic changed the quantified noun phrase family")
        };
        assert_eq!(
            *quantity,
            degree(Some(arithmetic_modifier(
                "vocab:Preposition/Plus",
                1,
                Numeral::Cardinal
            )))
        );
        assert_eq!(head.realize(common::lexicon()).unwrap(), "+1/+1 counters");
    }
    for text in [
        "that many plus one +1/+1 counter",
        "that many plus one life",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
    assert!(
        common::readings("that many plus one +1/+1 counters", Category::MeasurePhrase).is_empty()
    );
}

#[test]
fn independently_built_determiner_and_modifier_preserve_both_roundtrip_laws() {
    let pp = arithmetic_modifier("vocab:Preposition/Plus", 1, Numeral::Cardinal);
    let value = degree(Some(pp.clone()));
    for (value, text, category) in [
        (pp, " plus one", Category::ArithmeticModifier),
        (
            value,
            "that many plus one",
            Category::QuantitativeDeterminer,
        ),
        (degree(None), "that many", Category::QuantitativeDeterminer),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        assert_eq!(common::readings(text, category), BTreeSet::from([value]));
    }
    let invalid = degree(Some(arithmetic_modifier(
        "vocab:Preposition/During",
        1,
        Numeral::Cardinal,
    )));
    assert!(invalid.admit(common::lexicon()).is_err());
    assert!(invalid.realize(common::lexicon()).is_err());
}

#[test]
fn cardinal_complements_retain_spelled_digit_and_subtraction_structure() {
    // PPs attested in Hardened Scales, The Water Crystal, Torbran and Dark Deal.
    for (owner, value, notation, text) in [
        ("vocab:Preposition/Plus", 1, Numeral::Cardinal, " plus one"),
        ("vocab:Preposition/Plus", 4, Numeral::Cardinal, " plus four"),
        (
            "vocab:Preposition/Plus",
            2,
            Numeral::Arabic(false),
            " plus 2",
        ),
        (
            "vocab:Preposition/Minus",
            1,
            Numeral::Cardinal,
            " minus one",
        ),
    ] {
        let expected = arithmetic_modifier(owner, value, notation);
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::ArithmeticModifier),
            BTreeSet::from([expected])
        );
    }
    assert!(common::readings(" plus X", Category::ArithmeticModifier).is_empty());
}

#[test]
fn scalar_arithmetic_and_multiplier_controls_keep_their_existing_routes() {
    let scalar = "Its power is 2 plus the number of Swamps your opponents control.";
    let values = common::readings(scalar, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut measures = Vec::new();
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ArithmeticMeasure { .. } = node {
                    measures.push(node.clone());
                }
                if node.category() == Category::ArithmeticModifier {
                    modifiers.push(node.clone());
                }
            })
            .unwrap();
        assert_ne!(measures, [] as [Reading; 0]);
        assert_eq!(modifiers, [] as [Reading; 0]);
    }
    common::assert_constituents(
        scalar,
        Category::Document,
        &[(
            Category::MeasurePhrase,
            "2 plus the number of Swamps your opponents control",
        )],
    );
    let twice = HARDENED.replace("that many plus one", "twice that many");
    let values = common::readings(&twice, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut multipliers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::PredeterminerNounPhrase { .. } = node {
                    multipliers.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(multipliers.len(), 1);
        assert_eq!(
            multipliers[0].realize(common::lexicon()).unwrap(),
            "twice that many +1/+1 counters"
        );
    }
}
