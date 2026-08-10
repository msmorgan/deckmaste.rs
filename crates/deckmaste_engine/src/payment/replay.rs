use std::collections::HashMap;

use super::FulfillmentWitness;
use super::IouId;
use super::PaymentCommand;
use super::PaymentController;
use super::PaymentFrame;
use super::PaymentRecordId;
use super::PaymentStage;
use crate::decide::Decision;
use crate::event::GameEvent;
use crate::object::CardId;
use crate::object::ObjectId;
use crate::player::FloatingManaId;
use crate::player::ManaActionId;
use crate::player::PlayerId;
use crate::state::GameImage;
use crate::state::GameState;
use crate::step::StepOutcome;

/// A stable transaction-local name for an object whose concrete slot-map key
/// may change during reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicalObject {
    Card(CardId),
    Player(PlayerId),
    Created {
        record: PaymentRecordId,
        ordinal: usize,
    },
}

/// Concrete IDs minted while replaying logical objects and mana production.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayMap {
    pub objects: HashMap<LogicalObject, ObjectId>,
    pub mana: HashMap<(PaymentRecordId, usize), FloatingManaId>,
}

/// An ordinary decision sequence made while one transaction command was in
/// flight. Replay feeds these answers back through the same decision handlers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DecisionTranscript {
    pub answers: Vec<Decision>,
}

/// The deterministic operation represented by one payment-ledger record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayCommand {
    Fulfill {
        iou: IouId,
        witness: FulfillmentWitness,
    },
    ManaAbility {
        action: ManaActionId,
        source: LogicalObject,
        ability: usize,
        /// False when decline abandoned the activation after a barred cost
        /// had already happened but before the stackless mana effect began.
        submitted: bool,
    },
}

/// A concrete CR 733 prohibition found after replacements finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReversalBarrier {
    MovedToLibrary,
    MovedFromLibrary,
    ShuffledLibrary,
    RevealedLibraryCard,
}

/// Information that cannot be forgotten even when physical state rewinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObservationBarrier {
    RandomOutcome,
    HiddenZoneDisclosure,
}

/// One completed payment operation and the evidence needed to replay it
/// without choosing, rerolling, or repairing anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionRecord {
    pub id: PaymentRecordId,
    pub command: ReplayCommand,
    pub transcript: DecisionTranscript,
    /// Concrete input IDs captured at submission and their stable logical
    /// identities. Replay resolves these afresh after earlier zone remints.
    pub object_inputs: HashMap<ObjectId, LogicalObject>,
    /// Fulfillment records owned by a mana ability's nested payment frame.
    /// They are replay instructions for the action, not separately selectable
    /// CR 733 mana reversals.
    pub children: Vec<TransactionRecord>,
    pub facts: Vec<GameEvent>,
    pub produced_mana: Vec<FloatingManaId>,
    pub spent_mana: Vec<FloatingManaId>,
    pub dependencies: Vec<PaymentRecordId>,
    pub reversal_barriers: Vec<ReversalBarrier>,
    pub observation_barriers: Vec<ObservationBarrier>,
}

impl TransactionRecord {
    #[must_use]
    pub fn iou(&self) -> Option<IouId> {
        match self.command {
            ReplayCommand::Fulfill { iou, .. } => Some(iou),
            ReplayCommand::ManaAbility { .. } => None,
        }
    }

