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
/// **What the field actually records.** `ParseFacts::ties` says the packed
/// forest retained more than one undominated, cost-equal alternative at a
/// span. The selected candidate is included. Selection applies named cost
/// dimensions, declared construction dominance, and stable production
/// identity; it is deterministic, not a coin flip.
///
/// Historical measurements used numeric rule IDs and counted every unique
/// minimum as a tie, so those clusters are intentionally not carried forward.
/// Re-measure with the current stable construction identities before citing
/// any precise corpus rate.
///
/// **Corroborating evidence this is expected behavior, not a bug.**
/// `deckmaste_english`'s own unit test,
/// `equal_cost_uses_stable_production_identity_and_preserves_incomparable_ties`
/// in `src/forest.rs`, explicitly requires incomparable equal-cost readings to
/// survive while still selecting one stable construction.
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
            signature: tie.construction.map_or_else(
                || "construction ?".to_string(),
                |id| format!("construction {id}"),
            ),
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
    use deckmaste_english::ConstructionId;
    use deckmaste_english::Span;

    use super::*;

    fn test_construction() -> ConstructionId {
        deckmaste_english::construction_families()[0].id()
    }

    #[test]
    fn a_parse_without_ties_is_clean() {
        assert!(no_tie(&ParseFacts::default()).is_empty());
    }

    /// Not named `..._is_a_violation`, unlike its `forest_growth` neighbors
    /// below: per `no_tie`'s doc comment, a tie is reported for information,
    /// never a violation.
    #[test]
    fn a_tied_selection_still_produces_one_finding() {
        let construction = test_construction();
        let tied = ParseFacts {
            ties: vec![TieFact {
                span: Span::new(0, 4),
                construction: Some(construction),
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
        assert_eq!(
            found[0].signature,
            format!("construction {construction}"),
            "the stable construction is the cluster key",
        );
        assert!(found[0].detail.contains("bytes 0..4"));
    }

    #[test]
    fn ties_in_the_same_construction_share_a_signature_across_different_bytes() {
        let construction = test_construction();
        let facts = |span| ParseFacts {
            ties: vec![TieFact {
                span,
                construction: Some(construction),
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
    fn a_tie_without_a_construction_signs_as_unknown() {
        let tied = ParseFacts {
            ties: vec![TieFact {
                span: Span::new(0, 4),
                construction: None,
                alternatives: 2,
            }],
            ..ParseFacts::default()
        };
        let found = no_tie(&tied);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0].signature, "construction ?",
            "a tie with no construction must still get a stable cluster key"
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
