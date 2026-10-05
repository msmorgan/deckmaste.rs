mod common;

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
