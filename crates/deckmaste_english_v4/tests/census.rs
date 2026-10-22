//! The census on the seed lexicon: each verdict, and the per-Stratum counts.

#[allow(dead_code, reason = "the seed module also serves seed_proof.rs")]
mod seed;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_english_v4::census::Unread;
use deckmaste_english_v4::census::Verdict;
use deckmaste_english_v4::corpus::Corpus;
use deckmaste_english_v4::corpus::Face;
use deckmaste_lexical_source_v4::LexicalEnvironment;

fn face(identity: &str, text: &str) -> Face {
    Face {
        identity: identity.to_owned(),
        name: identity.to_owned(),
        card: identity.to_owned(),
        scope: BTreeSet::new(),
        type_line: Some("Sorcery".to_owned()),
        oracle_text: Some(text.to_owned()),
    }
}

#[test]
fn each_unit_is_read_known_wrong_only_or_unread() {
    let env = LexicalEnvironment {
        lexicon: seed::lexicon(),
        frames: seed::frames(),
    };
    let mut corpus = Corpus::default();
    corpus.push(face(
        "a",
        "Destroy target creature.\nSearch your library for a card.",
    ));
    corpus.push(face("b", "Put the card."));
    corpus.push(face("c", "Destroy target wombat."));
    corpus.push(face("d", "Destroy  target creature."));
    // The judgment makes every Reading of the second line wrong.
    let judgments = deckmaste_english_v4::judgment::parse_file(
        "test",
        r#"+ Search your library for a card. => PrepositionPhrase "for a card" as Modifier of library"#,
    )
    .unwrap();
    let baseline = BTreeMap::from([("a".to_owned(), true), ("b".to_owned(), false)]);
    let report = deckmaste_english_v4::census::run(&corpus, &env, &judgments, Some(&baseline));
    assert_eq!(report.bind_error, None);
    let verdicts: Vec<(&str, &Verdict)> = report
        .units
        .iter()
        .filter(|u| u.text != "Sorcery")
        .map(|u| (u.text.as_str(), &u.verdict))
        .collect();
    assert_eq!(
        verdicts,
        [
            ("Destroy target creature.", &Verdict::Read),
            ("Search your library for a card.", &Verdict::KnownWrongOnly),
            ("Put the card.", &Verdict::Unread(Unread::NoReading)),
            (
                "Destroy target wombat.",
                &Verdict::Unread(Unread::VocabularyGap)
            ),
            (
                "Destroy  target creature.",
                &Verdict::Unread(Unread::NoReading)
            ),
        ]
    );
    // Type lines are units too; the seed lexicon has no card types.
    assert!(
        report
            .units
            .iter()
            .filter(|u| u.text == "Sorcery")
            .all(|u| u.verdict == Verdict::Unread(Unread::VocabularyGap))
    );
    let totals = report.strata.iter().fold((0, 0, 0, 0), |t, r| {
        (
            t.0 + r.units,
            t.1 + r.read,
            t.2 + r.known_wrong_only,
            t.3 + r.unread,
        )
    });
    assert_eq!(totals, (9, 1, 1, 7));
    assert_eq!(
        report.strata.iter().map(|r| r.law_failures).sum::<usize>(),
        0
    );
    // No face is read: each has an unread type line.
    assert_eq!(report.strata.iter().map(|r| r.faces_read).sum::<usize>(), 0);
    assert_eq!(
        report
            .strata
            .iter()
            .filter_map(|r| r.baseline_unread_faces)
            .sum::<usize>(),
        1
    );
    assert_eq!(report.baseline_unread_missing, Some(Vec::new()));
}

#[test]
fn an_unbound_grammar_leaves_every_unit_unread() {
    let env = LexicalEnvironment {
        lexicon: seed::lexicon(),
        frames: deckmaste_syntax::FrameCatalog::default(),
    };
    let mut corpus = Corpus::default();
    corpus.push(face("a", "Destroy target creature."));
    let report = deckmaste_english_v4::census::run(&corpus, &env, &[], None);
    assert!(report.bind_error.is_some());
    assert!(
        report
            .units
            .iter()
            .all(|u| u.verdict == Verdict::Unread(Unread::Unbound))
    );
}

