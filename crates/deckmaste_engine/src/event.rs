use deckmaste_core::{ColorOrColorless, StepOrPhase, Uint};

use crate::object::ObjectId;
use crate::player::PlayerId;

/// Why a player lost ([CR#704.5]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossReason {
    /// [CR#704.5a].
    LifeZero,
    /// [CR#704.5c].
    DrewFromEmpty,
}

/// A concrete occurrence: what `Emit` pushes through the (future) cant →
/// replace → apply pipe. Scheduled as an intent, returned from apply as the
/// occurred fact — `CardDrawn.object` binds at apply time, and a draw from
/// an empty library applies as `DrewFromEmpty` instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    TurnBegan {
        player: PlayerId,
        turn: Uint,
    },
    StepBegan(StepOrPhase),
    Untapped(ObjectId),
    CardDrawn {
        player: PlayerId,
        object: Option<ObjectId>,
    },
    DrewFromEmpty(PlayerId),
    LandPlayed {
        object: ObjectId,
    },
    Tapped(ObjectId),
    ManaAdded {
        player: PlayerId,
        mana: ColorOrColorless,
        amount: Uint,
    },
    ManaEmptied(PlayerId),
    Discarded {
        player: PlayerId,
        object: ObjectId,
    },
    PlayerLost {
        player: PlayerId,
        reason: LossReason,
    },
}
