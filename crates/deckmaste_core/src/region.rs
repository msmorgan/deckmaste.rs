use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use serde::ser::{
    Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};

use crate::OneShotEffect;

/// The ordinal assigned to a value defined in a [`Region`].
///
/// Parameters occupy the first ordinals. Instruction products added by later
/// region stages continue the same sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DefId(pub u32);

/// A read of an earlier [`DefId`] in the same region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RefId(pub u32);

impl From<DefId> for RefId {
    fn from(value: DefId) -> Self {
        RefId(value.0)
    }
}

/// The runtime shape of a region register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Kind {
    /// One card, token, spell, ability, emblem, or player proxy.
    Object,
    /// A group of objects, preserving its semantic order.
    Objects,
    /// A non-negative game number.
    Number,
}

/// Why a region parameter exists and how the engine supplies it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Provenance {
    Source,
    Controller,
    EventObject,
    EventPatient,
    EventActor,
    DefendingPlayer,
    AnnouncedTarget(u32),
    AnnouncedX,
    /// Reserved by the capture stage; present now so region data has one
    /// stable provenance vocabulary.
    Capture(RefId),
    /// Reserved by the linked-memory stage.
    Linked(crate::Ident),
    /// Reserved by the discourse stage.
    LoopElement,
    /// Reserved by the discourse stage.
    Allotment,
    /// Reserved by candidate-region lowering.
    Candidate,
}

/// One declared input to a closed region.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Param {
    pub def: DefId,
    pub kind: Kind,
    pub provenance: Provenance,
}

/// A textual sequence of instructions.
///
/// Stage 1 keeps the established instruction tree as the instruction payload;
/// the following discourse stage splits decision products into individual
/// instruction arms. Keeping the sequence as a distinct type now makes region
/// entry and validation explicit without introducing a second core grammar.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct Block(pub Arc<[OneShotEffect]>);

impl Deref for Block {
    type Target = [OneShotEffect];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OneShotEffect> for Block {
    fn from(effect: OneShotEffect) -> Self {
        match effect {
            OneShotEffect::Sequentially(parts) => Block(parts),
            other => Block(Arc::from([other])),
        }
    }
}

impl From<Arc<[OneShotEffect]>> for Block {
    fn from(instructions: Arc<[OneShotEffect]>) -> Self {
        Block(instructions)
    }
}

/// A closed executable scope. Every binding it may read is declared in
/// `params`; instruction definitions extend the same ordinal sequence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Region {
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub params: Arc<[Param]>,
    pub body: Block,
}

impl Region {
    #[must_use]
    pub fn new(params: Arc<[Param]>, body: impl Into<Block>) -> Self {
        Self {
            params,
            body: body.into(),
        }
    }
}

impl From<OneShotEffect> for Region {
    fn from(effect: OneShotEffect) -> Self {
        Self::new(Arc::from([]), effect)
    }
}

/// A load-time region well-formedness failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Parameters must define the prefix `0, 1, ...` in order.
    ParamSequence { expected: DefId, found: DefId },
    /// Announced-target parameters must be numbered `0, 1, ...` in order.
    TargetSequence { expected: u32, found: u32 },
    /// An ability's target list and target-provenance parameters must agree.
    TargetCountMismatch { targets: usize, parameters: usize },
    /// A provenance whose payload is an index must fit the representation used
    /// by the rest of the core grammar.
    TargetIndexOutOfRange { index: u32, target_count: usize },
    /// A register read does not name a definition in this region.
    UndefinedRead {
        reference: RefId,
        definitions: usize,
    },
    /// A register is read through an expression of the wrong runtime kind.
    KindMismatch {
        reference: RefId,
        expected: Kind,
        found: Kind,
    },
    /// Serialization failed while walking the core grammar.
    Walk(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParamSequence { expected, found } => write!(
                f,
                "region parameter sequence expected DefId({}), found DefId({})",
                expected.0, found.0
            ),
            Self::TargetSequence { expected, found } => write!(
                f,
                "region announced-target sequence expected index {expected}, found {found}"
            ),
            Self::TargetCountMismatch {
                targets,
                parameters,
            } => write!(
                f,
                "ability declares {targets} targets but its region has {parameters} announced-target parameters"
            ),
            Self::TargetIndexOutOfRange {
                index,
                target_count,
            } => write!(
                f,
                "announced target provenance {index} is outside {target_count} declared targets"
            ),
            Self::UndefinedRead {
                reference,
                definitions,
            } => write!(
                f,
                "region read RefId({}) is not dominated by one of its {definitions} definitions",
                reference.0
            ),
            Self::KindMismatch {
                reference,
                expected,
                found,
            } => write!(
                f,
                "region read RefId({}) expects {expected:?}, but its definition is {found:?}",
                reference.0
            ),
            Self::Walk(message) => write!(f, "could not validate region: {message}"),
        }
    }
}

