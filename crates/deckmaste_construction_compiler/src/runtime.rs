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

#[derive(Debug, PartialEq, Eq)]
pub enum LinearizationError<E> {
    NoMatchingConstruction {
        group: &'static str,
    },
    MultipleMatchingConstructions {
        group: &'static str,
        first: &'static str,
        second: &'static str,
    },
    NoMatchingForm {
        construction: &'static str,
    },
    MultipleMatchingForms {
        construction: &'static str,
        first: u16,
        second: u16,
    },
    Visitor(E),
}

/// Structural event sink used by declaration-emitted value linearizers.
/// Generic value callbacks preserve the concrete Rust type until the
/// consumer chooses whether to downcast or handle it generically.
pub trait LinearizationVisitor {
    type Error;

    /// Starts one selected declaration form.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the event cannot
    /// be accepted.
    fn begin_form(
        &mut self,
        _construction: &'static str,
        _form: &'static str,
        _ordinal: u16,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Ends the selected declaration form.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the event cannot
    /// be accepted.
    fn end_form(&mut self, _construction: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visits a literal surface token.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the token cannot
    /// be accepted.
    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error>;

    /// Visits a typed subtree field.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the subtree
    /// cannot be accepted.
    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>;

    /// Visits a typed scalar field.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the scalar cannot
    /// be accepted.
    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>;

    /// Visits a typed lexical identity. `provider` is an opaque language-local
    /// adapter name; `value_type` documents the concrete Rust type retained in
    /// `value`.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the identity
    /// cannot be accepted.
    fn identity<T: std::any::Any>(
        &mut self,
        _provider: &'static str,
        _value_type: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        panic!("linearization visitor does not support identity fields")
    }

    /// Visits a sequence scalar derived from member position and count.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the derived value
    /// cannot be accepted.
    fn derived_sequence_scalar(
        &mut self,
        _field: &'static str,
        _codec: &'static str,
        _index: usize,
        _len: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Starts a sequence field.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the sequence
    /// cannot be accepted.
    fn begin_sequence(&mut self, _field: &'static str, _len: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Starts one member of the current sequence field.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the member cannot
    /// be accepted.
    fn sequence_member(&mut self, _field: &'static str, _index: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Ends the current sequence field.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the event cannot
    /// be accepted.
    fn end_sequence(&mut self, _field: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visits the presence state of an optional field.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the state cannot
    /// be accepted.
    fn optional(&mut self, _field: &'static str, _present: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visits a value supplied by a bound element declaration.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the value cannot
    /// be accepted.
    fn bound_value<T: std::any::Any>(
        &mut self,
        _element: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visits a stored form witness.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the witness cannot
    /// be accepted.
    fn stored_witness<T: std::any::Any>(
        &mut self,
        _name: &'static str,
        _path: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// One type-erased field value supplied to a declaration-emitted builder.
/// Values remain owned so generated lowering can move boxed subtrees and
/// sequences into the final construction without cloning them.
pub type ErasedValue = Box<dyn std::any::Any>;

pub type ErasedBuilder = fn(Vec<ErasedValue>) -> Result<ErasedValue, ErasedBuildError>;
pub type ErasedSequenceBuilder = fn(Vec<ErasedValue>) -> Result<ErasedValue, ErasedBuildError>;
pub type ErasedProjector = fn(ErasedValue) -> Result<ErasedValue, ErasedBuildError>;
pub type ErasedRecognizer = fn(&dyn std::any::Any) -> bool;

#[derive(Debug, PartialEq, Eq)]
pub enum ErasedBuildError {
    MissingField {
        owner: &'static str,
        field: &'static str,
    },
    WrongFieldType {
        owner: &'static str,
        field: &'static str,
        expected: &'static str,
    },
    ExtraFields {
        owner: &'static str,
    },
    Declaration(DeclarationViolation),
}

/// Consumes the next positional erased field and restores its declared type.
/// Emitted builders call this in declaration order, which keeps all
/// downcasts in compiler-generated code and out of language adapters.
///
/// # Errors
///
/// Returns [`ErasedBuildError::MissingField`] when there is no next value, or
/// [`ErasedBuildError::WrongFieldType`] when it cannot be downcast to `T`.
pub fn take_erased<T: std::any::Any>(
    values: &mut impl Iterator<Item = ErasedValue>,
    owner: &'static str,
    field: &'static str,
    expected: &'static str,
) -> Result<T, ErasedBuildError> {
    let value = values
        .next()
        .ok_or(ErasedBuildError::MissingField { owner, field })?;
    value
        .downcast::<T>()
        .map(|value| *value)
        .map_err(|_| ErasedBuildError::WrongFieldType {
            owner,
            field,
            expected,
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupData {
    pub name: &'static str,
    pub elements: &'static [&'static str],
    pub element_data: &'static [ElementData],
    /// Typed record rebuild schemas. Lenses are deliberately separate from
    /// semantic elements: they do not mint chart categories or surface atoms.
    pub lenses: &'static [LensData],
    pub constructions: &'static [ConstructionData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LensData {
    pub name: &'static str,
    pub owner_type: &'static str,
    pub fields: &'static [LensFieldData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LensFieldData {
    pub name: &'static str,
    pub kind: LensFieldKindData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensFieldKindData {
    Value { value_type: &'static str },
    Optional { value_type: &'static str },
    Vector { element_type: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LensApplicationData {
    pub owner: &'static str,
    pub source: Option<&'static str>,
    pub edits: &'static [LensEditData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LensEditData {
    pub target: &'static str,
    pub value: &'static str,
    pub kind: LensEditKindData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensEditKindData {
    Focus,
    Prepend,
    Append,
}

#[allow(
    unpredictable_function_pointer_comparisons,
    reason = "metadata equality is used for fixture structure; builders are never selected by address"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementData {
    pub name: &'static str,
    pub bind_path: Option<&'static str>,
    pub fields: &'static [FieldData],
    pub variants: &'static [ElementVariantData],
    /// One entry for a struct-shaped element, or one per enum variant in
    /// declaration order.
    pub erased_builders: &'static [ErasedBuilder],
    pub erased_sequence_builder: Option<ErasedSequenceBuilder>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementVariantData {
    pub name: &'static str,
    pub payload: FieldKindData,
}

#[allow(
    unpredictable_function_pointer_comparisons,
    reason = "metadata equality is used for fixture structure; builders are never selected by address"
)]
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
    pub lens: Option<LensApplicationData>,
    pub projection_variant: Option<&'static str>,
    pub fields: &'static [FieldData],
    pub witnesses: &'static [WitnessData],
    pub deserialize: bool,
    pub selection_unique: bool,
    pub dominates: &'static [&'static str],
    pub dominated_by: &'static [&'static str],
    pub forms: &'static [FormData],
    pub requirements: &'static [RequirementData],
    pub recognition_requirements: &'static [RequirementData],
    pub feature_combinators: &'static [FeatureCombinatorData],
    pub evidence: Option<EvidenceData>,
    /// Semantic assembly used only while chart feature validation is still
    /// in progress. This applies authored `require` clauses and the bind
    /// adapter, but deliberately does not claim the completed value satisfies
    /// an inverse form.
    pub erased_partial_builder: Option<ErasedBuilder>,
    /// Final type-erased ingress. This crosses the same checked typed builder
    /// as public construction, including complete-value inverse recognition.
    pub erased_builder: Option<ErasedBuilder>,
    pub erased_projector: Option<ErasedProjector>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceData {
    pub kind: EvidenceKindData,
    pub label: &'static str,
    pub source: EvidenceSourceData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKindData {
    Guard,
    Feature,
    Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceSourceData {
    Requirement(&'static str),
    Output(&'static str),
    Field(&'static str),
    Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequirementData {
    pub description: &'static str,
    pub predicate: PredicateData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateData {
    LenAtLeast {
        path: &'static str,
        min: u32,
    },
    LenIs {
        path: &'static str,
        len: u32,
    },
    In {
        path: &'static str,
        allowed: &'static [&'static str],
    },
    IsSome {
        path: &'static str,
    },
    IsNone {
        path: &'static str,
    },
    All(&'static [PredicateData]),
    Any(&'static [PredicateData]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureCombinatorData {
    pub target: &'static str,
    pub combinator: &'static str,
    pub args: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldData {
    pub name: &'static str,
    pub kind: FieldKindData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKindData {
    Identity {
        value_type: &'static str,
        provider: &'static str,
    },
    Subtree {
        category: &'static str,
        boxed: bool,
    },
    Scalar {
        codec: &'static str,
    },
    TypedScalar {
        value_type: &'static str,
        codec: &'static str,
    },
    /// Surface-only scalar omitted from a bound semantic element and derived
    /// from sequence position during linearization.
    SurfaceScalar {
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

#[allow(
    unpredictable_function_pointer_comparisons,
    reason = "metadata equality is used for fixture structure; recognizers are never selected by address"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormData {
    pub name: &'static str,
    pub ordinal: u16,
    pub guarded: bool,
    /// Generated semantic guard used by lowering after typed construction.
    /// Field-feature guards remain enforced during reduction as well.
    pub erased_recognizer: Option<ErasedRecognizer>,
    pub atoms: &'static [AtomData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomData {
    Literal(&'static str),
    /// Dotted path, exactly as the author wrote it.
    Hole(&'static str),
    Lexeme(&'static str),
    Identity(&'static str),
}
