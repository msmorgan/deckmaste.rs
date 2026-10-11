use crate::common;
use crate::scalar_property_values::{assert_laws, word};
use deckmaste_english_v3::grammar::{Category, Reading};
use deckmaste_lexical::WordForm;

fn assert_property_predication(
    card: Option<&str>,
    text: &str,
    category: Category,
    head_text: &str,
    equality_text: &str,
    quantified: bool,
) {
    let readings = match card {
        Some(card) => common::readings_on_card(card, text, category),
        None => common::readings(text, category),
    };
    assert!(!readings.is_empty(), "{text}");
    let mut full_comparison = false;
    for reading in readings {
        let mut predications = 0;
        let mut adjuncts = 0;
        reading
            .visit(&mut |node| {
                if let Reading::MeasuredAttribute { head, quantity, .. } = node
                    && head.realize(common::lexicon()).unwrap() == head_text
                {
                    let mut coordinated_head = false;
                    head.visit(&mut |head_node| {
                        if let Reading::Coordination { category, .. } = head_node {
                            assert_eq!(*category, Category::Nominal);
                            coordinated_head = true;
                        }
                    })
                    .unwrap();
                    assert!(
                        coordinated_head,
                        "the predicand is the property coordination"
                    );
                    let Reading::ScalarMeasureValue {
                        category, value, ..
                    } = quantity.as_ref()
                    else {
                        panic!("the AdjP must retain the existing property-value route")
                    };
                    assert_eq!(*category, Category::PropertyValue);
                    let Reading::ComparativeScalarValue { value, .. } = value.as_ref() else {
                        panic!("the value retains its comparative AdjP")
                    };
                    assert_eq!(value.category(), Category::ComparativeAdjectivePhrase);
                    full_comparison |= value.realize(common::lexicon()).unwrap() == equality_text;
                    match (quantified, value.as_ref()) {
                        (
                            true,
                            Reading::QuantifiedComparativeAdjectivePhrase {
                                quantifier, value, ..
                            },
                        ) => {
                            assert_eq!(
                                common::lexicon().realize(&quantifier.value).unwrap(),
                                "each"
                            );
                            assert_eq!(value.category(), Category::ComparativeAdjectivePhrase);
                        }
                        (false, Reading::ComparativeAdjectivePhrase { .. }) => {}
                        _ => panic!("quantification belongs inside the property AdjP"),
                    }
                    predications += 1;
                }
                if let Reading::QuantifiedComparativeAdjectivePhrase { .. } = node {
                    adjuncts += 1;
                }
            })
            .unwrap();
        assert_eq!(
            predications, 1,
            "every Reading predicates the complete coordinated head"
        );
        assert_eq!(
            adjuncts,
            usize::from(quantified),
            "one quantificational route"
        );
    }
    assert!(full_comparison, "the entire comparison remains admitted");
}

#[test]
fn ticket_witnesses_keep_the_coordinated_predicand_and_internal_adjunct() {
    for (card, text, category, head, equality) in [
        (
            "Karn's Touch",
            "Target noncreature artifact becomes an artifact creature with power and toughness each equal to its mana value until end of turn.",
            Category::Sentence,
            "power and toughness",
            "each equal to its mana value",
        ),
        (
            "Porcelain Gallery",
            "Creatures you control have base power and toughness each equal to the number of creatures you control.",
            Category::Sentence,
            "base power and toughness",
            "each equal to the number of creatures you control",
        ),
        (
            "Trench Gorger",
            "If you do, this creature has base power and base toughness each equal to the number of cards exiled this way.",
            Category::Sentence,
            "base power and base toughness",
            "each equal to the number of cards exiled this way",
        ),
        (
            "Sydri, Galvanic Genius",
            "{U}: Target noncreature artifact becomes an artifact creature with power and toughness each equal to its mana value until end of turn.",
            Category::Document,
            "power and toughness",
            "each equal to its mana value",
        ),
    ] {
        assert_property_predication(Some(card), text, category, head, equality, true);
    }
}

