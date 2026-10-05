use deckmaste_lexical::*;
use std::collections::BTreeSet;

fn noun(surface: &str) -> Lexeme {
    let mut noun = Lexeme::noun(
        "measured",
        surface,
        vec![Countability::Count],
        Source {
            kind: SourceKind::Core,
            path: "tests/declared-measured-compound".into(),
            owner: "measured".into(),
        },
    );
    noun.surface_structure = SurfaceStructure::MeasuredCompound;
    noun
}

#[test]
fn declared_measured_compounds_preserve_the_complete_lexical_value() {
    let lexicon = Lexicon::new([noun("+1/+1 counter")]).unwrap();
    for (form, number, text) in [
        (WordForm::Singular, Number::Singular, "+1/+1 counter"),
        (WordForm::Plural, Number::Plural, "+1/+1 counters"),
    ] {
        let expected = LexicalReading::Word(LexicalValue {
            lexeme: "measured".into(),
            form,
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        });
        assert_eq!(lexicon.realize(&expected).unwrap(), text);
        let actual: BTreeSet<_> = lexicon
            .analyze(text)
            .matches
            .into_iter()
            .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
            .map(|matched| matched.reading)
            .collect();
        assert_eq!(actual, BTreeSet::from([expected]));
    }
    assert!(
        lexicon
            .analyze("x+1/+1 counter")
            .matches
            .into_iter()
            .all(|matched| matched.start != 1)
    );
    assert!(
        lexicon
            .analyze("+1/+1 countersx")
            .matches
            .into_iter()
            .all(|matched| matched.end != 14)
    );
}

#[test]
fn measured_structure_rejects_noncanonical_components_and_other_categories() {
    for bad in [
        "++1/+1 counter",
        "+-1/+1 counter",
        "+1/ counter",
        "/+1 counter",
        "+1/+1/+1 counter",
        "+01/+1 counter",
        "-00/+1 counter",
        "word/+1 counter",
        "+1/+1  counter",
        "+1/+1 counter tail",
        "+1/+1counter",
        "+1/+1 counter\n",
    ] {
        assert!(Lexicon::new([noun(bad)]).is_err(), "{bad:?}");
    }
    let mut wrong_category = noun("+1/+1 counter");
    wrong_category.category = Category::Adjective;
    assert!(Lexicon::new([wrong_category]).is_err());
    let mut wrong_binding = noun("+1/+1 counter");
    wrong_binding.binding = Binding::Suffix;
    assert!(Lexicon::new([wrong_binding]).is_err());
    let mut plain_multiword = noun("+1/+1 counter");
    plain_multiword.surface_structure = SurfaceStructure::Multiword;
    assert!(Lexicon::new([plain_multiword]).is_err());
}

#[test]
fn a_declared_signed_zero_preserves_its_written_sign_in_both_noun_forms() {
    let lexicon = Lexicon::new([noun("-0/-1 counter")]).unwrap();
    for (form, number, text) in [
        (WordForm::Singular, Number::Singular, "-0/-1 counter"),
        (WordForm::Plural, Number::Plural, "-0/-1 counters"),
    ] {
        let expected = LexicalReading::Word(LexicalValue {
            lexeme: "measured".into(),
            form,
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        });
        assert_eq!(lexicon.realize(&expected).unwrap(), text);
        let actual: BTreeSet<_> = lexicon
            .analyze(text)
            .matches
            .into_iter()
            .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
            .map(|matched| matched.reading)
            .collect();
        assert_eq!(actual, BTreeSet::from([expected]));
    }
}
