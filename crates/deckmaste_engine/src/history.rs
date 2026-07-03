//! Turn/game event history ([CR#608.2i] history reads): an append-only log of
//! the facts that have occurred, each tagged with the turn it happened in. The
//! one source of truth the condition layer queries — `Count::EventCount`/
//! `Count::EventSum` tally matching entries, `Condition::Happened` tests for
//! any match. Full-game retention (a bounded game's event count is trivial);
//! the window selects which turn-tagged entries to read.

use deckmaste_core::Lookback;
use deckmaste_core::Uint;

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

    /// The facts visible through `within`, given `current_turn`
    /// ([CR#608.2i]): `ThisTurn` is this turn's entries, `LastTurn` the
    /// previous turn's, `ThisGame` all of them. The sub-turn lookbacks
    /// (`ThisCombat`/`ThisStep`/`SinceYour`) need combat/step markers the
    /// log doesn't record yet — a query through one must trip loudly, not
    /// silently read empty.
    pub(crate) fn scan(
        &self,
        within: Lookback,
        current_turn: Uint,
    ) -> impl Iterator<Item = &GameEvent> {
        if matches!(
            within,
            Lookback::ThisCombat | Lookback::ThisStep | Lookback::SinceYour(_)
        ) {
            todo!("engine-history-windows: no combat/step markers in the history log yet")
        }
        self.0
            .iter()
            .filter(move |e| match within {
                Lookback::ThisTurn => e.turn == current_turn,
                Lookback::ThisGame => true,
                Lookback::LastTurn => e.turn + 1 == current_turn,
                Lookback::ThisCombat | Lookback::ThisStep | Lookback::SinceYour(_) => {
                    unreachable!("gated above")
                }
            })
            .map(|e| &e.fact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::ObjectId;

    /// `ThisTurn` returns only the current turn's entries, `LastTurn` only
    /// the previous turn's; `ThisGame` returns all of them.
    #[test]
    fn scan_windows_select_by_turn() {
        let mut h = History::default();
        h.record(1, GameEvent::SpellCast(ObjectId::from_raw(1)));
        h.record(2, GameEvent::SpellCast(ObjectId::from_raw(2)));
        h.record(2, GameEvent::SpellCast(ObjectId::from_raw(3)));

        assert_eq!(
            h.scan(Lookback::ThisTurn, 2).count(),
            2,
            "ThisTurn = turn-2 entries"
        );
        assert_eq!(
            h.scan(Lookback::ThisTurn, 1).count(),
            1,
            "ThisTurn = turn-1 entries"
        );
        assert_eq!(
            h.scan(Lookback::ThisGame, 2).count(),
            3,
            "ThisGame = all entries"
        );
        assert_eq!(
            h.scan(Lookback::LastTurn, 2).count(),
            1,
            "LastTurn from turn 2 = turn-1 entries"
        );
        assert_eq!(
            h.scan(Lookback::LastTurn, 3).count(),
            2,
            "LastTurn from turn 3 = turn-2 entries"
        );
    }
}
