use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

fn noun(owner: &str, number: Number) -> Reading {
    Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: if number == Number::Singular {
                    WordForm::Singular
                } else {
                    WordForm::Plural
                },
                features: FeatureBundle {
                    number: Some(number),
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

fn assert_roll_object(text: &str, category: Category, owner: &str, number: Number) {
    let expected = noun(owner, number);
    let readings = common::readings(text, category);
    assert!(!readings.is_empty(), "{text}");
    for reading in readings {
        let mut objects = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(value) = &head.value
                    && value.lexeme.as_str() == "core-verb:Roll"
                {
                    for complement in complements {
                        if let FrameValue::Argument(object) = complement
                            && object.category() == Category::AccusativePhrase
                        {
                            let mut nouns = Vec::new();
                            object
                                .visit(&mut |node| {
                                    if let Reading::Noun { .. } = node {
                                        nouns.push(node.clone());
                                    }
                                })
                                .unwrap();
                            objects.push(nouns);
                        }
                    }
                }
            })
            .unwrap();
        assert_eq!(objects, vec![vec![expected.clone()]], "{text}");
    }
}

#[test]
fn die_nouns_head_the_object_of_roll() {
    // Attested constituents of Ebony Fly, Bag of Tricks, Netherese Puzzle-Ward,
    // Sword of Hours and Underdark Rift; the Endeavors supply plural Objects.
    for (die, owner) in [
        ("d4", "lexeme:CommonNoun/D4"),
        ("d6", "lexeme:CommonNoun/D6"),
        ("d8", "lexeme:CommonNoun/D8"),
        ("d10", "lexeme:CommonNoun/D10"),
        ("d12", "lexeme:CommonNoun/D12"),
    ] {
        assert_roll_object(
            &format!("Roll a {die}."),
            Category::Sentence,
            owner,
            Number::Singular,
        );
        assert_roll_object(
            &format!("Roll two {die} and choose one result."),
            Category::Sentence,
            owner,
            Number::Plural,
        );
    }
    // Berserker's Frenzy supplies the existing noun's invariant plural.
    assert_roll_object(
        "roll two d20",
        Category::SecondaryVerbPhrase,
        "lexeme:CommonNoun/D20",
        Number::Plural,
    );
}

#[test]
fn independently_constructed_nouns_preserve_singular_and_invariant_plural() {
    for (die, owner) in [
        ("d4", "lexeme:CommonNoun/D4"),
        ("d6", "lexeme:CommonNoun/D6"),
        ("d8", "lexeme:CommonNoun/D8"),
        ("d10", "lexeme:CommonNoun/D10"),
        ("d12", "lexeme:CommonNoun/D12"),
        ("d20", "lexeme:CommonNoun/D20"),
    ] {
        let expected = BTreeSet::from([noun(owner, Number::Singular), noun(owner, Number::Plural)]);
        for value in &expected {
            value.admit(common::lexicon()).unwrap();
            assert_eq!(value.realize(common::lexicon()).unwrap(), die);
        }
        assert_eq!(common::readings(die, Category::Nominal), expected);
    }
}

#[test]
fn existing_die_controls_keep_their_object_heads() {
    assert_roll_object(
        "Roll a d20.",
        Category::Sentence,
        "lexeme:CommonNoun/D20",
        Number::Singular,
    );
    assert_roll_object(
        "Roll a six-sided die.",
        Category::Sentence,
        "lexeme:CommonNoun/Die",
        Number::Singular,
    );
    assert_roll_object(
        "Whenever you roll one or more dice, draw a card.",
        Category::Sentence,
        "lexeme:CommonNoun/Die",
        Number::Plural,
    );
}
