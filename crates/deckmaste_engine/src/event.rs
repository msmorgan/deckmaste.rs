use deckmaste_core::{ColorOrColorless, Phase, Uint, Zone};

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
/// occurred fact — the draw's library top binds at `WillDraw` apply time, and a
/// draw from an empty library applies as `DrewFromEmpty` instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    TurnBegan {
        player: PlayerId,
        turn: Uint,
    },
    StepBegan(Phase),
    Untapped(ObjectId),
    /// The INTENT of a draw ([CR#120.1]). Replaceable (Notion Thief, Lab
    /// Maniac — future). Its apply checks the library: a card present → bind
    /// the top, bump `CardsDrawn`, and evolve into `ZoneWillChange(Library →
    /// Hand)`; an empty library → `DrewFromEmpty` ([CR#120.3,704.5c]). `source`
    /// is the object that drew the card, or `None` for the turn-based draw-step
    /// draw ([CR#504.1]) — "the first card you draw on your draw step" keys on
    /// `None`.
    WillDraw {
        player: PlayerId,
        source: Option<ObjectId>,
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
    /// The INTENT of a zone change ([CR#400.7]). Replacements act here. Its
    /// apply captures LKI, moves+remints the object, folds the object's own
    /// `AsEnters` self-replacements into the entering status, and emits
    /// `ZoneChanged`. `enters` is present only when `to == Battlefield`.
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
    /// [CR#603.8]: a triggered ability finished resolving and vanishes — no
    /// zone move, the source untouched. Its apply removes the stack entry whose
    /// `id` is the carried (minted) token; that token is then discarded.
    TriggerResolved(ObjectId),
}

/// How a permanent enters the battlefield ([CR#110.5] status;
/// counters/face-down are later). Present on a `ZoneWillChange` only when `to
/// == Battlefield`.
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
