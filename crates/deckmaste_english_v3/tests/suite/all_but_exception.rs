use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Numeral, SurfaceCase, WordForm,
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

fn cardinal(notation: Numeral) -> Reading {
    let head = Word {
        value: LexicalReading::Numeral {
            value: 1,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    };
    if notation == Numeral::Cardinal {
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
    }
}

fn exception(quantity: Reading, degree: bool) -> Reading {
    Reading::ExceptionDeterminativePhrase {
        form: usize::from(degree),
        head: word("vocab:FloatedQuantifier/All"),
        exception: Box::new(Reading::QuantitativePrepositionPhrase {
            category: if degree {
                Category::ExceptionDegreePrepositionPhrase
            } else {
                Category::ExceptionPrepositionPhrase
            },
            form: 0,
            head: word("vocab:Preposition/But"),
            complement: Box::new(quantity),
        }),
    }
}

fn degree() -> Reading {
    Reading::DegreeDeterminativePhrase {
        form: 0,
        modifier: word("vocab:SingularDemonstrative/That"),
        head: word("vocab:Determinative/Many"),
        arithmetic_modifier: None,
    }
}

fn assert_exception(text: &str, category: Category, expected: &Reading) {
    let values = common::readings(text, category);
    assert!(!values.is_empty(), "{text}");
    for value in values {
        let mut observed = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ExceptionDeterminativePhrase { .. } = node {
                    observed.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(
            observed.as_slice(),
            std::slice::from_ref(expected),
            "{text}"
        );
    }
}

#[test]
fn independent_exception_phrases_keep_the_head_pp_and_both_roundtrip_laws() {
    for (expected, text) in [
        (
            exception(cardinal(Numeral::Arabic(false)), false),
            "all but 1",
        ),
        (exception(cardinal(Numeral::Cardinal), false), "all but one"),
        (exception(degree(), true), "all but that many"),
    ] {
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::QuantitativeDeterminer),
            BTreeSet::from([expected])
        );
    }
}

#[test]
fn prenominal_exceptions_are_whole_determiners() {
    // Soulgorger Orgg, Lily Bowen and Choice of Damnations.
    for (text, expected) in [
        (
            "all but 1 life",
            exception(cardinal(Numeral::Arabic(false)), false),
        ),
        (
            "all but one +1/+1 counter",
            exception(cardinal(Numeral::Cardinal), false),
        ),
        ("all but that many permanents", exception(degree(), true)),
    ] {
        assert_exception(text, Category::NounPhrase, &expected);
        for value in common::readings(text, Category::NounPhrase) {
            let Reading::QuantifiedNounPhrase { quantity, .. } = value else {
                panic!("exception must be the whole NP Determiner")
            };
            assert_eq!(*quantity, expected);
        }
    }
}

#[test]
fn partitives_keep_the_whole_exception_as_their_fused_head() {
    for (text, expected) in [
        (
            "all but 1 of that damage",
            exception(cardinal(Numeral::Arabic(false)), false),
        ),
        (
            "all but one of them",
            exception(cardinal(Numeral::Cardinal), false),
        ),
    ] {
        assert_exception(text, Category::NounPhrase, &expected);
        for value in common::readings(text, Category::NounPhrase) {
            let Reading::PartitiveNounPhrase { quantity, .. } = value else {
                panic!("exception must be the partitive head")
            };
            assert_eq!(*quantity, expected);
        }
    }
}

