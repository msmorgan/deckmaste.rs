use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, FrameItem, FrameSlot, LexicalReading, LexicalValue, Numeral, Relation,
    SurfaceCase, WordForm,
};

use crate::common;

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn symbol(owner: &str, countability: Option<bool>) -> Reading {
    let mut head = word(owner);
    head.countability = countability;
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::MeasuredSymbolNominal {
                form: 0,
                head: Box::new(Reading::NominalSymbol { form: 0, head }),
                rest: vec![],
            }),
        }),
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::CardinalDeterminer {
        form: 0,
        value: Box::new(Reading::Cardinal {
            form: 0,
            modifier: None,
            head: Word {
                value: LexicalReading::Numeral {
                    value,
                    notation: Numeral::Cardinal,
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
    }
}

fn degree(owner: &str) -> Reading {
    Reading::DegreeDeterminativePhrase {
        form: 0,
        modifier: word("vocab:SingularDemonstrative/That"),
        head: word(owner),
        arithmetic_modifier: None,
    }
}

fn quantified(quantity: Reading, head: Reading) -> Reading {
    Reading::QuantifiedNounPhrase {
        form: 0,
        quantity: Box::new(quantity),
        head: Box::new(head),
    }
}

fn one_or_more() -> Reading {
    let Reading::CardinalDeterminer { value, .. } = cardinal(1) else {
        unreachable!()
    };
    Reading::ComparativeQuantityDeterminer {
        form: 0,
        value: Box::new(Reading::NumeralComparativeCoordination {
            form: 0,
            left: Box::new(Reading::CardinalQuantityConjunct { form: 0, value }),
            marker: word("vocab:Coordinator/Or"),
            right: word("vocab:ComparativeQuantifier/More"),
        }),
    }
}

fn assert_laws(text: &str, expected: &Reading) {
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    let observed = common::readings(text, Category::NounPhrase);
    assert_eq!(observed, BTreeSet::from([expected.clone()]), "{text}");
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let observed = observed.into_iter().next().unwrap();
    let mut observed_words = Vec::new();
    observed
        .visit_words(&mut |word| observed_words.push(word.clone()))
        .unwrap();
    assert_eq!(observed_words, expected_words);
}

#[test]
fn independent_quantifiers_have_one_symbol_head_and_preserve_both_laws() {
    // Castle Garenbrig, Irencrag Feat, Sakiko, Sphinx of the Revelation
    // and Aethersquall Ancient supply these attested constituents.
    for (text, expected) in [
        (
            "one or more {E}",
            quantified(
                one_or_more(),
                symbol("vocab:FixedCostSymbol/E", Some(false)),
            ),
        ),
        (
            "six {G}",
            quantified(cardinal(6), symbol("vocab:FixedCostSymbol/Green", None)),
        ),
        (
            "seven {R}",
            quantified(cardinal(7), symbol("vocab:FixedCostSymbol/Red", None)),
        ),
        (
            "that much {G}",
            quantified(
                degree("vocab:Determinative/Much"),
                symbol("vocab:FixedCostSymbol/Green", None),
            ),
        ),
        (
            "that many {E}",
            quantified(
                degree("vocab:Determinative/Many"),
                symbol("vocab:FixedCostSymbol/E", Some(false)),
            ),
        ),
        (
            "eight {E}",
            quantified(cardinal(8), symbol("vocab:FixedCostSymbol/E", Some(false))),
        ),
    ] {
        assert_laws(text, &expected);
    }
}

#[test]
fn witnesses_quantify_the_symbol_inside_the_selected_object() {
    for (card, text, object_text, verb) in [
        (
            "Castle Garenbrig",
            "{2}{G}{G}, {T}: Add six {G}.",
            "six {G}",
            "core-verb:Add",
        ),
        (
            "Irencrag Feat",
            "Add seven {R}.",
            "seven {R}",
            "core-verb:Add",
        ),
        (
            "Sakiko, Mother of Summer",
            "Whenever a creature you control deals combat damage to a player, add that much {G}.",
            "that much {G}",
            "core-verb:Add",
        ),
        (
            "Sphinx of the Revelation",
            "Whenever you gain life, you get that many {E}.",
            "that many {E}",
            "core-verb:Get",
        ),
        (
            "Pia Nalaar, Chief Mechanic",
            "At the beginning of your end step, you may pay one or more {E}.",
            "one or more {E}",
            "core-verb:Pay",
        ),
    ] {
        let expected = common::readings(object_text, Category::NounPhrase);
        assert!(!expected.is_empty(), "{card}");
        for object in &expected {
            let Reading::QuantifiedNounPhrase { quantity, head, .. } = object else {
                panic!("{card}: Object must be a quantified Noun Phrase");
            };
            assert_eq!(quantity.category(), Category::QuantitativeDeterminer);
            let Reading::MeasuredSymbolNominal { head, rest, .. } = common::nominal_core(head)
            else {
                panic!("{card}: the symbol must head its Nominal");
            };
            assert_eq!(rest.as_slice(), []);
            assert_eq!(head.category(), Category::NominalSymbol);
        }
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        for reading in readings {
            let mut objects = BTreeSet::new();
            reading
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        head, complements, ..
                    } = node
                        && let LexicalReading::Word(value) = &head.value
                        && value.lexeme.as_str() == verb
                    {
                        let frame = &common::lexicon().lexemes()[&value.lexeme].properties.frames
                            [head.frame.unwrap()];
                        assert!(frame.items.contains(&FrameItem::Argument(FrameSlot {
                            relation: Relation::Object,
                            category: "NounPhrase".into(),
                        })));
                        for complement in complements {
                            if let FrameValue::Argument(object) = complement {
                                object
                                    .visit(&mut |part| {
                                        if part.category() == Category::NounPhrase
                                            && part.realize(common::lexicon()).unwrap()
                                                == object_text
                                        {
                                            objects.insert(part.clone());
                                        }
                                    })
                                    .unwrap();
                            }
                        }
                    }
                })
                .unwrap();
            assert!(!objects.is_empty(), "{card}");
            assert!(objects.is_subset(&expected), "{card}");
        }
    }
}

