//! The machinery test suite, on fixture types mirroring the serde patterns
//! consumers use: name-erasing struct kinds (`Subtype`, `CardFace`),
//! remembering enum kinds (`Ability`, `Effect`, `Filter`), and a
//! literal-sugar kind (`Quantity`).

use serde::ser::{SerializeStructVariant, Serializer};
use serde::{Deserialize, Serialize};

use crate::{
    Expansion, ExpansionArgs, Ident, InsertError, Kind, KindSet, MacroDef, MacroSet, ParamType,
    ParamTypeSet, Params,
};

/// The deckmaste dialect, for parity with the real consumer: the intercept
/// layer has to coexist with `implicit_some` and `unwrap_variant_newtypes`
/// (`Intercept::SkipStructs` exists because of the latter).
fn options() -> ron::Options {
    ron::Options::default().with_default_extension(
        ron::extensions::Extensions::IMPLICIT_SOME
            | ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}

fn kinds() -> KindSet {
    let mut kinds = KindSet::new();
    kinds.add(Kind::new("Subtype"));
    kinds.add(Kind::new("CardFace"));
    kinds.add(Kind::new("Ability").remembers_expansion());
    kinds.add(Kind::new("Effect").remembers_expansion());
    kinds.add(Kind::new("Filter").remembers_expansion());
    kinds.add(
        Kind::new("Quantity")
            .remembers_expansion()
            .literal_wrapper("Literal"),
    );
    kinds
}

fn empty() -> MacroSet { MacroSet::new(kinds()).with_options(options()) }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum Type {
    Land,
    Creature,
}

/// A name-erasing struct kind, like deckmaste's `Subtype`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct Subtype {
    name: Ident,
    types: Vec<Type>,
}

/// A remembering enum kind whose manual `Serialize` delegates `Expanded` to
/// the invocation — the consumer-side half of the round-trip contract.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum Filter {
    Any,
    Type(Type),
    Named(String),
    OneOf(Vec<Filter>),
    AllOf(Vec<Filter>),
    Power(PowerFilter),
    Expanded(Expansion<Filter>),
}

/// Struct content fused into a newtype variant by `unwrap_variant_newtypes`:
/// the shape `Intercept::SkipStructs` exists for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
struct PowerFilter {
    min: u32,
}

impl Serialize for Filter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Filter::Any => serializer.serialize_unit_variant("Filter", 0, "Any"),
            Filter::Type(t) => serializer.serialize_newtype_variant("Filter", 1, "Type", t),
            Filter::Named(n) => serializer.serialize_newtype_variant("Filter", 2, "Named", n),
            Filter::OneOf(fs) => serializer.serialize_newtype_variant("Filter", 3, "OneOf", fs),
            Filter::AllOf(fs) => serializer.serialize_newtype_variant("Filter", 4, "AllOf", fs),
            Filter::Power(p) => serializer.serialize_newtype_variant("Filter", 5, "Power", p),
            // The invocation, not the struct.
            Filter::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum StaticEffect {
    CantAttack,
}

/// A second remembering enum kind, like `Ability`: a struct-variant body
/// (`Static(effects: […])`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum Ability {
    Static { effects: Vec<StaticEffect> },
    Expanded(Expansion<Ability>),
}

impl Serialize for Ability {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Ability::Static { effects } => {
                let mut sv = serializer.serialize_struct_variant("Ability", 0, "Static", 1)?;
                sv.serialize_field("effects", effects)?;
                sv.end()
            }
            Ability::Expanded(e) => e.serialize(serializer),
        }
    }
}

