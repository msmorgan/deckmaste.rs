use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn expected(owner: &str, amount: i32, capitalization: SurfaceCase) -> Reading {
    Reading::BoundQualityCostKeyword {
        form: 0,
        modifier: None,
        quality: Word {
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
        },
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "lexeme:keyword_ability/typecycling/bound-suffix".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        cost: Box::new(Reading::SymbolSequence {
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
        }),
    }
}

fn check(text: &str, owner: &str, amount: i32) {
    let value = expected(owner, amount, SurfaceCase::Initial);
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert_eq!(
        common::readings(text, Category::KeywordPhrase),
        BTreeSet::from([value.clone()]),
    );
    assert_eq!(common::readings(text, Category::Document).len(), 1);
    let Reading::BoundQualityCostKeyword {
        quality,
        head,
        cost,
        ..
    } = &value
    else {
        unreachable!();
    };
    let mut words = vec![quality.clone(), head.clone()];
    cost.visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    let mut visited = Vec::new();
    value
        .visit_words(&mut |word| visited.push(word.clone()))
        .unwrap();
    assert_eq!(visited, words);
}

#[test]
fn canyon_crawler_swampcycling() {
    check("Swampcycling {2}", "lexeme:land_subtype/swamp", 2);
}

#[test]
fn shepherding_spirits_plainscycling() {
    check("Plainscycling {2}", "lexeme:land_subtype/plains", 2);
}

#[test]
fn step_through_wizardcycling() {
    check("Wizardcycling {2}", "lexeme:creature_subtype/wizard", 2);
}

#[test]
fn generous_ent_forestcycling() {
    check("Forestcycling {1}", "lexeme:land_subtype/forest", 1);
}

#[test]
fn other_attested_classes_keep_their_declared_owners() {
    // Oliphaunt, Lórien Revealed, Vedalken Aethermage, Ash Barrens,
    // Homing Sliver and Hobbit Hole attest these keyword lines.
    for (text, owner, amount) in [
        ("Mountaincycling {1}", "lexeme:land_subtype/mountain", 1),
        ("Islandcycling {1}", "lexeme:land_subtype/island", 1),
        ("Wizardcycling {3}", "lexeme:creature_subtype/wizard", 3),
        ("Slivercycling {3}", "lexeme:creature_subtype/sliver", 3),
        ("Halflingcycling {4}", "lexeme:creature_subtype/halfling", 4),
    ] {
        check(text, owner, amount);
    }
    // Ash Barrens attests this constituent under Basic (R8b), in lowercase.
    let value = expected("lexeme:type/land", 1, SurfaceCase::Declared);
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), "landcycling {1}");
    assert_eq!(
        common::readings("landcycling {1}", Category::KeywordPhrase),
        BTreeSet::from([value])
    );
}

#[test]
fn stem_class_binding_and_mandatory_cost_constrain_admission() {
    for text in [
        "Wizardwalk",
        "Sliverwalk",
        "Artifactcycling {2}",
        "Equipmentcycling {2}",
        "Swampscycling {2}",
        "Swamp cycling {2}",
        "SwampCycling {2}",
        "Swampcycling",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
    assert_eq!(common::readings("Swampwalk", Category::Document).len(), 1);
    let value = expected("lexeme:creature_subtype/wizard", 2, SurfaceCase::Initial);
    let Reading::BoundQualityCostKeyword { quality, .. } = value else {
        unreachable!();
    };
    let walk = common::readings("Swampwalk", Category::KeywordPhrase)
        .pop_first()
        .unwrap();
    let Reading::BoundQualityKeyword { head, .. } = walk else {
        unreachable!();
    };
    assert!(
        Reading::BoundQualityKeyword {
            form: 0,
            modifier: None,
            quality,
            head
        }
        .admit(common::lexicon())
        .is_err()
    );
}

#[test]
fn gleam_of_resistance_reads_the_ruled_supertype_premodifier() {
    crate::supertype_premodified_bound_keyword::assert_keyword(
        "Basic landcycling {1}{W}",
        &Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:Supertype/Basic".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Initial,
            }),
            frame: None,
            countability: None,
        },
        &match expected("lexeme:type/land", 1, SurfaceCase::Declared) {
            Reading::BoundQualityCostKeyword { quality, .. } => quality,
            _ => unreachable!(),
        },
        Some("{1}{W}"),
    );
}

#[test]
fn plain_cycling_keeps_its_free_lexical_owner() {
    for text in ["Cycling {2}", "Cycling {1}", "Cycling {1}{W}"] {
        let readings = common::readings(text, Category::KeywordPhrase);
        assert_eq!(readings.len(), 1);
        let mut owners = Vec::new();
        readings
            .first()
            .unwrap()
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value {
                    owners.push(value.lexeme);
                }
            })
            .unwrap();
        assert_eq!(owners[0], "lexeme:keyword_ability/cycling");
    }
    assert!(
        !common::lexicon()
            .values()
            .any(|value| value.lexeme == "lexeme:keyword_ability/typecycling")
    );
}
