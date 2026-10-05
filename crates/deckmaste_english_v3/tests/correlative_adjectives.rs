use deckmaste_english_v3::{
    grammar::{Category, Grammar, Reading, Word},
    parse,
};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Lexicon, Number, SurfaceCase, WordForm,
};
use std::{collections::BTreeSet, path::Path, sync::OnceLock};

fn lexicon() -> &'static Lexicon {
    static LEXICON: OnceLock<Lexicon> = OnceLock::new();
    LEXICON.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        Lexicon::new(
            deckmaste_lexical_source::load_workspace(&root)
                .unwrap()
                .lexemes,
        )
        .unwrap()
    })
}
fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = lexicon().analyze(text);
    let forest = parse(&grammar, lexicon(), &input, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|value| {
            let value = value.unwrap();
            assert_eq!(value.realize(lexicon()).unwrap(), text);
            value
        })
        .collect()
}
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

fn color(id: &str) -> Reading {
    Reading::Adjective {
        form: 0,
        head: word(&format!("vocab:ColorWord/{id}"), WordForm::Invariant, None),
    }
}

#[test]
fn independently_constructed_correlative_adjectives_have_explicit_hosts() {
    let both = Reading::BothCoordination {
        category: Category::CorrelativeAdjectivePhrase,
        form: 0,
        marker: word("vocab:FloatedQuantifier/Both", WordForm::Invariant, None),
        left: Box::new(color("Red")),
        coordinator: word("vocab:Coordinator/And", WordForm::Invariant, None),
        right: Box::new(color("Green")),
    };
    assert_laws(
        both.clone(),
        "both red and green",
        Category::CorrelativeAdjectivePhrase,
    );
    let complement = Reading::AdjectivalCorrelativeComplement {
        form: 0,
        phrase: Box::new(both.clone()),
    };
    assert_laws(
        complement,
        "both red and green",
        Category::PredicativeComplement,
    );
    let mut creature = word(
        "lexeme:type/creature",
        WordForm::Singular,
        Some(Number::Singular),
    );
    creature.countability = Some(true);
    let nominal = Reading::CorrelativePostpositiveNominal {
        form: 0,
        head: Box::new(Reading::Noun {
            form: 0,
            head: creature,
        }),
        modifier: Box::new(both),
    };
    assert_laws(nominal, "creature both red and green", Category::Nominal);
    for text in [
        "it is both red and green",
        "it is either red or green",
        "it is neither red nor green",
    ] {
        assert!(
            !readings(text, Category::FiniteClause).is_empty(),
            "no predicative Reading for {text:?}"
        );
    }
    assert!(readings("both red and green", Category::AdjectivePhrase).is_empty());
    for text in [
        "the both red and green creature",
        "the either red or green creature",
        "the neither red nor green creature",
    ] {
        assert!(
            readings(text, Category::NounPhrase).is_empty(),
            "correlative premodifier incorrectly licensed for {text:?}"
        );
    }
}

#[test]
fn correlative_adjectival_serials_preserve_marker_and_oxford_comma() {
    let end = Reading::CorrelativeSeriesEnd {
        category: Category::CorrelativeAdjectiveSeries,
        form: 0,
        left: Box::new(color("Green")),
        coordinator: word("vocab:Coordinator/Or", WordForm::Invariant, None),
        right: Box::new(color("Blue")),
    };
    let serial = Reading::EitherSerialCoordination {
        category: Category::CorrelativeAdjectivePhrase,
        form: 0,
        marker: word("vocab:Determinative/Either", WordForm::Invariant, None),
        left: Box::new(color("Red")),
        rest: Box::new(end),
    };
    assert_laws(
        serial,
        "either red, green, or blue",
        Category::CorrelativeAdjectivePhrase,
    );
    for text in [
        "neither red, green, nor blue",
        "either red, green, black, or blue",
    ] {
        assert!(
            !readings(text, Category::CorrelativeAdjectivePhrase).is_empty(),
            "no serial for {text:?}"
        );
    }
    for text in [
        "both red, green, and blue",
        "either red, green or blue",
        "neither red, green nor blue",
        "both red or green",
        "neither red or green",
        "either red nor green",
    ] {
        assert!(
            readings(text, Category::CorrelativeAdjectivePhrase).is_empty(),
            "invalid marker or punctuation for {text:?}"
        );
    }
}
