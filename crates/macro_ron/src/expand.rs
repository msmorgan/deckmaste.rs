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
//! untouched. By assumption, arguments never reference an outer macro's
//! params — frames would need a caller link to support that.
//!
//! Captured fragments and invocation arguments are borrowed subslices of
//! the text they were read from — the document, a macro body, or a spliced
//! string — never copies. The one place new text is built is `Param`
//! substitution inside untagged content; those splices are owned by the
//! read-long [`ReadCtx`], because visitors are entitled to borrow from
//! their input, and drop when the read finishes.

use std::cell::Cell;
use std::fmt;
use std::ops::Range;

use elsa::FrozenVec;
use ron::value::RawValue;
use serde::Deserialize;
use serde::de::value::StrDeserializer;
use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, Error as _, IgnoredAny, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use crate::param::ParamType;
use crate::set::{MacroDef, MacroSet, Params};
use crate::{Ident, IdentSeed};

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

    fn splice(&self, s: String) -> &str { self.splices.push_get(s.into_boxed_str()) }
}

/// A macro expansion in flight: the invocation's arguments, which the body's
/// `Param(...)` holes resolve against.
struct Frame<'de> {
    /// The macro's name, for error messages.
    name: Ident,
    args: FrameArgs<'de>,
}

/// Invocation arguments, as raw source each, shaped like the signature.
enum FrameArgs<'de> {
    Positional(Vec<&'de str>),
    Named(Vec<(Ident, &'de str)>),
}

/// What a `Param(...)` hole addresses: `Param(0)` or `Param(cost)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum ParamKey {
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
}

/// Macros aren't a programming language: there is no recursion, so any chain
/// of expansions deeper than this is a definition cycle, reported as an
/// error rather than run into a stack overflow.
const MAX_DEPTH: usize = 64;

impl<'de> Ctx<'de, '_> {
    /// Resolves a `Param(...)` hole against the current frame.
    fn param(&self, key: ParamKey) -> Result<&'de str, String> {
        let frame = self
            .frame
            .ok_or_else(|| format!("Param({key}) outside any macro expansion"))?;
        let arg = match (&frame.args, key) {
            (FrameArgs::Positional(args), ParamKey::Index(index)) => args.get(index).copied(),
            (FrameArgs::Named(args), ParamKey::Name(name)) => {
                args.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
            }
            _ => None,
        };
        arg.ok_or_else(|| format!("macro `{}` has no Param({key})", frame.name))
    }

    /// The context for reading an argument or expansion that isn't a body:
    /// no frame, so a stray `Param` errors instead of resolving against the
    /// wrong macro.
    fn frameless(&self) -> Ctx<'de, 'de> {
        Ctx {
            read: self.read,
            frame: None,
            depth: self.depth,
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
}

impl<'de, D> MacroAware<'de, 'de, D> {
    pub(crate) fn new(de: D, read: &'de ReadCtx<'de>) -> Self {
        Self {
            de,
            ctx: Ctx {
                read,
                frame: None,
                depth: 0,
            },
            intercept: Intercept::Full,
        }
    }
}

impl<'de, 'f, D> MacroAware<'de, 'f, D> {
    fn with_ctx(de: D, ctx: Ctx<'de, 'f>) -> Self {
        Self {
            de,
            ctx,
            intercept: Intercept::Full,
        }
    }

    fn wrap<V>(&self, visitor: V) -> Wrap<'de, 'f, V> {
        Wrap {
            visitor,
            ctx: self.ctx,
        }
    }
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
    })
    .map_err(|e| E::custom(de.span_error(e)))?;
    de.end().map_err(|e| E::custom(de.span_error(e)))?;
    Ok(value)
}

/// What a captured fragment resolved to, when it isn't an ordinary value.
enum Invocation<'de> {
    /// A `Param(...)` hole addressing the current frame.
    Param(ParamKey),
    /// An invocation of a macro in scope at this position.
    Macro {
        name: Ident,
        def: &'de MacroDef,
        args: FrameArgs<'de>,
    },
}

/// The seed behind [`probe`]: reads a fragment through ron's enum channel —
/// the one data-model position that carries an arbitrary identifier — and
/// resolves that identifier against the reserved name `Param` and the
/// macros in scope.
struct Probe<'a, 'de> {
    /// The position's struct name, if its macros may stand here.
    position: Option<&'static str>,
    macros: &'de MacroSet,
    /// Set once a leading identifier has been read: failures before that
    /// mean "not an invocation", failures after are real.
    entered: &'a Cell<bool>,
}

impl<'de> DeserializeSeed<'de> for Probe<'_, 'de> {
    type Value = Option<Invocation<'de>>;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_enum("", &[], self)
    }
}

