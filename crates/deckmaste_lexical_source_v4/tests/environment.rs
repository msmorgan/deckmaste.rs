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
