//! The Targeting Marker `det:target` is the only thing that builds a
//! Targeting Marker determiner: an adjective that needs an overt determiner
//! (*only*) premodifies an ordinary nominal. Judgments name no Production, so
//! this checks the Readings directly.

use deckmaste_lexical::LexicalReading;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_syntax::Reading;

/// Whether some Constituent has the Targeting Marker as a leaf of its own.
fn marked(reading: &Reading) -> bool {
    match reading {
        Reading::Node(node) => {
            node.children.iter().any(|c| {
                matches!(c, Reading::Leaf(LexicalReading::Word(w)) if w.lexeme == "det:target")
            }) || node.children.iter().any(marked)
        }
        Reading::Leaf(_) => false,
    }
}

/// The lexeme ids of a Reading's word leaves.
fn leaves(reading: &Reading, out: &mut Vec<String>) {
    match reading {
        Reading::Node(node) => node.children.iter().for_each(|c| leaves(c, out)),
        Reading::Leaf(LexicalReading::Word(word)) => out.push(word.lexeme.to_string()),
        Reading::Leaf(_) => {}
    }
}

fn noun_phrase_readings(text: &str) -> Vec<Reading> {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let analyzed = env.lexicon.analyze(text);
    grammar
        .parse_category(&analyzed, "NounPhrase")
        .unwrap()
        .readings()
}

#[test]
fn only_heads_no_targeting_determiner() {
    let readings = noun_phrase_readings("the only creature you control");
    assert_eq!(readings.len(), 1);
    assert!(!marked(&readings[0]), "{:#?}", readings[0]);
}

#[test]
fn the_targeting_marker_heads_the_determiner() {
    for text in [
        "target creature",
        "two target creatures",
        "another target creature",
        "up to one target artifact or creature",
    ] {
        let readings = noun_phrase_readings(text);
        assert_eq!(readings.len(), 1, "{text}");
        assert!(marked(&readings[0]), "{text}: {:#?}", readings[0]);
        let mut words = Vec::new();
        leaves(&readings[0], &mut words);
        assert!(
            words.contains(&"det:target".to_owned()),
            "{text}: {words:?}"
        );
        assert!(
            !words.contains(&"adj:target".to_owned()),
            "{text}: {words:?}"
        );
    }
}
