use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::ChooseSpec;
use crate::Condition;
use crate::Cost;
use crate::Count;
use crate::Expand;
use crate::Expansion;
use crate::Mode;
use crate::SupportsMacros;
use crate::TargetSpec;
use crate::ability::TriggeredAbility;
use crate::action::Action;
use crate::continuous::Duration;
use crate::continuous::StaticEffect;
use crate::reference::Reference;

/// An effect an ability produces ([CR#608]). Compartmentalized in Rust; flat in
/// RON (`DrawCard(You)`, never `Act(DrawCard(You))`): the `Act` tag never
/// appears in text. Its `#[macro_ron(flatten)]` marker lifts [`Action`]'s
/// accepted names into `OneShotEffect`'s dispatch, so a player verb
/// (`DrawCard(You)`) reads at an effect slot exactly like a source-agent verb
/// — the agent is spelled, never a read-time default — and the write is
/// transparent.
///
/// A single instruction stands bare (`effect: DealDamage(This, 3, It)`); the
/// structural forms (`Sequentially`, `May`, `If`, …) are the corpus's
/// connective tissue — data the engine interprets, never seen by the macro
/// layer as control flow. The struct-carrying forms delegate to inner derived
/// structs (`May`, …), which read flat via `unwrap_variant_newtypes` and carry
/// the field defaults and shapes.
// No `large_enum_variant` suppression: the top variants cluster within clippy's
// threshold (`Distribute` ~568 B, `Act`/`Each`/`With` ~488 B, `SeparatePiles`
// ~416 B — a spread under 200 B), so the lint does not fire. `Act(Action)` is
// kept inline deliberately: `Action` is a big *balanced* leaf enum (no single fat
// field to box), and `OneShotEffect` is the hot, recursively-matched node of the
// effect grammar — boxing `Act` would inject a deref into every match on the
// `resolve` hot path for no lint gain. The recursive sub-effect fields are
// already boxed (`May.effect`, …).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum OneShotEffect {
    /// A single intrinsic instruction (the `Act` compartment, transparent in
    /// RON).
    #[macro_ron(flatten)]
    Act(Action),
    /// Explicit "then" — ordered sub-effects ([CR#608.2c]).
    Sequentially(Arc<[OneShotEffect]>),
    /// Simultaneously sub-effects — the written spec. **One snapshot:** every
    /// member reads game state as of one pre-application view (an exchange
    /// works BECAUSE both halves read the pre-state). **One timestamp, one
    /// batch:** application emits the member facts as one batch sharing a
    /// batch id — ONE occurrence for `OneOrMore` triggers ([CR#603.3b]).
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
    /// "You may [do]" ([CR#603,608]) — with "if you do"/"if you don't".
    May(May),
    /// "If [condition], [then]; otherwise [else]" ([CR#603.4]-style branch).
    If(If),
    /// "[actor] may pay [cost]; if they do, [`and_then`], else [`or_else`]"
    /// ([CR#603,608]) — a resolution-time kicker over the full [`Cost`] algebra
    /// (the may-pay→branch shape [`MustPay`](OneShotEffect::MustPay) can't
    /// spell).
    MayPay(MayPay),
    /// "[actor] must pay [cost], or else [`or_else`]" ([CR#118.12a]) — the
    /// resolution-time punisher (Mana Leak's "counter target spell unless its
    /// controller pays {N}") over the full [`Cost`] algebra. The English
    /// "[do] unless [who] pays [cost]" order is the builtin `Unless` MACRO —
    /// a render name over this node; core keeps only the CR-family form.
    MustPay(MustPay),
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
    /// Targets scoped over an inner effect ([CR#115.1,601.2c]): the rules-
    /// faithful home for the word "target" — declared on the effect that
    /// consumes it, its announced slots read back by the anaphors
    /// (`It`/`That(Sort)`/`They`, or `Target(n)` for the nth announced slot).
    Targeted(Targeted),
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
    /// A remembered `OneShotEffect` macro invocation (declared compound verbs
    /// like `Investigate`). Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<OneShotEffect>),
}

