mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

#[test]
fn property_values_are_dependents_of_the_property_noun() {
    // Disembowel, Mental Misstep, Easy Prey, Kor Line-Slinger,
    // Retribution of the Meek and Valorous Stance, with bare-value controls.
    for (text, property, value) in [
        ("mana value X", "mana value", "X"),
        ("mana value 1", "mana value", "1"),
        ("mana value 2 or less", "mana value", "2 or less"),
        ("mana value 2", "mana value", "2"),
        ("power 3 or less", "power", "3 or less"),
        ("power 3", "power", "3"),
        ("power 4 or greater", "power", "4 or greater"),
        ("power 4", "power", "4"),
        ("toughness 4 or greater", "toughness", "4 or greater"),
        ("toughness 4", "toughness", "4"),
    ] {
        let values = readings(text, Category::NounPhrase);
        assert_eq!(values.len(), 1, "{text}: {values:?}");
        let Reading::MeasuredAttribute { head, quantity, .. } = values.first().unwrap() else {
            panic!("property noun and its dependent value must remain constituents")
        };
        assert_eq!(lexicon().realize(&head.value).unwrap(), property);
        assert_eq!(quantity.realize(lexicon()).unwrap(), value);
    }
}

#[test]
fn with_property_values_postmodifies_the_target_nominal() {
    for (text, nominal, property) in [
        (
            "Destroy target creature with mana value X.",
            "creature with mana value X",
            "mana value X",
        ),
        (
            "Counter target spell with mana value 1.",
            "spell with mana value 1",
            "mana value 1",
        ),
        (
            "Destroy target creature with mana value 2 or less.",
            "creature with mana value 2 or less",
            "mana value 2 or less",
        ),
        (
            "{T}: Tap target creature with power 3 or less.",
            "creature with power 3 or less",
            "power 3 or less",
        ),
        (
            "Destroy all creatures with power 4 or greater.",
            "creatures with power 4 or greater",
            "power 4 or greater",
        ),
        (
            "Destroy target creature with toughness 4 or greater.",
            "creature with toughness 4 or greater",
            "toughness 4 or greater",
        ),
    ] {
        assert_constituents(
            text,
            Category::Document,
            &[
                (Category::Nominal, nominal),
                (Category::NounPhrase, property),
            ],
        );
    }
}

#[test]
fn scalar_values_reuse_one_coordination_without_determiner_or_clause_overlap() {
    // Here Comes a New Hero!, Easy Prey and Retribution of the Meek.
    for text in ["X", "2", "2 or less", "X or less", "4 or greater"] {
        assert_eq!(
            readings(text, Category::ScalarPropertyValue).len(),
            1,
            "{text}"
        );
    }
    let quantity = readings("2 or less", Category::ScalarPropertyValue);
    let expected = readings("2 or less", Category::ComparativeQuantity);
    let Reading::ScalarPropertyCoordination { value, .. } = quantity.first().unwrap() else {
        panic!("reuse the prerequisite's coordination")
    };
    assert!(expected.contains(value.as_ref()));
    for text in ["2 or less", "X or less", "4 or greater"] {
        assert!(readings(text, Category::Clause).is_empty(), "{text}");
        assert!(readings(text, Category::Cardinal).is_empty(), "{text}");
    }
    assert!(readings("4 or greater", Category::QuantitativeDeterminer).is_empty());
    for text in [
        "2 nor less",
        "4 nor greater",
        "2 and less",
        "4 and greater",
        "more than one",
        "2/2",
    ] {
        assert!(
            readings(text, Category::ScalarPropertyValue).is_empty(),
            "{text}"
        );
    }
    // The licence belongs to scalar property nouns, not all nouns.
    for text in ["creature 2", "mana 2", "mana values 2"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

fn word(owner: &str, form: deckmaste_lexical::WordForm) -> deckmaste_english_v3::grammar::Word {
    let value = lexicon()
        .values()
        .find(|value| value.lexeme.as_str() == owner && value.form == form)
        .unwrap();
    let lexeme = &lexicon().lexemes()[&value.lexeme];
    deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Word(value.clone()),
        frame: (!lexeme.properties.frames.is_empty()).then_some(0),
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn numeric(value: i32) -> deckmaste_english_v3::grammar::Word {
    deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Numeral {
            value,
            notation: deckmaste_lexical::Numeral::Arabic(false),
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn assert_laws(value: &Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert!(
        parsed.contains(value),
        "independent value missing for {text:?}"
    );
    let observed = parsed.get(value).unwrap();
    let (mut nodes, mut observed_nodes, mut words, mut observed_words) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    observed
        .visit(&mut |node| observed_nodes.push(node.clone()))
        .unwrap();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    observed
        .visit_words(&mut |word| observed_words.push(word.clone()))
        .unwrap();
    assert_eq!(observed_nodes, nodes);
    assert_eq!(observed_words, words);
}

#[test]
fn independently_constructed_property_values_preserve_both_roundtrip_laws() {
    use deckmaste_lexical::WordForm;
    // Constituents of Disembowel, Mental Misstep, Easy Prey and Retribution.
    for (property, surface, quantity) in [
        (
            "ManaValue",
            "mana value X",
            Reading::ScalarPropertyMeasure {
                form: 0,
                value: Box::new(Reading::ScalarVariable {
                    form: 0,
                    head: word("vocab:Variable/X", WordForm::Invariant),
                }),
            },
        ),
        (
            "ManaValue",
            "mana value 1",
            Reading::ScalarPropertyMeasure {
                form: 0,
                value: Box::new(Reading::UngroupedScalarNumeral {
                    form: 0,
                    head: numeric(1),
                }),
            },
        ),
        (
            "Power",
            "power 3",
            Reading::ScalarPropertyMeasure {
                form: 0,
                value: Box::new(Reading::UngroupedScalarNumeral {
                    form: 0,
                    head: numeric(3),
                }),
            },
        ),
        (
            "ManaValue",
            "mana value 2 or less",
            Reading::ScalarPropertyCoordination {
                form: 0,
                value: Box::new(Reading::NumeralComparativeCoordination {
                    form: 0,
                    left: Box::new(Reading::NumericQuantityConjunct {
                        form: 0,
                        head: numeric(2),
                    }),
                    marker: word("vocab:Coordinator/Or", WordForm::Invariant),
                    right: word("vocab:Determinative/Less", WordForm::Invariant),
                }),
            },
        ),
        (
            "Power",
            "power 4 or greater",
            Reading::AdjectivalScalarCoordination {
                form: 0,
                left: Box::new(Reading::NumericQuantityConjunct {
                    form: 0,
                    head: numeric(4),
                }),
                marker: word("vocab:Coordinator/Or", WordForm::Invariant),
                right: word("vocab:ScalarDegree/Greater", WordForm::Invariant),
            },
        ),
    ] {
        let value_surface = surface
            .split_once(if property == "ManaValue" { "mana value " } else { "power " })
            .unwrap()
            .1;
        assert_laws(&quantity, value_surface, Category::ScalarPropertyValue);
        let np = Reading::MeasuredAttribute {
            form: 0,
            head: word(&format!("lexeme:CommonNoun/{property}"), WordForm::Singular),
            quantity: Box::new(quantity),
        };
        assert_laws(&np, surface, Category::NounPhrase);
        let pp = Reading::PrepositionPhrase {
            form: 0,
            head: word("vocab:Preposition/With", WordForm::Invariant),
            complement: Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(np),
            }),
        };
        assert_laws(&pp, &format!("with {surface}"), Category::PrepositionPhrase);
    }
}
