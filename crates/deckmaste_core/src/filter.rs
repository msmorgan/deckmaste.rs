use std::fmt;

use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ident::IdentSeed;
use crate::{Ident, Reference, Supertype, Type, Zone};

/// What kind of object something is (CR 109.1). Players are objects here
/// too — the engine gives players ObjectIds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ObjectKind {
    Card,
    Emblem,
    Permanent,
    Player,
    Spell,
    Token,
}

/// Characteristic atoms (CR 109.3): facts printed on or defined for the
/// object. `Subtype` filters by *name* — validating that the name is a
/// declared subtype is a lint, not a parse concern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CharacteristicFilter {
    Type(Type),
    Subtype(Ident),
    Supertype(Supertype),
}

/// State atoms: where the object is and what's on it — not
/// characteristics (CR 110.5a, 122.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StateFilter {
    InZone(Zone),
}

/// Structural relations the engine owns. Relations are
/// implicitly existential: `Controller(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RelationFilter {
    Controller(Box<Filter>),
    Owner(Box<Filter>),
    OpponentOf(Box<Filter>),
}

/// A predicate over game objects, players included. Compartmentalized in
/// Rust; flat in RON (`Type(Creature)`, never
/// `Characteristic(Type(Creature))`) via the manual serde impls below.
///
/// Conjunction is explicit (`AllOf`) — an enum position never carries a
/// bare list. Canonical filters are context-free-correct: state the whole
/// predicate even where engine context would make parts redundant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    Kind(ObjectKind),
    Characteristic(CharacteristicFilter),
    State(StateFilter),
    Relation(RelationFilter),
    Is(Reference),
    AllOf(Vec<Filter>),
    OneOf(Vec<Filter>),
    Not(Box<Filter>),
}

/// Every name a Filter position accepts, compartments flattened: the
/// variant list the macro layer's enum interception checks before trying
/// Filter macros. Names must stay globally unique across compartments.
const VARIANTS: &[&str] = &[
    "Kind",
    "Type",
    "Subtype",
    "Supertype",
    "InZone",
    "Controller",
    "Owner",
    "OpponentOf",
    "Is",
    "AllOf",
    "OneOf",
    "Not",
];

// Manual serde, not `#[serde(untagged)]` wrappers: untagged variants
// deserialize through `deserialize_any`, which never reaches the macro
// layer's `deserialize_enum` interception — `AnyTarget` would stop
// expanding at Filter positions. Dispatching by name over one combined
// variant list keeps the RON flat *and* the positions macro-aware.
impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FilterVisitor;

        impl<'de> Visitor<'de> for FilterVisitor {
            type Value = Filter;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a filter")
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Filter, A::Error> {
                use CharacteristicFilter as C;
                use RelationFilter as R;
                use StateFilter as S;

                let (ident, v) = data.variant_seed(IdentSeed)?;
                Ok(match ident.as_str() {
                    // Adding an atom? Update VARIANTS above to match.
                    "Kind" => Filter::Kind(v.newtype_variant()?),
                    "Type" => Filter::Characteristic(C::Type(v.newtype_variant()?)),
                    "Subtype" => Filter::Characteristic(C::Subtype(v.newtype_variant()?)),
                    "Supertype" => Filter::Characteristic(C::Supertype(v.newtype_variant()?)),
                    "InZone" => Filter::State(S::InZone(v.newtype_variant()?)),
                    "Controller" => Filter::Relation(R::Controller(v.newtype_variant()?)),
                    "Owner" => Filter::Relation(R::Owner(v.newtype_variant()?)),
                    "OpponentOf" => Filter::Relation(R::OpponentOf(v.newtype_variant()?)),
                    "Is" => Filter::Is(v.newtype_variant()?),
                    "AllOf" => Filter::AllOf(v.newtype_variant()?),
                    "OneOf" => Filter::OneOf(v.newtype_variant()?),
                    "Not" => Filter::Not(v.newtype_variant()?),
                    _ => return Err(de::Error::unknown_variant(&ident, VARIANTS)),
                })
            }
        }

        deserializer.deserialize_enum("Filter", VARIANTS, FilterVisitor)
    }
}

