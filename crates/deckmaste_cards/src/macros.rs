//! The plugin macro system: parameterized RON templates, defined in
//! `plugins/*/macros/**/*.ron` (the paths and file names are organizational
//! only) and invoked by name where a value of one of their kinds is
//! expected.
//!
//! A definition file is a bare struct naming the macro, the kinds of value
//! it can expand to, its parameter signature, and the expansion body with
//! `Param(...)` holes:
//!
//! ```ron
//! // CR 205.3i
//! (
//!     name: "LandType",
//!     kinds: [Subtype],
//!     params: [String],
//!     body: Subtype(
//!         name: Param(0),
//!         types: [Land],
//!     ),
//! )
//! ```
//!
//! The signature's shape decides the invocation grammar: positional
//! `params: [String]` is invoked `LandType("Forest")` with `Param(0)` holes,
//! named `params: {"cost": String}` is invoked `Boast(cost: "{1}")` with
//! `Param(cost)` holes.
//!
//! Bodies are raw RON source, read in place of the invocation with the
//! invocation's arguments in scope — see [`MacroSet::read_str`] and the
//! [`crate::expand`] layer it drives. Subtype declarations join the same
//! namespace as nullary macros whose bodies are the declaration source,
//! verbatim.

use std::collections::HashMap;
use std::fmt;

use deckmaste_core::Ident;
use ron::value::RawValue;
use serde::Deserialize;
use serde::de::{DeserializeOwned, Deserializer, MapAccess, SeqAccess, Visitor};

/// The kinds of value a macro can expand to: the types whose parse positions
/// consult the macro namespace. Variant names must match the Rust types'
/// serde names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum MacroKind {
    Ability,
    CardFace,
    CostComponent,
    Effect,
    Filter,
    Reference,
    Selection,
    Subtype,
}

impl MacroKind {
    /// The kind a position with this (serde) type name checks, if any.
    #[must_use]
    pub fn from_position(name: &str) -> Option<Self> {
        Some(match name {
            "Ability" => MacroKind::Ability,
            "CardFace" => MacroKind::CardFace,
            "CostComponent" => MacroKind::CostComponent,
            "Effect" => MacroKind::Effect,
            "Filter" => MacroKind::Filter,
            "Reference" => MacroKind::Reference,
            "Selection" => MacroKind::Selection,
            "Subtype" => MacroKind::Subtype,
            _ => return None,
        })
    }
}

impl fmt::Display for MacroKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{self:?}") }
}

/// The declared type of one macro parameter. Only the signature's shape and
/// arity are checked today.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ParamType {
    String,
}

/// A macro's parameter signature, whose shape decides the invocation
/// grammar: positional (`M(a, b)`, holes `Param(0)`) or named
/// (`M(x: a, y: b)`, holes `Param(x)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Params {
    Positional(Vec<ParamType>),
    // ParamType currently has one variant, hence zero-sized; more are coming.
    #[expect(clippy::zero_sized_map_values)]
    Named(HashMap<Ident, ParamType>),
}

impl Default for Params {
    fn default() -> Self { Params::Positional(vec![]) }
}

impl<'de> Deserialize<'de> for Params {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ShapeVisitor;
        impl<'de> Visitor<'de> for ShapeVisitor {
            type Value = Params;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a parameter list or a name-to-type map")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut params = Vec::new();
                while let Some(param) = seq.next_element()? {
                    params.push(param);
                }
                Ok(Params::Positional(params))
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut params = HashMap::new();
                while let Some((name, param)) = map.next_entry()? {
                    params.insert(name, param);
                }
                Ok(Params::Named(params))
            }
        }

        deserializer.deserialize_any(ShapeVisitor)
    }
}

/// A macro: self-describing, matching its definition file.
///
/// Definition files may carry extra metadata fields (e.g. `template:`,
/// the rules text a use of the macro renders as); serde ignores them
/// here, so don't add `deny_unknown_fields`.
#[derive(Debug, Clone, Deserialize)]
pub struct MacroDef {
    pub name: Ident,
    pub kinds: Vec<MacroKind>,
    #[serde(default)]
    pub params: Params,
    /// Raw RON source with `Param(...)` holes.
    #[serde(deserialize_with = "raw_body")]
    body: Box<str>,
}

fn raw_body<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Box<str>, D::Error> {
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    Ok(raw.get_ron().trim().into())
}

impl MacroDef {
    #[must_use]
    pub fn body(&self) -> &str { &self.body }
}

/// Two macros (or subtype declarations) of one kind tried to use the same
/// name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateMacro {
    pub kind: MacroKind,
    pub name: Ident,
}

impl fmt::Display for DuplicateMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a {} macro named `{}` is already defined",
            self.kind, self.name,
        )
    }
}

impl std::error::Error for DuplicateMacro {}

