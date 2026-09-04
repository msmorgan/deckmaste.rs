use std::cell::RefCell;
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
    pub mana: HashMap<(PaymentRecordId, usize), QualifiedManaId>,
    /// Concrete object generations paired while comparing replay facts.
    ///
    /// A transaction-created object can move again before its record
    /// completes. `objects` names the currently live generation by logical
    /// identity; this map preserves the fact-local old generations needed to
    /// compare the earlier creation and move facts.
    object_generations: RefCell<HashMap<ObjectId, ObjectId>>,
}

/// A floating-mana key qualified by the pool that owns it. `FloatingManaId`
/// is intentionally pool-local, so transaction dependencies must never
/// compare the bare integer across players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QualifiedManaId {
    pub player: PlayerId,
    pub id: FloatingManaId,
}

/// An ordinary decision sequence made while one transaction command was in
/// flight. Replay feeds these answers back through the same decision handlers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DecisionTranscript {
    pub answers: Vec<Decision>,
}

/// `ChaCha` cursor immediately after a recorded random payment selection.
///
/// Replaying the selected objects without restoring this cursor would preserve
/// the visible outcome while silently reusing the same entropy for the next
/// random event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordedRngState {
    pub stream: u64,
    pub word_pos: u128,
}

/// Stable identities and RNG continuation for one random payment binder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRandomOutcome {
    pub objects: Vec<LogicalObject>,
    pub post_sample_rng: RecordedRngState,
}

/// The deterministic operation represented by one payment-ledger record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayCommand {
    Fulfill {
        iou: IouId,
        witness: FulfillmentWitness,
        /// Stable handles sampled by a random binder, together with the RNG
        /// continuation after that sample. Replay binds these exact objects
        /// without rerolling, then resumes later randomness at the same point.
        random_outcome: Option<RecordedRandomOutcome>,
        /// False when decline interrupted the fulfillment after a
        /// nonreversible prefix but before its sentinel completed.
        completed: bool,
    },
    ManaAbility {
        action: ManaActionId,
        source: LogicalObject,
        ability: usize,
        /// False when decline abandoned the activation after a barred cost
        /// had already happened but before the stackless mana effect began.
        submitted: bool,
        /// False when decline interrupted a submitted stackless effect after a
        /// nonreversible prefix but before its remaining work/choices finished.
        completed: bool,
    },
}

/// A concrete [CR#733.1] prohibition found after replacements finished.
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
    /// Transaction records owned by this command's nested payment frame.
    /// They replay at their recorded transcript boundary. Nested mana actions
    /// remain separately selectable [CR#733.1] reversal units.
    pub children: Vec<TransactionRecord>,
    pub facts: Vec<GameEvent>,
    pub produced_mana: Vec<QualifiedManaId>,
    pub spent_mana: Vec<QualifiedManaId>,
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
    pub(crate) fn mana_action(&self) -> Option<ManaActionId> {
        match self.command {
            ReplayCommand::ManaAbility { action, .. } => Some(action),
            ReplayCommand::Fulfill { .. } => None,
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
    pub children: Vec<TransactionRecord>,
    pub facts: Vec<GameEvent>,
    pub spent_mana: Vec<QualifiedManaId>,
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
    pub logical_objects: HashMap<crate::object::ObjectSource, LogicalObject>,
    pub activations: crate::activation::ActivationTable,
    pub next_activation: u64,
}

pub(crate) struct ReconstructedDecline {
    pub working: GameImage,
    pub logical_objects: HashMap<crate::object::ObjectSource, LogicalObject>,
    pub activations: crate::activation::ActivationTable,
    pub next_activation: u64,
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
    replay_frame.activations.clone_from(&frame.activations);
    replay_frame.next_activation = frame.next_activation;
    replay_frame.observations.clone_from(&frame.observations);
    replay_frame.payment_base = Some(base.clone());
    replay_frame.stage = PaymentStage::Paying;
    replay_frame.coverage.clone_from(&frame.coverage);
    let prompt = replay_frame.prompt(Vec::new());
    replay_frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
    let payment_logical_objects = frame
        .logical_objects
        .iter()
        .filter(|(source, _)| base.objects.iter().any(|object| object.source == **source))
        .map(|(&source, &logical)| (source, logical))
        .collect();
    let mut state = GameState {
        committed: replay_frame.working.clone(),
        payment: Some(PaymentController {
            frames: vec![replay_frame],
            mana_actions: Vec::new(),
            next_record: 0,
        }),
        incidents: Vec::new(),
        next_mana_action: 0,
        payment_observations: std::collections::HashSet::new(),
        payment_logical_objects,
        activations: std::cell::RefCell::new(frame.activations.clone()),
        next_activation: std::cell::Cell::new(frame.next_activation),
    };

    let mut replay_map = ReplayMap::default();
    // `payment_base` already contains every mana action completed during
    // PrePayment. Their records remain selectable metadata, but replaying
    // those actions here would enact them twice (and commonly fail because a
    // tap/sacrifice cost is already reflected in the base). Only fulfillment
    // records advance a frame forward from its payment base.
    for record in records.iter().filter(|record| record.iou().is_some()) {
        replay_record(&mut state, record, &frame.records, &mut replay_map, &[])?;
    }

    let logical_objects = state.payment_logical_objects.clone();
    let activations = state.activations.borrow().clone();
    let next_activation = state.next_activation.get();
    let mut controller = state.payment.take().ok_or(ReplayError::MissingBase)?;
    let mut rebuilt = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    rebuilt.records.clone_from(&records);
    rebuilt.recording = None;
    if records.iter().all(|record| record.iou().is_none()) && frame.locked.has_mana_payment {
        rebuilt.stage = PaymentStage::PrePayment;
        rebuilt.coverage = None;
        rebuilt.payment_base = None;
    }
    rebuilt.working.pending = None;
    if let Some(rng) = frame.observed_rng {
        rebuilt.working.rng.set_stream(rng.stream);
        rebuilt.working.rng.set_word_pos(rng.word_pos);
    }
    let rebuilt_stage = rebuilt.stage;
    let coverage = rebuilt.coverage.clone();
    let fulfilled = rebuilt.fulfilled.clone();
    materialize_observations(&mut rebuilt.working, &frame.observations, &replay_map)?;
    let working = rebuilt.working;
    Ok(ReconstructedFrame {
        working,
        stage: rebuilt_stage,
        coverage,
        fulfilled,
        records,
        logical_objects,
        activations,
        next_activation,
    })
}

fn selected_descendant_mana_outputs(
    records: &[TransactionRecord],
    reversed: &[ManaActionId],
    facts: &mut Vec<GameEvent>,
    produced_mana: &mut Vec<QualifiedManaId>,
) {
    for record in records {
        let selected = record
            .mana_action()
            .is_some_and(|action| reversed.contains(&action));
        if selected {
            facts.extend(record.facts.iter().cloned());
            produced_mana.extend(record.produced_mana.iter().copied());
            if matches!(
                record.command,
                ReplayCommand::ManaAbility {
                    completed: true,
                    ..
                }
            ) {
                selected_descendant_mana_outputs(&record.children, reversed, facts, produced_mana);
            }
        } else {
            selected_descendant_mana_outputs(&record.children, reversed, facts, produced_mana);
        }
    }
}

fn retained_record_facts(record: &TransactionRecord, reversed: &[ManaActionId]) -> Vec<GameEvent> {
    let mut removed = Vec::new();
    selected_descendant_mana_outputs(&record.children, reversed, &mut removed, &mut Vec::new());
    let mut retained = record.facts.clone();
    for fact in removed {
        if let Some(index) = retained.iter().position(|candidate| *candidate == fact) {
            retained.remove(index);
        }
    }
    retained
}

fn retained_record_produced_mana(
    record: &TransactionRecord,
    reversed: &[ManaActionId],
) -> Vec<(usize, QualifiedManaId)> {
    let mut removed = Vec::new();
    selected_descendant_mana_outputs(&record.children, reversed, &mut Vec::new(), &mut removed);
    record
        .produced_mana
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(ordinal, mana)| {
            if let Some(index) = removed.iter().position(|candidate| *candidate == mana) {
                removed.remove(index);
                None
            } else {
                Some((ordinal, mana))
            }
        })
        .collect()
}

