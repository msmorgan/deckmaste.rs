use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, Numeral, SurfaceCase, WordForm,
};

use crate::common;

fn word(owner: &str, form: WordForm, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: if form == WordForm::Singular {
                FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                }
            } else {
                FeatureBundle::default()
            },
            variant: 0,
            capitalization,
        }),
        frame: (form == WordForm::Singular).then_some(0),
        countability: (form == WordForm::Singular).then_some(true),
    }
}

fn modifier(owner: &str, capitalization: SurfaceCase) -> Word {
    word(owner, WordForm::Invariant, capitalization)
}

fn stem(owner: &str) -> Word {
    word(
        &format!("{owner}/bound-keyword-stem"),
        WordForm::Singular,
        SurfaceCase::Declared,
    )
}

fn suffix(keyword: &str) -> Word {
    word(
        &format!("lexeme:keyword_ability/{keyword}/bound-suffix"),
        WordForm::Invariant,
        SurfaceCase::Declared,
    )
}

fn numeric_cost(amount: i32) -> Reading {
    Reading::SymbolSequence {
        category: Category::CostSymbols,
        form: 0,
        first: Box::new(Reading::NumericCostSymbol {
            form: 0,
            number: Word {
                value: LexicalReading::Numeral {
                    value: amount,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        rest: vec![],
    }
}

pub(super) fn assert_keyword(
    text: &str,
    expected_modifier: &Word,
    expected_stem: &Word,
    cost_text: Option<&str>,
) {
    let readings = common::readings(text, Category::KeywordPhrase);
    assert!(!readings.is_empty(), "{text}");
    for value in readings {
        let (modifier, quality, head, cost) = match &value {
            Reading::BoundQualityKeyword {
                modifier,
                quality,
                head,
                ..
            } => (modifier, quality, head, None),
            Reading::BoundQualityCostKeyword {
                modifier,
                quality,
                head,
                cost,
                ..
            } => (modifier, quality, head, Some(&**cost)),
            _ => panic!("a supertype must premodify the bound keyword stem"),
        };
        assert_eq!(modifier.as_ref(), Some(expected_modifier));
        assert_eq!(quality, expected_stem);
        assert_eq!(
            head,
            &suffix(if cost_text.is_some() { "typecycling" } else { "landwalk" })
        );
        assert_eq!(
            cost.map(|value| value.realize(common::lexicon()).unwrap()),
            cost_text.map(str::to_owned)
        );
    }
}

#[test]
fn attested_premodifiers_preserve_lexical_roles_and_both_roundtrip_laws() {
    // Rime Dryad, Ayumi, the Last Visitor, Dryad Sophisticate, and Ash Barrens.
    for (text, adjective, owner, cost) in [
        (
            "Snow forestwalk",
            "vocab:Supertype/Snow",
            "lexeme:land_subtype/forest",
            None,
        ),
        (
            "Legendary landwalk",
            "vocab:PredicativeAdjective/Legendary",
            "lexeme:type/land",
            None,
        ),
        (
            "Nonbasic landwalk",
            "vocab:Supertype/Basic/non",
            "lexeme:type/land",
            None,
        ),
        (
            "Basic landcycling {1}",
            "vocab:Supertype/Basic",
            "lexeme:type/land",
            Some(1),
        ),
    ] {
        let modifier = Some(modifier(adjective, SurfaceCase::Initial));
        let quality = stem(owner);
        let expected = if let Some(amount) = cost {
            Reading::BoundQualityCostKeyword {
                form: 0,
                modifier,
                quality,
                head: suffix("typecycling"),
                cost: Box::new(numeric_cost(amount)),
            }
        } else {
            Reading::BoundQualityKeyword {
                form: 0,
                modifier,
                quality,
                head: suffix("landwalk"),
            }
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::KeywordPhrase),
            BTreeSet::from([expected.clone()])
        );
        let documents = common::readings(text, Category::Document);
        assert!(!documents.is_empty(), "{text}");
        for document in documents {
            let mut keywords = Vec::new();
            document
                .visit(&mut |node| {
                    if node.category() == Category::KeywordPhrase {
                        keywords.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(keywords, vec![expected.clone()]);
        }
    }
}

#[test]
fn gleam_of_resistance_cost_is_the_keyword_parameter() {
    assert_keyword(
        "Basic landcycling {1}{W}",
        &modifier("vocab:Supertype/Basic", SurfaceCase::Initial),
        &stem("lexeme:type/land"),
        Some("{1}{W}"),
    );
}

#[test]
fn running_mentions_keep_one_keyword_complement_and_lowercase_premodification() {
    let text = "Equipped creature has nonbasic landwalk."; // Trailblazer's Boots.
    let expected = Reading::BoundQualityKeyword {
        form: 0,
        modifier: Some(modifier("vocab:Supertype/Basic/non", SurfaceCase::Declared)),
        quality: stem("lexeme:type/land"),
        head: suffix("landwalk"),
    };
    let readings = common::readings(text, Category::Sentence);
    assert!(!readings.is_empty());
    for reading in readings {
        let mut keywords = Vec::new();
        reading
            .visit(&mut |node| {
                if node.category() == Category::KeywordPhrase {
                    keywords.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(keywords, vec![expected.clone()]);
    }
    common::assert_constituents(
        text,
        Category::Sentence,
        &[
            (Category::KeywordPhrase, "nonbasic landwalk"),
            (Category::GrantedAbility, "nonbasic landwalk"),
        ],
    );
    // Livonya Silone: the coordinated second keyword has interior capitalization.
    common::assert_constituents(
        "First strike; legendary landwalk",
        Category::Document,
        &[(Category::KeywordPhrase, "legendary landwalk")],
    );
    assert_keyword(
        "legendary landwalk",
        &modifier(
            "vocab:PredicativeAdjective/Legendary",
            SurfaceCase::Declared,
        ),
        &stem("lexeme:type/land"),
        None,
    );
}

#[test]
fn nonsupertype_and_phrasal_modifiers_do_not_enter_the_bound_keyword_route() {
    let invalid = Reading::BoundQualityKeyword {
        form: 0,
        modifier: Some(modifier("vocab:ColorWord/Green", SurfaceCase::Initial)),
        quality: stem("lexeme:type/land"),
        head: suffix("landwalk"),
    };
    assert!(invalid.admit(common::lexicon()).is_err());

    for text in [
        "Green landwalk",
        "Tapped landwalk",
        "Snow and legendary landwalk",
        "Basic land cycling {1}",
        "Basic Landcycling {1}",
        "Basic landcycling",
    ] {
        assert!(
            common::readings(text, Category::KeywordPhrase).is_empty(),
            "{text}"
        );
    }
    for text in [
        "Landcycling {2}",
        "Plainscycling {2}",
        "Swampcycling {2}",
        "Forestwalk",
        "Landwalk",
    ] {
        for value in common::readings(text, Category::KeywordPhrase) {
            let (Reading::BoundQualityKeyword { modifier, .. }
            | Reading::BoundQualityCostKeyword { modifier, .. }) = value
            else {
                panic!("the existing bound route must survive");
            };
            assert_eq!(modifier, None);
        }
    }
}
