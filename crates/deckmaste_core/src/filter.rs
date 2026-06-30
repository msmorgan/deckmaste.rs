use serde::Deserialize;
use serde::Serialize;

use crate::Cmp;
use crate::Color;
use crate::Condition;
use crate::Count;
use crate::Expand;
use crate::Expansion;
use crate::Ident;
use crate::Normalize;
use crate::Reference;
use crate::Stat;
use crate::Status;
use crate::Supertype;
use crate::SupportsMacros;
use crate::Type;
use crate::Zone;

/// What kind of object something is ([CR#109.1]). Players are objects here
/// too — the engine gives players `ObjectId`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ObjectKind {
    /// An activated or triggered ability on the stack
    /// ([CR#602.2a,603.3]). Distinct from `Spell` (a card on the
    /// stack): an ability on the stack has no card identity of its own. The
    /// `Kind(Ability)` filter is what "counter target activated ability" /
    /// "target activated or triggered ability" (Stifle, Disallow) selects
    /// over.
    Ability,
    Card,
    Emblem,
    Player,
    Spell,
    Token,
}

/// Characteristic atoms ([CR#109.3]): facts printed on or defined for the
/// object. `Subtype`/`Named`/`Has` filter by *name* — validating that
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
    /// The object HAS the named keyword ability ([CR#702]) — a bare-ident
    /// keyword reference (`Has(Flying)`), matching by NAME (a parameterized
    /// keyword matches regardless of its arguments — card text says "has
    /// ward", never "has ward {2}").
    Has(crate::KeywordRef),
}

/// State atoms: where the object is and what's on it — not
/// characteristics ([CR#110.5a,122.1]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum StateFilter {
    InZone(Zone),
    /// The object's status ([CR#110.5]).
    Status(Status),
    /// The object has at least one counter of the named kind ([CR#122.1]).
    /// The kind is a bare `CounterRef` (`HasCounter(P1P1Counter)`), not a
    /// string.
    HasCounter(crate::CounterRef),
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
/// implicitly existential: `ControlledBy(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum RelationFilter {
    /// The object's controller matches ([CR#109.5]).
    ControlledBy(Box<Filter>),
    /// The object is a player who controls a matching object — the inverse
    /// of [`ControlledBy`](RelationFilter::ControlledBy) ([CR#109.5]).
    /// Zone-agnostic: control spans the battlefield, the stack (spells and
    /// abilities), and the command zone, so the inner filter carries any
    /// zone restriction it needs (e.g. `Controls(AllOf([Permanent, …]))`
    /// for "controls a permanent").
    Controls(Box<Filter>),
    /// The object's owner matches ([CR#108.3]).
    Owner(Box<Filter>),
    /// The object is an opponent of a matching player ([CR#102.2,102.3]) — a
    /// player NOT on the matching player's team.
    OpponentOf(Box<Filter>),
    /// The object is a teammate of a matching player ([CR#102.3,810.1]) —
    /// ANOTHER player on the matching player's team (never that player itself).
    /// PRIMITIVE, not `Not(OpponentOf …)`: in Two-Headed Giant a teammate is
    /// neither you nor an opponent ([CR#810]). "your team" ([CR#102.4]) is
    /// `OneOf([Ref(You), TeammateOf(Ref(You))])`.
    TeammateOf(Box<Filter>),
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
    /// The candidate matches iff a [`Condition`] holds with `It` bound to
    /// it — the bridge that lets a per-object filter slot reach the whole
    /// condition language ([CR#603.4] predicates) against the object being
    /// matched. [`Reference::It`](crate::Reference::It) inside the condition
    /// resolves to that candidate; `Ref(This)`/`Ref(You)` still anchor to the
    /// carrier. Boxed to break the `Filter` → `Condition` → `Filter` size
    /// cycle. The one candidate-relative escape hatch: "shares a color with ~",
    /// "has the same name as ~", etc., expressed as
    /// `Where(SharesColor(It, This))` and kin.
    Where(Box<Condition>),
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
    pub fn any() -> Filter {
        Filter::Any
    }

    /// Matches objects of a single card type ([CR#109.3]) — the flat
    /// `Type(t)` atom, spelled without its `Characteristic` compartment.
    #[must_use]
    pub fn type_(t: Type) -> Filter {
        Filter::Characteristic(CharacteristicFilter::Type(t))
    }

    /// Matches creatures — [`Filter::type_`] for [`Type::Creature`], the most
    /// common typed filter across the card base.
    #[must_use]
    pub fn creature() -> Filter {
        Filter::type_(Type::Creature)
    }

    /// Whether this filter is exactly the self-reference (`Ref(This)`) — the
    /// "~ itself" predicate rendering and the replacement layer test for.
    #[must_use]
    pub fn is_this(&self) -> bool {
        matches!(self, Filter::Ref(Reference::This))
    }
}