    #[must_use]
    pub fn selectively_reversible(&self) -> bool {
        self.reversal_barriers.is_empty() && self.observation_barriers.is_empty()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PendingRecord {
    pub id: PaymentRecordId,
    pub command: ReplayCommand,
    pub transcript: DecisionTranscript,
    pub object_inputs: HashMap<ObjectId, LogicalObject>,
    pub history_start: usize,
    pub spent_mana: Vec<FloatingManaId>,
    pub reversal_barriers: Vec<ReversalBarrier>,
    pub observation_barriers: Vec<ObservationBarrier>,
}

/// Reconstruction rejects rather than changing a recorded choice or outcome.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReplayError {
    #[error("the payment frame has no reconstruction base")]
    MissingBase,
    #[error("a retained payment record is missing")]
    MissingRecord,
    #[error("a retained child-cost record did not match the replayed cost trace")]
    MissingChildRecord,
    #[error("the replayed mana action was no longer active when cancellation ran")]
    MissingActiveManaAction,
    #[error("a recorded decision transcript ended early")]
    MissingDecision,
    #[error("a recorded decision is no longer legal: {0}")]
    DecisionRejected(String),
    #[error("replay reached game end before the payment command completed")]
    GameEnded,
    #[error("the replayed fact trace differs from the recorded trace")]
    FactMismatch,
    #[error("this replay command is not implemented yet")]
    UnsupportedCommand,
}

pub(crate) struct ReconstructedFrame {
    pub working: GameImage,
    pub stage: PaymentStage,
    pub coverage: Option<super::ManaCoverage>,
    pub fulfilled: Vec<(IouId, FulfillmentWitness)>,
    pub records: Vec<TransactionRecord>,
}

/// Rebuild only `retained` records from the frame's single payment base.
///
/// # Errors
///
/// Returns a [`ReplayError`] without mutating `frame` when a command, recorded
/// answer, or concrete fact trace cannot be reproduced exactly.
pub fn reconstruct(
    frame: &PaymentFrame,
    retained: &[PaymentRecordId],
) -> Result<GameImage, ReplayError> {
    reconstruct_frame(frame, retained).map(|rebuilt| rebuilt.working)
}

pub(crate) fn reconstruct_frame(
    frame: &PaymentFrame,
    retained: &[PaymentRecordId],
) -> Result<ReconstructedFrame, ReplayError> {
    let base = frame.payment_base.clone().ok_or(ReplayError::MissingBase)?;
    let records: Vec<TransactionRecord> = retained
        .iter()
        .map(|id| {
            frame
                .records
                .iter()
                .find(|record| record.id == *id)
                .cloned()
                .ok_or(ReplayError::MissingRecord)
        })
        .collect::<Result<_, _>>()?;

    let mut replay_frame =
        PaymentFrame::new(base.clone(), frame.purpose.clone(), frame.locked.clone());
    replay_frame.payment_base = Some(base);
    replay_frame.stage = PaymentStage::Paying;
    replay_frame.coverage = frame.coverage.clone();
    let prompt = replay_frame.prompt(Vec::new());
    replay_frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
    let mut state = GameState {
        committed: replay_frame.working.clone(),
        payment: Some(PaymentController {
            frames: vec![replay_frame],
            mana_actions: Vec::new(),
            next_record: 0,
        }),
        incidents: Vec::new(),
        next_mana_action: 0,
    };

    let mut replay_map = ReplayMap::default();
    // `payment_base` already contains every mana action completed during
    // PrePayment. Their records remain selectable metadata, but replaying
    // those actions here would enact them twice (and commonly fail because a
    // tap/sacrifice cost is already reflected in the base). Only fulfillment
    // records advance a frame forward from its payment base.
    for record in records.iter().filter(|record| record.iou().is_some()) {
        replay_record(&mut state, record, &frame.records, &mut replay_map)?;
    }

    let mut controller = state.payment.take().ok_or(ReplayError::MissingBase)?;
    let mut rebuilt = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    rebuilt.records = records.clone();
    rebuilt.recording = None;
    if records.iter().all(|record| record.iou().is_none()) && frame.locked.has_mana_payment {
        rebuilt.stage = PaymentStage::PrePayment;
        rebuilt.coverage = None;
        rebuilt.payment_base = None;
    }
    rebuilt.working.pending = None;
    let stage = rebuilt.stage;
    let coverage = rebuilt.coverage.clone();
    let fulfilled = rebuilt.fulfilled.clone();
    let working = rebuilt.working;
    Ok(ReconstructedFrame {
        working,
        stage,
        coverage,
        fulfilled,
        records,
    })
}

fn replay_record(
    state: &mut GameState,
    record: &TransactionRecord,
    all_records: &[TransactionRecord],
    replay_map: &mut ReplayMap,
) -> Result<(), ReplayError> {
    let history_start = state.history.len();
    match &record.command {
        ReplayCommand::Fulfill { iou, witness } => {
            let witness = rebind_witness(witness, &record.object_inputs, state)?;
            state
                .submit_payment_command(PaymentCommand::Fulfill { iou: *iou, witness })
                .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
            replay_fulfillment_answers(state, &record.transcript)?;
        }
        ReplayCommand::ManaAbility {
            action,
            source,
            ability,
            submitted,
        } => {
            let source = resolve_logical_object(state, *source)?;
            state.next_mana_action = action.0;
            state
                .submit_payment_command(PaymentCommand::ActivateManaAbility {
                    source,
                    ability: *ability,
                })
                .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
            replay_mana_action_answers(
                state,
                *action,
                &record.transcript,
                &record.children,
                all_records,
                replay_map,
                *submitted,
            )?;
        }
    }
    let replayed = state.history.facts_from(history_start);
    if replayed.len() != record.facts.len()
        || replayed
            .iter()
            .zip(&record.facts)
            .any(|(actual, expected)| !facts_equivalent(actual, expected))
    {
        return Err(ReplayError::FactMismatch);
    }
    let produced: Vec<FloatingManaId> = replayed
        .iter()
        .filter_map(|fact| match fact {
            GameEvent::ManaAdded(event) => Some(event.units.iter().copied()),
            _ => None,
        })
        .flatten()
        .collect();
    if produced.len() != record.produced_mana.len() {
        return Err(ReplayError::FactMismatch);
    }
    for (ordinal, id) in produced.into_iter().enumerate() {
        replay_map.mana.insert((record.id, ordinal), id);
    }
    Ok(())
}

fn replay_fulfillment_answers(
    state: &mut GameState,
    transcript: &DecisionTranscript,
) -> Result<(), ReplayError> {
    let mut answers = transcript.answers.iter().cloned();
    loop {
        if let Some(pending) = state.pending.clone() {
            if matches!(pending, crate::decide::PendingDecision::Payment(_)) {
                break;
            }
            let answer = answers.next().ok_or(ReplayError::MissingDecision)?;
            state
                .submit_decision(answer)
                .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
            continue;
        }
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(crate::decide::PendingDecision::Payment(_)) => break,
            StepOutcome::NeedsDecision(_) => {}
            StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
        }
    }
    if answers.next().is_some() {
        return Err(ReplayError::DecisionRejected(
            "the recorded transcript has unused answers".into(),
        ));
    }
    Ok(())
}

