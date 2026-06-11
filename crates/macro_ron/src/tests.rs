//! The machinery test suite, on fixture types mirroring the serde patterns
//! consumers use: name-erasing struct kinds (`Subtype`, `CardFace`),
//! remembering enum kinds (`Ability`, `Effect`, `Filter`), and a
//! literal-sugar kind (`Quantity`).

use serde::Deserialize;
use serde::Serialize;
use serde::de::EnumAccess;
use serde::de::VariantAccess;
use serde::ser::SerializeStructVariant;
use serde::ser::Serializer;

use crate::Expansion;
use crate::ExpansionArgs;
use crate::Ident;
use crate::IdentSeed;
use crate::InsertError;
use crate::Kind;
use crate::KindSet;
use crate::MacroDef;
use crate::MacroSet;
use crate::ParamType;
use crate::ParamTypeSet;
use crate::Params;

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
    // Meta-macro positions: `MacroDef` reads (serde name "Macro").
    kinds.add(Kind::new("Macro"));
    kinds.add(Kind::new("Ability").remembers_expansion());
    kinds.add(Kind::new("Effect").remembers_expansion());
    kinds.add(Kind::new("Filter").remembers_expansion());
    kinds.add(
        Kind::new("Quantity")
            .remembers_expansion()
            .literal_wrapper("Literal"),
    );
    // embeds_untagged fixture kinds: EmbedHost embeds EmbedRef untagged;
    // EmbedRef remembers its own macro expansions.
    kinds.add(
        Kind::new("EmbedHost")
            .remembers_expansion()
            .embeds_untagged(),
    );
    kinds.add(Kind::new("EmbedRef").remembers_expansion());
    kinds
}

fn empty() -> MacroSet { MacroSet::new(kinds()).with_options(options()) }

/// Parses a definition from file-shaped source, as plugin loading does.
fn def(source: &str) -> MacroDef { options().from_str(source).unwrap() }

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
        Params::Positional(vec![ParamType::plain("String")])
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
            params: Params::Positional(vec![ParamType::plain("Any")]),
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
            params: Params::Positional(vec![ParamType::plain("Any")]),
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
            params: Params::Positional(vec![ParamType::plain("String")]),
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
            vec![ParamType::plain("Any")],
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
            vec![ParamType::plain("String"), ParamType::plain("Any")],
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
                    ("name".into(), ParamType::plain("String")),
                    ("cost".into(), ParamType::plain("Any")),
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
            params: Params::Named([("cost".into(), ParamType::plain("String"))].into()),
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
            vec![ParamType::plain("String")],
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
            params: Params::Positional(vec![ParamType::plain("String"), ParamType::plain("Any")]),
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
            vec![ParamType::plain("Sorcery")],
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
            params: Params::Positional(vec![ParamType::plain("Any")]),
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
            vec![ParamType::plain("String")],
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
            params: Params::Positional(vec![ParamType::plain("Any")]),
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
            params: Params::Positional(vec![ParamType::plain("Number")]),
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

/// A definition file read through the macro-aware reader: `kinds:` reads
/// bare identifiers via `deserialize_enum("", &[], …)`, which must fall
/// through to the visitor instead of erroring as an unknown variant.
#[test]
fn macro_defs_read_through_the_macro_aware_reader() {
    let def: MacroDef = empty()
        .read_str(r#"(name: "Bears", kinds: [Filter], body: Type(Creature))"#)
        .unwrap();
    assert_eq!(def.name, "Bears");
    assert_eq!(def.kinds, vec![Ident::from("Filter")]);
    assert_eq!(def.body(), "Type(Creature)");
}

/// Pins ron's private raw-value token, which the expand layer matches by
/// string. If a ron upgrade renames it, this fails before anything subtle.
#[test]
fn raw_value_token_drift_pin() {
    struct Spy<'a>(&'a std::cell::Cell<&'static str>);
    impl<'de> serde::Deserializer<'de> for Spy<'_> {
        type Error = serde::de::value::Error;
        fn deserialize_any<V: serde::de::Visitor<'de>>(
            self,
            _: V,
        ) -> Result<V::Value, Self::Error> {
            Err(serde::de::Error::custom("any"))
        }
        fn deserialize_newtype_struct<V: serde::de::Visitor<'de>>(
            self,
            name: &'static str,
            _: V,
        ) -> Result<V::Value, Self::Error> {
            self.0.set(name);
            Err(serde::de::Error::custom("spied"))
        }
        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
            string bytes byte_buf option unit unit_struct seq tuple
            tuple_struct map struct enum identifier ignored_any
        }
    }
    let seen = std::cell::Cell::new("");
    let _ = <&ron::value::RawValue as serde::Deserialize>::deserialize(Spy(&seen));
    assert_eq!(seen.get(), crate::expand::RAW_VALUE_TOKEN);
}

