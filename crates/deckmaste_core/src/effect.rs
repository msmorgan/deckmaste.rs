use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::ChooseSpec;
use crate::Condition;
use crate::Cost;
use crate::Count;
use crate::Mode;
use crate::ability::TriggeredAbility;
use crate::action::Action;
use crate::continuous::Duration;
use crate::continuous::StaticEffect;
use crate::reference::Reference;

/// An effect an ability produces ([CR#608]). Core RON keeps the `Act` wrapper
/// explicit around an [`Action`] (`effect: Act(DealDamage(This, 3, It))`). The
/// structural forms (`Sequentially`, `May`, `If`, …) are the corpus's
/// connective tissue interpreted by the engine. Semantic lowering preserves
/// these forms and wraps authored action shorthand in `Act`.
// No `large_enum_variant` suppression: the top variants cluster within clippy's
// threshold (`Distribute` ~568 B, `Act`/`Each`/`With` ~488 B, `SeparatePiles`
// ~416 B — a spread under 200 B), so the lint does not fire. `Act(Action)` is
// kept inline deliberately: `Action` is a big *balanced* leaf enum (no single fat
// field to box), and `OneShotEffect` is the hot, recursively-matched node of the
// effect grammar — boxing `Act` would inject a deref into every match on the
// `resolve` hot path for no lint gain. The recursive sub-effect fields are
// already boxed (`May.effect`, …).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Instr {
    /// A single intrinsic instruction (the `Act` compartment, transparent in
    /// RON).
    Act(Action),
    /// Explicit "then" — ordered sub-effects ([CR#608.2c]).
    Sequentially(Arc<[OneShotEffect]>),
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
    Simultaneously(Arc<[OneShotEffect]>),
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
    Until(Duration, Arc<[StaticEffect]>),
    /// `Label { as, effect }` — names the antecedents the inner effect
    /// introduces so later clauses can read them explicitly, rather than by
    /// the positional/anaphoric defaults ([CR#608.2d]).
    Label(Label),
    /// `SeparatePiles { group, into, by, note, then }` — `by` separates
    /// `group` into labeled piles ([CR#700.3a]; piles may be empty). Each
    /// label becomes a Many antecedent (read as the plural anaphor
    /// [`Selection::They`](crate::Selection::They)); `note:`
    /// persists the piles as noted groups keyed by (note, label, divider),
    /// read back via [`Selection::PilesOf`](crate::Selection::PilesOf).
    SeparatePiles(SeparatePiles),
    /// `ChoosePile { from, by, random, then }` — `by` picks one pile
    /// ([CR#700.3b] — the Fact-or-Fiction shape); `then` runs with the
    /// chosen pile bound as a Many antecedent
    /// ([`Selection::Them`](crate::Selection::Them)`(Pile)`).
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
    /// "As an additional cost, [pay]; then [body]" ([CR#601.2f,118.8]) —
    /// imposes an additional cost whose paid object the body reads through
    /// the event references (`EventObject`/`EventActor`/`EventPatient`):
    /// "sacrifice a creature: ~ deals damage equal to its power" (Fling,
    /// Momentous Fall). The payment is an event, so
    /// [`AdditionalCost::body`] reads the sacrificed/exiled object with the
    /// SAME anaphors a trigger uses. At a spell/ability root the engine
    /// hoists it to cast/activation time (the printed additional cost,
    /// [CR#601.2f]); nested, it is an extra resolution-time cost. Mirrors
    /// the Idris `AdditionalCost (pay : Cost) body`.
    AdditionalCost(AdditionalCost),
    /// "For each [element of `binder`], [do]" — iterates the many-binder
    /// ([`Binder`](crate::Binder), cardinality Many), binding each element in
    /// turn as the iteration anaphor [`Reference::It`](crate::Reference::It),
    /// then runs the body once per element ([CR#608]). Mirrors the Idris
    /// `Each : Bindable b Many k -> …`.
    Each(Each),
    /// `With(binder, body)` — binds what `binder` yields into the frame as the
    /// body's anaphor, then runs `body` once. A one-binder
    /// ([`Binder::TheRef`](crate::Binder::TheRef) /
    /// [`Binder::ChooseOne`](crate::Binder::ChooseOne)) binds a single object
    /// read as [`Reference::That`](crate::Reference::That); a many-binder
    /// ([`Binder::Choose`](crate::Binder::Choose) /
    /// [`Binder::Existing`](crate::Binder::Existing)) binds a group read as
    /// [`Selection::That`](crate::Selection::That). Never distributes (that
    /// is `Each`, which exposes [`Reference::It`](crate::Reference::It) per
    /// element); `This` never rebinds. Mirrors the Idris
    /// `With : Bindable b card k -> …`.
    With(With),
    /// Divide an `amount` among the elements of a many-binder
    /// ([`Binder`](crate::Binder)) "as you choose", binding each element in
    /// turn as the iteration anaphor [`Reference::It`](crate::Reference::It)
    /// with its [`Count::Allotment`](crate::Count::Allotment) share, then
    /// running `body` once per element ([CR#601.2d]). The split is
    /// resolution-time (≥1 each, summing to `amount`). "Divide" and
    /// "distribute" are ONE mechanic, not two — [CR#115.7f] quotes both words
    /// for the same effect — so one primitive subsumes divided damage
    /// (`body: DealDamage(This, Allotment, It)`) AND distributed counters
    /// (`body: PutCounters(It, <kind>, Allotment)`); the body reads the
    /// allotment anaphor. Named for the Idris north-star `Distribute : Count b
    /// -> Bindable b Many k -> …`, the general divide-or-distribute primitive.
    Distribute(Distribute),
    /// A delayed triggered ability created on resolution ([CR#603.7]).
    /// Note the object set the inner effect moves/touches under `key`
    /// ([CR#607.2a] exiled-with linkage).
    Noting(Noting),
    Delayed(Arc<TriggeredAbility>),
    /// A reflexive triggered ability created on resolution ([CR#603.12]).
    Reflexive(Arc<TriggeredAbility>),
    /// A modal effect: choose modes, then apply them ([CR#700.2]).
    Modal(Modal),
    /// "[body], [count] times": resolution follows the general
    /// spell/ability resolution walk ([CR#608.2]) — there is no dedicated CR
    /// rule defining a "do N times" quantifier; this is engine-side
    /// shorthand, sibling to [`Each`](OneShotEffect::Each)/
    /// [`Distribute`](OneShotEffect::Distribute) rather than the
    /// manner-adverb family
    /// ([`Simultaneously`](OneShotEffect::Simultaneously)/
    /// [`Continuously`](OneShotEffect::Continuously)): a `Count` over ONE
    /// body, not a manner over a list. `body` elaborates in the SAME
    /// context every iteration — no iteration-index binder; Storm/Replicate
    /// per-iteration semantics are engine-side. `body` is boxed to break
    /// the `OneShotEffect` → `Repeat` → `OneShotEffect` size cycle. Mirrors
    /// the Idris `Repeat : (count : Count b) -> OneShotEffect
    /// b -> OneShotEffect b`.
    Repeat(Count, Arc<OneShotEffect>),
    /// `Repeat`'s BATCHING twin ([CR#616.1g]): performs `body` `count` times
    /// as ONE aggregate containing event — the count tier where "twice that
    /// many" replacements bite ([CR#121.2a,616.1g]) and aggregate triggers
    /// read. THIS shape is the shell only: it currently resolves purely
    /// sequentially, identically to [`Repeat`](OneShotEffect::Repeat) — a
    /// later pass rebuilds it into the true aggregate-count tier. Boxed for
    /// the same `OneShotEffect` → `Batch` → `OneShotEffect` size-cycle
    /// reason as `Repeat`. Mirrors the Idris `Batch : (count : Count b) ->
    /// OneShotEffect b -> OneShotEffect b`.
    Batch(Count, Arc<OneShotEffect>),
    /// The variable-length DIG-UNTIL ([CR#702.85] cascade, [CR#701.57]
    /// discover): `whose` reveals cards off the top of their library one at a
    /// time until one matches `matches`; `body` then runs with the found card
    /// bound as [`Reference::It`](crate::Reference::It) and the passed-over
    /// prefix bound as the plural anaphor
    /// ([`Selection::They`](crate::Selection::They)) — the Idris
    /// `bindFound` binding. Reveal-nothing (no match in the
    /// library) degrades at runtime, never a compile-time gate. Mirrors the
    /// Idris `RevealUntil : (whose : Reference b APlayer) -> (match :
    /// Predicate b AnObject) -> OneShotEffect (bindFound match b) ->
    /// OneShotEffect b`.
    RevealUntil(RevealUntil),
}

