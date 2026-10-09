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

fn prevention_witness(index: usize) {
    let witnesses = [
        (
            "Hold at Bay",
            "Prevent the next 7 damage that would be dealt to any target this turn.",
            Category::Sentence,
            "7 damage",
        ),
        (
            "Esper Battlemage",
            "{W}, {T}: Prevent the next 2 damage that would be dealt to you this turn.",
            Category::Document,
            "2 damage",
        ),
        (
            "Anoint",
            "Prevent the next 3 damage that would be dealt to target creature this turn.",
            Category::Sentence,
            "3 damage",
        ),
        (
            "Barbed Wire",
            "{2}: Prevent the next 1 damage that would be dealt by this artifact this turn.",
            Category::Document,
            "1 damage",
        ),
        (
            "Kitsune Healer",
            "{T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            Category::Document,
            "1 damage",
        ),
    ];
    let (card, text, category, nominal) = witnesses[index];
    assert!(
        !common::readings_on_card(card, text, category).is_empty(),
        "{card}"
    );
    common::assert_constituents(text, category, &[(Category::Nominal, nominal)]);
}

#[test]
fn hold_at_bay_keeps_its_measured_nominal_and_relative_postmodifier() {
    prevention_witness(0);
}

#[test]
fn esper_battlemage_keeps_its_measured_nominal_and_relative_postmodifier() {
    prevention_witness(1);
}

#[test]
fn anoint_keeps_its_measured_nominal_and_relative_postmodifier() {
    prevention_witness(2);
}

#[test]
fn barbed_wire_keeps_its_measured_nominal_and_relative_postmodifier() {
    prevention_witness(3);
}

#[test]
fn kitsune_healer_keeps_its_measured_nominal_and_relative_postmodifier() {
    prevention_witness(4);
}

fn amount(owner: &str) -> Reading {
    Reading::MeasuredNounPhrase {
        form: 0,
        quantity: Box::new(Reading::UngroupedScalarNumeral {
            form: 0,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 3,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
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
            countability: Some(false),
        },
    }
}

#[test]
fn independent_bare_measurements_have_one_constituency_and_both_roundtrip_laws() {
    // Lightning Bolt and mana-production instructions supply the constituents.
    for (text, owner) in [
        ("3 damage", "lexeme:CommonNoun/Damage"),
        ("3 mana", "lexeme:CommonNoun/Mana"),
    ] {
        let expected = amount(owner);
        let observed = common::readings(text, Category::NounPhrase);
        assert_eq!(
            observed,
            std::collections::BTreeSet::from([expected.clone()])
        );
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
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
        let mut owners = Vec::new();
        parsed
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value {
                    owners.push(value.lexeme.to_string());
                }
            })
            .unwrap();
        assert_eq!(owners, [owner]);
    }
    assert_eq!(common::readings("Add 3 mana.", Category::Sentence).len(), 1);
    assert_eq!(
        common::readings("Prevent 3 damage.", Category::Sentence).len(),
        1
    );
    assert_eq!(
        common::readings("deals 3 damage", Category::FinitePredicate).len(),
        1
    );
}

#[test]
fn numeral_measurements_allow_determiners_and_postmodifiers_without_count_plural() {
    common::assert_constituents(
        "the next 3 mana",
        Category::NounPhrase,
        &[(Category::Nominal, "3 mana")],
    );
    common::assert_constituents(
        "the next 3 damage that would be dealt to you this turn",
        Category::NounPhrase,
        &[(
            Category::Nominal,
            "damage that would be dealt to you this turn",
        )],
    );
    for text in [
        "3 damages",
        "3 manas",
        "3 damage that would be dealt to you this turn",
        "3 damage to any target",
        "3 mana of any color",
        "two damage",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}
