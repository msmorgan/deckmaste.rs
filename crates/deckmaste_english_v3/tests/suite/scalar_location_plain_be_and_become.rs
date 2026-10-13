use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{LexicalReading, Number, Numeral, Person, SurfaceCase, WordForm};

use crate::common;

fn word(owner: &str, form: WordForm) -> Word {
    let value = common::lexicon()
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
    let lexeme = &common::lexicon().lexemes()[&value.lexeme];
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn framed_word(owner: &str, form: WordForm, kind: &str) -> Word {
    let mut head = word(owner, form);
    head.frame = Some(
        common::lexicon().lexemes()[owner]
            .properties
            .frames
            .iter()
            .position(|frame| frame.kind == kind)
            .unwrap(),
    );
    head
}

fn scalar_zero() -> Reading {
    Reading::ScalarMeasureValue {
        category: Category::ScalarValue,
        form: 0,
        value: Box::new(Reading::UngroupedScalarNumeral {
            form: 0,
            modifier: None,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 0,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
    }
}

fn plain_scalar_predicate() -> Reading {
    Reading::ScalarLocationPredicate {
        category: Category::BareScalarLocationPredicate,
        form: 0,
        head: framed_word("core-verb:Be", WordForm::Plain, "ScalarLocation"),
        complements: vec![FrameValue::Argument(Box::new(scalar_zero()))],
    }
}

fn lamp_clause() -> Reading {
    Reading::ScalarLocation {
        form: 0,
        subject: Box::new(Reading::ScalarVariable {
            category: Category::NominativePhrase,
            form: 0,
            head: word("vocab:Variable/X", WordForm::Invariant),
        }),
        predicate: Box::new(Reading::BareAuxiliaryPredicate {
            category: Category::ScalarLocationPredicate,
            form: 0,
            head: framed_word("core-verb:Cant", WordForm::Present, "Auxiliary"),
            complement: Box::new(Reading::OvertComplement {
                category: Category::BareScalarLocationComplement,
                form: 0,
                predicate: Box::new(plain_scalar_predicate()),
            }),
        }),
    }
}

fn become_clause() -> Reading {
    Reading::ScalarLocation {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::PossessiveNounPhrase {
                form: 0,
                possessor: word("vocab:PossessiveDeterminerPronoun/Its", WordForm::Invariant),
                head: Box::new(Reading::NominalProjection {
                    form: 0,
                    head: Box::new(Reading::NominalCoreProjection {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word("lexeme:CommonNoun/Power", WordForm::Singular),
                        }),
                    }),
                }),
            }),
        }),
        predicate: Box::new(Reading::ScalarLocationPredicate {
            category: Category::ScalarLocationPredicate,
            form: 0,
            head: framed_word("core-verb:Become", WordForm::Present, "ScalarLocation"),
            complements: vec![FrameValue::Argument(Box::new(scalar_zero()))],
        }),
    }
}

#[test]
fn witnesses_keep_scalar_location_complement_roles() {
    // Aladdin's Lamp; Gogo, Master of Mimicry; Worldfire; Vraska, Relic
    // Seeker; Null Profusion; Cursed Rack. These are the ticket's lines.
    for (text, category, predicate, value) in [
        ("X can't be 0.", Category::Sentence, "can't be 0", "0"),
        (
            "This ability can't be copied and X can't be 0.",
            Category::Sentence,
            "can't be 0",
            "0",
        ),
        (
            "Each player's life total becomes 1.",
            Category::Sentence,
            "becomes 1",
            "1",
        ),
        (
            "Your maximum hand size is two.",
            Category::Sentence,
            "is two",
            "two",
        ),
        (
            "The chosen player's maximum hand size is four.",
            Category::Sentence,
            "is four",
            "four",
        ),
    ] {
        common::assert_constituents(
            text,
            category,
            &[
                (Category::ScalarLocationPredicate, predicate),
                (Category::ScalarValue, value),
            ],
        );
    }
    // The value is the second occurrence of "1", after the loyalty cost.
    common::assert_constituent_occurrences(
        "−10: Target player's life total becomes 1.",
        Category::Document,
        &[
            (Category::ScalarLocationPredicate, "becomes 1", 0),
            (Category::ScalarValue, "1", 1),
        ],
    );
    common::assert_constituents(
        "X can't be 0.",
        Category::Sentence,
        &[(Category::BareScalarLocationPredicate, "be 0")],
    );
}

