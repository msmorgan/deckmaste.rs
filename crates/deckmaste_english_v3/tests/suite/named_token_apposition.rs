use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{LexicalReading, SurfaceCase, WordForm};

use crate::common;

fn supplement_value(anchor: Reading, supplement: Reading) -> Reading {
    Reading::AscriptiveNounPhraseSupplement {
        form: 0,
        anchor: Box::new(anchor),
        supplement: Box::new(supplement),
    }
}

fn create_objects(value: &Reading) -> Vec<Reading> {
    let mut objects = Vec::new();
    value
        .visit(&mut |node| {
            if let Reading::SelectedPredicate {
                head, complements, ..
            } = node
                && let LexicalReading::Word(word) = &head.value
                && word.lexeme.as_str() == "lexeme:keyword_action/create"
            {
                let [FrameValue::Argument(object)] = complements.as_slice() else {
                    panic!("create must select one Object");
                };
                let Reading::CasePhrase { category, head, .. } = object.as_ref() else {
                    panic!("Object must be a case-marked NP");
                };
                assert_eq!(*category, Category::AccusativePhrase);
                objects.push((**head).clone());
            }
        })
        .unwrap();
    objects
}

// Remove only this ticket's name-anchor wrapping. Keep the whole Sentence,
// every description bracket and any independently licensed VP Adjunct intact.
fn without_supplement(value: &Reading) -> Reading {
    let mut result = value.clone();
    match &mut result {
        Reading::AscriptiveNounPhraseSupplement { supplement, .. } => {
            return (**supplement).clone();
        }
        Reading::Sentence { clause, .. } | Reading::InitialPreposition { clause, .. } => {
            **clause = without_supplement(clause);
        }
        Reading::Imperative { predicate, .. } => **predicate = without_supplement(predicate),
        Reading::BarePredicate { head, .. }
        | Reading::PrepositionPredicate { head, .. }
        | Reading::CasePhrase { head, .. } => **head = without_supplement(head),
        Reading::SelectedPredicate { complements, .. } => {
            for complement in complements {
                if let FrameValue::Argument(argument) = complement {
                    **argument = without_supplement(argument);
                }
            }
        }
        _ => {}
    }
    result
}

fn witness(card: &str, text: &str, name: &str) {
    let anchors = common::readings_on_card(card, name, Category::NounPhrase);
    assert!(!anchors.is_empty(), "{card}: name");
    let control_text = text.replacen(&format!("{name}, "), "", 1);
    let controls = common::readings_on_card(card, &control_text, Category::Sentence);
    assert!(!controls.is_empty(), "{card}: control");
    let descriptions: BTreeSet<_> = controls.iter().flat_map(create_objects).collect();
    let expected: BTreeSet<_> = anchors
        .iter()
        .flat_map(|anchor| {
            descriptions
                .iter()
                .map(|description| supplement_value(anchor.clone(), description.clone()))
        })
        .collect();
    let values = common::readings_on_card(card, text, Category::Sentence);
    // NA7 / W9-A: counts are related to the control, never incidental constants.
    assert_eq!(
        values.len(),
        controls.len(),
        "{card}: witness equals control"
    );
    let mut normalized = BTreeSet::new();
    let mut observed = BTreeSet::new();
    for value in values {
        let objects = create_objects(&value);
        let mut supplements = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::AscriptiveNounPhraseSupplement {
                    anchor, supplement, ..
                } = node
                {
                    assert!(anchors.contains(anchor.as_ref()));
                    assert!(descriptions.contains(supplement.as_ref()));
                    let Reading::ProperNameNounPhrase { head, .. } = anchor.as_ref() else {
                        panic!("{card}: Supplement anchor must be a proper-name NP");
                    };
                    assert_eq!(head.category(), Category::Name);
                    assert!(expected.contains(node), "{card}: Object structure");
                    observed.insert(node.clone());
                    supplements.push(node.clone());
                }
            })
            .unwrap();
        // NA3 / NA7: the sole Object is the supplemented NP itself, never
        // an asyndetic coordinate, an extra Object or an outer NP Postmodifier.
        assert_eq!(objects, supplements, "{card}");
        assert_eq!(objects.len(), 1, "{card}: one Object and one Supplement");
        value.admit(common::lexicon()).unwrap();
        assert!(
            normalized.insert(without_supplement(&value)),
            "{card}: no supplementation twin"
        );
    }
    assert_eq!(
        observed, expected,
        "{card}: all description Readings survive"
    );
    assert_eq!(
        normalized, controls,
        "{card}: every control shape survives unchanged"
    );
}

