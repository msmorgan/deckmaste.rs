use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use serde::ser::Impossible;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::ser::SerializeStructVariant;
use serde::ser::SerializeTuple;
use serde::ser::SerializeTupleStruct;
use serde::ser::SerializeTupleVariant;

use crate::Instruction;

/// The ordinal assigned to a value defined in a [`Region`].
///
/// Parameters occupy the first ordinals. Instruction products added by later
/// region stages continue the same sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct DefId(pub u32);

/// A read of an earlier [`DefId`] in the same region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
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
    /// One Entity — an object ([CR#109.1]) or a player ([CR#102.1]). The
    /// engine still addresses a player through a proxy carrying an
    /// `ObjectId`; that adapter is storage, and this register shape names the
    /// Entity it stands for rather than calling a player an object.
    Entity,
    /// A group of Entities, preserving its semantic order.
    Entities,
    /// One temporary pile: an object group whose members remain individual
    /// objects ([CR#700.3b]).
    Pile,
    /// A non-negative game number.
    Number,
    /// A chosen symbolic value (currently a color or card name).
    Symbol,
}

/// Why a region parameter exists and how the engine supplies it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Provenance {
    Source,
    Controller,
    EventObject,
    EventPatient,
    EventActor,
    /// The amount carried by the triggering event.
    EventAmount,
    DefendingPlayer,
    AnnouncedTarget(u32),
    AnnouncedX,
    /// A register of the enclosing region, snapshotted when THIS region was
    /// created ([CR#603.7,603.12]) — the only thing that crosses a region
    /// boundary (ADR law 7). The snapshot is taken once, at creation, of the
    /// value the creating region had already chased to its post-move
    /// incarnation ([CR#400.7j] — "other parts of that effect can find that
    /// object"); it is NEVER chased again at the read site. [CR#603.7c]
    /// settles that: a captured object that is no longer in the zone it was
    /// expected to be in is not affected, and one that left and returned "is
    /// a new object and thus won't be affected". Information reads therefore
    /// fall back to last-known information ([CR#608.2h]) while actions find
    /// nothing.
    Capture(RefId),
    /// A linked memory cell of the card ([CR#607.1] — ADR law 8), written by
    /// an [`Instruction::Remember`](crate::Instruction::Remember) on
    /// another ability of the same card and supplied to this region at entry.
    Linked(crate::Ident),
    /// The current element supplied when entering an `Each` or `Distribute`
    /// body region.
    LoopElement,
    /// The share supplied alongside a `Distribute` body region's loop element.
    Allotment,
    /// The per-candidate subject of a predicate region, carrying the Entity
    /// domain it ranges over ([CR#109.1,102.1] — ADR law 2). The domain fixes
    /// what the engine enumerates before the predicate runs, and
    /// [`crate::validate`] refuses a predicate whose subject cannot inhabit
    /// it — a Player-only predicate never enters the Object domain.
    Candidate(crate::Domain),
}

/// One declared input to a closed region.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Param {
    pub def: DefId,
    pub kind: Kind,
    pub provenance: Provenance,
}

/// The engine-supplied parameters of an engine rule or cost scope.
///
/// These scopes are closed regions even when they have no authored body of
/// their own: source and controller are ordinary indexed inputs, not an
/// out-of-band execution-frame environment.
#[must_use]
pub fn source_controller_params() -> Arc<[Param]> {
    Arc::from([
        Param {
            def: DefId(0),
            kind: Kind::Entity,
            provenance: Provenance::Source,
        },
        Param {
            def: DefId(1),
            kind: Kind::Entity,
            provenance: Provenance::Controller,
        },
    ])
}

/// The engine-supplied parameter prefix of a spell or activated-ability region.
///
/// # Panics
///
/// Panics if `target_count` cannot be represented by the region's 32-bit
/// definition and target indices.
#[must_use]
pub fn announced_region_params(target_count: usize) -> Arc<[Param]> {
    let mut params = source_controller_params().to_vec();
    params.extend((0..target_count).map(|index| Param {
        def: DefId(u32::try_from(2 + index).expect("parameter count fits u32")),
        kind: Kind::Entities,
        provenance: Provenance::AnnouncedTarget(
            u32::try_from(index).expect("target index fits u32"),
        ),
    }));
    params.push(Param {
        def: DefId(u32::try_from(params.len()).expect("parameter count fits u32")),
        kind: Kind::Number,
        provenance: Provenance::AnnouncedX,
    });
    params.into()
}

/// The fixed event-role prefix shared by static, triggered, and transitional
/// floating-replacement regions.
///
/// Announced targets and X, where applicable, follow this prefix.
#[must_use]
pub fn event_region_params() -> Arc<[Param]> {
    Arc::from([
        Param {
            def: DefId(0),
            kind: Kind::Entity,
            provenance: Provenance::Source,
        },
        Param {
            def: DefId(1),
            kind: Kind::Entity,
            provenance: Provenance::Controller,
        },
        Param {
            def: DefId(2),
            kind: Kind::Entity,
            provenance: Provenance::EventObject,
        },
        Param {
            def: DefId(3),
            kind: Kind::Entity,
            provenance: Provenance::EventPatient,
        },
        Param {
            def: DefId(4),
            kind: Kind::Entity,
            provenance: Provenance::EventActor,
        },
        Param {
            def: DefId(5),
            kind: Kind::Entity,
            provenance: Provenance::DefendingPlayer,
        },
        Param {
            def: DefId(6),
            kind: Kind::Number,
            provenance: Provenance::EventAmount,
        },
    ])
}

/// The event-role prefix plus a triggered ability's announced targets and X.
///
/// # Panics
///
/// Panics if `target_count` cannot be represented by the region's 32-bit
/// definition and target indices.
#[must_use]
pub fn triggered_region_params(target_count: usize) -> Arc<[Param]> {
    let mut params = event_region_params().to_vec();
    for index in 0..target_count {
        params.push(Param {
            def: DefId(u32::try_from(params.len()).expect("parameter count fits u32")),
            kind: Kind::Entities,
            provenance: Provenance::AnnouncedTarget(
                u32::try_from(index).expect("target index fits u32"),
            ),
        });
    }
    params.push(Param {
        def: DefId(u32::try_from(params.len()).expect("parameter count fits u32")),
        kind: Kind::Number,
        provenance: Provenance::AnnouncedX,
    });
    params.into()
}

/// A textual sequence of instructions in one lexical region.
///
/// Definitions produced by an instruction are visible to later instructions
/// in this block and to nested blocks, but not after the block closes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Block(pub Arc<[Instruction]>);

impl Deref for Block {
    type Target = [Instruction];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Instruction> for Block {
    fn from(effect: Instruction) -> Self {
        match effect {
            Instruction::Sequentially(parts) => Block(parts),
            other => Block(Arc::from([other])),
        }
    }
}

impl From<Arc<[Instruction]>> for Block {
    fn from(instructions: Arc<[Instruction]>) -> Self {
        Block(instructions)
    }
}

/// A closed executable scope. Every binding it may read is declared in
/// `params`; instruction definitions extend the same ordinal sequence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Region<T = Block> {
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub params: Arc<[Param]>,
    pub body: T,
}

impl<T> Region<T> {
    #[must_use]
    pub fn new(params: Arc<[Param]>, body: T) -> Self {
        Self { params, body }
    }

    /// Construct a region with no runtime parameters.
    #[must_use]
    pub fn closed(body: T) -> Self {
        Self::new(Arc::<[Param]>::from([]), body)
    }

    /// Construct a region with one parameter at register zero.
    #[must_use]
    pub fn unary(kind: Kind, provenance: Provenance, body: T) -> Self {
        Self::new(
            Arc::from([Param {
                def: DefId(0),
                kind,
                provenance,
            }]),
            body,
        )
    }

    /// Construct a predicate-like region whose subject is register zero,
    /// declaring the narrowest Entity domain the body's own subject admits
    /// ([CR#109.1,102.1] — ADR law 2). The declared domain is a function of
    /// the body alone, so a region built here and the same region built by
    /// lowering declare the same domain.
    #[must_use]
    pub fn candidate(body: T) -> Self
    where
        T: CandidateBody,
    {
        let domain = body.candidate_domain();
        Self::candidate_in(domain, body)
    }

