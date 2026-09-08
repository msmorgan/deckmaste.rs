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
//! `Param(cost)` holes. When positional argument reading is enabled, a named
//! signature may also be invoked in its declaration order. A named param may
//! carry a default — `{"template": Default(String, Param(name))}` — filled (and validated)
//! when the invocation omits it; defaults may reference only always-supplied
//! params of the same signature. Either shape of param may instead be
//! declared `Elidable(String)`: an omitted argument is then filled with
//! nothing, and the body entry holding its hole is dropped before the body is
//! read, so the destination's own serde default applies. Elidable positional
//! params must be the trailing ones, since a call supplies a prefix.
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
use crate::frames::FrameSpec;
use crate::kind::KindSet;
use crate::param::ParamType;
use crate::param::ParamTypeSet;

/// A macro's parameter signature, whose shape decides the invocation
/// grammar: positional (`M(a, b)`, holes `Param(0)`) or named
/// (`M(x: a, y: b)`, holes `Param(x)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Params {
    Positional(Vec<ParamType>),
    /// Named parameters in declaration order. The order is observable when
    /// the consumer enables positional application for named signatures.
    Named(Vec<(Ident, ParamType)>),
}

impl Default for Params {
    fn default() -> Self {
        Params::Positional(vec![])
    }
}

impl Params {
    /// Whether every parameter has a default, so the macro can be invoked
    /// with no arguments — and, for a named signature, by its bare name.
    /// Only named signatures qualify: positional defaults are never filled
    /// (their arity is required), so a bare positional name reads as the
    /// already-supported zero-arg unit form, not a defaulted call.
    ///
    /// An all-`Elidable(...)` signature deliberately does NOT qualify: the
    /// native struct-variant grammar these mirror has no bare spelling
    /// either (`Drawn` is a parse error where `Drawn()` reads), so granting
    /// one would invent an author form nothing round-trips back into.
    pub(crate) fn all_defaulted(&self) -> bool {
        match self {
            Params::Named(signature) => signature.iter().all(|(_, ty)| ty.default.is_some()),
            Params::Positional(_) => false,
        }
    }

