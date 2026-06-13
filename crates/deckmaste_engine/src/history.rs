//! Turn/game event history ([CR#608.2i] history reads): an append-only log of
//! the facts that have occurred, each tagged with the turn it happened in. The
//! one source of truth the condition layer queries — `Count::Query` counts
//! matching entries, `Condition::Happened` tests for any match. Full-game
//! retention (a bounded game's event count is trivial); the window selects
//! which turn-tagged entries to read.

use deckmaste_core::Uint;
use deckmaste_core::Window;

use crate::event::GameEvent;

/// One recorded fact and the turn ([CR#500.1]) it occurred in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistEntry {
    pub turn: Uint,
    pub fact: GameEvent,
}

/// The append-only history log. Never truncated within a game.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History(Vec<HistEntry>);

impl History {
    /// Records `fact` as having occurred on `turn`.
    pub(crate) fn record(&mut self, turn: Uint, fact: GameEvent) {
        self.0.push(HistEntry { turn, fact });
    }

    /// The facts visible through `within`, given `current_turn`. Only the
    /// history-lookback windows are meaningful here ([CR#608.2i]): `ThisTurn`
    /// is this turn's entries, `ThisGame` is all of them. The timing windows
    /// (`InstantSpeed`/`SorcerySpeed`/`DuringTurn`/`DuringStep`) are not
    /// lookbacks; callers gate them out, so they read as empty defensively.
    pub(crate) fn scan(
        &self,
        within: Window,
        current_turn: Uint,
    ) -> impl Iterator<Item = &GameEvent> {
        self.0
            .iter()
            .filter(move |e| match within {
                Window::ThisTurn => e.turn == current_turn,
                Window::ThisGame => true,
                _ => false,
            })
            .map(|e| &e.fact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::ObjectId;

    /// `ThisTurn` returns only the current turn's entries; `ThisGame` returns
    /// all of them.
    #[test]
    fn scan_windows_select_by_turn() {
        let mut h = History::default();
        h.record(1, GameEvent::SpellCast(ObjectId::from_raw(1)));
        h.record(2, GameEvent::SpellCast(ObjectId::from_raw(2)));
        h.record(2, GameEvent::SpellCast(ObjectId::from_raw(3)));

        assert_eq!(
            h.scan(Window::ThisTurn, 2).count(),
            2,
            "ThisTurn = turn-2 entries"
        );
        assert_eq!(
            h.scan(Window::ThisTurn, 1).count(),
            1,
            "ThisTurn = turn-1 entries"
        );
        assert_eq!(
            h.scan(Window::ThisGame, 2).count(),
            3,
            "ThisGame = all entries"
        );
    }
}
