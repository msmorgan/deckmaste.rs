//! The card-domain macro configuration: which core types are macroable, and
//! with what reader policy, glued onto [`macro_ron`]. Definitions live in
//! `plugins/*/macros/**/*.ron` (paths and file names are organizational
//! only) and are invoked by name where a value of one of their kinds is
//! expected — see [`crate::plugin`] for loading and the `macro_ron` crate
//! docs for the language itself.

pub use macro_ron::{InsertError, MacroDef, MacroSet, ParamType, Params};
use macro_ron::{Kind, KindSet};

/// The kinds that remember their expansions: their Rust types bear
/// `Expanded(Expansion<Self>)`, and the engine consults the remembered name
/// for ability/verb/event identity and for provenance. (`Quantity` is
/// registered separately for its literal sugar.)
const REMEMBERING_KINDS: [&str; 10] = [
    "Ability",
    "Condition",
    "CostComponent",
    "Effect",
    "Event",
    "Filter",
    "Reference",
    "Selection",
    "StaticEffect",
    "TargetSpec",
];

/// The kinds of value a macro can expand to: the core types whose parse
/// positions consult the macro namespace. Names must match the Rust types'
/// serde names.
///
/// The struct kinds `CardFace` and `Subtype` are name-erasing: `Subtype`
/// already self-names and nothing engine-meaningful invokes `CardFace`
/// macros. A bare digit-led value at a `Quantity` position is reader sugar
/// for `Literal(N)` — core's grammar stays strict.
#[must_use]
pub fn kinds() -> KindSet {
    let mut kinds = KindSet::new();
    for name in REMEMBERING_KINDS {
        kinds.add(Kind::new(name).remembers_expansion());
    }
    kinds.add(
        Kind::new("Quantity")
            .remembers_expansion()
            .literal_wrapper("Literal"),
    );
    kinds.add(Kind::new("CardFace"));
    kinds.add(Kind::new("Subtype"));
    kinds
}

/// An empty [`MacroSet`] over the card kinds, reading deckmaste's RON
/// dialect.
#[must_use]
pub fn macro_set() -> MacroSet {
    MacroSet::new(kinds()).with_options(deckmaste_core::ron::options())
}

#[cfg(test)]
mod tests {
    use deckmaste_core::{
        Ability, CardFace, CharacteristicFilter, Condition, CostComponent, Effect, Event, Filter,
        ObjectKind, Quantity, Reference, Selection, StateFilter, StaticEffect, Subtype, TargetSpec,
        Type, Zone,
    };

    use super::*;

    /// Parses a definition the way plugin loading does: from RON source in
    /// deckmaste's dialect. (`MacroDef`'s body field is crate-private in
    /// `macro_ron`, so file-shaped source is the construction path here —
    /// which is also what real definitions are.)
    fn def(source: &str) -> MacroDef { deckmaste_core::ron::options().from_str(source).unwrap() }

    /// The registry matches on serde type names, which a Rust rename
    /// changes without a compile error — this is the tie. (A
    /// `#[serde(rename)]` on a core type would still slip through; none
    /// carries one.)
    #[test]
    fn kind_names_track_the_core_types() {
        fn name_of<T>() -> &'static str { std::any::type_name::<T>().rsplit("::").next().unwrap() }
        let names = [
            name_of::<Ability>(),
            name_of::<CardFace>(),
            name_of::<Condition>(),
            name_of::<CostComponent>(),
            name_of::<Effect>(),
            name_of::<Event>(),
            name_of::<Filter>(),
            name_of::<Quantity>(),
            name_of::<Reference>(),
            name_of::<Selection>(),
            name_of::<StaticEffect>(),
            name_of::<Subtype>(),
            name_of::<TargetSpec>(),
        ];
        let kinds = kinds();
        for name in names {
            assert!(kinds.contains(name), "`{name}` is not a registered kind");
        }
        assert_eq!(kinds.len(), names.len());
    }

