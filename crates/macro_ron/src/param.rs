//! Macro parameter types: the declared type of each argument, and the set of
//! types in scope with the validators that enforce them.
//!
//! A param type is a *name* (`String`, `Color`, `Any`); the [`ParamTypeSet`]
//! maps it to a [`Validator`] that checks an argument's raw source. `macro_ron`
//! ships the domain-neutral `Any` (accepts anything) and `String` (a quoted
//! literal); domain types like `Color` are injected by the embedding crate,
//! the same way kinds are.

use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;
use serde::de::{Deserializer, EnumAccess, VariantAccess, Visitor};

use crate::set::MacroSet;
use crate::{Ident, IdentSeed};

/// The declared type of one macro parameter: a type *name*, resolved against
/// the [`ParamTypeSet`] in scope. Written as a bare identifier in definition
/// files (`params: [String, Color]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamType(pub Ident);

impl<'de> Deserialize<'de> for ParamType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `kinds: [Subtype]` reads through (see
        // `set::kind_names`).
        struct TypeName;
        impl<'de> Visitor<'de> for TypeName {
            type Value = ParamType;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a parameter type name")
            }
            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                let (ident, variant) = data.variant_seed(IdentSeed)?;
                variant.unit_variant()?;
                Ok(ParamType(ident))
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