fn replay_mana_action_answers(
    state: &mut GameState,
    action: ManaActionId,
    transcript: &DecisionTranscript,
    children: &[TransactionRecord],
    all_records: &[TransactionRecord],
    replay_map: &ReplayMap,
    submitted: bool,
) -> Result<(), ReplayError> {
    for answer in transcript.answers.iter().cloned() {
        while state.pending.is_none() {
            match state.step() {
                StepOutcome::Progress(_) | StepOutcome::NeedsDecision(_) => {}
                StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
            }
        }
        state
            .submit_decision(rebind_decision(
                answer,
                children,
                all_records,
                replay_map,
                state,
            )?)
            .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
    }
    if !submitted {
        while state.pending.is_none() {
            match state.step() {
                StepOutcome::Progress(_) | StepOutcome::NeedsDecision(_) => {}
                StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
            }
        }
        if !matches!(
            state.pending,
            Some(crate::decide::PendingDecision::Payment(_))
        ) {
            return Err(ReplayError::MissingDecision);
        }
        return cancel_replayed_mana_proposal(state, action, children);
    }
    loop {
        let finished = state.payment.as_ref().is_some_and(|controller| {
            controller.frames.len() == 1
                && controller
                    .mana_actions
                    .iter()
                    .all(|active| active.id != action)
        });
        if finished
            && matches!(
                state.pending,
                Some(crate::decide::PendingDecision::Payment(_))
            )
        {
            return Ok(());
        }
        if state.pending.is_some() {
            return Err(ReplayError::MissingDecision);
        }
        match state.step() {
            StepOutcome::Progress(_) | StepOutcome::NeedsDecision(_) => {}
            StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
        }
    }
}

