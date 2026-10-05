#![allow(
    dead_code,
    reason = "each integration test uses its own subset of this support module"
)]

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;

pub static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

pub fn lexicon() -> &'static Lexicon {
    &LEXICON
}

pub fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    readings_with_lexicon(lexicon(), text, category)
}

pub fn readings_with_lexicon(
    lexicon: &Lexicon,
    text: &str,
    category: Category,
) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &category).unwrap();
    let mut values = BTreeSet::new();
    for value in grammar.readings(&forest) {
        let value = value.unwrap();
        assert_eq!(value.realize(lexicon).unwrap(), text);
        assert!(values.insert(value), "duplicate Reading for {text:?}");
    }
    values
}

pub fn ability_readings(text: &str) -> BTreeSet<Reading> {
    readings(text, Category::Ability)
}

pub fn noun_phrase_readings(text: &str) -> BTreeSet<Reading> {
    readings(text, Category::NounPhrase)
}

/// Require one admitted Reading containing every requested Category and exact
/// constituent surface. Repeated source substrings match any occurrence. The
/// whole node must realize to the substring; a substring of a node does not
/// count. Each call searches all Readings independently, so separate calls can
/// witness alternative analyses of the same text.
pub fn assert_constituents(text: &str, category: Category, expected: &[(Category, &str)]) {
    for (_, surface) in expected {
        assert!(
            !surface.is_empty() && text.contains(surface),
            "expected constituent surface {surface:?} is not a nonempty substring of {text:?}"
        );
    }
    let values = readings(text, category);
    let mut observed = Vec::new();
    for value in &values {
        let mut constituents = BTreeSet::new();
        value
            .visit(&mut |node| {
                if expected
                    .iter()
                    .any(|(category, _)| *category == node.category())
                {
                    constituents.insert((node.category(), node.realize(lexicon()).unwrap()));
                }
            })
            .unwrap();
        if expected
            .iter()
            .all(|(category, surface)| constituents.contains(&(*category, (*surface).to_owned())))
        {
            return;
        }
        observed.push(constituents);
    }
    panic!(
        "no single Reading of {text:?} at {category:?} contains all constituents {expected:?}; \
         observed constituents by Reading: {observed:?}"
    );
}