/// The meta-macro flow end to end: a `Macro`-kind macro whose body is a
/// definition template. Invoking it at a `MacroDef` read produces a
/// registrable definition with the frame's holes filled — including inside
/// the raw-captured `body`, which ordinary capture leaves dangling.
#[test]
fn meta_macro_produces_a_working_definition() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "CreatureType",
            kinds: [Macro],
            params: { "name": String, "template": String },
            body: (
                name: Param(name),
                kinds: [Subtype],
                body: Subtype(name: Param(template), types: [Creature]),
            ),
        )"#))
        .unwrap();
    let produced: MacroDef = macros
        .read_str(r#"CreatureType(name: "AssemblyWorker", template: "Assembly-Worker")"#)
        .unwrap();
    assert_eq!(produced.name, "AssemblyWorker");
    // The raw-captured body has the meta's holes filled (exact whitespace
    // follows the body source, so assert on content, not spelling).
    assert!(
        produced.body().contains(r#""Assembly-Worker""#) && !produced.body().contains("Param"),
        "unspliced body: {}",
        produced.body(),
    );
    macros.insert(&produced).unwrap();
    let subtype: Subtype = macros.read_str("AssemblyWorker").unwrap();
    assert_eq!(subtype.name, "Assembly-Worker");
    assert_eq!(subtype.types, [Type::Creature]);
}

/// A meta-produced definition may itself be parameterized: holes the
/// meta's frame doesn't resolve pass through the raw capture verbatim and
/// resolve at the produced macro's own invocation.
#[test]
fn unresolved_holes_pass_through_to_the_produced_definition() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "FilterMaker",
            kinds: [Macro],
            params: { "name": String },
            body: (
                name: Param(name),
                kinds: [Filter],
                params: { "extra": Any },
                body: AllOf([Type(Creature), Param(extra)]),
            ),
        )"#))
        .unwrap();
    let produced: MacroDef = macros
        .read_str(r#"FilterMaker(name: "CreatureAnd")"#)
        .unwrap();
    macros.insert(&produced).unwrap();
    let filter: Filter = macros
        .read_str(r#"CreatureAnd(extra: Named("Bear"))"#)
        .unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter");
    };
    assert_eq!(
        *expanded.value,
        Filter::AllOf(vec![
            Filter::Type(Type::Creature),
            Filter::Named("Bear".into()),
        ])
    );
}

/// A produced definition whose `body` is a single hole: resolved by the
/// meta's frame when the frame owns it, left verbatim when it belongs to
/// the produced definition.
#[test]
fn whole_body_holes_resolve_or_pass_through() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Alias",
            kinds: [Macro],
            params: { "name": String, "body": Any },
            body: (name: Param(name), kinds: [Filter], body: Param(body)),
        )"#))
        .unwrap();
    let produced: MacroDef = macros
        .read_str(r#"Alias(name: "Bears", body: Type(Creature))"#)
        .unwrap();
    assert_eq!(produced.body(), "Type(Creature)");

    macros
        .insert(&def(r#"(
            name: "Deferred",
            kinds: [Macro],
            params: { "name": String },
            body: (
                name: Param(name),
                kinds: [Filter],
                params: { "extra": Any },
                body: Param(extra),
            ),
        )"#))
        .unwrap();
    let produced: MacroDef = macros.read_str(r#"Deferred(name: "Itself")"#).unwrap();
    assert_eq!(produced.body(), "Param(extra)");
    macros.insert(&produced).unwrap();
    let filter: Filter = macros.read_str("Itself(extra: Any)").unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter");
    };
    assert_eq!(*expanded.value, Filter::Any);
}

/// A typo'd hole in a produced *nullary* definition surfaces when the
/// macro is expanded (the cards loader expands every declared subtype at
/// load, so this is a load-time error there).
#[test]
fn dangling_hole_in_a_produced_nullary_def_errors_at_expansion() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Meta",
            kinds: [Macro],
            params: { "name": String },
            body: (name: Param(name), kinds: [Filter], body: Named(Param(nme))),
        )"#))
        .unwrap();
    let produced: MacroDef = macros.read_str(r#"Meta(name: "Foo")"#).unwrap();
    macros.insert(&produced).unwrap();
    let err = macros.read_str::<Filter>("Foo").unwrap_err();
    assert!(err.to_string().contains("has no Param(nme)"), "{err}");
}

/// Parse-position hole resolution stays strict: an unknown hole in an
/// ordinary body is an error, pass-through is raw-capture-only.
#[test]
fn normal_bodies_still_reject_unknown_holes() {
    let mut macros = empty();
    macros
        .insert(&def(
            r#"(name: "Oops", kinds: [Filter], params: [Any], body: Type(Param(1)))"#,
        ))
        .unwrap();
    let err = macros.read_str::<Filter>("Oops(Creature)").unwrap_err();
    assert!(err.to_string().contains("has no Param(1)"), "{err}");
}

// ---------------------------------------------------------------------------
// embeds_untagged fixture
//
// `EmbedHost` mirrors `Selection`: it has its own variants and embeds
// `EmbedRef` untagged via `visit_newtype_struct`. `EmbedRef` mirrors
// `Reference`: a plain unit variant and a remembered `Expanded` variant for
// macro-in-slot tests.
//
// Both kinds are registered above in `kinds()`. `EmbedHost` is registered
// with `.remembers_expansion().embeds_untagged()`; `EmbedRef` with
// `.remembers_expansion()`.

/// The embedded "reference-like" type. A derive is enough because none of
/// its variants need special treatment.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum EmbedRef {
    Bare,
    Counted(u32),
    Expanded(Expansion<EmbedRef>),
}

/// The host type. Manual `Deserialize` so `visit_newtype_struct` can wrap an
/// `EmbedRef` in `Wrapped`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum EmbedHost {
    Own(u32),
    Wrapped(EmbedRef),
    Expanded(Expansion<EmbedHost>),
}

/// The host's own variant names — what the macro layer checks against to
/// decide whether to fall through to the embedded type.
const EMBED_HOST_VARIANTS: &[&str] = &["Own", "Wrapped", "Expanded"];

