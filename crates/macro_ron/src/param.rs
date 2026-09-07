//! Macro parameter types: the declared type of each argument, and the set of
//! types in scope with the validators that enforce them.
//!
//! A param type is a *name* (`String`, `Color`, `Any`); the [`ParamTypeSet`]
//! maps it to a [`Validator`] that checks an argument's raw source. `macro_ron`
//! ships the domain-neutral `Any` (accepts anything) and `String` (a quoted
//! literal); domain types like `Color` are injected by the embedding crate,
//! the same way kinds are. A named param may wrap its type as
//! `Default(String, <expr>)` to declare a default expression, filled when an
//! invocation omits the argument, and any param may wrap its type as
//! `Elidable(String)` to declare that an omitted argument is filled with
//! *nothing* — the body entry holding its hole is removed before the body is
//! read, so the destination type's own serde default applies.

use std::collections::HashMap;
use std::fmt;

use ron::value::RawValue;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde::de::Deserializer;
use serde::de::EnumAccess;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;

use crate::Ident;
use crate::IdentSeed;
use crate::set::MacroSet;

/// The declared type of one macro parameter: a type *name*, resolved against
/// the [`ParamTypeSet`] in scope, an optional default expression, an
/// elidability flag, and an optional binder contract. Written in definition
/// files as a bare identifier (`params: [String, Color]`), a
/// `Default(String, <expr>)` (named signatures only), an
/// `Elidable(String)`, or a binder contract `Effect(binds: [It])` —
/// `Default` and `Elidable` are thereby reserved as spellings; a registered
/// param type by either name would be unreachable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamType {
    pub name: Ident,
    /// Raw RON source filled in for an omitted argument; its `Param(...)`
    /// holes may reference non-defaulted siblings (checked at insert).
    pub default: Option<Box<str>>,
    /// ELIDABLE: an omitted argument is filled with nothing at all. The body
    /// entry whose value is this param's hole — a struct/map `key: Param(p)`
    /// entry, or a trailing tuple element — is removed from the body text
    /// before the body is read, so the destination type never sees the key
    /// and applies its own serde default. Mutually exclusive with `default`.
    pub elidable: bool,
    /// The BINDER CONTRACT ([typed-holes delta 2]): the anaphora the macro's
    /// body wraps around this hole. `Effect(binds: [It])` = "the body
    /// introduces an It-binder over this hole", so an argument reading `It` is
    /// legal. DEFAULT = empty = no anaphora beyond the call site: an argument
    /// reading an anaphor the contract doesn't grant is a definition/call-site
    /// error. Splice hygiene becomes checked, not conventional.
    pub binds: Vec<Ident>,
}

impl ParamType {
    /// A plain (non-defaulted, no-binder-contract) param type.
    #[must_use]
    pub fn plain(name: impl Into<Ident>) -> Self {
        ParamType {
            name: name.into(),
            default: None,
            elidable: false,
            binds: Vec::new(),
        }
    }

    /// A param type that grants a binder contract (`Effect(binds: [It])`).
    #[must_use]
    pub fn binding(name: impl Into<Ident>, binds: Vec<Ident>) -> Self {
        ParamType {
            name: name.into(),
            default: None,
            elidable: false,
            binds,
        }
    }

    /// An elidable param type (`Elidable(Any)`): see [`ParamType::elidable`].
    #[must_use]
    pub fn elidable(name: impl Into<Ident>) -> Self {
        ParamType {
            name: name.into(),
            default: None,
            elidable: true,
            binds: Vec::new(),
        }
    }
}

/// The `ParamType` spellings don't fit one serde variant shape — a bare
/// ident (unit variant), `Default(T, expr)` (tuple variant), `Elidable(T)`
/// (newtype variant), and `T(binds: […])` (struct variant) all read after the
/// same tag, and serde can't peek which. So capture the source and route by
/// structural shape, then re-parse the pieces with ron.
impl<'de> Deserialize<'de> for ParamType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        let src = raw.get_ron().trim();
        // A bare ident: no arguments, so no `(`.
        let Some(open) = src.find('(') else {
            return Ok(ParamType::plain(src));
        };
        let head = src[..open].trim();
        let shape = match head {
            "Default" => Shape::Default,
            "Elidable" => Shape::Elidable,
            _ => Shape::BinderContract,
        };
        let mut de = ron::de::Deserializer::from_str(src).map_err(D::Error::custom)?;
        let value = de
            .deserialize_enum("", &[], ParamTypeVisitor { shape })
            .map_err(|e| D::Error::custom(de.span_error(e)))?;
        Ok(value)
    }
}

/// Which of the argument-bearing `ParamType` spellings the captured source is.
enum Shape {
    Default,
    Elidable,
    BinderContract,
}

struct ParamTypeVisitor {
    shape: Shape,
}

