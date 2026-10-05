use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &input, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|reading| {
            let reading = reading.unwrap();
            assert_eq!(reading.realize(&LEXICON).unwrap(), text);
            reading
        })
        .collect()
}

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn frequency(quantity: Reading, number: Number) -> Reading {
    let mut head = word(
        "lexeme:CommonNoun/Time",
        if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
        FeatureBundle {
            number: Some(number),
            ..Default::default()
        },
    );
    head.countability = Some(true);
    Reading::CountedFrequency {
        form: 0,
        quantity: Box::new(quantity),
        head,
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::Cardinal {
        form: 0,
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

fn variable() -> Reading {
    Reading::VariableCount {
        form: 0,
        head: word(
            "vocab:Variable/X",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
    }
}

#[test]
fn independent_frequency_values_preserve_quantity_number_and_count_sense() {
    for (text, expected) in [
        ("X times", frequency(variable(), Number::Plural)),
        ("two times", frequency(cardinal(2), Number::Plural)),
        ("one time", frequency(cardinal(1), Number::Singular)),
    ] {
        assert_eq!(expected.realize(&LEXICON).unwrap(), text);
        assert_eq!(
            readings(text, Category::FrequencyPhrase),
            BTreeSet::from([expected])
        );
    }
    for text in ["one times", "two time", "X cards"] {
        assert!(
            readings(text, Category::FrequencyPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn independent_frequency_attachment_preserves_the_intransitive_frame() {
    let mut head = word(
        "lexeme:keyword_action/explore",
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            case: None,
        },
    );
    head.frame = Some(0);
    let expected = Reading::FrequencyPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: Box::new(Reading::IntransitivePredicate {
            category: Category::FinitePredicate,
            form: 0,
            head,
        }),
        modifier: Box::new(frequency(variable(), Number::Plural)),
    };
    assert_eq!(expected.realize(&LEXICON).unwrap(), "explores X times");
    let values = readings("explores X times", Category::FinitePredicate);
    assert!(values.contains(&expected));
    assert!(values.iter().any(|value| matches!(
        value,
        Reading::TransitivePredicate {
            category: Category::FinitePredicate,
            form: 0,
            ..
        }
    )));
}

#[test]
fn authentic_explore_frequency_and_transitive_frame_are_preserved() {
    let spelunker = readings(
        "When this creature enters, it explores X times.",
        Category::Document,
    );
    assert!(spelunker.iter().any(|reading| {
        let mut found = false;
        reading.visit(&mut |node| {
            if let Reading::FrequencyPredicate { category: Category::FinitePredicate, form: 0,head, .. } = node {
                if let Reading::IntransitivePredicate { category: Category::FinitePredicate, form: 0,head, .. } = head.as_ref() {
                    found |= matches!(&head.value, LexicalReading::Word(value) if value.lexeme == "lexeme:keyword_action/explore") && head.frame == Some(0);
                }
            }
        }).unwrap();
        found
    }));
    // Nicanzil's authentic "explores a land card" and "explores a nonland
    // card" establish the transitive use. Their compound nominals are an
    // existing grammar gap; this reduced diagnostic isolates that frame.
    let values = readings("explores a card", Category::FinitePredicate);
    assert!(values.iter().any(|value| matches!(value, Reading::TransitivePredicate { category: Category::FinitePredicate, form: 0,head, .. }
        if head.frame == Some(1) && matches!(&head.value, LexicalReading::Word(value) if value.lexeme == "lexeme:keyword_action/explore"))));
    let values = readings("draws cards two times", Category::FinitePredicate);
    assert!(values.iter().any(
        |value| matches!(value, Reading::FrequencyPredicate { category: Category::FinitePredicate, form: 0,head, .. }
        if matches!(head.as_ref(), Reading::TransitivePredicate { category: Category::FinitePredicate, form: 0,.. }))
    ));
}
