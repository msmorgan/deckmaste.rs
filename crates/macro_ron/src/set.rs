//! Macro definitions and the set of macros in scope.
//!
//! A definition file is a bare struct naming the macro, the kinds of value
//! it can expand to, its parameter signature, and the expansion body with
//! `Param(...)` holes:
//!
//! ```ron
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
//! expand layer it drives.

use std::collections::HashMap;
use std::fmt;

use ron::value::RawValue;
use serde::Deserialize;
use serde::de::{
    DeserializeOwned, DeserializeSeed, Deserializer, EnumAccess, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use crate::kind::KindSet;
use crate::param::{ParamType, ParamTypeSet};
use crate::{Ident, IdentSeed};

/// A macro's parameter signature, whose shape decides the invocation
/// grammar: positional (`M(a, b)`, holes `Param(0)`) or named
/// (`M(x: a, y: b)`, holes `Param(x)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Params {
    Positional(Vec<ParamType>),
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
    /// The kinds this macro can expand to, by position name; written as
    /// bare identifiers in definition files (`kinds: [Subtype]`).
    #[serde(deserialize_with = "kind_names")]
    pub kinds: Vec<Ident>,
    #[serde(default)]
    pub params: Params,
    /// Raw RON source with `Param(...)` holes.
    #[serde(deserialize_with = "raw_body")]
    pub(crate) body: Box<str>,
}

fn raw_body<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Box<str>, D::Error> {
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    Ok(raw.get_ron().trim().into())
}

/// Reads `kinds: [Subtype, Filter]` — bare identifiers, which in the serde
/// data model are unit enum variants, so each element goes through the enum
/// channel like the old hardcoded kind enum did.
fn kind_names<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Ident>, D::Error> {
    struct KindName;

    impl<'de> DeserializeSeed<'de> for KindName {
        type Value = Ident;

        fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
            de.deserialize_enum("", &[], self)
        }
    }

    impl<'de> Visitor<'de> for KindName {
        type Value = Ident;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("a kind name") }

        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
            let (ident, variant) = data.variant_seed(IdentSeed)?;
            variant.unit_variant()?;
            Ok(ident)
        }
    }

    struct KindNames;

    impl<'de> Visitor<'de> for KindNames {
        type Value = Vec<Ident>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a list of kind names")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut kinds = Vec::new();
            while let Some(kind) = seq.next_element_seed(KindName)? {
                kinds.push(kind);
            }
            Ok(kinds)
        }
    }

    deserializer.deserialize_seq(KindNames)
}

impl MacroDef {
    #[must_use]
    pub fn body(&self) -> &str { &self.body }
}

/// Why a macro couldn't be registered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertError {
    /// Two macros (or declarations) of one kind tried to use the same name.
    Duplicate { kind: Ident, name: Ident },
    /// A definition named a kind no [`Kind`](crate::Kind) was registered for.
    UnknownKind { kind: Ident, name: Ident },
    /// A definition named a param type no validator was registered for.
    UnknownParamType { type_name: Ident, name: Ident },
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InsertError::Duplicate { kind, name } => {
                write!(f, "a {kind} macro named `{name}` is already defined")
            }
            InsertError::UnknownKind { kind, name } => {
                write!(f, "macro `{name}` declares unregistered kind `{kind}`")
            }
            InsertError::UnknownParamType { type_name, name } => {
                write!(
                    f,
                    "macro `{name}` declares unregistered param type `{type_name}`"
                )
            }
        }
    }
}

impl std::error::Error for InsertError {}

/// The macros in scope, keyed by name.
///
/// This is the entry point for macro-aware reading: [`MacroSet::read_str`]
/// parses a RON document, expanding macro invocations wherever they stand in
/// for a real value.
#[derive(Debug, Clone)]
pub struct MacroSet {
    kinds: KindSet,
    options: ron::Options,
    param_types: ParamTypeSet,
    /// Macros namespaced by kind — a macro is only visible at positions of
    /// the types it expands to, so kinds can reuse names.
    macros: HashMap<Ident, HashMap<Ident, MacroDef>>,
}

impl MacroSet {
    /// An empty set over the given kind registry, reading the default RON
    /// dialect; see [`MacroSet::with_options`].
    #[must_use]
    pub fn new(kinds: KindSet) -> Self {
        MacroSet {
            kinds,
            options: ron::Options::default(),
            param_types: ParamTypeSet::default(),
            macros: HashMap::new(),
        }
    }

    /// Sets the RON dialect every read uses — the document, macro bodies,
    /// and invocation arguments alike.
    #[must_use]
    pub fn with_options(mut self, options: ron::Options) -> Self {
        self.options = options;
        self
    }

    /// Sets the param types in scope, with the validators that enforce them.
    /// The default ([`ParamTypeSet::default`]) already provides `Any` and
    /// `String`; embedders add domain types like `Color`.
    #[must_use]
    pub fn with_param_types(mut self, param_types: ParamTypeSet) -> Self {
        self.param_types = param_types;
        self
    }

