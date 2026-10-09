use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn word(id: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn anywhere() -> Reading {
    Reading::FusedDeterminerHeadNounPhrase {
        form: 0,
        head: word("vocab:CompoundDeterminative/Anywhere"),
    }
}

fn source() -> Reading {
    Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/From"),
        complement: Box::new(Reading::CasePhrase {
            form: 0,
            category: Category::AccusativePhrase,
            head: Box::new(anywhere()),
        }),
    }
}

fn witness(card: &str, text: &str, expected_count: usize) {
    let readings = common::readings_on_card(card, text, Category::Document);
    assert_eq!(readings.len(), expected_count, "{card}");
    for reading in readings {
        let mut sources = Vec::new();
        reading
            .visit(&mut |node| {
                if node == &source() {
                    sources.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(sources, vec![source()], "{card}");
    }
}

#[test]
fn brinebound_gift() {
    witness(
        "Brinebound Gift",
        "If this Aura would be put into a graveyard from anywhere, exile it instead.",
        12,
    );
}

#[test]
fn yawgmoths_agenda() {
    witness(
        "Yawgmoth's Agenda",
        "If a card would be put into your graveyard from anywhere, exile it instead.",
        12,
    );
}

#[test]
fn planar_void() {
    witness(
        "Planar Void",
        "Whenever another card is put into a graveyard from anywhere, exile that card.",
        3,
    );
}

#[test]
fn rest_in_peace() {
    witness(
        "Rest in Peace",
        "If a card or token would be put into a graveyard from anywhere, exile it instead.",
        12,
    );
}

#[test]
fn generous_soul() {
    witness(
        "Generous Soul",
        "If Generous Soul would be put into a graveyard from anywhere, exile it instead.",
        12,
    );
}

#[test]
fn independently_constructed_fused_head_and_source_obey_both_laws() {
    // The ticket witnesses attest the NP and the PP, independently constructed
    // here.
    for (value, text, category) in [
        (anywhere(), "anywhere", Category::NounPhrase),
        (source(), "from anywhere", Category::PrepositionPhrase),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let parsed = common::readings(text, category);
        assert_eq!(parsed, BTreeSet::from([value.clone()]));
        let observed = parsed.get(&value).unwrap();
        let mut nodes = Vec::new();
        let mut observed_nodes = Vec::new();
        value.visit(&mut |node| nodes.push(node.clone())).unwrap();
        observed
            .visit(&mut |node| observed_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(observed_nodes, nodes);
        let mut words = Vec::new();
        let mut observed_words = Vec::new();
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        observed
            .visit_words(&mut |word| observed_words.push(word.clone()))
            .unwrap();
        assert_eq!(observed_words, words);
    }
}

#[test]
fn compound_determinative_replaces_the_adverb_owner() {
    let lexemes = common::lexicon().lexemes();
    let owners: Vec<_> = lexemes
        .values()
        .filter(|lexeme| lexeme.lemma == "anywhere")
        .map(|lexeme| (lexeme.id.to_string(), lexeme.category))
        .collect();
    assert_eq!(
        owners,
        vec![(
            "vocab:CompoundDeterminative/Anywhere".to_owned(),
            deckmaste_lexical::Category::Determinative
        )]
    );
    assert!(!lexemes.contains_key("vocab:LocativeProform/Anywhere"));
}
