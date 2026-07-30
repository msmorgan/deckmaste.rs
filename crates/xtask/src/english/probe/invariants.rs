//! Structural laws over [`ParseFacts`].
//!
//! `no_tie` is measured for information only, never gated — see its own doc
//! comment: a measurement overturned the earlier assumption that a non-empty
//! tie list meant the parser guessed. `forest_growth` is the one law here
//! whose findings are gating, checked against calibrated ceilings.

use super::facts::ParseFacts;
use super::facts::TieFact;

/// One law's report about one composition.
///
/// `signature` is the cluster key and is set here, at construction — clustering
/// never parses `detail`, which exists only to be read by a human.
///
/// Named `Finding` rather than `Violation`: `no_tie`'s findings are
/// deliberately not violations — see its doc comment below.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Finding {
    pub(super) law: &'static str,
    pub(super) signature: String,
    pub(super) detail: String,
}

/// Ceilings derived from the printed corpus by [`super::calibrate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Thresholds {
    pub(super) max_alternatives: usize,
    pub(super) constituent_nodes: usize,
}

/// Invariant 1, demoted: cost ties are measured, never gated.
///
/// A previous version of this doc comment claimed a non-empty tie list meant
/// "two derivations scored equal and one was returned arbitrarily," and
/// treated that as proof-grade evidence of a defect. That claim was wrong —
/// caught by measurement, not by code review. Do not restore it, or route
/// this function's findings back into a gating set, without re-deriving the
/// reasoning below.
///
/// **What the field actually records.** `ParseFacts::ties` (built from
/// `ParseSelection::tied_alternatives`) says the packed forest had more than
/// one alternative in cost-equal contention at a span. Which alternative the
/// parser actually picked from that contention is not arbitrary:
/// `Forest::best_root_matching`'s own doc comment says roots are ranked "by
/// the ordinary cost and stable-node tiebreak" — a documented, deterministic
/// resolution, not a coin flip. So a non-empty tie list means "more than one
/// equally-cheap derivation existed here," not "the parser guessed."
///
/// **The measurement.** `cargo xtask english probe --printed`, run over all
/// 31,685 supported printed faces, found that 29,490 of them (93.07%) tie on
/// cost somewhere in their parse — and 28,899 of those tie specifically under
/// rule 176. A law that fires on 93% of ordinary, unmodified printed cards
/// cannot discriminate a synthesized composition from an ordinary one; worse,
/// a gate that fires almost always trains reviewers to ignore it.
///
/// **Those counts are parser-version-dependent. Re-measure rather than citing
/// them as current.** They were taken before `english: preserve mixed
/// coordination grouping` and `english: document minimum-cost alternatives`
/// landed; on the grammar immediately after those, the same run reports 29,499
/// (93.10%) and 28,911 under rule 176. `data/derived/cards.jsonl` was byte
/// identical across both runs, so the movement is grammar drift, not corpus
/// drift. The conclusion here is insensitive to the exact figures — the order
/// of magnitude is the point — but a precise number carrying no revision
/// invites exactly the misplaced confidence this comment exists to prevent.
///
/// **Corroborating evidence this is expected behavior, not a bug.**
/// `deckmaste_english`'s own unit test,
/// `provenance_identifies_selected_rules_without_copying_source` in
/// `src/parse.rs`, asserts `!selection.tied_alternatives().is_empty()` for
/// the input `"Draw a card."` — the simplest possible ability text. The
/// crate's own test suite requires a tie to exist on trivial input; treating
/// a tie's mere existence as pathological is incompatible with that.
///
/// **Therefore:** this function still runs, but its findings are reported
/// for information only — the sweep's `baseline_relative` list, never
/// `gating` — and must never drive `--deny` or any other pass/fail decision.
pub(super) fn no_tie(facts: &ParseFacts) -> Vec<Finding> {
    facts
        .ties
        .iter()
        .map(|tie: &TieFact| Finding {
            law: "no-tie",
            signature: tie
                .rule
                .map_or_else(|| "rule ?".to_string(), |rule| format!("rule {rule}")),
            detail: format!(
                "{} derivations tied at bytes {}..{}",
                tie.alternatives, tie.span.start, tie.span.end,
            ),
        })
        .collect()
}

