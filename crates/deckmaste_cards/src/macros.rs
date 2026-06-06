//! The plugin macro system: parameterized RON templates, defined in
//! `plugins/*/macros/<kind>/<Name>.ron` and invoked by name elsewhere in the
//! plugin (e.g. `plugins/*/types/land/Forest.ron` containing
//! `LandType("Forest")`).
//!
//! A definition file is tagged with the kind of value it expands to, declares
//! its parameter signature, and gives the expansion body with `Param(n)`
//! holes (written exactly like that):
//!
//! ```ron
//! // CR 205.3i
//! Subtype(
//!     params: [String],
//!     subtype: Subtype(
//!         name: Param(0),
//!         types: [Land],
//!     ),
//! )
//! ```
//!
//! The macro's name is the definition file's stem, not part of its contents.
//!
//! Bodies are raw RON source, read in place of the invocation with the
//! invocation's arguments in scope: a `Param(n)` hole resolves, at whatever
//! position it occupies, to the n-th argument's source text — see
//! [`MacroSet::from_str`] and the [`crate::expand`] layer it drives. The
//! macros don't know what they expand to; the parse position gives the
//! result its meaning. Subtype declarations join the same namespace as
//! nullary macros whose bodies are the declaration source, verbatim.

use std::collections::HashMap;
use std::fmt;

use deckmaste_core::Ident;
use ron::value::RawValue;
use serde::Deserialize;
use serde::de::DeserializeOwned;

/// The declared type of one macro parameter. Only arity is checked today.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
pub enum ParamType {
    String,
}

/// A macro definition file, tagged by the kind of value it expands to.
#[derive(Debug, Deserialize)]
pub enum MacroFile {
    Subtype {
        params: Vec<ParamType>,
        subtype: Box<RawValue>,
    },
    Target {
        params: Vec<ParamType>,
        target: Box<RawValue>,
    },
}

/// A loaded macro: its signature and raw RON body with `Param(n)` holes.
#[derive(Debug, Clone)]
pub struct MacroDef {
    /// The kind tag, which is also the name of the type the body expands to.
    pub kind: Ident,
    pub params: Vec<ParamType>,
    body: Box<str>,
}

impl From<MacroFile> for MacroDef {
    fn from(file: MacroFile) -> Self {
        let (kind, params, body) = match file {
            MacroFile::Subtype { params, subtype } => ("Subtype", params, subtype),
            MacroFile::Target { params, target } => ("Target", params, target),
        };
        MacroDef {
            kind: kind.into(),
            params,
            body: body.get_ron().trim().into(),
        }
    }
}

impl MacroDef {
    pub fn body(&self) -> &str { &self.body }
}

/// Two macros (or subtype declarations) of one kind tried to use the same
/// name.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DuplicateMacro {
    pub kind: Ident,
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
/// This is the entry point for macro-aware reading: [`MacroSet::from_str`]
/// parses a RON document, expanding macro invocations wherever they stand in
/// for a real value.
#[derive(Default, Debug, Clone)]
pub struct MacroSet {
    /// Macros namespaced by their kind — the name of the type they expand
    /// to. A macro is only visible at positions of that type, so kinds can
    /// reuse names.
    macros: HashMap<Ident, HashMap<Ident, MacroDef>>,
}

impl MacroSet {
    /// The macro `name` expanding to the type named `kind`, if defined.
    pub fn get(&self, kind: &str, name: &str) -> Option<&MacroDef> {
        self.macros.get(kind)?.get(name)
    }

