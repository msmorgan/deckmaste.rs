//! CR-rule coverage: scan citations, tier by binding strength, ratchet.

/// Every rule mentioned in `[CR#…]` brackets in `text`.
///
/// Bracket content is comma-separated tokens; a `A..B` token yields both
/// endpoints (ranges in this repo are always adjacent subrules, so the two
/// endpoints are the whole range — and this keeps the scan free of any
/// `cr.json` ordering dependency).
#[must_use]
pub fn extract_bracket_rules(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[CR#") {
        rest = &rest[start + 4..];
        let Some(end) = rest.find(']') else { break };
        let body = &rest[..end];
        rest = &rest[end + 1..];
        // An ellipsis-only body is prose *about* the citation format (a doc
        // writing `[CR#…]`), not a citation — the citation checker resolves it
        // as "placeholder" and ignores it; match that. Deliberately narrow:
        // `[CR#]` and `[CR#rule]` still fall through, as those are typos.
        if matches!(body.trim(), "..." | "…") {
            continue;
        }
        for tok in body.split(',') {
            let tok = tok.trim();
            if let Some((a, b)) = tok.split_once("..") {
                push_rule(&mut out, a);
                push_rule(&mut out, b);
            } else {
                push_rule(&mut out, tok);
            }
        }
    }
    out
}

fn push_rule(out: &mut Vec<String>, s: &str) {
    let s = s.trim();
    if !s.is_empty() {
        out.push(s.to_string());
    }
}

/// Every rule in `#[cr("…", "…")]` attributes in `text`.
#[must_use]
pub fn extract_attr_rules(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("#[cr(") {
        rest = &rest[start + 5..];
        let Some(end) = rest.find(')') else { break };
        let body = &rest[..end];
        rest = &rest[end + 1..];
        let mut chars = body.char_indices();
        while let Some((i, c)) = chars.next() {
            if c == '"'
                && let Some(close) = body[i + 1..].find('"')
            {
                push_rule(&mut out, &body[i + 1..i + 1 + close]);
                // advance past the closing quote
                for _ in 0..=close {
                    chars.next();
                }
            }
        }
    }
    out
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

/// Binding strength of a citation site. Ordered `Mentioned < Bound < Tested`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Tier {
    Mentioned,
    Bound,
    Tested,
}

/// Dirs never walked: VCS/build/generated/fixture trees. Repo-specific source
/// excludes come from `cite-config.json` and are passed in separately.
const SKIP_DIRS: &[&str] = &[
    ".git",
    ".jj",
    ".workspaces",
    "target",
    "data",
    ".claude",
    "plugins/wizards",
    "docs/superpowers",
];
const EXTENSIONS: &[&str] = &["rs", "md", "ron", "idr", "lean"];

/// Classify a repo-relative path into its citation tier.
#[must_use]
#[expect(
    clippy::case_sensitive_file_extension_comparisons,
    reason = "repo paths are always lowercase-extension; a `Path::extension()` \
              round-trip would only add allocation for the same answer"
)]
pub fn tier_for_path(rel: &str) -> Tier {
    if rel.starts_with("docs/") || rel.ends_with(".md") {
        return Tier::Mentioned;
    }
    let is_test =
        rel.contains("/tests/") || rel.contains("/Proofs/") || rel.starts_with("plugins/testing/");
    if is_test { Tier::Tested } else { Tier::Bound }
}

/// Walk `root`, extracting every cited rule and keeping the strongest tier.
///
/// `excludes` are repo-relative path prefixes to skip (from
/// `cite-config.json`'s `sources.exclude`, e.g. `crates/xtask/`).
#[must_use]
pub fn scan_tree(root: &Path, excludes: &[String]) -> BTreeMap<String, Tier> {
    let mut acc: BTreeMap<String, Tier> = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let rel = path.strip_prefix(root).unwrap_or(&path);
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if path.is_dir() {
                if SKIP_DIRS.iter().any(|s| rel_str == *s)
                    || excludes.iter().any(|e| starts_with_prefix(&rel_str, e))
                {
                    continue;
                }
                stack.push(path);
                continue;
            }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !EXTENSIONS.contains(&ext) {
                continue;
            }
            if excludes.iter().any(|e| starts_with_prefix(&rel_str, e)) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let tier = tier_for_path(&rel_str);
            let mut rules = extract_bracket_rules(&text);
            if ext == "rs" {
                rules.extend(extract_attr_rules(&text));
            }
            for rule in rules {
                let slot = acc.entry(rule).or_insert(Tier::Mentioned);
                if tier > *slot {
                    *slot = tier;
                }
            }
        }
    }
    acc
}

