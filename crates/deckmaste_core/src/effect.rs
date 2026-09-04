use std::fmt;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::ChooseSpec;
use crate::Condition;
use crate::Count;
use crate::Mode;
use crate::ability::TriggeredAbility;
use crate::action::Action;
use crate::continuous::Duration;
use crate::continuous::StaticSpec;
use crate::reference::Reference;

/// Executable semantic syntax — the work a resolving spell or ability is told
/// to perform ([CR#608]). An `Instruction` is NOT an effect: "text itself is
/// never an effect" ([CR#609.1]). Executing one is what produces the one-shot
/// effects ([CR#610.1]) and, through
/// [`Continuously`](Instruction::Continuously)/[`Until`](Instruction::Until),
/// the continuous effects ([CR#611.2]) the game records. Core RON keeps the
/// `Act` wrapper explicit around an [`Action`]
/// (`effect: Act(DealDamage(This, 3, It))`). The structural forms
/// (`Sequentially`, `May`, `If`, …) are the corpus's connective tissue
/// interpreted by the engine. Semantic lowering preserves these forms and
/// wraps authored action shorthand in `Act`.
// No `large_enum_variant` suppression: the top variants cluster within clippy's
// threshold (`Distribute` ~568 B, `Act`/`Each` ~488 B, `SeparatePiles`
// ~416 B — a spread under 200 B), so the lint does not fire. `Act(Action)` is
// kept inline deliberately: `Action` is a big *balanced* leaf enum (no single fat
// field to box), and `Instruction` is the hot, recursively-matched node of the
// effect grammar — boxing `Act` would inject a deref into every match on the
// `resolve` hot path for no lint gain. The recursive sub-effect fields are
// already boxed (`May.effect`, …).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    /// A single primitive instruction (the `Act` compartment, transparent in
    /// RON).
    Act {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        dest: Option<crate::DefId>,
        action: Action,
    },
    /// A resolution-time object choice. The selected group is written to
    /// `dest` before the next instruction runs.
    Choose(Choose),
    /// A resolution-time scalar/symbolic choice written directly to `dest`.
    ChooseValue(ChooseValue),
    /// A hidden-zone search whose found group is written directly to `dest`.
    Search(Search),
    /// Pin a pure expression at this program point.
    Let(Let),
    /// Write one of the card's linked memory cells ([CR#607.1]) — the writer
    /// half of a linked pair. `value` is a register in THIS region; `cell`
    /// names the memory the card's other abilities read through a
    /// [`Provenance::Linked`](crate::Provenance::Linked) parameter, and
    /// `kind` is that cell's declared runtime shape. The instruction IS the
    /// declaration: a card's memory cells are exactly the cells its
    /// `Remember`s write, which is what makes a reader with no writer a
    /// load-time refusal rather than [CR#607.5a]'s runtime-undefined (that
    /// rule covers a GAINED reader whose linked writer was not copied, never
    /// two abilities printed on one object).
    Remember(Remember),
    /// Explicit "then" — ordered sub-effects ([CR#608.2c]).
    Sequentially(Arc<[Instruction]>),
    /// Simultaneously sub-effects — the written spec. **One snapshot:** every
    /// member reads game state as of one pre-application view (an exchange
    /// works BECAUSE both halves read the pre-state). **One timestamp, one
    /// batch:** application emits the member facts as one batch sharing a
    /// batch id — ONE occurrence for `OneOrMore` triggers ([CR#603.2c]).
    /// **Replacements apply per member fact independently** (each event is
    /// replaceable on its own, ordering per the affected object/player's
    /// controller as usual, [CR#616.1]); replacing one member does not
    /// unapply the others. **SBAs run after the whole batch**, never
    /// between members. **All-or-nothing:** a member that evaluates to no
    /// events voids the whole set ([CR#701.12a] — "if the entire exchange
    /// can't be completed, no part of the exchange occurs"). `Fight` is NOT
    /// `Simultaneously` sugar — it stays a primitive verb
    /// ([CR#701.14a..701.14d]). Restricted to the exchange-family macros'
    /// bodies until the wiring generalizes.
    Simultaneously(Arc<[Instruction]>),
    /// A one-shot-created continuous effect ([CR#611.2]).
    Continuously(Continuously),
    /// A one-shot-created continuous effect over a LIST of static parts —
    /// `Until(EndOfTurn, [Modify(…), Deontic(…)])` ([CR#611.2]). Fixed-vs-
    /// live affected sets are PER PART ([CR#611.2c]): characteristic-/
    /// controller-modifying parts gather their affected set once at start;
    /// deontic/prevention/replacement/cost parts stay live — the class
    /// follows each part's kind.
    /// (`Continuously` is the single-part spelling; `Static` ability
    /// position stays live re-gathering, [CR#611.3a].)
    Until(Duration, Arc<[StaticSpec]>),
    /// `SeparatePiles { dests, group, by, then }` — `by` separates `group`
    /// into the pile registers in `dests` ([CR#700.3a]; piles may be empty).
    SeparatePiles(SeparatePiles),
    /// `ChoosePile { dest, from, by, random, then }` — `by` picks one of the
    /// pile registers in `from` ([CR#700.3b]), writes it to `dest`, and runs
    /// `then` with that register in scope.
    ChoosePile(ChoosePile),
    /// "You may [do]" ([CR#603,608]) — with "if you do"/"if you don't". The
    /// may-pay/must-pay family collapses into this node ([CR#118.12a]):
    /// `effect: Act(Pay(cost))` plus `if_did`/`if_not` is the resolution-time
    /// kicker/punisher over the full [`Cost`] algebra. The English "[do]
    /// unless [who] pays [cost]" order is the builtin `Unless` MACRO — a
    /// render name over this node; core keeps only the `May` form.
    May(May),
    /// "If [condition], [then]; otherwise [else]" ([CR#603.4]-style branch).
    If(If),
    /// "For each [element of `over`], [do]" — enters `body` once per element,
    /// supplying its `LoopElement` parameter ([CR#608]).
    Each(Each),
    /// Divide an `amount` among the elements of `over` "as you choose",
    /// entering `body` with `LoopElement` and `Allotment` parameters for each
    /// element ([CR#601.2d]). The split is
    /// resolution-time (≥1 each, summing to `amount`). "Divide" and
    /// "distribute" are ONE mechanic, not two — [CR#115.7f] quotes both words
    /// for the same effect — so one primitive subsumes divided damage
    /// (`body: DealDamage(This, Allotment, It)`) AND distributed counters
    /// (`body: PutCounters(It, <kind>, Allotment)`); the body reads the
    /// allotment parameter. Named for the Idris north-star `Distribute : Count
    /// b -> Bindable b Many k -> …`, the general divide-or-distribute
    /// primitive.
    Distribute(Distribute),
    /// A delayed triggered ability created on resolution ([CR#603.7]).
    /// Note the object set the inner effect moves/touches under `key`
    /// ([CR#607.2a] exiled-with linkage).
    Delayed(Arc<TriggeredAbility>),
    /// A reflexive triggered ability created on resolution ([CR#603.12]).
    Reflexive(Arc<TriggeredAbility>),
    /// A modal effect: choose modes, then apply them ([CR#700.2]).
    Modal(Modal),
    /// "[body], [count] times": resolution follows the general
    /// spell/ability resolution walk ([CR#608.2]) — there is no dedicated CR
    /// rule defining a "do N times" quantifier; this is engine-side
    /// shorthand, sibling to [`Each`](Instruction::Each)/
    /// [`Distribute`](Instruction::Distribute) rather than the
    /// manner-adverb family
    /// ([`Simultaneously`](Instruction::Simultaneously)/
    /// [`Continuously`](Instruction::Continuously)): a `Count` over ONE
    /// body, not a manner over a list. `body` elaborates in the SAME
    /// context every iteration — no iteration-index binder; Storm/Replicate
    /// per-iteration semantics are engine-side. `body` is boxed to break
    /// the `Instruction` → `Repeat` → `Instruction` size cycle. Mirrors
    /// the Idris `Repeat : (count : Count b) -> Instruction
    /// b -> Instruction b`.
    Repeat(Count, Arc<Instruction>),
    /// `Repeat`'s BATCHING twin ([CR#616.1g]): performs `body` `count` times
    /// as ONE aggregate containing event — the count tier where "twice that
    /// many" replacements bite ([CR#121.2a,616.1g]) and aggregate triggers
    /// read. THIS shape is the shell only: it currently resolves purely
    /// sequentially, identically to [`Repeat`](Instruction::Repeat) — a
    /// later pass rebuilds it into the true aggregate-count tier. Boxed for
    /// the same `Instruction` → `Batch` → `Instruction` size-cycle
    /// reason as `Repeat`. Mirrors the Idris `Batch : (count : Count b) ->
    /// Instruction b -> Instruction b`.
    Batch(Count, Arc<Instruction>),
    /// The variable-length DIG-UNTIL ([CR#702.85] cascade, [CR#701.57]
    /// discover): `whose` reveals cards off the top of their library one at a
    /// time until one matches `matches`; `body` then runs with the found card
    /// and passed-over prefix supplied through explicit body parameters — the
    /// Idris `bindFound` binding. Reveal-nothing (no match in the
    /// library) degrades at runtime, never a compile-time gate. Mirrors the
    /// Idris `RevealUntil : (whose : Reference b APlayer) -> (match :
    /// Predicate b AnObject) -> Instruction (bindFound match b) ->
    /// Instruction b`.
    RevealUntil(RevealUntil),
}

