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
    /// [CR#601.2i] — a spell becomes cast. Applies by promoting `announcing`
    /// onto the stack. The Stage-3 "whenever you cast" seam.
    SpellCast(ObjectId),
    /// [CR#119] — damage to a creature (marked) or a player (life loss).
    DamageDealt {
        source: ObjectId,
        target: ObjectId,
        amount: Uint,
    },
    /// A permanent spell resolving onto the battlefield ([CR#608.3]).
    EntersBattlefield(ObjectId),
    /// An instant/sorcery leaving the stack for its owner's graveyard after
    /// resolution or fizzle ([CR#608.2m]).
    SpellResolved(ObjectId),
    /// [CR#704.5g] result: a permanent destroyed to its owner's graveyard.
    Destroyed(ObjectId),
    /// [CR#119.3]: a player loses life directly (not via damage).
    LifeLost {
        player: PlayerId,
        amount: Uint,
    },
}

/// A scheduled occurrence: one event, or a set of simultaneous events applied
/// and matched together ([CR#603.3b], [CR#700.4]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Occurrence {
    Single(GameEvent),
    Batch(Vec<GameEvent>),
}

impl Occurrence {
    /// Convenience: wrap a single event.
    #[must_use]
    pub fn single(event: GameEvent) -> Self { Occurrence::Single(event) }
}
