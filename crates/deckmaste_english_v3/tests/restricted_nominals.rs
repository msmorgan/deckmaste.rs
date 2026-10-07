mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::noun_phrase_readings as readings;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str) -> Reading {
    Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Singular,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(true),
        },
    }
}

fn exact(text: &str, expected: &Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let actual = readings(text);
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

#[test]
fn authentic_bare_status_nominals_keep_modifier_and_head_identity() {
    // Torch Gauntlet: "Equipped creature gets +2/+0.\nEquip {2}"
    // Annex: "Enchant land\nYou control enchanted land."
    // Evil Presence: "Enchant land\nEnchanted land is a Swamp."
    for (owner, head, capitalization, text) in [
        (
            "lexeme:keyword_ability/equip/adjective",
            "lexeme:type/creature",
            SurfaceCase::Initial,
            "Equipped creature",
        ),
        (
            "lexeme:keyword_ability/enchant/adjective",
            "lexeme:type/land",
            SurfaceCase::Declared,
            "enchanted land",
        ),
        (
            "lexeme:keyword_ability/enchant/adjective",
            "lexeme:type/land",
            SurfaceCase::Initial,
            "Enchanted land",
        ),
    ] {
        exact(
            text,
            &Reading::BareStatusNounPhrase {
                form: 0,
                modifier: invariant(owner, capitalization),
                head: Box::new(noun(head)),
            },
        );
    }
}

#[test]
fn a_status_modifier_keeps_its_existing_determiner_bearing_composition() {
    // Ramses Overdark: "{T}: Destroy target enchanted creature."
    exact(
        "target enchanted creature",
        &Reading::TargetNounPhrase {
            form: 0,
            marker: invariant("vocab:TargetingMarker/Target", SurfaceCase::Declared),
            head: Box::new(Reading::PremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::Adjective {
                    form: 0,
                    head: invariant(
                        "lexeme:keyword_ability/enchant/adjective",
                        SurfaceCase::Declared,
                    ),
                }),
                head: Box::new(noun("lexeme:type/creature")),
            }),
        },
    );
}

#[test]
fn unlicensed_adjectives_do_not_admit_bare_singular_count_nominals() {
    // These are negative grammatical probes; they are not invented positive
    // card instructions.
    assert!(
        LEXICON
            .lexemes()
            .contains_key("lexeme:keyword_ability/kicker/adjective")
    );
    for text in ["red creature", "kicked creature"] {
        assert!(LEXICON.analyze(text).unknown_words().is_empty(), "{text}");
        assert!(readings(text).is_empty(), "{text}");
    }
    let invalid = Reading::BareStatusNounPhrase {
        form: 0,
        modifier: invariant(
            "lexeme:keyword_ability/kicker/adjective",
            SurfaceCase::Declared,
        ),
        head: Box::new(noun("lexeme:type/creature")),
    };
    assert!(invalid.admit(&LEXICON).is_err());
}

#[test]
fn authentic_full_genitive_possessor_preserves_internal_structure() {
    // Seedborn Muse: "Untap all permanents you control during each other
    // player's untap step."
    let possessor = Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:FloatedQuantifier/Each", SurfaceCase::Declared),
        head: Box::new(Reading::PremodifiedNominal {
            form: 0,
            modifier: Box::new(Reading::IntransitiveAdjective {
                form: 0,
                head: {
                    let mut head =
                        invariant("vocab:AttributiveAdjective/Other", SurfaceCase::Declared);
                    head.frame = Some(0);
                    head
                },
            }),
            head: Box::new(noun("lexeme:CommonNoun/Player")),
        }),
    };
    let head = Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::Adjective {
            form: 0,
            head: invariant("vocab:AttributiveAdjective/Untap", SurfaceCase::Declared),
        }),
        head: Box::new(noun("lexeme:CommonNoun/Step")),
    };
    // Existing lexical turn-part ownership and compositional adjective + noun
    // are both retained.
    let expected: BTreeSet<_> = [head, noun("lexeme:turn_part/untapStep")]
        .into_iter()
        .map(|head| Reading::GenitiveNounPhrase {
            form: 0,
            possessor: Box::new(possessor.clone()),
            marker: invariant("vocab:Genitive/Default", SurfaceCase::Declared),
            head: Box::new(head),
        })
        .collect();
    for value in &expected {
        value.admit(&LEXICON).unwrap();
        assert_eq!(
            value.realize(&LEXICON).unwrap(),
            "each other player's untap step"
        );
    }
    assert_eq!(readings("each other player's untap step"), expected);
    // Negative probes exercise case, already-genitive pronouns, and plural
    // possessors.
    for text in [
        "you's untap step",
        "its's untap step",
        "the players's untap step",
    ] {
        assert!(LEXICON.analyze(text).unknown_words().is_empty(), "{text}");
        assert!(readings(text).is_empty(), "{text}");
    }
}
