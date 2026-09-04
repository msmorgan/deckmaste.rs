//! The stack ([CR#405]) and the single in-flight announce slot ([CR#601.2] /
//! [CR#602.2]). The stack holds spells, triggered abilities, and activated
//! abilities; the announce slot serves casts ([CR#601.2]) and activations
//! ([CR#602.2]).

use std::sync::Arc;

use deckmaste_core::CostComponent;
use deckmaste_core::ManaCost;
use deckmaste_core::Zone;

use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::trigger::TriggerBindings;

/// What sits on (or is going onto) the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObject {
    /// A card moved to the stack and cast ([CR#601.2a]).
    Spell(ObjectId),
    /// A triggered ability on the stack ([CR#603.3]). It has no card identity
    /// of its own — its `StackEntry.id` is a freshly minted token — and carries
    /// the firing object's last-known information in `bindings`.
    ///
    /// `ability` indexes the source's printed abilities for an ordinary
    /// ([CR#603.2]) trigger; `created` overrides it with a by-value body for a
    /// delayed/reflexive ([CR#603.7,603.12]) trigger, which is printed on no
    /// permanent, so its text is carried like an `Activated` ability's rather
    /// than re-derived from the source.
    Triggered {
        source: ObjectSource,
        ability: usize,
        /// `Some` for a delayed/reflexive trigger created at resolution — its
        /// authoritative body (`ability` is then a placeholder). `None` for a
        /// printed trigger, read via `abilities_of_source(source)[ability]`.
        created: Option<Arc<deckmaste_core::TriggeredAbility>>,
        bindings: TriggerBindings,
    },
    /// An activated ability on the stack ([CR#602.2a]). Carries the ability's
    /// text — "It has the text of the ability that created it" — so resolution
    /// never re-derives from the (possibly gone, possibly changed) source.
    /// `bindings.this` is the source's announce-time snapshot; `~` reads it
    /// like a trigger's LKI.
    Activated {
        source: ObjectId,
        ability: Box<deckmaste_core::ActivatedAbility>,
        bindings: TriggerBindings,
    },
}

impl StackObject {
    /// The object a *spell* entry is "on" — the spell's id. Used by the
    /// permanent-spell / fizzle paths in `resolve_object`. A triggered or
    /// activated ability has no such object (it is identified on the stack by
    /// its `StackEntry.id`).
    ///
    /// # Panics
    ///
    /// Panics on a `Triggered` or `Activated` entry — those are keyed by
    /// `StackEntry.id`, not by a backing object.
    #[must_use]
    pub fn object(&self) -> ObjectId {
        match self {
            StackObject::Spell(o) => *o,
            StackObject::Triggered { .. } | StackObject::Activated { .. } => {
                unreachable!(
                    "a triggered or activated ability has no backing object id; key on StackEntry.id"
                )
            }
        }
    }
}

/// A committed stack object: resolvable, and (stage 3) scanned by triggers and
/// SBAs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackEntry {
    /// The stack identity ([CR#405]). For a spell it is the spell's own object
    /// id; for a triggered ability it is a freshly minted token (the ability
    /// has no card identity). `Resolve` keys on this.
    pub id: ObjectId,
    /// Register file allocated when this spell or ability starts announcing.
    pub activation: crate::ActivationId,
    pub object: StackObject,
    pub controller: PlayerId,
    /// Chosen at announce ([CR#601.2c]) or at trigger placement ([CR#603.3d]);
    /// read back by the slot-bound anaphors. One inner set per `TargetSpec`
    /// slot (singleton for a quantity-one slot, several for a plural slot).
    pub targets: Vec<Vec<ObjectId>>,
    /// [CR#601.2b,700.2]: the modal choices locked during announcement, in
    /// the order chosen. Empty for a nonmodal object or a modal "up to"
    /// choice that selected no modes.
    pub chosen_modes: Arc<[deckmaste_core::Uint]>,
    /// [CR#107.3a]: the announced X — copied from the announce slot at promote.
    /// `None` for triggers and non-X spells.
    pub x: Option<deckmaste_core::Uint>,
    /// [CR#601.2b,702.33d]: which tagged optional costs were announced paid,
    /// with multiplicity (multikicker pays a tag several times,
    /// [CR#702.33c]) — the record `Condition::PaidCost` / `Count::TimesPaid`
    /// read while this entry resolves ([CR#607.2] linked reads). Copied from
    /// the announce slot at promote; empty for triggers.
    pub paid_costs: Vec<(deckmaste_core::CostTag, deckmaste_core::Uint)>,
    /// [CR#707.10]: this entry is a COPY — no card behind it; it vanishes
    /// instead of moving zones when it leaves the stack ([CR#707.10a]).
    /// Otherwise a full citizen: "a copy of a spell is itself a spell".
    pub copy: bool,
}