/// `cite-config.json` excludes are written like `crates/xtask/` (trailing
/// slash) or `CLAUDE.md`; match both dir-prefix and exact-file forms.
fn starts_with_prefix(rel: &str, prefix: &str) -> bool {
    let p = prefix.trim_end_matches('/');
    rel == p || rel.starts_with(&format!("{p}/"))
}

/// The committed coverage floor. Sets, not counts — each named rule is pinned
/// at its tier and may not regress.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Lock {
    pub tested: Vec<String>,
    pub strong: Vec<String>,
}

/// Split a scan into the ratcheted tiers (`strong` = Tested ∪ Bound).
#[must_use]
pub fn lock_from_scan(scan: &BTreeMap<String, Tier>) -> Lock {
    let mut tested: Vec<String> = scan
        .iter()
        .filter(|(_, t)| **t == Tier::Tested)
        .map(|(r, _)| r.clone())
        .collect();
    let mut strong: Vec<String> = scan
        .iter()
        .filter(|(_, t)| **t >= Tier::Bound)
        .map(|(r, _)| r.clone())
        .collect();
    tested.sort();
    tested.dedup();
    strong.sort();
    strong.dedup();
    Lock { tested, strong }
}

/// Rules in `baseline` that `current` no longer covers at their tier.
/// Empty result = the ratchet passes.
#[must_use]
pub fn check_ratchet(current: &Lock, baseline: &Lock) -> Vec<String> {
    let cur_tested: BTreeSet<&String> = current.tested.iter().collect();
    let cur_strong: BTreeSet<&String> = current.strong.iter().collect();
    let mut out = Vec::new();
    for rule in &baseline.tested {
        if !cur_tested.contains(rule) {
            out.push(format!("{rule}: lost Tested coverage"));
        }
    }
    for rule in &baseline.strong {
        if !cur_strong.contains(rule) {
            out.push(format!("{rule}: lost Strong coverage"));
        }
    }
    out
}

const LOCK_FILE: &str = "cr-coverage.lock";

/// Entry point for `cargo xtask cite coverage [--check|--bless]`.
///
/// # Errors
/// Ratchet failure (`--check`), or any IO/parse error.
pub fn run(args: &[String]) -> anyhow::Result<()> {
    let root = crate::cite::repo_root();
    let excludes = load_source_excludes(&root)?;
    let scan = scan_tree(&root, &excludes);
    let current = lock_from_scan(&scan);

    if args.iter().any(|a| a == "--bless") {
        let path = root.join(LOCK_FILE);
        let json = serde_json::to_string_pretty(&current)?;
        std::fs::write(&path, format!("{json}\n"))
            .with_context(|| format!("writing {}", path.display()))?;
        println!(
            "blessed {LOCK_FILE}: {} tested, {} strong",
            current.tested.len(),
            current.strong.len()
        );
        return Ok(());
    }

    if args.iter().any(|a| a == "--check") {
        let baseline = read_lock(&root)?;
        let regressions = check_ratchet(&current, &baseline);
        if regressions.is_empty() {
            println!(
                "coverage ratchet OK: {} tested, {} strong (>= floor)",
                current.tested.len(),
                current.strong.len()
            );
            return Ok(());
        }
        for line in &regressions {
            eprintln!("coverage REGRESSION: {line}");
        }
        anyhow::bail!(
            "{} coverage regression(s); add citations back or justify + re-bless",
            regressions.len()
        );
    }

    // No gate flag: this command only reports coverage.
    report(&root, &scan)
}

