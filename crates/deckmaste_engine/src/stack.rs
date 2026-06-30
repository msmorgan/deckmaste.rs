//! The stack ([CR#405]) and the single in-flight announce slot ([CR#601.2] /
//! [CR#602.2]). The stack holds spells, triggered abilities, and activated
//! abilities; the announce slot serves casts ([CR#601.2]) and activations
//! ([CR#602.2]).

use deckmaste_core::CostComponent;
use deckmaste_core::ManaCost;
use deckmaste_core::Zone;

use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::trigger::EventPatient;
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

/// Cardinality of a binder/anaphor slot ([CR#608.2]) — mirrors the Idris
/// `Cardinality`. A one-binder (`TheRef`/`ChooseOne`) binds `One`, read as the
/// singular [`Reference::That`](deckmaste_core::Reference::That); a many-binder
/// (`Choose`/`Existing`) binds `Many`, read as the group
/// [`Selection::That`](deckmaste_core::Selection::That). Keeping the
/// cardinality on the binding makes the first-of-many read structurally
/// impossible: a singular read of a `Many` slot is an error, never `.first()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    /// A single bound element.
    One,
    /// A bound group of elements.
    Many,
}

/// The element kind of a bound slot — object vs. player ([CR#120.3]). Mirrors
/// the engine-relevant arms of the Idris `RefKind` (`AnObject`/`APlayer`); the
/// engine needs only the object/player split (a player proxy is zoneless and
/// carries no LKI snapshot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefKind {
    /// A card/token element.
    Object,
    /// A player element (resolves via the player's proxy object).
    Player,
}

/// A bound iteration/projection element — the [`Reference::It`] anaphor's value
/// ([CR#608.2]). Kind-poly ([CR#120.3]): a card/token element carries its LKI
/// snapshot so reads survive its removal (an `Each(creatures, Destroy(It))`
/// element read after destruction), while a player element is zoneless and
/// carries only its id.
///
/// [`Reference::It`]: deckmaste_core::Reference::It
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItBinding {
    /// A card/token element: its LKI snapshot (id + last-known counters/state).
    Object(LkiSnapshot),
    /// A player element: resolves via the player's proxy object.
    Player(PlayerId),
}

impl ItBinding {
    /// This element's [`RefKind`].
    #[must_use]
    pub fn kind(&self) -> RefKind {
        match self {
            ItBinding::Object(_) => RefKind::Object,
            ItBinding::Player(_) => RefKind::Player,
        }
    }
}

/// The group bound by an enclosing [`Effect::With`](deckmaste_core::With) /
/// [`Each`](deckmaste_core::Each) /
/// [`DivideAmong`](deckmaste_core::DivideAmong) many-binder — the
/// [`Reference::That`]/[`Selection::That`] anaphor's value, carrying per-slot
/// **kind + cardinality** (the Idris `thatKind : Maybe (Cardinality,
/// RefKind)`). The `group` holds the bound ids, order-preserved (top→down for a
/// library window); a one-binder stores its single element as the sole member.
/// Reads resolve by slot: [`Reference::That`] requires a `(One, k)` binding
/// (returns the single id), [`Selection::That`] requires a `(Many, k)` binding
/// (returns the group) — a singular read of a `Many` binding is an error,
/// making the dropped-cardinality first-of-many bug unrepresentable.
///
/// [`Reference::That`]: deckmaste_core::Reference::That
/// [`Selection::That`]: deckmaste_core::Selection::That
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThatBinding {
    /// One (a one-binder) vs. Many (a many-binder group).
    pub cardinality: Cardinality,
    /// The bound elements' kind ([CR#120.3]).
    pub kind: RefKind,
    /// The bound ids, order-preserved; a one-binder is a singleton.
    pub group: Vec<ObjectId>,
}

