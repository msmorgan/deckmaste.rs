use deckmaste_core::ActivatedManaProfile;

use super::DecisionTranscript;
use super::LogicalObject;
use super::PaymentFrame;
use super::PaymentProgress;
use super::PaymentRecordId;
use super::PaymentStage;
use super::TransactionRecord;
use crate::agenda::WorkItem;
use crate::decide::DecisionError;
use crate::event::GameEvent;
use crate::event::ManaAbilityActivated;
use crate::event::ManaProduced;
use crate::event::Occurrence;
use crate::event::TappedForMana;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::ManaActionId;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::trigger::TriggerBindings;

/// One activated mana ability from the instant its nested announcement opens
/// until its stackless effect and caused immediate work finish.
#[derive(Debug, Clone)]
pub(crate) struct ManaAction {
    pub id: ManaActionId,
    pub record: PaymentRecordId,
    pub source: LkiSnapshot,
    pub ability: usize,
    pub controller: PlayerId,
    pub profile: ActivatedManaProfile,
    pub tap_cost: bool,
    pub standalone: bool,
    pub submission: ManaActionSubmission,
    pub production_facts_emitted: bool,
    pub transcript: DecisionTranscript,
    pub facts: Vec<GameEvent>,
    pub cost_records: Vec<TransactionRecord>,
    pub reversal_barriers: Vec<super::ReversalBarrier>,
    pub observation_barriers: Vec<super::ObservationBarrier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ManaActionSubmission {
    Proposed,
    Submitted,
}

fn produced_units_for_action<'a>(
    facts: impl IntoIterator<Item = &'a GameEvent>,
    action: ManaActionId,
) -> Vec<crate::player::ManaUnit> {
    facts
        .into_iter()
        .filter_map(|fact| match fact {
            GameEvent::ManaAdded(event) if event.provenance.action == Some(action) => Some(event),
            _ => None,
        })
        .flat_map(|event| {
            event.units.iter().map(|&unit| crate::player::ManaUnit {
                id: unit,
                kind: event.mana,
                riders: event.riders.clone(),
                provenance: event.provenance,
            })
        })
        .collect()
}

impl ManaAction {
    pub(crate) fn is_submitted(&self) -> bool {
        self.submission == ManaActionSubmission::Submitted
    }
}

impl GameState {
    /// Whether an announcement-time modal selection keeps the current mana
    /// action inside the ability's derived mana-ability profile.
    pub(crate) fn payment_mana_modes_legal(&self, modes: &[deckmaste_core::Uint]) -> bool {
        if let Some(action) = self.payment.as_ref().and_then(|controller| {
            let action = controller.mana_actions.last()?;
            controller
                .frames
                .last()
                .is_some_and(|frame| frame.mana_action == Some(action.id))
                .then_some(action)
        }) {
            return match &action.profile {
                ActivatedManaProfile::Always => true,
                ActivatedManaProfile::ByAnnouncedMode(classes) => {
                    let selected: Option<Vec<_>> = modes
                        .iter()
                        .map(|&mode| {
                            usize::try_from(mode)
                                .ok()
                                .and_then(|i| classes.get(i).copied())
                        })
                        .collect();
                    selected.is_some_and(|selected| {
                        !selected.is_empty()
                            && selected.iter().all(|class| class.targetless)
                            && selected.iter().any(|class| class.adds_mana)
                    })
                }
            };
        }

        let Some((source, ability_index, compiled)) = self.pending_root_modal_mana_activation()
        else {
            return true;
        };
        let payload = compiled
            .as_activated()
            .expect("the root modal mana context is an activated ability");
        let controller = self
            .announcing
            .as_ref()
            .expect("the root modal mana context has an announcement")
            .controller;
        let layers = self.layers();
        if compiled.mana_profile_for_modes(modes) {
            self.can_activate_mana(&layers, controller, source, ability_index, payload)
        } else {
            // The initial priority offer is existential over modal completions:
            // one qualifying mana mode exempts the mixed ability from blanket
            // activation lockouts. Once an ordinary mode is chosen, re-run the
            // full non-mana gate before committing that choice.
            self.can_activate(&layers, controller, source, ability_index, payload)
        }
    }

