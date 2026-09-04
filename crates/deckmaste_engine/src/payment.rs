mod coverage;
mod fulfill;
mod iou;
mod mana;
mod replay;

use std::sync::Arc;

pub use coverage::ManaCoverage;
pub use coverage::ManaPayment;
use deckmaste_core::Action;
use deckmaste_core::Cmp;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PayAct;
use deckmaste_core::PipClass;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RunnableCostAction;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Stat;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
pub use iou::FulfillmentWitness;
pub use iou::IouId;
pub use iou::IouKind;
pub use iou::ManaPip;
pub use iou::PaymentIou;
pub use iou::PaymentRecordId;
pub use replay::DecisionTranscript;
pub use replay::LogicalObject;
pub use replay::ObservationBarrier;
pub use replay::QualifiedManaId;
pub use replay::RecordedRandomOutcome;
pub use replay::RecordedRngState;
pub use replay::ReplayCommand;
pub use replay::ReplayError;
pub use replay::ReplayMap;
pub use replay::ReversalBarrier;
pub use replay::TransactionRecord;
pub(crate) use replay::contextual_barriers_for;
pub use replay::reconstruct;

use crate::object::ObjectId;
use crate::player::ManaActionId;
use crate::player::ManaUnit;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::state::GameImage;
use crate::state::GameState;

/// The externally visible phase of one locked payment transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStage {
    /// The payer may activate mana abilities, then submits complete mana-pip
    /// coverage. No cost has been paid yet ([CR#601.2g]).
    PrePayment,
    /// Coverage is locked (when applicable) and IOUs may be fulfilled in a
    /// payer-chosen legal order ([CR#601.2h]).
    Paying,
    /// Every IOU is fulfilled; only submission, editing, decline, or
    /// concession remains.
    Ready,
}

/// Whether the active payment frame is waiting at its command boundary or is
/// suspended inside one replaceable/action-producing fulfillment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentProgress {
    Idle,
    Fulfilling {
        iou: IouId,
        witness: FulfillmentWitness,
    },
}

/// Why a payment frame exists. Decline and successful submission promote or
/// discard the working image differently for announcements and optional
/// payments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentPurpose {
    Announcement,
    Optional {
        if_did: Option<Arc<deckmaste_core::Instruction>>,
        if_not: Option<Arc<deckmaste_core::Instruction>>,
        frame: Box<ExecutionFrame>,
    },
}

/// The spell, ability, or resolving effect whose cost is being paid. The
/// object returned by [`Self::object`] is the object mana spend restrictions
/// judge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaymentSubject {
    Spell(ObjectId),
    Activated { ability: ObjectId, source: ObjectId },
    Effect { source: ObjectId },
}

impl PaymentSubject {
    /// The permanent/card bound by source-relative nonmana costs.
    #[must_use]
    pub fn object(self) -> ObjectId {
        match self {
            Self::Spell(object) => object,
            Self::Activated { source, .. } | Self::Effect { source } => source,
        }
    }

    /// The live spell/ability/effect object that mana spend restrictions
    /// judge. An activated ability's source may legally leave the battlefield
    /// while its announcement is still being paid for.
    #[must_use]
    pub fn spend_object(self) -> ObjectId {
        match self {
            Self::Spell(object) => object,
            Self::Activated { ability, .. } => ability,
            Self::Effect { source } => source,
        }
    }
}

/// A client command at the payment decision boundary. Concession remains the
/// universal ordinary decision and is intentionally not duplicated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentCommand {
    BeginPayment(ManaCoverage),
    ActivateManaAbility {
        source: ObjectId,
        ability: usize,
    },
    Fulfill {
        iou: IouId,
        witness: FulfillmentWitness,
    },
    RescindFulfillment(IouId),
    SubmitPayment,
    DeclinePayment,
}

/// The facts a client needs to render a payment decision. It deliberately
/// contains no recommended source, order, or repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentPrompt {
    pub payer: PlayerId,
    pub subject: PaymentSubject,
    pub stage: PaymentStage,
    pub outstanding: Vec<PaymentIou>,
    pub fulfilled: Vec<IouId>,
    pub floating_mana: Vec<ManaUnit>,
    pub coverage: Option<ManaCoverage>,
    /// Outstanding IOUs in the currently legal [CR#601.2h] tier.
    pub fulfillable: Vec<IouId>,
    pub mana_abilities: Vec<(ObjectId, usize)>,
    pub rescindable: Vec<IouId>,
}

/// The immutable IOU graph created after announce-time choices and total-cost
/// locking. `has_mana_payment` distinguishes an explicit `{0}` from an empty
/// cost, even though neither creates a payable pip IOU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedPayment {
    pub payer: PlayerId,
    pub subject: PaymentSubject,
    pub frame: ExecutionFrame,
    pub ious: Vec<PaymentIou>,
    pub stage: PaymentStage,
    pub has_mana_payment: bool,
}

impl LockedPayment {
    #[must_use]
    pub fn mana_pips(&self) -> Vec<&PaymentIou> {
        self.ious
            .iter()
            .filter(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
            .collect()
    }

    fn proposal_placeholder(
        payer: PlayerId,
        subject: PaymentSubject,
        frame: ExecutionFrame,
    ) -> Self {
        Self {
            payer,
            subject,
            frame,
            ious: Vec::new(),
            stage: PaymentStage::Ready,
            has_mana_payment: false,
        }
    }
}

/// One full speculative game image and the metadata needed to edit or commit
/// its locked payment. `payment_base` is established when a mana-paying frame
/// crosses `BeginPayment`; pure nonmana frames establish it on entry.
#[derive(Debug, Clone)]
pub struct PaymentFrame {
    pub(crate) working: GameImage,
    /// The full image before this announcement or optional-payment branch
    /// began. Announcement decline reconstructs retained mana actions from
    /// this one base instead of stacking per-action snapshots.
    pub(crate) proposal_base: GameImage,
    pub(crate) payment_base: Option<GameImage>,
    pub payer: PlayerId,
    pub subject: PaymentSubject,
    pub purpose: PaymentPurpose,
    pub locked: LockedPayment,
    pub stage: PaymentStage,
    pub coverage: Option<ManaCoverage>,
    pub fulfilled: Vec<(IouId, FulfillmentWitness)>,
    pub progress: PaymentProgress,
    pub(crate) fulfillment_continuation: Option<FulfillmentContinuation>,
    pub(crate) records: Vec<TransactionRecord>,
    pub(crate) recording: Option<replay::PendingRecord>,
    /// Stable-handle observations made anywhere in this transaction. Every
    /// active frame receives the union so a later parent reconstruction also
    /// preserves knowledge gained inside a child mana action.
    pub(crate) observations: std::collections::HashSet<(PlayerId, LogicalObject)>,
    pub(crate) logical_objects:
        std::collections::HashMap<crate::object::ObjectSource, LogicalObject>,
    pub(crate) replay_random_outcome: Option<(Vec<ObjectId>, RecordedRngState)>,
    /// Furthest RNG cursor whose outcome became observable while this
    /// transaction was speculative. Decline may reverse physical work, but
    /// it must never make already-seen entropy available to draw again.
    pub(crate) observed_rng: Option<RecordedRngState>,
    /// Activation records referenced by frames in the speculative image.
    /// Replay starts from this snapshot because activations live outside
    /// `GameImage`.
    pub(crate) activations: crate::activation::ActivationTable,
    pub(crate) next_activation: u64,
    /// The negative branch of a resolution-time `May(Cast)` announcement.
    /// Submission leaves the queued `if_did` continuation intact; decline
    /// restores `proposal_base` and schedules this branch instead.
    pub(crate) announcement_if_not: Option<(Arc<deckmaste_core::Instruction>, Box<ExecutionFrame>)>,
    /// The activated mana action whose announcement this nested frame owns.
    /// Root announcements and optional payments leave this absent.
    pub(crate) mana_action: Option<ManaActionId>,
}

/// Rules-control work suspended behind one in-flight fulfillment. Optional
/// decline keeps changes that have already happened in the working image, but
/// abandons the unfinished command and resumes the containing resolution from
/// exactly the queue/control boundary that preceded it.
#[derive(Debug, Clone)]
pub(crate) struct FulfillmentContinuation {
    agenda: std::collections::VecDeque<crate::agenda::WorkItem>,
    control: crate::control::ControlSnapshot,
    arrange_scope: Option<crate::state::ArrangeScope>,
}

impl FulfillmentContinuation {
    fn capture(image: &GameImage) -> Self {
        Self {
            agenda: image.agenda.clone(),
            control: crate::control::ControlSnapshot {
                announcing: image.announcing.clone(),
                pending: None,
                choice: image.choice.clone(),
                placing_trigger: image.placing_trigger.clone(),
                replace_state: image.replace_state.clone(),
            },
            arrange_scope: image.arrange_scope.clone(),
        }
    }

    fn restore(self, image: &mut GameImage) {
        image.agenda = self.agenda;
        image.announcing = self.control.announcing;
        image.pending = None;
        image.choice = self.control.choice;
        image.placing_trigger = self.control.placing_trigger;
        image.replace_state = self.control.replace_state;
        image.arrange_scope = self.arrange_scope;
    }
}

impl PaymentFrame {
    #[must_use]
    pub fn new(working: GameImage, purpose: PaymentPurpose, locked: LockedPayment) -> Self {
        let payment_base = (locked.stage != PaymentStage::PrePayment).then(|| working.clone());
        Self {
            proposal_base: working.clone(),
            working,
            payment_base,
            payer: locked.payer,
            subject: locked.subject,
            purpose,
            stage: locked.stage,
            locked,
            coverage: None,
            fulfilled: Vec::new(),
            progress: PaymentProgress::Idle,
            fulfillment_continuation: None,
            records: Vec::new(),
            recording: None,
            observations: std::collections::HashSet::new(),
            logical_objects: std::collections::HashMap::new(),
            replay_random_outcome: None,
            observed_rng: None,
            activations: std::collections::HashMap::new(),
            next_activation: 0,
            announcement_if_not: None,
            mana_action: None,
        }
    }

    /// Start an isolated announcement before its total cost has locked. The
    /// placeholder is never exposed as a payment prompt; [`Self::initialize`]
    /// replaces it when `OpenPayment` runs.
    pub(crate) fn proposal(working: GameImage, payer: PlayerId) -> Self {
        let source = working.players[payer.index()].object;
        let subject = PaymentSubject::Effect { source };
        let frame = ExecutionFrame {
            activation: crate::ActivationId::NONE,
            payment: None,
        };
        Self::new(
            working,
            PaymentPurpose::Announcement,
            LockedPayment::proposal_placeholder(payer, subject, frame),
        )
    }

    pub(crate) fn initialize(&mut self, locked: LockedPayment) {
        self.payment_base =
            (locked.stage != PaymentStage::PrePayment).then(|| self.working.clone());
        self.payer = locked.payer;
        self.subject = locked.subject;
        self.stage = locked.stage;
        self.locked = locked;
        self.coverage = None;
        self.fulfilled.clear();
        self.progress = PaymentProgress::Idle;
        self.fulfillment_continuation = None;
        self.records.clear();
        self.recording = None;
        self.replay_random_outcome = None;
    }

    pub(crate) fn begin_fulfillment(&mut self, iou: IouId, witness: FulfillmentWitness) {
        self.fulfillment_continuation = Some(FulfillmentContinuation::capture(&self.working));
        self.progress = PaymentProgress::Fulfilling { iou, witness };
    }

    pub(crate) fn abandon_fulfillment(&mut self) {
        if let Some(continuation) = self.fulfillment_continuation.take() {
            continuation.restore(&mut self.working);
        } else {
            self.working.pending = None;
        }
        self.progress = PaymentProgress::Idle;
        self.recording = None;
    }