/// The literal-sugar kind, like `Quantity`: strict grammar (`Literal(3)`),
/// with bare digit-led values spliced by the reader.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum Quantity {
    X,
    CountOf(Box<Filter>),
    Literal(u32),
    Expanded(Expansion<Quantity>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum Selection {
    Target(u32),
}

/// A remembering verb kind, like `Effect`: nests `Quantity` positions.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum Effect {
    DealDamage(Selection, Quantity),
    DrawCards(Quantity),
    Expanded(Expansion<Effect>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum Color {
    White,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum SimpleManaSymbol {
    Generic(u32),
    #[serde(untagged)]
    Specific(Color),
}

/// Mirrors deckmaste's `ManaSymbol`: partially untagged, so its contents
/// buffer through `deserialize_any` — the `Param` splicing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum ManaSymbol {
    Hybrid(SimpleManaSymbol, Color),
    #[serde(untagged)]
    Simple(SimpleManaSymbol),
}

/// A name-erasing struct kind with named-parameter macros, like `CardFace`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CardFace {
    name: String,
    #[serde(default)]
    mana_cost: Vec<ManaSymbol>,
    types: Vec<Type>,
}

fn subtype_macro(name: &str, params: Vec<ParamType>, body: &str) -> MacroDef {
    MacroDef {
        name: name.into(),
        kinds: vec!["Subtype".into()],
        params: Params::Positional(params),
        body: body.trim().into(),
    }
}

fn land_type() -> MacroDef {
    options()
        .from_str::<MacroDef>(
            r#"(
                name: "LandType",
                kinds: [Subtype],
                params: [String],
                body: Subtype(
                    name: Param(0),
                    types: [Land],
                ),
            )"#,
        )
        .unwrap()
}

fn forest() -> Subtype {
    Subtype {
        name: "Forest".into(),
        types: vec![Type::Land],
    }
}

fn macros() -> MacroSet {
    let mut set = empty();
    set.insert(&land_type()).unwrap();
    set
}

#[test]
fn definition_files_are_self_describing() {
    let def = land_type();
    assert_eq!(def.name, "LandType");
    assert_eq!(def.kinds, ["Subtype"]);
    assert_eq!(
        def.params,
        Params::Positional(vec![ParamType("String".into())])
    );
    assert!(def.body().starts_with("Subtype("), "{}", def.body());
}

