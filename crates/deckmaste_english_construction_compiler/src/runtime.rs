//! Span-free declaration-as-data types. The macro emits one `GroupData`
//! static per `constructicon!` group; validation, registry rows, `inspect`
//! metadata, and the provenance gate all read this one projection, which is
//! how "all projections share one source" stays demonstrable. Grows fields
//! as consumers land (registry: Milestone 3); it never grows spans —
//! provenance stays in the compile-time IR.

/// A `require` clause an ingress value failed. One error class for every
/// generated door — `try_new` and validating deserialization — and one
/// definition for every group: minted here rather than per-`constructicon!`
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
    pub constructions: &'static [ConstructionData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructionData {
    pub id: &'static str,
    pub category: &'static str,
    pub internal: bool,
    /// `Some(type name)` for own-mode constructions; `None` for bind mode
    /// (whose emission lands with the chart adapter).
    pub own_type: Option<&'static str>,
    pub bind_path: Option<&'static str>,
    pub deserialize: bool,
    pub selection_unique: bool,
    pub dominates: &'static [&'static str],
    pub forms: &'static [FormData],
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