impl<'de> Deserialize<'de> for EmbedHost {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::fmt;

        use serde::de::Visitor;

        struct EmbedHostVisitor;

        impl<'de> Visitor<'de> for EmbedHostVisitor {
            type Value = EmbedHost;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an EmbedHost value or an EmbedRef value")
            }

            /// The untagged-embed fall-through: an identifier not belonging to
            /// `EmbedHost` arrives as newtype content; read it as an `EmbedRef`
            /// (which re-enters the macro layer under the `EmbedRef` namespace)
            /// and wrap it.
            fn visit_newtype_struct<D: serde::Deserializer<'de>>(
                self,
                de: D,
            ) -> Result<Self::Value, D::Error> {
                Ok(EmbedHost::Wrapped(EmbedRef::deserialize(de)?))
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                use serde::de::Error;
                let (ident, v) = data.variant_seed(IdentSeed)?;
                Ok(match ident.as_str() {
                    "Own" => EmbedHost::Own(v.newtype_variant()?),
                    "Wrapped" => EmbedHost::Wrapped(v.newtype_variant()?),
                    "Expanded" => EmbedHost::Expanded(v.newtype_variant()?),
                    other => {
                        return Err(A::Error::custom(format_args!(
                            "`{other}` is neither an EmbedHost variant nor an EmbedRef"
                        )));
                    }
                })
            }
        }

        deserializer.deserialize_enum("EmbedHost", EMBED_HOST_VARIANTS, EmbedHostVisitor)
    }
}

/// A host-native variant reads directly — the embed path is not taken.
#[test]
fn embed_host_own_variant_reads_directly() {
    let host: EmbedHost = empty().read_str("Own(7)").unwrap();
    assert_eq!(host, EmbedHost::Own(7));
}

/// A bare `EmbedRef` variant in a host slot falls through to the embedded
/// type and wraps in `Wrapped`.
#[test]
fn embed_host_bare_ref_variant_wraps() {
    let host: EmbedHost = empty().read_str("Bare").unwrap();
    assert_eq!(host, EmbedHost::Wrapped(EmbedRef::Bare));

    let host: EmbedHost = empty().read_str("Counted(3)").unwrap();
    assert_eq!(host, EmbedHost::Wrapped(EmbedRef::Counted(3)));
}

/// An `EmbedRef` macro in a host slot routes to the embedded type's
/// `Expanded`, not the host's: the `visit_newtype_struct` path re-enters the
/// macro layer under `EmbedRef`, so the macro is looked up there and
/// remembered as `EmbedRef::Expanded`.
#[test]
fn embed_host_ref_macro_routes_to_embedded_expanded() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "RefMacro".into(),
            kinds: vec!["EmbedRef".into()],
            params: Params::default(),
            body: "Bare".into(),
        })
        .unwrap();
    let host: EmbedHost = macros.read_str("RefMacro").unwrap();
    // The macro was looked up under EmbedRef and remembered there.
    let EmbedHost::Wrapped(EmbedRef::Expanded(expanded)) = host else {
        panic!("expected Wrapped(Expanded(…)), got {host:?}");
    };
    assert_eq!(expanded.name, "RefMacro");
    assert_eq!(*expanded.value, EmbedRef::Bare);
}

/// An `EmbedHost` macro in a host slot expands at the host level and is
/// remembered as `EmbedHost::Expanded` — the embed path is not taken.
#[test]
fn embed_host_own_macro_remembered_as_host_expanded() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "HostMacro".into(),
            kinds: vec!["EmbedHost".into()],
            params: Params::default(),
            body: "Own(1)".into(),
        })
        .unwrap();
    let host: EmbedHost = macros.read_str("HostMacro").unwrap();
    let EmbedHost::Expanded(expanded) = host else {
        panic!("expected EmbedHost::Expanded(…), got {host:?}");
    };
    assert_eq!(expanded.name, "HostMacro");
    assert_eq!(*expanded.value, EmbedHost::Own(1));
}

// ---------------------------------------------------------------------------
// #[derive(SupportsMacros)] fixtures
// ---------------------------------------------------------------------------

#[cfg(feature = "derive")]
mod derived {
    use crate::Expand;
    use crate::Expansion;
    use crate::KindSet;
    use crate::MacroDef;
    use crate::MacroSet;
    use crate::ParamType;
    use crate::Params;
    use crate::SupportsMacros;

