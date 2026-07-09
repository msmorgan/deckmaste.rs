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
    /// A copy of a card ([CR#109.1] lists it as its own object kind,
    /// distinct from `Card`): what copy effects that create card copies in
    /// non-stack zones produce ([CR#707.12]). A copy of a SPELL on the stack
    /// is a `Spell`; this is the grammar footing for the card-copy object —
    /// the engine's `object_kind` starts classifying copies when the copy
    /// grammar lands.
    CardCopy,
    Emblem,
    Player,
    Spell,
    Token,
}

/// Characteristic atoms ([CR#109.3]): facts printed on or defined for the
/// object. `Subtype`/`Named`/`Has` filter by *name* — validating that
/// the name is declared is a lint, not a parse concern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum CharacteristicPredicate {
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
pub enum StatePredicate {
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
    /// relation ([CR#607] family). Box because the inner predicate is a
    /// Predicate.
    RelatedBy(Ident, Box<Predicate>),
    /// Declared as an attacker, still in combat ([CR#508.1a]).
    Attacking,
    /// Declared as a blocker, still in combat ([CR#509.1a]).
    Blocking,
    /// Attacking and unblocked once blockers are declared ([CR#509.1h]).
    Unblocked,
    /// "that targets [desc]" ([CR#115.9b]): a stack object one of whose
    /// chosen targets CURRENTLY matches — departed targets are ignored,
    /// never read through LKI (the one value read with no LKI fallback).
    /// Box breaks the `Predicate` size cycle.
    Targets(Box<Predicate>),
    /// "with [N] target(s)" ([CR#115.9a]): counts the target instances
    /// chosen at stack-put; "targets only …" counts distinct chosen
    /// targets, then checks current state ([CR#115.9c]).
    TargetCount(crate::CountBound),
    /// The object's tagged optional cost
    /// ([`OptionalCost`](crate::OptionalCost)) was paid — the filter-language
    /// channel of the paid-cost linkage ([CR#702.33d..702.33e,607.2]): "a
    /// kicked spell" = `AllOf([Kind(Spell), WasPaidWith(Kicker)])`.
    WasPaidWith(crate::CostTag),
    /// The object was cast using the named alternative base cost
    /// ([CR#118.9,702.34a]) — the filter-language channel of the alt-cost
    /// linkage; the twin of `WasPaidWith` (optional/additional costs). Idris
    /// `WasCastWith`.
    WasCastWith(crate::CostTag),
}

/// Structural relations the engine owns. Relations are
/// implicitly existential: `ControlledBy(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum RelationPredicate {
    /// The object's controller matches ([CR#109.5]).
    ControlledBy(Box<Predicate>),
    /// The object is a player who controls a matching object — the inverse
    /// of [`ControlledBy`](RelationPredicate::ControlledBy) ([CR#109.5]).
    /// Zone-agnostic: control spans the battlefield, the stack (spells and
    /// abilities), and the command zone, so the inner filter carries any
    /// zone restriction it needs (e.g. `Controls(AllOf([Permanent, …]))`
    /// for "controls a permanent").
    Controls(Box<Predicate>),
    /// The object's owner matches ([CR#108.3]).
    Owner(Box<Predicate>),
    /// The object is an opponent of a matching player ([CR#102.2,102.3]) — a
    /// player NOT on the matching player's team.
    OpponentOf(Box<Predicate>),
    /// The object is a teammate of a matching player ([CR#102.3,810.1]) —
    /// ANOTHER player on the matching player's team (never that player itself).
    /// PRIMITIVE, not `Not(OpponentOf …)`: in Two-Headed Giant a teammate is
    /// neither you nor an opponent ([CR#810]). "your team" ([CR#102.4]) is
    /// `OneOf([Ref(You), TeammateOf(Ref(You))])`.
    TeammateOf(Box<Predicate>),
    /// The object is attached to a matching object ([CR#301.5,303.4]).
    AttachedTo(Box<Predicate>),
    /// The object has a matching attachment ([CR#301.5,303.4]).
    Attachment(Box<Predicate>),
}

