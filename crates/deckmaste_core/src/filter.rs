use serde::Deserialize;
use serde::Serialize;

use crate::Cmp;
use crate::Color;
use crate::Count;
use crate::Expand;
use crate::Expansion;
use crate::Ident;
use crate::Reference;
use crate::Stat;
use crate::Status;
use crate::Supertype;
use crate::SupportsMacros;
use crate::Type;
use crate::Zone;

/// What kind of object something is ([CR#109.1]). Players are objects here
/// too — the engine gives players `ObjectId`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum ObjectKind {
    Card,
    Emblem,
    Player,
    Spell,
    Token,
}

/// Characteristic atoms ([CR#109.3]): facts printed on or defined for the
/// object. `Subtype`/`Named`/`HasAbility` filter by *name* — validating that
/// the name is declared is a lint, not a parse concern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum CharacteristicFilter {
    Type(Type),
    Subtype(Ident),
    Supertype(Supertype),
    /// The object is the given color ([CR#105.2,202.2]).
    ColorIs(Color),
    /// The object has the given name ([CR#201]).
    Named(Ident),
    /// A printed/defined stat compares as stated, e.g.
    /// `Stat(Power, AtLeast, 3)` — mana value via `Stat(ManaValue, …)`
    /// ([CR#208,202.3]).
    Stat(Stat, Cmp, Count),
    /// Two or more colors ([CR#105.2b]).
    Multicolored,
    /// No colors ([CR#105.2c] — colorless is not a color).
    Colorless,
    /// The object has the named keyword ability ([CR#702]).
    HasAbility(Ident),
}

/// State atoms: where the object is and what's on it — not
/// characteristics ([CR#110.5a,122.1]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum StateFilter {
    InZone(Zone),
    /// The object's status ([CR#110.5]).
    Status(Status),
    /// The object has at least one counter of the named kind ([CR#122.1]).
    HasCounter(Ident),
    /// The object carries the named designation (taxonomy §8) — derived
    /// designations (`Designated(Modified)`) work with no special casing.
    Designated(Ident),
    /// The object is related to a matching object by a named, declared
    /// relation ([CR#607] family). Box because the inner predicate is a Filter.
    RelatedBy(Ident, Box<Filter>),
    /// Declared as an attacker, still in combat ([CR#508.1a]).
    Attacking,
    /// Declared as a blocker, still in combat ([CR#509.1a]).
    Blocking,
    /// Attacking and unblocked once blockers are declared ([CR#509.1h]).
    Unblocked,
    /// "that targets [desc]" ([CR#115.9b]): a stack object one of whose
    /// chosen targets CURRENTLY matches — departed targets are ignored,
    /// never read through LKI (the one value read with no LKI fallback).
    /// Box breaks the `Filter` size cycle.
    Targets(Box<Filter>),
    /// "with [N] target(s)" ([CR#115.9a]): counts the target instances
    /// chosen at stack-put; "targets only …" counts distinct chosen
    /// targets, then checks current state ([CR#115.9c]).
    TargetCount(crate::CountBound),
}

/// Structural relations the engine owns. Relations are
/// implicitly existential: `Controller(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum RelationFilter {
    /// The object's controller matches ([CR#109.5]).
    Controller(Box<Filter>),
    /// The object's owner matches ([CR#108.3]).
    Owner(Box<Filter>),
    /// The object is an opponent of a matching player ([CR#102.2,102.3]).
    OpponentOf(Box<Filter>),
    /// The object is attached to a matching object ([CR#301.5,303.4]).
    AttachedTo(Box<Filter>),
    /// The object has a matching attachment ([CR#301.5,303.4]).
    Attachment(Box<Filter>),
}

/// A predicate over game objects, players included. Compartmentalized in
/// Rust; flat in RON (`Type(Creature)`, never
/// `Characteristic(Type(Creature))`) via the `#[macro_ron(flatten)]`
/// markers: each compartment's variant names lift into `Filter`'s dispatch
/// and the compartment tag never appears in text.
///
/// Generated dispatch, not `#[serde(untagged)]` wrappers: untagged variants
/// deserialize through `deserialize_any`, which never reaches the macro
/// layer's `deserialize_enum` interception — a `Filter` macro would stop
/// expanding at Filter positions. Dispatching by name over one combined
/// variant list keeps the RON flat *and* the positions macro-aware.
///
/// Conjunction is explicit (`AllOf`) — an enum position never carries a
/// bare list. Canonical filters are context-free-correct: state the whole
/// predicate even where engine context would make parts redundant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Filter {
    Kind(ObjectKind),
    #[macro_ron(flatten)]
    Characteristic(CharacteristicFilter),
    #[macro_ron(flatten)]
    State(StateFilter),
    #[macro_ron(flatten)]
    Relation(RelationFilter),
    Ref(Reference),
    AllOf(Vec<Filter>),
    OneOf(Vec<Filter>),
    Not(Box<Filter>),
    /// Matches every object — the bare-Filter default for event participant
    /// slots (`Event::Performed`'s `by`/`on`).
    Any,
    /// A remembered `Filter` macro invocation (evasion sets, protection
    /// qualities, …). Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<Filter>),
}

impl Filter {
    /// The match-anything filter, as a serde `default` for fields like
    /// `Event::Performed`'s `by`/`on`.
    #[must_use]
    pub fn any() -> Filter { Filter::Any }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Supertype;
    use crate::Type;
    use crate::Zone;

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
            read("Stat(Power, AtLeast, Literal(3))"),
            Filter::Characteristic(CharacteristicFilter::Stat(
                Stat::Power,
                Cmp::AtLeast,
                Count::Literal(3),
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
            read("Controller(Ref(You))"),
            Filter::Relation(RelationFilter::Controller(Box::new(Filter::Ref(
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

    /// The compartment Serialize delegation must produce text the
    /// Deserialize accepts back — every compartment, plus the new atoms.
    #[test]
    fn compartment_round_trips() {
        let cases = [
            "Controller(Ref(You))",
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
            "Stat(Toughness, Greater, Literal(0))",
            r#"HasAbility("Flying")"#,
            "Not(Kind(Player))",
            "Ref(You)",
            "Any",
        ];
        for source in cases {
            let parsed = read(source);
            let written = crate::ron::options().to_string(&parsed).unwrap();
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }
}