#[test]
fn equality_without_each_also_predicates_the_coordination() {
    // PE4 controls: the same construction with its quantificational Adjunct omitted.
    for text in [
        "It becomes an artifact creature with power and toughness equal to its mana value.",
        "It has power and toughness equal to its mana value.",
    ] {
        assert_property_predication(
            None,
            text,
            Category::Sentence,
            "power and toughness",
            "equal to its mana value",
            false,
        );
    }
}

#[test]
fn quantification_requires_plural_property_agreement_and_one_adjunct() {
    for text in [
        "power each equal to its mana value",
        "toughness each equal to its mana value",
        "power and toughness each each equal to its mana value",
        "creature each equal to its mana value",
        "power and toughness each greater than its mana value",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
    // The reduced predication is property-internal, so it cannot supply a
    // second analysis of the existing post-auxiliary quantification.
    assert!(common::readings("each equal to its mana value", Category::AdjectivePhrase).is_empty());
}

#[test]
fn clausal_single_property_and_slash_pair_controls_keep_their_roles() {
    for (text, category, surface) in [
        (
            "This creature's power and toughness are each equal to the number of cards in your hand.",
            Category::FinitePredicate,
            "are each equal to the number of cards in your hand",
        ),
        (
            "It becomes an artifact creature with power and toughness 4/4.",
            Category::PropertyValue,
            "4/4",
        ),
        (
            "It has base power and toughness 3/3.",
            Category::PropertyValue,
            "3/3",
        ),
        (
            "Creatures you control have power equal to the number of creatures you control.",
            Category::NounPhrase,
            "power equal to the number of creatures you control",
        ),
    ] {
        common::assert_constituents(text, Category::Sentence, &[(category, surface)]);
    }
    // Refining equality does not distribute a plain scalar or an ordering
    // comparative across two properties.
    for text in [
        "power and toughness 3",
        "power and toughness greater than its mana value",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn independently_composed_property_predication_preserves_both_roundtrip_laws() {
    // Karn's Touch's constituent; assemble the coordinated head and the
    // quantificational Adjunct independently of the enclosing parsed Reading.
    let noun = |owner| Reading::Noun {
        form: 0,
        head: word(owner, WordForm::Singular),
    };
    let head = Reading::Coordination {
        category: Category::Nominal,
        form: 0,
        left: Box::new(noun("lexeme:CommonNoun/Power")),
        coordinator: word("vocab:Coordinator/And", WordForm::Invariant),
        right: Box::new(noun("lexeme:CommonNoun/Toughness")),
    };
    let comparison = common::readings(
        "equal to its mana value",
        Category::ComparativeAdjectivePhrase,
    )
    .into_iter()
    .next()
    .unwrap();
    let quantified = Reading::QuantifiedComparativeAdjectivePhrase {
        form: 0,
        quantifier: word("vocab:FloatedQuantifier/Each", WordForm::Invariant),
        value: Box::new(comparison.clone()),
    };
    assert_laws(
        &quantified,
        "each equal to its mana value",
        Category::ComparativeAdjectivePhrase,
    );
    for (value, surface) in [
        (comparison, "equal to its mana value"),
        (quantified, "each equal to its mana value"),
    ] {
        let quantity = Reading::ScalarMeasureValue {
            category: Category::PropertyValue,
            form: 0,
            value: Box::new(Reading::ComparativeScalarValue {
                form: 0,
                value: Box::new(value),
            }),
        };
        assert_laws(&quantity, surface, Category::PropertyValue);
        let property = Reading::MeasuredAttribute {
            form: 0,
            head: Box::new(head.clone()),
            quantity: Box::new(quantity),
        };
        assert_laws(
            &property,
            &format!("power and toughness {surface}"),
            Category::NounPhrase,
        );
        if surface.starts_with("each ") {
            let mut singular = property;
            let Reading::MeasuredAttribute { head, .. } = &mut singular else {
                unreachable!()
            };
            **head = noun("lexeme:CommonNoun/Power");
            assert!(singular.admit(common::lexicon()).is_err());
            assert!(singular.realize(common::lexicon()).is_err());
        }
    }
}
