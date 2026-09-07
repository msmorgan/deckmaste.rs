//! The macro-aware deserialization layer: a wrapper around the RON
//! deserializer that expands macro invocations wherever they stand in for a
//! real value, so the data types themselves stay plain serde derives.
//!
//! The core move, for an enum position:
//!
//! > if the next identifier is not a real variant, expand the macro by that
//! > name and read this position again from the expansion.
//!
//! Variant lists arrive with every `deserialize_enum` call, so unknown
//! variants are caught exactly where serde would have errored. Struct
//! positions don't carry that information — a wrong leading identifier dies
//! inside the parser — so positions whose type some macro expands to
//! (registered by kind in [`MacroSet`]) are captured as raw source first and
//! checked for a leading macro name.
//!
//! Expanding a macro pushes a [`Frame`] holding the invocation's arguments
//! (as raw source text); the stack of macros in flight is the call stack of
//! body re-reads. While a frame is current, every value position watches for
//! the reserved name `Param`: `Param(n)` resolves to the n-th argument,
//! re-read at the position the hole occupies. Because `Param` is only
//! recognized where a value is expected, string literals mentioning it are
//! untouched. A nested macro invocation's own arguments (e.g.
//! `PowerAndToughness(Up(Param(0)), Up(Param(1)))` inside a body) are eagerly
//! pre-substituted against the *caller's* current frame the moment they're
//! captured — see [`read_args`] — so a body can forward its own params into a
//! macro it invokes; holes the caller's frame can't resolve pass through
//! unchanged, because they belong to the invoked macro's own frame once it's
//! pushed.
//!
//! Captured fragments and invocation arguments are borrowed subslices of
//! the text they were read from — the document, a macro body, or a spliced
//! string — never copies. The one place new text is built is `Param`
//! substitution inside untagged content and eager nested-argument
//! forwarding; those splices are owned by the read-long [`ReadCtx`], because
//! visitors are entitled to borrow from their input, and drop when the read
//! finishes.

use std::cell::Cell;
use std::fmt;
use std::ops::Range;

use elsa::FrozenVec;
use ron::value::RawValue;
use serde::Deserialize;
use serde::de::DeserializeSeed;
use serde::de::Deserializer;
use serde::de::EnumAccess;
use serde::de::Error as _;
use serde::de::IgnoredAny;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;
use serde::de::value::StrDeserializer;

use crate::Ident;
use crate::IdentSeed;
use crate::param::ParamType;
use crate::set::MacroDef;
use crate::set::MacroSet;
use crate::set::Params;

/// ron's private raw-value marker: `RawValue` deserializes through
/// `deserialize_newtype_struct` with this name. ron doesn't export it, so
/// it is pinned here (and by `raw_value_token_drift_pin` in the tests).
pub(crate) const RAW_VALUE_TOKEN: &str = "$ron::private::RawValue";

/// The reserved name for the generic list-splice ([typed-holes delta 5]):
/// `Splice(X)` at a `Vec` position inlines the list `X` resolves to into the
/// surrounding list — the one rule generalizing the old ad-hoc
/// `Cost(Param(i))` / `Modification::Several` flatten idioms. Legal ONLY at a
/// sequence position; `Quote` (delta 4) defers a param through a two-level
/// meta-macro.
pub(crate) const SPLICE: &str = "Splice";

/// The reserved name for the two-level meta-macro stage marker
/// ([typed-holes delta 4]): `Quote(Param(i))` in a meta-macro's body emits a
/// literal `Param(i)` into the PRODUCED definition (deferred to that
/// definition's own frame) instead of resolving eagerly against the meta's
/// frame. The one construct that lets a meta forward a param into a nested
/// macro invocation.
pub(crate) const QUOTE: &str = "Quote";

/// The read-long half of the context: the macros in scope, and the strings
/// spliced together while reading one document, so they can be borrowed
/// like the input is.
pub(crate) struct ReadCtx<'m> {
    macros: &'m MacroSet,
    /// An append-only arena: the [`FrozenVec`] hands out borrows that
    /// outlive later pushes, because the boxed strs never move even when
    /// its spine reallocates.
    splices: FrozenVec<Box<str>>,
}

impl<'m> ReadCtx<'m> {
    pub(crate) fn new(macros: &'m MacroSet) -> Self {
        ReadCtx {
            macros,
            splices: FrozenVec::new(),
        }
    }

    fn splice(&self, s: String) -> &str {
        self.splices.push_get(s.into_boxed_str())
    }
}

/// A macro expansion in flight: the invocation's arguments, which the body's
/// `Param(...)` holes resolve against.
struct Frame<'de> {
    /// The macro's name, for error messages.
    name: Ident,
    args: FrameArgs<'de>,
    /// The `Elidable(...)` params this invocation omitted. They have no
    /// argument text at all: [`elide_body`] removes the body entries their
    /// holes stand in, so the destination type never sees those keys and
    /// applies its own defaults. Empty for every non-elidable signature, which
    /// is what keeps expansion byte-identical for defs predating the form.
    elided: Vec<ParamKey>,
}

/// One invocation argument: its raw source, and whether that source is
/// restricted (author-facing) text.
///
/// Restriction rides on the ARGUMENT, not on the frame, because it follows the
/// text's provenance rather than the frame the text is substituted into: a
/// card-written argument stays restricted inside a free macro body, and — since
/// [`forward_arg`] carries the flag into the nested invocation's own argument —
/// stays restricted however many macros forward it on.
///
/// Provenance is tracked per argument, not per byte. An argument a body
/// assembles out of its own text and a forwarded hole (`Up(Param(0))`) is
/// therefore restricted as a whole: the author's half decides, since the
/// alternative is laundering it. Definition text that gets over-restricted this
/// way fails loudly at the identity-macro gate; it can never pass silently.
#[derive(Clone, Copy)]
pub(crate) struct Arg<'de> {
    text: &'de str,
    restricted: bool,
}

/// Invocation arguments, as raw source each, shaped like the signature.
#[derive(Clone)]
pub(crate) enum FrameArgs<'de> {
    Positional(Vec<Arg<'de>>),
    /// Name, argument, and whether the argument is a filled-in default
    /// (excluded from synthesized invocations so short calls round-trip).
    Named(Vec<(Ident, Arg<'de>, bool)>),
}

/// What a `Param(...)` hole addresses: `Param(0)` or `Param(cost)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub(crate) enum ParamKey {
    #[serde(untagged)]
    Index(usize),
    #[serde(untagged)]
    Name(Ident),
}

impl fmt::Display for ParamKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamKey::Index(index) => write!(f, "{index}"),
            ParamKey::Name(name) => write!(f, "{name}"),
        }
    }
}

/// What every layer of the wrapper carries: the read-long [`ReadCtx`] and
/// the innermost expansion frame, if a macro body is being read.
#[derive(Clone, Copy)]
struct Ctx<'de, 'f> {
    read: &'de ReadCtx<'de>,
    frame: Option<&'f Frame<'de>>,
    /// How many expansions deep this position is; bounded by [`MAX_DEPTH`].
    depth: usize,
    /// Whether the text being read here is restricted author vocabulary, in
    /// which case a registered kind's own variant idents are suppressed as
    /// native candidates and must route through identity macros instead
    /// (spec §4). Default `false`: restriction is opt-in at the entry point.
    restricted: bool,
}

/// Macros aren't a programming language: there is no recursion, so any chain
/// of expansions deeper than this is a definition cycle, reported as an
/// error rather than run into a stack overflow.
const MAX_DEPTH: usize = 64;

impl<'de> Ctx<'de, '_> {
    /// Resolves a `Param(...)` hole against the current frame, with the
    /// restriction its own text was captured under ([`Arg`]). A filled-in
    /// default is the definition's own text, so it is free unless the default
    /// expression spliced an argument of the invocation into itself.
    fn param(&self, key: ParamKey) -> Result<(&'de str, bool), String> {
        let frame = self
            .frame
            .ok_or_else(|| format!("Param({key}) outside any macro expansion"))?;
        let arg = match (&frame.args, key) {
            (FrameArgs::Positional(args), ParamKey::Index(index)) => {
                args.get(index).map(|arg| (arg.text, arg.restricted))
            }
            (FrameArgs::Named(args), ParamKey::Name(name)) => args
                .iter()
                .find(|(k, _, _)| *k == name)
                .map(|(_, arg, _)| (arg.text, arg.restricted)),
            _ => None,
        };
        arg.ok_or_else(|| {
            if frame.elided.contains(&key) {
                // The hole survived `elide_body`: it doesn't stand at a
                // droppable body entry, so there is nothing to fall back to.
                format!(
                    "macro `{}` omitted elidable param `{key}`, but its body uses \
                     it somewhere that can't be elided",
                    frame.name,
                )
            } else {
                format!("macro `{}` has no Param({key})", frame.name)
            }
        })
    }

    /// Whether `ident`, naming a variant of `position`, may be taken as that
    /// native variant here. Under restriction a registered kind's own
    /// variants lose native candidacy (spec §4), so the ident routes to its
    /// identity macro instead. This is candidacy suppression, not
    /// match-then-reject: no value tree is ever built for a banned spelling.
    ///
    /// Suppression applies at registered kinds only — closed atoms outside the
    /// macro system (`Cmp`, the phase/step enums, `FaceLayout`) are
    /// unregistered and keep parsing natively — and skips the rows a kind
    /// declares [`natively_spellable`](crate::Kind::natively_spellable),
    /// which is why this takes the ident and not just the position.
    fn native_variant_ok(&self, position: &str, ident: &str) -> bool {
        !self.restricted || self.read.macros.natively_spellable(position, ident)
    }

    /// The context for reading an argument or expansion that isn't a body:
    /// no frame, so a stray `Param` errors instead of resolving against the
    /// wrong macro. `restricted` is the argument text's own provenance (from
    /// [`Ctx::param`]), NOT this context's — that is what keeps a card-written
    /// argument restricted after it is substituted into a free macro body.
    fn frameless(&self, restricted: bool) -> Ctx<'de, 'de> {
        Ctx {
            read: self.read,
            frame: None,
            depth: self.depth,
            restricted,
        }
    }

    /// The context for reading `frame`'s macro's body, one level deeper.
    fn expansion<'g>(&self, frame: &'g Frame<'de>) -> Result<Ctx<'de, 'g>, String> {
        if self.depth >= MAX_DEPTH {
            return Err(format!(
                "`{}` is more than {MAX_DEPTH} macro expansions deep; \
                 macros don't recurse",
                frame.name,
            ));
        }
        Ok(Ctx {
            read: self.read,
            frame: Some(frame),
            depth: self.depth + 1,
            // A macro body is definition text, which is free vocabulary
            // however the invocation was written (spec §4 container table).
            // Arguments substituted into it carry their own restriction back
            // through `param`.
            restricted: false,
        })
    }
}

/// How much of the next value position may be intercepted. Applies for one
/// level only — nested positions re-enable through the wrappers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Intercept {
    /// Capture and check anywhere a hole or invocation could stand.
    Full,
    /// Everything but struct positions: set for newtype variant content,
    /// where `unwrap_variant_newtypes` fuses a struct into the variant's
    /// parentheses mid-stream and no whole value can be captured. Scalar,
    /// sequence, and enum contents are ordinary values and stay
    /// interceptable, so `Generic(Param(0))` resolves.
    SkipStructs,
    /// Nothing: the value was just captured and is being read back as
    /// itself, so capturing again would loop forever.
    Skip,
}