    #[must_use]
    pub(crate) fn prompt(&self, mana_abilities: Vec<(ObjectId, usize)>) -> PaymentPrompt {
        let fulfilled: Vec<IouId> = self.fulfilled.iter().map(|(iou, _)| *iou).collect();
        let fulfilled_set: std::collections::HashSet<IouId> = fulfilled.iter().copied().collect();
        let outstanding: Vec<PaymentIou> = self
            .locked
            .ious
            .iter()
            .filter(|iou| !fulfilled_set.contains(&iou.id))
            .cloned()
            .collect();
        let fulfillable = if self.stage == PaymentStage::Paying {
            fulfill::current_tier(&outstanding)
                .into_iter()
                .map(|iou| iou.id)
                .collect()
        } else {
            Vec::new()
        };
        PaymentPrompt {
            payer: self.payer,
            subject: self.subject,
            stage: self.stage,
            outstanding,
            fulfilled,
            floating_mana: self.working.players[self.payer.index()]
                .mana_pool
                .units()
                .to_vec(),
            coverage: self.coverage.clone(),
            fulfillable,
            mana_abilities,
            rescindable: self
                .records
                .iter()
                .filter(|record| record.selectively_reversible())
                .filter_map(TransactionRecord::iou)
                .collect(),
        }
    }
}

fn automatic_floating_coverage(state: &GameState, locked: &LockedPayment) -> Option<ManaCoverage> {
    fn search(
        state: &GameState,
        locked: &LockedPayment,
        pips: &[(IouId, ManaPip)],
        units: &[ManaUnit],
        used: &mut [bool],
        assignments: &mut Vec<(IouId, crate::player::FloatingManaId)>,
    ) -> bool {
        let Some(&(iou, pip)) = pips.get(assignments.len()) else {
            return true;
        };
        for (index, unit) in units.iter().enumerate() {
            if used[index]
                || !coverage::pip_accepts_unit(pip, unit.kind, &unit.riders)
                || !state.unit_spendable_on(unit, locked.subject.spend_object())
            {
                continue;
            }
            used[index] = true;
            assignments.push((iou, unit.id));
            if search(state, locked, pips, units, used, assignments) {
                return true;
            }
            assignments.pop();
            used[index] = false;
        }
        false
    }

    let pips: Vec<(IouId, ManaPip)> = locked
        .mana_pips()
        .into_iter()
        .map(|iou| {
            let IouKind::ManaPip(pip) = iou.kind else {
                unreachable!("mana_pips returns only mana IOUs")
            };
            (iou.id, pip)
        })
        .collect();
    let units = state.player(locked.payer).mana_pool.units();
    let mut used = vec![false; units.len()];
    let mut assignments = Vec::with_capacity(pips.len());
    if !search(state, locked, &pips, units, &mut used, &mut assignments) {
        return None;
    }
    let mut coverage = ManaCoverage::empty();
    for (iou, unit) in assignments {
        coverage.insert(iou, ManaPayment::Floating(unit));
    }
    Some(coverage)
}

pub(crate) fn automatic_activation_cost_usable(
    state: &GameState,
    source: ObjectId,
    ability: usize,
) -> bool {
    let abilities = crate::derive::usable_abilities(state, source);
    let Some(compiled) = abilities.get(ability) else {
        return false;
    };
    let Some(activated) = crate::activate::as_activated(compiled) else {
        return false;
    };
    let Some(summary) = crate::activate::cost_summary(activated.cost.as_ref()) else {
        return false;
    };
    let object = state.objects.obj(source);
    if (summary.tap && object.tapped) || (summary.untap && !object.tapped) {
        return false;
    }
    let controller = object.controller;
    let frame = state.frame(source, controller);
    let mana = state.resolve_cost_mana(&summary, source, controller);
    if !crate::cast::can_pay(&state.spendable_pool(controller, source), &mana)
        || summary
            .verbs
            .iter()
            .any(|verb| !state.verb_cost_payable(verb, controller, &frame))
    {
        return false;
    }
    if summary
        .steps
        .iter()
        .any(|step| !automatic_step_is_satisfiable(state, step, &frame))
    {
        return false;
    }
    summary.tap_totals.iter().all(|requirement| {
        let count = state.eval_count(&requirement.count, &frame);
        automatic_tap_total_subset(
            state,
            requirement.stat,
            requirement.cmp,
            count,
            &requirement.filter,
            &frame,
        )
        .is_some()
    })
}

/// Whether the automatic payer can satisfy one cost instruction against
/// `frame` ([CR#601.2b]) — the gate half of [`automatic_step_witness`].
fn automatic_step_is_satisfiable(
    state: &GameState,
    step: &CostComponent,
    frame: &ExecutionFrame,
) -> bool {
    !matches!(step, CostComponent::Choose(_) | CostComponent::Sample(_))
        || automatic_step_witness(state, step, frame).is_some()
}

/// The witness the automatic payer offers for one cost instruction
/// ([CR#601.2b,601.2h]). A choice needs a concrete legal object set; a search
/// takes whatever the searched zones hold ([CR#701.23d]); everything else is
/// already bound.
fn automatic_step_witness(
    state: &GameState,
    step: &CostComponent,
    frame: &ExecutionFrame,
) -> Option<FulfillmentWitness> {
    let watcher = Some(state.frame_watcher(frame));
    match step {
        CostComponent::Choose(choice) => {
            let mut candidates = crate::target::candidates_region_with_activation(
                state,
                &choice.filter,
                watcher,
                frame.activation,
            );
            // [CR#107.1c]: "any number" includes zero, so a choice with no
            // lower bound is completely paid by choosing nothing — the payer
            // spends exactly what the cost demands and no more ([CR#601.2h]:
            // the prohibition is on PARTIAL payment, which a zero-lower-bound
            // cost paid at zero is not).
            let (lo, _) = choice.quantity.bounds();
            let count = lo.map_or(0, |count| state.eval_count(count, frame));
            let count = usize::try_from(count).ok()?;
            (candidates.len() >= count).then(|| {
                candidates.truncate(count);
                FulfillmentWitness::Objects(candidates)
            })
        }
        CostComponent::Sample(sample) => {
            let candidates = crate::target::candidates_region_with_activation(
                state,
                &sample.filter,
                watcher,
                frame.activation,
            );
            let (lo, _) = sample.quantity.bounds();
            let count = lo.map_or(0, |count| state.eval_count(count, frame));
            let count = usize::try_from(count).ok()?;
            (candidates.len() >= count).then_some(FulfillmentWitness::Bound)
        }
        CostComponent::Search(search) => {
            let mut candidates = automatic_search_candidates(
                state,
                &search.whose,
                &search.from,
                &search.filter.body,
                frame,
            );
            let (_, hi) = search.quantity.bounds();
            let count = hi
                .map_or(candidates.len(), |count| {
                    usize::try_from(state.eval_count(count, frame)).unwrap_or(candidates.len())
                })
                .min(candidates.len());
            candidates.truncate(count);
            Some(FulfillmentWitness::Objects(candidates))
        }
        _ => Some(FulfillmentWitness::Bound),
    }
}

fn automatic_search_candidates(
    state: &GameState,
    whose: &Reference,
    from: &[Zone],
    filter: &Predicate,
    frame: &ExecutionFrame,
) -> Vec<ObjectId> {
    let Some(owner) = state.eval_player_ref(whose, frame) else {
        return Vec::new();
    };
    let watcher = Some(state.frame_watcher(frame));
    state
        .objects
        .iter()
        .filter(|object| object.zone.is_some_and(|zone| from.contains(&zone)))
        .filter(|object| state.owner_of(object.id) == owner)
        .filter(|object| crate::target::matches_with(state, object.id, filter, watcher))
        .map(|object| object.id)
        .collect()
}

fn automatic_tap_total_subset(
    state: &GameState,
    stat: Stat,
    cmp: Cmp,
    count: Uint,
    filter: &Predicate,
    frame: &ExecutionFrame,
) -> Option<Vec<ObjectId>> {
    fn search(
        candidates: &[(ObjectId, Uint)],
        index: usize,
        selected: &mut Vec<ObjectId>,
        sum: Uint,
        cmp: Cmp,
        count: Uint,
        best: &mut Option<(Vec<ObjectId>, Uint)>,
    ) {
        if index == candidates.len() {
            if !cmp.apply(sum, count) {
                return;
            }
            let improves = best.as_ref().is_none_or(|(current, current_sum)| {
                selected.len() < current.len()
                    || (selected.len() == current.len() && sum > *current_sum)
            });
            if improves {
                *best = Some((selected.clone(), sum));
            }
            return;
        }
        search(candidates, index + 1, selected, sum, cmp, count, best);
        selected.push(candidates[index].0);
        search(
            candidates,
            index + 1,
            selected,
            sum.saturating_add(candidates[index].1),
            cmp,
            count,
            best,
        );
        selected.pop();
    }

    let watcher = Some(state.frame_watcher(frame));
    let view = state.layers();
    let candidates: Vec<(ObjectId, Uint)> = crate::target::candidates_with(state, filter, watcher)
        .into_iter()
        .filter(|&object| {
            state.objects.get(object).is_some_and(|candidate| {
                candidate.zone == Some(Zone::Battlefield) && !candidate.tapped
            })
        })
        .filter_map(|object| {
            state
                .cost_stat_value(&view, object, stat)
                .map(|value| (object, value))
        })
        .collect();
    let mut best = None;
    search(&candidates, 0, &mut Vec::new(), 0, cmp, count, &mut best);
    best.map(|(objects, _)| objects)
}

fn automatic_tap_total_witness(
    state: &GameState,
    id: IouId,
    stat: Stat,
) -> Option<FulfillmentWitness> {
    let view = state.layers();
    let mut best: Option<(Vec<ObjectId>, Uint)> = None;
    for subset in state.legal_tap_total_subsets(id) {
        let sum = subset.iter().fold(0 as Uint, |sum, &object| {
            sum.saturating_add(state.cost_stat_value(&view, object, stat).unwrap_or(0))
        });
        let improves = best.as_ref().is_none_or(|(current, current_sum)| {
            subset.len() < current.len() || (subset.len() == current.len() && sum > *current_sum)
        });
        if improves {
            best = Some((subset, sum));
        }
    }
    best.map(|(objects, _)| FulfillmentWitness::Objects(objects))
}

fn collect_mana_reversal_candidates(
    records: &[TransactionRecord],
    candidates: &mut Vec<ManaActionId>,
) {
    for record in records {
        collect_mana_reversal_candidates(&record.children, candidates);
        if let ReplayCommand::ManaAbility { action, .. } = record.command
            && record.selectively_reversible()
            && !candidates.contains(&action)
        {
            candidates.push(action);
        }
    }
}

fn all_descendant_mana_actions_are_reversed(
    records: &[TransactionRecord],
    reversed: &[ManaActionId],
) -> bool {
    records.iter().all(|record| {
        record
            .mana_action()
            .is_none_or(|action| reversed.contains(&action))
            && all_descendant_mana_actions_are_reversed(&record.children, reversed)
    })
}

fn mana_reversal_set_preserves_nested_units(
    records: &[TransactionRecord],
    reversed: &[ManaActionId],
) -> bool {
    records.iter().all(|record| {
        record.mana_action().is_none_or(|action| {
            !reversed.contains(&action)
                || all_descendant_mana_actions_are_reversed(&record.children, reversed)
        }) && mana_reversal_set_preserves_nested_units(&record.children, reversed)
    })
}

fn collect_retained_descendant_mana_records(
    records: &[TransactionRecord],
    reversed: &[ManaActionId],
    retained: &mut Vec<TransactionRecord>,
) {
    for record in records {
        if let Some(action) = record.mana_action() {
            if !reversed.contains(&action) {
                retained.push(record.clone());
            }
        } else {
            collect_retained_descendant_mana_records(&record.children, reversed, retained);
        }
    }
}

fn retained_decline_records(
    records: &[TransactionRecord],
    retained_ids: &[PaymentRecordId],
    reversed: &[ManaActionId],
) -> Vec<TransactionRecord> {
    let mut retained = Vec::new();
    for record in records {
        if retained_ids.contains(&record.id) {
            retained.push(record.clone());
        } else {
            collect_retained_descendant_mana_records(&record.children, reversed, &mut retained);
        }
    }
    retained
}

fn collect_forced_retained_record_ids(
    records: &[TransactionRecord],
    reversible_actions: &std::collections::HashSet<ManaActionId>,
    reversed: &[ManaActionId],
    top_level: bool,
    forced: &mut Vec<PaymentRecordId>,
) {
    for record in records {
        if record
            .mana_action()
            .is_some_and(|action| reversed.contains(&action))
        {
            continue;
        }
        let forced_record = if let Some(action) = record.mana_action() {
            !record.reversal_barriers.is_empty() || !reversible_actions.contains(&action)
        } else {
            top_level && !record.reversal_barriers.is_empty()
        };
        if forced_record && !forced.contains(&record.id) {
            forced.push(record.id);
        }
        collect_forced_retained_record_ids(
            &record.children,
            reversible_actions,
            reversed,
            false,
            forced,
        );
    }
}

fn records_cross_reversal_barrier(records: &[TransactionRecord]) -> bool {
    records.iter().any(|record| {
        !record.reversal_barriers.is_empty() || records_cross_reversal_barrier(&record.children)
    })
}

fn records_cross_observation_barrier(records: &[TransactionRecord]) -> bool {
    records.iter().any(|record| {
        !record.observation_barriers.is_empty()
            || records_cross_observation_barrier(&record.children)
    })
}

fn records_contain_mana_action(records: &[TransactionRecord]) -> bool {
    records.iter().any(|record| {
        record.mana_action().is_some() || records_contain_mana_action(&record.children)
    })
}

/// Transaction metadata lives outside every [`GameImage`], preventing image
/// clones from recursively cloning their owner.
#[derive(Debug, Clone, Default)]
pub struct PaymentController {
    pub(crate) frames: Vec<PaymentFrame>,
    pub(crate) mana_actions: Vec<mana::ManaAction>,
    next_record: u64,
}

impl GameState {
    fn payment_replay_frame(&self, frame: &PaymentFrame) -> PaymentFrame {
        let mut replay = frame.clone();
        replay.activations.clone_from(&self.activations.borrow());
        replay.next_activation = self.next_activation.get();
        replay
    }

