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
//! `Param(cost)` holes. A named param may carry a default —
//! `{"template": Default(String, Param(name))}` — filled (and validated)
//! when the invocation omits it; defaults may reference only non-defaulted
//! params of the same signature.
//!
//! Bodies are raw RON source, read in place of the invocation with the
//! invocation's arguments in scope — see [`MacroSet::read_str`] and the
//! expand layer it drives.

use std::collections::HashMap;
use std::fmt;

use ron::value::RawValue;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde::de::DeserializeSeed;
use serde::de::Deserializer;
use serde::de::EnumAccess;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;

use crate::Ident;
use crate::IdentSeed;
use crate::kind::KindSet;
use crate::param::ParamType;
use crate::param::ParamTypeSet;

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
/// Serde-named `Macro`: that is the position name the macro-aware reader
/// sees, so meta-macros declare `kinds: [Macro]`. Definition files read as
/// anonymous structs, so the rename is otherwise invisible.
///
/// Definition files may carry extra metadata fields (e.g. `template:`,
/// the rules text a use of the macro renders as); serde ignores them
/// here, so don't add `deny_unknown_fields`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Macro")]
pub struct MacroDef {
    pub name: Ident,
    /// The kinds this macro can expand to, by position name; written as
    /// bare identifiers in definition files (`kinds: [Subtype]`).
    #[serde(deserialize_with = "kind_names")]
    pub kinds: Vec<Ident>,
    #[serde(default)]
    pub params: Params,
    /// Optional human-readable rules-text template (metadata): `~` = self,
    /// `{i}` = the i-th positional arg. Consumed by the card-text renderer.
    #[serde(default)]
    pub template: Option<String>,
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

    #[must_use]
    pub fn template(&self) -> Option<&str> { self.template.as_deref() }
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
    /// A definition's name can't be invoked: macros are invoked as bare
    /// identifiers, so the name must be one.
    InvalidName { name: Ident },
    /// A meta-macro (kind `Macro`) declared positional params: hole
    /// indices would be ambiguous between the meta's frame and the
    /// produced definition's own params.
    MetaParamsPositional { name: Ident },
    /// A positional signature declared a `Default(...)` param; defaults are
    /// named-only (trailing-default arity is out of scope).
    PositionalDefault { name: Ident },
    /// A named param's default expression is unusable: unparseable, or it
    /// references a param that is missing, defaulted, or index-addressed.
    BadDefault {
        name: Ident,
        param: Ident,
        reason: String,
    },
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
            InsertError::InvalidName { name } => {
                write!(
                    f,
                    "macro name `{name}` is not a bare identifier; macros are invoked bare"
                )
            }
            InsertError::MetaParamsPositional { name } => {
                write!(
                    f,
                    "meta-macro `{name}` must use named params (indices are \
                     ambiguous between the meta and its produced definition)"
                )
            }
            InsertError::PositionalDefault { name } => {
                write!(
                    f,
                    "macro `{name}` declares a positional param with a default; \
                     defaults are named-only"
                )
            }
            InsertError::BadDefault {
                name,
                param,
                reason,
            } => {
                write!(f, "macro `{name}` param `{param}` default: {reason}")
            }
        }
    }
}

impl std::error::Error for InsertError {}

fn is_bare_ident(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The set-independent definition checks: an invocable name, and named
/// params on meta-macros (hole indices can't tell the meta's frame from
/// the produced definition's own params).
fn check_def(def: &MacroDef) -> Result<(), InsertError> {
    if !is_bare_ident(&def.name) {
        return Err(InsertError::InvalidName { name: def.name });
    }
    if def.kinds.iter().any(|kind| kind.as_str() == "Macro")
        && matches!(&def.params, Params::Positional(types) if !types.is_empty())
    {
        return Err(InsertError::MetaParamsPositional { name: def.name });
    }
    Ok(())
}

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

    /// See [`Kind::embeds_untagged`](crate::Kind::embeds_untagged).
    pub(crate) fn embeds_untagged(&self, position: &str) -> bool {
        self.kinds.get(position).is_some_and(|kind| kind.embeds)
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
            Params::Positional(types) => types.iter().try_for_each(|t| check(t.name)),
            Params::Named(types) => types.values().try_for_each(|t| check(t.name)),
        }
    }

    /// Default expressions must be confined to named signatures and reference
    /// only *non-defaulted* sibling params — required params are always
    /// supplied, so fill-time splicing (`HoleMode::Strict`) is total, and
    /// cycles are impossible by construction. Checked at insert so a bad
    /// default fails at load, not first invocation. (The default *value* is
    /// not validated here: validators read with macros in scope, and load
    /// order would make that flaky — the filled text is validated per
    /// invocation instead.)
    fn check_defaults(&self, def: &MacroDef) -> Result<(), InsertError> {
        let signature = match &def.params {
            Params::Positional(types) => {
                if types.iter().any(|t| t.default.is_some()) {
                    return Err(InsertError::PositionalDefault { name: def.name });
                }
                return Ok(());
            }
            Params::Named(signature) => signature,
        };
        for (&param, ty) in signature {
            let Some(default) = ty.default.as_deref() else {
                continue;
            };
            let bad = |reason: String| InsertError::BadDefault {
                name: def.name,
                param,
                reason,
            };
            let mut keys = Vec::new();
            crate::expand::collect_param_keys(default, &self.options, &mut keys).map_err(&bad)?;
            for key in keys {
                let crate::expand::ParamKey::Name(referenced) = key else {
                    return Err(bad(format!(
                        "references Param({key}); named signatures have no indices"
                    )));
                };
                match signature.get(&referenced) {
                    None => {
                        return Err(bad(format!("references unknown param `{referenced}`")));
                    }
                    Some(t) if t.default.is_some() => {
                        return Err(bad(format!(
                            "references defaulted param `{referenced}`; defaults may \
                             only reference required params"
                        )));
                    }
                    Some(_) => {}
                }
            }
        }
        Ok(())
    }

    /// Registers `def` under each of its kinds.
    ///
    /// # Errors
    /// If any of those kinds is unregistered or already has a macro with
    /// `def`'s name, or the definition repeats a kind (which would otherwise
    /// self-overwrite silently).
    pub fn insert(&mut self, def: &MacroDef) -> Result<(), InsertError> {
        check_def(def)?;
        self.check_kinds(def)?;
        self.check_param_types(def)?;
        self.check_defaults(def)?;
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
            template: None,
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
        check_def(def)?;
        self.check_kinds(def)?;
        self.check_param_types(def)?;
        self.check_defaults(def)?;
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
            template: None,
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