#[test]
fn invocations_read_as_the_expansion() {
    let subtype: Subtype = macros().read_str(r#"LandType("Forest")"#).unwrap();
    assert_eq!(subtype, forest());
}

#[test]
fn plain_values_still_read() {
    let subtype: Subtype = macros()
        .read_str(r#"Subtype(name: "Forest", types: [Land])"#)
        .unwrap();
    assert_eq!(subtype, forest());
}

#[test]
fn declared_names_are_nullary_macros() {
    let mut macros = macros();
    macros
        .declare("Subtype", "Forest".into(), r#"LandType("Forest")"#)
        .unwrap();
    let subtype: Subtype = macros.read_str("Forest").unwrap();
    assert_eq!(subtype, forest());
}

#[test]
fn unknown_names_are_an_error() {
    assert!(
        macros()
            .read_str::<Subtype>(r#"IslandType("Tropical")"#)
            .is_err()
    );
}

#[test]
fn wrong_arity_is_an_error() {
    for call in ["LandType", r#"LandType("Forest", "Island")"#] {
        assert!(
            macros().read_str::<Subtype>(call).is_err(),
            "{call} should not expand",
        );
    }
}

#[test]
fn unknown_kinds_are_an_error() {
    let mut macros = empty();
    let error = macros
        .insert(&MacroDef {
            name: "Bogus".into(),
            kinds: vec!["Sorcery".into()],
            params: Params::default(),
            body: "()".into(),
        })
        .unwrap_err();
    assert_eq!(
        error,
        InsertError::UnknownKind {
            kind: "Sorcery".into(),
            name: "Bogus".into(),
        }
    );
    assert!(
        error.to_string().contains("unregistered kind"),
        "unexpected error: {error}"
    );
}

#[test]
fn enum_positions_expand_unknown_variants() {
    // `Flying` is not a variant of Ability; the macro fills it in, and
    // its expansion is wrapped in `Ability::Expanded` carrying the name.
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Flying".into(),
            kinds: vec!["Ability".into()],
            params: Params::default(),
            body: "Static(effects: [CantAttack])".into(),
        })
        .unwrap();
    let ability: Ability = macros.read_str("Flying").unwrap();
    let Ability::Expanded(expanded) = ability else {
        panic!("expected a remembered ability, got {ability:?}");
    };
    assert_eq!(expanded.name, "Flying");
    assert!(expanded.args.is_none());
    assert_eq!(
        *expanded.value,
        Ability::Static {
            effects: vec![StaticEffect::CantAttack],
        }
    );
}

#[test]
fn macros_can_expand_to_macros() {
    // try_again(), twice: Woods is a macro reading `Forest`, itself a
    // declaration reading `LandType("Forest")`.
    let mut macros = macros();
    macros
        .declare("Subtype", "Forest".into(), r#"LandType("Forest")"#)
        .unwrap();
    macros.declare("Subtype", "Woods".into(), "Forest").unwrap();
    let subtype: Subtype = macros.read_str("Woods").unwrap();
    assert_eq!(subtype, forest());
}

#[test]
fn duplicate_names_are_an_error() {
    let duplicate = Err(InsertError::Duplicate {
        kind: "Subtype".into(),
        name: "LandType".into(),
    });
    let mut macros = macros();
    assert_eq!(macros.insert(&land_type()), duplicate);
    assert_eq!(
        macros.declare("Subtype", "LandType".into(), "Forest"),
        duplicate
    );
}

#[test]
fn recursion_is_an_error_not_a_stack_overflow() {
    let mut macros = macros();
    macros
        .declare("Subtype", "Ouroboros".into(), "Ouroboros")
        .unwrap();
    macros.declare("Subtype", "Ping".into(), "Pong").unwrap();
    macros.declare("Subtype", "Pong".into(), "Ping").unwrap();
    for cycle in ["Ouroboros", "Ping"] {
        let error = macros.read_str::<Subtype>(cycle).unwrap_err();
        assert!(
            error.to_string().contains("macros don't recurse"),
            "unexpected error: {error}"
        );
    }
}

#[test]
fn macros_are_namespaced_by_kind() {
    // One macro can serve several kinds, and is only visible at
    // positions of those kinds.
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Self".into(),
            kinds: vec!["Subtype".into(), "Filter".into()],
            params: Params::Positional(vec![ParamType("Any".into())]),
            body: "Param(0)".into(),
        })
        .unwrap();

    let filter: Filter = macros.read_str("Self(Type(Land))").unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(expanded.name, "Self");
    assert_eq!(
        expanded.args,
        ExpansionArgs::Positional(vec!["Type(Land)".to_owned()]),
    );
    assert_eq!(*expanded.value, Filter::Type(Type::Land));

    // The macro is invisible at an Ability position.
    let error = macros.read_str::<Ability>("Self(Static)").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("neither a variant of `Ability` nor a known `Ability` macro"),
        "unexpected error: {error}"
    );
}

/// Positions *inside* an expansion stay macro-aware: a body's nested Filter
/// position invokes another Filter macro, remembered at its own level.
#[test]
fn macros_expand_inside_expansion_bodies() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Inner".into(),
            kinds: vec!["Filter".into()],
            params: Params::default(),
            body: "AllOf([Type(Creature)])".into(),
        })
        .unwrap();
    macros
        .insert(&MacroDef {
            name: "Outer".into(),
            kinds: vec!["Filter".into()],
            params: Params::default(),
            body: "OneOf([Any, Inner])".into(),
        })
        .unwrap();
    let filter: Filter = macros.read_str("Outer").unwrap();
    let Filter::Expanded(outer) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(outer.name, "Outer");
    let Filter::OneOf(arms) = *outer.value else {
        panic!("expected OneOf, got {:?}", outer.value);
    };
    assert_eq!(arms[0], Filter::Any);
    let Filter::Expanded(inner) = &arms[1] else {
        panic!("expected a nested wrapper, got {:?}", arms[1]);
    };
    assert_eq!(inner.name, "Inner");
    assert_eq!(
        *inner.value,
        Filter::AllOf(vec![Filter::Type(Type::Creature)])
    );
    assert_eq!(arms.len(), 2);
}

