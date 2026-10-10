use std::collections::BTreeSet;

use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, SurfaceCase, WordForm};

pub(super) fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == WordForm::Invariant
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: None,
    }
}

fn adjunct() -> Reading {
    Reading::VerblessClauseComplementPreposition {
        form: 0,
        head: word("vocab:Preposition/While"),
        complement: Box::new(Reading::VerblessClause {
            form: 0,
            predicate: word("lexeme:designation/saddled"),
        }),
    }
}

fn add_attack_adjunct(mut control: Reading) -> Reading {
    let Reading::Sentence { clause, .. } = &mut control else {
        panic!("expected Sentence");
    };
    let Reading::InitialPreposition { dependent, .. } = clause.as_mut() else {
        panic!("expected trigger Adjunct");
    };
    let Reading::ClauseComplementPreposition { complement, .. } = dependent.as_mut() else {
        panic!("expected finite trigger Complement");
    };
    let Reading::FiniteClause { predicate, .. } = complement.as_mut() else {
        panic!("expected trigger Clause");
    };
    let Reading::SelectedPredicate { head, .. } = predicate.as_ref() else {
        panic!("expected attack Predicate");
    };
    let LexicalReading::Word(value) = &head.value else {
        panic!("expected verb Word");
    };
    assert_eq!(value.lexeme, "core-verb:Attack");
    **predicate = Reading::PrepositionPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: predicate.clone(),
        modifier: Box::new(adjunct()),
    };
    control
}

#[test]
fn witnesses_attach_one_verbless_route_to_the_attack_predicate() {
    for (card, text) in [
        (
            "Gilded Ghoda",
            "Whenever this creature attacks while saddled, create a Treasure token.",
        ),
        (
            "Alacrian Jaguar",
            "Whenever this creature attacks while saddled, it gets +2/+2 until end of turn.",
        ),
        (
            "District Mascot",
            "Whenever this creature attacks while saddled, put a +1/+1 counter on it.",
        ),
        (
            "Dracosaur Auxiliary",
            "Whenever this creature attacks while saddled, it deals 2 damage to any target.",
        ),
    ] {
        let controls = common::readings_on_card(
            card,
            &text.replace(" while saddled", ""),
            Category::Sentence,
        );
        assert!(!controls.is_empty(), "missing control for {card}");
        let expected: BTreeSet<_> = controls.into_iter().map(add_attack_adjunct).collect();
        let actual = common::readings_on_card(card, text, Category::Sentence);
        assert_eq!(
            actual, expected,
            "incorrect attachment or duplicate route for {card}"
        );
        for reading in actual {
            reading.admit(common::lexicon()).unwrap();
        }
    }
}

#[test]
fn independent_complement_keeps_owner_roundtrips_and_lexical_licences() {
    let expected = adjunct();
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "while saddled"
    );
    assert_eq!(
        common::readings("while saddled", Category::PrepositionPhrase),
        BTreeSet::from([expected.clone()])
    );
    assert!(common::readings("creature while saddled", Category::NounPhrase).is_empty());
    assert!(common::readings("while saddled", Category::LocativeComplement).is_empty());
    // VW8 replaces the speculative-member exclusion with class permission.
    let expected_tapped = common::readings("It has flying.", Category::Sentence)
        .into_iter()
        .map(|control| {
            super::verbless_state_predicate_class::add_predicate_adjunct(
                control,
                "lexeme:keyword_action/tap/adjective",
            )
        })
        .collect();
    assert_eq!(
        common::readings("It has flying while tapped.", Category::Sentence),
        expected_tapped
    );
    for (owner, feature, value) in [
        ("vocab:Preposition/While", "VerblessClauseComplement", "No"),
        ("lexeme:designation/saddled", "VerblessPredicateUse", "No"),
        (
            "vocab:Preposition/While",
            "PrepositionFunctionLicence",
            "Modifier",
        ),
    ] {
        let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
        lexemes
            .iter_mut()
            .find(|entry| entry.id.as_str() == owner)
            .unwrap()
            .properties
            .features
            .insert(feature.into(), value.into());
        let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
        assert!(expected.admit(&lexicon).is_err(), "unchecked {feature}");
        assert!(
            common::readings_with_lexicon(&lexicon, "while saddled", Category::PrepositionPhrase)
                .is_empty(),
            "unchecked {feature}"
        );
    }
}

#[test]
fn finite_while_copular_saddled_and_if_able_readings_are_preserved() {
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|entry| entry.id.as_str() == "vocab:Preposition/While")
        .unwrap()
        .properties
        .features
        .insert("VerblessClauseComplement".into(), "No".into());
    lexemes
        .iter_mut()
        .find(|entry| entry.id.as_str() == "lexeme:designation/saddled")
        .unwrap()
        .properties
        .features
        .remove("VerblessPredicateUse");
    let parent_licences = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    for text in [
        "Whenever this creature attacks while it's saddled, it gets +2/+2 until end of turn.",
        "Whenever this creature attacks, draw a card.",
        "This creature is saddled.",
        "You may look at it while it's tapped.",
        "This creature attacks each combat if able.",
    ] {
        let before = common::readings_with_lexicon(&parent_licences, text, Category::Sentence);
        assert!(!before.is_empty(), "missing control: {text}");
        assert_eq!(
            common::readings(text, Category::Sentence),
            before,
            "changed control: {text}"
        );
    }
}
