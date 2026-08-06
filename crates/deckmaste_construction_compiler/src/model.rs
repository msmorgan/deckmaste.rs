//! The construction declaration IR: plain data types describing a group of
//! constructions, produced by [`crate::parse::parse_group`] and consumed by
//! [`crate::validate`]. All fields are `pub`; this module has no invariants
//! of its own — validation lives in `validate.rs`.

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: proc_macro2::Span,
}

impl<T> Spanned<T> {
    pub fn call_site(value: T) -> Self {
        Self {
            value,
            span: proc_macro2::Span::call_site(),
        }
    }
}

// `proc_macro2::Span` has no `PartialEq`/`Eq` impl, so these can't be
// derived. Span is provenance metadata, not data, so equality is defined
// over `value` alone — this is what lets `FieldKind`/`SelectionPromise`
// derive `PartialEq, Eq` per the model contract.
impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Spanned<T> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupDeclaration {
    pub name: Spanned<String>,
    pub constructions: Vec<ConstructionDeclaration>,
    pub elements: Vec<ElementDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionDeclaration {
    pub id: Spanned<String>,
    pub category: Spanned<String>,
    pub internal: bool,
    pub ast: AstShape,
    pub constraints: Vec<Constraint>,
    pub witnesses: Vec<WitnessDeclaration>,
    pub forms: Vec<FormDeclaration>,
    pub dominance: Vec<DominanceEdge>,
    pub selection: SelectionPromise,
    pub deserialize: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstShape {
    Bind {
        path: Spanned<String>,
        fields: Vec<FieldBinding>,
    },
    Own {
        name: Spanned<String>,
        fields: Vec<FieldBinding>,
    },
}

impl AstShape {
    #[must_use]
    pub fn fields(&self) -> &[FieldBinding] {
        match self {
            Self::Bind { fields, .. } | Self::Own { fields, .. } => fields,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldBinding {
    pub field: Spanned<String>,
    pub kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Subtree {
        category: Spanned<String>,
        boxed: bool,
    },
    Scalar {
        codec: Spanned<String>,
    },
    Sequence {
        element: Spanned<String>,
    },
    /// `opt <kind>`. Wraps exactly one non-Optional kind — the parser
    /// rejects `opt opt`, so nesting is unrepresentable in parsed input.
    Optional {
        inner: Box<FieldKind>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementDeclaration {
    pub name: Spanned<String>,
    pub bind_path: Option<Spanned<String>>,
    pub fields: Vec<FieldBinding>,
    pub variants: Vec<ElementVariantDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementVariantDeclaration {
    pub name: Spanned<String>,
    pub payload: FieldKind,
}

#[derive(Debug, Clone)]
pub struct FieldPath {
    pub segments: Vec<Spanned<String>>,
    pub span: proc_macro2::Span,
}

impl FieldPath {
    #[must_use]
    pub fn call_site(dotted: &str) -> Self {
        Self {
            segments: dotted
                .split('.')
                .map(|s| Spanned::call_site(s.to_owned()))
                .collect(),
            span: proc_macro2::Span::call_site(),
        }
    }

    /// The path as the author dotted it — the only rendering diagnostics
    /// and abstraction keys may use.
    #[must_use]
    pub fn dotted(&self) -> String {
        let names: Vec<&str> = self.segments.iter().map(|s| s.value.as_str()).collect();
        names.join(".")
    }
}

// Span is provenance, not data (same contract as `Spanned<T>`), and
// `proc_macro2::Span` has no `PartialEq` — so equality is segments-only.
impl PartialEq for FieldPath {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for FieldPath {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    Require(Spanned<Predicate>),
    DeriveFeature {
        target: FieldPath,
        combinator: Spanned<String>,
        args: Vec<FieldPath>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate {
    LenAtLeast {
        path: FieldPath,
        min: u32,
    },
    LenIs {
        path: FieldPath,
        len: u32,
    },
    In {
        path: FieldPath,
        allowed: Vec<String>,
    },
    IsSome {
        path: FieldPath,
    },
    IsNone {
        path: FieldPath,
    },
    All(Vec<Predicate>),
    Any(Vec<Predicate>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessDeclaration {
    pub name: Spanned<String>,
    pub class: WitnessClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessClass {
    Stored {
        path: FieldPath,
    },
    Derived {
        combinator: Spanned<String>,
        args: Vec<FieldPath>,
    },
    Free {
        ty: Spanned<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormDeclaration {
    pub name: Spanned<String>,
    pub ordinal: Spanned<u16>,
    pub surface: Vec<SurfaceAtom>,
    pub guard: Option<Spanned<Predicate>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceAtom {
    Literal(Spanned<String>),
    Hole(FieldPath),
    Lexeme(FieldPath),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DominanceEdge {
    pub winner: Spanned<String>,
    pub loser: Spanned<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionPromise {
    Packed,
    Unique,
}

pub const KNOWN_COMBINATORS: &[&str] = &["from_first", "fixed"];

/// `snake_case` to `PascalCase`. Shared between `validate.rs`'s EC006
/// generated-name collision check and `emit.rs`'s struct-name rendering —
/// they must agree on exactly the same transform, or a collision the
/// checker finds could differ from the one the emitter actually produces
/// (or vice versa), so the algorithm lives once.
#[must_use]
pub fn pascal_case(snake: &str) -> String {
    let mut out = String::new();
    for part in snake.split('_') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

/// `PascalCase` (including ordinary acronym boundaries) to `snake_case`.
/// Shared by emitted bound-enum builder names and their collision checks.
#[must_use]
pub fn snake_case(pascal: &str) -> String {
    let chars: Vec<char> = pascal.chars().collect();
    let mut out = String::new();
    for (index, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let previous_is_lower_or_digit =
                index > 0 && (chars[index - 1].is_lowercase() || chars[index - 1].is_ascii_digit());
            let next_is_lower = chars.get(index + 1).is_some_and(|next| next.is_lowercase());
            if index > 0 && (previous_is_lower_or_digit || next_is_lower) {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_path_equality_ignores_spans() {
        let a = FieldPath::call_site("members.last.comma");
        let mut b = FieldPath::call_site("members.last.comma");
        b.span = proc_macro2::Span::mixed_site();
        b.segments[0].span = proc_macro2::Span::mixed_site();
        assert_eq!(a, b);
        assert_ne!(a, FieldPath::call_site("members.comma"));
        assert_eq!(a.dotted(), "members.last.comma");
    }

    #[test]
    fn pascal_variant_names_become_stable_builder_suffixes() {
        assert_eq!(snake_case("EventClause"), "event_clause");
        assert_eq!(snake_case("URLValue"), "url_value");
        assert_eq!(snake_case("PowerToughness"), "power_toughness");
    }
}