    /// Construct a predicate-like region whose subject is register zero and
    /// ranges over `domain` ([CR#109.1,102.1]).
    #[must_use]
    pub fn candidate_in(domain: crate::Domain, body: T) -> Self {
        Self::unary(Kind::Entity, Provenance::Candidate(domain), body)
    }

    /// The Entity domain this region's candidate ranges over
    /// ([CR#109.1,102.1]), or [`crate::Domain::Entity`] when it declares no
    /// candidate parameter.
    #[must_use]
    pub fn candidate_domain(&self) -> crate::Domain {
        self.params
            .iter()
            .find_map(|param| match param.provenance {
                Provenance::Candidate(domain) => Some(domain),
                _ => None,
            })
            .unwrap_or(crate::Domain::Entity)
    }

    /// Return the register declared for `provenance`, if this region has one.
    #[must_use]
    pub fn reference_for(&self, provenance: &Provenance) -> Option<RefId> {
        self.params
            .iter()
            .find(|param| &param.provenance == provenance)
            .map(|param| param.def.into())
    }

    /// The registers this region captures from the region that created it
    /// ([CR#603.7,603.12] — ADR law 7). Each item is `(here, there)`: the
    /// parameter this region reads and the enclosing register it was taken
    /// from. The list IS the declaration a created body's supplier must
    /// satisfy in full; nothing else crosses the boundary.
    pub fn captures(&self) -> impl Iterator<Item = (RefId, RefId, Kind)> + '_ {
        self.params
            .iter()
            .filter_map(|param| match param.provenance {
                Provenance::Capture(outer) => Some((param.def.into(), outer, param.kind)),
                _ => None,
            })
    }

    /// The linked memory cells this region reads ([CR#607.1] — ADR law 8),
    /// as `(here, cell, kind)`.
    pub fn linked_cells(&self) -> impl Iterator<Item = (RefId, &crate::Ident, Kind)> + '_ {
        self.params
            .iter()
            .filter_map(|param| match &param.provenance {
                Provenance::Linked(cell) => Some((param.def.into(), cell, param.kind)),
                _ => None,
            })
    }

    /// Return the provenance declared for a register in this region.
    #[must_use]
    pub fn provenance_of(&self, reference: RefId) -> Option<&Provenance> {
        self.params
            .get(reference.0 as usize)
            .map(|param| &param.provenance)
    }
}

impl fmt::Display for Region<Block> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Region {{")?;
        writeln!(f, "    params: [")?;
        for param in self.params.iter() {
            writeln!(
                f,
                "        {:?}: {:?} <- {:?},",
                param.def, param.kind, param.provenance
            )?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "    body: [")?;
        for instruction in self.body.iter() {
            let rendered = instruction.to_string();
            let mut lines = rendered.lines().peekable();
            while let Some(line) = lines.next() {
                if lines.peek().is_none() {
                    writeln!(f, "        {line},")?;
                } else {
                    writeln!(f, "        {line}")?;
                }
            }
        }
        writeln!(f, "    ],")?;
        write!(f, "}}")
    }
}

impl Region<crate::Predicate> {
    /// A predicate region whose declared candidate domain is the narrowest
    /// one the predicate's own atoms admit ([CR#109.1,102.1]). Lowering builds
    /// every per-candidate region this way, so a filter that only Objects can
    /// satisfy declares the Object domain and one that only Players can
    /// satisfy declares the Player domain.
    ///
    /// The predicate-named spelling of [`Region::candidate`]; both declare the
    /// same domain for the same body.
    #[must_use]
    pub fn over(body: crate::Predicate) -> Self {
        Self::candidate(body)
    }
}

/// A body a candidate region can carry, and the Entity domain its subject
/// admits ([CR#109.1,102.1] — ADR law 2).
///
/// [`Region::candidate`] reads the declared domain from here rather than
/// taking it as an argument, so every construction of the same candidate
/// region agrees — core, engine, and lowering alike. Only a predicate body
/// classifies its own subject; every other body leaves the candidate
/// unconstrained at the Entity boundary and answers [`crate::Domain::Entity`].
pub trait CandidateBody {
    /// The narrowest Entity domain this body's candidate can inhabit.
    fn candidate_domain(&self) -> crate::Domain;
}

impl CandidateBody for crate::Predicate {
    fn candidate_domain(&self) -> crate::Domain {
        self.subject_domain()
    }
}

/// Bodies that say nothing about their candidate's Entity class: a value or
/// effect computed PER candidate, whose domain the enclosing selection fixes.
macro_rules! unconstrained_candidate_body {
    ($($body:ty),+ $(,)?) => {$(
        impl CandidateBody for $body {
            fn candidate_domain(&self) -> crate::Domain {
                crate::Domain::Entity
            }
        }
    )+};
}

unconstrained_candidate_body!(crate::Condition, crate::Count, crate::StaticSpec, Block,);

impl From<Instruction> for Region<Block> {
    fn from(effect: Instruction) -> Self {
        Self::new(Arc::from([]), effect.into())
    }
}

/// A pure expression whose value can be pinned by [`crate::Instruction::Let`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Expr {
    Object(crate::Reference),
    Objects(crate::Selection),
    Number(crate::Count),
}

impl Expr {
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            Self::Object(_) => Kind::Entity,
            Self::Objects(_) => Kind::Entities,
            Self::Number(_) => Kind::Number,
        }
    }
}