impl Instruction {
    /// "`who` mills `count`" ([CR#701.17a]) — a slice-family keyword action:
    /// [`Batch`](Instruction::Batch) over the per-unit [`Action::mill_one`],
    /// so a count-doubling replacement (Bruvac, [CR#121.2a,616.1g]) bites the
    /// ONE aggregate window and the whole top slice commits as ONE simultaneous
    /// batch of per-card, individually-redirectable moves. The
    /// [`Mill`](../plugins)/`Mills` macros expand to this.
    #[must_use]
    pub fn mill(who: crate::Reference, count: Count) -> Instruction {
        Instruction::Batch(count, Arc::new(Instruction::act(Action::mill_one(who))))
    }

    /// "`who` draws `count`" ([CR#121.1]) — a slice-family keyword action:
    /// [`Batch`](Instruction::Batch) over the per-unit [`Action::draw_one`],
    /// `count` SEQUENTIAL single-card draws ([CR#121.2], each seeing prior
    /// state and empty-checking BEFORE its move). The `Draw`/`Draws` macros
    /// expand to this.
    #[must_use]
    pub fn draw(who: crate::Reference, count: Count) -> Instruction {
        Instruction::Batch(count, Arc::new(Instruction::act(Action::draw_one(who))))
    }

    #[must_use]
    pub fn act(action: Action) -> Self {
        Self::Act { dest: None, action }
    }

