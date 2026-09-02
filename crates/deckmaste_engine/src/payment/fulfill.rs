use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Cmp;
use deckmaste_core::CostBinder;
use deckmaste_core::CostComponent;
use deckmaste_core::Destination;
use deckmaste_core::LifeOp;
use deckmaste_core::PayAct;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::Stat;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::FulfillmentWitness;
use super::IouId;
use super::IouKind;
use super::ManaPayment;
use super::PaymentIou;
use super::PaymentProgress;
use super::PaymentStage;
use super::ReplayCommand;
use crate::agenda::WorkItem;
use crate::decide::DecisionError;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::LifeLost;
use crate::event::Occurrence;
use crate::event::Tapped;
use crate::object::ObjectId;
use crate::player::FloatingManaId;
use crate::stack::Frame;
use crate::state::GameState;

/// Return the outstanding obligations currently admitted by [CR#601.2h]. The
/// nonrandom/nonlibrary tier must finish before any remaining obligation that
/// introduces randomness or moves a card out of a library.
pub(super) fn current_tier(outstanding: &[PaymentIou]) -> Vec<&PaymentIou> {
    let ordinary: Vec<&PaymentIou> = outstanding.iter().filter(|iou| !is_deferred(iou)).collect();
    if ordinary.is_empty() { outstanding.iter().collect() } else { ordinary }
}

fn is_deferred(iou: &PaymentIou) -> bool {
    match &iou.kind {
        IouKind::Act(action) => action_is_deferred(action),
        IouKind::ChooseAndPay { binder, .. } => binder_is_deferred(binder),
        IouKind::ManaPip(_)
        | IouKind::PayLife(_)
        | IouKind::Tap
        | IouKind::Untap
        | IouKind::TapTotal { .. } => false,
    }
}

fn binder_is_deferred(binder: &CostBinder) -> bool {
    match binder {
        CostBinder::SearchOne { from, .. } | CostBinder::Search { from, .. } => {
            from.contains(&Zone::Library)
        }
        CostBinder::Produce(action) => action_is_deferred(action),
        CostBinder::Existing(Selection::Random(..)) => true,
        CostBinder::TheRef(_)
        | CostBinder::ChooseOne { .. }
        | CostBinder::Choose { .. }
        | CostBinder::Existing(_) => false,
    }
}

fn action_is_deferred(action: &Action) -> bool {
    if matches!(
        action,
        Action::DrawCard(_)
            | Action::FlipCoins(..)
            | Action::RollDice(..)
            | Action::RollPlanarDie(_)
            | Action::Shuffle(_)
    ) {
        return true;
    }
    match action {
        Action::Move(_, destination, _, Some(Zone::Library)) => {
            !matches!(destination, Destination::Library(_))
        }
        Action::Composite { body, .. } => effect_is_deferred(body),
        _ => false,
    }
}

fn effect_is_deferred(effect: &deckmaste_core::OneShotEffect) -> bool {
    use deckmaste_core::OneShotEffect;

    match effect {
        OneShotEffect::Act { action, .. } => action_is_deferred(action),
        OneShotEffect::Sequentially(effects) | OneShotEffect::Simultaneously(effects) => {
            effects.iter().any(effect_is_deferred)
        }
        _ => false,
    }
}

struct FulfillmentPlan {
    spend: Option<FloatingManaId>,
    work: Vec<WorkItem>,
}

