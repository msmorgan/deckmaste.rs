use std::fmt;

use serde::de::{self, EnumAccess, SeqAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ident::IdentSeed;
use crate::{
    Cmp, Color, Expansion, Ident, Quantity, Reference, Stat, Status, Supertype, Type, Zone,
};

/// What kind of object something is (CR 109.1). Players are objects here
/// too — the engine gives players `ObjectId`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ObjectKind {
    Card,
    Emblem,
    Player,
    Spell,
    Token,
}

/// Characteristic atoms (CR 109.3): facts printed on or defined for the
/// object. `Subtype`/`Named`/`HasAbility` filter by *name* — validating that
/// the name is declared is a lint, not a parse concern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CharacteristicFilter {
    Type(Type),
    Subtype(Ident),
    Supertype(Supertype),
    /// The object is the given color (CR 105.2, 202.2).
    ColorIs(Color),
    /// The object has the given name (CR 201).
    Named(Ident),
    /// A printed/defined stat compares as stated, e.g.
    /// `Stat(Power, AtLeast, 3)` — mana value via `Stat(ManaValue, …)`
    /// (CR 208, 202.3).
    Stat(Stat, Cmp, Quantity),
    /// The object has the named keyword ability (CR 702).
    HasAbility(Ident),
}

/// State atoms: where the object is and what's on it — not
/// characteristics (CR 110.5a, 122.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StateFilter {
    InZone(Zone),
    /// The object's status (CR 110.5).
    Status(Status),
    /// The object has at least one counter of the named kind (CR 122.1).
    HasCounter(Ident),
    /// The object carries the named designation (CR 109.4) — derived
    /// designations (`Designated(Modified)`) work with no special casing.
    Designated(Ident),
    /// The object is related to a matching object by a named, declared
    /// relation (CR 607 family). Box because the inner predicate is a Filter.
    RelatedBy(Ident, Box<Filter>),
}

/// Structural relations the engine owns. Relations are
/// implicitly existential: `Controller(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RelationFilter {
    /// The object's controller matches (CR 109.5).
    Controller(Box<Filter>),
    /// The object's owner matches (CR 108.3).
    Owner(Box<Filter>),
    /// The object is an opponent of a matching player (CR 102.1).
    OpponentOf(Box<Filter>),
    /// The object is attached to a matching object (CR 301.5, 303.4).
    AttachedTo(Box<Filter>),
    /// The object has a matching attachment (CR 301.5, 303.4).
    Attachment(Box<Filter>),
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
    /// Matches every object — the bare-Filter default for event participant
    /// slots (`Event::Performed`'s `by`/`on`).
    Any,
    /// A remembered `Filter` macro invocation (`AnyTarget`, evasion sets,
    /// protection qualities, …). Serialized as the invocation, not the struct.
    Expanded(Expansion<Filter>),
}

impl Filter {
    /// The match-anything filter, as a serde `default` for fields like
    /// `Event::Performed`'s `by`/`on`.
    #[must_use]
    pub fn any() -> Filter { Filter::Any }
}

/// Every name a Filter position accepts, compartments flattened: the
/// variant list the macro layer's enum interception checks before trying
/// Filter macros. Names must stay globally unique across compartments.
const VARIANTS: &[&str] = &[
    "Kind",
    "Type",
    "Subtype",
    "Supertype",
    "ColorIs",
    "Named",
    "Stat",
    "HasAbility",
    "InZone",
    "Status",
    "HasCounter",
    "Designated",
    "RelatedBy",
    "Controller",
    "Owner",
    "OpponentOf",
    "AttachedTo",
    "Attachment",
    "Is",
    "AllOf",
    "OneOf",
    "Not",
    "Any",
    "Expanded",
];

// Visitor for the 3-tuple `Stat(Stat, Cmp, Quantity)` atom.
struct StatTriple(Stat, Cmp, Quantity);

struct StatTripleVisitor;

impl<'de> Visitor<'de> for StatTripleVisitor {
    type Value = StatTriple;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a (Stat, Cmp, Quantity) tuple")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let stat = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let cmp = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let q = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        Ok(StatTriple(stat, cmp, q))
    }
}

// Visitor for the 2-tuple `RelatedBy(Ident, Box<Filter>)` atom.
struct RelatedByPair(Ident, Box<Filter>);

struct RelatedByVisitor;

