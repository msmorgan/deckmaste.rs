use serde::Deserialize;
use serde::Serialize;

use crate::ChooseSpec;
use crate::Condition;
use crate::Cost;
use crate::Expand;
use crate::Expansion;
use crate::Mode;
use crate::SupportsMacros;
use crate::TargetSpec;
use crate::ability::TriggeredAbility;
use crate::action::Action;
use crate::action::PlayerAction;
use crate::continuous::Duration;
use crate::continuous::StaticEffect;
use crate::reference::Reference;

/// An effect an ability produces ([CR#608]). Compartmentalized in Rust; flat in
/// RON (`Draw(1)`, never `Act(By(You, Draw(1)))`): the `Act` tag never appears
/// in text. Its `#[macro_ron(flatten)]` marker lifts [`Action`]'s accepted
/// names into `Effect`'s dispatch — transitively through `Action::By`'s embed,
/// so a bare player verb (`Draw(1)`) reads at an effect slot as the
/// implicit-`You` default `Act(By(You, …))` — and the write is transparent.
///
/// A single instruction stands bare (`effect: DealDamage(Target(0), 3)`); the
/// structural forms (`Sequence`, `May`, `If`, …) are the corpus's connective
/// tissue — data the engine interprets, never seen by the macro layer as
/// control flow. The struct-carrying forms delegate to inner derived structs
/// (`May`, …), which read flat via `unwrap_variant_newtypes` and carry
/// the field defaults and shapes.
// `Act(Action)` is the largest variant: `Action` is a big *balanced* leaf enum
// (no single fat field to box), and `Effect` is the hot, recursively-matched
// node of the effect grammar. Boxing `Act` would inject indirection + a deref
// into every `Effect::Act` match (incl. the `resolve` hot path) and still leave
// `Effect` over the bar via the next-largest variant — so it buys nothing for
// the lint without a sweeping multi-box of `Action`/`Condition` embeddings.
// The recursive sub-effect fields are already boxed (`May.effect`, …);
// this `allow` is the "balanced AST leaf" exception, same call as `Ability`.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Effect {
    /// A single intrinsic instruction (the `Act` compartment, transparent in
    /// RON).
    #[macro_ron(flatten)]
    Act(Action),
    /// Explicit "then" — ordered sub-effects ([CR#608.2c]).
    Sequence(Vec<Effect>),
    /// A one-shot-created continuous effect ([CR#611.2]).
    Continuously(Continuously),
    /// "You may [do]" ([CR#603,608]) — with "if you do"/"if you don't".
    May(May),
    /// "If [condition], [then]; otherwise [else]" ([CR#603.4]-style branch).
    If(If),
    /// "[do] unless [you pay]" ([CR#118.12a]). The bare-[`CostComponent`]-list
    /// sugar that [`MustPay`](Effect::MustPay) supersedes; kept for the corpus
    /// already spelled with it.
    Unless(Unless),
    /// "[actor] may pay [cost]; if they do, [`and_then`], else [`or_else`]"
    /// ([CR#603,608]) — a resolution-time kicker over the full [`Cost`] algebra
    /// (the may-pay→branch shape `Unless` can't spell).
    MayPay(MayPay),
    /// "[actor] must pay [cost], or else [`or_else`]" ([CR#118.12a]) — the
    /// resolution-time punisher (Mana Leak's "counter target spell unless its
    /// controller pays {N}") over the full [`Cost`] algebra. Supersedes
    /// [`Unless`](Effect::Unless).
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
    /// resolution-time (≥1 each, summing to `amount`). One primitive subsumes
    /// divided damage (`body: DealDamage(It, Allotment)`) AND divided counters
    /// (`body: PutCounters(It, <kind>, Allotment)`) — the body reads the
    /// allotment anaphor. Mirrors the Idris `Distribute : Count b ->
    /// Bindable b Many k -> …`; named `DivideAmong` to avoid colliding with
    /// the unrelated scry-partition `PlayerAction::Distribute`.
    DivideAmong(DivideAmong),
    /// A delayed triggered ability created on resolution ([CR#603.7]).
    /// Note the object set the inner effect moves/touches under `key`
    /// ([CR#607.2a] exiled-with linkage).
    Noting(Noting),
    Delayed(Box<TriggeredAbility>),
    /// A reflexive triggered ability created on resolution ([CR#603.12]).
    Reflexive(Box<TriggeredAbility>),
    /// A modal effect: choose modes, then apply them ([CR#700.2]). This is the
    /// realized form of the design's `Resolvable::Modal` — see the report.
    Modal(Modal),
    /// Targets scoped over an inner effect ([CR#115.1,601.2c]): the rules-
    /// faithful home for the word "target" — declared on the effect that
    /// consumes it, with `Reference::Target(n)` indexing this node's list.
    Targeted(Targeted),
    /// A remembered `Effect` macro invocation (declared compound verbs like
    /// `Investigate`). Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<Effect>),
}

