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
//! Captured fragments and resolved arguments are owned by a [`Scratch`] that
//! lives for one document read: visitors are entitled to borrow from their
//! input, and the input here is text built mid-read. It all drops when the
//! read finishes.

use std::cell::RefCell;
use std::fmt;

use deckmaste_core::Ident;
use deckmaste_core::ident::IdentSeed;
use ron::value::RawValue;
use serde::Deserialize;
use serde::de::value::StrDeserializer;
use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, Error as _, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};

use crate::macros::{MacroSet, Params};

/// Owns every string built while reading one document — captured fragments,
/// resolved arguments — so they can be borrowed like the input is.
#[derive(Default)]
pub(crate) struct Scratch(RefCell<Vec<Box<str>>>);

impl Scratch {
    fn alloc(&self, s: String) -> &str {
        let boxed = s.into_boxed_str();
        let ptr = std::ptr::from_ref::<str>(&*boxed);
        self.0.borrow_mut().push(boxed);
        // SAFETY: the heap allocation behind `ptr` is owned by the pushed box
        // and lives until `self` drops; moving the box into the Vec (or the
        // Vec reallocating) doesn't move the boxed str itself, and nothing
        // removes entries.
        unsafe { &*ptr }
    }

    fn alloc_raw(&self, raw: &RawValue) -> &str { self.alloc(raw.get_ron().to_owned()) }
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

/// What every layer of the wrapper carries: the macros in scope, the scratch
/// for mid-read strings, and the innermost expansion frame, if a macro body
/// is being read.
#[derive(Clone, Copy)]
struct Ctx<'de, 'f> {
    macros: &'de MacroSet,
    scratch: &'de Scratch,
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
            macros: self.macros,
            scratch: self.scratch,
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
            macros: self.macros,
            scratch: self.scratch,
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
    pub(crate) fn new(de: D, macros: &'de MacroSet, scratch: &'de Scratch) -> Self {
        Self {
            de,
            ctx: Ctx {
                macros,
                scratch,
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

/// Builds a fresh macro-aware deserializer over `source` and runs `f` on it,
/// typically to re-read the position the source was expanded for.
fn reread<'de, 'f, T, E: serde::de::Error>(
    source: &'de str,
    ctx: Ctx<'de, 'f>,
    intercept: Intercept,
    f: impl FnOnce(MacroAware<'de, 'f, &mut ron::de::Deserializer<'de>>) -> Result<T, ron::Error>,
) -> Result<T, E> {
    let mut de =
        ron::de::Deserializer::from_str_with_options(source, &deckmaste_core::ron::options())
            .map_err(E::custom)?;
    let value = f(MacroAware {
        de: &mut de,
        ctx,
        intercept,
    })
    .map_err(|e| E::custom(de.span_error(e)))?;
    de.end().map_err(|e| E::custom(de.span_error(e)))?;
    Ok(value)
}

/// The leading identifier of a RON fragment, skipping whitespace and
/// comments; `None` if it doesn't open with one.
fn leading_ident(source: &str) -> Option<&str> {
    let mut rest = source;
    loop {
        rest = rest.trim_start();
        if let Some(comment) = rest.strip_prefix("//") {
            rest = comment.split_once('\n').map_or("", |(_, tail)| tail);
        } else if rest.starts_with("/*") {
            // Block comments nest, so the close is found by the same walk
            // `substitute_params` uses.
            rest = rest.get(skip_block_comment(rest, 0)..).unwrap_or("");
        } else {
            break;
        }
    }
    let end = rest
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(rest.len());
    (end > 0 && !rest.starts_with(|c: char| c.is_ascii_digit())).then(|| &rest[..end])
}

/// Replaces every `Param(n)` hole in a RON fragment with the corresponding
/// argument's source text, skipping string literals and comments. Only used
/// for untagged enum content, where serde buffers through `deserialize_any`
/// and parse-position resolution can't reach.
fn substitute_params<'de>(
    source: &'de str,
    ctx: &Ctx<'de, '_>,
) -> Result<std::borrow::Cow<'de, str>, String> {
    let bytes = source.as_bytes();
    let mut out = String::new();
    let mut copied = 0; // start of the region not yet copied to `out`
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => i = skip_string(source, i),
            // Raw strings — but a plain identifier can also start with `r`.
            b'r' if raw_string_quote(bytes, i).is_some() => {
                let hashes = raw_string_quote(bytes, i).expect("just checked");
                i = skip_raw_string(source, i, hashes);
            }
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                i += source[i..].find('\n').map_or(source.len() - i, |n| n + 1);
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => i = skip_block_comment(source, i),
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                if &source[start..i] != "Param" {
                    continue;
                }
                // `Param ( key )`, or it isn't a hole (e.g. a field name).
                let Some((key, end)) = parse_call_key(source, i) else {
                    continue;
                };
                out.push_str(&source[copied..start]);
                out.push_str(ctx.param(key)?);
                i = end;
                copied = end;
            }
            _ => i += 1,
        }
    }
    if copied == 0 {
        return Ok(std::borrow::Cow::Borrowed(source));
    }
    out.push_str(&source[copied..]);
    Ok(std::borrow::Cow::Owned(out))
}

/// For a `r`, `r#`, ... at `start`: the number of hashes if a raw string
/// opens here.
fn raw_string_quote(bytes: &[u8], start: usize) -> Option<usize> {
    let hashes = bytes[start + 1..]
        .iter()
        .take_while(|&&b| b == b'#')
        .count();
    (bytes.get(start + 1 + hashes) == Some(&b'"')).then_some(hashes)
}

/// Returns the index just past the string literal opening at `start`.
fn skip_string(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let mut i = start + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }
    i
}

