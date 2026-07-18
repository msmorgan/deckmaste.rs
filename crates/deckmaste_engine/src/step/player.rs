//! `EventApply` handlers (and, for the two bare-payload variants, plain
//! dispatch functions) for player/object bookkeeping events: tapping, mana,
//! life, counters, win/loss, designations, and control changes.

use crate::event::ControlChanged;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DesignationChanged;
use crate::event::GameEvent;
use crate::event::GotDesignation;
use crate::event::LifeGained;
use crate::event::LifeLost;
use crate::event::ManaAdded;
use crate::event::ManaEmptied;
use crate::event::PlayerLost;
use crate::event::PlayerWon;
use crate::event::Tapped;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameOutcome;
use crate::state::GameState;
use crate::step::EventApply;

impl EventApply for Tapped {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.objects.obj_mut(self.object).tapped = true;
        None
    }
}

impl EventApply for ManaAdded {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.player_mut(self.player)
            .mana_pool
            .add_riders(self.mana, self.amount, &self.riders);
        None
    }
}

impl EventApply for ManaEmptied {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.player_mut(self.player).mana_pool.empty_after(self.ending);
        None
    }
}

impl EventApply for PlayerLost {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.player_mut(self.player).lost = true;
        None
    }
}

impl EventApply for PlayerWon {
    // [CR#104.2b,104.1]: an effect-driven win ends the game outright —
    // this player wins, no one else wins or loses. Distinct from the
    // derived last-player-standing win `check_game_end` computes.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        if g.outcome.is_none() {
            g.outcome = Some(GameOutcome::Win(self.player));
            g.agenda.clear();
        }
        None
    }
}

impl EventApply for LifeLost {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.player_mut(self.player).life -=
            deckmaste_core::Int::try_from(self.amount).expect("life loss fits in i32");
        None
    }
}

impl EventApply for LifeGained {
    // [CR#119.3]: a player gains life — the life total adjusts up.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.player_mut(self.player).life +=
            deckmaste_core::Int::try_from(self.amount).expect("life gain fits in i32");
        None
    }
}

impl EventApply for CounterPlaced {
    // [CR#122.1]: counters live in the object's (or player proxy's)
    // counter map. Placement sums by kind; removal saturates at zero
    // and DROPS the key, so an absent kind reads as zero everywhere
    // (the layer-7c P/T read, `HasCounter`). The occurred fact
    // carries the carrier's before/after TOTALS, apply-computed —
    // the [CR#714.2b] `Crossed` reads run off the fact, never a
    // post-hoc map read.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let entry = g
            .objects
            .obj_mut(self.object)
            .counters
            .entry(self.kind)
            .or_insert(0);
        let before = *entry;
        *entry += self.amount;
        let after = *entry;
        Some(GameEvent::CounterPlaced(CounterPlaced {
            object: self.object,
            kind: self.kind,
            amount: self.amount,
            before,
            after,
            cause: self.cause.clone(),
        }))
    }
}

impl EventApply for CounterRemoved {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.remove_counters_clamped(self.object, &self.kind, self.amount);
        None
    }
}

impl EventApply for DesignationChanged {
    fn apply(&self, _g: &mut GameState) -> Option<GameEvent> {
        todo!("P0.W6: game-scope designation flip apply ([CR#731.1a])")
    }
}

impl EventApply for GotDesignation {
    // [CR#702.131c]: set the player-scope flag once; never removed.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.designations
            .players
            .entry((self.player, self.name))
            .or_insert(crate::state::DesignationValue::Flag);
        None
    }
}

impl EventApply for ControlChanged {
    // [CR#701.12b,613.1b]: a one-shot control TRANSITION — re-home
    // the object (a control change is never a zone move; the object
    // keeps its identity). The base controller moves; layer-2
    // continuous control effects still override on top. The new
    // controller has not controlled it continuously since their
    // last turn began, so it is summoning-sick for them
    // ([CR#302.6]).
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        if g.objects.get(self.object).is_some() {
            g.objects.obj_mut(self.object).controller = self.to;
            g.objects.obj_mut(self.object).summoning_sick = true;
        }
        None
    }
}

/// `Untapped` carries a bare `ObjectId` — no dedicated payload struct
/// (Tasks 3.1–3.2 structified only multi-field variants) — so it dispatches
/// through a plain function rather than `EventApply`.
pub(crate) fn handle_untapped(g: &mut GameState, id: ObjectId) -> Option<GameEvent> {
    g.objects.obj_mut(id).tapped = false;
    None
}

/// `DrewFromEmpty` carries a bare `PlayerId` — see `handle_untapped`.
///
/// The `Act(Draw)` apply-time transform produces this on an empty library;
/// this handler also serves future direct emitters (a replacement effect
/// rewriting a draw).
pub(crate) fn handle_drew_from_empty(g: &mut GameState, player: PlayerId) -> Option<GameEvent> {
    g.player_mut(player).drew_from_empty = true;
    None
}