/// An announce in flight ([CR#601.2] / [CR#602.2]). At most one exists, ever
/// (no priority is held during the procedure). Carries scratch a committed
/// entry never has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingStackEntry {
    /// The stack identity the announce commits under ([CR#405]): a spell's
    /// own object id, or an activated ability's identity minted when the
    /// announce opens — the ability exists on the stack from announcement
    /// ([CR#602.2a]), so announce-time deontic `by` rows (including
    /// stack-zone-keyed ones) evaluate against the real id, not a source
    /// stand-in.
    pub id: ObjectId,
    /// Register file shared by announcement and the later resolution.
    pub activation: crate::ActivationId,
    pub object: StackObject,
    pub controller: PlayerId,
    /// Where a spell was cast from — for cast-from-zone effects, not undo;
    /// `Hand` in stage 2.
    pub origin: Zone,
    /// One inner set per `TargetSpec` slot (see [`StackEntry::targets`]).
    pub targets: Vec<Vec<ObjectId>>,
    /// [CR#601.2b,700.2]: modal choices locked before X, targets, and payment.
    /// Empty until `AnnounceModes` completes, and for nonmodal objects.
    pub chosen_modes: Arc<[deckmaste_core::Uint]>,
    /// [CR#601.2b,107.3a]: the value announced for `{X}` in the cost, or `None`
    /// when the cost has no `{X}`. Chosen at the `AnnounceX` step.
    pub x: Option<deckmaste_core::Uint>,
    /// [CR#601.2b]: the announced concretization of the printed cost — its
    /// hybrid/Phyrexian symbols resolved to a `Simple`-only `ManaCost` plus the
    /// verb costs the Phyrexian-life picks incur ([CR#107.4f]). Set by
    /// `ChooseCostOptions` (always — directly for a plain cost, via the
    /// player's answer otherwise); `None` only between `begin_cast`/
    /// `begin_activate` and that step. `PayCost` reads it for the mana decision
    /// and the extra verbs.
    pub concretized: Option<(ManaCost, Vec<CostComponent>)>,
    /// [CR#601.2b,702.33d]: the tagged optional costs announced paid so far
    /// (tag → times), filled by the `AnnounceOptionalCosts` step. Promoted
    /// onto the committed entry for the linked reads.
    pub paid_costs: Vec<(deckmaste_core::CostTag, deckmaste_core::Uint)>,
    /// [CR#601.2f]: the cost components those announcements ADD to the total
    /// cost — folded into the payment demand by `PayCost` (a kicked spell's
    /// `{2}` joins the mana decision; a `Do(...)` kicker joins the verb
    /// window).
    pub optional_components: Vec<CostComponent>,
    /// [CR#118.9,702.35a]: an ALTERNATIVE base cost this cast pays RATHER THAN
    /// the card's mana cost — a resolution-time `Cast(what, [cost])` (madness's
    /// madness cost). `None` (the common case) leaves the printed mana cost as
    /// the base. `ChooseCostOptions` reads it in place of the printed cost.
    pub alternative_cost: Option<deckmaste_core::Cost>,
}

/// A cost payment in progress ([CR#118.10]): stamped on every
/// [`ExecutionFrame`] the payment's own drain runs against (each cost-eligible verb,
/// each toll component), never on the frame of what comes AFTER a payment
/// (`if_did`/`if_not`) — those name the CONSEQUENCE, not the payment. `id` is a
/// fresh monotonic id minted once per
/// payment ([`crate::state::GameState::mint_payment`]) and shared by every
/// `ExecutionFrame` in that one payment's drain — the representation [CR#118.10]
/// needs to be checkable ("a payment... applies to only one spell, ability,
/// or effect"; no event belongs to two payments), though nothing reads it
/// back yet ([CR#118.10] correction — see the T4 brief). Presence alone
/// already answers the AGENCY question (`Cause::*` construction sites read
/// `frame.payment.is_some()` to choose `Agency::CostPayment`); `id` answers
/// the identity question.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Payment {
    pub id: deckmaste_core::Uint,
}

/// The lightweight execution cursor carried by agenda work. Binding values
/// live in the activation table; cloning a frame never clones the region's
/// register file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionFrame {
    /// The one shared activation record for this region entry.
    pub activation: crate::activation::ActivationId,
    /// `Some` iff this frame is running as part of a cost payment's drain
    /// ([CR#118.10]) — see [`Payment`]. `None` keeps every existing `Cause::*` construction exactly
    /// as `Agency::EffectInstruction`.
    pub payment: Option<Payment>,
}

impl ExecutionFrame {
    pub(crate) fn source(&self, state: &crate::state::GameState) -> ObjectId {
        state.activation_source(self.activation)
    }

    pub(crate) fn controller(&self, state: &crate::state::GameState) -> PlayerId {
        state.activation_controller(self.activation)
    }

    pub(crate) fn source_lki(&self, state: &crate::state::GameState) -> Option<LkiSnapshot> {
        state.activation_source_lki(self.activation)
    }
}
