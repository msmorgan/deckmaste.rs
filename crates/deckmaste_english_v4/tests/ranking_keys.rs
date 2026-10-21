//! The Ranking Key extracted from the Packed Forest equals the key computed
//! from each Reading's tree, over the judgment set.

use deckmaste_english_v4::judgment::Judge;
use deckmaste_lexical_source_v4::LexicalEnvironment;

#[test]
fn forest_keys_equal_tree_keys_over_the_judgment_set() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let files =
        deckmaste_english_v4::judgment::load_dir(&deckmaste_english_v4::judgments_dir()).unwrap();
    let mut checked = 0;
    let mut failures = Vec::new();
    for (file, entries) in files {
        if file == "seed.txt" {
            continue;
        }
        for entry in entries.iter().filter(|e| e.grammatical) {
            let Some(parse) = judge.parse(entry) else {
                continue;
            };
            if parse.derivation_count() > deckmaste_english_v4::judgment::READING_LIMIT {
                continue;
            }
            for (key, reading) in parse.best_ranked(usize::MAX) {
                checked += 1;
                if grammar.ranking_key(&reading, &env.lexicon) != Ok(key) {
                    failures.push(format!("{}: {}", entry.location, entry.text));
                    break;
                }
            }
        }
    }
    assert!(checked > 0);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
