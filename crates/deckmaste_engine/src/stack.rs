//! The stack ([CR#405]) and the single in-flight announce slot ([CR#601.2] /
//! [CR#602.2]). The stack holds spells, triggered abilities, and activated
//! abilities; the announce slot serves casts ([CR#601.2]) and activations
//! ([CR#602.2]).

use deckmaste_core::CostComponent;
use deckmaste_core::ManaCost;
use deckmaste_core::Zone;

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
    Triggered {
        source: ObjectSource,
        ability: usize,
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
    pub object: StackObject,
    pub controller: PlayerId,
    /// Chosen at announce ([CR#601.2c]) or at trigger placement ([CR#603.3d]);
    /// read by `Reference::Target(n)`.
    pub targets: Vec<ObjectId>,
    /// [CR#107.3a]: the announced X — copied from the announce slot at promote.
    /// `None` for triggers and non-X spells.
    pub x: Option<deckmaste_core::Uint>,
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
    pub object: StackObject,
    pub controller: PlayerId,
    /// Where a spell was cast from — for cast-from-zone effects, not undo;
    /// `Hand` in stage 2.
    pub origin: Zone,
    pub targets: Vec<ObjectId>,
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
}

/// The bindings an effect reads during resolution ([CR#608.2]). Grows `x`,
/// `that_object`, bound roles, etc. in later stages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub source: ObjectId,
    pub controller: PlayerId,
    pub targets: Vec<ObjectId>,
    /// The trigger's last-known information ([CR#608.2]) — `None` for a spell
    /// frame. When present, `Reference::This`/`~` reads the firing object's
    /// snapshot rather than the live `source` (via `eval_reference`).
    pub bindings: Option<TriggerBindings>,
    /// A `Choose`/`Random` selection resolved into this frame for a re-run
    /// ([CR#608.2d]). Set only on the continuation frame the choice produces;
    /// `eval_selection_set` reads it for the `Choose`/`Random` slot. `None`
    /// on a fresh frame.
    pub chosen: Option<Vec<ObjectId>>,
    /// [CR#107.3a]: the announced X for the resolving object — read by
    /// `Count::X`. `None` for triggers and non-X spells.
    pub x: Option<deckmaste_core::Uint>,
    /// The candidate a per-object filter is currently matching — bound only by
    /// `Filter::Where` when it hands a condition to `condition_holds`, read by
    /// `Reference::Subject`. `None` on every other frame (resolution, triggers,
    /// activation): `Subject` is meaningful only inside filter matching.
    pub subject: Option<ObjectId>,
}
