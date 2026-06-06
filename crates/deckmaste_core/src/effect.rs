// Manual serde, not `#[serde(untagged)]` wrappers: untagged variants
// deserialize through `deserialize_any`, which never reaches the macro
// layer's `deserialize_enum` interception — effect macros would stop
// expanding at Effect positions. Dispatching by name over one combined
// variant list keeps the RON flat *and* the positions macro-aware.
//
// The `Act` compartment is never written to RON. Its payload (`Action`)
// serializes transparently so RON sees `DrawCards(1)`, not `Act(DrawCards(1))`.
// Structural variants (`Sequence`, `May`, …) join VARIANTS in Plan 5.

use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, EnumAccess, SeqAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::action::Action;
use crate::ident::IdentSeed;
use crate::mana::ManaSpec;
use crate::{Selection, Uint};

/// An effect that an ability produces. Compartmentalized in Rust; flat in RON
/// (`DrawCards(1)`, never `Act(DrawCards(1))`) via the manual serde below.
///
/// `Act(Action)` is the only compartment today; structural variants join
/// VARIANTS in Plan 5.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    Act(Action),
}

/// Every name an Effect position accepts. Must stay in sync with
/// `visit_enum` below (the drift-guard test catches missing arms).
/// Names are the Action variant names; `Act` never appears in RON.
const VARIANTS: &[&str] = &[
    "AddMana",
    "DealDamage",
    "DrawCards",
    "GainLife",
    "Sacrifice",
];

// Visitor for 2-field tuple variants (DealDamage and AddMana).
struct Pair<A, B>(A, B);

struct PairVisitor<A, B>(std::marker::PhantomData<(A, B)>);

impl<'de, A: Deserialize<'de>, B: Deserialize<'de>> Visitor<'de> for PairVisitor<A, B> {
    type Value = Pair<A, B>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("a 2-element tuple") }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let a = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let b = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(Pair(a, b))
    }
}

impl<'de> Deserialize<'de> for Effect {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EffectVisitor;

        impl<'de> Visitor<'de> for EffectVisitor {
            type Value = Effect;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("an effect") }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Effect, A::Error> {
                let (ident, v) = data.variant_seed(IdentSeed)?;
                // Adding an atom? Update VARIANTS above to match.
                Ok(match ident.as_str() {
                    "AddMana" => {
                        let Pair(n, spec) =
                            v.tuple_variant(2, PairVisitor::<Uint, ManaSpec>(PhantomData))?;
                        Effect::Act(Action::AddMana(n, spec))
                    }
                    "DealDamage" => {
                        let Pair(sel, n) =
                            v.tuple_variant(2, PairVisitor::<Selection, Uint>(PhantomData))?;
                        Effect::Act(Action::DealDamage(sel, n))
                    }
                    "DrawCards" => Effect::Act(Action::DrawCards(v.newtype_variant()?)),
                    "GainLife" => Effect::Act(Action::GainLife(v.newtype_variant()?)),
                    "Sacrifice" => Effect::Act(Action::Sacrifice(v.newtype_variant()?)),
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
        // variant, so the text stays flat (`DrawCards(1)`, not `Act(DrawCards(1))`).
        match self {
            Effect::Act(action) => action.serialize(serializer),
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
        assert_eq!(read("DrawCards(1)"), Effect::Act(Action::DrawCards(1)),);
        assert_eq!(read("GainLife(3)"), Effect::Act(Action::GainLife(3)),);
        assert_eq!(
            read("Sacrifice(That(This))"),
            Effect::Act(Action::Sacrifice(Selection::That(Reference::This))),
        );
        assert_eq!(
            read("DealDamage(That(Target(0)), 3)"),
            Effect::Act(Action::DealDamage(Selection::That(Reference::Target(0)), 3)),
        );
        assert_eq!(
            read("AddMana(1, AnyColor)"),
            Effect::Act(Action::AddMana(1, ManaSpec::AnyColor)),
        );
    }

    #[test]
    fn act_serializes_flat() {
        assert_eq!(write(&Effect::Act(Action::DrawCards(1))), "DrawCards(1)");
    }

    #[test]
    fn effects_round_trip() {
        let cases = [
            "DrawCards(1)",
            "GainLife(3)",
            "Sacrifice(That(This))",
            "DealDamage(That(Target(0)),3)",
            "AddMana(1,AnyColor)",
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
}