    /// How many leading positional params must be supplied: elidable ones
    /// form a trailing run (checked at insert), so a call's arity may be
    /// anything from this up to the full list length.
    pub(crate) fn required_positional(types: &[ParamType]) -> usize {
        types.iter().take_while(|ty| !ty.elidable).count()
    }
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
                let mut params = Vec::new();
                while let Some((name, param)) = map.next_entry()? {
                    if params.iter().any(|(declared, _)| declared == &name) {
                        return Err(serde::de::Error::custom(format_args!(
                            "duplicate parameter `{name}`"
                        )));
                    }
                    params.push((name, param));
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
/// Consumers may attach their own typed metadata through `Metadata`. The
/// default unit payload keeps existing definition readers unchanged; a
/// consumer-specific reader can instead request `MacroDef<ItsMetadata>`.
/// The metadata is deliberately opaque to `macro_ron`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Macro")]
pub struct MacroDef<Metadata = ()> {
    pub name: Ident,
    /// The kinds this macro can expand to, by position name; written as
    /// bare identifiers in definition files (`kinds: [Subtype]`).
    #[serde(deserialize_with = "kind_names")]
    pub kinds: Vec<Ident>,
    #[serde(default)]
    pub params: Params,
    /// Optional human-readable rules-text template (metadata): `~` = self,
    /// `${i}` = the i-th positional arg, `${name}` = the named arg
    /// (single-brace `{…}` is a literal game symbol). Consumed by the
    /// card-text renderer.
    #[serde(default)]
    pub template: Option<String>,
    /// Optional plural surface of `template`'s head noun (metadata): "Merfolk"
    /// for a macro whose regular-rules plural (see
    /// `deckmaste_legacy_render::template::plural::pluralize`) is wrong or
    /// ambiguous. Absent for the common case, where the regular rules
    /// suffice — every existing definition file omits this field, so it
    /// reads as `None` unchanged.
    #[serde(default)]
    pub plural: Option<String>,
    /// English renderings of this macro, with the guard that decides which
    /// applies when more than one is defined — see
    /// [`FrameSpec`](crate::frames::FrameSpec). Defaults to empty, so
    /// every macro file predating this field still loads unchanged. A bare
    /// string in the list (`frames: ["draw <Param(1)> cards"]`) is sugar for
    /// an unguarded frame; `FrameSpec`'s own `Deserialize` impl resolves the
    /// sugar, so no special handling is needed here.
    #[serde(default)]
    pub frames: Vec<FrameSpec>,
    /// Consumer-owned declaration metadata.
    #[serde(default)]
    pub metadata: Metadata,
    /// Raw RON source with `Param(...)` holes.
    #[serde(deserialize_with = "raw_body")]
    pub(crate) body: Box<str>,
}

fn raw_body<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Box<str>, D::Error> {
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    Ok(body_text(&raw))
}

/// The stored text of a body captured as a raw RON value: the value's own
/// source, trimmed.
///
/// Shared with [`crate::frames::ConstructorFrames`]'s optional body, which is
/// the same thing in a different schema — a term with `Param(...)` leaves,
/// authored bare (`body: By(Param(0), GainLife(Param(1)))`), never quoted.
/// Both go through this one function so the two schemas cannot disagree on
/// what "the body's text" is; a second `get_ron().trim()` elsewhere would be
/// a copy of the contract rather than a use of it.
pub(crate) fn body_text(raw: &RawValue) -> Box<str> {
    raw.get_ron().trim().into()
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

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a kind name")
        }

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

impl<Metadata> MacroDef<Metadata> {
    /// This definition with its consumer metadata dropped: the same name,
    /// kinds, signature, template and body, carrying the unit payload
    /// [`MacroSet`] stores.
    ///
    /// The metadata is opaque to this crate, so a consumer that reads
    /// declarations under its OWN metadata type — or under an ignoring one,
    /// which is how a second consumer of the same declaration files reads
    /// them — has no other way to register what it read: the registry's
    /// entries are `MacroDef<()>`, and `body` is crate-private, so the value
    /// cannot be rebuilt from outside.
    #[must_use]
    pub fn erase_metadata(&self) -> MacroDef {
        MacroDef {
            name: self.name,
            kinds: self.kinds.clone(),
            params: self.params.clone(),
            template: self.template.clone(),
            plural: self.plural.clone(),
            frames: self.frames.clone(),
            metadata: (),
            body: self.body.clone(),
        }
    }

    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// The consumer-owned metadata payload.
    #[must_use]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    #[must_use]
    pub fn template(&self) -> Option<&str> {
        self.template.as_deref()
    }

    #[must_use]
    pub fn plural(&self) -> Option<&str> {
        self.plural.as_deref()
    }

    #[must_use]
    pub fn frames(&self) -> &[FrameSpec] {
        &self.frames
    }

    /// The body's OUTERMOST identifier, if it has one — the thing the body
    /// constructs. `None` for a scalar, a list, or an unnamed struct. Shallow
    /// by construction; see
    /// [`leading_invoked_name`](crate::expand::leading_invoked_name).
    #[must_use]
    pub fn body_head(&self, macros: &MacroSet) -> Option<Ident> {
        crate::expand::leading_invoked_name(self.body(), macros.options())
    }

    /// Every `Param(...)` hole in the body, each rendered the way it is
    /// addressed (`0`, `cost`), in walk order. Lets a caller check a body
    /// against its declared signature in both directions.
    ///
    /// # Errors
    /// If the body is not readable as RON.
    pub fn body_param_keys(&self, macros: &MacroSet) -> Result<Vec<String>, String> {
        let mut keys = Vec::new();
        crate::expand::collect_param_keys(self.body(), macros.options(), &mut keys)?;
        Ok(keys.iter().map(ToString::to_string).collect())
    }
}

/// Why a macro couldn't be registered.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InsertError {
    /// Two macros (or declarations) of one kind tried to use the same name.
    #[error("a {kind} macro named `{name}` is already defined")]
    Duplicate { kind: Ident, name: Ident },
    /// A definition named a kind no [`Kind`](crate::Kind) was registered for.
    #[error("macro `{name}` declares unregistered kind `{kind}`")]
    UnknownKind { kind: Ident, name: Ident },
    /// A definition named a param type no validator was registered for.
    #[error("macro `{name}` declares unregistered param type `{type_name}`")]
    UnknownParamType { type_name: Ident, name: Ident },
    /// A definition's name can't be invoked: macros are invoked as bare
    /// identifiers, so the name must be one.
    #[error("macro name `{name}` is not a bare identifier; macros are invoked bare")]
    InvalidName { name: Ident },
    /// A meta-macro (kind `Macro`) declared positional params: hole
    /// indices would be ambiguous between the meta's frame and the
    /// produced definition's own params.
    #[error(
        "meta-macro `{name}` must use named params (indices are \
         ambiguous between the meta and its produced definition)"
    )]
    MetaParamsPositional { name: Ident },
    /// A positional signature declared a `Default(...)` param; defaults are
    /// named-only (trailing-default arity is out of scope).
    #[error(
        "macro `{name}` declares a positional param with a default; \
         defaults are named-only"
    )]
    PositionalDefault { name: Ident },
    /// A positional signature's `Elidable(...)` params don't form a trailing
    /// run: a call supplies a prefix of the list, so an elidable param before
    /// a required one could never actually be omitted.
    #[error(
        "macro `{name}` declares an `Elidable(...)` positional param \
         before a required one; elidable positional params must be \
         the trailing ones"
    )]
    ElidableNotTrailing { name: Ident },
    /// A positional signature's ONE param is `Elidable(...)`. A one-param
    /// call reads through the newtype channel, which has no zero-argument
    /// spelling, so the short form the marker promises could never be
    /// invoked — the declaration would silently mean nothing.
    #[error(
        "macro `{name}`'s only positional param is `Elidable(...)`, \
         but a one-param call has no zero-argument spelling; make it \
         required, or give the signature a required param first"
    )]
    LoneElidablePositional { name: Ident },
    /// A named param's default expression is unusable: unparseable, or it
    /// references a param that is missing, defaulted, or index-addressed.
    #[error("macro `{name}` param `{param}` default: {reason}")]
    BadDefault {
        name: Ident,
        param: Ident,
        reason: String,
    },
    /// A definition's body can't be elided the way its `Elidable(...)` params
    /// promise — a comment between its entries, chiefly. Caught here so it
    /// fails at load, not the first time a card writes the short form.
    #[error("macro `{name}`: {reason}")]
    UnelidableBody { name: Ident, reason: String },
    /// A definition's body invokes a macro whose expansion eventually
    /// invokes it again — directly (a self-reference) or through a chain of
    /// other macros. Caught here so it fails at load, not only at runtime
    /// once the `MAX_DEPTH` expansion cap trips.
    #[error(
        "macro `{name}` forms an expansion cycle: {}",
        path.iter().map(Ident::as_str).collect::<Vec<_>>().join(" -> ")
    )]
    Cycle { name: Ident, path: Vec<Ident> },
}

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