    /// Deterministic compatibility answer for the currently pending payment.
    ///
    /// This intentionally narrow runner shim covers the current monocolor
    /// clients: activate the first advisory-payable mana ability until exact
    /// floating-mana coverage exists, choose deterministic complete witnesses
    /// for object and aggregate costs, fulfill in the engine-provided tier
    /// order, then submit. Interactive clients remain responsible for exposing
    /// the full choice set; this runner policy never changes core legality.
    ///
    /// # Panics
    ///
    /// Panics if an internally inconsistent payment prompt has no active frame.
    #[must_use]
    pub fn auto_payment_pending(&self) -> Option<crate::decide::Decision> {
        let Some(crate::decide::DecisionPointKind::Payment(prompt)) = self.pending.as_ref() else {
            return None;
        };
        let command = match prompt.stage {
            PaymentStage::PrePayment => {
                let frame = self
                    .payment
                    .as_ref()
                    .and_then(|controller| controller.frames.last())
                    .expect("a Payment prompt has an active frame");
                if let Some(coverage) = automatic_floating_coverage(self, &frame.locked) {
                    PaymentCommand::BeginPayment(coverage)
                } else if let Some(&(source, ability)) =
                    prompt.mana_abilities.iter().find(|&&(source, ability)| {
                        automatic_activation_cost_usable(self, source, ability)
                    })
                {
                    PaymentCommand::ActivateManaAbility { source, ability }
                } else {
                    PaymentCommand::DeclinePayment
                }
            }
            PaymentStage::Paying => {
                let Some(iou) = prompt
                    .fulfillable
                    .first()
                    .and_then(|id| prompt.outstanding.iter().find(|iou| iou.id == *id))
                else {
                    return Some(crate::decide::Decision::Payment(
                        PaymentCommand::DeclinePayment,
                    ));
                };
                let witness = match &iou.kind {
                    IouKind::ManaPip(_) => FulfillmentWitness::CoveredMana,
                    IouKind::PayLife(_) => FulfillmentWitness::PayLife,
                    IouKind::Tap | IouKind::Untap | IouKind::Act { .. } | IouKind::Let(_) => {
                        FulfillmentWitness::Bound
                    }
                    IouKind::Choose(_) | IouKind::Sample(_) | IouKind::Search(_) => {
                        let frame = self
                            .payment
                            .as_ref()
                            .and_then(|controller| controller.frames.last())
                            .expect("a Payment prompt has an active frame");
                        let step = match &iou.kind {
                            IouKind::Choose(choice) => CostComponent::Choose(choice.clone()),
                            IouKind::Sample(sample) => CostComponent::Sample(sample.clone()),
                            IouKind::Search(search) => CostComponent::Search(search.clone()),
                            _ => unreachable!("the match arm selected a decision IOU"),
                        };
                        let Some(witness) =
                            automatic_step_witness(self, &step, &frame.locked.frame)
                        else {
                            return Some(crate::decide::Decision::Payment(
                                PaymentCommand::DeclinePayment,
                            ));
                        };
                        witness
                    }
                    IouKind::TapTotal { stat, .. } => {
                        let Some(witness) = automatic_tap_total_witness(self, iou.id, *stat) else {
                            return Some(crate::decide::Decision::Payment(
                                PaymentCommand::DeclinePayment,
                            ));
                        };
                        witness
                    }
                };
                PaymentCommand::Fulfill {
                    iou: iou.id,
                    witness,
                }
            }
            PaymentStage::Ready => PaymentCommand::SubmitPayment,
        };
        Some(crate::decide::Decision::Payment(command))
    }

    pub(crate) fn mint_payment_record(&mut self) -> PaymentRecordId {
        let controller = self
            .payment
            .as_mut()
            .expect("payment records belong to an active controller");
        let id = PaymentRecordId(controller.next_record);
        controller.next_record = controller
            .next_record
            .checked_add(1)
            .expect("payment record id overflow");
        id
    }

    pub(crate) fn note_payment_created_object(&mut self, event: &crate::event::GameEvent) {
        let crate::event::GameEvent::ZoneChange(crate::event::ZoneChange {
            snapshot: Some(snapshot),
            from: None,
            ..
        }) = event
        else {
            return;
        };
        let Some(record) = self.payment.as_ref().and_then(|controller| {
            controller
                .frames
                .last()
                .and_then(|frame| frame.recording.as_ref().map(|draft| draft.id))
                .or_else(|| controller.mana_actions.last().map(|action| action.record))
        }) else {
            return;
        };
        if self.payment_logical_objects.contains_key(&snapshot.source) {
            return;
        }
        let ordinal = self
            .payment_logical_objects
            .values()
            .filter(|logical| {
                matches!(logical, LogicalObject::Created { record: owner, .. } if *owner == record)
            })
            .count();
        self.payment_logical_objects
            .insert(snapshot.source, LogicalObject::Created { record, ordinal });
        if let Some(controller) = self.payment.as_mut() {
            for frame in &mut controller.frames {
                frame
                    .logical_objects
                    .insert(snapshot.source, LogicalObject::Created { record, ordinal });
            }
        }
    }

    pub(crate) fn payment_logical_object(&self, object: ObjectId) -> LogicalObject {
        let source = self.objects.obj(object).source;
        self.payment_logical_source(source)
    }

    pub(crate) fn payment_logical_source(
        &self,
        source: crate::object::ObjectSource,
    ) -> LogicalObject {
        self.payment_logical_objects
            .get(&source)
            .copied()
            .unwrap_or(match source {
                crate::object::ObjectSource::Card(card) => LogicalObject::Card(card),
                crate::object::ObjectSource::Player(player) => LogicalObject::Player(player),
            })
    }

    pub(crate) fn payment_record_object_inputs(
        &self,
        facts: &[crate::event::GameEvent],
        mut inputs: std::collections::HashMap<ObjectId, LogicalObject>,
    ) -> std::collections::HashMap<ObjectId, LogicalObject> {
        for object in self.objects.iter() {
            if let Some(&logical) = self.payment_logical_objects.get(&object.source) {
                inputs.insert(object.id, logical);
            }
        }
        for fact in facts {
            if let crate::event::GameEvent::ZoneChange(event) = fact
                && let Some(snapshot) = event.snapshot.as_ref()
                && let Some(&logical) = self.payment_logical_objects.get(&snapshot.source)
            {
                inputs.insert(event.object, logical);
                inputs.insert(snapshot.object, logical);
            }
        }
        inputs
    }

    pub(crate) fn grant_payment_look(&mut self, player: PlayerId, object: ObjectId) {
        let logical = self
            .payment
            .is_some()
            .then(|| self.payment_logical_object(object));
        self.look_grants.insert((player, object));
        let Some(logical) = logical else { return };
        self.payment_observations.insert((player, logical));
        if let Some(controller) = self.payment.as_mut() {
            for frame in &mut controller.frames {
                frame.observations.insert((player, logical));
            }
        }
    }

    pub(crate) fn take_replay_random_outcome(
        &mut self,
    ) -> Option<(Vec<ObjectId>, RecordedRngState)> {
        self.payment
            .as_mut()
            .and_then(|controller| controller.frames.last_mut())
            .and_then(|frame| frame.replay_random_outcome.take())
    }

    pub(crate) fn record_payment_random_outcome(&mut self, objects: &[ObjectId]) {
        let post_sample_rng = RecordedRngState {
            stream: self.rng.get_stream(),
            word_pos: self.rng.get_word_pos(),
        };
        let logical: Vec<_> = objects
            .iter()
            .map(|&object| self.payment_logical_object(object))
            .collect();
        let Some(controller) = self.payment.as_mut() else {
            return;
        };
        for frame in &mut controller.frames {
            frame.observed_rng = Some(post_sample_rng);
        }
        let Some(recording) = controller
            .frames
            .last_mut()
            .and_then(|frame| frame.recording.as_mut())
        else {
            return;
        };
        let ReplayCommand::Fulfill { random_outcome, .. } = &mut recording.command else {
            return;
        };
        *random_outcome = Some(RecordedRandomOutcome {
            objects: logical.clone(),
            post_sample_rng,
        });
        recording
            .observation_barriers
            .push(ObservationBarrier::RandomOutcome);
        recording
            .object_inputs
            .extend(objects.iter().copied().zip(logical));
    }

    pub(crate) fn clear_payment_metadata(&mut self) {
        self.payment_observations.clear();
        self.payment_logical_objects.clear();
    }

    /// Clone the active image before any proposal mutation and route ordinary
    /// engine field access into that isolated working image.
    pub(crate) fn begin_payment_proposal(&mut self, payer: PlayerId) {
        let working = self.active().clone();
        let observations = self.payment_observations.clone();
        let logical_objects = self.payment_logical_objects.clone();
        let activations = self.activations.borrow().clone();
        let next_activation = self.next_activation.get();
        let controller = self.payment.get_or_insert_with(PaymentController::default);
        let mut frame = PaymentFrame::proposal(working, payer);
        frame.observations = observations;
        frame.logical_objects = logical_objects;
        frame.activations = activations;
        frame.next_activation = next_activation;
        controller.frames.push(frame);
    }

    /// Replace a resolution-time cast proposal's speculative announce tail
    /// with the containing resolution queue used on decline, and retain the
    /// `May(Cast)` negative branch at that same transaction boundary.
    pub(crate) fn configure_resolution_cast_decline(
        &mut self,
        resume: &[crate::agenda::WorkItem],
        if_not: Option<(Arc<deckmaste_core::Instruction>, Box<ExecutionFrame>)>,
    ) {
        let frame = self
            .payment
            .as_mut()
            .and_then(|controller| controller.frames.last_mut())
            .expect("a resolution-time cast opens an announcement payment frame");
        frame.proposal_base.agenda = resume.iter().cloned().collect();
        frame.announcement_if_not = if_not;
    }