impl<'de> Visitor<'de> for ParamTypeVisitor {
    type Value = ParamType;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Default(type, expression), Elidable(type), or Type(binds: [It, …])")
    }
    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        use serde::de::Error;
        let (ident, variant) = data.variant_seed(IdentSeed)?;
        match self.shape {
            Shape::Default => variant.tuple_variant(2, DefaultArgs),
            Shape::Elidable => {
                let inner: ParamType = variant.newtype_variant()?;
                if inner.default.is_some() {
                    return Err(A::Error::custom(
                        "a param is either Elidable(...) or Default(..., expr), not both",
                    ));
                }
                if inner.elidable {
                    return Err(A::Error::custom("Elidable(...) does not nest"));
                }
                Ok(ParamType {
                    name: inner.name,
                    default: None,
                    elidable: true,
                    // An elidable param may still carry a binder contract
                    // (`Elidable(Effect(binds: [It]))`).
                    binds: inner.binds,
                })
            }
            Shape::BinderContract => {
                let binds = variant.struct_variant(&["binds"], BinderContractArgs)?;
                Ok(ParamType {
                    name: ident,
                    default: None,
                    elidable: false,
                    binds,
                })
            }
        }
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
        if inner.elidable {
            return Err(A::Error::custom(
                "a param is either Elidable(...) or Default(..., expr), not both",
            ));
        }
        let expr: Box<RawValue> = seq
            .next_element()?
            .ok_or_else(|| A::Error::custom("Default(type, expression) needs an expression"))?;
        Ok(ParamType {
            name: inner.name,
            default: Some(expr.get_ron().trim().into()),
            elidable: false,
            // A defaulted param may still carry a binder contract
            // (`Default(Effect(binds: [It]), …)`).
            binds: inner.binds,
        })
    }
}

/// Reads the `(binds: [It, That])` struct content into the granted-anaphor
/// list. `binds` is the one recognized field; the anaphor names are bare
/// identifiers, read through the same unit-variant channel as `kinds`.
struct BinderContractArgs;
impl<'de> Visitor<'de> for BinderContractArgs {
    type Value = Vec<Ident>;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("binds: [It, That, …]")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        use serde::de::Error;
        let mut binds: Option<Vec<Ident>> = None;
        while let Some(key) = map.next_key_seed(IdentSeed)? {
            if key != "binds" {
                return Err(A::Error::custom(format_args!(
                    "unknown binder-contract field `{key}`; only `binds` is recognized"
                )));
            }
            if binds.is_some() {
                return Err(A::Error::custom("duplicate `binds` field"));
            }
            binds = Some(map.next_value_seed(BareIdentList)?);
        }
        binds.ok_or_else(|| A::Error::custom("a binder contract needs a `binds` list"))
    }
}

/// A `[It, That]` list of bare identifiers (unit enum variants) — the same
/// channel `kinds: [Subtype]` reads through.
struct BareIdentList;
impl<'de> serde::de::DeserializeSeed<'de> for BareIdentList {
    type Value = Vec<Ident>;
    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_seq(self)
    }
}
impl<'de> Visitor<'de> for BareIdentList {
    type Value = Vec<Ident>;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a list of anaphor names")
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        struct BareIdent;
        impl<'de> serde::de::DeserializeSeed<'de> for BareIdent {
            type Value = Ident;
            fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
                de.deserialize_enum("", &[], self)
            }
        }
        impl<'de> Visitor<'de> for BareIdent {
            type Value = Ident;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an anaphor name")
            }
            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
                let (ident, variant) = data.variant_seed(IdentSeed)?;
                variant.unit_variant()?;
                Ok(ident)
            }
        }
        let mut out = Vec::new();
        while let Some(ident) = seq.next_element_seed(BareIdent)? {
            out.push(ident);
        }
        Ok(out)
    }
}

/// Checks an argument's raw source against a param type, with the macros in
/// scope (so an argument may itself be a macro that expands to the type).
/// `Ok(())` accepts; `Err` explains the rejection.
///
/// The `bool` is the ARGUMENT TEXT's own restriction (spec §4's textual
/// provenance): a validator reading the argument as its type must read it the
/// same way the later `param` re-read will, or a banned spelling passes
/// validation and is rejected further downstream, blamed on the macro body
/// rather than on the call site that wrote it.
pub type Validator = fn(&str, &MacroSet, bool) -> Result<(), String>;

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

    /// Registers `name` as a param type validated by parsing the argument as
    /// `T` with the macros in scope — the one mechanical pattern every domain
    /// param type shares, so callers register a type rather than re-spell the
    /// closure. `T` is typically a `SupportsMacros` grammar enum (whose
    /// `DeserializeOwned` supertrait guarantees this works), but any owned-
    /// deserializable type qualifies (`String`, `Vec<CostComponent>`).
    pub fn add_typed<T: DeserializeOwned>(&mut self, name: impl Into<Ident>) {
        self.add(name, |src, macros, restricted| {
            if restricted {
                macros.read_str_restricted::<T>(src)
            } else {
                macros.read_str::<T>(src)
            }
            .map(drop)
            .map_err(|e| e.to_string())
        });
    }

    /// The validator for `name`, if registered.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Validator> {
        self.validators.get(name).copied()
    }

    /// Whether `name` is a registered param type.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.validators.contains_key(name)
    }

    /// Every registered param type name, in no particular order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.validators.keys().map(AsRef::as_ref)
    }
}

impl Default for ParamTypeSet {
    /// The two domain-neutral built-ins: `Any` (accepts anything) and
    /// `String` (a quoted literal).
    fn default() -> Self {
        let mut set = ParamTypeSet::empty();
        set.add("Any", |_, _, _| Ok(()));
        set.add_typed::<String>("String");
        set
    }
}
