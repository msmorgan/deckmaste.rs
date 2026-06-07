use std::fmt;

use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::replacement::{Prevention, Replacement};
use crate::{
    Ability, Color, Condition, CostComponent, Event, Expansion, Filter, Ident, IdentSeed, Quantity,
    Reference, Supertype, Type,
};

/// How long a one-shot-created continuous effect lasts (CR 611.2). Static
/// abilities don't carry this — their duration is implicit ("while it
/// functions", CR 611.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Duration {
    /// CR 611.2 ("until end of turn").
    UntilEndOfTurn,
    /// "Until your next turn".
    UntilYourNextTurn,
    /// "Until end of combat".
    UntilEndOfCombat,
    /// While a condition holds (with the never-started rule, CR 611.2b).
    While(Condition),
    /// Until an event happens (the engine pairs the undo one-shot, CR 610.3).
    UntilEvent(Event),
    /// For the rest of the game (CR 611.2).
    EndOfGame,
}

/// The set of objects a `Modify` applies to (CR 611.2c vs 611.3 — lock-in
/// is provenance the engine applies, not stored here).
///
/// `Of` wraps a single reference (the spec's dead `That` renamed); `These`
/// a fixed list; `Matching` a filter-shaped, possibly-floating set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Scope {
    /// One referenced object.
    Of(Reference),
    /// A fixed list of referenced objects.
    These(Vec<Reference>),
    /// Every object matching a filter (anthem-shaped).
    Matching(Filter),
}

/// A flat primitive characteristic-change op (CR 613). Layers are DERIVED
/// from the op, never written: `Add*` stats → 7c, `Set*` stats → 7b (7a when
/// CDA-flagged), `Switch` → 7d, types → 4, colors → 5, abilities → 6,
/// controller → 2, text → 3 (CR 613.1). One effect's `changes` is a list
/// because it can span layers applied to the same set (CR 613.6).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Modification {
    SetPower(Quantity),
    AddPower(Quantity),
    SetToughness(Quantity),
    AddToughness(Quantity),
    /// Switch power and toughness (CR 613.4d).
    SwitchPowerToughness,
    SetColors(Vec<Color>),
    AddColors(Vec<Color>),
    SetCardTypes(Vec<Type>),
    AddCardTypes(Vec<Type>),
    /// Subtypes by name (the class is derivable from the values, CR 205.1b).
    SetSubtypes(Vec<Ident>),
    AddSubtypes(Vec<Ident>),
    SetSupertypes(Vec<Supertype>),
    AddSupertypes(Vec<Supertype>),
    /// Gain an ability (CR 613.1f). Boxed: `Ability` is the enum's largest
    /// variant by far, so indirection keeps `Modification` small.
    GainAbility(Box<Ability>),
    /// Lose a named keyword ability (CR 613.1f).
    LoseAbility(Ident),
    /// Lose all abilities (CR 613.1f).
    LoseAllAbilities,
    /// Can't have or gain the named ability (CR 613.1f).
    CantHaveAbility(Ident),
    /// Change controller (CR 613.1b).
    SetController(Reference),
    /// Change text (CR 613.1c).
    SetText(String),
    /// Set base loyalty (CR 613.1d, planeswalker).
    SetBaseLoyalty(Quantity),
    /// Set base defense (battle).
    SetBaseDefense(Quantity),
    /// The CR 305.7 bundle: replace land types ∧ lose printed abilities ∧
    /// gain the basic-land mana ability (Blood Moon). One intrinsic, not
    /// reachable from the plain `Set*` ops.
    BecomeBasicLandType(Vec<Ident>),
}

/// What an object can't do (CR 509.1b — absolute, asymmetric with
/// `Requirement`). Evasion abilities are Restrictions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Restriction {
    CantAttack,
    CantBlock,
    /// CR 702.9 (Flying-style): can't be blocked except by matching blockers.
    CantBeBlockedExceptBy(Filter),
    CantBeTargetedBy(Filter),
    CantCastSpells,
}

/// What an object must do (CR 509.1c — maximized but violable). Goaded's
/// "attacks each combat if able" is a Requirement.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Requirement {
    AttacksEachCombatIfAble,
    MustBlock(Filter),
}

/// A permission an object grants (CR 611.3d "as though"). Flash-likes,
/// cast-from-other-zones.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Permission {
    MayCastFrom(crate::Zone),
    HasFlash,
}

/// A cost modification (CR 118.7).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CostChange {
    Increase(Vec<CostComponent>),
    Reduce(Vec<CostComponent>),
}

/// The shared currency between an "anthem" static ability and a "+3/+3 until
/// end of turn" one-shot (CR 611). The difference is who wraps it: a static
/// ability (`StaticAbility`) or a one-shot `Effect::Continuously`.
///
/// Manual serde for macro interception (it bears `Expanded`): unknown names
/// at `StaticEffect` positions fall through to the macro layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticEffect {
    /// Change a scope's characteristics (anthems, pumps).
    Modify {
        of: Scope,
        changes: Vec<Modification>,
    },
    /// A "can't" (CR 509.1b).
    Restriction(Restriction),
    /// A "must" (CR 509.1c).
    Requirement(Requirement),
    /// An "as though" / "may" permission (CR 611.3d).
    Permission(Permission),
    /// A cost modifier (CR 118.7).
    CostModifier { of: Filter, change: CostChange },
    /// A replacement effect (CR 614).
    Replacement(Replacement),
    /// A prevention effect (CR 615).
    Prevention(Prevention),
    /// A remembered `StaticEffect` macro invocation. Serialized as the
    /// invocation, not the struct.
    Expanded(Expansion<StaticEffect>),
}

