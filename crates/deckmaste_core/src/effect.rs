use serde::{Deserialize, Serialize};

use crate::ability::TriggeredAbility;
use crate::action::Action;
use crate::continuous::{Duration, StaticEffect};
use crate::{ChooseSpec, Condition, Expand, Expansion, Filter, Mode, SupportsMacros};

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
/// (`MayEffect`, …), which read flat via `unwrap_variant_newtypes` and carry
/// the field defaults and shapes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Effect {
    /// A single intrinsic instruction (the `Act` compartment, transparent in
    /// RON).
    #[macro_ron(flatten)]
    Act(Action),
    /// Explicit "then" — ordered sub-effects ([CR#608.2c]).
    Sequence(Vec<Effect>),
    /// A one-shot-created continuous effect ([CR#611.2]).
    Continuously(ContinuouslyEffect),
    /// "You may [do]" ([CR#603,608]) — with "if you do"/"if you don't".
    May(MayEffect),
    /// "If [condition], [then]; otherwise [else]" ([CR#603.4]-style branch).
    If(IfEffect),
    /// "[do] unless [you pay]" ([CR#118.12a]).
    Unless(UnlessEffect),
    /// "For each [over], [do]" — binds the iterated object ([CR#608]).
    ForEach(ForEachEffect),
    /// A delayed triggered ability created on resolution ([CR#603.7]).
    Delayed(Box<TriggeredAbility>),
    /// A reflexive triggered ability created on resolution ([CR#603.12]).
    Reflexive(Box<TriggeredAbility>),
    /// A modal effect: choose modes, then apply them ([CR#700.2]). This is the
    /// realized form of the design's `Resolvable::Modal` — see the report.
    Modal(ModalEffect),
    /// A remembered `Effect` macro invocation (declared compound verbs like
    /// `Investigate`). Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<Effect>),
}

/// `Continuously { effect, duration }` ([CR#611.2]). `effect` is boxed to break
/// the `Effect` → `StaticEffect` → `Replacement` → `Effect` size cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct ContinuouslyEffect {
    pub effect: Box<StaticEffect>,
    pub duration: Duration,
}

/// `May { do, if_did, if_not }` — `do` is a keyword, so the field is `effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct MayEffect {
    pub effect: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_did: Option<Box<Effect>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_not: Option<Box<Effect>>,
}

/// `If { condition, then, else }` — `else` is a keyword, so the field is
/// `otherwise`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct IfEffect {
    pub condition: Condition,
    pub then: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otherwise: Option<Box<Effect>>,
}

/// `Unless { do, unless }` — `do` is a keyword, so the field is `effect`;
/// `unless` is the cost the affected player may pay to avoid it ([CR#118.12a]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct UnlessEffect {
    pub effect: Box<Effect>,
    pub unless: Vec<crate::CostComponent>,
}

/// `ForEach { over, do }` — `do` is a keyword, so the field is `effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct ForEachEffect {
    pub over: Filter,
    pub effect: Box<Effect>,
}

/// `Modal { choose, modes }` ([CR#700.2]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct ModalEffect {
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

    fn read(source: &str) -> Effect { crate::ron::options().from_str(source).unwrap() }
    fn write(effect: &Effect) -> String { crate::ron::options().to_string(effect).unwrap() }

    /// Wraps a bare player action in the implicit-you default — the form a
    /// player verb written bare in an effect slot reads as.
    fn act_by_you(pa: PlayerAction) -> Effect { Effect::Act(Action::By(Reference::You, pa)) }

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
            act_by_you(PlayerAction::AddMana(Count::Literal(1), ManaSpec::AnyColor)),
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
            read("Discard(Literal(1))"),
            act_by_you(PlayerAction::Discard(Count::Literal(1))),
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
        assert_eq!(
            write(&act_by_you(PlayerAction::Draw(Count::Literal(1)))),
            "Draw(Literal(1))"
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

    #[test]
    fn unknown_names_error() {
        assert!(
            crate::ron::options()
                .from_str::<Effect>("Bogus(1)")
                .is_err()
        );
    }
}