    /// Open a resolution-time optional cost as a real payment transaction.
    /// There is no preliminary affordability oracle or Yes/No commitment: the
    /// payer either submits a complete payment or declines the frame.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "the payment frame owns the optional-cost description at its transaction boundary"
    )]
    pub(crate) fn begin_optional_payment(
        &mut self,
        payer: PlayerId,
        cost: Cost,
        if_did: Option<Arc<deckmaste_core::Instruction>>,
        if_not: Option<Arc<deckmaste_core::Instruction>>,
        frame: ExecutionFrame,
    ) {
        // A cost's `You` is its payer. Fork the containing effect activation
        // so payment decisions and their registers are isolated from sibling
        // branches, then rebind only its controller channel; the original
        // frame remains the continuation for `if_did`/`if_not`.
        let mut payment_frame = self.fork_frame(&frame);
        self.frame_set_controller(&mut payment_frame, payer);
        payment_frame.payment = Some(self.mint_payment());

        // Install the isolated image first so the payment id and all later
        // mutations belong only to this optional transaction.
        let working = self.active().clone();
        let observations = self.payment_observations.clone();
        let logical_objects = self.payment_logical_objects.clone();
        let activations = self.activations.borrow().clone();
        let next_activation = self.next_activation.get();
        let mut proposal = PaymentFrame::proposal(working, payer);
        proposal.observations = observations;
        proposal.logical_objects = logical_objects;
        proposal.activations = activations;
        proposal.next_activation = next_activation;
        self.payment
            .get_or_insert_with(PaymentController::default)
            .frames
            .push(proposal);

        let subject = PaymentSubject::Effect {
            source: frame.source(self),
        };
        let locked = lock_cost(self, payer, subject, &payment_frame, &cost, &[])
            .expect("a lowered optional cost is concrete at the runnable boundary");
        let controller = self.payment.as_mut().expect("optional frame is installed");
        let top = controller
            .frames
            .last_mut()
            .expect("optional frame remains installed");
        top.purpose = PaymentPurpose::Optional {
            if_did,
            if_not,
            frame: Box::new(frame),
        };
        top.initialize(locked);
        let prompt = top.prompt(Vec::new());
        top.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
        self.refresh_payment_prompt();
    }

    /// Install a just-locked IOU graph on the top proposal frame and surface
    /// its first payment prompt.
    pub(crate) fn open_locked_payment(&mut self, locked: LockedPayment) {
        let controller = self
            .payment
            .as_mut()
            .expect("OpenPayment runs inside an isolated proposal");
        let frame = controller
            .frames
            .last_mut()
            .expect("OpenPayment has a proposal frame");
        frame.initialize(locked);
        let prompt = frame.prompt(Vec::new());
        frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
        self.refresh_payment_prompt();
    }

    /// Apply one payment-protocol command. Every rejection occurs before any
    /// frame or image mutation, preserving the pending prompt verbatim.
    pub(crate) fn submit_payment_command(
        &mut self,
        command: PaymentCommand,
    ) -> Result<(), crate::decide::DecisionError> {
        match command {
            PaymentCommand::BeginPayment(coverage) => self.begin_payment(coverage),
            PaymentCommand::Fulfill { iou, witness } => self.fulfill_payment_iou(iou, witness),
            PaymentCommand::SubmitPayment => self.commit_payment(),
            PaymentCommand::DeclinePayment => self.decline_payment(),
            PaymentCommand::ActivateManaAbility { source, ability } => {
                self.activate_payment_mana_ability(source, ability)
            }
            PaymentCommand::RescindFulfillment(iou) => self.rescind_payment_fulfillment(iou),
        }
    }

    pub(crate) fn record_payment_decision(&mut self, decision: &crate::decide::Decision) {
        let Some(controller) = self.payment.as_mut() else {
            return;
        };
        if matches!(
            decision,
            crate::decide::Decision::Payment(PaymentCommand::ActivateManaAbility { .. })
        ) {
            // Decisions are recorded after their handler succeeds. The last
            // action is therefore the action just opened by this command; its
            // enclosing action/fulfillment owns the activation boundary.
            let mut recorded_by_parent_action = false;
            if controller.mana_actions.len() >= 2 {
                let parent = controller.mana_actions.len() - 2;
                if controller.mana_actions[parent].is_submitted() {
                    controller.mana_actions[parent]
                        .transcript
                        .answers
                        .push(decision.clone());
                    recorded_by_parent_action = true;
                }
            }
            if !recorded_by_parent_action {
                for recording in controller
                    .frames
                    .iter_mut()
                    .filter_map(|frame| frame.recording.as_mut())
                {
                    recording.transcript.answers.push(decision.clone());
                }
            }
            return;
        }
        let active_action_is_submitted = controller
            .mana_actions
            .last()
            .is_some_and(mana::ManaAction::is_submitted);
        let active_action_owns_top_frame = controller.mana_actions.last().is_some_and(|action| {
            controller
                .frames
                .last()
                .is_some_and(|frame| frame.mana_action == Some(action.id))
        });
        let has_active_fulfillment = controller
            .frames
            .iter()
            .any(|frame| frame.recording.is_some());
        let is_fulfill_boundary = matches!(
            decision,
            crate::decide::Decision::Payment(PaymentCommand::Fulfill { .. })
        );
        if let Some(action) = controller.mana_actions.last_mut()
            && (active_action_is_submitted
                || active_action_owns_top_frame
                || !has_active_fulfillment
                || is_fulfill_boundary)
        {
            action.transcript.answers.push(decision.clone());
            return;
        }
        match decision {
            // The Fulfill command is already the innermost draft's replay
            // command. An enclosing in-flight command still needs the answer
            // because it must reopen this nested optional payment on replay.
            crate::decide::Decision::Payment(PaymentCommand::Fulfill { iou, .. }) => {
                let mut skipped_own_command = false;
                for recording in controller
                    .frames
                    .iter_mut()
                    .rev()
                    .filter_map(|frame| frame.recording.as_mut())
                {
                    if !skipped_own_command
                        && matches!(recording.command, ReplayCommand::Fulfill { iou: own, .. } if own == *iou)
                    {
                        skipped_own_command = true;
                    } else {
                        recording.transcript.answers.push(decision.clone());
                    }
                }
            }
            crate::decide::Decision::Payment(_) => {
                for recording in controller
                    .frames
                    .iter_mut()
                    .filter_map(|frame| frame.recording.as_mut())
                {
                    recording.transcript.answers.push(decision.clone());
                }
            }
            _ => {
                // The innermost command owns the ordinary answer, and every
                // enclosing suspended command receives the flattened answer
                // needed to drive that child to the same boundary.
                for recording in controller
                    .frames
                    .iter_mut()
                    .filter_map(|frame| frame.recording.as_mut())
                {
                    recording.transcript.answers.push(decision.clone());
                }
            }
        }
    }

    fn rescind_payment_fulfillment(
        &mut self,
        iou: IouId,
    ) -> Result<(), crate::decide::DecisionError> {
        let retained = {
            let controller = self
                .payment
                .as_ref()
                .expect("a Payment decision has a controller");
            let frame = controller
                .frames
                .last()
                .expect("a Payment decision has a frame");
            if frame.progress != PaymentProgress::Idle {
                return Err(crate::decide::DecisionError::Illegal {
                    reason: "RescindFulfillment is unavailable while a fulfillment is in flight"
                        .into(),
                });
            }
            let Some(selected) = frame
                .records
                .iter()
                .find(|record| record.iou() == Some(iou))
            else {
                return Err(crate::decide::DecisionError::Illegal {
                    reason: "RescindFulfillment must name a fulfilled IOU".into(),
                });
            };
            if !selected.selectively_reversible() {
                return Err(crate::decide::DecisionError::Illegal {
                    reason: "that fulfillment crossed a replay barrier".into(),
                });
            }
            frame
                .records
                .iter()
                .filter(|record| record.iou() != Some(iou))
                .map(|record| record.id)
                .collect::<Vec<_>>()
        };

        let rebuilt = {
            let frame = self
                .payment
                .as_ref()
                .and_then(|controller| controller.frames.last())
                .expect("a Payment decision has a frame");
            let replay_frame = self.payment_replay_frame(frame);
            replay::reconstruct_frame(&replay_frame, &retained).map_err(|error| {
                crate::decide::DecisionError::Illegal {
                    reason: format!("payment replay failed: {error}"),
                }
            })?
        };

        self.payment_logical_objects
            .clone_from(&rebuilt.logical_objects);
        self.activations.replace(rebuilt.activations);
        self.next_activation.set(rebuilt.next_activation);
        let controller = self.payment.as_mut().expect("controller remains live");
        let frame = controller.frames.last_mut().expect("frame remains live");
        frame.working = rebuilt.working;
        frame.stage = rebuilt.stage;
        frame.coverage = rebuilt.coverage;
        frame.fulfilled = rebuilt.fulfilled;
        frame.records = rebuilt.records;
        frame.progress = PaymentProgress::Idle;
        frame.recording = None;
        let prompt = frame.prompt(Vec::new());
        frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
        self.refresh_payment_prompt();
        Ok(())
    }

    fn begin_payment(
        &mut self,
        coverage: ManaCoverage,
    ) -> Result<(), crate::decide::DecisionError> {
        let controller = self
            .payment
            .as_ref()
            .expect("a Payment decision has a controller");
        let frame = controller
            .frames
            .last()
            .expect("a Payment decision has a frame");
        if frame.stage != PaymentStage::PrePayment {
            return Err(crate::decide::DecisionError::Illegal {
                reason: "BeginPayment is legal only during PrePayment".into(),
            });
        }
        frame.locked.validate_coverage(self, &coverage)?;

        // Validation above is read-only. From here on the command cannot fail.
        self.pending = None;
        let payment_base = self.active().clone();
        let controller = self.payment.as_mut().expect("controller remains live");
        let frame = controller.frames.last_mut().expect("frame remains live");
        frame.payment_base = Some(payment_base);
        frame.coverage = Some(coverage);
        frame.stage = if frame.locked.ious.is_empty() {
            PaymentStage::Ready
        } else {
            PaymentStage::Paying
        };
        let prompt = frame.prompt(Vec::new());
        frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
        self.refresh_payment_prompt();
        Ok(())
    }

    fn commit_payment(&mut self) -> Result<(), crate::decide::DecisionError> {
        let controller = self
            .payment
            .as_ref()
            .expect("a Payment decision has a controller");
        let frame = controller
            .frames
            .last()
            .expect("a Payment decision has a frame");
        if frame.stage != PaymentStage::Ready {
            return Err(crate::decide::DecisionError::Illegal {
                reason: "SubmitPayment requires every IOU to be fulfilled".into(),
            });
        }
        let root = controller.frames.len() == 1;
        let root_mana_action = root && frame.mana_action.is_some();
        let purpose = frame.purpose.clone();
        let nested_resolution_announcement =
            !root && matches!(purpose, PaymentPurpose::Announcement) && frame.mana_action.is_none();
        self.pending = None;
        if root && !root_mana_action {
            let mut controller = self.payment.take().expect("controller remains live");
            let frame = controller.frames.pop().expect("root frame remains live");
            self.committed = frame.working;
            self.clear_payment_metadata();
        } else if root_mana_action {
            self.payment
                .as_mut()
                .expect("controller remains live")
                .mana_actions
                .last_mut()
                .expect("a submitted root mana frame owns its action")
                .submission = mana::ManaActionSubmission::Submitted;
        } else if matches!(purpose, PaymentPurpose::Optional { .. })
            || nested_resolution_announcement
        {
            self.promote_nested_payment_frame(false);
        } else if !root {
            let controller = self.payment.as_mut().expect("controller remains live");
            let child = controller
                .frames
                .last_mut()
                .expect("child frame remains live");
            let (cost_records, nested_actions): (Vec<_>, Vec<_>) = child
                .records
                .drain(..)
                .partition(|record| record.iou().is_some());
            child.records = nested_actions;
            let action = controller
                .mana_actions
                .last_mut()
                .expect("a submitted mana child belongs to an active mana action");
            action.cost_records = cost_records;
            action.submission = mana::ManaActionSubmission::Submitted;
        }
        if let PaymentPurpose::Optional { if_did, frame, .. } = purpose
            && let Some(effect) = if_did
        {
            self.schedule_front(vec![crate::agenda::WorkItem::RunEffect {
                effect,
                frame: *frame,
            }]);
        }
        Ok(())
    }

    /// Close a nested resolution-time payment without treating it as the
    /// announcement frame of the currently resolving mana action. Optional
    /// costs keep every completed change on either branch; their replay
    /// records therefore join the enclosing mana action when one exists, or
    /// the containing payment frame otherwise.
    fn promote_nested_payment_frame(&mut self, abandon_fulfillment: bool) {
        let controller = self.payment.as_mut().expect("controller remains live");
        let mut child = controller
            .frames
            .pop()
            .expect("nested optional frame remains live");
        assert!(
            child.mana_action.is_none(),
            "a nested nonmana payment frame never owns a mana action"
        );
        if abandon_fulfillment {
            child.abandon_fulfillment();
        }
        let records = std::mem::take(&mut child.records);
        {
            let parent = controller
                .frames
                .last_mut()
                .expect("a nested optional frame has a parent");
            parent.working = child.working;
            parent.observations.extend(child.observations);
            parent.logical_objects.extend(child.logical_objects);
        }
        let active_action_is_submitted = controller
            .mana_actions
            .last()
            .is_some_and(mana::ManaAction::is_submitted);
        if !active_action_is_submitted
            && let Some(recording) = controller
                .frames
                .iter_mut()
                .rev()
                .find_map(|frame| frame.recording.as_mut())
        {
            recording.children.extend(records);
        } else if let Some(action) = controller.mana_actions.last_mut() {
            action.cost_records.extend(records);
        } else {
            controller
                .frames
                .last_mut()
                .expect("a nested optional frame has a parent")
                .records
                .extend(records);
        }
    }

    fn decline_payment(&mut self) -> Result<(), crate::decide::DecisionError> {
        let controller = self
            .payment
            .as_ref()
            .expect("a Payment decision has a controller");
        let frame = controller
            .frames
            .last()
            .expect("a Payment decision has a frame");
        let purpose = frame.purpose.clone();
        let payer = frame.payer;
        let subject = frame.subject;
        let nested = controller.frames.len() > 1;
        let nested_mana_action = frame.mana_action.is_some();

        // `submit_decision` routes DeclinePayment directly so that it remains
        // available while another decision temporarily covers a payment
        // prompt. Record an Optional branch choice here before its frame is
        // promoted; announcement decline is recovery metadata, not replayed
        // resolution input.
        if matches!(&purpose, PaymentPurpose::Optional { .. })
            || (nested && matches!(&purpose, PaymentPurpose::Announcement) && !nested_mana_action)
        {
            self.record_payment_decision(&crate::decide::Decision::Payment(
                PaymentCommand::DeclinePayment,
            ));
        }

        match purpose {
            PaymentPurpose::Optional { if_not, frame, .. } => {
                if nested {
                    self.promote_nested_payment_frame(true);
                } else {
                    let mut controller = self.payment.take().expect("controller remains live");
                    let mut child = controller.frames.pop().expect("optional root remains live");
                    assert!(
                        child.mana_action.is_none(),
                        "an optional payment frame never owns a mana action"
                    );
                    child.abandon_fulfillment();
                    self.committed = child.working;
                    self.clear_payment_metadata();
                }
                if let Some(effect) = if_not {
                    self.schedule_front(vec![crate::agenda::WorkItem::RunEffect {
                        effect,
                        frame: *frame,
                    }]);
                }
            }
            PaymentPurpose::Announcement => {
                if nested {
                    if nested_mana_action {
                        return self.decline_nested_mana_action(payer, subject);
                    }
                    return self.decline_nested_resolution_announcement(payer, subject);
                }
                let legal = self.legal_mana_reversal_sets();
                if legal.iter().any(|set| !set.is_empty()) {
                    self.pending = Some(crate::decide::DecisionPointKind::ChooseManaReversals(
                        crate::decide::pending::ChooseManaReversals {
                            player: payer,
                            legal,
                        },
                    ));
                    return Ok(());
                }
                return self.complete_root_announcement_decline(&[]);
            }
        }
        Ok(())
    }

    fn decline_nested_resolution_announcement(
        &mut self,
        payer: PlayerId,
        subject: PaymentSubject,
    ) -> Result<(), crate::decide::DecisionError> {
        let legal = self.legal_mana_reversal_sets();
        if legal.iter().any(|set| !set.is_empty()) {
            self.pending = Some(crate::decide::DecisionPointKind::ChooseManaReversals(
                crate::decide::pending::ChooseManaReversals {
                    player: payer,
                    legal,
                },
            ));
            return Ok(());
        }
        self.complete_nested_resolution_announcement_decline(&[], payer, subject)
    }

    fn complete_nested_resolution_announcement_decline(
        &mut self,
        reversed: &[ManaActionId],
        payer: PlayerId,
        subject: PaymentSubject,
    ) -> Result<(), crate::decide::DecisionError> {
        let reversible_actions: std::collections::HashSet<ManaActionId> = self
            .legal_mana_reversal_sets()
            .into_iter()
            .flatten()
            .collect();
        let (replay_frame, crossed_reversal, crossed_observation, announcement_if_not) = {
            let frame = self
                .payment
                .as_ref()
                .and_then(|controller| controller.frames.last())
                .expect("nested resolution cast has an announcement frame");
            let mut replay_frame = self.payment_replay_frame(frame);
            if let Some(record) = self.partial_fulfillment_record(frame)
                && (!record.reversal_barriers.is_empty()
                    || records_contain_mana_action(&record.children))
            {
                replay_frame.records.push(record);
            }
            (
                replay_frame,
                records_cross_reversal_barrier(&frame.records)
                    || frame.recording.as_ref().is_some_and(|record| {
                        !record.reversal_barriers.is_empty()
                            || records_cross_reversal_barrier(&record.children)
                    }),
                records_cross_observation_barrier(&frame.records)
                    || frame.recording.as_ref().is_some_and(|record| {
                        !record.observation_barriers.is_empty()
                            || records_cross_observation_barrier(&record.children)
                    }),
                frame.announcement_if_not.clone(),
            )
        };
        let retained: Vec<PaymentRecordId> = replay_frame
            .records
            .iter()
            .filter(|record| match record.command {
                ReplayCommand::ManaAbility { action, .. } => !reversed.contains(&action),
                ReplayCommand::Fulfill {
                    ref random_outcome, ..
                } => !record.reversal_barriers.is_empty() || random_outcome.is_some(),
            })
            .map(|record| record.id)
            .collect();
        let mut retained_records =
            retained_decline_records(&replay_frame.records, &retained, reversed);
        let mut forced = Vec::new();
        collect_forced_retained_record_ids(
            &retained_records,
            &reversible_actions,
            reversed,
            true,
            &mut forced,
        );
        let reconstructed = replay::reconstruct_decline(&replay_frame, &retained, reversed)
            .map_err(|error| crate::decide::DecisionError::Illegal {
                reason: format!("nested cast payment decline replay failed: {error}"),
            })?;
        for record in &mut retained_records {
            replay::prune_reversed_mana_children(record, reversed).map_err(|error| {
                crate::decide::DecisionError::Illegal {
                    reason: format!("nested cast payment metadata pruning failed: {error}"),
                }
            })?;
        }
        self.payment_logical_objects
            .clone_from(&reconstructed.logical_objects);
        self.activations.replace(reconstructed.activations);
        self.next_activation.set(reconstructed.next_activation);
        {
            let child = self
                .payment
                .as_mut()
                .and_then(|controller| controller.frames.last_mut())
                .expect("nested resolution cast frame remains live");
            child.working = reconstructed.working;
            child.records = retained_records;
            child.logical_objects = reconstructed.logical_objects;
        }
        self.promote_nested_payment_frame(false);
        if let Some((effect, frame)) = announcement_if_not {
            self.schedule_front(vec![crate::agenda::WorkItem::RunEffect {
                effect,
                frame: *frame,
            }]);
        }
        self.incidents
            .push(crate::state::EngineIncident::PaymentDeclined(
                crate::state::PaymentDeclined {
                    player: payer,
                    subject,
                    forced_retained_records: forced,
                    crossed_reversal_barrier: crossed_reversal,
                    crossed_observation_barrier: crossed_observation,
                },
            ));
        Ok(())
    }

    fn decline_nested_mana_action(
        &mut self,
        payer: PlayerId,
        subject: PaymentSubject,
    ) -> Result<(), crate::decide::DecisionError> {
        let legal = self.legal_nested_mana_reversal_sets();
        if legal.iter().any(|set| !set.is_empty()) {
            self.pending = Some(crate::decide::DecisionPointKind::ChooseManaReversals(
                crate::decide::pending::ChooseManaReversals {
                    player: payer,
                    legal,
                },
            ));
            return Ok(());
        }
        self.complete_nested_announcement_decline(&[], payer, subject)
    }

    fn complete_nested_announcement_decline(
        &mut self,
        reversed: &[ManaActionId],
        payer: PlayerId,
        subject: PaymentSubject,
    ) -> Result<(), crate::decide::DecisionError> {
        let reversible_actions: std::collections::HashSet<ManaActionId> = self
            .legal_nested_mana_reversal_sets()
            .into_iter()
            .flatten()
            .collect();
        let (replay_frame, action, crossed_observation) = {
            let controller = self.payment.as_ref().expect("controller remains live");
            let child = controller.frames.last().expect("nested frame remains live");
            let action_id = child
                .mana_action
                .expect("a nested announcement frame owns a mana action");
            let action = controller
                .mana_actions
                .last()
                .filter(|action| action.id == action_id)
                .cloned()
                .expect("a nested mana frame owns its active action");
            let mut forced_costs: Vec<TransactionRecord> = child
                .records
                .iter()
                .filter(|record| record.iou().is_some() && !record.reversal_barriers.is_empty())
                .cloned()
                .collect();
            let partial_fulfillment = self.partial_fulfillment_record(child);
            if let Some(record) = partial_fulfillment.as_ref()
                && !record.reversal_barriers.is_empty()
            {
                forced_costs.push(record.clone());
            }
            let crossed_observation = records_cross_observation_barrier(&child.records)
                || child.recording.as_ref().is_some_and(|record| {
                    !record.observation_barriers.is_empty()
                        || records_cross_observation_barrier(&record.children)
                })
                || !action.observation_barriers.is_empty();
            let mut replay_frame = self.payment_replay_frame(child);
            if !forced_costs.is_empty()
                || (action.is_submitted() && !action.reversal_barriers.is_empty())
                || (action.is_submitted() && !action.observation_barriers.is_empty())
            {
                replay_frame
                    .records
                    .push(self.partial_mana_record(&action, forced_costs));
            }
            if let Some(record) = partial_fulfillment
                && record.reversal_barriers.is_empty()
                && records_contain_mana_action(&record.children)
            {
                replay_frame.records.push(record);
            }
            (replay_frame, action, crossed_observation)
        };
        let retained: Vec<PaymentRecordId> = replay_frame
            .records
            .iter()
            .filter(|record| match record.command {
                ReplayCommand::ManaAbility { action, .. } => {
                    !reversed.contains(&action) || !record.reversal_barriers.is_empty()
                }
                ReplayCommand::Fulfill { .. } => false,
            })
            .map(|record| record.id)
            .collect();
        let mut retained_actions =
            retained_decline_records(&replay_frame.records, &retained, reversed);
        let mut forced = Vec::new();
        collect_forced_retained_record_ids(
            &retained_actions,
            &reversible_actions,
            reversed,
            true,
            &mut forced,
        );
        let crossed_reversal = !forced.is_empty();
        let reconstructed = replay::reconstruct_decline(&replay_frame, &retained, reversed)
            .map_err(|error| crate::decide::DecisionError::Illegal {
                reason: format!("nested payment decline replay failed: {error}"),
            })?;
        for record in &mut retained_actions {
            replay::prune_reversed_mana_children(record, reversed).map_err(|error| {
                crate::decide::DecisionError::Illegal {
                    reason: format!("nested payment decline metadata pruning failed: {error}"),
                }
            })?;
        }
        self.payment_logical_objects = reconstructed.logical_objects;
        self.activations.replace(reconstructed.activations);
        self.next_activation.set(reconstructed.next_activation);
        let mut rebuilt = reconstructed.working;
        rebuilt.pending = None;
        rebuilt.finish_mana_resolution_scope();
        rebuilt
            .resume_control()
            .map_err(|error| crate::decide::DecisionError::Illegal {
                reason: format!("nested payment decline could not resume its parent: {error}"),
            })?;
        let controller = self.payment.as_mut().expect("controller remains live");
        let child = controller.frames.pop().expect("nested frame remains live");
        let active = controller
            .mana_actions
            .pop()
            .expect("a nested mana frame owns an active action");
        assert_eq!(child.mana_action, Some(active.id));
        assert_eq!(active.id, action.id);
        let parent = controller
            .frames
            .last_mut()
            .expect("a nested mana frame has a parent");
        parent.working = rebuilt;
        parent.records.extend(retained_actions);
        self.refresh_payment_prompt();
        self.incidents
            .push(crate::state::EngineIncident::PaymentDeclined(
                crate::state::PaymentDeclined {
                    player: payer,
                    subject,
                    forced_retained_records: forced,
                    crossed_reversal_barrier: crossed_reversal,
                    crossed_observation_barrier: crossed_observation,
                },
            ));
        Ok(())
    }

    fn partial_mana_record(
        &self,
        action: &mana::ManaAction,
        forced_costs: Vec<TransactionRecord>,
    ) -> TransactionRecord {
        let children =
            if action.is_submitted() { action.cost_records.clone() } else { forced_costs };
        let facts: Vec<_> = if action.is_submitted() {
            action.facts.clone()
        } else {
            children
                .iter()
                .flat_map(|record| record.facts.iter().cloned())
                .collect()
        };
        let spent_mana: Vec<_> = children
            .iter()
            .flat_map(|record| record.spent_mana.iter().copied())
            .collect();
        let dependencies = self
            .payment
            .as_ref()
            .into_iter()
            .flat_map(|controller| controller.frames.iter().rev().skip(1))
            .flat_map(|frame| frame.records.iter())
            .filter(|record| {
                record
                    .produced_mana
                    .iter()
                    .any(|mana| spent_mana.contains(mana))
            })
            .map(|record| record.id)
            .collect();
        let mut reversal_barriers = action.reversal_barriers.clone();
        reversal_barriers.extend(
            children
                .iter()
                .flat_map(|record| record.reversal_barriers.iter().copied()),
        );
        let mut observation_barriers = action.observation_barriers.clone();
        observation_barriers.extend(
            children
                .iter()
                .flat_map(|record| record.observation_barriers.iter().copied()),
        );
        reversal_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
        reversal_barriers.dedup();
        observation_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
        observation_barriers.dedup();
        let source = self.payment_logical_source(action.source.source);
        let mut object_inputs =
            self.payment_record_object_inputs(&facts, std::collections::HashMap::new());
        if matches!(source, LogicalObject::Created { .. }) {
            object_inputs.insert(action.source.object, source);
        }
        let produced_mana = facts
            .iter()
            .filter_map(|fact| match fact {
                crate::event::GameEvent::ManaAdded(event) => Some(event.units.iter().copied().map(
                    |id| replay::QualifiedManaId {
                        player: event.player,
                        id,
                    },
                )),
                _ => None,
            })
            .flatten()
            .collect();
        TransactionRecord {
            id: action.record,
            command: ReplayCommand::ManaAbility {
                action: action.id,
                source,
                ability: action.ability,
                submitted: action.is_submitted(),
                completed: false,
            },
            transcript: action.transcript.clone(),
            object_inputs,
            children,
            facts,
            produced_mana,
            spent_mana,
            dependencies,
            reversal_barriers,
            observation_barriers,
        }
    }

    fn partial_fulfillment_record(&self, frame: &PaymentFrame) -> Option<TransactionRecord> {
        let draft = frame.recording.as_ref()?.clone();
        // A suspended fulfillment owns only facts recorded while its frame is
        // topmost. Nested mana actions are child records with their own fact
        // and mana namespaces; folding the whole history suffix into this
        // parent would make a child's mana appear to have two producers.
        let facts = draft.facts;
        let mut reversal_barriers = draft.reversal_barriers;
        let mut observation_barriers = draft.observation_barriers;
        reversal_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
        reversal_barriers.dedup();
        observation_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
        observation_barriers.dedup();
        let produced_mana = facts
            .iter()
            .filter_map(|fact| match fact {
                crate::event::GameEvent::ManaAdded(event) => Some(event.units.iter().copied().map(
                    |id| replay::QualifiedManaId {
                        player: event.player,
                        id,
                    },
                )),
                _ => None,
            })
            .flatten()
            .collect();
        let object_inputs = self.payment_record_object_inputs(&facts, draft.object_inputs);
        Some(TransactionRecord {
            id: draft.id,
            command: draft.command,
            transcript: draft.transcript,
            object_inputs,
            children: draft.children,
            facts,
            produced_mana,
            spent_mana: draft.spent_mana,
            dependencies: Vec::new(),
            reversal_barriers,
            observation_barriers,
        })
    }

    fn legal_nested_mana_reversal_sets(&self) -> Vec<Vec<ManaActionId>> {
        let Some(controller) = self.payment.as_ref() else {
            return Vec::new();
        };
        let Some(child) = controller.frames.last() else {
            return Vec::new();
        };
        let Some(action) = child.mana_action.and_then(|id| {
            controller
                .mana_actions
                .last()
                .filter(|action| action.id == id)
        }) else {
            return Vec::new();
        };
        let mut forced_costs = child
            .records
            .iter()
            .filter(|record| record.iou().is_some() && !record.reversal_barriers.is_empty())
            .cloned()
            .collect::<Vec<_>>();
        let partial_fulfillment = self.partial_fulfillment_record(child);
        if let Some(record) = partial_fulfillment.as_ref()
            && !record.reversal_barriers.is_empty()
        {
            forced_costs.push(record.clone());
        }
        let mut replay_frame = self.payment_replay_frame(child);
        if !forced_costs.is_empty() {
            replay_frame
                .records
                .push(self.partial_mana_record(action, forced_costs));
        }
        if let Some(record) = partial_fulfillment
            && record.reversal_barriers.is_empty()
            && records_contain_mana_action(&record.children)
        {
            replay_frame.records.push(record);
        }
        let mut candidates = Vec::new();
        collect_mana_reversal_candidates(&replay_frame.records, &mut candidates);
        let combinations = 1usize
            .checked_shl(u32::try_from(candidates.len()).unwrap_or(u32::MAX))
            .unwrap_or(0);
        let mut legal = Vec::new();
        for mask in 0..combinations {
            let reversed: Vec<ManaActionId> = candidates
                .iter()
                .enumerate()
                .filter_map(|(index, &action)| ((mask & (1 << index)) != 0).then_some(action))
                .collect();
            let retained: Vec<PaymentRecordId> = replay_frame
                .records
                .iter()
                .filter(|record| match record.command {
                    ReplayCommand::ManaAbility { action, .. } => {
                        !reversed.contains(&action) || !record.reversal_barriers.is_empty()
                    }
                    ReplayCommand::Fulfill { .. } => false,
                })
                .map(|record| record.id)
                .collect();
            if mana_reversal_set_preserves_nested_units(&replay_frame.records, &reversed)
                && replay::reconstruct_decline(&replay_frame, &retained, &reversed).is_ok()
            {
                legal.push(reversed);
            }
        }
        legal
    }

    fn legal_mana_reversal_sets(&self) -> Vec<Vec<ManaActionId>> {
        let Some(frame) = self
            .payment
            .as_ref()
            .and_then(|controller| controller.frames.last())
        else {
            return Vec::new();
        };
        let mut replay_frame = self.payment_replay_frame(frame);
        if let Some(record) = self.partial_fulfillment_record(frame)
            && (!record.reversal_barriers.is_empty()
                || records_contain_mana_action(&record.children))
        {
            replay_frame.records.push(record);
        }
        let mut candidates = Vec::new();
        collect_mana_reversal_candidates(&replay_frame.records, &mut candidates);
        let mut legal = Vec::new();
        let combinations = 1usize
            .checked_shl(u32::try_from(candidates.len()).unwrap_or(u32::MAX))
            .unwrap_or(0);
        for mask in 0..combinations {
            let reversed: Vec<ManaActionId> = candidates
                .iter()
                .enumerate()
                .filter_map(|(index, &action)| ((mask & (1 << index)) != 0).then_some(action))
                .collect();
            let retained: Vec<PaymentRecordId> = replay_frame
                .records
                .iter()
                .filter(|record| match record.command {
                    ReplayCommand::ManaAbility { action, .. } => !reversed.contains(&action),
                    ReplayCommand::Fulfill {
                        ref random_outcome, ..
                    } => !record.reversal_barriers.is_empty() || random_outcome.is_some(),
                })
                .map(|record| record.id)
                .collect();
            if mana_reversal_set_preserves_nested_units(&replay_frame.records, &reversed)
                && replay::reconstruct_decline(&replay_frame, &retained, &reversed).is_ok()
            {
                legal.push(reversed);
            }
        }
        legal
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "a submitted reversal set is an owned decision payload"
    )]
    pub(crate) fn submit_mana_reversals(
        &mut self,
        actions: Vec<ManaActionId>,
    ) -> Result<(), crate::decide::DecisionError> {
        let nested = self
            .payment
            .as_ref()
            .is_some_and(|controller| controller.frames.len() > 1);
        if nested {
            let frame = self
                .payment
                .as_ref()
                .and_then(|controller| controller.frames.last())
                .expect("a nested reversal decision has a payment frame");
            if frame.mana_action.is_some() {
                self.complete_nested_announcement_decline(&actions, frame.payer, frame.subject)
            } else {
                self.complete_nested_resolution_announcement_decline(
                    &actions,
                    frame.payer,
                    frame.subject,
                )
            }
        } else {
            self.complete_root_announcement_decline(&actions)
        }
    }

    fn complete_root_announcement_decline(
        &mut self,
        reversed: &[ManaActionId],
    ) -> Result<(), crate::decide::DecisionError> {
        let reversible_actions: std::collections::HashSet<ManaActionId> = self
            .legal_mana_reversal_sets()
            .into_iter()
            .flatten()
            .collect();
        let (
            payer,
            subject,
            retained,
            forced,
            crossed_reversal,
            crossed_observation,
            rebuilt,
            activations,
            next_activation,
            announcement_if_not,
        ) = {
            let frame = self
                .payment
                .as_ref()
                .and_then(|controller| controller.frames.last())
                .expect("announcement decline has an active frame");
            let mut replay_frame = self.payment_replay_frame(frame);
            if let Some(record) = self.partial_fulfillment_record(frame)
                && (!record.reversal_barriers.is_empty()
                    || records_contain_mana_action(&record.children))
            {
                replay_frame.records.push(record);
            }
            let retained: Vec<PaymentRecordId> = replay_frame
                .records
                .iter()
                .filter(|record| match record.command {
                    ReplayCommand::ManaAbility { action, .. } => !reversed.contains(&action),
                    ReplayCommand::Fulfill {
                        ref random_outcome, ..
                    } => !record.reversal_barriers.is_empty() || random_outcome.is_some(),
                })
                .map(|record| record.id)
                .collect();
            let retained_records =
                retained_decline_records(&replay_frame.records, &retained, reversed);
            let mut forced = Vec::new();
            collect_forced_retained_record_ids(
                &retained_records,
                &reversible_actions,
                reversed,
                true,
                &mut forced,
            );
            let crossed_reversal = records_cross_reversal_barrier(&frame.records)
                || frame.recording.as_ref().is_some_and(|record| {
                    !record.reversal_barriers.is_empty()
                        || records_cross_reversal_barrier(&record.children)
                });
            let crossed_observation = records_cross_observation_barrier(&frame.records)
                || frame.recording.as_ref().is_some_and(|record| {
                    !record.observation_barriers.is_empty()
                        || records_cross_observation_barrier(&record.children)
                });
            let rebuilt = replay::reconstruct_decline(&replay_frame, &retained, reversed).map_err(
                |error| crate::decide::DecisionError::Illegal {
                    reason: format!("payment decline replay failed: {error}"),
                },
            )?;
            let activations = rebuilt.activations;
            let next_activation = rebuilt.next_activation;
            (
                frame.payer,
                frame.subject,
                retained,
                forced,
                crossed_reversal,
                crossed_observation,
                rebuilt.working,
                activations,
                next_activation,
                frame.announcement_if_not.clone(),
            )
        };
        let _ = retained;
        self.committed = rebuilt;
        self.activations.replace(activations);
        self.next_activation.set(next_activation);
        self.payment = None;
        self.clear_payment_metadata();
        if let Some((effect, frame)) = announcement_if_not {
            self.schedule_front(vec![crate::agenda::WorkItem::RunEffect {
                effect,
                frame: *frame,
            }]);
        }
        self.incidents
            .push(crate::state::EngineIncident::PaymentDeclined(
                crate::state::PaymentDeclined {
                    player: payer,
                    subject,
                    forced_retained_records: forced,
                    crossed_reversal_barrier: crossed_reversal,
                    crossed_observation_barrier: crossed_observation,
                },
            ));
        Ok(())
    }

    /// The active frame's internal suspension state, when payment exists.
    #[must_use]
    pub fn payment_progress(&self) -> Option<PaymentProgress> {
        self.payment
            .as_ref()
            .and_then(|controller| controller.frames.last())
            .map(|frame| frame.progress.clone())
    }

    /// Number of active speculative payment frames, including nested mana
    /// abilities.
    #[must_use]
    pub fn payment_depth(&self) -> usize {
        self.payment
            .as_ref()
            .map_or(0, |controller| controller.frames.len())
    }

    #[must_use]
    pub fn payment_records(&self) -> Option<&[TransactionRecord]> {
        self.payment
            .as_ref()
            .and_then(|controller| controller.frames.last())
            .map(|frame| frame.records.as_slice())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PaymentLockError {
    #[error("a multi-way or variable mana symbol was not concretized before payment locking")]
    UnconcretizedManaSymbol,
    #[error("ManaCostOf resolved to an object without a mana cost")]
    MissingManaCost,
    #[error("an expanded cost component survived lowering")]
    ExpandedCost,
}

/// Materialize the locked cost as stable runtime IOUs. The caller supplies the
/// announce/resolution frame so X and reference-bearing counts freeze against
/// the same bindings as the cost itself.
///
/// # Errors
///
/// Returns [`PaymentLockError`] if a multi-way or variable mana symbol has not
/// been concretized, `ManaCostOf` names a non-card object, or lowering left an
/// expanded cost wrapper at the runnable boundary.
pub fn lock_cost(
    state: &GameState,
    payer: PlayerId,
    subject: PaymentSubject,
    frame: &ExecutionFrame,
    cost: &Cost,
    pay_pips: &[(PipClass, PayAct)],
) -> Result<LockedPayment, PaymentLockError> {
    let mut builder = LockBuilder {
        state,
        frame,
        pay_pips,
        ious: Vec::new(),
        next_iou: 0,
        has_mana_payment: false,
        defer_next_action: false,
    };
    builder.lock_components(cost)?;
    let entry_stage = if builder.has_mana_payment {
        PaymentStage::PrePayment
    } else if builder.ious.is_empty() {
        PaymentStage::Ready
    } else {
        PaymentStage::Paying
    };
    Ok(LockedPayment {
        payer,
        subject,
        frame: frame.clone(),
        ious: builder.ious,
        stage: entry_stage,
        has_mana_payment: builder.has_mana_payment,
    })
}

struct LockBuilder<'a> {
    state: &'a GameState,
    frame: &'a ExecutionFrame,
    pay_pips: &'a [(PipClass, PayAct)],
    ious: Vec<PaymentIou>,
    next_iou: u64,
    has_mana_payment: bool,
    /// A deferred sample or library search carries its tier onto the action
    /// that consumes the register it writes.
    defer_next_action: bool,
}

