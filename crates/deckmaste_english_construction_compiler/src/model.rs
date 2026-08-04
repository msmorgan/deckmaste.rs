//! The construction declaration IR: plain data types describing a group of
//! constructions, produced upstream (spike/macro layer) and consumed by
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

#[derive(Debug, Clone)]
pub struct GroupDeclaration {
    pub name: Spanned<String>,
    pub constructions: Vec<ConstructionDeclaration>,
    pub elements: Vec<ElementDeclaration>,
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
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
    pub fn fields(&self) -> &[FieldBinding] {
        match self {
            Self::Bind { fields, .. } | Self::Own { fields, .. } => fields,
        }
    }
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct ElementDeclaration {
    pub name: Spanned<String>,
    pub fields: Vec<FieldBinding>,
}

#[derive(Debug, Clone)]
pub struct FieldPath {
    pub segments: Vec<String>,
    pub span: proc_macro2::Span,
}

impl FieldPath {
    pub fn call_site(dotted: &str) -> Self {
        Self {
            segments: dotted.split('.').map(str::to_owned).collect(),
            span: proc_macro2::Span::call_site(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Require(Spanned<Predicate>),
    DeriveFeature {
        target: FieldPath,
        combinator: Spanned<String>,
        args: Vec<FieldPath>,
    },
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct WitnessDeclaration {
    pub name: Spanned<String>,
    pub class: WitnessClass,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FormDeclaration {
    pub name: Spanned<String>,
    pub ordinal: Spanned<u16>,
    pub surface: Vec<SurfaceAtom>,
    pub guard: Option<Spanned<Predicate>>,
}

#[derive(Debug, Clone)]
pub enum SurfaceAtom {
    Literal(Spanned<String>),
    Hole(FieldPath),
    Lexeme(FieldPath),
}

#[derive(Debug, Clone)]
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