/// A load-time region well-formedness failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Parameters must define the prefix `0, 1, ...` in order.
    ParamSequence { expected: DefId, found: DefId },
    /// Instruction definitions must continue the parameter prefix densely.
    DefinitionSequence { expected: DefId, found: DefId },
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
    /// Pure expressions cannot contain a player/random decision.
    DecisionInExpression,
    /// A predicate's subject cannot inhabit the candidate domain its region
    /// declares — a Player-only predicate in the Object domain, or the
    /// reverse ([CR#109.1,102.1]).
    CandidateDomain {
        declared: crate::Domain,
        subject: crate::Domain,
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
            Self::DefinitionSequence { expected, found } => write!(
                f,
                "region instruction sequence expected DefId({}), found DefId({})",
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
            Self::DecisionInExpression => {
                write!(f, "a decision-bearing selection cannot appear inside Let")
            }
            Self::CandidateDomain { declared, subject } => write!(
                f,
                "a predicate whose subject is a {subject:?} cannot appear in a region whose \
                 candidate domain is {declared:?}"
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
pub fn validate(region: &Region<Block>) -> Result<(), ValidationError> {
    validate_region(region, None, None)
}

/// Validate a static-ability region and its nested per-subject regions.
///
/// # Errors
///
/// Returns [`ValidationError`] when a declaration or register read violates
/// the region ABI.
pub fn validate_static(region: &Region<crate::StaticSpec>) -> Result<(), ValidationError> {
    validate_params(region, None)?;
    let mut definitions = Definitions::new(&region.params);
    validate_static_effect(&region.body, &mut definitions)
}

/// Validate a rules-defined state-based-action region.
///
/// # Errors
///
/// Returns [`ValidationError`] when the SBA's scope, condition, or instruction
/// body violates the region ABI.
pub fn validate_sba(region: &Region<crate::SbaBody>) -> Result<(), ValidationError> {
    validate_params(region, None)?;
    region.body.scope.serialize(RegisterWalker {
        params: &region.params,
        visible: None,
        ignore_registers: false,
        reject_decisions: false,
    })?;
    validate_predicate_regions(&region.body.scope, &region.params)?;
    region.body.when.serialize(RegisterWalker {
        params: &region.params,
        visible: None,
        ignore_registers: false,
        reject_decisions: false,
    })?;
    validate_condition_regions(&region.body.when, &region.params)?;

    let mut definitions = Definitions::new(&region.params);
    let block = Block::from(region.body.then.clone());
    validate_instructions(&block, &mut definitions)
}

fn validate_static_effect(
    effect: &crate::StaticSpec,
    definitions: &mut Definitions,
) -> Result<(), ValidationError> {
    match effect {
        crate::StaticSpec::Each(over, body) => {
            over.serialize(definitions.walker())?;
            validate_selection_regions(over, &definitions.params)?;
            validate_params(body, Some(&definitions.params))?;
            let mut body_definitions = Definitions::new(&body.params);
            validate_static_effect(&body.body, &mut body_definitions)
        }
        crate::StaticSpec::Conditionally(condition, body) => {
            condition.serialize(definitions.walker())?;
            validate_condition_regions(condition, &definitions.params)?;
            validate_static_effect(body, definitions)
        }
        crate::StaticSpec::Deontic(deontic) => validate_deontic(deontic, definitions),
        crate::StaticSpec::ConditionallyDo { when, then } => {
            when.serialize(definitions.walker())?;
            validate_condition_regions(when, &definitions.params)?;
            let outer = definitions.params.len();
            validate_instructions(std::slice::from_ref(then.as_ref()), definitions)?;
            definitions.hide_since(outer);
            Ok(())
        }
        _ => effect.serialize(definitions.walker()),
    }
}

fn validate_deontic(
    deontic: &crate::Deontic,
    definitions: &mut Definitions,
) -> Result<(), ValidationError> {
    let (action, gate) = match deontic {
        crate::Deontic::May(action)
        | crate::Deontic::Cant(action)
        | crate::Deontic::Must(action) => (action, None),
        crate::Deontic::Gate(action, cost) => (action, Some(cost)),
    };
    match action {
        crate::DeonticAction::Cast { what, by, cost, .. } => {
            what.serialize(definitions.walker())?;
            by.serialize(definitions.walker())?;
            validate_predicate_regions(what, &definitions.params)?;
            validate_predicate_regions(by, &definitions.params)?;
            if let Some(crate::AlternativeCost::Components(components)) = cost {
                validate_cost(&crate::Cost(components.clone()), definitions)?;
            }
        }
        _ => action.serialize(definitions.walker())?,
    }
    if let Some(cost) = gate {
        validate_cost(&crate::Cost(cost.clone()), definitions)?;
    }
    Ok(())
}

fn validate_region(
    region: &Region<Block>,
    outer: Option<&[Param]>,
    cost: Option<&crate::Cost>,
) -> Result<(), ValidationError> {
    validate_params(region, outer)?;
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
            | Provenance::Candidate(_) => Some(Kind::Entity),
            Provenance::AnnouncedTarget(_) => Some(Kind::Entities),
            Provenance::AnnouncedX | Provenance::EventAmount | Provenance::Allotment => {
                Some(Kind::Number)
            }
            Provenance::Capture(_) | Provenance::Linked(_) => None,
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
                let Some(outer) = outer else {
                    return Err(ValidationError::UndefinedRead {
                        reference,
                        definitions: 0,
                    });
                };
                validate_read(outer, reference, Some(param.kind))?;
            }
            _ => {}
        }
    }

    let mut definitions = Definitions::new(&region.params);
    // [CR#601.2b]: the announcement block runs first, so its paid products
    // occupy the definitions the body reads.
    if let Some(cost) = cost {
        validate_cost(cost, &mut definitions)?;
    }
    validate_instructions(&region.body, &mut definitions)?;
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "the exhaustive instruction validator keeps definition sequencing visible in one dispatch"
)]
fn validate_instructions(
    instructions: &[Instruction],
    definitions: &mut Definitions,
) -> Result<(), ValidationError> {
    use crate::Instruction as E;
    for instruction in instructions {
        match instruction {
            E::Act { dest, action } => {
                validate_action(action, definitions)?;
                if let Some(dest) = dest {
                    let kind = if action.produces_runtime_magnitude() {
                        Kind::Number
                    } else {
                        match action {
                            crate::Action::MoveGroup { .. } | crate::Action::Create { .. } => {
                                Kind::Entities
                            }
                            _ => Kind::Entity,
                        }
                    };
                    append_definition(definitions, *dest, kind)?;
                }
            }
            E::Choose(choice) => {
                choice.by.serialize(definitions.walker())?;
                choice.quantity.serialize(definitions.walker())?;
                validate_predicate_region(&choice.filter, &definitions.params)?;
                append_definition(definitions, choice.dest, Kind::Entities)?;
            }
            E::ChooseValue(choice) => {
                choice.by.serialize(definitions.walker())?;
                let kind = match choice.domain {
                    crate::ChosenValueKind::Number => Kind::Number,
                    crate::ChosenValueKind::CardName | crate::ChosenValueKind::Color => {
                        Kind::Symbol
                    }
                };
                append_definition(definitions, choice.dest, kind)?;
            }
            E::ChoosePile(choice) => {
                choice.from.serialize(definitions.walker())?;
                for source in choice.from.iter() {
                    validate_read(&definitions.params, *source, Some(Kind::Pile))?;
                }
                choice.by.serialize(definitions.walker())?;
                append_definition(definitions, choice.dest, Kind::Pile)?;
                let outer = definitions.params.len();
                validate_instructions(std::slice::from_ref(choice.then.as_ref()), definitions)?;
                definitions.hide_since(outer);
            }
            E::SeparatePiles(separate) => {
                separate.group.serialize(definitions.walker())?;
                separate.by.serialize(definitions.walker())?;
                validate_selection_regions(&separate.group, &definitions.params)?;
                for dest in separate.dests.iter() {
                    append_definition(definitions, *dest, Kind::Pile)?;
                }
                if let Some(then) = &separate.then {
                    let outer = definitions.params.len();
                    validate_instructions(std::slice::from_ref(then.as_ref()), definitions)?;
                    definitions.hide_since(outer);
                }
            }
            E::Search(search) => {
                search.by.serialize(definitions.walker())?;
                search.whose.serialize(definitions.walker())?;
                search.quantity.serialize(definitions.walker())?;
                validate_predicate_region(&search.filter, &definitions.params)?;
                let outer = definitions.params.len();
                validate_instructions(&search.if_none, definitions)?;
                definitions.hide_since(outer);
                append_definition(definitions, search.dest, Kind::Entities)?;
            }
            E::Let(binding) => {
                validate_pure_expr(&binding.expr)?;
                binding.expr.serialize(definitions.walker())?;
                match &binding.expr {
                    Expr::Objects(selection) => {
                        validate_selection_regions(selection, &definitions.params)?;
                    }
                    Expr::Number(count) => validate_count_regions(count, &definitions.params)?,
                    Expr::Object(_) => {}
                }
                append_definition(definitions, binding.dest, binding.expr.kind())?;
            }
            // [CR#607.1]: the write half of a linked pair. It reads one
            // register of this region and defines nothing here — the value
            // crosses to another ability on the same card, never to a later
            // instruction in this one.
            E::Remember(remember) => {
                // The declared cell kind is checked FIRST: it is the cell's
                // declaration, so a contradiction between it and the register
                // is what the author needs told, not the generic object read.
                validate_read(&definitions.params, remember.value, Some(remember.kind))?;
                crate::Reference::Reg(remember.value).serialize(definitions.walker())?;
            }
            E::Sequentially(block) => validate_instructions(block, definitions)?,
            E::Repeat(count, body) | E::Batch(count, body) => {
                count.serialize(definitions.walker())?;
                validate_count_regions(count, &definitions.params)?;
                let outer = definitions.params.len();
                validate_instructions(std::slice::from_ref(body.as_ref()), definitions)?;
                definitions.hide_since(outer);
            }
            E::Simultaneously(parts) => {
                for part in parts.iter() {
                    let outer = definitions.params.len();
                    validate_instructions(std::slice::from_ref(part), definitions)?;
                    definitions.hide_since(outer);
                }
            }
            E::May(may) => {
                may.who.serialize(definitions.walker())?;
                for branch in [Some(&may.effect), may.if_did.as_ref(), may.if_not.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    let outer = definitions.params.len();
                    validate_instructions(std::slice::from_ref(branch.as_ref()), definitions)?;
                    definitions.hide_since(outer);
                }
            }
            E::Each(each) => {
                each.over.serialize(definitions.walker())?;
                validate_selection_regions(&each.over, &definitions.params)?;
                validate_region(&each.body, Some(&definitions.params), None)?;
            }
            E::Distribute(distribute) => {
                distribute.amount.serialize(definitions.walker())?;
                distribute.over.serialize(definitions.walker())?;
                validate_count_regions(&distribute.amount, &definitions.params)?;
                validate_selection_regions(&distribute.over, &definitions.params)?;
                validate_region(&distribute.body, Some(&definitions.params), None)?;
            }
            E::RevealUntil(reveal) => {
                reveal.whose.serialize(definitions.walker())?;
                validate_predicate_region(&reveal.matches, &definitions.params)?;
                append_definition(definitions, reveal.found, Kind::Entity)?;
                append_definition(definitions, reveal.passed, Kind::Entities)?;
                validate_region(&reveal.body, Some(&definitions.params), None)?;
            }
            E::Delayed(ability) | E::Reflexive(ability) => {
                validate_telescope_with_outer(
                    &ability.effect,
                    &ability.targets,
                    &definitions.params,
                )?;
            }
            E::Modal(modal) => {
                modal.choose.serialize(definitions.walker())?;
                for mode in modal.modes.iter() {
                    // A mode's own cost joins the total at announcement
                    // ([CR#700.2h,601.2f]); its products define into the
                    // mode's region ahead of the mode body.
                    validate_telescope_inner(
                        &mode.effect,
                        &mode.targets,
                        Some(&definitions.params),
                        Some(&mode.cost),
                    )?;
                }
            }
            E::If(branch) => {
                branch.condition.serialize(definitions.walker())?;
                validate_condition_regions(&branch.condition, &definitions.params)?;
                let outer = definitions.params.len();
                validate_instructions(std::slice::from_ref(branch.then.as_ref()), definitions)?;
                definitions.hide_since(outer);
                if let Some(otherwise) = &branch.otherwise {
                    validate_instructions(std::slice::from_ref(otherwise.as_ref()), definitions)?;
                    definitions.hide_since(outer);
                }
            }
            other => other.serialize(definitions.walker())?,
        }
    }
    Ok(())
}

