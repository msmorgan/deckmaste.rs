use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

fn stem(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: format!("{owner}/bound-keyword-stem").as_str().into(),
            form: WordForm::Singular,
            features: FeatureBundle {
                number: Some(Number::Singular),
                ..Default::default()
            },
            variant: 0,
            capitalization,
        }),
        frame: Some(0),
        countability: Some(true),
    }
}

fn suffix() -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "lexeme:keyword_ability/landwalk/bound-suffix".into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn keyword_labels_preserve_bound_structure_and_both_roundtrip_laws() {
    // Sheoldred, Scarwood Bandits, River Bear, Mountain Goat, Graceful Antelope,
    // and Desert Nomads attest these labels.
    for (text, owner) in [
        ("Swampwalk", "lexeme:land_subtype/swamp"),
        ("Forestwalk", "lexeme:land_subtype/forest"),
        ("Islandwalk", "lexeme:land_subtype/island"),
        ("Mountainwalk", "lexeme:land_subtype/mountain"),
        ("Plainswalk", "lexeme:land_subtype/plains"),
        ("Desertwalk", "lexeme:land_subtype/desert"),
    ] {
        let quality = stem(owner, SurfaceCase::Initial);
        let head = suffix();
        let expected = Reading::BoundQualityKeyword {
            form: 0,
            modifier: None,
            quality: quality.clone(),
            head: head.clone(),
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::KeywordPhrase),
            BTreeSet::from([expected.clone()]),
            "{text}"
        );
        let mut words = Vec::new();
        expected
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        assert_eq!(words, vec![quality, head]);
        assert_eq!(
            common::readings(text, Category::Document).len(),
            1,
            "{text}"
        );
    }
}

#[test]
fn running_text_witnesses_retain_lowercase_stems() {
    for (card, text, owner) in [
        (
            "Goblin King",
            "Other Goblins get +1/+1 and have mountainwalk.",
            "lexeme:land_subtype/mountain",
        ),
        (
            "Veldrane of Sengir",
            "Veldrane gets -3/-0 and gains forestwalk until end of turn.",
            "lexeme:land_subtype/forest",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Sentence);
        assert!(!readings.is_empty(), "{card}");
        for reading in readings {
            let mut words = Vec::new();
            reading
                .visit_words(&mut |word| words.push(word.clone()))
                .unwrap();
            assert_eq!(
                words
                    .iter()
                    .filter(|word| **word == stem(owner, SurfaceCase::Declared))
                    .count(),
                1
            );
            assert_eq!(words.iter().filter(|word| **word == suffix()).count(), 1);
        }
    }
}

#[test]
fn bound_stems_do_not_admit_free_nouns_or_separated_keywords() {
    for text in ["Swamp walk", "Swampswalk", "IslandWalk", "Walk"] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
    for text in ["swamp", "forest", "island", "mountain", "plains", "desert"] {
        assert!(
            common::readings(text, Category::Nominal).is_empty(),
            "{text}"
        );
    }
    assert!(
        common::readings(
            "Other Goblins get +1/+1 and have Mountainwalk.",
            Category::Sentence,
        )
        .is_empty()
    );
}

#[test]
fn generic_landwalk_has_only_the_composed_lexical_route() {
    let readings = common::readings("landwalk", Category::KeywordPhrase);
    assert_eq!(
        readings,
        BTreeSet::from([Reading::BoundQualityKeyword {
            form: 0,
            modifier: None,
            quality: stem("lexeme:type/land", SurfaceCase::Declared),
            head: suffix(),
        }])
    );
    assert!(
        !common::lexicon()
            .values()
            .any(|value| value.lexeme == "lexeme:keyword_ability/landwalk")
    );
}