/// Invariant 5 — ambiguity must stay inside calibrated headroom.
pub(super) fn forest_growth(facts: &ParseFacts, limits: &Thresholds) -> Vec<Finding> {
    let mut found = Vec::new();
    if facts.max_alternatives > limits.max_alternatives {
        found.push(Finding {
            law: "forest-growth",
            signature: "max_alternatives".to_string(),
            detail: format!(
                "max_alternatives {} exceeds calibrated ceiling {}",
                facts.max_alternatives, limits.max_alternatives
            ),
        });
    }
    if facts.constituent_nodes > limits.constituent_nodes {
        found.push(Finding {
            law: "forest-growth",
            signature: "constituent_nodes".to_string(),
            detail: format!(
                "constituent_nodes {} exceeds calibrated ceiling {}",
                facts.constituent_nodes, limits.constituent_nodes
            ),
        });
    }
    found
}

#[cfg(test)]
mod tests {
    use deckmaste_english::Span;

    use super::*;

    #[test]
    fn a_parse_without_ties_is_clean() {
        assert!(no_tie(&ParseFacts::default()).is_empty());
    }

    /// Not named `..._is_a_violation`, unlike its `forest_growth` neighbors
    /// below: per `no_tie`'s doc comment, a tie is reported for information,
    /// never a violation.
    #[test]
    fn a_tied_selection_still_produces_one_finding() {
        let tied = ParseFacts {
            ties: vec![TieFact {
                span: Span::new(0, 4),
                rule: Some(7),
                alternatives: 2,
            }],
            ..ParseFacts::default()
        };
        let found = no_tie(&tied);
        assert_eq!(
            found.len(),
            1,
            "one tie at one span is one Finding, reported for information — \
             see the doc comment above for why this is not a defect signal"
        );
        assert_eq!(found[0].signature, "rule 7", "the rule is the cluster key");
        assert!(found[0].detail.contains("bytes 0..4"));
    }

    #[test]
    fn ties_in_the_same_rule_share_a_signature_across_different_bytes() {
        let facts = |span| ParseFacts {
            ties: vec![TieFact {
                span,
                rule: Some(7),
                alternatives: 2,
            }],
            ..ParseFacts::default()
        };
        assert_eq!(
            no_tie(&facts(Span::new(0, 4)))[0].signature,
            no_tie(&facts(Span::new(9, 13)))[0].signature,
            "byte offsets must not fragment a cluster"
        );
    }

    #[test]
    fn a_tie_without_a_rule_id_signs_as_rule_unknown() {
        let tied = ParseFacts {
            ties: vec![TieFact {
                span: Span::new(0, 4),
                rule: None,
                alternatives: 2,
            }],
            ..ParseFacts::default()
        };
        let found = no_tie(&tied);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0].signature, "rule ?",
            "a tie with no rule id must still get a stable cluster key"
        );
    }

    #[test]
    fn growth_within_limits_is_clean() {
        let facts = ParseFacts {
            max_alternatives: 3,
            constituent_nodes: 40,
            ..ParseFacts::default()
        };
        let limits = Thresholds {
            max_alternatives: 8,
            constituent_nodes: 500,
        };
        assert!(forest_growth(&facts, &limits).is_empty());
    }

    #[test]
    fn growth_past_the_calibrated_limit_is_a_violation() {
        let facts = ParseFacts {
            max_alternatives: 99,
            constituent_nodes: 40,
            ..ParseFacts::default()
        };
        let limits = Thresholds {
            max_alternatives: 8,
            constituent_nodes: 500,
        };
        let found = forest_growth(&facts, &limits);
        assert_eq!(found.len(), 1);
        assert!(found[0].detail.contains("99"));
    }

    #[test]
    fn growth_past_the_constituent_node_ceiling_is_a_violation() {
        let facts = ParseFacts {
            max_alternatives: 3,
            constituent_nodes: 999,
            ..ParseFacts::default()
        };
        let limits = Thresholds {
            max_alternatives: 8,
            constituent_nodes: 500,
        };
        let found = forest_growth(&facts, &limits);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].signature, "constituent_nodes");
        assert!(found[0].detail.contains("999"));
    }
}