fn read_lock(root: &Path) -> anyhow::Result<Lock> {
    let path = root.join(LOCK_FILE);
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {} — run `--bless` first", path.display()))?;
    Ok(serde_json::from_str(&text)?)
}

/// `sources.exclude` from `cite-config.json`, so coverage scans the same
/// universe as `cite check`.
fn load_source_excludes(root: &Path) -> anyhow::Result<Vec<String>> {
    let path = root.join("cite-config.json");
    let text =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let cfg: serde_json::Value = serde_json::from_str(&text)?;
    Ok(cfg["sources"]["exclude"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default())
}

/// The leading integer of a rule number (its "section"), e.g. `704.5f -> 704`.
fn section_of(rule: &str) -> Option<u32> {
    let digits: String = rule.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// The CR chapter's title, e.g. `1 -> "Game Concepts"`. `None` for any
/// chapter number the CR doesn't currently define a name for (still printed,
/// just without a title).
fn chapter_name(chapter: u32) -> Option<&'static str> {
    Some(match chapter {
        1 => "Game Concepts",
        2 => "Parts of a Card",
        3 => "Card Types",
        4 => "Zones",
        5 => "Turn Structure",
        6 => "Spells, Abilities, and Effects",
        7 => "Additional Rules",
        8 => "Multiplayer Rules",
        9 => "Casual Variants",
        _ => return None,
    })
}

/// Per-chapter tallies of in-scope rules for the report table.
#[derive(Default)]
struct ChapterCounts {
    extant: usize,
    strong: usize,
    tested: usize,
}

/// Expand `coverage.out_of_scope` to the concrete set of excluded rules.
///
/// Key shapes: bare integer `123` (whole section), `N..M` (section range),
/// or a rule/prefix `103.4` (the rule and its lettered subrules).
///
/// # Errors
/// A key that matches no extant rule, or an empty reason.
pub fn parse_out_of_scope(
    cfg: &serde_json::Value,
    extant: &BTreeSet<String>,
) -> anyhow::Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    let Some(map) = cfg["coverage"]["out_of_scope"].as_object() else {
        return Ok(out);
    };
    for (key, reason) in map {
        anyhow::ensure!(
            reason.as_str().is_some_and(|r| !r.trim().is_empty()),
            "coverage.out_of_scope[{key}] needs a non-empty reason"
        );
        let matched: Vec<&String> = if let Some((a, b)) = key.split_once("..") {
            let (lo, hi): (u32, u32) = (a.trim().parse()?, b.trim().parse()?);
            extant
                .iter()
                .filter(|r| section_of(r).is_some_and(|s| (lo..=hi).contains(&s)))
                .collect()
        } else if key.chars().all(|c| c.is_ascii_digit()) {
            let sec: u32 = key.parse()?;
            extant
                .iter()
                .filter(|r| section_of(r) == Some(sec))
                .collect()
        } else {
            // rule or prefix: exact match, plus lettered subrules `key + [a-z]+`
            extant
                .iter()
                .filter(|r| {
                    *r == key
                        || r.strip_prefix(key.as_str()).is_some_and(|suf| {
                            !suf.is_empty() && suf.chars().all(|c| c.is_ascii_lowercase())
                        })
                })
                .collect()
        };
        anyhow::ensure!(
            !matched.is_empty(),
            "coverage.out_of_scope[{key}] matches no extant rule"
        );
        out.extend(matched.into_iter().cloned());
    }
    Ok(out)
}

fn load_extant(root: &Path) -> anyhow::Result<BTreeSet<String>> {
    let path = root.join("data/rules/cr.json");
    let text = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "reading {} — the report needs the CR snapshot (local-only)",
            path.display()
        )
    })?;
    let map: BTreeMap<String, serde_json::Value> = serde_json::from_str(&text)?;
    Ok(map.into_keys().collect())
}