    /// P1 fixture: unit, newtype, 2-tuple, literal, expanded.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Amount {
        X,
        Twice(Box<Amount>),
        Per(String, Box<Amount>),
        #[macro_ron(literal)]
        Lit(u32),
        #[macro_ron(expanded)]
        Expanded(Expansion<Amount>),
    }

    /// A `MacroSet` over the generated kind facts, with one macro:
    /// `Double(x)` expands to `Twice(x)`.
    fn amount_set() -> MacroSet {
        let mut kinds = KindSet::new();
        kinds.add(Amount::kind());
        let mut set = MacroSet::new(kinds).with_options(super::options());
        set.insert(&MacroDef {
            name: "Double".into(),
            kinds: vec!["Amount".into()],
            params: Params::Positional(vec![ParamType::plain("Any")]),
            body: "Twice(Param(0))".into(),
        })
        .unwrap();
        set
    }

    /// The generated `Serialize`/`Deserialize` round-trip plain variants
    /// through plain ron — no macro layer involved.
    #[test]
    fn p1_round_trips() {
        // unit variant: "X" ↔ Amount::X
        let unit_text = super::options().to_string(&Amount::X).unwrap();
        assert_eq!(unit_text, "X");
        let unit_back: Amount = super::options().from_str(&unit_text).unwrap();
        assert_eq!(unit_back, Amount::X);

        // newtype-with-Box variant: "Twice(X)" ↔ Amount::Twice(Box::new(Amount::X))
        let newtype = Amount::Twice(Box::new(Amount::X));
        let newtype_text = super::options().to_string(&newtype).unwrap();
        assert_eq!(newtype_text, "Twice(X)");
        let newtype_back: Amount = super::options().from_str(&newtype_text).unwrap();
        assert_eq!(newtype_back, newtype);

        // 2-tuple variant: write→read round-trip
        let amount = Amount::Per("land".to_owned(), Box::new(Amount::Lit(2)));
        let text = super::options().to_string(&amount).unwrap();
        assert!(text.starts_with("Per("), "{text}");
        let back: Amount = super::options().from_str(&text).unwrap();
        assert_eq!(back, amount);
    }

    /// `kind()` carries the literal-wrapper fact: a bare digit-led value at
    /// an `Amount` position reads as `Lit(N)`.
    #[test]
    fn p1_kind_facts() {
        let amount: Amount = amount_set().read_str("3").unwrap();
        assert_eq!(amount, Amount::Lit(3));
    }

    /// A macro invocation is remembered in the `expanded` variant, writes the
    /// invocation back, and `expanded()` constructs the same value.
    #[test]
    fn p1_expanded_writes_invocation_and_constructs() {
        let amount: Amount = amount_set().read_str("Double(Lit(2))").unwrap();
        let Amount::Expanded(e) = amount.clone() else {
            panic!("expected a remembered amount, got {amount:?}");
        };
        assert_eq!(e.name, "Double");
        assert_eq!(*e.value, Amount::Twice(Box::new(Amount::Lit(2))));
        assert_eq!(
            super::options().to_string(&amount).unwrap(),
            "Double(Lit(2))"
        );
        assert_eq!(Amount::expanded(e), Some(amount));
    }

    /// `expand_all` strips `Expanded` nodes recursively, rebuilding the tree.
    #[test]
    fn p1_expand_all_strips_recursively() {
        let amount: Amount = amount_set().read_str("Twice(Double(Lit(2)))").unwrap();
        assert_eq!(
            amount.expand_all(),
            Amount::Twice(Box::new(Amount::Twice(Box::new(Amount::Lit(2))))),
        );
    }

    /// Struct-variant fixture: generated helper struct with forwarded serde
    /// field attrs (default) reading flat via `unwrap_variant_newtypes`.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Clause {
        When {
            verb: String,
            #[serde(default)]
            count: u32,
        },
        #[macro_ron(expanded)]
        Expanded(Expansion<Clause>),
    }

    /// A struct variant reads flat (`When(verb: "draw")`), the forwarded
    /// `#[serde(default)]` fills the missing field, and a full value
    /// round-trips through write→read.
    #[test]
    fn struct_variant_round_trips_with_defaults() {
        // The defaulted field is absent in the text; serde fills it.
        let read: Clause = super::options().from_str(r#"When(verb: "draw")"#).unwrap();
        assert_eq!(
            read,
            Clause::When {
                verb: "draw".into(),
                count: 0,
            }
        );

        // Write→read round-trip with every field set.
        let clause = Clause::When {
            verb: "draw".into(),
            count: 3,
        };
        let text = super::options().to_string(&clause).unwrap();
        let back: Clause = super::options().from_str(&text).unwrap();
        assert_eq!(back, clause);
    }

    /// Embed fixtures, mirroring `Selection::Ref` (newtype, name-erased) and
    /// `Action::By` (tuple, defaulted head).
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Who {
        Me,
        Them,
        #[macro_ron(expanded)]
        Expanded(Expansion<Who>),
    }

    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Pick {
        All,
        #[macro_ron(embed)]
        Ref(Who),
        #[macro_ron(expanded)]
        Expanded(Expansion<Pick>),
    }

    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Deed {
        Smash(u32),
        #[macro_ron(embed)]
        By(#[macro_ron(default = "Who::Me")] Who, Box<Pick>),
    }

    /// A newtype embed is name-erased: the embedded type's variants read
    /// bare at the host position (the `from_variant` fall-through), the
    /// write is always bare, and the variant's own tag is not a name.
    #[test]
    fn newtype_embed_lifts_and_writes_bare() {
        // Plain-ron path (no macro layer): an unknown ident dispatches into
        // Who via the from_variant fall-through.
        let read: Pick = super::options().from_str("Them").unwrap();
        assert_eq!(read, Pick::Ref(Who::Them));

        // The write is ALWAYS bare.
        let text = super::options().to_string(&Pick::Ref(Who::Them)).unwrap();
        assert_eq!(text, "Them");

        // "Ref" is name-erased — the tag itself is not part of the grammar.
        assert!(super::options().from_str::<Pick>("Ref(Them)").is_err());
    }

    /// A tuple embed fills defaulted fields on read, writes bare only when
    /// every defaulted field equals its default, and keeps its tag otherwise.
    #[test]
    fn tuple_embed_defaults_head_and_round_trips() {
        // The payload is the Box<Pick> field — exercises Box peeling.
        let read: Deed = super::options().from_str("All").unwrap();
        assert_eq!(read, Deed::By(Who::Me, Box::new(Pick::All)));
        let Deed::By(head, payload) = &read else { panic!("expected By") };
        assert_eq!(head, &Who::Me, "defaulted head");
        assert_eq!(payload.as_ref(), &Pick::All, "Box-peeled payload");

        // The default head writes bare …
        let text = super::options().to_string(&read).unwrap();
        assert_eq!(text, "All");

        // … and a non-default head keeps the tag and round-trips.
        let deed = Deed::By(Who::Them, Box::new(Pick::All));
        let text = super::options().to_string(&deed).unwrap();
        assert!(text.starts_with("By("), "{text}");
        let back: Deed = super::options().from_str(&text).unwrap();
        assert_eq!(back, deed);
    }

    /// An unknown ident at a Pick position routes through the macro layer's
    /// embed hook (`visit_newtype_struct`) into the *Who* namespace, where
    /// the macro expands and is remembered as `Who::Expanded` — and the
    /// invocation writes back through the bare embed.
    #[test]
    fn embed_macro_layer_falls_through_to_embedded_namespace() {
        let mut kinds = KindSet::new();
        kinds.add(Who::kind());
        kinds.add(Pick::kind());
        let mut set = MacroSet::new(kinds).with_options(super::options());
        set.insert(&MacroDef {
            name: "Us".into(),
            kinds: vec!["Who".into()],
            params: Params::default(),
            body: "Me".into(),
        })
        .unwrap();

        let pick: Pick = set.read_str("Us").unwrap();
        let Pick::Ref(Who::Expanded(e)) = &pick else {
            panic!("expected Ref(Expanded(…)), got {pick:?}");
        };
        assert_eq!(e.name, "Us");
        assert_eq!(*e.value, Who::Me);

        // Invocation write-back through the bare embed.
        assert_eq!(super::options().to_string(&pick).unwrap(), "Us");
    }

    /// The variant lists pin the name-erasure semantics: a newtype embed's
    /// tag is absent from `OWN_VARIANTS`, a tuple embed's is kept, and
    /// `ALL_VARIANTS` appends the embedded type's dispatch set.
    #[test]
    fn variant_list_composition() {
        assert_eq!(<Deed as SupportsMacros>::OWN_VARIANTS, &["Smash", "By"]);
        assert_eq!(<Pick as SupportsMacros>::OWN_VARIANTS, &["All", "Expanded"]);
        // The duplicate "Expanded" is intentional — once from Pick's own slot,
        // once appended from Who's dispatch set. concat_variants does not
        // dedupe; OWN_VARIANTS drives ownership decisions, and ALL_VARIANTS is
        // only a membership/hint set for the macro layer.
        assert_eq!(
            <Pick as SupportsMacros>::ALL_VARIANTS,
            &["All", "Expanded", "Me", "Them", "Expanded"],
        );
    }

    /// Flatten fixture, mirroring `Effect::Act(Action)`: inner names lift,
    /// write is transparent, and flatten composes with the inner's embed.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Step {
        Wait,
        #[macro_ron(flatten)]
        Do(Deed),
        #[macro_ron(expanded)]
        Expanded(Expansion<Step>),
    }

    /// A flattened compartment lifts the payload's accepted names into the
    /// host position — transitively through the payload's own embed — and
    /// writes transparently (the `Do` tag never appears in text).
    #[test]
    fn flatten_lifts_inner_names_transitively() {
        // The inner's own variant lifts.
        let read: Step = super::options().from_str("Smash(3)").unwrap();
        assert_eq!(read, Step::Do(Deed::Smash(3)));

        // Reached through Deed's embed — flatten composes transitively.
        let read: Step = super::options().from_str("All").unwrap();
        assert_eq!(read, Step::Do(Deed::By(Who::Me, Box::new(Pick::All))));

        // Transparent write: the flatten arm delegates to the payload …
        let step = Step::Do(Deed::Smash(3));
        let text = super::options().to_string(&step).unwrap();
        assert!(text.starts_with("Smash("), "{text}");
        let back: Step = super::options().from_str(&text).unwrap();
        assert_eq!(back, step);

        // … and composes with the inner's bare embed (bare through BOTH
        // layers: Do is erased by flatten, By by its all-default head).
        let step = Step::Do(Deed::By(Who::Me, Box::new(Pick::All)));
        let text = super::options().to_string(&step).unwrap();
        assert_eq!(text, "All");
        let back: Step = super::options().from_str(&text).unwrap();
        assert_eq!(back, step);

        // An ordinary unit variant is untouched by the flatten machinery.
        let text = super::options().to_string(&Step::Wait).unwrap();
        assert_eq!(text, "Wait");
        let back: Step = super::options().from_str(&text).unwrap();
        assert_eq!(back, Step::Wait);
    }

    /// The variant lists pin flatten's name-erasure: the `Do` tag is erased
    /// from `OWN_VARIANTS`, and `ALL_VARIANTS` appends the payload's full
    /// dispatch set.
    #[test]
    fn flatten_variant_lists() {
        assert_eq!(
            <Step as SupportsMacros>::OWN_VARIANTS,
            &["Wait", "Expanded"]
        );
        // ["Wait", "Expanded"] ++ Deed::ALL_VARIANTS, which is itself
        // ["Smash", "By"] ++ Pick::ALL_VARIANTS. The repeated "Expanded"
        // entries are intentional — one from Step's own slot, one from Pick's,
        // one from Who's; concat_variants does not dedupe (ALL_VARIANTS is a
        // membership set for dispatch, not an ownership list).
        assert_eq!(
            <Step as SupportsMacros>::ALL_VARIANTS,
            &[
                "Wait", "Expanded", "Smash", "By", "All", "Expanded", "Me", "Them", "Expanded"
            ],
        );
    }

    /// Flatten lifts *names*, not macro namespaces: Step has no embed
    /// variant, so its kind has `embeds_untagged = false` and an unknown
    /// ident at a Step position is looked up among *Step* macros only. A
    /// `Who` macro does not expand there (mirroring production Effect:
    /// `PlayerAction` macros do not expand at Effect slots).
    #[test]
    fn flatten_does_not_open_macro_namespaces() {
        let mut kinds = KindSet::new();
        kinds.add(Who::kind());
        kinds.add(Pick::kind());
        kinds.add(Step::kind());
        let mut set = MacroSet::new(kinds).with_options(super::options());
        set.insert(&MacroDef {
            name: "Us".into(),
            kinds: vec!["Who".into()],
            params: Params::default(),
            body: "Me".into(),
        })
        .unwrap();

        // "Us" is not in Step's native list and Step doesn't embed, so the
        // macro layer tries Step macros, finds none, and errors.
        assert!(set.read_str::<Step>("Us").is_err());
    }

    /// `expand_all` strips `Expanded` nodes through both `flatten` and `embed`
    /// arms, recursing into all fields.
    ///
    /// * Identity: plain variants are returned unchanged.
    /// * Embed seam: a `Pick` read via the `Who` macro carries
    ///   `Pick::Ref(Who::Expanded(..))` from the macro layer; `expand_all`
    ///   strips that inner `Expanded`, yielding `Pick::Ref(Who::Me)`.
    /// * Tuple embed: wrapping that `Pick` in a `Deed::By` and calling
    ///   `expand_all` recurses through the `Box` in the tuple field.
    #[test]
    fn expand_all_strips_through_flatten_and_embed() {
        // Build the same MacroSet as `flatten_does_not_open_macro_namespaces`
        // and `embed_macro_layer_falls_through_to_embedded_namespace`.
        let mut kinds = KindSet::new();
        kinds.add(Who::kind());
        kinds.add(Pick::kind());
        kinds.add(Step::kind());
        let mut set = MacroSet::new(kinds).with_options(super::options());
        set.insert(&MacroDef {
            name: "Us".into(),
            kinds: vec!["Who".into()],
            params: Params::default(),
            body: "Me".into(),
        })
        .unwrap();

        // Identity: unit and plain newtype variants are unchanged.
        assert_eq!(Step::Wait.expand_all(), Step::Wait);
        assert_eq!(
            Step::Do(Deed::Smash(3)).expand_all(),
            Step::Do(Deed::Smash(3))
        );

        // The real seam: read "Us" at a Pick position — routes through the
        // embed hook into the Who namespace, yielding Pick::Ref(Who::Expanded).
        let pick: Pick = set.read_str("Us").unwrap();
        let Pick::Ref(Who::Expanded(_)) = &pick else {
            panic!("expected Pick::Ref(Who::Expanded(…)), got {pick:?}");
        };
        // expand_all strips the nested Expanded, regardless of the embed layer.
        assert_eq!(pick.expand_all(), Pick::Ref(Who::Me));

        // Tuple embed: Deed::By holds a Box<Pick>; expand_all recurses through
        // the Box in the second field and strips the inner Expanded there too.
        let pick2: Pick = set.read_str("Us").unwrap();
        let deed = Deed::By(Who::Me, Box::new(pick2));
        assert_eq!(
            deed.expand_all(),
            Deed::By(Who::Me, Box::new(Pick::Ref(Who::Me)))
        );
    }

    /// Standalone Expand derive on a plain struct + enum.
    #[derive(Debug, Clone, PartialEq, crate::Expand)]
    struct Plain {
        who: Who2,
        n: u32,
    }

    #[derive(Debug, Clone, PartialEq, crate::Expand)]
    enum Who2 {
        Me,
        Named(String),
    }

    #[test]
    fn plain_expand_recurses_fields() {
        let p = Plain {
            who: Who2::Named("x".into()),
            n: 3,
        };
        assert_eq!(p.clone().expand_all(), p);

        // The unit-variant arm is identity too.
        let p = Plain {
            who: Who2::Me,
            n: 0,
        };
        assert_eq!(p.clone().expand_all(), p);
    }
}

