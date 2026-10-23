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
                let scope = deckmaste_english_v4::judgment::scope(entry);
                if grammar.ranking_key(&reading, &env.lexicon, &scope) != Ok(key) {
                    failures.push(format!("{}: {}", entry.location, entry.text));
                    break;
                }
            }
        }
    }
    assert!(checked > 0);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// A distributive adjunct (*for each …*) attaches high: to the matrix
/// predicate, not inside the hollow infinitival.
#[test]
fn a_distributive_adjunct_attaches_to_the_outermost_predicate() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let entry = &deckmaste_english_v4::judgment::parse_file(
        "test",
        "+ Creature spells you cast cost {1} less to cast for each +1/+1 counter on this creature. => PrepositionPhrase \"for each +1/+1 counter on this creature\" as Adjunct of cost",
    )
    .unwrap()[0];
    let parse = judge.parse(entry).unwrap();
    let (key, best) = parse.best_ranked(1).remove(0);
    assert!(key.right_association_words < 0, "{key:?}");
    let (text, constituents) = grammar.constituents(&best, &env.lexicon).unwrap();
    let located = deckmaste_english_v4::judgment::Located { text, constituents };
    assert!(judge.endorses(entry, &located));
}

/// On Nicol Bolas, the Deceiver, *Nicol Bolas* is the face's own Shortened
/// Name and also the full name of another card. Both stay Readings; the
/// face's own name ranks first, in the forest and on the tree alike.
#[test]
fn a_face_s_own_name_ranks_before_another_card_s_name_spelled_alike() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let own = deckmaste_english_v4::corpus::card_name_lexeme("Nicol Bolas, the Deceiver");
    let scope = std::collections::BTreeSet::from([own]);
    let text = "Nicol Bolas deals 7 damage to each opponent.";
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &scope));
    let names = |reading: &deckmaste_syntax::Reading| {
        let (_, constituents) = grammar.constituents(reading, &env.lexicon).unwrap();
        constituents
            .iter()
            .filter_map(|c| c.head)
            .filter(|h| h.as_str().starts_with("name:"))
            .collect::<Vec<_>>()
    };
    let ranked = parse.best_ranked(usize::MAX);
    assert_eq!(ranked.len(), 2);
    let other = deckmaste_english_v4::corpus::card_name_lexeme("Nicol Bolas");
    assert!(names(&ranked[0].1).iter().all(|h| *h == own));
    assert!(names(&ranked[1].1).iter().all(|h| *h == other));
    assert!(ranked[0].0 < ranked[1].0);
    for (key, reading) in &ranked {
        assert_eq!(grammar.ranking_key(reading, &env.lexicon, &scope), Ok(*key));
    }
}

/// One forest item packs prefixes that divide its text differently: here the
/// postposed relative's head and Object meet before or after *tapped*, a
/// depictive in the head or a premodifier in the Object. The Adjunct *at
/// random* reads its head's words, so each prefix keeps its own key, and every
/// Reading's forest key is the key of its own tree.
#[test]
fn forest_keys_follow_each_packed_prefix_s_own_division_of_the_text() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let scope = std::collections::BTreeSet::new();
    let text = "Return to the battlefield tapped creature cards at random that were put into your graveyard this turn.";
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &scope));
    let ranked = parse.best_ranked(usize::MAX);
    let mut objects = std::collections::BTreeSet::new();
    for (key, reading) in &ranked {
        assert_eq!(grammar.ranking_key(reading, &env.lexicon, &scope), Ok(*key));
        let (realized, constituents) = grammar.constituents(reading, &env.lexicon).unwrap();
        objects.extend(
            constituents
                .iter()
                .filter(|c| c.function.is_some_and(|f| f.as_str() == "object"))
                .map(|c| realized[c.span.clone()].to_owned()),
        );
    }
    assert!(objects.contains("tapped creature cards"), "{objects:?}");
    assert!(objects.contains("creature cards"), "{objects:?}");
    let (least, _) = parse.cheapest_tier().unwrap();
    assert_eq!(least, ranked[0].0);
}
