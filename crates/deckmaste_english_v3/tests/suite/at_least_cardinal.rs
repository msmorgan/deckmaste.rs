use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

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

fn modifier() -> Reading {
    Reading::QuantitativePrepositionPhrase {
        category: Category::CardinalModifier,
        form: 0,
        head: word("vocab:Preposition/At"),
        complement: Box::new(Reading::QuantitativeAdjectiveComplement {
            form: 0,
            head: word("vocab:Adjective/Least"),
        }),
    }
}

fn bounded(value: i32, notation: Numeral) -> Reading {
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

fn witness(card: &str, text: &str, count: i32, expected_readings: usize) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert_eq!(values.len(), expected_readings, "{card}");
    for value in values {
        let mut quantities = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::Cardinal {
                    modifier: Some(_), ..
                } = node
                {
                    quantities.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(quantities, [bounded(count, Numeral::Cardinal)], "{card}");
    }
}

#[test]
fn sokka_keeps_the_bounded_cardinal_inside_its_coordinated_subject() {
    witness(
        "Sokka, Lateral Strategist",
        "Whenever Sokka and at least one other creature attack, draw a card.",
        1,
        1,
    );
}

#[test]
fn svyelun_keeps_the_bounded_cardinal_in_determiner_function() {
    witness(
        "Svyelun of Sea and Sky",
        "Svyelun has indestructible as long as you control at least two other Merfolk.",
        2,
        1,
    );
}

#[test]
fn renegade_reaper_keeps_singular_agreement_under_the_bound() {
    witness(
        "Renegade Reaper",
        "If at least one Angel card is milled this way, you gain 4 life.",
        1,
        2,
    );
}

#[test]
fn synchronized_eviction_keeps_the_relative_clause_on_the_nominal() {
    witness(
        "Synchronized Eviction",
        "This spell costs {2} less to cast if you control at least two creatures that share a creature type.",
        2,
        2,
    );
}

#[test]
fn glimmer_lens_keeps_the_bounded_cardinal_inside_its_coordinated_subject() {
    witness(
        "Glimmer Lens",
        "Whenever equipped creature and at least one other creature attack, draw a card.",
        1,
        1,
    );
}

#[test]
fn bound_agreement_follows_the_cardinal() {
    // Sokka and Svyelun supply the positive constituents.
    for text in ["at least one other creature", "at least two other Merfolk"] {
        assert_eq!(
            common::readings(text, Category::NounPhrase).len(),
            1,
            "{text}"
        );
    }
    for text in [
        "at least one other creatures",
        "at least two other creature",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn bounded_cardinals_compose_with_both_measure_consumers() {
    // Righteous Valkyrie and The Emperor of Palamecia supply these
    // constituents; the comparative tail of the former belongs to its
    // separate ticket.
    for (text, category, expected) in [
        (
            "at least 7 life",
            Category::MeasurePhrase,
            bounded(7, Numeral::Arabic(false)),
        ),
        (
            "at least four mana",
            Category::Cardinal,
            bounded(4, Numeral::Cardinal),
        ),
    ] {
        let values = common::readings(text, Category::NounPhrase);
        assert_eq!(values.len(), 1, "{text}");
        for value in values {
            let mut quantities = Vec::new();
            value
                .visit(&mut |node| {
                    if node.category() == category {
                        quantities.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                quantities.as_slice(),
                std::slice::from_ref(&expected),
                "{text}"
            );
        }
    }
}

#[test]
fn independently_constructed_bounds_preserve_both_roundtrip_laws_and_identity() {
    for (value, text, category) in [
        (
            bounded(1, Numeral::Cardinal),
            "at least one",
            Category::Cardinal,
        ),
        (
            bounded(2, Numeral::Cardinal),
            "at least two",
            Category::Cardinal,
        ),
        (
            bounded(7, Numeral::Arabic(false)),
            "at least 7",
            Category::MeasurePhrase,
        ),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let observed = common::readings(text, category);
        assert_eq!(observed, BTreeSet::from([value.clone()]));
        let mut expected_words = Vec::new();
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        let mut actual_words = Vec::new();
        observed
            .first()
            .unwrap()
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);
    }
}

#[test]
fn the_bound_is_neither_a_free_adjunct_nor_an_np_complement() {
    for text in ["at least one other creature", "at least two other Merfolk"] {
        assert!(
            common::readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
    // At most is unattested and is not licensed by the declared feature.
    assert!(common::readings("at most two", Category::Cardinal).is_empty());
    assert!(common::readings("at least", Category::QuantitativeDeterminer).is_empty());
}