#[test]
fn effect_positions_expand_macros() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Investigate".into(),
            kinds: vec!["Effect".into()],
            params: Params::default(),
            body: "DrawCards(1)".into(),
        })
        .unwrap();
    let effect: Effect = macros.read_str("Investigate").unwrap();
    let Effect::Expanded(expanded) = effect else {
        panic!("expected a remembered effect, got {effect:?}");
    };
    assert_eq!(expanded.name, "Investigate");
    // The body's bare `1` hits the literal sugar *inside* the expansion.
    assert_eq!(*expanded.value, Effect::DrawCards(Quantity::Literal(1)));
}

/// A chain of remembering macros nests `Expanded` at each link: an Ability
/// macro whose body is another Ability macro's name yields one wrapper per
/// expansion, outermost first.
#[test]
fn remembering_chains_nest_expanded() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Inner".into(),
            kinds: vec!["Ability".into()],
            params: Params::default(),
            body: "Static(effects: [CantAttack])".into(),
        })
        .unwrap();
    macros
        .insert(&MacroDef {
            name: "Outer".into(),
            kinds: vec!["Ability".into()],
            params: Params::default(),
            body: "Inner".into(),
        })
        .unwrap();
    let ability: Ability = macros.read_str("Outer").unwrap();
    let Ability::Expanded(outer) = ability else {
        panic!("expected the outer wrapper, got {ability:?}");
    };
    assert_eq!(outer.name, "Outer");
    let Ability::Expanded(inner) = &*outer.value else {
        panic!("expected a nested wrapper, got {:?}", outer.value);
    };
    assert_eq!(inner.name, "Inner");
    assert_eq!(
        *inner.value,
        Ability::Static {
            effects: vec![StaticEffect::CantAttack],
        }
    );
}

/// A remembered invocation round-trips as the invocation: reading a nullary
/// Ability macro then serializing yields exactly its name, and a
/// parameterized Filter macro invocation serializes back to the original
/// call text — not the expansion.
#[test]
fn remembered_invocations_round_trip_as_invocations() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Flying".into(),
            kinds: vec!["Ability".into()],
            params: Params::default(),
            body: "Static(effects: [CantAttack])".into(),
        })
        .unwrap();
    macros
        .insert(&MacroDef {
            name: "OfType".into(),
            kinds: vec!["Filter".into()],
            params: Params::Positional(vec![ParamType("Any".into())]),
            body: "Type(Param(0))".into(),
        })
        .unwrap();

    let ability: Ability = macros.read_str("Flying").unwrap();
    assert_eq!(options().to_string(&ability).unwrap(), "Flying");

    let filter: Filter = macros.read_str("OfType(Creature)").unwrap();
    assert_eq!(options().to_string(&filter).unwrap(), "OfType(Creature)");
}

/// The raw argument source survives verbatim in the remembered args,
/// including a string literal with embedded quotes.
#[test]
fn argument_source_survives_verbatim() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "NamedAs".into(),
            kinds: vec!["Filter".into()],
            params: Params::Positional(vec![ParamType("String".into())]),
            body: "Named(Param(0))".into(),
        })
        .unwrap();
    let filter: Filter = macros.read_str(r#"NamedAs("Goblin \"Token\"")"#).unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(
        expanded.args,
        ExpansionArgs::Positional(vec![r#""Goblin \"Token\"""#.to_owned()]),
    );
    // And it serializes back to the exact invocation.
    assert_eq!(
        options().to_string(&Filter::Expanded(expanded)).unwrap(),
        r#"NamedAs("Goblin \"Token\"")"#,
    );
}

#[test]
fn params_resolve_at_enum_positions() {
    let mut macros = macros();
    macros
        .insert(&subtype_macro(
            "WithType",
            vec![ParamType("Any".into())],
            r#"Subtype(name: "Forest", types: [Param(0)])"#,
        ))
        .unwrap();
    let subtype: Subtype = macros.read_str("WithType(Land)").unwrap();
    assert_eq!(subtype, forest());
}

#[test]
fn params_resolve_per_argument() {
    let mut macros = macros();
    macros
        .insert(&subtype_macro(
            "Pair",
            vec![ParamType("String".into()), ParamType("Any".into())],
            "Subtype(name: Param(0), types: [Param(1)])",
        ))
        .unwrap();
    let subtype: Subtype = macros.read_str(r#"Pair("Forest", Land)"#).unwrap();
    assert_eq!(subtype, forest());
}

#[test]
fn named_parameters_invoke_struct_shaped() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Vanilla".into(),
            kinds: vec!["CardFace".into()],
            params: Params::Named(
                [
                    ("name".into(), ParamType("String".into())),
                    ("cost".into(), ParamType("Any".into())),
                ]
                .into(),
            ),
            body: r"CardFace(
                name: Param(name),
                mana_cost: [Generic(Param(cost))],
                types: [Creature],
            )"
            .into(),
        })
        .unwrap();

    let face: CardFace = macros
        .read_str(r#"Vanilla(name: "Bear", cost: 2)"#)
        .unwrap();
    assert_eq!(face.name, "Bear");
    assert_eq!(
        face.mana_cost,
        vec![ManaSymbol::Simple(SimpleManaSymbol::Generic(2))]
    );

    // Wrong argument names are errors, both ways.
    for call in [
        r#"Vanilla(name: "Bear")"#,
        r#"Vanilla(name: "Bear", cost: 2, power: 2)"#,
    ] {
        assert!(
            macros.read_str::<CardFace>(call).is_err(),
            "{call} should not expand",
        );
    }
}