/// A predicate over game objects, players included. Compartmentalized in
/// Rust; flat in RON (`Type(Creature)`, never
/// `Characteristic(Type(Creature))`) via the `#[macro_ron(flatten)]`
/// markers: each compartment's variant names lift into `Predicate`'s dispatch
/// and the compartment tag never appears in text.
///
/// Generated dispatch, not `#[serde(untagged)]` wrappers: untagged variants
/// deserialize through `deserialize_any`, which never reaches the macro
/// layer's `deserialize_enum` interception — a `Predicate` macro would stop
/// expanding at Predicate positions. Dispatching by name over one combined
/// variant list keeps the RON flat *and* the positions macro-aware.
///
/// Conjunction is explicit (`AllOf`) — an enum position never carries a
/// bare list. Canonical filters are context-free-correct: state the whole
/// predicate even where engine context would make parts redundant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Predicate {
    Kind(ObjectKind),
    #[macro_ron(flatten)]
    Characteristic(CharacteristicPredicate),
    #[macro_ron(flatten)]
    State(StatePredicate),
    #[macro_ron(flatten)]
    Relation(RelationPredicate),
    Ref(Reference),
    /// Lifts a quality to a stack ABILITY's source ([CR#702.11d] "abilities
    /// … from [quality] sources"): matches an activated/triggered ability on
    /// the stack whose SOURCE — the object that generated it ([CR#113.7]) —
    /// matches the inner filter. Strict to abilities by construction: a
    /// spell carries its qualities itself, so "red spells or abilities from
    /// red sources" is `OneOf([AllOf([Kind(Spell), ColorIs(Red)]),
    /// FromSource(ColorIs(Red))])`. Boxed like the other one-child atoms.
    FromSource(Box<Predicate>),
    AllOf(Vec<Predicate>),
    OneOf(Vec<Predicate>),
    Not(Box<Predicate>),
    /// The candidate matches iff a [`Condition`] holds with `It` bound to
    /// it — the bridge that lets a per-object filter slot reach the whole
    /// condition language ([CR#603.4] predicates) against the object being
    /// matched. [`Reference::It`](crate::Reference::It) inside the condition
    /// resolves to that candidate; `Ref(This)`/`Ref(You)` still anchor to the
    /// carrier. Boxed to break the `Predicate` → `Condition` → `Predicate` size
    /// cycle. The one candidate-relative escape hatch: "shares a color with ~",
    /// "has the same name as ~", etc., expressed as
    /// `Where(SharesColor(It, This))` and kin.
    Where(Box<Condition>),
    /// Matches every object — the bare-Predicate default for event participant
    /// slots (an `EventFilter` master form's `who`/`what`).
    Any,
    /// A remembered `Predicate` macro invocation (evasion sets, protection
    /// qualities, …). Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<Predicate>),
}

impl Predicate {
    /// The match-anything filter, as a serde `default` for fields like an
    /// `EventFilter` master form's `who`/`what`.
    #[must_use]
    pub fn any() -> Predicate {
        Predicate::Any
    }

    /// Matches objects of a single card type ([CR#109.3]) — the flat
    /// `Type(t)` atom, spelled without its `Characteristic` compartment.
    #[must_use]
    pub fn type_(t: Type) -> Predicate {
        Predicate::Characteristic(CharacteristicPredicate::Type(t))
    }

    /// Matches creatures — [`Predicate::type_`] for [`Type::Creature`], the
    /// most common typed filter across the card base.
    #[must_use]
    pub fn creature() -> Predicate {
        Predicate::type_(Type::Creature)
    }

    /// Whether this filter is exactly the self-reference (`Ref(This)`) — the
    /// "~ itself" predicate rendering and the replacement layer test for.
    #[must_use]
    pub fn is_this(&self) -> bool {
        matches!(self, Predicate::Ref(Reference::This))
    }
}

