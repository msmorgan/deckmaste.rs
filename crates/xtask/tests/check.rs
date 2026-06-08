// Drives the pure check logic against the fixture + in-memory sources, so it
// never touches the gitignored real cr.txt.
use std::collections::BTreeMap;
use std::path::Path;

use xtask::cite::check_sources;
use xtask::cr::{Rules, checksum_text};
use xtask::lockfile::Lockfile;

const FIXTURE: &str = include_str!("fixtures/cr_fixture.txt");

fn lock_with(entries: &[(&str, &str)]) -> Lockfile {
    Lockfile {
        cr_date: "2026-04-17".into(),
        wizards_url: "x".into(),
        checksums: entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    }
}

#[test]
fn clean_when_checksums_match() {
    let rules = Rules::parse(FIXTURE).unwrap();
    let lock = lock_with(&[("702.158b", &checksum_text("Placeholder sector text."))]);
    let sources = vec![(
        Path::new("x.rs").to_path_buf(),
        "// [CR#702.158b]\n".to_string(),
    )];
    let outcome = check_sources(&rules, &lock, &sources).unwrap();
    assert!(outcome.stale.is_empty());
}

#[test]
fn flags_changed_and_gone() {
    let rules = Rules::parse(FIXTURE).unwrap();
    let lock = lock_with(&[
        ("702.158b", "deadbeefdeadbeef"), // wrong checksum -> CHANGED
        ("999.9z", "0000000000000000"),   // not in fixture -> GONE
    ]);
    let sources = vec![(
        Path::new("x.rs").to_path_buf(),
        "// [CR#702.158b]\n// [CR#999.9z]\n".to_string(),
    )];
    let outcome = check_sources(&rules, &lock, &sources).unwrap();
    let kinds: BTreeMap<_, _> = outcome
        .stale
        .iter()
        .map(|s| (s.rule.clone(), s.reason))
        .collect();
    assert_eq!(kinds.get("702.158b"), Some(&xtask::cite::Reason::Changed));
    assert_eq!(kinds.get("999.9z"), Some(&xtask::cite::Reason::Gone));
}

#[test]
fn list_collects_sites_with_members() {
    let rules = Rules::parse(FIXTURE).unwrap();
    let sources = vec![(
        std::path::Path::new("x.rs").to_path_buf(),
        "// [CR#704.5k..704.5n]\n".to_string(),
    )];
    let listed = xtask::cite::list_sites(&rules, &sources).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].1, vec!["704.5k", "704.5m", "704.5n"]);
}

#[test]
fn bless_builds_lockfile_from_cited_rules() {
    let rules = Rules::parse(FIXTURE).unwrap();
    let sources = vec![(
        std::path::Path::new("x.rs").to_path_buf(),
        "// [CR#702.158b,100.1a]\n".to_string(),
    )];
    let lock =
        xtask::cite::build_lockfile(&rules, &sources, "https://example.test/cr.txt").unwrap();
    assert_eq!(lock.cr_date, "2026-04-17");
    assert_eq!(lock.wizards_url, "https://example.test/cr.txt");
    assert_eq!(lock.checksums.len(), 2);
    assert_eq!(
        lock.checksums["702.158b"],
        checksum_text("Placeholder sector text.")
    );
}

#[test]
fn range_with_missing_endpoint_is_gone_not_panic() {
    let rules = Rules::parse(FIXTURE).unwrap();
    let lock = lock_with(&[]);
    // Built via format! so this test file doesn't itself contain a literal
    // `[CR#…]` token that `cite check` would scan as a real citation.
    let content = format!("// [CR#{}]\n", "704.5k..999.9z");
    let sources = vec![(Path::new("x.rs").to_path_buf(), content)];
    let outcome = check_sources(&rules, &lock, &sources).unwrap();
    assert_eq!(outcome.stale.len(), 1);
    assert_eq!(outcome.stale[0].reason, xtask::cite::Reason::Gone);
    assert_eq!(outcome.stale[0].rule, "704.5k..999.9z");
}

#[test]
fn format_diff_shows_old_and_new() {
    let out = xtask::cite::format_diff("702.158b", Some("old text"), Some("new text"));
    assert!(out.contains("702.158b"));
    assert!(out.contains("- old text"));
    assert!(out.contains("+ new text"));
}