/// Returns the index just past the raw string literal opening at `start`.
fn skip_raw_string(source: &str, start: usize, hashes: usize) -> usize {
    let close: String = std::iter::once('"')
        .chain(std::iter::repeat_n('#', hashes))
        .collect();
    let body = start + 1 + hashes + 1;
    source[body..]
        .find(&close)
        .map_or(source.len(), |n| body + n + close.len())
}

/// Returns the index just past the (nestable) block comment opening at `start`.
fn skip_block_comment(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let mut depth = 1;
    let mut i = start + 2;
    while i + 1 < bytes.len() && depth > 0 {
        match &bytes[i..i + 2] {
            b"/*" => {
                depth += 1;
                i += 2;
            }
            b"*/" => {
                depth -= 1;
                i += 2;
            }
            _ => i += 1,
        }
    }
    i
}

/// For a `(` call opening at or after `at`: the enclosed [`ParamKey`] and
/// the index just past the closing parenthesis.
fn parse_call_key(source: &str, at: usize) -> Option<(ParamKey, usize)> {
    let open = at + source[at..].len() - source[at..].trim_start().len();
    let rest = source[open..].strip_prefix('(')?.trim_start();
    let token = rest.len()
        - rest
            .trim_start_matches(|c: char| c.is_ascii_alphanumeric() || c == '_')
            .len();
    let key = deckmaste_core::ron::options()
        .from_str(&rest[..token])
        .ok()?;
    let close = rest[token..].trim_start().strip_prefix(')')?;
    Some((key, source.len() - close.len()))
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

/// Parses a whole `Param(...)` source fragment.
fn param_key<E: serde::de::Error>(source: &str) -> Result<ParamKey, E> {
    #[derive(Deserialize)]
    enum Hole {
        Param(ParamKey),
    }

    deckmaste_core::ron::options()
        .from_str(source)
        .map(|Hole::Param(key)| key)
        .map_err(E::custom)
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
                return self.via_capture(move |de| de.$method($($arg,)* visitor));
            }
            let wrapped = self.wrap(visitor);
            self.de.$method($($arg,)* wrapped)
        })*
    };
}

