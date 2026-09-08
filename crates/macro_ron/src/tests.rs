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
use crate::ParamDefault;
use crate::ParamType;
use crate::ParamTypeSet;
use crate::Params;
use crate::VariantSignature;

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
    kinds.add(
        Kind::new("Filter")
            .remembers_expansion()
            .with_variants(FILTER_VARIANTS)
            .with_signatures(FILTER_SIGNATURES),
    );
    kinds.add(
        Kind::new("Quantity")
            .remembers_expansion()
            .literal_wrapper("Literal")
            // Declared so the cycle check can tell `Literal`'s identity macro
            // from a self-invocation, the way the derive supplies it in anger.
            .with_variants(QUANTITY_VARIANTS),
    );
    // embeds_untagged fixture kinds: EmbedHost embeds EmbedRef untagged;
    // EmbedRef remembers its own macro expansions.
    kinds.add(
        Kind::new("EmbedHost")
            .remembers_expansion()
            .embeds_untagged()
            .with_variants(EMBED_HOST_VARIANTS),
    );
    kinds.add(
        Kind::new("EmbedRef")
            .remembers_expansion()
            .with_variants(EMBED_REF_VARIANTS),
    );
    // A non-remembering position kind (no `Expanded` variant), like
    // deckmaste's `Modification`: exercises nested-macro param forwarding —
    // a body invoking another macro can forward its own `Param`s into that
    // invocation's arguments (eager pre-substitution in `read_args`).
    kinds.add(Kind::new("Modification"));
    kinds
}

fn empty() -> MacroSet {
    // `Count` and `NumericOp` back the nested-macro param-forwarding fixture
    // (`Modification`/`NumericOp` above): a plain numeric literal and the
    // `NumericOp` enum itself, both real registered param types like any
    // domain type an embedder injects (mirrors
    // `injected_param_types_validate`).
    let mut param_types = ParamTypeSet::default();
    param_types.add_typed::<u32>("Count");
    param_types.add_typed::<NumericOp>("NumericOp");
    MacroSet::new(kinds())
        .with_options(options())
        .with_param_types(param_types)
}