impl OneShotEffect {
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

/// `Continuously { effect, duration }` ([CR#611.2]). `effect` is boxed to break
/// the `OneShotEffect` → `StaticEffect` → `Replacement` → `OneShotEffect` size
/// cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Continuously {
    pub effect: Arc<StaticEffect>,
    pub duration: Duration,
}

/// `Targeted { targets, effect }` ([CR#115.1,601.2c]) — declares the
/// targets its inner effect consumes, scoping the announced-slot reads to this
/// list. Targets are chosen at announcement and stored on the stack object;
/// at resolution this node is transparent (the inner effect runs with
/// `frame.targets` already bound), and per-instance illegal-target handling
/// ([CR#608.2b]) reads each inner instruction's referenced targets. `effect`
/// is boxed to break the `OneShotEffect` → `Targeted` → `OneShotEffect` size
/// cycle (mirrors `May`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Targeted {
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub targets: Arc<[TargetSpec]>,
    pub effect: Arc<OneShotEffect>,
}

impl Targeted {
    /// Scopes `targets` over `effect`, boxing the inner effect. Builds the
    /// wrapper without the caller spelling the `Arc::new` / field order.
    #[must_use]
    pub fn new(targets: Arc<[TargetSpec]>, effect: OneShotEffect) -> Targeted {
        Targeted {
            targets,
            effect: Arc::new(effect),
        }
    }
}

/// `May { who, do, if_did, if_not }` — `do` is a keyword, so the field is
/// `effect`. `who` is the decider ([CR#608.2d]; Browbeat's decider ≠
/// performer — "Any player may have Browbeat deal 5 damage to them" — proves
/// the slot is not derivable, Law 3), spelling today's implicit "you may"
/// decider (no read-time default, Law 2). Branch semantics (`if_did`/
/// `if_not` invoking [CR#118.12] cost semantics on `effect`) stay unwired —
/// grammar only; collapsing `MayPay`/`MustPay` into this node is a later
/// task.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct If {
    pub condition: Condition,
    pub then: Arc<OneShotEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otherwise: Option<Arc<OneShotEffect>>,
}

/// `Noting { key, effect }` — see `OneShotEffect::Noting`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Noting {
    pub key: crate::Ident,
    pub effect: Arc<OneShotEffect>,
}

/// serde default for the paying/acting player — "you" unless the text names
/// another ([CR#118.12a]).
fn ref_you() -> Reference {
    Reference::You
}

/// `skip_serializing_if` predicate: the default `You` is omitted from RON.
fn ref_is_you(r: &Reference) -> bool {
    matches!(r, Reference::You)
}

/// `MayPay { actor, cost, and_then, or_else }` — "[actor] may pay [cost]; if
/// they do, [`and_then`]; if they don't, [`or_else`]" ([CR#603,608]): a
/// resolution-time kicker over the full [`Cost`] algebra. `actor` is the paying
/// player ("you" unless the text names another); it defaults to `You` and is
/// omitted from RON when it is. `or_else` (the "if you don't" branch) is
/// omitted when absent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct MayPay {
    #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
    pub actor: Reference,
    pub cost: Cost,
    pub and_then: Arc<OneShotEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or_else: Option<Arc<OneShotEffect>>,
}

/// `MustPay { actor, cost, or_else }` — "[actor] must pay [cost], or else
/// [`or_else`]" ([CR#118.12a]): the resolution-time punisher (Mana Leak's
/// "counter target spell unless its controller pays {N}") over the full
/// [`Cost`] algebra. The English-order spelling
/// `Unless(effect: or_else, who: actor, unless: cost)` is the builtin
/// `Unless` macro, which expands to exactly this node. `actor` defaults to
/// `You` and is omitted from RON when it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct MustPay {
    #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
    pub actor: Reference,
    pub cost: Cost,
    pub or_else: Arc<OneShotEffect>,
}

/// `AdditionalCost { pay, body }` — "As an additional cost, [pay]; then run
/// [body]" ([CR#601.2f,118.8]). The payment is an event, so `body` reads the
/// sacrificed/exiled object through the event references
/// (`EventObject`/`EventActor`/`EventPatient`) — the cost-side twin of a
/// trigger's event bindings ("the sacrificed creature's power" =
/// `StatOf(EventObject, Power)`, Fling/Momentous Fall). Unlike
/// [`MayPay`]/[`MustPay`] there is no `actor`: an additional cost is always
/// paid by the spell/ability's controller ([CR#601.2b]). `body` is boxed to
/// break the `OneShotEffect` → `AdditionalCost` → `OneShotEffect` size cycle
/// (mirrors [`May`]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct AdditionalCost {
    pub pay: Cost,
    pub body: Arc<OneShotEffect>,
}