fn replay_record(
    state: &mut GameState,
    record: &TransactionRecord,
    all_records: &[TransactionRecord],
    replay_map: &mut ReplayMap,
    reversed: &[ManaActionId],
) -> Result<(), ReplayError> {
    state
        .payment
        .as_mut()
        .ok_or(ReplayError::MissingBase)?
        .next_record = record.id.0;
    let history_start = state.history.len();
    let incomplete_fulfillment_facts = match &record.command {
        ReplayCommand::Fulfill {
            iou,
            witness,
            random_outcome,
            completed,
        } => {
            let parent_depth = state.payment_depth();
            if let Some(random_outcome) = random_outcome {
                let rebound_random = random_outcome
                    .objects
                    .iter()
                    .map(|&logical| resolve_logical_object(state, replay_map, logical))
                    .collect::<Result<Vec<_>, _>>()?;
                state
                    .payment
                    .as_mut()
                    .and_then(|controller| controller.frames.last_mut())
                    .ok_or(ReplayError::MissingBase)?
                    .replay_random_outcome = Some((rebound_random, random_outcome.post_sample_rng));
            }
            let witness = rebind_witness(witness, &record.object_inputs, replay_map, state)?;
            state
                .submit_payment_command(PaymentCommand::Fulfill { iou: *iou, witness })
                .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
            replay_fulfillment_answers(
                state,
                record,
                all_records,
                replay_map,
                reversed,
                *completed,
                parent_depth,
            )?
        }
        ReplayCommand::ManaAbility {
            action,
            source,
            ability,
            submitted,
            completed,
        } => {
            let source = resolve_logical_object(state, replay_map, *source)?;
            let parent_depth = state.payment_depth();
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
                &record.object_inputs,
                replay_map,
                reversed,
                *submitted,
                *completed,
                parent_depth,
            )?;
            None
        }
    };
    let replayed_history = state.history.facts_from(history_start);
    refresh_created_object_map(state, replay_map);
    let replayed = match record.command {
        ReplayCommand::Fulfill {
            completed: true, ..
        }
        | ReplayCommand::ManaAbility {
            completed: true, ..
        } => recorded_replay_facts(state, record.id).ok_or(ReplayError::MissingChildRecord)?,
        ReplayCommand::Fulfill {
            completed: false, ..
        } => incomplete_fulfillment_facts.ok_or(ReplayError::MissingChildRecord)?,
        _ => replayed_history,
    };
    let expected_facts = retained_record_facts(record, reversed);
    if replayed.len() != expected_facts.len()
        || replayed
            .iter()
            .zip(&expected_facts)
            .any(|(actual, expected)| {
                !facts_equivalent_mapped(actual, expected, &record.object_inputs, replay_map)
            })
    {
        return Err(ReplayError::FactMismatch);
    }
    let produced: Vec<QualifiedManaId> = replayed
        .iter()
        .filter_map(|fact| match fact {
            GameEvent::ManaAdded(event) => {
                Some(event.units.iter().copied().map(|id| QualifiedManaId {
                    player: event.player,
                    id,
                }))
            }
            _ => None,
        })
        .flatten()
        .collect();
    let expected_produced = retained_record_produced_mana(record, reversed);
    if produced.len() != expected_produced.len() {
        return Err(ReplayError::FactMismatch);
    }
    for (mana, (ordinal, _)) in produced.into_iter().zip(expected_produced) {
        replay_map.mana.insert((record.id, ordinal), mana);
    }
    Ok(())
}

fn replay_fulfillment_answers(
    state: &mut GameState,
    record: &TransactionRecord,
    all_records: &[TransactionRecord],
    replay_map: &mut ReplayMap,
    reversed: &[ManaActionId],
    completed: bool,
    parent_depth: usize,
) -> Result<Option<Vec<GameEvent>>, ReplayError> {
    let mut answers = record.transcript.answers.iter().cloned();
    let mut child_cursor = 0;
    let mut incomplete_facts = None;
    loop {
        if let Some(pending) = state.pending.clone() {
            if matches!(pending, crate::decide::DecisionPointKind::Payment(_))
                && state.payment_depth() == parent_depth
            {
                break;
            }
            if let Some(answer) = answers.next() {
                refresh_created_object_map(state, replay_map);
                let rebound_children =
                    next_replay_child(&answer, &record.children, &mut child_cursor);
                if matches!(
                    answer,
                    Decision::Payment(PaymentCommand::ActivateManaAbility { .. })
                ) {
                    let child = rebound_children
                        .first()
                        .ok_or(ReplayError::MissingChildRecord)?;
                    if child
                        .mana_action()
                        .is_some_and(|action| reversed.contains(&action))
                    {
                        continue;
                    }
                    replay_record(state, child, all_records, replay_map, reversed)?;
                    continue;
                }
                let answer = rebind_decision(
                    answer,
                    rebound_children,
                    all_records,
                    &record.object_inputs,
                    replay_map,
                    state,
                )?;
                if let Decision::Payment(PaymentCommand::Fulfill { .. }) = &answer
                    && let Some(child) = rebound_children.first()
                {
                    state
                        .payment
                        .as_mut()
                        .ok_or(ReplayError::MissingBase)?
                        .next_record = child.id.0;
                }
                state
                    .submit_decision(answer)
                    .map_err(|error| ReplayError::DecisionRejected(error.to_string()))?;
            } else if completed {
                return Err(ReplayError::MissingDecision);
            } else {
                let frame = state
                    .payment
                    .as_mut()
                    .and_then(|controller| controller.frames.last_mut())
                    .ok_or(ReplayError::MissingBase)?;
                incomplete_facts = Some(
                    frame
                        .recording
                        .as_ref()
                        .filter(|draft| draft.id == record.id)
                        .ok_or(ReplayError::MissingChildRecord)?
                        .facts
                        .clone(),
                );
                frame.abandon_fulfillment();
                break;
            }
            continue;
        }
        match state.step() {
            StepOutcome::NeedsDecision(crate::decide::DecisionPointKind::Payment(_))
                if state.payment_depth() == parent_depth =>
            {
                break;
            }
            StepOutcome::Progress(_) | StepOutcome::NeedsDecision(_) => {}
            StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
        }
    }
    if answers.next().is_some() {
        return Err(ReplayError::DecisionRejected(
            "the recorded transcript has unused answers".into(),
        ));
    }
    Ok(incomplete_facts)
}

