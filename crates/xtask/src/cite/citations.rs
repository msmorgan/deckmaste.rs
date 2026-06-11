//! Extract and parse `[CR#…]` citations from source files.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use anyhow::bail;

use crate::cite::cr::Rules;

const OPEN: &str = "[CR#";

/// Find every `[CR#…]` token in `content`. `raw` is the inside text; `context`
/// is the trimmed source line. Malformed inner text is left for validation —
/// this only splits on the delimiters.
#[must_use]
pub fn scan_text(file: &Path, content: &str) -> Vec<Site> {
    let mut sites = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let mut rest = line;
        let mut consumed = 0usize;
        while let Some(rel) = rest.find(OPEN) {
            let after = consumed + rel + OPEN.len();
            if let Some(close_rel) = content_close(&line[after..]) {
                sites.push(Site {
                    file: file.to_path_buf(),
                    line: i + 1,
                    raw: line[after..after + close_rel].to_string(),
                    context: line.trim().to_string(),
                });
                consumed = after + close_rel + 1;
            } else {
                consumed = after;
            }
            rest = &line[consumed..];
        }
    }
    sites
}

/// Offset of the closing `]` within `s` (which starts just after `[CR#`).
fn content_close(s: &str) -> Option<usize> { s.find(']') }

/// Expand a citation's refs into concrete rule numbers, in order.
pub fn members(refs: &[Ref], rules: &Rules) -> anyhow::Result<Vec<String>> {
    let mut out = Vec::new();
    for r in refs {
        match r {
            Ref::One(n) => out.push(n.clone()),
            Ref::Range(a, b) => out.extend(rules.expand_range(a, b)?),
        }
    }
    Ok(out)
}

/// A syntactically valid rule number: `\d{1,3}` optionally `.` `\d+` `[a-z]*`.
fn is_rule_number(s: &str) -> bool {
    let (section, sub) = match s.split_once('.') {
        Some((sec, sub)) => (sec, Some(sub)),
        None => (s, None),
    };
    let section_ok =
        (1..=3).contains(&section.len()) && section.bytes().all(|b| b.is_ascii_digit());
    let sub_ok = match sub {
        None => true,
        Some(sub) => {
            let digits = sub.trim_end_matches(|c: char| c.is_ascii_lowercase());
            !digits.is_empty()
                && digits.bytes().all(|b| b.is_ascii_digit())
                && sub[digits.len()..].chars().all(|c| c.is_ascii_lowercase())
        }
    };
    section_ok && sub_ok
}