/// A face's units are parsed and counted one by one: its count is the sum of
/// its units' Derivations, never their product, and a quotation spanning a
/// line break stays in one unit.
#[test]
fn a_face_counts_the_sum_of_its_units() {
    let env = LexicalEnvironment {
        lexicon: seed::lexicon(),
        frames: seed::frames(),
    };
    let mut corpus = Corpus::default();
    let lines = [
        "Exile target creature with a counter on it.",
        "Untap all permanents you control during each other player's untap step.",
        "Exile target creature with a counter on it.",
    ];
    corpus.push(face("a", &lines.join("\n")));
    corpus.push(face(
        "b",
        "Destroy target creature.\n\"Destroy\ntarget creature.\"",
    ));
    let report = deckmaste_english_v4::census::run(&corpus, &env, &[], None);
    let counts: Vec<u128> = report
        .units
        .iter()
        .filter(|u| u.face == "a" && u.text != "Sorcery")
        .map(|u| u.derivations)
        .collect();
    assert_eq!(counts.len(), 3);
    assert!(counts.iter().all(|&d| d >= 2), "{counts:?}");
    let sum: u128 = counts.iter().sum();
    assert!(sum < counts.iter().product::<u128>());
    let face_a: u128 = report
        .units
        .iter()
        .filter(|u| u.face == "a")
        .map(|u| u.derivations)
        .sum();
    assert_eq!(face_a, sum);
    assert_eq!(
        report.total_derivations,
        report.units.iter().map(|u| u.derivations).sum::<u128>()
    );
    let b: Vec<&str> = report
        .units
        .iter()
        .filter(|u| u.face == "b" && u.text != "Sorcery")
        .map(|u| u.text.as_str())
        .collect();
    assert_eq!(
        b,
        ["Destroy target creature.", "\"Destroy\ntarget creature.\""]
    );
}

#[test]
fn each_editorial_correction_is_well_formed_and_changes_its_text() {
    let corrections = deckmaste_english_v4::corpus::editorial_corrections();
    let records = deckmaste_english_v4::EDITORIAL_CORRECTIONS
        .lines()
        .filter(|line| line.starts_with("card: "))
        .count();
    assert_eq!(corrections.len(), records);
    for correction in &corrections {
        assert_ne!(correction.oracle, correction.corrected, "{correction:?}");
    }
}

#[test]
fn the_deferral_list_names_each_unit_once() {
    let lines = deckmaste_english_v4::DEFERRED
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .count();
    let deferrals = deckmaste_english_v4::census::deferrals();
    assert_eq!(deferrals.len(), lines);
    assert!(
        deferrals
            .iter()
            .all(|(name, text)| !name.is_empty() && !text.is_empty())
    );
}

/// A unit of two sentences, each with 2 Readings: 4 Derivations, the product,
/// and a Surplus of 2, one per ambiguous Segment.
#[test]
fn a_unit_counts_its_segments_and_their_surplus() {
    use deckmaste_syntax::Segmentation;
    let env = LexicalEnvironment {
        lexicon: seed::lexicon(),
        frames: seed::frames(),
    };
    let sentence = "Untap all permanents you control during each other player's untap step.";
    let mut corpus = Corpus::default();
    corpus.push(face(
        "a",
        &format!("{sentence} {sentence}\nDestroy target creature."),
    ));
    let report = deckmaste_english_v4::census::run(&corpus, &env, &[], None);
    let unit = |text: &str| report.units.iter().find(|u| u.text == text).unwrap();
    let pair = unit(&format!("{sentence} {sentence}"));
    assert_eq!(pair.derivations, 4);
    assert_eq!(pair.surplus, 2);
    let Some(Segmentation::Segments(segments)) = &pair.segments else {
        panic!("{:?}", pair.segments);
    };
    let counts: Vec<(&str, u128)> = segments
        .iter()
        .map(|s| (s.text.as_str(), s.derivations))
        .collect();
    let bare = sentence.trim_end_matches('.');
    assert_eq!(counts, [(bare, 2), (bare, 2)]);
    let single = unit("Destroy target creature.");
    assert_eq!((single.derivations, single.surplus), (1, 0));
    // A unit without a Derivation has no Segments and no Surplus.
    let type_line = unit("Sorcery");
    assert_eq!((type_line.segments.clone(), type_line.surplus), (None, 0));
    assert_eq!(report.total_surplus, 2);
    assert_eq!(report.ambiguous_units, 1);
    assert_eq!(report.ambiguous_segments, 2);
    assert_eq!(report.not_segmentable, Vec::<String>::new());
    assert_eq!(
        report.total_surplus,
        report.units.iter().map(|u| u.surplus).sum::<u128>()
    );
}
