use deckmaste_core::{ColorOrColorless, StepOrPhase, Uint, Zone};

use crate::object::{ObjectId, ObjectSource};
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
    /// The INTENT of a zone change ([CR#400.7]). Replacements act here (none
    /// wired in 5a). Its apply captures LKI, moves+remints the object, and
    /// emits `ZoneChanged`. `enters` is present only when `to == Battlefield`.
    ZoneWillChange {
        object: ObjectId,
        from: Option<Zone>,
        to: Zone,
        enters: Option<EnterStatus>,
    },
    /// The FACT ([CR#603.6]) — unreplaceable; carries the moved object's LKI.
    /// Triggers (later tasks) fire on it.
    ZoneChanged {
        snapshot: crate::lki::LkiSnapshot,
        from: Option<Zone>,
        to: Zone,
    },
    /// [CR#119.3]: a player loses life directly (not via damage).
    LifeLost {
        player: PlayerId,
        amount: Uint,
    },
    /// [CR#603.2]: a triggered ability triggered. Its apply notes it into
    /// `pending_triggers`. Routed as an event so Stage-4 replacements/cant can
    /// intercept (Panharmonicon/Hushwing).
    TriggerFired {
        source: ObjectSource,
        ability: Uint,
        controller: PlayerId,
        bindings: crate::trigger::TriggerBindings,
    },
}

/// How a permanent enters the battlefield ([CR#110.5] status;
/// counters/face-down are later). Present on a `ZoneWillChange` only when `to
/// == Battlefield`. In 5a it is always `None` from the permanent-zone causes
/// (enters-tapped is a later task).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EnterStatus {
    pub tapped: bool,
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