impl Instr {
    /// "`who` mills `count`" ([CR#701.17a]) — a slice-family keyword action:
    /// [`Batch`](OneShotEffect::Batch) over the per-unit [`Action::mill_one`],
    /// so a count-doubling replacement (Bruvac, [CR#121.2a,616.1g]) bites the
    /// ONE aggregate window and the whole top slice commits as ONE simultaneous
    /// batch of per-card, individually-redirectable moves. The
    /// [`Mill`](../plugins)/`Mills` macros expand to this.
    #[must_use]
    pub fn mill(who: crate::Reference, count: Count) -> OneShotEffect {
        OneShotEffect::Batch(count, Arc::new(OneShotEffect::Act(Action::mill_one(who))))
    }

    /// "`who` draws `count`" ([CR#121.1]) — a slice-family keyword action:
    /// [`Batch`](OneShotEffect::Batch) over the per-unit [`Action::draw_one`],
    /// `count` SEQUENTIAL single-card draws ([CR#121.2], each seeing prior
    /// state and empty-checking BEFORE its move). The `Draw`/`Draws` macros
    /// expand to this.
    #[must_use]
    pub fn draw(who: crate::Reference, count: Count) -> OneShotEffect {
        OneShotEffect::Batch(count, Arc::new(OneShotEffect::Act(Action::draw_one(who))))
    }
}