impl Normalize for RelationPredicate {
    /// Recurse into the related-object filter each relation carries.
    fn normalize(self) -> Self {
        use RelationPredicate as R;
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

impl Normalize for StatePredicate {
    /// Recurse into the inner filter the relation/target state atoms carry;
    /// every other state atom is a leaf for normalization.
    fn normalize(self) -> Self {
        use StatePredicate as S;
        match self {
            S::RelatedBy(rel, f) => S::RelatedBy(rel, f.normalize()),
            S::Targets(f) => S::Targets(f.normalize()),
            other => other,
        }
    }
}

impl Normalize for Predicate {
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
            Predicate::Relation(r) => Predicate::Relation(r.normalize()),
            Predicate::State(s) => Predicate::State(s.normalize()),
            Predicate::Not(inner) => Predicate::Not(inner.normalize()),
            Predicate::FromSource(inner) => Predicate::FromSource(inner.normalize()),

            Predicate::AllOf(children) => {
                let mut flat = Vec::with_capacity(children.len());
                for child in children {
                    match child.normalize() {
                        // Associativity: splice a nested AllOf in.
                        Predicate::AllOf(inner) => flat.extend(inner),
                        other => flat.push(other),
                    }
                }
                // Singleton collapse: AllOf([x]) → x.
                if flat.len() == 1 {
                    flat.pop().expect("len checked")
                } else {
                    Predicate::AllOf(flat)
                }
            }
            Predicate::OneOf(children) => {
                let mut flat = Vec::with_capacity(children.len());
                for child in children {
                    match child.normalize() {
                        Predicate::OneOf(inner) => flat.extend(inner),
                        other => flat.push(other),
                    }
                }
                if flat.len() == 1 {
                    flat.pop().expect("len checked")
                } else {
                    Predicate::OneOf(flat)
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

    fn read(source: &str) -> Predicate {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn atoms_read_flat() {
        assert_eq!(
            read("Type(Creature)"),
            Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature)),
        );
        assert_eq!(
            read(r#"Subtype("Forest")"#),
            Predicate::Characteristic(CharacteristicPredicate::Subtype("Forest".into())),
        );
        assert_eq!(
            read("Supertype(Basic)"),
            Predicate::Characteristic(CharacteristicPredicate::Supertype(Supertype::Basic)),
        );
        assert_eq!(
            read("InZone(Battlefield)"),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
        );
        assert_eq!(read("Kind(Player)"), Predicate::Kind(ObjectKind::Player));
        assert_eq!(read("Kind(Ability)"), Predicate::Kind(ObjectKind::Ability));
        assert_eq!(read("Any"), Predicate::Any);
    }

    /// The team-relative player relation `TeammateOf` reads flat alongside its
    /// `OpponentOf` sibling and round-trips ([CR#102.3,810.1]).
    #[test]
    fn teammate_of_reads_and_round_trips() {
        use crate::Reference;
        let v = read("TeammateOf(Ref(You))");
        assert_eq!(
            v,
            Predicate::Relation(RelationPredicate::TeammateOf(Box::new(Predicate::Ref(
                Reference::You
            )))),
        );
        let written = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(written, "TeammateOf(Ref(You))");
        assert_eq!(read(&written), v);
        // "your team" ([CR#102.4]) composes from the two primitives.
        assert_eq!(
            read("OneOf([Ref(You), TeammateOf(Ref(You))])"),
            Predicate::OneOf(vec![
                Predicate::Ref(Reference::You),
                Predicate::Relation(RelationPredicate::TeammateOf(Box::new(Predicate::Ref(
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
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(Color::Red)),
        );
        assert_eq!(
            read(r#"Named("Forest")"#),
            Predicate::Characteristic(CharacteristicPredicate::Named("Forest".into())),
        );
        assert_eq!(
            read("Stat(Power, AtLeast, Literal(3))"),
            Predicate::Characteristic(CharacteristicPredicate::Stat(
                Stat::Power,
                Cmp::AtLeast,
                Count::Literal(3),
            )),
        );
        assert_eq!(
            read("Has(Flying)"),
            Predicate::Characteristic(CharacteristicPredicate::Has("Flying".into())),
        );
        assert_eq!(
            read("Status(Tapped)"),
            Predicate::State(StatePredicate::Status(Status::Tapped)),
        );
        assert_eq!(
            // The counter kind is a bare ident (`CounterRef`), not a string.
            read("HasCounter(P1P1Counter)"),
            Predicate::State(StatePredicate::HasCounter("P1P1Counter".into())),
        );
        assert_eq!(
            read(r#"Designated("Monstrous")"#),
            Predicate::State(StatePredicate::Designated("Monstrous".into())),
        );
        assert_eq!(
            // The paid-cost tag is a bare ident (`CostTag`), not a string.
            read("WasPaidWith(Kicker)"),
            Predicate::State(StatePredicate::WasPaidWith("Kicker".into())),
        );
        assert_eq!(
            // The alt-cost tag is a bare ident (`CostTag`), not a string.
            read("WasCastWith(Flashback)"),
            Predicate::State(StatePredicate::WasCastWith("Flashback".into())),
        );
        assert_eq!(
            read(r#"RelatedBy("PairedWith", Type(Creature))"#),
            Predicate::State(StatePredicate::RelatedBy(
                "PairedWith".into(),
                Box::new(Predicate::Characteristic(CharacteristicPredicate::Type(
                    Type::Creature
                ))),
            )),
        );
        assert_eq!(
            read("AttachedTo(Type(Creature))"),
            Predicate::Relation(RelationPredicate::AttachedTo(Box::new(
                Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature),)
            ))),
        );
        assert_eq!(
            read("Attachment(Type(Enchantment))"),
            Predicate::Relation(RelationPredicate::Attachment(Box::new(
                Predicate::Characteristic(CharacteristicPredicate::Type(Type::Enchantment),)
            ))),
        );
    }

    /// `FromSource` lifts a quality to a stack ability's source
    /// ([CR#702.11d]): it reads flat, nests a full filter, and round-trips.
    #[test]
    fn from_source_reads_and_round_trips() {
        let v = read("FromSource(ColorIs(Red))");
        assert_eq!(
            v,
            Predicate::FromSource(Box::new(Predicate::Characteristic(
                CharacteristicPredicate::ColorIs(Color::Red)
            ))),
        );
        let written = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&written), v);
        // The hexproof-from-red agent shape: red spells, or abilities from
        // red sources ([CR#702.11d]).
        let agent = read("OneOf([AllOf([Kind(Spell), ColorIs(Red)]), FromSource(ColorIs(Red))])");
        assert!(matches!(agent, Predicate::OneOf(_)));
        // Normalize recurses through FromSource.
        assert_eq!(
            read("FromSource(AllOf([ColorIs(Red)]))").normalize(),
            read("FromSource(ColorIs(Red))"),
        );
    }

    #[test]
    fn combinators_nest() {
        assert_eq!(
            read("AllOf([InZone(Battlefield), Type(Creature)])"),
            Predicate::AllOf(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature)),
            ]),
        );
        assert_eq!(
            read("Not(Kind(Player))"),
            Predicate::Not(Box::new(Predicate::Kind(ObjectKind::Player))),
        );
    }

    #[test]
    fn relations_take_filters() {
        use crate::Reference;
        assert_eq!(
            read("ControlledBy(Ref(You))"),
            Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                Reference::You
            )))),
        );
        assert_eq!(
            read("Controls(Type(Land))"),
            Predicate::Relation(RelationPredicate::Controls(Box::new(
                Predicate::Characteristic(CharacteristicPredicate::Type(Type::Land))
            ))),
        );
    }

    /// The compartment wrappers must not appear in the text: Rust nests,
    /// RON stays flat.
    #[test]
    fn serialization_stays_flat() {
        let filter = Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature));
        assert_eq!(
            crate::ron::options().to_string(&filter).unwrap(),
            "Type(Creature)"
        );
    }

    #[test]
    fn unknown_names_error() {
        assert!(
            crate::ron::options()
                .from_str::<Predicate>("Bogus(1)")
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
            "WasPaidWith(Kicker)",
            "WasCastWith(Flashback)",
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