pub(crate) fn cancel_replayed_mana_proposal(
    state: &mut GameState,
    action: ManaActionId,
    retained_costs: &[TransactionRecord],
) -> Result<(), ReplayError> {
    let (child, retained) = {
        let child = state
            .payment
            .as_ref()
            .and_then(|controller| controller.frames.last())
            .ok_or(ReplayError::MissingBase)?;
        let retained = retained_costs
            .iter()
            .map(|expected| {
                child
                    .records
                    .iter()
                    .find(|actual| {
                        actual.iou() == expected.iou()
                            && actual.facts.len() == expected.facts.len()
                            && actual
                                .facts
                                .iter()
                                .zip(&expected.facts)
                                .all(|(actual, expected)| facts_equivalent(actual, expected))
                    })
                    .map(|record| record.id)
                    .ok_or(ReplayError::MissingChildRecord)
            })
            .collect::<Result<Vec<_>, _>>()?;
        (child, retained)
    };
    let rebuilt = { reconstruct_frame(child, &retained)? };
    let controller = state.payment.as_mut().ok_or(ReplayError::MissingBase)?;
    let mut child = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    child.working = rebuilt.working;
    child.working.pending = None;
    if let Some(announcing) = child.working.announcing.take()
        && child.working.objects.get(announcing.id).is_some()
    {
        child.working.objects.remove(announcing.id);
    }
    child
        .working
        .resume_control()
        .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
    let active = controller
        .mana_actions
        .pop()
        .ok_or(ReplayError::MissingActiveManaAction)?;
    if active.id != action {
        return Err(ReplayError::MissingActiveManaAction);
    }
    let parent = controller
        .frames
        .last_mut()
        .ok_or(ReplayError::MissingBase)?;
    parent.working = child.working;
    Ok(())
}

fn rebind_decision(
    decision: Decision,
    children: &[TransactionRecord],
    records: &[TransactionRecord],
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<Decision, ReplayError> {
    match decision {
        Decision::Payment(PaymentCommand::BeginPayment(coverage)) => {
            let mut rebound = super::ManaCoverage::empty();
            for (&iou, payment) in coverage.iter() {
                let payment = match *payment {
                    super::ManaPayment::Floating(id) => {
                        super::ManaPayment::Floating(rebind_mana(id, records, replay_map)?)
                    }
                    super::ManaPayment::PayPips {
                        object,
                        alternative,
                    } => super::ManaPayment::PayPips {
                        object,
                        alternative,
                    },
                };
                rebound.insert(iou, payment);
            }
            Ok(Decision::Payment(PaymentCommand::BeginPayment(rebound)))
        }
        Decision::Payment(PaymentCommand::Fulfill { iou, witness }) => {
            let rebound =
                if let Some(record) = children.iter().find(|record| record.iou() == Some(iou)) {
                    rebind_witness(&witness, &record.object_inputs, state)?
                } else if matches!(witness, FulfillmentWitness::Objects(_)) {
                    return Err(ReplayError::DecisionRejected(
                        "a recorded object fulfillment has no child record".into(),
                    ));
                } else {
                    witness
                };
            Ok(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: rebound,
            }))
        }
        other => Ok(other),
    }
}

fn rebind_mana(
    original: FloatingManaId,
    records: &[TransactionRecord],
    replay_map: &ReplayMap,
) -> Result<FloatingManaId, ReplayError> {
    let Some((record, ordinal)) = records.iter().find_map(|record| {
        record
            .produced_mana
            .iter()
            .position(|&id| id == original)
            .map(|ordinal| (record.id, ordinal))
    }) else {
        return Ok(original);
    };
    replay_map
        .mana
        .get(&(record, ordinal))
        .copied()
        .ok_or_else(|| {
            ReplayError::DecisionRejected(
                "retained mana depends on a producer that was not replayed".into(),
            )
        })
}