impl Normalize for RelationFilter {
    /// Recurse into the related-object filter each relation carries.
    fn normalize(self) -> Self {
        use RelationFilter as R;
        match self {
            R::ControlledBy(f) => R::ControlledBy(f.normalize()),
            R::Controls(f) => R::Controls(f.normalize()),
            R::Owner(f) => R::Owner(f.normalize()),
            R::OpponentOf(f) => R::OpponentOf(f.normalize()),
            R::TeammateOf(f) => R::TeammateOf(f.normalize()),
            R::AttachedTo(f) => R::AttachedTo(f.normalize()),
            R::Attachment(f) => R::Attachment(f.normalize()),
        }
    }
}

impl Normalize for StateFilter {
    /// Recurse into the inner filter the relation/target state atoms carry;
    /// every other state atom is a leaf for normalization.
    fn normalize(self) -> Self {
        use StateFilter as S;
        match self {
            S::RelatedBy(rel, f) => S::RelatedBy(rel, f.normalize()),
            S::Targets(f) => S::Targets(f.normalize()),
            other => other,
        }
    }
}

impl Normalize for Filter {
    /// Normalize a predicate (bottom-up): recurse into child filters, then
    /// collapse the boolean combinators. `AllOf`/`OneOf` are flattened by
    /// associativity (a nested `AllOf` inside an `AllOf` splices in — same for
    /// `OneOf`) and a singleton `AllOf([x])`/`OneOf([x])` collapses to `x`.
    /// Both rewrites preserve meaning: conjunction/disjunction are associative,
    /// and a one-element conjunction/disjunction is its element.
    ///
    /// Out of scope for this pass (demo identities only): `Not(Not x) → x`,
    /// `Any`-absorption, dedup/sort. `Where`'s inner [`Condition`] is left as
    /// authored (no `Condition` normalization yet).
    fn normalize(self) -> Self {
        match self {
            // Recurse into compartments that hold child filters.
            Filter::Relation(r) => Filter::Relation(r.normalize()),
            Filter::State(s) => Filter::State(s.normalize()),
            Filter::Not(inner) => Filter::Not(inner.normalize()),

            Filter::AllOf(children) => {
                let mut flat = Vec::with_capacity(children.len());
                for child in children {
                    match child.normalize() {
                        // Associativity: splice a nested AllOf in.
                        Filter::AllOf(inner) => flat.extend(inner),
                        other => flat.push(other),
                    }
                }
                // Singleton collapse: AllOf([x]) → x.
                if flat.len() == 1 {
                    flat.pop().expect("len checked")
                } else {
                    Filter::AllOf(flat)
                }
            }
            Filter::OneOf(children) => {
                let mut flat = Vec::with_capacity(children.len());
                for child in children {
                    match child.normalize() {
                        Filter::OneOf(inner) => flat.extend(inner),
                        other => flat.push(other),
                    }
                }
                if flat.len() == 1 {
                    flat.pop().expect("len checked")
                } else {
                    Filter::OneOf(flat)
                }
            }

            // Leaves (no in-scope child filter to recurse / no redundancy).
            // `Where` carries a Condition, not normalized in this pass.
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Supertype;
    use crate::Type;
    use crate::Zone;

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
        assert_eq!(read("Kind(Ability)"), Filter::Kind(ObjectKind::Ability));
        assert_eq!(read("Any"), Filter::Any);
    }

    /// The team-relative player relation `TeammateOf` reads flat alongside its
    /// `OpponentOf` sibling and round-trips ([CR#102.3,810.1]).
    #[test]
    fn teammate_of_reads_and_round_trips() {
        use crate::Reference;
        let v = read("TeammateOf(Ref(You))");
        assert_eq!(
            v,
            Filter::Relation(RelationFilter::TeammateOf(Box::new(Filter::Ref(
                Reference::You
            )))),
        );
        let written = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(written, "TeammateOf(Ref(You))");
        assert_eq!(read(&written), v);
        // "your team" ([CR#102.4]) composes from the two primitives.
        assert_eq!(
            read("OneOf([Ref(You), TeammateOf(Ref(You))])"),
            Filter::OneOf(vec![
                Filter::Ref(Reference::You),
                Filter::Relation(RelationFilter::TeammateOf(Box::new(Filter::Ref(
                    Reference::You
                )))),
            ]),
        );
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
            read("Has(Flying)"),
            Filter::Characteristic(CharacteristicFilter::Has("Flying".into())),
        );
        assert_eq!(
            read("Status(Tapped)"),
            Filter::State(StateFilter::Status(Status::Tapped)),
        );
        assert_eq!(
            // The counter kind is a bare ident (`CounterRef`), not a string.
            read("HasCounter(P1P1Counter)"),
            Filter::State(StateFilter::HasCounter("P1P1Counter".into())),
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
            read("ControlledBy(Ref(You))"),
            Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                Reference::You
            )))),
        );
        assert_eq!(
            read("Controls(Type(Land))"),
            Filter::Relation(RelationFilter::Controls(Box::new(Filter::Characteristic(
                CharacteristicFilter::Type(Type::Land)
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

    /// `Normalize` flattens nested `AllOf`/`OneOf` (associativity) and
    /// collapses a singleton combinator to its element.
    #[test]
    fn normalize_flattens_and_collapses_combinators() {
        // Associativity: AllOf([AllOf([a, b]), c]) → AllOf([a, b, c]).
        let nested = read("AllOf([AllOf([Type(Creature), Type(Land)]), InZone(Battlefield)])");
        assert_eq!(
            nested.clone().normalize(),
            read("AllOf([Type(Creature), Type(Land), InZone(Battlefield)])"),
        );

        // Same for OneOf.
        let nested_or = read("OneOf([OneOf([Type(Creature), Type(Land)]), InZone(Battlefield)])");
        assert_eq!(
            nested_or.normalize(),
            read("OneOf([Type(Creature), Type(Land), InZone(Battlefield)])"),
        );

        // Singleton collapse: AllOf([x]) → x, OneOf([x]) → x.
        assert_eq!(
            read("AllOf([Type(Creature)])").normalize(),
            read("Type(Creature)")
        );
        assert_eq!(
            read("OneOf([Type(Creature)])").normalize(),
            read("Type(Creature)")
        );

        // Nested singletons collapse from the inside out.
        assert_eq!(
            read("AllOf([AllOf([Type(Creature)])])").normalize(),
            read("Type(Creature)")
        );

        // A combinator under a compartment filter is normalized too.
        assert_eq!(
            read("ControlledBy(AllOf([Type(Creature)]))").normalize(),
            read("ControlledBy(Type(Creature))"),
        );

        // Distinct combinators are NOT merged (OneOf inside AllOf stays).
        let mixed = read("AllOf([OneOf([Type(Creature), Type(Land)]), InZone(Battlefield)])");
        assert_eq!(
            mixed.clone().normalize(),
            mixed,
            "AllOf/OneOf don't cross-flatten"
        );

        // Idempotent.
        let normd = nested.normalize();
        assert_eq!(normd.clone().normalize(), normd, "normalize is idempotent");
    }

    /// The compartment Serialize delegation must produce text the
    /// Deserialize accepts back — every compartment, plus the new atoms.
    #[test]
    fn compartment_round_trips() {
        let cases = [
            "ControlledBy(Ref(You))",
            "Controls(InZone(Battlefield))",
            "Owner(Kind(Player))",
            "OpponentOf(Kind(Player))",
            "TeammateOf(Kind(Player))",
            "AttachedTo(Type(Creature))",
            "Attachment(Type(Enchantment))",
            "InZone(Battlefield)",
            "Status(Tapped)",
            "HasCounter(P1P1Counter)",
            r#"Designated("Monstrous")"#,
            r#"RelatedBy("PairedWith", Type(Creature))"#,
            r#"Subtype("Forest")"#,
            "Supertype(Basic)",
            "ColorIs(Green)",
            r#"Named("Forest")"#,
            "Stat(Toughness, Greater, Literal(0))",
            "Has(Flying)",
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