impl Effect {
    /// A bare player verb in the implicit-"you" default — `Act(By(You, …))`,
    /// the form a player verb written bare in an effect slot reads as.
    #[must_use]
    pub fn act_by_you(action: PlayerAction) -> Effect {
        Effect::Act(Action::by_you(action))
    }
}

/// `Continuously { effect, duration }` ([CR#611.2]). `effect` is boxed to break
/// the `Effect` → `StaticEffect` → `Replacement` → `Effect` size cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Continuously {
    pub effect: Box<StaticEffect>,
    pub duration: Duration,
}

/// `Targeted { targets, effect }` ([CR#115.1,601.2c]) — declares the
/// targets its inner effect consumes, scoping `Reference::Target(n)` to this
/// list. Targets are chosen at announcement and stored on the stack object;
/// at resolution this node is transparent (the inner effect runs with
/// `frame.targets` already bound), and per-instance illegal-target handling
/// ([CR#608.2b]) reads each inner instruction's referenced targets. `effect`
/// is boxed to break the `Effect` → `Targeted` → `Effect` size cycle
/// (mirrors `May`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Targeted {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<TargetSpec>,
    pub effect: Box<Effect>,
}

impl Targeted {
    /// Scopes `targets` over `effect`, boxing the inner effect. Builds the
    /// wrapper without the caller spelling the `Box::new` / field order.
    #[must_use]
    pub fn new(targets: Vec<TargetSpec>, effect: Effect) -> Targeted {
        Targeted {
            targets,
            effect: Box::new(effect),
        }
    }
}

/// `May { do, if_did, if_not }` — `do` is a keyword, so the field is `effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct May {
    pub effect: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_did: Option<Box<Effect>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_not: Option<Box<Effect>>,
}

/// `If { condition, then, else }` — `else` is a keyword, so the field is
/// `otherwise`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct If {
    pub condition: Condition,
    pub then: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otherwise: Option<Box<Effect>>,
}

/// `Noting { key, effect }` — see `Effect::Noting`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Noting {
    pub key: crate::Ident,
    pub effect: Box<Effect>,
}

/// `Unless { do, who, unless }` — `do` is a keyword, so the field is `effect`;
/// `unless` is the cost the affected player may pay to avoid it ([CR#118.12a]).
/// `who` is that affected/paying player — "you" unless the text names another
/// ("target player … unless that player pays …"); it defaults to `You` and is
/// omitted from RON when it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Unless {
    pub effect: Box<Effect>,
    #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
    pub who: Reference,
    pub unless: Vec<crate::CostComponent>,
}

/// serde default for [`Unless::who`] — the affected player is "you"
/// unless the text names another ([CR#118.12a]).
fn ref_you() -> Reference {
    Reference::You
}

/// `skip_serializing_if` predicate for [`Unless::who`]: the default `You`
/// is omitted from RON.
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
    pub and_then: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or_else: Option<Box<Effect>>,
}

/// `MustPay { actor, cost, or_else }` — "[actor] must pay [cost], or else
/// [`or_else`]" ([CR#118.12a]): the resolution-time punisher (Mana Leak's
/// "counter target spell unless its controller pays {N}") over the full
/// [`Cost`] algebra. Supersedes [`Unless`](Effect::Unless), whose `unless` is a
/// bare [`CostComponent`](crate::CostComponent) list:
/// `MustPay { actor, cost, or_else }` is exactly
/// `Unless { who: actor, unless: cost, effect: or_else }`. `actor` defaults to
/// `You` and is omitted from RON when it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct MustPay {
    #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
    pub actor: Reference,
    pub cost: Cost,
    pub or_else: Box<Effect>,
}

