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

use crate::common::LEXICON;
use crate::common::readings;

fn keyword(name: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: format!("lexeme:keyword_ability/{name}").as_str().into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Initial,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str) -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
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
            }),
        }),
    }
}

#[test]
fn champion_article_keeps_the_subject_role_and_both_roundtrip_laws() {
    // Nova Chaser / Supreme Exemplar, Wren's Run Packmaster, Thoughtweft Trio,
    // Wanderwine Prophets, Mistbind Clique, Boggart Mob and the three Changelings.
    for (text, owner) in [
        ("Champion an Elemental", "lexeme:creature_subtype/elemental"),
        ("Champion an Elf", "lexeme:creature_subtype/elf"),
        ("Champion a Kithkin", "lexeme:creature_subtype/kithkin"),
        ("Champion a Merfolk", "lexeme:creature_subtype/merfolk"),
        ("Champion a Faerie", "lexeme:creature_subtype/faerie"),
        ("Champion a Goblin", "lexeme:creature_subtype/goblin"),
        ("Champion a creature", "lexeme:type/creature"),
    ] {
        let expected = Reading::SubjectKeyword {
            form: 0,
            head: keyword("champion"),
            subject: Box::new(noun(owner)),
        };
        expected.admit(&LEXICON).unwrap();
        assert_eq!(expected.realize(&LEXICON).unwrap(), text);
        assert_eq!(
            readings(text, Category::KeywordPhrase),
            BTreeSet::from([expected])
        );
    }
}

#[test]
fn champion_coordinated_descriptions_remain_the_subject_payload() {
    // Lightning Crafter and Unstoppable Ash.
    for (text, nominal) in [
        ("Champion a Goblin or Shaman", "Goblin or Shaman"),
        ("Champion a Treefolk or Warrior", "Treefolk or Warrior"),
    ] {
        let expected: BTreeSet<_> = readings(nominal, Category::Nominal)
            .into_iter()
            .map(|subject| Reading::SubjectKeyword {
                form: 0,
                head: keyword("champion"),
                subject: Box::new(subject),
            })
            .collect();
        assert!(!expected.is_empty());
        for value in &expected {
            value.admit(&LEXICON).unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), text);
        }
        assert_eq!(readings(text, Category::KeywordPhrase), expected);
    }
}

#[test]
fn enchant_retains_its_bare_subject_and_champion_requires_the_licensed_article() {
    // Evil Presence.
    let expected = Reading::SubjectKeyword {
        form: 0,
        head: keyword("enchant"),
        subject: Box::new(noun("lexeme:type/land")),
    };
    assert_eq!(expected.realize(&LEXICON).unwrap(), "Enchant land");
    assert_eq!(
        readings("Enchant land", Category::KeywordPhrase),
        BTreeSet::from([expected])
    );
    for text in [
        "Champion Elemental",
        "Champion a Elemental",
        "Champion an creature",
        "Champion the creature",
        "Champion a creatures",
        "Enchant a land",
    ] {
        assert!(readings(text, Category::KeywordPhrase).is_empty(), "{text}");
    }
}
