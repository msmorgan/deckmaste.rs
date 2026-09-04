use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Cmp;
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

/// Return the outstanding obligations currently admitted by [CR#601.2h]. The
/// nonrandom/nonlibrary tier must finish before any remaining obligation that
/// introduces randomness or moves a card out of a library.
///
/// Within a tier, a payment-time subject instruction also gates everything
/// declared after it in the cost block: the verbs that follow pay through the
/// register it writes, so offering them first would ask the payer to spend an
/// unwritten binding. Choices and samples are both such writers. Obligation
/// ids are minted in block order, so "declared after" is "has a larger id".
pub(super) fn current_tier(outstanding: &[PaymentIou]) -> Vec<&PaymentIou> {
    let first_writer = outstanding
        .iter()
        .filter(|iou| writes_a_register(iou))
        .map(|iou| iou.id)
        .min();
    let ordinary: Vec<&PaymentIou> = outstanding
        .iter()
        .filter(|iou| !is_deferred(iou))
        .filter(|iou| first_writer.is_none_or(|writer| iou.id <= writer))
        .collect();
    let tier: Vec<&PaymentIou> =
        if ordinary.is_empty() { outstanding.iter().collect() } else { ordinary };
    let Some(first_writer) = first_writer else {
        return tier;
    };
    tier.into_iter()
        .filter(|iou| iou.id <= first_writer)
        .collect()
}

/// Whether this obligation writes a register later obligations read
/// ([CR#601.2b]).
fn writes_a_register(iou: &PaymentIou) -> bool {
    match &iou.kind {
        IouKind::Choose(_) | IouKind::Sample(_) | IouKind::Search(_) | IouKind::Let(_) => true,
        IouKind::Act { dest, .. } => dest.is_some(),
        IouKind::ManaPip(_)
        | IouKind::PayLife(_)
        | IouKind::Tap
        | IouKind::Untap
        | IouKind::TapTotal { .. } => false,
    }
}