impl Serialize for Filter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_newtype_variant's index argument is ignored by RON.
        match self {
            Filter::Kind(kind) => {
                serializer.serialize_newtype_variant("Filter", 0, "Kind", kind)
            }
            // The compartments serialize transparently: RON writes only the
            // inner variant, so the text stays flat.
            Filter::Characteristic(c) => c.serialize(serializer),
            Filter::State(s) => s.serialize(serializer),
            Filter::Relation(r) => r.serialize(serializer),
            Filter::Is(r) => serializer.serialize_newtype_variant("Filter", 8, "Is", r),
            Filter::AllOf(fs) => {
                serializer.serialize_newtype_variant("Filter", 9, "AllOf", fs)
            }
            Filter::OneOf(fs) => {
                serializer.serialize_newtype_variant("Filter", 10, "OneOf", fs)
            }
            Filter::Not(f) => serializer.serialize_newtype_variant("Filter", 11, "Not", f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Supertype, Type, Zone};

    fn read(source: &str) -> Filter {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn atoms_read_flat() {
        assert_eq!(
            read("Type(Creature)"),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        );
        assert_eq!(
            read(r#"Subtype("Forest")"#),
            Filter::Characteristic(CharacteristicFilter::Subtype("Forest".into())),
        );
        assert_eq!(
            read("Supertype(Basic)"),
            Filter::Characteristic(CharacteristicFilter::Supertype(Supertype::Basic)),
        );
        assert_eq!(
            read("InZone(Battlefield)"),
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
        );
        assert_eq!(read("Kind(Player)"), Filter::Kind(ObjectKind::Player));
    }

    #[test]
    fn combinators_nest() {
        assert_eq!(
            read("AllOf([Kind(Permanent), Type(Creature)])"),
            Filter::AllOf(vec![
                Filter::Kind(ObjectKind::Permanent),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ]),
        );
        assert_eq!(
            read("Not(Kind(Player))"),
            Filter::Not(Box::new(Filter::Kind(ObjectKind::Player))),
        );
    }

    #[test]
    fn relations_take_filters() {
        use crate::Reference;
        assert_eq!(
            read("Controller(Is(You))"),
            Filter::Relation(RelationFilter::Controller(Box::new(Filter::Is(
                Reference::You
            )))),
        );
    }

    /// The compartment wrappers must not appear in the text: Rust nests,
    /// RON stays flat.
    #[test]
    fn serialization_stays_flat() {
        let filter = Filter::Characteristic(CharacteristicFilter::Type(Type::Creature));
        assert_eq!(
            crate::ron::options().to_string(&filter).unwrap(),
            "Type(Creature)"
        );
    }

    #[test]
    fn filters_round_trip() {
        let source = "OneOf([AllOf([Kind(Permanent),Type(Battle)]),Kind(Player)])";
        let parsed = read(source);
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }

    #[test]
    fn unknown_names_error() {
        assert!(
            crate::ron::options().from_str::<Filter>("Bogus(1)").is_err()
        );
    }

    /// Every VARIANTS entry must be handled in visit_enum: a missing arm
    /// surfaces as serde's unknown_variant error, which the macro layer
    /// would otherwise misreport as a failed macro lookup. (The reverse
    /// drift — an arm missing from VARIANTS — can't be detected here; a
    /// comment at both sites guards it.)
    #[test]
    fn variants_list_matches_visit_enum() {
        for &name in VARIANTS {
            if let Err(error) = crate::ron::options().from_str::<Filter>(name) {
                let message = error.to_string();
                assert!(
                    !message.contains("Unexpected variant") && !message.contains("unknown variant"),
                    "VARIANTS entry `{name}` is not handled in visit_enum: {message}"
                );
            }
        }
    }

    /// The compartment Serialize delegation must produce text the
    /// Deserialize accepts back — every compartment, not just the ones the
    /// other tests happen to touch.
    #[test]
    fn compartment_round_trips() {
        let cases = [
            "Controller(Is(You))",
            "Owner(Kind(Player))",
            "OpponentOf(Kind(Player))",
            "InZone(Battlefield)",
            r#"Subtype("Forest")"#,
            "Supertype(Basic)",
            "Not(Kind(Player))",
            "Is(You)",
        ];
        for source in cases {
            let parsed = read(source);
            let written = crate::ron::options().to_string(&parsed).unwrap();
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }
}
