use deckmaste_core::ColorOrColorless;
use deckmaste_core::Phase;
use deckmaste_core::Token;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;

/// Why a player lost ([CR#104.3,704.5]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossReason {
    /// [CR#704.5a].
    LifeZero,
    /// [CR#704.5b].
    DrewFromEmpty,
    /// Ten or more poison counters ([CR#704.5c]; Two-Headed Giant swaps in
    /// the fifteen-counter TEAM check [CR#704.6b] — variant-gated).
    Poison,
    /// Concession ([CR#104.3a]) — immediate, any time, and UNSTOPPABLE:
    /// the single exception to card-beats-rules ([CR#101.1]); no `CantLose`
    /// gate touches it, and a controlled player's controller can't prevent
    /// it ([CR#723.6]).
    Conceded,
}

/// A concrete occurrence: what `Emit` pushes through the (future) cant →
/// replace → apply pipe. Scheduled as an intent, returned from apply as the
/// occurred fact — the draw's library top binds at `WillDraw` apply time, and a
/// draw from an empty library applies as `DrewFromEmpty` instead.
/// The cause triple riding an event (mtg-rules events.md §3): the named
/// VERB view performed ("Sacrifice", "Discard", "Play", …), the AGENCY
/// that demanded it, and the AGENT — the causing object and its
/// controller, `None` for turn-based / state-based actions. Trigger
/// patterns (`CausePattern`) predicate over these coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cause {
    pub verb: deckmaste_core::Ident,
    pub agency: deckmaste_core::Agency,
    pub agent: Option<(ObjectId, PlayerId)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    TurnBegan {
        player: PlayerId,
        turn: Uint,
    },
    StepBegan(Phase),
    Untapped(ObjectId),
    // Shaped, unbuilt (no fixture forces it yet): `WillDiscard` (madness, above
    // Hand→Graveyard) — the replaceable intent above its committed zone change,
    // like `WillDraw` below.
    /// The INTENT of a destruction ([CR#701.8a]). Replaceable above the
    /// committed Battlefield→Graveyard move (like `WillDraw`): its apply checks
    /// the object's derived view for a destruction-replacement static —
    /// indestructible ([CR#702.12b]) or, once they exist, a regeneration
    /// shield (an `engine-replacements` seam). Present → the destroy is replaced
    /// to nothing (the object is untouched). Absent → it evolves into
    /// `ZoneWillChange(Battlefield → Graveyard)` carrying `cause`, one of
    /// "destroyed"'s two causes ([CR#701.8b]).
    WillDestroy {
        object: ObjectId,
        cause: Option<Cause>,
    },
    /// The INTENT of a draw ([CR#121.1]). Replaceable (Notion Thief, Lab
    /// Maniac — future). Its apply checks the library: a card present → bind
    /// the top, bump `CardsDrawn`, and evolve into `ZoneWillChange(Library →
    /// Hand)`; an empty library → `DrewFromEmpty` ([CR#121.4,704.5b]). `source`
    /// is the object that drew the card, or `None` for the turn-based draw-step
    /// draw ([CR#504.1]) — "the first card you draw on your draw step" keys on
    /// `None`.
    WillDraw {
        player: PlayerId,
        source: Option<ObjectId>,
    },
    DrewFromEmpty(PlayerId),

    Tapped {
        object: ObjectId,
        /// Tap causes are trigger-visible language ([CR#107.5] cost vs
        /// [CR#508.1f] attack vs [CR#701.26a] effect vs [CR#106.12] mana).
        cause: Option<Cause>,
    },
    ManaAdded {
        player: PlayerId,
        mana: ColorOrColorless,
        amount: Uint,
    },
    ManaEmptied(PlayerId),

    /// [CR#701.7a,111.2]: `player` creates one token with the characteristics
    /// `token` specifies. Its apply synthesizes a token entry in the card
    /// table (owner = creator), mints the object straight onto the battlefield
    /// (controller = creator), folds `AsEnters` self-replacements, and emits
    /// the `ZoneChanged { from: None, to: Battlefield }` fact so enter-triggers
    /// fire. Creating N tokens is a `Batch` of N of these (one instruction,
    /// simultaneous). "Whenever you create a token" triggers match here.
    TokenCreated {
        player: PlayerId,
        token: Token,
    },
    /// [CR#704.5d,111.7]: a token found in a zone other than the battlefield
    /// ceases to exist. Its apply removes the object from its zone and the
    /// store outright — no remint, no `ZoneChanged` fact (the token doesn't
    /// move, it stops existing). Zone-leave triggers already fired at the
    /// move that stranded it ([CR#111.7]'s note); anything still pointing at
    /// it reads the LKI that rode that fact.
    TokenCeased(ObjectId),
    PlayerLost {
        player: PlayerId,
        reason: LossReason,
    },
    /// [CR#601.2i] — a spell becomes cast. Applies by promoting `announcing`
    /// onto the stack. The Stage-3 "whenever you cast" seam.
    SpellCast(ObjectId),
    /// [CR#602.2a] — an ability becomes activated. Applies by minting the
    /// stack identity, promoting `announcing` onto the stack, and bumping the
    /// activation ledger. The "whenever … activates an ability" trigger seam
    /// (engine-trigger-events).
    AbilityActivated {
        source: ObjectId,
        ability: usize,
    },
    /// [CR#120.3] — damage to a creature (marked) or a player (life loss).
    DamageDealt {
        source: ObjectId,
        target: ObjectId,
        amount: Uint,
    },
    /// The INTENT of a zone change ([CR#400.7]). Replacements act here. Its
    /// apply captures LKI, moves+remints the object, folds the object's own
    /// `AsEnters` self-replacements into the entering status, and emits
    /// `ZoneChanged`. `enters` is present only when `to == Battlefield`.
    /// `position` is present only when `to == Library`: the insertion index
    /// counted from the top (`0` = top), clamped to the bottom when the
    /// library is shorter ([CR#401.7]); `None` means the top.
    ZoneWillChange {
        object: ObjectId,
        from: Option<Zone>,
        to: Zone,
        enters: Option<EnterStatus>,
        position: Option<Uint>,
        /// The face shown on arrival — the master event's `face`
        /// coordinate; `None` = the default, face up ([CR#110.5b]). No
        /// emitter sets `Down` yet (morph/manifest are post-P0 macros);
        /// reveal-on-leave ([CR#708.9]) hooks here when they do.
        face: Option<deckmaste_core::Face>,
        /// `None` = an unattributed move; named views (sacrificed,
        /// discarded, played) ride here as cause triples.
        cause: Option<Cause>,
    },
    /// The FACT ([CR#603.6]) — unreplaceable; carries the moved object's LKI.
    /// Triggers (later tasks) fire on it.
    ZoneChanged {
        snapshot: crate::lki::LkiSnapshot,
        from: Option<Zone>,
        to: Zone,
        /// Copied through from the `ZoneWillChange` intent.
        face: Option<deckmaste_core::Face>,
        /// Copied through from the `ZoneWillChange` intent.
        cause: Option<Cause>,
    },
    /// [CR#119.3]: a player loses life directly (not via damage).
    LifeLost {
        player: PlayerId,
        amount: Uint,
    },
    /// [CR#119.3]: a player gains life.
    LifeGained {
        player: PlayerId,
        amount: Uint,
    },
    /// [CR#508.1a]: a creature was declared as an attacker. Its apply records
    /// it in `CombatState` and taps it ([CR#508.1f]). The "whenever ~ attacks"
    /// trigger seam (`StateFilterEvent::Attacking`).
    Attacking(ObjectId),
    /// [CR#509.1a]: a creature was declared as a blocker against `attacker`. Its
    /// apply records the block in `CombatState` and marks `attacker` blocked
    /// ([CR#509.1h]). The "whenever ~ blocks / becomes blocked" trigger seam.
    Blocked {
        blocker: ObjectId,
        attacker: ObjectId,
    },
    /// [CR#603.2]: a triggered ability triggered. Its apply notes it into
    /// `pending_triggers`. Routed as an event so Stage-4 replacements/cant can
    /// intercept (Panharmonicon/Hushwing).
    /// A coin flip's outcome ([CR#705.1..705.2]).
    CoinFlipped {
        player: PlayerId,
        heads: bool,
    },
    /// A die roll's outcome ([CR#706.1..706.2]); an IGNORED roll is
    /// considered never to have happened — no triggers ([CR#706.6]).
    DieRolled {
        player: PlayerId,
        sides: Uint,
        natural: Uint,
        result: Uint,
    },
    /// Counters placed on an object or player proxy ([CR#122.1]).
    CounterPlaced {
        object: ObjectId,
        kind: deckmaste_core::Ident,
        count: Uint,
        cause: Option<Cause>,
    },
    /// Counters removed ([CR#122.1]).
    CounterRemoved {
        object: ObjectId,
        kind: deckmaste_core::Ident,
        count: Uint,
        cause: Option<Cause>,
    },
    TriggerFired {
        source: ObjectSource,
        ability: Uint,
        controller: PlayerId,
        bindings: crate::trigger::TriggerBindings,
    },
    /// [CR#608.2n]: a triggered or activated ability finished
    /// resolving (or fizzled) and vanishes — no zone move. Its apply removes
    /// the stack entry whose `id` is the carried (minted) token.
    AbilityResolved(ObjectId),
    /// Cards shown ([CR#701.20a]); `to: None` = revealed to ALL players,
    /// `Some` = "look at" — the same operation shown to a subset
    /// ([CR#701.20e]). Revealing never moves the card ([CR#701.20b]).
    /// Shaped, unbuilt: the `Reveal` resolve seam emits it (P0.W6); the
    /// reveal WINDOW (how long it stays shown) is effect-instance
    /// machinery.
    Revealed {
        objects: Vec<ObjectId>,
        to: Option<Vec<PlayerId>>,
    },
    /// A GAME-scope designation transition in the W5 registry (day/night,
    /// [CR#731.1] — "day becomes night" = losing one designation and
    /// gaining the other, [CR#731.1a]). Shaped, unbuilt: designation
    /// GRANTING effects are P0.W5/W6 seams. Object/player designation
    /// deltas ride their own facts when granting lands.
    DesignationChanged {
        name: deckmaste_core::Ident,
        becomes: Option<deckmaste_core::Ident>,
    },
    /// A library was shuffled ([CR#701.24a]) — an INFORMATION event:
    /// order knowledge is destroyed for every player; revealed cards in
    /// it stop being revealed and become new objects ([CR#701.20d] —
    /// revealed-state reset is a P0.W6 seam). Why library actions never
    /// rewind: [CR#733.1].
    Shuffled(PlayerId),
    /// An object became the target of the spell/ability `source` at
    /// announce ([CR#601.2c]; ward is the family exemplar [CR#702.21a]).
    /// Shaped, unbuilt: the announce flow emits it (P0.W7 seam).
    BecameTarget {
        target: ObjectId,
        source: ObjectId,
    },
    /// An object changed controller — a becomes-delta, never a zone move
    /// (the object keeps its identity). Shaped, unbuilt: control-changing
    /// continuous effects are a layers seam (L2); its apply will re-home
    /// the object and fire `StateFilterEvent::ControlledBy` patterns.
    ControlChanged {
        object: ObjectId,
        to: PlayerId,
    },
}