impl GameState {
    #[expect(
        clippy::too_many_lines,
        reason = "payment IOU fulfillment keeps validation and its atomic state transition together"
    )]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "the decision boundary owns its witness even when a branch borrows it"
    )]
    pub(super) fn fulfill_payment_iou(
        &mut self,
        id: IouId,
        witness: FulfillmentWitness,
    ) -> Result<(), DecisionError> {
        let (payer, subject, payment, iou, coverage, outstanding, progress, stage) = {
            let controller = self
                .payment
                .as_ref()
                .expect("a Payment decision has a controller");
            let frame = controller
                .frames
                .last()
                .expect("a Payment decision has a frame");
            let fulfilled: std::collections::HashSet<IouId> =
                frame.fulfilled.iter().map(|(iou, _)| *iou).collect();
            let outstanding: Vec<PaymentIou> = frame
                .locked
                .ious
                .iter()
                .filter(|iou| !fulfilled.contains(&iou.id))
                .cloned()
                .collect();
            let iou = outstanding.iter().find(|iou| iou.id == id).cloned();
            (
                frame.payer,
                frame.subject,
                frame.locked.frame.payment,
                iou,
                frame.coverage.clone(),
                outstanding,
                frame.progress.clone(),
                frame.stage,
            )
        };

        if stage != PaymentStage::Paying {
            return illegal("Fulfill is legal only during Paying");
        }
        if progress != PaymentProgress::Idle {
            return illegal("another payment fulfillment is still in flight");
        }
        let Some(iou) = iou else {
            return illegal("Fulfill must name an outstanding IOU");
        };
        if !current_tier(&outstanding)
            .iter()
            .any(|candidate| candidate.id == id)
        {
            return illegal("that IOU is not in the currently legal payment tier");
        }

        let source = subject.object();
        let cause_agent = Some((source, payer));
        let payment_id = payment.map(|payment| payment.id);
        let plan = match &iou.kind {
            IouKind::ManaPip(pip) => {
                if witness != FulfillmentWitness::CoveredMana {
                    return illegal("a mana-pip IOU requires CoveredMana");
                }
                let Some(coverage) = coverage.as_ref() else {
                    return illegal("mana cannot be fulfilled before BeginPayment");
                };
                let Some(selected) = coverage.get(id) else {
                    return illegal("the locked coverage omits this mana pip");
                };
                match *selected {
                    ManaPayment::Floating(unit_id) => {
                        let Some(unit) = self.player(payer).mana_pool.get(unit_id) else {
                            return illegal("the covered mana unit is no longer available");
                        };
                        if !super::coverage::pip_accepts_unit(*pip, unit.kind, &unit.riders)
                            || !self.unit_spendable_on(unit, subject.spend_object())
                        {
                            return illegal(
                                "the covered mana unit is no longer legal for this pip",
                            );
                        }
                        FulfillmentPlan {
                            spend: Some(unit_id),
                            work: Vec::new(),
                        }
                    }
                    ManaPayment::PayPips {
                        object,
                        alternative,
                    } => {
                        let Some(act) = iou.alternatives.get(alternative) else {
                            return illegal("the covered PayPips alternative is unavailable");
                        };
                        super::coverage::validate_pay_pips_object(
                            self,
                            &self
                                .payment
                                .as_ref()
                                .expect("controller remains live")
                                .frames
                                .last()
                                .expect("frame remains live")
                                .locked,
                            act,
                            object,
                        )?;
                        let item = match act {
                            PayAct::TapToPay(_) => {
                                WorkItem::Emit(Occurrence::single(GameEvent::Tapped(Tapped {
                                    object,
                                    cause: Some(
                                        Cause::tap(Agency::CostPayment, cause_agent)
                                            .with_payment(payment_id),
                                    ),
                                })))
                            }
                            PayAct::ExileToPay(_) => WorkItem::Emit(Occurrence::single(
                                self.relocate_from_current(object, Zone::Exile, None),
                            )),
                        };
                        FulfillmentPlan {
                            spend: None,
                            work: vec![item],
                        }
                    }
                }
            }
            IouKind::Tap => {
                if witness != FulfillmentWitness::Bound {
                    return illegal("a tap IOU requires the bound source");
                }
                let Some(object) = self.objects.get(source) else {
                    return illegal("the bound source no longer exists");
                };
                if object.zone != Some(Zone::Battlefield) || object.tapped {
                    return illegal("the bound source cannot currently be tapped");
                }
                FulfillmentPlan {
                    spend: None,
                    work: vec![WorkItem::Emit(Occurrence::single(GameEvent::Tapped(
                        Tapped {
                            object: source,
                            cause: Some(
                                Cause::tap(Agency::CostPayment, cause_agent)
                                    .with_payment(payment_id),
                            ),
                        },
                    )))],
                }
            }
            IouKind::Untap => {
                if witness != FulfillmentWitness::Bound {
                    return illegal("an untap IOU requires the bound source");
                }
                let Some(object) = self.objects.get(source) else {
                    return illegal("the bound source no longer exists");
                };
                if object.zone != Some(Zone::Battlefield) || !object.tapped {
                    return illegal("the bound source cannot currently be untapped");
                }
                FulfillmentPlan {
                    spend: None,
                    work: vec![WorkItem::Emit(Occurrence::single(GameEvent::Untapped(
                        source,
                        Some(
                            Cause::untap(Agency::CostPayment, cause_agent).with_payment(payment_id),
                        ),
                    )))],
                }
            }
            IouKind::PayLife(amount) => {
                if witness != FulfillmentWitness::PayLife {
                    return illegal("a life IOU requires PayLife");
                }
                if self.player(payer).life < i32::try_from(*amount).unwrap_or(i32::MAX) {
                    return illegal("the payer cannot pay more life than their life total");
                }
                let work = (*amount > 0)
                    .then(|| {
                        WorkItem::Emit(Occurrence::single(GameEvent::LifeLost(LifeLost {
                            player: payer,
                            amount: *amount,
                            cause: Some(
                                Cause::lose_life(Agency::CostPayment, cause_agent)
                                    .with_payment(payment_id),
                            ),
                        })))
                    })
                    .into_iter()
                    .collect();
                FulfillmentPlan { spend: None, work }
            }
            IouKind::Act(action) => {
                if witness != FulfillmentWitness::Bound {
                    return illegal("a bound action IOU requires Bound");
                }
                let frame = self
                    .payment
                    .as_ref()
                    .expect("controller remains live")
                    .frames
                    .last()
                    .expect("frame remains live")
                    .locked
                    .frame
                    .clone();
                if !self.verb_cost_payable(action.as_action(), payer, &frame) {
                    return illegal("the bound action cost cannot currently be paid");
                }
                FulfillmentPlan {
                    spend: None,
                    work: vec![WorkItem::RunEffect {
                        effect: std::sync::Arc::new(deckmaste_core::OneShotEffect::act(
                            action.as_action().clone(),
                        )),
                        frame,
                    }],
                }
            }
            IouKind::ChooseAndPay { dest, binder, body } => {
                let mut frame = self
                    .payment
                    .as_ref()
                    .expect("controller remains live")
                    .frames
                    .last()
                    .expect("frame remains live")
                    .locked
                    .frame
                    .clone();
                let chosen = self.validate_binder_witness(binder, &witness, payer, &frame)?;
                self.frame_set_chosen(&mut frame, chosen);
                let group = self.validate_bound_cost_body(*dest, binder, body, payer, &frame)?;
                self.activation_write_objects(frame.activation, *dest, &group);
                self.frame_set_chosen(&mut frame, None);
                FulfillmentPlan {
                    spend: None,
                    work: vec![WorkItem::RunEffect {
                        effect: std::sync::Arc::new(runnable_cost_body_effect(body)?),
                        frame,
                    }],
                }
            }
            IouKind::TapTotal {
                stat,
                cmp,
                count,
                filter,
            } => {
                let FulfillmentWitness::Objects(objects) = &witness else {
                    return illegal("a TapTotal IOU requires a complete object set");
                };
                let frame = self
                    .payment
                    .as_ref()
                    .expect("controller remains live")
                    .frames
                    .last()
                    .expect("frame remains live")
                    .locked
                    .frame
                    .clone();
                if !self.tap_total_witness_is_legal(objects, *stat, *cmp, *count, filter, &frame) {
                    return illegal("the TapTotal witness is duplicate, stale, or insufficient");
                }
                let work = objects
                    .iter()
                    .map(|&object| {
                        WorkItem::Emit(Occurrence::single(GameEvent::Tapped(Tapped {
                            object,
                            cause: Some(
                                Cause::tap(Agency::CostPayment, cause_agent)
                                    .with_payment(payment_id),
                            ),
                        })))
                    })
                    .collect();
                FulfillmentPlan { spend: None, work }
            }
        };

        // Everything above is read-only. From here on the command cannot fail.
        self.pending = None;
        let record = self.mint_payment_record();
        let object_inputs = match &witness {
            FulfillmentWitness::Objects(objects) => objects
                .iter()
                .map(|&object| (object, self.payment_logical_object(object)))
                .collect(),
            _ => std::collections::HashMap::new(),
        };
        {
            let controller = self.payment.as_mut().expect("controller remains live");
            let frame = controller.frames.last_mut().expect("frame remains live");
            frame.recording = Some(super::replay::PendingRecord {
                id: record,
                command: ReplayCommand::Fulfill {
                    iou: id,
                    witness: witness.clone(),
                    random_outcome: None,
                    completed: false,
                },
                transcript: super::DecisionTranscript::default(),
                object_inputs,
                children: Vec::new(),
                facts: Vec::new(),
                spent_mana: plan
                    .spend
                    .into_iter()
                    .map(|id| super::replay::QualifiedManaId { player: payer, id })
                    .collect(),
                reversal_barriers: Vec::new(),
                observation_barriers: Vec::new(),
            });
            frame.begin_fulfillment(id, witness.clone());
        }
        if let Some(unit) = plan.spend {
            self.player_mut(payer)
                .mana_pool
                .remove_ids(&[unit])
                .expect("the exact covered unit was validated above");
        }
        let mut work = plan.work;
        work.push(WorkItem::FinishPaymentFulfillment(id));
        self.schedule_front(work);
        Ok(())
    }

    pub(crate) fn finish_payment_fulfillment(&mut self, id: IouId) {
        let (progress, draft) = {
            let controller = self
                .payment
                .as_mut()
                .expect("a fulfillment sentinel belongs to a payment controller");
            let frame = controller
                .frames
                .last_mut()
                .expect("a fulfillment sentinel belongs to a payment frame");
            (
                std::mem::replace(&mut frame.progress, PaymentProgress::Idle),
                frame
                    .recording
                    .take()
                    .expect("an in-flight fulfillment owns a replay draft"),
            )
        };
        let PaymentProgress::Fulfilling { iou, witness } = progress else {
            panic!("a fulfillment sentinel requires an in-flight IOU");
        };
        assert_eq!(
            iou, id,
            "the fulfillment sentinel matches its in-flight IOU"
        );
        self.payment
            .as_mut()
            .expect("a fulfillment sentinel belongs to a payment controller")
            .frames
            .last_mut()
            .expect("a fulfillment sentinel belongs to a payment frame")
            .fulfillment_continuation = None;
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
                GameEvent::ManaAdded(event) => {
                    Some(
                        event
                            .units
                            .iter()
                            .copied()
                            .map(|id| super::replay::QualifiedManaId {
                                player: event.player,
                                id,
                            }),
                    )
                }
                _ => None,
            })
            .flatten()
            .collect();
        let mut command = draft.command;
        let ReplayCommand::Fulfill { completed, .. } = &mut command else {
            unreachable!("a fulfillment draft owns a fulfillment replay command");
        };
        *completed = true;
        let object_inputs = self.payment_record_object_inputs(&facts, draft.object_inputs);
        let record = super::TransactionRecord {
            id: draft.id,
            command,
            transcript: draft.transcript,
            object_inputs,
            children: draft.children,
            facts,
            produced_mana,
            spent_mana: draft.spent_mana,
            dependencies: Vec::new(),
            reversal_barriers,
            observation_barriers,
        };
        let controller = self
            .payment
            .as_mut()
            .expect("a fulfillment sentinel belongs to a payment controller");
        let frame = controller
            .frames
            .last_mut()
            .expect("a fulfillment sentinel belongs to a payment frame");
        frame.records.push(record);
        frame.fulfilled.push((id, witness));
        if frame.fulfilled.len() == frame.locked.ious.len() {
            frame.stage = PaymentStage::Ready;
        }
        let prompt = frame.prompt(Vec::new());
        frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
        self.refresh_payment_prompt();
    }

    fn validate_binder_witness(
        &self,
        binder: &CostBinder,
        witness: &FulfillmentWitness,
        _payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> Result<Option<Vec<ObjectId>>, DecisionError> {
        if !deckmaste_core::cost_binder_is_runnable(binder) {
            return illegal("the cost binder retains an unsupported unresolved operation");
        }
        match binder {
            CostBinder::ChooseOne { filter, .. } => {
                let FulfillmentWitness::Objects(objects) = witness else {
                    return illegal("ChooseOne requires one selected object");
                };
                self.validate_choice_objects(objects, 1, 1, filter, frame)?;
                Ok(Some(objects.clone()))
            }
            CostBinder::Choose {
                quantity, filter, ..
            } => {
                let FulfillmentWitness::Objects(objects) = witness else {
                    return illegal("Choose requires its complete selected object set");
                };
                let (lo, hi) = quantity.bounds();
                let min = lo.map_or(0, |count| self.eval_count(count, frame));
                let max = hi.map_or(Uint::MAX, |count| self.eval_count(count, frame));
                self.validate_choice_objects(objects, min, max, filter, frame)?;
                Ok(Some(objects.clone()))
            }
            CostBinder::Existing(Selection::Random(quantity, filter)) => {
                if witness != &FulfillmentWitness::Bound {
                    return illegal("a random cost binder requires Bound");
                }
                let watcher = Some(self.frame_watcher(frame));
                let available = crate::target::candidates_with(self, filter, watcher).len();
                let (lo, _) = quantity.bounds();
                let required = lo.map_or(0, |count| self.eval_count(count, frame));
                if Uint::try_from(available).unwrap_or(Uint::MAX) < required {
                    return illegal("the random cost has too few legal subjects");
                }
                Ok(None)
            }
            CostBinder::TheRef(_) | CostBinder::Existing(_)
                if witness == &FulfillmentWitness::Bound =>
            {
                Ok(None)
            }
            CostBinder::TheRef(_) | CostBinder::Existing(_) => {
                illegal("a nonchoice binder requires Bound")
            }
            CostBinder::SearchOne {
                filter,
                whose,
                from,
                ..
            } => {
                let FulfillmentWitness::Objects(objects) = witness else {
                    return illegal("SearchOne requires its complete searched object set");
                };
                let candidates = self.payment_search_candidates(whose, from, filter, frame);
                let (_, max) =
                    self.choice_bounds(&deckmaste_core::Quantity::one(), candidates.len(), frame);
                let min = if search_is_bare_quantity(filter) { max } else { 0 };
                validate_search_witness(objects, &candidates, min, max)?;
                Ok(Some(objects.clone()))
            }
            CostBinder::Search {
                quantity,
                filter,
                whose,
                from,
                ..
            } => {
                let FulfillmentWitness::Objects(objects) = witness else {
                    return illegal("Search requires its complete searched object set");
                };
                let candidates = self.payment_search_candidates(whose, from, filter, frame);
                let (lo, hi) = self.choice_bounds(quantity, candidates.len(), frame);
                let min = if search_is_bare_quantity(filter) { lo } else { 0 };
                validate_search_witness(objects, &candidates, min, hi)?;
                Ok(Some(objects.clone()))
            }
            CostBinder::Produce(action) => {
                if witness != &FulfillmentWitness::Bound {
                    return illegal("a producer cost binder requires Bound");
                }
                let Action::Move(subject, _, _, from) = action.as_ref() else {
                    return illegal("a producer cost binder currently requires a Move action");
                };
                let object = self.eval_reference(subject, frame);
                let Some(current) = self.objects.get(object) else {
                    return illegal("the producer cost's bound subject no longer exists");
                };
                if from.is_some_and(|zone| current.zone != Some(zone)) {
                    return illegal(
                        "the producer cost's bound subject is no longer in its required zone",
                    );
                }
                Ok(None)
            }
        }
    }

    fn validate_bound_cost_body(
        &self,
        dest: deckmaste_core::DefId,
        binder: &CostBinder,
        body: &deckmaste_core::Cost,
        payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> Result<Vec<ObjectId>, DecisionError> {
        let mut projected = self.clone();
        let mut bound = frame.clone();
        let group = match binder {
            CostBinder::Produce(action) => {
                let Action::Move(subject, ..) = action.as_ref() else {
                    return illegal("a producer cost binder currently requires a Move action");
                };
                let object = projected.eval_reference(subject, &bound);
                if projected.objects.get(object).is_none()
                    || !projected.preflight_cost_action(action, payer, &bound)
                {
                    return illegal("the producer cost cannot currently be paid");
                }
                vec![object]
            }
            CostBinder::Existing(Selection::Random(_, filter)) => {
                let watcher = Some(projected.frame_watcher(&bound));
                crate::target::candidates_with(&projected, filter, watcher)
            }
            _ => projected.resolve_binder(binder, &bound),
        };
        let cardinality = match binder {
            CostBinder::TheRef(_)
            | CostBinder::ChooseOne { .. }
            | CostBinder::Produce(_)
            | CostBinder::SearchOne { .. } => crate::stack::Cardinality::One,
            CostBinder::Choose { .. } | CostBinder::Existing(_) | CostBinder::Search { .. } => {
                crate::stack::Cardinality::Many
            }
        };
        if cardinality == crate::stack::Cardinality::One && group.len() != 1 {
            return illegal("a singular cost binder must resolve to one live object");
        }
        if group
            .iter()
            .any(|&object| projected.objects.get(object).is_none())
        {
            return illegal("the bound cost subject no longer exists");
        }
        projected.frame_set_chosen(&mut bound, None);
        projected.activation_write_objects(bound.activation, dest, &group);
        if projected.preflight_cost_components(body, payer, &bound) {
            Ok(group)
        } else {
            illegal("the complete bound cost body cannot currently be paid")
        }
    }

    fn preflight_cost_components(
        &mut self,
        body: &deckmaste_core::Cost,
        payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> bool {
        body.iter().all(|component| match component {
            CostComponent::Act(action) => {
                self.preflight_cost_action(action.as_action(), payer, frame)
            }
            CostComponent::Tap => self.preflight_cost_action(
                &Action::Tap(Reference::source_parameter()),
                payer,
                frame,
            ),
            CostComponent::Untap => self.preflight_cost_action(
                &Action::Untap(Reference::source_parameter()),
                payer,
                frame,
            ),
            CostComponent::Cost(inner) => self.preflight_cost_components(inner, payer, frame),
            CostComponent::ChooseAndPay { .. }
            | CostComponent::Mana(_)
            | CostComponent::ManaCostOf(_)
            | CostComponent::TapTotal { .. } => false,
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the checked cost-action set is intentionally reviewed in one exhaustive match"
    )]
    fn preflight_cost_action(
        &mut self,
        action: &Action,
        payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> bool {
        match action {
            Action::Sacrifice(agent, subject) => {
                let Some(actor) = self.eval_player_ref(agent, frame) else {
                    return false;
                };
                let object = self.eval_reference(subject, frame);
                let Some(candidate) = self.objects.get(object) else {
                    return false;
                };
                if actor != payer
                    || candidate.zone != Some(Zone::Battlefield)
                    || candidate.controller != actor
                {
                    return false;
                }
                let candidate = self.objects.obj_mut(object);
                candidate.zone = Some(Zone::Graveyard);
                candidate.tapped = false;
                candidate.counters.clear();
                candidate.damage.clear();
                candidate.attached_to = None;
                candidate.summoning_sick = false;
                candidate.skip_next_untap = false;
                candidate.side = crate::object::Side::Front;
                true
            }
            Action::Move(subject, destination, riders, from) => {
                let object = self.eval_reference(subject, frame);
                let Some(candidate) = self.objects.get(object) else {
                    return false;
                };
                if from.is_some_and(|required| candidate.zone != Some(required)) {
                    return false;
                }
                let owner = match candidate.source {
                    crate::object::ObjectSource::Card(_) => Some(self.owner_of(object)),
                    crate::object::ObjectSource::Player(_) => None,
                };
                let mut controller = candidate.controller;
                for rider in riders.iter() {
                    match rider {
                        deckmaste_core::EnterRider::UnderControlOf(who) => {
                            let Some(new_controller) = self.eval_player_ref(who, frame) else {
                                return false;
                            };
                            controller = new_controller;
                        }
                        deckmaste_core::EnterRider::UnderOwnersControl => {
                            let Some(owner) = owner else {
                                return false;
                            };
                            controller = owner;
                        }
                        _ => {}
                    }
                }
                let zone = match destination {
                    Destination::Zone(zone) => *zone,
                    Destination::Library(_) => Zone::Library,
                };
                let counter_additions = riders
                    .iter()
                    .filter_map(|rider| match rider {
                        deckmaste_core::EnterRider::WithCounters(kind, count) => {
                            Some((kind.0, self.eval_count(count, frame)))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let candidate = self.objects.obj_mut(object);
                candidate.zone = Some(zone);
                candidate.controller = controller;
                candidate.tapped = zone == Zone::Battlefield
                    && riders
                        .iter()
                        .any(|rider| matches!(rider, deckmaste_core::EnterRider::Tapped));
                candidate.counters.clear();
                candidate.damage.clear();
                candidate.attached_to = None;
                candidate.summoning_sick = zone == Zone::Battlefield;
                candidate.skip_next_untap = false;
                candidate.side = crate::object::Side::Front;
                for (kind, amount) in counter_additions {
                    let counter = candidate.counters.entry(kind).or_default();
                    *counter = counter.saturating_add(amount);
                }
                true
            }
            Action::Tap(subject) | Action::Untap(subject) => {
                let tap = matches!(action, Action::Tap(_));
                let object = self.eval_reference(subject, frame);
                let Some(candidate) = self.objects.get(object) else {
                    return false;
                };
                if candidate.zone != Some(Zone::Battlefield)
                    || candidate.controller != payer
                    || candidate.tapped == tap
                {
                    return false;
                }
                self.objects.obj_mut(object).tapped = tap;
                true
            }
            Action::ChangeLife(subject, operation) => {
                let Some(recipient) = self.eval_player_ref(subject, frame) else {
                    return false;
                };
                let (count, direction) = match operation {
                    LifeOp::Down(count) => (count, -1),
                    LifeOp::Up(count) => (count, 1),
                    LifeOp::Set(count) => (count, 0),
                };
                let Ok(amount) = i32::try_from(self.eval_count(count, frame)) else {
                    return false;
                };
                let life = self.player(recipient).life;
                let next = match direction {
                    -1 if life >= amount => life.checked_sub(amount),
                    1 => life.checked_add(amount),
                    0 => Some(amount),
                    _ => None,
                };
                let Some(next) = next else { return false };
                self.player_mut(recipient).life = next;
                true
            }
            Action::PutCounters(subject, kind, count)
            | Action::RemoveCounters(subject, kind, count) => {
                let remove = matches!(action, Action::RemoveCounters(..));
                let object = self.eval_reference(subject, frame);
                let amount = self.eval_count(count, frame);
                let Some(candidate) = self.objects.get(object) else {
                    return false;
                };
                let current = candidate.counters.get(kind.as_str()).copied().unwrap_or(0);
                if remove && current < amount {
                    return false;
                }
                let counter = self
                    .objects
                    .obj_mut(object)
                    .counters
                    .entry(kind.0)
                    .or_default();
                *counter = if remove {
                    counter.saturating_sub(amount)
                } else {
                    counter.saturating_add(amount)
                };
                true
            }
            Action::Reveal { what, to } => {
                let object = self.eval_reference(what, frame);
                self.objects.get(object).is_some()
                    && to
                        .as_ref()
                        .is_none_or(|who| self.eval_player_ref(who, frame).is_some())
            }
            Action::Composite { name, body } if name.as_str() == "Discard" => {
                self.preflight_discard_effect(body, payer, frame)
            }
            _ => false,
        }
    }

    fn preflight_discard_effect(
        &mut self,
        effect: &deckmaste_core::OneShotEffect,
        payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> bool {
        match effect {
            deckmaste_core::OneShotEffect::Act { action, .. } => {
                if let Action::Move(subject, _, _, Some(Zone::Hand)) = action {
                    let object = self.eval_reference(subject, frame);
                    if self.objects.get(object).is_none()
                        || self.owner_of(object) != payer
                        || self.objects.obj(object).zone != Some(Zone::Hand)
                    {
                        return false;
                    }
                }
                self.preflight_cost_action(action, payer, frame)
            }
            deckmaste_core::OneShotEffect::Each(each) => {
                let objects = self.eval_selection_set(&each.over, frame);
                for object in objects {
                    if self.objects.get(object).is_none() {
                        return false;
                    }
                    let mut element = frame.clone();
                    element.activation = self.enter_loop_region(&each.body, frame, object, None);
                    if !each
                        .body
                        .body
                        .iter()
                        .all(|effect| self.preflight_discard_effect(effect, payer, &element))
                    {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn payment_search_candidates(
        &self,
        whose: &deckmaste_core::Reference,
        from: &[Zone],
        filter: &Predicate,
        frame: &Frame,
    ) -> Vec<ObjectId> {
        let whose = self.acting_player(whose, frame);
        let watcher = Some(self.frame_watcher(frame));
        self.objects
            .iter()
            .filter(|object| object.zone.is_some_and(|zone| from.contains(&zone)))
            .filter(|object| self.owner_of(object.id) == whose)
            .filter(|object| crate::target::matches_with(self, object.id, filter, watcher))
            .map(|object| object.id)
            .collect()
    }

    fn validate_choice_objects(
        &self,
        objects: &[ObjectId],
        min: Uint,
        max: Uint,
        filter: &Predicate,
        frame: &Frame,
    ) -> Result<(), DecisionError> {
        let len = Uint::try_from(objects.len()).unwrap_or(Uint::MAX);
        let distinct: std::collections::HashSet<ObjectId> = objects.iter().copied().collect();
        let watcher = Some(self.frame_watcher(frame));
        if len < min
            || len > max
            || distinct.len() != objects.len()
            || objects.iter().any(|&object| {
                self.objects.get(object).is_none()
                    || !crate::target::matches_with(self, object, filter, watcher)
            })
        {
            return illegal("the chosen cost subjects must be distinct live legal candidates");
        }
        Ok(())
    }

    fn tap_total_witness_is_legal(
        &self,
        objects: &[ObjectId],
        stat: Stat,
        cmp: Cmp,
        count: Uint,
        filter: &Predicate,
        frame: &Frame,
    ) -> bool {
        let distinct: std::collections::HashSet<ObjectId> = objects.iter().copied().collect();
        if distinct.len() != objects.len() {
            return false;
        }
        let watcher = Some(self.frame_watcher(frame));
        let view = self.layers();
        let mut sum: Uint = 0;
        for &object in objects {
            let Some(candidate) = self.objects.get(object) else {
                return false;
            };
            if candidate.zone != Some(Zone::Battlefield)
                || candidate.tapped
                || !crate::target::matches_with(self, object, filter, watcher)
            {
                return false;
            }
            let Some(value) = self.cost_stat_value(&view, object, stat) else {
                return false;
            };
            sum = sum.saturating_add(value);
        }
        cmp.apply(sum, count)
    }

    /// Enumerate every currently legal witness for one outstanding `TapTotal`
    /// IOU. The core exposes the full set; runner preference stays external.
    #[must_use]
    pub fn legal_tap_total_subsets(&self, id: IouId) -> Vec<Vec<ObjectId>> {
        let Some(controller) = self.payment.as_ref() else {
            return Vec::new();
        };
        let Some(payment) = controller.frames.last() else {
            return Vec::new();
        };
        if payment.stage != PaymentStage::Paying
            || payment.progress != PaymentProgress::Idle
            || payment
                .fulfilled
                .iter()
                .any(|(fulfilled, _)| *fulfilled == id)
        {
            return Vec::new();
        }
        let Some(iou) = payment.locked.ious.iter().find(|iou| iou.id == id) else {
            return Vec::new();
        };
        let IouKind::TapTotal {
            stat,
            cmp,
            count,
            filter,
        } = &iou.kind
        else {
            return Vec::new();
        };
        let fulfilled: std::collections::HashSet<IouId> =
            payment.fulfilled.iter().map(|(iou, _)| *iou).collect();
        let outstanding: Vec<PaymentIou> = payment
            .locked
            .ious
            .iter()
            .filter(|iou| !fulfilled.contains(&iou.id))
            .cloned()
            .collect();
        if !current_tier(&outstanding)
            .iter()
            .any(|candidate| candidate.id == id)
        {
            return Vec::new();
        }

        let frame = &payment.locked.frame;
        let watcher = Some(self.frame_watcher(frame));
        let view = self.layers();
        let candidates: Vec<ObjectId> = crate::target::candidates_with(self, filter, watcher)
            .into_iter()
            .filter(|&object| {
                self.objects.get(object).is_some_and(|candidate| {
                    candidate.zone == Some(Zone::Battlefield) && !candidate.tapped
                }) && self.cost_stat_value(&view, object, *stat).is_some()
            })
            .collect();
        let mut subsets = Vec::new();
        let mut selected = Vec::new();
        self.enumerate_tap_total_subsets(
            &candidates,
            0,
            &mut selected,
            &mut subsets,
            *stat,
            *cmp,
            *count,
            filter,
            frame,
        );
        subsets
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "the private subset recursion threads one locked TapTotal predicate"
    )]
    fn enumerate_tap_total_subsets(
        &self,
        candidates: &[ObjectId],
        index: usize,
        selected: &mut Vec<ObjectId>,
        out: &mut Vec<Vec<ObjectId>>,
        stat: Stat,
        cmp: Cmp,
        count: Uint,
        filter: &Predicate,
        frame: &Frame,
    ) {
        if index == candidates.len() {
            if self.tap_total_witness_is_legal(selected, stat, cmp, count, filter, frame) {
                out.push(selected.clone());
            }
            return;
        }
        self.enumerate_tap_total_subsets(
            candidates,
            index + 1,
            selected,
            out,
            stat,
            cmp,
            count,
            filter,
            frame,
        );
        selected.push(candidates[index]);
        self.enumerate_tap_total_subsets(
            candidates,
            index + 1,
            selected,
            out,
            stat,
            cmp,
            count,
            filter,
            frame,
        );
        selected.pop();
    }
}

fn illegal<T>(reason: impl Into<String>) -> Result<T, DecisionError> {
    Err(DecisionError::Illegal {
        reason: reason.into(),
    })
}

fn search_is_bare_quantity(filter: &Predicate) -> bool {
    matches!(
        filter,
        Predicate::Kind(deckmaste_core::ObjectKind::Card) | Predicate::Any
    )
}

fn validate_search_witness(
    objects: &[ObjectId],
    candidates: &[ObjectId],
    min: Uint,
    max: Uint,
) -> Result<(), DecisionError> {
    let len = Uint::try_from(objects.len()).unwrap_or(Uint::MAX);
    let selected: std::collections::HashSet<ObjectId> = objects.iter().copied().collect();
    let available: std::collections::HashSet<ObjectId> = candidates.iter().copied().collect();
    if len < min || len > max || selected.len() != objects.len() || !selected.is_subset(&available)
    {
        return illegal("the searched cost subjects must be a complete distinct legal set");
    }
    Ok(())
}

fn runnable_cost_body_effect(
    body: &deckmaste_core::Cost,
) -> Result<deckmaste_core::OneShotEffect, DecisionError> {
    use deckmaste_core::OneShotEffect;

    fn component_effect(component: &CostComponent) -> Result<OneShotEffect, DecisionError> {
        match component {
            CostComponent::Act(action) => Ok(OneShotEffect::act(action.as_action().clone())),
            CostComponent::Tap => Ok(OneShotEffect::act(Action::Tap(
                deckmaste_core::Reference::source_parameter(),
            ))),
            CostComponent::Untap => Ok(OneShotEffect::act(Action::Untap(
                deckmaste_core::Reference::source_parameter(),
            ))),
            CostComponent::ChooseAndPay { body, .. } => runnable_cost_body_effect(body),
            CostComponent::Cost(inner) => runnable_cost_body_effect(inner),
            CostComponent::Mana(_)
            | CostComponent::ManaCostOf(_)
            | CostComponent::TapTotal { .. } => {
                illegal("a chosen action body cannot contain a second payment resource kind")
            }
        }
    }

    let mut effects = body
        .iter()
        .map(component_effect)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(match effects.len() {
        1 => effects.pop().expect("one effect"),
        _ => OneShotEffect::Sequentially(effects.into()),
    })
}
