//! Per-(object, ability-index) activation counts ([CR#602.5b]) backing
//! "Activate only once …" limits. Turn/game *history* lives in `history.rs`
//! now (the old `Tally`/`Tallies` per-turn registry was folded into the event
//! log; a cached fast-path is the `engine-history-tallies-cache` todo).

use std::collections::BTreeMap;

use deckmaste_core::Uint;

use crate::object::ObjectId;

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
    /// Count one activation of `key` — both the per-turn and per-game windows.
    /// Called when `AbilityActivated` applies ([CR#602.2a]).
    pub(crate) fn bump(&mut self, key: (ObjectId, usize)) {
        *self.this_turn.entry(key).or_insert(0) += 1;
        *self.this_game.entry(key).or_insert(0) += 1;
    }

    /// Activations of `key` since the turn began.
    #[must_use]
    pub fn turn_count(&self, key: (ObjectId, usize)) -> Uint {
        self.this_turn.get(&key).copied().unwrap_or(0)
    }

    /// Activations of `key` this game.
    #[must_use]
    pub fn game_count(&self, key: (ObjectId, usize)) -> Uint {
        self.this_game.get(&key).copied().unwrap_or(0)
    }

    /// The per-turn reset (`begin_turn`).
    pub(crate) fn reset_turn(&mut self) { self.this_turn.clear(); }
}

#[cfg(test)]
mod tests {
    use super::ActivationLedger;
    use crate::object::ObjectId;

    #[test]
    fn ledger_bump_tracks_turn_and_game() {
        let mut ledger = ActivationLedger::default();
        let key = (ObjectId::from_raw(1), 0);
        ledger.bump(key);
        ledger.bump(key);
        assert_eq!(
            ledger.turn_count(key),
            2,
            "two bumps should give turn count 2"
        );
        assert_eq!(
            ledger.game_count(key),
            2,
            "two bumps should give game count 2"
        );
    }

    #[test]
    fn ledger_reset_turn_clears_turn_keeps_game() {
        let mut ledger = ActivationLedger::default();
        let key = (ObjectId::from_raw(1), 0);
        ledger.bump(key);
        ledger.bump(key);
        ledger.reset_turn();
        assert_eq!(
            ledger.turn_count(key),
            0,
            "reset_turn should clear turn count"
        );
        assert_eq!(
            ledger.game_count(key),
            2,
            "reset_turn must not clear game count"
        );
    }

    #[test]
    fn ledger_distinct_ability_indices_tracked_separately() {
        let mut ledger = ActivationLedger::default();
        let obj = ObjectId::from_raw(1);
        ledger.bump((obj, 0));
        ledger.bump((obj, 1));
        ledger.bump((obj, 1));
        assert_eq!(ledger.turn_count((obj, 0)), 1);
        assert_eq!(ledger.turn_count((obj, 1)), 2);
    }
}