impl<'de> Visitor<'de> for Probe<'_, 'de> {
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
        let Some(def) = self.position.and_then(|kind| self.macros.get(kind, &ident)) else {
            return Ok(None);
        };
        let args = read_args(ident, variant, &def.params, self.macros)?;
        Ok(Some(Invocation::Macro {
            name: ident,
            def,
            args,
        }))
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
        macros: ctx.read.macros,
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

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("untagged content") }

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

/// Walks `fragment` — a subslice of `root` — recording every `Param(...)`
/// hole as its byte range in `root` and the argument text that fills it.
fn collect_holes<'de>(
    root: &str,
    fragment: &'de str,
    ctx: &Ctx<'de, '_>,
    edits: &mut Vec<(Range<usize>, &'de str)>,
) -> Result<(), String> {
    match decompose(fragment, ctx.read.macros.options())? {
        Node::Hole(key) => {
            let start = fragment.as_ptr() as usize - root.as_ptr() as usize;
            edits.push((start..start + fragment.len(), ctx.param(key)?));
        }
        Node::Branch(children) => {
            for child in children {
                collect_holes(root, child.get_ron(), ctx, edits)?;
            }
        }
    }
    Ok(())
}

/// Replaces every `Param(n)` hole in a RON fragment with the corresponding
/// argument's source text. Only used for untagged enum content, where serde
/// buffers through `deserialize_any` and parse-position resolution can't
/// reach. ron itself locates every value as a subslice of the fragment, so
/// holes are spliced by offset and a string literal mentioning `Param` is
/// never confused for one.
fn substitute_params<'de>(
    source: &'de str,
    ctx: &Ctx<'de, '_>,
) -> Result<std::borrow::Cow<'de, str>, String> {
    let mut edits = Vec::new();
    collect_holes(source, source, ctx, &mut edits)?;
    if edits.is_empty() {
        return Ok(std::borrow::Cow::Borrowed(source));
    }
    let mut out = String::new();
    let mut copied = 0;
    for (range, argument) in edits {
        out.push_str(&source[copied..range.start]);
        out.push_str(argument);
        copied = range.end;
    }
    out.push_str(&source[copied..]);
    Ok(std::borrow::Cow::Owned(out))
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
                let arg = self.ctx.param(key).map_err(D::Error::custom)?;
                reread(arg, self.ctx.frameless(), Intercept::Full, f)
            }
            Some(Invocation::Macro { name, def, args }) => {
                let frame = Frame { name, args };
                let ctx = self.ctx.expansion(&frame).map_err(D::Error::custom)?;
                reread(def.body(), ctx, Intercept::Full, f).map_err(|e| in_expansion_of(name, e))
            }
            None => reread(source, self.ctx, Intercept::Skip, f),
        }
    }
}

