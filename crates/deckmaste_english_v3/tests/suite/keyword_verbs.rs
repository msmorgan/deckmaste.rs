use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

use crate::common;

fn verb(keyword: &str, form: WordForm, frame: usize) -> Word {
    let features = match form {
        WordForm::Present | WordForm::Preterite => FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(if form == WordForm::Present { Tense::Present } else { Tense::Past }),
            finiteness: Some(Finiteness::Finite),
            case: None,
        },
        WordForm::PastParticiple => FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..Default::default()
        },
        _ => panic!("witnesses use finite or perfect/passive verb forms"),
    };
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: format!("lexeme:keyword_ability/{keyword}/verb")
                .as_str()
                .into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: Some(frame),
        countability: None,
    }
}

const WITNESSES: &[(&str, &str, WordForm, Category, usize)] = &[
    (
        "this creature crews a Vehicle",
        "crew",
        WordForm::Present,
        Category::FinitePredicate,
        0,
    ),
    (
        "This creature crews Vehicles",
        "crew",
        WordForm::Present,
        Category::FinitePredicate,
        0,
    ),
    (
        "this creature mutates",
        "mutate",
        WordForm::Present,
        Category::FinitePredicate,
        0,
    ),
    (
        "this creature has mutated",
        "mutate",
        WordForm::PastParticiple,
        Category::SecondaryVerbPhrase,
        0,
    ),
    (
        "this creature is paired with another creature",
        "soulbond",
        WordForm::PastParticiple,
        Category::SecondaryVerbPhrase,
        1,
    ),
    (
        "it's paired with a creature",
        "soulbond",
        WordForm::PastParticiple,
        Category::SecondaryVerbPhrase,
        1,
    ),
    (
        "this creature exploits a creature",
        "exploit",
        WordForm::Present,
        Category::FinitePredicate,
        0,
    ),
    (
        "it exploited that creature",
        "exploit",
        WordForm::Preterite,
        Category::FinitePredicate,
        0,
    ),
    (
        "This creature escapes with a +1/+1 counter on it",
        "escape",
        WordForm::Present,
        Category::FinitePredicate,
        0,
    ),
    (
        "it escaped",
        "escape",
        WordForm::Preterite,
        Category::FinitePredicate,
        0,
    ),
    (
        "the creature this card haunts dies",
        "haunt",
        WordForm::Present,
        Category::FiniteObjectGap,
        0,
    ),
    (
        "the creature it haunts dies",
        "haunt",
        WordForm::Present,
        Category::FiniteObjectGap,
        0,
    ),
];

#[test]
fn attested_keyword_verbs_have_their_declared_roles_in_every_reading() {
    // In order: Veteran Motorist, Hotshot Mechanic, Dreamtail Heron,
    // Huntmaster Liger, Wingcrafter, Joint Assault, Fell Stinger,
    // Silumgar Scavenger, Phoenix of Ash, Kroxa, Cry of Contrition,
    // and Orzhov Pontiff. These rows pin the verb role, not the whole unit.
    for &(text, keyword, form, role, frame) in WITNESSES {
        let expected = verb(keyword, form, frame);
        let readings = common::readings(text, Category::FiniteClause);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            let mut heads = Vec::new();
            reading
                .visit(&mut |node| {
                    let (Reading::SelectedPredicate { head, .. }
                    | Reading::SelectedFramePassive { head, .. }
                    | Reading::FiniteObjectGap { head, .. }) = node
                    else {
                        return;
                    };
                    if let LexicalReading::Word(value) = &head.value
                        && value.lexeme == format!("lexeme:keyword_ability/{keyword}/verb").as_str()
                    {
                        heads.push((node.category(), head.clone()));
                    }
                })
                .unwrap();
            assert_eq!(heads, vec![(role, expected.clone())], "{text}");
        }
    }
}

#[test]
fn independently_built_mutate_predicates_preserve_both_roundtrip_laws() {
    // Dreamtail Heron's mutates; Huntmaster Liger's perfect mutated.
    for (text, category, form) in [
        ("mutates", Category::FinitePredicate, WordForm::Present),
        (
            "mutated",
            Category::SecondaryVerbPhrase,
            WordForm::PastParticiple,
        ),
    ] {
        let expected = Reading::SelectedPredicate {
            category,
            form: 0,
            head: verb("mutate", form, 0),
            complements: vec![],
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(common::readings(text, category), BTreeSet::from([expected]));
    }
}

#[test]
fn henry_wus_exploited_creature_is_a_verbal_premodifier() {
    let text = "the exploited creature";
    common::assert_constituents(
        text,
        Category::NounPhrase,
        &[(Category::VerbalPremodifier, "exploited")],
    );
    for reading in common::readings(text, Category::NounPhrase) {
        let mut heads = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::VerbalPremodifier { head, .. } = node {
                    heads.push(head.clone());
                }
            })
            .unwrap();
        assert_eq!(heads, vec![verb("exploit", WordForm::PastParticiple, 0)]);
    }
}

#[test]
fn saddle_keeps_the_attested_adjectival_state_routes() {
    // Stubborn Burrowfiend and Archmage's Newt, respectively. Retiring this
    // adjective before a replacement for become would lose the former.
    for text in ["this creature becomes saddled", "this creature is saddled"] {
        common::assert_constituents(
            text,
            Category::FiniteClause,
            &[
                (Category::PredicativeComplement, "saddled"),
                (Category::AdjectivePhrase, "saddled"),
            ],
        );
        let readings = common::readings(text, Category::FiniteClause);
        let mut state_readings = Vec::new();
        for reading in readings {
            let mut state = false;
            reading
                .visit(&mut |node| {
                    state |= node.category() == Category::PredicativeComplement;
                })
                .unwrap();
            if state {
                state_readings.push(reading);
            }
        }
        assert!(!state_readings.is_empty(), "missing adjective for {text}");
        for reading in state_readings {
            let mut words = Vec::new();
            reading
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(value) = &word.value
                        && value.lexeme == "lexeme:designation/saddled"
                    {
                        words.push(word.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                words,
                vec![Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "lexeme:designation/saddled".into(),
                        form: WordForm::Invariant,
                        features: FeatureBundle::default(),
                        variant: 0,
                        capitalization: SurfaceCase::Declared,
                    }),
                    frame: None,
                    countability: None,
                }]
            );
        }
    }
}
