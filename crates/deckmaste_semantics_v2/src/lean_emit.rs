//! Rendering a v2 value back into the Lean source it mirrors.
//!
//! The card soundness gate (`docs/decisions/semantics-v2.md` §13) writes each
//! expanded card as a fully applied Lean term and asks the kernel to prove
//! `Card.check` empty. This module is that renderer: it turns any mirrored
//! value into Lean source and nothing else — the command, the file writing,
//! the `lake` invocation and the baseline ratchet live in `xtask`.
//!
//! Emission goes through [`serde::Serialize`], so it covers the whole mirror
//! by construction rather than by ~200 hand-written arms that could fall
//! behind a new constructor. The serde data model carries exactly what a Lean
//! term needs: a variant's declaring type and name, a struct's type and its
//! fields in declaration order (which is constructor argument order, §2), and
//! `u32`/`i32` apart (`Nat` against `Int`).
//!
//! `Macros.lean` plays no part: the emitted term is FULLY EXPANDED, so it is
//! raw constructors throughout and the `spelled` elaborator — which refuses
//! raw constructors by design — is not the gate's device. The gate writes a
//! plain `def` and a `theorem … := by decide`, which is the pin discipline.
//!
//! Constructors and structure instances are written fully qualified and
//! ascribed (`Semantics.Card.singleFaced`,
//! `({ … } : Semantics.Characteristics)`) and every compound argument is
//! parenthesised, so an emitted fragment needs no `open` and no surrounding
//! precedence. The polymorphic spellings — `none`, `some x`, and the list
//! literal `[…]` — are deliberately left unqualified and do still take their
//! element type from the position they sit in; inside a card term that
//! position is always a constructor argument or an ascribed structure field,
//! so the type is fixed by the enclosing form.
//!
//! One wrinkle of the two derives the mirror carries: a `SupportsMacros` type
//! lowers each struct variant through a private `__TypeVariant` helper struct
//! and writes it as newtype content (`macro_ron_derive`, so RON's
//! `unwrap_variant_newtypes` keeps the text flat), while a plainly
//! serde-derived type writes the same variant through `serialize_struct_variant`.
//! Both spell one Lean constructor applied to its fields in declaration order,
//! so the renderer unwraps the helper rather than emitting it as a structure
//! the Lean side does not have.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fmt::Write as _;

use serde::Serialize;
use serde::ser;

use crate::card::Card;

/// The Lean namespace the mirrored syntax lives in.
pub const LEAN_NAMESPACE: &str = "Semantics";

// ---------------------------------------------------------------------------
// The name mapping
// ---------------------------------------------------------------------------
//
// One mapping, used in both directions: `tests/lean_drift.rs` reads Lean and
// compares it to Rust through [`rust_variant`]/[`rust_field`]; the emitter
// writes Rust back as Lean through [`lean_variant`]/[`lean_field`]. Keeping
// the pair here — with the drift test asserting the round trip over every name
// the six Lean files declare — is what stops the gate and the drift test from
// disagreeing about what a constructor is called.

/// Rust keywords that a raw identifier (`r#type`) can escape. Serde reads and
/// writes a raw identifier under its bare word, so the RON surface — and the
/// name this module receives — keeps the Lean spelling.
pub const RAWABLE: &[&str] = &[
    "as", "break", "const", "continue", "dyn", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "static", "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "abstract",
    "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual",
    "yield", "try", "gen", "async", "await",
];

/// Rust keywords no raw identifier can escape; these take a trailing
/// underscore and a `serde(rename)` back to the Lean spelling.
pub const UNRAWABLE: &[&str] = &["crate", "self", "Self", "super"];

/// The words Lean's own grammar reserves, so the mirrored declaration spells
/// them with a trailing underscore (`from_`, `while_`). The list is the exact
/// set the seven syntax files use; the drift test's round trip over every Lean
/// name is what keeps it from going stale.
pub const LEAN_ESCAPED: &[&str] = &[
    "as", "by", "class", "end", "exists", "from", "repeat", "return", "then", "unless", "until",
    "while",
];

/// A Lean constructor name as its Rust variant: the same word, dropping Lean's
/// trailing-underscore keyword escape, in `UpperCamelCase`.
#[must_use]
pub fn rust_variant(lean: &str) -> String {
    let stem = lean.trim_end_matches('_');
    let mut chars = stem.chars();
    let mut out: String = match chars.next() {
        Some(first) => first.to_uppercase().collect(),
        None => String::new(),
    };
    out.push_str(chars.as_str());
    if UNRAWABLE.contains(&out.as_str()) {
        out.push('_');
    }
    out
}