/// The wrapping deserializer. Forwards everything to the inner deserializer
/// with re-wrapped visitors (so nesting stays macro-aware), and intercepts
/// the positions where a macro invocation or a `Param(n)` hole could stand.
pub struct MacroAware<'de, 'f, D> {
    de: D,
    ctx: Ctx<'de, 'f>,
    intercept: Intercept,
    /// The constructor the next struct position belongs to, when this
    /// deserializer stands at a struct variant's payload. A struct variant
    /// generated by `SupportsMacros` lowers through a private helper struct,
    /// so `deserialize_struct` is handed `__InstructionRerollStored` where the
    /// author wrote `RerollStored`; the real name is threaded here from the
    /// variant access that read the tag, for the unknown-field refusal to
    /// name. `None` everywhere else, where the struct's own serde name is the
    /// name to use.
    owner: Option<Ident>,
}

impl<'de, D> MacroAware<'de, 'de, D> {
    pub(crate) fn new(de: D, read: &'de ReadCtx<'de>) -> Self {
        Self::with_restriction(de, read, false)
    }

    /// The restricted-container entry (spec §4): the document's own text is
    /// author vocabulary, so a registered kind's variant idents are suppressed
    /// as native candidates and must route through identity macros.
    pub(crate) fn new_restricted(de: D, read: &'de ReadCtx<'de>) -> Self {
        Self::with_restriction(de, read, true)
    }

    fn with_restriction(de: D, read: &'de ReadCtx<'de>, restricted: bool) -> Self {
        Self {
            de,
            ctx: Ctx {
                read,
                frame: None,
                depth: 0,
                restricted,
            },
            intercept: Intercept::Full,
            owner: None,
        }
    }
}

impl<'de, 'f, D> MacroAware<'de, 'f, D> {
    fn with_ctx(de: D, ctx: Ctx<'de, 'f>) -> Self {
        Self {
            de,
            ctx,
            intercept: Intercept::Full,
            owner: None,
        }
    }

    fn wrap<V>(&self, visitor: V) -> Wrap<'de, 'f, V> {
        Wrap {
            visitor,
            ctx: self.ctx,
            declared: None,
        }
    }

    /// [`wrap`](Self::wrap) for a struct position: the map behind the visitor
    /// checks its keys against `fields` when the set refuses unknown ones.
    fn wrap_struct<V>(
        &self,
        owner: Ident,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Wrap<'de, 'f, V> {
        Wrap {
            visitor,
            ctx: self.ctx,
            declared: declared(self.ctx, owner, fields),
        }
    }
}

/// The unknown-field check for a struct position, or `None` when the set
/// does not refuse unknown fields or the position declares no fields (the
/// shape-agnostic `read_args` capture).
fn declared(ctx: Ctx<'_, '_>, owner: Ident, fields: &'static [&'static str]) -> Option<Declared> {
    (ctx.read.macros.denies_unknown_fields() && !fields.is_empty())
        .then_some(Declared { owner, fields })
}

/// A native ron deserializer over `source`, reading the same dialect as the
/// document — [`MacroSet`]'s options, threaded to every re-read.
fn ron_deserializer<'de>(
    source: &'de str,
    options: &ron::Options,
) -> ron::error::SpannedResult<ron::de::Deserializer<'de>> {
    ron::de::Deserializer::from_str_with_options(source, options)
}

/// Builds a fresh macro-aware deserializer over `source` and runs `f` on it,
/// typically to re-read the position the source was expanded for.
fn reread<'de, 'f, T, E: serde::de::Error>(
    source: &'de str,
    ctx: Ctx<'de, 'f>,
    intercept: Intercept,
    f: impl FnOnce(MacroAware<'de, 'f, &mut ron::de::Deserializer<'de>>) -> Result<T, ron::Error>,
) -> Result<T, E> {
    let mut de = ron_deserializer(source, ctx.read.macros.options()).map_err(E::custom)?;
    let value = f(MacroAware {
        de: &mut de,
        ctx,
        intercept,
        owner: None,
    })
    .map_err(|e| E::custom(de.span_error(e)))?;
    de.end().map_err(|e| E::custom(de.span_error(e)))?;
    Ok(value)
}

/// What an invocation's argument list read as: the arguments themselves, and
/// the `Elidable(...)` params the call omitted.
struct Invoked<'de> {
    args: FrameArgs<'de>,
    elided: Vec<ParamKey>,
}

/// What a captured fragment resolved to, when it isn't an ordinary value.
enum Invocation<'de> {
    /// A `Param(...)` hole addressing the current frame.
    Param(ParamKey),
    /// An invocation of a macro in scope at this position.
    Macro {
        name: Ident,
        def: &'de MacroDef,
        invoked: Invoked<'de>,
    },
}

/// The seed behind [`probe`]: reads a fragment through ron's enum channel —
/// the one data-model position that carries an arbitrary identifier — and
/// resolves that identifier against the reserved name `Param` and the
/// macros in scope.
struct Probe<'a, 'de, 'f> {
    /// The position's struct name, if its macros may stand here.
    position: Option<&'static str>,
    /// The caller's context: the macros in scope, the splice arena that
    /// owns any filled-in default (or forwarded-argument) text, and the
    /// frame — if any — a nested invocation's own arguments pre-substitute
    /// against (see [`read_args`]).
    ctx: Ctx<'de, 'f>,
    /// Set once a leading identifier has been read: failures before that
    /// mean "not an invocation", failures after are real.
    entered: &'a Cell<bool>,
}

impl<'de> DeserializeSeed<'de> for Probe<'_, 'de, '_> {
    type Value = Option<Invocation<'de>>;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_enum("", &[], self)
    }
}

impl<'de> Visitor<'de> for Probe<'_, 'de, '_> {
    type Value = Option<Invocation<'de>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a macro invocation or `Param` hole")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, variant) = data.variant_seed(IdentSeed)?;
        self.entered.set(true);
        if ident == "Param" {
            return Ok(Some(Invocation::Param(variant.newtype_variant()?)));
        }
        // `Splice`/`Quote` are not invocations: probe reports "not a macro" so
        // the value re-reads and — at a real value position — the
        // `EnumIntercept` guards below flag it. A raw meta-body `Quote(...)`
        // is unwrapped by `substitute_params` before it reaches here.
        let Some(def) = self
            .position
            .and_then(|kind| self.ctx.read.macros.get(kind, &ident))
        else {
            return Ok(None);
        };
        let invoked = read_args(ident, variant, &def.params, self.ctx)?;
        Ok(Some(Invocation::Macro {
            name: ident,
            def,
            invoked,
        }))
    }
}

/// Reads just the leading identifier of a captured fragment through the enum
/// channel, or `None` if the fragment doesn't open with one (a scalar, a
/// sequence, …). Used by the untagged-embed path to decide whether the value
/// is one of the host kind's own variants before falling through.
fn leading_ident<'de, E: serde::de::Error>(
    source: &'de str,
    ctx: Ctx<'de, '_>,
) -> Result<Option<Ident>, E> {
    struct LeadSeed<'a>(&'a Cell<bool>);

    impl<'de> DeserializeSeed<'de> for LeadSeed<'_> {
        type Value = Ident;

        fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
            de.deserialize_enum("", &[], self)
        }
    }

    impl<'de> Visitor<'de> for LeadSeed<'_> {
        type Value = Ident;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an identifier-led value")
        }

        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
            let (ident, _variant) = data.variant_seed(IdentSeed)?;
            self.0.set(true);
            // The variant body is left unread: the caller only wants the tag,
            // and the fragment is re-read in full afterward.
            Ok(ident)
        }
    }

    let mut de = ron_deserializer(source, ctx.read.macros.options()).map_err(E::custom)?;
    let entered = Cell::new(false);
    match LeadSeed(&entered).deserialize(&mut de) {
        Ok(ident) => Ok(Some(ident)),
        Err(_) if !entered.get() => Ok(None),
        Err(error) => Err(E::custom(de.span_error(error))),
    }
}

/// Try-parses a captured fragment as a `Param(...)` hole or — when the
/// position's struct name is given — an invocation of one of its macros.
/// `None` means an ordinary value to read natively: the fragment doesn't
/// open with an identifier, or its identifier names no macro.
fn probe<'de, E: serde::de::Error>(
    source: &'de str,
    position: Option<&'static str>,
    ctx: Ctx<'de, '_>,
) -> Result<Option<Invocation<'de>>, E> {
    let mut de = ron_deserializer(source, ctx.read.macros.options()).map_err(E::custom)?;
    let entered = Cell::new(false);
    let seed = Probe {
        position,
        ctx,
        entered: &entered,
    };
    match seed.deserialize(&mut de) {
        Ok(Some(invocation)) => {
            de.end().map_err(|e| E::custom(de.span_error(e)))?;
            Ok(Some(invocation))
        }
        Ok(None) => Ok(None),
        Err(_) if !entered.get() => Ok(None),
        Err(error) => Err(E::custom(de.span_error(error))),
    }
}

/// One value of untagged content, decomposed by ron.
enum Node<'de> {
    /// A `Param(...)` hole.
    Hole(ParamKey),
    /// A `Quote(X)` stage marker ([typed-holes delta 4]): its single child,
    /// to be emitted verbatim (the `Quote(` wrapper stripped) so a param
    /// defers through a two-level meta-macro to the produced definition.
    Quote(&'de RawValue),
    /// Anything else: its immediate children, each captured as a raw
    /// subslice of the fragment. Scalars and bare identifiers have none.
    Branch(Vec<&'de RawValue>),
}

/// Builds a native deserializer over `fragment` and applies `seed`. The
/// fragment was captured by ron, so its end isn't re-checked.
fn read_fragment<'de, S: DeserializeSeed<'de>>(
    fragment: &'de str,
    options: &ron::Options,
    seed: S,
) -> Result<S::Value, String> {
    let mut de = ron_deserializer(fragment, options).map_err(|e| e.to_string())?;
    seed.deserialize(&mut de)
        .map_err(|e| de.span_error(e).to_string())
}

/// The enum-channel half of [`decompose`]: a `Param(...)` hole resolves,
/// and any other leading identifier's contents are collected in the given
/// shape.
struct IdentLed<'a> {
    /// See [`Probe::entered`].
    entered: &'a Cell<bool>,
    /// Whether to read struct-shaped `(key: value)` contents rather than
    /// tuple-shaped.
    named: bool,
}

impl<'de> DeserializeSeed<'de> for IdentLed<'_> {
    type Value = Node<'de>;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_enum("", &[], self)
    }
}

impl<'de> Visitor<'de> for IdentLed<'_> {
    type Value = Node<'de>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an identifier-led value")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, variant) = data.variant_seed(IdentSeed)?;
        self.entered.set(true);
        if ident == "Param" {
            return Ok(Node::Hole(variant.newtype_variant()?));
        }
        if ident == QUOTE {
            return Ok(Node::Quote(variant.newtype_variant()?));
        }
        let children = if self.named {
            variant.struct_variant(&[], Children)?
        } else {
            variant.tuple_variant(0, Children)?
        };
        Ok(Node::Branch(children))
    }
}

macro_rules! leaf_visits {
    ($($method:ident$(($ty:ty))?),* $(,)?) => {
        $(fn $method<E: serde::de::Error>(self $(, _: $ty)?) -> Result<Self::Value, E> {
            Ok(Vec::new())
        })*
    };
}

/// Collects every immediate child of a value as a raw subslice: sequence
/// elements, map and struct values, the contents of `Some` and newtypes.
/// Scalars have none, and keys are skipped — holes can't stand there.
struct Children;