    fn pending_root_modal_mana_activation(
        &self,
    ) -> Option<(ObjectId, usize, deckmaste_core::Ability)> {
        let controller = self.payment.as_ref()?;
        if controller.frames.len() != 1
            || controller
                .frames
                .last()
                .is_none_or(|frame| frame.mana_action.is_some())
        {
            return None;
        }
        let pending = self.announcing.as_ref()?;
        let StackObject::Activated { source, .. } = &pending.object else {
            return None;
        };
        let source = *source;
        let ability_index = self.agenda.iter().find_map(|item| match item {
            WorkItem::Emit(Occurrence::Single(GameEvent::AbilityActivated(event)))
                if event.source == source =>
            {
                Some(event.ability)
            }
            _ => None,
        })?;
        let compiled = crate::derive::usable_abilities(self, source)
            .get(ability_index)?
            .clone();
        matches!(
            compiled.mana_profile(),
            Some(ActivatedManaProfile::ByAnnouncedMode(_))
        )
        .then_some((source, ability_index, compiled))
    }

    /// Rebuild the active payment prompt from the live working image. A parent
    /// prompt restored after a child mana ability must expose the new pool and
    /// the newly tapped/sacrificed board, not its pre-child display snapshot.
    pub(super) fn refresh_payment_prompt(&mut self) {
        let mana_abilities = self.legal_payment_mana_abilities();
        let Some(controller) = self.payment.as_mut() else {
            return;
        };
        let Some(frame) = controller.frames.last_mut() else {
            return;
        };
        if matches!(
            frame.working.pending,
            Some(crate::decide::DecisionPointKind::Payment(_))
        ) {
            let prompt = frame.prompt(mana_abilities);
            frame.working.pending = Some(crate::decide::DecisionPointKind::Payment(prompt));
        }
    }

    fn legal_payment_mana_abilities(&self) -> Vec<(ObjectId, usize)> {
        let Some(controller) = self.payment.as_ref() else {
            return Vec::new();
        };
        let Some(frame) = controller.frames.last() else {
            return Vec::new();
        };
        if frame.stage != PaymentStage::PrePayment || frame.progress != PaymentProgress::Idle {
            return Vec::new();
        }
        let payer = frame.payer;
        let layers = self.layers();
        let mut legal = Vec::new();
        for &source in &self.zones.battlefield {
            if layers.controller(source) != payer {
                continue;
            }
            for (ability, compiled) in crate::derive::usable_abilities(self, source)
                .iter()
                .enumerate()
            {
                if !compiled.is_activated_mana_ability() {
                    continue;
                }
                let payload = compiled
                    .as_activated()
                    .expect("an activated mana ability has an activated payload");
                if controller
                    .mana_actions
                    .iter()
                    .any(|active| active.source.object == source && active.ability == ability)
                {
                    continue;
                }
                if self.can_activate_mana(&layers, payer, source, ability, payload) {
                    legal.push((source, ability));
                }
            }
        }
        legal
    }

    /// Start an activated mana ability chosen from an ordinary priority
    /// prompt. `submit_decision` has already opened the root proposal image;
    /// unlike a nested action, this action owns that root frame directly.
    pub(crate) fn activate_root_mana_ability(&mut self, source: ObjectId, ability: usize) {
        let compiled = crate::derive::usable_abilities(self, source)
            .get(ability)
            .cloned()
            .expect("a legal priority action keeps its derived ability index");
        let profile = compiled
            .mana_profile()
            .expect("the root mana path receives an activated mana ability");
        let id = self.register_root_mana_action(source, ability, profile);

        let mut items = vec![
            WorkItem::BeginActivate {
                object: source,
                ability,
            },
            WorkItem::AnnounceModes,
            WorkItem::AnnounceOptionalCosts { index: 0 },
            WorkItem::AnnounceX,
            WorkItem::AnnounceTargets,
            WorkItem::ChooseCostOptions,
            WorkItem::OpenPayment,
            WorkItem::BeginManaAction(id),
        ];
        items.extend(Self::priority_tail());
        self.schedule_front(items);
    }

