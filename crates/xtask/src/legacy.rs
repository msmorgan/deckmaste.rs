//! Wide-net scanner: find citation-looking strings that are not canonical
//! `[CR#…]`.

use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

use crate::citations::Site;

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

pub fn scan_noncompliant(file: &Path, content: &str) -> Vec<Site> {
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
        Some(o) => line[o..].find(']').map(|c| o + c >= at).unwrap_or(false),
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
}