impl<'de> DeserializeSeed<'de> for Children {
    type Value = Vec<&'de RawValue>;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Children {
    type Value = Vec<&'de RawValue>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("untagged content")
    }

    leaf_visits! {
        visit_bool(bool),
        visit_i8(i8), visit_i16(i16), visit_i32(i32), visit_i64(i64), visit_i128(i128),
        visit_u8(u8), visit_u16(u16), visit_u32(u32), visit_u64(u64), visit_u128(u128),
        visit_f32(f32), visit_f64(f64),
        visit_char(char),
        visit_str(&str), visit_borrowed_str(&'de str), visit_string(String),
        visit_bytes(&[u8]), visit_borrowed_bytes(&'de [u8]), visit_byte_buf(Vec<u8>),
        visit_none, visit_unit,
    }

    fn visit_some<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        Ok(vec![<&RawValue>::deserialize(de)?])
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.visit_some(de)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut children = Vec::new();
        while let Some(child) = seq.next_element()? {
            children.push(child);
        }
        Ok(children)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut children = Vec::new();
        while map.next_key::<IgnoredAny>()?.is_some() {
            children.push(map.next_value()?);
        }
        Ok(children)
    }
}

/// Decomposes one fragment of untagged content. Identifier-led values go
/// through the enum channel — `deserialize_any` collapses newtype contents
/// and drops the identifiers that mark holes — trying the tuple-shaped
/// grammar first, then the struct-shaped one. Everything else decomposes
/// through [`Children`] directly.
fn decompose<'de>(fragment: &'de str, options: &ron::Options) -> Result<Node<'de>, String> {
    let entered = Cell::new(false);
    for named in [false, true] {
        let ident_led = IdentLed {
            entered: &entered,
            named,
        };
        match read_fragment(fragment, options, ident_led) {
            Ok(node) => return Ok(node),
            // Not identifier-led: decompose through the data model instead.
            Err(_) if !entered.get() => break,
            // A shape mismatch: the struct-shaped grammar is next, and a
            // bare identifier — or anything malformed, which the native
            // parse will report — is a leaf.
            Err(_) => {}
        }
    }
    if entered.get() {
        return Ok(Node::Branch(Vec::new()));
    }
    read_fragment(fragment, options, Children).map(Node::Branch)
}

/// What to do with a hole the current frame can't resolve.
#[derive(Clone, Copy, PartialEq, Eq)]
enum HoleMode {
    /// Error — untagged content's holes must all belong to the frame.
    Strict,
    /// Leave the hole verbatim — a raw capture inside a frame may be a
    /// produced definition's body, whose remaining holes belong to that
    /// definition's own params.
    PassThrough,
}

/// Walks `fragment` — a subslice of `root` — recording every `Param(...)`
/// hole as its byte range in `root`, the argument text that fills it, and
/// whether that text is restricted. The last is what stops text-level splicing
/// from laundering provenance: the splice loses the seam between the two texts,
/// so the caller has to carry the flag out with it.
fn collect_holes<'de>(
    root: &str,
    fragment: &'de str,
    ctx: &Ctx<'de, '_>,
    mode: HoleMode,
    edits: &mut Vec<(Range<usize>, &'de str, bool)>,
) -> Result<(), String> {
    match decompose(fragment, ctx.read.macros.options())? {
        Node::Hole(key) => match (ctx.param(key), mode) {
            (Ok((arg, restricted)), _) => {
                let start = fragment.as_ptr() as usize - root.as_ptr() as usize;
                edits.push((start..start + fragment.len(), arg, restricted));
            }
            (Err(reason), HoleMode::Strict) => return Err(reason),
            // The produced definition's own hole: leave the text alone.
            (Err(_), HoleMode::PassThrough) => {}
        },
        // `Quote(X)` unwraps to `X` verbatim, regardless of frame ownership
        // ([typed-holes delta 4]): the meta's frame does NOT resolve the inner
        // hole, so the produced definition carries a bare `Param(i)` its own
        // frame resolves. The inner text is left un-walked.
        Node::Quote(inner) => {
            let start = fragment.as_ptr() as usize - root.as_ptr() as usize;
            // The inner text is the meta's own body text, never an argument.
            edits.push((start..start + fragment.len(), inner.get_ron().trim(), false));
        }
        Node::Branch(children) => {
            for child in children {
                collect_holes(root, child.get_ron(), ctx, mode, edits)?;
            }
        }
    }
    Ok(())
}

/// Walks `fragment`, collecting every `Param(...)` hole key without
/// resolving anything — the insert-time validation of default expressions.
pub(crate) fn collect_param_keys(
    fragment: &str,
    options: &ron::Options,
    keys: &mut Vec<ParamKey>,
) -> Result<(), String> {
    match decompose(fragment, options)? {
        Node::Hole(key) => keys.push(key),
        // The inner hole still references a param — collect it, so a default
        // expression carrying a `Quote(Param(x))` still validates `x`.
        Node::Quote(inner) => collect_param_keys(inner.get_ron(), options, keys)?,
        Node::Branch(children) => {
            for child in children {
                collect_param_keys(child.get_ron(), options, keys)?;
            }
        }
    }
    Ok(())
}

/// A macro body's own OUTERMOST identifier, if it has one — `Foo(...)`,
/// `Foo { ... }`, or bare `Foo` all yield `Foo`; a leading `Quote(X)` stage
/// marker unwraps to `X`'s own leading identifier; anything else (a plain
/// struct/enum constructor for the position's real type, a scalar, a list)
/// yields `None`.
///
/// Deliberately shallow — it never descends into the value's own
/// children/arguments. Every macro-to-macro chain in this codebase is
/// expressed as "my body's entire value is a call to the next macro" (see
/// `body_forwards_params_into_nested_macro`'s `Pair`/`PumpUp`), so this
/// catches every real chain. Descending into arguments was tried and
/// reverted: a nested argument can be an ordinary bare-identifier leaf value
/// (e.g. `CounterRef`, spelled exactly like a nullary macro invocation with
/// no way to tell the two apart from raw text alone — `M1M1Counter`'s own
/// body names itself that way, as data, not as a self-invocation) and
/// mistaking one for an invocation is a false cycle, which is worse than
/// missing a real one — a missed one still trips the runtime `MAX_DEPTH`
/// cap, same as before this check existed.
pub(crate) fn leading_invoked_name(body: &str, options: &ron::Options) -> Option<Ident> {
    /// The [`IdentLed`] shape-probe, but it remembers the identifier it read
    /// (in `last_ident`, shared across the tuple/struct retry so the
    /// unit-variant fallback can still recover it) instead of discarding it
    /// — [`decompose`] doesn't need the name of an ordinary struct/enum
    /// variant, but this walk is looking for one that might *be* a macro
    /// invocation.
    struct IdentLedNamed<'a> {
        entered: &'a Cell<bool>,
        last_ident: &'a Cell<Option<Ident>>,
        named: bool,
    }

    /// Either the leading identifier, or (for `Quote(X)`) the inner
    /// fragment to keep unwrapping.
    enum Shape<'de> {
        Named(Ident),
        Quote(&'de RawValue),
        Other,
    }

    impl<'de> DeserializeSeed<'de> for IdentLedNamed<'_> {
        type Value = Shape<'de>;

        fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
            de.deserialize_enum("", &[], self)
        }
    }

    impl<'de> Visitor<'de> for IdentLedNamed<'_> {
        type Value = Shape<'de>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an identifier-led value")
        }

        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
            let (ident, variant) = data.variant_seed(IdentSeed)?;
            self.entered.set(true);
            self.last_ident.set(Some(ident));
            if ident == "Param" {
                let _: ParamKey = variant.newtype_variant()?;
                return Ok(Shape::Other);
            }
            if ident == QUOTE {
                return Ok(Shape::Quote(variant.newtype_variant()?));
            }
            // Consumed only to leave the parser in a valid end state; the
            // children themselves are never inspected (see the doc comment
            // above).
            if self.named {
                variant.struct_variant(&[], Children)?;
            } else {
                variant.tuple_variant(0, Children)?;
            }
            Ok(Shape::Named(ident))
        }
    }

    /// [`decompose`]'s shape-probing loop, keeping the leading identifier
    /// instead of discarding it. A malformed fragment reads as [`Shape::Other`]
    /// (never a hard error) — a malformed body is reported elsewhere, when
    /// it's read for real at first invocation, so the static check simply
    /// treats it as having nothing to walk.
    fn shape_of<'de>(fragment: &'de str, options: &ron::Options) -> Shape<'de> {
        let entered = Cell::new(false);
        let last_ident: Cell<Option<Ident>> = Cell::new(None);
        for named in [false, true] {
            let ident_led = IdentLedNamed {
                entered: &entered,
                last_ident: &last_ident,
                named,
            };
            match read_fragment(fragment, options, ident_led) {
                Ok(shape) => return shape,
                // Not identifier-led: it's an ordinary value, not a macro
                // invocation.
                Err(_) if !entered.get() => return Shape::Other,
                // A shape mismatch: the struct-shaped grammar is next, or
                // (both having failed) this is a bare unit-variant
                // identifier — either way it was already recorded.
                Err(_) => {}
            }
        }
        let ident = last_ident
            .get()
            .expect("entered implies visit_enum recorded an identifier");
        Shape::Named(ident)
    }

    let mut fragment = body;
    loop {
        match shape_of(fragment, options) {
            Shape::Named(ident) => return Some(ident),
            Shape::Quote(inner) => fragment = inner.get_ron(),
            Shape::Other => return None,
        }
    }
}

/// Replaces every `Param(n)` hole in a RON fragment with the corresponding
/// argument's source text. Two callers, distinguished by [`HoleMode`]:
/// untagged enum content (serde buffers through its private Content type,
/// so parse-position resolution can't reach inside — `Strict`), and
/// raw-value captures inside a frame (the capture stores text before any
/// position is dispatched; holes the frame doesn't own belong to the
/// produced definition — `PassThrough`). ron itself locates every value as
/// a subslice of the fragment, so holes are spliced by offset and a string
/// literal mentioning `Param` is never confused for one.
///
/// The second half of the result is the spliced text's own restriction: `ctx`'s
/// own, plus that of every argument spliced in. See [`Arg`] for why the whole
/// fragment takes the restricted half's flag.
fn substitute_params<'de>(
    source: &'de str,
    ctx: &Ctx<'de, '_>,
    mode: HoleMode,
) -> Result<(std::borrow::Cow<'de, str>, bool), String> {
    let mut edits = Vec::new();
    collect_holes(source, source, ctx, mode, &mut edits)?;
    let restricted = ctx.restricted || edits.iter().any(|(_, _, restricted)| *restricted);
    if edits.is_empty() {
        return Ok((std::borrow::Cow::Borrowed(source), restricted));
    }
    let mut out = String::new();
    let mut copied = 0;
    for (range, argument, _) in edits {
        out.push_str(&source[copied..range.start]);
        out.push_str(argument);
        copied = range.end;
    }
    out.push_str(&source[copied..]);
    Ok((std::borrow::Cow::Owned(out), restricted))
}

/// [`substitute_params`], with the result interned in the read arena so it can
/// be re-read as borrowed source: the spliced text and its restriction.
fn substitute_into<'de>(
    source: &'de str,
    ctx: &Ctx<'de, '_>,
    mode: HoleMode,
) -> Result<(&'de str, bool), String> {
    let (resolved, restricted) = substitute_params(source, ctx, mode)?;
    Ok((
        match resolved {
            std::borrow::Cow::Borrowed(source) => source,
            std::borrow::Cow::Owned(resolved) => ctx.read.splice(resolved),
        },
        restricted,
    ))
}