    /// Route a conditionally classified root activation after its modes have
    /// been announced. A qualifying selection replaces only the ordinary
    /// becomes-activated work item; an ordinary/targeted/library-moving mode
    /// keeps the normal stack path.
    pub(crate) fn route_root_modal_mana_mode(&mut self, modes: &[deckmaste_core::Uint]) {
        let Some((source, ability_index, compiled)) = self.pending_root_modal_mana_activation()
        else {
            return;
        };
        let Some(profile) = compiled.mana_profile() else {
            return;
        };
        if !compiled.mana_profile_for_modes(modes) {
            return;
        }
        let id = self.register_root_mana_action(source, ability_index, profile);
        let queued = self.agenda.iter_mut().find(|item| {
            matches!(
                item,
                WorkItem::Emit(Occurrence::Single(GameEvent::AbilityActivated(event)))
                    if event.source == source && event.ability == ability_index
            )
        });
        *queued.expect("a root activation keeps its becomes-activated continuation queued") =
            WorkItem::BeginManaAction(id);
    }

    fn register_root_mana_action(
        &mut self,
        source: ObjectId,
        ability: usize,
        profile: ActivatedManaProfile,
    ) -> ManaActionId {
        let compiled = crate::derive::usable_abilities(self, source)
            .get(ability)
            .cloned()
            .expect("a legal priority action keeps its derived ability index");
        let tap_cost = crate::activate::cost_summary(
            crate::activate::as_activated(&compiled)
                .expect("an activated mana wrapper contains an activated payload")
                .cost
                .as_ref(),
        )
        .expect("priority legality validated the cost structure")
        .tap;
        let controller_player = self.objects.obj(source).controller;
        let source_snapshot = LkiSnapshot::capture(self, source);
        let id = self.mint_mana_action();
        let record = self.mint_payment_record();
        self.begin_mana_resolution_scope();
        let controller = self
            .payment
            .as_mut()
            .expect("a root mana proposal has an active controller");
        assert_eq!(controller.frames.len(), 1, "priority opens one root frame");
        let frame = controller.frames.last_mut().expect("the root frame exists");
        assert!(frame.mana_action.is_none());
        frame.mana_action = Some(id);
        controller.mana_actions.push(ManaAction {
            id,
            record,
            source: source_snapshot,
            ability,
            controller: controller_player,
            profile,
            tap_cost,
            standalone: true,
            submission: ManaActionSubmission::Proposed,
            production_facts_emitted: false,
            transcript: DecisionTranscript::default(),
            facts: Vec::new(),
            cost_records: Vec::new(),
            reversal_barriers: Vec::new(),
            observation_barriers: Vec::new(),
        });
        id
    }

    pub(super) fn activate_payment_mana_ability(
        &mut self,
        source: ObjectId,
        ability: usize,
    ) -> Result<(), DecisionError> {
        if !self
            .legal_payment_mana_abilities()
            .contains(&(source, ability))
        {
            return Err(DecisionError::Illegal {
                reason: "that activated mana ability is not legal in this prepayment window".into(),
            });
        }
        let compiled = crate::derive::usable_abilities(self, source)
            .get(ability)
            .cloned()
            .expect("the offered derived ability index remains live");
        let profile = compiled
            .mana_profile()
            .expect("the offered ability classifies as activated mana");
        let tap_cost = crate::activate::cost_summary(
            crate::activate::as_activated(&compiled)
                .expect("an activated mana ability has an activated payload")
                .cost
                .as_ref(),
        )
        .expect("can_activate_mana validated the cost structure")
        .tap;
        let controller_player = self.objects.obj(source).controller;
        let source_snapshot = LkiSnapshot::capture(self, source);

        // Validation above is read-only. The child starts from a full clone of
        // the parent's current working image; the parent's controller slots are
        // suspended inside that clone while ordinary announcement machinery
        // runs.
        let mut working = self.active().clone();
        working.suspend_control();
        working.begin_mana_resolution_scope();
        let id = self.mint_mana_action();
        let record = self.mint_payment_record();
        let controller = self
            .payment
            .as_mut()
            .expect("a payment command has an active controller");
        controller.mana_actions.push(ManaAction {
            id,
            record,
            source: source_snapshot,
            ability,
            controller: controller_player,
            profile,
            tap_cost,
            standalone: false,
            submission: ManaActionSubmission::Proposed,
            production_facts_emitted: false,
            transcript: DecisionTranscript::default(),
            facts: Vec::new(),
            cost_records: Vec::new(),
            reversal_barriers: Vec::new(),
            observation_barriers: Vec::new(),
        });
        let mut child = PaymentFrame::proposal(working, controller_player);
        child.activations.clone_from(&self.activations.borrow());
        child.next_activation = self.next_activation.get();
        child.mana_action = Some(id);
        controller.frames.push(child);

        self.schedule_front(vec![
            WorkItem::BeginActivate {
                object: source,
                ability,
            },
            WorkItem::AnnounceModes,
            WorkItem::AnnounceOptionalCosts { index: 0 },
            WorkItem::AnnounceX,
            WorkItem::AnnounceTargets,
            WorkItem::ChooseCostOptions,
            WorkItem::OpenPayment,
            WorkItem::BeginManaAction(id),
        ]);
        Ok(())
    }