    /// Filter's manual Deserialize must go through `deserialize_enum` with
    /// the full flattened variant list: that is what lets unknown names at
    /// Filter positions fall through to the macro namespace.
    #[test]
    fn filter_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "AnyTargetish",
                    kinds: [Filter],
                    body: OneOf([Kind(Player), AllOf([InZone(Battlefield), Type(Creature)])]),
                )"#))
            .unwrap();
        let filter: Filter = macros.read_str("AnyTargetish").unwrap();
        // The invocation is remembered; the expansion lives under `.value`.
        let Filter::Expanded(expanded) = filter else {
            panic!("expected a remembered filter, got {filter:?}");
        };
        assert_eq!(expanded.name, "AnyTargetish");
        let Filter::OneOf(arms) = *expanded.value else {
            panic!("expected OneOf, got {:?}", expanded.value);
        };
        assert_eq!(arms[0], Filter::Kind(ObjectKind::Player));
        // The nested arm proves Filter positions *inside* an expansion stay
        // macro-aware too.
        assert_eq!(
            arms[1],
            Filter::AllOf(vec![
                Filter::State(StateFilter::InZone(Zone::Battlefield)),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ])
        );
        assert_eq!(arms.len(), 2);
    }

    /// Same pin for Selection positions: nothing exercises Selection macros
    /// in real data yet, and Plan 2 will make this path load-bearing.
    #[test]
    fn selection_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "EachCreature",
                    kinds: [Selection],
                    body: Each(Type(Creature)),
                )"#))
            .unwrap();
        let selection: Selection = macros.read_str("EachCreature").unwrap();
        let Selection::Expanded(expanded) = selection else {
            panic!("expected a remembered selection, got {selection:?}");
        };
        assert_eq!(expanded.name, "EachCreature");
        assert_eq!(
            *expanded.value,
            Selection::Each(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            )))
        );
    }

    /// Same pin for the `TargetSpec` positions (the announce-list type).
    #[test]
    fn target_spec_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "TargetCreature",
                    kinds: [TargetSpec],
                    body: Target(Type(Creature)),
                )"#))
            .unwrap();
        let spec: TargetSpec = macros.read_str("TargetCreature").unwrap();
        let TargetSpec::Expanded(expanded) = spec else {
            panic!("expected a remembered target spec, got {spec:?}");
        };
        assert_eq!(expanded.name, "TargetCreature");
        assert_eq!(
            *expanded.value,
            TargetSpec::Target(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            )))
        );
    }

    /// And for Reference positions.
    #[test]
    fn reference_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "MyController",
                    kinds: [Reference],
                    body: ControllerOf(This),
                )"#))
            .unwrap();
        let reference: Reference = macros.read_str("MyController").unwrap();
        let Reference::Expanded(expanded) = reference else {
            panic!("expected a remembered reference, got {reference:?}");
        };
        assert_eq!(expanded.name, "MyController");
        assert_eq!(
            *expanded.value,
            Reference::ControllerOf(Box::new(Reference::This))
        );
    }

    /// `CostComponent` positions participate in macro expansion: a
    /// registered `CostComponent` macro is expanded in place of its name.
    #[test]
    fn cost_positions_expand_macros() {
        use deckmaste_core::Action;

        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "SacThis",
                    kinds: [CostComponent],
                    body: Do(Sacrifice(This)),
                )"#))
            .unwrap();
        let cost: CostComponent = macros.read_str("SacThis").unwrap();
        let CostComponent::Expanded(expanded) = cost else {
            panic!("expected a remembered cost component, got {cost:?}");
        };
        assert_eq!(expanded.name, "SacThis");
        assert_eq!(
            *expanded.value,
            CostComponent::Do(Action::Sacrifice(Selection::from(Reference::This)))
        );
    }

    /// A remembered invocation round-trips as the invocation through the
    /// real core types' hand-written `Serialize` impls: a nullary Ability
    /// macro serializes back to its bare name, a parameterized Filter macro
    /// to the original call text — not the expansion.
    #[test]
    fn remembered_invocations_round_trip_as_invocations() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "Flying",
                    kinds: [Ability],
                    body: Static(effects: [Restriction(CantAttack)]),
                )"#))
            .unwrap();
        macros
            .insert(&def(r#"(
                    name: "OfType",
                    kinds: [Filter],
                    params: [String],
                    body: Type(Param(0)),
                )"#))
            .unwrap();

        let ability: Ability = macros.read_str("Flying").unwrap();
        assert_eq!(
            deckmaste_core::ron::options().to_string(&ability).unwrap(),
            "Flying"
        );

        let filter: Filter = macros.read_str("OfType(Creature)").unwrap();
        assert_eq!(
            deckmaste_core::ron::options().to_string(&filter).unwrap(),
            "OfType(Creature)"
        );
    }

    /// The literal sugar applies to the real `Quantity` through the glue's
    /// registry: a bare numeral splices to `Literal`, identifier-led
    /// variants pass straight through.
    #[test]
    fn quantity_sugar_applies_to_core_quantity() {
        let quantity: Quantity = macro_set().read_str("3").unwrap();
        assert_eq!(quantity, Quantity::Literal(3));
        let quantity: Quantity = macro_set().read_str("X").unwrap();
        assert_eq!(quantity, Quantity::X);
    }
}