    #[must_use]
    pub fn producing(dest: crate::DefId, action: Action) -> Self {
        Self::Act {
            dest: Some(dest),
            action,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// `Continuously { effect, duration }` ([CR#611.2]). `effect` is boxed to break
/// the `Instruction` → `StaticSpec` → `Replacement` → `Instruction` size
/// cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Continuously {
    pub effect: Arc<StaticSpec>,
    pub duration: Duration,
}

/// `May { who, do, if_did, if_not }` — `do` is a keyword, so the field is
/// `effect`. `who` is the decider ([CR#608.2d]; Browbeat's decider ≠
/// performer — "Any player may have Browbeat deal 5 damage to them" — proves
/// the slot is not derivable, Law 3), spelling today's implicit "you may"
/// decider (no read-time default, Law 2). Branch semantics: when `effect` is
/// `Act(Pay(cost))`, `if_did`/`if_not` invoke [CR#118.12] cost semantics — the
/// collapsed `MayPay`/`MustPay` shape (the doer decides whether they PAID,
/// "regardless of what events actually occurred", the Dermoplasm clause).
/// Any other `effect` takes the plain [CR#608.2]-family branch (yes runs
/// `effect` then `if_did`; no runs `if_not`). A branchless `May` (`if_did`/
/// `if_not` both `None`) is the same node either way — one node serves both.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct May {
    pub who: Reference,
    pub effect: Arc<Instruction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_did: Option<Arc<Instruction>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_not: Option<Arc<Instruction>>,
}

/// `If { condition, then, else }` — `else` is a keyword, so the field is
/// `otherwise`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct If {
    pub condition: Condition,
    pub then: Arc<Instruction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otherwise: Option<Arc<Instruction>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Choose {
    pub dest: crate::DefId,
    pub by: Reference,
    pub quantity: crate::Quantity,
    pub filter: Arc<crate::Region<crate::Predicate>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ChooseValue {
    pub dest: crate::DefId,
    pub by: Reference,
    pub domain: crate::ChosenValueKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Search {
    pub dest: crate::DefId,
    pub by: Reference,
    pub whose: Reference,
    pub from: Arc<[crate::Zone]>,
    pub quantity: crate::Quantity,
    pub filter: Arc<crate::Region<crate::Predicate>>,
    #[serde(default)]
    pub if_none: crate::Block,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Let {
    pub dest: crate::DefId,
    pub expr: crate::Expr,
}

/// `Remember { cell, kind, value }` — see [`Instruction::Remember`]
/// ([CR#607.1]). Writes the register `value` into the card's memory cell
/// `cell`; a reading ability on the same card declares that cell as a
/// [`Provenance::Linked`](crate::Provenance::Linked) parameter of its own
/// region. Unlike a [`Let`], this defines nothing in the writing region: the
/// value it publishes crosses to another ability, never to a later
/// instruction here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Remember {
    pub cell: crate::Ident,
    pub kind: crate::Kind,
    pub value: crate::RefId,
}

/// `Each { over, body }` enters the body region once per selected element.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Each {
    pub over: crate::Selection,
    pub body: crate::Region,
}

/// `Distribute { amount, over, body }` enters the body region with each
/// recipient and its allotted share as parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Distribute {
    pub amount: crate::Count,
    pub over: crate::Selection,
    pub body: crate::Region,
}

/// `RevealUntil { whose, matches, body }` — see
/// [`Instruction::RevealUntil`]. `whose`'s library is revealed from the top
/// one card at a time until one satisfies `matches`; `body` receives the
/// found card and passed-over prefix through explicit parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RevealUntil {
    pub found: crate::DefId,
    pub passed: crate::DefId,
    pub whose: crate::Reference,
    pub matches: Arc<crate::Region<crate::Predicate>>,
    pub body: crate::Region,
}

/// `Modal { choose, modes }` ([CR#700.2]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Modal {
    pub choose: ChooseSpec,
    pub modes: Arc<[Mode]>,
}

/// `SeparatePiles { dests, group, by, then }` — see
/// [`Instruction::SeparatePiles`]. Each destination is one pile register;
/// labels exist only in the authored semantics and are erased by lowering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SeparatePiles {
    pub dests: Arc<[crate::DefId]>,
    pub group: crate::Selection,
    pub by: Reference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub then: Option<Arc<Instruction>>,
}

/// `ChoosePile { dest, from, by, random, then }` — see
/// [`Instruction::ChoosePile`]. `from` contains only pile-register reads;
/// the chosen pile is written to `dest`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ChoosePile {
    /// The chosen pile, available to the nested body and later instructions.
    pub dest: crate::DefId,
    pub from: Arc<[crate::RefId]>,
    pub by: Reference,
    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub random: bool,
    pub then: Arc<Instruction>,
}
