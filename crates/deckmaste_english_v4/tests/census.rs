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
                &Verdict::Unread(Unread::Law(
                    "law 1: the spacing at char 7 is not the spacing realization writes".to_owned()
                ))
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
        1
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
