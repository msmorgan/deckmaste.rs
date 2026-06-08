// Manual serde, not `#[serde(untagged)]` wrappers: untagged variants
// deserialize through `deserialize_any`, which never reaches the macro
// layer's `deserialize_enum` interception — effect macros would stop
// expanding at Effect positions. Dispatching by name over one combined
// variant list keeps the RON flat *and* the positions macro-aware.
//
// The `Act` compartment is never written to RON. Its payload (`Action`)
// serializes transparently so RON sees `Draw(1)`, not `Act(By(You, Draw(1)))`.
//
// The structural variants (`Sequence`, `May`, …) delegate to inner derived
// structs (`MayEffect`, …): each `visit_enum` arm is
// `Effect::May(v.newtype_variant()?)`, which reads flat via
// `unwrap_variant_newtypes`. This keeps the manual impl thin — the field
// defaults and shapes live on the inner structs' derives.

use std::fmt;

use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ability::TriggeredAbility;
use crate::action::{Action, PlayerAction};
use crate::continuous::{Duration, StaticEffect};
use crate::mana::ManaSpec;
use crate::{
    ChooseSpec, Condition, Count, Expansion, Filter, IdentSeed, Mode, Reference, Selection, Token,
};

/// An effect an ability produces ([CR#608]). Compartmentalized in Rust; flat in
/// RON (`Draw(1)`, never `Act(By(You, Draw(1)))`) via the manual serde below.
///
/// A single instruction stands bare (`effect: DealDamage(Target(0), 3)`); the
/// structural forms (`Sequence`, `May`, `If`, …) are the corpus's connective
/// tissue — data the engine interprets, never seen by the macro layer as
/// control flow.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    /// A single intrinsic instruction (the `Act` compartment, transparent in
    /// RON).
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
    Expanded(Expansion<Effect>),
}

/// `Continuously { effect, duration }` ([CR#611.2]). `effect` is boxed to break
/// the `Effect` → `StaticEffect` → `Replacement` → `Effect` size cycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ContinuouslyEffect {
    pub effect: Box<StaticEffect>,
    pub duration: Duration,
}

/// `May { do, if_did, if_not }` — `do` is a keyword, so the field is `effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct MayEffect {
    pub effect: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_did: Option<Box<Effect>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_not: Option<Box<Effect>>,
}

/// `If { condition, then, else }` — `else` is a keyword, so the field is
/// `otherwise`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct IfEffect {
    pub condition: Condition,
    pub then: Box<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otherwise: Option<Box<Effect>>,
}

/// `Unless { do, unless }` — `do` is a keyword, so the field is `effect`;
/// `unless` is the cost the affected player may pay to avoid it ([CR#118.12a]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UnlessEffect {
    pub effect: Box<Effect>,
    pub unless: Vec<crate::CostComponent>,
}

/// `ForEach { over, do }` — `do` is a keyword, so the field is `effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ForEachEffect {
    pub over: Filter,
    pub effect: Box<Effect>,
}

/// `Modal { choose, modes }` ([CR#700.2]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ModalEffect {
    pub choose: ChooseSpec,
    pub modes: Vec<Mode>,
}

/// Every name an Effect position accepts. Must stay in sync with `visit_enum`
/// below (the drift-guard test catches missing arms). The first group are
/// `Action` variant names (`Act` never appears in RON); the rest are the
/// structural variants.
const VARIANTS: &[&str] = &[
    // Source-agent Action verbs (the Act compartment, flat).
    "DealDamage",
    "Destroy",
    "ReturnToHand",
    // An explicit player-agent action.
    "By",
    // Bare player verbs — read as `By(You, …)`, the implicit-you default.
    "Draw",
    "Discard",
    "GainLife",
    "LoseLife",
    "AddMana",
    "Create",
    "Sacrifice",
    "Exile",
    "Tap",
    "Untap",
    // Structural forms.
    "Sequence",
    "Continuously",
    "May",
    "If",
    "Unless",
    "ForEach",
    "Delayed",
    "Reflexive",
    "Modal",
    "Expanded",
];

/// Wraps a bare player action in the implicit-you default — the form a player
/// verb written bare in an effect slot reads as.
fn act_by_you(pa: PlayerAction) -> Effect { Effect::Act(Action::By(Reference::You, pa)) }

impl<'de> Deserialize<'de> for Effect {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EffectVisitor;

