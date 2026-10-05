use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::SurfaceCase;
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

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|value| {
            let value = value.unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), text);
            value
        })
        .collect()
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