/// Parse the inside of `[CR#…]` (the part between `CR#` and `]`) into refs.
pub fn parse_refs(inner: &str) -> anyhow::Result<Vec<Ref>> {
    if inner.is_empty() {
        bail!("empty citation");
    }
    inner
        .split(',')
        .map(|part| {
            if let Some((a, b)) = part.split_once("..") {
                if !is_rule_number(a) || !is_rule_number(b) {
                    bail!("bad range {part:?}");
                }
                Ok(Ref::Range(a.to_string(), b.to_string()))
            } else if is_rule_number(part) {
                Ok(Ref::One(part.to_string()))
            } else {
                bail!("bad rule number {part:?}");
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("parsing citation refs")
}

/// Strip an optional `[CR#…]` / `CR#…` wrapper down to the inner ref text.
/// The enclosing brackets are removed only as a matched pair, so a lone `[` or
/// `]` survives and is later rejected by [`parse_refs`].
#[must_use]
pub fn strip_citation_wrapper(arg: &str) -> &str {
    let s = arg.trim();
    let s = s
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .unwrap_or(s);
    let s = s.strip_prefix("CR#").unwrap_or(s);
    s.trim()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ref {
    One(String),
    Range(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub file: PathBuf,
    pub line: usize,
    pub raw: String,
    pub context: String,
}

/// The citation tool's own crate. Its only `[CR#…]` occurrences are
/// format-doc examples and test fixtures (e.g. the deliberately-fake
/// `[CR#999.9z]`), never real rules-engine citations — so the scanner skips it
/// to avoid flagging its own test data as stale.
const SELF_CRATE: &str = "crates/xtask/";

/// Tracked `.rs`/`.md`/`.ron` paths (absolute), via `jj file list` with a
/// read-only `git ls-files` fallback. Excludes gitignored trees (e.g.
/// wizards/) and the tool's own crate ([`SELF_CRATE`]).
pub fn tracked_files(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let listing = run_lister(root, &["file", "list"], "jj")
        .or_else(|_| run_lister(root, &["ls-files"], "git"))?;
    Ok(listing
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter(|l| !l.starts_with(SELF_CRATE))
        .filter(|l| {
            Path::new(l)
                .extension()
                .is_some_and(|e| matches!(e.to_str(), Some("rs" | "md" | "ron")))
        })
        .map(|l| root.join(l))
        .collect())
}

fn run_lister(root: &Path, args: &[&str], bin: &str) -> anyhow::Result<String> {
    let out = Command::new(bin).args(args).current_dir(root).output()?;
    if !out.status.success() {
        anyhow::bail!("{bin} {args:?} failed");
    }
    Ok(String::from_utf8(out.stdout)?)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn scans_sites_with_line_and_context() {
        let content =
            "fn x() {} // see [CR#702.158b] for sectors\nplain line\n/// [CR#100.1a,101.2e]\n";
        let sites = scan_text(Path::new("a.rs"), content);
        assert_eq!(sites.len(), 2);
        assert_eq!(sites[0].line, 1);
        assert_eq!(sites[0].raw, "702.158b");
        assert_eq!(
            sites[0].context,
            "fn x() {} // see [CR#702.158b] for sectors"
        );
        assert_eq!(sites[1].line, 3);
        assert_eq!(sites[1].raw, "100.1a,101.2e");
    }

    #[test]
    fn members_expands_ranges_and_lists() {
        let rules =
            crate::cite::cr::Rules::parse(include_str!("../../tests/fixtures/cr_fixture.txt"))
                .unwrap();
        let refs = parse_refs("704.5k..704.5n,100.1a").unwrap();
        assert_eq!(
            members(&refs, &rules).unwrap(),
            vec!["704.5k", "704.5m", "704.5n", "100.1a"]
        );
    }

    #[test]
    fn strip_wrapper_accepts_all_forms() {
        assert_eq!(strip_citation_wrapper("[CR#704.5g]"), "704.5g");
        assert_eq!(strip_citation_wrapper("CR#704.5g"), "704.5g");
        assert_eq!(strip_citation_wrapper("704.5g"), "704.5g");
        assert_eq!(
            strip_citation_wrapper("  [CR#601.2g,106.4]  "),
            "601.2g,106.4"
        );
        assert_eq!(
            strip_citation_wrapper("[CR#704.5k..704.5n]"),
            "704.5k..704.5n"
        );
    }

    #[test]
    fn strip_wrapper_leaves_unmatched_brackets() {
        // A lone bracket isn't stripped, so parse_refs later rejects it.
        assert_eq!(strip_citation_wrapper("[CR#704.5g"), "[CR#704.5g");
        assert_eq!(strip_citation_wrapper("704.5g]"), "704.5g]");
        assert!(parse_refs(strip_citation_wrapper("[CR#704.5g")).is_err());
        assert!(parse_refs(strip_citation_wrapper("704.5g]")).is_err());
    }

    #[test]
    fn parses_singles_lists_ranges() {
        assert_eq!(
            parse_refs("702.158b").unwrap(),
            vec![Ref::One("702.158b".into())]
        );
        assert_eq!(
            parse_refs("100.1a,101.2e").unwrap(),
            vec![Ref::One("100.1a".into()), Ref::One("101.2e".into())],
        );
        assert_eq!(
            parse_refs("704.5k..704.5n").unwrap(),
            vec![Ref::Range("704.5k".into(), "704.5n".into())],
        );
        assert_eq!(
            parse_refs("113.3b,602").unwrap(),
            vec![Ref::One("113.3b".into()), Ref::One("602".into())],
        );
    }

    #[test]
    fn rejects_malformed() {
        assert!(parse_refs("").is_err());
        assert!(parse_refs("702.158b,").is_err()); // trailing empty
        assert!(parse_refs("xyz").is_err()); // not a rule number
        assert!(parse_refs("1.1a..1.1b..1.1c").is_err()); // double range
        assert!(parse_refs("702.158 b").is_err()); // internal whitespace
    }

    #[test]
    fn tracked_files_scans_others_and_skips_own_crate() {
        let files = tracked_files(&crate::cite::repo_root()).unwrap();
        // finds real-citation sources elsewhere in the workspace
        assert!(
            files
                .iter()
                .any(|p| p.ends_with("crates/deckmaste_core/src/action.rs"))
        );
        // skips the tool's own crate (only format-doc/test [CR#…] fixtures there)
        assert!(
            files
                .iter()
                .all(|p| !p.starts_with(crate::cite::repo_root().join("crates/xtask")))
        );
        assert!(files.iter().all(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("rs" | "md" | "ron")
            )
        }));
    }
}
