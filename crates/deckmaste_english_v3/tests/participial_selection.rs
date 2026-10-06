mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
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

fn word(owner: &str, form: WordForm, features: FeatureBundle, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

fn verb(owner: &str, frame: usize, form: WordForm) -> Word {
    word(
        owner,
        form,
        if form == WordForm::PastParticiple {
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..Default::default()
            }
        } else {
            FeatureBundle::default()
        },
        Some(frame),
    )
}

fn passive(owner: &str, frame: usize) -> Reading {
    Reading::PassivePredicate {
        form: 0,
        head: verb(owner, frame, WordForm::PastParticiple),
    }
}

fn be_passive(form: WordForm, predicate: Reading) -> Reading {
    Reading::ParticipialAuxiliaryPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: verb("core-verb:Be", 1, form),
        complement: Box::new(Reading::OvertComplement {
            category: Category::ParticipialComplement,
            form: 0,
            predicate: Box::new(Reading::PassiveComplement {
                form: 0,
                head: Box::new(predicate),
            }),
        }),
    }
}

fn roundtrip(value: &Reading, text: &str, category: Category) {
    assert_eq!(value.realize(&LEXICON).unwrap(), text);
    assert!(readings(text, category).contains(value));
    let mut expected_words = Vec::new();
    value
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let parsed = readings(text, category);
    let mut parsed_words = Vec::new();
    parsed
        .get(value)
        .unwrap()
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(expected_words, parsed_words);
}

#[test]
fn independent_authentic_perfect_passives_preserve_both_auxiliary_layers() {
    // Thieves' Auction: "...until all cards exiled this way have been chosen."
    // Struggle for Sanity: "...until all cards in that hand have been exiled."
    for (owner, text) in [
        ("core-verb:Choose", "have been chosen"),
        ("lexeme:keyword_action/exile", "have been exiled"),
    ] {
        let value = Reading::PerfectAuxiliaryPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: word(
                "core-verb:Have",
                WordForm::Present,
                FeatureBundle {
                    number: Some(Number::Plural),
                    person: Some(Person::Third),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    ..Default::default()
                },
                Some(4),
            ),
            complement: Box::new(Reading::OvertComplement {
                category: Category::PerfectComplement,
                form: 0,
                predicate: Box::new(Reading::PerfectComplement {
                    form: 0,
                    head: Box::new(be_passive(WordForm::PastParticiple, passive(owner, 0))),
                }),
            }),
        };
        roundtrip(&value, text, Category::FinitePredicate);
    }
}

#[test]
fn authentic_mixed_coordination_retains_ordinary_and_passive_predicates() {
    // Sneaky Homunculus: "This creature can't block or be blocked by creatures
    // with power 2 or greater."
    let value = Reading::Coordination {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        left: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: verb("core-verb:Block", 0, WordForm::Plain),
            complements: vec![],
        }),
        coordinator: word(
            "vocab:Coordinator/Or",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        ),
        right: Box::new(be_passive(WordForm::Plain, passive("core-verb:Block", 1))),
    };
    roundtrip(&value, "block or be blocked", Category::SecondaryVerbPhrase);
    assert_eq!(
        readings("block or be blocked", Category::SecondaryVerbPhrase),
        BTreeSet::from([value])
    );
}

#[test]
fn auxiliary_selection_rejects_bare_passives_and_repeated_passive_auxiliaries() {
    // Invalid diagnostic strings are negative witnesses, not Oracle
    // instructions.
    for text in ["have destroyed", "be been destroyed"] {
        assert!(
            readings(text, Category::SecondaryVerbPhrase).is_empty(),
            "{text:?}"
        );
    }
    let bare = passive("lexeme:keyword_action/destroy", 0);
    let invalid_perfect = Reading::PerfectComplement {
        form: 0,
        head: Box::new(bare.clone()),
    };
    assert!(invalid_perfect.admit(&LEXICON).is_err());
    let invalid_passive = Reading::PassiveComplement {
        form: 0,
        head: Box::new(be_passive(WordForm::PastParticiple, bare)),
    };
    assert!(invalid_passive.admit(&LEXICON).is_err());
}