/// The macros in scope, keyed by name.
///
/// This is the entry point for macro-aware reading: [`MacroSet::read_str`]
/// parses a RON document, expanding macro invocations wherever they stand in
/// for a real value.
#[derive(Debug, Clone, Default)]
pub struct MacroSet {
    /// Macros namespaced by kind — a macro is only visible at positions of
    /// the types it expands to, so kinds can reuse names.
    macros: HashMap<MacroKind, HashMap<Ident, MacroDef>>,
}

impl MacroSet {
    /// The macro `name` for positions of the type named `kind`, if defined.
    #[must_use]
    pub fn get(&self, kind: &str, name: &str) -> Option<&MacroDef> {
        self.macros.get(&MacroKind::from_position(kind)?)?.get(name)
    }

    /// Registers `def` under each of its kinds.
    ///
    /// # Errors
    /// If any of those kinds already has a macro with `def`'s name, or the
    /// definition repeats a kind (which would otherwise self-overwrite
    /// silently).
    pub fn insert(&mut self, def: &MacroDef) -> Result<(), DuplicateMacro> {
        for (i, &kind) in def.kinds.iter().enumerate() {
            let duplicate = def.kinds[..i].contains(&kind)
                || self
                    .macros
                    .get(&kind)
                    .is_some_and(|named| named.contains_key(&def.name));
            if duplicate {
                return Err(DuplicateMacro {
                    kind,
                    name: def.name,
                });
            }
        }
        for &kind in &def.kinds {
            self.macros
                .entry(kind)
                .or_default()
                .insert(def.name, def.clone());
        }
        Ok(())
    }

    /// Whether some macro expands to the struct named `name`, i.e. whether
    /// that parse position needs macro interception.
    pub(crate) fn expands_to_struct(&self, name: &str) -> bool {
        MacroKind::from_position(name).is_some_and(|kind| self.macros.contains_key(&kind))
    }

    /// Declares a subtype as a nullary macro whose body is the declaration
    /// source, verbatim: once `Forest.ron` declares `LandType("Forest")`, a
    /// bare `Forest` re-reads that invocation.
    ///
    /// # Errors
    /// If a Subtype macro named `name` already exists.
    pub fn declare(&mut self, name: Ident, declaration: &str) -> Result<(), DuplicateMacro> {
        self.insert(&MacroDef {
            name,
            kinds: vec![MacroKind::Subtype],
            params: Params::default(),
            body: declaration.trim().into(),
        })
    }

    /// Registers `def` under each of its kinds, overriding same-kind
    /// entries already in scope. Layer-to-layer overriding is legal — last
    /// plugin wins — so the caller is responsible for rejecting duplicates
    /// *within* one layer.
    pub(crate) fn replace(&mut self, def: &MacroDef) {
        for &kind in &def.kinds {
            self.macros
                .entry(kind)
                .or_default()
                .insert(def.name, def.clone());
        }
    }

    /// Like [`MacroSet::declare`], but overriding: see
    /// [`MacroSet::replace`].
    pub(crate) fn redeclare(&mut self, name: Ident, declaration: &str) {
        self.replace(&MacroDef {
            name,
            kinds: vec![MacroKind::Subtype],
            params: Params::default(),
            body: declaration.trim().into(),
        });
    }

    /// Reads a RON document with these macros in scope: an identifier that
    /// isn't real at its position is expanded and the position re-read from
    /// the expansion.
    ///
    /// `T` must be owned: any text spliced during expansion drops when the
    /// read finishes.
    ///
    /// # Errors
    /// On RON syntax errors, names that are neither variants nor macros of
    /// the position's kind, malformed invocations, unresolvable `Param(...)`
    /// holes, and expansion cycles.
    pub fn read_str<T: DeserializeOwned>(&self, source: &str) -> ron::error::SpannedResult<T> {
        let read = crate::expand::ReadCtx::new(self);
        let mut deserializer =
            ron::de::Deserializer::from_str_with_options(source, &deckmaste_core::ron::options())?;
        let value = T::deserialize(crate::expand::MacroAware::new(&mut deserializer, &read))
            .map_err(|e| deserializer.span_error(e))?;
        deserializer.end().map_err(|e| deserializer.span_error(e))?;
        Ok(value)
    }
}

