use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::lexicon;
use crate::common::readings;
use crate::common::readings_on_card;

fn noun(form: WordForm, number: Number) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "lexeme:CommonNoun/Difference".into(),
            form,
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: Some(true),
    }
}

fn difference() -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:DefiniteMarker/The".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: noun(WordForm::Singular, Number::Singular),
                }),
            }),
        }),
    }
}

fn accusative_difference() -> Reading {
    Reading::CasePhrase {
        form: 0,
        category: Category::AccusativePhrase,
        head: Box::new(difference()),
    }
}

#[test]
fn independently_constructed_anaphoric_noun_roundtrips() {
    // Balance of Power's anaphoric Complement, under project ruling DN1.
    let expected = difference();
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "the difference");
    assert_eq!(
        readings("the difference", Category::NounPhrase),
        BTreeSet::from([expected]),
    );
}

#[test]
fn difference_owns_its_regular_count_noun_paradigm() {
    // Owner principle (2026-10-10): the regular plural is part of the
    // count noun's paradigm even when it is unattested in Oracle text.
    let declaration =
        &lexicon().lexemes()[&deckmaste_lexical::LexemeId::new("lexeme:CommonNoun/Difference")];
    assert_eq!(declaration.properties.countability, [Countability::Count]);
    assert_eq!(
        declaration
            .properties
            .features
            .get("ScalarDenotation")
            .map(AsRef::<str>::as_ref),
        Some("Yes")
    );
    assert_eq!(
        declaration.properties.features.get("NumberTransparency"),
        None
    );
    for (surface, form, number) in [
        ("difference", WordForm::Singular, Number::Singular),
        ("differences", WordForm::Plural, Number::Plural),
    ] {
        let expected = noun(form, number);
        assert_eq!(lexicon().realize(&expected.value).unwrap(), surface);
        let analyses = lexicon().analyze(surface);
        let full_words: BTreeSet<_> = analyses
            .matches
            .into_iter()
            .filter(|found| found.start == 0 && found.end == surface.len())
            .map(|found| found.reading)
            .collect();
        assert_eq!(full_words, BTreeSet::from([expected.value]));
    }
}

#[test]
fn oracle_witnesses_select_difference_in_the_complement_role() {
    // Independent ambiguity elsewhere is retained (standing ruling W9-A).
    let comparative = Reading::NominalComparativeComplement {
        form: 0,
        value: Box::new(accusative_difference()),
    };
    let predicative = Reading::NominalComplement {
        form: 0,
        phrase: Box::new(accusative_difference()),
    };
    for (card, text, expected) in [
        (
            "Balance of Power",
            "If target opponent has more cards in hand than you, draw cards equal to the difference.",
            &comparative,
        ),
        (
            "Kozilek, the Great Distortion",
            "When you cast this spell, if you have fewer than seven cards in hand, draw cards equal to the difference.",
            &comparative,
        ),
        (
            "Ancient Cellarspawn",
            "Whenever you cast a spell, if the amount of mana spent to cast it was less than its mana value, target opponent loses life equal to the difference.",
            &comparative,
        ),
        (
            "Shadow of Mortality",
            "If your life total is less than your starting life total, this spell costs {X} less to cast, where X is the difference.",
            &predicative,
        ),
    ] {
        let values = readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut complements = Vec::new();
            value
                .visit(&mut |node| {
                    let phrase = match node {
                        Reading::NominalComparativeComplement { value, .. } => value,
                        Reading::NominalComplement { phrase, .. } => phrase,
                        _ => return,
                    };
                    if phrase.realize(lexicon()).unwrap() == "the difference" {
                        complements.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                complements.as_slice(),
                std::slice::from_ref(expected),
                "{card}"
            );
        }
    }
}
