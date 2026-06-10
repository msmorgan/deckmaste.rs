//! Wide-net scanner: find citation-looking strings that are not canonical
//! `[CR#…]`.

use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::cite::citations::Site;

/// Files whose rule-number-looking strings are data, format-doc examples, or
/// test fixtures rather than missing `[CR#…]` citations: the academyruins
/// JSON data-model (rule numbers as `format` examples), and the keyword /
/// ability-word stub generators (a rule-number data const plus serialization
/// fixtures). The wide-net scan skips them so it doesn't demand bracket form
/// for non-citations. Their *real* citations (e.g. `ability_word_todos`'
/// `[CR#207.2c]`) are unaffected — only this scan exempts them; the staleness
/// check still covers them.
const NONCOMPLIANT_EXEMPT: &[&str] = &[
    "crates/deckmaste_migrations/src/data/academyruins.rs",
    "crates/deckmaste_migrations/src/stubs/keyword_todos.rs",
    "crates/deckmaste_migrations/src/stubs/ability_word_todos.rs",
];

/// True if `file` is exempt from the wide-net noncompliance scan (see
/// [`NONCOMPLIANT_EXEMPT`]).
fn is_exempt(file: &Path) -> bool {
    NONCOMPLIANT_EXEMPT
        .iter()
        .any(|suffix| file.ends_with(suffix))
}

/// Wide-net patterns for citation-looking strings. Deliberately over-matches;
/// the human filters during migration. Skips anything already inside `[CR#…]`.
fn patterns() -> &'static [Regex] {
    static P: OnceLock<Vec<Regex>> = OnceLock::new();
    P.get_or_init(|| {
        vec![
            Regex::new(r"CR ?\d{1,3}(\.\d+[a-z]*)?").unwrap(),
            Regex::new(r"\brule \d{1,3}(\.\d+[a-z]*)?").unwrap(),
            Regex::new(r"\b\d{3}\.\d+[a-z]*\b").unwrap(),
        ]
    })
}

#[must_use]
pub fn scan_noncompliant(file: &Path, content: &str) -> Vec<Site> {
    if is_exempt(file) {
        return Vec::new();
    }
    let mut sites = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (i, line) in content.lines().enumerate() {
        for re in patterns() {
            for m in re.find_iter(line) {
                // Skip matches inside a canonical `[CR#…]` token, and de-dup
                // overlapping matches at the same position.
                if inside_citation(line, m.start()) {
                    continue;
                }
                if !seen.insert((i, m.start())) {
                    continue;
                }
                sites.push(Site {
                    file: file.to_path_buf(),
                    line: i + 1,
                    raw: m.as_str().to_string(),
                    context: line.trim().to_string(),
                });
            }
        }
    }
    sites
}

/// True if byte offset `at` falls within a `[CR#…]` span on this line.
fn inside_citation(line: &str, at: usize) -> bool {
    match line[..at].rfind("[CR#") {
        Some(o) => line[o..].find(']').is_some_and(|c| o + c >= at),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn flags_legacy_forms_but_not_canonical() {
        let content = concat!(
            "// CR 107.3 and CR 113.3b, 602\n",
            "// see rule 602 and bare 509.1h here\n",
            "// canonical [CR#702.158b] is fine\n",
        );
        let hits: Vec<_> = scan_noncompliant(Path::new("a.rs"), content)
            .into_iter()
            .map(|s| (s.line, s.raw))
            .collect();
        assert!(hits.iter().any(|(l, r)| *l == 1 && r == "CR 107.3"));
        assert!(hits.iter().any(|(_, r)| r == "509.1h"));
        // line 3 is canonical -> not flagged
        assert!(hits.iter().all(|(l, _)| *l != 3));
    }

    #[test]
    fn exempt_files_are_not_scanned() {
        let content = "// bare 509.1h and CR 107.3 here\n";
        // A normal file flags the bare forms.
        assert!(!scan_noncompliant(Path::new("src/lib.rs"), content).is_empty());
        // An exempt data/fixture file is skipped entirely, even with the same
        // citation-looking strings.
        let exempt = Path::new("/repo/crates/deckmaste_migrations/src/data/academyruins.rs");
        assert!(scan_noncompliant(exempt, content).is_empty());
    }
}
