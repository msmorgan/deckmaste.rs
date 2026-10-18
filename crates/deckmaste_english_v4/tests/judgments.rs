//! The judgment set (`judgments/README.md`): every file parses; the seed file
//! holds on the seed lexicon; every entry at or below the completed Stratum
//! holds on the full lexical environment, except one whose text is a deferred
//! unit (`deferred.txt`), which is pending while it stays deferred.

#[allow(dead_code, reason = "the seed module also serves seed_proof.rs")]
mod seed;

use std::sync::OnceLock;

use deckmaste_english_v4::COMPLETED_STRATUM;
use deckmaste_english_v4::corpus::UnitKind;
use deckmaste_english_v4::judgment::Judge;
use deckmaste_english_v4::judgment::Outcome;
use deckmaste_english_v4::profile::Profiler;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_syntax::CompiledGrammar;

fn environment() -> &'static (LexicalEnvironment, CompiledGrammar) {
    static ENVIRONMENT: OnceLock<(LexicalEnvironment, CompiledGrammar)> = OnceLock::new();
    ENVIRONMENT.get_or_init(|| {
        let env = LexicalEnvironment::load().unwrap();
        let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
        (env, grammar)
    })
}

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
    check_judgments(COMPLETED_STRATUM);
}

#[test]
fn catalog_determinative_judgments_hold() {
    let (env, grammar) = environment();
    let judge = Judge {
        grammar,
        lexicon: &env.lexicon,
    };
    let (_, entries) = files()
        .into_iter()
        .find(|(name, _)| name == "oracle_determinatives.txt")
        .unwrap();
    for entry in entries {
        assert!(
            matches!(judge.evaluate(&entry), Outcome::Holds),
            "{}: {}: {:?}",
            entry.location,
            entry.text,
            judge.evaluate(&entry)
        );
    }
}

fn check_judgments(completed: Option<u8>) {
    let files = files();
    let (env, grammar) = environment();
    let deferred: std::collections::BTreeSet<&str> = deckmaste_english_v4::census::deferrals()
        .into_iter()
        .map(|(_, text)| text)
        .collect();
    let deferred = &deferred;
    let entries: Vec<_> = files
        .iter()
        .filter(|(name, _)| name != "seed.txt")
        .flat_map(|(_, entries)| entries)
        .collect();
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let chunk = entries.len().div_ceil(threads).max(1);
    let (enforced, pending, failures) = std::thread::scope(|scope| {
        let workers: Vec<_> = entries
            .chunks(chunk)
            .map(|entries| {
                scope.spawn(move || {
                    let profiler = Profiler::new(env);
                    let judge = Judge {
                        grammar,
                        lexicon: &env.lexicon,
                    };
                    let mut enforced = 0;
                    let mut pending = 0;
                    let mut failures = Vec::new();
                    for entry in entries {
                        let analyzed = env.lexicon.analyze_source_scoped(
                            &entry.text,
                            None,
                            &deckmaste_english_v4::judgment::scope(entry),
                        );
                        let stratum = profiler.profile(UnitKind::Line, &analyzed).stratum;
                        if completed.is_none_or(|done| stratum > done)
                            || deferred.contains(entry.text.as_str())
                        {
                            pending += 1;
                            continue;
                        }
                        enforced += 1;
                        if let Outcome::Fails(why) = judge.evaluate(entry) {
                            failures.push(format!(
                                "{} (Stratum {stratum}): {} — {why}",
                                entry.location, entry.text
                            ));
                        }
                    }
                    (enforced, pending, failures)
                })
            })
            .collect();
        // Joining contiguous chunks in input order keeps diagnostics
        // deterministic.
        workers.into_iter().fold(
            (0, 0, Vec::new()),
            |(enforced, pending, mut failures), worker| {
                let (worker_enforced, worker_pending, worker_failures) =
                    worker.join().expect("judgment worker");
                failures.extend(worker_failures);
                (
                    enforced + worker_enforced,
                    pending + worker_pending,
                    failures,
                )
            },
        )
    });
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
    let (env, _) = environment();
    let lexemes = env.lexicon.lexemes();
    // A card name is no lemma: *Greater Good* does not declare *greater*.
    let declared = |name: &str| {
        lexemes.iter().any(|(id, l)| {
            id.as_str() == name
                || !id.as_str().starts_with("name:") && l.lemma.eq_ignore_ascii_case(name)
        })
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
    assert!(
        unknown.is_empty(),
        "not a lemma or lexeme id:\n{}",
        unknown.join("\n")
    );
}

#[test]
fn stratum_zero_spines_hold_over_seed_payloads() {
    let (env, grammar) = environment();
    let judge = Judge {
        grammar,
        lexicon: &env.lexicon,
    };
    let (_, entries) = files()
        .into_iter()
        .find(|(name, _)| name == "stratum_zero.txt")
        .unwrap();
    let failures: Vec<_> = entries
        .iter()
        .filter_map(|entry| match judge.evaluate(entry) {
            Outcome::Holds => None,
            Outcome::Fails(why) => Some(format!("{}: {} — {why}", entry.location, entry.text)),
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