#[test]
fn named_parameters_at_enum_positions() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Boast".into(),
            kinds: vec!["Ability".into()],
            params: Params::Named([("cost".into(), ParamType("String".into()))].into()),
            body: "Static(effects: [CantAttack])".into(),
        })
        .unwrap();
    let ability: Ability = macros.read_str(r#"Boast(cost: "{1}")"#).unwrap();
    let Ability::Expanded(expanded) = ability else {
        panic!("expected a remembered ability, got {ability:?}");
    };
    assert_eq!(expanded.name, "Boast");
    // The named argument's raw source survives verbatim.
    assert_eq!(
        expanded.args,
        ExpansionArgs::Named(vec![("cost".into(), r#""{1}""#.to_owned())]),
    );
}

#[test]
fn string_literals_mentioning_param_are_untouched() {
    // `Param` is only recognized where a value is expected, so it can
    // appear verbatim inside body strings.
    let mut macros = macros();
    macros
        .insert(&subtype_macro(
            "Weird",
            vec![],
            r#"Subtype(name: "literally Param(0)", types: [Land])"#,
        ))
        .unwrap();
    let subtype: Subtype = macros.read_str("Weird").unwrap();
    assert_eq!(subtype.name, "literally Param(0)");
}

#[test]
fn out_of_range_params_are_an_error() {
    let mut macros = macros();
    macros
        .insert(&subtype_macro(
            "OffByOne",
            vec![ParamType("String".into())],
            "Subtype(name: Param(1), types: [Land])",
        ))
        .unwrap();
    let error = macros
        .read_str::<Subtype>(r#"OffByOne("Forest")"#)
        .unwrap_err();
    assert!(
        error.to_string().contains("no Param(1)"),
        "unexpected error: {error}"
    );
}

#[test]
fn params_outside_macros_are_an_error() {
    let error = macros().read_str::<Subtype>("Param(0)").unwrap_err();
    assert!(
        error.to_string().contains("outside any macro expansion"),
        "unexpected error: {error}"
    );
}

#[test]
fn params_resolve_as_enum_variant_contents() {
    // `Generic(Param(1))`: a hole as the entire content of a newtype
    // variant, inside a partially untagged enum — content that buffers
    // through `deserialize_any`, where holes are spliced by offset.
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Vanilla".into(),
            kinds: vec!["CardFace".into()],
            params: Params::Positional(vec![ParamType("String".into()), ParamType("Any".into())]),
            body: r"CardFace(
                name: Param(0),
                mana_cost: [Hybrid(Generic(Param(1)), White), Green],
                types: [Creature],
            )"
            .into(),
        })
        .unwrap();
    let face: CardFace = macros.read_str(r#"Vanilla("Bear", 2)"#).unwrap();
    assert_eq!(face.name, "Bear");
    assert_eq!(
        face.mana_cost,
        vec![
            ManaSymbol::Hybrid(SimpleManaSymbol::Generic(2), Color::White),
            ManaSymbol::Simple(SimpleManaSymbol::Specific(Color::Green)),
        ]
    );
}

/// A bare numeral at a Quantity position is reader sugar for `Literal(N)`:
/// the type's grammar stays strict, the macro layer splices it.
#[test]
fn bare_numeral_at_quantity_is_literal_sugar() {
    let quantity: Quantity = empty().read_str("3").unwrap();
    assert_eq!(quantity, Quantity::Literal(3));
}

/// The strict form still reads through the macro layer unchanged.
#[test]
fn tagged_literal_reads_at_quantity() {
    let quantity: Quantity = empty().read_str("Literal(3)").unwrap();
    assert_eq!(quantity, Quantity::Literal(3));
}

/// `DealDamage(Target(0), 3)` — the bare `3` sits at a Quantity position
/// nested inside a verb — reads through the macro layer with the literal
/// spliced in.
#[test]
fn bare_numeral_nested_in_a_verb_is_literal() {
    let effect: Effect = empty().read_str("DealDamage(Target(0), 3)").unwrap();
    assert_eq!(
        effect,
        Effect::DealDamage(Selection::Target(0), Quantity::Literal(3)),
    );
}

/// `X` is a real Quantity variant — identifier-led, so the sugar path's
/// digit check passes it straight through to the enum reader.
#[test]
fn identifier_quantity_variants_still_read() {
    let quantity: Quantity = empty().read_str("X").unwrap();
    assert_eq!(quantity, Quantity::X);
}

/// A Filter macro inside `CountOf` at a Quantity position: macros work
/// *under* the sugar kind. The inner invocation is remembered as
/// `Filter::Expanded`.
#[test]
fn filter_macros_expand_under_quantity() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "AnyTargetish".into(),
            kinds: vec!["Filter".into()],
            params: Params::default(),
            body: "OneOf([Any, Type(Creature)])".into(),
        })
        .unwrap();
    let quantity: Quantity = macros.read_str("CountOf(AnyTargetish)").unwrap();
    let Quantity::CountOf(filter) = quantity else {
        panic!("expected CountOf, got {quantity:?}");
    };
    let Filter::Expanded(expanded) = *filter else {
        panic!("expected a remembered filter under CountOf, got {filter:?}");
    };
    assert_eq!(expanded.name, "AnyTargetish");
    let Filter::OneOf(arms) = *expanded.value else {
        panic!("expected OneOf, got {:?}", expanded.value);
    };
    assert_eq!(arms[0], Filter::Any);
    assert_eq!(arms.len(), 2);
}

