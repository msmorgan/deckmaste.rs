//! Per-turn (later per-game) counters: a small keyed registry replacing the
//! ad-hoc `lands_played_this_turn` field. Game-wide tallies (storm count) get a
//! parallel `Tallies` instance on `GameState` when a card forces them.

use std::collections::BTreeMap;

use deckmaste_core::Uint;

use crate::object::ObjectId;

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

/// Per-(object, ability-index) activation counts backing "Activate only
/// once …" limits ([CR#602.5b]). Reminting on a zone change yields a fresh
/// `ObjectId`, so a permanent that leaves and returns starts fresh (a new
/// object); a controller change does not reset it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActivationLedger {
    this_turn: BTreeMap<(ObjectId, usize), Uint>,
    this_game: BTreeMap<(ObjectId, usize), Uint>,
}

impl ActivationLedger {
    // Called from the activation pipeline (Task 7) and the test module.
    #[allow(dead_code)]
    pub(crate) fn bump(&mut self, key: (ObjectId, usize)) {
        *self.this_turn.entry(key).or_insert(0) += 1;
        *self.this_game.entry(key).or_insert(0) += 1;
    }

    /// Activations of `key` since the turn began.
    #[must_use]
    pub fn this_turn(&self, key: (ObjectId, usize)) -> Uint {
        self.this_turn.get(&key).copied().unwrap_or(0)
    }

    /// Activations of `key` this game.
    #[must_use]
    pub fn this_game(&self, key: (ObjectId, usize)) -> Uint {
        self.this_game.get(&key).copied().unwrap_or(0)
    }

    /// The per-turn reset (`begin_turn`).
    pub(crate) fn reset_turn(&mut self) { self.this_turn.clear(); }
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

    // ActivationLedger tests
    use super::ActivationLedger;
    use crate::object::ObjectId;

    #[test]
    fn ledger_bump_tracks_turn_and_game() {
        let mut ledger = ActivationLedger::default();
        let key = (ObjectId(1), 0);
        ledger.bump(key);
        ledger.bump(key);
        assert_eq!(
            ledger.this_turn(key),
            2,
            "two bumps should give turn count 2"
        );
        assert_eq!(
            ledger.this_game(key),
            2,
            "two bumps should give game count 2"
        );
    }

    #[test]
    fn ledger_reset_turn_clears_turn_keeps_game() {
        let mut ledger = ActivationLedger::default();
        let key = (ObjectId(1), 0);
        ledger.bump(key);
        ledger.bump(key);
        ledger.reset_turn();
        assert_eq!(
            ledger.this_turn(key),
            0,
            "reset_turn should clear turn count"
        );
        assert_eq!(
            ledger.this_game(key),
            2,
            "reset_turn must not clear game count"
        );
    }

    #[test]
    fn ledger_distinct_ability_indices_tracked_separately() {
        let mut ledger = ActivationLedger::default();
        let obj = ObjectId(1);
        ledger.bump((obj, 0));
        ledger.bump((obj, 1));
        ledger.bump((obj, 1));
        assert_eq!(ledger.this_turn((obj, 0)), 1);
        assert_eq!(ledger.this_turn((obj, 1)), 2);
    }
}