    pub fn insert(&mut self, name: Ident, def: MacroDef) -> Result<(), DuplicateMacro> {
        let kind = def.kind;
        match self.macros.entry(kind).or_default().entry(name) {
            std::collections::hash_map::Entry::Occupied(_) => Err(DuplicateMacro { kind, name }),
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(def);
                Ok(())
            }
        }
    }

    /// Whether some macro expands to the struct named `name`, i.e. whether
    /// that parse position needs macro interception.
    pub(crate) fn expands_to_struct(&self, name: &str) -> bool { self.macros.contains_key(name) }

    /// Declares a subtype as a nullary macro whose body is the declaration
    /// source, verbatim: once `Forest.ron` declares `LandType("Forest")`, a
    /// bare `Forest` re-reads that invocation.
    pub fn declare(&mut self, name: Ident, declaration: &str) -> Result<(), DuplicateMacro> {
        self.insert(
            name,
            MacroDef {
                kind: "Subtype".into(),
                params: vec![],
                body: declaration.trim().into(),
            },
        )
    }

    /// Reads a RON document with these macros in scope: an identifier that
    /// isn't real at its position is expanded and the position re-read from
    /// the expansion.
    ///
    /// `T` must be owned: expansions live in a scratch that drops when the
    /// read finishes.
    pub fn from_str<T: DeserializeOwned>(&self, source: &str) -> ron::error::SpannedResult<T> {
        let scratch = crate::expand::Scratch::default();
        let mut deserializer =
            ron::de::Deserializer::from_str_with_options(source, &deckmaste_core::ron::options())?;
        let value = T::deserialize(crate::expand::MacroAware::new(
            &mut deserializer,
            self,
            &scratch,
        ))
        .map_err(|e| deserializer.span_error(e))?;
        deserializer.end().map_err(|e| deserializer.span_error(e))?;
        Ok(value)
    }
}

