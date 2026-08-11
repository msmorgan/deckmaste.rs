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

/// An ordered sequence that statically contains at least one value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmpty<T>(Vec<T>);

impl<T> NonEmpty<T> {
    #[must_use]
    pub fn from_first(first: T) -> Self {
        Self(vec![first])
    }

    #[must_use]
    ///
    /// # Panics
    ///
    /// Only if the private nonempty invariant has been violated by unsafe
    /// code; every safe constructor preserves at least one member.
    pub fn first(&self) -> &T {
        self.0
            .first()
            .expect("NonEmpty construction preserves one member")
    }

    #[must_use]
    pub fn rest(&self) -> &[T] {
        &self.0[1..]
    }

    #[must_use]
    ///
    /// # Panics
    ///
    /// Only if the private nonempty invariant has been violated by unsafe
    /// code; every safe constructor preserves at least one member.
    pub fn last(&self) -> &T {
        self.0
            .last()
            .expect("NonEmpty construction preserves one member")
    }

    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> TryFrom<Vec<T>> for NonEmpty<T> {
    type Error = EmptySequence;

    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() { Err(EmptySequence) } else { Ok(Self(values)) }
    }
}

impl<T> AsRef<[T]> for NonEmpty<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> std::ops::Deref for NonEmpty<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> IntoIterator for NonEmpty<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NonEmpty<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: serde::Serialize> serde::Serialize for NonEmpty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for NonEmpty<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values = Vec::<T>::deserialize(deserializer)?;
        Self::try_from(values).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptySequence;

impl std::fmt::Display for EmptySequence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a nonempty sequence requires at least one member")
    }
}

impl std::error::Error for EmptySequence {}

/// One continuation in a separated nonempty sequence.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Separated<T, S> {
    separator: S,
    value: T,
}

impl<T, S> Separated<T, S> {
    #[must_use]
    pub const fn new(separator: S, value: T) -> Self {
        Self { separator, value }
    }

    #[must_use]
    pub const fn separator(&self) -> &S {
        &self.separator
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn into_parts(self) -> (S, T) {
        (self.separator, self.value)
    }
}

/// A nonempty sequence whose first value has no separator and whose every
/// continuation has exactly one typed separator.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SeparatedNonEmpty<T, S> {
    first: T,
    rest: Vec<Separated<T, S>>,
}

pub struct SeparatedNonEmptyIter<'a, T, S> {
    first: Option<&'a T>,
    rest: std::slice::Iter<'a, Separated<T, S>>,
}

