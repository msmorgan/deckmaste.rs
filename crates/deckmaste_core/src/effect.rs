use serde::Deserialize;
use serde::Serialize;

use crate::ChooseSpec;
use crate::Condition;
use crate::Cost;
use crate::Expand;
use crate::Expansion;
use crate::Filter;
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
    /// "For each [over], [do]" — binds the iterated object ([CR#608]).
    ForEach(ForEach),
    /// `With(selection, body)` — binds the WHOLE ordered `selection` into the
    /// frame as the plural anaphor `Those`, then runs `body` once. Never
    /// distributes (that is `Each`/`ForEach`, which bind singular `That`).
    /// `This` never rebinds; the moving roles are `That` / `Those`.
    With(With),
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

/// `ForEach { over, do }` — `do` is a keyword, so the field is `effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ForEach {
    pub over: Filter,
    pub effect: Box<Effect>,
}

/// `With { selection, body }` — `body`/`do` is a keyword, so the field is
/// `body`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct With {
    pub selection: crate::Selection,
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
            act_by_you(PlayerAction::Sacrifice(Selection::Ref(Reference::This))),
        );
        assert_eq!(
            read("DealDamage(Target(0), Literal(3))"),
            Effect::Act(Action::deal_damage(
                Selection::Ref(Reference::Target(0)),
                Count::Literal(3),
            )),
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
    #[test]
    fn new_verbs_read_flat() {
        assert_eq!(
            read("Destroy(Each(Type(Creature)))"),
            Effect::Act(Action::Destroy(Selection::Each(Filter::Characteristic(
                crate::CharacteristicFilter::Type(crate::Type::Creature)
            )))),
        );
        assert_eq!(
            read("Tap(This)"),
            act_by_you(PlayerAction::Tap(Selection::Ref(Reference::This))),
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
                Selection::Ref(Reference::This),
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
            "Destroy(Each(Type(Creature)))",
            "Sequence([Draw(Literal(1)),GainLife(Literal(1))])",
            "May(effect:Draw(Literal(1)))",
            "ForEach(over:Type(Creature),effect:Draw(Literal(1)))",
            // Brainstorm's shape: choose 2 cards, put them on top of library.
            // Core reader has no macros, so the `Quantity` is the bare `Range`
            // primitive (`Exactly(2)` is the cards-layer macro spelling).
            "Move(Choose(Range(Literal(2),Literal(2)),InZone(Hand)),Library(FromTop(Literal(0))))",
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
            Effect::Act(Action::deal_damage(
                Selection::Ref(Reference::Target(0)),
                Count::Literal(3),
            )),
        );
        assert_eq!(read(&write(&parsed)), parsed, "round-trip");
    }

    #[test]
    fn with_binds_a_group() {
        let v = read("With(selection:TopOfLibrary(count:2),body:Sequence([]))");
        assert!(matches!(v, Effect::With(_)));
        assert_eq!(read(&write(&v)), v);
    }
}