/// A Lean field name as its Rust field: the same word, dropping Lean's
/// trailing-underscore keyword escape, in `snake_case`, with Rust's own escape
/// applied where the word is a keyword.
#[must_use]
pub fn rust_field(lean: &str) -> String {
    let stem = lean.trim_end_matches('_');
    let mut snake = String::new();
    for ch in stem.chars() {
        if ch.is_uppercase() {
            snake.push('_');
            snake.extend(ch.to_lowercase());
        } else {
            snake.push(ch);
        }
    }
    if RAWABLE.contains(&snake.as_str()) {
        return format!("r#{snake}");
    }
    if UNRAWABLE.contains(&snake.as_str()) {
        snake.push('_');
    }
    snake
}

/// A Rust variant name as its Lean constructor: `lowerCamelCase`, with Lean's
/// keyword escape restored.
///
/// It accepts either spelling of the Rust side — the ident (`Self_`) or the
/// serde name the emitter sees (`Self`) — because the unrawable escape is
/// exactly what the `serde(rename)` undoes.
#[must_use]
pub fn lean_variant(rust: &str) -> String {
    let stem = strip_unrawable(rust);
    let mut chars = stem.chars();
    let mut out: String = match chars.next() {
        Some(first) => first.to_lowercase().collect(),
        None => String::new(),
    };
    out.push_str(chars.as_str());
    escape_lean(out)
}

/// A Rust field name as its Lean field: `lowerCamelCase`, with the raw-ident
/// prefix and the unrawable escape removed and Lean's keyword escape restored.
#[must_use]
pub fn lean_field(rust: &str) -> String {
    let stem = strip_unrawable(rust.strip_prefix("r#").unwrap_or(rust));
    let mut out = String::new();
    let mut capitalize = false;
    for ch in stem.chars() {
        if ch == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            out.push(ch);
        }
    }
    escape_lean(out)
}

/// Drops the trailing underscore Rust adds to a keyword it cannot raw-escape.
fn strip_unrawable(rust: &str) -> &str {
    match rust.strip_suffix('_') {
        Some(stem) if UNRAWABLE.contains(&stem) => stem,
        _ => rust,
    }
}

/// Restores Lean's trailing-underscore escape on a word Lean reserves.
fn escape_lean(mut name: String) -> String {
    if LEAN_ESCAPED.contains(&name.as_str()) {
        name.push('_');
    }
    name
}

// ---------------------------------------------------------------------------
// Identifiers
// ---------------------------------------------------------------------------

/// The `def` name for a card: `card_` plus the card's name in `snake_case`,
/// keeping ASCII alphanumerics and collapsing everything else to one
/// underscore.
#[must_use]
pub fn card_ident(name: &str) -> String {
    let mut out = String::from("card");
    let mut pending = true;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending {
                out.push('_');
                pending = false;
            }
            out.extend(ch.to_lowercase());
        } else {
            pending = true;
        }
    }
    out
}

/// One `def` name per card name, collision-suffixed and stable across runs:
/// names are taken in sorted order and a repeat of an already-taken ident gets
/// the next free `_2`, `_3`, … suffix.
#[must_use]
pub fn assign_idents<'a, I>(names: I) -> Vec<(String, String)>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut sorted: Vec<&str> = names.into_iter().collect();
    sorted.sort_unstable();
    sorted.dedup();
    let mut taken: BTreeMap<String, usize> = BTreeMap::new();
    let mut out = Vec::with_capacity(sorted.len());
    for name in sorted {
        let base = card_ident(name);
        let count = taken.entry(base.clone()).or_insert(0);
        *count += 1;
        let ident = if *count == 1 { base } else { format!("{base}_{count}") };
        out.push((name.to_owned(), ident));
    }
    out
}

// ---------------------------------------------------------------------------
// Modules
// ---------------------------------------------------------------------------

/// One card's place in a generated module: the name it was read under, the
/// Lean `def` it became, and the line its block opens on (1-based). The gate
/// attributes a Lean diagnostic to a card by that line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedCard {
    /// The card's identity in the plugin (its file stem).
    pub name: String,
    /// The Lean `def` name; the theorem beside it is `<ident>_ok`.
    pub ident: String,
    /// The 1-based line the card's `def` opens on.
    pub start_line: usize,
}