/// `AdditionalCost { pay, body }` — "As an additional cost, [pay]; then run
/// [body]" ([CR#601.2f,118.8]). The payment is an event, so `body` reads the
/// sacrificed/exiled object through the event references
/// (`EventObject`/`EventActor`/`EventPatient`) — the cost-side twin of a
/// trigger's event bindings ("the sacrificed creature's power" =
/// `StatOf(EventObject, Power)`, Fling/Momentous Fall). Unlike
/// [`MayPay`]/[`MustPay`] there is no `actor`: an additional cost is always
/// paid by the spell/ability's controller ([CR#601.2b]). `body` is boxed to
/// break the `Effect` → `AdditionalCost` → `Effect` size cycle (mirrors
/// [`May`]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct AdditionalCost {
    pub pay: Cost,
    pub body: Box<Effect>,
}

/// `Each { binder, do }` — `do` is a keyword, so the field is `effect`.
/// `binder` is the many-cardinality [`Binder`](crate::Binder) iterated; each
/// element binds in turn as [`Reference::It`](crate::Reference::It) for one run
/// of `effect` ([CR#608]). Mirrors the Idris `Each : Bindable b Many k -> …`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Each {
    pub binder: crate::Binder,
    pub effect: Box<Effect>,
}

/// `With { binder, body }` — `body`/`do` is a keyword, so the field is `body`.
/// `binder` is the [`Binder`](crate::Binder) whose one/many cardinality picks
/// the body's anaphor: a one-binder binds
/// [`Reference::That`](crate::Reference::That), a many-binder binds
/// [`Selection::That`](crate::Selection::That).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct With {
    pub binder: crate::Binder,
    pub body: Box<Effect>,
}

/// `DivideAmong { amount, binder, body }` — see [`Effect::DivideAmong`].
/// `amount` is the total to split, `binder` the many-cardinality
/// [`Binder`](crate::Binder) of recipients (each bound as
/// [`Reference::It`](crate::Reference::It) in turn), and `body` the per-element
/// effect that reads [`Count::Allotment`](crate::Count::Allotment) for that
/// element's share. `body` is boxed to break the `Effect` → `DivideAmong` →
/// `Effect` size cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct DivideAmong {
    pub amount: crate::Count,
    pub binder: crate::Binder,
    pub body: Box<Effect>,
}