/// Macro names appear as bare identifiers at value positions, so a
/// non-ident name is dead on arrival — rejected at registration, loudly.
#[test]
fn non_ident_macro_names_are_rejected_at_insert() {
    let err = empty()
        .insert(&def(
            r#"(name: "Assembly-Worker", kinds: [Filter], body: Any)"#,
        ))
        .unwrap_err();
    assert!(matches!(err, InsertError::InvalidName { .. }), "{err}");
}

/// A meta-macro's holes and its produced definition's holes share the
/// body text: indices can't tell the levels apart, so metas must use
/// named params.
#[test]
fn positional_meta_macros_are_rejected_at_insert() {
    let err = empty()
        .insert(&def(r#"(
                name: "Meta",
                kinds: [Macro],
                params: [String],
                body: (name: Param(0), kinds: [Filter], body: Any),
            )"#))
        .unwrap_err();
    assert!(
        matches!(err, InsertError::MetaParamsPositional { .. }),
        "{err}"
    );
}

/// `Default(Type, <expr>)` in a named signature parses: the inner type name
/// and the raw default expression, captured verbatim.
#[test]
fn default_param_type_parses() {
    let def = def(r#"(
        name: "M",
        kinds: [Subtype],
        params: { "name": String, "template": Default(String, Param(name)) },
        body: Subtype(name: Param(template), types: [Creature]),
    )"#);
    let Params::Named(signature) = &def.params else {
        panic!("expected named params");
    };
    let name = &signature[&Ident::from("name")];
    assert_eq!(name.name, "String");
    assert_eq!(name.default, None);
    let template = &signature[&Ident::from("template")];
    assert_eq!(template.name, "String");
    assert_eq!(template.default.as_deref(), Some("Param(name)"));
}

/// Defaults are named-only: a positional signature with a `Default(...)`
/// param is rejected at insert (trailing-default arity games are out of
/// scope).
#[test]
fn positional_default_is_rejected_at_insert() {
    let error = empty()
        .insert(&def(r#"(
            name: "M",
            kinds: [Subtype],
            params: [Default(String, "x")],
            body: Subtype(name: Param(0), types: []),
        )"#))
        .unwrap_err();
    assert!(matches!(error, InsertError::PositionalDefault { .. }));
}

/// A default may reference only non-defaulted siblings: referencing another
/// defaulted param is rejected (kills fill-order questions and cycles).
#[test]
fn default_referencing_defaulted_param_is_rejected() {
    let error = empty()
        .insert(&def(r#"(
            name: "M",
            kinds: [Subtype],
            params: { "a": Default(String, Param(b)), "b": Default(String, "x") },
            body: Subtype(name: Param(a), types: []),
        )"#))
        .unwrap_err();
    assert!(matches!(error, InsertError::BadDefault { .. }), "{error}");
}

/// Referencing a param that doesn't exist is rejected at insert, not left
/// to fail at invocation time.
#[test]
fn default_referencing_unknown_param_is_rejected() {
    let error = empty()
        .insert(&def(r#"(
            name: "M",
            kinds: [Subtype],
            params: { "a": Default(String, Param(nope)) },
            body: Subtype(name: Param(a), types: []),
        )"#))
        .unwrap_err();
    assert!(matches!(error, InsertError::BadDefault { .. }), "{error}");
}

/// `Param(0)`-style index holes never resolve in a named signature.
#[test]
fn default_with_index_hole_is_rejected() {
    let error = empty()
        .insert(&def(r#"(
            name: "M",
            kinds: [Subtype],
            params: { "a": Default(String, Param(0)) },
            body: Subtype(name: Param(a), types: []),
        )"#))
        .unwrap_err();
    assert!(matches!(error, InsertError::BadDefault { .. }), "{error}");
}

/// The inner type of a `Default(...)` goes through the same registration
/// check as a plain type name.
#[test]
fn default_inner_type_must_be_registered() {
    let error = empty()
        .insert(&def(r#"(
            name: "M",
            kinds: [Subtype],
            params: { "a": Default(Bogus, "x") },
            body: Subtype(name: Param(a), types: []),
        )"#))
        .unwrap_err();
    assert!(matches!(error, InsertError::UnknownParamType { .. }));
}

/// The headline: an omitted defaulted arg fills from the default expression,
/// whose hole resolves against the *supplied* args.
#[test]
fn omitted_defaulted_arg_fills_from_sibling() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Named",
            kinds: [Subtype],
            params: { "name": String, "label": Default(String, Param(name)) },
            body: Subtype(name: Param(label), types: [Creature]),
        )"#))
        .unwrap();
    let subtype: Subtype = macros.read_str(r#"Named(name: "Zombie")"#).unwrap();
    assert_eq!(subtype.name, "Zombie");
}

/// A supplied arg overrides its default.
#[test]
fn supplied_arg_overrides_default() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Named",
            kinds: [Subtype],
            params: { "name": String, "label": Default(String, Param(name)) },
            body: Subtype(name: Param(label), types: [Creature]),
        )"#))
        .unwrap();
    let subtype: Subtype = macros
        .read_str(r#"Named(name: "AssemblyWorker", label: "Assembly-Worker")"#)
        .unwrap();
    assert_eq!(subtype.name, "Assembly-Worker");
}

/// A hole-free literal default fills verbatim.
#[test]
fn literal_default_fills() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Sized",
            kinds: [Filter],
            params: { "min": Default(Any, 1) },
            body: Power(min: Param(min)),
        )"#))
        .unwrap();
    let filter: Filter = macros.read_str("Sized()").unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter");
    };
    assert_eq!(*expanded.value, Filter::Power(PowerFilter { min: 1 }));
}

/// Omitting a *non-defaulted* arg still errors as before.
#[test]
fn omitted_required_arg_still_errors() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Named",
            kinds: [Subtype],
            params: { "name": String, "label": Default(String, Param(name)) },
            body: Subtype(name: Param(label), types: [Creature]),
        )"#))
        .unwrap();
    let error = macros.read_str::<Subtype>("Named()").unwrap_err();
    assert!(
        error.to_string().contains("missing argument `name`"),
        "unexpected error: {error}"
    );
}

