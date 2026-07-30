//! Invariant-5 ceilings, derived from the printed corpus rather than guessed.

use super::super::data::OracleData;
use super::super::data::map_supported_faces;
use super::facts::facts_for;
use super::invariants::Thresholds;

/// Nearest-rank percentile of an ascending slice.
pub(super) fn percentile(sorted: &[usize], p: f64) -> usize {
    if sorted.is_empty() {
        return 0;
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "sorted.len() is a corpus-sized sample count, far below f64's 2^53 exact-integer range"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "p is used only as a percentile fraction in [0.0, 1.0] and sorted.len() is non-negative, \
        so the product and its ceiling are never negative"
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the ceiling is bounded by sorted.len(), a corpus-sized count far below usize::MAX"
    )]
    let rank = (p * sorted.len() as f64).ceil() as usize;
    let index = rank.saturating_sub(1).min(sorted.len() - 1);
    sorted[index]
}

/// Ceilings at the 99.9th percentile of printed supported faces.
///
/// A violation then means "more ambiguous than all but the worst printed card",
/// which is defensible; a hand-picked number would not be.
pub(super) fn calibrate(data: &OracleData) -> Thresholds {
    let samples = map_supported_faces(&data.faces, |_, card| {
        let facts = facts_for(&card.oracle_text, &data.catalogs);
        (facts.max_alternatives, facts.constituent_nodes)
    });

    let mut alternatives: Vec<usize> = samples.iter().map(|(a, _)| *a).collect();
    let mut nodes: Vec<usize> = samples.iter().map(|(_, n)| *n).collect();
    alternatives.sort_unstable();
    nodes.sort_unstable();

    Thresholds {
        max_alternatives: percentile(&alternatives, 0.999),
        constituent_nodes: percentile(&nodes, 0.999),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_picks_the_nearest_rank_value() {
        let sorted: Vec<usize> = (1..=100).collect();
        assert_eq!(percentile(&sorted, 0.50), 50);
        assert_eq!(percentile(&sorted, 0.999), 100);
        assert_eq!(percentile(&sorted, 0.0), 1);
    }

    #[test]
    fn percentile_of_one_sample_is_that_sample() {
        assert_eq!(percentile(&[7], 0.999), 7);
    }

    #[test]
    fn percentile_of_nothing_is_zero() {
        assert_eq!(percentile(&[], 0.999), 0);
    }
}