#[test]
fn declared_mana_class_supplies_the_quantified_paradigm() {
    // Derive the paradigm from the class, including hybrid and Phyrexian
    // members, rather than enumerating the colours in the witnesses.
    for lexeme in common::lexicon().lexemes().values().filter(|lexeme| {
        lexeme
            .properties
            .features
            .get("ManaSymbolUse")
            .map(String::as_str)
            == Some("Yes")
            && lexeme
                .properties
                .features
                .get("SymbolNominalUse")
                .map(String::as_str)
                == Some("Count")
    }) {
        let expected = quantified(cardinal(6), symbol(lexeme.id.as_str(), None));
        let text = expected.realize(common::lexicon()).unwrap();
        assert_laws(&text, &expected);
    }
}

#[test]
fn generic_and_bare_notation_keep_their_protected_routes() {
    for text in [
        "six {2}",
        "six {X}",
        "that much {2}",
        "that many {X}",
        "{G}",
        "{G}{G}{G}",
        "{E}",
        "{E}{E}",
    ] {
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::new(),
            "{text}"
        );
    }
    for text in ["{2}", "{X}"] {
        assert_eq!(
            common::readings(text, Category::ManaPhrase),
            BTreeSet::new(),
            "{text}"
        );
    }
    for (text, notation, category) in [
        ("Add {G}{G}{G}.", "{G}{G}{G}", Category::ManaPhrase),
        ("You get {E}{E}.", "{E}{E}", Category::CostSymbols),
    ] {
        let readings = common::readings(text, Category::Document);
        assert!(!readings.is_empty());
        for reading in readings {
            let mut symbols = Vec::new();
            let mut nominals = Vec::new();
            reading
                .visit(&mut |node| {
                    if node.category() == category {
                        symbols.push(node.realize(common::lexicon()).unwrap());
                    }
                    if let Reading::MeasuredSymbolNominal { .. } = node {
                        nominals.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(symbols, [notation]);
            assert_eq!(nominals, Vec::<Reading>::new());
        }
    }
}