/// The filled text is validated against the inner type like a supplied
/// argument, with the macro and param named in the error.
#[test]
fn filled_default_is_validated() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Named",
            kinds: [Subtype],
            params: { "label": Default(String, Bear) },
            body: Subtype(name: Param(label), types: [Creature]),
        )"#))
        .unwrap();
    let error = macros.read_str::<Subtype>("Named()").unwrap_err();
    let msg = error.to_string();
    assert!(
        msg.contains("Named") && msg.contains("label") && msg.contains("String"),
        "unexpected error: {msg}"
    );
}

/// A remembering kind round-trips the *short* invocation: filled defaults
/// are excluded from the synthesized args (re-reading re-fills them).
#[test]
fn remembered_invocation_excludes_filled_defaults() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Sized",
            kinds: [Filter],
            params: { "kind": Any, "min": Default(Any, 1) },
            body: AllOf([Type(Param(kind)), Power(min: Param(min))]),
        )"#))
        .unwrap();
    let filter: Filter = macros.read_str("Sized(kind: Creature)").unwrap();
    assert_eq!(
        options().to_string(&filter).unwrap(),
        "Sized(kind:Creature)"
    );
    // An explicit override IS remembered.
    let filter: Filter = macros.read_str("Sized(kind: Creature, min: 3)").unwrap();
    assert_eq!(
        options().to_string(&filter).unwrap(),
        "Sized(kind:Creature,min:3)"
    );
}

