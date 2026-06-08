use std::fmt;

use serde::de::{self, Deserializer, EnumAccess, VariantAccess, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::{Expansion, Filter, IdentSeed, Quantity, Reference};

/// A Filter lifted into a resolution-time choice context: who picks, when,
/// how many ([CR#608.2d]). Verb object slots take exactly one `Selection`.
///
/// Targeting is NOT here — it lives in [`crate::TargetSpec`], the announce
/// list — because a target has legality recheck and retargeting rules the
/// other choice forms lack ([CR#115]). A [`Reference`] lifts into `Selection`
/// via [`Selection::Ref`] so a bound object can stand where a choice would.
///
/// `Ref` embeds `Reference` **untagged**: authored RON writes a reference
/// bare in a `Selection` slot (`Target(0)`, `This`, `ControllerOf(This)`) with
/// no `Ref(...)` wrapper, and reads back as `Ref(...)`. Both `Serialize` and
/// `Deserialize` are manual to carry that: the reader tries `Selection`'s own
/// variants and macros first, then falls through to a `Reference` (the macro
/// layer's `embeds_untagged` hook routes the fall-through here); the writer
/// emits the bare reference. The other variants' `Serialize` mirrors the
/// derive, and `Expanded` writes the invocation back.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selection {
    /// Every matching object, one at a time — distributive "each", no
    /// targeting, evaluated when the instruction applies ([CR#608.2d]).
    Each(Filter),
    /// All matching objects as one set — the shape continuous-effect
    /// scopes and set-wide instructions consume.
    Filter(Filter),
    /// A quantity of untargeted choices made at resolution ([CR#608.2d]).
    Choose(Quantity, Filter),
    /// A random selection of a quantity of matching objects.
    Random(Quantity, Filter),
    /// A bound object reference, lifted into a choice slot. Written bare in
    /// RON (no `Ref(...)` wrapper) — see the type docs.
    Ref(Reference),
    /// A remembered `Selection` macro invocation.
    Expanded(Expansion<Selection>),
}

/// `Selection`'s own variant names — the ones the reader recognizes before
/// falling through to a [`Reference`]. The macro layer's `embeds_untagged`
/// hook hands every other identifier-led value to `visit_newtype_struct`.
const VARIANTS: &[&str] = &["Each", "Filter", "Choose", "Random", "Expanded"];

impl<'de> Deserialize<'de> for Selection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SelectionVisitor;

        impl<'de> Visitor<'de> for SelectionVisitor {
            type Value = Selection;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a selection or a reference")
            }

            /// The untagged-embed fall-through: an identifier the enum channel
            /// didn't recognize as one of `Selection`'s own variants (or a
            /// `Selection` macro) arrives here as newtype content. Reading a
            /// `Reference` re-enters the macro layer under the `Reference`
            /// namespace, so bare `This`/`Target(0)`/`ControllerOf(This)` and
            /// `Reference` macros alike read and wrap in `Ref`.
            fn visit_newtype_struct<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                Ok(Selection::Ref(Reference::deserialize(deserializer)?))
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                let (ident, v) = data.variant_seed(IdentSeed)?;
                Ok(match ident.as_str() {
                    "Each" => Selection::Each(v.newtype_variant()?),
                    "Filter" => Selection::Filter(v.newtype_variant()?),
                    "Choose" => {
                        let (q, f) = v.tuple_variant(2, QuantityFilterVisitor)?;
                        Selection::Choose(q, f)
                    }
                    "Random" => {
                        let (q, f) = v.tuple_variant(2, QuantityFilterVisitor)?;
                        Selection::Random(q, f)
                    }
                    "Expanded" => Selection::Expanded(v.newtype_variant()?),
                    // Not one of Selection's own variants: lift a bare
                    // reference. (Reached only without the macro layer's
                    // embed hook — e.g. a plain `ron` read; with the hook the
                    // fall-through arrives at `visit_newtype_struct` above.)
                    _ => Selection::Ref(reference_from(ident.as_str(), v)?),
                })
            }
        }

        deserializer.deserialize_enum("Selection", VARIANTS, SelectionVisitor)
    }
}