/// How a permanent enters the battlefield ([CR#110.5] status;
/// counters/face-down are later). Present on a `ZoneWillChange` only when `to
/// == Battlefield`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EnterStatus {
    pub tapped: bool,
}

/// Who learns an event's full payload — the projection-boundary annotation
/// the per-player view (a RUNNER concern; the engine stays
/// full-information) consumes. mtg-rules information.md §6: replay,
/// netplay, and AI all read this layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Audience {
    /// Everyone sees the whole payload ([CR#400.2] public-zone default).
    Public,
    /// The full payload is for these players; everyone else learns only
    /// THAT the event happened (shape and count, not identity — a hidden
    /// zone's size is public, [CR#401.3,402.3]).
    Restricted(Vec<PlayerId>),
}

impl GameEvent {
    /// The event's audience, DERIVED from kind + payload (the
    /// schema-from-kind pattern — no stored field to drift). Coarse first
    /// pass: a move between two hidden zones discloses the card's identity
    /// to its owner alone ([CR#400.2] — a draw, a mulligan bottoming);
    /// either endpoint public makes the identity public history (a
    /// discard arrives face up in a public zone) even though a hidden
    /// destination then conceals the card. Subset reveals restrict to the
    /// named players ([CR#701.20e]). Refinements — face-down commitments
    /// ([CR#708.2]), stateful look grants ([CR#406.3]) — arrive with
    /// their machinery.
    #[must_use]
    pub fn audience(&self, state: &crate::state::GameState) -> Audience {
        match self {
            GameEvent::ZoneWillChange {
                object,
                from: Some(from),
                to,
                ..
            } if from.is_hidden() && to.is_hidden() => {
                Audience::Restricted(vec![state.owner_of(*object)])
            }
            GameEvent::ZoneChanged {
                snapshot,
                from: Some(from),
                to,
                ..
            } if from.is_hidden() && to.is_hidden() => match snapshot.source {
                ObjectSource::Card(card) => Audience::Restricted(vec![state.cards.get(card).owner]),
                ObjectSource::Player(p) => Audience::Restricted(vec![p]),
            },
            GameEvent::Revealed {
                to: Some(players), ..
            } => Audience::Restricted(players.clone()),
            _ => Audience::Public,
        }
    }
}

/// A scheduled occurrence: one event, or a set of simultaneous events applied
/// and matched together ([CR#603.3b], [CR#700.1]).
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