pub(crate) fn reconstruct_decline(
    frame: &PaymentFrame,
    retained: &[PaymentRecordId],
) -> Result<GameImage, ReplayError> {
    let base = frame.proposal_base.clone();
    let mut replay_frame =
        PaymentFrame::new(base.clone(), frame.purpose.clone(), frame.locked.clone());
    replay_frame.stage = PaymentStage::PrePayment;
    replay_frame.payment_base = None;
    replay_frame.coverage = None;
    let prompt = replay_frame.prompt(Vec::new());
    replay_frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
    let mut state = GameState {
        committed: base.clone(),
        payment: Some(PaymentController {
            frames: vec![replay_frame],
            mana_actions: Vec::new(),
            next_record: frame
                .records
                .iter()
                .map(|record| record.id.0)
                .max()
                .map_or(0, |id| id.saturating_add(1)),
        }),
        incidents: Vec::new(),
        next_mana_action: 0,
    };
    let mut replay_map = ReplayMap::default();
    let retained_records = frame
        .records
        .iter()
        .filter(|record| retained.contains(&record.id))
        .collect::<Vec<_>>();
    for record in retained_records
        .iter()
        .copied()
        .filter(|record| record.iou().is_none())
    {
        replay_record(&mut state, record, &frame.records, &mut replay_map)?;
    }
    if retained_records.iter().any(|record| record.iou().is_some()) {
        // Root announcement costs can themselves cross a CR 733 barrier.
        // Provision the already-locked IOU graph over the reconstructed
        // preannouncement image, then enact only the retained fulfillments.
        // The announcement object is intentionally absent: declining will
        // discard it, while the locked frame still supplies `This`, payment
        // cause, X, targets, and other cost bindings.
        let controller = state.payment.as_mut().ok_or(ReplayError::MissingBase)?;
        let replay_frame = controller
            .frames
            .last_mut()
            .ok_or(ReplayError::MissingBase)?;
        replay_frame.payer = frame.payer;
        replay_frame.subject = frame.subject;
        replay_frame.locked = frame.locked.clone();
        replay_frame.stage = PaymentStage::Paying;
        replay_frame.coverage = frame.coverage.clone();
        replay_frame.payment_base = Some(replay_frame.working.clone());
        replay_frame.fulfilled.clear();
        replay_frame.progress = super::PaymentProgress::Idle;
        replay_frame.recording = None;
        let prompt = replay_frame.prompt(Vec::new());
        replay_frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
        for record in retained_records
            .iter()
            .copied()
            .filter(|record| record.iou().is_some())
        {
            replay_record(&mut state, record, &frame.records, &mut replay_map)?;
        }
    }
    let mut controller = state.payment.take().ok_or(ReplayError::MissingBase)?;
    let mut rebuilt = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    rebuilt.working.pending = base.pending;
    Ok(rebuilt.working)
}

fn resolve_logical_object(
    state: &GameState,
    logical: LogicalObject,
) -> Result<ObjectId, ReplayError> {
    match logical {
        LogicalObject::Player(player) => Ok(state.player(player).object),
        LogicalObject::Card(card) => state
            .objects
            .iter()
            .find(|object| object.source == crate::object::ObjectSource::Card(card))
            .map(|object| object.id)
            .ok_or_else(|| {
                ReplayError::DecisionRejected(
                    "a recorded card has no live object during replay".into(),
                )
            }),
        LogicalObject::Created { .. } => Err(ReplayError::DecisionRejected(
            "a created logical object has not been rebound".into(),
        )),
    }
}