/// How a value's immediate children sit in its source, which decides whether
/// one of them can be dropped and how much text goes with it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// Keyed entries — `Head(k: v, …)`, `(k: v, …)`, `{k: v, …}`. Any entry
    /// may be dropped: the survivors still name themselves.
    Keyed,
    /// Ordered entries — `Head(a, b, …)`, `[a, b, …]`. Only a trailing run
    /// may be dropped: dropping an earlier one would shift the rest.
    Ordered,
    /// Neither — a scalar, an `Option`/newtype wrapper, a reserved form. Its
    /// children are positions in their own right, not droppable entries.
    Opaque,
}

macro_rules! opaque_visits {
    ($($method:ident$(($ty:ty))?),* $(,)?) => {
        $(fn $method<E: serde::de::Error>(self $(, _: $ty)?) -> Result<Self::Value, E> {
            Ok(Layout::Opaque)
        })*
    };
}

/// The identifier-led half of [`entry_layout`]: a struct-shaped variant body
/// is [`Layout::Keyed`], and anything reserved (or `Option`-shaped) is
/// [`Layout::Opaque`] — dropping a child out of `Some(...)` would change what
/// the value IS, not just which of its entries are present.
struct LayoutSeed<'a>(&'a Cell<bool>);

impl<'de> DeserializeSeed<'de> for LayoutSeed<'_> {
    type Value = Layout;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_enum("", &[], self)
    }
}

impl<'de> Visitor<'de> for LayoutSeed<'_> {
    type Value = Layout;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an identifier-led value")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, variant) = data.variant_seed(IdentSeed)?;
        self.0.set(true);
        if matches!(ident.as_str(), "Param" | QUOTE | SPLICE | "Some" | "None") {
            // The variant body is left unread; the fragment is never
            // re-read from this deserializer.
            return Ok(Layout::Opaque);
        }
        variant.struct_variant(&[], Children)?;
        Ok(Layout::Keyed)
    }
}

/// The non-identifier-led half of [`entry_layout`].
struct BareLayout;

impl<'de> DeserializeSeed<'de> for BareLayout {
    type Value = Layout;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for BareLayout {
    type Value = Layout;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a value")
    }

    opaque_visits! {
        visit_bool(bool),
        visit_i8(i8), visit_i16(i16), visit_i32(i32), visit_i64(i64), visit_i128(i128),
        visit_u8(u8), visit_u16(u16), visit_u32(u32), visit_u64(u64), visit_u128(u128),
        visit_f32(f32), visit_f64(f64),
        visit_char(char),
        visit_str(&str), visit_borrowed_str(&'de str), visit_string(String),
        visit_bytes(&[u8]), visit_borrowed_bytes(&'de [u8]), visit_byte_buf(Vec<u8>),
        visit_none, visit_unit,
    }

    fn visit_some<D: Deserializer<'de>>(self, _: D) -> Result<Self::Value, D::Error> {
        Ok(Layout::Opaque)
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, _: D) -> Result<Self::Value, D::Error> {
        Ok(Layout::Opaque)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok(Layout::Ordered)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(Layout::Keyed)
    }
}

/// How `fragment`'s immediate children — the ones [`decompose`] hands back —
/// sit in its source. Mirrors [`decompose`]'s own tuple-then-struct probing,
/// keeping the answer it discards.
fn entry_layout(fragment: &str, options: &ron::Options) -> Layout {
    let entered = Cell::new(false);
    match read_fragment(fragment, options, LayoutSeed(&entered)) {
        Ok(layout) => layout,
        // Identifier-led but not struct-shaped: a tuple variant.
        Err(_) if entered.get() => Layout::Ordered,
        Err(_) => read_fragment(fragment, options, BareLayout).unwrap_or(Layout::Opaque),
    }
}

/// A raw child's byte range within `root`, trimmed of the surrounding
/// whitespace a raw subslice can carry — the cut arithmetic needs the value's
/// own extent, not the gap around it.
fn span_in(root: &str, fragment: &str) -> Range<usize> {
    let base = fragment.as_ptr() as usize - root.as_ptr() as usize;
    let lead = fragment.len() - fragment.trim_start().len();
    let trail = fragment.len() - fragment.trim_end().len();
    (base + lead)..(base + fragment.len() - trail)
}

/// The byte index in `root` just past `fragment`'s opening delimiter — where
/// its first entry's text begins. The head identifier can't contain one, so
/// the first `(`/`[`/`{` is always the opener.
fn container_content_start(root: &str, fragment: &str) -> Result<usize, String> {
    let base = fragment.as_ptr() as usize - root.as_ptr() as usize;
    fragment
        .find(['(', '[', '{'])
        .map(|i| base + i + 1)
        .ok_or_else(|| format!("`{}` has no entries to elide from", fragment.trim()))
}

/// The byte index in `root` where the next entry's own text starts, scanning
/// from just past the container's opener or the previous entry's end: at most
/// one separating comma and any whitespace lie in between. A comment there is
/// refused rather than guessed past.
fn entry_start(root: &str, from: usize) -> Result<usize, String> {
    let bytes = root.as_bytes();
    let mut i = from;
    let mut comma = false;
    while i < bytes.len() {
        match bytes[i] {
            b',' if !comma => {
                comma = true;
                i += 1;
            }
            c if c.is_ascii_whitespace() => i += 1,
            b'/' => {
                return Err(
                    "a comment between a macro body's entries makes an elidable \
                     param's entry impossible to locate"
                        .to_owned(),
                );
            }
            _ => return Ok(i),
        }
    }
    Ok(i)
}

/// Records the byte ranges of `fragment`'s entries whose value is one of
/// `elided`'s holes, recursing into the ones that stay. Cuts are disjoint by
/// construction: an entry with a surviving predecessor takes the separator
/// BEFORE it, and one in the leading run takes the separator after it
/// instead, so the container's opener is never left facing a comma.
///
/// The recursion descends into every surviving child, INCLUDING the arguments
/// of a nested macro invocation the body writes. An elision can therefore cut
/// an inner call's argument, not just a top-level entry of the body's own
/// constructor — uniform with how a hole resolves anywhere it stands, and the
/// only reading under which `M(inner: Inner(x: Param(x)))` elides `x` at all.
fn collect_elisions(
    root: &str,
    fragment: &str,
    elided: &[ParamKey],
    options: &ron::Options,
    cuts: &mut Vec<Range<usize>>,
) -> Result<(), String> {
    let children = match decompose(fragment, options)? {
        Node::Hole(_) | Node::Quote(_) => return Ok(()),
        Node::Branch(children) => children,
    };
    if children.is_empty() {
        return Ok(());
    }
    let absent: Vec<bool> = children
        .iter()
        .map(|child| {
            matches!(
                decompose(child.get_ron(), options),
                Ok(Node::Hole(key)) if elided.contains(&key)
            )
        })
        .collect();
    let droppable: Vec<bool> = match entry_layout(fragment, options) {
        Layout::Keyed => absent.clone(),
        Layout::Ordered => {
            // A call supplies a PREFIX of a positional list, so only the
            // trailing run is genuinely absent; an earlier hole is left for
            // `Ctx::param` to report.
            let from = absent.iter().rposition(|dead| !dead).map_or(0, |i| i + 1);
            (0..absent.len()).map(|i| i >= from && absent[i]).collect()
        }
        Layout::Opaque => vec![false; absent.len()],
    };
    if droppable.iter().all(|drop| !drop) {
        for child in children {
            collect_elisions(root, child.get_ron(), elided, options, cuts)?;
        }
        return Ok(());
    }
    let spans: Vec<Range<usize>> = children
        .iter()
        .map(|child| span_in(root, child.get_ron()))
        .collect();
    for i in 0..spans.len() {
        if !droppable[i] {
            collect_elisions(root, children[i].get_ron(), elided, options, cuts)?;
            continue;
        }
        if i > 0 && (0..i).any(|j| !droppable[j]) {
            // No comment scan here, unlike the leading-run branch below: this
            // cut runs between two known value spans, where RON admits only
            // whitespace, one comma, and comments — all non-semantic, so
            // taking the whole span is safe without locating the separator.
            cuts.push(spans[i - 1].end..spans[i].end);
            continue;
        }
        let from = if i == 0 { container_content_start(root, fragment)? } else { spans[i - 1].end };
        // The separator AFTER this entry goes with it. For the last entry
        // that scan lands on the closing delimiter, taking a trailing comma
        // with it — `(a: Param(a),)` must not become `(,)`.
        cuts.push(entry_start(root, from)?..entry_start(root, spans[i].end)?);
    }
    Ok(())
}

/// The load-time pre-flight for a definition that declares `Elidable(...)`
/// params: runs the elision walk over `body` for the SHORTEST call — every
/// elidable param omitted at once — and reports what it would refuse.
///
/// [`entry_start`] refuses a comment between a body's entries, which without
/// this fires the first time a card writes the short form rather than when the
/// definition loads. These files are hand-owned, so the difference matters:
/// the trailing comment reads as harmless right up until someone else's card
/// fails.
pub(crate) fn check_elidable_entries(
    body: &str,
    elidable: &[ParamKey],
    options: &ron::Options,
) -> Result<(), String> {
    let mut cuts = Vec::new();
    collect_elisions(body, body, elidable, options, &mut cuts)
}

/// The body text to re-read for an expansion whose invocation omitted an
/// `Elidable(...)` param: the definition's body, minus every entry whose value
/// is one of those params' holes.
///
/// This is the body-side half of elision, and it is a TEXT edit made before
/// the body is parsed at all — a body is spliced and re-read as source, so
/// "the destination never sees this key" can only mean "the key isn't in the
/// source the destination reads". Entries are located by ron's own value
/// spans, so a `Param` inside a string literal is never mistaken for one.
///
/// Returns `body` itself when the invocation omitted nothing — which is every
/// invocation of every signature without an `Elidable(...)` param, so no
/// existing definition's expansion text changes by a byte.
fn elide_body<'de>(
    body: &'de str,
    frame: &Frame<'de>,
    read: &'de ReadCtx<'de>,
) -> Result<&'de str, String> {
    if frame.elided.is_empty() {
        return Ok(body);
    }
    let mut cuts = Vec::new();
    collect_elisions(body, body, &frame.elided, read.macros.options(), &mut cuts)
        .map_err(|reason| format!("macro `{}`: {reason}", frame.name))?;
    if cuts.is_empty() {
        return Ok(body);
    }
    cuts.sort_by_key(|cut| cut.start);
    let mut out = String::new();
    let mut copied = 0;
    for cut in cuts {
        out.push_str(&body[copied..cut.start.max(copied)]);
        copied = cut.end.max(copied);
    }
    out.push_str(&body[copied..]);
    Ok(read.splice(out))
}