fn is_deferred(iou: &PaymentIou) -> bool {
    if iou.deferred {
        return true;
    }
    match &iou.kind {
        IouKind::Act { action, .. } => action_is_deferred(action),
        // [CR#601.2h]: a search that moves a card out of a library belongs to
        // the second payment tier, as does a random pick.
        IouKind::Search(search) => search.from.contains(&Zone::Library),
        IouKind::Sample(_) => true,
        IouKind::ManaPip(_)
        | IouKind::PayLife(_)
        | IouKind::Tap
        | IouKind::Untap
        | IouKind::Choose(_)
        | IouKind::Let(_)
        | IouKind::TapTotal { .. } => false,
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
    sample: Option<PaymentSample>,
    work: Vec<WorkItem>,
}

struct PaymentSample {
    activation: crate::ActivationId,
    dest: deckmaste_core::DefId,
    candidates: Vec<ObjectId>,
    count: usize,
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
                            sample: None,
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
                            sample: None,
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
                    sample: None,
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
                    sample: None,
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
                FulfillmentPlan {
                    spend: None,
                    sample: None,
                    work,
                }
            }
            IouKind::Act { dest, action } => {
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
                // [CR#400.7]: the paid product. The moved object's PRE-move id
                // is written; register reads chase it to its post-move
                // incarnation, the same way an effect-side producing
                // instruction's product is read.
                if let Some(dest) = dest {
                    // [CR#701.21a]: a sacrifice is a move to the graveyard, so
                    // both verbs name their product in the same slot.
                    let (Action::Move(subject, ..) | Action::Sacrifice(_, subject)) =
                        action.as_action()
                    else {
                        return illegal(
                            "a producing cost action currently requires a Move or Sacrifice",
                        );
                    };
                    let object = self.eval_reference(subject, &frame);
                    if self.objects.get(object).is_none() {
                        return illegal("the producing cost's subject no longer exists");
                    }
                    self.activation_write_object(frame.activation, *dest, object);
                }
                FulfillmentPlan {
                    spend: None,
                    sample: None,
                    work: vec![WorkItem::RunEffect {
                        effect: std::sync::Arc::new(deckmaste_core::OneShotEffect::act(
                            action.as_action().clone(),
                        )),
                        frame,
                    }],
                }
            }
            // [CR#601.2b]: a payment-time decision writes its register in the
            // ANNOUNCE activation, so the verbs after it — and the ability
            // body — read the payment subject by index.
            IouKind::Choose(choice) => {
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
                let FulfillmentWitness::Objects(objects) = &witness else {
                    return illegal("a payment-time choice requires its complete object set");
                };
                let (lo, hi) = choice.quantity.bounds();
                let min = lo.map_or(0, |count| self.eval_count(count, &frame));
                let max = hi.map_or(Uint::MAX, |count| self.eval_count(count, &frame));
                self.validate_choice_region_objects(objects, min, max, &choice.filter, &frame)?;
                let mut projected = self.clone();
                projected.activation_write_objects(frame.activation, choice.dest, objects);
                if !projected.preflight_decision_body(&outstanding, id, payer, &frame) {
                    return illegal("the complete chosen cost body cannot currently be paid");
                }
                self.activation_write_objects(frame.activation, choice.dest, objects);
                FulfillmentPlan {
                    spend: None,
                    sample: None,
                    work: Vec::new(),
                }
            }
            IouKind::Sample(sample) => {
                if witness != FulfillmentWitness::Bound {
                    return illegal("a random payment sample requires Bound");
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
                let watcher = Some(self.frame_watcher(&frame));
                let candidates = crate::target::candidates_region_with_activation(
                    self,
                    &sample.filter,
                    watcher,
                    frame.activation,
                );
                let (lo, _) = sample.quantity.bounds();
                let required = lo.map_or(0, |count| self.eval_count(count, &frame));
                if Uint::try_from(candidates.len()).unwrap_or(Uint::MAX) < required {
                    return illegal("a random payment sample has too few legal subjects");
                }
                let (_, count) = self.choice_bounds(&sample.quantity, candidates.len(), &frame);
                let count = usize::try_from(count).expect("sample count fits usize");
                let mut projected = self.clone();
                let selected = if let Some((recorded, post_sample_rng)) =
                    projected.take_replay_random_outcome()
                {
                    projected.rng.set_stream(post_sample_rng.stream);
                    projected.rng.set_word_pos(post_sample_rng.word_pos);
                    recorded
                } else {
                    let indices =
                        rand::seq::index::sample(&mut projected.rng, candidates.len(), count);
                    indices.into_iter().map(|index| candidates[index]).collect()
                };
                projected.activation_write_objects(frame.activation, sample.dest, &selected);
                if !projected.preflight_decision_body(&outstanding, id, payer, &frame) {
                    return illegal("the complete sampled cost body cannot currently be paid");
                }
                FulfillmentPlan {
                    spend: None,
                    sample: Some(PaymentSample {
                        activation: frame.activation,
                        dest: sample.dest,
                        candidates,
                        count,
                    }),
                    work: Vec::new(),
                }
            }
            IouKind::Search(search) => {
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
                let FulfillmentWitness::Objects(objects) = &witness else {
                    return illegal("a payment-time search requires its complete object set");
                };
                let candidates = self.payment_search_candidates(
                    &search.whose,
                    &search.from,
                    &search.filter.body,
                    &frame,
                );
                let (lo, hi) = self.choice_bounds(&search.quantity, candidates.len(), &frame);
                // [CR#701.23b..701.23d]: a stated-quality search never compels
                // a find; only a bare-quantity search must take as many as
                // exist.
                let min = if crate::resolve::search_is_bare_quantity(&search.filter.body) {
                    lo
                } else {
                    0
                };
                validate_search_witness(objects, &candidates, min, hi)?;
                let mut projected = self.clone();
                projected.activation_write_objects(frame.activation, search.dest, objects);
                if !projected.preflight_decision_body(&outstanding, id, payer, &frame) {
                    return illegal("the complete searched cost body cannot currently be paid");
                }
                self.activation_write_objects(frame.activation, search.dest, objects);
                FulfillmentPlan {
                    spend: None,
                    sample: None,
                    work: Vec::new(),
                }
            }
            IouKind::Let(binding) => {
                if witness != FulfillmentWitness::Bound {
                    return illegal("a pinned payment subject requires Bound");
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
                let mut projected = self.clone();
                projected.write_let(binding, &frame);
                if !projected.preflight_decision_body(&outstanding, id, payer, &frame) {
                    return illegal("the complete pinned cost body cannot currently be paid");
                }
                self.write_let(binding, &frame);
                FulfillmentPlan {
                    spend: None,
                    sample: None,
                    work: Vec::new(),
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
                FulfillmentPlan {
                    spend: None,
                    sample: None,
                    work,
                }
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
                    .iter()
                    .copied()
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
        if let Some(sample) = plan.sample {
            let selected = if let Some((recorded, post_sample_rng)) =
                self.take_replay_random_outcome()
            {
                self.rng.set_stream(post_sample_rng.stream);
                self.rng.set_word_pos(post_sample_rng.word_pos);
                recorded
            } else {
                let indices =
                    rand::seq::index::sample(&mut self.rng, sample.candidates.len(), sample.count);
                indices
                    .into_iter()
                    .map(|index| sample.candidates[index])
                    .collect()
            };
            self.record_payment_random_outcome(&selected);
            self.activation_write_objects(sample.activation, sample.dest, &selected);
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
        frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
        self.refresh_payment_prompt();
    }

    /// The cards a payment-time search may find ([CR#701.23]): `whose`'s
    /// objects in the searched `from` zones that match the filter.
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

    /// Validate a payment-time choice's witness against its CANDIDATE REGION
    /// ([CR#601.2b]): the filter is a region whose candidate parameter each
    /// object is written into, so a filter reading the payment's own
    /// registers judges every candidate correctly.
    fn validate_choice_region_objects(
        &self,
        objects: &[ObjectId],
        min: Uint,
        max: Uint,
        filter: &deckmaste_core::Region<Predicate>,
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
                    || !crate::target::matches_region_with_activation(
                        self,
                        object,
                        filter,
                        watcher,
                        frame.activation,
                    )
            })
        {
            return illegal("the chosen cost subjects must be distinct live legal candidates");
        }
        Ok(())
    }

    /// Check the action instructions immediately following a payment-time
    /// subject writer as one atomic body. Lowering places the lifted
    /// `Choose`/`Sample`/`Search`/`Let` directly before the action IOUs that
    /// spend its register; the next non-action IOU begins another payment
    /// block. The projection is discarded whether the check succeeds or
    /// fails, so no cost mutation or random observation escapes preflight
    /// ([CR#601.2h,733.1]).
    fn preflight_decision_body(
        &mut self,
        outstanding: &[PaymentIou],
        writer: IouId,
        payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> bool {
        outstanding
            .iter()
            .skip_while(|iou| iou.id != writer)
            .skip(1)
            .map_while(|iou| match &iou.kind {
                IouKind::Act { action, .. } => Some(action.as_action()),
                _ => None,
            })
            .all(|action| self.preflight_cost_action(action, payer, frame))
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the projected cost-action set is intentionally reviewed in one exhaustive match"
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
                    deckmaste_core::LifeOp::Down(count) => (count, -1),
                    deckmaste_core::LifeOp::Up(count) => (count, 1),
                    deckmaste_core::LifeOp::Set(count) => (count, 0),
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

    /// Write a pinned payment subject's register ([CR#608.2h]).
    fn write_let(&self, binding: &deckmaste_core::Let, frame: &Frame) {
        match &binding.expr {
            deckmaste_core::Expr::Object(reference) => {
                let object = self.eval_reference(reference, frame);
                self.activation_write_object(frame.activation, binding.dest, object);
            }
            deckmaste_core::Expr::Objects(selection) => {
                let objects = self.eval_selection_set(selection, frame);
                self.activation_write_objects(frame.activation, binding.dest, &objects);
            }
            deckmaste_core::Expr::Number(count) => {
                let number = self.eval_count(count, frame);
                self.activation_write_number(frame.activation, binding.dest, number);
            }
        }
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
