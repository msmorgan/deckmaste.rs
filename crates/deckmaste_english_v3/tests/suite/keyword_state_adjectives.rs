use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn word(owner: &str, form: WordForm, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                        && (form != WordForm::Present
                            || (value.features.number == Some(Number::Singular)
                                && value.features.person == Some(Person::Third)))
                })
                .unwrap_or_else(|| panic!("missing declaration {owner}"))
                .clone(),
        ),
        frame,
        countability: None,
    }
}

fn contains_adjective(reading: &Reading, owner: &str) -> bool {
    let mut found = false;
    reading
        .visit(&mut |node| {
            if let Reading::AdjectivalComplement { phrase, .. } = node {
                phrase
                    .visit_words(&mut |word| {
                        if let LexicalReading::Word(value) = &word.value {
                            found |= value.lexeme == owner;
                        }
                    })
                    .unwrap();
            }
        })
        .unwrap();
    found
}

#[test]
fn independent_state_complements_preserve_both_roundtrip_laws() {
    // Fiend of the Shadows and Mobilizer Mech: attested predicate constituents.
    for (verb, adjective, text) in [
        (
            "core-verb:Remain",
            "lexeme:keyword_action/exile/adjective",
            "remains exiled",
        ),
        (
            "core-verb:Become",
            "lexeme:keyword_ability/crew/verb/adjective",
            "becomes crewed",
        ),
    ] {
        let expected = Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: word(verb, WordForm::Present, Some(0)),
            complements: vec![FrameValue::Argument(Box::new(
                Reading::AdjectivalComplement {
                    form: 0,
                    phrase: Box::new(Reading::Adjective {
                        form: 0,
                        head: word(adjective, WordForm::Invariant, None),
                    }),
                },
            ))],
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::FinitePredicate),
            BTreeSet::from([expected]),
        );
    }
}

#[test]
fn remain_and_become_witnesses_select_the_declared_adjective() {
    for (text, owner) in [
        (
            "You may play that card for as long as it remains exiled.",
            "lexeme:keyword_action/exile/adjective",
        ),
        (
            "You may look at those cards for as long as they remain exiled.",
            "lexeme:keyword_action/exile/adjective",
        ),
        (
            "Whenever this Vehicle becomes crewed, up to one other target Vehicle you control becomes an artifact creature until end of turn.",
            "lexeme:keyword_ability/crew/verb/adjective",
        ),
    ] {
        let readings = common::readings(text, Category::Document);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            assert!(contains_adjective(&reading, owner), "{text}");
        }
    }
    assert!(common::readings("it remains destroyed", Category::FiniteClause).is_empty());
}

#[test]
fn be_preserves_the_adjectival_and_verbal_passive_analyses() {
    for (text, adjective, verb) in [
        (
            "is exiled",
            "lexeme:keyword_action/exile/adjective",
            "lexeme:keyword_action/exile",
        ),
        (
            "is crewed",
            "lexeme:keyword_ability/crew/verb/adjective",
            "lexeme:keyword_ability/crew/verb",
        ),
        (
            "is saddled",
            "lexeme:designation/saddled",
            "lexeme:keyword_ability/saddle/verb",
        ),
    ] {
        let mut roles = BTreeSet::new();
        for reading in common::readings(text, Category::FinitePredicate) {
            let adjective_role = contains_adjective(&reading, adjective);
            let mut passive_role = false;
            reading
                .visit(&mut |node| {
                    if let Reading::PassivePredicate { .. } = node {
                        node.visit_words(&mut |word| {
                            if let LexicalReading::Word(value) = &word.value {
                                passive_role |=
                                    value.lexeme == verb && value.form == WordForm::PastParticiple;
                            }
                        })
                        .unwrap();
                    }
                })
                .unwrap();
            assert!(
                roles.insert((adjective_role, passive_role)),
                "duplicate role for {text}"
            );
        }
        assert_eq!(
            roles,
            BTreeSet::from([(true, false), (false, true)]),
            "{text}"
        );
    }
}

#[test]
fn nominal_participles_keep_their_route_and_by_adjunct_matches_tapped() {
    for (text, category) in [
        ("the exiled card", Category::NounPhrase),
        ("exiled cards", Category::NounPhrase),
        ("cards exiled with this creature", Category::NounPhrase),
        ("cards exiled this way", Category::NounPhrase),
    ] {
        let readings = common::readings(text, category);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            reading
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(value) = &word.value {
                        assert_ne!(
                            value.lexeme, "lexeme:keyword_action/exile/adjective",
                            "{text}"
                        );
                        assert_ne!(
                            value.lexeme, "lexeme:keyword_ability/crew/verb/adjective",
                            "{text}"
                        );
                    }
                })
                .unwrap();
        }
    }

    // KS-R2: the claim parent's tapped state already admits this by Adjunct.
    let mut roles = BTreeSet::new();
    for reading in common::readings("was crewed by two creatures", Category::FinitePredicate) {
        let adjective_role =
            contains_adjective(&reading, "lexeme:keyword_ability/crew/verb/adjective");
        let mut passive_role = false;
        let mut state_by_adjunct = false;
        reading
            .visit(&mut |node| {
                if let Reading::PassivePredicate { .. } = node {
                    passive_role = true;
                }
                if let Reading::PrepositionPredicate { head, modifier, .. } = node
                    && contains_adjective(head, "lexeme:keyword_ability/crew/verb/adjective")
                    && let Reading::PrepositionPhrase { head, .. } = modifier.as_ref()
                {
                    assert_eq!(
                        *head,
                        word("vocab:Preposition/By", WordForm::Invariant, None)
                    );
                    state_by_adjunct = true;
                }
            })
            .unwrap();
        assert_eq!(adjective_role, state_by_adjunct);
        roles.insert((adjective_role, passive_role));
    }
    assert_eq!(roles, BTreeSet::from([(true, false), (false, true)]));
}

#[test]
fn saddle_active_witnesses_use_the_exported_verb() {
    // Deathless Pilot and Reckless Velocitaur: attested finite Clause constituents.
    for text in [
        "this creature saddles Mounts",
        "this creature saddles a Mount",
    ] {
        let readings = common::readings(text, Category::FiniteClause);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            let mut heads = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate { head, .. } = node {
                        heads.push(head.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                heads,
                vec![word(
                    "lexeme:keyword_ability/saddle/verb",
                    WordForm::Present,
                    Some(0)
                )]
            );
        }
    }
}
