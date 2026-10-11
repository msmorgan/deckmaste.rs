use std::collections::{BTreeMap, BTreeSet};

use deckmaste_lexical::Case;
use deckmaste_lexical::Category;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::LEXICON;

fn value(owner: &str, case: Case, number: Number, casing: SurfaceCase) -> LexicalValue {
    LexicalValue {
        lexeme: owner.into(),
        form: if owner.starts_with("vocab:SubjectPronoun/") && case == Case::Genitive {
            WordForm::IndependentGenitive
        } else {
            WordForm::Invariant
        },
        features: FeatureBundle {
            number: Some(number),
            person: Some(Person::Second),
            case: Some(case),
            tense: None,
            finiteness: None,
        },
        variant: 0,
        capitalization: casing,
    }
}

#[test]
fn second_person_pronouns_have_exact_singular_case_and_capitalization_values() {
    let mut expected_by_owner: BTreeMap<&str, BTreeSet<LexicalValue>> = BTreeMap::new();
    for (owner, spelling, case) in [
        ("vocab:SubjectPronoun/You", "you", Case::Nominative),
        ("vocab:ObjectPronoun/You", "you", Case::Accusative),
        (
            "vocab:PossessiveDeterminerPronoun/Your",
            "your",
            Case::Genitive,
        ),
        ("vocab:SubjectPronoun/You", "yours", Case::Genitive),
        (
            "vocab:ReflexivePronoun/Yourself",
            "yourself",
            Case::Accusative,
        ),
    ] {
        assert_eq!(LEXICON.lexemes()[owner].category, Category::Pronoun);
        let mut expected = BTreeSet::new();
        for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
            let lexical_value = value(owner, case, Number::Singular, casing);
            let expected_reading = LexicalReading::Word(lexical_value.clone());
            let text = if casing == SurfaceCase::Initial {
                format!("Y{}", &spelling[1..])
            } else {
                spelling.into()
            };
            assert_eq!(LEXICON.realize(&expected_reading).unwrap(), text);
            let analyses: BTreeSet<_> = LEXICON
                .analyze(&text)
                .matches
                .into_iter()
                .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
                .filter_map(|matched| match &matched.reading {
                    LexicalReading::Word(value) if value.lexeme == owner => Some(matched.reading),
                    _ => None,
                })
                .collect();
            assert_eq!(analyses, BTreeSet::from([expected_reading]));
            expected.insert(lexical_value);
            assert!(
                LEXICON
                    .realize(&LexicalReading::Word(value(
                        owner,
                        case,
                        Number::Plural,
                        casing
                    )))
                    .is_err(),
                "plural {owner}"
            );
        }
        expected_by_owner.entry(owner).or_default().extend(expected);
    }
    for (owner, expected) in expected_by_owner {
        let actual: BTreeSet<_> = LEXICON
            .values()
            .filter(|value| value.lexeme == owner)
            .cloned()
            .collect();
        assert_eq!(actual, expected, "{owner}");
    }
}

#[test]
fn yourselves_has_no_declared_lexeme_or_licensed_independent_value() {
    let owner = "vocab:ReflexivePronoun/Yourselves";
    assert!(!LEXICON.lexemes().contains_key(owner));
    assert!(!LEXICON.values().any(|value| value.lexeme == owner));
    for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
        assert!(
            LEXICON
                .realize(&LexicalReading::Word(value(
                    owner,
                    Case::Accusative,
                    Number::Plural,
                    casing
                )))
                .is_err()
        );
        let text = if casing == SurfaceCase::Initial { "Yourselves" } else { "yourselves" };
        assert!(!LEXICON.analyze(text).matches.iter().any(|matched| matches!(&matched.reading, LexicalReading::Word(value) if LEXICON.lexemes()[&value.lexeme].category == Category::Pronoun)));
    }
}

#[test]
fn unrelated_third_person_pronouns_retain_their_existing_values() {
    for (owner, case, number) in [
        (
            "vocab:SubjectPronoun/They",
            Case::Nominative,
            Number::Plural,
        ),
        ("vocab:ObjectPronoun/Them", Case::Accusative, Number::Plural),
        (
            "vocab:PossessiveDeterminerPronoun/Their",
            Case::Genitive,
            Number::Plural,
        ),
        ("vocab:SubjectPronoun/He", Case::Genitive, Number::Singular),
        (
            "vocab:ReflexivePronoun/Himself",
            Case::Accusative,
            Number::Singular,
        ),
    ] {
        let mut expected: BTreeSet<_> = [SurfaceCase::Declared, SurfaceCase::Initial]
            .into_iter()
            .map(|casing| {
                let mut value = value(owner, case, number, casing);
                value.features.person = Some(Person::Third);
                value
            })
            .collect();
        if owner == "vocab:SubjectPronoun/They" {
            for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
                let mut independent = value(owner, Case::Genitive, number, casing);
                independent.features.person = Some(Person::Third);
                expected.insert(independent);
            }
        }
        if owner == "vocab:SubjectPronoun/He" {
            for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
                let mut nominative = value(owner, Case::Nominative, number, casing);
                nominative.features.person = Some(Person::Third);
                expected.insert(nominative);
            }
        }
        assert_eq!(
            LEXICON
                .values()
                .filter(|value| value.lexeme == owner)
                .cloned()
                .collect::<BTreeSet<_>>(),
            expected
        );
    }
}
