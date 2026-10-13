use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common::{lexicon, readings};

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

fn keyword(owner: &str) -> Word {
    invariant(
        &format!("lexeme:keyword_ability/{owner}"),
        SurfaceCase::Initial,
    )
}

#[test]
fn witnesses_select_quality_and_cost_without_a_second_keyword_route() {
    // Ace's Baseball Bat, Steelclaw Lance, Robe of the Archmagi; QC1–QC5/QC9.
    for (text, owner, quality_text, cost_text) in [
        (
            "Equip legendary creature {1}",
            "equipQuality",
            "legendary creature",
            "{1}",
        ),
        ("Equip Knight {1}", "equipQuality", "Knight", "{1}"),
        (
            "Equip Shaman, Warlock, or Wizard {1}",
            "equipQuality",
            "Shaman, Warlock, or Wizard",
            "{1}",
        ),
    ] {
        let qualities = readings(quality_text, Category::KeywordQuality);
        let costs = readings(cost_text, Category::CostSymbols);
        assert!(!qualities.is_empty(), "{quality_text}");
        assert!(!costs.is_empty(), "{cost_text}");
        let expected: BTreeSet<_> = qualities
            .into_iter()
            .flat_map(|quality| {
                costs.iter().map(move |cost| Reading::QualityCostKeyword {
                    form: 1,
                    head: keyword(owner),
                    quality: Box::new(quality.clone()),
                    cost: Box::new(cost.clone()),
                })
            })
            .collect();
        assert_eq!(readings(text, Category::KeywordPhrase), expected, "{text}");
        let documents = readings(text, Category::Document);
        assert!(!documents.is_empty(), "{text}");
        let mut parameters = BTreeSet::new();
        for document in documents {
            document
                .visit(&mut |node| {
                    if let Reading::QualityCostKeyword { .. } = node {
                        parameters.insert(node.clone());
                    }
                })
                .unwrap();
        }
        assert_eq!(parameters, expected, "{text}");
    }
}

fn nominal(owner: &str) -> Reading {
    Reading::NominalKeywordQuality {
        form: 0,
        phrase: Box::new(Reading::NominalProjection {
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
        }),
    }
}

#[test]
fn independently_constructed_parameters_roundtrip_with_their_lexical_owners() {
    for (text, owner, quality) in [
        (
            "Equip Knight {1}",
            "equipQuality",
            nominal("lexeme:creature_subtype/knight"),
        ),
        (
            "Equip Human {1}",
            "equipQuality",
            nominal("lexeme:creature_subtype/human"),
        ),
    ] {
        // Steelclaw Lance, constructed independently.
        let value = Reading::QualityCostKeyword {
            form: 1,
            head: keyword(owner),
            quality: Box::new(quality),
            cost: Box::new(Reading::SymbolSequence {
                category: Category::CostSymbols,
                form: 0,
                first: Box::new(Reading::NumericCostSymbol {
                    form: 0,
                    number: Word {
                        value: LexicalReading::Numeral {
                            value: 1,
                            notation: deckmaste_lexical::Numeral::Arabic(false),
                            capitalization: SurfaceCase::Declared,
                        },
                        frame: None,
                        countability: None,
                    },
                }),
                rest: vec![],
            }),
        };
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
        assert_eq!(
            readings(text, Category::KeywordPhrase),
            BTreeSet::from([value])
        );
    }
}

#[test]
fn cost_only_equip_keeps_its_owner_and_quality_cannot_be_empty() {
    for text in ["Equip {1}", "Equip—Pay 3 life."] {
        let values = readings(text, Category::KeywordPhrase);
        assert!(!values.is_empty());
        for value in values {
            let Reading::CostKeyword { head, .. } = value else {
                panic!("{text}");
            };
            assert_eq!(head, keyword("equip"));
        }
    }
    // QC9 preserves Craft's existing semantic signature until the owner
    // decides the new ticket. Braided Net and The Enigma Jewel remain unread.
    for value in readings("Craft {1}{U}", Category::KeywordPhrase) {
        let Reading::CostKeyword { head, .. } = value else {
            panic!("Craft Cost-only signature");
        };
        assert_eq!(head, keyword("craft"));
    }
    assert!(!readings("Craft {1}{U}", Category::KeywordPhrase).is_empty());
    for text in [
        "Craft with artifact {1}{U}",
        "Craft with four or more nonlands with activated abilities {8}{U}",
        "Craft with {1}{U}",
        "Craft with artifact",
        "Craft onto artifact {1}{U}",
        "Equip with Knight {1}",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
}

#[test]
fn marked_quality_controls_keep_their_selected_parameters() {
    for (text, class) in [
        ("Splice onto Arcane {1}{R}", Category::KeywordPhrase),
        ("Affinity for artifacts", Category::KeywordPhrase),
        (
            "Protection from legendary creatures",
            Category::KeywordPhrase,
        ),
    ] {
        let values = readings(text, class);
        assert!(!values.is_empty());
        for value in values {
            let (Reading::QualityCostKeyword { quality, .. }
            | Reading::QualityKeyword { quality, .. }) = value
            else {
                panic!("{text}");
            };
            assert!(matches!(
                *quality,
                Reading::KeywordQualityPreposition { .. }
            ));
        }
    }
}

#[test]
fn bare_keyword_premodifiers_keep_the_cost_only_owner() {
    // Auriok Steelshaper and Leonin Shikari: the new parameter recipe must
    // not create a twin of the existing bare keyword noun premodifier.
    for text in ["equip costs", "equip abilities"] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty());
        for value in values {
            let mut owners = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::NounPremodifier { head, .. } = node {
                        owners.push(head.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                owners,
                vec![Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "lexeme:keyword_ability/equip".into(),
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
