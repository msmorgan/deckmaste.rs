use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, Numeral, SurfaceCase, WordForm};

use crate::common;

fn word(owner: &str, form: WordForm, countability: Option<bool>) -> Word {
    let mut value = common::lexicon()
        .values()
        .find(|value| value.lexeme.as_str() == owner && value.form == form && value.variant == 0)
        .unwrap()
        .clone();
    value.capitalization = SurfaceCase::Declared;
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability,
    }
}

fn invariant(owner: &str) -> Word {
    word(owner, WordForm::Invariant, None)
}

fn numeral(value: i32, notation: Numeral) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::Cardinal {
        form: 0,
        modifier: None,
        head: numeral(value, Numeral::Cardinal),
    }
}

fn comparative(value: i32) -> Reading {
    Reading::ComparativeQuantityDeterminer {
        form: 0,
        value: Box::new(Reading::NumeralComparativeCoordination {
            form: 0,
            left: Box::new(Reading::CardinalQuantityConjunct {
                form: 0,
                value: Box::new(cardinal(value)),
            }),
            marker: invariant("vocab:Coordinator/Or"),
            right: invariant("vocab:ComparativeQuantifier/More"),
        }),
    }
}

fn bounded(value: i32) -> Reading {
    Reading::PrepositionDeterminer {
        form: 0,
        value: Box::new(Reading::QuantitativePrepositionPhrase {
            category: Category::QuantitativePrepositionPhrase,
            form: 0,
            head: invariant("vocab:Preposition/Up"),
            complement: Box::new(Reading::QuantitativePrepositionPhrase {
                category: Category::CardinalPrepositionPhrase,
                form: 0,
                head: invariant("vocab:Preposition/To"),
                complement: Box::new(cardinal(value)),
            }),
        }),
    }
}

fn degree(owner: &str) -> Reading {
    Reading::DegreeDeterminativePhrase {
        form: 0,
        modifier: invariant("vocab:SingularDemonstrative/That"),
        head: invariant(owner),
    }
}

fn determined(owner: &str, determiner: &str, form: WordForm, countable: bool) -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant(determiner),
        head: Box::new(Reading::Noun {
            form: 0,
            head: word(owner, form, Some(countable)),
        }),
    }
}

fn cards() -> Reading {
    determined(
        "lexeme:CommonNoun/Card",
        "vocab:Determinative/Those",
        WordForm::Plural,
        true,
    )
}

fn tokens() -> Reading {
    determined(
        "lexeme:CommonNoun/Token",
        "vocab:Determinative/Those",
        WordForm::Plural,
        true,
    )
}

fn damage() -> Reading {
    determined(
        "lexeme:CommonNoun/Damage",
        "vocab:SingularDemonstrative/That",
        WordForm::Singular,
        false,
    )
}

fn them() -> Reading {
    Reading::AccusativePronoun {
        form: 0,
        head: invariant("vocab:ObjectPronoun/Them"),
    }
}

fn partitive(quantity: Reading, oblique: Reading, form: usize) -> Reading {
    Reading::PartitiveNounPhrase {
        form,
        quantity: Box::new(quantity),
        complement: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: invariant("vocab:Preposition/Of"),
            complement: Box::new(Reading::CasePhrase {
                form: 0,
                category: Category::AccusativePhrase,
                head: Box::new(oblique),
            }),
        }),
        modifiers: vec![],
    }
}

fn numeric(value: i32) -> Reading {
    Reading::NumericQuantityConjunct {
        form: 0,
        head: numeral(value, Numeral::Arabic(false)),
    }
}

fn assert_laws(value: Reading, text: &str) {
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert_eq!(common::noun_phrase_readings(text), BTreeSet::from([value]));
}