#[expect(
    clippy::too_many_arguments,
    reason = "replay validates one mana action against its full recorded boundary"
)]
fn replay_mana_action_answers(
    state: &mut GameState,
    action: ManaActionId,
    transcript: &DecisionTranscript,
    children: &[TransactionRecord],
    all_records: &[TransactionRecord],
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &mut ReplayMap,
    reversed: &[ManaActionId],
    submitted: bool,
    completed: bool,
    parent_depth: usize,
) -> Result<(), ReplayError> {
    let mut child_cursor = 0;
    for answer in transcript.answers.iter().cloned() {
        while state.pending.is_none() {
            match state.step() {
                StepOutcome::Progress(_) | StepOutcome::NeedsDecision(_) => {}
                StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
            }
        }
        refresh_created_object_map(state, replay_map);
        let rebound_children = next_replay_child(&answer, children, &mut child_cursor);
        if matches!(
            answer,
            Decision::Payment(PaymentCommand::ActivateManaAbility { .. })
        ) {
            let child = rebound_children
                .first()
                .ok_or(ReplayError::MissingChildRecord)?;
            if child
                .mana_action()
                .is_some_and(|child_action| reversed.contains(&child_action))
            {
                continue;
            }
            replay_record(state, child, all_records, replay_map, reversed)?;
            continue;
        }
        if matches!(answer, Decision::Payment(PaymentCommand::Fulfill { .. }))
            && let Some(child) = rebound_children.first()
        {
            replay_record(state, child, all_records, replay_map, reversed)?;
            continue;
        }
        let decision = rebind_decision(
            answer,
            rebound_children,
            all_records,
            inputs,
            replay_map,
            state,
        )?;
        if let Decision::Payment(PaymentCommand::Fulfill { .. }) = &decision
            && let Some(record) = rebound_children.first()
        {
            state
                .payment
                .as_mut()
                .ok_or(ReplayError::MissingBase)?
                .next_record = record.id.0;
        }
        state
            .submit_decision(decision)
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
            Some(crate::decide::DecisionPointKind::Payment(_))
        ) {
            return Err(ReplayError::MissingDecision);
        }
        return cancel_replayed_mana_proposal(state, action, children);
    }
    if !completed {
        while state.pending.is_none() {
            match state.step() {
                StepOutcome::Progress(_) | StepOutcome::NeedsDecision(_) => {}
                StepOutcome::GameOver(_) => return Err(ReplayError::GameEnded),
            }
        }
        return cancel_replayed_mana_resolution(state, action);
    }
    loop {
        let finished = state.payment.as_ref().is_some_and(|controller| {
            controller.frames.len() == parent_depth
                && controller
                    .mana_actions
                    .iter()
                    .all(|active| active.id != action)
        });
        if finished
            && matches!(
                state.pending,
                Some(crate::decide::DecisionPointKind::Payment(_))
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

fn next_replay_child<'a>(
    decision: &Decision,
    children: &'a [TransactionRecord],
    cursor: &mut usize,
) -> &'a [TransactionRecord] {
    let relative = match decision {
        Decision::Payment(PaymentCommand::Fulfill { iou, .. }) => children[*cursor..]
            .iter()
            .position(|record| record.iou() == Some(*iou)),
        Decision::Payment(PaymentCommand::ActivateManaAbility { ability, .. }) => {
            children[*cursor..].iter().position(|record| {
                matches!(
                    record.command,
                    ReplayCommand::ManaAbility {
                        ability: recorded,
                        ..
                    } if recorded == *ability
                )
            })
        }
        _ => return children,
    };
    let Some(relative) = relative else {
        return &[];
    };
    let index = *cursor + relative;
    *cursor = index + 1;
    std::slice::from_ref(&children[index])
}

/// Remove reversed nested mana actions from retained replay metadata together
/// with the activation answers that marked their chronological boundaries.
/// Trial replay still uses the original tree; this is only for a retained
/// parent record that survives a nested announcement decline.
pub(crate) fn prune_reversed_mana_children(
    record: &mut TransactionRecord,
    reversed: &[ManaActionId],
) -> Result<(), ReplayError> {
    let mut child_cursor = 0;
    let mut remove_children = vec![false; record.children.len()];
    let mut remove_answers = vec![false; record.transcript.answers.len()];
    for (answer_index, answer) in record.transcript.answers.iter().enumerate() {
        let Decision::Payment(PaymentCommand::ActivateManaAbility { ability, .. }) = answer else {
            continue;
        };
        let Some(relative) = record.children[child_cursor..].iter().position(|child| {
            matches!(
                child.command,
                ReplayCommand::ManaAbility {
                    ability: recorded,
                    ..
                } if recorded == *ability
            )
        }) else {
            continue;
        };
        let child_index = child_cursor + relative;
        child_cursor = child_index + 1;
        if record.children[child_index]
            .mana_action()
            .is_some_and(|action| reversed.contains(&action))
        {
            remove_children[child_index] = true;
            remove_answers[answer_index] = true;
        }
    }
    if record.children.iter().enumerate().any(|(index, child)| {
        child
            .mana_action()
            .is_some_and(|action| reversed.contains(&action))
            && !remove_children[index]
    }) {
        return Err(ReplayError::MissingChildRecord);
    }

    let removed_spent = record
        .children
        .iter()
        .enumerate()
        .filter(|(index, _)| remove_children[*index])
        .flat_map(|(_, child)| child.spent_mana.iter().copied())
        .collect::<Vec<_>>();
    for (index, child) in record.children.iter_mut().enumerate() {
        if !remove_children[index] {
            prune_reversed_mana_children(child, reversed)?;
        }
    }
    record.transcript.answers = std::mem::take(&mut record.transcript.answers)
        .into_iter()
        .enumerate()
        .filter_map(|(index, answer)| (!remove_answers[index]).then_some(answer))
        .collect();
    record.children = std::mem::take(&mut record.children)
        .into_iter()
        .enumerate()
        .filter_map(|(index, child)| (!remove_children[index]).then_some(child))
        .collect();
    record
        .spent_mana
        .retain(|mana| !removed_spent.contains(mana));
    Ok(())
}

fn refresh_created_object_map(state: &GameState, replay_map: &mut ReplayMap) {
    for (&source, &logical) in &state.payment_logical_objects {
        if matches!(logical, LogicalObject::Created { .. })
            && let Some(object) = state.objects.iter().find(|object| object.source == source)
        {
            replay_map.objects.insert(logical, object.id);
        }
    }
}

fn recorded_replay_facts(state: &GameState, id: PaymentRecordId) -> Option<Vec<GameEvent>> {
    fn find(records: &[TransactionRecord], id: PaymentRecordId) -> Option<&TransactionRecord> {
        records.iter().find_map(|record| {
            (record.id == id)
                .then_some(record)
                .or_else(|| find(&record.children, id))
        })
    }

    state.payment.as_ref().and_then(|controller| {
        controller.frames.iter().find_map(|frame| {
            find(&frame.records, id)
                .or_else(|| {
                    frame
                        .recording
                        .as_ref()?
                        .children
                        .iter()
                        .find(|r| r.id == id)
                })
                .map(|record| record.facts.clone())
        })
    })
}