fn rebind_witness(
    witness: &FulfillmentWitness,
    inputs: &HashMap<ObjectId, LogicalObject>,
    state: &GameState,
) -> Result<FulfillmentWitness, ReplayError> {
    let FulfillmentWitness::Objects(objects) = witness else {
        return Ok(witness.clone());
    };
    objects
        .iter()
        .map(|object| {
            let logical = inputs.get(object).copied().ok_or_else(|| {
                ReplayError::DecisionRejected(
                    "a recorded object witness has no logical binding".into(),
                )
            })?;
            resolve_logical_object(state, logical)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(FulfillmentWitness::Objects)
}

fn facts_equivalent(actual: &GameEvent, expected: &GameEvent) -> bool {
    match (actual, expected) {
        (GameEvent::Tapped(actual), GameEvent::Tapped(expected)) => {
            actual.object == expected.object && causes_equivalent(&actual.cause, &expected.cause)
        }
        (GameEvent::ZoneChange(actual), GameEvent::ZoneChange(expected)) => {
            actual.object == expected.object
                && actual.snapshot == expected.snapshot
                && actual.from == expected.from
                && actual.to == expected.to
                && actual.enters == expected.enters
                && actual.position == expected.position
                && actual.face == expected.face
                && causes_equivalent(&actual.cause, &expected.cause)
        }
        (GameEvent::ManaAdded(actual), GameEvent::ManaAdded(expected)) => {
            actual.player == expected.player
                && actual.mana == expected.mana
                && actual.amount == expected.amount
                && actual.riders == expected.riders
                && actual.provenance == expected.provenance
                && actual.units.len() == expected.units.len()
        }
        (GameEvent::ManaProduced(actual), GameEvent::ManaProduced(expected)) => {
            actual.source == expected.source
                && actual.controller == expected.controller
                && actual.action == expected.action
                && mana_units_equivalent(&actual.produced, &expected.produced)
        }
        (GameEvent::TappedForMana(actual), GameEvent::TappedForMana(expected)) => {
            actual.source == expected.source
                && actual.controller == expected.controller
                && actual.action == expected.action
                && mana_units_equivalent(&actual.produced, &expected.produced)
        }
        _ => actual == expected,
    }
}

fn causes_equivalent(
    actual: &Option<crate::event::Cause>,
    expected: &Option<crate::event::Cause>,
) -> bool {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            actual.verb == expected.verb
                && actual.agency == expected.agency
                && actual.agent == expected.agent
        }
        (None, None) => true,
        _ => false,
    }
}

fn mana_units_equivalent(
    actual: &[crate::player::ManaUnit],
    expected: &[crate::player::ManaUnit],
) -> bool {
    actual.len() == expected.len()
        && actual.iter().zip(expected).all(|(actual, expected)| {
            actual.kind == expected.kind
                && actual.riders == expected.riders
                && actual.provenance == expected.provenance
        })
}

pub(crate) fn barriers_for(facts: &[GameEvent]) -> (Vec<ReversalBarrier>, Vec<ObservationBarrier>) {
    use deckmaste_core::Zone;

    use crate::event::ZoneChange;

    let mut reversal = Vec::new();
    let mut observation = Vec::new();
    for fact in facts {
        match fact {
            GameEvent::ZoneChange(ZoneChange {
                snapshot: Some(_),
                from,
                to,
                ..
            }) => {
                if *to == Zone::Library {
                    reversal.push(ReversalBarrier::MovedToLibrary);
                }
                if *from == Some(Zone::Library) && *to != Zone::Stack {
                    reversal.push(ReversalBarrier::MovedFromLibrary);
                }
                if from.is_some_and(Zone::is_hidden) && !to.is_hidden() {
                    observation.push(ObservationBarrier::HiddenZoneDisclosure);
                }
            }
            GameEvent::Shuffled(_) => {
                reversal.push(ReversalBarrier::ShuffledLibrary);
                observation.push(ObservationBarrier::RandomOutcome);
            }
            GameEvent::Revealed(_) => {
                observation.push(ObservationBarrier::HiddenZoneDisclosure);
            }
            GameEvent::CoinFlipped(_) | GameEvent::DieRolled(_) => {
                observation.push(ObservationBarrier::RandomOutcome);
            }
            _ => {}
        }
    }
    reversal.sort_unstable_by_key(|barrier| *barrier as u8);
    reversal.dedup();
    observation.sort_unstable_by_key(|barrier| *barrier as u8);
    observation.dedup();
    (reversal, observation)
}

/// Classify information-sensitive facts while their referenced objects still
/// have the zones they occupied at observation time. A bare `Revealed` fact
/// does not encode its source zone, so CR 733's library-only reveal barrier
/// cannot be recovered later from the fact alone.
pub(crate) fn contextual_barriers_for(
    state: &GameState,
    fact: &GameEvent,
) -> (Vec<ReversalBarrier>, Vec<ObservationBarrier>) {
    let (mut reversal, observation) = barriers_for(std::slice::from_ref(fact));
    if let GameEvent::Revealed(event) = fact
        && event.objects.iter().any(|&object| {
            state
                .objects
                .get(object)
                .is_some_and(|object| object.zone == Some(deckmaste_core::Zone::Library))
        })
    {
        reversal.push(ReversalBarrier::RevealedLibraryCard);
    }
    (reversal, observation)
}