/// A Quantity *macro* expands and is remembered: a nullary Quantity macro
/// reads as `Quantity::Expanded` wrapping its body.
#[test]
fn quantity_macros_expand_and_are_remembered() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "DevotionIsh".into(),
            kinds: vec!["Quantity".into()],
            params: Params::default(),
            body: "CountOf(Type(Creature))".into(),
        })
        .unwrap();
    let quantity: Quantity = macros.read_str("DevotionIsh").unwrap();
    let Quantity::Expanded(expanded) = quantity else {
        panic!("expected a remembered quantity, got {quantity:?}");
    };
    assert_eq!(expanded.name, "DevotionIsh");
    assert_eq!(
        *expanded.value,
        Quantity::CountOf(Box::new(Filter::Type(Type::Creature))),
    );
}

/// `Power(min: 2)` is a struct fused into a newtype variant by
/// `unwrap_variant_newtypes`; read inside a macro frame it must take the
/// `SkipStructs` path, not try to capture a whole value mid-stream.
#[test]
fn newtype_variant_struct_content_in_a_body() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Beefy".into(),
            kinds: vec!["Filter".into()],
            params: Params::default(),
            body: "Power(min: 2)".into(),
        })
        .unwrap();
    let filter: Filter = macros.read_str("Beefy").unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(*expanded.value, Filter::Power(PowerFilter { min: 2 }));
}

