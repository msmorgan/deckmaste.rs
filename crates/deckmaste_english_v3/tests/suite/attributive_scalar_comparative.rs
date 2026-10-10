use crate::common::{lexicon, readings, readings_on_card};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

fn word(owner: &str, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

#[test]
fn implicit_standards_are_adjective_modifiers_of_property_nouns() {
    // Constituents attested on Rushed Rebirth, Prehistoric Pet, and V.A.T.S.
    for (text, adjective, noun, owner) in [
        ("lesser mana value", "lesser", "mana value", "Lesser"),
        ("greater power", "greater", "power", "Greater"),
        ("equal toughness", "equal", "toughness", "Equal"),
    ] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let Reading::BareNounPhrase { head: nominal, .. } = value else {
                panic!("the comparative property must form a bare noun phrase");
            };
            let Reading::PremodifiedNominal { modifier, head, .. } = nominal.as_ref() else {
                panic!("the comparative adjective must premodify the property noun");
            };
            assert_eq!(
                **modifier,
                Reading::IntransitiveAdjective {
                    form: 0,
                    head: word(&format!("vocab:ScalarDegree/{owner}"), Some(1)),
                }
            );
            assert_eq!(modifier.realize(lexicon()).unwrap(), adjective);
            assert_eq!(head.realize(lexicon()).unwrap(), noun);
        }
    }
    // Reinterpret and Body Launderer share one nominal head under adjective coordination.
    for text in ["equal or lesser mana value", "equal or lesser power"] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty());
        for value in values {
            let Reading::BareNounPhrase { head: nominal, .. } = value else {
                unreachable!()
            };
            let Reading::PremodifiedNominal { modifier, .. } = nominal.as_ref() else {
                unreachable!()
            };
            let Reading::Coordination {
                category,
                left,
                right,
                ..
            } = modifier.as_ref()
            else {
                panic!("the adjective governors must coordinate before their shared head");
            };
            assert_eq!(*category, Category::AdjectivePhrase);
            assert_eq!(left.realize(lexicon()).unwrap(), "equal");
            assert_eq!(right.realize(lexicon()).unwrap(), "lesser");
        }
    }
}

#[test]
fn overt_standards_belong_to_one_discontinuous_governor() {
    // Conformer Shuriken, Triple Triad, and Saruman of Many Colors.
    for (text, governor, property, standard) in [
        (
            "greater power than this creature",
            "greater",
            "power",
            "this creature",
        ),
        ("lesser mana value than it", "lesser", "mana value", "it"),
        (
            "equal or lesser mana value than that spell",
            "equal or lesser",
            "mana value",
            "that spell",
        ),
    ] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let Reading::BareNounPhrase { head: nominal, .. } = value else {
                unreachable!()
            };
            let Reading::DiscontinuousComparativeNominal {
                category,
                governor: compared,
                head,
                marker,
                complement,
                ..
            } = nominal.as_ref()
            else {
                panic!("the standard must be the governor's comparative complement");
            };
            assert_eq!(*category, Category::Nominal);
            assert_eq!(
                compared.category(),
                Category::DiscontinuousComparativeGovernor
            );
            assert_eq!(compared.realize(lexicon()).unwrap(), governor);
            assert_eq!(head.realize(lexicon()).unwrap(), property);
            assert_eq!(*marker, word("vocab:Preposition/Than", None));
            assert_eq!(complement.category(), Category::AccusativePhrase);
            assert_eq!(complement.realize(lexicon()).unwrap(), standard);
            if governor == "equal or lesser" {
                let expected = Reading::Coordination {
                    category: Category::DiscontinuousComparativeGovernor,
                    form: 0,
                    left: Box::new(Reading::DiscontinuousComparativeGovernorHead {
                        form: 0,
                        head: word("vocab:ScalarDegree/Equal", Some(0)),
                    }),
                    coordinator: word("vocab:Coordinator/Or", None),
                    right: Box::new(Reading::DiscontinuousComparativeGovernorHead {
                        form: 0,
                        head: word("vocab:ScalarDegree/Lesser", Some(0)),
                    }),
                };
                assert_eq!(**compared, expected);
            }
        }
    }
}

