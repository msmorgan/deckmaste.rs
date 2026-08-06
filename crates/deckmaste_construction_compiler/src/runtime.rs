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

    fn begin_form(
        &mut self,
        _construction: &'static str,
        _form: &'static str,
        _ordinal: u16,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end_form(&mut self, _construction: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error>;

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>;

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>;

    fn begin_sequence(&mut self, _field: &'static str, _len: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    fn sequence_member(&mut self, _field: &'static str, _index: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end_sequence(&mut self, _field: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn optional(&mut self, _field: &'static str, _present: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn bound_value<T: std::any::Any>(
        &mut self,
        _element: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

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
    pub constructions: &'static [ConstructionData],
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
    pub fields: &'static [FieldData],
    pub witnesses: &'static [WitnessData],
    pub deserialize: bool,
    pub selection_unique: bool,
    pub dominates: &'static [&'static str],
    pub forms: &'static [FormData],
    pub erased_builder: Option<ErasedBuilder>,
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