#[test]
fn auxiliary_class_selects_the_plain_head_and_finite_value_paradigm() {
    // Class substitutions over Aladdin's Lamp's predicate, using the same
    // Scalar Value constituents as the finite route (SL3).
    for auxiliary in [
        "can", "can't", "could", "couldn't", "must", "may", "might", "would",
    ] {
        for value in ["0", "X", "two", "1 or less"] {
            let text = format!("{auxiliary} be {value}");
            let readings = common::readings(&text, Category::ScalarLocationPredicate);
            assert!(!readings.is_empty(), "{text}");
            let values = common::readings(value, Category::ScalarValue);
            for reading in readings {
                let Reading::BareAuxiliaryPredicate { complement, .. } = reading else {
                    panic!("scalar location must retain the auxiliary's Complement")
                };
                let Reading::OvertComplement { predicate, .. } = *complement else {
                    panic!("the scalar predicate must be overt")
                };
                let Reading::ScalarLocationPredicate {
                    head, complements, ..
                } = *predicate
                else {
                    panic!("the plain head must select scalar location")
                };
                assert_eq!(
                    head,
                    framed_word("core-verb:Be", WordForm::Plain, "ScalarLocation")
                );
                assert!(values.iter().any(|value| {
                    complements == vec![FrameValue::Argument(Box::new(value.clone()))]
                }));
            }
        }
    }
}

#[test]
fn independent_values_preserve_both_laws_and_lexical_identity() {
    for (value, text) in [
        (lamp_clause(), "X can't be 0"),
        (become_clause(), "its power becomes 0"),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let parsed = common::readings(text, Category::FiniteClause);
        assert_eq!(parsed, BTreeSet::from([value.clone()]));
        let observed = parsed.get(&value).unwrap();
        let (mut words, mut observed_words) = (Vec::new(), Vec::new());
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        observed
            .visit_words(&mut |word| observed_words.push(word.clone()))
            .unwrap();
        assert_eq!(observed_words, words);
    }
}

#[test]
fn non_scalar_subjects_and_wrong_head_forms_stay_excluded() {
    // SL2: preserve the finite copula's Subject restriction, including It.
    for text in [
        "It is 0.",
        "It can't be 0.",
        "It becomes 0.",
        "This creature becomes 0.",
    ] {
        assert!(
            common::readings(text, Category::Sentence).is_empty(),
            "{text}"
        );
    }
    let mut wrong_form = plain_scalar_predicate();
    let Reading::ScalarLocationPredicate { head, .. } = &mut wrong_form else {
        unreachable!()
    };
    *head = framed_word("core-verb:Be", WordForm::Present, "ScalarLocation");
    assert!(wrong_form.admit(common::lexicon()).is_err());
    assert!(wrong_form.realize(common::lexicon()).is_err());
    let mut wrong_frame = become_clause();
    let Reading::ScalarLocation { predicate, .. } = &mut wrong_frame else {
        unreachable!()
    };
    let Reading::ScalarLocationPredicate { head, .. } = predicate.as_mut() else {
        unreachable!()
    };
    *head = framed_word("core-verb:Become", WordForm::Present, "Predicate");
    assert!(wrong_frame.admit(common::lexicon()).is_err());
    assert!(wrong_frame.realize(common::lexicon()).is_err());
}

#[test]
fn existing_copular_predicative_and_passive_routes_keep_their_values() {
    for text in [
        "X is 0.",
        "Its power is 0.",
        "Your life total is 1.",
        "X can't be greater than 3.",
        "Your life total becomes half your starting life total.",
        "Its power becomes equal to 3.",
        "You have no maximum hand size.",
        "It can't be a creature.",
        "It can't be blocked.",
        "It becomes a 1/1 creature.",
    ] {
        assert!(
            !common::readings(text, Category::Sentence).is_empty(),
            "{text}"
        );
    }
    common::assert_constituents(
        "It becomes a 1/1 creature.",
        Category::Sentence,
        &[(Category::PredicativeComplement, "a 1/1 creature")],
    );
}