impl std::error::Error for ValidationError {}

impl serde::ser::Error for ValidationError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Walk(msg.to_string())
    }
}

/// Validate the declaration half of a region.
///
/// Reference dominance and kind checking are layered onto this walker as the
/// old reference variants are retired; parameter sequencing is enforced from
/// the first substrate revision so activation records can always index by
/// `DefId` directly.
///
/// # Errors
///
/// Returns [`ValidationError`] when declarations are not dense and ordered or
/// a read violates dominance, kind, range, target order, or region closure.
///
/// # Panics
///
/// Panics only if the region declares more than `u32::MAX` parameters.
pub fn validate(region: &Region) -> Result<(), ValidationError> {
    for (index, param) in region.params.iter().enumerate() {
        let expected = DefId(u32::try_from(index).expect("region parameter count fits in u32"));
        if param.def != expected {
            return Err(ValidationError::ParamSequence {
                expected,
                found: param.def,
            });
        }
    }
    let target_count = region
        .params
        .iter()
        .filter(|param| matches!(param.provenance, Provenance::AnnouncedTarget(_)))
        .count();
    let mut next_target = 0_u32;
    for param in region.params.iter() {
        let expected_kind = match param.provenance {
            Provenance::Source
            | Provenance::Controller
            | Provenance::EventObject
            | Provenance::EventPatient
            | Provenance::EventActor
            | Provenance::DefendingPlayer
            | Provenance::LoopElement
            | Provenance::Candidate => Some(Kind::Object),
            Provenance::AnnouncedTarget(_) => Some(Kind::Objects),
            Provenance::AnnouncedX | Provenance::Allotment => Some(Kind::Number),
            Provenance::Capture(reference) => region
                .params
                .get(reference.0 as usize)
                .map(|captured| captured.kind),
            Provenance::Linked(_) => None,
        };
        if let Some(expected) = expected_kind
            && param.kind != expected
        {
            return Err(ValidationError::KindMismatch {
                reference: param.def.into(),
                expected,
                found: param.kind,
            });
        }
        match param.provenance {
            Provenance::AnnouncedTarget(index) => {
                if index as usize >= target_count {
                    return Err(ValidationError::TargetIndexOutOfRange {
                        index,
                        target_count,
                    });
                }
                if index != next_target {
                    return Err(ValidationError::TargetSequence {
                        expected: next_target,
                        found: index,
                    });
                }
                next_target += 1;
            }
            Provenance::Capture(reference) => {
                validate_read(region.params.as_ref(), reference, None)?;
            }
            _ => {}
        }
    }

    region.body.serialize(RegisterWalker {
        params: region.params.as_ref(),
    })?;
    for instruction in region.body.iter() {
        validate_nested_regions(instruction)?;
    }
    Ok(())
}

/// Validate an ability region together with its target telescope and optional
/// announced-X definition. A target constraint may read only parameters
/// declared before that target's own slot.
///
/// # Errors
///
/// Returns [`ValidationError`] when the region is invalid, target declarations
/// do not match their parameters, or a target/`where X` expression reads an
/// unavailable register.
///
/// # Panics
///
/// Panics only if the ability declares more than `u32::MAX` targets.
pub fn validate_telescope(
    region: &Region,
    targets: &[crate::TargetSpec],
    where_x: Option<&crate::Count>,
) -> Result<(), ValidationError> {
    validate(region)?;
    let target_parameters = region
        .params
        .iter()
        .filter(|param| matches!(param.provenance, Provenance::AnnouncedTarget(_)))
        .count();
    if targets.len() != target_parameters {
        return Err(ValidationError::TargetCountMismatch {
            targets: targets.len(),
            parameters: target_parameters,
        });
    }
    for (index, target) in targets.iter().enumerate() {
        validate_distinct_indices(target, index)?;
        let limit = region
            .params
            .iter()
            .position(|param| {
                param.provenance
                    == Provenance::AnnouncedTarget(u32::try_from(index).expect("target index fits"))
            })
            .unwrap_or(region.params.len());
        target.serialize(RegisterWalker {
            params: &region.params[..limit],
        })?;
    }
    if let Some(where_x) = where_x {
        where_x.serialize(RegisterWalker {
            params: region.params.as_ref(),
        })?;
    }
    Ok(())
}

fn validate_distinct_indices(
    target: &crate::TargetSpec,
    current: usize,
) -> Result<(), ValidationError> {
    if let crate::TargetSpec::Distinct(indices, inner) = target {
        for &index in indices.iter() {
            if index >= current {
                return Err(ValidationError::TargetIndexOutOfRange {
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                    target_count: current,
                });
            }
        }
        validate_distinct_indices(inner, current)?;
    }
    Ok(())
}

