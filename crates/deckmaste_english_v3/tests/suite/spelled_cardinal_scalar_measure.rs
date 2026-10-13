use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Numeral, SurfaceCase, WordForm,
};

fn cardinal(value: i32) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation: Numeral::Cardinal,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

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

fn assert_laws(value: &Reading, text: &str, category: Category) {
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, category).contains(value));
}

fn assert_operand(card: Option<&str>, text: &str, category: Category, on_left: bool) {
    let values = match card {
        Some(card) => common::readings_on_card(card, text, category),
        None => common::readings(text, category),
    };
    assert!(!values.is_empty(), "{text}");
    for value in values {
        let mut operands = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ArithmeticMeasure { left, right, .. } = node {
                    let (spelled, nominal) = if on_left { (left, right) } else { (right, left) };
                    let Reading::CardinalScalarOperand { head, .. } = spelled.as_ref() else {
                        panic!("the spelled cardinal must be the arithmetic operand")
                    };
                    assert_eq!(*head, cardinal(1));
                    let Reading::ScalarNominalOperand { value, .. } = nominal.as_ref() else {
                        panic!("the other operand must retain its scalar Noun Phrase")
                    };
                    assert_eq!(value.category(), Category::NounPhrase);
                    operands.push(head.clone());
                }
            })
            .unwrap();
        assert_eq!(operands, [cardinal(1)]);
    }
}

#[test]
fn journey_on_keeps_the_cardinal_as_the_left_operand() {
    assert_operand(
        Some("Journey On"),
        "Create X Map tokens, where X is one plus the number of opponents who control an artifact.",
        Category::Document,
        true,
    );
}

#[test]
fn welcome_to_sweettooth_keeps_the_cardinal_as_the_left_operand() {
    assert_operand(
        Some("Welcome to Sweettooth"),
        "III — Put X +1/+1 counters on target creature you control, where X is one plus the number of Foods you control.",
        Category::Document,
        true,
    );
}

#[test]
fn bosss_chauffeur_keeps_the_cardinal_as_the_left_operand() {
    assert_operand(
        Some("Boss's Chauffeur"),
        "This creature enters with a number of +1/+1 counters on it equal to one plus the number of other creatures you control.",
        Category::Document,
        true,
    );
}

#[test]
fn mind_burst_keeps_the_cardinal_as_the_left_operand() {
    assert_operand(
        Some("Mind Burst"),
        "Target player discards X cards, where X is one plus the number of cards named Mind Burst in all graveyards.",
        Category::Document,
        true,
    );
}

#[test]
fn sea_gate_restoration_and_profts_eidetic_memory_keep_right_operands() {
    // Arithmetic constituents of the respective Oracle instructions.
    for text in [
        "the number of cards in your hand plus one",
        "the number of cards you've drawn this turn minus one",
    ] {
        assert_operand(None, text, Category::MeasurePhrase, false);
    }
}

#[test]
fn comparisons_keep_the_cardinal_as_the_governors_complement() {
    // Devotion Gods, life-total conditions, and Clockwork Steed/Avian.
    for (text, amount) in [
        ("less than five", 5),
        ("less than seven", 7),
        ("greater than four", 4),
    ] {
        let values = common::readings(text, Category::ComparativeAdjectivePhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let Reading::ComparativeAdjectivePhrase {
                governor,
                complement,
                ..
            } = value
            else {
                panic!("the comparison must retain its governor and standard")
            };
            assert_eq!(governor.category(), Category::ComparativeGovernorHead);
            assert_eq!(
                *complement,
                Reading::CardinalComparativeComplement {
                    form: 0,
                    head: cardinal(amount)
                }
            );
        }
    }
}

#[test]
fn bare_scalar_values_keep_the_existing_cardinal_and_coordination_routes() {
    let five = Reading::ScalarCardinalValue {
        form: 0,
        head: cardinal(5),
    };
    assert_eq!(
        common::readings("five", Category::ScalarValue),
        [five].into()
    );
    for (text, amount) in [("five or greater", 5), ("seven or greater", 7)] {
        let value = Reading::AdjectivalScalarCoordination {
            form: 0,
            left: Box::new(Reading::CardinalQuantityConjunct {
                form: 0,
                value: Box::new(Reading::Cardinal {
                    form: 0,
                    modifier: None,
                    head: cardinal(amount),
                }),
            }),
            marker: word("vocab:Coordinator/Or", None),
            right: word("vocab:ScalarDegree/Greater", Some(0)),
        };
        assert_eq!(
            common::readings(text, Category::ScalarValue),
            [value].into()
        );
    }
    assert!(common::readings("five", Category::MeasurePhrase).is_empty());
}

#[test]
fn comparison_quantities_keep_only_the_determiner_route() {
    for (text, quantity_text, nominal_text) in [
        ("more than one creature", "more than one", "creature"),
        ("fewer than seven cards", "fewer than seven", "cards"),
    ] {
        let values = common::readings(text, Category::NounPhrase);
        assert!(!values.is_empty());
        for value in values {
            let Reading::QuantifiedNounPhrase { quantity, head, .. } = value else {
                panic!("the comparison must determine the nominal")
            };
            assert_eq!(quantity.realize(common::lexicon()).unwrap(), quantity_text);
            assert_eq!(head.realize(common::lexicon()).unwrap(), nominal_text);
            let Reading::ComparativeQuantityDeterminer { value, .. } = *quantity else {
                panic!("the quantitative Determiner must own the comparison")
            };
            assert_eq!(value.category(), Category::ComparativeQuantity);
        }
    }
    // A scalar standard has predicative distribution, without creating an
    // adjective Premodifier + noun route for the measured-amount residual.
    assert!(common::readings("less than one mana", Category::NounPhrase).is_empty());
}