impl FromIterator<MacroDef> for MacroSet {
    /// Panics on duplicate names; use [`MacroSet::insert`] to handle them.
    fn from_iter<I: IntoIterator<Item = MacroDef>>(iter: I) -> Self {
        let mut set = Self::default();
        for def in iter {
            set.insert(&def).expect("duplicate macro name");
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::{Ability, CharacteristicFilter, Filter, ObjectKind, Subtype, Type};

    use super::*;

    /// `from_position` matches on serde type names, which a Rust rename
    /// changes without a compile error — this is the tie. (A `#[serde(rename)]`
    /// on a core type would still slip through; none carries one.)
    #[test]
    fn position_names_track_the_core_types() {
        fn position<T>() -> Option<MacroKind> {
            let name = std::any::type_name::<T>().rsplit("::").next().unwrap();
            MacroKind::from_position(name)
        }

        assert_eq!(position::<Ability>(), Some(MacroKind::Ability));
        assert_eq!(
            position::<deckmaste_core::CardFace>(),
            Some(MacroKind::CardFace)
        );
        assert_eq!(
            position::<deckmaste_core::CostComponent>(),
            Some(MacroKind::CostComponent)
        );
        assert_eq!(
            position::<deckmaste_core::Effect>(),
            Some(MacroKind::Effect)
        );
        assert_eq!(position::<Filter>(), Some(MacroKind::Filter));
        assert_eq!(
            position::<deckmaste_core::Reference>(),
            Some(MacroKind::Reference)
        );
        assert_eq!(
            position::<deckmaste_core::Selection>(),
            Some(MacroKind::Selection)
        );
        assert_eq!(position::<Subtype>(), Some(MacroKind::Subtype));
    }

    fn subtype_macro(name: &str, params: Vec<ParamType>, body: &str) -> MacroDef {
        MacroDef {
            name: name.into(),
            kinds: vec![MacroKind::Subtype],
            params: Params::Positional(params),
            body: body.trim().into(),
        }
    }

    fn land_type() -> MacroDef {
        deckmaste_core::ron::options()
            .from_str::<MacroDef>(
                r#"// CR 205.3i
                (
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

    fn macros() -> MacroSet { [land_type()].into_iter().collect() }

    #[test]
    fn definition_files_are_self_describing() {
        let def = land_type();
        assert_eq!(def.name, "LandType");
        assert_eq!(def.kinds, [MacroKind::Subtype]);
        assert_eq!(def.params, Params::Positional(vec![ParamType::String]));
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
    fn declared_subtypes_are_nullary_macros() {
        let mut macros = macros();
        macros
            .declare("Forest".into(), r#"LandType("Forest")"#)
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
    fn enum_positions_expand_unknown_variants() {
        // `Flying` is not a variant of Ability; the macro fills it in, and
        // its expansion is read where the invocation stood.
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Flying".into(),
                kinds: vec![MacroKind::Ability],
                params: Params::default(),
                body: r#"Keyword(keyword: "Flying", expanded: (params: [], value: Static))"#.into(),
            })
            .unwrap();
        let ability: Ability = macros.read_str("Flying").unwrap();
        let Ability::Keyword(keyword) = ability else {
            panic!("expected a keyword ability, got {ability:?}");
        };
        assert_eq!(keyword.keyword, "Flying");
    }

    #[test]
    fn macros_can_expand_to_macros() {
        // try_again(), twice: Woods is a macro reading `Forest`, itself a
        // declaration reading `LandType("Forest")`.
        let mut macros = macros();
        macros
            .declare("Forest".into(), r#"LandType("Forest")"#)
            .unwrap();
        macros.declare("Woods".into(), "Forest").unwrap();
        let subtype: Subtype = macros.read_str("Woods").unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn duplicate_names_are_an_error() {
        let duplicate = Err(DuplicateMacro {
            kind: MacroKind::Subtype,
            name: "LandType".into(),
        });
        let mut macros = macros();
        assert_eq!(macros.insert(&land_type()), duplicate);
        assert_eq!(macros.declare("LandType".into(), "Forest"), duplicate);
    }

    #[test]
    fn recursion_is_an_error_not_a_stack_overflow() {
        let mut macros = macros();
        macros.declare("Ouroboros".into(), "Ouroboros").unwrap();
        macros.declare("Ping".into(), "Pong").unwrap();
        macros.declare("Pong".into(), "Ping").unwrap();
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
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Self".into(),
                kinds: vec![MacroKind::Subtype, MacroKind::Filter],
                params: Params::Positional(vec![ParamType::String]),
                body: "Param(0)".into(),
            })
            .unwrap();

        let filter: Filter = macros.read_str("Self(Kind(Player))").unwrap();
        assert_eq!(filter, Filter::Kind(ObjectKind::Player));

        // The macro is invisible at an Ability position.
        let error = macros.read_str::<Ability>("Self(Static)").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("neither a variant of `Ability` nor a known `Ability` macro"),
            "unexpected error: {error}"
        );
    }

    /// Filter's manual Deserialize must go through `deserialize_enum` with
    /// the full flattened variant list: that is what lets unknown names at
    /// Filter positions fall through to the macro namespace.
    #[test]
    fn filter_positions_expand_macros() {
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "AnyTargetish".into(),
                kinds: vec![MacroKind::Filter],
                params: Params::default(),
                body: "OneOf([Kind(Player), AllOf([Kind(Permanent), Type(Creature)])])".into(),
            })
            .unwrap();
        let filter: Filter = macros.read_str("AnyTargetish").unwrap();
        let Filter::OneOf(arms) = filter else {
            panic!("expected OneOf, got {filter:?}");
        };
        assert_eq!(arms[0], Filter::Kind(ObjectKind::Player));
        // The nested arm proves Filter positions *inside* an expansion stay
        // macro-aware too.
        assert_eq!(
            arms[1],
            Filter::AllOf(vec![
                Filter::Kind(ObjectKind::Permanent),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ])
        );
        assert_eq!(arms.len(), 2);
    }

    /// Same pin for Selection positions: nothing exercises Selection macros
    /// in real data yet, and Plan 2 will make this path load-bearing.
    #[test]
    fn selection_positions_expand_macros() {
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "EachCreature".into(),
                kinds: vec![MacroKind::Selection],
                params: Params::default(),
                body: "Each(Type(Creature))".into(),
            })
            .unwrap();
        let selection: deckmaste_core::Selection = macros.read_str("EachCreature").unwrap();
        assert_eq!(
            selection,
            deckmaste_core::Selection::Each(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            )))
        );
    }

    /// And for Reference positions.
    #[test]
    fn reference_positions_expand_macros() {
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "MyController".into(),
                kinds: vec![MacroKind::Reference],
                params: Params::default(),
                body: "ControllerOf(This)".into(),
            })
            .unwrap();
        let reference: deckmaste_core::Reference = macros.read_str("MyController").unwrap();
        assert_eq!(
            reference,
            deckmaste_core::Reference::ControllerOf(Box::new(deckmaste_core::Reference::This))
        );
    }

    /// `CostComponent` positions participate in macro expansion: a registered
    /// `CostComponent` macro is expanded in place of its name.
    #[test]
    fn cost_positions_expand_macros() {
        use deckmaste_core::{Action, CostComponent, Reference, Selection};

        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "SacThis".into(),
                kinds: vec![MacroKind::CostComponent],
                params: Params::default(),
                body: "Do(Sacrifice(That(This)))".into(),
            })
            .unwrap();
        let cost: CostComponent = macros.read_str("SacThis").unwrap();
        assert_eq!(
            cost,
            CostComponent::Do(Action::Sacrifice(Selection::That(Reference::This)))
        );
    }

    /// Effect positions participate in macro expansion: a registered `Effect`
    /// macro is expanded in place of its name.
    #[test]
    fn effect_positions_expand_macros() {
        use deckmaste_core::{Action, Effect};

        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Investigate".into(),
                kinds: vec![MacroKind::Effect],
                params: Params::default(),
                body: "DrawCards(1)".into(),
            })
            .unwrap();
        let effect: Effect = macros.read_str("Investigate").unwrap();
        assert_eq!(effect, Effect::Act(Action::DrawCards(1)));
    }

    #[test]
    fn params_resolve_at_enum_positions() {
        let mut macros = macros();
        macros
            .insert(&subtype_macro(
                "WithType",
                vec![ParamType::String],
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
                vec![ParamType::String, ParamType::String],
                "Subtype(name: Param(0), types: [Param(1)])",
            ))
            .unwrap();
        let subtype: Subtype = macros.read_str(r#"Pair("Forest", Land)"#).unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn named_parameters_invoke_struct_shaped() {
        use deckmaste_core::{CardFace, ManaSymbol, SimpleManaSymbol};

        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Vanilla".into(),
                kinds: vec![MacroKind::CardFace],
                params: Params::Named(
                    [
                        ("name".into(), ParamType::String),
                        ("cost".into(), ParamType::String),
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
            Vec::from(face.mana_cost),
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
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Boast".into(),
                kinds: vec![MacroKind::Ability],
                params: Params::Named([("cost".into(), ParamType::String)].into()),
                body: r"Keyword(keyword: Param(cost), expanded: (params: [], value: Static))"
                    .into(),
            })
            .unwrap();
        let ability: Ability = macros.read_str(r#"Boast(cost: "{1}")"#).unwrap();
        let Ability::Keyword(keyword) = ability else {
            panic!("expected a keyword ability, got {ability:?}");
        };
        assert_eq!(keyword.keyword, "{1}");
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
                vec![ParamType::String],
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
        use deckmaste_core::{CardFace, Color, ManaSymbol};

        // `Generic(Param(1))`: a hole as the entire content of a newtype
        // variant, inside a partially untagged enum — the shape of
        // `cost: [Mana(Generic(Param(0)))]` down the road.
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Vanilla".into(),
                kinds: vec![MacroKind::CardFace],
                params: Params::Positional(vec![ParamType::String, ParamType::String]),
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
            Vec::from(face.mana_cost),
            vec![
                ManaSymbol::Hybrid(2.into(), Color::White),
                Color::Green.into(),
            ]
        );
    }
}
