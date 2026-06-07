// Manual serde, not `#[serde(untagged)]` wrappers: untagged variants
// deserialize through `deserialize_any`, which never reaches the macro
// layer's `deserialize_enum` interception — effect macros would stop
// expanding at Effect positions. Dispatching by name over one combined
// variant list keeps the RON flat *and* the positions macro-aware.
//
// The `Act` compartment is never written to RON. Its payload (`Action`)
// serializes transparently so RON sees `DrawCards(1)`, not `Act(DrawCards(1))`.
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
use crate::action::Action;
use crate::continuous::{Duration, StaticEffect};
use crate::mana::ManaSpec;
use crate::{
    ChooseSpec, Condition, Expansion, Filter, IdentSeed, Mode, Quantity, Selection, Token,
};

/// An effect an ability produces (CR 608). Compartmentalized in Rust; flat in
/// RON (`DrawCards(1)`, never `Act(DrawCards(1))`) via the manual serde below.
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
    /// Explicit "then" — ordered sub-effects (CR 608.2c).
    Sequence(Vec<Effect>),
    /// A one-shot-created continuous effect (CR 611.2).
    Continuously(ContinuouslyEffect),
    /// "You may [do]" (CR 603.x, 608) — with "if you do"/"if you don't".
    May(MayEffect),
    /// "If [condition], [then]; otherwise [else]" (CR 603.4-style branch).
    If(IfEffect),
    /// "[do] unless [you pay]" (CR 118.12a).
    Unless(UnlessEffect),
    /// "For each [over], [do]" — binds the iterated object (CR 608).
    ForEach(ForEachEffect),
    /// A delayed triggered ability created on resolution (CR 603.7).
    Delayed(Box<TriggeredAbility>),
    /// A reflexive triggered ability created on resolution (CR 603.12).
    Reflexive(Box<TriggeredAbility>),
    /// A modal effect: choose modes, then apply them (CR 700.2). This is the
    /// realized form of the design's `Resolvable::Modal` — see the report.
    Modal(ModalEffect),
    /// A remembered `Effect` macro invocation (declared compound verbs like
    /// `Investigate`). Serialized as the invocation, not the struct.
    Expanded(Expansion<Effect>),
}

/// `Continuously { effect, duration }` (CR 611.2). `effect` is boxed to break
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
/// `unless` is the cost the affected player may pay to avoid it (CR 118.12a).
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

/// `Modal { choose, modes }` (CR 700.2).
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
    // Action verbs (the Act compartment, flat).
    "AddMana",
    "Create",
    "DealDamage",
    "Destroy",
    "Discard",
    "DrawCards",
    "Exile",
    "GainLife",
    "LoseLife",
    "ReturnToHand",
    "Sacrifice",
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
                    // --- Action verbs (flat through the Act compartment) ---
                    "AddMana" => {
                        let (n, spec) = v.tuple_variant(2, Pair::<Quantity, ManaSpec>::new())?;
                        Effect::Act(Action::AddMana(n, spec))
                    }
                    "Create" => {
                        let (n, token) = v.tuple_variant(2, Pair::<Quantity, Token>::new())?;
                        Effect::Act(Action::Create(n, token))
                    }
                    "DealDamage" => {
                        let (sel, n) = v.tuple_variant(2, Pair::<Selection, Quantity>::new())?;
                        Effect::Act(Action::DealDamage(sel, n))
                    }
                    "Destroy" => Effect::Act(Action::Destroy(v.newtype_variant()?)),
                    "Discard" => Effect::Act(Action::Discard(v.newtype_variant()?)),
                    "DrawCards" => Effect::Act(Action::DrawCards(v.newtype_variant()?)),
                    "Exile" => Effect::Act(Action::Exile(v.newtype_variant()?)),
                    "GainLife" => Effect::Act(Action::GainLife(v.newtype_variant()?)),
                    "LoseLife" => Effect::Act(Action::LoseLife(v.newtype_variant()?)),
                    "ReturnToHand" => Effect::Act(Action::ReturnToHand(v.newtype_variant()?)),
                    "Sacrifice" => Effect::Act(Action::Sacrifice(v.newtype_variant()?)),
                    "Tap" => Effect::Act(Action::Tap(v.newtype_variant()?)),
                    "Untap" => Effect::Act(Action::Untap(v.newtype_variant()?)),
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

    #[test]
    fn verbs_read_flat() {
        assert_eq!(
            read("DrawCards(Literal(1))"),
            Effect::Act(Action::DrawCards(Quantity::Literal(1))),
        );
        assert_eq!(
            read("GainLife(Literal(3))"),
            Effect::Act(Action::GainLife(Quantity::Literal(3))),
        );
        assert_eq!(
            read("Sacrifice(This)"),
            Effect::Act(Action::Sacrifice(Selection::This)),
        );
        assert_eq!(
            read("DealDamage(Target(0), Literal(3))"),
            Effect::Act(Action::DealDamage(
                Selection::Target(0),
                Quantity::Literal(3)
            )),
        );
        assert_eq!(
            read("AddMana(Literal(1), AnyColor)"),
            Effect::Act(Action::AddMana(Quantity::Literal(1), ManaSpec::AnyColor)),
        );
    }

    /// The new intrinsic verbs read flat too.
    #[test]
    fn new_verbs_read_flat() {
        assert_eq!(
            read("Destroy(Each(Type(Creature)))"),
            Effect::Act(Action::Destroy(Selection::Each(Filter::Characteristic(
                crate::CharacteristicFilter::Type(crate::Type::Creature)
            )))),
        );
        assert_eq!(read("Tap(This)"), Effect::Act(Action::Tap(Selection::This)),);
        assert_eq!(
            read("Discard(Literal(1))"),
            Effect::Act(Action::Discard(Quantity::Literal(1))),
        );
    }

    /// Structural forms read flat (the inner-struct delegation) and the
    /// Option fields default to None.
    #[test]
    fn structural_forms_read_flat() {
        assert_eq!(
            read("Sequence([DrawCards(Literal(1)), GainLife(Literal(1))])"),
            Effect::Sequence(vec![
                Effect::Act(Action::DrawCards(Quantity::Literal(1))),
                Effect::Act(Action::GainLife(Quantity::Literal(1))),
            ]),
        );
        let may = read("May(effect: DrawCards(Literal(1)))");
        let Effect::May(may) = may else {
            panic!("expected May");
        };
        assert_eq!(
            *may.effect,
            Effect::Act(Action::DrawCards(Quantity::Literal(1)))
        );
        assert!(may.if_did.is_none() && may.if_not.is_none());
    }

    #[test]
    fn act_serializes_flat() {
        assert_eq!(
            write(&Effect::Act(Action::DrawCards(Quantity::Literal(1)))),
            "DrawCards(Literal(1))"
        );
    }

    #[test]
    fn effects_round_trip() {
        let cases = [
            "DrawCards(Literal(1))",
            "GainLife(Literal(3))",
            "Sacrifice(This)",
            "DealDamage(Target(0),Literal(3))",
            "AddMana(Literal(1),AnyColor)",
            "Destroy(Each(Type(Creature)))",
            "Sequence([DrawCards(Literal(1)),GainLife(Literal(1))])",
            "May(effect:DrawCards(Literal(1)))",
            "ForEach(over:Type(Creature),effect:DrawCards(Literal(1)))",
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
