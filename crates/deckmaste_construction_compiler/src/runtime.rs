//! Span-free declaration-as-data types. The macro emits one `GroupData`
//! static per `constructions!` group; validation, registry rows, `inspect`
//! metadata, and the provenance gate all read this one projection, which is
//! how "all projections share one source" stays demonstrable. Grows fields
//! as consumers land (registry: Milestone 3); it never grows spans —
//! provenance stays in the compile-time IR.

/// A `require` clause an ingress value failed. One error class for every
/// generated door — `try_new` and validating deserialization — and one
/// definition for every group: minted here rather than per-`constructions!`
/// invocation because a per-group struct of the same name collides
/// (`error[E0252]`) the moment two groups share a module. Consumers already
/// depend on this crate for every runtime type the emitted `GroupData`
/// static names by absolute path, so referring to this type the same way
/// adds no new obligation.
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationViolation {
    pub construction: &'static str,
    pub requirement: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupData {
    pub name: &'static str,
    pub elements: &'static [&'static str],
    pub element_data: &'static [ElementData],
    pub constructions: &'static [ConstructionData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementData {
    pub name: &'static str,
    pub bind_path: Option<&'static str>,
    pub fields: &'static [FieldData],
    pub variants: &'static [ElementVariantData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementVariantData {
    pub name: &'static str,
    pub payload: FieldKindData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructionData {
    pub id: &'static str,
    pub category: &'static str,
    pub internal: bool,
    /// `Some(type name)` for own-mode constructions; `None` for bind mode,
    /// which has no owned struct at all — the emitter instead produces a
    /// checked `build_<id>` builder and a `parts_<id>` destructurer
    /// (Milestone 3 Task 2).
    pub own_type: Option<&'static str>,
    pub bind_path: Option<&'static str>,
    pub fields: &'static [FieldData],
    pub witnesses: &'static [WitnessData],
    pub deserialize: bool,
    pub selection_unique: bool,
    pub dominates: &'static [&'static str],
    pub forms: &'static [FormData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldData {
    pub name: &'static str,
    pub kind: FieldKindData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKindData {
    Subtree {
        category: &'static str,
        boxed: bool,
    },
    Scalar {
        codec: &'static str,
    },
    Sequence {
        element: &'static str,
    },
    /// `opt <kind>`; the inner reference is const-promoted in the emitted
    /// static. Non-nesting is a parser guarantee, mirrored here by data.
    Optional {
        inner: &'static FieldKindData,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WitnessData {
    pub name: &'static str,
    pub class: WitnessClassData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessClassData {
    /// Dotted path, exactly as the author wrote it.
    Stored {
        path: &'static str,
    },
    Derived {
        combinator: &'static str,
    },
    Free {
        ty: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormData {
    pub name: &'static str,
    pub ordinal: u16,
    pub guarded: bool,
    pub atoms: &'static [AtomData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomData {
    Literal(&'static str),
    /// Dotted path, exactly as the author wrote it.
    Hole(&'static str),
    Lexeme(&'static str),
}