/// All params defaulted: the empty named call `M()` survives the round trip
/// (`Named([])` synthesizes as an empty struct call, which re-reads).
#[test]
fn all_defaulted_invocation_round_trips() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Sized",
            kinds: [Filter],
            params: { "min": Default(Any, 1) },
            body: Power(min: Param(min)),
        )"#))
        .unwrap();
    let filter: Filter = macros.read_str("Sized()").unwrap();
    let written = options().to_string(&filter).unwrap();
    assert_eq!(written, "Sized()");
    let reread: Filter = macros.read_str(&written).unwrap();
    assert_eq!(reread, filter);
}

/// A meta-macro's own params may be defaulted: the subtype meta-macro shape
/// this feature was built for, end to end.
#[test]
fn meta_macro_with_defaulted_template_param() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "CreatureType",
            kinds: [Macro],
            params: { "name": String, "template": Default(String, Param(name)) },
            body: (
                name: Param(name),
                kinds: [Subtype],
                body: Subtype(name: Param(template), types: [Creature]),
            ),
        )"#))
        .unwrap();
    // Omitted template: defaults to the name.
    let produced: MacroDef = macros.read_str(r#"CreatureType(name: "Zombie")"#).unwrap();
    assert_eq!(produced.name, "Zombie");
    macros.insert(&produced).unwrap();
    let subtype: Subtype = macros.read_str("Zombie").unwrap();
    assert_eq!(subtype.name, "Zombie");
    // Supplied template: overrides.
    let produced: MacroDef = macros
        .read_str(r#"CreatureType(name: "AssemblyWorker", template: "Assembly-Worker")"#)
        .unwrap();
    macros.insert(&produced).unwrap();
    let subtype: Subtype = macros.read_str("AssemblyWorker").unwrap();
    assert_eq!(subtype.name, "Assembly-Worker");
}

/// A default expression inside a meta BODY (a produced definition's own
/// `params:`) rides the raw-capture splice: holes the meta frame owns fill
/// into the default text, holes naming the produced def's own params
/// survive verbatim and fill at the produced macro's invocation.
#[test]
fn produced_definition_defaults_splice_and_pass_through() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "FilterMaker",
            kinds: [Macro],
            params: { "name": String, "fallback": Any },
            body: (
                name: Param(name),
                kinds: [Filter],
                params: { "kind": Any, "extra": Default(Any, Type(Param(fallback))) },
                body: AllOf([Type(Param(kind)), Param(extra)]),
            ),
        )"#))
        .unwrap();
    let produced: MacroDef = macros
        .read_str(r#"FilterMaker(name: "KindAnd", fallback: Land)"#)
        .unwrap();
    macros.insert(&produced).unwrap();
    // `extra` omitted: the default — `Type(Land)` after the meta spliced
    // `fallback` — fills. `kind` (produced def's own param) passed through.
    let filter: Filter = macros.read_str("KindAnd(kind: Creature)").unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter");
    };
    assert_eq!(
        *expanded.value,
        Filter::AllOf(vec![Filter::Type(Type::Creature), Filter::Type(Type::Land)])
    );
}