impl<'de, D: Deserializer<'de>> MacroAware<'de, '_, D> {
    /// Captures the next value as source text; a `Param(n)` hole resolves to
    /// the current frame's argument, anything else re-reads as itself.
    fn via_capture<T>(
        self,
        f: impl FnOnce(MacroAware<'de, '_, &mut ron::de::Deserializer<'de>>) -> Result<T, ron::Error>,
    ) -> Result<T, D::Error> {
        let source = self
            .ctx
            .scratch
            .alloc_raw(&Box::<RawValue>::deserialize(self.de)?);
        if leading_ident(source) == Some("Param") {
            let key = param_key::<D::Error>(source)?;
            let arg = self.ctx.param(key).map_err(D::Error::custom)?;
            reread(arg, self.ctx.frameless(), Intercept::Full, f)
        } else {
            reread(source, self.ctx, Intercept::Skip, f)
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
    /// fragment's `Param(n)` holes are resolved textually — string-literal
    /// aware — and the result is handed to ron natively, as if written out.
    /// Macro invocations (other than `Param`) inside untagged content remain
    /// unsupported.
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.intercept == Intercept::Skip || self.ctx.frame.is_none() {
            let wrapped = self.wrap(visitor);
            return self.de.deserialize_any(wrapped);
        }

        let source = self
            .ctx
            .scratch
            .alloc_raw(&Box::<RawValue>::deserialize(self.de)?);
        if leading_ident(source) == Some("Param") {
            let key = param_key::<Self::Error>(source)?;
            let arg = self.ctx.param(key).map_err(Self::Error::custom)?;
            return reread(arg, self.ctx.frameless(), Intercept::Full, |de| {
                de.deserialize_any(visitor)
            });
        }
        let resolved = match substitute_params(source, &self.ctx).map_err(Self::Error::custom)? {
            std::borrow::Cow::Borrowed(source) => source,
            std::borrow::Cow::Owned(resolved) => self.ctx.scratch.alloc(resolved),
        };
        let mut de =
            ron::de::Deserializer::from_str_with_options(resolved, &deckmaste_core::ron::options())
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
        let interceptable = self.intercept == Intercept::Full
            && (self.ctx.frame.is_some() || self.ctx.macros.expands_to_struct(name));
        if !interceptable {
            let wrapped = self.wrap(visitor);
            return self.de.deserialize_struct(name, fields, wrapped);
        }

        // Capture the next value as source text; if it's a hole or opens
        // with a macro name, read the position from the resolution instead.
        let source = self
            .ctx
            .scratch
            .alloc_raw(&Box::<RawValue>::deserialize(self.de)?);
        let leading = leading_ident(source);
        if leading == Some("Param") {
            let key = param_key::<Self::Error>(source)?;
            let arg = self.ctx.param(key).map_err(Self::Error::custom)?;
            return reread(arg, self.ctx.frameless(), Intercept::Full, |de| {
                de.deserialize_struct(name, fields, visitor)
            });
        }
        match leading.and_then(|ident| self.ctx.macros.get(name, ident)) {
            Some(def) => {
                let args = invocation_args::<Self::Error>(source, &def.params, self.ctx.scratch)?;
                let frame = Frame {
                    name: leading.expect("had a leading identifier").into(),
                    args,
                };
                let ctx = self.ctx.expansion(&frame).map_err(Self::Error::custom)?;
                reread(def.body(), ctx, Intercept::Full, |de| {
                    de.deserialize_struct(name, fields, visitor)
                })
                .map_err(|e| in_expansion_of(frame.name, e))
            }
            None => reread(source, self.ctx, Intercept::Skip, |de| {
                de.deserialize_struct(name, fields, visitor)
            }),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
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

/// Reads the arguments of the macro invocation `source`, as raw text each.
fn invocation_args<'de, E: serde::de::Error>(
    source: &'de str,
    params: &Params,
    scratch: &'de Scratch,
) -> Result<FrameArgs<'de>, E> {
    struct Args<'a, 'de>(&'a Params, &'de Scratch);
    impl<'de> Visitor<'de> for Args<'_, 'de> {
        type Value = FrameArgs<'de>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a macro invocation")
        }

        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
            let (_, variant) = data.variant_seed(IdentSeed)?;
            read_args(variant, self.0, self.1)
        }
    }

    let mut de =
        ron::de::Deserializer::from_str_with_options(source, &deckmaste_core::ron::options())
            .map_err(E::custom)?;
    let args = de
        .deserialize_enum("Macro", &[], Args(params, scratch))
        .map_err(|e| E::custom(de.span_error(e)))?;
    de.end().map_err(|e| E::custom(de.span_error(e)))?;
    Ok(args)
}

/// Reads the arguments the definition's signature says to expect: its shape
/// decides between the positional call grammar (unit, newtype, or tuple by
/// arity) and the named, struct-shaped one.
fn read_args<'de, A: VariantAccess<'de>>(
    variant: A,
    params: &Params,
    scratch: &'de Scratch,
) -> Result<FrameArgs<'de>, A::Error> {
    use serde::de::Error;

    struct RawArgs<'de>(&'de Scratch);
    impl<'de> Visitor<'de> for RawArgs<'de> {
        type Value = Vec<&'de str>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("macro arguments")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut args = Vec::new();
            while let Some(raw) = seq.next_element::<Box<RawValue>>()? {
                args.push(self.0.alloc_raw(&raw));
            }
            Ok(args)
        }
    }

    struct NamedArgs<'de>(&'de Scratch);
    impl<'de> Visitor<'de> for NamedArgs<'de> {
        type Value = Vec<(Ident, &'de str)>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("named macro arguments")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut args = Vec::new();
            while let Some(key) = map.next_key_seed(IdentSeed)? {
                let raw: Box<RawValue> = map.next_value()?;
                args.push((key, self.0.alloc_raw(&raw)));
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
                1 => vec![scratch.alloc_raw(&variant.newtype_variant::<Box<RawValue>>()?)],
                arity => variant.tuple_variant(arity, RawArgs(scratch))?,
            };
            if args.len() != types.len() {
                return Err(A::Error::custom(format_args!(
                    "expected {} macro arguments, got {}",
                    types.len(),
                    args.len(),
                )));
            }
            Ok(FrameArgs::Positional(args))
        }
        Params::Named(signature) => {
            let args = variant.struct_variant(&[], NamedArgs(scratch))?;
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
        let def = self.ctx.macros.get(self.name, &ident).ok_or_else(|| {
            A::Error::custom(format_args!(
                "`{ident}` is neither a variant of `{0}` nor a known `{0}` macro",
                self.name,
            ))
        })?;
        let args = read_args(variant, &def.params, self.ctx.scratch)?;
        let frame = Frame { name: ident, args };
        let ctx = self.ctx.expansion(&frame).map_err(A::Error::custom)?;
        reread(def.body(), ctx, Intercept::Full, |de| {
            de.deserialize_enum(self.name, self.variants, self.visitor)
        })
        .map_err(|e| in_expansion_of(frame.name, e))
    }
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
