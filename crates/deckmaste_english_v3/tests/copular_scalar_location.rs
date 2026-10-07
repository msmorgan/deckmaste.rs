mod common;

use common::assert_constituents;
use common::readings;
use deckmaste_english_v3::grammar::Category;

#[test]
fn savage_swipe_locates_a_scalar_property() {
    assert_constituents(
        "its power is 2",
        Category::FiniteClause,
        &[(Category::ScalarValue, "2")],
    );
    assert!(readings("that creature is 2", Category::FiniteClause).is_empty());
}

#[test]
fn copular_witnesses_retain_the_value_as_one_complement() {
    // Stature, Depressurize, Domestication, Guidelight Pathmaker, Technodrome.
    for (text, subject, value) in [
        ("her power is 1 or less", "her power", "1 or less"),
        (
            "that creature's power is 0 or less",
            "that creature's power",
            "0 or less",
        ),
        (
            "enchanted creature's power is 4 or greater",
            "enchanted creature's power",
            "4 or greater",
        ),
        ("its mana value is 2 or less", "its mana value", "2 or less"),
        ("its power is 6 or greater", "its power", "6 or greater"),
        // Disrupting Shoal.
        (
            "that spell's mana value is X",
            "that spell's mana value",
            "X",
        ),
    ] {
        let values = readings(text, Category::FiniteClause);
        assert_eq!(values.len(), 1, "{text}: {values:?}");
        assert_constituents(
            text,
            Category::FiniteClause,
            &[
                (Category::NominativePhrase, subject),
                (Category::ScalarValue, value),
            ],
        );
    }
    for text in [
        "that creature is 0 or less",
        "that creature is 4 or greater",
    ] {
        assert!(readings(text, Category::FiniteClause).is_empty(), "{text}");
    }
}

#[test]
fn number_of_scalar_location_keeps_singular_concord() {
    assert_constituents(
        "The number of cards in your hand is three.",
        Category::Document,
        &[(
            Category::NominativePhrase,
            "The number of cards in your hand",
        )],
    );
    assert!(
        readings(
            "The number of cards in your hand are three.",
            Category::Document
        )
        .is_empty()
    );
}

use common::lexicon;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn word(owner: &str, form: WordForm) -> Word {
    let value = lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == owner
                && value.form == form
                && (form != WordForm::Present
                    || (value.features.number == Some(Number::Singular)
                        && value.features.person == Some(Person::Third)))
        })
        .unwrap()
        .clone();
    let lexeme = &lexicon().lexemes()[&value.lexeme];
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn numeric(value: i32) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn scalar_location(possessor: &str, value: Reading) -> Reading {
    let mut head = word("core-verb:Be", WordForm::Present);
    head.frame = lexicon().lexemes()["core-verb:Be"]
        .properties
        .frames
        .iter()
        .position(|frame| frame.kind == "ScalarLocation");
    Reading::ScalarLocation {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            form: 0,
            category: Category::NominativePhrase,
            head: Box::new(Reading::PossessiveNounPhrase {
                form: 0,
                possessor: word(possessor, WordForm::Invariant),
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word("lexeme:CommonNoun/Power", WordForm::Singular),
                }),
            }),
        }),
        predicate: Box::new(Reading::ScalarLocationPredicate {
            form: 0,
            head,
            complements: vec![FrameValue::Argument(Box::new(value))],
        }),
    }
}

fn scalar(value: i32) -> Reading {
    Reading::ScalarPropertyMeasure {
        form: 0,
        value: Box::new(Reading::UngroupedScalarNumeral {
            form: 0,
            head: numeric(value),
        }),
    }
}

