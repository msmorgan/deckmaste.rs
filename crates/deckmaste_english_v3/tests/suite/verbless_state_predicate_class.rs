use std::collections::BTreeSet;

use crate::common;
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading};

use super::verbless_while_adjunct::word;

fn adjunct(preposition: &str, owner: &str) -> Reading {
    let entry = &common::lexicon().lexemes()[owner];
    let mut predicate = word(owner);
    let participial = entry
        .properties
        .features
        .get("AdjectiveOrigin")
        .is_some_and(|origin| origin == "Participial");
    let form = if participial && entry.properties.frames.is_empty() {
        1
    } else if participial {
        predicate.frame = Some(
            entry
                .properties
                .frames
                .iter()
                .position(|frame| frame.kind == "Predicate" && frame.items.is_empty())
                .unwrap(),
        );
        2
    } else {
        0
    };
    Reading::VerblessClauseComplementPreposition {
        form: 0,
        head: word(preposition),
        complement: Box::new(Reading::VerblessClause { form, predicate }),
    }
}

fn finite_predicate(reading: &mut Reading) -> &mut Reading {
    let Reading::Sentence { clause, .. } = reading else {
        panic!("expected Sentence");
    };
    let Reading::Declarative { clause, .. } = clause.as_mut() else {
        panic!("expected Declarative Clause");
    };
    let Reading::FiniteClause { predicate, .. } = clause.as_mut() else {
        panic!("expected Finite Clause");
    };
    predicate.as_mut()
}

pub(super) fn add_predicate_adjunct(mut control: Reading, owner: &str) -> Reading {
    let predicate = finite_predicate(&mut control);
    *predicate = Reading::PrepositionPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: Box::new(predicate.clone()),
        modifier: Box::new(adjunct("vocab:Preposition/While", owner)),
    };
    control
}

#[test]
fn state_derivations_supply_one_verbless_route_and_preserve_control_ambiguity() {
    // VW7–VW10 authorize hypothetical class members, independently of attestation.
    for (surface, owner) in [
        ("saddled", "lexeme:designation/saddled"),
        ("prepared", "lexeme:designation/prepared"),
        ("tapped", "lexeme:keyword_action/tap/adjective"),
        ("untapped", "lexeme:keyword_action/untap/adjective"),
        ("blocked", "core-verb:Block/adjective"),
        ("crewed", "lexeme:keyword_ability/crew/verb/adjective"),
        ("exiled", "lexeme:keyword_action/exile/adjective"),
        ("equipped", "lexeme:keyword_ability/equip/adjective"),
        ("enchanted", "lexeme:keyword_ability/enchant/adjective"),
        ("suspended", "lexeme:keyword_ability/suspend/adjective"),
        ("haunted", "lexeme:keyword_ability/haunt/adjective"),
    ] {
        let text = format!("This creature attacks while {surface}.");
        let controls = common::readings("This creature attacks.", Category::Sentence);
        assert!(!controls.is_empty());
        let expected: BTreeSet<_> = controls
            .into_iter()
            .map(|control| add_predicate_adjunct(control, owner))
            .collect();
        assert_eq!(common::readings(&text, Category::Sentence), expected);
        for reading in expected {
            reading.admit(common::lexicon()).unwrap();
            assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
        }
    }
    let expected = common::readings("This creature gets +1/+1.", Category::Sentence)
        .into_iter()
        .map(|control| add_predicate_adjunct(control, "lexeme:designation/saddled"))
        .collect();
    assert_eq!(
        common::readings(
            "This creature gets +1/+1 while saddled.",
            Category::Sentence
        ),
        expected
    );
}

#[test]
fn independently_composed_state_complements_keep_existing_owners_and_both_laws() {
    for (surface, owner) in [
        ("saddled", "lexeme:designation/saddled"),
        ("tapped", "lexeme:keyword_action/tap/adjective"),
        ("crewed", "lexeme:keyword_ability/crew/verb/adjective"),
        ("equipped", "lexeme:keyword_ability/equip/adjective"),
        ("haunted", "lexeme:keyword_ability/haunt/adjective"),
    ] {
        for (preposition, text) in [
            ("vocab:Preposition/While", format!("while {surface}")),
            ("vocab:Preposition/If", format!("if {surface}")),
        ] {
            let expected = adjunct(preposition, owner);
            expected.admit(common::lexicon()).unwrap();
            assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
            assert_eq!(
                common::readings(&text, Category::PrepositionPhrase),
                BTreeSet::from([expected])
            );
        }
    }
    assert!(common::readings("while legendary", Category::PrepositionPhrase).is_empty());
}

#[test]
fn haunted_adds_the_declared_state_complement_beside_the_preserved_passive() {
    let parent = deckmaste_lexical::Lexicon::new(
        common::lexicon()
            .lexemes()
            .values()
            .filter(|entry| entry.id != "lexeme:keyword_ability/haunt/adjective")
            .cloned(),
    )
    .unwrap();
    let text = "This creature is haunted.";
    let mut expected = common::readings_with_lexicon(&parent, text, Category::Sentence);
    assert!(!expected.is_empty(), "missing existing passive");
    for mut control in common::readings("This creature is equipped.", Category::Sentence) {
        let Reading::SelectedPredicate { complements, .. } = finite_predicate(&mut control) else {
            panic!("expected copular Predicate");
        };
        let [FrameValue::Argument(complement)] = complements.as_mut_slice() else {
            panic!("expected selected predicative Complement");
        };
        **complement = Reading::AdjectivalComplement {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word("lexeme:keyword_ability/haunt/adjective"),
            }),
        };
        control.admit(common::lexicon()).unwrap();
        assert_eq!(control.realize(common::lexicon()).unwrap(), text);
        expected.insert(control);
    }
    assert_eq!(common::readings(text, Category::Sentence), expected);
}
