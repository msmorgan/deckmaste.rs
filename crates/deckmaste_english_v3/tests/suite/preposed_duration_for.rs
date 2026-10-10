use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, SurfaceCase, WordForm};

use crate::common::{lexicon, readings};

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    let mut value = lexicon()
        .values()
        .find(|value| {
            value.lexeme == owner
                && value.form == WordForm::Invariant
                && value.capitalization == SurfaceCase::Declared
        })
        .unwrap()
        .clone();
    value.capitalization = capitalization;
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: None,
    }
}

fn duration(complement: Reading) -> Reading {
    Reading::AdverbComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/For", SurfaceCase::Initial),
        complement: Box::new(Reading::EquativeAdverb {
            form: 0,
            governor: invariant("vocab:Adverb/EquativeAs", SurfaceCase::Declared),
            head: invariant("vocab:Adverb/Long", SurfaceCase::Declared),
            marker: invariant("vocab:Preposition/As", SurfaceCase::Declared),
            complement: Box::new(complement),
        }),
    }
}

fn preposed_sentence(dependent: Reading, clause: Reading) -> Reading {
    Reading::Sentence {
        form: 0,
        clause: Box::new(Reading::InitialPreposition {
            form: 0,
            dependent: Box::new(dependent),
            clause: Box::new(clause),
        }),
    }
}

fn preposed_scopes(dependent: Reading, clause: Reading) -> Vec<Reading> {
    let mut values = vec![preposed_sentence(dependent.clone(), clause.clone())];
    // PD4 preserves the existing whole-Coordination / first-Clause scope pair.
    if let Reading::ClauseCoordination {
        form,
        left,
        coordinator,
        right,
    } = clause
    {
        values.push(Reading::Sentence {
            form: 0,
            clause: Box::new(Reading::ClauseCoordination {
                form,
                left: Box::new(Reading::InitialPreposition {
                    form: 0,
                    dependent: Box::new(dependent),
                    clause: left,
                }),
                coordinator,
                right,
            }),
        });
    }
    values
}

fn exact(text: &str, expected: &BTreeSet<Reading>) {
    assert!(!expected.is_empty());
    for value in expected {
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
    }
    assert_eq!(&readings(text, Category::Sentence), expected, "{text}");
}

#[test]
fn witnesses_have_one_duration_pp_with_the_licensed_preposed_clause_scopes() {
    for (card, condition, host) in [
        (
            "Prowl, Stoic Strategist",
            "that card remains exiled",
            "its owner may play it",
        ),
        (
            "Release to the Wind",
            "that card remains exiled",
            "its owner may cast it without paying its mana cost",
        ),
        (
            "Preacher",
            "this creature remains tapped",
            "gain control of target creature",
        ),
        (
            "Minas Morgul, Dark Fortress",
            "that creature has a shadow counter on it",
            "it's a Wraith in addition to its other types",
        ),
        (
            "Flameshape",
            "they remain exiled",
            "you may play them if you control a Wizard",
        ),
        (
            "Immovable Rod",
            "this artifact remains tapped",
            "another target permanent loses all abilities and can't attack or block",
        ),
    ] {
        let complements = readings(condition, Category::FiniteClause);
        let clauses = readings(host, Category::Clause);
        assert!(!complements.is_empty(), "{card}: duration Complement");
        assert!(!clauses.is_empty(), "{card}: main Clause");
        let expected = complements
            .into_iter()
            .flat_map(|complement| {
                clauses
                    .iter()
                    .cloned()
                    .flat_map(move |clause| preposed_scopes(duration(complement.clone()), clause))
            })
            .collect();
        exact(&format!("For as long as {condition}, {host}."), &expected);
    }
    // The same scope pair already belongs to the existing preposed Adjunct
    // construction. This control uses the Rod predicate to compare its roles.
    let host = "another target permanent loses all abilities and can't attack or block";
    let clauses = readings(host, Category::Clause);
    let expected = readings("Until end of turn", Category::PrepositionPhrase)
        .into_iter()
        .flat_map(|dependent| {
            clauses
                .iter()
                .cloned()
                .flat_map(move |clause| preposed_scopes(dependent.clone(), clause))
        })
        .collect();
    exact(&format!("Until end of turn, {host}."), &expected);
}

#[test]
fn conditional_controls_have_only_the_compound_preposition_analysis() {
    for (condition, host) in [
        ("that card remains exiled", "you may play it"),
        (
            "that creature has a shadow counter on it",
            "it's a Wraith in addition to its other types",
        ),
    ] {
        let clauses = readings(host, Category::Clause);
        let expected = readings(condition, Category::FiniteClause)
            .into_iter()
            .flat_map(|complement| {
                clauses.iter().cloned().map(move |clause| {
                    preposed_sentence(
                        Reading::ClauseComplementPreposition {
                            form: 0,
                            head: invariant("vocab:Preposition/AsLongAs", SurfaceCase::Initial),
                            complement: Box::new(complement.clone()),
                        },
                        clause,
                    )
                })
            })
            .collect();
        exact(&format!("As long as {condition}, {host}."), &expected);
    }
}

#[test]
fn clause_final_controls_keep_the_duration_complement_analysis() {
    for text in [
        "Its owner may play it for as long as it remains exiled.",
        "Gain control of target creature for as long as you control this creature.",
    ] {
        let values = readings(text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut duration_pps = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::AdverbComplementPreposition { complement, .. } = node {
                        duration_pps.push(complement.as_ref().clone());
                    }
                })
                .unwrap();
            assert_eq!(duration_pps.len(), 1, "one duration Adjunct: {text}");
            let Reading::EquativeAdverb { complement, .. } = &duration_pps[0] else {
                panic!("duration Complement must retain Scalar Equality")
            };
            assert_eq!(complement.category(), Category::FiniteClause);
        }
    }
}

#[test]
fn duration_complement_does_not_license_nominal_or_conditional_twins() {
    // The duration PP from Master Thief cannot postmodify its Object.
    assert!(
        readings(
            "target artifact for as long as you control this creature",
            Category::NounPhrase,
        )
        .is_empty()
    );
    for text in [
        "as long as you control this creature",
        "for as long as you control this creature",
    ] {
        let values = readings(text, Category::PrepositionPhrase);
        assert!(!values.is_empty());
        for value in values {
            match (text.starts_with("for "), value) {
                (true, Reading::AdverbComplementPreposition { complement, .. }) => {
                    let Reading::EquativeAdverb { .. } = *complement else {
                        panic!("duration must have the Scalar Equality Complement")
                    };
                }
                (false, Reading::ClauseComplementPreposition { head, .. }) => {
                    let LexicalReading::Word(head) = head.value else { panic!() };
                    assert_eq!(head.lexeme, "vocab:Preposition/AsLongAs");
                }
                _ => panic!("unintended duration/conditional twin"),
            }
        }
    }
}