/// A generated Lean module: its source, and where each card sits in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedModule {
    pub source: String,
    pub cards: Vec<GeneratedCard>,
}

/// Renders one plugin's cards as a Lean module under `namespace`.
///
/// Each card becomes a `def` of its expanded term and a `theorem` proving its
/// refusal list empty by `decide` — the pin discipline, applied to emitted
/// rather than hand-spelled terms.
///
/// # Errors
/// If a card holds a shape the Lean mirror has no form for.
pub fn render_module(
    namespace: &str,
    cards: &[(String, &Card)],
) -> Result<GeneratedModule, EmitError> {
    let mut source = String::new();
    source.push_str("-- Generated by `cargo xtask lean-check`; untracked, do not edit.\n");
    source.push_str("import Semantics\n");
    source.push_str("import Semantics.Check.Card\n\n");
    let _ = writeln!(source, "namespace {namespace}\n");

    let idents: BTreeMap<String, String> =
        assign_idents(cards.iter().map(|(name, _)| name.as_str()))
            .into_iter()
            .collect();
    let mut entries = Vec::with_capacity(cards.len());
    for (name, card) in cards {
        let ident = idents
            .get(name)
            .ok_or_else(|| EmitError::Custom(format!("no identifier assigned for card {name:?}")))?
            .clone();
        let start_line = source.lines().count() + 1;
        let term = term(*card)?;
        let _ = writeln!(source, "-- {}", comment_text(name));
        let _ = writeln!(source, "def {ident} : {LEAN_NAMESPACE}.Card :=\n  {term}");
        // The guarded `#eval` beside the theorem is what puts the REFUSAL LIST
        // into Lean's output. `decide`'s own failure says only that
        // `<card>.check = []` is false, so every refuted card would otherwise
        // carry one indistinguishable reason and the gate's baseline could not
        // tell one broken law from another. A card that checks matches the
        // docstring and `#guard_msgs` stays silent; a card that does not gets
        // an `info` line naming exactly which refusals it earned.
        let _ = writeln!(source, "/-- info: [] -/\n#guard_msgs in");
        let _ = writeln!(source, "#eval {LEAN_NAMESPACE}.Card.check {ident}");
        let _ = writeln!(
            source,
            "theorem {ident}_ok : {LEAN_NAMESPACE}.Card.check {ident} = [] := by decide\n"
        );
        entries.push(GeneratedCard {
            name: name.clone(),
            ident,
            start_line,
        });
    }
    let _ = writeln!(source, "end {namespace}");
    Ok(GeneratedModule {
        source,
        cards: entries,
    })
}

/// A card name as a single-line Lean comment body.
fn comment_text(name: &str) -> String {
    name.chars()
        .map(|ch| if ch.is_control() { ' ' } else { ch })
        .collect()
}

// ---------------------------------------------------------------------------
// The term renderer
// ---------------------------------------------------------------------------

/// Why a value would not render as Lean.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EmitError {
    /// A serde shape the Lean mirror has no form for.
    #[error("the Lean mirror has no form for a Rust {0}")]
    Unsupported(&'static str),
    /// Anything `serde` itself reports.
    #[error("{0}")]
    Custom(String),
}

impl ser::Error for EmitError {
    fn custom<T: Display>(message: T) -> Self {
        EmitError::Custom(message.to_string())
    }
}

/// Renders any mirrored value as a Lean term, safe to splice as a single
/// argument (compounds arrive parenthesised).
///
/// # Errors
/// If the value holds a shape the Lean mirror has no form for.
pub fn term<T: Serialize + ?Sized>(value: &T) -> Result<String, EmitError> {
    value.serialize(LeanSerializer::plain())
}

/// A Lean string literal.
#[must_use]
pub fn string_literal(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if (ch as u32) < 0x20 || ch as u32 == 0x7f => {
                let _ = write!(out, "\\x{:02x}", ch as u32);
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// A constructor's fully qualified Lean name.
fn constructor(type_name: &str, variant: &str) -> String {
    format!("{LEAN_NAMESPACE}.{type_name}.{}", lean_variant(variant))
}

/// `macro_ron_derive`'s private per-struct-variant helper struct.
fn helper_ident(type_name: &str, variant: &str) -> String {
    format!("__{type_name}{variant}")
}

/// A constructor applied to its arguments, parenthesised when applied.
fn application(head: String, args: &[String]) -> String {
    if args.is_empty() { head } else { format!("({head} {})", args.join(" ")) }
}

/// The renderer. `unwrap` names the one `SupportsMacros` helper struct whose
/// fields belong to the enclosing constructor rather than to a structure of
/// their own; it is set for exactly one level, by `serialize_newtype_variant`.
#[derive(Clone, Copy)]
struct LeanSerializer<'a> {
    unwrap: Option<(&'a str, &'a Cell<bool>)>,
}

impl LeanSerializer<'_> {
    /// The ordinary renderer, with no helper struct to unwrap.
    const fn plain() -> Self {
        LeanSerializer { unwrap: None }
    }
}