/// The text-internal (**endophoric**) binding environment a resolving effect
/// reads ([CR#608.2]) — every referent an OPERATOR introduces and binds for a
/// sub-scope, in either direction (anaphora "choose a creature; destroy *it*"
/// AND cataphora "deal 2 damage to *each creature*"). Mirrors the single Idris
/// `Bindings` record. Folds together what `engine-anaphor-threading` landed as
/// separate `Frame` fields: the announce/event state (`targets`, `chosen`,
/// `x`), the per-slot anaphor bindings threaded by `With`/`Each`/`DivideAmong`
/// (the `It` iteration/projection element, the `That` one/many group, the
/// `DivideAmong` per-element allotment share), and the firing event's
/// provenance-explicit roles (`that_object`/`that_player`/`that_patient`).
///
/// Exophoric refs — `This`/`~`, `You`, `Opponent`, `DefendingPlayer` — name the
/// game *situation*, are never bound by an operator, and so live on the
/// [`Frame`] OUTSIDE this record.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Endophora {
    /// Chosen at announce ([CR#601.2c]) or trigger placement ([CR#603.3d]);
    /// read by `Reference::Target(n)`.
    pub targets: Vec<ObjectId>,
    /// A `Choose`/`Random` selection resolved into this scope for a re-run
    /// ([CR#608.2d]). Set only on the continuation the choice produces;
    /// `eval_selection_set` reads it for the `Choose`/`Random` slot. `None`
    /// on a fresh scope.
    pub chosen: Option<Vec<ObjectId>>,
    /// [CR#107.3a]: the announced X for the resolving object — read by
    /// `Count::X`. `None` for triggers and non-X spells.
    pub x: Option<deckmaste_core::Uint>,
    /// The current iteration / projection element — the `It` anaphor
    /// ([CR#608.2]). Bound per element by an enclosing `Each`/`DivideAmong`
    /// loop, and by `Filter::Where` / `Selection::Pick` while they test a
    /// candidate (the role the old `Subject` named). `None` at every frameless
    /// position. Mirrors the Idris `itKind` + its `It` value.
    pub it: Option<ItBinding>,
    /// The group bound by an enclosing `Effect::With`/cost `With` many-binder,
    /// carrying cardinality + kind so the singular `Reference::That` and the
    /// group `Selection::That` resolve by slot (the Idris `thatKind`). `None`
    /// outside a `With`. Replaces the old untyped `those` whose dropped
    /// cardinality caused the first-of-many bug.
    pub that: Option<ThatBinding>,
    /// The per-element share in scope inside a `DivideAmong` body — read by
    /// `Count::Allotment` ([CR#601.2d]). Set per element when `DivideAmong`
    /// binds its loop element (the Idris `bindAllot`), and CLEARED whenever an
    /// inner `Each`/`DivideAmong` rebinds `It` (the Idris allotment-clearing
    /// `bindIt`), so an outer share can never leak into a nested loop. `None`
    /// outside a `DivideAmong` body.
    pub allotment: Option<deckmaste_core::Uint>,
    /// The event OBJECT ([CR#603.2e,608.2k]) — the moved/acting object a fired
    /// trigger carried, or the object an `AdditionalCost` payment bound. Read
    /// by `Reference::EventObject`.
    pub that_object: Option<LkiSnapshot>,
    /// The event ACTOR ([CR#608.2k]) — the responsible player ("that player").
    /// Read by `Reference::EventActor`.
    pub that_player: Option<PlayerId>,
    /// The event PATIENT ([CR#608.2k,120.3]) — the acted-upon thing, kind-poly
    /// (object or player). Read by `Reference::EventPatient`.
    pub that_patient: Option<EventPatient>,
}

impl Endophora {
    /// An empty binding environment: no targets, no anaphor bound, no event
    /// role. The starting point every fresh `Frame` builds from (combine with
    /// functional-record-update — `Endophora { targets, ..Endophora::empty()
    /// }`).
    #[must_use]
    pub const fn empty() -> Self {
        Endophora {
            targets: Vec::new(),
            chosen: None,
            x: None,
            it: None,
            that: None,
            allotment: None,
            that_object: None,
            that_player: None,
            that_patient: None,
        }
    }
}

/// The binding environment a resolving effect reads ([CR#608.2]), split along
/// the endophora/exophora line. The always-available **exophoric** refs that
/// name the game *situation* stay here — `source`/`controller` (read by
/// `This`/`You`/`Opponent`), the firing object's last-known self (`this`), and
/// the combat `defending_player` — while everything an operator binds for a
/// sub-scope lives in the nested [`Endophora`] record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub source: ObjectId,
    pub controller: PlayerId,
    /// The firing object's last-known self (`~`/`This`/source), exophoric —
    /// `None` is a spell frame (`Reference::This` reads the live `source`),
    /// `Some` a trigger/activated frame (`This` reads this snapshot, which
    /// survives the source's removal — [CR#603.10a]).
    pub this: Option<LkiSnapshot>,
    /// The combat DEFENDING player ([CR#506.2,508.5]) — always a player,
    /// exophoric. Read by `Reference::DefendingPlayer`; `None` outside combat.
    pub defending_player: Option<PlayerId>,
    /// The text-internal binding environment — targets, the `It`/`That`
    /// anaphors, the `DivideAmong` allotment, the chosen value, X, and the
    /// firing event's roles. See [`Endophora`].
    pub endophora: Endophora,
}

impl Frame {
    /// A bare resolution frame: the exophoric `source`/`controller`, no trigger
    /// snapshot (a spell frame — `Reference::This` reads the live `source`), no
    /// combat defender, and an empty [`Endophora`]. The common starting shape
    /// for gate/payability/instant frames; add targets, X, or an anaphor by
    /// populating `.endophora`.
    #[must_use]
    pub fn bare(source: ObjectId, controller: PlayerId) -> Self {
        Frame {
            source,
            controller,
            this: None,
            defending_player: None,
            endophora: Endophora::empty(),
        }
    }
}