/// `Modal { choose, modes }` ([CR#700.2]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Modal {
    pub choose: ChooseSpec,
    pub modes: Vec<Mode>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Count;
    use crate::action::PlayerAction;
    use crate::mana::ManaSpec;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> Effect {
        crate::ron::options().from_str(source).unwrap()
    }
    fn write(effect: &Effect) -> String {
        crate::ron::options().to_string(effect).unwrap()
    }

    /// Wraps a bare player action in the implicit-you default — the form a
    /// player verb written bare in an effect slot reads as.
    fn act_by_you(pa: PlayerAction) -> Effect {
        Effect::Act(Action::By(Reference::You, pa))
    }

    /// Bare player verbs read flat as `By(You, …)`; source verbs read native.
    #[test]
    fn verbs_read_flat() {
        assert_eq!(
            read("Draw(Literal(1))"),
            act_by_you(PlayerAction::Draw(Count::Literal(1))),
        );
        assert_eq!(
            read("GainLife(Literal(3))"),
            act_by_you(PlayerAction::GainLife(Count::Literal(3))),
        );
        assert_eq!(
            read("Sacrifice(This)"),
            act_by_you(PlayerAction::Sacrifice(Reference::This)),
        );
        assert_eq!(
            read("DealDamage(Target(0), Literal(3))"),
            Effect::Act(Action::deal_damage(Reference::Target(0), Count::Literal(3),)),
        );
        assert_eq!(
            read("AddMana(Literal(1), AnyColor)"),
            act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::AnyColor.into()
            )),
        );
    }

    /// The source verbs read native; the player verbs read as `By(You, …)`.
    /// Verb patients are now a single bare [`Reference`].
    #[test]
    fn new_verbs_read_flat() {
        assert_eq!(
            read("Destroy(This)"),
            Effect::Act(Action::Destroy(Reference::This)),
        );
        assert_eq!(
            read("Tap(This)"),
            act_by_you(PlayerAction::Tap(Reference::This)),
        );
        assert_eq!(
            read("Discard(count: Literal(1))"),
            act_by_you(PlayerAction::Discard {
                count: Count::Literal(1),
                what: None
            }),
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
            Effect::Act(Action::Move(
                Reference::This,
                Destination::Library(Anchor::FromTop(Count::Literal(0))),
            )),
        );
    }

    /// An explicit player agent reads native — `By(Target(0), Draw(3))`.
    #[test]
    fn explicit_agent_reads_flat() {
        assert_eq!(
            read("By(Target(0), Draw(Literal(3)))"),
            Effect::Act(Action::By(
                Reference::Target(0),
                PlayerAction::Draw(Count::Literal(3)),
            )),
        );
    }

    /// Structural forms read flat (the inner-struct delegation) and the
    /// Option fields default to None.
    #[test]
    fn structural_forms_read_flat() {
        assert_eq!(
            read("Sequence([Draw(Literal(1)), GainLife(Literal(1))])"),
            Effect::Sequence(vec![
                act_by_you(PlayerAction::Draw(Count::Literal(1))),
                act_by_you(PlayerAction::GainLife(Count::Literal(1))),
            ]),
        );
        let may = read("May(effect: Draw(Literal(1)))");
        let Effect::May(may) = may else {
            panic!("expected May");
        };
        assert_eq!(
            *may.effect,
            act_by_you(PlayerAction::Draw(Count::Literal(1)))
        );
        assert!(may.if_did.is_none() && may.if_not.is_none());
    }

    #[test]
    fn act_serializes_flat() {
        // A `Count` literal writes bare — `1`, never `Literal(1)`.
        assert_eq!(
            write(&act_by_you(PlayerAction::Draw(Count::Literal(1)))),
            "Draw(1)"
        );
    }

    #[test]
    fn effects_round_trip() {
        let cases = [
            "Draw(Literal(1))",
            "GainLife(Literal(3))",
            "Sacrifice(This)",
            "By(Target(0),Draw(Literal(3)))",
            "DealDamage(Target(0),Literal(3))",
            "AddMana(Literal(1),AnyColor)",
            // Verb patients are a single bare `Reference` now.
            "Destroy(This)",
            "Sequence([Draw(Literal(1)),GainLife(Literal(1))])",
            "May(effect:Draw(Literal(1)))",
            // `Each.binder` is a many-`Binder` (the set of all creatures wrapped
            // in `Existing`), binding `It` per element.
            "Each(binder:Existing(Filter(Type(Creature))),effect:Draw(Literal(1)))",
            // Brainstorm's shape in the new model: choose 2 cards (a many-binder
            // `With`), then `Each` over the bound group (`Existing(That)`), moving
            // each onto the library via the `It` element. Core reader has no
            // macros, so the `Quantity` is the bare `Range` primitive
            // (`Exactly(2)` is the cards-layer macro spelling).
            "With(binder:Choose(Range(Literal(2),Literal(2)),InZone(Hand)),\
             body:Each(binder:Existing(That),\
             effect:Move(It,Library(FromTop(Literal(0))))))",
        ];
        for source in cases {
            let parsed = read(source);
            let written = write(&parsed);
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }

    /// `Unless::who` defaults to `You` when omitted (and is dropped from
    /// the written form); an explicit non-`You` payer round-trips.
    #[test]
    fn unless_who_defaults_to_you_and_round_trips() {
        let omitted = "Unless(effect:LoseLife(1),unless:[Mana([Generic(2)])])";
        let parsed = read(omitted);
        let Effect::Unless(u) = &parsed else {
            panic!("expected Unless");
        };
        assert_eq!(u.who, Reference::You, "omitted who defaults to You");
        assert_eq!(write(&parsed), omitted, "default who is omitted on write");

        let explicit = "Unless(effect:LoseLife(1),who:Target(0),unless:[Mana([Generic(2)])])";
        assert_eq!(write(&read(explicit)), explicit, "explicit who round-trips");
    }

    /// `MustPay` reads flat over the full `Cost` algebra, defaults `actor` to
    /// `You` (omitted on write), and round-trips — the Mana Leak shape
    /// ([CR#118.12a]) that supersedes `Unless`.
    #[test]
    fn must_pay_defaults_actor_and_round_trips() {
        // Mana Leak: "counter target spell unless its controller pays {3}".
        let mana_leak = "MustPay(actor:ControllerOf(Target(0)),cost:[Mana([Generic(3)])],or_else:Counter(Target(0)))";
        let parsed = read(mana_leak);
        let Effect::MustPay(m) = &parsed else {
            panic!("expected MustPay, got {parsed:?}");
        };
        assert_eq!(
            m.actor,
            Reference::ControllerOf(Box::new(Reference::Target(0)))
        );
        assert_eq!(
            m.cost.0.len(),
            1,
            "the full Cost carries the {{3}} component"
        );
        assert_eq!(write(&parsed), mana_leak, "Mana Leak shape round-trips");

        // Default actor (You) is omitted on write.
        let omitted = "MustPay(cost:[Mana([Generic(2)])],or_else:LoseLife(1))";
        let parsed = read(omitted);
        let Effect::MustPay(m) = &parsed else {
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
        let bare = "MayPay(cost:[Mana([Generic(1)])],and_then:Draw(1))";
        let parsed = read(bare);
        let Effect::MayPay(m) = &parsed else {
            panic!("expected MayPay, got {parsed:?}");
        };
        assert_eq!(m.actor, Reference::You);
        assert!(m.or_else.is_none());
        assert_eq!(write(&parsed), bare, "bare MayPay round-trips");

        // With an explicit actor and an "if you don't" branch.
        let full = "MayPay(actor:Target(0),cost:[Mana([Generic(2)])],and_then:Draw(2),or_else:LoseLife(1))";
        assert_eq!(write(&read(full)), full, "full MayPay round-trips");
    }

    /// `AdditionalCost { pay, body }` reads flat over the full `Cost` algebra
    /// and round-trips — the printed/nested additional-cost shape
    /// ([CR#601.2f,118.8]) whose body reads the paid object via the event
    /// references (Fling's "sacrifice a creature: ~ deals damage equal to
    /// its power", over the core primitives — no card-layer macros).
    #[test]
    fn additional_cost_reads_and_round_trips() {
        let src = "AdditionalCost(pay:[Do(Sacrifice(This))],body:DealDamage(Target(0),StatOf(EventObject,Power)))";
        let parsed = read(src);
        let Effect::AdditionalCost(ac) = &parsed else {
            panic!("expected AdditionalCost, got {parsed:?}");
        };
        assert_eq!(
            ac.pay.0.len(),
            1,
            "the additional cost carries the sacrifice"
        );
        assert_eq!(
            *ac.body,
            Effect::Act(Action::deal_damage(
                Reference::Target(0),
                Count::StatOf(Reference::EventObject, crate::Stat::Power),
            )),
            "the body reads the paid object via EventObject",
        );
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
        assert_eq!(write(&parsed), src, "writes back to the flat form");
    }

    #[test]
    fn unknown_names_error() {
        assert!(
            crate::ron::options()
                .from_str::<Effect>("Bogus(1)")
                .is_err()
        );
    }

    /// A `Targeted` wrapper declares its targets and scopes `Target(n)`
    /// over the inner effect; it reads flat through the newtype variant and
    /// round-trips ([CR#115.1,601.2c]).
    #[test]
    fn targeted_effect_reads_and_round_trips() {
        let src = "Targeted(targets:[Target(Range(Literal(1),Literal(1)),Type(Creature))],effect:DealDamage(Target(0),Literal(3)))";
        let parsed = read(src);
        let Effect::Targeted(te) = &parsed else {
            panic!("expected Targeted, got {parsed:?}");
        };
        assert_eq!(te.targets.len(), 1);
        assert_eq!(
            *te.effect,
            Effect::Act(Action::deal_damage(Reference::Target(0), Count::Literal(3),)),
        );
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
    }

    /// `With.binder` carries a [`Binder`](crate::Binder); a many-binder
    /// `Existing(<selection>)` binds the group as `Selection::That`.
    #[test]
    fn with_binds_a_group() {
        let v = read("With(binder:Existing(TopOfLibrary(count:2)),body:Sequence([]))");
        let Effect::With(w) = &v else {
            panic!("expected With, got {v:?}");
        };
        assert!(matches!(
            w.binder,
            crate::Binder::Existing(Selection::TopOfLibrary { .. })
        ));
        assert_eq!(read(&write(&v)), v);
    }

    /// `DivideAmong` reads flat through the newtype variant; its body reads the
    /// `Allotment` anaphor over the per-element `It` — divided damage
    /// (`DealDamage(It, Allotment)`) and divided counters round-trip
    /// ([CR#601.2d]). `binder` is a many-`Binder`.
    #[test]
    fn divide_among_reads_and_round_trips() {
        let damage = read(
            "DivideAmong(amount: 3, binder: Existing(Filter(Type(Creature))), \
             body: DealDamage(It, Allotment))",
        );
        let Effect::DivideAmong(d) = &damage else {
            panic!("expected DivideAmong, got {damage:?}");
        };
        assert_eq!(d.amount, Count::Literal(3));
        assert!(matches!(
            d.binder,
            crate::Binder::Existing(Selection::Filter(_))
        ));
        assert_eq!(
            *d.body,
            Effect::Act(Action::deal_damage(Reference::It, Count::Allotment,)),
        );
        assert_eq!(read(&write(&damage)), damage, "round-trip");

        // Divided counters: the same primitive, a different body, iterating the
        // With-bound group (`Existing(That)`).
        let counters = read(
            "DivideAmong(amount: X, binder: Existing(That), \
             body: PutCounters(It, P1P1Counter, Allotment))",
        );
        assert!(matches!(counters, Effect::DivideAmong(_)));
        assert_eq!(read(&write(&counters)), counters, "round-trip");
    }

    /// `Each` iterates a many-`Binder`, exposing each element as
    /// [`Reference::It`]; it reads flat and round-trips (the Idris
    /// `Each : Bindable b Many k -> …`).
    #[test]
    fn each_binds_via_binder() {
        let v = read("Each(binder:Existing(Filter(Type(Creature))),effect:Draw(Literal(1)))");
        let Effect::Each(e) = &v else {
            panic!("expected Each, got {v:?}");
        };
        assert!(matches!(
            e.binder,
            crate::Binder::Existing(Selection::Filter(_))
        ));
        assert_eq!(read(&write(&v)), v, "round-trip");

        // Iterating the With-bound group: `Each(Existing(That), …)` reads `It`.
        let over_group = read("Each(binder:Existing(That),effect:Destroy(It))");
        assert!(matches!(
            over_group,
            Effect::Each(ref e) if matches!(e.binder, crate::Binder::Existing(Selection::That)),
        ));
        assert_eq!(read(&write(&over_group)), over_group, "round-trip");
    }

    /// The one `That` anaphor is resolved BY SLOT: the same `That` token reads
    /// as [`Reference::That`] in a `Reference`-typed position and as
    /// [`Selection::That`] in a `Selection`-typed position — serde/RON picks
    /// the variant from the field's type. Both forms round-trip; there is
    /// no `Those` spelling. (The Idris `Reference.That` / `Selection.That`
    /// split.)
    #[test]
    fn that_resolves_by_slot() {
        // Reference slot: `Destroy(That)`'s patient is a single `Reference`.
        let reference_slot = read("Destroy(That)");
        assert_eq!(
            reference_slot,
            Effect::Act(Action::Destroy(Reference::That)),
            "`That` in a Reference slot is Reference::That",
        );
        assert_eq!(read(&write(&reference_slot)), reference_slot, "round-trip");

        // Selection slot: a many-`Binder` `Existing(That)` wraps a `Selection`.
        let selection_slot = read("With(binder:Existing(That),body:Sequence([]))");
        let Effect::With(w) = &selection_slot else {
            panic!("expected With, got {selection_slot:?}");
        };
        assert_eq!(
            w.binder,
            crate::Binder::Existing(Selection::That),
            "`That` in a Selection slot is Selection::That",
        );
        assert_eq!(read(&write(&selection_slot)), selection_slot, "round-trip");
    }
}
