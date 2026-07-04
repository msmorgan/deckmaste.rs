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
    /// The simultaneous batch this fact was a member of ([CR#603.3b]) —
    /// every substantive fact of one applied `Occurrence::Batch` shares one
    /// fresh id, so "these happened as ONE occurrence" ([CR#603.2c]) is
    /// readable from the log. `None` = a `Single` occurrence.
    pub batch: Option<Uint>,
    pub fact: GameEvent,
}

/// The append-only history log. Never truncated within a game.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History(Vec<HistEntry>);

impl History {
    /// Records `fact` as having occurred on `turn`, as a member of `batch`
    /// (`None` for a `Single` occurrence).
    pub(crate) fn record(&mut self, turn: Uint, batch: Option<Uint>, fact: GameEvent) {
        self.0.push(HistEntry { turn, batch, fact });
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

    /// The facts visible through `within`, given `current_turn`
    /// ([CR#608.2i]): `ThisTurn` is this turn's entries, `LastTurn` the
    /// previous turn's, `ThisGame` all of them. The sub-turn lookbacks
    /// (`ThisCombat`/`ThisStep`/`SinceYour`) need combat/step markers the
    /// log doesn't record yet — load-capped (E-BRIDGE-CAP, the
    /// `Lookback:*` bridge-caps rows) so no loadable card reaches one; an
    /// engine-built query through one trips loudly, never silently reads
    /// empty.
    pub(crate) fn scan(
        &self,
        within: Lookback,
        current_turn: Uint,
    ) -> impl Iterator<Item = &GameEvent> {
        if matches!(
            within,
            Lookback::ThisCombat | Lookback::ThisStep | Lookback::SinceYour(_)
        ) {
            todo!(
                "load-capped (E-BRIDGE-CAP): no combat/step markers in the history log yet \
                 (engine-history-windows)"
            )
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
        h.record(1, None, GameEvent::SpellCast(ObjectId::from_raw(1)));
        h.record(2, None, GameEvent::SpellCast(ObjectId::from_raw(2)));
        h.record(2, None, GameEvent::SpellCast(ObjectId::from_raw(3)));

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