#[test]
fn independent_quantity_heads_keep_one_route_and_both_roundtrip_laws() {
    // Scalpelexis, The Mana Rig, Parallel Lives, Urza's Armor and Nelly Borca.
    for (value, text) in [
        (
            partitive(comparative(2), cards(), 0),
            "two or more of those cards",
        ),
        (
            partitive(
                comparative(1),
                Reading::PossessiveNounPhrase {
                    form: 0,
                    possessor: invariant("vocab:PossessiveDeterminerPronoun/Your"),
                    head: Box::new(Reading::Noun {
                        form: 0,
                        head: word("lexeme:CommonNoun/Opponent", WordForm::Plural, Some(true)),
                    }),
                },
                0,
            ),
            "one or more of your opponents",
        ),
        (partitive(bounded(2), them(), 0), "up to two of them"),
        (
            partitive(degree("vocab:Determinative/Many"), tokens(), 0),
            "that many of those tokens",
        ),
        (partitive(numeric(1), damage(), 1), "1 of that damage"),
    ] {
        assert_laws(value, text);
    }
}

#[test]
fn multiplier_remains_external_to_the_degree_headed_partitive() {
    // Parallel Lives: twice modifies the complete partitive host.
    assert_laws(
        Reading::PredeterminerNounPhrase {
            form: 0,
            modifier: invariant("vocab:FrequencyAdverb/Twice"),
            head: Box::new(partitive(degree("vocab:Determinative/Many"), tokens(), 0)),
        },
        "twice that many of those tokens",
    );
}

#[test]
fn bare_cardinal_partitives_use_the_same_quantitative_head_route() {
    // Commune with Evil and the ticket's bare-cardinal control class.
    for (value, oblique, text) in [
        (1, them(), "one of them"),
        (2, cards(), "two of those cards"),
    ] {
        assert_laws(
            partitive(
                Reading::CardinalDeterminer {
                    form: 0,
                    value: Box::new(cardinal(value)),
                },
                oblique,
                0,
            ),
            text,
        );
    }
    // The superseded Cardinal slot and its QuantityConjunct projection
    // cannot produce twin analyses of the attested one-of mass constituent.
    for (quantity, form) in [
        (cardinal(1), 0),
        (
            Reading::CardinalQuantityConjunct {
                form: 0,
                value: Box::new(cardinal(1)),
            },
            1,
        ),
    ] {
        let invalid = partitive(quantity, damage(), form);
        assert!(invalid.admit(common::lexicon()).is_err());
        assert!(invalid.realize(common::lexicon()).is_err());
    }
}

fn witness(card: &str, text: &str, expected: &Reading) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut partitives = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::PartitiveNounPhrase { .. } = node {
                    partitives.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(
            partitives.as_slice(),
            std::slice::from_ref(expected),
            "{card}: quantity must be the fused head and of-PP its Complement"
        );
    }
}

#[test]
fn urzas_armor_has_a_numeric_head_and_noncount_partitive_oblique() {
    witness(
        "Urza's Armor",
        "If a source would deal damage to you, prevent 1 of that damage.",
        &partitive(numeric(1), damage(), 1),
    );
}

#[test]
fn scalpelexis_keeps_the_quantity_coordination_inside_the_fused_head() {
    witness(
        "Scalpelexis",
        "Whenever this creature deals combat damage to a player, that player exiles the top four cards of their library. If two or more of those cards have the same name, repeat this process.",
        &partitive(comparative(2), cards(), 0),
    );
}

#[test]
fn mangara_has_a_comparative_quantity_subject() {
    let creatures = determined(
        "lexeme:type/creature",
        "vocab:Determinative/Those",
        WordForm::Plural,
        true,
    );
    witness(
        "Mangara, the Diplomat",
        "Whenever an opponent attacks with creatures, if two or more of those creatures are attacking you and/or planeswalkers you control, draw a card.",
        &partitive(comparative(2), creatures, 0),
    );
}

#[test]
fn mana_rig_has_a_quantitative_preposition_phrase_as_fused_head() {
    witness(
        "The Mana Rig",
        "{X}{X}{X}, {T}: Look at the top X cards of your library. Put up to two of them into your hand and the rest on the bottom of your library in a random order.",
        &partitive(bounded(2), them(), 0),
    );
}

#[test]
fn parallel_lives_has_one_degree_head_under_the_existing_predeterminer() {
    witness(
        "Parallel Lives",
        "If an effect would create one or more tokens under your control, it creates twice that many of those tokens instead.",
        &partitive(degree("vocab:Determinative/Many"), tokens(), 0),
    );
}