/// Every name a `StaticEffect` position accepts, compartments flattened. Must
/// stay in sync with `visit_enum` (the drift-guard test catches missing arms).
const VARIANTS: &[&str] = &[
    "Modify",
    "Restriction",
    "Requirement",
    "Permission",
    "CostModifier",
    "Replacement",
    "Prevention",
    "Expanded",
];

/// `Modify`, deserialized as its own struct (newtype-variant delegation,
/// flat in RON via `unwrap_variant_newtypes`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct Modify {
    of: Scope,
    changes: Vec<Modification>,
}

/// `CostModifier`, deserialized as its own struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct CostModifier {
    of: Filter,
    change: CostChange,
}

impl<'de> Deserialize<'de> for StaticEffect {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StaticEffectVisitor;

        impl<'de> Visitor<'de> for StaticEffectVisitor {
            type Value = StaticEffect;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a static effect")
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<StaticEffect, A::Error> {
                let (ident, v) = data.variant_seed(IdentSeed)?;
                // Adding a variant? Update VARIANTS above to match.
                Ok(match ident.as_str() {
                    "Modify" => {
                        let m: Modify = v.newtype_variant()?;
                        StaticEffect::Modify {
                            of: m.of,
                            changes: m.changes,
                        }
                    }
                    "Restriction" => StaticEffect::Restriction(v.newtype_variant()?),
                    "Requirement" => StaticEffect::Requirement(v.newtype_variant()?),
                    "Permission" => StaticEffect::Permission(v.newtype_variant()?),
                    "CostModifier" => {
                        let c: CostModifier = v.newtype_variant()?;
                        StaticEffect::CostModifier {
                            of: c.of,
                            change: c.change,
                        }
                    }
                    "Replacement" => StaticEffect::Replacement(v.newtype_variant()?),
                    "Prevention" => StaticEffect::Prevention(v.newtype_variant()?),
                    "Expanded" => StaticEffect::Expanded(v.newtype_variant()?),
                    _ => return Err(de::Error::unknown_variant(&ident, VARIANTS)),
                })
            }
        }

        deserializer.deserialize_enum("StaticEffect", VARIANTS, StaticEffectVisitor)
    }
}

impl Serialize for StaticEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StaticEffect::Modify { of, changes } => {
                let m = Modify {
                    of: of.clone(),
                    changes: changes.clone(),
                };
                serializer.serialize_newtype_variant("StaticEffect", 0, "Modify", &m)
            }
            StaticEffect::Restriction(r) => {
                serializer.serialize_newtype_variant("StaticEffect", 1, "Restriction", r)
            }
            StaticEffect::Requirement(r) => {
                serializer.serialize_newtype_variant("StaticEffect", 2, "Requirement", r)
            }
            StaticEffect::Permission(p) => {
                serializer.serialize_newtype_variant("StaticEffect", 3, "Permission", p)
            }
            StaticEffect::CostModifier { of, change } => {
                let c = CostModifier {
                    of: of.clone(),
                    change: change.clone(),
                };
                serializer.serialize_newtype_variant("StaticEffect", 4, "CostModifier", &c)
            }
            StaticEffect::Replacement(r) => {
                serializer.serialize_newtype_variant("StaticEffect", 5, "Replacement", r)
            }
            StaticEffect::Prevention(p) => {
                serializer.serialize_newtype_variant("StaticEffect", 6, "Prevention", p)
            }
            // The invocation, not the struct: `Expansion`'s Serialize emits it.
            StaticEffect::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str) -> StaticEffect { crate::ron::options().from_str(source).unwrap() }

    /// The anthem shape reads flat and round-trips.
    #[test]
    fn modify_reads_flat() {
        let parsed = read(
            "Modify(of: Matching(Type(Creature)), changes: [AddPower(Literal(1)), AddToughness(Literal(1))])",
        );
        assert_eq!(
            parsed,
            StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(crate::CharacteristicFilter::Type(
                    Type::Creature
                ))),
                changes: vec![
                    Modification::AddPower(Quantity::Literal(1)),
                    Modification::AddToughness(Quantity::Literal(1)),
                ],
            },
        );
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }

    #[test]
    fn restriction_reads_flat() {
        assert_eq!(
            read("Restriction(CantAttack)"),
            StaticEffect::Restriction(Restriction::CantAttack),
        );
    }

    #[test]
    fn variants_list_matches_visit_enum() {
        for &name in VARIANTS {
            if let Err(error) = crate::ron::options().from_str::<StaticEffect>(name) {
                let message = error.to_string();
                assert!(
                    !message.contains("Unexpected variant") && !message.contains("unknown variant"),
                    "VARIANTS entry `{name}` is not handled in visit_enum: {message}"
                );
            }
        }
    }
}
