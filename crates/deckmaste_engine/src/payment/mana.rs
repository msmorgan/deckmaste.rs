use deckmaste_core::ActivatedManaProfile;
use deckmaste_core::ManaAbility;

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
use crate::stack::Anaphora;
use crate::stack::Frame;
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
    pub production_facts_emitted: bool,
    pub transcript: DecisionTranscript,
    pub facts: Vec<GameEvent>,
    pub cost_records: Vec<TransactionRecord>,
    pub reversal_barriers: Vec<super::ReversalBarrier>,
    pub observation_barriers: Vec<super::ObservationBarrier>,
}

impl GameState {
    /// Whether an announcement-time modal selection keeps the current mana
    /// action inside lowering's precomputed mana-ability profile.
    pub(crate) fn payment_mana_modes_legal(&self, modes: &[deckmaste_core::Uint]) -> bool {
        let Some(action) = self
            .payment
            .as_ref()
            .and_then(|controller| controller.mana_actions.last())
        else {
            return true;
        };
        match &action.profile {
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
                        && selected
                            .iter()
                            .all(|class| class.targetless && class.library_safe)
                        && selected.iter().any(|class| class.adds_mana)
                })
            }
        }
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
            Some(crate::decide::PendingDecision::Payment(_))
        ) {
            let prompt = frame.prompt(mana_abilities);
            frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
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
                let Some(ManaAbility::Activated {
                    ability: payload, ..
                }) = compiled.as_mana()
                else {
                    continue;
                };
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
        let ManaAbility::Activated { profile, .. } = compiled
            .as_mana()
            .expect("the root mana path receives a lowering-classified mana ability")
        else {
            unreachable!("a triggered mana ability is never a priority action")
        };
        let profile = profile.clone();
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
            production_facts_emitted: false,
            transcript: DecisionTranscript::default(),
            facts: Vec::new(),
            cost_records: Vec::new(),
            reversal_barriers: Vec::new(),
            observation_barriers: Vec::new(),
        });

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
        let ManaAbility::Activated { profile, .. } = compiled
            .as_mana()
            .expect("the offered ability is lowering-classified mana")
        else {
            unreachable!("payment prompts contain only activated mana abilities")
        };
        let profile = profile.clone();
        let tap_cost = crate::activate::cost_summary(
            crate::activate::as_activated(&compiled)
                .expect("an activated mana wrapper contains an activated payload")
                .cost
                .as_ref(),
        )
        .expect("can_activate_mana validated the cost structure")
        .tap;
        let controller_player = self.objects.obj(source).controller;
        let source_snapshot = LkiSnapshot::capture(self, source);

        // Validation above is read-only. The child starts from a full clone of
        // the parent's current working image; the parent's controller slots are
        // suspended inside that clone while ordinary announcement machinery runs.
        let mut working = self.active().clone();
        working.suspend_control();
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
            production_facts_emitted: false,
            transcript: DecisionTranscript::default(),
            facts: Vec::new(),
            cost_records: Vec::new(),
            reversal_barriers: Vec::new(),
            observation_barriers: Vec::new(),
        });
        let mut child = PaymentFrame::proposal(working, controller_player);
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

        let this = bindings
            .this
            .as_ref()
            .expect("begin_activate captures the source snapshot")
            .clone();
        let frame = Frame {
            source: this.object,
            controller: pending.controller,
            this: Some(this),
            defending_player: bindings.defending_player,
            payment: None,
            anaphora: Anaphora {
                targets: pending.targets.clone(),
                x: pending.x,
                that_object: bindings.that_object,
                that_player: bindings.that_player,
                that_patient: bindings.that_patient,
                produced_mana: bindings.produced_mana,
                ..Anaphora::empty()
            },
        };
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
            &ability.effect,
            &frame,
            pending.chosen_modes.as_ref(),
            &pending.targets,
        ));
        items.push(WorkItem::FinishManaAction(id));
        self.schedule_front(items);
    }

    pub(crate) fn finish_mana_action(&mut self, id: ManaActionId) {
        let action = self
            .payment
            .as_ref()
            .and_then(|controller| controller.mana_actions.last())
            .cloned()
            .expect("a finishing activated mana action remains registered");
        let produced: Vec<_> = self
            .player(action.controller)
            .mana_pool
            .units()
            .iter()
            .filter(|unit| unit.provenance.action == Some(id))
            .cloned()
            .collect();
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
        let transaction = (!action.standalone).then(|| {
            let produced_mana = action
                .facts
                .iter()
                .filter_map(|fact| match fact {
                    GameEvent::ManaAdded(event) => Some(event.units.iter().copied()),
                    _ => None,
                })
                .flatten()
                .collect();
            let spent_mana = spent_by_action;
            let mut reversal_barriers = action.reversal_barriers.clone();
            let mut observation_barriers = action.observation_barriers.clone();
            reversal_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
            reversal_barriers.dedup();
            observation_barriers.sort_unstable_by_key(|barrier| *barrier as u8);
            observation_barriers.dedup();
            let source = match action.source.source {
                ObjectSource::Card(card) => LogicalObject::Card(card),
                ObjectSource::Player(player) => LogicalObject::Player(player),
            };
            TransactionRecord {
                id: action.record,
                command: super::ReplayCommand::ManaAbility {
                    action: action.id,
                    source,
                    ability: action.ability,
                    submitted: true,
                },
                transcript: action.transcript.clone(),
                object_inputs: std::collections::HashMap::new(),
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
        } else {
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
            self.payment
                .as_mut()
                .expect("a nested mana action has a payment controller")
                .frames
                .last_mut()
                .expect("the completed child resumes its parent frame")
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
    ) -> ManaActionId {
        let id = self.mint_mana_action();
        self.resolving_mana_actions.push(id);
        let source = bindings
            .this
            .as_ref()
            .map_or_else(|| self.player(controller).object, |this| this.object);
        self.record_history_fact(
            self.turn.turn_number,
            None,
            GameEvent::AbilityUsed(crate::event::AbilityUsed {
                object: source,
                ability: deckmaste_core::Uint::try_from(ability_index)
                    .expect("ability index fits in Uint"),
            }),
        );
        let frame = Frame {
            source,
            controller,
            this: bindings.this.clone(),
            defending_player: bindings.defending_player,
            payment: None,
            anaphora: Anaphora {
                that_object: bindings.that_object,
                that_player: bindings.that_player,
                that_patient: bindings.that_patient,
                produced_mana: bindings.produced_mana,
                crossed: bindings.crossed,
                ..Anaphora::empty()
            },
        };
        let should_resolve = triggered
            .condition
            .as_ref()
            .is_none_or(|condition| self.condition_holds(condition, &frame));
        let mut items = Vec::new();
        if should_resolve {
            items.push(WorkItem::RunEffect {
                effect: Arc::new(triggered.effect.clone()),
                frame,
            });
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
        id
    }

    pub(crate) fn finish_triggered_mana_action(
        &mut self,
        id: ManaActionId,
        source: LkiSnapshot,
        controller: PlayerId,
    ) {
        let produced: Vec<_> = self
            .player(controller)
            .mana_pool
            .units()
            .iter()
            .filter(|unit| unit.provenance.action == Some(id))
            .cloned()
            .collect();
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
        assert_eq!(
            self.resolving_mana_actions.pop(),
            Some(id),
            "triggered mana actions finish in immediate causal order"
        );
    }
}
use std::sync::Arc;