impl<'de> Visitor<'de> for RelatedByVisitor {
    type Value = RelatedByPair;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a (relation name, Filter) tuple")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let name = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let filter = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(RelatedByPair(name, filter))
    }
}

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

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("a filter") }

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
                    "ColorIs" => Filter::Characteristic(C::ColorIs(v.newtype_variant()?)),
                    "Named" => Filter::Characteristic(C::Named(v.newtype_variant()?)),
                    "Stat" => {
                        let StatTriple(stat, cmp, q) = v.tuple_variant(3, StatTripleVisitor)?;
                        Filter::Characteristic(C::Stat(stat, cmp, q))
                    }
                    "HasAbility" => Filter::Characteristic(C::HasAbility(v.newtype_variant()?)),
                    "InZone" => Filter::State(S::InZone(v.newtype_variant()?)),
                    "Status" => Filter::State(S::Status(v.newtype_variant()?)),
                    "HasCounter" => Filter::State(S::HasCounter(v.newtype_variant()?)),
                    "Designated" => Filter::State(S::Designated(v.newtype_variant()?)),
                    "RelatedBy" => {
                        let RelatedByPair(name, f) = v.tuple_variant(2, RelatedByVisitor)?;
                        Filter::State(S::RelatedBy(name, f))
                    }
                    "Controller" => Filter::Relation(R::Controller(v.newtype_variant()?)),
                    "Owner" => Filter::Relation(R::Owner(v.newtype_variant()?)),
                    "OpponentOf" => Filter::Relation(R::OpponentOf(v.newtype_variant()?)),
                    "AttachedTo" => Filter::Relation(R::AttachedTo(v.newtype_variant()?)),
                    "Attachment" => Filter::Relation(R::Attachment(v.newtype_variant()?)),
                    "Is" => Filter::Is(v.newtype_variant()?),
                    "AllOf" => Filter::AllOf(v.newtype_variant()?),
                    "OneOf" => Filter::OneOf(v.newtype_variant()?),
                    "Not" => Filter::Not(v.newtype_variant()?),
                    "Any" => {
                        // Unit variant: consume the (empty) payload.
                        v.unit_variant()?;
                        Filter::Any
                    }
                    "Expanded" => Filter::Expanded(v.newtype_variant()?),
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
            Filter::Kind(kind) => serializer.serialize_newtype_variant("Filter", 0, "Kind", kind),
            // The compartments serialize transparently: RON writes only the
            // inner variant, so the text stays flat.
            Filter::Characteristic(c) => c.serialize(serializer),
            Filter::State(s) => s.serialize(serializer),
            Filter::Relation(r) => r.serialize(serializer),
            Filter::Is(r) => serializer.serialize_newtype_variant("Filter", 18, "Is", r),
            Filter::AllOf(fs) => serializer.serialize_newtype_variant("Filter", 19, "AllOf", fs),
            Filter::OneOf(fs) => serializer.serialize_newtype_variant("Filter", 20, "OneOf", fs),
            Filter::Not(f) => serializer.serialize_newtype_variant("Filter", 21, "Not", f),
            Filter::Any => serializer.serialize_unit_variant("Filter", 22, "Any"),
            // The invocation, not the struct: `Expansion`'s Serialize emits it.
            Filter::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Supertype, Type, Zone};

    fn read(source: &str) -> Filter { crate::ron::options().from_str(source).unwrap() }

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
        assert_eq!(read("Any"), Filter::Any);
    }

    /// The new §1 atoms read flat through their compartments.
    #[test]
    fn new_atoms_read_flat() {
        assert_eq!(
            read("ColorIs(Red)"),
            Filter::Characteristic(CharacteristicFilter::ColorIs(Color::Red)),
        );
        assert_eq!(
            read(r#"Named("Forest")"#),
            Filter::Characteristic(CharacteristicFilter::Named("Forest".into())),
        );
        assert_eq!(
            read("Stat(Power, AtLeast, 3)"),
            Filter::Characteristic(CharacteristicFilter::Stat(
                Stat::Power,
                Cmp::AtLeast,
                Quantity::Literal(3),
            )),
        );
        assert_eq!(
            read(r#"HasAbility("Flying")"#),
            Filter::Characteristic(CharacteristicFilter::HasAbility("Flying".into())),
        );
        assert_eq!(
            read("Status(Tapped)"),
            Filter::State(StateFilter::Status(Status::Tapped)),
        );
        assert_eq!(
            read(r#"HasCounter("+1/+1")"#),
            Filter::State(StateFilter::HasCounter("+1/+1".into())),
        );
        assert_eq!(
            read(r#"Designated("Monstrous")"#),
            Filter::State(StateFilter::Designated("Monstrous".into())),
        );
        assert_eq!(
            read(r#"RelatedBy("PairedWith", Type(Creature))"#),
            Filter::State(StateFilter::RelatedBy(
                "PairedWith".into(),
                Box::new(Filter::Characteristic(CharacteristicFilter::Type(
                    Type::Creature
                ))),
            )),
        );
        assert_eq!(
            read("AttachedTo(Type(Creature))"),
            Filter::Relation(RelationFilter::AttachedTo(Box::new(
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature),)
            ))),
        );
        assert_eq!(
            read("Attachment(Type(Enchantment))"),
            Filter::Relation(RelationFilter::Attachment(Box::new(
                Filter::Characteristic(CharacteristicFilter::Type(Type::Enchantment),)
            ))),
        );
    }

    #[test]
    fn combinators_nest() {
        assert_eq!(
            read("AllOf([InZone(Battlefield), Type(Creature)])"),
            Filter::AllOf(vec![
                Filter::State(StateFilter::InZone(Zone::Battlefield)),
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
    fn unknown_names_error() {
        assert!(
            crate::ron::options()
                .from_str::<Filter>("Bogus(1)")
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
    /// Deserialize accepts back — every compartment, plus the new atoms.
    #[test]
    fn compartment_round_trips() {
        let cases = [
            "Controller(Is(You))",
            "Owner(Kind(Player))",
            "OpponentOf(Kind(Player))",
            "AttachedTo(Type(Creature))",
            "Attachment(Type(Enchantment))",
            "InZone(Battlefield)",
            "Status(Tapped)",
            r#"HasCounter("+1/+1")"#,
            r#"Designated("Monstrous")"#,
            r#"RelatedBy("PairedWith", Type(Creature))"#,
            r#"Subtype("Forest")"#,
            "Supertype(Basic)",
            "ColorIs(Green)",
            r#"Named("Forest")"#,
            "Stat(Toughness, Greater, 0)",
            r#"HasAbility("Flying")"#,
            "Not(Kind(Player))",
            "Is(You)",
            "Any",
        ];
        for source in cases {
            let parsed = read(source);
            let written = crate::ron::options().to_string(&parsed).unwrap();
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }
}