/// Builds a [`Reference`] from an already-read variant tag and its access —
/// the fall-through path when no macro layer is present and the bare reference
/// arrives through `visit_enum`. Mirrors `Reference`'s own derived shapes.
fn reference_from<'de, A: VariantAccess<'de>>(ident: &str, v: A) -> Result<Reference, A::Error> {
    Ok(match ident {
        "This" => {
            v.unit_variant()?;
            Reference::This
        }
        "You" => {
            v.unit_variant()?;
            Reference::You
        }
        "Target" => Reference::Target(v.newtype_variant()?),
        "ThatObject" => {
            v.unit_variant()?;
            Reference::ThatObject
        }
        "ThatPlayer" => {
            v.unit_variant()?;
            Reference::ThatPlayer
        }
        "Bound" => Reference::Bound(v.newtype_variant()?),
        "Linked" => Reference::Linked(v.newtype_variant()?),
        "ControllerOf" => Reference::ControllerOf(v.newtype_variant()?),
        "OwnerOf" => Reference::OwnerOf(v.newtype_variant()?),
        "EnchantedObject" => {
            v.unit_variant()?;
            Reference::EnchantedObject
        }
        "EquippedCreature" => {
            v.unit_variant()?;
            Reference::EquippedCreature
        }
        "AttachedTo" => Reference::AttachedTo(v.newtype_variant()?),
        other => {
            return Err(de::Error::custom(format_args!(
                "`{other}` is neither a Selection variant nor a Reference"
            )));
        }
    })
}

/// Visitor for the 2-tuple `(Quantity, Filter)` content of `Choose`/`Random`.
struct QuantityFilterVisitor;

impl<'de> Visitor<'de> for QuantityFilterVisitor {
    type Value = (Quantity, Filter);

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a (Quantity, Filter) pair")
    }

    fn visit_seq<S: serde::de::SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let q = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let f = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok((q, f))
    }
}

impl Serialize for Selection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON. Mirrors the
        // shapes the derive produced, plus the `Ref` (untagged) and `Expanded`
        // (invocation) arms.
        match self {
            Selection::Each(f) => serializer.serialize_newtype_variant("Selection", 0, "Each", f),
            Selection::Filter(f) => {
                serializer.serialize_newtype_variant("Selection", 1, "Filter", f)
            }
            Selection::Choose(q, f) => {
                serializer.serialize_newtype_variant("Selection", 2, "Choose", &(q, f))
            }
            Selection::Random(q, f) => {
                serializer.serialize_newtype_variant("Selection", 3, "Random", &(q, f))
            }
            // Untagged: emit the reference directly, no `Ref(...)` wrapper.
            Selection::Ref(r) => r.serialize(serializer),
            // The invocation, not the struct.
            Selection::Expanded(e) => e.serialize(serializer),
        }
    }
}

/// Lifts a bound [`Reference`] into a `Selection`. Trivial now that the
/// reference embeds directly; kept for the call sites that build a selection
/// from a reference value.
impl From<Reference> for Selection {
    fn from(reference: Reference) -> Self { Selection::Ref(reference) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, Count, ObjectKind, Quantity, Type};

    fn read(source: &str) -> Selection { crate::ron::options().from_str(source).unwrap() }

    fn to_string(value: &Selection) -> String { crate::ron::options().to_string(value).unwrap() }

    #[test]
    fn quantifiers_wrap_filters() {
        assert_eq!(
            read("Each(Kind(Player))"),
            Selection::Each(Filter::Kind(ObjectKind::Player)),
        );
    }

    #[test]
    fn filter_variant_round_trip() {
        let v = Selection::Filter(Filter::Kind(ObjectKind::Player));
        assert_eq!(read(&to_string(&v)), v);
    }

    #[test]
    fn choose_single_round_trip() {
        let v = Selection::Choose(
            Quantity::Exactly(Count::Literal(1)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        );
        assert_eq!(read(&to_string(&v)), v);
    }

    /// References lift into Selection flat — `This`, `Target(0)`, … read
    /// directly into `Ref(...)`, no wrapper in the RON.
    #[test]
    fn references_lift_flat() {
        assert_eq!(read("This"), Selection::Ref(Reference::This));
        assert_eq!(read("Target(0)"), Selection::Ref(Reference::Target(0)));
        assert_eq!(
            read("ControllerOf(This)"),
            Selection::Ref(Reference::ControllerOf(Box::new(Reference::This))),
        );
    }

    /// `From<Reference>` is now the trivial `Ref` wrap.
    #[test]
    fn from_reference_wraps_in_ref() {
        assert_eq!(
            Selection::from(Reference::This),
            Selection::Ref(Reference::This),
        );
        assert_eq!(
            Selection::from(Reference::Target(2)),
            Selection::Ref(Reference::Target(2)),
        );
        assert_eq!(
            Selection::from(Reference::AttachedTo(Box::new(Reference::This))),
            Selection::Ref(Reference::AttachedTo(Box::new(Reference::This))),
        );
    }

    /// A bare reference round-trips through the embedded `Ref`: it writes
    /// without a `Ref(...)` wrapper and reads back to the same value.
    #[test]
    fn ref_round_trips_bare() {
        for v in [
            Selection::Ref(Reference::Target(0)),
            Selection::Ref(Reference::This),
            Selection::Ref(Reference::ControllerOf(Box::new(Reference::This))),
        ] {
            let written = to_string(&v);
            assert!(
                !written.contains("Ref("),
                "reference should write bare, got {written}"
            );
            assert_eq!(read(&written), v);
        }
    }
}
