use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::{
    Case, Category, FeatureBundle, LexicalReading, LexicalValue, Lexicon, Number, Person,
    SurfaceCase, WordForm,
};

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn value(owner: &str, case: Case, number: Number, casing: SurfaceCase) -> LexicalValue {
    LexicalValue {
        lexeme: owner.into(),
        form: WordForm::Invariant,
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
    for (owner, spelling, case) in [
        ("vocab:SubjectPronoun/You", "you", Case::Nominative),
        ("vocab:ObjectPronoun/You", "you", Case::Accusative),
        (
            "vocab:PossessiveDeterminerPronoun/Your",
            "your",
            Case::Genitive,
        ),
        (
            "vocab:PossessiveAbsolutePronoun/Yours",
            "yours",
            Case::Genitive,
        ),
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
fn unrelated_third_person_plural_pronouns_retain_their_existing_values() {
    for (owner, case) in [
        ("vocab:SubjectPronoun/They", Case::Nominative),
        ("vocab:ObjectPronoun/Them", Case::Accusative),
        ("vocab:PossessiveDeterminerPronoun/Their", Case::Genitive),
        ("vocab:PossessiveAbsolutePronoun/Theirs", Case::Genitive),
        ("vocab:ReflexivePronoun/Themselves", Case::Accusative),
    ] {
        let expected: BTreeSet<_> = [SurfaceCase::Declared, SurfaceCase::Initial]
            .into_iter()
            .map(|casing| {
                let mut value = value(owner, case, Number::Plural, casing);
                value.features.person = Some(Person::Third);
                value
            })
            .collect();
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