/// Transitional source-compatible name for the pre-region instruction enum.
/// New core code should say [`Instr`]; the alias keeps downstream crates
/// compiling while their public fixtures migrate to region terminology.
pub type OneShotEffect = Instr;

/// `Continuously { effect, duration }` ([CR#611.2]). `effect` is boxed to break
/// the `OneShotEffect` → `StaticEffect` → `Replacement` → `OneShotEffect` size
/// cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Continuously {
    pub effect: Arc<StaticEffect>,
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
    pub effect: Arc<OneShotEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_did: Option<Arc<OneShotEffect>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_not: Option<Arc<OneShotEffect>>,
}

/// `If { condition, then, else }` — `else` is a keyword, so the field is
/// `otherwise`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct If {
    pub condition: Condition,
    pub then: Arc<OneShotEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otherwise: Option<Arc<OneShotEffect>>,
}

/// `Noting { key, effect }` — see `OneShotEffect::Noting`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Noting {
    pub key: crate::Ident,
    pub effect: Arc<OneShotEffect>,
}

/// `AdditionalCost { pay, body }` — "As an additional cost, [pay]; then run
/// [body]" ([CR#601.2f,118.8]). The payment is an event, so `body` reads the
/// sacrificed/exiled object through the event references
/// (`EventObject`/`EventActor`/`EventPatient`) — the cost-side twin of a
/// trigger's event bindings ("the sacrificed creature's power" =
/// `StatOf(EventObject, Power)`, Fling/Momentous Fall). Unlike `May`'s
/// `Act(Pay(cost))` shape there is no `who`: an additional cost is always
/// paid by the spell/ability's controller ([CR#601.2b]). `body` is boxed to
/// break the `OneShotEffect` → `AdditionalCost` → `OneShotEffect` size cycle
/// (mirrors [`May`]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AdditionalCost {
    pub pay: Cost,
    pub body: Arc<OneShotEffect>,
}

