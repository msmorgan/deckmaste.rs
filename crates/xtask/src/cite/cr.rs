//! Parse the local `cr.txt` snapshot into an ordered rule set.

use std::collections::HashMap;
use std::hash::Hasher;

use anyhow::{Context, bail};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    Section,
    Leaf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub number: String,
    pub kind: RuleKind,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Rules {
    ordered: Vec<Rule>,
    index: HashMap<String, usize>,
    pub effective: String,
}

impl Rules {
    pub fn parse(src: &str) -> anyhow::Result<Rules> {
        // cr.txt writes numbered subrules with a trailing dot ("305.6. text")
        // and lettered ones without ("305.6a text"); accept an optional dot.
        let leaf_re = Regex::new(r"^(\d{1,3}\.\d+[a-z]*)\.? (.+)$").unwrap();
        let head_re = Regex::new(r"^(\d{1,3})\. (.+)$").unwrap();

        let lines: Vec<&str> = src.lines().collect();

        let effective = lines
            .iter()
            .find_map(|l| l.strip_prefix("These rules are effective as of "))
            .map(|d| parse_effective_date(d.trim().trim_end_matches('.')))
            .context("cr.txt: no effective-date line")??;

        let leading_section = |s: &str| -> Option<String> {
            let num = s.split_whitespace().next()?;
            let head = num.split('.').next()?;
            head.chars()
                .all(|c| c.is_ascii_digit())
                .then(|| head.to_string())
        };
        let next_numbered = |idx: usize| -> Option<&str> {
            lines[idx + 1..]
                .iter()
                .copied()
                .find(|l| l.starts_with(|c: char| c.is_ascii_digit()))
        };

        let mut ordered = Vec::new();
        let mut index = HashMap::new();
        let mut seen_leaf = false;

        for (i, line) in lines.iter().enumerate() {
            if seen_leaf && line.trim() == "Glossary" {
                break;
            }
            if let Some(c) = leaf_re.captures(line) {
                seen_leaf = true;
                push(
                    &mut ordered,
                    &mut index,
                    c[1].to_string(),
                    RuleKind::Leaf,
                    c[2].to_string(),
                )?;
            } else if let Some(c) = head_re.captures(line) {
                let num = c[1].to_string();
                let is_body_header = next_numbered(i)
                    .and_then(leading_section)
                    .is_some_and(|next| next == num);
                if is_body_header {
                    push(
                        &mut ordered,
                        &mut index,
                        num,
                        RuleKind::Section,
                        c[2].to_string(),
                    )?;
                }
            }
        }

        if ordered.is_empty() {
            bail!("cr.txt: parsed zero rules");
        }
        Ok(Rules {
            ordered,
            index,
            effective,
        })
    }

    pub fn get(&self, number: &str) -> Option<&Rule> {
        self.index.get(number).map(|&i| &self.ordered[i])
    }

    /// Inclusive range over leaf rules in document order. Both endpoints must
    /// be leaves; section-header lines are not members.
    pub fn expand_range(&self, start: &str, end: &str) -> anyhow::Result<Vec<String>> {
        let s = *self
            .index
            .get(start)
            .with_context(|| format!("range start {start} not found"))?;
        let e = *self
            .index
            .get(end)
            .with_context(|| format!("range end {end} not found"))?;
        if s > e {
            bail!("range {start}..{end} is reversed in document order");
        }
        Ok(self.ordered[s..=e]
            .iter()
            .filter(|r| r.kind == RuleKind::Leaf)
            .map(|r| r.number.clone())
            .collect())
    }

    /// Checksum of a single rule's text (None if the rule is absent).
    pub fn checksum(&self, number: &str) -> Option<String> {
        self.get(number).map(|r| checksum_text(&r.text))
    }
}

fn push(
    ordered: &mut Vec<Rule>,
    index: &mut HashMap<String, usize>,
    number: String,
    kind: RuleKind,
    text: String,
) -> anyhow::Result<()> {
    if index.contains_key(&number) {
        bail!("cr.txt: duplicate rule {number}");
    }
    index.insert(number.clone(), ordered.len());
    ordered.push(Rule { number, kind, text });
    Ok(())
}

/// FNV-1a/64 of the normalized text, as 16 lowercase hex digits. Normalization
/// collapses internal whitespace runs to a single space and trims the ends, so
/// the value is stable across machines and Rust versions.
pub fn checksum_text(text: &str) -> String {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut hasher = fnv::FnvHasher::default();
    hasher.write(normalized.as_bytes());
    format!("{:016x}", hasher.finish())
}

/// "April 17, 2026" -> "2026-04-17".
fn parse_effective_date(s: &str) -> anyhow::Result<String> {
    let (month_name, rest) = s.split_once(' ').context("bad date")?;
    let (day, year) = rest.split_once(", ").context("bad date")?;
    let month = match month_name {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        other => bail!("unknown month {other:?}"),
    };
    let day: u32 = day.trim().parse().context("bad day")?;
    let year: u32 = year.trim().parse().context("bad year")?;
    Ok(format!("{year:04}-{month:02}-{day:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../tests/fixtures/cr_fixture.txt");

    #[test]
    fn parses_date_sections_and_leaves() {
        let rules = Rules::parse(FIXTURE).unwrap();

        assert_eq!(rules.effective, "2026-04-17");

        // numbered subrule: trailing-dot form ("100.1. text")
        assert_eq!(rules.get("100.1").unwrap().kind, RuleKind::Leaf);
        assert_eq!(rules.get("100.1").unwrap().text, "Placeholder first rule.");
        // lettered subrule: no trailing dot ("100.1a text")
        assert_eq!(rules.get("100.1a").unwrap().kind, RuleKind::Leaf);
        assert_eq!(rules.get("100.1a").unwrap().text, "Placeholder subrule a.");
        assert_eq!(
            rules.get("702.158b").unwrap().text,
            "Placeholder sector text."
        );

        assert_eq!(rules.get("702").unwrap().kind, RuleKind::Section);
        assert_eq!(rules.get("702").unwrap().text, "Keyword Abilities");

        assert!(rules.get("1").is_none());
        assert!(rules.get("placeholderterm").is_none());
    }

    #[test]
    fn expands_range_in_document_order_skipping_l() {
        let rules = Rules::parse(FIXTURE).unwrap();
        assert_eq!(
            rules.expand_range("704.5k", "704.5n").unwrap(),
            vec!["704.5k", "704.5m", "704.5n"], // l/o never appear in cr.txt
        );
    }

    #[test]
    fn range_rejects_reversed_or_missing() {
        let rules = Rules::parse(FIXTURE).unwrap();
        assert!(rules.expand_range("704.5n", "704.5k").is_err()); // reversed
        assert!(rules.expand_range("704.5k", "999.9z").is_err()); // missing endpoint
    }

    #[test]
    fn checksum_is_stable_and_normalizes_whitespace() {
        assert_eq!(checksum_text("a   b"), checksum_text("a b"));
        assert_eq!(checksum_text("  a b  "), checksum_text("a b"));
        assert_eq!(
            checksum_text("Placeholder sector text."),
            "ab113946c53cef93"
        );
    }
}
