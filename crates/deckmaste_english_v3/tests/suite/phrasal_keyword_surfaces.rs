use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};

use crate::common;

fn keyword(name: &str, capitalization: SurfaceCase) -> Reading {
    Reading::BareKeyword {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: format!("lexeme:keyword_ability/{name}").as_str().into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization,
            }),
            frame: None,
            countability: None,
        },
    }
}

fn document(keyword: Reading) -> Reading {
    Reading::Document {
        form: 0,
        first: Box::new(Reading::KeywordLine {
            form: 0,
            first: Box::new(keyword),
            rest: vec![],
        }),
        rest: vec![],
    }
}

fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert_eq!(common::readings(text, category), BTreeSet::from([expected]));
}

#[test]
fn phrasal_keyword_lines_have_one_declared_route_and_roundtrip() {
    // PK1–PK4: these printed phrases are whole keyword surfaces.
    for (text, name) in [
        ("Choose a Background", "chooseABackground"),
        ("Doctor's companion", "doctorsCompanion"),
        ("Partner—Friends forever", "partnerFriendsForever"),
        ("Partner—Character select", "partnerCharacterSelect"),
        ("Partner—Father & son", "partnerFatherAndSon"),
        ("Partner—Survivors", "partnerSurvivors"),
    ] {
        exact(
            text,
            Category::Document,
            document(keyword(name, SurfaceCase::Initial)),
        );
    }
}

fn mention() -> Reading {
    Reading::KeywordComplementPreposition {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:Preposition/With".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        complement: Box::new(keyword("doctorsCompanion", SurfaceCase::Declared)),
    }
}

#[test]
fn doctors_companion_mentions_keep_the_whole_keyword_as_complement() {
    exact(
        "with doctor's companion",
        Category::PrepositionPhrase,
        mention(),
    );
    for (card, text) in [
        (
            "Rose Noble",
            "Whenever you cast a Doctor spell or creature spell with doctor's companion, draw a card.",
        ),
        (
            "An Unearthly Child",
            "I, II, III — Reveal cards from the top of your library until you reveal a Doctor card, a card with doctor's companion, or a Vehicle card. Put that card into your hand and the rest on the bottom of your library in a random order.",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        for reading in readings {
            let mut found = false;
            reading
                .visit(&mut |node| found |= node == &mention())
                .unwrap();
            assert!(found, "{card}: whole keyword Complement missing");
            reading.admit(common::lexicon()).unwrap();
            assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
        }
    }
}

#[test]
fn partner_and_the_period_terminated_choose_sentence_keep_their_roles() {
    exact(
        "Partner",
        Category::Document,
        document(keyword("partner", SurfaceCase::Initial)),
    );
    let text = "Choose a Background.";
    let readings = common::readings(text, Category::Document);
    assert!(!readings.is_empty());
    for reading in readings {
        let mut sentence = false;
        let mut new_keyword = false;
        reading
            .visit(&mut |node| {
                sentence |= node.category() == Category::Sentence;
                new_keyword |= node == &keyword("chooseABackground", SurfaceCase::Initial);
            })
            .unwrap();
        assert!(sentence);
        assert!(!new_keyword);
        assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
    }
}