#[test]
fn independent_scalar_location_preserves_both_laws_and_traversal_identity() {
    // Savage Swipe and Stature's exact finite-Clause constituents.
    let coordinated = Reading::ScalarPropertyCoordination {
        form: 0,
        value: Box::new(Reading::NumeralComparativeCoordination {
            form: 0,
            left: Box::new(Reading::NumericQuantityConjunct {
                form: 0,
                head: numeric(1),
            }),
            marker: word("vocab:Coordinator/Or", WordForm::Invariant),
            right: word("vocab:Determinative/Less", WordForm::Invariant),
        }),
    };
    for (value, text) in [
        (
            scalar_location("vocab:PossessiveDeterminerPronoun/Its", scalar(2)),
            "its power is 2",
        ),
        (
            scalar_location("vocab:PossessiveDeterminerPronoun/Her", coordinated),
            "her power is 1 or less",
        ),
    ] {
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
        let parsed = readings(text, Category::FiniteClause);
        assert_eq!(parsed, std::collections::BTreeSet::from([value.clone()]));
        let observed = parsed.get(&value).unwrap();
        let (mut nodes, mut parsed_nodes, mut words, mut parsed_words) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        value.visit(&mut |node| nodes.push(node.clone())).unwrap();
        observed
            .visit(&mut |node| parsed_nodes.push(node.clone()))
            .unwrap();
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        observed
            .visit_words(&mut |word| parsed_words.push(word.clone()))
            .unwrap();
        assert_eq!(parsed_nodes, nodes);
        assert_eq!(parsed_words, words);
    }
}

#[test]
fn independent_non_scalar_subject_and_wrong_frame_are_rejected() {
    let mut wrong_subject = scalar_location("vocab:PossessiveDeterminerPronoun/Its", scalar(2));
    let Reading::ScalarLocation { subject, .. } = &mut wrong_subject else {
        unreachable!()
    };
    *subject = Box::new(Reading::CasePhrase {
        form: 0,
        category: Category::NominativePhrase,
        head: Box::new(Reading::DeterminedNounPhrase {
            form: 0,
            determiner: word("vocab:SingularDemonstrative/That", WordForm::Invariant),
            head: Box::new(Reading::Noun {
                form: 0,
                head: word("lexeme:type/creature", WordForm::Singular),
            }),
        }),
    });
    let mut wrong_frame = scalar_location("vocab:PossessiveDeterminerPronoun/Its", scalar(2));
    let Reading::ScalarLocation { predicate, .. } = &mut wrong_frame else {
        unreachable!()
    };
    let Reading::ScalarLocationPredicate { head, .. } = predicate.as_mut() else {
        unreachable!()
    };
    head.frame = Some(0);
    for invalid in [wrong_subject, wrong_frame] {
        assert!(invalid.admit(lexicon()).is_err());
        assert!(invalid.realize(lexicon()).is_err());
    }
}

#[test]
fn scalar_values_and_other_copular_routes_do_not_overlap() {
    // Savage Swipe, Disrupting Shoal, Depressurize, and Domestication.
    for text in ["2", "X", "0 or less", "4 or greater", "three"] {
        assert_eq!(readings(text, Category::ScalarValue).len(), 1, "{text}");
    }
    for text in [
        "its power is 2",
        "its power is 6 or greater",
        "its mana value is 2 or less",
    ] {
        assert_eq!(readings(text, Category::FiniteClause).len(), 1, "{text}");
    }
    for text in [
        "that creature is 2",
        "X is 2",
        "its power creature is 2",
        "its power and that creature are 2",
        "its power are 2",
        "any number of target creatures are 2",
        "three creatures are 2",
        "its mana value is more than one",
        "its power is 2/2",
    ] {
        assert!(readings(text, Category::FiniteClause).is_empty(), "{text}");
    }
    // The shared coordination remains the prerequisite's Reading.
    let values = readings("0 or less", Category::ScalarValue);
    let Reading::ScalarPropertyCoordination { value, .. } = values.first().unwrap() else {
        panic!("the value must reuse the existing numeral Coordination")
    };
    assert_eq!(
        readings("0 or less", Category::ComparativeQuantity),
        std::collections::BTreeSet::from([value.as_ref().clone()])
    );
    assert!(readings("0 or less", Category::Clause).is_empty());
}