/// `Each { binder, do }` — `do` is a keyword, so the field is `effect`.
/// `binder` is the many-cardinality [`Binder`](crate::Binder) iterated; each
/// element binds in turn as [`Reference::It`](crate::Reference::It) for one run
/// of `effect` ([CR#608]). Mirrors the Idris `Each : Bindable b Many k -> …`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Each {
    pub binder: crate::Binder,
    pub effect: Arc<OneShotEffect>,
}

/// `With { binder, body }` — `body`/`do` is a keyword, so the field is `body`.
/// `binder` is the [`Binder`](crate::Binder) whose one/many cardinality picks
/// the body's anaphor: a one-binder binds
/// [`Reference::That`](crate::Reference::That), a many-binder binds
/// [`Selection::That`](crate::Selection::That).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct RevealUntil {
    pub whose: crate::Reference,
    pub matches: crate::Predicate,
    pub body: Arc<OneShotEffect>,
}

/// `Modal { choose, modes }` ([CR#700.2]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Modal {
    pub choose: ChooseSpec,
    pub modes: Arc<[Mode]>,
}

/// `Label { as, effect }` — see [`OneShotEffect::Label`]. `as` is a Rust
/// keyword, hence the raw identifier; the RON field is spelled `as`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Label {
    pub r#as: crate::Ident,
    pub effect: Arc<OneShotEffect>,
}

/// `SeparatePiles { group, into, by, note, then }` — see
/// [`OneShotEffect::SeparatePiles`]. `by` defaults to `You` and is omitted from
/// RON when it is; `note`/`then` are omitted when absent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct SeparatePiles {
    pub group: crate::Selection,
    pub into: Arc<[crate::Ident]>,
    #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
    pub by: Reference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<crate::Ident>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub then: Option<Arc<OneShotEffect>>,
}

/// `ChoosePile { from, by, random, then }` — see [`OneShotEffect::ChoosePile`].
/// `by` defaults to `You`; `random` defaults to `false`; both are omitted
/// from RON at their defaults.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ChoosePile {
    pub from: PileSource,
    #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
    pub by: Reference,
    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub random: bool,
    pub then: Arc<OneShotEffect>,
}