/// Validate a cost block ([CR#601.2b]): an ordered run of cost instructions
/// whose destinations continue the enclosing region's definition sequence, so
/// a paid product stays visible to every later component AND to the ability
/// body that reads it.
fn validate_cost(cost: &crate::Cost, definitions: &mut Definitions) -> Result<(), ValidationError> {
    use crate::CostComponent;

    for component in cost.0.iter() {
        match component {
            CostComponent::Act { dest, action } => {
                validate_action(action.as_action(), definitions)?;
                if let Some(dest) = dest {
                    let kind = match action.as_action() {
                        crate::Action::MoveGroup { .. } | crate::Action::Create { .. } => {
                            Kind::Entities
                        }
                        _ => Kind::Entity,
                    };
                    append_definition(definitions, *dest, kind)?;
                }
            }
            CostComponent::Choose(choice) => {
                choice.by.serialize(definitions.walker())?;
                choice.quantity.serialize(definitions.walker())?;
                validate_predicate_region(&choice.filter, &definitions.params)?;
                append_definition(definitions, choice.dest, Kind::Entities)?;
            }
            CostComponent::Sample(sample) => {
                sample.quantity.serialize(definitions.walker())?;
                validate_predicate_region(&sample.filter, &definitions.params)?;
                append_definition(definitions, sample.dest, Kind::Entities)?;
            }
            CostComponent::Search(search) => {
                search.by.serialize(definitions.walker())?;
                search.whose.serialize(definitions.walker())?;
                search.quantity.serialize(definitions.walker())?;
                validate_predicate_region(&search.filter, &definitions.params)?;
                let outer = definitions.params.len();
                validate_instructions(&search.if_none, definitions)?;
                definitions.hide_since(outer);
                append_definition(definitions, search.dest, Kind::Entities)?;
            }
            CostComponent::Let(binding) => {
                validate_pure_expr(&binding.expr)?;
                binding.expr.serialize(definitions.walker())?;
                match &binding.expr {
                    Expr::Objects(selection) => {
                        validate_selection_regions(selection, &definitions.params)?;
                    }
                    Expr::Number(count) => validate_count_regions(count, &definitions.params)?,
                    Expr::Object(_) => {}
                }
                append_definition(definitions, binding.dest, binding.expr.kind())?;
            }
            CostComponent::Cost(nested) => validate_cost(nested, definitions)?,
            other => other.serialize(definitions.walker())?,
        }
    }
    Ok(())
}

fn validate_action(
    action: &crate::Action,
    definitions: &mut Definitions,
) -> Result<(), ValidationError> {
    match action {
        crate::Action::Pay(cost) => {
            // [CR#118.12a,608.2d]: a resolution-time payment may make
            // decisions inside the enclosing effect's register file. Those
            // definitions dominate the remaining cost components, then leave
            // scope with the payment rather than leaking into the effect that
            // follows.
            let outer = definitions.params.len();
            validate_cost(cost, definitions)?;
            definitions.hide_since(outer);
            Ok(())
        }
        crate::Action::Composite { body, .. } => {
            let outer = definitions.params.len();
            validate_instructions(std::slice::from_ref(body.as_ref()), definitions)?;
            definitions.hide_since(outer);
            Ok(())
        }
        crate::Action::CreateReplacement { replacement, .. } => {
            // Floating replacements become capture-declared regions in Stage 3.
            // Until then, their carried template is closed over the same fixed
            // event-role prefix used when the engine applies the instance.
            let params = event_region_params();
            let mut carried = Definitions::new(&params);
            match replacement.as_ref() {
                crate::Replacement::Instead { would, instead } => {
                    would.serialize(carried.walker())?;
                    validate_instructions(std::slice::from_ref(instead), &mut carried)
                }
                crate::Replacement::Also { would, also } => {
                    would.serialize(carried.walker())?;
                    validate_instructions(std::slice::from_ref(also), &mut carried)
                }
                crate::Replacement::Skip { .. } => Ok(()),
            }
        }
        _ => action.serialize(definitions.walker()),
    }
}

fn validate_pure_expr(expr: &Expr) -> Result<(), ValidationError> {
    expr.serialize(RegisterWalker {
        params: &[],
        visible: None,
        ignore_registers: true,
        reject_decisions: true,
    })
}

fn validate_value_region<T: Serialize>(
    region: &Region<T>,
    outer: &[Param],
) -> Result<(), ValidationError> {
    validate_params(region, Some(outer))?;
    region.body.serialize(RegisterWalker {
        params: &region.params,
        visible: None,
        ignore_registers: false,
        reject_decisions: false,
    })
}

fn validate_predicate_region(
    region: &Region<crate::Predicate>,
    outer: &[Param],
) -> Result<(), ValidationError> {
    validate_value_region(region, outer)?;
    validate_candidate_domain(region.candidate_domain(), &region.body)?;
    validate_predicate_regions(&region.body, &region.params)
}

/// Refuse a predicate whose subject cannot inhabit `declared` — a player is
/// not an object ([CR#109.1,102.1]). Combinators keep the domain; a
/// relation's related filter switches to the relatum's own domain.
fn validate_candidate_domain(
    declared: crate::Domain,
    predicate: &crate::Predicate,
) -> Result<(), ValidationError> {
    use crate::Predicate as P;
    let subject = predicate.subject_domain();
    if subject != crate::Domain::Entity && !declared.admits(entity_class(subject)) {
        return Err(ValidationError::CandidateDomain { declared, subject });
    }
    match predicate {
        P::And(parts) | P::Or(parts) => {
            for part in parts.iter() {
                validate_candidate_domain(declared, part)?;
            }
            Ok(())
        }
        P::Not(part) => validate_candidate_domain(declared, part),
        // Relata that can only be objects: a stack ability's source
        // ([CR#113.7]), a linked relation's other end ([CR#607.1]), and a
        // damage source ([CR#120.1] — only an object deals damage).
        P::FromSource(part)
        | P::State(
            crate::StatePredicate::RelatedBy(_, part)
            | crate::StatePredicate::WasDealtDamageBy(part),
        ) => validate_candidate_domain(crate::Domain::Object, part),
        P::State(crate::StatePredicate::Targets(part)) => {
            validate_candidate_domain(crate::Domain::Entity, part)
        }
        P::Relation(relation) => {
            let (part, domain) = relation.relatum();
            validate_candidate_domain(domain, part)
        }
        _ => Ok(()),
    }
}

/// The Entity class a narrowed domain names. `Entity` has no single class and
/// is filtered out before this is called.
fn entity_class(domain: crate::Domain) -> crate::EntityClass {
    match domain {
        crate::Domain::Player => crate::EntityClass::Player,
        crate::Domain::Object | crate::Domain::Entity => crate::EntityClass::Object,
    }
}