/// Fills `body`'s `Param(i)` holes from `args`, positionally, outside any
/// real read.
///
/// This is [`substitute_params`] — the same walk a macro body goes through
/// during expansion, `Strict` so an unfillable hole is an error rather than
/// silently surviving — driven by a synthetic one-frame context instead of by
/// the deserializer. The frame-schema layer needs exactly that: a lexicon
/// entry's `body:` is a body template with no macro registered behind it, so
/// it never reaches a `read_str`, yet its holes must be filled by *this*
/// splice-by-offset rule and no other. A textual `Param(0)`-for-argument
/// replacement would substitute inside string literals, which the offset walk
/// provably does not (`ron` locates every value as a subslice).
///
/// `owner` names the body in error messages, the way a macro's name does.
///
/// # Errors
/// If `body` is not readable as RON, or holes a param `args` has no entry for.
pub(crate) fn fill_positional_params(
    owner: Ident,
    body: &str,
    args: &[&str],
    macros: &MacroSet,
) -> Result<String, String> {
    let read = ReadCtx::new(macros);
    let frame = Frame {
        name: owner,
        args: FrameArgs::Positional(
            args.iter()
                .map(|text| Arg {
                    text,
                    restricted: false,
                })
                .collect(),
        ),
        elided: Vec::new(),
    };
    let ctx = Ctx {
        read: &read,
        frame: Some(&frame),
        depth: 0,
        restricted: false,
    };
    substitute_params(body, &ctx, HoleMode::Strict).map(|(filled, _)| filled.into_owned())
}

/// Every positional `Param(i)` index `body` holes, in the order the walk
/// finds them. A named hole (`Param(cost)`) is an error: a frame-schema body
/// is addressed positionally, like the frames' own `<Param(i)>` sigils.
///
/// [`collect_param_keys`] is the walk; this is the positional projection of
/// it, so the "which params does this body actually use" question is answered
/// by the same decomposition that fills them.
///
/// # Errors
/// If `body` is not readable as RON, or holes a named param.
pub(crate) fn positional_param_indices(
    body: &str,
    macros: &MacroSet,
) -> Result<Vec<usize>, String> {
    let mut keys = Vec::new();
    collect_param_keys(body, macros.options(), &mut keys)?;
    keys.into_iter()
        .map(|key| match key {
            ParamKey::Index(index) => Ok(index),
            ParamKey::Name(name) => Err(format!(
                "holes `Param({name})`, but a frame body is addressed positionally"
            )),
        })
        .collect()
}

/// The inner source of a top-level `Splice(X)` element, or `None` when the
/// element isn't a splice. Reads through the enum channel: a `Splice`-led
/// value's single newtype child is `X`.
fn splice_child<'de>(
    element: &'de str,
    options: &ron::Options,
) -> Result<Option<&'de str>, String> {
    struct SpliceSeed<'a>(&'a Cell<bool>);
    impl<'de> DeserializeSeed<'de> for SpliceSeed<'_> {
        type Value = Option<&'de str>;
        fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
            de.deserialize_enum("", &[], self)
        }
    }
    impl<'de> Visitor<'de> for SpliceSeed<'_> {
        type Value = Option<&'de str>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a `Splice(...)`-led value")
        }
        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
            let (ident, variant) = data.variant_seed(IdentSeed)?;
            self.0.set(true);
            if ident != SPLICE {
                // Not a splice; leave the body unread (the caller re-reads the
                // element in full).
                return Ok(None);
            }
            let inner: &RawValue = variant.newtype_variant()?;
            Ok(Some(inner.get_ron().trim()))
        }
    }

    let mut de = ron_deserializer(element, options).map_err(|e| e.to_string())?;
    let entered = Cell::new(false);
    match SpliceSeed(&entered).deserialize(&mut de) {
        Ok(inner) => Ok(inner),
        // Not identifier-led (a scalar, a nested list): never a splice.
        Err(_) if !entered.get() => Ok(None),
        Err(error) => Err(de.span_error(error).to_string()),
    }
}

/// Rewrites a captured `[...]` list source, inlining every top-level
/// `Splice(X)` element ([typed-holes delta 5]): `X` is resolved against the
/// current frame (typically `Splice(Param(i))`), must itself be a `[...]`
/// list, and its elements replace the `Splice(...)` in place. Returns `None`
/// when the list carries no `Splice` element, so the caller re-reads it
/// unchanged; otherwise the rewritten list and whether any inlined element
/// came from restricted argument text (see [`Arg`]).
fn splice_seq<'de>(source: &'de str, ctx: &Ctx<'de, '_>) -> Result<Option<(String, bool)>, String> {
    let options = ctx.read.macros.options();
    let mut de = ron_deserializer(source, options).map_err(|e| e.to_string())?;
    // Top-level elements as raw subslices; a non-list source (a whole-value
    // hole was already handled) simply has no splices.
    let Ok(elements) = Vec::<&RawValue>::deserialize(&mut de) else {
        return Ok(None);
    };
    let mut spliced_any = false;
    let mut spliced_restricted = false;
    let mut out: Vec<String> = Vec::new();
    for element in elements {
        let element = element.get_ron();
        let Some(inner) = splice_child(element, options)? else {
            out.push(element.trim().to_owned());
            continue;
        };
        spliced_any = true;
        // Resolve `X` (typically `Param(i)`) against the frame, then require a
        // `[...]` list and inline its elements.
        let (list_src, restricted) = substitute_params(inner, ctx, HoleMode::Strict)?;
        let list_src: String = list_src.into_owned();
        spliced_restricted |= restricted;
        let mut list_de = ron_deserializer(&list_src, options).map_err(|e| e.to_string())?;
        let inner_elements = Vec::<&RawValue>::deserialize(&mut list_de).map_err(|_| {
            format!("`Splice({inner})` resolves to `{list_src}`, which is not a list")
        })?;
        for inner_element in inner_elements {
            out.push(inner_element.get_ron().trim().to_owned());
        }
    }
    if !spliced_any {
        return Ok(None);
    }
    Ok(Some((format!("[{}]", out.join(", ")), spliced_restricted)))
}

/// Names the macro whose body failed: the reread's span points into a
/// detached fragment, so without this nothing says which definition to look
/// at. Only the frame nearest the failure is named — propagating errors
/// (recursion chains especially) would otherwise collect one prefix per level.
fn in_expansion_of<E: serde::de::Error>(name: Ident, error: E) -> E {
    let message = error.to_string();
    if message.contains("in the expansion of") {
        return error;
    }
    E::custom(format_args!("in the expansion of `{name}`: {message}"))
}

macro_rules! forward {
    ($($method:ident),* $(,)?) => {
        $(fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            let wrapped = self.wrap(visitor);
            self.de.$method(wrapped)
        })*
    };
}

/// Forwards like `forward!`, except that inside a macro body the value is
/// captured first so a `Param(n)` hole can stand in for it.
macro_rules! forward_or_param {
    ($($method:ident($($arg:ident: $ty:ty),*)),* $(,)?) => {
        $(fn $method<V: Visitor<'de>>(
            self,
            $($arg: $ty,)*
            visitor: V,
        ) -> Result<V::Value, Self::Error> {
            if self.intercept != Intercept::Skip && self.ctx.frame.is_some() {
                return self.via_capture(None, move |de| de.$method($($arg,)* visitor));
            }
            let wrapped = self.wrap(visitor);
            self.de.$method($($arg,)* wrapped)
        })*
    };
}

impl<'de, D: Deserializer<'de>> MacroAware<'de, '_, D> {
    /// Captures the next value as source text and probes it: a `Param(n)`
    /// hole resolves to the current frame's argument, a macro invocation
    /// (when the position's struct name is given) expands, and anything
    /// else re-reads as itself.
    fn via_capture<T>(
        self,
        position: Option<&'static str>,
        f: impl FnOnce(MacroAware<'de, '_, &mut ron::de::Deserializer<'de>>) -> Result<T, ron::Error>,
    ) -> Result<T, D::Error> {
        let source = <&RawValue>::deserialize(self.de)?.get_ron();
        match probe::<D::Error>(source, position, self.ctx)? {
            Some(Invocation::Param(key)) => {
                let (arg, restricted) = self.ctx.param(key).map_err(D::Error::custom)?;
                reread(arg, self.ctx.frameless(restricted), Intercept::Full, f)
            }
            Some(Invocation::Macro { name, def, invoked }) => {
                let frame = Frame {
                    name,
                    args: invoked.args,
                    elided: invoked.elided,
                };
                let ctx = self.ctx.expansion(&frame).map_err(D::Error::custom)?;
                let body =
                    elide_body(def.body(), &frame, self.ctx.read).map_err(D::Error::custom)?;
                reread(body, ctx, Intercept::Full, f).map_err(|e| in_expansion_of(name, e))
            }
            None => reread(source, self.ctx, Intercept::Skip, f),
        }
    }
}

impl<'de, D: Deserializer<'de>> Deserializer<'de> for MacroAware<'de, '_, D> {
    type Error = D::Error;

    forward! {
        deserialize_identifier, deserialize_ignored_any,
    }

    forward_or_param! {
        // `deserialize_option` MUST capture the hole rather than forward it:
        // under `implicit_some` ron commits to `Some` for any input that is
        // not literally `None`, and `Param(i)` is not. Forwarding therefore
        // decided the `Option` before the hole resolved, and an argument of
        // `None` was then read at the INNER type — `Phyrexian(White, None)`
        // failing with "`None` is neither a variant of `Color` nor a known
        // `Color` macro". Capturing first resolves the hole to `None` and
        // re-reads that at the option position, where ron reads it as such.
        deserialize_option(),
        deserialize_bool(), deserialize_i8(), deserialize_i16(), deserialize_i32(),
        deserialize_i64(), deserialize_i128(), deserialize_u8(), deserialize_u16(),
        deserialize_u32(), deserialize_u64(), deserialize_u128(), deserialize_f32(),
        deserialize_f64(), deserialize_char(), deserialize_str(), deserialize_string(),
        deserialize_bytes(), deserialize_byte_buf(), deserialize_unit(),
        deserialize_map(),
        deserialize_unit_struct(name: &'static str),
        deserialize_tuple(len: usize),
        deserialize_tuple_struct(name: &'static str, len: usize),
    }

    /// Like the `forward_or_param!` members, but splice-aware: inside a macro
    /// body a `[...]` list is captured first, so a whole-value `Param(n)`
    /// hole stands in for the whole list AND each top-level `Splice(X)`
    /// element inlines the list `X` resolves to ([typed-holes delta 5]).
    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.intercept == Intercept::Skip || self.ctx.frame.is_none() {
            let wrapped = self.wrap(visitor);
            return self.de.deserialize_seq(wrapped);
        }
        let source = <&RawValue>::deserialize(self.de)?.get_ron();
        // A whole-value hole (`changes: Param(0)`) resolves against the frame.
        if let Some(Invocation::Param(key)) = probe::<Self::Error>(source, None, self.ctx)? {
            let (arg, restricted) = self.ctx.param(key).map_err(Self::Error::custom)?;
            return reread(arg, self.ctx.frameless(restricted), Intercept::Full, |de| {
                de.deserialize_seq(visitor)
            });
        }
        // Element-level splices: inline every top-level `Splice(X)`.
        if let Some((rewritten, restricted)) =
            splice_seq(source, &self.ctx).map_err(Self::Error::custom)?
        {
            let spliced = self.ctx.read.splice(rewritten);
            // Inlining loses the seam between the body's own elements and the
            // author's, so the rewritten list carries the restricted half's
            // provenance (see [`Arg`]). The frame stays: the body's own
            // elements may still hole.
            let ctx = Ctx {
                restricted: self.ctx.restricted || restricted,
                ..self.ctx
            };
            return reread(spliced, ctx, Intercept::Skip, |de| {
                de.deserialize_seq(visitor)
            });
        }
        // Plain list: re-read verbatim (Skip so this capture doesn't loop; the
        // elements re-enable macro-awareness through `WrapSeq`).
        reread(source, self.ctx, Intercept::Skip, |de| {
            de.deserialize_seq(visitor)
        })
    }

    /// Like the `forward_or_param!` members, except that a raw-value
    /// capture inside an expansion frame snapshots **with the frame
    /// applied**: holes the frame resolves are spliced eagerly
    /// ([`HoleMode::PassThrough`] leaves the rest for the captured
    /// definition's own params). `MacroDef.body` is the consumer — without
    /// this, a meta-produced definition would carry the meta's holes
    /// dangling.
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if name == RAW_VALUE_TOKEN && self.intercept != Intercept::Skip && self.ctx.frame.is_some()
        {
            let source = <&RawValue>::deserialize(self.de)?.get_ron();
            // A whole-value hole (`body: Param(b)`) resolves if the frame
            // owns it; otherwise it passes through like any other.
            if let Some(Invocation::Param(key)) = probe::<Self::Error>(source, None, self.ctx)?
                && let Ok((arg, restricted)) = self.ctx.param(key)
            {
                return reread(arg, self.ctx.frameless(restricted), Intercept::Skip, |de| {
                    de.deserialize_newtype_struct(name, visitor)
                });
            }
            // The capture is a produced definition's body, read back as raw
            // text (`Skip`). The substitution's restriction is DROPPED here,
            // and that is a known gap, not a decision deferred to the use
            // site: a definition body is read free when used, so a
            // card-written argument spliced into one launders its
            // restriction. Unreachable today — no type the restricted entry
            // reads carries a `RawValue`. See
            // docs/tickets/planned/macro-ron-raw-value-restriction-drop.md.
            let (resolved, _) = substitute_into(source, &self.ctx, HoleMode::PassThrough)
                .map_err(Self::Error::custom)?;
            return reread(resolved, self.ctx, Intercept::Skip, |de| {
                de.deserialize_newtype_struct(name, visitor)
            });
        }
        if self.intercept != Intercept::Skip && self.ctx.frame.is_some() {
            return self.via_capture(None, move |de| de.deserialize_newtype_struct(name, visitor));
        }
        let wrapped = self.wrap(visitor);
        self.de.deserialize_newtype_struct(name, wrapped)
    }