    pub(crate) fn options(&self) -> &ron::Options { &self.options }

    /// The macro `name` for positions of the type named `kind`, if defined.
    #[must_use]
    pub fn get(&self, kind: &str, name: &str) -> Option<&MacroDef> {
        self.macros.get(kind)?.get(name)
    }

    /// See [`Kind::literal_wrapper`](crate::Kind::literal_wrapper).
    pub(crate) fn literal_wrapper(&self, position: &str) -> Option<&'static str> {
        self.kinds.get(position)?.literal
    }

    /// The validator for the param type named `name`, if registered.
    pub(crate) fn param_validator(&self, name: &str) -> Option<crate::param::Validator> {
        self.param_types.get(name)
    }

    /// See [`Kind::remembers_expansion`](crate::Kind::remembers_expansion).
    pub(crate) fn remembers_expansion(&self, position: &str) -> bool {
        self.kinds.get(position).is_some_and(|kind| kind.remembers)
    }

    /// Whether some macro expands to the struct named `name`, i.e. whether
    /// that parse position needs macro interception.
    pub(crate) fn expands_to_struct(&self, name: &str) -> bool { self.macros.contains_key(name) }

    fn check_kinds(&self, def: &MacroDef) -> Result<(), InsertError> {
        for &kind in &def.kinds {
            if self.kinds.get(&kind).is_none() {
                return Err(InsertError::UnknownKind {
                    kind,
                    name: def.name,
                });
            }
        }
        Ok(())
    }

    fn check_param_types(&self, def: &MacroDef) -> Result<(), InsertError> {
        let check = |type_name: Ident| {
            if self.param_types.contains(&type_name) {
                Ok(())
            } else {
                Err(InsertError::UnknownParamType {
                    type_name,
                    name: def.name,
                })
            }
        };
        match &def.params {
            Params::Positional(types) => types.iter().try_for_each(|t| check(t.0)),
            Params::Named(types) => types.values().try_for_each(|t| check(t.0)),
        }
    }

    /// Registers `def` under each of its kinds.
    ///
    /// # Errors
    /// If any of those kinds is unregistered or already has a macro with
    /// `def`'s name, or the definition repeats a kind (which would otherwise
    /// self-overwrite silently).
    pub fn insert(&mut self, def: &MacroDef) -> Result<(), InsertError> {
        self.check_kinds(def)?;
        self.check_param_types(def)?;
        for (i, &kind) in def.kinds.iter().enumerate() {
            let duplicate = def.kinds[..i].contains(&kind)
                || self
                    .macros
                    .get(&kind)
                    .is_some_and(|named| named.contains_key(&def.name));
            if duplicate {
                return Err(InsertError::Duplicate {
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

    /// Declares `name` as a nullary macro of `kind` whose body is
    /// `declaration`, verbatim: once `Forest.ron` declares
    /// `LandType("Forest")`, a bare `Forest` re-reads that invocation.
    ///
    /// # Errors
    /// If `kind` is unregistered or already has a macro named `name`.
    pub fn declare(
        &mut self,
        kind: &str,
        name: Ident,
        declaration: &str,
    ) -> Result<(), InsertError> {
        self.insert(&MacroDef {
            name,
            kinds: vec![kind.into()],
            params: Params::default(),
            body: declaration.trim().into(),
        })
    }

    /// Registers `def` under each of its kinds, overriding same-kind
    /// entries already in scope. Layer-to-layer overriding is legal — last
    /// layer wins — so the caller is responsible for rejecting duplicates
    /// *within* one layer.
    ///
    /// # Errors
    /// If any of `def`'s kinds is unregistered.
    pub fn replace(&mut self, def: &MacroDef) -> Result<(), InsertError> {
        self.check_kinds(def)?;
        self.check_param_types(def)?;
        for &kind in &def.kinds {
            self.macros
                .entry(kind)
                .or_default()
                .insert(def.name, def.clone());
        }
        Ok(())
    }

    /// Like [`MacroSet::declare`], but overriding: see [`MacroSet::replace`].
    ///
    /// # Errors
    /// If `kind` is unregistered.
    pub fn redeclare(
        &mut self,
        kind: &str,
        name: Ident,
        declaration: &str,
    ) -> Result<(), InsertError> {
        self.replace(&MacroDef {
            name,
            kinds: vec![kind.into()],
            params: Params::default(),
            body: declaration.trim().into(),
        })
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
        let mut deserializer = ron::de::Deserializer::from_str_with_options(source, &self.options)?;
        let value = T::deserialize(crate::expand::MacroAware::new(&mut deserializer, &read))
            .map_err(|e| deserializer.span_error(e))?;
        deserializer.end().map_err(|e| deserializer.span_error(e))?;
        Ok(value)
    }
}
