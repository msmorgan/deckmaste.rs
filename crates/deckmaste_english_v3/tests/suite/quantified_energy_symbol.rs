use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn energy_head() -> Reading {
    Reading::MeasuredSymbolNominal {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:FixedCostSymbol/E".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(false),
        },
    }
}

fn measured_energy(value: i32) -> Reading {
    Reading::CardinalMeasuredNounPhrase {
        form: 0,
        quantity: Box::new(Reading::Cardinal {
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
        head: Box::new(energy_head()),
    }
}

#[test]
fn independently_constructed_energy_measurement_preserves_both_roundtrip_laws() {
    // Aethersquall Ancient's Cost supplies the attested constituent.
    let expected = measured_energy(8);
    let observed = common::readings("eight {E}", Category::NounPhrase);
    assert_eq!(observed, BTreeSet::from([expected.clone()]));
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "eight {E}");
    let parsed = observed.get(&expected).unwrap();
    let mut actual_nodes = Vec::new();
    parsed
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    let mut expected_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
}

fn cardinal_witness(card: &str, text: &str, value: i32) {
    let readings = common::readings_on_card(card, text, Category::Document);
    assert!(!readings.is_empty(), "{card}");
    for reading in readings {
        let mut measurements = BTreeSet::new();
        reading
            .visit(&mut |node| {
                if let Reading::CardinalMeasuredNounPhrase { .. } = node {
                    measurements.insert(node.clone());
                }
            })
            .unwrap();
        assert_eq!(measurements, BTreeSet::from([measured_energy(value)]));
    }
}

#[test]
fn aethertide_whale_gets_a_cardinal_measurement_with_a_symbol_head() {
    cardinal_witness(
        "Aethertide Whale",
        "When this creature enters, you get six {E}.",
        6,
    );
}

#[test]
fn aethersquall_ancient_pays_a_cardinal_measurement_as_a_cost() {
    cardinal_witness(
        "Aethersquall Ancient",
        "Pay eight {E}: Return all other creatures to their owners' hands. Activate only as a sorcery.",
        8,
    );
}

#[test]
fn roil_cartographer_pays_a_measurement_beside_a_symbol_cost() {
    cardinal_witness(
        "Roil Cartographer",
        "{T}, Pay six {E}: Draw three cards.",
        6,
    );
}

#[test]
fn overclocked_electromancer_gets_a_variable_measurement_with_a_symbol_head() {
    let text = "Whenever this creature deals combat damage to a creature, if that creature was dealt excess damage this turn, you get X {E}, where X is that excess damage.";
    let readings = common::readings_on_card("Overclocked Electromancer", text, Category::Document);
    assert!(!readings.is_empty());
    for reading in readings {
        let mut heads = BTreeSet::new();
        reading
            .visit(&mut |node| {
                if let Reading::CardinalMeasuredNounPhrase { quantity, head, .. } = node {
                    assert_eq!(quantity.realize(common::lexicon()).unwrap(), "X");
                    heads.insert(head.as_ref().clone());
                }
            })
            .unwrap();
        assert_eq!(heads, BTreeSet::from([energy_head()]));
    }
}

#[test]
fn bare_energy_symbols_keep_notation_without_a_measured_nominal_route() {
    // QE3's attested controls; every admitted analysis must keep the notation route.
    for (card, text) in [
        (
            "Nissa, Worldsoul Speaker",
            "Landfall — Whenever a land you control enters, you get {E}{E}.",
        ),
        ("Solstice Zealot", "{T}, Pay {E}: Tap target creature."),
        ("Inventor's Axe", "Equip—Pay {E}{E}."),
        (
            "Peema Aether-Seer",
            "When this creature enters, you get an amount of {E} equal to the greatest power among creatures you control.",
        ),
        ("Aether Spike", "You may pay any amount of {E}."),
    ] {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        for reading in readings {
            let mut measured_heads = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::MeasuredSymbolNominal { .. } = node {
                        measured_heads.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(measured_heads, Vec::<Reading>::new(), "{card}");
        }
    }
}

#[test]
fn an_energy_nominal_requires_a_dependent_to_form_a_noun_phrase() {
    assert_eq!(
        common::readings("{E}", Category::Nominal),
        BTreeSet::from([energy_head()])
    );
    for text in ["{E}", "{E}{E}"] {
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::new()
        );
    }
    let bare = Reading::BareNounPhrase {
        form: 0,
        head: Box::new(energy_head()),
    };
    assert!(bare.realize(common::lexicon()).is_err());
}