    /// Untagged enum content arrives here: serde buffers it through its
    /// private Content type, which our wrapped access objects interfere with
    /// (and whose synthetic map keys can't be captured). Instead, the
    /// fragment's `Param(n)` holes are spliced out — ron locates every value
    /// as a subslice of the capture — and the result is handed to ron
    /// natively, as if written out. Macro invocations (other than `Param`)
    /// inside untagged content remain unsupported.
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.intercept == Intercept::Skip || self.ctx.frame.is_none() {
            let wrapped = self.wrap(visitor);
            return self.de.deserialize_any(wrapped);
        }

        let source = <&RawValue>::deserialize(self.de)?.get_ron();
        if let Some(Invocation::Param(key)) = probe::<Self::Error>(source, None, self.ctx)? {
            let (arg, restricted) = self.ctx.param(key).map_err(Self::Error::custom)?;
            return reread(arg, self.ctx.frameless(restricted), Intercept::Full, |de| {
                de.deserialize_any(visitor)
            });
        }
        // The resolved fragment goes to ron NATIVELY (no wrapper), so no
        // restriction applies inside untagged content however it was written —
        // the spliced provenance has nowhere to be consulted. Untagged content
        // is buffered scalars and containers, not a macro-dispatch position.
        let (resolved, _) =
            substitute_into(source, &self.ctx, HoleMode::Strict).map_err(Self::Error::custom)?;
        let mut de = ron_deserializer(resolved, self.ctx.read.macros.options())
            .map_err(Self::Error::custom)?;
        let value = de
            .deserialize_any(visitor)
            .map_err(|e| Self::Error::custom(de.span_error(e)))?;
        de.end()
            .map_err(|e| Self::Error::custom(de.span_error(e)))?;
        Ok(value)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        // A struct position is captured when a hole or invocation could
        // stand there: inside a macro body, or whenever some macro expands
        // to this struct.
        let hosts_macros = self.ctx.read.macros.expands_to_struct(name);
        let interceptable = match self.intercept {
            Intercept::Full => self.ctx.frame.is_some() || hosts_macros,
            // Newtype-variant content is `SkipStructs` because
            // `unwrap_variant_newtypes` can fuse a struct into the variant's
            // parens mid-stream, where no whole value is capturable. A position
            // that HOSTS struct-macros (a `TypeDef`/`Subtype` filter-atom ref)
            // is the exception: its content is a bare macro name or an explicit
            // `(...)` struct — both capturable — so it stays interceptable and
            // the bare name still expands even nested in a newtype variant.
            Intercept::SkipStructs => hosts_macros,
            Intercept::Skip => false,
        };
        if !interceptable {
            let owner = self.owner.unwrap_or_else(|| name.into());
            let wrapped = self.wrap_struct(owner, fields, visitor);
            return self.de.deserialize_struct(name, fields, wrapped);
        }
        self.via_capture(Some(name), move |de| {
            de.deserialize_struct(name, fields, visitor)
        })
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        // Literal sugar: at a kind whose grammar is strict but whose reader
        // accepts a bare numeral (`Quantity`: `3` for `Literal(3)`; `StatValue`:
        // `-1` for `Number(-1)`, e.g. Spinal Parasite's -1/-1), capture the next
        // value first. If it's numeral-led (a digit, or `-` then a digit), splice
        // it into the wrapper and re-read; otherwise re-read it verbatim. The
        // capture runs wherever a value can be captured (`Full`, and the
        // newtype-content `SkipStructs` — an enum position is an ordinary value);
        // only `Skip` opts out, so the re-reads below can't loop: the spliced
        // `Literal(N)` re-read runs with `Full` but is no longer numeral-led (it
        // is `Wrapper(...)`), so it falls to the verbatim branch, and the
        // verbatim re-read runs with `Skip`, which skips this capture.
        if self.intercept != Intercept::Skip
            && let Some(wrapper) = self.ctx.read.macros.literal_wrapper(name)
        {
            let source = <&RawValue>::deserialize(self.de)?.get_ron();
            let trimmed = source.trim();
            let numeral_led = trimmed.starts_with(|c: char| c.is_ascii_digit())
                || trimmed
                    .strip_prefix('-')
                    .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()));
            if numeral_led {
                let spliced = self.ctx.read.splice(format!("{wrapper}({source})"));
                // The wrapper is text the READER invented, and restriction
                // follows textual provenance (spec §4): read it free, or a
                // bare numeral would route through the wrapper's identity
                // macro and change what a remembering kind stores. The frame
                // is kept, so a `Param` hole inside `source` still resolves.
                // An author who spells `Literal(3)` out is not numeral-led and
                // takes the verbatim branch below, still restricted.
                let free = Ctx {
                    restricted: false,
                    ..self.ctx
                };
                return reread(spliced, free, Intercept::Full, |de| {
                    de.deserialize_enum(name, variants, visitor)
                });
            }
            return reread(source, self.ctx, Intercept::Skip, |de| {
                de.deserialize_enum(name, variants, visitor)
            });
        }

        // Untagged embed: at a kind that embeds another type, an identifier
        // that names neither one of this kind's own variants nor one of its
        // macros falls through to the embedded type. Capture the value, read
        // its leading identifier, and — if it isn't native here — re-present
        // the whole value to the host visitor through `visit_newtype_struct`,
        // so its `Deserialize` reads the embedded type and wraps it. That
        // re-read names the embedded type's own `Deserialize`, which re-enters
        // under the embedded namespace (its variants and macros), so a macro
        // of the embedded kind is remembered as that kind. A native value
        // re-reads verbatim with `Skip` through the normal `EnumIntercept`
        // below; the fall-through re-reads with `Full` so the embedded
        // `deserialize_enum` stays intercepted.
        if self.intercept != Intercept::Skip && self.ctx.read.macros.embeds_untagged(name) {
            let source = <&RawValue>::deserialize(self.de)?.get_ron();
            let native = match leading_ident::<Self::Error>(source, self.ctx)? {
                Some(ident) => {
                    // The second native-candidacy consult (spec §4): it runs
                    // before `EnumIntercept`, so restriction must suppress
                    // here too or a banned ident at an embed-hosting kind
                    // falls through to the embedded type instead of erroring.
                    (self.ctx.native_variant_ok(name, ident.as_str())
                        && variants.contains(&ident.as_str()))
                        || ident == "Param"
                        || self.ctx.read.macros.get(name, &ident).is_some()
                }
                // Not identifier-led (a scalar, a sequence): the host kind's
                // own grammar handles it.
                None => true,
            };
            if native {
                return reread(source, self.ctx, Intercept::Skip, |de| {
                    de.deserialize_enum(name, variants, visitor)
                });
            }
            // Hand the value to the host visitor as newtype content: its
            // `visit_newtype_struct` reads the embedded type and wraps it.
            // The wrapped deserializer is macro-aware (`Full`), so the
            // embedded `Deserialize`'s `deserialize_enum` re-enters the
            // intercept under the embedded type's namespace — variants and
            // macros alike. Calling the visitor directly (not ron's named
            // `deserialize_newtype_struct`) avoids re-parsing the value as a
            // struct named after the host kind.
            return reread(source, self.ctx, Intercept::Full, |de| {
                visitor.visit_newtype_struct(de)
            });
        }

        // Bare defaulted invocation: a macro whose params are all defaulted
        // may be written by its bare name (`Hexproof` for `Hexproof()`). ron
        // reads a bare identifier through the unit-variant channel, but a
        // named macro's arguments come through the struct-variant channel,
        // which errors on it — and `VariantAccess` is one-shot, so the shape
        // can't be re-tried once chosen. Pre-scan instead: capture the value,
        // and if it's a bare identifier naming such a macro, splice the
        // explicit empty-args form and re-read, expanding identically to
        // `Hexproof()`. A parenthesized form, a non-macro identifier, or a
        // non-identifier value re-reads verbatim with `Skip` through the
        // normal `EnumIntercept` path below (the re-read opts out of this
        // scan, so it can't loop).
        if self.intercept != Intercept::Skip && self.ctx.read.macros.has_bare_invocable(name) {
            let source = <&RawValue>::deserialize(self.de)?.get_ron();
            if let Some(ident) = leading_ident::<Self::Error>(source, self.ctx)?
                && source.trim() == ident.as_str()
                && self
                    .ctx
                    .read
                    .macros
                    .get(name, &ident)
                    .is_some_and(|def| def.params.all_defaulted())
            {
                let spliced = self.ctx.read.splice(format!("{ident}()"));
                return reread(spliced, self.ctx, Intercept::Full, |de| {
                    de.deserialize_enum(name, variants, visitor)
                });
            }
            return reread(source, self.ctx, Intercept::Skip, |de| {
                de.deserialize_enum(name, variants, visitor)
            });
        }

        self.de.deserialize_enum(
            name,
            variants,
            EnumIntercept {
                name,
                variants,
                visitor,
                ctx: self.ctx,
            },
        )
    }

    fn is_human_readable(&self) -> bool {
        self.de.is_human_readable()
    }
}