/// The human-readable coverage table: overall tiered summary against
/// in-scope (extant, non-out-of-scope) rules, plus a per-chapter breakdown.
///
/// # Errors
/// Missing/unparseable `data/rules/cr.json` or `cite-config.json`, or an
/// invalid `coverage.out_of_scope` entry.
fn report(root: &Path, scan: &BTreeMap<String, Tier>) -> anyhow::Result<()> {
    let extant = load_extant(root)?;
    let cfg_text = std::fs::read_to_string(root.join("cite-config.json"))?;
    let cfg: serde_json::Value = serde_json::from_str(&cfg_text)?;
    let oos = parse_out_of_scope(&cfg, &extant)?;

    let in_scope: Vec<&String> = extant.iter().filter(|r| !oos.contains(*r)).collect();
    let denom = in_scope.len().max(1);
    let tested = in_scope
        .iter()
        .filter(|r| scan.get(**r) == Some(&Tier::Tested))
        .count();
    let strong = in_scope
        .iter()
        .filter(|r| matches!(scan.get(**r), Some(Tier::Tested | Tier::Bound)))
        .count();

    println!(
        "CR coverage (in-scope leaves: {denom}, out-of-scope excluded: {})",
        oos.len()
    );
    println!(
        "  Strong (Tested ∪ Bound): {strong:>5}  {:>3}%",
        100 * strong / denom
    );
    println!(
        "  Tested (gold):           {tested:>5}  {:>3}%",
        100 * tested / denom
    );

    // Group in-scope rules by chapter (leading section / 100); BTreeMap keeps
    // chapters sorted ascending for free.
    let mut chapters: BTreeMap<u32, ChapterCounts> = BTreeMap::new();
    for rule in in_scope.iter().copied() {
        let Some(chapter) = section_of(rule).map(|section| section / 100) else {
            continue;
        };
        let counts = chapters.entry(chapter).or_default();
        counts.extant += 1;
        match scan.get(rule) {
            Some(Tier::Tested) => {
                counts.strong += 1;
                counts.tested += 1;
            }
            Some(Tier::Bound) => counts.strong += 1,
            _ => {}
        }
    }

    println!("Per chapter:");
    for (chapter, counts) in &chapters {
        let label = chapter_name(*chapter)
            .map_or_else(|| chapter.to_string(), |name| format!("{chapter} {name}"));
        let chapter_denom = counts.extant.max(1);
        let strong_ratio = format!("{}/{}", counts.strong, counts.extant);
        println!(
            "  {label:<32} {strong_ratio:>9} ({:>3}%)   {:>5} ({:>3}%)",
            100 * counts.strong / chapter_denom,
            counts.tested,
            100 * counts.tested / chapter_denom
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_single_and_list() {
        assert_eq!(extract_bracket_rules("foo [CR#714] bar"), vec!["714"]);
        assert_eq!(
            extract_bracket_rules("[CR#701.12a,701.12b]"),
            vec!["701.12a", "701.12b"]
        );
        assert_eq!(
            extract_bracket_rules("x [CR#601.2g,106.4] y"),
            vec!["601.2g", "106.4"]
        );
    }

    #[test]
    fn brackets_range_yields_endpoints() {
        assert_eq!(
            extract_bracket_rules("[CR#601.2a..601.2b]"),
            vec!["601.2a", "601.2b"]
        );
    }

    #[test]
    fn brackets_multiple_and_none() {
        assert_eq!(
            extract_bracket_rules("[CR#100.1] then [CR#200.2]"),
            vec!["100.1", "200.2"]
        );
        assert!(extract_bracket_rules("no citations here").is_empty());
    }

    #[test]
    fn brackets_ellipsis_placeholder_is_ignored() {
        assert!(extract_bracket_rules("cite as [CR#...] in prose").is_empty());
        assert!(extract_bracket_rules("cite as [CR#…] in prose").is_empty());
        // Narrow on purpose: typo shapes still yield their (bogus) token.
        assert_eq!(extract_bracket_rules("[CR#rule]"), vec!["rule"]);
        assert_eq!(
            extract_bracket_rules("[CR#…] then [CR#704.5g]"),
            vec!["704.5g"]
        );
    }

    #[test]
    fn attrs_extract_quoted_rules() {
        assert_eq!(
            extract_attr_rules(r#"#[cr("704.5f", "704.7")]"#),
            vec!["704.5f", "704.7"]
        );
        assert_eq!(extract_attr_rules(r#"#[cr( "305.2" )]"#), vec!["305.2"]);
        assert!(extract_attr_rules("#[test]").is_empty());
    }

    #[test]
    fn tier_by_path() {
        assert_eq!(
            tier_for_path("crates/deckmaste_engine/tests/activate.rs"),
            Tier::Tested
        );
        assert_eq!(tier_for_path("plugins/testing/cards/Foo.ron"), Tier::Tested);
        assert_eq!(
            tier_for_path("crates/deckmaste_engine/src/step/mod.rs"),
            Tier::Bound
        );
        assert_eq!(
            tier_for_path("plugins/builtin/rules/sba/toughness-zero.ron"),
            Tier::Bound
        );
        assert_eq!(tier_for_path("idris/src/Core.idr"), Tier::Bound);
        assert_eq!(tier_for_path("docs/tickets/done/foo.md"), Tier::Mentioned);
        // any markdown is prose regardless of dir
        assert_eq!(tier_for_path("README.md"), Tier::Mentioned);
    }

    #[test]
    fn tier_docs_nonmd_is_mentioned() {
        assert_eq!(
            tier_for_path("docs/superpowers/reviews/x/Attack.idr"),
            Tier::Mentioned
        );
        assert_eq!(tier_for_path("docs/tickets/done/foo.md"), Tier::Mentioned);
    }

    #[test]
    fn tier_ordering() {
        assert!(Tier::Tested > Tier::Bound);
        assert!(Tier::Bound > Tier::Mentioned);
    }

    #[test]
    fn scan_tree_strongest_tier_wins() {
        let dir = tempdir_with(&[
            ("crates/e/src/a.rs", "// [CR#100.1] bound here"),
            ("crates/e/tests/a.rs", "// [CR#100.1] also tested"),
            ("docs/x.md", "[CR#200.2] only mentioned"),
            ("crates/e/tests/b.rs", r#"#[cr("300.3")]"#),
        ]);
        let scan = scan_tree(dir.path(), &[]);
        assert_eq!(scan.get("100.1"), Some(&Tier::Tested)); // test beats src
        assert_eq!(scan.get("200.2"), Some(&Tier::Mentioned));
        assert_eq!(scan.get("300.3"), Some(&Tier::Tested)); // attribute counts
    }

    #[test]
    fn scan_tree_skips_local_workspaces() {
        let dir = tempdir_with(&[
            ("crates/e/src/a.rs", "// [CR#100.1] tracked source"),
            (
                ".workspaces/fake-test-audit/crates/e/src/lib.rs",
                "// [CR#701.37] ignored workspace source",
            ),
        ]);
        let scan = scan_tree(dir.path(), &[]);
        assert_eq!(scan.get("100.1"), Some(&Tier::Bound));
        assert!(!scan.contains_key("701.37"));
    }

    #[test]
    fn lock_from_scan_partitions_tiers() {
        let mut scan = BTreeMap::new();
        scan.insert("100.1".to_string(), Tier::Tested);
        scan.insert("200.2".to_string(), Tier::Bound);
        scan.insert("300.3".to_string(), Tier::Mentioned);
        let lock = lock_from_scan(&scan);
        assert_eq!(lock.tested, vec!["100.1"]);
        assert_eq!(lock.strong, vec!["100.1", "200.2"]); // sorted, Mentioned excluded
    }

    #[test]
    fn ratchet_passes_on_superset() {
        let base = Lock {
            tested: vec!["100.1".into()],
            strong: vec!["100.1".into(), "200.2".into()],
        };
        let cur = Lock {
            tested: vec!["100.1".into(), "400.4".into()],
            strong: vec!["100.1".into(), "200.2".into(), "400.4".into()],
        };
        assert!(check_ratchet(&cur, &base).is_empty());
    }

    #[test]
    fn ratchet_fails_and_names_regression() {
        let base = Lock {
            tested: vec!["100.1".into()],
            strong: vec!["100.1".into(), "200.2".into()],
        };
        let cur = Lock {
            tested: vec![],
            strong: vec!["100.1".into()],
        };
        let regressions = check_ratchet(&cur, &base);
        assert_eq!(regressions.len(), 2); // 100.1 lost Tested, 200.2 lost Strong
        assert!(
            regressions
                .iter()
                .any(|r| r.contains("100.1") && r.contains("Tested"))
        );
        assert!(
            regressions
                .iter()
                .any(|r| r.contains("200.2") && r.contains("Strong"))
        );
    }

    #[test]
    fn ratchet_end_to_end_over_temp_tree() {
        let dir = tempdir_with(&[
            ("crates/e/tests/a.rs", "// [CR#100.1]"),
            ("crates/e/src/b.rs", "// [CR#200.2]"),
        ]);
        let scan = scan_tree(dir.path(), &[]);
        let lock = lock_from_scan(&scan);
        assert_eq!(lock.tested, vec!["100.1"]);
        assert_eq!(lock.strong, vec!["100.1", "200.2"]);

        // Remove the test citation -> ratchet must flag 100.1 losing Tested.
        std::fs::write(dir.path().join("crates/e/tests/a.rs"), "// no cite").unwrap();
        let scan2 = scan_tree(dir.path(), &[]);
        let regressions = check_ratchet(&lock_from_scan(&scan2), &lock);
        assert!(
            regressions
                .iter()
                .any(|r| r.contains("100.1") && r.contains("Tested"))
        );
    }

    fn extant_set(rules: &[&str]) -> BTreeSet<String> {
        rules.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn out_of_scope_expands_three_shapes() {
        let extant = extant_set(&[
            "100.1", "123.1", "123.1a", "800.1", "999.9", "103.4", "103.4a", "500.1",
        ]);
        let cfg: serde_json::Value = serde_json::from_str(
            r#"{ "coverage": { "out_of_scope": {
                "123": "stickers",
                "800..999": "multiplayer",
                "103.4": "mulligans"
            } } }"#,
        )
        .unwrap();
        let oos = parse_out_of_scope(&cfg, &extant).unwrap();
        assert!(oos.contains("123.1") && oos.contains("123.1a")); // section
        assert!(oos.contains("800.1") && oos.contains("999.9")); // section range
        assert!(oos.contains("103.4") && oos.contains("103.4a")); // rule + subrule
        assert!(!oos.contains("100.1") && !oos.contains("500.1")); // untouched
    }

    #[test]
    fn out_of_scope_rejects_unknown_rule() {
        let extant = extant_set(&["100.1"]);
        let cfg: serde_json::Value =
            serde_json::from_str(r#"{ "coverage": { "out_of_scope": { "404": "nope" } } }"#)
                .unwrap();
        let err = parse_out_of_scope(&cfg, &extant).unwrap_err();
        assert!(err.to_string().contains("matches no extant rule"));
    }

    #[test]
    fn out_of_scope_rejects_empty_reason() {
        let extant = extant_set(&["100.1"]);
        let cfg: serde_json::Value =
            serde_json::from_str(r#"{ "coverage": { "out_of_scope": { "100": "" } } }"#).unwrap();
        let err = parse_out_of_scope(&cfg, &extant).unwrap_err();
        assert!(err.to_string().contains("non-empty reason"));
    }

    fn tempdir_with(files: &[(&str, &str)]) -> TempDir {
        let dir = TempDir::new();
        for (rel, body) in files {
            let path = dir.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, body).unwrap();
        }
        dir
    }

    /// Minimal self-cleaning temp dir (avoids adding the `tempfile` crate).
    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn new() -> Self {
            // process id + a monotonically increasing counter keeps this unique
            // without Date/rand (both unavailable / undesirable here).
            use std::sync::atomic::AtomicU32;
            use std::sync::atomic::Ordering;
            static N: AtomicU32 = AtomicU32::new(0);
            let base = std::env::temp_dir().join(format!(
                "xtask-cov-{}-{}",
                std::process::id(),
                N.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&base).unwrap();
            Self(base)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