/// Where a [`ChoosePile`] takes its piles from: labels introduced in scope
/// (`Labels(["a", "b"])`, the Fact-or-Fiction shape) or piles noted earlier
/// under a key, per divider (`Noted { note, of }`, the Whims-of-the-Fates
/// shape) ([CR#700.3a..700.3b]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PileSource {
    Labels(Arc<[crate::Ident]>),
    Noted { note: crate::Ident, of: Reference },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Count;
    use crate::action::LifeOp;
    use crate::mana::ManaSpec;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> OneShotEffect {
        crate::ron::options().from_str(source).unwrap()
    }
    fn write(effect: &OneShotEffect) -> String {
        crate::ron::options().to_string(effect).unwrap()
    }

    /// Bare player verbs read flat with the agent spelled; source verbs read
    /// native.
    #[test]
    fn verbs_read_flat() {
        assert_eq!(
            read("ChangeLife(You, Down(1))"),
            OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Down(Count::Literal(1))
            )),
        );
        assert_eq!(
            read("ChangeLife(You, Up(3))"),
            OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(3))
            )),
        );
        assert_eq!(
            read("Sacrifice(You, This)"),
            OneShotEffect::Act(Action::Sacrifice(Reference::You, Reference::This)),
        );
        assert_eq!(
            read("DealDamage(This, Literal(3), It)"),
            OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3),)),
        );
        assert_eq!(
            read("AddMana(You, Literal(1), AnyColor)"),
            OneShotEffect::Act(Action::AddMana(
                Reference::You,
                Count::Literal(1),
                ManaSpec::AnyColor.into()
            )),
        );
    }

    /// The source verbs read native; the player verbs spell their agent.
    /// Verb patients are now a single bare [`Reference`].
    #[test]
    fn new_verbs_read_flat() {
        // `Counter` is a source verb (Destroy is now the `Composite` the
        // `Action::destroy` ctor builds, not a bareword core verb).
        assert_eq!(
            read("Counter(This)"),
            OneShotEffect::Act(Action::Counter(Reference::This)),
        );
        assert_eq!(
            read("Tap(This)"),
            OneShotEffect::Act(Action::Tap(Reference::This)),
        );
        // Discard is now the `Composite { name, body }` the `Action::discard`
        // ctor builds ([CR#701.9]), like Destroy — the struct-variant spelling
        // reads all-named. The chosen form's body is a `With(Choose(..InHand..),
        // Each(They, ..))` choose-then-act step (Task 8: the discard choice
        // rides the ordinary `Choose` binder, not a bespoke selection).
        assert_eq!(
            read(
                "Composite(name: Discard, body: With(binder: Choose(quantity: Range(1, 1), \
                 filter: And([InZone(Hand), Owner(Ref(You))])), body: Each(binder: \
                 Existing(They), effect: Composite(name: Discard, body: Move(It, \
                 Graveyard)))))"
            ),
            OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(1), false)),
        );
    }

    /// Brainstorm's second half: putting cards on top of the library is the
    /// `Move`-to-library form — a source verb read natively at an effect slot,
    /// with the position an `Anchor` (`FromTop(0)` = top). [CR#401.7]
    #[test]
    fn move_to_library_reads_at_effect_slot() {
        use crate::Anchor;
        use crate::Destination;
        assert_eq!(
            read("Move(This, Library(FromTop(0)))"),
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Library(Anchor::FromTop(Count::Literal(0))),
                vec![].into(),
                None,
            )),
        );
    }

    /// A non-`You` player agent reads native.
    #[test]
    fn explicit_agent_reads_flat() {
        assert_eq!(
            read("ChangeLife(It, Up(3))"),
            OneShotEffect::Act(Action::ChangeLife(
                Reference::It,
                LifeOp::Up(Count::Literal(3)),
            )),
        );
    }

    /// Structural forms read flat (the inner-struct delegation) and the
    /// Option fields default to None. `May.who` is always spelled (Law 2).
    #[test]
    fn structural_forms_read_flat() {
        assert_eq!(
            read("Sequentially([ChangeLife(You, Up(1)), ChangeLife(You, Up(1))])"),
            OneShotEffect::Sequentially(
                vec![
                    OneShotEffect::Act(Action::ChangeLife(
                        Reference::You,
                        LifeOp::Up(Count::Literal(1))
                    )),
                    OneShotEffect::Act(Action::ChangeLife(
                        Reference::You,
                        LifeOp::Up(Count::Literal(1))
                    )),
                ]
                .into()
            ),
        );
        let may = read("May(who: You, effect: ChangeLife(You, Up(1)))");
        let OneShotEffect::May(may) = may else {
            panic!("expected May");
        };
        assert_eq!(may.who, Reference::You);
        assert_eq!(
            *may.effect,
            OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1))
            ))
        );
        assert!(may.if_did.is_none() && may.if_not.is_none());
    }

    #[test]
    fn act_serializes_flat() {
        // A `Count` literal writes bare — `1`, never `Literal(1)`.
        assert_eq!(
            write(&OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1))
            ))),
            "ChangeLife(You,Up(1))"
        );
    }

    #[test]
    fn effects_round_trip() {
        let cases = [
            "ChangeLife(You,Up(1))",
            "ChangeLife(You,Up(3))",
            "Sacrifice(You,This)",
            "ChangeLife(It,Up(3))",
            "DealDamage(This,Literal(3),It)",
            "AddMana(You,Literal(1),AnyColor)",
            // Destroy is the `Composite { name, body }` the `Action::destroy`
            // ctor builds; the struct-variant spelling round-trips through RON.
            "Composite(name:Destroy,body:Move(This,Graveyard))",
            "Sequentially([ChangeLife(You,Up(1)),ChangeLife(You,Up(1))])",
            "May(who:You,effect:ChangeLife(You,Up(1)))",
            // `Each.binder` is a many-`Binder` (the set of all creatures wrapped
            // in `Existing`), binding `It` per element.
            "Each(binder:Existing(SelectAll(Supertype(Basic))),effect:ChangeLife(You,Up(1)))",
            // Brainstorm's shape in the new model: choose 2 cards (a many-binder
            // `With`), then `Each` over the bound group (`Existing(They)`), moving
            // each onto the library via the `It` element. Core reader has no
            // macros, so the `Quantity` is the bare `Range` primitive
            // (`Exactly(2)` is the cards-layer macro spelling).
            "With(binder:Choose(quantity:Range(Literal(2),Literal(2)),filter:InZone(Hand)),\
             body:Each(binder:Existing(They),\
             effect:Move(It,Library(FromTop(Literal(0))))))",
            // `Batch` is `Repeat`'s BATCHING twin ([CR#616.1g]) — in THIS
            // task shell-equivalent to `Repeat`, so it round-trips the same
            // `(Count, Arc<OneShotEffect>)` shape.
            "Batch(Literal(2),ChangeLife(You,Up(1)))",
        ];
        for source in cases {
            let parsed = read(source);
            let written = write(&parsed);
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }

    /// `MustPay` reads flat over the full `Cost` algebra, defaults `actor` to
    /// `You` (omitted on write), and round-trips — the Mana Leak shape
    /// ([CR#118.12a]); the English "unless" order is the `Unless` macro.
    #[test]
    fn must_pay_defaults_actor_and_round_trips() {
        // Mana Leak: "counter target spell unless its controller pays {3}".
        let mana_leak =
            "MustPay(actor:ControllerOf(It),cost:[Mana([Generic(3)])],or_else:Counter(It))";
        let parsed = read(mana_leak);
        let OneShotEffect::MustPay(m) = &parsed else {
            panic!("expected MustPay, got {parsed:?}");
        };
        assert_eq!(m.actor, Reference::ControllerOf(Arc::new(Reference::It)));
        assert_eq!(
            m.cost.0.len(),
            1,
            "the full Cost carries the {{3}} component"
        );
        assert_eq!(write(&parsed), mana_leak, "Mana Leak shape round-trips");

        // Default actor (You) is omitted on write.
        let omitted = "MustPay(cost:[Mana([Generic(2)])],or_else:ChangeLife(You,Down(1)))";
        let parsed = read(omitted);
        let OneShotEffect::MustPay(m) = &parsed else {
            panic!("expected MustPay");
        };
        assert_eq!(m.actor, Reference::You, "omitted actor defaults to You");
        assert_eq!(write(&parsed), omitted, "default actor is omitted on write");
    }

    /// `MayPay` reads flat, omits the default `actor` and the absent `or_else`,
    /// and round-trips with and without the "if you don't" branch
    /// ([CR#603,608]).
    #[test]
    fn may_pay_round_trips_with_and_without_or_else() {
        // No "if you don't" branch — `or_else` omitted.
        let bare = "MayPay(cost:[Mana([Generic(1)])],and_then:ChangeLife(You,Up(1)))";
        let parsed = read(bare);
        let OneShotEffect::MayPay(m) = &parsed else {
            panic!("expected MayPay, got {parsed:?}");
        };
        assert_eq!(m.actor, Reference::You);
        assert!(m.or_else.is_none());
        assert_eq!(write(&parsed), bare, "bare MayPay round-trips");

        // With an explicit actor and an "if you don't" branch.
        let full = "MayPay(actor:It,cost:[Mana([Generic(2)])],and_then:ChangeLife(You,Up(2)),\
                     or_else:ChangeLife(You,Down(1)))";
        assert_eq!(write(&read(full)), full, "full MayPay round-trips");
    }

    /// `AdditionalCost { pay, body }` reads flat over the full `Cost` algebra
    /// and round-trips — the printed/nested additional-cost shape
    /// ([CR#601.2f,118.8]) whose body reads the paid object via the event
    /// references (Fling's "sacrifice a creature: ~ deals damage equal to
    /// its power", over the core primitives — no card-layer macros).
    #[test]
    fn additional_cost_reads_and_round_trips() {
        let src = "AdditionalCost(pay:[Do(Sacrifice(You,This))],body:DealDamage(This,StatOf(EventObject,Power),It))";
        let parsed = read(src);
        let OneShotEffect::AdditionalCost(ac) = &parsed else {
            panic!("expected AdditionalCost, got {parsed:?}");
        };
        assert_eq!(
            ac.pay.0.len(),
            1,
            "the additional cost carries the sacrifice"
        );
        assert_eq!(
            *ac.body,
            OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::StatOf(Reference::EventObject, crate::Stat::Power),
            )),
            "the body reads the paid object via EventObject",
        );
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
        assert_eq!(write(&parsed), src, "writes back to the flat form");
    }

    #[test]
    fn unknown_names_error() {
        let err = crate::ron::options()
            .from_str::<OneShotEffect>("Bogus(1)")
            .unwrap_err();
        let err = err.to_string();
        assert!(err.contains("Bogus"));
    }

    /// A `Targeted` wrapper declares its targets and scopes the slot reads
    /// over the inner effect; it reads flat through the newtype variant and
    /// round-trips ([CR#115.1,601.2c]).
    #[test]
    fn targeted_effect_reads_and_round_trips() {
        let src = "Targeted(targets:[Target(Range(Literal(1),Literal(1)),Supertype(Basic))],effect:DealDamage(This,Literal(3),It))";
        let parsed = read(src);
        let OneShotEffect::Targeted(te) = &parsed else {
            panic!("expected Targeted, got {parsed:?}");
        };
        assert_eq!(te.targets.len(), 1);
        assert_eq!(
            *te.effect,
            OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3),)),
        );
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
    }

    /// `With.binder` carries a [`Binder`](crate::Binder); a many-binder
    /// `Existing(<selection>)` binds the group as `Selection::That`.
    #[test]
    fn with_binds_a_group() {
        let v = read("With(binder:Existing(TopOfLibrary(count:2)),body:Sequentially([]))");
        let OneShotEffect::With(w) = &v else {
            panic!("expected With, got {v:?}");
        };
        assert!(matches!(
            w.binder,
            crate::Binder::Existing(Selection::TopOfLibrary { .. })
        ));
        assert_eq!(read(&write(&v)), v);
    }

    /// `Distribute` reads flat through the newtype variant; its body reads the
    /// `Allotment` anaphor over the per-element `It` — divided damage
    /// (`DealDamage(This, Allotment, It)`) and divided counters round-trip
    /// ([CR#601.2d]). `binder` is a many-`Binder`.
    #[test]
    fn divide_among_reads_and_round_trips() {
        let damage = read(
            "Distribute(amount: 3, binder: Existing(SelectAll(Supertype(Basic))), \
             body: DealDamage(This, Allotment, It))",
        );
        let OneShotEffect::Distribute(d) = &damage else {
            panic!("expected Distribute, got {damage:?}");
        };
        assert_eq!(d.amount, Count::Literal(3));
        assert!(matches!(
            d.binder,
            crate::Binder::Existing(Selection::SelectAll(_))
        ));
        assert_eq!(
            *d.body,
            OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Allotment,)),
        );
        assert_eq!(read(&write(&damage)), damage, "round-trip");

        // Divided counters: the same primitive, a different body, iterating the
        // With-bound group (`Existing(They)`).
        let counters = read(
            "Distribute(amount: X, binder: Existing(They), \
             body: PutCounters(It, P1P1Counter, Allotment))",
        );
        assert!(matches!(counters, OneShotEffect::Distribute(_)));
        assert_eq!(read(&write(&counters)), counters, "round-trip");
    }

    /// `Each` iterates a many-`Binder`, exposing each element as
    /// [`Reference::It`]; it reads flat and round-trips (the Idris
    /// `Each : Bindable b Many k -> …`).
    #[test]
    fn each_binds_via_binder() {
        let v =
            read("Each(binder:Existing(SelectAll(Supertype(Basic))),effect:ChangeLife(You,Up(1)))");
        let OneShotEffect::Each(e) = &v else {
            panic!("expected Each, got {v:?}");
        };
        assert!(matches!(
            e.binder,
            crate::Binder::Existing(Selection::SelectAll(_))
        ));
        assert_eq!(read(&write(&v)), v, "round-trip");

        // Iterating the With-bound group: `Each(Existing(They), …)` reads `It`.
        let over_group = read("Each(binder:Existing(They),effect:Counter(It))");
        assert!(matches!(
            over_group,
            OneShotEffect::Each(ref e) if matches!(e.binder, crate::Binder::Existing(Selection::They)),
        ));
        assert_eq!(read(&write(&over_group)), over_group, "round-trip");
    }

    /// The sorted anaphors resolve BY SLOT: `That(Card)` in a
    /// `Reference`-typed position, `They`/`Them(Sort)` in a
    /// `Selection`-typed position — serde/RON picks the variant from the
    /// field's type. Both forms round-trip.
    #[test]
    fn sorted_anaphors_resolve_by_slot() {
        // Reference slot: `Counter(That(Creature))`'s patient is a single
        // sorted anaphor.
        let reference_slot = read("Counter(That(Creature))");
        assert_eq!(
            reference_slot,
            OneShotEffect::Act(Action::Counter(Reference::That(crate::Sort::OfType(
                crate::Type::Creature
            )))),
            "`That(Creature)` in a Reference slot is Reference::That",
        );
        assert_eq!(read(&write(&reference_slot)), reference_slot, "round-trip");

        // Selection slot: a many-`Binder` `Existing(They)` wraps a `Selection`.
        let selection_slot = read("With(binder:Existing(They),body:Sequentially([]))");
        let OneShotEffect::With(w) = &selection_slot else {
            panic!("expected With, got {selection_slot:?}");
        };
        assert_eq!(
            w.binder,
            crate::Binder::Existing(Selection::They),
            "`They` in a Selection slot is Selection::They",
        );
        assert_eq!(read(&write(&selection_slot)), selection_slot, "round-trip");
    }

    /// `Until(duration, parts)` takes a LIST of static parts and
    /// round-trips ([CR#611.2,611.2c]) — the Boros Charm mode-2 shape.
    #[test]
    fn until_reads_a_part_list() {
        let src = "Until(FixedUntil(EndOfTurn),[Modify(This,Power(Up(Literal(1))))])";
        let parsed = read(src);
        let OneShotEffect::Until(_, parts) = &parsed else {
            panic!("expected Until, got {parsed:?}");
        };
        assert_eq!(parts.len(), 1);
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
    }

    /// `Label { as, effect }` names an introduction later clauses read back.
    /// The raw-keyword field spells `as` in RON.
    #[test]
    fn label_round_trips() {
        let src = "Label(as:\"exiled\",effect:Move(It,Exile))";
        let parsed = read(src);
        let OneShotEffect::Label(label) = &parsed else {
            panic!("expected Label, got {parsed:?}");
        };
        assert_eq!(label.r#as.as_str(), "exiled");
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
    }

    /// `SeparatePiles`/`ChoosePile` — the pile shapes ([CR#700.3a..700.3b])
    /// — read flat, default `by: You`/`random: false`, and round-trip.
    #[test]
    fn piles_round_trip() {
        let separate = read("SeparatePiles(group: TopOfLibrary(count: 5), into: [\"a\", \"b\"])");
        let OneShotEffect::SeparatePiles(sp) = &separate else {
            panic!("expected SeparatePiles, got {separate:?}");
        };
        assert_eq!(sp.by, Reference::You, "omitted by defaults to You");
        assert!(sp.note.is_none() && sp.then.is_none());
        assert_eq!(read(&write(&separate)), separate, "round-trip");

        let choose = read(
            "ChoosePile(from: Labels([\"a\", \"b\"]), by: Opponent, \
             then: Each(binder: Existing(Them(Pile)), effect: Move(It, Hand)))",
        );
        let OneShotEffect::ChoosePile(cp) = &choose else {
            panic!("expected ChoosePile, got {choose:?}");
        };
        assert!(!cp.random, "omitted random defaults to false");
        assert_eq!(read(&write(&choose)), choose, "round-trip");

        // The noted per-player form (the Whims shape).
        let noted = read(
            "ChoosePile(from: Noted(note: \"whims\", of: It), random: true, \
             then: Sequentially([]))",
        );
        assert_eq!(read(&write(&noted)), noted, "round-trip");
    }
}