#[test]
fn token_descriptions_supplement_the_name_inside_the_object() {
    for (card, text, name) in [
        (
            "Rinoa Heartilly",
            "When Rinoa Heartilly enters, create Angelo, a legendary 1/1 green and white Dog creature token.",
            "Angelo",
        ),
        (
            "Minsc, Beloved Ranger",
            "Create Boo, a legendary 1/1 red Hamster creature token with trample and haste.",
            "Boo",
        ),
        (
            "Stangg",
            "When Stangg enters, create Stangg Twin, a legendary 3/4 red and green Human Warrior creature token.",
            "Stangg Twin",
        ),
        (
            "Tamiyo, Compleated Sage",
            "Create Tamiyo's Notebook, a legendary colorless Book artifact token.",
            "Tamiyo's Notebook",
        ),
        (
            "Tuktuk the Explorer",
            "When Tuktuk the Explorer dies, create Tuktuk the Returned, a legendary 5/5 colorless Goblin Golem artifact creature token.",
            "Tuktuk the Returned",
        ),
    ] {
        witness(card, text, name);
    }
}

#[test]
fn controls_keep_their_unsupplemented_structures() {
    for (card, text) in [
        ("Rinoa Heartilly", "Create Angelo."),
        (
            "Rinoa Heartilly",
            "Create a legendary 1/1 green and white Dog creature token named Angelo.",
        ),
        (
            "Rinoa Heartilly",
            "Create a legendary 2/2 white and black Dog creature token.",
        ),
        ("Stangg", "Create a token named Stangg Twin."),
        (
            "Stangg",
            "Sacrifice Stangg when that token leaves the battlefield.",
        ),
    ] {
        let values = common::readings_on_card(card, text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            value
                .visit(&mut |node| {
                    if let Reading::AscriptiveNounPhraseSupplement { .. } = node {
                        panic!("{text}: control must retain its existing structure");
                    }
                })
                .unwrap();
        }
    }
}

#[test]
fn supplement_requires_a_name_anchor_and_indefinite_article() {
    for text in [
        "Angelo, the legendary 1/1 green and white Dog creature token",
        "a token, a legendary 1/1 green and white Dog creature token",
        "Angelo, target creature",
        "Angelo, creatures",
    ] {
        assert!(common::noun_phrase_readings(text).is_empty(), "{text}");
    }
}

#[test]
fn independently_composed_supplement_preserves_both_roundtrip_laws() {
    // Rinoa Heartilly's Object; construct the anchor and supplementation
    // independently, composing each existing description Reading intact.
    let mut name = common::lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == "lexeme:Name/Angelo"
                && value.form == WordForm::Invariant
                && value.variant == 0
        })
        .unwrap()
        .clone();
    name.capitalization = SurfaceCase::Declared;
    let anchor = Reading::ProperNameNounPhrase {
        form: 0,
        head: Box::new(Reading::CatalogName {
            form: 0,
            head: Word {
                value: LexicalReading::Word(name),
                frame: None,
                countability: None,
            },
        }),
    };
    let text = "Angelo, a legendary 1/1 green and white Dog creature token";
    let expected: BTreeSet<_> =
        common::noun_phrase_readings("a legendary 1/1 green and white Dog creature token")
            .into_iter()
            .map(|description| supplement_value(anchor.clone(), description))
            .collect();
    assert!(!expected.is_empty());
    for value in &expected {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
    assert_eq!(common::noun_phrase_readings(text), expected);
}