impl LockBuilder<'_> {
    fn lock_components(&mut self, cost: &Cost) -> Result<(), PaymentLockError> {
        for component in cost {
            match component {
                CostComponent::Mana(mana) => self.lock_mana(mana)?,
                CostComponent::ManaCostOf(reference) => {
                    let object = self.state.eval_reference(reference, self.frame);
                    if self
                        .state
                        .objects
                        .get(object)
                        .and_then(crate::object::GameObject::card_id)
                        .is_none()
                    {
                        return Err(PaymentLockError::MissingManaCost);
                    }
                    let mana = self
                        .state
                        .mana_cost(object)
                        .ok_or(PaymentLockError::MissingManaCost)?;
                    self.lock_mana(&mana)?;
                }
                CostComponent::Tap => self.push(IouKind::Tap, Vec::new()),
                CostComponent::Untap => self.push(IouKind::Untap, Vec::new()),
                CostComponent::Act { dest, action } => self.lock_action(*dest, action),
                CostComponent::Cost(inner) => self.lock_components(inner)?,
                CostComponent::TapTotal {
                    stat,
                    cmp,
                    count,
                    filter,
                } => {
                    let count = self.state.eval_count(count, self.frame);
                    self.push(
                        IouKind::TapTotal {
                            stat: *stat,
                            cmp: *cmp,
                            count,
                            filter: Arc::clone(filter),
                        },
                        Vec::new(),
                    );
                }
                CostComponent::Choose(choice) => {
                    self.push(IouKind::Choose(choice.clone()), Vec::new());
                }
                CostComponent::Sample(sample) => {
                    self.push(IouKind::Sample(sample.clone()), Vec::new());
                    self.defer_next_action = true;
                }
                CostComponent::Search(search) => {
                    self.push(IouKind::Search(search.clone()), Vec::new());
                    self.defer_next_action |= search.from.contains(&deckmaste_core::Zone::Library);
                }
                CostComponent::Let(binding) => {
                    self.push(IouKind::Let(binding.clone()), Vec::new());
                }
            }
        }
        Ok(())
    }

    fn lock_action(&mut self, dest: Option<deckmaste_core::DefId>, action: &RunnableCostAction) {
        let deferred = std::mem::take(&mut self.defer_next_action);
        let kind = match action.as_action() {
            Action::ChangeLife(reference, LifeOp::Down(count))
                if dest.is_none() && reference == &Reference::controller_parameter() =>
            {
                IouKind::PayLife(self.state.eval_count(count, self.frame))
            }
            _ => IouKind::Act {
                dest,
                action: action.clone(),
            },
        };
        self.push_deferred(kind, Vec::new(), deferred);
    }

    fn lock_mana(&mut self, mana: &ManaCost) -> Result<(), PaymentLockError> {
        // An empty ManaCost is what remains when every Phyrexian symbol was
        // announced as life. `{0}` is nonempty (`Generic(0)`) and therefore
        // still grants the project's explicit prepayment window.
        self.has_mana_payment |= !mana.is_empty();
        for symbol in mana.iter() {
            match symbol {
                ManaSymbol::Simple(SimpleManaSymbol::Generic(amount)) => {
                    for _ in 0..*amount {
                        self.push_mana_pip(ManaPip::Generic);
                    }
                }
                ManaSymbol::Simple(SimpleManaSymbol::Specific(kind)) => match kind.color() {
                    Some(color) => self.push_mana_pip(ManaPip::Colored(color)),
                    None => self.push_mana_pip(ManaPip::Colorless),
                },
                ManaSymbol::Snow => self.push_mana_pip(ManaPip::Snow),
                ManaSymbol::Variable | ManaSymbol::Hybrid(..) | ManaSymbol::Phyrexian(..) => {
                    return Err(PaymentLockError::UnconcretizedManaSymbol);
                }
            }
        }
        Ok(())
    }

    fn push_mana_pip(&mut self, pip: ManaPip) {
        let alternatives = self
            .pay_pips
            .iter()
            .filter(|(class, _)| pip.accepts_alternative(*class))
            .map(|(_, act)| act.clone())
            .collect();
        self.push(IouKind::ManaPip(pip), alternatives);
    }

    fn push(&mut self, kind: IouKind, alternatives: Vec<PayAct>) {
        self.push_deferred(kind, alternatives, false);
    }

    fn push_deferred(&mut self, kind: IouKind, alternatives: Vec<PayAct>, deferred: bool) {
        let id = IouId(self.next_iou);
        self.next_iou = self
            .next_iou
            .checked_add(1)
            .expect("payment IOU id overflow");
        self.ious.push(PaymentIou {
            id,
            kind,
            alternatives,
            deferred,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_core::Color;
    use deckmaste_core::Cost;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Count;
    use deckmaste_core::EventFilter;
    use deckmaste_core::Instruction;
    use deckmaste_core::LifeOp;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaRider;
    use deckmaste_core::May;
    use deckmaste_core::PayAct;
    use deckmaste_core::PipClass;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Replacement;
    use deckmaste_core::StaticSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use super::*;
    use crate::CostOptionChoices;
    use crate::Decision;
    use crate::DecisionPointKind;
    use crate::GameConfig;
    use crate::GameState;
    use crate::ManaProvenance;
    use crate::ObjectSource;
    use crate::PlayerConfig;
    use crate::PlayerId;
    use crate::StartingPlayer;
    use crate::StepOutcome;
    use crate::SymbolChoice;
    use crate::concretize;

    fn fixture(printed_mana: &str) -> (GameState, PlayerId, crate::ObjectId) {
        let payer = PlayerId(0);
        let card = Arc::new(Card::Normal(CardFace {
            name: "Payment subject".into(),
            mana_cost: printed_mana.parse().expect("test mana cost parses"),
            ..CardFace::default()
        }));
        let state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: vec![card] },
                PlayerConfig { deck: vec![] },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(payer),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let subject = state.zones.hands[payer.index()][0];
        (state, payer, subject)
    }

    fn cost(components: Vec<CostComponent>) -> Cost {
        Cost(components.into())
    }

    fn may_pay(
        cost: Cost,
        if_did: Option<Arc<Instruction>>,
        if_not: Option<Arc<Instruction>>,
    ) -> Instruction {
        Instruction::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(Instruction::act(Action::Pay(cost))),
            if_did,
            if_not,
        })
    }

    fn gain_life(amount: u32) -> Arc<Instruction> {
        Arc::new(Instruction::act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::Literal(amount)),
        )))
    }

    fn run_until_payment(state: &mut GameState) {
        for _ in 0..20 {
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => return,
                other => panic!("expected payment to resume, got {other:?}"),
            }
        }
        panic!("payment did not resume");
    }

    fn lock_components(printed_mana: &str, components: Vec<CostComponent>) -> LockedPayment {
        let (state, payer, subject) = fixture(printed_mana);
        lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &state.frame(subject, payer),
            &cost(components),
            &[],
        )
        .expect("test cost locks")
    }

    fn lock_test_cost(mana: &str) -> LockedPayment {
        lock_components(
            "",
            vec![CostComponent::Mana(
                mana.parse::<ManaCost>().expect("test mana cost parses"),
            )],
        )
    }

    #[test]
    fn random_outcome_mana_records_are_not_reversal_candidates() {
        let record = TransactionRecord {
            id: PaymentRecordId(0),
            command: ReplayCommand::ManaAbility {
                action: ManaActionId(0),
                source: LogicalObject::Player(PlayerId(0)),
                ability: 0,
                submitted: true,
                completed: true,
            },
            transcript: DecisionTranscript::default(),
            object_inputs: std::collections::HashMap::new(),
            children: Vec::new(),
            facts: Vec::new(),
            produced_mana: Vec::new(),
            spent_mana: Vec::new(),
            dependencies: Vec::new(),
            reversal_barriers: Vec::new(),
            observation_barriers: vec![ObservationBarrier::RandomOutcome],
        };
        let mut candidates = Vec::new();

        collect_mana_reversal_candidates(std::slice::from_ref(&record), &mut candidates);

        assert!(candidates.is_empty());

        let mut disclosure = record;
        disclosure.observation_barriers = vec![ObservationBarrier::HiddenZoneDisclosure];
        collect_mana_reversal_candidates(&[disclosure], &mut candidates);
        assert!(candidates.is_empty());
    }

    #[test]
    fn generic_mana_expands_to_stable_pips() {
        let locked = lock_test_cost("{2}{B}{G}");
        let pips = locked.mana_pips();

        assert_eq!(pips.len(), 4);
        assert_eq!(
            pips.iter().map(|iou| iou.id).collect::<HashSet<_>>().len(),
            4
        );
    }

    #[test]
    fn exact_coverage_is_atomic_and_unique() {
        let (mut state, payer, subject) = fixture("");
        let green = state.player_mut(payer).mana_pool.add(
            Color::Green.into(),
            2,
            ManaProvenance::default(),
        );
        let locked = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &state.frame(subject, payer),
            &cost(vec![CostComponent::Mana("{1}{G}".parse().unwrap())]),
            &[],
        )
        .unwrap();
        let pips = locked.mana_pips();
        let mut coverage = ManaCoverage::empty();
        coverage.insert(pips[0].id, ManaPayment::Floating(green[0]));
        coverage.insert(pips[1].id, ManaPayment::Floating(green[0]));
        let before = state.player(payer).mana_pool.clone();

        assert!(locked.validate_coverage(&state, &coverage).is_err());
        assert_eq!(state.player(payer).mana_pool, before);
    }

    #[test]
    fn zero_and_nonmana_costs_choose_the_required_entry_stage() {
        assert_eq!(lock_test_cost("{0}").stage, PaymentStage::PrePayment);
        assert_eq!(
            lock_components("", vec![CostComponent::Tap]).stage,
            PaymentStage::Paying
        );
        assert_eq!(lock_components("", vec![]).stage, PaymentStage::Ready);
    }

    #[test]
    fn mana_cost_of_and_pay_pips_become_concrete_pip_options() {
        let (state, payer, subject) = fixture("{2}{G}");
        let alternative = PayAct::TapToPay(Predicate::Any);
        let locked = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &state.frame(subject, payer),
            &cost(vec![CostComponent::ManaCostOf(Reference::Reg(
                deckmaste_core::RefId(0),
            ))]),
            &[
                (PipClass::Generic, alternative.clone()),
                (PipClass::Colored(Color::Green), alternative.clone()),
            ],
        )
        .unwrap();

        let pips = locked.mana_pips();
        assert_eq!(pips.len(), 3);
        assert!(
            pips.iter()
                .all(|pip| pip.alternatives.contains(&alternative))
        );
    }

    #[test]
    fn announced_phyrexian_life_skips_prepayment_but_snow_requires_provenance() {
        let original: ManaCost = "{G/P}".parse().unwrap();
        let (mana, mut verbs) = concretize(
            &original,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Life],
            },
        )
        .unwrap();
        let mut components = vec![CostComponent::Mana(mana)];
        components.append(&mut verbs);
        let life = lock_components("", components);
        assert_eq!(life.stage, PaymentStage::Paying);
        assert!(matches!(life.ious[0].kind, IouKind::PayLife(2)));

        let (mut state, payer, subject) = fixture("");
        let plain = state.player_mut(payer).mana_pool.add(
            Color::Green.into(),
            1,
            ManaProvenance::default(),
        )[0];
        let snow = state.player_mut(payer).mana_pool.add_riders(
            Color::Green.into(),
            1,
            &[ManaRider::Snow],
            ManaProvenance::default(),
        )[0];
        let locked = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &state.frame(subject, payer),
            &cost(vec![CostComponent::Mana("{S}".parse().unwrap())]),
            &[],
        )
        .unwrap();
        let pip = locked.mana_pips()[0].id;
        let mut plain_coverage = ManaCoverage::empty();
        plain_coverage.insert(pip, ManaPayment::Floating(plain));
        let mut snow_coverage = ManaCoverage::empty();
        snow_coverage.insert(pip, ManaPayment::Floating(snow));

        assert!(locked.validate_coverage(&state, &plain_coverage).is_err());
        assert!(locked.validate_coverage(&state, &snow_coverage).is_ok());
    }

    #[test]
    fn hybrid_reading_is_locked_before_iou_creation() {
        let original: ManaCost = "{W/U}".parse().unwrap();
        let (mana, verbs) = concretize(
            &original,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Mana(Color::White.into())],
            },
        )
        .unwrap();
        assert!(verbs.is_empty());
        let locked = lock_components("", vec![CostComponent::Mana(mana)]);

        assert!(matches!(
            locked.mana_pips()[0].kind,
            IouKind::ManaPip(ManaPip::Colored(Color::White))
        ));
    }

    #[test]
    fn spend_restrictions_and_pay_pips_objects_are_validated() {
        let (mut state, payer, subject) = fixture("");
        let restricted = state.player_mut(payer).mana_pool.add_riders(
            Color::Green.into(),
            1,
            &[ManaRider::SpendOnly(Predicate::Not(Arc::new(
                Predicate::Any,
            )))],
            ManaProvenance::default(),
        )[0];
        let green = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &state.frame(subject, payer),
            &cost(vec![CostComponent::Mana("{G}".parse().unwrap())]),
            &[],
        )
        .unwrap();
        let mut restricted_coverage = ManaCoverage::empty();
        restricted_coverage.insert(green.mana_pips()[0].id, ManaPayment::Floating(restricted));
        assert!(
            green
                .validate_coverage(&state, &restricted_coverage)
                .is_err()
        );

        let resource_card = state.cards.push(
            Arc::new(Card::Normal(CardFace {
                name: "Payment resource".into(),
                ..CardFace::default()
            })),
            payer,
        );
        let resource = state.objects.mint(
            ObjectSource::Card(resource_card),
            payer,
            Some(deckmaste_core::Zone::Battlefield),
        );
        state.zones.battlefield.push(resource);
        let generic = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &state.frame(subject, payer),
            &cost(vec![CostComponent::Mana("{1}".parse().unwrap())]),
            &[(PipClass::Generic, PayAct::TapToPay(Predicate::Any))],
        )
        .unwrap();
        let mut pay_pips = ManaCoverage::empty();
        pay_pips.insert(
            generic.mana_pips()[0].id,
            ManaPayment::PayPips {
                object: resource,
                alternative: 0,
            },
        );
        assert!(generic.validate_coverage(&state, &pay_pips).is_ok());

        state.objects.obj_mut(resource).tapped = true;
        assert!(generic.validate_coverage(&state, &pay_pips).is_err());
    }

    #[test]
    fn optional_decline_keeps_pre_payment_mana_and_runs_if_not_without_incident() {
        let (mut state, payer, subject) = fixture("");
        state.agenda.clear();
        let before_life = state.player(payer).life;
        let frame = state.frame(subject, payer);
        state.run_effect(
            may_pay(
                cost(vec![CostComponent::Mana("{G}".parse().unwrap())]),
                Some(gain_life(10)),
                Some(gain_life(1)),
            ),
            &frame,
        );

        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("May(Pay) should open an optional payment frame");
        };
        assert_eq!(prompt.stage, PaymentStage::PrePayment);
        let green = state.player_mut(payer).mana_pool.add(
            Color::Green.into(),
            1,
            ManaProvenance::default(),
        )[0];

        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();

        assert_eq!(state.payment_depth(), 0);
        assert!(state.player(payer).mana_pool.get(green).is_some());
        assert!(state.incidents().is_empty());
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
        assert_eq!(state.player(payer).life, before_life + 1);
    }

    #[test]
    fn optional_submit_commits_cost_and_runs_if_did() {
        let (mut state, payer, subject) = fixture("");
        state.agenda.clear();
        let before_life = state.player(payer).life;
        let frame = state.frame(subject, payer);
        state.run_effect(
            may_pay(
                cost(vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(2)),
                ))]),
                Some(gain_life(10)),
                Some(gain_life(1)),
            ),
            &frame,
        );
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("May(Pay) should open an optional payment frame");
        };
        assert_eq!(prompt.stage, PaymentStage::Paying);
        let iou = prompt.outstanding[0].id;

        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::PayLife,
            }))
            .unwrap();
        run_until_payment(&mut state);
        assert_eq!(state.player(payer).life, before_life - 2);
        assert_eq!(state.committed().players[payer.index()].life, before_life);

        state
            .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
            .unwrap();
        assert_eq!(state.payment_depth(), 0);
        assert_eq!(
            state.committed().players[payer.index()].life,
            before_life - 2
        );
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
        assert_eq!(state.player(payer).life, before_life + 8);
        assert!(state.incidents().is_empty());
    }

    #[test]
    fn nested_optional_submit_promotes_its_records_to_the_parent_transaction() {
        let (mut state, payer, subject) = fixture("");
        state.agenda.clear();
        state.begin_test_frame();
        let before_life = state.player(payer).life;
        state.run_effect(
            may_pay(
                cost(vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(2)),
                ))]),
                None,
                None,
            ),
            &state.frame(subject, payer),
        );
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("nested May(Pay) should open an optional payment frame");
        };
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: prompt.outstanding[0].id,
                witness: FulfillmentWitness::PayLife,
            }))
            .unwrap();
        run_until_payment(&mut state);

        state
            .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
            .unwrap();

        assert_eq!(state.payment_depth(), 1);
        assert_eq!(state.player(payer).life, before_life - 2);
        assert_eq!(
            state.payment.as_ref().unwrap().frames[0].records.len(),
            1,
            "the enclosing transaction must retain the optional payment trace",
        );
    }

    #[test]
    fn nested_optional_decline_keeps_completed_records_in_the_parent_transaction() {
        let (mut state, payer, subject) = fixture("");
        state.agenda.clear();
        state.begin_test_frame();
        let before_life = state.player(payer).life;
        state.run_effect(
            may_pay(
                cost(vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(2)),
                ))]),
                None,
                None,
            ),
            &state.frame(subject, payer),
        );
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("nested May(Pay) should open an optional payment frame");
        };
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: prompt.outstanding[0].id,
                witness: FulfillmentWitness::PayLife,
            }))
            .unwrap();
        run_until_payment(&mut state);

        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();

        assert_eq!(state.payment_depth(), 1);
        assert_eq!(state.player(payer).life, before_life - 2);
        assert_eq!(
            state.payment.as_ref().unwrap().frames[0].records.len(),
            1,
            "optional decline keeps completed changes and their replay trace",
        );
    }

    #[test]
    fn optional_decline_abandons_a_suspended_fulfillment_continuation() {
        let (mut state, payer, subject) = fixture("");
        state.agenda.clear();
        state.zones.hands[payer.index()].retain(|&object| object != subject);
        state.objects.obj_mut(subject).zone = Some(Zone::Battlefield);
        state.objects.obj_mut(subject).summoning_sick = false;
        state.zones.battlefield.push(subject);

        let replacement = Replacement::Instead {
            would: EventFilter::ZoneChange {
                what: Predicate::Any,
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            },
            instead: Instruction::Sequentially(Arc::from([])),
        };
        let shield_card = state.cards.push(
            Arc::new(Card::Normal(CardFace {
                name: "Two replacement shields".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![
                    deckmaste_core::Ability::r#static(StaticSpec::Replacement(Arc::new(
                        replacement.clone(),
                    ))),
                    deckmaste_core::Ability::r#static(StaticSpec::Replacement(Arc::new(
                        replacement,
                    ))),
                ],
                ..CardFace::default()
            })),
            payer,
        );
        let shield = state.objects.mint(
            ObjectSource::Card(shield_card),
            payer,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(shield);

        let life_before = state.player(payer).life;
        let frame = state.frame(subject, payer);
        state.run_effect(
            may_pay(
                cost(vec![CostComponent::do_action(Action::Sacrifice(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Reference::Reg(deckmaste_core::RefId(0)),
                ))]),
                None,
                Some(gain_life(1)),
            ),
            &frame,
        );
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("May(Pay) should open an optional payment frame");
        };
        let iou = prompt.outstanding[0].id;
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Bound,
            }))
            .unwrap();
        loop {
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::ChooseReplacement(_)) => break,
                other => panic!("expected a suspended replacement choice, got {other:?}"),
            }
        }

        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();
        for _ in 0..8 {
            if state.agenda.is_empty() {
                break;
            }
            assert!(matches!(state.step(), StepOutcome::Progress(_)));
        }

        assert_eq!(state.payment_depth(), 0);
        assert!(state.zones.battlefield.contains(&subject));
        assert_eq!(state.player(payer).life, life_before + 1);
        assert!(state.incidents().is_empty());
    }

    #[test]
    fn concession_from_a_nested_payment_commits_the_active_image_and_terminates() {
        let (mut state, payer, subject) = fixture("");
        state.agenda.clear();
        state.begin_test_frame();
        let frame = state.frame(subject, payer);
        state.run_effect(
            may_pay(
                cost(vec![CostComponent::Mana("{G}".parse().unwrap())]),
                None,
                None,
            ),
            &frame,
        );
        assert_eq!(state.payment_depth(), 2);
        let green = state.player_mut(payer).mana_pool.add(
            Color::Green.into(),
            1,
            ManaProvenance::default(),
        )[0];

        state
            .submit_decision(Decision::Act(crate::decide::Action::Concede))
            .unwrap();

        assert_eq!(state.payment_depth(), 0);
        assert!(
            state.committed().players[payer.index()]
                .mana_pool
                .get(green)
                .is_some()
        );
        let mut ended = false;
        for _ in 0..4 {
            if matches!(state.step(), StepOutcome::GameOver(_)) {
                ended = true;
                break;
            }
        }
        assert!(
            ended,
            "concession should terminate through normal game-end processing"
        );
    }

    /// Put `n` vanilla creatures onto the battlefield under `owner`'s control.
    fn creatures_on_field(
        state: &mut GameState,
        owner: PlayerId,
        n: usize,
    ) -> Vec<crate::ObjectId> {
        (0..n)
            .map(|index| {
                let cid = state.cards.push(
                    Arc::new(Card::Normal(CardFace {
                        name: format!("Sacrificial creature {index}").into(),
                        types: vec![Type::Creature.def()],
                        ..CardFace::default()
                    })),
                    owner,
                );
                let id =
                    state
                        .objects
                        .mint(ObjectSource::Card(cid), owner, Some(Zone::Battlefield));
                state.zones.battlefield.push(id);
                id
            })
            .collect()
    }

    /// One payment-time object choice over the creatures on the battlefield.
    fn choose_creatures(quantity: deckmaste_core::Quantity) -> CostComponent {
        CostComponent::Choose(deckmaste_core::Choose {
            dest: deckmaste_core::DefId(9),
            by: Reference::controller_parameter(),
            quantity,
            filter: Arc::new(deckmaste_core::Region::candidate(Predicate::And(
                vec![
                    Predicate::creature(),
                    Predicate::State(deckmaste_core::StatePredicate::InZone(Zone::Battlefield)),
                ]
                .into(),
            ))),
        })
    }

    /// [CR#107.1c]: "If a rule or ability instructs a player to choose 'any
    /// number,' that player may choose any positive number or zero." A cost
    /// choice with no lower bound is therefore COMPLETELY paid by choosing
    /// nothing — that is a whole payment, not a partial one ([CR#601.2h]), and
    /// no rule requires a payer to spend more than the cost demands. The
    /// automatic payer must not sacrifice a permanent the cost never asked
    /// for.
    #[test]
    fn the_automatic_payer_spends_nothing_on_an_any_number_cost_choice() {
        let (mut state, payer, subject) = fixture("{0}");
        let creatures = creatures_on_field(&mut state, payer, 2);
        let frame = crate::test_support::frame_src(&state, subject);

        let witness = super::automatic_step_witness(
            &state,
            &choose_creatures(deckmaste_core::Quantity::Range(None, None)),
            &frame,
        );
        assert_eq!(
            witness,
            Some(FulfillmentWitness::Objects(Vec::new())),
            "an \"any number\" cost is paid in full by choosing zero ([CR#107.1c]); \
             the {} creatures on the battlefield stay put",
            creatures.len()
        );
    }

    /// The same payer takes exactly the lower bound when the cost states one —
    /// the demand is met, and nothing beyond it is spent ([CR#601.2h]).
    #[test]
    fn the_automatic_payer_spends_exactly_the_costs_lower_bound() {
        let (mut state, payer, subject) = fixture("{0}");
        let creatures = creatures_on_field(&mut state, payer, 3);
        let frame = crate::test_support::frame_src(&state, subject);

        let Some(FulfillmentWitness::Objects(chosen)) = super::automatic_step_witness(
            &state,
            &choose_creatures(deckmaste_core::Quantity::Range(
                Some(Count::Literal(2)),
                None,
            )),
            &frame,
        ) else {
            panic!("three candidates satisfy a lower bound of two");
        };
        assert_eq!(chosen.len(), 2, "exactly the demanded two, of three legal");
        assert!(chosen.iter().all(|id| creatures.contains(id)));
    }

    /// A cost the battlefield cannot satisfy has no witness at all
    /// ([CR#601.2h]: unpayable costs can't be paid).
    #[test]
    fn the_automatic_payer_offers_no_witness_for_an_unsatisfiable_choice() {
        let (mut state, payer, subject) = fixture("{0}");
        creatures_on_field(&mut state, payer, 1);
        let frame = crate::test_support::frame_src(&state, subject);

        assert_eq!(
            super::automatic_step_witness(
                &state,
                &choose_creatures(deckmaste_core::Quantity::Range(
                    Some(Count::Literal(2)),
                    None,
                )),
                &frame,
            ),
            None,
            "one creature cannot pay a cost demanding two"
        );
    }
}