fn validate_read(
    params: &[Param],
    reference: RefId,
    expected: Option<Kind>,
) -> Result<(), ValidationError> {
    let Some(param) = params.get(reference.0 as usize) else {
        return Err(ValidationError::UndefinedRead {
            reference,
            definitions: params.len(),
        });
    };
    if let Some(expected) = expected {
        let object_compatible = matches!(expected, Kind::Object | Kind::Objects)
            && matches!(param.kind, Kind::Object | Kind::Objects);
        if param.kind != expected && !object_compatible {
            return Err(ValidationError::KindMismatch {
                reference,
                expected,
                found: param.kind,
            });
        }
    }
    Ok(())
}

fn validate_nested_regions(effect: &OneShotEffect) -> Result<(), ValidationError> {
    use crate::OneShotEffect as E;
    let children: Vec<&OneShotEffect> = match effect {
        E::Sequentially(parts) | E::Simultaneously(parts) => parts.iter().collect(),
        E::Label(value) => vec![&value.effect],
        E::SeparatePiles(value) => value.then.iter().map(AsRef::as_ref).collect(),
        E::ChoosePile(value) => vec![&value.then],
        E::May(value) => std::iter::once(value.effect.as_ref())
            .chain(value.if_did.iter().map(AsRef::as_ref))
            .chain(value.if_not.iter().map(AsRef::as_ref))
            .collect(),
        E::If(value) => std::iter::once(value.then.as_ref())
            .chain(value.otherwise.iter().map(AsRef::as_ref))
            .collect(),
        E::AdditionalCost(value) => vec![&value.body],
        E::Each(value) => vec![&value.effect],
        E::With(value) => vec![&value.body],
        E::Distribute(value) => vec![&value.body],
        E::Noting(value) => vec![&value.effect],
        E::Repeat(_, body) | E::Batch(_, body) => vec![body],
        E::RevealUntil(value) => vec![&value.body],
        E::Delayed(ability) | E::Reflexive(ability) => {
            validate_telescope(&ability.effect, &ability.targets, ability.where_x.as_ref())?;
            Vec::new()
        }
        E::Modal(modal) => {
            for mode in modal.modes.iter() {
                validate_telescope(&mode.effect, &mode.targets, None)?;
            }
            Vec::new()
        }
        E::Act(_) | E::Continuously(_) | E::Until(_, _) => Vec::new(),
    };
    for child in children {
        validate_nested_regions(child)?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct RegisterWalker<'a> {
    params: &'a [Param],
}

struct Compound<'a> {
    walker: RegisterWalker<'a>,
    skip: bool,
}

macro_rules! walk_value {
    ($self:ident, $value:ident) => {
        if $self.skip { Ok(()) } else { $value.serialize($self.walker) }
    };
}

