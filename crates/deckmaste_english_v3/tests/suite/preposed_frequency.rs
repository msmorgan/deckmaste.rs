use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};

use crate::common::{lexicon, readings};

fn assert_preposed_frequency(value: &Reading, interval: &str) {
    let mut adjuncts = Vec::new();
    value
        .visit(&mut |node| {
            if let Reading::InitialAdverb {
                modifier, clause, ..
            } = node
                && modifier.realize(lexicon()).unwrap() == "Once"
            {
                let Reading::Adverb { head, .. } = &**modifier else {
                    panic!("frequency must retain its Adverb category")
                };
                let LexicalReading::Word(word) = &head.value else {
                    panic!("frequency must retain its lexical identity")
                };
                assert_eq!(word.lexeme.as_str(), "vocab:FrequencyAdverb/Once");
                let Reading::InitialPreposition {
                    dependent, clause, ..
                } = &**clause
                else {
                    panic!("the interval must be the next preposed Adjunct")
                };
                assert_eq!(dependent.realize(lexicon()).unwrap(), interval);
                assert_eq!(
                    dependent.category(),
                    if interval.starts_with("during") {
                        Category::PrepositionPhrase
                    } else {
                        Category::NominalAdjunctPhrase
                    }
                );
                assert!(clause.realize(lexicon()).unwrap().starts_with("you may "));
                adjuncts.push((modifier.clone(), dependent.clone()));
            }
            if let Reading::FiniteClause { subject, .. } = node {
                assert_ne!(subject.realize(lexicon()).unwrap(), "each turn");
            }
        })
        .unwrap();
    assert_eq!(
        adjuncts.len(),
        1,
        "one frequency/interval structure per Reading"
    );
}

#[test]
fn witnesses_have_two_ordered_preposed_adjuncts() {
    for (text, interval) in [
        (
            "Once each turn, you may cast an instant or sorcery spell from the top of your library.",
            "each turn",
        ),
        (
            "Once each turn, you may pay {0} rather than pay the mana cost for a colorless spell you cast from your hand.",
            "each turn",
        ),
        (
            "Once each turn, you may pay {0} rather than pay the mana cost for a spell you cast with mana value X or less, where X is the number of time counters on this enchantment.",
            "each turn",
        ),
        (
            "Once during each of your turns, you may cast a permanent spell with mana value 2 or less from your graveyard.",
            "during each of your turns",
        ),
        (
            "Once during each of your turns, you may cast an instant or sorcery spell from your graveyard. If a spell cast this way would be put into your graveyard, exile it instead.",
            "during each of your turns",
        ),
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            assert_preposed_frequency(&value, interval);
        }
    }
}

fn once() -> Reading {
    Reading::Adverb {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:FrequencyAdverb/Once".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Initial,
            }),
            frame: None,
            countability: None,
        },
    }
}

#[test]
fn independent_adjunct_composition_preserves_all_readings_and_roundtrips() {
    // Johann and Lurrus supply the main Clauses and both interval Categories.
    for (interval, category, form, main_clause) in [
        (
            "each turn",
            Category::NominalAdjunctPhrase,
            2,
            "you may cast an instant or sorcery spell from the top of your library",
        ),
        (
            "during each of your turns",
            Category::PrepositionPhrase,
            0,
            "you may cast a permanent spell with mana value 2 or less from your graveyard",
        ),
    ] {
        let mut expected = BTreeSet::new();
        for dependent in readings(interval, category) {
            for clause in readings(main_clause, Category::Clause) {
                let value = Reading::InitialAdverb {
                    form: 0,
                    modifier: Box::new(once()),
                    clause: Box::new(Reading::InitialPreposition {
                        form,
                        dependent: Box::new(dependent.clone()),
                        clause: Box::new(clause),
                    }),
                };
                value.admit(lexicon()).unwrap();
                assert_eq!(
                    value.realize(lexicon()).unwrap(),
                    format!("Once {interval}, {main_clause}")
                );
                assert_preposed_frequency(&value, interval);
                expected.insert(value);
            }
        }
        assert!(!expected.is_empty());
        assert_eq!(
            readings(&format!("Once {interval}, {main_clause}"), Category::Clause),
            expected
        );
    }
    for text in [
        "once each turn",
        "once during each of your turns",
        "once a creature dies",
    ] {
        assert!(readings(text, Category::AdverbPhrase).is_empty(), "{text}");
    }
}

#[test]
fn final_frequency_retains_the_interval_over_the_adverb_predicate() {
    for (text, frequency) in [
        ("Draw a card once each turn.", "once"),
        ("Activate only once each turn.", "only once"),
        ("This ability triggers only once each turn.", "only once"),
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty());
        for value in values {
            let mut intervals = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::NominalAdjunctPredicate { modifier, head, .. } = node {
                        assert_eq!(modifier.realize(lexicon()).unwrap(), "each turn");
                        let Reading::AdverbPredicate { modifier, .. } = &**head else {
                            panic!("final interval must scope the frequency Adjunct")
                        };
                        assert_eq!(modifier.realize(lexicon()).unwrap(), frequency);
                        intervals.push(modifier.clone());
                    }
                })
                .unwrap();
            assert_eq!(intervals.len(), 1);
        }
    }
}