/// Checks one captured argument against its declared param type, naming the
/// macro and the argument's position in any error. The validator reads the
/// argument as its type with macros in scope, so the check is the real
/// grammar — a bad `Color`, say, fails exactly as it would at a real position.
///
/// `restricted` is the ARGUMENT TEXT's own provenance, so the validator reads
/// it exactly as the later `param` re-read will. Validating free would let a
/// banned spelling through here and fail it downstream, where the error blames
/// the macro body instead of the call site that wrote the argument.
fn validate_arg(
    macro_name: Ident,
    position: impl fmt::Display,
    ty: &ParamType,
    arg: &str,
    macros: &MacroSet,
    restricted: bool,
) -> Result<(), String> {
    let Some(validator) = macros.param_validator(&ty.name) else {
        // Unreachable for an inserted macro (param types are checked at
        // insert), but don't panic on a hand-built `MacroDef`.
        return Err(format!(
            "macro `{macro_name}` declares unregistered param type `{}`",
            ty.name
        ));
    };
    validator(arg.trim(), macros, restricted).map_err(|reason| {
        format!(
            "macro `{macro_name}` argument {position} ({}): {reason}",
            ty.name
        )
    })
}

/// Fills the omitted defaulted params of a named signature: each default
/// expression's holes are spliced against the supplied args (the insert
/// check confines them to non-defaulted — i.e. present — siblings, so
/// Strict mode is total), and the filled text is validated like any
/// supplied argument.
fn fill_defaults<'de>(
    name: Ident,
    signature: &'de std::collections::HashMap<Ident, ParamType>,
    missing: Vec<&'de Ident>,
    args: &mut Vec<(Ident, Arg<'de>, bool)>,
    read: &'de ReadCtx<'de>,
) -> Result<(), String> {
    // The supplied args are what a default expression's holes splice against;
    // each carries its own restriction, which the splice carries out with it.
    let supplied = Frame {
        name,
        args: FrameArgs::Named(args.clone()),
        elided: Vec::new(),
    };
    let fill_ctx = Ctx {
        read,
        frame: Some(&supplied),
        depth: 0,
        // A default expression is the definition's own text: free until it
        // splices one of the invocation's arguments into itself.
        restricted: false,
    };
    for key in missing {
        let ty = &signature[key];
        let default = ty
            .default
            .as_deref()
            .expect("non-defaulted missing params errored by the caller");
        let (text, restricted) = substitute_into(default, &fill_ctx, HoleMode::Strict)?;
        validate_arg(name, *key, ty, text, read.macros, restricted)?;
        args.push((*key, Arg { text, restricted }, true));
    }
    Ok(())
}

/// Pre-substitutes a just-captured, not-yet-validated argument against the
/// *caller's* frame (`ctx`), before it becomes the invoked macro's own
/// argument text — the mechanism that lets a body forward its own `Param`s
/// into a nested macro it invokes (`Pair(Up(Param(0)), Up(Param(1)))`).
/// `HoleMode::PassThrough` leaves any hole the caller's frame can't resolve
/// untouched, since it belongs to the invoked macro's own frame once pushed.
/// At the top level (`ctx.frame` is `None`) this returns the input unchanged
/// *by construction*: there is no frame to forward, so `collect_holes` (which
/// would also unwrap any literal `Quote(...)` it finds, frame or no) never
/// runs, keeping the argument byte-identical to before this pre-substitution
/// step existed — including a misused top-level `Quote(...)`, which stays
/// intact for the invoked macro's own reader to reject.
///
/// The forwarded argument carries its own restriction ([`Arg`]) rather than
/// inheriting the invoked macro's frame: at the top level that is the
/// document's (a card's arguments are the author's text), and inside a body it
/// is the restriction of whatever the splice pulled in. Without this a
/// card-written argument went free the moment a body passed it on to a nested
/// macro.
fn forward_arg<'de>(raw: &'de str, ctx: Ctx<'de, '_>) -> Result<Arg<'de>, String> {
    if ctx.frame.is_none() {
        return Ok(Arg {
            text: raw,
            restricted: ctx.restricted,
        });
    }
    let (text, restricted) = substitute_into(raw, &ctx, HoleMode::PassThrough)?;
    Ok(Arg { text, restricted })
}

/// Reads the arguments the definition's signature says to expect: its shape
/// decides between the positional call grammar (unit, newtype, or tuple by
/// arity) and the named, struct-shaped one. Omitted defaulted params are
/// filled here — see [`fill_defaults`] — so a `Param` hole downstream never
/// sees the difference. Each captured raw argument is forwarded (see
/// [`forward_arg`]) against `ctx` — the caller's frame, if any — before
/// validation, so nested-macro invocations can forward the caller's own
/// params into their own arguments.
fn read_args<'de, 'f, A: VariantAccess<'de>>(
    name: Ident,
    variant: A,
    params: &'de Params,
    ctx: Ctx<'de, 'f>,
) -> Result<Invoked<'de>, A::Error> {
    use serde::de::Error;

    struct RawArgs;
    impl<'de> Visitor<'de> for RawArgs {
        type Value = Vec<&'de str>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("macro arguments")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut args = Vec::new();
            while let Some(raw) = seq.next_element::<&RawValue>()? {
                args.push(raw.get_ron());
            }
            Ok(args)
        }
    }

    struct NamedArgs;
    impl<'de> Visitor<'de> for NamedArgs {
        type Value = Vec<(Ident, &'de str)>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("named macro arguments")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut args = Vec::new();
            while let Some(key) = map.next_key_seed(IdentSeed)? {
                let raw: &RawValue = map.next_value()?;
                args.push((key, raw.get_ron()));
            }
            Ok(args)
        }
    }

    match params {
        Params::Positional(types) => {
            let args = match types.len() {
                0 => {
                    variant.unit_variant()?;
                    vec![]
                }
                1 => vec![variant.newtype_variant::<&RawValue>()?.get_ron()],
                arity => variant.tuple_variant(arity, RawArgs)?,
            };
            // Trailing `Elidable(...)` params may go unsupplied; without any,
            // `required` equals `types.len()` and this is the exact-arity
            // check it has always been.
            let required = Params::required_positional(types);
            if args.len() < required || args.len() > types.len() {
                return Err(A::Error::custom(if required == types.len() {
                    format!(
                        "expected {} macro arguments, got {}",
                        types.len(),
                        args.len(),
                    )
                } else {
                    format!(
                        "expected {required} to {} macro arguments, got {}",
                        types.len(),
                        args.len(),
                    )
                }));
            }
            let args: Vec<Arg<'de>> = args
                .into_iter()
                .map(|raw| forward_arg(raw, ctx))
                .collect::<Result<Vec<_>, _>>()
                .map_err(A::Error::custom)?;
            for (i, ty) in types.iter().take(args.len()).enumerate() {
                let arg = args[i];
                validate_arg(name, i + 1, ty, arg.text, ctx.read.macros, arg.restricted)
                    .map_err(A::Error::custom)?;
            }
            let elided = (args.len()..types.len()).map(ParamKey::Index).collect();
            Ok(Invoked {
                args: FrameArgs::Positional(args),
                elided,
            })
        }
        Params::Named(signature) => {
            let args = variant.struct_variant(&[], NamedArgs)?;
            let args: Vec<(Ident, Arg<'de>)> = args
                .into_iter()
                .map(|(key, raw)| forward_arg(raw, ctx).map(|arg| (key, arg)))
                .collect::<Result<Vec<_>, _>>()
                .map_err(A::Error::custom)?;
            for (i, (key, _)) in args.iter().enumerate() {
                if !signature.contains_key(key) {
                    return Err(A::Error::custom(format_args!(
                        "`{key}` is not one of this macro's parameters",
                    )));
                }
                // ron doesn't reject duplicate keys for us, and resolution
                // takes the first match, so a repeat would be dropped silently.
                if args[..i].iter().any(|(k, _)| k == key) {
                    return Err(A::Error::custom(format_args!("duplicate argument `{key}`")));
                }
            }
            let mut missing: Vec<&Ident> = signature
                .keys()
                .filter(|key| !args.iter().any(|(k, _)| k == *key))
                .collect();
            missing.sort_unstable_by_key(|key| key.as_str());
            // An omitted elidable param is filled with nothing: it leaves
            // `missing` here (so `fill_defaults` never sees it) and is
            // recorded on the frame for `elide_body` instead.
            let elided: Vec<ParamKey> = missing
                .iter()
                .filter(|key| signature[**key].elidable)
                .map(|key| ParamKey::Name(**key))
                .collect();
            missing.retain(|key| !signature[*key].elidable);
            if let Some(key) = missing
                .iter()
                .find(|key| signature[**key].default.is_none())
            {
                return Err(A::Error::custom(format_args!("missing argument `{key}`")));
            }
            for (key, arg) in &args {
                let ty = signature
                    .get(key)
                    .expect("argument keys were checked against the signature above");
                validate_arg(name, *key, ty, arg.text, ctx.read.macros, arg.restricted)
                    .map_err(A::Error::custom)?;
            }
            let mut args: Vec<(Ident, Arg<'de>, bool)> =
                args.into_iter().map(|(k, v)| (k, v, false)).collect();
            fill_defaults(name, signature, missing, &mut args, ctx.read)
                .map_err(A::Error::custom)?;
            Ok(Invoked {
                args: FrameArgs::Named(args),
                elided,
            })
        }
    }
}

/// The visitor for intercepted enum positions: known variants are forwarded,
/// `Param(n)` holes resolve, and anything else is tried as a macro.
struct EnumIntercept<'de, 'f, V> {
    name: &'static str,
    variants: &'static [&'static str],
    visitor: V,
    ctx: Ctx<'de, 'f>,
}

impl<'de, V: Visitor<'de>> Visitor<'de> for EnumIntercept<'de, '_, V> {
    type Value = V::Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.visitor.expecting(f)
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, variant) = data.variant_seed(IdentSeed)?;
        if ident == "Param" {
            let key = variant.newtype_variant::<ParamKey>()?;
            let (arg, restricted) = self.ctx.param(key).map_err(A::Error::custom)?;
            return reread(arg, self.ctx.frameless(restricted), Intercept::Full, |de| {
                de.deserialize_enum(self.name, self.variants, self.visitor)
            });
        }
        if ident == SPLICE {
            return Err(A::Error::custom(format_args!(
                "`Splice(...)` is only legal at a list position, not at a `{}`",
                self.name,
            )));
        }
        if ident == QUOTE {
            return Err(A::Error::custom(
                "`Quote(Param(i))` is only legal in a meta-macro body \
                 (a `Macro`-kind definition)",
            ));
        }
        // An empty variant list marks an arbitrary-identifier reader
        // (`kind_names`, `ParamType` — derived enums always pass their real
        // list): forward the ident to the visitor instead of treating it as
        // a macro name.
        if self.variants.is_empty()
            || (self.ctx.native_variant_ok(self.name, ident.as_str())
                && self.variants.contains(&ident.as_str()))
        {
            return self.visitor.visit_enum(Known {
                ident,
                variant,
                ctx: self.ctx,
            });
        }

        // if not_a_real_variant(next_ident) { expand_macro(next_ident); try_again(); }
        let def = self.ctx.read.macros.get(self.name, &ident).ok_or_else(|| {
            // A suppressed variant with no identity macro is the restricted
            // case: the spelling exists in the grammar but is not author
            // vocabulary, which is a different mistake from a typo.
            if self.variants.contains(&ident.as_str()) {
                return A::Error::custom(format_args!(
                    "`{ident}` is not author vocabulary at `{0}`; it names a \
                     `{0}` variant with no macro of that name",
                    self.name,
                ));
            }
            A::Error::custom(format_args!(
                "`{ident}` is neither a variant of `{0}` nor a known `{0}` macro",
                self.name,
            ))
        })?;
        let invoked = read_args(ident, variant, &def.params, self.ctx)?;
        let frame = Frame {
            name: ident,
            args: invoked.args,
            elided: invoked.elided,
        };
        let ctx = self.ctx.expansion(&frame).map_err(A::Error::custom)?;
        let body = elide_body(def.body(), &frame, self.ctx.read).map_err(A::Error::custom)?;

        // When the position's kind remembers its invocation, re-read a
        // synthesized `Expanded(name: …, value: <body>)` wrapper instead of
        // the bare body — the kind's own Deserialize then builds
        // `T::Expanded(Expansion { … })`. The frame (built from the original
        // input above) stays in scope, so `Param` holes inside the body copy
        // resolve exactly as they would against the body itself.
        let remembers = self.ctx.read.macros.remembers_expansion(self.name);
        let source = if remembers {
            let synthesized = synthesize_expanded(ident, &frame.args, def.template(), body);
            self.ctx.read.splice(synthesized)
        } else {
            body
        };
        reread(source, ctx, Intercept::Full, |de| {
            de.deserialize_enum(self.name, self.variants, self.visitor)
        })
        .map_err(|e| in_expansion_of(frame.name, e))
    }
}