/// A nested `Default(Default(...), ...)` is malformed, not a type name.
#[test]
fn nested_default_is_rejected_at_parse() {
    let error = options()
        .from_str::<MacroDef>(
            r#"(
            name: "M",
            kinds: [Subtype],
            params: { "p": Default(Default(String, "x"), "y") },
            body: Subtype(name: "n", types: []),
        )"#,
        )
        .unwrap_err();
    assert!(
        error.to_string().contains("Default"),
        "unexpected error: {error}"
    );
}

mod support_runtime {
    use crate::Expand;
    use crate::concat_variants;

    #[test]
    fn concat_variants_concatenates_in_order() {
        const A: &[&str] = &["X", "Y"];
        const B: &[&str] = &["Z"];
        const ALL: &[&str] = &concat_variants::<{ A.len() + B.len() }>(&[A, B]);
        assert_eq!(ALL, &["X", "Y", "Z"]);
    }

    #[test]
    fn expand_containers_recurse() {
        // Leaves are identity; containers map through.
        assert_eq!(3u32.expand_all(), 3);
        assert_eq!(vec![1u32, 2].expand_all(), vec![1, 2]);
        assert_eq!(Some(Box::new(7u32)).expand_all(), Some(Box::new(7)));
        assert_eq!(
            (1u32, String::from("a")).expand_all(),
            (1, String::from("a"))
        );
    }
}