impl SerializeSeq for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl SerializeTuple for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl SerializeTupleStruct for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl SerializeTupleVariant for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl SerializeMap for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        walk_value!(self, key)
    }
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl SerializeStruct for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl SerializeStructVariant for Compound<'_> {
    type Ok = ();
    type Error = ValidationError;
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        walk_value!(self, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a> serde::Serializer for RegisterWalker<'a> {
    type Ok = ();
    type Error = ValidationError;
    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Compound<'a>;
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if variant == "Reg" {
            let mut capture = RefCapture(None);
            value.serialize(&mut capture)?;
            let reference = RefId(capture.0.ok_or_else(|| {
                ValidationError::Walk("Reg did not contain an integer RefId".into())
            })?);
            let expected = match name {
                "Count" => Some(Kind::Number),
                "Selection" => Some(Kind::Objects),
                "Reference" => Some(Kind::Object),
                _ => None,
            };
            validate_read(self.params, reference, expected)
        } else {
            value.serialize(self)
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(compound(self, name == "Region"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(compound(self, false))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(compound(self, false))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(compound(self, false))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(compound(self, false))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(compound(self, false))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(compound(self, false))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(self)
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), Self::Error> {
        value.serialize(self)
    }

    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u8(self, _: u8) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u16(self, _: u16) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u32(self, _: u32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u64(self, _: u64) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_f32(self, _: f32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_f64(self, _: f64) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn compound(walker: RegisterWalker<'_>, skip: bool) -> Compound<'_> {
    Compound { walker, skip }
}

struct RefCapture(Option<u32>);

impl serde::Serializer for &mut RefCapture {
    type Ok = ();
    type Error = ValidationError;
    type SerializeSeq = Impossible<(), ValidationError>;
    type SerializeTuple = Impossible<(), ValidationError>;
    type SerializeTupleStruct = Impossible<(), ValidationError>;
    type SerializeTupleVariant = Impossible<(), ValidationError>;
    type SerializeMap = Impossible<(), ValidationError>;
    type SerializeStruct = Impossible<(), ValidationError>;
    type SerializeStructVariant = Impossible<(), ValidationError>;
    fn serialize_u32(self, value: u32) -> Result<(), Self::Error> {
        self.0 = Some(value);
        Ok(())
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(self)
    }
    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_u8(self, value: u8) -> Result<(), Self::Error> {
        self.serialize_u32(value.into())
    }
    fn serialize_u16(self, value: u16) -> Result<(), Self::Error> {
        self.serialize_u32(value.into())
    }
    fn serialize_u64(self, value: u64) -> Result<(), Self::Error> {
        self.serialize_u32(u32::try_from(value).map_err(serde::ser::Error::custom)?)
    }
    fn serialize_f32(self, _: f32) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_f64(self, _: f64) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ValidationError::Walk("expected RefId".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn param(def: u32, provenance: Provenance) -> Param {
        Param {
            def: DefId(def),
            kind: Kind::Object,
            provenance,
        }
    }

    #[test]
    fn sequential_effect_becomes_a_textual_block() {
        let block = Block::from(OneShotEffect::Sequentially(Arc::from([])));
        assert!(block.is_empty());
    }

    #[test]
    fn parameter_definitions_are_a_dense_prefix() {
        let valid = Region::new(
            Arc::from([
                param(0, Provenance::Source),
                param(1, Provenance::Controller),
            ]),
            Arc::from([]),
        );
        assert_eq!(validate(&valid), Ok(()));

        let invalid = Region::new(
            Arc::from([
                param(0, Provenance::Source),
                param(2, Provenance::Controller),
            ]),
            Arc::from([]),
        );
        assert_eq!(
            validate(&invalid),
            Err(ValidationError::ParamSequence {
                expected: DefId(1),
                found: DefId(2),
            })
        );
    }

    #[test]
    fn rejects_out_of_range_and_wrong_kind_reads() {
        let out_of_range = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            OneShotEffect::Act(crate::Action::Counter(crate::Reference::Reg(RefId(1)))),
        );
        assert_eq!(
            validate(&out_of_range),
            Err(ValidationError::UndefinedRead {
                reference: RefId(1),
                definitions: 1,
            })
        );

        let wrong_kind = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            OneShotEffect::Repeat(
                crate::Count::Reg(RefId(0)),
                Arc::new(OneShotEffect::Sequentially(Arc::from([]))),
            ),
        );
        assert_eq!(
            validate(&wrong_kind),
            Err(ValidationError::KindMismatch {
                reference: RefId(0),
                expected: Kind::Number,
                found: Kind::Object,
            })
        );
    }

    #[test]
    fn nested_regions_are_closed_against_their_own_params() {
        let nested = Region::new(
            Arc::from([]),
            OneShotEffect::Act(crate::Action::Counter(crate::Reference::Reg(RefId(0)))),
        );
        let ability = crate::TriggeredAbility {
            ability_word: None,
            event: crate::EventFilter::ZoneChange {
                what: crate::Predicate::any(),
                from: None,
                to: None,
                cause: None,
            },
            from: None,
            condition: None,
            limits: Arc::from([]),
            where_x: None,
            targets: Arc::from([]),
            effect: nested,
        };
        let outer = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            OneShotEffect::Delayed(Arc::new(ability)),
        );
        assert_eq!(
            validate(&outer),
            Err(ValidationError::UndefinedRead {
                reference: RefId(0),
                definitions: 0,
            })
        );
    }

    #[test]
    fn target_parameters_match_the_ability_telescope() {
        let duplicate = Region::new(
            Arc::from([
                Param {
                    def: DefId(0),
                    kind: Kind::Objects,
                    provenance: Provenance::AnnouncedTarget(0),
                },
                Param {
                    def: DefId(1),
                    kind: Kind::Objects,
                    provenance: Provenance::AnnouncedTarget(0),
                },
            ]),
            Arc::from([]),
        );
        assert_eq!(
            validate(&duplicate),
            Err(ValidationError::TargetSequence {
                expected: 1,
                found: 0,
            })
        );

        let target = crate::TargetSpec::Target(crate::Quantity::one(), crate::Predicate::any());
        let missing = Region::new(Arc::from([]), Arc::from([]));
        assert_eq!(
            validate_telescope(&missing, &[target], None),
            Err(ValidationError::TargetCountMismatch {
                targets: 1,
                parameters: 0,
            })
        );
    }
}
