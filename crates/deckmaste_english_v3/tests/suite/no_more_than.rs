use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::assert_constituents;
use crate::common::lexicon;
use crate::common::readings;

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn comparison(modifier: Option<&str>, head: &str, complement: Reading) -> Reading {
    let (form, category) = match complement.category() {
        Category::AdverbPhrase => (2, Category::AdverbFrequencyPrepositionPhrase),
        Category::FrequencyPhrase => (1, Category::CountedFrequencyPrepositionPhrase),
        Category::Cardinal => (0, Category::ComparativePrepositionPhrase),
        _ => panic!("unsupported test Complement"),
    };
    Reading::ComparativeDeterminativePhrase {
        form,
        modifier: modifier.map(word),
        head: word(head),
        complement: Box::new(Reading::QuantitativePrepositionPhrase {
            category,
            form: 0,
            head: word("vocab:Preposition/Than"),
            complement: Box::new(complement),
        }),
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::Cardinal {
        form: 0,
        modifier: None,
        head: Word {
            value: LexicalReading::Numeral {
                value,
                notation: Numeral::Cardinal,
                capitalization: SurfaceCase::Declared,
            },
            frame: None,
            countability: None,
        },
    }
}

fn assert_laws(expected: Reading, text: &str, category: Category) {
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    assert_eq!(readings(text, category), BTreeSet::from([expected]));
}

#[test]
fn comparative_modifier_is_internal_to_the_determinative_phrase() {
    // Judoon Enforcers and Crawlspace: no modifies more; than selects the
    // count.
    for (surface, count) in [("no more than one", 1), ("no more than two", 2)] {
        let quantity = comparison(
            Some("vocab:Determinative/No"),
            "vocab:ComparativeQuantifier/More",
            cardinal(count),
        );
        assert_laws(quantity.clone(), surface, Category::ComparativeQuantity);
        assert_laws(
            Reading::ComparativeQuantityDeterminer {
                form: 0,
                value: Box::new(quantity),
            },
            surface,
            Category::QuantitativeDeterminer,
        );
    }
    assert_constituents(
        "no more than one creature",
        Category::NounPhrase,
        &[
            (Category::QuantitativeDeterminer, "no more than one"),
            (Category::ComparativePrepositionPhrase, "than one"),
            (Category::Nominal, "creature"),
        ],
    );
}

#[test]
fn existing_quantificational_modifier_class_is_licensed() {
    // Owner principle, 2026-10-10: constituent diagnostics beyond attestation.
    for (modifier, owner) in [
        ("no", "vocab:Determinative/No"),
        ("any", "vocab:Determinative/Any"),
        ("many", "vocab:Determinative/Many"),
        ("much", "vocab:Determinative/Much"),
        ("some", "vocab:Determinative/Some"),
    ] {
        assert_laws(
            comparison(Some(owner), "vocab:ComparativeQuantifier/More", cardinal(2)),
            &format!("{modifier} more than two"),
            Category::ComparativeQuantity,
        );
    }
    assert_laws(
        comparison(
            Some("vocab:Determinative/No"),
            "vocab:ComparativeQuantifier/Fewer",
            cardinal(2),
        ),
        "no fewer than two",
        Category::ComparativeQuantity,
    );
    for text in [
        "every more than two",
        "the more than two",
        "none more than two",
    ] {
        assert!(
            readings(text, Category::ComparativeQuantity).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn frequency_comparison_preserves_its_complement_and_roundtrip() {
    // Pit Imp, under NM3: the frequency Adverb is inside the than Complement.
    for modifier in [None, Some("vocab:Determinative/No")] {
        let expected = comparison(
            modifier,
            "vocab:ComparativeQuantifier/More",
            Reading::Adverb {
                form: 0,
                head: word("vocab:FrequencyAdverb/Twice"),
            },
        );
        let text = if modifier.is_some() { "no more than twice" } else { "more than twice" };
        assert_laws(expected, text, Category::ComparativeQuantity);
    }
    for (text, complement, category) in [
        (
            "Activate no more than twice each turn.",
            "twice",
            Category::AdverbPhrase,
        ),
        (
            "Activate no more than three times each turn.",
            "three times",
            Category::FrequencyPhrase,
        ),
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty(), "{text}");
        assert!(values.iter().any(|value| {
            let mut found = false;
            value
                .visit(&mut |node| {
                    if let Reading::FrequencyPredicate { modifier, .. } = node {
                        found |= modifier.category() == Category::ComparativeQuantity
                            && modifier
                                .realize(lexicon())
                                .unwrap()
                                .starts_with("no more than");
                    }
                })
                .unwrap();
            found
        }));
        assert_constituents(text, Category::Document, &[(category, complement)]);
    }
    for text in ["no more than again", "no more than secretly"] {
        assert!(
            readings(text, Category::ComparativeQuantity).is_empty(),
            "{text}"
        );
    }
    assert!(readings("no more than twice creatures", Category::NounPhrase).is_empty());
}

#[test]
fn authentic_witnesses_retain_the_comparative_modifier_role() {
    for text in [
        "No more than two creatures can attack you each combat.",
        "No more than one creature can attack you each combat.",
        "{B}: This creature gets +1/+0 until end of turn. Activate no more than twice each turn.",
        "{B}, Pay 1 life: This creature gets +1/+0 until end of turn. Activate no more than three times each turn.",
        "You can cast spells only during your turn and you can cast no more than two spells each turn.",
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut found = false;
            value
                .visit(&mut |node| {
                    if let Reading::ComparativeDeterminativePhrase {
                        modifier: Some(modifier),
                        head,
                        ..
                    } = node
                    {
                        if let LexicalReading::Word(observed) = &modifier.value {
                            assert_eq!(
                                observed.lexeme,
                                deckmaste_lexical::LexemeId::new("vocab:Determinative/No")
                            );
                        } else {
                            panic!("comparative Modifier must preserve its lexical owner");
                        }
                        if let LexicalReading::Word(observed) = &head.value {
                            assert_eq!(
                                observed.lexeme,
                                deckmaste_lexical::LexemeId::new(
                                    "vocab:ComparativeQuantifier/More"
                                )
                            );
                        } else {
                            panic!("comparative head must preserve its lexical owner");
                        }
                        found = true;
                    }
                })
                .unwrap();
            assert!(found, "{text}");
        }
    }
}

#[test]
fn comparative_modifier_preserves_agreement_and_polarity_boundaries() {
    // Controls vary number to isolate NM5; they do not assert invented effects.
    for text in ["no more than one creatures", "no more than two creature"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for text in ["no more than one creature", "no more than two creatures"] {
        assert!(!readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for (modified, control) in [
        (
            "No more than two creatures can attack you each combat.",
            "More than two creatures can attack you each combat.",
        ),
        (
            "You can cast no more than two spells each turn.",
            "You can cast more than two spells each turn.",
        ),
        (
            "You can't cast no more than two spells each turn.",
            "You can't cast more than two spells each turn.",
        ),
    ] {
        let modified = readings(modified, Category::Document);
        let controls = readings(control, Category::Document);
        assert!(!controls.is_empty());
        assert_eq!(modified.len(), controls.len());
    }
    // The existing control sets retain their complete independent values.
    assert_laws(
        comparison(None, "vocab:ComparativeQuantifier/More", cardinal(1)),
        "more than one",
        Category::ComparativeQuantity,
    );
    let no = readings("no creature", Category::NounPhrase);
    assert_eq!(no.len(), 1);
    assert_eq!(
        readings(
            "No creatures can attack you each combat.",
            Category::Document
        )
        .len(),
        2
    );
}
