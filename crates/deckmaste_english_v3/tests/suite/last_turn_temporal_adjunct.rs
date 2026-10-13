use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn word(owner: &str, form: WordForm, features: FeatureBundle, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability,
    }
}

fn last_turn() -> Reading {
    Reading::NominalAdjunctPhrase {
        form: 1,
        determiner: word(
            "vocab:AttributiveAdjective/Last",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        ),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word(
                        "lexeme:CommonNoun/Turn",
                        WordForm::Singular,
                        FeatureBundle {
                            number: Some(Number::Singular),
                            ..Default::default()
                        },
                        Some(true),
                    ),
                }),
            }),
        }),
    }
}

fn witness(card: &str, text: &str) {
    let readings = common::readings_on_card(card, text, Category::Document);
    assert!(!readings.is_empty(), "{card}");
    let Reading::NominalAdjunctPhrase {
        determiner: expected_dependent,
        ..
    } = last_turn()
    else {
        unreachable!()
    };
    for reading in readings {
        reading.admit(common::lexicon()).unwrap();
        assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
        let mut adjuncts = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::NominalAdjunctPhrase { determiner, .. } = node
                    && determiner == &expected_dependent
                {
                    adjuncts.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(
            adjuncts,
            vec![last_turn()],
            "{card}: last turn is a temporal Adjunct"
        );
        let mut last_words = Vec::new();
        reading
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value
                    && value.lexeme.as_str() == "vocab:AttributiveAdjective/Last"
                {
                    last_words.push(word.clone());
                }
            })
            .unwrap();
        let Reading::NominalAdjunctPhrase { determiner, .. } = last_turn() else {
            unreachable!()
        };
        assert_eq!(
            last_words,
            vec![determiner],
            "{card}: no extra Object or Subject analysis"
        );
    }
}

#[test]
fn first_response() {
    witness(
        "First Response",
        "At the beginning of each upkeep, if you lost life last turn, create a 1/1 white Soldier creature token.",
    );
}

#[test]
fn lambholt_elder() {
    witness(
        "Lambholt Elder",
        "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
    );
}

#[test]
fn silverpelt_werewolf() {
    witness(
        "Silverpelt Werewolf",
        "At the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
    );
}

#[test]
fn call_of_the_full_moon() {
    witness(
        "Call of the Full Moon",
        "At the beginning of each upkeep, if a player cast two or more spells last turn, sacrifice this Aura.",
    );
}

#[test]
fn feast_on_the_fallen() {
    witness(
        "Feast on the Fallen",
        "At the beginning of each upkeep, if an opponent lost life last turn, put a +1/+1 counter on target creature you control.",
    );
}

#[test]
fn independently_constructed_deictic_temporal_adjunct_obeys_both_laws() {
    let expected = last_turn();
    let mut wrong_category = expected.clone();
    if let Reading::NominalAdjunctPhrase { form, .. } = &mut wrong_category {
        *form = 0;
    }
    assert!(wrong_category.admit(common::lexicon()).is_err());
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "last turn");
    assert_eq!(
        common::readings("last turn", Category::NominalAdjunctPhrase),
        BTreeSet::from([expected])
    );
    assert!(common::readings("turn", Category::NominalAdjunctPhrase).is_empty());
    assert!(common::readings("next turn", Category::NominalAdjunctPhrase).is_empty());
}

#[test]
fn last_has_one_word_owner_and_keeps_ordinary_premodification() {
    let expected = word(
        "vocab:AttributiveAdjective/Last",
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
    );
    let alternatives = common::lexicon().analyze("last");
    assert_eq!(
        alternatives
            .matches
            .iter()
            .map(|m| m.reading.clone())
            .collect::<Vec<_>>(),
        vec![expected.value.clone()]
    );
    assert_eq!(
        common::readings("last", Category::AdjectivePhrase),
        BTreeSet::from([Reading::Adjective {
            form: 0,
            head: expected
        }])
    );
    // Attested constituents retain the ordinary adjective route, as next does.
    for text in ["the last time", "your last turn", "the last card"] {
        let readings = common::readings(text, Category::NounPhrase);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            reading.admit(common::lexicon()).unwrap();
            assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
        }
    }
}