fn validate_predicate_regions(
    predicate: &crate::Predicate,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    use crate::Predicate as P;
    match predicate {
        P::Where(region) => {
            validate_value_region(region, definitions)?;
            validate_condition_regions(&region.body, &region.params)
        }
        P::And(parts) | P::Or(parts) => {
            for part in parts.iter() {
                validate_predicate_regions(part, definitions)?;
            }
            Ok(())
        }
        P::Not(part)
        | P::FromSource(part)
        | P::State(
            crate::StatePredicate::RelatedBy(_, part)
            | crate::StatePredicate::Targets(part)
            | crate::StatePredicate::WasDealtDamageBy(part),
        ) => validate_predicate_regions(part, definitions),
        P::Characteristic(crate::CharacteristicPredicate::Stat(_, _, count))
        | P::PlayerStatCmp(_, _, count) => validate_count_regions(count, definitions),
        P::Relation(relation) => {
            let part = match relation {
                crate::RelationPredicate::ControlledBy(part)
                | crate::RelationPredicate::Controls(part)
                | crate::RelationPredicate::Owner(part)
                | crate::RelationPredicate::OpponentOf(part)
                | crate::RelationPredicate::TeammateOf(part)
                | crate::RelationPredicate::AttachedTo(part)
                | crate::RelationPredicate::Attachment(part) => part,
            };
            validate_predicate_regions(part, definitions)
        }
        _ => Ok(()),
    }
}

fn validate_condition_regions(
    condition: &crate::Condition,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    use crate::Condition as C;
    match condition {
        C::Compare(left, _, right) => {
            validate_count_regions(left, definitions)?;
            validate_count_regions(right, definitions)
        }
        C::Exists(predicate)
        | C::Matches(_, predicate)
        | C::DealtDamageBy(_, predicate)
        | C::TurnOf(predicate) => validate_predicate_regions(predicate, definitions),
        C::Crossed { value, thresholds } => {
            validate_count_regions(value, definitions)?;
            for threshold in thresholds.iter() {
                validate_count_regions(threshold, definitions)?;
            }
            Ok(())
        }
        C::And(parts) | C::Or(parts) => {
            for part in parts.iter() {
                validate_condition_regions(part, definitions)?;
            }
            Ok(())
        }
        C::Not(part) => validate_condition_regions(part, definitions),
        _ => Ok(()),
    }
}

fn validate_selection_regions(
    selection: &crate::Selection,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    use crate::Selection as S;
    match selection {
        // Each register read declares its own collection domain
        // ([CR#700.3b] — a pile is not an Entity group), so the shape it
        // demands follows from the constructor with no cross-domain
        // allowance.
        S::Reg(_) | S::Pile(_) => validate_read(
            definitions,
            match selection {
                S::Reg(reference) | S::Pile(reference) => *reference,
                _ => unreachable!("matched above"),
            },
            Some(selection.collection_domain().register_kind()),
        ),
        S::SelectAll(region) | S::Random(_, region) => {
            validate_predicate_region(region, definitions)
        }
        S::Union(parts) => {
            for part in parts {
                validate_selection_regions(part, definitions)?;
            }
            Ok(())
        }
        S::InChosenOrder(inner, _) => validate_selection_regions(inner, definitions),
        S::Pick { proj, .. } => validate_projection_regions(proj, definitions),
        _ => Ok(()),
    }
}

fn validate_projection_regions(
    projection: &crate::Projection,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    validate_countable_regions(&projection.of, definitions)?;
    validate_value_region(&projection.by, definitions)?;
    validate_count_regions(&projection.by.body, &projection.by.params)
}

fn validate_countable_regions(
    countable: &crate::Countable,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    match countable {
        crate::Countable::Objects(region) | crate::Countable::Players(region) => {
            validate_predicate_region(region, definitions)
        }
        _ => Ok(()),
    }
}

fn validate_count_regions(
    count: &crate::Count,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    use crate::Count as C;
    match count {
        C::CountOf(countable) | C::CountDistinct(_, countable) => {
            validate_countable_regions(countable, definitions)
        }
        C::Min(a, b)
        | C::Max(a, b)
        | C::Plus(a, b)
        | C::Minus(a, b)
        | C::Times(a, b)
        | C::Divide(_, a, b)
        | C::Mod(a, b)
        | C::Pow(a, b) => {
            validate_count_regions(a, definitions)?;
            validate_count_regions(b, definitions)
        }
        C::Half(_, value) => validate_count_regions(value, definitions),
        C::Aggregate(_, projection) => validate_projection_regions(projection, definitions),
        _ => Ok(()),
    }
}

fn validate_params<T>(region: &Region<T>, outer: Option<&[Param]>) -> Result<(), ValidationError> {
    for (index, param) in region.params.iter().enumerate() {
        let expected = DefId(u32::try_from(index).expect("region parameter count fits in u32"));
        if param.def != expected {
            return Err(ValidationError::ParamSequence {
                expected,
                found: param.def,
            });
        }
        if let Provenance::Capture(reference) = param.provenance {
            let Some(outer) = outer else {
                return Err(ValidationError::UndefinedRead {
                    reference,
                    definitions: 0,
                });
            };
            validate_read(outer, reference, Some(param.kind))?;
        }
    }
    Ok(())
}

fn append_definition(
    definitions: &mut Definitions,
    found: DefId,
    kind: Kind,
) -> Result<(), ValidationError> {
    let expected =
        DefId(u32::try_from(definitions.params.len()).expect("definition count fits u32"));
    if found != expected {
        return Err(ValidationError::DefinitionSequence { expected, found });
    }
    definitions.params.push(Param {
        def: found,
        kind,
        provenance: Provenance::Capture(found.into()),
    });
    definitions.visible.push(true);
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
) -> Result<(), ValidationError> {
    validate_telescope_inner(region, targets, None, None)
}

/// Validate an ability whose announcement declares a COST block ([CR#601.2b])
/// alongside its target telescope. The cost's instructions define into the
/// region's ordinal sequence ahead of the body, so the body reads a paid
/// product by register.
///
/// # Errors
///
/// Returns [`ValidationError`] on the same failures as [`validate_telescope`],
/// plus a cost instruction whose destination breaks the definition sequence.
pub fn validate_announced(
    region: &Region,
    targets: &[crate::TargetSpec],
    cost: &crate::Cost,
) -> Result<(), ValidationError> {
    validate_telescope_inner(region, targets, None, Some(cost))
}

fn validate_telescope_with_outer(
    region: &Region,
    targets: &[crate::TargetSpec],
    outer: &[Param],
) -> Result<(), ValidationError> {
    validate_telescope_inner(region, targets, Some(outer), None)
}

fn validate_telescope_inner(
    region: &Region,
    targets: &[crate::TargetSpec],
    outer: Option<&[Param]>,
    cost: Option<&crate::Cost>,
) -> Result<(), ValidationError> {
    validate_region(region, outer, cost)?;
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
            visible: None,
            ignore_registers: false,
            reject_decisions: false,
        })?;
        validate_target_regions(target, &region.params[..limit])?;
    }
    Ok(())
}

fn validate_target_regions(
    target: &crate::TargetSpec,
    definitions: &[Param],
) -> Result<(), ValidationError> {
    match target {
        crate::TargetSpec::Target(_, filter) => validate_predicate_region(filter, definitions),
        crate::TargetSpec::Distinct(_, inner) => validate_target_regions(inner, definitions),
    }
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
        let object_compatible = matches!(expected, Kind::Entity | Kind::Entities)
            && matches!(param.kind, Kind::Entity | Kind::Entities);
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

#[derive(Clone)]
struct Definitions {
    params: Vec<Param>,
    visible: Vec<bool>,
}

impl Definitions {
    fn new(params: &[Param]) -> Self {
        Self {
            params: params.to_vec(),
            visible: vec![true; params.len()],
        }
    }

    fn walker(&self) -> RegisterWalker<'_> {
        RegisterWalker {
            params: &self.params,
            visible: Some(&self.visible),
            ignore_registers: false,
            reject_decisions: false,
        }
    }

    fn hide_since(&mut self, start: usize) {
        self.visible[start..].fill(false);
    }
}