impl FromIterator<(Ident, MacroDef)> for MacroSet {
    /// Panics on duplicate names; use [`MacroSet::insert`] to handle them.
    fn from_iter<I: IntoIterator<Item = (Ident, MacroDef)>>(iter: I) -> Self {
        let mut set = Self::default();
        for (name, def) in iter {
            set.insert(name, def).expect("duplicate macro name");
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::{Ability, Subtype, Type};

    use super::*;

    fn land_type() -> MacroDef {
        deckmaste_core::ron::options()
            .from_str::<MacroFile>(
                "// CR 205.3i
                Subtype(
                    params: [String],
                    subtype: Subtype(
                        name: Param(0),
                        types: [Land],
                    ),
                )",
            )
            .unwrap()
            .into()
    }

    fn forest() -> Subtype {
        Subtype {
            name: "Forest".into(),
            types: vec![Type::Land],
        }
    }

    fn macros() -> MacroSet { [("LandType".into(), land_type())].into_iter().collect() }

    #[test]
    fn invocations_read_as_the_expansion() {
        let subtype: Subtype = macros().from_str(r#"LandType("Forest")"#).unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn plain_values_still_read() {
        let subtype: Subtype = macros()
            .from_str(r#"Subtype(name: "Forest", types: [Land])"#)
            .unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn declared_subtypes_are_nullary_macros() {
        let mut macros = macros();
        macros
            .declare("Forest".into(), r#"LandType("Forest")"#)
            .unwrap();
        let subtype: Subtype = macros.from_str("Forest").unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn unknown_names_are_an_error() {
        assert!(
            macros()
                .from_str::<Subtype>(r#"IslandType("Tropical")"#)
                .is_err()
        );
    }

    #[test]
    fn wrong_arity_is_an_error() {
        for call in ["LandType", r#"LandType("Forest", "Island")"#] {
            assert!(
                macros().from_str::<Subtype>(call).is_err(),
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
            .insert(
                "Flying".into(),
                MacroDef {
                    kind: "Ability".into(),
                    params: vec![],
                    body: r#"Keyword(keyword: "Flying", expanded: (params: [], value: Static))"#
                        .into(),
                },
            )
            .unwrap();
        let ability: Ability = macros.from_str("Flying").unwrap();
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
        let subtype: Subtype = macros.from_str("Woods").unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn duplicate_names_are_an_error() {
        let duplicate = Err(DuplicateMacro {
            kind: "Subtype".into(),
            name: "LandType".into(),
        });
        let mut macros = macros();
        assert_eq!(macros.insert("LandType".into(), land_type()), duplicate);
        assert_eq!(macros.declare("LandType".into(), "Forest"), duplicate);
    }

    #[test]
    fn recursion_is_an_error_not_a_stack_overflow() {
        let mut macros = macros();
        macros.declare("Ouroboros".into(), "Ouroboros").unwrap();
        macros.declare("Ping".into(), "Pong").unwrap();
        macros.declare("Pong".into(), "Ping").unwrap();
        for cycle in ["Ouroboros", "Ping"] {
            let error = macros.from_str::<Subtype>(cycle).unwrap_err();
            assert!(
                error.to_string().contains("macros don't recurse"),
                "unexpected error: {error}"
            );
        }
    }

    #[test]
    fn macros_are_namespaced_by_kind() {
        // The same name can mean different things for different kinds, and
        // each is only visible at positions of its own kind.
        let mut macros = MacroSet::default();
        for kind in ["Subtype", "Target"] {
            macros
                .insert(
                    "Self".into(),
                    MacroDef {
                        kind: kind.into(),
                        params: vec![ParamType::String],
                        body: "Param(0)".into(),
                    },
                )
                .unwrap();
        }

        let target: deckmaste_core::Target = macros.from_str("Self(Player)").unwrap();
        assert_eq!(target, deckmaste_core::Target::Player);

        // A Target macro is invisible at an Ability position.
        let error = macros.from_str::<Ability>("Self(Static)").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("neither a variant of `Ability` nor a known `Ability` macro"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn params_resolve_at_enum_positions() {
        let mut macros = macros();
        macros
            .insert(
                "WithType".into(),
                MacroDef {
                    kind: "Subtype".into(),
                    params: vec![ParamType::String],
                    body: r#"Subtype(name: "Forest", types: [Param(0)])"#.into(),
                },
            )
            .unwrap();
        let subtype: Subtype = macros.from_str("WithType(Land)").unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn params_resolve_per_argument() {
        let mut macros = macros();
        macros
            .insert(
                "Pair".into(),
                MacroDef {
                    kind: "Subtype".into(),
                    params: vec![ParamType::String, ParamType::String],
                    body: "Subtype(name: Param(0), types: [Param(1)])".into(),
                },
            )
            .unwrap();
        let subtype: Subtype = macros.from_str(r#"Pair("Forest", Land)"#).unwrap();
        assert_eq!(subtype, forest());
    }

    #[test]
    fn string_literals_mentioning_param_are_untouched() {
        // `Param` is only recognized where a value is expected, so it can
        // appear verbatim inside body strings.
        let mut macros = macros();
        macros
            .insert(
                "Weird".into(),
                MacroDef {
                    kind: "Subtype".into(),
                    params: vec![],
                    body: r#"Subtype(name: "literally Param(0)", types: [Land])"#.into(),
                },
            )
            .unwrap();
        let subtype: Subtype = macros.from_str("Weird").unwrap();
        assert_eq!(subtype.name, "literally Param(0)");
    }

    #[test]
    fn out_of_range_params_are_an_error() {
        let mut macros = macros();
        macros
            .insert(
                "OffByOne".into(),
                MacroDef {
                    kind: "Subtype".into(),
                    params: vec![ParamType::String],
                    body: "Subtype(name: Param(1), types: [Land])".into(),
                },
            )
            .unwrap();
        let error = macros
            .from_str::<Subtype>(r#"OffByOne("Forest")"#)
            .unwrap_err();
        assert!(
            error.to_string().contains("no Param(1)"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn params_outside_macros_are_an_error() {
        let error = macros().from_str::<Subtype>("Param(0)").unwrap_err();
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
            .insert(
                "Vanilla".into(),
                MacroDef {
                    kind: "CardFace".into(),
                    params: vec![ParamType::String, ParamType::String],
                    body: r#"CardFace(
                    name: Param(0),
                    mana_cost: [Hybrid(Generic(Param(1)), White), Green],
                    types: [Creature],
                )"#
                    .into(),
                },
            )
            .unwrap();
        let face: CardFace = macros.from_str(r#"Vanilla("Bear", 2)"#).unwrap();
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
