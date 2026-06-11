//! Macro parameter types: the declared type of each argument, and the set of
//! types in scope with the validators that enforce them.
//!
//! A param type is a *name* (`String`, `Color`, `Any`); the [`ParamTypeSet`]
//! maps it to a [`Validator`] that checks an argument's raw source. `macro_ron`
//! ships the domain-neutral `Any` (accepts anything) and `String` (a quoted
//! literal); domain types like `Color` are injected by the embedding crate,
//! the same way kinds are. A named param may wrap its type as
//! `Default(String, <expr>)` to declare a default expression, filled when an
//! invocation omits the argument.

use std::collections::HashMap;
use std::fmt;

use ron::value::RawValue;
use serde::Deserialize;
use serde::de::Deserializer;
use serde::de::EnumAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;

use crate::Ident;
use crate::IdentSeed;
use crate::set::MacroSet;

/// The declared type of one macro parameter: a type *name*, resolved against
/// the [`ParamTypeSet`] in scope, plus an optional default expression.
/// Written in definition files as a bare identifier (`params: [String,
/// Color]`) or, in named signatures, `Default(String, <expr>)` — `Default` is
/// thereby reserved as a spelling; a registered param type by that name would
/// be unreachable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamType {
    pub name: Ident,
    /// Raw RON source filled in for an omitted argument; its `Param(...)`
    /// holes may reference non-defaulted siblings (checked at insert).
    pub default: Option<Box<str>>,
}

impl ParamType {
    /// A plain (non-defaulted) param type.
    #[must_use]
    pub fn plain(name: impl Into<Ident>) -> Self {
        ParamType {
            name: name.into(),
            default: None,
        }
    }
}

impl<'de> Deserialize<'de> for ParamType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `kinds: [Subtype]` reads through (see
        // `set::kind_names`). `Default(...)` arrives as a tuple variant.
        struct TypeName;
        impl<'de> Visitor<'de> for TypeName {
            type Value = ParamType;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a parameter type name or Default(type, expression)")
            }
            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                let (ident, variant) = data.variant_seed(IdentSeed)?;
                if ident == "Default" {
                    return variant.tuple_variant(2, DefaultArgs);
                }
                variant.unit_variant()?;
                Ok(ParamType {
                    name: ident,
                    default: None,
                })
            }
        }
        struct DefaultArgs;
        impl<'de> Visitor<'de> for DefaultArgs {
            type Value = ParamType;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("Default(type, expression)")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                use serde::de::Error;
                let inner: ParamType = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::custom("Default(type, expression) needs a type"))?;
                if inner.default.is_some() {
                    return Err(A::Error::custom("Default(...) does not nest"));
                }
                let expr: Box<RawValue> = seq.next_element()?.ok_or_else(|| {
                    A::Error::custom("Default(type, expression) needs an expression")
                })?;
                Ok(ParamType {
                    name: inner.name,
                    default: Some(expr.get_ron().trim().into()),
                })
            }
        }
        deserializer.deserialize_enum("", &[], TypeName)
    }
}

/// Checks an argument's raw source against a param type, with the macros in
/// scope (so an argument may itself be a macro that expands to the type).
/// `Ok(())` accepts; `Err` explains the rejection.
pub type Validator = fn(&str, &MacroSet) -> Result<(), String>;

/// The param types in scope, each with the validator that enforces it.
#[derive(Debug, Clone)]
pub struct ParamTypeSet {
    validators: HashMap<Ident, Validator>,
}

impl ParamTypeSet {
    /// An empty set — usually you want [`ParamTypeSet::default`], which
    /// registers the built-ins.
    #[must_use]
    pub fn empty() -> Self {
        ParamTypeSet {
            validators: HashMap::new(),
        }
    }

    /// Registers `name` with `validator`, replacing any previous entry.
    pub fn add(&mut self, name: impl Into<Ident>, validator: Validator) {
        self.validators.insert(name.into(), validator);
    }

    /// The validator for `name`, if registered.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Validator> { self.validators.get(name).copied() }

    /// Whether `name` is a registered param type.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool { self.validators.contains_key(name) }
}

impl Default for ParamTypeSet {
    /// The two domain-neutral built-ins: `Any` (accepts anything) and
    /// `String` (a quoted literal).
    fn default() -> Self {
        let mut set = ParamTypeSet::empty();
        set.add("Any", |_, _| Ok(()));
        set.add("String", |src, macros| {
            macros
                .read_str::<String>(src)
                .map(drop)
                .map_err(|e| e.to_string())
        });
        set
    }
}