#[test]
fn authentic_witnesses_roundtrip_and_controls_keep_their_complement_roles() {
    for (card, text) in [
        (
            "Reinterpret",
            "Counter target spell. You may cast a spell with equal or lesser mana value from your hand without paying its mana cost.",
        ),
        (
            "Body Launderer",
            "When this creature dies, return another target non-Rogue creature card with equal or lesser power from your graveyard to the battlefield.",
        ),
        (
            "Prehistoric Pet",
            "This creature can't be blocked by creatures with greater power.",
        ),
        (
            "Rushed Rebirth",
            "Choose target creature. When that creature dies this turn, search your library for a creature card with lesser mana value, put it onto the battlefield tapped, then shuffle.",
        ),
    ] {
        assert!(
            !readings_on_card(card, text, Category::Document).is_empty(),
            "{card}"
        );
    }
    // The authentic comparative clause; the face's separate "difference" gap is recorded.
    assert!(
        !readings_on_card(
            "Conformer Shuriken",
            "that creature has greater power than this creature",
            Category::FiniteClause,
        )
        .is_empty()
    );
    for text in ["power greater than 3", "mana value less than or equal to X"] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty());
        for value in values {
            let Reading::MeasuredAttribute { quantity, .. } = value else {
                panic!("postpositive scalar comparisons must remain property values");
            };
            let Reading::ScalarMeasureValue {
                category,
                value: scalar,
                ..
            } = quantity.as_ref()
            else {
                panic!("the property specification must retain its scalar-value projection");
            };
            assert_eq!(*category, Category::PropertyValue);
            let Reading::ComparativeScalarValue {
                value: compared, ..
            } = scalar.as_ref()
            else {
                panic!("the comparative must retain its selected complement");
            };
            assert_eq!(compared.category(), Category::ComparativeAdjectivePhrase);
        }
    }
    for text in ["5 or greater", "five or greater"] {
        let values = readings(text, Category::ScalarValue);
        assert!(!values.is_empty());
        for value in values {
            let Reading::AdjectivalScalarCoordination { right, .. } = value else {
                panic!("greater must retain its scalar conjunct function");
            };
            assert_eq!(right, word("vocab:ScalarDegree/Greater", Some(0)));
        }
    }
    // Amrou Kithkin: "or greater" compares the property value, not "blocked".
    let amrou = readings(
        "This creature can't be blocked by creatures with power 3 or greater.",
        Category::Document,
    );
    assert!(!amrou.is_empty());
    for value in amrou {
        let mut scalar_comparisons = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::AdjectivalScalarCoordination { right, .. } = node {
                    scalar_comparisons.push(right.clone());
                }
            })
            .unwrap();
        assert_eq!(
            scalar_comparisons,
            vec![word("vocab:ScalarDegree/Greater", Some(0))]
        );
    }
    let equal = readings("equal to X", Category::AdjectivePhrase);
    assert!(!equal.is_empty());
    for value in equal {
        let Reading::ComparativeAdjective { phrase, .. } = value else {
            panic!("to X must remain a selected comparative complement");
        };
        assert_eq!(phrase.category(), Category::ComparativeAdjectivePhrase);
    }
}

#[test]
fn independently_constructed_attributive_property_roundtrips() {
    // Prehistoric Pet's comparative property nominal, independent of parsing.
    let mut power = word("lexeme:CommonNoun/Power", None);
    let LexicalReading::Word(lexical) = &mut power.value else { unreachable!() };
    lexical.form = WordForm::Singular;
    lexical.features.number = Some(Number::Singular);
    power.countability = Some(true);
    let value = Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::IntransitiveAdjective {
            form: 0,
            head: word("vocab:ScalarDegree/Greater", Some(1)),
        }),
        head: Box::new(Reading::Noun {
            form: 0,
            head: power,
        }),
    };
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), "greater power");
    assert_eq!(readings("greater power", Category::Nominal), [value].into());
}

#[test]
fn complement_free_comparatives_do_not_enter_predicative_function() {
    assert!(readings("Its power is greater.", Category::Sentence).is_empty());
    assert!(readings("Its power is equal or lesser.", Category::Sentence).is_empty());
}
