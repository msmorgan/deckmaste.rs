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

fn exact(text: &str, category: Category, expected: Reading) {
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

#[test]
fn authentic_card_references_have_independent_proper_name_np_values() {
    // Lightning Bolt: "Lightning Bolt deals 3 damage to any target."
    // Rift Bolt: "Rift Bolt deals 3 damage to any target."
    for name in ["Lightning Bolt", "Rift Bolt"] {
        exact(
            name,
            Category::NounPhrase,
            Reading::ProperNameNounPhrase {
                form: 0,
                head: Box::new(Reading::CatalogName {
                    form: 0,
                    head: invariant(
                        &format!("catalog:card-names.txt/{name}"),
                        SurfaceCase::Declared,
                    ),
                }),
            },
        );
    }
}