        impl<'de> Visitor<'de> for EffectVisitor {
            type Value = Effect;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("an effect") }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Effect, A::Error> {
                use crate::de_util::Pair;

                let (ident, v) = data.variant_seed(IdentSeed)?;
                // Adding a verb or form? Update VARIANTS above to match.
                Ok(match ident.as_str() {
                    // --- Source-agent Action verbs (flat through Act) ---
                    "DealDamage" => {
                        let (sel, n) = v.tuple_variant(2, Pair::<Selection, Count>::new())?;
                        Effect::Act(Action::DealDamage(sel, n))
                    }
                    "Destroy" => Effect::Act(Action::Destroy(v.newtype_variant()?)),
                    "ReturnToHand" => Effect::Act(Action::ReturnToHand(v.newtype_variant()?)),
                    // An explicit player-agent action: `By(Target(0), Draw(3))`.
                    "By" => {
                        let (who, pa) =
                            v.tuple_variant(2, Pair::<Reference, PlayerAction>::new())?;
                        Effect::Act(Action::By(who, pa))
                    }
                    // --- Bare player verbs → `By(You, …)`, implicit-you ---
                    "Draw" => act_by_you(PlayerAction::Draw(v.newtype_variant()?)),
                    "Discard" => act_by_you(PlayerAction::Discard(v.newtype_variant()?)),
                    "GainLife" => act_by_you(PlayerAction::GainLife(v.newtype_variant()?)),
                    "LoseLife" => act_by_you(PlayerAction::LoseLife(v.newtype_variant()?)),
                    "AddMana" => {
                        let (n, spec) = v.tuple_variant(2, Pair::<Count, ManaSpec>::new())?;
                        act_by_you(PlayerAction::AddMana(n, spec))
                    }
                    "Create" => {
                        let (n, token) = v.tuple_variant(2, Pair::<Count, Token>::new())?;
                        act_by_you(PlayerAction::Create(n, token))
                    }
                    "Sacrifice" => act_by_you(PlayerAction::Sacrifice(v.newtype_variant()?)),
                    "Exile" => act_by_you(PlayerAction::Exile(v.newtype_variant()?)),
                    "Tap" => act_by_you(PlayerAction::Tap(v.newtype_variant()?)),
                    "Untap" => act_by_you(PlayerAction::Untap(v.newtype_variant()?)),
                    // --- Structural forms (inner-struct delegation) ---
                    "Sequence" => Effect::Sequence(v.newtype_variant()?),
                    "Continuously" => Effect::Continuously(v.newtype_variant()?),
                    "May" => Effect::May(v.newtype_variant()?),
                    "If" => Effect::If(v.newtype_variant()?),
                    "Unless" => Effect::Unless(v.newtype_variant()?),
                    "ForEach" => Effect::ForEach(v.newtype_variant()?),
                    "Delayed" => Effect::Delayed(v.newtype_variant()?),
                    "Reflexive" => Effect::Reflexive(v.newtype_variant()?),
                    "Modal" => Effect::Modal(v.newtype_variant()?),
                    "Expanded" => Effect::Expanded(v.newtype_variant()?),
                    _ => return Err(de::Error::unknown_variant(&ident, VARIANTS)),
                })
            }
        }

        deserializer.deserialize_enum("Effect", VARIANTS, EffectVisitor)
    }
}

impl Serialize for Effect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // The `Act` compartment is transparent: RON sees only the Action
        // variant. The structural variants serialize as their newtype
        // wrapper (flat via unwrap_variant_newtypes for the struct ones).
        match self {
            Effect::Act(action) => action.serialize(serializer),
            Effect::Sequence(es) => {
                serializer.serialize_newtype_variant("Effect", 13, "Sequence", es)
            }
            Effect::Continuously(e) => {
                serializer.serialize_newtype_variant("Effect", 14, "Continuously", e)
            }
            Effect::May(e) => serializer.serialize_newtype_variant("Effect", 15, "May", e),
            Effect::If(e) => serializer.serialize_newtype_variant("Effect", 16, "If", e),
            Effect::Unless(e) => serializer.serialize_newtype_variant("Effect", 17, "Unless", e),
            Effect::ForEach(e) => serializer.serialize_newtype_variant("Effect", 18, "ForEach", e),
            Effect::Delayed(t) => serializer.serialize_newtype_variant("Effect", 19, "Delayed", t),
            Effect::Reflexive(t) => {
                serializer.serialize_newtype_variant("Effect", 20, "Reflexive", t)
            }
            Effect::Modal(e) => serializer.serialize_newtype_variant("Effect", 21, "Modal", e),
            // The invocation, not the struct: `Expansion`'s Serialize emits it.
            Effect::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> Effect { crate::ron::options().from_str(source).unwrap() }
    fn write(effect: &Effect) -> String { crate::ron::options().to_string(effect).unwrap() }

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

    /// Every VARIANTS entry must be handled in `visit_enum`: a missing arm
    /// surfaces as serde's `unknown_variant` error, which the macro layer
    /// would otherwise misreport as a failed macro lookup. (The reverse
    /// drift — an arm missing from VARIANTS — can't be detected here; a
    /// comment at both sites guards it.)
    #[test]
    fn variants_list_matches_visit_enum() {
        for &name in VARIANTS {
            if let Err(error) = crate::ron::options().from_str::<Effect>(name) {
                let message = error.to_string();
                assert!(
                    !message.contains("Unexpected variant") && !message.contains("unknown variant"),
                    "VARIANTS entry `{name}` is not handled in visit_enum: {message}"
                );
            }
        }
    }

    // Silence unused-import warnings for Reference in case other tests drop.
    #[allow(unused)]
    fn _uses_reference() -> Reference { Reference::This }
}