#[test]
fn witness_sentences_preserve_their_controls_independent_ambiguity() {
    for (card, text, category, expected) in [
        (
            "Soulgorger Orgg",
            "When this creature enters, you lose all but 1 life.",
            Category::Document,
            exception(cardinal(Numeral::Arabic(false)), false),
        ),
        (
            "Temple Altisaur",
            "If a source would deal damage to another Dinosaur you control, prevent all but 1 of that damage.",
            Category::Document,
            exception(cardinal(Numeral::Arabic(false)), false),
        ),
        (
            "Choice of Damnations",
            "Target opponent chooses a number. You may have that player lose that much life. If you don't, that player sacrifices all but that many permanents of their choice.",
            Category::Document,
            exception(degree(), true),
        ),
        (
            "Lily Bowen, Raging Grandma",
            "At the beginning of your upkeep, double the number of +1/+1 counters on Lily Bowen if its power is 16 or less. Otherwise, remove all but one +1/+1 counter from it, then you gain 1 life for each +1/+1 counter removed this way.",
            Category::Document,
            exception(cardinal(Numeral::Cardinal), false),
        ),
        (
            "Aladdin's Lamp",
            "put all but one of them on the bottom of your library in a random order, then draw a card",
            Category::BarePredicate,
            exception(cardinal(Numeral::Cardinal), false),
        ),
    ] {
        let values = common::readings_on_card(card, text, category);
        let control = text.replace("all but ", "");
        let mut controls = common::readings_on_card(card, &control, category);
        if card == "Lily Bowen, Raging Grandma" {
            // Bare one has both a lexical Determinative and a Cardinal route.
            // AB1 selects the Cardinal quantity within the exception PP;
            // compare independent ambiguity against that control route.
            let cardinal_quantity = Reading::CardinalDeterminer {
                form: 0,
                value: Box::new(Reading::Cardinal {
                    form: 0,
                    modifier: None,
                    head: Word {
                        value: LexicalReading::Numeral {
                            value: 1,
                            notation: Numeral::Cardinal,
                            capitalization: SurfaceCase::Declared,
                        },
                        frame: None,
                        countability: None,
                    },
                }),
            };
            controls.retain(|value| {
                let mut cardinal_routes = Vec::new();
                value
                    .visit(&mut |node| {
                        if let Reading::QuantifiedNounPhrase { quantity, .. } = node
                            && **quantity == cardinal_quantity
                        {
                            cardinal_routes.push(node.clone());
                        }
                    })
                    .unwrap();
                !cardinal_routes.is_empty()
            });
        }
        assert!(!values.is_empty(), "{card}");
        assert_eq!(values.len(), controls.len(), "{card}: W9-A");
        for value in values {
            let mut observed = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::ExceptionDeterminativePhrase { .. } = node {
                        observed.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                observed.as_slice(),
                std::slice::from_ref(&expected),
                "{card}"
            );
        }
    }
}

#[test]
fn declared_licences_exclude_coordinator_and_floated_quantifier_twins() {
    for text in [
        "each but one +1/+1 counter",
        "every but one +1/+1 counter",
        "all but one +1/+1 counters",
        "all but that many permanent",
        "all but the bottom card",
        "all but one life",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
    assert!(common::readings("but 1 of that damage", Category::PrepositionPhrase).is_empty());
    assert!(
        common::readings("but 1", Category::ExceptionPrepositionPhrase).contains(
            &Reading::QuantitativePrepositionPhrase {
                category: Category::ExceptionPrepositionPhrase,
                form: 0,
                head: word("vocab:Preposition/But"),
                complement: Box::new(cardinal(Numeral::Arabic(false))),
            }
        )
    );
    let mut wrong_head = exception(cardinal(Numeral::Cardinal), false);
    if let Reading::ExceptionDeterminativePhrase { head, .. } = &mut wrong_head {
        *head = word("vocab:Determinative/Every");
    }
    assert!(wrong_head.admit(common::lexicon()).is_err());
}

#[test]
fn ordinary_determination_partitives_and_prevention_controls_keep_reading_values() {
    // Attested constituents of the ticket's controls.
    for (text, category) in [
        (
            "If a source would deal damage to you, prevent that damage.",
            Category::Document,
        ),
        (
            "If a source would deal damage to you, prevent 1 damage.",
            Category::Document,
        ),
        ("Destroy one of them.", Category::Document),
        (
            "Whenever this creature attacks, tap all creatures defending player controls.",
            Category::Document,
        ),
        (
            "At the beginning of your upkeep, if you have 40 or more life, you win the game.",
            Category::Document,
        ),
    ] {
        let values = common::readings(text, category);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut exceptions = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::ExceptionDeterminativePhrase { .. } = node {
                        exceptions.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(exceptions, Vec::<Reading>::new(), "{text}");
        }
    }
}
