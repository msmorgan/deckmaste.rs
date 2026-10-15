//! Registered label exclusivity: an open flavor-word label never duplicates a
//! registered ability word [CR#207.2c] or keyword ability [CR#702.1].

use deckmaste_lexical::ItalicHead;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_lexical_source_v4::Sources;

fn capitalized(lemma: &str) -> String {
    let mut chars = lemma.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

fn flavor_labels(environment: &LexicalEnvironment, text: &str) -> Vec<String> {
    environment
        .lexicon
        .analyze(text)
        .matches
        .into_iter()
        .filter_map(|found| match found.reading {
            LexicalReading::FlavorWord { label } => Some(label),
            _ => None,
        })
        .collect()
}

#[test]
fn a_registered_label_is_never_a_flavor_word() {
    let environment = LexicalEnvironment::load().unwrap();
    let sources = Sources::load().unwrap();
    for word in &sources.ability_words {
        let label = capitalized(&word.lemma);
        assert!(
            flavor_labels(&environment, &format!("{label} — Draw a card.")).is_empty(),
            "ability word {label}"
        );
        assert!(
            matches!(
                environment.lexicon.analyze_italic_head(&label),
                ItalicHead::AbilityWord(_)
            ),
            "ability word {label}"
        );
    }
    for ability in &sources.keyword_abilities {
        let label = capitalized(&ability.lemma);
        assert!(
            flavor_labels(&environment, &format!("{label} — Draw a card.")).is_empty(),
            "keyword {label}"
        );
        assert!(
            !matches!(
                environment.lexicon.analyze_italic_head(&label),
                ItalicHead::FlavorWord { .. }
            ),
            "keyword {label}"
        );
    }
}

#[test]
fn an_unregistered_label_is_a_flavor_word() {
    let environment = LexicalEnvironment::load().unwrap();
    // Scryfall's flavor words are, by definition, not registered labels.
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/catalogs/flavor-words.json");
    let catalog: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    for label in catalog["data"].as_array().unwrap() {
        let label = label.as_str().unwrap();
        assert_eq!(
            flavor_labels(&environment, &format!("{label} — Draw a card.")),
            [label],
            "{label}"
        );
    }
}

#[test]
fn an_anchor_word_is_a_registered_label_on_the_cards_that_print_it() {
    let environment = LexicalEnvironment::load().unwrap();
    let sources = Sources::load().unwrap();
    for anchor in &sources.anchor_words {
        let line = format!("• {} — Draw a card.", anchor.word);
        assert!(
            matches!(
                environment.lexicon.analyze_italic_head(&anchor.word),
                ItalicHead::AnchorWord(_)
            ),
            "anchor word {}",
            anchor.word
        );
        for card in &anchor.cards {
            let scope = [LexemeId::new(&format!("name:{card}"))].into();
            let analyzed = environment
                .lexicon
                .analyze_source_scoped(&line, None, &scope);
            assert!(
                !analyzed
                    .matches
                    .iter()
                    .any(|found| matches!(found.reading, LexicalReading::FlavorWord { .. })),
                "{} on {card}",
                anchor.word
            );
            assert!(
                analyzed.matches.iter().any(|found| matches!(
                    &found.reading,
                    LexicalReading::Word(value)
                        if value.lexeme == format!("anchor_word:{}", anchor.word).as_str()
                )),
                "{} on {card}",
                anchor.word
            );
        }
    }
}
