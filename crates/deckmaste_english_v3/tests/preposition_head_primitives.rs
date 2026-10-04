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

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|reading| {
            let reading = reading.unwrap();
            assert_eq!(reading.realize(&LEXICON).unwrap(), text);
            reading
        })
        .collect()
}

#[test]
fn selected_np_preposition_heads_are_independent_roundtrips() {
    for owner in [
        "vocab:Preposition/Over",
        "vocab:Preposition/Under",
        "vocab:Preposition/Of",
    ] {
        let value = Reading::SelectedPrepositionHead {
            form: 0,
            head: word(owner, WordForm::Invariant, FeatureBundle::default()),
        };
        let text = value.realize(&LEXICON).unwrap();
        assert!(readings(&text, Category::SelectedPrepositionHead).contains(&value));
    }
    for text in ["over cards", "and", "draw"] {
        assert!(
            readings(text, Category::SelectedPrepositionHead).is_empty(),
            "{text}"
        );
    }
}
