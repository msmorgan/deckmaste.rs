//! Turn/game event history ([CR#608.2i] history reads): an append-only log of
//! the facts that have occurred, each tagged with the turn it happened in and
//! carrying its per-fact LKI [`FactView`] ([CR#603.10a]) — captured at RECORD
//! time, so history matching reads participants as they were, never the live
//! store through a stale id. The one source of truth the condition layer
//! queries — `Count::EventCount`/`Count::EventSum` tally matching entries,
//! `Condition::Happened` tests for any match. Full-game retention (a bounded
//! game's event count is trivial); the window selects which turn-tagged
//! entries to read.

use deckmaste_core::Lookback;
use deckmaste_core::Uint;

use crate::eval::FactView;
use crate::eval::window_contains;
use crate::event::GameEvent;

/// One recorded fact and the turn ([CR#500.1]) it occurred in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistEntry {
    pub turn: Uint,
    /// The simultaneous batch this fact was a member of ([CR#603.3b]) —
    /// every substantive fact of one applied `Occurrence::Batch` shares one
    /// fresh id, so "these happened as ONE occurrence" ([CR#603.2c]) is
    /// readable from the log. `None` = a `Single` occurrence.
    pub batch: Option<Uint>,
    pub fact: GameEvent,
    /// The fact's record for matching, with per-fact LKI participants
    /// ([CR#603.10a]) — built at record time. `None` for plumbing facts no
    /// pattern watches (they still ride the log for batch/debug reads).
    pub(crate) view: Option<FactView<'static>>,
}

/// The append-only history log. Never truncated within a game.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History(Vec<HistEntry>);

impl History {
    /// Records `fact` as having occurred on `turn`, as a member of `batch`
    /// (`None` for a `Single` occurrence), with its per-fact LKI `view`.
    /// The entry's log position and turn are stamped onto the view — the
    /// [`crate::eval::Lane::History`] ordinal
    /// ([`deckmaste_core::EventFilter::Nth`]) and window reads key off
    /// them. Prefer [`crate::state::GameState::record_history_fact`], which
    /// builds the view.
    pub(crate) fn record(
        &mut self,
        turn: Uint,
        batch: Option<Uint>,
        fact: GameEvent,
        mut view: Option<FactView<'static>>,
    ) {
        if let Some(v) = view.as_mut() {
            v.time = turn;
            v.seq = Some(self.0.len());
        }
        self.0.push(HistEntry {
            turn,
            batch,
            fact,
            view,
        });
    }

    /// The recorded entries, oldest first — batch-id reads (the
    /// [CR#603.3b] "one occurrence" grouping) go through here; the
    /// fact-only view is [`scan`](History::scan).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the batch-id read surface — core-anaphor-surface's product-antecedent \
                      reads consume it; the batch/fixture tests pin it meanwhile"
        )
    )]
    pub(crate) fn entries(&self) -> impl Iterator<Item = &HistEntry> {
        self.0.iter()
    }

    /// The entries visible through `within`, given `current_turn`
    /// ([CR#608.2i]), with their log positions — the one evaluator's
    /// history-lane feed (`Happened`/`EventCount`/`EventSum`/`Nth`
    /// counting). The sub-turn lookbacks (`ThisCombat`/`ThisStep`/
    /// `SinceYour`) stay load-capped (E-BRIDGE-CAP, the `Lookback:*` rows;
    /// engine-history-windows) and are unreachable here.
    pub(crate) fn in_window(
        &self,
        within: Lookback,
        current_turn: Uint,
    ) -> impl Iterator<Item = (usize, &HistEntry)> {
        self.0
            .iter()
            .enumerate()
            .filter(move |(_, e)| window_contains(within, e.turn, current_turn))
    }

    /// The facts visible through `within`, given `current_turn`
    /// ([CR#608.2i]) — the raw-fact view of [`in_window`](History::in_window)
    /// for readers that consume `GameEvent`s directly
    /// (`lands_played_this_turn` and kin).
    pub(crate) fn scan(
        &self,
        within: Lookback,
        current_turn: Uint,
    ) -> impl Iterator<Item = &GameEvent> {
        self.in_window(within, current_turn).map(|(_, e)| &e.fact)
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
        h.record(1, None, GameEvent::SpellCast(ObjectId::from_raw(1)), None);
        h.record(2, None, GameEvent::SpellCast(ObjectId::from_raw(2)), None);
        h.record(2, None, GameEvent::SpellCast(ObjectId::from_raw(3)), None);

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