#[test]
fn unknown_param_types_are_an_error() {
    let mut macros = empty();
    let error = macros
        .insert(&subtype_macro(
            "Bogus",
            vec![ParamType("Sorcery".into())],
            "Subtype(name: Param(0), types: [Land])",
        ))
        .unwrap_err();
    assert_eq!(
        error,
        InsertError::UnknownParamType {
            type_name: "Sorcery".into(),
            name: "Bogus".into(),
        }
    );
    assert!(
        error.to_string().contains("param type"),
        "unexpected error: {error}"
    );
}

/// A `Param` hole at a Quantity position resolves to the argument and then
/// re-reads at that position — so a bare-numeral argument hits the
/// digit-sugar path: `DealDamage(Target(0), Param(0))` invoked with `3`.
#[test]
fn param_holes_resolve_at_quantity_positions() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "BoltFor".into(),
            kinds: vec!["Effect".into()],
            params: Params::Positional(vec![ParamType("Any".into())]),
            body: "DealDamage(Target(0), Param(0))".into(),
        })
        .unwrap();
    let effect: Effect = macros.read_str("BoltFor(3)").unwrap();
    let Effect::Expanded(expanded) = effect else {
        panic!("expected a remembered effect, got {effect:?}");
    };
    assert_eq!(expanded.name, "BoltFor");
    assert_eq!(
        *expanded.value,
        Effect::DealDamage(Selection::Target(0), Quantity::Literal(3)),
    );
}

#[test]
fn arg_type_mismatch_is_an_error() {
    // A `String` param rejects a bare (unquoted) argument at the call site,
    // naming the macro and the type — the type is enforced, not just counted.
    let mut macros = macros();
    macros
        .insert(&subtype_macro(
            "Named",
            vec![ParamType("String".into())],
            "Subtype(name: Param(0), types: [Land])",
        ))
        .unwrap();
    // Quoted: accepted, expands normally.
    let ok: Subtype = macros.read_str(r#"Named("Forest")"#).unwrap();
    assert_eq!(ok, forest());
    // Bare: rejected before expansion, with a message naming macro and type.
    let error = macros.read_str::<Subtype>("Named(Forest)").unwrap_err();
    let msg = error.to_string();
    assert!(
        msg.contains("Named") && msg.contains("String"),
        "unexpected error: {msg}"
    );
}

#[test]
fn any_accepts_every_shape() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Echo".into(),
            kinds: vec!["Filter".into()],
            params: Params::Positional(vec![ParamType("Any".into())]),
            body: "Param(0)".into(),
        })
        .unwrap();
    // A bare variant, a compound value, and a nested macro-free value all pass.
    for call in ["Echo(Any)", "Echo(Type(Land))", "Echo(OneOf([Any]))"] {
        assert!(
            macros.read_str::<Filter>(call).is_ok(),
            "{call} should expand"
        );
    }
}

#[test]
fn injected_param_types_validate() {
    // The embedder's path: register a domain validator, then it enforces.
    let mut param_types = ParamTypeSet::default();
    param_types.add("Number", |src, macros| {
        macros
            .read_str::<u32>(src)
            .map(drop)
            .map_err(|e| e.to_string())
    });
    let mut macros = MacroSet::new(kinds())
        .with_options(options())
        .with_param_types(param_types);
    macros
        .insert(&MacroDef {
            name: "Repeat".into(),
            kinds: vec!["Effect".into()],
            params: Params::Positional(vec![ParamType("Number".into())]),
            body: "DrawCards(Param(0))".into(),
        })
        .unwrap();
    // A number is accepted.
    assert!(macros.read_str::<Effect>("Repeat(2)").is_ok());
    // A non-number is rejected at the call site, naming macro and type.
    let error = macros.read_str::<Effect>("Repeat(Creature)").unwrap_err();
    let msg = error.to_string();
    assert!(
        msg.contains("Repeat") && msg.contains("Number"),
        "unexpected error: {msg}"
    );
}