/// Builds the nullary `MacroDef` shared by [`MacroSet::declare`] and
/// [`MacroSet::redeclare`]: `name` is a parameter-less macro of `kind` whose
/// body is `declaration` (trimmed), verbatim.
fn decl_def(kind: &str, name: Ident, declaration: &str) -> MacroDef {
    MacroDef {
        name,
        kinds: vec![kind.into()],
        params: Params::default(),
        template: None,
        plural: None,
        frames: Vec::new(),
        metadata: (),
        body: declaration.trim().into(),
    }
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
    /// Whether a field a constructor does not declare is refused rather than
    /// ignored — see [`MacroSet::denying_unknown_fields`].
    deny_unknown_fields: bool,
    /// Whether a constructor or named-signature macro may be applied
    /// positionally — see
    /// [`MacroSet::reading_positional_arguments`].
    positional_arguments: bool,
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
            deny_unknown_fields: false,
            positional_arguments: false,
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

    /// Refuses a named field the constructor at that position does not
    /// declare, naming both, instead of letting serde ignore it.
    ///
    /// serde's own default is to skip an unrecognized key, so a misspelled or
    /// retired argument reads as an absent one: `amount:` written for a
    /// `quantity` field produced a count-less removal, and a `conferral:`
    /// argument left over from a retired signature passed unnoticed. Off by
    /// default, because a consumer whose files carry consumer-private keys
    /// (the declaration file's metadata half) depends on the skipping.
    #[must_use]
    pub fn denying_unknown_fields(mut self) -> Self {
        self.deny_unknown_fields = true;
        self
    }

    /// Whether unknown fields are refused — see
    /// [`denying_unknown_fields`](Self::denying_unknown_fields).
    pub(crate) fn denies_unknown_fields(&self) -> bool {
        self.deny_unknown_fields
    }

    /// Accepts arguments in their declared order. This covers constructors —
    /// `Hybrid(Generic(1), Red)` for `Hybrid(left: Generic(1), right: Red)` —
    /// and named-signature macros such as `hasType(Creature)` for
    /// `hasType(type: Creature)`.
    ///
    /// The two forms are never mixed: ron's own value scanner refuses
    /// `C(a, b: c)` while deciding which one is written. A constructor of ONE
    /// field is exempt and keeps its binder: inside a newtype variant ron
    /// reads `(x)` as a newtype rather than a one-element tuple, and its
    /// `handle_any_struct` then turns a bare identifier into `visit_unit`,
    /// discarding the very name the argument is (`ron` 0.12 `de/mod.rs`,
    /// `deserialize_any` → `handle_any_struct` → `StructType::Unit`).
    ///
    /// Off by default: it changes which visitor method a struct position
    /// reaches, so a consumer opts in.
    #[must_use]
    pub fn reading_positional_arguments(mut self) -> Self {
        self.positional_arguments = true;
        self
    }

    /// Whether a constructor or named-signature macro may be applied
    /// positionally — see
    /// [`reading_positional_arguments`](Self::reading_positional_arguments).
    pub(crate) fn reads_positional_arguments(&self) -> bool {
        self.positional_arguments
    }

    pub(crate) fn options(&self) -> &ron::Options {
        &self.options
    }

    /// The macro `name` for positions of the type named `kind`, if defined.
    #[must_use]
    pub fn get(&self, kind: &str, name: &str) -> Option<&MacroDef> {
        self.macros.get(kind)?.get(name)
    }

    /// Iterate every registered macro as `(kind, def)`, where `kind` is the
    /// position kind it is registered under — a multi-kind macro yields once
    /// per kind. Order is unspecified (it follows the backing hash maps).
    /// The reverse-index builder consumes this to compile templates by kind.
    pub fn iter(&self) -> impl Iterator<Item = (&Ident, &MacroDef)> {
        self.macros
            .iter()
            .flat_map(|(kind, named)| named.values().map(move |def| (kind, def)))
    }

    /// See [`Kind::literal_wrapper`](crate::Kind::literal_wrapper). The
    /// second element is the wrapper's binder name when it is a struct
    /// variant (`Kind::literal_binder`).
    pub(crate) fn literal_wrapper(
        &self,
        position: &str,
    ) -> Option<(&'static str, Option<&'static str>)> {
        let kind = self.kinds.get(position)?;
        Some((kind.literal?, kind.literal_binder))
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

    /// Whether `ident` keeps native candidacy at `position` under a
    /// restricted read (spec §4). Two ways to say yes: `position` is not a
    /// registered macroable kind at all — closed atoms deliberately outside
    /// the macro system (`Cmp`, the phase/step enums, `FaceLayout`) keep
    /// parsing natively — or the kind names `ident` in its own carve-out
    /// list ([`Kind::natively_spellable`](crate::Kind::natively_spellable)).
    pub(crate) fn natively_spellable(&self, position: &str, ident: &str) -> bool {
        self.kinds
            .get(position)
            .is_none_or(|kind| kind.is_natively_spellable(ident))
    }

    /// Whether some macro expands to the struct named `name`, i.e. whether
    /// that parse position needs macro interception.
    pub(crate) fn expands_to_struct(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Whether any macro at `position` may be invoked by its bare name — a
    /// named signature whose parameters are all defaulted. Gates the
    /// bare-invocation pre-scan so only positions that host such a macro pay
    /// the capture cost.
    pub(crate) fn has_bare_invocable(&self, position: &str) -> bool {
        self.macros
            .get(position)
            .is_some_and(|named| named.values().any(|def| def.params.all_defaulted()))
    }

    /// Whether `position` hosts a macro with a named signature. Such a
    /// position is captured when positional argument reading is enabled so
    /// the invocation's argument-list shape can be classified before ron
    /// commits its one-shot `VariantAccess` to map or sequence syntax.
    pub(crate) fn has_named_signature(&self, position: &str) -> bool {
        self.macros.get(position).is_some_and(|named| {
            named
                .values()
                .any(|def| matches!(def.params, Params::Named(_)))
        })
    }

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
            Params::Named(types) => types.iter().try_for_each(|(_, t)| check(t.name)),
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
    ///
    /// The `Elidable(...)` params get their own load-time pre-flight here: the
    /// body-side elision walk is run for the shortest call this signature
    /// admits, so a body it could not cut (a comment between its entries) is
    /// refused at load rather than at the first card that writes the short
    /// form. See [`crate::expand::check_elidable_entries`].
    fn check_defaults(&self, def: &MacroDef) -> Result<(), InsertError> {
        let elidable: Vec<crate::expand::ParamKey> = match &def.params {
            Params::Positional(types) => types
                .iter()
                .enumerate()
                .filter(|(_, ty)| ty.elidable)
                .map(|(i, _)| crate::expand::ParamKey::Index(i))
                .collect(),
            Params::Named(signature) => signature
                .iter()
                .filter(|(_, ty)| ty.elidable)
                .map(|(name, _)| crate::expand::ParamKey::Name(*name))
                .collect(),
        };
        if !elidable.is_empty() {
            crate::expand::check_elidable_entries(def.body(), &elidable, &self.options).map_err(
                |reason| InsertError::UnelidableBody {
                    name: def.name,
                    reason,
                },
            )?;
        }
        let signature = match &def.params {
            Params::Positional(types) => {
                if types.iter().any(|t| t.default.is_some()) {
                    return Err(InsertError::PositionalDefault { name: def.name });
                }
                // Elidable positional params must be a contiguous suffix,
                // mirroring the derive's own trailing-default rule for the
                // native tuple variants these signatures shadow.
                if types[Params::required_positional(types)..]
                    .iter()
                    .any(|t| !t.elidable)
                {
                    return Err(InsertError::ElidableNotTrailing { name: def.name });
                }
                // A ONE-param positional call reads through the newtype
                // channel (`read_args`), which has no zero-argument spelling,
                // so an elidable marker there promises a short form nothing
                // could ever invoke. Refused at load rather than left to
                // silently mean nothing. Two or more params read through the
                // tuple channel, which does admit `M()`, so an all-elidable
                // signature is fine from there up.
                if matches!(types.as_slice(), [only] if only.elidable) {
                    return Err(InsertError::LoneElidablePositional { name: def.name });
                }
                return Ok(());
            }
            Params::Named(signature) => signature,
        };
        for (param, ty) in signature {
            let param = *param;
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
                match signature
                    .iter()
                    .find(|(name, _)| *name == referenced)
                    .map(|(_, ty)| ty)
                {
                    None => {
                        return Err(bad(format!("references unknown param `{referenced}`")));
                    }
                    Some(t) if t.default.is_some() || t.elidable => {
                        return Err(bad(format!(
                            "references omittable param `{referenced}`; defaults may \
                             only reference required params"
                        )));
                    }
                    Some(_) => {}
                }
            }
        }
        Ok(())
    }

    /// Rejects `def` if its body forms an expansion cycle: invoking a macro
    /// whose own body (directly, or through a chain of further such
    /// top-level invocations) invokes `def` again. No graph persists
    /// between calls — every already-registered macro was accepted by this
    /// same check, so it's already acyclic; only the edges the new `def`
    /// introduces need walking.
    ///
    /// Resolution is deliberately conservative in two ways. First, only a
    /// body's own *outermost* value is examined ([`leading_invoked_name`]) —
    /// never its nested arguments, which may be ordinary bare-identifier
    /// leaf data (a `CounterRef`-style reference) indistinguishable, in raw
    /// text, from a macro invocation. Every real macro-to-macro chain in
    /// this codebase is written as "my body's entire value is a call to the
    /// next macro", so this still catches every real chain. Second, a
    /// resolved name only counts as an edge if it names `def` itself or a
    /// macro already registered under one of `def`'s own kinds; anything
    /// else resolves to nothing and is silently skipped. A missed edge is
    /// at worst a cycle caught later by the runtime `MAX_DEPTH` cap
    /// (unchanged from before this check existed); a spurious one would
    /// reject a legitimate macro outright.
    fn check_cycles(&self, def: &MacroDef) -> Result<(), InsertError> {
        let mut stack = vec![def.name];
        let mut on_stack = std::collections::HashSet::from([def.name]);
        let mut cur = def;
        loop {
            let Some(name) = crate::expand::leading_invoked_name(cur.body(), &self.options) else {
                return Ok(());
            };
            // A body ident naming a native variant at one of `def`'s kinds is
            // not an invocation at all: a definition body is free vocabulary,
            // so an identity macro whose body spells its own variant (`Any`
            // at kind `Filter` with body `Any`) is not a self-cycle. Without
            // this, the identity macros spec §5 requires are unregistrable.
            if def.kinds.iter().any(|kind| {
                self.kinds
                    .get(kind)
                    .is_some_and(|k| k.variants.contains(&name.as_str()))
            }) {
                return Ok(());
            }
            let target = if name == def.name {
                def
            } else {
                let Some(target) = def
                    .kinds
                    .iter()
                    .find_map(|kind| self.macros.get(kind).and_then(|named| named.get(&name)))
                else {
                    return Ok(());
                };
                target
            };
            if on_stack.contains(&target.name) {
                let mut path = stack;
                path.push(target.name);
                return Err(InsertError::Cycle {
                    name: def.name,
                    path,
                });
            }
            stack.push(target.name);
            on_stack.insert(target.name);
            cur = target;
        }
    }

    /// Validates `def` and registers it under each of its kinds. The shared
    /// body of [`insert`](Self::insert) and [`replace`](Self::replace): the two
    /// differ only in whether a same-kind name collision is rejected
    /// (`allow_overwrite == false`) or silently overwritten
    /// (`allow_overwrite == true`).
    ///
    /// # Errors
    /// If `def` fails validation, any of its kinds is unregistered, or — when
    /// `allow_overwrite` is `false` — a kind already has a macro named
    /// `def.name` or `def` repeats a kind (which would otherwise self-overwrite
    /// silently).
    fn register(&mut self, def: &MacroDef, allow_overwrite: bool) -> Result<(), InsertError> {
        check_def(def)?;
        self.check_kinds(def)?;
        self.check_param_types(def)?;
        self.check_defaults(def)?;
        self.check_cycles(def)?;
        if !allow_overwrite {
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
        }
        for &kind in &def.kinds {
            self.macros
                .entry(kind)
                .or_default()
                .insert(def.name, def.clone());
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
        self.register(def, false)
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
        self.insert(&decl_def(kind, name, declaration))
    }

    /// Registers `def` under each of its kinds, overriding same-kind
    /// entries already in scope. Layer-to-layer overriding is legal — last
    /// layer wins — so the caller is responsible for rejecting duplicates
    /// *within* one layer.
    ///
    /// # Errors
    /// If any of `def`'s kinds is unregistered.
    pub fn replace(&mut self, def: &MacroDef) -> Result<(), InsertError> {
        self.register(def, true)
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
        self.replace(&decl_def(kind, name, declaration))
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
        self.read_str_with(source, false)
    }

    /// Reads a RON document as **restricted author vocabulary** (spec §4): at
    /// every registered kind, the kind's own variant identifiers lose native
    /// candidacy and must route through a macro of the same name. Macro
    /// definition text reached by expansion stays free, and an argument keeps
    /// the restriction of the text it was written in.
    ///
    /// This is the entry the card/token containers read through; the ~230
    /// other [`read_str`](Self::read_str) callers are unaffected, because
    /// restriction is opt-in at the entry and never a global default.
    ///
    /// # Errors
    /// As [`read_str`](Self::read_str), plus identifiers that name a variant
    /// of a registered kind with no macro of that name.
    pub fn read_str_restricted<T: DeserializeOwned>(
        &self,
        source: &str,
    ) -> ron::error::SpannedResult<T> {
        self.read_str_with(source, true)
    }

    fn read_str_with<T: DeserializeOwned>(
        &self,
        source: &str,
        restricted: bool,
    ) -> ron::error::SpannedResult<T> {
        let read = crate::expand::ReadCtx::new(self);
        let mut deserializer = ron::de::Deserializer::from_str_with_options(source, &self.options)?;
        let macro_aware = if restricted {
            crate::expand::MacroAware::new_restricted(&mut deserializer, &read)
        } else {
            crate::expand::MacroAware::new(&mut deserializer, &read)
        };
        let value = T::deserialize(macro_aware).map_err(|e| deserializer.span_error(e))?;
        deserializer.end().map_err(|e| deserializer.span_error(e))?;
        Ok(value)
    }
}
