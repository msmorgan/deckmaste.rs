//! The judgment set (`judgments/README.md`): every file parses; the seed file
//! holds on the seed lexicon; every entry at or below the completed Stratum
//! holds on the full lexical environment.

#[allow(dead_code, reason = "the seed module also serves seed_proof.rs")]
mod seed;

use deckmaste_english_v4::COMPLETED_STRATUM;
use deckmaste_english_v4::corpus::UnitKind;
use deckmaste_english_v4::judgment::Judge;
use deckmaste_english_v4::judgment::Outcome;
use deckmaste_english_v4::profile::Profiler;
use deckmaste_lexical_source_v4::LexicalEnvironment;

fn files() -> Vec<(String, Vec<deckmaste_english_v4::judgment::Judgment>)> {
    deckmaste_english_v4::judgment::load_dir(&deckmaste_english_v4::judgments_dir())
        .unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn the_seed_judgments_hold() {
    let lexicon = seed::lexicon();
    let grammar = deckmaste_english_v4::bind(&lexicon, &seed::frames()).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &lexicon,
    };
    let (_, entries) = files()
        .into_iter()
        .find(|(name, _)| name == "seed.txt")
        .expect("judgments/seed.txt");
    let failures: Vec<String> = entries
        .iter()
        .filter_map(|e| match judge.evaluate(e) {
            Outcome::Holds => None,
            Outcome::Fails(why) => Some(format!("{}: {} — {why}", e.location, e.text)),
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn judgments_at_or_below_the_completed_stratum_hold() {
    let files = files();
    let env = LexicalEnvironment::load().unwrap();
    let profiler = Profiler::new(&env);
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames);
    let mut enforced = 0;
    let mut pending = 0;
    let mut failures = Vec::new();
    for (_, entries) in files.iter().filter(|(name, _)| name != "seed.txt") {
        for entry in entries {
            let analyzed = env.lexicon.analyze(&entry.text);
            let stratum = profiler.profile(UnitKind::Line, &analyzed).stratum;
            if COMPLETED_STRATUM.is_none_or(|done| stratum > done) {
                pending += 1;
                continue;
            }
            enforced += 1;
            let outcome = match &grammar {
                Ok(grammar) => Judge {
                    grammar,
                    lexicon: &env.lexicon,
                }
                .evaluate(entry),
                Err(e) => Outcome::Fails(format!("the grammar does not bind: {e}")),
            };
            if let Outcome::Fails(why) = outcome {
                failures.push(format!(
                    "{} (Stratum {stratum}): {} — {why}",
                    entry.location, entry.text
                ));
            }
        }
    }
    println!("judgments: {enforced} enforced, {pending} pending");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn false_judgments_fail() {
    let lexicon = seed::lexicon();
    let grammar = deckmaste_english_v4::bind(&lexicon, &seed::frames()).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &lexicon,
    };
    let false_entries = r##"
+ Destroy target creature. => NounPhrase "target creature" as Subject of destroy
+ Destroy target creature. => NounPhrase "target creature" as Object of exile
+ Destroy target creature. => NounPhrase "creature target"
+ Destroy target creature. => PrepositionPhrase "target creature"
+ Destroy target creature. => readings 2
+ Search your library for a card. => PrepositionPhrase "for a card" as Modifier of library
+ Search your library for a card. => !NounPhrase "a card"
+ Exile target creature with a counter on it. => PrepositionPhrase "with a counter on it" as Adjunct of creature
+ Exile the card you put into your hand. => Clause "you put into your hand" gap "hand"
+ Exile the card you put into your hand. => NounPhrase "the card"#2
+ Put the card.
* Destroy target creature.
+ Destroy target wombat.
*NounPhrase target creature
+Nonesuch target creature
"##;
    for entry in deckmaste_english_v4::judgment::parse_file("false", false_entries).unwrap() {
        assert!(
            matches!(judge.evaluate(&entry), Outcome::Fails(_)),
            "{}: {}",
            entry.location,
            entry.text
        );
    }
}

#[test]
fn every_word_a_judgment_names_is_declared() {
    let env = LexicalEnvironment::load().unwrap();
    let lexemes = env.lexicon.lexemes();
    let declared = |name: &str| {
        lexemes
            .iter()
            .any(|(id, l)| id.as_str() == name || l.lemma.eq_ignore_ascii_case(name))
    };
    let mut unknown = Vec::new();
    for (name, entries) in files() {
        if name == "seed.txt" {
            continue;
        }
        for entry in entries {
            for assertion in entry.groups.iter().flatten().chain(&entry.negated) {
                let words = assertion
                    .head
                    .iter()
                    .chain(assertion.function.iter().map(|(_, of)| of));
                for word in words.filter(|w| !declared(w)) {
                    unknown.push(format!("{}: {word}", entry.location));
                }
            }
        }
    }
    assert!(unknown.is_empty(), "not a lemma or lexeme id:\n{}", unknown.join("\n"));
}