/// Builds the `Expanded(...)` wrapper text the macro reader re-reads when a
/// remembering kind's macro expands: `Expanded(name: "M", value: <body>)`, or
/// with `template:` and/or `args:` fields when the macro has a template or the
/// invocation carried arguments. Each raw argument source is escaped as a RON
/// string with `{arg:?}` (RON's string syntax matches Rust's debug formatting).
/// The body is spliced verbatim into the `value:` position, where it re-reads
/// with the frame in scope.
///
/// Field order: `name`, then `template` (if any), then `args` (if any), then
/// `value`. Callers with no template receive byte-identical output to before.
pub(crate) fn synthesize_expanded(
    name: Ident,
    args: &FrameArgs,
    template: Option<&str>,
    body: &str,
) -> String {
    use std::fmt::Write as _;

    // `Ident`'s Debug is the tuple-struct form `Ident("…")`; the RON string
    // literal we want is the debug of the &str behind it. Argument source is
    // trimmed of the surrounding whitespace ron's `RawValue` subslice carries
    // (a value's leading/trailing ws is never meaningful) and escaped as a RON
    // string with `{:?}` — RON's string syntax matches Rust's debug, so the
    // text is carried verbatim, quotes and all.
    let name = name.as_str();
    let mut out = String::new();
    write!(out, "Expanded(name: {name:?}").unwrap();
    if let Some(t) = template {
        // `Option<String>` with implicit_some enabled: bare value is treated as
        // `Some(…)`. Use the explicit `Some(…)` form so it works under either
        // dialect — the reader accepts both.
        write!(out, ", template: Some({t:?})").unwrap();
    }
    match args {
        FrameArgs::Positional(args) if args.is_empty() => {}
        FrameArgs::Positional(args) => {
            write!(out, ", args: Positional([").unwrap();
            for (i, arg) in args.iter().enumerate() {
                let sep = if i > 0 { ", " } else { "" };
                write!(out, "{sep}{:?}", arg.text.trim()).unwrap();
            }
            write!(out, "])").unwrap();
        }
        FrameArgs::Named(args) => {
            // Filled defaults are excluded: the write side then reproduces
            // the short invocation, and re-reading re-fills them. An
            // all-defaulted call keeps `Named([])` — deliberately NOT the
            // no-args form, which serializes as the bare name a named macro
            // can't be invoked as; `Named([])` writes `M()`, which re-reads.
            write!(out, ", args: Named([").unwrap();
            let supplied = args.iter().filter(|(_, _, defaulted)| !defaulted);
            for (i, (key, arg, _)) in supplied.enumerate() {
                let sep = if i > 0 { ", " } else { "" };
                write!(out, "{sep}({:?}, {:?})", key.as_str(), arg.text.trim()).unwrap();
            }
            write!(out, "])").unwrap();
        }
    }
    write!(out, ", value: {body})").unwrap();
    out
}

/// The enum access handed to the real visitor for a known variant: replays
/// the already-read tag, then forwards the content.
struct Known<'de, 'f, A> {
    ident: Ident,
    variant: A,
    ctx: Ctx<'de, 'f>,
}

impl<'de, 'f, A: VariantAccess<'de>> EnumAccess<'de> for Known<'de, 'f, A> {
    type Error = A::Error;
    type Variant = WrapVariant<'de, 'f, A>;

    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self::Variant), Self::Error> {
        let tag = seed.deserialize(StrDeserializer::new(self.ident.as_str()))?;
        Ok((
            tag,
            WrapVariant {
                variant: self.variant,
                ctx: self.ctx,
                owner: Some(self.ident),
            },
        ))
    }
}

/// Re-wraps everything a visitor can receive, so nested positions stay
/// macro-aware: ron hands seeds its own deserializer, which would otherwise
/// drop out of this layer after one level.
struct Wrap<'de, 'f, V> {
    visitor: V,
    ctx: Ctx<'de, 'f>,
    /// The constructor whose declared fields a map at this position must keep
    /// to, when the set refuses unknown fields. `None` at every position that
    /// is not a struct or struct-variant body, and whenever the set does not
    /// refuse them.
    declared: Option<Declared>,
}

/// A struct position's owner and its declared field names, in declaration
/// order — what [`MacroSet::denying_unknown_fields`](crate::MacroSet::denying_unknown_fields)
/// checks a written key against.
#[derive(Clone, Copy)]
struct Declared {
    /// The constructor named in the refusal: the variant for a struct
    /// variant, the type for a plain struct.
    owner: Ident,
    /// The fields serde declared for this position. Never empty — an empty
    /// list marks a shape-agnostic read (`read_args`' named-argument
    /// capture), which is checked against the macro's own signature instead.
    fields: &'static [&'static str],
}

impl Declared {
    /// The check itself: `key` must be one of the declared fields.
    fn check<E: serde::de::Error>(&self, key: Ident) -> Result<(), E> {
        if self.fields.contains(&key.as_str()) {
            return Ok(());
        }
        Err(E::custom(format_args!(
            "`{}` has no field `{key}`; it declares {}",
            self.owner,
            self.fields
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", "),
        )))
    }
}

macro_rules! forward_visits {
    ($($method:ident: $ty:ty),* $(,)?) => {
        $(fn $method<E: serde::de::Error>(self, v: $ty) -> Result<Self::Value, E> {
            self.visitor.$method(v)
        })*
    };
}

impl<'de, V: Visitor<'de>> Visitor<'de> for Wrap<'de, '_, V> {
    type Value = V::Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.visitor.expecting(f)
    }

    forward_visits! {
        visit_bool: bool,
        visit_i8: i8, visit_i16: i16, visit_i32: i32, visit_i64: i64, visit_i128: i128,
        visit_u8: u8, visit_u16: u16, visit_u32: u32, visit_u64: u64, visit_u128: u128,
        visit_f32: f32, visit_f64: f64,
        visit_char: char,
        visit_str: &str, visit_borrowed_str: &'de str, visit_string: String,
        visit_bytes: &[u8], visit_borrowed_bytes: &'de [u8], visit_byte_buf: Vec<u8>,
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.visitor.visit_none()
    }

    fn visit_some<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.visitor.visit_some(MacroAware::with_ctx(de, self.ctx))
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.visitor.visit_unit()
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.visitor
            .visit_newtype_struct(MacroAware::with_ctx(de, self.ctx))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        self.visitor.visit_seq(WrapSeq { seq, ctx: self.ctx })
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        self.visitor.visit_map(WrapMap {
            map,
            ctx: self.ctx,
            declared: self.declared,
        })
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        self.visitor.visit_enum(WrapEnum {
            data,
            ctx: self.ctx,
        })
    }
}

struct WrapSeed<'de, 'f, S> {
    seed: S,
    ctx: Ctx<'de, 'f>,
    intercept: Intercept,
    /// Set only by [`WrapVariant::newtype_variant_seed`] — see
    /// [`MacroAware::owner`].
    owner: Option<Ident>,
}

impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for WrapSeed<'de, '_, S> {
    type Value = S::Value;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.seed.deserialize(MacroAware {
            de,
            ctx: self.ctx,
            intercept: self.intercept,
            owner: self.owner,
        })
    }
}

struct WrapSeq<'de, 'f, A> {
    seq: A,
    ctx: Ctx<'de, 'f>,
}

impl<'de, A: SeqAccess<'de>> SeqAccess<'de> for WrapSeq<'de, '_, A> {
    type Error = A::Error;

    fn next_element_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.seq.next_element_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::Full,
            owner: None,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.seq.size_hint()
    }
}

struct WrapMap<'de, 'f, A> {
    map: A,
    ctx: Ctx<'de, 'f>,
    declared: Option<Declared>,
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for WrapMap<'de, '_, A> {
    type Error = A::Error;

    /// Under [`Declared`] the key is read as an identifier first and checked
    /// against the position's declared fields, then replayed to the caller's
    /// seed — so a field the constructor does not declare is refused here
    /// rather than skipped by the visitor behind us.
    fn next_key_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        if let Some(declared) = self.declared {
            let Some(key) = self.map.next_key_seed(IdentSeed)? else {
                return Ok(None);
            };
            declared.check(key)?;
            return seed
                .deserialize(StrDeserializer::new(key.as_str()))
                .map(Some);
        }
        self.map.next_key_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::Full,
            owner: None,
        })
    }

    fn next_value_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.map.next_value_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::Full,
            owner: None,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.map.size_hint()
    }
}

struct WrapEnum<'de, 'f, A> {
    data: A,
    ctx: Ctx<'de, 'f>,
}

impl<'de, 'f, A: EnumAccess<'de>> EnumAccess<'de> for WrapEnum<'de, 'f, A> {
    type Error = A::Error;
    type Variant = WrapVariant<'de, 'f, A::Variant>;

    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self::Variant), Self::Error> {
        let (tag, variant) = self.data.variant_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::Full,
            owner: None,
        })?;
        Ok((
            tag,
            WrapVariant {
                variant,
                ctx: self.ctx,
                owner: None,
            },
        ))
    }
}

struct WrapVariant<'de, 'f, A> {
    variant: A,
    ctx: Ctx<'de, 'f>,
    /// The variant's own name, when the tag was read here (the [`Known`]
    /// path). `None` when the tag went straight to the caller's seed and this
    /// layer never saw it, which costs the unknown-field check its subject.
    owner: Option<Ident>,
}

impl<'de, A: VariantAccess<'de>> VariantAccess<'de> for WrapVariant<'de, '_, A> {
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.variant.unit_variant()
    }

    /// The variant's own name rides along: a `SupportsMacros` struct variant
    /// lowers through a private helper struct read here as newtype content,
    /// and the helper's generated name is not what a reader diagnostic should
    /// say (see [`MacroAware::owner`]).
    fn newtype_variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.variant.newtype_variant_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::SkipStructs,
            owner: self.owner,
        })
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.variant.tuple_variant(
            len,
            Wrap {
                visitor,
                ctx: self.ctx,
                declared: None,
            },
        )
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let declared = self
            .owner
            .and_then(|owner| declared(self.ctx, owner, fields));
        self.variant.struct_variant(
            fields,
            Wrap {
                visitor,
                ctx: self.ctx,
                declared,
            },
        )
    }
}
