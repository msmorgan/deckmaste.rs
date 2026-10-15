//! The whole Lexical Environment loads, and its vocabulary analyses as
//! declared.

use deckmaste_lexical::LexicalReading;
use deckmaste_lexical_source_v4::LexicalEnvironment;

#[test]
fn the_environment_loads() {
    let environment = LexicalEnvironment::load().unwrap();
    let analyzed = environment.lexicon.analyze("Destroy target creature.");
    let ids: Vec<String> = analyzed
        .matches
        .iter()
        .filter_map(|m| match &m.reading {
            LexicalReading::Word(value) => Some(value.lexeme.to_string()),
            _ => None,
        })
        .collect();
    assert!(ids.contains(&"action:destroy".to_owned()), "{ids:?}");
}

#[test]
fn capitalized_prefix_has_one_lexical_analysis() {
    let environment = LexicalEnvironment::load().unwrap();
    for (text, prefix_end) in [("Nonbasic", 3), ("Non-basic", 4)] {
        let analyzed = environment.lexicon.analyze(text);
        let prefixes: Vec<_> = analyzed.matches.iter().filter(|found| {
            found.start == 0 && found.end == prefix_end && matches!(&found.reading, LexicalReading::Word(value) if value.lexeme.as_str() == "affix:non")
        }).collect();
        assert_eq!(prefixes.len(), 1, "{text}: {prefixes:?}");
    }
}

#[test]
fn catalog_determinatives_are_explicit_and_retain_verbal_homographs() {
    use deckmaste_lexical::{Category, SourceKind, WordForm};
    let env = LexicalEnvironment::load().unwrap();
    let sources = deckmaste_lexical_source_v4::Sources::load().unwrap();
    for (text, ability) in [("enchanted", "enchant"), ("equipped", "equip")] {
        let owner = sources
            .keyword_abilities
            .iter()
            .find(|entry| entry.lemma == ability)
            .unwrap()
            .rule
            .number();
        let analyzed = env.lexicon.analyze(text);
        let values: Vec<_> = analyzed
            .matches
            .iter()
            .filter_map(|m| {
                if m.start != 0 || m.end != text.chars().count() {
                    return None;
                }
                match &m.reading {
                    LexicalReading::Word(v) => Some(v),
                    _ => None,
                }
            })
            .collect();
        assert!(
            values.iter().any(|v| {
                let l = &env.lexicon.lexemes()[&v.lexeme];
                l.category == Category::Determinative
                    && l.source.kind == SourceKind::Catalog
                    && l.source.owner == owner
            }),
            "{text}"
        );
        assert!(
            values.iter().any(
                |v| env.lexicon.lexemes()[&v.lexeme].category == Category::Verb
                    && v.form == WordForm::PastParticiple
            ),
            "{text}"
        );
    }
    for text in ["tapped", "white", "enchanting", "equipping"] {
        let analyzed = env.lexicon.analyze(text);
        assert!(!analyzed.matches.iter().any(|m| matches!(&m.reading, LexicalReading::Word(v) if env.lexicon.lexemes()[&v.lexeme].category == Category::Determinative)), "{text}");
    }
}
