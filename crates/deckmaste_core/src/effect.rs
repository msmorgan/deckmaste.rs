use serde::Deserialize;
use serde::Serialize;

use crate::ChooseSpec;
use crate::Condition;
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
    /// "[do] unless [you pay]" ([CR#118.12a]).
    Unless(Unless),
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
            Effect::Act(Action::DealDamage(
                Selection::Ref(Reference::Target(0)),
                Count::Literal(3)
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

    /// Brainstorm's second half: a bare `PutInLibrary` reads at an effect slot
    /// as the implicit-`You` default, with the position a `Count` (0 = top).
    /// The 2-tuple `(Selection, Count)` exercises the `Pair` visitor.
    #[test]
    fn put_in_library_reads_at_effect_slot() {
        assert_eq!(
            read("PutInLibrary(This, Literal(0))"),
            act_by_you(PlayerAction::PutInLibrary(
                Selection::Ref(Reference::This),
                Count::Literal(0),
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
            // Brainstorm's shape: choose 2 cards, put them on top (position 0).
            "PutInLibrary(Choose(Exactly(Literal(2)),InZone(Hand)),Literal(0))",
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
        let src = "Targeted(targets:[Target(Exactly(Literal(1)),Type(Creature))],effect:DealDamage(Target(0),Literal(3)))";
        let parsed = read(src);
        let Effect::Targeted(te) = &parsed else {
            panic!("expected Targeted, got {parsed:?}");
        };
        assert_eq!(te.targets.len(), 1);
        assert_eq!(
            *te.effect,
            Effect::Act(Action::DealDamage(
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