impl ser::Serializer for LeanSerializer<'_> {
    type Ok = String;
    type Error = EmitError;
    type SerializeSeq = SeqEmitter;
    type SerializeTuple = SeqEmitter;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = VariantEmitter;
    type SerializeMap = Unsupported;
    type SerializeStruct = StructEmitter;
    type SerializeStructVariant = VariantEmitter;

    fn serialize_bool(self, value: bool) -> Result<String, EmitError> {
        Ok(if value { "true" } else { "false" }.to_owned())
    }

    fn serialize_i8(self, value: i8) -> Result<String, EmitError> {
        Ok(int_literal(i64::from(value)))
    }
    fn serialize_i16(self, value: i16) -> Result<String, EmitError> {
        Ok(int_literal(i64::from(value)))
    }
    fn serialize_i32(self, value: i32) -> Result<String, EmitError> {
        Ok(int_literal(i64::from(value)))
    }
    fn serialize_i64(self, value: i64) -> Result<String, EmitError> {
        Ok(int_literal(value))
    }

    fn serialize_u8(self, value: u8) -> Result<String, EmitError> {
        Ok(value.to_string())
    }
    fn serialize_u16(self, value: u16) -> Result<String, EmitError> {
        Ok(value.to_string())
    }
    fn serialize_u32(self, value: u32) -> Result<String, EmitError> {
        Ok(value.to_string())
    }
    fn serialize_u64(self, value: u64) -> Result<String, EmitError> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, _: f32) -> Result<String, EmitError> {
        Err(EmitError::Unsupported("float"))
    }
    fn serialize_f64(self, _: f64) -> Result<String, EmitError> {
        Err(EmitError::Unsupported("float"))
    }

    fn serialize_char(self, value: char) -> Result<String, EmitError> {
        Ok(string_literal(&value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<String, EmitError> {
        Ok(string_literal(value))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<String, EmitError> {
        Err(EmitError::Unsupported("byte string"))
    }

    fn serialize_none(self) -> Result<String, EmitError> {
        Ok("none".to_owned())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<String, EmitError> {
        Ok(format!(
            "(some {})",
            value.serialize(LeanSerializer::plain())?
        ))
    }

    fn serialize_unit(self) -> Result<String, EmitError> {
        Err(EmitError::Unsupported("unit"))
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<String, EmitError> {
        Err(EmitError::Unsupported("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<String, EmitError> {
        Ok(constructor(name, variant))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<String, EmitError> {
        value.serialize(LeanSerializer::plain())
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<String, EmitError> {
        let head = constructor(name, variant);
        let helper = helper_ident(name, variant);
        let unwrapped = Cell::new(false);
        let rendered = value.serialize(LeanSerializer {
            unwrap: Some((&helper, &unwrapped)),
        })?;
        if !unwrapped.get() {
            return Ok(application(head, &[rendered]));
        }
        // The helper stood for the constructor's own fields, already rendered
        // as an argument list.
        Ok(if rendered.is_empty() { head } else { format!("({head} {rendered})") })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<SeqEmitter, EmitError> {
        Ok(SeqEmitter {
            items: Vec::with_capacity(len.unwrap_or_default()),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<SeqEmitter, EmitError> {
        Ok(SeqEmitter {
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Unsupported, EmitError> {
        Err(EmitError::Unsupported("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<VariantEmitter, EmitError> {
        Ok(VariantEmitter {
            head: constructor(name, variant),
            args: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Unsupported, EmitError> {
        Err(EmitError::Unsupported("map"))
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<StructEmitter, EmitError> {
        let unwrap = self.unwrap.is_some_and(|(helper, seen)| {
            let matched = helper == name;
            seen.set(matched);
            matched
        });
        Ok(StructEmitter {
            type_name: name,
            unwrap,
            fields: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<VariantEmitter, EmitError> {
        Ok(VariantEmitter {
            head: constructor(name, variant),
            args: Vec::with_capacity(len),
        })
    }
}

/// A `Nat`-shaped literal for a nonnegative value and an ascribed `Int` for a
/// negative one: `Amount.lit` carries an `Int` [CR#107.1], and `-1` alone is
/// `Neg.neg` at whatever type unification lands on.
fn int_literal(value: i64) -> String {
    if value < 0 { format!("({value} : Int)") } else { value.to_string() }
}

/// A Lean list literal.
pub struct SeqEmitter {
    items: Vec<String>,
}

impl ser::SerializeSeq for SeqEmitter {
    type Ok = String;
    type Error = EmitError;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), EmitError> {
        self.items.push(value.serialize(LeanSerializer::plain())?);
        Ok(())
    }

    fn end(self) -> Result<String, EmitError> {
        Ok(format!("[{}]", self.items.join(", ")))
    }
}

impl ser::SerializeTuple for SeqEmitter {
    type Ok = String;
    type Error = EmitError;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), EmitError> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    /// A Rust tuple is Lean's `Prod` (nested right-associatively for more
    /// than two elements), spelled `(a, b)`, never a list — `Vec<(Option
    /// Cost, Instruction)>`'s `Instruction::ChooseModes.modes` field
    /// (Vote's `chooseModes`) is the case that exposed this: emitting `[a,
    /// b]` here type-mismatches Lean's `Prod` expectation.
    fn end(self) -> Result<String, EmitError> {
        Ok(format!("({})", self.items.join(", ")))
    }
}

/// A constructor applied to its fields, in declaration order — which is
/// constructor argument order (`docs/decisions/semantics-v2.md` §2).
pub struct VariantEmitter {
    head: String,
    args: Vec<String>,
}

impl ser::SerializeStructVariant for VariantEmitter {
    type Ok = String;
    type Error = EmitError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _: &'static str,
        value: &T,
    ) -> Result<(), EmitError> {
        self.args.push(value.serialize(LeanSerializer::plain())?);
        Ok(())
    }

    fn end(self) -> Result<String, EmitError> {
        Ok(application(self.head.clone(), &self.args))
    }
}

impl ser::SerializeTupleVariant for VariantEmitter {
    type Ok = String;
    type Error = EmitError;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), EmitError> {
        self.args.push(value.serialize(LeanSerializer::plain())?);
        Ok(())
    }

    fn end(self) -> Result<String, EmitError> {
        Ok(application(self.head.clone(), &self.args))
    }
}

/// A structure instance, ascribed so it elaborates without an expected type.
pub struct StructEmitter {
    type_name: &'static str,
    /// Whether this is a `SupportsMacros` helper standing for the enclosing
    /// constructor's fields, in which case the rendering is the argument list.
    unwrap: bool,
    fields: Vec<String>,
}

impl ser::SerializeStruct for StructEmitter {
    type Ok = String;
    type Error = EmitError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), EmitError> {
        let rendered = value.serialize(LeanSerializer::plain())?;
        self.fields.push(if self.unwrap {
            rendered
        } else {
            format!("{} := {rendered}", lean_field(key))
        });
        Ok(())
    }

    fn end(self) -> Result<String, EmitError> {
        if self.unwrap {
            return Ok(self.fields.join(" "));
        }
        Ok(format!(
            "({{ {} }} : {LEAN_NAMESPACE}.{})",
            self.fields.join(", "),
            self.type_name
        ))
    }
}

/// The compound emitters the mirror never reaches; constructing one is
/// already the error.
pub enum Unsupported {}

impl ser::SerializeTupleStruct for Unsupported {
    type Ok = String;
    type Error = EmitError;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, _: &T) -> Result<(), EmitError> {
        match *self {}
    }

    fn end(self) -> Result<String, EmitError> {
        match self {}
    }
}

impl ser::SerializeMap for Unsupported {
    type Ok = String;
    type Error = EmitError;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, _: &T) -> Result<(), EmitError> {
        match *self {}
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, _: &T) -> Result<(), EmitError> {
        match *self {}
    }

    fn end(self) -> Result<String, EmitError> {
        match self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abilities::Characteristics;
    use crate::card::CardFace;
    use crate::phrase::Amount;
    use crate::words::CardType;
    use crate::words::Color;
    use crate::words::ColorOrColorless;
    use crate::words::Delta;
    use crate::words::Determiner;
    use crate::words::ManaSymbol;
    use crate::words::SimpleManaSymbol;
    use crate::words::Subtype;

    /// A creature's characteristics with every optional slot absent.
    fn bare(name: &str) -> Characteristics {
        Characteristics {
            name: Some(name.to_owned()),
            cost: None,
            colors: vec![],
            supertypes: vec![],
            types: vec![CardType::Creature],
            subtypes: vec![],
            text: vec![],
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        }
    }

    #[test]
    fn a_nonnegative_number_is_a_bare_literal_and_a_negative_one_an_ascribed_int() {
        assert_eq!(term(&3u32).unwrap(), "3");
        assert_eq!(term(&0u32).unwrap(), "0");
        assert_eq!(term(&3i32).unwrap(), "3");
        assert_eq!(term(&-3i32).unwrap(), "(-3 : Int)");
    }

    /// `Amount.lit` carries an `Int`, and a printed negative face is spellable
    /// (Spinal Parasite's −1/−1), so the negative literal has to elaborate at
    /// `Int` in argument position.
    #[test]
    fn a_negative_amount_literal_elaborates_at_int() {
        assert_eq!(
            term(&Amount::Lit { value: -1 }).unwrap(),
            "(Semantics.Amount.lit (-1 : Int))"
        );
    }

    #[test]
    fn a_bool_is_a_lean_bool() {
        assert_eq!(term(&true).unwrap(), "true");
        assert_eq!(term(&false).unwrap(), "false");
    }

    #[test]
    fn an_option_is_some_or_none_and_some_is_parenthesised() {
        assert_eq!(term(&Option::<u32>::None).unwrap(), "none");
        assert_eq!(term(&Some(2u32)).unwrap(), "(some 2)");
        assert_eq!(
            term(&Some(Amount::Lit { value: 2 })).unwrap(),
            "(some (Semantics.Amount.lit 2))"
        );
    }

    #[test]
    fn a_vec_is_a_lean_list() {
        assert_eq!(term(&Vec::<u32>::new()).unwrap(), "[]");
        assert_eq!(term(&vec![1u32, 2, 3]).unwrap(), "[1, 2, 3]");
        assert_eq!(
            term(&vec![CardType::Creature, CardType::Land]).unwrap(),
            "[Semantics.CardType.creature, Semantics.CardType.land]"
        );
        assert_eq!(
            term(&vec![vec![ColorOrColorless::Colorless]]).unwrap(),
            "[[Semantics.ColorOrColorless.colorless]]"
        );
    }

    /// A Rust tuple is Lean's `Prod`, spelled `(a, b)` — never a list. The
    /// distinction is load-bearing: `Instruction::ChooseModes.modes` is a
    /// `Vec<(Option<Cost>, Instruction)>`, and emitting `[a, b]` for its
    /// elements type-mismatches Lean's `Prod` expectation (found via `Face A
    /// Villainous Choice`'s canon card, the first to invoke `chooseModes`
    /// through a real card).
    #[test]
    fn a_tuple_is_a_lean_pair_not_a_list() {
        assert_eq!(term(&(1u32, true)).unwrap(), "(1, true)");
        assert_eq!(
            term(&(Option::<u32>::None, CardType::Creature)).unwrap(),
            "(none, Semantics.CardType.creature)"
        );
        assert_eq!(
            term(&vec![(Option::<u32>::None, CardType::Creature)]).unwrap(),
            "[(none, Semantics.CardType.creature)]"
        );
    }

    #[test]
    fn a_string_is_quoted_and_escaped() {
        assert_eq!(term(&"Bear").unwrap(), "\"Bear\"");
        assert_eq!(term(&"a\"b\\c").unwrap(), "\"a\\\"b\\\\c\"");
        assert_eq!(term(&"a\nb").unwrap(), "\"a\\nb\"");
        assert_eq!(string_literal("\u{7}"), "\"\\x07\"");
    }

    #[test]
    fn a_unit_variant_is_a_qualified_constructor_and_a_payload_variant_is_applied() {
        assert_eq!(term(&Color::Green).unwrap(), "Semantics.Color.green");
        assert_eq!(
            term(&Subtype::Of {
                host: CardType::Creature,
                label: "Bear".into()
            })
            .unwrap(),
            "(Semantics.Subtype.of Semantics.CardType.creature \"Bear\")"
        );
        assert_eq!(
            term(&ManaSymbol::Simple {
                symbol: SimpleManaSymbol::Generic { amount: 1 }
            })
            .unwrap(),
            "(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 1))"
        );
    }

    #[test]
    fn a_struct_is_an_ascribed_structure_instance() {
        let face = CardFace {
            characteristics: Characteristics {
                power: Some(Amount::Lit { value: 2 }),
                toughness: Some(Amount::Lit { value: 2 }),
                ..bare("Grizzly Bears")
            },
            choices: vec![],
        };
        let rendered = term(&face).unwrap();
        assert!(
            rendered.starts_with("({ characteristics := ({ name := (some \"Grizzly Bears\")"),
            "{rendered}"
        );
        assert!(
            rendered.ends_with("choices := [] } : Semantics.CardFace)"),
            "{rendered}"
        );
        assert!(
            rendered.contains("} : Semantics.Characteristics)"),
            "the nested structure carries its own ascription: {rendered}"
        );
    }

    /// The three escapes the mirror documents: `Self` is a Rust keyword no raw
    /// identifier escapes, `Delta` is the one generic type, and `Amount.lit`
    /// is the number carrier.
    #[test]
    fn the_three_named_escapes_render_as_their_lean_spellings() {
        assert_eq!(
            term(&Determiner::Self_).unwrap(),
            "Semantics.Determiner.self"
        );
        assert_eq!(
            term(&Delta::Up { amount: 2u32 }).unwrap(),
            "(Semantics.Delta.up 2)"
        );
        assert_eq!(
            term(&Amount::Lit { value: 3 }).unwrap(),
            "(Semantics.Amount.lit 3)"
        );
    }

    #[test]
    fn the_name_mapping_round_trips_lean_spellings() {
        for lean in ["hasType", "return_", "self", "modalDfc", "variable"] {
            assert_eq!(lean_variant(&rust_variant(lean)), lean);
        }
        for lean in ["from_", "type", "by_", "callerWidth", "while_", "name"] {
            assert_eq!(lean_field(&rust_field(lean)), lean);
        }
        // The serde name, not the Rust ident, is what the emitter receives.
        assert_eq!(lean_variant("Self"), "self");
        assert_eq!(lean_field("type"), "type");
    }

    #[test]
    fn a_card_ident_is_snake_case_and_collisions_get_a_stable_suffix() {
        assert_eq!(card_ident("Grizzly Bears"), "card_grizzly_bears");
        assert_eq!(
            card_ident("Elesh Norn, Grand Cenobite"),
            "card_elesh_norn_grand_cenobite"
        );
        assert_eq!(card_ident("Fire // Ice"), "card_fire_ice");
        assert_eq!(card_ident("!!!"), "card");

        let assigned = assign_idents(["B a", "b-a", "A"]);
        assert_eq!(
            assigned,
            vec![
                ("A".to_owned(), "card_a".to_owned()),
                ("B a".to_owned(), "card_b_a".to_owned()),
                ("b-a".to_owned(), "card_b_a_2".to_owned()),
            ]
        );
    }

    #[test]
    fn a_module_carries_a_def_and_a_decide_theorem_per_card() {
        let card = Card::SingleFaced {
            face: CardFace {
                characteristics: bare("Grizzly Bears"),
                choices: vec![],
            },
        };
        let module =
            render_module("Generated.Testing", &[("Grizzly Bears".to_owned(), &card)]).unwrap();
        assert!(module.source.contains("import Semantics.Check.Card"));
        assert!(module.source.contains("namespace Generated.Testing"));
        assert!(
            module
                .source
                .contains("def card_grizzly_bears : Semantics.Card :=")
        );
        assert!(module.source.contains(
            "theorem card_grizzly_bears_ok : Semantics.Card.check card_grizzly_bears = [] := by \
             decide"
        ));
        assert!(
            module.source.contains(
                "/-- info: [] -/\n#guard_msgs in\n#eval Semantics.Card.check card_grizzly_bears\n"
            ),
            "the guarded eval that names the refusals sits in the card's block:\n{}",
            module.source
        );
        let [entry] = module.cards.as_slice() else {
            panic!("one card, one entry");
        };
        assert_eq!(entry.ident, "card_grizzly_bears");
        let opened = module
            .source
            .lines()
            .nth(entry.start_line - 1)
            .expect("the recorded line exists");
        assert_eq!(opened, "-- Grizzly Bears");
    }
}
