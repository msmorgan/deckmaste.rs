mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str) -> Reading {
    Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Singular,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(true),
        },
    }
}

fn exact(text: &str, category: Category, expected: &Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let actual = readings(text, category);
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

fn article() -> Word {
    let mut word = invariant("vocab:Article/Indefinite", SurfaceCase::Declared);
    if let LexicalReading::Word(value) = &mut word.value {
        value.features.number = Some(Number::Singular);
    }
    word
}
fn counter(kind: &str) -> Reading {
    noun(&format!("lexeme:counter_kind/{kind}Counter/compound-noun"))
}
fn indefinite(kind: &str) -> Reading {
    Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: article(),
        head: Box::new(counter(kind)),
    }
}
#[test]
fn authentic_counter_compounds_preserve_macro_owner_and_noun_inflection() {
    // Coalition Relic: "{T}: Put a charge counter on this artifact."
    exact(
        "a charge counter",
        Category::NounPhrase,
        &indefinite("charge"),
    );
    // Coalition Relic: "...remove all charge counters from this artifact."
    let mut counters = counter("charge");
    if let Reading::Noun { head, .. } = &mut counters
        && let LexicalReading::Word(value) = &mut head.value
    {
        value.form = WordForm::Plural;
        value.features.number = Some(Number::Plural);
    }
    exact(
        "charge counters",
        Category::NounPhrase,
        &Reading::BarePlural {
            form: 0,
            head: Box::new(counters),
        },
    );
}
#[test]
fn authentic_counter_np_coordination_keeps_whole_compound_conjuncts() {
    // Flycatcher Giraffid: "This creature enters with your choice of a reach
    // counter or a vigilance counter on it."
    exact(
        "a reach counter or a vigilance counter",
        Category::NounPhrase,
        &Reading::Coordination {
            category: Category::NounPhrase,
            form: 0,
            left: Box::new(indefinite("reach")),
            coordinator: invariant("vocab:Coordinator/Or", SurfaceCase::Declared),
            right: Box::new(indefinite("vigilance")),
        },
    );
    // Negative shared-modifier probe: macro compounds do not license free
    // charge/loyalty modifiers.
    assert!(readings("charge and loyalty counters", Category::NounPhrase).is_empty());
}

#[test]
fn authentic_numeric_counter_compounds_preserve_article_pronunciation() {
    // Sapphire Drake: "Each creature you control with a +1/+1 counter on it has
    // flying."
    exact("a +1/+1 counter", Category::NounPhrase, &indefinite("p1p1"));
    // Bloodied Ghost: "This creature enters with a -1/-1 counter on it."
    exact("a -1/-1 counter", Category::NounPhrase, &indefinite("m1m1"));
    for text in ["an +1/+1 counter", "an -1/-1 counter"] {
        assert!(readings(text, Category::NounPhrase).is_empty());
    }
}

#[test]
fn authentic_additional_numeric_counter_kinds_keep_signed_zero_notation() {
    for (text, owner) in [
        // Ebon Praetor.
        ("a +1/+0 counter", "P1P0"),
        // Takklemaggot.
        ("a -0/-1 counter", "M0M1"),
        // Greater Werewolf.
        ("a -0/-2 counter", "M0M2"),
        // Jabari's Influence.
        ("a -1/-0 counter", "M1M0"),
    ] {
        exact(
            text,
            Category::NounPhrase,
            &Reading::IndefiniteNounPhrase {
                form: 0,
                determiner: article(),
                head: Box::new(noun(&format!(
                    "lexeme:counter_kind_numeric/{owner}/compound-noun"
                ))),
            },
        );
    }
}