impl<'a, T, S> Iterator for SeparatedNonEmptyIter<'a, T, S> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.first
            .take()
            .or_else(|| self.rest.next().map(Separated::value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T, S> ExactSizeIterator for SeparatedNonEmptyIter<'_, T, S> {
    fn len(&self) -> usize {
        usize::from(self.first.is_some()) + self.rest.len()
    }
}

impl<T, S> std::iter::FusedIterator for SeparatedNonEmptyIter<'_, T, S> {}

impl<T, S> SeparatedNonEmpty<T, S> {
    #[must_use]
    pub const fn new(first: T, rest: Vec<Separated<T, S>>) -> Self {
        Self { first, rest }
    }

    #[must_use]
    pub const fn from_first(first: T) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    #[must_use]
    pub const fn first(&self) -> &T {
        &self.first
    }

    #[must_use]
    pub fn rest(&self) -> &[Separated<T, S>] {
        &self.rest
    }

    #[must_use]
    pub fn last(&self) -> &T {
        self.rest.last().map_or(&self.first, Separated::value)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rest.len() + 1
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> SeparatedNonEmptyIter<'_, T, S> {
        SeparatedNonEmptyIter {
            first: Some(&self.first),
            rest: self.rest.iter(),
        }
    }

    #[must_use]
    pub fn into_parts(self) -> (T, Vec<Separated<T, S>>) {
        (self.first, self.rest)
    }
}

impl<'a, T, S> IntoIterator for &'a SeparatedNonEmpty<T, S> {
    type Item = &'a T;
    type IntoIter = SeparatedNonEmptyIter<'a, T, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
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

/// Failure to select exactly one declared canonical form for a typed value.
/// This is the visitor-independent half of [`LinearizationError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormSelectionError {
    NoMatchingForm {
        construction: &'static str,
    },
    MultipleMatchingForms {
        construction: &'static str,
        first: u16,
        second: u16,
    },
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

    /// Visits a subtree through the stable role-bearing construction
    /// projection. Generated linearizers call this beside [`Self::subtree`];
    /// ordinary renderers can ignore it, while projection consumers never
    /// need to recover a role from Rust field layout.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// subtree cannot be accepted.
    fn projected_subtree(
        &mut self,
        _role: &'static str,
        _category: &'static str,
        _value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Starts one variant of a declaration-owned sum role. Payload events are
    /// emitted between this callback and [`Self::end_sum_variant`].
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected sum
    /// cannot be accepted.
    fn begin_sum_variant(
        &mut self,
        _role: &'static str,
        _element: &'static str,
        _variant: &'static str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Ends the current declaration-owned sum role.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected sum
    /// cannot be closed.
    fn end_sum_variant(&mut self, _role: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Starts one declaration-owned record product. Field-role events are
    /// emitted between this callback and [`Self::end_product`].
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// product cannot be accepted.
    fn begin_product(
        &mut self,
        _role: &'static str,
        _element: &'static str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Ends the current declaration-owned record product.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// product cannot be closed.
    fn end_product(&mut self, _role: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

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

    /// Visits a scalar through its stable declared role and codec.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// scalar cannot be accepted.
    fn projected_scalar(
        &mut self,
        _role: &'static str,
        _codec: &'static str,
        _value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

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

    /// Visits a lexical identity through its stable declared role and
    /// provider.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// identity cannot be accepted.
    fn projected_identity(
        &mut self,
        _role: &'static str,
        _provider: &'static str,
        _value_type: &'static str,
        _value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        Ok(())
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

    /// Starts the stable projection of one declared sequence element.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// element cannot be accepted.
    fn projected_bound_value(
        &mut self,
        _element: &'static str,
        _value: &dyn ProjectionInput,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Records the stable variant identity of a declared sum element.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the variant
    /// cannot be accepted.
    fn element_variant(
        &mut self,
        _element: &'static str,
        _variant: &'static str,
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

    /// Retains one declaration-owned surface witness without exposing the
    /// Rust path used to store it as structural identity.
    ///
    /// # Errors
    ///
    /// Returns an implementation-defined visitor error when the projected
    /// witness cannot be accepted.
    fn projected_stored_witness(
        &mut self,
        _name: &'static str,
        _path: &'static str,
        _value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// An owned, comparable value that may cross the transient construction
/// projection boundary without exposing its Rust representation. The
/// declaration supplies the stable role plus scalar codec or identity
/// provider; this erased payload preserves typed equality and diagnostics.
pub trait ProjectionInput: std::any::Any + std::fmt::Debug + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T> ProjectionInput for T
where
    T: std::any::Any + std::fmt::Debug + Send + Sync,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait ProjectionValue: ProjectionInput {
    fn clone_projection_value(&self) -> Box<dyn ProjectionValue>;
    fn projection_eq(&self, other: &dyn ProjectionValue) -> bool;
}

impl<T> ProjectionValue for T
where
    T: std::any::Any + Clone + std::fmt::Debug + Eq + Send + Sync,
{
    fn clone_projection_value(&self) -> Box<dyn ProjectionValue> {
        Box::new(self.clone())
    }

    fn projection_eq(&self, other: &dyn ProjectionValue) -> bool {
        other.as_any().downcast_ref::<T>() == Some(self)
    }
}

/// Coerces one construction root or subtree into the borrowed erased input
/// protocol. Unlike scalar and identity payloads it need not be cloneable:
/// it is recursively projected while its typed owner is still borrowed.
#[must_use]
pub fn projection_input<T: ProjectionInput>(value: &T) -> &dyn ProjectionInput {
    value
}

/// Coerces one declaration field into the erased projection-value protocol
/// while preserving inference of the concrete referenced type.
#[must_use]
pub fn projection_value<T: ProjectionValue>(value: &T) -> &dyn ProjectionValue {
    value
}

/// One owned scalar or lexical identity payload in a construction
/// projection. Equality is the concrete value's typed equality, never a
/// serialized spelling or a Rust type/variant name.
pub struct OwnedProjectionValue(Box<dyn ProjectionValue>);

impl OwnedProjectionValue {
    #[must_use]
    pub fn new(value: &dyn ProjectionValue) -> Self {
        Self(value.clone_projection_value())
    }

    #[must_use]
    pub fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        self.0.as_ref().as_any().downcast_ref()
    }

    #[must_use]
    pub fn as_projection_value(&self) -> &dyn ProjectionValue {
        self.0.as_ref()
    }
}

impl Clone for OwnedProjectionValue {
    fn clone(&self) -> Self {
        Self(self.0.clone_projection_value())
    }
}

impl std::fmt::Debug for OwnedProjectionValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl PartialEq for OwnedProjectionValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.projection_eq(other.0.as_ref())
    }
}

impl Eq for OwnedProjectionValue {}

/// A declaration-owned scalar or lexical identity. The namespace is stable
/// declaration data; the erased payload retains typed value identity without
/// publishing Rust enum names or field layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectedAtom {
    /// Temporary grammatical adapter for a subtree category whose owning
    /// output family has not yet gained construction declarations. The
    /// category and parent role are declaration identities; the typed value
    /// remains opaque and carries no Rust layout into the spelling contract.
    FlatSubtree {
        category: &'static str,
        value: OwnedProjectionValue,
    },
    Scalar {
        codec: &'static str,
        value: OwnedProjectionValue,
    },
    Identity {
        provider: &'static str,
        value_type: &'static str,
        value: OwnedProjectionValue,
    },
    DerivedSequenceScalar {
        codec: &'static str,
        index: usize,
        len: usize,
    },
}

/// One named grammatical role in a selected construction form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectedValue {
    Construction(Box<ConstructionProjection>),
    Atom(ProjectedAtom),
    Optional(Option<Box<ProjectedValue>>),
    Sequence(ProjectedSequence),
    Variant(ProjectedVariant),
    Product(ProjectedProduct),
}

/// One selected alternative of a direct declaration-owned sum role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedVariant {
    pub element: &'static str,
    pub variant: &'static str,
    pub roles: std::collections::BTreeMap<&'static str, ProjectedValue>,
}

/// One direct declaration-owned record value. Its named roles are structural;
/// the bound Rust record layout is not part of the projection contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedProduct {
    pub element: &'static str,
    pub roles: std::collections::BTreeMap<&'static str, ProjectedValue>,
}

/// One member of a declaration-owned sequence. `element` and `variant` are
/// declaration identities; member storage layout is absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedMember {
    pub element: &'static str,
    pub variant: Option<&'static str>,
    pub roles: std::collections::BTreeMap<&'static str, ProjectedValue>,
}

/// A named sequence role and its declaration-owned members.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedSequence {
    pub role: &'static str,
    pub members: Vec<ProjectedMember>,
}

/// One retained surface witness. `path` is diagnostic provenance only; role
/// lookup and matching use `name`, never the Rust storage path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedWitness {
    pub path: &'static str,
    pub value: OwnedProjectionValue,
}

/// The transient, declaration-generated spelling boundary for one selected
/// English construction. Maps make role identity independent of declaration
/// field order; stable construction and form names replace serde type and
/// variant names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionProjection {
    pub category: &'static str,
    pub construction: &'static str,
    pub form: &'static str,
    pub ordinal: u16,
    pub roles: std::collections::BTreeMap<&'static str, ProjectedValue>,
    pub witnesses: std::collections::BTreeMap<&'static str, ProjectedWitness>,
    /// Literal atoms are retained as declaration-surface evidence. Matching
    /// keys on the stable form identity, so this vector is diagnostic and
    /// exactness metadata rather than a second grammar.
    pub literals: Vec<&'static str>,
}

/// Object-safe sink used by a declaration's erased projection entry point.
/// The ordinary generic visitor remains the renderer API; generated glue
/// mirrors its events here with stable role identity and erased typed values.
pub trait ProjectionSink {
    /// Starts the selected declaration form.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the sink cannot open this form.
    fn begin_form(
        &mut self,
        construction: &'static str,
        form: &'static str,
        ordinal: u16,
    ) -> Result<(), String>;
    /// Ends the selected declaration form.
    ///
    /// # Errors
    ///
    /// Returns a structural error when this is not the form currently open.
    fn end_form(&mut self, construction: &'static str) -> Result<(), String>;
    /// Records one declaration-owned literal.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the sink cannot accept the literal.
    fn literal(&mut self, literal: &'static str) -> Result<(), String>;
    /// Records a typed subtree under its declared grammatical role.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the subtree cannot be projected or
    /// inserted.
    fn subtree(
        &mut self,
        role: &'static str,
        category: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), String>;
    /// Starts one selected alternative of a declaration-owned sum. Payload
    /// roles arrive through the ordinary role callbacks until
    /// [`Self::end_sum_variant`].
    ///
    /// # Errors
    ///
    /// Returns a structural error when the variant payload cannot be projected
    /// or inserted under `role`.
    fn begin_sum_variant(
        &mut self,
        _role: &'static str,
        _element: &'static str,
        _variant: &'static str,
    ) -> Result<(), String> {
        Err("projection sink does not support direct sum roles".to_owned())
    }
    /// Ends the current declaration-owned sum role.
    ///
    /// # Errors
    ///
    /// Returns a structural error when `role` is not the open sum.
    fn end_sum_variant(&mut self, _role: &'static str) -> Result<(), String> {
        Ok(())
    }
    /// Starts one declaration-owned record product. Product fields arrive
    /// through the ordinary role callbacks until [`Self::end_product`].
    ///
    /// # Errors
    ///
    /// Returns a structural error when the product cannot be projected or
    /// inserted under `role`.
    fn begin_product(&mut self, _role: &'static str, _element: &'static str) -> Result<(), String> {
        Err("projection sink does not support direct product roles".to_owned())
    }
    /// Ends the current declaration-owned record product.
    ///
    /// # Errors
    ///
    /// Returns a structural error when `role` is not the open product.
    fn end_product(&mut self, _role: &'static str) -> Result<(), String> {
        Ok(())
    }
    /// Records a typed scalar under its declared role and codec.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the scalar cannot be inserted.
    fn scalar(
        &mut self,
        role: &'static str,
        codec: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), String>;
    /// Records a typed lexical identity under its declared role and provider.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the identity cannot be inserted.
    fn identity(
        &mut self,
        role: &'static str,
        provider: &'static str,
        value_type: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), String>;
    /// Starts a declaration-owned sequence role.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the sequence cannot be opened.
    fn begin_sequence(&mut self, role: &'static str, len: usize) -> Result<(), String>;
    /// Starts the next member of the current sequence.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the role or member index is out of
    /// order.
    fn sequence_member(&mut self, role: &'static str, index: usize) -> Result<(), String>;
    /// Records the typed value supplied by a declared sequence element.
    ///
    /// # Errors
    ///
    /// Returns a structural error when there is no current member or its
    /// adapter fails.
    fn bound_value(
        &mut self,
        element: &'static str,
        value: &dyn ProjectionInput,
    ) -> Result<(), String>;
    /// Records the stable variant identity of the current sequence element.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the variant does not follow its element.
    fn element_variant(
        &mut self,
        element: &'static str,
        variant: &'static str,
    ) -> Result<(), String>;
    /// Records a scalar derived from the current sequence position and length.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the derived role cannot be inserted.
    fn derived_sequence_scalar(
        &mut self,
        role: &'static str,
        codec: &'static str,
        index: usize,
        len: usize,
    ) -> Result<(), String>;
    /// Ends the current declaration-owned sequence role.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the role or emitted member count
    /// differs.
    fn end_sequence(&mut self, role: &'static str) -> Result<(), String>;
    /// Records whether an optional grammatical role is present.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the optional role cannot be inserted.
    fn optional(&mut self, role: &'static str, present: bool) -> Result<(), String>;
    /// Retains one named surface witness with diagnostic storage provenance.
    ///
    /// # Errors
    ///
    /// Returns a structural error when the witness conflicts with an earlier
    /// value.
    fn stored_witness(
        &mut self,
        name: &'static str,
        path: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), String>;
}

/// Adapts an object-safe projection sink to the generic generated
/// linearization visitor. Its ordinary typed callbacks are intentionally
/// inert; the paired role-bearing callbacks carry the projection contract.
pub struct ProjectionSinkAdapter<'a> {
    sink: &'a mut dyn ProjectionSink,
}

impl<'a> ProjectionSinkAdapter<'a> {
    #[must_use]
    pub fn new(sink: &'a mut dyn ProjectionSink) -> Self {
        Self { sink }
    }
}

impl LinearizationVisitor for ProjectionSinkAdapter<'_> {
    type Error = String;

    fn begin_form(
        &mut self,
        construction: &'static str,
        form: &'static str,
        ordinal: u16,
    ) -> Result<(), Self::Error> {
        self.sink.begin_form(construction, form, ordinal)
    }

    fn end_form(&mut self, construction: &'static str) -> Result<(), Self::Error> {
        self.sink.end_form(construction)
    }

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.sink.literal(literal)
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        _category: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn projected_subtree(
        &mut self,
        role: &'static str,
        category: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        self.sink.subtree(role, category, value)
    }

    fn begin_sum_variant(
        &mut self,
        role: &'static str,
        element: &'static str,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.sink.begin_sum_variant(role, element, variant)
    }

    fn end_sum_variant(&mut self, role: &'static str) -> Result<(), Self::Error> {
        self.sink.end_sum_variant(role)
    }

    fn begin_product(
        &mut self,
        role: &'static str,
        element: &'static str,
    ) -> Result<(), Self::Error> {
        self.sink.begin_product(role, element)
    }

    fn end_product(&mut self, role: &'static str) -> Result<(), Self::Error> {
        self.sink.end_product(role)
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        _codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn projected_scalar(
        &mut self,
        role: &'static str,
        codec: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        self.sink.scalar(role, codec, value)
    }

    fn identity<T: std::any::Any>(
        &mut self,
        _provider: &'static str,
        _value_type: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn projected_identity(
        &mut self,
        role: &'static str,
        provider: &'static str,
        value_type: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        self.sink.identity(role, provider, value_type, value)
    }

    fn begin_sequence(&mut self, role: &'static str, len: usize) -> Result<(), Self::Error> {
        self.sink.begin_sequence(role, len)
    }

    fn sequence_member(&mut self, role: &'static str, index: usize) -> Result<(), Self::Error> {
        self.sink.sequence_member(role, index)
    }

    fn bound_value<T: std::any::Any>(
        &mut self,
        _element: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn projected_bound_value(
        &mut self,
        element: &'static str,
        value: &dyn ProjectionInput,
    ) -> Result<(), Self::Error> {
        self.sink.bound_value(element, value)
    }

    fn element_variant(
        &mut self,
        element: &'static str,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.sink.element_variant(element, variant)
    }

    fn derived_sequence_scalar(
        &mut self,
        role: &'static str,
        codec: &'static str,
        index: usize,
        len: usize,
    ) -> Result<(), Self::Error> {
        self.sink.derived_sequence_scalar(role, codec, index, len)
    }

    fn end_sequence(&mut self, role: &'static str) -> Result<(), Self::Error> {
        self.sink.end_sequence(role)
    }

    fn optional(&mut self, role: &'static str, present: bool) -> Result<(), Self::Error> {
        self.sink.optional(role, present)
    }

    fn stored_witness<T: std::any::Any>(
        &mut self,
        _name: &'static str,
        _path: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn projected_stored_witness(
        &mut self,
        name: &'static str,
        path: &'static str,
        value: &dyn ProjectionValue,
    ) -> Result<(), Self::Error> {
        self.sink.stored_witness(name, path, value)
    }
}

pub type ErasedLinearizer =
    fn(&dyn ProjectionInput, &mut dyn ProjectionSink) -> Result<(), ErasedLinearizationError>;

#[derive(Debug)]
pub enum ErasedLinearizationError {
    WrongValueType {
        construction: &'static str,
        expected: &'static str,
    },
    Linearization(LinearizationError<String>),
}

/// Placeholder used only by hand-built metadata fixtures that exercise
/// registry or feature logic rather than declaration-emitted projection.
///
/// # Errors
///
/// Always returns [`ErasedLinearizationError::WrongValueType`] to mark the
/// synthetic declaration as unavailable for runtime projection.
pub fn unavailable_erased_linearizer(
    _value: &dyn ProjectionInput,
    _sink: &mut dyn ProjectionSink,
) -> Result<(), ErasedLinearizationError> {
    Err(ErasedLinearizationError::WrongValueType {
        construction: "<synthetic>",
        expected: "<unavailable>",
    })
}

/// One type-erased field value supplied to a declaration-emitted builder.
/// Values remain owned so generated lowering can move boxed subtrees and
/// sequences into the final construction without cloning them.
pub type ErasedValue = Box<dyn std::any::Any>;

pub type ErasedBuilder = fn(Vec<ErasedValue>) -> Result<ErasedValue, ErasedBuildError>;
pub type ErasedFieldBuilder = fn(Vec<ErasedValue>) -> Result<ErasedValue, ErasedBuildError>;
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
    EmptySequence {
        owner: &'static str,
        field: &'static str,
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
    pub backend: ConstructionBackendData,
    pub elements: &'static [&'static str],
    pub element_data: &'static [ElementData],
    /// Typed record rebuild schemas. Lenses are deliberately separate from
    /// semantic elements: they do not mint chart categories or surface atoms.
    pub lenses: &'static [LensData],
    pub constructions: &'static [ConstructionData],
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConstructionBackendData {
    #[default]
    Chart,
    Ability,
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
    pub adapter: Option<LensEditAdapterData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LensEditAdapterData {
    pub constructor: &'static str,
    pub destructurer: &'static str,
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
    /// Declaration-emitted inverse traversal through stable construction,
    /// form, role, scalar, identity, and witness events. This is the spelling
    /// boundary; it does not expose Rust storage or serde layout.
    pub erased_linearizer: ErasedLinearizer,
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

#[allow(
    unpredictable_function_pointer_comparisons,
    reason = "metadata equality is used for fixture structure; field builders are never selected by address"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKindData {
    Unit,
    TupleProduct {
        fields: &'static [FieldData],
    },
    StructProduct {
        fields: &'static [FieldData],
    },
    Identity {
        value_type: &'static str,
        provider: &'static str,
    },
    Subtree {
        category: &'static str,
        boxed: bool,
    },
    TypedSubtree {
        value_type: &'static str,
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
    NonEmptySequence {
        element: &'static str,
        erased_builder: ErasedFieldBuilder,
    },
    SeparatedNonEmptySequence {
        element: &'static str,
        separator: &'static str,
        erased_builder: ErasedFieldBuilder,
    },
    Sum {
        element: &'static str,
        boxed: bool,
    },
    Product {
        element: &'static str,
        boxed: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonempty_rejects_empty_ingress_and_serializes_as_a_sequence() {
        assert_eq!(NonEmpty::<u8>::try_from(Vec::new()), Err(EmptySequence));
        let values = NonEmpty::try_from(vec![1_u8, 2]).unwrap();
        assert_eq!(values.first(), &1);
        assert_eq!(values.rest(), &[2]);
        assert_eq!(serde_json::to_string(&values).unwrap(), "[1,2]");
        assert!(serde_json::from_str::<NonEmpty<u8>>("[]").is_err());
    }

    #[test]
    fn separated_nonempty_preserves_one_separator_per_continuation() {
        let values = SeparatedNonEmpty::new(
            "first",
            vec![Separated::new(',', "second"), Separated::new(';', "third")],
        );
        assert_eq!(
            values.iter().copied().collect::<Vec<_>>(),
            ["first", "second", "third"]
        );
        assert_eq!(values.iter().len(), 3);
        assert_eq!(values.rest()[0].separator(), &',');
        assert_eq!(values.rest()[0].value(), &"second");
        assert_eq!(values.last(), &"third");
    }
}
