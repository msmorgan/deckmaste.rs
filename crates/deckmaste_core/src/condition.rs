use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Count;
use crate::EventFilter;
use crate::Predicate;
use crate::Reference;

/// A numeric comparison ([CR#107.3]). The named forms keep RON readable —
/// `AtLeast` rather than `>=`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Cmp {
    Eq,
    AtLeast,
    AtMost,
    Greater,
    Less,
}

impl Cmp {
    /// Does `lhs <op> rhs` hold? The single comparator table every numeric
    /// test routes through — `Condition::Compare`'s two evaluated counts and
    /// `CountBound::satisfied_by`'s matched-set cardinality alike.
    #[must_use]
    pub fn apply(self, lhs: crate::Uint, rhs: crate::Uint) -> bool {
        match self {
            Cmp::Eq => lhs == rhs,
            Cmp::AtLeast => lhs >= rhs,
            Cmp::AtMost => lhs <= rhs,
            Cmp::Greater => lhs > rhs,
            Cmp::Less => lhs < rhs,
        }
    }
}

use crate::Lookback;

/// A truth-valued test the engine evaluates ([CR#603.4] intervening-if,
/// [CR#118.12a] "unless", ability words). Ability words (`Threshold`,
/// `Delirium`, `Morbid`) are expanded by the semantics layer before values
/// reach core.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Condition {
    /// Compare two scalar counts ([CR#107.1]).
    Compare(Count, Cmp, Count),
    /// At least one object matches ([CR#603.4], "if you control a …").
    Exists(Predicate),
    /// A referenced object matches a predicate ([CR#603.4], "if it is a …").
    /// Idris `Matches`.
    Matches(Reference, Predicate),
    /// The referenced object has marked damage from at least one source whose
    /// deal-time abilities match the predicate ([CR#704.5h]). The query reads
    /// stored damage history; it does not resolve the source as an object.
    DealtDamageBy(Reference, Predicate),
    /// The referenced attachment is LEGALLY attached ([CR#701.3b,303.4d]): it
    /// has a host AND that (attachment, host) pair passes the attachment
    /// legality predicate (host-type / protection / `Cant(Attach)`). False when
    /// the object is unattached, attached to an illegal host, or self-attached
    /// — the exact "or is not attached / attached to an illegal object" trigger
    /// of the Aura graveyard SBA ([CR#704.5m]), which the Aura subtype confers
    /// as `StateBased(Not(LegallyAttached(Ref(This))), Move(Ref(This),
    /// Graveyard))`. The
    /// engine evaluates it through the one `attachment_legal` predicate (the
    /// same one the [CR#701.3b] attach no-op uses), never by subtype.
    LegallyAttached(Reference),
    /// An event happened within a window (morbid/raid, [CR#608.2i]).
    ///
    /// `event` is boxed: unboxed it made `Happened` a ~359 B outlier that set
    /// `Condition`'s whole size; the box drops the variant to ~16 B. serde
    /// treats `Arc<T>` transparently, so the RON reads flat (`Happened(event:
    /// …, within: …)`) unchanged — build one via [`Condition::happened`]
    /// rather than the struct literal. See `engine-event-size-boxing`.
    Happened {
        event: Arc<EventFilter>,
        within: Lookback,
    },
    /// The watched `value` was below a threshold before the triggering
    /// occurrence and became at least it after — [CR#714.2b]'s "the number of
    /// lore counters on it was less than N and became at least N", verbatim: a
    /// chapter ability's intervening-if. `value` NAMES the crossing quantity
    /// (a saga's lore total, `CounterCount(This, LoreCounter)`) for the
    /// checker to tie to the trigger's counter event; the engine reads the
    /// fired FACT's `before`/`after` channel, never a recomputation — a
    /// doubled 0→2 placement is one fact whose crossing both chapter I and
    /// chapter II observe.
    ///
    /// `thresholds` holds ONE OR MORE chapter numbers: a range
    /// `{rN1}, {rN2}—[Effect]` ([CR#714.2c] — the same effect at each listed
    /// chapter) is one condition whose crossing fires if the fact carried the
    /// count across ANY of them. Each threshold is a plain [`Count`] — crossing
    /// is inherently "became AT LEAST N" ([CR#714.2b]), so no comparator is
    /// stored. (Per-crossed-member firing when a single doubled batch crosses
    /// two members of one range at once is engine-sagas follow-up; normal
    /// one-at-a-time play fires each member on its own placement event.)
    Crossed {
        value: Count,
        thresholds: Arc<[Count]>,
    },
    /// The tagged optional cost ([`OptionalCost`](crate::OptionalCost)) was
    /// paid for this object — "if it was kicked" ([CR#702.33d]; buyback's
    /// "if the buyback cost was paid", [CR#702.27a]). The linked-ability read
    /// ([CR#702.33e,607.2]): the tag names the declaring `CostOption` on the
    /// same object. Replaces the bespoke `WasKicked` flag.
    PaidCost(crate::CostTag),
    /// "if its [keyword] cost was paid" — the alt-cost read, gating a rider
    /// on the alt base cost used to cast the resolving/source object
    /// ([CR#702.34a,702.74a]). The alt-cost twin of `PaidCost` (which reads
    /// optional/additional costs). Idris `WasCastWith`.
    CastWith(crate::CostTag),
    /// It is the evaluating player's turn (the `you` of the evaluation
    /// context — an ability's controller). Sugar for `TurnOf(Ref(You))`, kept
    /// as the common, frame-robust specialization.
    YourTurn,
    /// It is a matching player's turn ([CR#603.4]) — the active player
    /// satisfies the player predicate. Generalizes
    /// [`YourTurn`](Condition::YourTurn) (which is `TurnOf(Ref(You))`):
    /// "during an opponent's turn" is `TurnOf(OpponentOf(Ref(You)))`. The
    /// `Predicate` is a player predicate (`Ref(You)`, `OpponentOf(Ref(You))`,
    /// `Kind(Player)`, …).
    TurnOf(Predicate),
    /// The current phase/step is exactly the given one. Main phases are
    /// single-step bare variants, so `DuringPhase(PrecombatMain)` works
    /// today; phase-class matching (any combat step) accretes when a card
    /// needs it.
    DuringPhase(crate::PhaseStep),
    /// All sub-conditions hold.
    And(Arc<[Condition]>),
    /// At least one sub-condition holds.
    Or(Arc<[Condition]>),
    /// The sub-condition does not hold.
    Not(Arc<Condition>),
}

impl Condition {
    /// Construct [`Condition::Happened`], boxing the fat `event` field, so call
    /// sites build `Condition::happened(event, within)` rather than boxing by
    /// hand — the counterpart of [`Ability`](crate::Ability)'s boxing
    /// constructors.
    #[must_use]
    pub fn happened(event: EventFilter, within: Lookback) -> Self {
        Condition::Happened {
            event: Arc::new(event),
            within,
        }
    }
}
