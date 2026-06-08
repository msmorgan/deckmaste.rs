//! Per-turn (later per-game) counters: a small keyed registry replacing the
//! ad-hoc `lands_played_this_turn` field. Game-wide tallies (storm count) get a
//! parallel `Tallies` instance on `GameState` when a card forces them.

use std::collections::BTreeMap;

use deckmaste_core::Uint;

/// A counter key. Accretes as cards force new counters (like `StateFilterEvent`
/// in core). Per-player today; the same enum will serve game-wide tallies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tally {
    /// Lands played this turn ([CR#305.2]): one per turn by default.
    LandsPlayed,
    /// Cards drawn this turn ([CR#120]); read by "the first card you draw …".
    CardsDrawn,
}

/// A keyed counter bag. An absent key reads as `0`. `BTreeMap` for
/// deterministic iteration/equality.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tallies(BTreeMap<Tally, Uint>);

impl Tallies {
    /// The count for `tally` (`0` if never bumped).
    #[must_use]
    pub fn count(&self, tally: Tally) -> Uint { self.0.get(&tally).copied().unwrap_or(0) }

    /// Increments `tally` by one.
    pub(crate) fn bump(&mut self, tally: Tally) { *self.0.entry(tally).or_insert(0) += 1; }

    /// Clears every counter — the per-turn reset.
    pub(crate) fn reset(&mut self) { self.0.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_defaults_to_zero_then_counts_bumps() {
        let mut t = Tallies::default();
        assert_eq!(t.count(Tally::LandsPlayed), 0);
        t.bump(Tally::LandsPlayed);
        t.bump(Tally::LandsPlayed);
        assert_eq!(t.count(Tally::LandsPlayed), 2);
        assert_eq!(t.count(Tally::CardsDrawn), 0);
    }

    #[test]
    fn reset_clears_all_counters() {
        let mut t = Tallies::default();
        t.bump(Tally::CardsDrawn);
        t.reset();
        assert_eq!(t.count(Tally::CardsDrawn), 0);
    }
}