/// Parses a definition from file-shaped source, as plugin loading does.
fn def(source: &str) -> MacroDef {
    options().from_str(source).unwrap()
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
struct ConsumerMetadata {
    spelling: String,
}

#[test]
fn definition_metadata_is_consumer_typed_and_defaults_to_unit() {
    let ordinary = def(r#"(name:"Plain",kinds:[Subtype],body:Type(Land))"#);
    assert_eq!(ordinary.metadata(), &());

    let typed: MacroDef<ConsumerMetadata> = options()
        .from_str(
            r#"(
                name: "Typed",
                kinds: [Subtype],
                metadata: (spelling: "typed"),
                body: Type(Land),
            )"#,
        )
        .unwrap();
    assert_eq!(typed.metadata().spelling, "typed");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum Type {
    Land,
    Creature,
}

/// `Filter`'s dispatch set. Hand-written because the fixture types carry
/// hand-written `Deserialize` impls rather than the derive that would supply
/// `ALL_VARIANTS`.
const FILTER_VARIANTS: &[&str] = &[
    "Any", "Type", "Named", "OneOf", "AllOf", "Power", "Expanded",
];

/// `Filter`'s signature lookup, hand-written for the same reason
/// `FILTER_VARIANTS` is: every non-unit variant is a newtype.
const FILTER_SIGNATURES: &[(&str, VariantSignature)] = &[
    ("Any", VariantSignature::Unit),
    (
        "Type",
        VariantSignature::Positional(&[ParamDefault::Required]),
    ),
    (
        "Named",
        VariantSignature::Positional(&[ParamDefault::Required]),
    ),
    (
        "OneOf",
        VariantSignature::Positional(&[ParamDefault::Required]),
    ),
    (
        "AllOf",
        VariantSignature::Positional(&[ParamDefault::Required]),
    ),
    (
        "Power",
        VariantSignature::Positional(&[ParamDefault::Required]),
    ),
    (
        "Expanded",
        VariantSignature::Positional(&[ParamDefault::Required]),
    ),
];

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

/// `Quantity`'s dispatch set, hand-written like [`FILTER_VARIANTS`].
const QUANTITY_VARIANTS: &[&str] = &["X", "CountOf", "Literal", "Expanded"];

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

/// A `NumericOp`-shaped param type, like deckmaste's own: the payload a
/// nested-macro invocation forwards a caller's numeric `Param` into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum NumericOp {
    Up(u32),
}

/// The `Modification` position kind: a non-remembering enum (unlike
/// `Filter`/`Ability`/`Effect` above) whose `Several` variant flattens a
/// list of modifications — the nested-macro param-forwarding fixture.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
enum Modification {
    Power(NumericOp),
    Toughness(NumericOp),
    Several(Vec<Modification>),
}

fn subtype_macro(name: &str, params: Vec<ParamType>, body: &str) -> MacroDef {
    MacroDef {
        name: name.into(),
        kinds: vec!["Subtype".into()],
        params: Params::Positional(params),
        template: None,
        plural: None,
        frames: Vec::new(),
        metadata: (),
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
    let err = macros()
        .read_str::<Subtype>(r#"IslandType("Tropical")"#)
        .unwrap_err();
    assert!(err.to_string().contains("IslandType"));
}

#[test]
fn wrong_arity_is_an_error() {
    for call in ["LandType", r#"LandType("Forest", "Island")"#] {
        let err = macros().read_str::<Subtype>(call).unwrap_err();
        assert!(
            {
                let err = err.to_string();
                err.contains("Expected opening `(`")
                    || err.contains("Expected a `ron::value::RawValue`")
            },
            "{call} should fail with a parser-structure error, got: {err}"
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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

/// A macro cycle declared this way used to only blow the runtime
/// `MAX_DEPTH` expansion cap the first time it was read (see the `depth`
/// guard in `expand.rs`, whose message is still "macros don't recurse");
/// the static cycle check now rejects it at `declare` — which routes
/// through `insert` — so it can no longer be constructed at all.
#[test]
fn recursion_is_rejected_at_declare_not_only_at_runtime() {
    let mut macros = macros();
    let error = macros
        .declare("Subtype", "Ouroboros".into(), "Ouroboros")
        .unwrap_err();
    assert!(matches!(error, InsertError::Cycle { .. }), "{error}");

    // `Ping` naming not-yet-declared `Pong` is fine — the walk finds no
    // edge to a macro that doesn't exist yet — but `Pong` naming `Ping`
    // back closes the loop.
    macros.declare("Subtype", "Ping".into(), "Pong").unwrap();
    let error = macros
        .declare("Subtype", "Pong".into(), "Ping")
        .unwrap_err();
    assert!(matches!(error, InsertError::Cycle { .. }), "{error}");
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "AllOf([Type(Creature)])".into(),
        })
        .unwrap();
    macros
        .insert(&MacroDef {
            name: "Outer".into(),
            kinds: vec!["Filter".into()],
            params: Params::default(),
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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

/// A body invoking a nested macro can forward its OWN `Param`s into that
/// invocation's arguments (eager pre-substitution in `read_args`, before
/// this task frameless re-reads rejected `Up(Param(0))` with "outside any
/// macro expansion"): `PumpUp`'s body is `Pair(Up(Param(0)), Up(Param(1)))`,
/// and `Pair` reads those forwarded, already-resolved arguments as its own.
#[test]
fn body_forwards_params_into_nested_macro() {
    let mut set = macros();
    set.insert(&def(r#"(
        name: "Pair",
        kinds: [Modification],
        params: [NumericOp, NumericOp],
        body: Several([Power(Param(0)), Toughness(Param(1))]),
    )"#))
        .unwrap();
    set.insert(&def(r#"(
        name: "PumpUp",
        template: "gets +${0}/+${1}",
        kinds: [Modification],
        params: [Count, Count],
        body: Pair(Up(Param(0)), Up(Param(1))),
    )"#))
        .unwrap();
    let m: Modification = set.read_str("PumpUp(2, 3)").unwrap();
    assert_eq!(
        options().to_string(&m).unwrap(),
        "Several([Power(Up(2)),Toughness(Up(3))])"
    );
}

/// A misused top-level `Quote(...)` — legal only inside a meta-macro body —
/// must keep erroring exactly as it did before nested-macro param forwarding
/// was added: `forward_arg` only pre-substitutes a captured argument against
/// a *caller's* frame, and at the top level there is no frame, so the
/// argument (including any `Quote(...)` it contains) is untouched and the
/// invoked macro's own reader rejects the stray `Quote` as always. Before
/// the fix, `forward_arg` ran `collect_holes` unconditionally, whose
/// `Node::Quote` branch strips the wrapper regardless of frame ownership —
/// silently turning `Quote(Up(5))` into `Up(5)` here and letting the
/// misuse through instead of erroring.
#[test]
fn top_level_nested_macro_arg_with_stray_quote_still_errors() {
    let mut set = macros();
    set.insert(&def(r#"(
        name: "Pair",
        kinds: [Modification],
        params: [NumericOp, NumericOp],
        body: Several([Power(Param(0)), Toughness(Param(1))]),
    )"#))
        .unwrap();
    let err = set
        .read_str::<Modification>("Pair(Quote(Up(5)), Up(3))")
        .unwrap_err();
    assert!(
        err.to_string().contains("only legal in a meta-macro body"),
        "{err}"
    );
}

/// The named/struct-position branch of `read_args` (the second `forward_arg`
/// call site, alongside the positional branch `body_forwards_params_into_
/// nested_macro` above already covers) also forwards a caller's own `Param`
/// into a nested macro it invokes with struct-call syntax
/// (`Inner(op: Param(0))`).
#[test]
fn body_forwards_params_into_nested_named_macro() {
    let mut set = macros();
    set.insert(&def(r#"(
        name: "Inner",
        kinds: [Modification],
        params: { "op": NumericOp },
        body: Power(Param(op)),
    )"#))
        .unwrap();
    set.insert(&def(r#"(
        name: "Outer",
        template: "boosts by ${0}",
        kinds: [Modification],
        params: [NumericOp],
        body: Inner(op: Param(0)),
    )"#))
        .unwrap();
    let m: Modification = set.read_str("Outer(Up(5))").unwrap();
    assert_eq!(options().to_string(&m).unwrap(), "Power(Up(5))");
}

/// A body forwards its own `Param` as the whole, bare argument of a nested
/// positional macro invocation (`Boost(Param(0))`).
#[test]
fn body_forwards_whole_value_param_into_positional_macro() {
    let mut set = macros();
    set.insert(&def(r#"(
        name: "Boost",
        kinds: [Modification],
        params: [NumericOp],
        body: Power(Param(0)),
    )"#))
        .unwrap();
    set.insert(&def(r#"(
        name: "Relay",
        kinds: [Modification],
        params: [NumericOp],
        body: Boost(Param(0)),
    )"#))
        .unwrap();
    let m: Modification = set.read_str("Relay(Up(5))").unwrap();
    assert_eq!(options().to_string(&m).unwrap(), "Power(Up(5))");
}

/// A list-typed whole-value parameter reaches a nested sequence position.
/// The `deserialize_seq` path must resolve a hole forwarded from the caller's
/// frame.
#[test]
fn body_forwards_list_typed_whole_value_param_into_seq_position() {
    let mut param_types = ParamTypeSet::default();
    param_types.add_typed::<u32>("Count");
    param_types.add_typed::<NumericOp>("NumericOp");
    param_types.add_typed::<Vec<Modification>>("Mods");
    let mut set = MacroSet::new(kinds())
        .with_options(options())
        .with_param_types(param_types);
    set.insert(&def(r#"(
        name: "WrapMods",
        kinds: [Modification],
        params: [Mods],
        body: Several(Param(0)),
    )"#))
        .unwrap();
    set.insert(&def(r#"(
        name: "RelayMods",
        kinds: [Modification],
        params: [Mods],
        body: WrapMods(Param(0)),
    )"#))
        .unwrap();
    let m: Modification = set.read_str("RelayMods([Power(Up(1))])").unwrap();
    assert_eq!(options().to_string(&m).unwrap(), "Several([Power(Up(1))])");
}

#[test]
fn effect_positions_expand_macros() {
    let mut macros = empty();
    macros
        .insert(&MacroDef {
            name: "Investigate".into(),
            kinds: vec!["Effect".into()],
            params: Params::default(),
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "Static(effects: [CantAttack])".into(),
        })
        .unwrap();
    macros
        .insert(&MacroDef {
            name: "Outer".into(),
            kinds: vec!["Ability".into()],
            params: Params::default(),
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "Static(effects: [CantAttack])".into(),
        })
        .unwrap();
    macros
        .insert(&MacroDef {
            name: "OfType".into(),
            kinds: vec!["Filter".into()],
            params: Params::Positional(vec![ParamType::plain("Any")]),
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
        let err = macros.read_str::<CardFace>(call).unwrap_err();
        assert!(
            err.to_string().contains("cost") || err.to_string().contains("power"),
            "{call} should fail with a param-shape parse error, got: {err}"
        );
    }
}

#[test]
fn named_parameters_may_invoke_positionally_in_declaration_order() {
    let mut macros = empty().reading_positional_arguments();
    macros
        .insert(&def(r#"(
            name: "hasType",
            kinds: [Filter],
            params: { "type": Any },
            body: Type(Param(type)),
        )"#))
        .unwrap();

    let filter: Filter = macros.read_str("hasType(Creature)").unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(*expanded.value, Filter::Type(Type::Creature));
    assert_eq!(
        options().to_string(&Filter::Expanded(expanded)).unwrap(),
        "hasType(Creature)"
    );
}

#[test]
fn named_parameter_declaration_order_maps_a_positional_call() {
    let mut macros = empty().reading_positional_arguments();
    macros
        .insert(&def(r#"(
            name: "ordered",
            kinds: [Filter],
            params: { "lastAlphabetically": Any, "firstAlphabetically": Any },
            body: AllOf([Param(lastAlphabetically), Param(firstAlphabetically)]),
        )"#))
        .unwrap();

    let filter: Filter = macros
        .read_str(r#"ordered(Type(Creature), Named("Bear"))"#)
        .unwrap();
    let Filter::Expanded(expanded) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(
        *expanded.value,
        Filter::AllOf(vec![
            Filter::Type(Type::Creature),
            Filter::Named("Bear".into())
        ])
    );
}

#[test]
fn named_macro_arguments_cannot_mix_named_and_positional_forms() {
    let mut macros = empty().reading_positional_arguments();
    macros
        .insert(&def(r#"(
            name: "ordered",
            kinds: [Filter],
            params: { "left": Any, "right": Any },
            body: AllOf([Param(left), Param(right)]),
        )"#))
        .unwrap();

    for source in [
        "ordered(Type(Creature), right: Any)",
        "ordered(left: Type(Creature), Any)",
    ] {
        macros
            .read_str::<Filter>(source)
            .expect_err("mixed argument forms must be refused");
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "Param(0)".into(),
        })
        .unwrap();
    // A bare variant, a compound value, and a nested macro-free value all pass.
    for (call, expected) in [
        ("Echo(Any)", Filter::Any),
        ("Echo(Type(Land))", Filter::Type(Type::Land)),
        ("Echo(OneOf([Any]))", Filter::OneOf(vec![Filter::Any])),
    ] {
        let parsed: Filter = macros.read_str(call).unwrap();
        let Filter::Expanded(expanded) = parsed else {
            panic!("expected Echo expansion for {call}, got {parsed:?}");
        };
        assert_eq!(expanded.name, "Echo", "macro name should survive expansion");
        assert_eq!(
            *expanded.value, expected,
            "{call} should expand to {expected:?}"
        );
    }
}

#[test]
fn injected_param_types_validate() {
    // The embedder's path: register a domain validator, then it enforces.
    let mut param_types = ParamTypeSet::default();
    param_types.add("Number", |src, macros, _restricted| {
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "DrawCards(Param(0))".into(),
        })
        .unwrap();
    // A number is accepted.
    let effect = macros.read_str::<Effect>("Repeat(2)").unwrap();
    let Effect::Expanded(expanded) = effect else {
        panic!("expected a remembered effect, got {effect:?}");
    };
    assert_eq!(*expanded.value, Effect::DrawCards(Quantity::Literal(2)));
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

/// `template:` metadata is captured in `MacroDef` when present, and `None`
/// when absent.
#[test]
fn macro_def_captures_template() {
    let with: MacroDef =
        def(r#"(name: "Bears", kinds: [Filter], template: "bears", body: Type(Creature))"#);
    assert_eq!(with.template(), Some("bears"));

    let without: MacroDef = def(r#"(name: "Bears", kinds: [Filter], body: Type(Creature))"#);
    assert_eq!(without.template(), None);
}

/// `plural:` metadata is captured in `MacroDef` when present, and `None`
/// when absent — backward-compatible with every existing definition file,
/// which carries no `plural:` field at all.
#[test]
fn plural_field_is_optional() {
    let d = def(
        r#"( name: "Merfolk", template: "Merfolk", plural: "Merfolk", kinds: [Subtype], body: Subtype(name: "Merfolk", types: []) )"#,
    );
    assert_eq!(d.plural(), Some("Merfolk"));

    let d2 = def(
        r#"( name: "Goblin", template: "goblin", kinds: [Subtype], body: Subtype(name: "Goblin", types: []) )"#,
    );
    assert_eq!(d2.plural(), None);
}

/// `frames:` defaults to empty, so every macro file predating the field
/// still loads unchanged.
#[test]
fn frames_field_defaults_to_empty() {
    let d = def(r#"(name: "Bears", kinds: [Filter], body: Type(Creature))"#);
    assert_eq!(d.frames(), &[]);
}

/// A bare string in `frames:` is sugar for an unguarded `FrameSpec` — the
/// hole sigils in the text (`<Param(1)>`) pass through untouched.
#[test]
fn frames_field_bare_string_is_unguarded() {
    let d = def(r#"(name: "Draw", kinds: [OneShotEffect], params: [Count],
            frames: ["draw <Param(1)> cards"], body: Batch(Param(0), By(You, DrawCard)))"#);
    assert_eq!(
        d.frames(),
        &[crate::frames::FrameSpec::bare("draw <Param(1)> cards")]
    );
}

/// The full struct form of a frame carries its guard: `when` param
/// pre-bindings and the optional `position` key.
#[test]
fn frames_field_full_form_carries_guard() {
    let d = def(r#"(name: "Draw", kinds: [OneShotEffect], params: [Count],
            frames: [(text: "draw <Param(1)> cards", when: [(0, "You")], position: Main)],
            metadata: (),
            body: Batch(Param(0), By(You, DrawCard)))"#);
    let frame = &d.frames()[0];
    assert_eq!(frame.text, "draw <Param(1)> cards");
    assert_eq!(frame.when, vec![(0, "You".to_string())]);
    assert_eq!(frame.position, Some(crate::frames::FramePosition::Main));
}

/// Both spellings coexist in one `frames:` list, and both round-trip.
#[test]
fn frames_field_mixes_bare_and_full_spellings() {
    let d = def(r#"(name: "Draw", kinds: [OneShotEffect], params: [Count],
            frames: [
                "draw <Param(1)> cards",
                (text: "<Param(0)> draws <Param(1)> cards", when: [(0, "You")]),
            ],
            body: Batch(Param(0), By(You, DrawCard)))"#);
    assert_eq!(d.frames().len(), 2);
    assert_eq!(
        d.frames()[0],
        crate::frames::FrameSpec::bare("draw <Param(1)> cards")
    );
    assert_eq!(d.frames()[1].when, vec![(0, "You".to_string())]);
    assert_eq!(d.frames()[1].position, None);

    let round_tripped = options().to_string(&d.frames()[0]).unwrap();
    let back: crate::frames::FrameSpec = options().from_str(&round_tripped).unwrap();
    assert_eq!(back, d.frames()[0]);
}

/// `synthesize_expanded` emits `template: Some("…")` when a template is given,
/// and emits no template field when absent — byte-identical to the pre-template
/// form.
#[test]
fn synthesize_expanded_emits_template_when_present() {
    use crate::expand::FrameArgs;
    use crate::expand::synthesize_expanded;

    let no_args = FrameArgs::Positional(vec![]);

    // With template: the output contains `, template: Some("…")`.
    let with_tmpl = synthesize_expanded("M".into(), &no_args, Some("any target"), "Body");
    assert!(
        with_tmpl.contains(r#"template: Some("any target")"#),
        "expected template field in: {with_tmpl}"
    );
    assert!(
        with_tmpl.starts_with(r#"Expanded(name: "M", template:"#),
        "{with_tmpl}"
    );

    // Without template: byte-identical to the pre-template form.
    let no_tmpl = synthesize_expanded("M".into(), &no_args, None, "Body");
    assert_eq!(no_tmpl, r#"Expanded(name: "M", value: Body)"#);
}

/// When a macro with a `template:` field is expanded at a remembering kind,
/// the resulting `Expansion.template` carries the template text.
#[test]
fn template_is_carried_on_expansion() {
    let mut macros = empty();
    macros
        .insert(
            &options()
                .from_str::<MacroDef>(
                    r#"(
                    name: "Flying",
                    kinds: [Ability],
                    template: "flying",
                    body: Static(effects: [CantAttack]),
                )"#,
                )
                .unwrap(),
        )
        .unwrap();

    let ability: Ability = macros.read_str("Flying").unwrap();
    let Ability::Expanded(expanded) = ability else {
        panic!("expected a remembered ability, got {ability:?}");
    };
    assert_eq!(expanded.template.as_deref(), Some("flying"));

    // A macro WITHOUT a template leaves it as None.
    macros
        .insert(
            &options()
                .from_str::<MacroDef>(
                    r#"(
                    name: "Reach",
                    kinds: [Ability],
                    body: Static(effects: [CantAttack]),
                )"#,
                )
                .unwrap(),
        )
        .unwrap();
    let ability2: Ability = macros.read_str("Reach").unwrap();
    let Ability::Expanded(exp2) = ability2 else {
        panic!("expected a remembered ability, got {ability2:?}");
    };
    assert_eq!(exp2.template, None);
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
/// decide whether to fall through to the embedded type. Faithful to the derive
/// (`generate.rs`'s `concat_lists`): own names plus flattened compartments',
/// NEVER the embed payload's, which is exactly what leaves an inherited
/// identifier to `visit_newtype_struct`.
const EMBED_HOST_VARIANTS: &[&str] = &["Own", "Wrapped", "Expanded"];

/// The embedded type's dispatch set. A real kind always carries one (the
/// derive supplies it); the restricted-read ban and the cycle check both
/// consult it, so the fixture carries one too.
const EMBED_REF_VARIANTS: &[&str] = &["Bare", "Counted", "Expanded"];

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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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

/// A macro def of `name` at `kind` whose body is `body` — the shape an
/// identity macro takes (name = variant, body = the variant spelling).
fn identity_def(kind: &str, name: &str, body: &str) -> MacroDef {
    MacroDef {
        name: name.into(),
        kinds: vec![kind.into()],
        params: Params::default(),
        template: None,
        plural: None,
        frames: Vec::new(),
        metadata: (),
        body: body.into(),
    }
}

/// A name the host only inherits from the type it embeds still reaches that
/// type's identity macro under a RESTRICTED read — how a bare `Green` reads at
/// a `ManaSymbol` position in a card. Such a name has no macro at the host and
/// must not have one: a host-registered macro whose body spells the same name
/// re-enters the same kind and recurses (`authoring::embed_safe`).
///
/// This route does NOT depend on the pre-scan's suppression, and the fixture is
/// built so that stays visible: `EMBED_HOST_VARIANTS` excludes the embed
/// payload's names exactly as the derive does (`generate.rs`'s `concat_lists`
/// concatenates OWN plus FLATTEN tails, never the embed payload — that is what
/// makes `embeds_untagged` reachable at all). So `variants.contains("Bare")` is
/// already false and the `&&` short-circuits before restriction is consulted.
/// The free twin is `embed_host_bare_ref_variant_wraps`; what this adds is that
/// restriction does not disturb the route.
#[test]
fn restricted_embed_host_routes_an_inherited_name_to_the_embedded_macro() {
    let mut macros = empty();
    macros
        .insert(&identity_def("EmbedRef", "Bare", "Bare"))
        .unwrap();
    let host: EmbedHost = macros.read_str_restricted("Bare").unwrap();
    let EmbedHost::Wrapped(EmbedRef::Expanded(expanded)) = host else {
        panic!("expected Wrapped(Expanded(…)), got {host:?}");
    };
    assert_eq!(expanded.name, "Bare");
    assert_eq!(*expanded.value, EmbedRef::Bare);
}

/// The host's OWN variant still routes at the host: its identity macro is
/// registered there, so the pre-scan finds it before any fall-through.
#[test]
fn restricted_embed_host_routes_its_own_variant_to_its_own_macro() {
    let mut macros = empty();
    macros
        .insert(&identity_def("EmbedHost", "Own", "Own(7)"))
        .unwrap();
    let host: EmbedHost = macros.read_str_restricted("Own").unwrap();
    let EmbedHost::Expanded(expanded) = host else {
        panic!("expected EmbedHost::Expanded(…), got {host:?}");
    };
    assert_eq!(*expanded.value, EmbedHost::Own(7));
}

/// **The discriminating case for the pre-scan consult, and the whole of what
/// suppressing it changes.** Only the host's OWN (and flatten-inherited) names
/// reach `variants.contains` here — see the fixture above — so this is the one
/// route restriction redirects: `Own` loses host candidacy, falls through to
/// the embedded type, and the error is reported THERE ("not an `EmbedRef`")
/// rather than as the host-side "not author vocabulary".
///
/// That is a WORSE message, and it is the accepted cost, pinned so it stays a
/// decision. Leaving `variants.contains` unsuppressed would not open a hole —
/// `EnumIntercept` is installed unconditionally, so the re-read would still hit
/// its own suppressed consult and still refuse the spelling, with the better
/// host-side message. The reason to suppress here anyway is that spec §4's
/// container NOTE is normative: suppression must cover BOTH native-candidacy
/// consults. One rule at one place beats two consults with two policies.
#[test]
fn restricted_embed_host_variant_with_no_macro_anywhere_errors_embed_side() {
    let err = empty()
        .read_str_restricted::<EmbedHost>("Own(7)")
        .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("EmbedRef"), "{msg}");
    assert!(!msg.contains("not author vocabulary"), "{msg}");
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
    use crate::NamedParam;
    use crate::ParamDefault;
    use crate::ParamType;
    use crate::Params;
    use crate::SupportsMacros;
    use crate::VariantSignature;

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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "Twice(Param(0))".into(),
        })
        .unwrap();
        set
    }

    /// The generated `Serialize`/`Deserialize` round-trip the non-literal
    /// variants through plain ron. A `literal` variant writes bare (`2`, not
    /// `Lit(2)`), so reading it back needs the kind-aware reader that splices
    /// the bare numeral — exercised via the 2-tuple case below.
    #[test]
    fn p1_round_trips() {
        // unit variant: "X" ↔ Amount::X
        let unit_text = super::options().to_string(&Amount::X).unwrap();
        assert_eq!(unit_text, "X");
        let unit_back: Amount = super::options().from_str(&unit_text).unwrap();
        assert_eq!(unit_back, Amount::X);

        // newtype-with-Box variant: "Twice(X)" ↔
        // Amount::Twice(Box::new(Amount::X))
        let newtype = Amount::Twice(Box::new(Amount::X));
        let newtype_text = super::options().to_string(&newtype).unwrap();
        assert_eq!(newtype_text, "Twice(X)");
        let newtype_back: Amount = super::options().from_str(&newtype_text).unwrap();
        assert_eq!(newtype_back, newtype);

        // 2-tuple variant carrying a literal: it writes bare (`Per("land", 2)`)
        // and reads back through the kind-aware reader (bare-numeral splice).
        let amount = Amount::Per("land".to_owned(), Box::new(Amount::Lit(2)));
        let text = super::options().to_string(&amount).unwrap();
        assert_eq!(text, r#"Per("land",2)"#, "literal writes bare");
        let back: Amount = amount_set().read_str(&text).unwrap();
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

    /// Embed fixtures: `Pick::Ref` (newtype, name-erased) and `Deed::By`,
    /// mirroring core's `Action::By` (tuple, defaulted head).
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
        let error = super::options().from_str::<Pick>("Ref(Them)").unwrap_err();
        let msg = error.to_string();
        assert!(msg.contains("Ref"), "unexpected error: {msg}");
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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

    /// Exclude fixtures, mirroring `Destination::Zone` with
    /// `exclude(Library, Stack)`: `Spot` is the flattened payload, `Nook`
    /// narrows the compartment so `Cellar` and `Vault` do not lift. `Nook`
    /// reclaims `Cellar` with its OWN variant (as `Destination::Library`
    /// reclaims `Library` for the anchored form); `Vault` is excluded with no
    /// reclaim (as `Stack` is never a destination).
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Spot {
        Attic,
        Cellar,
        Vault,
    }

    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Nook {
        #[macro_ron(flatten, exclude(Cellar, Vault))]
        Spot(Spot),
        Cellar(u32),
    }

    /// `exclude(...)` narrows a `flatten`: excluded payload names do not lift
    /// (a reclaimed one routes to the parent's OWN variant; an unreclaimed one
    /// is rejected on read), while un-excluded names still flatten and
    /// round-trip.
    #[test]
    fn flatten_exclude_narrows_the_compartment() {
        // An un-excluded payload name still lifts and round-trips.
        let read: Nook = super::options().from_str("Attic").unwrap();
        assert_eq!(read, Nook::Spot(Spot::Attic));
        assert_eq!(super::options().to_string(&read).unwrap(), "Attic");

        // A reclaimed name routes to the parent's OWN variant, not the payload.
        let read: Nook = super::options().from_str("Cellar(7)").unwrap();
        assert_eq!(read, Nook::Cellar(7));

        // An excluded, unreclaimed name is rejected (mirrors bare `Stack`).
        let error = super::options().from_str::<Nook>("Vault").unwrap_err();
        assert!(
            error.to_string().contains("Vault"),
            "unexpected error: {error}"
        );

        // ALL_VARIANTS: OWN (["Cellar"]) ++ Spot's set minus the excluded
        // names — so "Vault" is absent and "Cellar" appears once (the OWN one).
        assert_eq!(<Nook as SupportsMacros>::ALL_VARIANTS, &["Cellar", "Attic"]);
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
            body: "Me".into(),
        })
        .unwrap();

        // "Us" is not in Step's native list and Step doesn't embed, so the
        // macro layer tries Step macros, finds none, and errors.
        let error = set.read_str::<Step>("Us").unwrap_err();
        assert!(
            error.to_string().contains("Us"),
            "unexpected error: {error}"
        );
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
            template: None,
            plural: None,
            frames: Vec::new(),
            metadata: (),
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

    /// Signature fixture: unit, newtype variants — enough to pin the shape
    /// mapping without restating the full `Filter` grammar. Distinct from
    /// (and unrelated to) the hand-built `super::Filter`: that one has no
    /// `SupportsMacros` impl to read `ALL_SIGNATURES` from, so a real
    /// derive-backed fixture is needed here, reusing the name for parity.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Filter {
        Any,
        Named(String),
        Power(u32),
    }

    /// The derive exposes each variant's shape, not just its name —
    /// scaffolding an identity macro needs the arity to mirror.
    #[test]
    #[allow(
        clippy::map_unwrap_or,
        reason = "verbatim task-brief test code: map().unwrap_or_else() reads as the intended shape here"
    )]
    fn the_derive_exposes_variant_signatures() {
        let sigs = Filter::ALL_SIGNATURES;
        let get = |name: &str| {
            sigs.iter()
                .find(|(n, _)| *n == name)
                .map(|(_, s)| *s)
                .unwrap_or_else(|| panic!("no signature for `{name}`"))
        };
        assert_eq!(get("Any"), VariantSignature::Unit);
        assert_eq!(
            get("Named"),
            VariantSignature::Positional(&[ParamDefault::Required])
        );
        assert_eq!(
            get("Power"),
            VariantSignature::Positional(&[ParamDefault::Required])
        );
    }

    /// A host with its own variant plus an embed, mirroring `Pick` above.
    /// Distinct from (and unrelated to) the hand-built `super::EmbedHost` used
    /// for the `embeds_untagged` runtime tests: this one is derive-backed, so
    /// `ALL_SIGNATURES`/`ALL_VARIANTS` are real associated consts, not a
    /// hand-maintained list.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum EmbedHost {
        Own(u32),
        #[macro_ron(embed)]
        Ref(Who),
        #[macro_ron(expanded)]
        Expanded(Expansion<EmbedHost>),
    }

    /// Signatures concatenate transitively, exactly like `ALL_VARIANTS`: a
    /// host that embeds another type must be able to look up the embedded
    /// type's variants too, or every inherited row is unscaffoldable.
    #[test]
    fn signatures_are_transitive_like_the_dispatch_set() {
        for (name, _) in EmbedHost::ALL_SIGNATURES {
            assert!(EmbedHost::ALL_VARIANTS.contains(name));
        }
        for name in EmbedHost::ALL_VARIANTS {
            assert!(
                EmbedHost::ALL_SIGNATURES.iter().any(|(n, _)| n == name),
                "`{name}` dispatches at EmbedHost but has no signature"
            );
        }
    }

    /// Tuple-variant fixture mirroring `Action::Move`'s real shape (see
    /// `deckmaste_core::action::Action::Move`): two required fields, then two
    /// trailing `#[macro_ron(default = ...)]` fields.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Move {
        Go(
            u32,
            u32,
            #[macro_ron(default = "0")] u32,
            #[macro_ron(default = "None")] Option<u32>,
        ),
    }

    /// A tuple variant's trailing defaults must be visible in its signature —
    /// arity alone (the brief's original `Positional(usize)` recipe) can't
    /// tell a scaffold which positions are optional or what to fill them
    /// with.
    #[test]
    fn tuple_signature_reports_trailing_defaults() {
        let sigs = Move::ALL_SIGNATURES;
        let (_, sig) = *sigs.iter().find(|(n, _)| *n == "Go").unwrap();
        let VariantSignature::Positional(params) = sig else {
            panic!("expected a Positional signature for `Go`");
        };
        assert_eq!(
            params.to_vec(),
            vec![
                ParamDefault::Required,
                ParamDefault::Required,
                ParamDefault::Expr("0"),
                ParamDefault::Expr("None"),
            ]
        );
    }

    /// Struct-variant fixture: one field forwards `#[serde(default)]`, one
    /// doesn't — and a second variant carries no defaults at all.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Note {
        Say {
            text: String,
            #[serde(default)]
            loud: bool,
        },
        Ask {
            text: String,
            urgency: u32,
        },
    }

    /// A struct variant reports `Implicit` for a field forwarding
    /// `#[serde(default ...)]`, `Required` for one without.
    /// `#[macro_ron(default = ...)]` never applies here (rejected on struct
    /// variants by `input::validate`), so this is the only source of a
    /// struct field's default.
    #[test]
    fn struct_signature_reports_implicit_default() {
        let sigs = Note::ALL_SIGNATURES;
        let get_named = |variant: &str| {
            let (_, sig) = *sigs.iter().find(|(n, _)| *n == variant).unwrap();
            let VariantSignature::Named(params) = sig else {
                panic!("expected a Named signature for `{variant}`");
            };
            params
        };
        let field = |params: &[NamedParam], name: &str| {
            params.iter().find(|p| p.name == name).unwrap().default
        };
        let say = get_named("Say");
        assert_eq!(field(say, "text"), ParamDefault::Required);
        assert_eq!(field(say, "loud"), ParamDefault::Implicit);
    }

    /// A struct variant with no defaulted fields reports `Required`
    /// throughout.
    #[test]
    fn struct_signature_with_no_defaults_is_all_required() {
        let sigs = Note::ALL_SIGNATURES;
        let (_, sig) = *sigs.iter().find(|(n, _)| *n == "Ask").unwrap();
        let VariantSignature::Named(params) = sig else {
            panic!("expected a Named signature for `Ask`");
        };
        assert!(params.iter().all(|p| p.default == ParamDefault::Required));
    }

    /// Payload fixture for the `spliced` marker: a named struct whose fields
    /// a newtype variant lifts into its own call. `when` carries no
    /// `#[serde(default)]` — serde fills a missing `Option` field anyway, so
    /// it must still report droppable. `r#as` pins the raw-identifier
    /// spelling (RON says `as`).
    #[derive(
        Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, Expand, crate::MacroFields,
    )]
    struct Wish {
        what: String,
        #[serde(default)]
        urgent: bool,
        when: Option<u32>,
        r#as: String,
    }

    /// A newtype-over-named-struct variant, boxed the way the real grammar
    /// boxes its ability payloads.
    #[derive(Debug, Clone, PartialEq, crate::SupportsMacros)]
    enum Grant {
        Plain(u32),
        #[macro_ron(spliced)]
        Wish(std::sync::Arc<Wish>),
    }

    /// The payload struct's own field list is what `MacroFields` exposes —
    /// names in declaration order, droppability from the serde attribute or
    /// an `Option` type.
    #[test]
    fn macro_fields_reports_a_structs_own_fields() {
        use crate::MacroFields as _;

        assert_eq!(
            Wish::FIELDS.to_vec(),
            vec![
                NamedParam {
                    name: "what",
                    default: ParamDefault::Required
                },
                NamedParam {
                    name: "urgent",
                    default: ParamDefault::Implicit
                },
                NamedParam {
                    name: "when",
                    default: ParamDefault::Implicit
                },
                NamedParam {
                    name: "as",
                    default: ParamDefault::Required
                },
            ]
        );
    }

    /// A `spliced` newtype variant reports its PAYLOAD's fields as a `Named`
    /// signature — the shape canon actually spells (`Wish(what: …)`, never
    /// `Wish((what: …))`) — while an ordinary newtype stays one positional
    /// slot. Wrappers (`Arc`/`Box`/`Rc`) are seen through.
    #[test]
    fn a_spliced_newtype_variant_reports_its_payloads_fields() {
        use crate::MacroFields as _;

        let sigs = Grant::ALL_SIGNATURES;
        let get = |name: &str| sigs.iter().find(|(n, _)| *n == name).unwrap().1;
        assert_eq!(
            get("Plain"),
            VariantSignature::Positional(&[ParamDefault::Required])
        );
        assert_eq!(get("Wish"), VariantSignature::Named(Wish::FIELDS));
    }

    /// The marker changes the reported SHAPE only: reading and writing a
    /// `spliced` variant stays the ordinary flat newtype round trip.
    #[test]
    fn a_spliced_variant_still_reads_and_writes_flat() {
        let source = r#"Wish(what: "rain", as: "weather")"#;
        let value: Grant = super::options()
            .from_str(source)
            .expect("a spliced variant reads field-spliced");
        assert_eq!(
            value,
            Grant::Wish(std::sync::Arc::new(Wish {
                what: "rain".into(),
                urgent: false,
                when: None,
                r#as: "weather".into(),
            }))
        );
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
    assert_eq!(signature[0].0, "name");
    assert_eq!(signature[1].0, "template");
    let name = &signature[0].1;
    assert_eq!(name.name, "String");
    assert_eq!(name.default, None);
    let template = &signature[1].1;
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

/// A self-referential body — the minimal cycle — is rejected at insert
/// rather than only tripping the runtime `MAX_DEPTH` expansion cap.
#[test]
fn self_referential_body_is_rejected_at_insert() {
    let err = empty()
        .insert(&def(r#"(
            name: "Loop",
            kinds: [Modification],
            body: Loop,
        )"#))
        .unwrap_err();
    assert!(matches!(err, InsertError::Cycle { .. }), "{err}");
}

/// A two-macro cycle is rejected once it closes: `A`'s body naming `B`
/// (not yet defined) inserts fine — `B` simply isn't a macro yet, so the
/// walk finds no edge — but `B`'s body naming `A` closes the loop back to
/// `B` itself.
#[test]
fn mutually_recursive_bodies_are_rejected_at_insert() {
    let mut set = empty();
    set.insert(&def(r#"( name: "A", kinds: [Modification], body: B )"#))
        .unwrap();
    let err = set
        .insert(&def(r#"( name: "B", kinds: [Modification], body: A )"#))
        .unwrap_err();
    assert!(matches!(err, InsertError::Cycle { .. }), "{err}");
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
    let mut macros = empty().reading_positional_arguments();
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

    // Positional provenance has the same omission rule.
    let filter: Filter = macros.read_str("Sized(Creature)").unwrap();
    assert_eq!(options().to_string(&filter).unwrap(), "Sized(Creature)");
    let filter: Filter = macros.read_str("Sized(Creature, 3)").unwrap();
    assert_eq!(options().to_string(&filter).unwrap(), "Sized(Creature,3)");
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

/// All params defaulted: the bare name `Sized` reads as the zero-arg call,
/// expanding identically to the explicit `Sized()`, and still writes back as
/// `Sized()` (serialization stays parenthesized).
#[test]
fn bare_name_reads_as_all_defaulted_invocation() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Sized",
            kinds: [Filter],
            params: { "min": Default(Any, 1) },
            body: Power(min: Param(min)),
        )"#))
        .unwrap();
    let bare: Filter = macros.read_str("Sized").unwrap();
    let parens: Filter = macros.read_str("Sized()").unwrap();
    assert_eq!(bare, parens, "bare name expands like the empty-args form");
    // Serialization stays `Sized()`, which re-reads to the same value.
    let written = options().to_string(&bare).unwrap();
    assert_eq!(written, "Sized()");
    let reread: Filter = macros.read_str(&written).unwrap();
    assert_eq!(reread, bare);
}

/// A bare name is only a zero-arg call when *every* param is defaulted: a
/// named macro with a required param still errors when invoked bare.
#[test]
fn bare_name_with_required_param_still_errors() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Sized",
            kinds: [Filter],
            params: { "kind": Any, "min": Default(Any, 1) },
            body: AllOf([Type(Param(kind)), Power(min: Param(min))]),
        )"#))
        .unwrap();
    // Not all-defaulted, so the bare-invocation pre-scan doesn't fire and the
    // named-args grammar rejects the missing parentheses.
    let error = macros.read_str::<Filter>("Sized").unwrap_err();
    let msg = error.to_string();
    assert!(
        msg.contains("Expected opening `(`"),
        "unexpected error: {msg}"
    );
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

/// `add_typed::<T>` registers a param type whose validator parses the argument
/// as `T` through `read_str` — the single mechanical pattern every domain
/// param type shares, instead of a hand-written closure per type.
#[test]
fn add_typed_registers_a_deserialize_validator() {
    let mut set = ParamTypeSet::empty();
    set.add_typed::<u32>("Num");
    assert!(set.contains("Num"));
    let validate = set.get("Num").expect("Num is registered");
    let macros = empty();
    assert_eq!(validate("7", &macros, false), Ok(()));
    let as_value: u32 = macros.read_str("7").unwrap();
    assert_eq!(as_value, 7);
    let err = validate("nope", &macros, false).unwrap_err();
    let err = err.clone();
    assert!(
        err.contains("Expected integer"),
        "non-numeral parse failed for expected reason: {err}"
    );
}

/// [typed-holes delta 2] A parameter type may declare a BINDER CONTRACT
/// `Effect(binds: [It])` — the anaphora the body wraps around the hole. The
/// default is no contract (empty `binds`); `Default(T(binds: […]), expr)`
/// carries a contract on a defaulted param too.
#[test]
fn param_type_parses_a_binder_contract() {
    let d = def(r#"(
        name: "WithLoop",
        kinds: [Effect],
        params: [Effect(binds: [It]), Count],
        body: Each(binder: X, effect: Param(0)),
    )"#);
    let Params::Positional(types) = &d.params else {
        panic!("positional signature");
    };
    assert_eq!(types[0].name, "Effect");
    assert_eq!(types[0].binds, vec![Ident::from("It")]);
    assert!(types[0].default.is_none());
    // A plain param carries an empty contract by default.
    assert_eq!(types[1].name, "Count");
    assert!(types[1].binds.is_empty());

    // A contract with several granted anaphora, and a defaulted param that
    // still carries one.
    let d = def(r#"(
        name: "Bind2",
        kinds: [Effect],
        params: { "e": Effect(binds: [It, That]), "d": Default(Effect(binds: [It]), Draw(1)) },
        body: Param(e),
    )"#);
    let Params::Named(sig) = &d.params else {
        panic!("named signature");
    };
    assert_eq!(
        sig.iter()
            .find(|(name, _)| *name == "e")
            .map(|(_, ty)| &ty.binds)
            .unwrap(),
        &vec![Ident::from("It"), Ident::from("That")]
    );
    let defaulted = sig
        .iter()
        .find(|(name, _)| *name == "d")
        .map(|(_, ty)| ty)
        .unwrap();
    assert_eq!(defaulted.binds, vec![Ident::from("It")]);
    assert!(defaulted.default.is_some());
}

/// [typed-holes delta 5] `Splice(Param(i))` inlines a list-valued argument's
/// elements into the surrounding list — the one generic rule replacing the
/// old ad-hoc `Cost(Param(i))` / `Modification::Several` flatten idioms.
#[test]
fn splice_inlines_a_list_param_into_a_surrounding_list() {
    let mut set = empty();
    set.insert(&def(r#"(
        name: "WithCreature",
        kinds: [Filter],
        params: [Any],
        body: AllOf([Splice(Param(0)), Type(Creature)]),
    )"#))
        .unwrap();
    let filter: Filter = set
        .read_str(r#"WithCreature([Named("a"), Named("b")])"#)
        .unwrap();
    let Filter::Expanded(exp) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(
        *exp.value,
        Filter::AllOf(vec![
            Filter::Named("a".into()),
            Filter::Named("b".into()),
            Filter::Type(Type::Creature),
        ]),
    );
}

/// A `Splice(...)` at a scalar/enum position (not a list) is a load error
/// naming the misplacement — the delta-5 reject shape.
#[test]
fn splice_at_a_non_list_position_is_an_error() {
    let mut set = empty();
    set.insert(&def(r#"(
        name: "BadSplice",
        kinds: [Filter],
        params: [Any],
        body: Type(Splice(Param(0))),
    )"#))
        .unwrap();
    let err = set
        .read_str::<Filter>("BadSplice(Creature)")
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("Splice") && err.contains("list position"),
        "unexpected error: {err}"
    );
}

/// A `Splice` argument that doesn't resolve to a list is a load error — the
/// splice needs a `[...]` to inline.
#[test]
fn splice_of_a_non_list_argument_is_an_error() {
    let mut set = empty();
    set.insert(&def(r#"(
        name: "SpliceScalar",
        kinds: [Filter],
        params: [Any],
        body: AllOf([Splice(Param(0))]),
    )"#))
        .unwrap();
    let err = set
        .read_str::<Filter>("SpliceScalar(Creature)")
        .unwrap_err()
        .to_string();
    assert!(err.contains("not a list"), "unexpected error: {err}");
}

/// [typed-holes delta 4] `Quote(Param(i))` in a meta-macro body emits a
/// literal `Param(i)` into the PRODUCED definition (deferred to that
/// definition's own frame) rather than resolving eagerly against the meta's
/// frame — the stage marker for two-level meta-macros. Here the meta OWNS `x`
/// (used eagerly for the produced macro's name), and `Quote(Param(x))` defers
/// a SECOND `x` — the produced macro's own parameter.
#[test]
fn quote_defers_a_meta_owned_param_to_the_produced_definition() {
    let mut set = empty();
    set.insert(&def(r#"(
        name: "FilterNamed",
        kinds: [Macro],
        params: { "x": String },
        body: (
            name: Param(x),
            kinds: [Filter],
            params: { "x": Any },
            body: Quote(Param(x)),
        ),
    )"#))
        .unwrap();
    // The meta resolves its own `x` for the produced macro's NAME; the body's
    // `Quote(Param(x))` stays a literal `Param(x)` in the produced definition.
    let produced: MacroDef = set.read_str(r#"FilterNamed(x: "MyFilter")"#).unwrap();
    assert_eq!(produced.name, "MyFilter");
    assert_eq!(produced.body(), "Param(x)", "the deferred hole survives");
    set.insert(&produced).unwrap();
    // Invoking the produced macro resolves its OWN `x` — proof the hole was
    // deferred, not eagerly bound to the meta's `"MyFilter"` string (which is
    // no Filter).
    let filter: Filter = set.read_str("MyFilter(x: Type(Creature))").unwrap();
    let Filter::Expanded(exp) = filter else {
        panic!("expected a remembered filter, got {filter:?}");
    };
    assert_eq!(*exp.value, Filter::Type(Type::Creature));
}

/// Without `Quote`, a meta-owned hole resolves EAGERLY against the meta frame:
/// the contrast that shows the stage marker earns its keep.
#[test]
fn a_bare_meta_owned_param_resolves_eagerly() {
    let mut set = empty();
    set.insert(&def(r#"(
        name: "EagerNamed",
        kinds: [Macro],
        params: { "x": String },
        body: (
            name: Param(x),
            kinds: [Filter],
            body: Named(Param(x)),
        ),
    )"#))
        .unwrap();
    // `Param(x)` in the body is the meta's `x`, spliced now — so the produced
    // body carries the string literal, not a hole.
    let produced: MacroDef = set.read_str(r#"EagerNamed(x: "Zombie")"#).unwrap();
    assert_eq!(produced.name, "Zombie");
    assert!(
        produced.body().contains(r#""Zombie""#) && !produced.body().contains("Param"),
        "the meta-owned hole was spliced eagerly, not deferred: {:?}",
        produced.body()
    );
}

// ---------------------------------------------------------------------------
// Variant signatures (spec §5)
// ---------------------------------------------------------------------------

/// The registry carries signatures through, so a consumer with only a
/// `KindSet` can scaffold without reaching back to the Rust types.
#[test]
fn registered_kinds_carry_their_signatures() {
    let kinds = kinds();
    let filter = kinds.get("Filter").expect("registered fixture kind");
    assert!(!filter.signatures().is_empty());
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

// ---------------------------------------------------------------------------
// Restricted author vocabulary (spec §4)
//
// `Filter` is a registered kind, so its variant idents lose native candidacy
// under restriction. `Type` (Land/Creature) is a plain enum outside the
// registry, standing in for the closed atoms the spec keeps native — `Cmp`,
// the phase/step enums, `FaceLayout`.

/// The ban is candidacy suppression at a registered kind, and a variant with
/// no macro of its name says so in those terms — a different mistake from a
/// misspelling, so a different message.
#[test]
fn restricted_read_rejects_a_variant_with_no_identity_macro() {
    let err = empty().read_str_restricted::<Filter>("Any").unwrap_err();
    assert!(err.to_string().contains("not author vocabulary"), "{err}");
}

/// Restriction is opt-in at the entry: the ordinary reader is untouched, which
/// is what keeps the other ~230 `read_str` callers out of this program.
#[test]
fn unrestricted_reads_are_unaffected_by_the_ban() {
    assert_eq!(empty().read_str::<Filter>("Any").unwrap(), Filter::Any);
}

/// An identity macro restores the spelling — invocation syntax equals variant
/// syntax, which is what makes the ban near-zero-churn for canon. At a
/// remembering kind the value arrives wrapped in invocation provenance (§5),
/// and the macro's own body spells the native variant freely because a
/// definition body is free vocabulary.
#[test]
fn an_identity_macro_covers_its_variant_and_its_body_stays_free() {
    let mut macros = empty();
    macros
        .insert(&def(
            r#"(name: "Any", kinds: [Filter], params: [], body: Any)"#,
        ))
        .unwrap();
    let value = macros.read_str_restricted::<Filter>("Any").unwrap();
    assert!(matches!(value, Filter::Expanded(_)), "{value:?}");
}

/// The crux of "provenance, not frame": an argument written in restricted
/// source stays restricted after substitution into a free body, so the body
/// cannot launder a banned spelling on the author's behalf.
#[test]
fn an_argument_keeps_its_restriction_inside_a_free_body() {
    let mut macros = empty();
    macros
        .insert(&def(
            r#"(name: "Wrap", kinds: [Filter], params: [Any], body: OneOf([Param(0)]))"#,
        ))
        .unwrap();
    let err = macros
        .read_str_restricted::<Filter>("Wrap(Any)")
        .unwrap_err();
    assert!(err.to_string().contains("not author vocabulary"), "{err}");
    macros.read_str::<Filter>("Wrap(Any)").unwrap();
}

/// The other half of provenance: a filled-in default is the definition's own
/// text, so it is free however the invocation was written.
#[test]
fn a_filled_in_default_is_free_vocabulary() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(name: "Defaulted", kinds: [Filter],
                params: { "f": Default(Any, Any) }, body: Param(f))"#))
        .unwrap();
    let value = macros.read_str_restricted::<Filter>("Defaulted()").unwrap();
    assert!(matches!(value, Filter::Expanded(_)), "{value:?}");
}

/// Suppression applies at registered kinds only: `Type` is outside the
/// registry, so `Creature` parses natively even as restricted argument text.
#[test]
fn unregistered_kinds_keep_their_native_variants() {
    let mut macros = empty();
    macros
        .insert(&def(
            r#"(name: "Type", kinds: [Filter], params: [Any], body: Type(Param(0)))"#,
        ))
        .unwrap();
    let value = macros
        .read_str_restricted::<Filter>("Type(Creature)")
        .unwrap();
    assert!(matches!(value, Filter::Expanded(_)), "{value:?}");
}

/// One hop further than
/// [`an_argument_keeps_its_restriction_inside_a_free_body`]: the body forwards
/// the card's argument into a NESTED macro invocation, so it travels as
/// `read_args` argument text rather than as a resolved hole. It is
/// still the author's text, so it stays restricted through the second frame.
/// `Any`'s validator accepts anything, so nothing else can catch it here — the
/// declared-param-type check is a second line, not this one.
#[test]
fn a_forwarded_argument_stays_restricted_in_a_nested_invocation() {
    let mut macros = empty();
    macros
        .insert(&def(
            r#"(name: "Wrap", kinds: [Filter], params: [Any], body: OneOf([Param(0)]))"#,
        ))
        .unwrap();
    macros
        .insert(&def(
            r#"(name: "Outer", kinds: [Filter], params: [Any], body: Wrap(Param(0)))"#,
        ))
        .unwrap();
    let err = macros
        .read_str_restricted::<Filter>("Outer(Any)")
        .unwrap_err();
    assert!(err.to_string().contains("not author vocabulary"), "{err}");
    macros.read_str::<Filter>("Outer(Any)").unwrap();
}

/// A default expression that splices the invocation's own argument carries
/// that argument's restriction: `defaulted` exempts the definition's text, not
/// the author's text spliced into it.
#[test]
fn a_default_that_splices_an_argument_keeps_its_restriction() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(name: "Echo", kinds: [Filter],
                params: { "f": Any, "g": Default(Any, Param(f)) }, body: Param(g))"#))
        .unwrap();
    let err = macros
        .read_str_restricted::<Filter>("Echo(f: Any)")
        .unwrap_err();
    assert!(err.to_string().contains("not author vocabulary"), "{err}");
    macros.read_str::<Filter>("Echo(f: Any)").unwrap();
}

/// The bare-numeral splice is text the READER invented, so by the same
/// provenance rule it is free: a bare `3` keeps reading as the native
/// `Literal(3)` under restriction rather than routing through the identity
/// macro and changing what round-trips.
#[test]
fn a_bare_numeral_splice_is_reader_text_and_reads_free() {
    assert_eq!(
        empty().read_str_restricted::<Quantity>("3").unwrap(),
        Quantity::Literal(3)
    );
    let mut macros = empty();
    macros
        .insert(&def(
            r#"(name: "Literal", kinds: [Quantity], params: [Count], body: Literal(Param(0)))"#,
        ))
        .unwrap();
    assert_eq!(
        macros.read_str_restricted::<Quantity>("3").unwrap(),
        Quantity::Literal(3)
    );
    // An author who spells the wrapper out wrote it, so it still routes
    // through the identity macro.
    let spelled = macros
        .read_str_restricted::<Quantity>("Literal(3)")
        .unwrap();
    assert!(matches!(spelled, Quantity::Expanded(_)), "{spelled:?}");
}

// ---------------------------------------------------------------------------
// Optional-parameter elision (`Elidable(Type)`)
//
// The two fixture shapes the capability exists for: a struct variant whose
// fields carry serde defaults (`EventFilter::Cast`), and a tuple variant with
// a trailing default (`Action::Cast`). Both are spelled BOTH ways in the card
// corpus — long, supplying the value the default would have filled, and
// short, omitting it — so one identity macro has to read both.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
enum Who {
    #[default]
    Anyone,
    You,
}

/// A struct variant with serde-defaulted fields, like `EventFilter::Cast`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum Event {
    Cast {
        #[serde(default)]
        who: Who,
        what: String,
        #[serde(default)]
        cause: Who,
    },
    /// Every field defaulted, so a call can omit them ALL — the shape most
    /// of `EventFilter`'s event variants actually have.
    Idle {
        #[serde(default)]
        who: Who,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
enum Loudness {
    #[default]
    Normal,
    Loud,
}

/// A tuple variant with a trailing default: hand-written because serde's
/// derive has no per-field default for tuple variants, while the
/// `SupportsMacros` derive generates exactly this shape for
/// `#[macro_ron(default = …)]` (`Action::Cast`'s trailing argument).
#[derive(Debug, Clone, PartialEq, Eq)]
enum Shout {
    Say(String, Loudness),
}

impl<'de> Deserialize<'de> for Shout {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::fmt;

        use serde::de::SeqAccess;
        use serde::de::Visitor;

        struct ShoutVisitor;
        impl<'de> Visitor<'de> for ShoutVisitor {
            type Value = Shout;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a Shout value")
            }
            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                use serde::de::Error;
                let (ident, variant) = data.variant_seed(IdentSeed)?;
                if ident != "Say" {
                    return Err(A::Error::custom(format_args!("`{ident}` is not a Shout")));
                }
                variant.tuple_variant(2, SayArgs)
            }
        }

        struct SayArgs;
        impl<'de> Visitor<'de> for SayArgs {
            type Value = Shout;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("Say(word, loudness?)")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                use serde::de::Error;
                let word: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("Say needs a word"))?;
                let loudness = seq.next_element()?.unwrap_or_default();
                Ok(Shout::Say(word, loudness))
            }
        }

        deserializer.deserialize_enum("Shout", &["Say"], ShoutVisitor)
    }
}

/// `Event`'s own variant names: an identity macro is only reachable when the
/// kind knows which idents are native (the cycle check, and the restricted
/// read's candidacy suppression, both consult this).
const EVENT_VARIANTS: &[&str] = &["Cast", "Idle"];
const SHOUT_VARIANTS: &[&str] = &["Say"];

/// The named identity macro under test: `who` and `cause` are elidable, `what`
/// is required — `EventFilter::Cast`'s real shape.
fn cast_event_macro() -> MacroDef {
    def(r#"(
        name: "Cast",
        kinds: [Event],
        params: { "who": Elidable(Any), "what": Any, "cause": Elidable(Any) },
        body: Cast(who: Param(who), what: Param(what), cause: Param(cause)),
    )"#)
}

/// A non-identity macro over the same shape, so elision is exercised on the
/// ordinary (unrestricted) path too, where an identity macro's name would be
/// shadowed by the native variant.
fn ignite_macro() -> MacroDef {
    def(r#"(
        name: "Ignite",
        kinds: [Event],
        params: { "who": Elidable(Any), "what": Any },
        body: Cast(who: Param(who), what: Param(what)),
    )"#)
}

/// The positional identity macro under test: a trailing elidable argument,
/// `Action::Cast`'s real shape.
fn say_shout_macro() -> MacroDef {
    def(r#"(
        name: "Say",
        kinds: [Shout],
        params: [Any, Elidable(Any)],
        body: Say(Param(0), Param(1)),
    )"#)
}

fn elision_set() -> MacroSet {
    let mut kinds = kinds();
    kinds.add(Kind::new("Event").with_variants(EVENT_VARIANTS));
    kinds.add(Kind::new("Shout").with_variants(SHOUT_VARIANTS));
    let mut macros = MacroSet::new(kinds).with_options(options());
    macros.insert(&cast_event_macro()).unwrap();
    macros.insert(&ignite_macro()).unwrap();
    macros.insert(&say_shout_macro()).unwrap();
    macros
}

/// A parameter the caller supplied is forwarded into the body.
#[test]
fn an_elided_param_is_forwarded_when_supplied() {
    let value: Event = elision_set()
        .read_str_restricted(r#"Cast(who: You, what: "bolt", cause: You)"#)
        .unwrap();
    assert_eq!(
        value,
        Event::Cast {
            who: Who::You,
            what: "bolt".into(),
            cause: Who::You,
        }
    );
}

/// The same parameter, omitted, leaves the body key out entirely — so the host
/// type's own serde default applies, exactly as it does when a card file omits
/// the field.
#[test]
fn an_elided_param_is_omitted_from_the_body_when_absent() {
    let short = r#"Cast(what: "bolt")"#;
    let value: Event = elision_set().read_str_restricted(short).unwrap();
    let native: Event = options().from_str(short).unwrap();
    assert_eq!(value, native);
    assert_eq!(
        value,
        Event::Cast {
            who: Who::Anyone,
            what: "bolt".into(),
            cause: Who::Anyone,
        }
    );
}

/// Only the omitted parameters vanish: a partially-supplied call keeps the
/// ones it gave.
#[test]
fn an_elided_param_omission_is_per_argument() {
    let source = r#"Cast(what: "bolt", cause: You)"#;
    let value: Event = elision_set().read_str_restricted(source).unwrap();
    let native: Event = options().from_str(source).unwrap();
    assert_eq!(value, native);
    assert_eq!(
        value,
        Event::Cast {
            who: Who::Anyone,
            what: "bolt".into(),
            cause: Who::You,
        }
    );
}

/// The same two spellings through a macro whose name isn't the variant's, so
/// the elision runs on the unrestricted path.
#[test]
fn an_elided_param_reads_both_spellings_unrestricted() {
    let long: Event = elision_set()
        .read_str(r#"Ignite(who: You, what: "bolt")"#)
        .unwrap();
    assert_eq!(
        long,
        Event::Cast {
            who: Who::You,
            what: "bolt".into(),
            cause: Who::Anyone,
        }
    );

    let short: Event = elision_set().read_str(r#"Ignite(what: "bolt")"#).unwrap();
    assert_eq!(
        short,
        Event::Cast {
            who: Who::Anyone,
            what: "bolt".into(),
            cause: Who::Anyone,
        }
    );
}

/// The positional form, both spellings: a trailing elidable argument may be
/// supplied or dropped, and dropping it drops the body's tuple element.
#[test]
fn an_elided_positional_param_reads_both_arities() {
    let long: Shout = elision_set()
        .read_str_restricted(r#"Say("hi", Loud)"#)
        .unwrap();
    assert_eq!(long, Shout::Say("hi".into(), Loudness::Loud));

    let short_src = r#"Say("hi")"#;
    let short: Shout = elision_set().read_str_restricted(short_src).unwrap();
    let native: Shout = options().from_str(short_src).unwrap();
    assert_eq!(short, native);
    assert_eq!(short, Shout::Say("hi".into(), Loudness::Normal));
}

/// An omitted elidable param whose hole doesn't stand at a droppable entry
/// can't be elided — there is nothing to remove. That's a definition error,
/// reported as one rather than read as something else.
#[test]
fn an_elided_param_used_where_it_cannot_be_dropped_errors() {
    let mut macros = empty();
    macros
        .insert(&def(r#"(
            name: "Whole",
            kinds: [Subtype],
            params: { "a": Elidable(Any) },
            body: Param(a),
        )"#))
        .unwrap();
    let error = macros
        .read_str::<Subtype>("Whole()")
        .unwrap_err()
        .to_string();
    assert!(error.contains("can't be elided"), "{error}");
}

/// Dropping a body's LAST entry takes the trailing comma that followed it,
/// so a container emptied by elision doesn't end up holding a bare comma.
#[test]
fn eliding_every_body_entry_takes_the_trailing_comma() {
    let mut macros = elision_set();
    macros
        .insert(&def(r#"(
            name: "Idle",
            kinds: [Event],
            params: { "who": Elidable(Any) },
            body: Idle(
                who: Param(who),
            ),
        )"#))
        .unwrap();
    let short = "Idle()";
    let value: Event = macros.read_str_restricted(short).unwrap();
    let native: Event = options().from_str(short).unwrap();
    assert_eq!(value, native);
    assert_eq!(value, Event::Idle { who: Who::Anyone });

    let long: Event = macros.read_str_restricted("Idle(who: You)").unwrap();
    assert_eq!(long, Event::Idle { who: Who::You });
}

/// A comment where an elidable entry's separator has to be located makes that
/// entry impossible to cut, and that is a property of the DEFINITION: refused
/// at load, not at the first card that happens to write the short form.
///
/// The elidable entry leads, so its cut takes the separator AFTER it — the one
/// branch that has to scan. (An entry with a surviving predecessor cuts between
/// two known value spans and swallows any comment in the gap, which is why the
/// same comment one entry later is not an error.)
#[test]
fn a_comment_between_elidable_body_entries_is_refused_at_load() {
    let error = elision_set()
        .insert(&def(r#"(
            name: "Commented",
            kinds: [Event],
            params: { "what": Any, "who": Elidable(Any) },
            body: Cast(
                who: Param(who),
                // a note
                what: Param(what),
            ),
        )"#))
        .unwrap_err()
        .to_string();
    assert!(error.contains("a comment between"), "{error}");
}

/// The same body without the comment loads and elides — the control that keeps
/// the check above from passing for the wrong reason.
#[test]
fn an_uncommented_elidable_body_still_loads() {
    let mut macros = elision_set();
    macros
        .insert(&def(r#"(
            name: "Uncommented",
            kinds: [Event],
            params: { "what": Any, "who": Elidable(Any) },
            body: Cast(
                who: Param(who),
                what: Param(what),
            ),
        )"#))
        .unwrap();
    let short: Event = macros
        .read_str_restricted(r#"Uncommented(what: "bolt")"#)
        .unwrap();
    assert_eq!(
        short,
        Event::Cast {
            who: Who::Anyone,
            what: "bolt".into(),
            cause: Who::Anyone,
        }
    );
}

/// Elidable positional params must be the trailing ones — a call supplies a
/// prefix, so an earlier one could never be omitted.
#[test]
fn an_elidable_positional_param_must_be_trailing() {
    let error = empty()
        .insert(&def(r#"(
            name: "Backwards",
            kinds: [Subtype],
            params: [Elidable(Any), Any],
            body: Subtype(name: Param(0), types: [Land]),
        )"#))
        .unwrap_err();
    assert_eq!(
        error,
        InsertError::ElidableNotTrailing {
            name: "Backwards".into()
        }
    );
}

/// A positional signature's ONE param may not be elidable: a one-param call
/// reads through the newtype channel, which has no zero-argument spelling, so
/// the short form the marker promises could never be invoked. Refused at load
/// rather than left to silently mean nothing.
#[test]
fn a_lone_elidable_positional_param_is_rejected() {
    let error = empty()
        .insert(&def(r#"(
            name: "Solo",
            kinds: [Subtype],
            params: [Elidable(Any)],
            body: Subtype(name: Param(0), types: [Land]),
        )"#))
        .unwrap_err();
    assert_eq!(
        error,
        InsertError::LoneElidablePositional {
            name: "Solo".into()
        }
    );
}

/// Two or more positional params DO read through the tuple channel, which
/// admits `M()`, so an all-elidable signature is legal from there up and every
/// arity in `[0, len]` reads.
#[test]
fn an_all_elidable_positional_signature_reads_every_arity() {
    let mut macros = elision_set();
    macros
        .insert(&def(r#"(
            name: "Pair",
            kinds: [Event],
            params: [Elidable(Any), Elidable(Any)],
            body: Cast(who: Param(0), what: "bolt", cause: Param(1)),
        )"#))
        .unwrap();
    let cast = |who, cause| Event::Cast {
        who,
        what: "bolt".into(),
        cause,
    };
    assert_eq!(
        macros.read_str::<Event>("Pair()").unwrap(),
        cast(Who::Anyone, Who::Anyone)
    );
    assert_eq!(
        macros.read_str::<Event>("Pair(You)").unwrap(),
        cast(Who::You, Who::Anyone)
    );
    assert_eq!(
        macros.read_str::<Event>("Pair(You, You)").unwrap(),
        cast(Who::You, Who::You)
    );
}

/// `Elidable` and `Default` are alternatives, not a combination.
#[test]
fn elidable_and_default_do_not_combine() {
    let error = options()
        .from_str::<MacroDef>(
            r#"(
            name: "Both",
            kinds: [Subtype],
            params: { "a": Elidable(Default(Any, 1)) },
            body: Subtype(name: Param(a), types: [Land]),
        )"#,
        )
        .unwrap_err()
        .to_string();
    assert!(error.contains("not both"), "{error}");
}

/// A default expression may not reference an elidable param: it might not be
/// there to reference.
#[test]
fn a_default_may_not_reference_an_elidable_param() {
    let error = empty()
        .insert(&def(r#"(
            name: "Leaning",
            kinds: [Subtype],
            params: { "a": Elidable(Any), "b": Default(Any, Param(a)) },
            body: Subtype(name: Param(b), types: [Land]),
        )"#))
        .unwrap_err()
        .to_string();
    assert!(error.contains("omittable param `a`"), "{error}");
}