#[test]
fn independently_constructed_cardinal_operands_and_standards_roundtrip() {
    let nominal = common::readings("the number of cards in your hand", Category::NounPhrase)
        .into_iter()
        .next()
        .unwrap();
    let nominal = Reading::ScalarNominalOperand {
        form: 0,
        value: Box::new(nominal),
    };
    let spelled = Reading::CardinalScalarOperand {
        form: 0,
        head: cardinal(1),
    };
    for (left, operator, right, text) in [
        (
            spelled.clone(),
            "vocab:Preposition/Plus",
            nominal.clone(),
            "one plus the number of cards in your hand",
        ),
        (
            nominal.clone(),
            "vocab:Preposition/Plus",
            spelled.clone(),
            "the number of cards in your hand plus one",
        ),
        (
            nominal,
            "vocab:Preposition/Minus",
            spelled,
            "the number of cards in your hand minus one",
        ),
    ] {
        let value = Reading::ArithmeticMeasure {
            form: 0,
            left: Box::new(left),
            operator: word(operator, None),
            right: Box::new(right),
        };
        assert_laws(&value, text, Category::MeasurePhrase);
    }
    let comparison = Reading::ComparativeAdjectivePhrase {
        form: 0,
        governor: Box::new(Reading::ComparativeGovernorHead {
            form: 0,
            head: word("vocab:ScalarDegree/Greater", Some(0)),
            marker: word("vocab:Preposition/Than", None),
        }),
        complement: Box::new(Reading::CardinalComparativeComplement {
            form: 0,
            head: cardinal(4),
        }),
    };
    assert_laws(
        &comparison,
        "greater than four",
        Category::ComparativeAdjectivePhrase,
    );
    let mut digit = cardinal(1);
    digit.value = LexicalReading::Numeral {
        value: 1,
        notation: Numeral::Arabic(false),
        capitalization: SurfaceCase::Declared,
    };
    for invalid in [
        Reading::CardinalScalarOperand {
            form: 0,
            head: digit.clone(),
        },
        Reading::CardinalComparativeComplement {
            form: 0,
            head: digit,
        },
    ] {
        assert!(invalid.admit(common::lexicon()).is_err());
        assert!(invalid.realize(common::lexicon()).is_err());
    }
}

// SC6 compares complete values, preserving every attachment and lexical identity
// outside the differently spelled scalar standard.
fn normalize_burn_standard(value: &mut Reading) -> bool {
    match value {
        Reading::CardinalComparativeComplement { head, .. } => {
            assert_eq!(*head, cardinal(3));
            let mut digit = head.clone();
            digit.value = LexicalReading::Numeral {
                value: 3,
                notation: Numeral::Arabic(false),
                capitalization: SurfaceCase::Declared,
            };
            *value = Reading::ScalarComparativeComplement {
                form: 0,
                value: Box::new(Reading::ScalarMeasurePhrase {
                    form: 0,
                    head: Box::new(Reading::UngroupedScalarNumeral {
                        form: 0,
                        modifier: None,
                        head: digit,
                    }),
                }),
            };
            true
        }
        Reading::Document { first, .. } | Reading::Paragraph { first, .. } => {
            normalize_burn_standard(first)
        }
        Reading::OrdinaryAbility { body, .. } => normalize_burn_standard(body),
        Reading::SentenceItem { sentence, .. } => normalize_burn_standard(sentence),
        Reading::Sentence { clause, .. } | Reading::Declarative { clause, .. } => {
            normalize_burn_standard(clause)
        }
        Reading::FiniteClause { predicate, .. } => normalize_burn_standard(predicate),
        Reading::NominalAdjunctPredicate { head, modifier, .. }
        | Reading::PostpositiveNominal { head, modifier, .. }
        | Reading::PostmodifiedNominal { head, modifier, .. } => {
            normalize_burn_standard(head) | normalize_burn_standard(modifier)
        }
        Reading::SelectedPredicate { complements, .. } => {
            let mut changed = false;
            for complement in complements {
                if let deckmaste_english_v3::grammar::FrameValue::Argument(child) = complement {
                    changed |= normalize_burn_standard(child);
                }
            }
            changed
        }
        Reading::NominalProjection { form: 0, head }
        | Reading::NominalCoreProjection { form: 0, head }
        | Reading::CasePhrase { head, .. }
        | Reading::BareNounPhrase { head, .. }
        | Reading::DeterminedNounPhrase { head, .. } => normalize_burn_standard(head),
        Reading::PrepositionPhrase { complement, .. }
        | Reading::ComparativeAdjectivePhrase { complement, .. } => {
            normalize_burn_standard(complement)
        }
        Reading::ComparativeAdjective { phrase, .. } => normalize_burn_standard(phrase),
        _ => false,
    }
}

#[test]
fn burn_at_the_stakes_new_comparison_readings_equal_the_digit_control() {
    let spelled = common::readings_on_card(
        "Burn at the Stake",
        "Burn at the Stake deals damage to any target equal to three times the number of creatures tapped this way.",
        Category::Document,
    );
    let normalized: std::collections::BTreeSet<_> = spelled
        .into_iter()
        .filter_map(|mut value| normalize_burn_standard(&mut value).then_some(value))
        .collect();
    assert!(!normalized.is_empty());
    let digit = common::readings_on_card(
        "Burn at the Stake",
        "Burn at the Stake deals damage to any target equal to 3 times the number of creatures tapped this way.",
        Category::Document,
    );
    assert_eq!(normalized, digit);
}