fn cancel_replayed_mana_resolution(
    state: &mut GameState,
    action: ManaActionId,
) -> Result<(), ReplayError> {
    let controller = state.payment.as_mut().ok_or(ReplayError::MissingBase)?;
    let mut child = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    child.working.agenda.clone_from(&child.proposal_base.agenda);
    child.working.pending = None;
    child.working.choice = None;
    child.working.placing_trigger = None;
    child.working.replace_state = None;
    if child.working.resolving_mana_actions.last() != Some(&action) {
        return Err(ReplayError::MissingActiveManaAction);
    }
    child.working.finish_mana_resolution_scope();
    child.working.resolving_mana_actions.pop();
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
    let mut replay_child = child.clone();
    replay_child
        .activations
        .clone_from(&state.activations.borrow());
    replay_child.next_activation = state.next_activation.get();
    let rebuilt = reconstruct_frame(&replay_child, &retained)?;
    state
        .payment_logical_objects
        .clone_from(&rebuilt.logical_objects);
    state.activations.replace(rebuilt.activations);
    state.next_activation.set(rebuilt.next_activation);
    let controller = state.payment.as_mut().ok_or(ReplayError::MissingBase)?;
    let mut child = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    child.working = rebuilt.working;
    child.working.pending = None;
    if let Some(announcing) = child.working.announcing.take()
        && child.working.objects.get(announcing.id).is_some()
    {
        child.working.objects.remove(announcing.id);
    }
    child.working.finish_mana_resolution_scope();
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
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<Decision, ReplayError> {
    match decision {
        Decision::Payment(PaymentCommand::BeginPayment(coverage)) => {
            let mut rebound = super::ManaCoverage::empty();
            for (&iou, payment) in coverage.iter() {
                let payment = match *payment {
                    super::ManaPayment::Floating(id) => {
                        let payer = state
                            .payment
                            .as_ref()
                            .and_then(|controller| controller.frames.last())
                            .ok_or(ReplayError::MissingBase)?
                            .payer;
                        super::ManaPayment::Floating(rebind_mana(
                            QualifiedManaId { player: payer, id },
                            records,
                            replay_map,
                        )?)
                    }
                    super::ManaPayment::PayPips {
                        object,
                        alternative,
                    } => super::ManaPayment::PayPips {
                        object: rebind_object(object, inputs, replay_map, state)?,
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
                    rebind_witness(&witness, &record.object_inputs, replay_map, state)?
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
        Decision::Payment(PaymentCommand::ActivateManaAbility { source, ability }) => {
            Ok(Decision::Payment(PaymentCommand::ActivateManaAbility {
                source: rebind_object(source, inputs, replay_map, state)?,
                ability,
            }))
        }
        Decision::Divide(assignments) => Ok(Decision::Divide(rebind_weighted_objects(
            assignments,
            inputs,
            replay_map,
            state,
        )?)),
        Decision::Discard(objects) => Ok(Decision::Discard(rebind_objects(
            objects, inputs, replay_map, state,
        )?)),
        Decision::Targets(slots) => Ok(Decision::Targets(
            slots
                .into_iter()
                .map(|objects| rebind_objects(objects, inputs, replay_map, state))
                .collect::<Result<_, _>>()?,
        )),
        Decision::Attackers(pairs) => Ok(Decision::Attackers(rebind_object_pairs(
            pairs, inputs, replay_map, state,
        )?)),
        Decision::Blocks(pairs) => Ok(Decision::Blocks(rebind_object_pairs(
            pairs, inputs, replay_map, state,
        )?)),
        Decision::Assignment(assignments) => Ok(Decision::Assignment(rebind_weighted_objects(
            assignments,
            inputs,
            replay_map,
            state,
        )?)),
        Decision::Chosen(objects) => Ok(Decision::Chosen(rebind_objects(
            objects, inputs, replay_map, state,
        )?)),
        Decision::Arranged(objects) => Ok(Decision::Arranged(rebind_objects(
            objects, inputs, replay_map, state,
        )?)),
        Decision::Act(action) => Ok(Decision::Act(rebind_priority_action(
            action, inputs, replay_map, state,
        )?)),
        Decision::ReplacementChoice(key) => Ok(Decision::ReplacementChoice(match key {
            crate::replace_registry::ReplacementKey::Static {
                source,
                ability,
                effect,
            } => crate::replace_registry::ReplacementKey::Static {
                source: rebind_object(source, inputs, replay_map, state)?,
                ability,
                effect,
            },
            crate::replace_registry::ReplacementKey::Floating(instance) => {
                crate::replace_registry::ReplacementKey::Floating(instance)
            }
        })),
        other => Ok(other),
    }
}

fn rebind_object(
    original: ObjectId,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<ObjectId, ReplayError> {
    inputs.get(&original).map_or(Ok(original), |&logical| {
        resolve_logical_object(state, replay_map, logical)
    })
}

fn rebind_objects(
    objects: Vec<ObjectId>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<Vec<ObjectId>, ReplayError> {
    objects
        .into_iter()
        .map(|object| rebind_object(object, inputs, replay_map, state))
        .collect()
}

fn rebind_object_pairs(
    pairs: Vec<(ObjectId, ObjectId)>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<Vec<(ObjectId, ObjectId)>, ReplayError> {
    pairs
        .into_iter()
        .map(|(first, second)| {
            Ok((
                rebind_object(first, inputs, replay_map, state)?,
                rebind_object(second, inputs, replay_map, state)?,
            ))
        })
        .collect()
}

fn rebind_weighted_objects(
    assignments: Vec<(ObjectId, deckmaste_core::Uint)>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<Vec<(ObjectId, deckmaste_core::Uint)>, ReplayError> {
    assignments
        .into_iter()
        .map(|(object, amount)| Ok((rebind_object(object, inputs, replay_map, state)?, amount)))
        .collect()
}

fn rebind_priority_action(
    action: crate::decide::Action,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    state: &GameState,
) -> Result<crate::decide::Action, ReplayError> {
    use crate::decide::Action;

    match action {
        Action::PlayLand { object } => Ok(Action::PlayLand {
            object: rebind_object(object, inputs, replay_map, state)?,
        }),
        Action::ActivateAbility { object, ability } => Ok(Action::ActivateAbility {
            object: rebind_object(object, inputs, replay_map, state)?,
            ability,
        }),
        Action::CastSpell { object } => Ok(Action::CastSpell {
            object: rebind_object(object, inputs, replay_map, state)?,
        }),
        Action::Special(special) => Ok(Action::Special(match special {
            crate::decide::SpecialAction::TurnFaceUp(object) => {
                crate::decide::SpecialAction::TurnFaceUp(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
            crate::decide::SpecialAction::EndEffect(object) => {
                crate::decide::SpecialAction::EndEffect(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
            crate::decide::SpecialAction::IgnoreStatic(object) => {
                crate::decide::SpecialAction::IgnoreStatic(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
            crate::decide::SpecialAction::SuspendCast(object) => {
                crate::decide::SpecialAction::SuspendCast(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
            crate::decide::SpecialAction::Foretell(object) => {
                crate::decide::SpecialAction::Foretell(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
            crate::decide::SpecialAction::PlotExile(object) => {
                crate::decide::SpecialAction::PlotExile(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
            crate::decide::SpecialAction::CompanionToHand => {
                crate::decide::SpecialAction::CompanionToHand
            }
            crate::decide::SpecialAction::UnlockHalf(object) => {
                crate::decide::SpecialAction::UnlockHalf(rebind_object(
                    object, inputs, replay_map, state,
                )?)
            }
        })),
        Action::Pass | Action::Concede => Ok(action),
    }
}

fn rebind_mana(
    original: QualifiedManaId,
    records: &[TransactionRecord],
    replay_map: &ReplayMap,
) -> Result<FloatingManaId, ReplayError> {
    fn producer(
        records: &[TransactionRecord],
        original: QualifiedManaId,
    ) -> Option<(PaymentRecordId, usize)> {
        records.iter().find_map(|record| {
            producer(&record.children, original).or_else(|| {
                record
                    .produced_mana
                    .iter()
                    .position(|&mana| mana == original)
                    .map(|ordinal| (record.id, ordinal))
            })
        })
    }

    let Some((record, ordinal)) = producer(records, original) else {
        return Ok(original.id);
    };
    replay_map
        .mana
        .get(&(record, ordinal))
        .copied()
        .and_then(|mana| (mana.player == original.player).then_some(mana.id))
        .ok_or_else(|| {
            ReplayError::DecisionRejected(
                "retained mana depends on a producer that was not replayed".into(),
            )
        })
}

fn replay_retained_descendant_mana_actions(
    state: &mut GameState,
    records: &[TransactionRecord],
    all_records: &[TransactionRecord],
    replay_map: &mut ReplayMap,
    reversed: &[ManaActionId],
) -> Result<(), ReplayError> {
    for record in records {
        if let Some(action) = record.mana_action() {
            if !reversed.contains(&action) {
                replay_record(state, record, all_records, replay_map, reversed)?;
            }
        } else {
            advance_past_omitted_random_binder(state, record);
            replay_retained_descendant_mana_actions(
                state,
                &record.children,
                all_records,
                replay_map,
                reversed,
            )?;
        }
    }
    Ok(())
}

fn advance_past_omitted_random_binder(state: &mut GameState, record: &TransactionRecord) {
    let ReplayCommand::Fulfill {
        random_outcome: Some(random_outcome),
        ..
    } = &record.command
    else {
        return;
    };
    state.rng.set_stream(random_outcome.post_sample_rng.stream);
    state
        .rng
        .set_word_pos(random_outcome.post_sample_rng.word_pos);
}

fn enter_decline_fulfillment_replay(
    state: &mut GameState,
    frame: &PaymentFrame,
) -> Result<(), ReplayError> {
    // Provision the already-locked IOU graph over the reconstructed
    // preannouncement image. The announcement object is intentionally absent:
    // declining will discard it, while the locked frame still supplies `This`,
    // payment cause, X, targets, and other cost bindings.
    let controller = state.payment.as_mut().ok_or(ReplayError::MissingBase)?;
    let replay_frame = controller
        .frames
        .last_mut()
        .ok_or(ReplayError::MissingBase)?;
    replay_frame.payer = frame.payer;
    replay_frame.subject = frame.subject;
    replay_frame.locked = frame.locked.clone();
    replay_frame.stage = PaymentStage::Paying;
    replay_frame.coverage.clone_from(&frame.coverage);
    replay_frame.payment_base = Some(replay_frame.working.clone());
    replay_frame.fulfilled.clear();
    replay_frame.progress = super::PaymentProgress::Idle;
    replay_frame.recording = None;
    let prompt = replay_frame.prompt(Vec::new());
    replay_frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
    Ok(())
}

fn replay_omitted_fulfillment_mana_actions(
    state: &mut GameState,
    record: &TransactionRecord,
    all_records: &[TransactionRecord],
    replay_map: &mut ReplayMap,
    reversed: &[ManaActionId],
) -> Result<(), ReplayError> {
    // The omitted fulfillment's Optional payment frame is absent, but its
    // separately retained mana children still need the same PrePayment window
    // at this exact root-record ordinal. This temporary frame supplies that
    // activation boundary without replaying the omitted fulfillment itself.
    let (working, payer, observations, logical_objects) = {
        let parent = state
            .payment
            .as_ref()
            .and_then(|controller| controller.frames.last())
            .ok_or(ReplayError::MissingBase)?;
        (
            parent.working.clone(),
            parent.payer,
            parent.observations.clone(),
            parent.logical_objects.clone(),
        )
    };
    let mut window = PaymentFrame::proposal(working, payer);
    window.activations.clone_from(&state.activations.borrow());
    window.next_activation = state.next_activation.get();
    window.stage = PaymentStage::PrePayment;
    window.payment_base = None;
    window.observations = observations;
    window.logical_objects = logical_objects;
    let prompt = window.prompt(Vec::new());
    window.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
    state
        .payment
        .as_mut()
        .ok_or(ReplayError::MissingBase)?
        .frames
        .push(window);
    state.refresh_payment_prompt();

    replay_retained_descendant_mana_actions(
        state,
        &record.children,
        all_records,
        replay_map,
        reversed,
    )?;

    let controller = state.payment.as_mut().ok_or(ReplayError::MissingBase)?;
    let mut window = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    if window.mana_action.is_some() {
        return Err(ReplayError::MissingActiveManaAction);
    }
    window.working.pending = None;
    let parent = controller
        .frames
        .last_mut()
        .ok_or(ReplayError::MissingBase)?;
    parent.working = window.working;
    parent.observations.extend(window.observations);
    parent.logical_objects.extend(window.logical_objects);
    let prompt = parent.prompt(Vec::new());
    parent.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
    Ok(())
}

fn collect_record_ids(
    records: &[TransactionRecord],
    ids: &mut std::collections::HashSet<PaymentRecordId>,
) {
    for record in records {
        ids.insert(record.id);
        collect_record_ids(&record.children, ids);
    }
}

pub(crate) fn reconstruct_decline(
    frame: &PaymentFrame,
    retained: &[PaymentRecordId],
    reversed: &[ManaActionId],
) -> Result<ReconstructedDecline, ReplayError> {
    let base = frame.proposal_base.clone();
    let mut replay_frame =
        PaymentFrame::new(base.clone(), frame.purpose.clone(), frame.locked.clone());
    replay_frame.activations.clone_from(&frame.activations);
    replay_frame.next_activation = frame.next_activation;
    replay_frame.observations.clone_from(&frame.observations);
    replay_frame.stage = PaymentStage::PrePayment;
    replay_frame.payment_base = None;
    replay_frame.coverage = None;
    let prompt = replay_frame.prompt(Vec::new());
    replay_frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
    let payment_logical_objects = frame
        .logical_objects
        .iter()
        .filter(|(source, _)| base.objects.iter().any(|object| object.source == **source))
        .map(|(&source, &logical)| (source, logical))
        .collect();
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
        payment_observations: std::collections::HashSet::new(),
        payment_logical_objects,
        activations: std::cell::RefCell::new(frame.activations.clone()),
        next_activation: std::cell::Cell::new(frame.next_activation),
    };
    let mut replay_map = ReplayMap::default();
    // Root payment mana actions all precede the first fulfillment: activated
    // mana abilities are legal only in a PrePayment frame. A synthetic partial
    // mana record is appended after its cost records, so replay the retained
    // mana roots first and remember which fulfillment records they replayed at
    // their own transcript boundaries.
    let mut replayed_inside_mana = std::collections::HashSet::new();
    for record in frame.records.iter().filter(|record| record.iou().is_none()) {
        if retained.contains(&record.id) {
            collect_record_ids(&record.children, &mut replayed_inside_mana);
            replay_record(
                &mut state,
                record,
                &frame.records,
                &mut replay_map,
                reversed,
            )?;
        } else {
            replay_retained_descendant_mana_actions(
                &mut state,
                &record.children,
                &frame.records,
                &mut replay_map,
                reversed,
            )?;
        }
    }

    let mut replaying_fulfillments = false;
    for record in frame.records.iter().filter(|record| record.iou().is_some()) {
        if replayed_inside_mana.contains(&record.id) {
            continue;
        }
        if !replaying_fulfillments {
            enter_decline_fulfillment_replay(&mut state, frame)?;
            replaying_fulfillments = true;
        }
        if retained.contains(&record.id) {
            replay_record(
                &mut state,
                record,
                &frame.records,
                &mut replay_map,
                reversed,
            )?;
        } else {
            advance_past_omitted_random_binder(&mut state, record);
            replay_omitted_fulfillment_mana_actions(
                &mut state,
                record,
                &frame.records,
                &mut replay_map,
                reversed,
            )?;
        }
    }
    let logical_objects = state.payment_logical_objects.clone();
    let activations = state.activations.borrow().clone();
    let next_activation = state.next_activation.get();
    let mut controller = state.payment.take().ok_or(ReplayError::MissingBase)?;
    let mut rebuilt = controller.frames.pop().ok_or(ReplayError::MissingBase)?;
    rebuilt.working.pending = base.pending;
    if let Some(rng) = frame.observed_rng {
        rebuilt.working.rng.set_stream(rng.stream);
        rebuilt.working.rng.set_word_pos(rng.word_pos);
    }
    materialize_observations(&mut rebuilt.working, &frame.observations, &replay_map)?;
    Ok(ReconstructedDecline {
        working: rebuilt.working,
        logical_objects,
        activations,
        next_activation,
    })
}

fn materialize_observations(
    image: &mut GameImage,
    observations: &std::collections::HashSet<(PlayerId, LogicalObject)>,
    replay_map: &ReplayMap,
) -> Result<(), ReplayError> {
    for &(player, logical) in observations {
        let object = match logical {
            LogicalObject::Player(owner) => image.players[owner.index()].object,
            LogicalObject::Card(card) => image
                .objects
                .iter()
                .find(|object| object.source == crate::object::ObjectSource::Card(card))
                .map(|object| object.id)
                .ok_or_else(|| {
                    ReplayError::DecisionRejected(
                        "an observed card has no live object after reconstruction".into(),
                    )
                })?,
            LogicalObject::Created { .. } => {
                let Some(object) = replay_map.objects.get(&logical).copied() else {
                    // Decline may reverse the record that created an observed
                    // token. Knowledge remains monotone, but no physical
                    // object survives to receive a view grant. A retained
                    // command that actually depends on this handle fails in
                    // `resolve_logical_object` instead of inventing an object.
                    continue;
                };
                object
            }
        };
        image.look_grants.insert((player, object));
    }
    Ok(())
}

fn resolve_logical_object(
    state: &GameState,
    replay_map: &ReplayMap,
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
        LogicalObject::Created { .. } => {
            replay_map.objects.get(&logical).copied().ok_or_else(|| {
                ReplayError::DecisionRejected(
                    "a retained command depends on an omitted created object".into(),
                )
            })
        }
    }
}

fn rebind_witness(
    witness: &FulfillmentWitness,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
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
            resolve_logical_object(state, replay_map, logical)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(FulfillmentWitness::Objects)
}

#[expect(
    clippy::too_many_lines,
    reason = "the closed event ledger comparison keeps every object-bearing fact shape explicit"
)]
fn facts_equivalent_mapped(
    actual: &GameEvent,
    expected: &GameEvent,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    match (actual, expected) {
        (
            GameEvent::Untapped(actual, actual_cause),
            GameEvent::Untapped(expected, expected_cause),
        ) => {
            objects_equivalent(*actual, *expected, inputs, replay_map)
                && causes_equivalent_mapped(
                    actual_cause.as_ref(),
                    expected_cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::Transformed(actual), GameEvent::Transformed(expected))
        | (GameEvent::TokenCeased(actual), GameEvent::TokenCeased(expected))
        | (GameEvent::SpellCast(actual), GameEvent::SpellCast(expected))
        | (GameEvent::AbilityResolved(actual), GameEvent::AbilityResolved(expected)) => {
            objects_equivalent(*actual, *expected, inputs, replay_map)
        }
        (GameEvent::Act(actual), GameEvent::Act(expected)) => {
            actual.verb == expected.verb
                && actual.who == expected.who
                && object_slices_equivalent(&actual.on, &expected.on, inputs, replay_map)
                && actual.from == expected.from
                && actual.to == expected.to
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
                && actual.committed == expected.committed
                && actual.contents == expected.contents
                && actual.batch == expected.batch
                && actual.inherited == expected.inherited
                && actual.contained == expected.contained
        }
        (GameEvent::Tapped(actual), GameEvent::Tapped(expected)) => {
            objects_equivalent(actual.object, expected.object, inputs, replay_map)
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::ManaAdded(actual), GameEvent::ManaAdded(expected)) => {
            actual.player == expected.player
                && actual.mana == expected.mana
                && actual.amount == expected.amount
                && actual.riders == expected.riders
                && mana_provenance_equivalent(
                    actual.provenance,
                    expected.provenance,
                    inputs,
                    replay_map,
                )
                && actual.units.len() == expected.units.len()
        }
        (GameEvent::Copied(actual), GameEvent::Copied(expected)) => {
            objects_equivalent(actual.original, expected.original, inputs, replay_map)
                && optional_objects_equivalent(actual.copy, expected.copy, inputs, replay_map)
                && actual.controller == expected.controller
        }
        (GameEvent::AbilityActivated(actual), GameEvent::AbilityActivated(expected)) => {
            objects_equivalent(actual.source, expected.source, inputs, replay_map)
                && actual.ability == expected.ability
        }
        (GameEvent::ManaAbilityActivated(actual), GameEvent::ManaAbilityActivated(expected)) => {
            snapshots_equivalent(
                Some(&actual.source),
                Some(&expected.source),
                inputs,
                replay_map,
                false,
            ) && actual.ability == expected.ability
                && actual.controller == expected.controller
                && actual.action == expected.action
        }
        (GameEvent::DamageDealt(actual), GameEvent::DamageDealt(expected)) => {
            objects_equivalent(actual.source, expected.source, inputs, replay_map)
                && objects_equivalent(actual.target, expected.target, inputs, replay_map)
                && actual.amount == expected.amount
                && actual.combat == expected.combat
        }
        (GameEvent::ZoneChange(actual), GameEvent::ZoneChange(expected)) => {
            let created_generation = inputs
                .get(&expected.object)
                .is_some_and(|logical| matches!(logical, LogicalObject::Created { .. }));
            let created_entry = expected.from.is_none() && created_generation;
            ((created_generation
                && bind_created_generation(actual.object, expected.object, replay_map))
                || objects_equivalent(actual.object, expected.object, inputs, replay_map))
                && snapshots_equivalent(
                    actual.snapshot.as_deref(),
                    expected.snapshot.as_deref(),
                    inputs,
                    replay_map,
                    created_entry,
                )
                && actual.from == expected.from
                && actual.to == expected.to
                && enter_statuses_equivalent(
                    actual.enters.as_ref(),
                    expected.enters.as_ref(),
                    inputs,
                    replay_map,
                )
                && actual.position == expected.position
                && actual.face == expected.face
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::TokenCreated(actual), GameEvent::TokenCreated(expected)) => {
            actual.player == expected.player
                && actual.token == expected.token
                && enter_statuses_equivalent(
                    actual.enters.as_ref(),
                    expected.enters.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::LifeLost(actual), GameEvent::LifeLost(expected)) => {
            actual.player == expected.player
                && actual.amount == expected.amount
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::LifeGained(actual), GameEvent::LifeGained(expected)) => {
            actual.player == expected.player
                && actual.amount == expected.amount
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::Attacking(actual), GameEvent::Attacking(expected)) => {
            objects_equivalent(actual.attacker, expected.attacker, inputs, replay_map)
                && objects_equivalent(actual.defending, expected.defending, inputs, replay_map)
        }
        (GameEvent::Blocked(actual), GameEvent::Blocked(expected)) => {
            objects_equivalent(actual.blocker, expected.blocker, inputs, replay_map)
                && objects_equivalent(actual.attacker, expected.attacker, inputs, replay_map)
        }
        (GameEvent::CounterPlaced(actual), GameEvent::CounterPlaced(expected)) => {
            objects_equivalent(actual.object, expected.object, inputs, replay_map)
                && actual.kind == expected.kind
                && actual.amount == expected.amount
                && actual.before == expected.before
                && actual.after == expected.after
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::CounterRemoved(actual), GameEvent::CounterRemoved(expected)) => {
            objects_equivalent(actual.object, expected.object, inputs, replay_map)
                && actual.kind == expected.kind
                && actual.amount == expected.amount
                && causes_equivalent_mapped(
                    actual.cause.as_ref(),
                    expected.cause.as_ref(),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::TriggerFired(actual), GameEvent::TriggerFired(expected)) => {
            actual.ability == expected.ability
                && actual.controller == expected.controller
                && actual.created == expected.created
                && trigger_bindings_equivalent(
                    &actual.bindings,
                    &expected.bindings,
                    inputs,
                    replay_map,
                )
                && (actual.source == expected.source
                    || snapshots_equivalent(
                        actual.bindings.this.as_ref(),
                        expected.bindings.this.as_ref(),
                        inputs,
                        replay_map,
                        false,
                    ))
        }
        (GameEvent::AbilityUsed(actual), GameEvent::AbilityUsed(expected)) => {
            objects_equivalent(actual.object, expected.object, inputs, replay_map)
                && actual.ability == expected.ability
        }
        (GameEvent::AbilityCountered(actual), GameEvent::AbilityCountered(expected)) => {
            objects_equivalent(actual.id, expected.id, inputs, replay_map)
                && causes_equivalent_mapped(
                    Some(&actual.cause),
                    Some(&expected.cause),
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::Revealed(actual), GameEvent::Revealed(expected)) => {
            object_slices_equivalent(&actual.objects, &expected.objects, inputs, replay_map)
                && actual.to == expected.to
        }
        (GameEvent::BecameTarget(actual), GameEvent::BecameTarget(expected)) => {
            objects_equivalent(actual.target, expected.target, inputs, replay_map)
                && objects_equivalent(actual.source, expected.source, inputs, replay_map)
        }
        (GameEvent::ControlChanged(actual), GameEvent::ControlChanged(expected)) => {
            objects_equivalent(actual.object, expected.object, inputs, replay_map)
                && actual.to == expected.to
        }
        (GameEvent::Attached(actual), GameEvent::Attached(expected)) => {
            objects_equivalent(actual.attachment, expected.attachment, inputs, replay_map)
                && objects_equivalent(actual.host, expected.host, inputs, replay_map)
        }
        (GameEvent::Unattached(actual), GameEvent::Unattached(expected)) => {
            objects_equivalent(actual.attachment, expected.attachment, inputs, replay_map)
                && objects_equivalent(actual.former_host, expected.former_host, inputs, replay_map)
        }
        (GameEvent::DamageRemoved(actual), GameEvent::DamageRemoved(expected)) => {
            objects_equivalent(actual.object, expected.object, inputs, replay_map)
        }
        (GameEvent::ManaProduced(actual), GameEvent::ManaProduced(expected)) => {
            snapshots_equivalent(
                Some(&actual.source),
                Some(&expected.source),
                inputs,
                replay_map,
                false,
            ) && actual.controller == expected.controller
                && actual.action == expected.action
                && mana_units_equivalent_mapped(
                    &actual.produced,
                    &expected.produced,
                    inputs,
                    replay_map,
                )
        }
        (GameEvent::TappedForMana(actual), GameEvent::TappedForMana(expected)) => {
            snapshots_equivalent(
                Some(&actual.source),
                Some(&expected.source),
                inputs,
                replay_map,
                false,
            ) && actual.controller == expected.controller
                && actual.action == expected.action
                && mana_units_equivalent_mapped(
                    &actual.produced,
                    &expected.produced,
                    inputs,
                    replay_map,
                )
        }
        _ => facts_equivalent(actual, expected),
    }
}

fn objects_equivalent(
    actual: ObjectId,
    expected: ObjectId,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    if let Some(&mapped) = replay_map.object_generations.borrow().get(&expected) {
        return actual == mapped;
    }
    let Some(logical @ LogicalObject::Created { .. }) = inputs.get(&expected) else {
        return actual == expected;
    };
    replay_map.objects.get(logical).copied() == Some(actual)
}

fn bind_created_generation(actual: ObjectId, expected: ObjectId, replay_map: &ReplayMap) -> bool {
    let mut generations = replay_map.object_generations.borrow_mut();
    if let Some(&mapped) = generations.get(&expected) {
        mapped == actual
    } else {
        generations.insert(expected, actual);
        true
    }
}

fn object_slices_equivalent(
    actual: &[ObjectId],
    expected: &[ObjectId],
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected)
            .all(|(&actual, &expected)| objects_equivalent(actual, expected, inputs, replay_map))
}

fn optional_objects_equivalent(
    actual: Option<ObjectId>,
    expected: Option<ObjectId>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    match (actual, expected) {
        (Some(actual), Some(expected)) => objects_equivalent(actual, expected, inputs, replay_map),
        (None, None) => true,
        _ => false,
    }
}

fn enter_statuses_equivalent(
    actual: Option<&crate::event::EnterStatus>,
    expected: Option<&crate::event::EnterStatus>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            actual.tapped == expected.tapped
                && optional_objects_equivalent(
                    actual.attach_to,
                    expected.attach_to,
                    inputs,
                    replay_map,
                )
                && actual.counters == expected.counters
                && actual.controller == expected.controller
                && optional_objects_equivalent(
                    actual.attacking,
                    expected.attacking,
                    inputs,
                    replay_map,
                )
        }
        (None, None) => true,
        _ => false,
    }
}

fn snapshots_equivalent(
    actual: Option<&crate::lki::LkiSnapshot>,
    expected: Option<&crate::lki::LkiSnapshot>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
    created_entry: bool,
) -> bool {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            let created = inputs
                .get(&expected.object)
                .is_some_and(|logical| matches!(logical, LogicalObject::Created { .. }));
            (created_entry
                || objects_equivalent(actual.object, expected.object, inputs, replay_map))
                && (created || actual.source == expected.source)
                && actual.controller == expected.controller
                && actual.tapped == expected.tapped
                && actual.damage == expected.damage
                && optional_objects_equivalent(
                    actual.attached_to,
                    expected.attached_to,
                    inputs,
                    replay_map,
                )
                && actual.counters == expected.counters
                && actual.left == expected.left
        }
        (None, None) => true,
        _ => false,
    }
}

fn causes_equivalent_mapped(
    actual: Option<&crate::event::Cause>,
    expected: Option<&crate::event::Cause>,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            actual.verb == expected.verb
                && actual.agency == expected.agency
                && match (actual.agent, expected.agent) {
                    (Some((actual, actual_player)), Some((expected, expected_player))) => {
                        actual_player == expected_player
                            && objects_equivalent(actual, expected, inputs, replay_map)
                    }
                    (None, None) => true,
                    _ => false,
                }
        }
        (None, None) => true,
        _ => false,
    }
}

fn mana_provenance_equivalent(
    actual: crate::player::ManaProvenance,
    expected: crate::player::ManaProvenance,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    actual.action == expected.action
        && optional_objects_equivalent(actual.source, expected.source, inputs, replay_map)
}

fn mana_units_equivalent_mapped(
    actual: &[crate::player::ManaUnit],
    expected: &[crate::player::ManaUnit],
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    actual.len() == expected.len()
        && actual.iter().zip(expected).all(|(actual, expected)| {
            actual.kind == expected.kind
                && actual.riders == expected.riders
                && mana_provenance_equivalent(
                    actual.provenance,
                    expected.provenance,
                    inputs,
                    replay_map,
                )
        })
}

fn trigger_bindings_equivalent(
    actual: &crate::trigger::TriggerBindings,
    expected: &crate::trigger::TriggerBindings,
    inputs: &HashMap<ObjectId, LogicalObject>,
    replay_map: &ReplayMap,
) -> bool {
    snapshots_equivalent(
        actual.this.as_ref(),
        expected.this.as_ref(),
        inputs,
        replay_map,
        false,
    ) && snapshots_equivalent(
        actual.that_object.as_ref(),
        expected.that_object.as_ref(),
        inputs,
        replay_map,
        false,
    ) && match (&actual.that_patient, &expected.that_patient) {
        (
            Some(crate::trigger::EventPatient::Object(actual)),
            Some(crate::trigger::EventPatient::Object(expected)),
        ) => snapshots_equivalent(Some(actual), Some(expected), inputs, replay_map, false),
        (
            Some(crate::trigger::EventPatient::Player(actual)),
            Some(crate::trigger::EventPatient::Player(expected)),
        ) => actual == expected,
        (None, None) => true,
        _ => false,
    } && actual.that_player == expected.that_player
        && actual.produced_mana == expected.produced_mana
        && actual.defending_player == expected.defending_player
        && actual.event_amount == expected.event_amount
        && actual.crossed == expected.crossed
}

fn facts_equivalent(actual: &GameEvent, expected: &GameEvent) -> bool {
    match (actual, expected) {
        (GameEvent::Tapped(actual), GameEvent::Tapped(expected)) => {
            actual.object == expected.object
                && causes_equivalent(actual.cause.as_ref(), expected.cause.as_ref())
        }
        (GameEvent::ZoneChange(actual), GameEvent::ZoneChange(expected)) => {
            actual.object == expected.object
                && actual.snapshot == expected.snapshot
                && actual.from == expected.from
                && actual.to == expected.to
                && actual.enters == expected.enters
                && actual.position == expected.position
                && actual.face == expected.face
                && causes_equivalent(actual.cause.as_ref(), expected.cause.as_ref())
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
    actual: Option<&crate::event::Cause>,
    expected: Option<&crate::event::Cause>,
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
/// does not encode its source zone, so [CR#733.1]'s library-only reveal barrier
/// cannot be recovered later from the fact alone.
pub(crate) fn contextual_barriers_for(
    state: &GameState,
    fact: &GameEvent,
) -> (Vec<ReversalBarrier>, Vec<ObservationBarrier>) {
    let (mut reversal, mut observation) = barriers_for(std::slice::from_ref(fact));
    if let GameEvent::Revealed(event) = fact {
        let revealed_zones: Vec<_> = event
            .objects
            .iter()
            .filter_map(|&object| state.objects.get(object).and_then(|object| object.zone))
            .collect();
        if revealed_zones
            .iter()
            .copied()
            .any(deckmaste_core::Zone::is_hidden)
        {
            observation.push(ObservationBarrier::HiddenZoneDisclosure);
        }
        if revealed_zones.contains(&deckmaste_core::Zone::Library) {
            reversal.push(ReversalBarrier::RevealedLibraryCard);
        }
    }
    observation.sort_unstable_by_key(|barrier| *barrier as u8);
    observation.dedup();
    (reversal, observation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapped_facts_rebind_created_subjects_and_cause_agents() {
        let old = ObjectId::from_raw(41);
        let reminted = ObjectId::from_raw(97);
        let logical = LogicalObject::Created {
            record: PaymentRecordId(3),
            ordinal: 0,
        };
        let inputs = HashMap::from([(old, logical)]);
        let replay_map = ReplayMap {
            objects: HashMap::from([(logical, reminted)]),
            ..ReplayMap::default()
        };
        let expected_cause = crate::event::Cause::tap(
            deckmaste_core::Agency::CostPayment,
            Some((old, PlayerId(0))),
        );
        let actual_cause = crate::event::Cause::tap(
            deckmaste_core::Agency::CostPayment,
            Some((reminted, PlayerId(0))),
        );
        let expected = GameEvent::Tapped(crate::event::Tapped {
            object: old,
            cause: Some(expected_cause),
        });
        let actual = GameEvent::Tapped(crate::event::Tapped {
            object: reminted,
            cause: Some(actual_cause),
        });
        assert!(facts_equivalent_mapped(
            &actual,
            &expected,
            &inputs,
            &replay_map,
        ));

        assert!(facts_equivalent_mapped(
            &GameEvent::AbilityUsed(crate::event::AbilityUsed {
                object: reminted,
                ability: 0,
            }),
            &GameEvent::AbilityUsed(crate::event::AbilityUsed {
                object: old,
                ability: 0,
            }),
            &inputs,
            &replay_map,
        ));

        let token = deckmaste_core::Token {
            name: Some("Mapped entrant".into()),
            color_indicator: Vec::new().into(),
            supertypes: Vec::new().into(),
            types: Vec::new().into(),
            subtypes: Vec::new().into(),
            abilities: Vec::new().into(),
            power: None,
            toughness: None,
        };
        assert!(facts_equivalent_mapped(
            &GameEvent::TokenCreated(crate::event::TokenCreated {
                player: PlayerId(0),
                token: token.clone(),
                enters: Some(crate::event::EnterStatus {
                    attach_to: Some(reminted),
                    attacking: Some(reminted),
                    ..crate::event::EnterStatus::default()
                }),
            }),
            &GameEvent::TokenCreated(crate::event::TokenCreated {
                player: PlayerId(0),
                token,
                enters: Some(crate::event::EnterStatus {
                    attach_to: Some(old),
                    attacking: Some(old),
                    ..crate::event::EnterStatus::default()
                }),
            }),
            &inputs,
            &replay_map,
        ));
    }
}