impl<'de, D: Deserializer<'de>> Deserializer<'de> for MacroAware<'de, '_, D> {
    type Error = D::Error;

    forward! {
        deserialize_option, deserialize_identifier, deserialize_ignored_any,
    }

    forward_or_param! {
        deserialize_bool(), deserialize_i8(), deserialize_i16(), deserialize_i32(),
        deserialize_i64(), deserialize_i128(), deserialize_u8(), deserialize_u16(),
        deserialize_u32(), deserialize_u64(), deserialize_u128(), deserialize_f32(),
        deserialize_f64(), deserialize_char(), deserialize_str(), deserialize_string(),
        deserialize_bytes(), deserialize_byte_buf(), deserialize_unit(), deserialize_seq(),
        deserialize_map(),
        deserialize_unit_struct(name: &'static str),
        deserialize_newtype_struct(name: &'static str),
        deserialize_tuple(len: usize),
        deserialize_tuple_struct(name: &'static str, len: usize),
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
            let arg = self.ctx.param(key).map_err(Self::Error::custom)?;
            return reread(arg, self.ctx.frameless(), Intercept::Full, |de| {
                de.deserialize_any(visitor)
            });
        }
        let resolved = match substitute_params(source, &self.ctx).map_err(Self::Error::custom)? {
            std::borrow::Cow::Borrowed(source) => source,
            std::borrow::Cow::Owned(resolved) => self.ctx.read.splice(resolved),
        };
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
        let interceptable = self.intercept == Intercept::Full
            && (self.ctx.frame.is_some() || self.ctx.read.macros.expands_to_struct(name));
        if !interceptable {
            let wrapped = self.wrap(visitor);
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
        // accepts a bare numeral (`Quantity`: `3` for `Literal(3)`), capture
        // the next value first. If it's digit-led, splice it into the wrapper
        // and re-read; otherwise re-read it verbatim. The capture runs wherever
        // a value can be captured (`Full`, and the newtype-content `SkipStructs`
        // — an enum position is an ordinary value); only `Skip` opts out, so the
        // re-reads below can't loop: the spliced `Literal(N)` re-read runs with
        // `Full` but is no longer digit-led, so it falls to the verbatim branch,
        // and the verbatim re-read runs with `Skip`, which skips this capture.
        if self.intercept != Intercept::Skip
            && let Some(wrapper) = self.ctx.read.macros.literal_wrapper(name)
        {
            let source = <&RawValue>::deserialize(self.de)?.get_ron();
            if source.trim().starts_with(|c: char| c.is_ascii_digit()) {
                let spliced = self.ctx.read.splice(format!("{wrapper}({source})"));
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

    fn is_human_readable(&self) -> bool { self.de.is_human_readable() }
}

/// Checks one captured argument against its declared param type, naming the
/// macro and the argument's position in any error. The validator reads the
/// argument as its type with macros in scope, so the check is the real
/// grammar — a bad `Color`, say, fails exactly as it would at a real position.
fn validate_arg(
    macro_name: Ident,
    position: impl fmt::Display,
    ty: &ParamType,
    arg: &str,
    macros: &MacroSet,
) -> Result<(), String> {
    let Some(validator) = macros.param_validator(&ty.0) else {
        // Unreachable for an inserted macro (param types are checked at
        // insert), but don't panic on a hand-built `MacroDef`.
        return Err(format!(
            "macro `{macro_name}` declares unregistered param type `{}`",
            ty.0
        ));
    };
    validator(arg.trim(), macros).map_err(|reason| {
        format!(
            "macro `{macro_name}` argument {position} ({}): {reason}",
            ty.0
        )
    })
}

/// Reads the arguments the definition's signature says to expect: its shape
/// decides between the positional call grammar (unit, newtype, or tuple by
/// arity) and the named, struct-shaped one.
fn read_args<'de, A: VariantAccess<'de>>(
    name: Ident,
    variant: A,
    params: &Params,
    macros: &MacroSet,
) -> Result<FrameArgs<'de>, A::Error> {
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
            if args.len() != types.len() {
                return Err(A::Error::custom(format_args!(
                    "expected {} macro arguments, got {}",
                    types.len(),
                    args.len(),
                )));
            }
            for (i, ty) in types.iter().enumerate() {
                validate_arg(name, i + 1, ty, args[i], macros).map_err(A::Error::custom)?;
            }
            Ok(FrameArgs::Positional(args))
        }
        Params::Named(signature) => {
            let args = variant.struct_variant(&[], NamedArgs)?;
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
            if let Some(key) = missing.first() {
                return Err(A::Error::custom(format_args!("missing argument `{key}`")));
            }
            for (key, arg) in &args {
                let ty = signature
                    .get(key)
                    .expect("argument keys were checked against the signature above");
                validate_arg(name, *key, ty, arg, macros).map_err(A::Error::custom)?;
            }
            Ok(FrameArgs::Named(args))
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

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { self.visitor.expecting(f) }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, variant) = data.variant_seed(IdentSeed)?;
        if ident == "Param" {
            let key = variant.newtype_variant::<ParamKey>()?;
            let arg = self.ctx.param(key).map_err(A::Error::custom)?;
            return reread(arg, self.ctx.frameless(), Intercept::Full, |de| {
                de.deserialize_enum(self.name, self.variants, self.visitor)
            });
        }
        if self.variants.contains(&ident.as_str()) {
            return self.visitor.visit_enum(Known {
                ident,
                variant,
                ctx: self.ctx,
            });
        }

        // if not_a_real_variant(next_ident) { expand_macro(next_ident); try_again(); }
        let def = self.ctx.read.macros.get(self.name, &ident).ok_or_else(|| {
            A::Error::custom(format_args!(
                "`{ident}` is neither a variant of `{0}` nor a known `{0}` macro",
                self.name,
            ))
        })?;
        let args = read_args(ident, variant, &def.params, self.ctx.read.macros)?;
        let frame = Frame { name: ident, args };
        let ctx = self.ctx.expansion(&frame).map_err(A::Error::custom)?;

        // When the position's kind remembers its invocation, re-read a
        // synthesized `Expanded(name: …, value: <body>)` wrapper instead of
        // the bare body — the kind's own Deserialize then builds
        // `T::Expanded(Expansion { … })`. The frame (built from the original
        // input above) stays in scope, so `Param` holes inside the body copy
        // resolve exactly as they would against the body itself.
        let remembers = self.ctx.read.macros.remembers_expansion(self.name);
        let source = if remembers {
            let synthesized = synthesize_expanded(ident, &frame.args, def.body());
            self.ctx.read.splice(synthesized)
        } else {
            def.body()
        };
        reread(source, ctx, Intercept::Full, |de| {
            de.deserialize_enum(self.name, self.variants, self.visitor)
        })
        .map_err(|e| in_expansion_of(frame.name, e))
    }
}

/// Builds the `Expanded(...)` wrapper text the macro reader re-reads when a
/// remembering kind's macro expands: `Expanded(name: "M", value: <body>)`, or
/// with an `args:` field when the invocation carried arguments. Each raw
/// argument source is escaped as a RON string with `{arg:?}` (RON's string
/// syntax matches Rust's debug formatting). The body is spliced verbatim into
/// the `value:` position, where it re-reads with the frame in scope.
fn synthesize_expanded(name: Ident, args: &FrameArgs, body: &str) -> String {
    use std::fmt::Write as _;

    // `Ident`'s Debug is the tuple-struct form `Ident("…")`; the RON string
    // literal we want is the debug of the &str behind it. Argument source is
    // trimmed of the surrounding whitespace ron's `RawValue` subslice carries
    // (a value's leading/trailing ws is never meaningful) and escaped as a RON
    // string with `{:?}` — RON's string syntax matches Rust's debug, so the
    // text is carried verbatim, quotes and all.
    let name = name.as_str();
    let mut out = String::new();
    match args {
        FrameArgs::Positional(args) if args.is_empty() => {
            write!(out, "Expanded(name: {name:?}, value: {body})").unwrap();
        }
        FrameArgs::Positional(args) => {
            write!(out, "Expanded(name: {name:?}, args: Positional([").unwrap();
            for (i, arg) in args.iter().enumerate() {
                let sep = if i > 0 { ", " } else { "" };
                write!(out, "{sep}{:?}", arg.trim()).unwrap();
            }
            write!(out, "]), value: {body})").unwrap();
        }
        FrameArgs::Named(args) => {
            write!(out, "Expanded(name: {name:?}, args: Named([").unwrap();
            for (i, (key, arg)) in args.iter().enumerate() {
                let sep = if i > 0 { ", " } else { "" };
                write!(out, "{sep}({:?}, {:?})", key.as_str(), arg.trim()).unwrap();
            }
            write!(out, "]), value: {body})").unwrap();
        }
    }
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

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { self.visitor.expecting(f) }

    forward_visits! {
        visit_bool: bool,
        visit_i8: i8, visit_i16: i16, visit_i32: i32, visit_i64: i64, visit_i128: i128,
        visit_u8: u8, visit_u16: u16, visit_u32: u32, visit_u64: u64, visit_u128: u128,
        visit_f32: f32, visit_f64: f64,
        visit_char: char,
        visit_str: &str, visit_borrowed_str: &'de str, visit_string: String,
        visit_bytes: &[u8], visit_borrowed_bytes: &'de [u8], visit_byte_buf: Vec<u8>,
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> { self.visitor.visit_none() }

    fn visit_some<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.visitor.visit_some(MacroAware::with_ctx(de, self.ctx))
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> { self.visitor.visit_unit() }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.visitor
            .visit_newtype_struct(MacroAware::with_ctx(de, self.ctx))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        self.visitor.visit_seq(WrapSeq { seq, ctx: self.ctx })
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        self.visitor.visit_map(WrapMap { map, ctx: self.ctx })
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
}

impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for WrapSeed<'de, '_, S> {
    type Value = S::Value;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        self.seed.deserialize(MacroAware {
            de,
            ctx: self.ctx,
            intercept: self.intercept,
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
        })
    }

    fn size_hint(&self) -> Option<usize> { self.seq.size_hint() }
}

struct WrapMap<'de, 'f, A> {
    map: A,
    ctx: Ctx<'de, 'f>,
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for WrapMap<'de, '_, A> {
    type Error = A::Error;

    fn next_key_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.map.next_key_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::Full,
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
        })
    }

    fn size_hint(&self) -> Option<usize> { self.map.size_hint() }
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
        })?;
        Ok((
            tag,
            WrapVariant {
                variant,
                ctx: self.ctx,
            },
        ))
    }
}

struct WrapVariant<'de, 'f, A> {
    variant: A,
    ctx: Ctx<'de, 'f>,
}

impl<'de, A: VariantAccess<'de>> VariantAccess<'de> for WrapVariant<'de, '_, A> {
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> { self.variant.unit_variant() }

    fn newtype_variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.variant.newtype_variant_seed(WrapSeed {
            seed,
            ctx: self.ctx,
            intercept: Intercept::SkipStructs,
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
            },
        )
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.variant.struct_variant(
            fields,
            Wrap {
                visitor,
                ctx: self.ctx,
            },
        )
    }
}
