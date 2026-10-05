mod common;

use std::collections::BTreeSet;

use common::lexicon;
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

fn word(id: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle {
                number,
                ..FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability: None,
        frame: None,
    }
}
fn assert_laws(value: Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert!(
        parsed.contains(&value),
        "independent value missing for {text:?}"
    );
    let reparsed = parsed.get(&value).unwrap();
    let mut nodes = Vec::new();
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    let mut parsed_nodes = Vec::new();
    reparsed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(nodes, parsed_nodes);
    let mut words = Vec::new();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    let mut parsed_words = Vec::new();
    reparsed
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(words, parsed_words);
}

fn keyword(id: &str) -> Reading {
    Reading::BareKeyword {
        form: 0,
        head: word(id, WordForm::Invariant, None),
    }
}
fn pp(marker: &str, complement: Reading) -> Reading {
    Reading::KeywordComplementPreposition {
        form: 0,
        head: word(
            &format!("vocab:Preposition/{marker}"),
            WordForm::Invariant,
            None,
        ),
        complement: Box::new(complement),
    }
}

#[test]
fn selected_keyword_pp_bases_roundtrip_before_coordination() {
    for (marker, text, id) in [
        ("With", "with flying", "lexeme:keyword_ability/flying"),
        ("Without", "without flying", "lexeme:keyword_ability/flying"),
        (
            "With",
            "with first strike",
            "lexeme:keyword_ability/firstStrike",
        ),
    ] {
        let value = pp(marker, keyword(id));
        assert_laws(value.clone(), text, Category::PrepositionPhrase);
        assert_eq!(
            readings(text, Category::PrepositionPhrase),
            BTreeSet::from([value])
        );
    }
    let without_selection = pp("From", keyword("lexeme:keyword_ability/flying"));
    assert!(without_selection.admit(lexicon()).is_err());
    assert!(readings("from flying", Category::PrepositionPhrase).is_empty());
}

#[test]
fn keyword_pp_coordinates_only_after_its_primitive_is_licensed() {
    let complement = Reading::Coordination {
        category: Category::KeywordPhrase,
        form: 0,
        left: Box::new(keyword("lexeme:keyword_ability/flying")),
        coordinator: word("vocab:Coordinator/And", WordForm::Invariant, None),
        right: Box::new(keyword("lexeme:keyword_ability/haste")),
    };
    assert_laws(
        pp("With", complement),
        "with flying and haste",
        Category::PrepositionPhrase,
    );
    // Deluge, Moat, and Wallop provide authentic whole-document witnesses.
    for text in [
        "Tap all creatures without flying.",
        "Creatures without flying can't attack.",
        "Destroy target blue or black creature with flying.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
}