#[derive(Clone, Copy)]
struct RegisterWalker<'a> {
    params: &'a [Param],
    visible: Option<&'a [bool]>,
    ignore_registers: bool,
    reject_decisions: bool,
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
        if variant == "Reg" && !self.ignore_registers {
            let mut capture = RefCapture(None);
            value.serialize(&mut capture)?;
            let reference = RefId(capture.0.ok_or_else(|| {
                ValidationError::Walk("Reg did not contain an integer RefId".into())
            })?);
            let expected = match name {
                "Count" => Some(Kind::Number),
                "Selection" => Some(Kind::Entities),
                "Reference" => Some(Kind::Entity),
                _ => None,
            };
            if self
                .visible
                .is_some_and(|visible| !visible.get(reference.0 as usize).copied().unwrap_or(false))
            {
                return Err(ValidationError::UndefinedRead {
                    reference,
                    definitions: self.params.len(),
                });
            }
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
        Ok(compound(self, name == "Region" && !self.ignore_registers))
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
        name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if self.reject_decisions
            && name == "Selection"
            && matches!(variant, "Random" | "InChosenOrder")
        {
            return Err(ValidationError::DecisionInExpression);
        }
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
            kind: Kind::Entity,
            provenance,
        }
    }

    #[test]
    fn region_display_pins_register_riders_and_parameter_provenance() {
        let region = Region::new(
            announced_region_params(1),
            Instruction::producing(
                DefId(4),
                crate::Action::Move(
                    crate::Reference::Reg(RefId(0)),
                    crate::Destination::Zone(crate::Zone::Exile),
                    Arc::from([]),
                    None,
                ),
            )
            .into(),
        );

        assert_eq!(validate(&region), Ok(()));
        assert_eq!(
            region.to_string(),
            "Region {
    params: [
        DefId(0): Entity <- Source,
        DefId(1): Entity <- Controller,
        DefId(2): Entities <- AnnouncedTarget(0),
        DefId(3): Number <- AnnouncedX,
    ],
    body: [
        Act { dest: Some(DefId(4)), action: Move(Reg(RefId(0)), Zone(Exile), [], None) },
    ],
}"
        );
    }

    /// A card on the stack is a Card AND a Spell ([CR#108.2,112.1]): the two
    /// classes are independently testable, so the conjunction is
    /// representable and the region it lowers into declares the Object
    /// domain.
    #[test]
    fn a_card_on_the_stack_is_both_card_and_spell() {
        let predicate = crate::Predicate::And(Arc::from([
            crate::Predicate::Class(crate::ObjectClass::Card),
            crate::Predicate::Class(crate::ObjectClass::Spell),
        ]));
        let region = Region::over(predicate);
        assert_eq!(region.candidate_domain(), crate::Domain::Object);
        assert_eq!(validate_predicate_region(&region, &[]), Ok(()));
    }

    /// A token on the battlefield is a Token AND a Permanent
    /// ([CR#111.1,110.1]) — the second nonexclusive pair the exclusive kind
    /// axis could not state.
    #[test]
    fn a_battlefield_token_is_both_token_and_permanent() {
        let predicate = crate::Predicate::And(Arc::from([
            crate::Predicate::Class(crate::ObjectClass::Token),
            crate::Predicate::Class(crate::ObjectClass::Permanent),
        ]));
        let region = Region::over(predicate);
        assert_eq!(region.candidate_domain(), crate::Domain::Object);
        assert_eq!(validate_predicate_region(&region, &[]), Ok(()));
    }

    /// A player is not an object ([CR#109.1,102.1]), so a Player-only
    /// predicate is refused at load in an Object-domain region — both the
    /// Entity-level class and the player-scope numeric read.
    #[test]
    fn a_player_only_predicate_is_refused_in_the_object_domain() {
        let object_domain = |body| Region::candidate_in(crate::Domain::Object, body);
        let refusal = ValidationError::CandidateDomain {
            declared: crate::Domain::Object,
            subject: crate::Domain::Player,
        };
        assert_eq!(
            validate_predicate_region(&object_domain(crate::Predicate::player()), &[]),
            Err(refusal.clone())
        );
        assert_eq!(
            validate_predicate_region(
                &object_domain(crate::Predicate::PlayerStatCmp(
                    crate::PlayerAttr::Life,
                    crate::Cmp::AtMost,
                    crate::Count::Literal(13),
                )),
                &[]
            ),
            Err(refusal.clone())
        );
        // Nested under a combinator too — the domain travels through And.
        assert_eq!(
            validate_predicate_region(
                &object_domain(crate::Predicate::And(Arc::from([
                    crate::Predicate::Class(crate::ObjectClass::Permanent),
                    crate::Predicate::player(),
                ]))),
                &[]
            ),
            Err(refusal)
        );
        // The same predicate is fine where players are the candidates.
        assert_eq!(
            validate_predicate_region(
                &Region::candidate_in(crate::Domain::Player, crate::Predicate::player()),
                &[]
            ),
            Ok(())
        );
    }

    /// The mirror refusal: an Object-only predicate cannot range over players.
    #[test]
    fn an_object_only_predicate_is_refused_in_the_player_domain() {
        assert_eq!(
            validate_predicate_region(
                &Region::candidate_in(
                    crate::Domain::Player,
                    crate::Predicate::Class(crate::ObjectClass::Token)
                ),
                &[]
            ),
            Err(ValidationError::CandidateDomain {
                declared: crate::Domain::Player,
                subject: crate::Domain::Object,
            })
        );
    }

    /// A relation's related filter switches domain: "controlled by an
    /// opponent" is an Object-domain predicate whose relatum is a player
    /// ([CR#109.5,102.2]).
    #[test]
    fn a_relation_switches_the_domain_of_its_relatum() {
        let controlled_by_a_player =
            |who| crate::Predicate::Relation(crate::RelationPredicate::ControlledBy(Arc::new(who)));
        let region = Region::over(controlled_by_a_player(crate::Predicate::player()));
        assert_eq!(region.candidate_domain(), crate::Domain::Object);
        assert_eq!(validate_predicate_region(&region, &[]), Ok(()));
        // The relatum of `ControlledBy` is a player, so an object class there
        // is refused even though the region's own domain is Object.
        assert_eq!(
            validate_predicate_region(
                &Region::over(controlled_by_a_player(crate::Predicate::Class(
                    crate::ObjectClass::Token
                ))),
                &[]
            ),
            Err(ValidationError::CandidateDomain {
                declared: crate::Domain::Player,
                subject: crate::Domain::Object,
            })
        );
    }

    /// [CR#120.1]: only an object deals damage, so the damage-source
    /// predicate reads its relatum in the Object domain — a player class
    /// there is refused, and the atom itself keeps the DAMAGED object in the
    /// Object domain.
    #[test]
    fn a_damage_source_is_read_in_the_object_domain() {
        let dealt_by = |source| {
            crate::Predicate::State(crate::StatePredicate::WasDealtDamageBy(Arc::new(source)))
        };
        let region = Region::over(dealt_by(crate::Predicate::Characteristic(
            crate::CharacteristicPredicate::Has(crate::KeywordRef::from("Deathtouch")),
        )));
        assert_eq!(region.candidate_domain(), crate::Domain::Object);
        assert_eq!(validate_predicate_region(&region, &[]), Ok(()));
        assert_eq!(
            validate_predicate_region(&Region::over(dealt_by(crate::Predicate::player())), &[]),
            Err(ValidationError::CandidateDomain {
                declared: crate::Domain::Object,
                subject: crate::Domain::Player,
            })
        );
    }

    /// Every query constructor names the Entity domain it ranges over, so a
    /// Player-only predicate cannot silently run over objects
    /// ([CR#109.1,102.1]). A register read is the one constructor that
    /// constrains nothing on its own — its declared parameter kind does.
    #[test]
    fn every_query_constructor_names_its_domain() {
        use crate::Domain;
        use crate::Reference;
        use crate::Selection;

        let reg = || Reference::Reg(RefId(0));
        // References: each relation names the class its result belongs to
        // ([CR#109.5,108.3,102.2,303.4b]).
        assert_eq!(reg().referent_domain(), Domain::Entity);
        assert_eq!(
            Reference::ControllerOf(Arc::new(reg())).referent_domain(),
            Domain::Player
        );
        assert_eq!(
            Reference::OwnerOf(Arc::new(reg())).referent_domain(),
            Domain::Player
        );
        assert_eq!(
            Reference::OpponentOf(Arc::new(reg())).referent_domain(),
            Domain::Player
        );
        // [CR#303.4b]: an Aura's host is "that object or player", so the
        // attachment host does NOT narrow to the Object domain.
        assert_eq!(
            Reference::AttachHostOf(Arc::new(reg())).referent_domain(),
            Domain::Entity
        );
        // A demoted selection inherits its members' domain.
        let players = Selection::SelectAll(Arc::new(Region::over(crate::Predicate::player())));
        assert_eq!(
            Reference::Single(Arc::new(players.clone())).referent_domain(),
            Domain::Player
        );
        // A coalesce over disagreeing arms widens rather than lying.
        assert_eq!(
            Reference::Coalesce(Arc::from([
                Reference::ControllerOf(Arc::new(reg())),
                Reference::Single(Arc::new(Selection::LibraryOf(reg()))),
            ]))
            .referent_domain(),
            Domain::Entity
        );
        assert_eq!(
            Reference::Coalesce(Arc::from([
                Reference::ControllerOf(Arc::new(reg())),
                Reference::OwnerOf(Arc::new(reg())),
            ]))
            .referent_domain(),
            Domain::Player
        );

        // Selections: the declared region domain, the zone-slice object
        // domain, and [CR#707.10d]'s explicitly mixed one.
        assert_eq!(players.element_domain(), Domain::Player);
        assert_eq!(
            Selection::Random(
                crate::Quantity::one(),
                Arc::new(Region::over(crate::Predicate::creature()))
            )
            .element_domain(),
            Domain::Object
        );
        assert_eq!(Selection::LibraryOf(reg()).element_domain(), Domain::Object);
        assert_eq!(Selection::Pile(RefId(1)).element_domain(), Domain::Object);
        assert_eq!(
            Selection::ValidTargetsFor(reg()).element_domain(),
            Domain::Entity
        );
        assert_eq!(Selection::Reg(RefId(1)).element_domain(), Domain::Entity);

        // And the collection domains stay apart ([CR#700.3b]).
        assert_eq!(
            players.collection_domain(),
            crate::CollectionDomain::Entities
        );
        assert_eq!(
            Selection::Pile(RefId(1)).collection_domain(),
            crate::CollectionDomain::Pile
        );
    }

    /// A `Ref` predicate is "the candidate IS r", so it carries the
    /// reference's own domain into the region's declaration: a filter naming
    /// a controller declares the Player domain, and an object class beside it
    /// is refused ([CR#109.5]).
    #[test]
    fn a_reference_predicate_narrows_to_its_referents_domain() {
        let controller = crate::Predicate::Ref(crate::Reference::ControllerOf(Arc::new(
            crate::Reference::source_parameter(),
        )));
        assert_eq!(controller.subject_domain(), crate::Domain::Player);
        assert_eq!(
            Region::over(controller.clone()).candidate_domain(),
            crate::Domain::Player
        );
        assert_eq!(
            validate_predicate_region(
                &Region::candidate_in(
                    crate::Domain::Object,
                    crate::Predicate::And(Arc::from([
                        controller,
                        crate::Predicate::Class(crate::ObjectClass::Permanent),
                    ]))
                ),
                &[]
            ),
            Err(ValidationError::CandidateDomain {
                declared: crate::Domain::Object,
                subject: crate::Domain::Player,
            })
        );
    }

    #[test]
    fn sequential_effect_becomes_a_textual_block() {
        let block = Block::from(Instruction::Sequentially(Arc::from([])));
        assert!(block.is_empty());
    }

    #[test]
    fn parameter_definitions_are_a_dense_prefix() {
        let valid = Region::new(
            Arc::from([
                param(0, Provenance::Source),
                param(1, Provenance::Controller),
            ]),
            Block::default(),
        );
        assert_eq!(validate(&valid), Ok(()));

        let invalid = Region::new(
            Arc::from([
                param(0, Provenance::Source),
                param(2, Provenance::Controller),
            ]),
            Block::default(),
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
            Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(1)))).into(),
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
            Instruction::Repeat(
                crate::Count::Reg(RefId(0)),
                Arc::new(Instruction::Sequentially(Arc::from([]))),
            )
            .into(),
        );
        assert_eq!(
            validate(&wrong_kind),
            Err(ValidationError::KindMismatch {
                reference: RefId(0),
                expected: Kind::Number,
                found: Kind::Entity,
            })
        );
    }

    fn always() -> crate::Condition {
        crate::Condition::Compare(
            crate::Count::Literal(0),
            crate::Cmp::Eq,
            crate::Count::Literal(0),
        )
    }

    fn object_let(dest: u32, source: u32) -> Instruction {
        Instruction::Let(crate::Let {
            dest: DefId(dest),
            expr: Expr::Object(crate::Reference::Reg(RefId(source))),
        })
    }

    #[test]
    fn branch_definitions_are_textual_but_lexically_scoped() {
        let branch = Instruction::If(crate::If {
            condition: always(),
            then: Arc::new(Instruction::Sequentially(Arc::from([
                object_let(1, 0),
                Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(1)))),
            ]))),
            otherwise: Some(Arc::new(Instruction::Sequentially(Arc::from([
                object_let(2, 0),
                Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(2)))),
            ])))),
        });
        let valid = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Block(Arc::from([
                branch.clone(),
                object_let(3, 0),
                Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(3)))),
            ])),
        );
        assert_eq!(validate(&valid), Ok(()));

        let sibling_read = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::If(crate::If {
                condition: always(),
                then: match branch {
                    Instruction::If(branch) => branch.then,
                    _ => unreachable!(),
                },
                otherwise: Some(Arc::new(Instruction::act(crate::Action::Counter(
                    crate::Reference::Reg(RefId(1)),
                )))),
            })
            .into(),
        );
        assert_eq!(
            validate(&sibling_read),
            Err(ValidationError::UndefinedRead {
                reference: RefId(1),
                definitions: 2,
            })
        );
    }

    /// [CR#118.12a,608.2d]: a decision-bearing resolution cost declares its
    /// chooser in the enclosing region long enough for the paying verb to read
    /// it, while the next effect instruction continues after that definition.
    #[test]
    fn pay_cost_decisions_extend_the_enclosing_definition_sequence() {
        let chosen = DefId(1);
        let cost = crate::Cost(
            vec![
                crate::CostComponent::Choose(crate::Choose {
                    dest: chosen,
                    by: crate::Reference::Reg(RefId(0)),
                    quantity: crate::Quantity::one(),
                    filter: Arc::new(Region::candidate(crate::Predicate::Any)),
                }),
                crate::CostComponent::Act {
                    dest: None,
                    action: crate::RunnableCostAction::try_new(crate::Action::Sacrifice(
                        crate::Reference::Reg(RefId(0)),
                        crate::Reference::Reg(chosen.into()),
                    ))
                    .expect("a bound sacrifice is a runnable cost action"),
                },
            ]
            .into(),
        );
        let region = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Block(vec![Instruction::act(crate::Action::Pay(cost)), object_let(2, 0)].into()),
        );

        assert_eq!(validate(&region), Ok(()));
    }

    #[test]
    fn nested_regions_are_closed_against_their_own_params() {
        let nested = Region::new(
            Arc::from([]),
            Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(0)))).into(),
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
            Instruction::Delayed(Arc::new(ability)).into(),
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
                    kind: Kind::Entities,
                    provenance: Provenance::AnnouncedTarget(0),
                },
                Param {
                    def: DefId(1),
                    kind: Kind::Entities,
                    provenance: Provenance::AnnouncedTarget(0),
                },
            ]),
            Block::default(),
        );
        assert_eq!(
            validate(&duplicate),
            Err(ValidationError::TargetSequence {
                expected: 1,
                found: 0,
            })
        );

        let target = crate::TargetSpec::Target(
            crate::Quantity::one(),
            Arc::new(Region::new(Arc::from([]), crate::Predicate::any())),
        );
        let missing = Region::new(Arc::from([]), Block::default());
        assert_eq!(
            validate_telescope(&missing, &[target]),
            Err(ValidationError::TargetCountMismatch {
                targets: 1,
                parameters: 0,
            })
        );
    }

    fn delayed_with(effect: Region<Block>) -> crate::TriggeredAbility {
        crate::TriggeredAbility {
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
            effect,
        }
    }

    /// ADR law 7: a created body's capture must name a definition of the region
    /// that CREATED it. One that does not is a load-time refusal, which is what
    /// makes "a declared capture is never unavailable at firing" true — the
    /// engine supplies every capture the validator let through.
    #[test]
    fn a_capture_naming_no_definition_of_the_creating_region_is_refused() {
        let body = Region::new(
            Arc::from([Param {
                def: DefId(0),
                kind: Kind::Entity,
                provenance: Provenance::Capture(RefId(3)),
            }]),
            Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(0)))).into(),
        );
        let outer = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::Delayed(Arc::new(delayed_with(body))).into(),
        );
        assert_eq!(
            validate(&outer),
            Err(ValidationError::UndefinedRead {
                reference: RefId(3),
                definitions: 1,
            })
        );
    }

    /// The same capture, naming a register the creating region really declares,
    /// validates — and the created body may read it.
    #[test]
    fn a_capture_of_a_declared_register_validates() {
        let body = Region::new(
            Arc::from([Param {
                def: DefId(0),
                kind: Kind::Entity,
                provenance: Provenance::Capture(RefId(0)),
            }]),
            Instruction::act(crate::Action::Counter(crate::Reference::Reg(RefId(0)))).into(),
        );
        let outer = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::Delayed(Arc::new(delayed_with(body))).into(),
        );
        assert_eq!(validate(&outer), Ok(()));
    }

    /// A capture at an ABILITY ROOT has no enclosing region to take a value
    /// from, so it is refused however it is numbered ([Core is explicit
    /// regions] law 7 — captures cross a boundary, and a root has none).
    #[test]
    fn a_capture_at_an_ability_root_is_refused() {
        let root = Region::new(
            Arc::from([Param {
                def: DefId(0),
                kind: Kind::Entity,
                provenance: Provenance::Capture(RefId(0)),
            }]),
            Block::default(),
        );
        assert_eq!(
            validate(&root),
            Err(ValidationError::UndefinedRead {
                reference: RefId(0),
                definitions: 0,
            })
        );
    }

    /// ADR law 8: `Remember` reads one register of its own region and defines
    /// nothing. A write of a register the region never defined is refused.
    #[test]
    fn remember_reads_a_register_of_its_own_region() {
        let valid = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::Remember(crate::Remember {
                cell: crate::Ident::from("exiled"),
                kind: Kind::Entity,
                value: RefId(0),
            })
            .into(),
        );
        assert_eq!(validate(&valid), Ok(()));

        let undefined = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::Remember(crate::Remember {
                cell: crate::Ident::from("exiled"),
                kind: Kind::Entity,
                value: RefId(1),
            })
            .into(),
        );
        assert_eq!(
            validate(&undefined),
            Err(ValidationError::UndefinedRead {
                reference: RefId(1),
                definitions: 1,
            })
        );
    }

    /// A `Remember` whose declared cell kind contradicts the register it reads
    /// is refused: the cell's runtime shape is a declaration, not a hint.
    #[test]
    fn remember_refuses_a_cell_kind_the_register_cannot_hold() {
        let mismatched = Region::new(
            Arc::from([Param {
                def: DefId(0),
                kind: Kind::Number,
                provenance: Provenance::AnnouncedX,
            }]),
            Instruction::Remember(crate::Remember {
                cell: crate::Ident::from("chosen"),
                kind: Kind::Symbol,
                value: RefId(0),
            })
            .into(),
        );
        assert_eq!(
            validate(&mismatched),
            Err(ValidationError::KindMismatch {
                reference: RefId(0),
                expected: Kind::Symbol,
                found: Kind::Number,
            })
        );
    }

    /// `Region::captures` IS the declaration a created body's supplier has to
    /// satisfy: every capture parameter, paired with the enclosing register it
    /// names.
    #[test]
    fn captures_enumerates_exactly_the_capture_parameters() {
        let body: Region<Block> = Region::new(
            Arc::from([
                param(0, Provenance::Source),
                Param {
                    def: DefId(1),
                    kind: Kind::Entities,
                    provenance: Provenance::Capture(RefId(4)),
                },
                Param {
                    def: DefId(2),
                    kind: Kind::Entity,
                    provenance: Provenance::Linked(crate::Ident::from("exiled")),
                },
            ]),
            Block::default(),
        );
        assert_eq!(
            body.captures().collect::<Vec<_>>(),
            vec![(RefId(1), RefId(4), Kind::Entities)]
        );
        assert_eq!(
            body.linked_cells()
                .map(|(here, cell, kind)| (here, cell.to_string(), kind))
                .collect::<Vec<_>>(),
            vec![(RefId(2), "exiled".to_owned(), Kind::Entity)]
        );
    }

    /// Pile-producing instructions form a typed register chain: separating
    /// defines each candidate pile, choosing reads only those pile registers,
    /// and the chosen pile is iterated through the pile-domain selection
    /// ([CR#700.3a..700.3b]). Re-spelled from the Entity-group register read
    /// this stage splits off: the subject, the chain, and the verdict are
    /// unchanged.
    #[test]
    fn pile_registers_validate_as_iterable_groups() {
        let loop_body = Region::new(
            Arc::from([Param {
                def: DefId(0),
                kind: Kind::Entity,
                provenance: Provenance::LoopElement,
            }]),
            Block::default(),
        );
        let region = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::SeparatePiles(crate::SeparatePiles {
                dests: Arc::from([DefId(1), DefId(2)]),
                group: crate::Selection::Union(Vec::new()),
                by: crate::Reference::Reg(RefId(0)),
                then: Some(Arc::new(Instruction::ChoosePile(crate::ChoosePile {
                    dest: DefId(3),
                    from: Arc::from([RefId(1), RefId(2)]),
                    by: crate::Reference::Reg(RefId(0)),
                    random: false,
                    then: Arc::new(Instruction::Each(crate::Each {
                        over: crate::Selection::Pile(RefId(3)),
                        body: loop_body,
                    })),
                }))),
            })
            .into(),
        );

        assert_eq!(validate(&region), Ok(()));
    }

    /// The two collection domains do not answer for each other: a pile
    /// register read as an Entity group, or an Entity group read as a pile,
    /// is refused at load ([CR#700.3b] — the pile is not an object).
    #[test]
    fn a_pile_register_is_not_an_entity_group() {
        // The same separate → choose chain as above; only the spelling of the
        // read over the chosen pile (register 3) varies.
        let over = |selection| {
            Region::new(
                Arc::from([param(0, Provenance::Source)]),
                Instruction::SeparatePiles(crate::SeparatePiles {
                    dests: Arc::from([DefId(1), DefId(2)]),
                    group: crate::Selection::Union(Vec::new()),
                    by: crate::Reference::Reg(RefId(0)),
                    then: Some(Arc::new(Instruction::ChoosePile(crate::ChoosePile {
                        dest: DefId(3),
                        from: Arc::from([RefId(1), RefId(2)]),
                        by: crate::Reference::Reg(RefId(0)),
                        random: false,
                        then: Arc::new(Instruction::Each(crate::Each {
                            over: selection,
                            body: Region::new(
                                Arc::from([Param {
                                    def: DefId(0),
                                    kind: Kind::Entity,
                                    provenance: Provenance::LoopElement,
                                }]),
                                Block::default(),
                            ),
                        })),
                    }))),
                })
                .into(),
            )
        };

        assert_eq!(validate(&over(crate::Selection::Pile(RefId(3)))), Ok(()));
        assert_eq!(
            validate(&over(crate::Selection::Reg(RefId(3)))),
            Err(ValidationError::KindMismatch {
                reference: RefId(3),
                expected: Kind::Entities,
                found: Kind::Pile,
            }),
        );
        assert_eq!(
            validate(&over(crate::Selection::Pile(RefId(0)))),
            Err(ValidationError::KindMismatch {
                reference: RefId(0),
                expected: Kind::Pile,
                found: Kind::Entity,
            }),
        );
    }

    /// A pile choice cannot consume an ordinary object register: a pile is an
    /// object group, but is not itself an object ([CR#700.3b]).
    #[test]
    fn choose_pile_rejects_non_pile_registers() {
        let region = Region::new(
            Arc::from([param(0, Provenance::Source)]),
            Instruction::ChoosePile(crate::ChoosePile {
                dest: DefId(1),
                from: Arc::from([RefId(0)]),
                by: crate::Reference::Reg(RefId(0)),
                random: false,
                then: Arc::new(Instruction::Sequentially(Arc::from([]))),
            })
            .into(),
        );

        assert_eq!(
            validate(&region),
            Err(ValidationError::KindMismatch {
                reference: RefId(0),
                expected: Kind::Pile,
                found: Kind::Entity,
            })
        );
    }
}
