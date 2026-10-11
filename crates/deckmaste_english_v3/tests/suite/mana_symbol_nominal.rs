use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, FrameItem, FrameSlot, LexicalReading, LexicalValue, Numeral, Relation,
    SurfaceCase, WordForm,
};

use crate::common;

fn symbol(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: format!("vocab:FixedCostSymbol/{owner}").as_str().into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn nominal(first: Word, form: usize, rest: Vec<Word>) -> Reading {
    Reading::MeasuredSymbolNominal {
        form: 0,
        head: Box::new(Reading::NominalSymbol { form, head: first }),
        rest: rest
            .into_iter()
            .map(|head| Reading::CountNominalSymbol {
                form: 0,
                head: Box::new(Reading::NominalSymbol { form: 0, head }),
            })
            .collect(),
    }
}

#[test]
fn independently_constructed_symbol_sequences_preserve_both_roundtrip_laws() {
    // Overgrowth and Taste of Paradise supply the attested constituents.
    let numeric = Word {
        value: LexicalReading::Numeral {
            value: 1,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    };
    for (text, expected) in [
        ("{G}{G}", nominal(symbol("Green"), 0, vec![symbol("Green")])),
        ("{1}{G}", nominal(numeric, 1, vec![symbol("Green")])),
    ] {
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::Nominal),
            BTreeSet::from([expected.clone()])
        );
        let bare = Reading::BareNounPhrase {
            form: 0,
            head: Box::new(expected),
        };
        assert!(bare.realize(common::lexicon()).is_err());
    }
}

fn symbol_nominal_head(nominal: &Reading) -> &Reading {
    match nominal {
        Reading::PostmodifiedNominal { head, .. } => symbol_nominal_head(head),
        Reading::MeasuredSymbolNominal { .. } => nominal,
        _ => panic!("expected a symbol-headed Nominal"),
    }
}

#[test]
fn additional_mana_is_the_head_of_the_selected_object_nominal() {
    for (card, text, owner, rest) in [
        (
            "Crypt Ghast",
            "Whenever you tap a Swamp for mana, add an additional {B}.",
            "Black",
            vec![],
        ),
        (
            "Wild Growth",
            "Whenever enchanted land is tapped for mana, its controller adds an additional {G}.",
            "Green",
            vec![],
        ),
        (
            "Overgrowth",
            "Whenever enchanted land is tapped for mana, its controller adds an additional {G}{G}.",
            "Green",
            vec![symbol("Green")],
        ),
        (
            "Elvish Guidance",
            "Whenever enchanted land is tapped for mana, its controller adds an additional {G} for each Elf on the battlefield.",
            "Green",
            vec![],
        ),
    ] {
        let expected = nominal(symbol(owner), 0, rest);
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        for reading in readings {
            let mut objects = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        head, complements, ..
                    } = node
                        && let LexicalReading::Word(word) = &head.value
                        && word.lexeme.as_str() == "core-verb:Add"
                    {
                        let frame = &common::lexicon().lexemes()[&word.lexeme].properties.frames
                            [head.frame.unwrap()];
                        assert!(frame.items.contains(&FrameItem::Argument(FrameSlot {
                            relation: Relation::Object,
                            category: "NounPhrase".into(),
                        })));
                        for complement in complements {
                            if let FrameValue::Argument(object) = complement {
                                assert_eq!(object.category(), Category::AccusativePhrase);
                                let mut heads = BTreeSet::new();
                                object
                                    .visit(&mut |part| {
                                        if let Reading::PremodifiedNominal {
                                            modifier, head, ..
                                        } = part
                                            && modifier.realize(common::lexicon()).unwrap()
                                                == "additional"
                                        {
                                            heads.insert(symbol_nominal_head(head).clone());
                                        }
                                    })
                                    .unwrap();
                                assert_eq!(heads, BTreeSet::from([expected.clone()]));
                                objects.push(object.as_ref().clone());
                            }
                        }
                    }
                })
                .unwrap();
            assert!(!objects.is_empty(), "{card}");
        }
    }
}

#[test]
#[ignore = "english-v3-mana-symbol-verbless-subject: MS4 needs a governed Mana Phrase Subject route that keeps bare {X} and {2} outside ManaPhrase"]
fn cost_notation_is_the_subject_of_the_licensed_locative_small_clause() {
    for (card, text, location) in [
        (
            "Matterbending Mage",
            "Whenever you cast a spell with {X} in its mana cost, this creature can't be blocked this turn.",
            "in its mana cost",
        ),
        (
            "Gaddock Teeg",
            "Noncreature spells with {X} in their mana costs can't be cast.",
            "in their mana costs",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        let predicates = common::readings(location, Category::LocativeComplement);
        assert!(!predicates.is_empty());
        for reading in readings {
            let mut subjects = BTreeSet::new();
            let mut locations = BTreeSet::new();
            reading
                .visit(&mut |node| {
                    if node.category() == Category::PrepositionPhrase
                        && node.realize(common::lexicon()).unwrap()
                            == format!("with {{X}} {location}")
                    {
                        node.visit(&mut |part| {
                            if part.category() == Category::ManaPhrase {
                                subjects.insert(part.realize(common::lexicon()).unwrap());
                            }
                            if part.category() == Category::LocativeComplement {
                                locations.insert(part.clone());
                            }
                        })
                        .unwrap();
                    }
                })
                .unwrap();
            assert_eq!(subjects, BTreeSet::from(["{X}".to_owned()]));
            assert!(!locations.is_empty());
            assert!(locations.is_subset(&predicates));
        }
    }
}

#[test]
fn bare_symbols_keep_their_notation_routes_and_cannot_be_noun_phrases() {
    for text in ["{G}", "{G}{G}", "{1}{G}", "{X}", "{E}", "{E}{E}"] {
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::new()
        );
    }
    // Valleymaker, Cabal Stronghold and the ticket's payment control.
    for (card, text) in [
        (
            "Valleymaker",
            "{T}, Sacrifice a Forest: Choose a player. That player adds {G}{G}{G}.",
        ),
        (
            "Cabal Stronghold",
            "{3}, {T}: Add {B} for each basic Swamp you control.",
        ),
        ("control", "You may pay {X}."),
    ] {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty());
        for reading in readings {
            let mut nominal_heads = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::MeasuredSymbolNominal { .. } = node {
                        nominal_heads.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(nominal_heads, Vec::<Reading>::new());
        }
    }
}