/// `Each { binder, do }` — `do` is a keyword, so the field is `effect`.
/// `binder` is the many-cardinality [`Binder`](crate::Binder) iterated; each
/// element binds in turn as [`Reference::It`](crate::Reference::It) for one run
/// of `effect` ([CR#608]). Mirrors the Idris `Each : Bindable b Many k -> …`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Each {
    pub binder: crate::Binder,
    pub effect: Arc<OneShotEffect>,
}

/// `With { binder, body }` — `body`/`do` is a keyword, so the field is `body`.
/// `binder` is the [`Binder`](crate::Binder) whose one/many cardinality picks
/// the body's anaphor: a one-binder binds
/// [`Reference::That`](crate::Reference::That), a many-binder binds
/// [`Selection::That`](crate::Selection::That).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct With {
    pub binder: crate::Binder,
    pub body: Arc<OneShotEffect>,
}

/// `Distribute { amount, binder, body }` — see [`OneShotEffect::Distribute`].
/// `amount` is the total to split, `binder` the many-cardinality
/// [`Binder`](crate::Binder) of recipients (each bound as
/// [`Reference::It`](crate::Reference::It) in turn), and `body` the per-element
/// effect that reads [`Count::Allotment`](crate::Count::Allotment) for that
/// element's share. `body` is boxed to break the `OneShotEffect` → `Distribute`
/// → `OneShotEffect` size cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Distribute {
    pub amount: crate::Count,
    pub binder: crate::Binder,
    pub body: Arc<OneShotEffect>,
}

/// `RevealUntil { whose, matches, body }` — see
/// [`OneShotEffect::RevealUntil`]. `whose`'s library is revealed from the top
/// one card at a time until one satisfies `matches`; `body` then runs with
/// the found card bound as `It` and the passed-over prefix bound as the
/// plural anaphor. `body` is boxed to break the `OneShotEffect` ->
/// `RevealUntil` -> `OneShotEffect` size cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RevealUntil {
    pub whose: crate::Reference,
    pub matches: crate::Predicate,
    pub body: Arc<OneShotEffect>,
}

/// `Modal { choose, modes }` ([CR#700.2]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Modal {
    pub choose: ChooseSpec,
    pub modes: Arc<[Mode]>,
}

/// `Label { as, effect }` — see [`OneShotEffect::Label`]. `as` is a Rust
/// keyword, hence the raw identifier; the RON field is spelled `as`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Label {
    pub r#as: crate::Ident,
    pub effect: Arc<OneShotEffect>,
}

/// `SeparatePiles { group, into, by, note, then }` — see
/// [`OneShotEffect::SeparatePiles`]. `by` defaults to `You` and is omitted from
/// RON when it is; `note`/`then` are omitted when absent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SeparatePiles {
    pub group: crate::Selection,
    pub into: Arc<[crate::Ident]>,
    pub by: Reference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<crate::Ident>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub then: Option<Arc<OneShotEffect>>,
}

/// `ChoosePile { from, by, random, then }` — see [`OneShotEffect::ChoosePile`].
/// `by` defaults to `You`; `random` defaults to `false`; both are omitted
/// from RON at their defaults.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ChoosePile {
    pub from: PileSource,
    pub by: Reference,
    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub random: bool,
    pub then: Arc<OneShotEffect>,
}

/// Where a [`ChoosePile`] takes its piles from: labels introduced in scope
/// (`Labels(["a", "b"])`, the Fact-or-Fiction shape) or piles noted earlier
/// under a key, per divider (`Noted { note, of }`, the Whims-of-the-Fates
/// shape) ([CR#700.3a..700.3b]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PileSource {
    Labels(Arc<[crate::Ident]>),
    Noted { note: crate::Ident, of: Reference },
}