    pub(crate) fn begin_mana_action(&mut self, id: ManaActionId) {
        let action = self
            .payment
            .as_ref()
            .and_then(|controller| {
                controller
                    .mana_actions
                    .iter()
                    .find(|action| action.id == id)
            })
            .cloned()
            .expect("a submitted mana action remains registered until its finish sentinel");
        let pending = self
            .announcing
            .take()
            .expect("a submitted activated mana ability remains announced");
        let StackObject::Activated {
            source,
            ability,
            bindings,
        } = pending.object
        else {
            panic!("a mana action is an activated ability announcement")
        };
        assert_eq!(source, action.source.object);
        assert_eq!(pending.controller, action.controller);
        let selected_is_mana = self.payment_mana_modes_legal(&pending.chosen_modes);
        assert!(
            selected_is_mana,
            "a nested payment activation must announce a lowering-classified mana mode"
        );

        let mut frame = ExecutionFrame {
            activation: pending.activation,
            payment: None,
        };
        self.frame_set_event_extras(
            &mut frame,
            bindings.event_amount,
            bindings.produced_mana,
            bindings.crossed,
        );
        self.objects.remove(pending.id);
        self.resolving_mana_actions.push(id);
        let mut items = vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ManaAbilityActivated(ManaAbilityActivated {
                source: action.source,
                ability: action.ability,
                controller: action.controller,
                action: id,
            }),
        ))];
        items.extend(crate::cast::announced_effect_items(
            self,
            &ability.effect,
            &frame,
            pending.chosen_modes.as_ref(),
            &pending.targets,
        ));
        items.push(WorkItem::FinishManaAction(id));
        self.schedule_front(items);
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the stackless completion boundary keeps emission, trigger drain, recording, and child promotion in causal order"
    )]
    pub(crate) fn finish_mana_action(&mut self, id: ManaActionId) {
        let action = self
            .payment
            .as_ref()
            .and_then(|controller| controller.mana_actions.last())
            .cloned()
            .expect("a finishing activated mana action remains registered");
        let produced = produced_units_for_action(&action.facts, id);
        if !action.production_facts_emitted && !produced.is_empty() {
            self.payment
                .as_mut()
                .expect("controller remains live")
                .mana_actions
                .last_mut()
                .expect("action remains live")
                .production_facts_emitted = true;
            let mut items = vec![WorkItem::Emit(Occurrence::single(GameEvent::ManaProduced(
                ManaProduced {
                    source: action.source.clone(),
                    controller: action.controller,
                    action: id,
                    produced: produced.clone(),
                },
            )))];
            if action.tap_cost {
                items.push(WorkItem::Emit(Occurrence::single(
                    GameEvent::TappedForMana(TappedForMana {
                        source: action.source,
                        controller: action.controller,
                        action: id,
                        produced,
                    }),
                )));
            }
            items.push(WorkItem::FinishManaAction(id));
            self.schedule_front(items);
            return;
        }
        if self.resume_after_deferred_triggered_mana(WorkItem::FinishManaAction(id)) {
            return;
        }
        self.finish_mana_resolution_scope();
        assert_eq!(
            self.resolving_mana_actions.pop(),
            Some(id),
            "mana actions finish in stackless resolution order"
        );
        let spent_by_action: Vec<_> = action
            .cost_records
            .iter()
            .flat_map(|record| record.spent_mana.iter().copied())
            .collect();
        let mut dependencies: Vec<_> = self
            .payment
            .as_ref()
            .into_iter()
            .flat_map(|controller| controller.frames.iter())
            .flat_map(|frame| frame.records.iter())
            .filter(|record| {
                record
                    .produced_mana
                    .iter()
                    .any(|mana| spent_by_action.contains(mana))
            })
            .map(|record| record.id)
            .collect();
        dependencies.sort_unstable();
        dependencies.dedup();
        let source = self.payment_logical_source(action.source.source);
        let mut object_inputs =
            self.payment_record_object_inputs(&action.facts, std::collections::HashMap::new());
        if matches!(source, LogicalObject::Created { .. }) {
            object_inputs.insert(action.source.object, source);
        }
        let transaction = (!action.standalone).then(|| {
            let produced_mana =
                action
                    .facts
                    .iter()
                    .filter_map(|fact| match fact {
                        GameEvent::ManaAdded(event) => {
                            Some(event.units.iter().copied().map(|id| {
                                super::replay::QualifiedManaId {
                                    player: event.player,
                                    id,
                                }
                            }))
                        }
                        _ => None,
                    })
                    .flatten()
                    .collect();
            let spent_mana = spent_by_action;
            let mut reversal_barriers = action.reversal_barriers.clone();
            let mut observation_barriers = action.observation_barriers.clone();
            reversal_barriers.extend(
                action
                    .cost_records
                    .iter()
                    .flat_map(|record| record.reversal_barriers.iter().copied()),
            );
            observation_barriers.extend(
                action
                    .cost_records
                    .iter()
                    .flat_map(|record| record.observation_barriers.iter().copied()),
            );
            reversal_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
            reversal_barriers.dedup();
            observation_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
            observation_barriers.dedup();
            TransactionRecord {
                id: action.record,
                command: super::ReplayCommand::ManaAbility {
                    action: action.id,
                    source,
                    ability: action.ability,
                    submitted: true,
                    completed: true,
                },
                transcript: action.transcript.clone(),
                object_inputs,
                children: action.cost_records.clone(),
                facts: action.facts.clone(),
                produced_mana,
                spent_mana,
                dependencies,
                reversal_barriers,
                observation_barriers,
            }
        });
        if action.standalone {
            let mut controller = self.payment.take().expect("root controller remains live");
            let action = controller
                .mana_actions
                .pop()
                .expect("a finishing root mana action remains registered");
            assert_eq!(action.id, id, "mana actions finish last-in-first-out");
            let frame = controller.frames.pop().expect("root frame remains live");
            assert_eq!(frame.mana_action, Some(id));
            self.committed = frame.working;
            self.clear_payment_metadata();
        } else {
            debug_assert!(
                action.is_submitted(),
                "only a submitted child resolves its mana effect"
            );
            {
                let controller = self
                    .payment
                    .as_mut()
                    .expect("a nested mana action has a payment controller");
                let action = controller
                    .mana_actions
                    .pop()
                    .expect("a finishing nested mana action remains registered");
                assert_eq!(action.id, id, "mana actions finish last-in-first-out");
            }
            self.resume_control()
                .expect("the completed child leaves no active control slot");
            let controller = self
                .payment
                .as_mut()
                .expect("a nested mana action has a payment controller");
            let child = controller
                .frames
                .pop()
                .expect("the completed child remains isolated until this boundary");
            assert_eq!(child.mana_action, Some(id));
            let parent = controller
                .frames
                .last_mut()
                .expect("the completed child resumes its parent frame");
            parent.working = child.working;
            parent.records.extend(child.records);
            parent
                .records
                .push(transaction.expect("a nested action has a transaction record"));
            self.refresh_payment_prompt();
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "the caller transfers the triggered ability into this payment action boundary"
    )]
    pub(crate) fn begin_triggered_mana_action(
        &mut self,
        _source: ObjectSource,
        ability_index: usize,
        triggered: Arc<deckmaste_core::TriggeredAbility>,
        controller: PlayerId,
        bindings: TriggerBindings,
    ) -> (ManaActionId, bool) {
        let id = self.mint_mana_action();
        let source = bindings
            .this
            .as_ref()
            .map_or_else(|| self.player(controller).object, |this| this.object);
        let ability_u =
            deckmaste_core::Uint::try_from(ability_index).expect("ability index fits in Uint");
        if bindings.this.is_some()
            && triggered.limits.iter().any(|limit| {
                let lookback = match limit {
                    deckmaste_core::UseLimit::OncePerTurn
                    | deckmaste_core::UseLimit::LoyaltyOncePerTurn => {
                        deckmaste_core::Lookback::ThisTurn
                    }
                    deckmaste_core::UseLimit::OncePerGame => deckmaste_core::Lookback::ThisGame,
                };
                self.ability_used_count(source, ability_u, lookback) >= 1
            })
        {
            return (id, false);
        }
        self.begin_mana_resolution_scope();
        self.resolving_mana_actions.push(id);
        self.record_history_fact(
            self.turn.turn_number,
            None,
            GameEvent::AbilityUsed(crate::event::AbilityUsed {
                object: source,
                ability: ability_u,
            }),
        );
        let mut seed = self.frame(source, controller);
        self.frame_set_source_lki(&mut seed, bindings.this.clone());
        self.frame_set_defending_player(&mut seed, bindings.defending_player);
        self.frame_set_event_bindings(
            &mut seed,
            bindings.that_object.clone(),
            bindings.that_player,
            bindings.that_patient.clone(),
        );
        let mut effect = triggered.effect.clone();
        if effect.params.is_empty() {
            effect.params = deckmaste_core::triggered_region_params(triggered.targets.len());
        }
        let activation = self.enter_created_region(&effect, &seed, &bindings.captures);
        let mut frame = ExecutionFrame {
            activation,
            payment: None,
        };
        self.frame_set_event_extras(
            &mut frame,
            bindings.event_amount,
            bindings.produced_mana,
            bindings.crossed,
        );
        // [CR#702.21b]: the toll's X is priced now, after the event roles are
        // in place, into the region's declared announced-X parameter.
        if let Some(where_x) = &triggered.where_x {
            let x = self.eval_count(where_x, &frame);
            self.activation_set_x(activation, x);
        }
        let should_resolve = triggered
            .condition
            .as_ref()
            .is_none_or(|condition| self.condition_holds(condition, &frame));
        let mut items = Vec::new();
        if should_resolve {
            items.extend(crate::cast::announced_effect_items(
                self,
                &triggered.effect,
                &frame,
                &[],
                &[],
            ));
        }
        let source_snapshot = bindings
            .this
            .expect("a triggered mana ability carries its source snapshot");
        items.push(WorkItem::FinishTriggeredMana {
            action: id,
            source: source_snapshot,
            controller,
        });
        self.schedule_front(items);
        (id, true)
    }

    pub(crate) fn finish_triggered_mana_action(
        &mut self,
        id: ManaActionId,
        source: LkiSnapshot,
        controller: PlayerId,
    ) {
        let produced =
            produced_units_for_action(self.history.entries().map(|entry| &entry.fact), id);
        if !produced.is_empty() {
            self.schedule_front(vec![
                WorkItem::Emit(Occurrence::single(GameEvent::ManaProduced(ManaProduced {
                    source,
                    controller,
                    action: id,
                    produced,
                }))),
                WorkItem::CompleteTriggeredMana(id),
            ]);
            return;
        }
        self.complete_triggered_mana_action(id);
    }

    pub(crate) fn complete_triggered_mana_action(&mut self, id: ManaActionId) {
        if self.resume_after_deferred_triggered_mana(WorkItem::CompleteTriggeredMana(id)) {
            return;
        }
        self.finish_mana_resolution_scope();
        assert_eq!(
            self.resolving_mana_actions.pop(),
            Some(id),
            "triggered mana actions finish in immediate causal order"
        );
    }

    fn resume_after_deferred_triggered_mana(&mut self, continuation: WorkItem) -> bool {
        let mut deferred = Vec::new();
        while matches!(
            self.agenda.front(),
            Some(WorkItem::ResolveTriggeredMana { .. })
        ) {
            deferred.push(
                self.agenda
                    .pop_front()
                    .expect("the checked deferred trigger remains queued"),
            );
        }
        if deferred.is_empty() {
            return false;
        }
        deferred.push(continuation);
        self.schedule_front(deferred);
        true
    }
}
use std::sync::Arc;
