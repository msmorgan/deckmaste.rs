use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Binder;
use deckmaste_core::Destination;
use deckmaste_core::PayAct;
use deckmaste_core::Zone;

use super::FulfillmentWitness;
use super::IouId;
use super::IouKind;
use super::ManaPayment;
use super::PaymentIou;
use super::PaymentProgress;
use super::PaymentStage;
use crate::agenda::WorkItem;
use crate::decide::DecisionError;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::LifeLost;
use crate::event::Occurrence;
use crate::event::Tapped;
use crate::player::FloatingManaId;
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
                            || !self.unit_spendable_on(unit, subject.object())
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
            IouKind::Act(_) | IouKind::ChooseAndPay { .. } | IouKind::TapTotal { .. } => {
                return illegal("this action-shaped IOU is not implemented yet");
            }
        };

        // Everything above is read-only. From here on the command cannot fail.
        self.pending = None;
        {
            let controller = self.payment.as_mut().expect("controller remains live");
            let frame = controller.frames.last_mut().expect("frame remains live");
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
        let controller = self
            .payment
            .as_mut()
            .expect("a fulfillment sentinel belongs to a payment controller");
        let frame = controller
            .frames
            .last_mut()
            .expect("a fulfillment sentinel belongs to a payment frame");
        let progress = std::mem::replace(&mut frame.progress, PaymentProgress::Idle);
        let PaymentProgress::Fulfilling { iou, witness } = progress else {
            panic!("a fulfillment sentinel requires an in-flight IOU");
        };
        assert_eq!(
            iou, id,
            "the fulfillment sentinel matches its in-flight IOU"
        );
        frame.fulfilled.push((id, witness));
        if frame.fulfilled.len() == frame.locked.ious.len() {
            frame.stage = PaymentStage::Ready;
        }
        let prompt = frame.prompt();
        frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
    }
}

fn illegal<T>(reason: impl Into<String>) -> Result<T, DecisionError> {
    Err(DecisionError::Illegal {
        reason: reason.into(),
    })
}
