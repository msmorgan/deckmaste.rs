use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Binder;
use deckmaste_core::Cmp;
use deckmaste_core::CostComponent;
use deckmaste_core::Destination;
use deckmaste_core::PayAct;
use deckmaste_core::Predicate;
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

/// Return the outstanding obligations currently admitted by CR 601.2h. The
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

fn binder_is_deferred(binder: &Binder) -> bool {
    match binder {
        Binder::SearchOne { from, .. } | Binder::Search { from, .. } => {
            from.contains(&Zone::Library)
        }
        Binder::Produce(action) => action_is_deferred(action),
        Binder::Expanded(expansion) => binder_is_deferred(&expansion.value),
        Binder::TheRef(_)
        | Binder::ChooseOne { .. }
        | Binder::Choose { .. }
        | Binder::Existing(_) => false,
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
        Action::Expanded(expansion) => action_is_deferred(&expansion.value),
        _ => false,
    }
}

fn effect_is_deferred(effect: &deckmaste_core::OneShotEffect) -> bool {
    use deckmaste_core::OneShotEffect;

    match effect {
        OneShotEffect::Act(action) => action_is_deferred(action),
        OneShotEffect::Sequentially(effects) | OneShotEffect::Simultaneously(effects) => {
            effects.iter().any(effect_is_deferred)
        }
        OneShotEffect::Expanded(expansion) => effect_is_deferred(&expansion.value),
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
                if !self.verb_cost_payable(action, payer, &frame) {
                    return illegal("the bound action cost cannot currently be paid");
                }
                FulfillmentPlan {
                    spend: None,
                    work: vec![WorkItem::RunEffect {
                        effect: std::sync::Arc::new(deckmaste_core::OneShotEffect::Act(
                            action.as_ref().clone(),
                        )),
                        frame,
                    }],
                }
            }
            IouKind::ChooseAndPay { binder, body } => {
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
                frame.anaphora.chosen =
                    self.validate_binder_witness(binder, &witness, payer, &frame)?;
                let group = self.resolve_binder(binder, &frame);
                let cardinality = match binder.as_ref() {
                    Binder::TheRef(_)
                    | Binder::ChooseOne { .. }
                    | Binder::SearchOne { .. }
                    | Binder::Produce(_) => crate::stack::Cardinality::One,
                    Binder::Choose { .. } | Binder::Existing(_) | Binder::Search { .. } => {
                        crate::stack::Cardinality::Many
                    }
                    Binder::Expanded(_) => unreachable!("provenance erased at lower"),
                };
                let kind =
                    group
                        .first()
                        .map_or(crate::stack::RefKind::Object, |&object| {
                            match self.objects.obj(object).source {
                                crate::object::ObjectSource::Player(_) => {
                                    crate::stack::RefKind::Player
                                }
                                crate::object::ObjectSource::Card(_) => {
                                    crate::stack::RefKind::Object
                                }
                            }
                        });
                frame.anaphora.that = Some(crate::stack::ThatBinding {
                    cardinality,
                    kind,
                    group,
                });
                frame.anaphora.chosen = None;
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
                .map(|&object| {
                    let source = self.objects.obj(object).source;
                    let logical = match source {
                        crate::object::ObjectSource::Card(card) => super::LogicalObject::Card(card),
                        crate::object::ObjectSource::Player(owner) => {
                            super::LogicalObject::Player(owner)
                        }
                    };
                    (object, logical)
                })
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
                },
                transcript: super::DecisionTranscript::default(),
                object_inputs,
                history_start: frame.working.history.len(),
                spent_mana: plan.spend.into_iter().collect(),
                reversal_barriers: Vec::new(),
                observation_barriers: Vec::new(),
            });
            frame.progress = PaymentProgress::Fulfilling {
                iou: id,
                witness: witness.clone(),
            };
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
        let facts = self.history.facts_from(draft.history_start);
        let mut reversal_barriers = draft.reversal_barriers;
        let mut observation_barriers = draft.observation_barriers;
        reversal_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
        reversal_barriers.dedup();
        observation_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
        observation_barriers.dedup();
        let produced_mana = facts
            .iter()
            .filter_map(|fact| match fact {
                GameEvent::ManaAdded(event) => Some(event.units.iter().copied()),
                _ => None,
            })
            .flatten()
            .collect();
        let record = super::TransactionRecord {
            id: draft.id,
            command: draft.command,
            transcript: draft.transcript,
            object_inputs: draft.object_inputs,
            children: Vec::new(),
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
        binder: &Binder,
        witness: &FulfillmentWitness,
        _payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> Result<Option<Vec<ObjectId>>, DecisionError> {
        match binder {
            Binder::ChooseOne { filter, .. } => {
                let FulfillmentWitness::Objects(objects) = witness else {
                    return illegal("ChooseOne requires one selected object");
                };
                self.validate_choice_objects(objects, 1, 1, filter, frame)?;
                Ok(Some(objects.clone()))
            }
            Binder::Choose {
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
            Binder::Existing(deckmaste_core::Selection::Random(..)) => {
                illegal("random cost binders require the deferred random fulfillment path")
            }
            Binder::TheRef(_) | Binder::Existing(_) if witness == &FulfillmentWitness::Bound => {
                Ok(None)
            }
            Binder::TheRef(_) | Binder::Existing(_) => illegal("a nonchoice binder requires Bound"),
            Binder::SearchOne { .. } | Binder::Search { .. } | Binder::Produce(_) => {
                illegal("search and producer cost binders are not implemented in payment yet")
            }
            Binder::Expanded(_) => unreachable!("provenance erased at lower"),
        }
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

fn runnable_cost_body_effect(
    body: &deckmaste_core::Cost,
) -> Result<deckmaste_core::OneShotEffect, DecisionError> {
    use deckmaste_core::OneShotEffect;

    fn component_effect(component: &CostComponent) -> Result<OneShotEffect, DecisionError> {
        match component {
            CostComponent::Act(action) => Ok(OneShotEffect::Act(action.as_ref().clone())),
            CostComponent::Tap => Ok(OneShotEffect::Act(Action::Tap(
                deckmaste_core::Reference::This,
            ))),
            CostComponent::Untap => Ok(OneShotEffect::Act(Action::Untap(
                deckmaste_core::Reference::This,
            ))),
            CostComponent::ChooseAndPay { binder, body } => {
                Ok(OneShotEffect::With(deckmaste_core::With {
                    binder: binder.as_ref().clone(),
                    body: std::sync::Arc::new(runnable_cost_body_effect(body)?),
                }))
            }
            CostComponent::Cost(inner) => runnable_cost_body_effect(inner),
            CostComponent::Mana(_)
            | CostComponent::ManaCostOf(_)
            | CostComponent::TapTotal { .. } => {
                illegal("a chosen action body cannot contain a second payment resource kind")
            }
            CostComponent::Expanded(_) => unreachable!("provenance erased at lower"),
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
