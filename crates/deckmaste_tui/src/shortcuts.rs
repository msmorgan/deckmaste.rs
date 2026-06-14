//! Runner-layer convenience over the decision loop (tui-shortcuts):
//! single-legal auto-resolve + per-player "pass" modes. Pure logic + small
//! state; no engine mutation, no ratatui — unit-tested headlessly. The engine
//! stays full-info and pure; what to auto-answer vs. surface is a runner
//! concern (like the autotapper).
use deckmaste_engine::Decision;
use deckmaste_engine::PendingDecision;

/// If every inner slice has exactly one element, the vector of those elements;
/// otherwise `None`. Generic so the "one candidate per slot" rule is testable
/// without constructing opaque `ObjectId`s.
#[must_use]
pub fn single_each<T: Copy>(legal: &[Vec<T>]) -> Option<Vec<T>> {
    if !legal.is_empty() && legal.iter().all(|slot| slot.len() == 1) {
        Some(legal.iter().map(|slot| slot[0]).collect())
    } else {
        None
    }
}

/// The forced answer to `pending` when exactly one legal answer exists and it
/// is not a priority window; otherwise `None`. Priority is never auto-resolved
/// here (passing is a timing choice, and auto-passing every pass-only window
/// globally would have no per-turn guard → both players pass to a decking
/// loss). The only interactive kind this changes is fully-forced targets; other
/// single-legal kinds (e.g. a lone trigger ordering) already auto-resolve via
/// the driver's `Strategy` partition (`is_interactive` returns false for them).
#[must_use]
pub fn auto_answer(pending: &PendingDecision) -> Option<Decision> {
    match pending {
        PendingDecision::ChooseTargets { legal, .. } => single_each(legal).map(Decision::Targets),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::PlayerId;

    use super::*;

    #[test]
    fn single_each_extracts_forced_slots() {
        assert_eq!(single_each(&[vec![1]]), Some(vec![1]));
        assert_eq!(single_each(&[vec![1], vec![2]]), Some(vec![1, 2]));
    }

    #[test]
    fn single_each_rejects_multi_empty_and_none() {
        assert_eq!(single_each(&[vec![1], vec![2, 3]]), None);
        assert_eq!(single_each::<i32>(&[vec![]]), None);
        assert_eq!(single_each::<i32>(&[]), None);
    }

    #[test]
    fn auto_answer_never_resolves_priority() {
        let p = PendingDecision::Priority {
            player: PlayerId(0),
            legal: vec![],
        };
        assert_eq!(auto_answer(&p), None);
    }

    #[test]
    fn auto_answer_ignores_non_target_kinds() {
        let p = PendingDecision::DiscardToHandSize {
            player: PlayerId(0),
            count: 1,
        };
        assert_eq!(auto_answer(&p), None);
    }
}
