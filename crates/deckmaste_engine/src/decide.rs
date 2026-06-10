use std::collections::HashSet;
use std::fmt;

use deckmaste_core::{KeywordAbility, Uint};

use crate::object::{ObjectId, ObjectSource};
use crate::player::PlayerId;

/// What the engine is waiting on. `step()` returns `NeedsDecision` (without
/// mutating) until `submit_decision` answers it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingDecision {
    /// [CR#117]: the holder may act or pass. `legal` is advisory UI data —
    /// submission re-validates.
    Priority {
        player: PlayerId,
        legal: Vec<Action>,
    },
    /// [CR#514.1]: discard down to maximum hand size.
    DiscardToHandSize { player: PlayerId, count: Uint },
    /// [CR#701.9b]: a resolving discard — `player` chooses which `count` cards
    /// from their hand to discard (`count` already clamped to the hand size).
    DiscardCards { player: PlayerId, count: Uint },
    /// [CR#106.1b]: a resolving `AddMana` whose production is a choice ("any
    /// color" offers the five colors per [CR#105.4]; "{W} or {U}" offers its
    /// printed set) — `player` picks one of `options`.
    ChooseManaColor {
        player: PlayerId,
        options: Vec<deckmaste_core::ColorOrColorless>,
        amount: Uint,
    },
    /// [CR#601.2c,115]: choose targets for the in-flight announce. `legal[i]`
    /// is the candidate set for `spec[i]`; `submit_decision` re-validates.
    ChooseTargets {
        player: PlayerId,
        spec: Vec<deckmaste_core::TargetSpec>,
        legal: Vec<Vec<ObjectId>>,
    },
    /// [CR#601.2g]: allocate pool mana to the in-flight cost.
    PayMana {
        player: PlayerId,
        cost: deckmaste_core::ManaCost,
        pool: crate::player::ManaPool,
    },
    /// [CR#603.3b]: a player controlling several simultaneous triggers orders
    /// them. The submitted `Order` is a permutation of `0..triggers.len()`.
    OrderTriggers {
        player: PlayerId,
        triggers: Vec<crate::trigger::NotedTrigger>,
    },
    /// [CR#508.1a]: the active player declares attackers. `legal` is the
    /// surfaced candidate set; `submit_decision` re-validates against it.
    DeclareAttackers {
        player: PlayerId,
        legal: Vec<ObjectId>,
    },
    /// [CR#509.1a]: the **defending** player declares blockers. `player` is the
    /// defender (the non-active player); `legal` is the surfaced candidate set
    /// of legal blockers; `submit_decision` re-validates against it.
    DeclareBlockers {
        player: PlayerId,
        legal: Vec<ObjectId>,
    },
    /// [CR#510.1c]: divide `source`'s combat damage among its `recipients`
    /// (free division — any split summing to `source`'s power is legal).
    /// Surfaced only for a multi-blocked attacker (≥ 2 recipients); forced
    /// cases auto-resolve. `player` is the source's controller.
    AssignCombatDamage {
        player: PlayerId,
        source: ObjectId,
        recipients: Vec<ObjectId>,
    },
}

/// An answer to the pending decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Answers `Priority`.
    Act(Action),
    /// Answers `DiscardToHandSize` and `DiscardCards`: which cards to discard.
    Discard(Vec<ObjectId>),
    /// Answers `ChooseManaColor`: the chosen mana.
    ManaColor(deckmaste_core::ColorOrColorless),
    /// Answers `ChooseTargets`: one chosen object per `TargetSpec`.
    Targets(Vec<ObjectId>),
    /// Answers `PayMana`: how the pool covers the cost.
    Pay(crate::cast::Payment),
    /// Answers `OrderTriggers`: a permutation of `0..triggers.len()` giving the
    /// placement order ([CR#603.3b]).
    Order(Vec<usize>),
    /// Answers `DeclareAttackers`: which creatures attack (possibly empty).
    Attackers(Vec<ObjectId>),
    /// Answers `DeclareBlockers`: `(blocker, the attacker it blocks)` pairs
    /// (possibly empty). Each blocker blocks exactly one attacker
    /// ([CR#509.1a]).
    Blocks(Vec<(ObjectId, ObjectId)>),
    /// Answers `AssignCombatDamage`: `(recipient, amount)` pairs whose amounts
    /// sum to the source's power, each recipient drawn from the offered set
    /// ([CR#510.1c]).
    Assignment(Vec<(ObjectId, Uint)>),
}

/// What a priority holder can do in the skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Pass,
    /// Special action, no stack ([CR#116.2a,305]).
    PlayLand {
        object: ObjectId,
    },
    /// Skeleton: mana abilities only — no stack ([CR#605.3a]). `ability`
    /// indexes the object's derived ability list.
    ActivateAbility {
        object: ObjectId,
        ability: usize,
    },
    /// Cast a spell from hand ([CR#601]). The announce block (targets, cost) is
    /// reified onto the agenda by `take_priority_action`.
    CastSpell {
        object: ObjectId,
    },
}

/// Why a submission was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionError {
    NothingPending,
    /// The decision kind doesn't answer the pending decision.
    WrongKind,
    Illegal {
        reason: String,
    },
}

impl fmt::Display for DecisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionError::NothingPending => f.write_str("no decision is pending"),
            DecisionError::WrongKind => f.write_str("decision doesn't answer what's pending"),
            DecisionError::Illegal { reason } => write!(f, "illegal: {reason}"),
        }
    }
}

impl std::error::Error for DecisionError {}

use crate::agenda::WorkItem;
use crate::derive;
use crate::event::{GameEvent, Occurrence};
use crate::state::GameState;

impl GameState {
    /// Answers the pending decision: validates, does the decision's
    /// bookkeeping, schedules the continuation at the agenda front, and
    /// clears `pending`. On error the decision stays pending.
    ///
    /// # Errors
    ///
    /// `NothingPending` with no decision open, `WrongKind` when the answer
    /// doesn't match the question, `Illegal` when it fails validation.
    ///
    /// # Panics
    ///
    /// Panics if a `ChooseTargets` decision is answered while no announce is in
    /// flight — an engine invariant (the announce slot is open across the
    /// decision), not caller input.
    // TODO: split by decision kind (priority / cast-procedure / combat) as the
    //   match grows past one screen.
    #[expect(clippy::too_many_lines)]
    pub fn submit_decision(&mut self, decision: Decision) -> Result<(), DecisionError> {
        let Some(pending) = &self.pending else {
            return Err(DecisionError::NothingPending);
        };
        match (pending, decision) {
            (PendingDecision::Priority { player, legal }, Decision::Act(action)) => {
                if !legal.contains(&action) {
                    return Err(DecisionError::Illegal {
                        reason: format!("{action:?} is not a legal action right now"),
                    });
                }
                let player = *player;
                self.pending = None;
                self.take_priority_action(player, &action);
                Ok(())
            }
            (
                PendingDecision::DiscardToHandSize { player, count }
                | PendingDecision::DiscardCards { player, count },
                Decision::Discard(objects),
            ) => {
                let (player, count) = (*player, *count);
                self.submit_discards(player, count, objects)
            }
            (
                PendingDecision::ChooseManaColor {
                    player,
                    options,
                    amount,
                },
                Decision::ManaColor(mana),
            ) => {
                // [CR#106.1b]: the choice is drawn from the offered set.
                if !options.contains(&mana) {
                    return Err(DecisionError::Illegal {
                        reason: format!("{mana:?} is not one of the offered mana options"),
                    });
                }
                let (player, amount) = (*player, *amount);
                self.pending = None;
                self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::ManaAdded {
                        player,
                        mana,
                        amount,
                    },
                ))]);
                Ok(())
            }
            (
                PendingDecision::ChooseTargets {
                    player: _,
                    spec,
                    legal,
                },
                Decision::Targets(chosen),
            ) => {
                // [CR#601.2c,115] / [CR#603.3d]: one chosen object per spec,
                // each drawn from that spec's legal candidate set.
                if chosen.len() != spec.len()
                    || chosen.iter().zip(legal).any(|(c, set)| !set.contains(c))
                {
                    return Err(DecisionError::Illegal {
                        reason: "illegal target selection".into(),
                    });
                }
                self.pending = None;
                if self.placing_trigger.is_some() {
                    // [CR#603.3d]: a triggered ability chose its targets at
                    // placement — commit it onto the stack and resume placement.
                    self.commit_placing_trigger(chosen);
                    self.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
                } else {
                    self.announcing
                        .as_mut()
                        .expect("an announce in flight")
                        .targets = chosen;
                }
                Ok(())
            }
            (
                PendingDecision::PayMana {
                    player,
                    cost,
                    pool: _,
                },
                Decision::Pay(payment),
            ) => {
                let player = *player;
                let cost = cost.clone();
                if !crate::cast::validate_payment(&self.player(player).mana_pool, &cost, &payment) {
                    return Err(DecisionError::Illegal {
                        reason: "payment does not cover the cost".into(),
                    });
                }
                self.pending = None;
                crate::cast::apply_payment(&mut self.player_mut(player).mana_pool, &cost, &payment);
                Ok(())
            }
            (PendingDecision::OrderTriggers { player, triggers }, Decision::Order(order)) => {
                let (player, triggers) = (*player, triggers.clone());
                self.submit_order_triggers(player, &triggers, &order)
            }
            (
                PendingDecision::DeclareAttackers { player: _, legal },
                Decision::Attackers(chosen),
            ) => {
                // [CR#508.1a]: each chosen creature must be in the surfaced
                // legal set, and no creature attacks twice.
                let distinct: HashSet<_> = chosen.iter().copied().collect();
                if distinct.len() != chosen.len() || !chosen.iter().all(|o| legal.contains(o)) {
                    return Err(DecisionError::Illegal {
                        reason: "attackers must be distinct, from the legal set".into(),
                    });
                }
                self.pending = None;
                // [CR#508.1f]: declaring taps the attacker. The whole declaration
                // is one simultaneous occurrence — a `Batch` (empty when no
                // attackers were declared, which schedules nothing observable).
                if !chosen.is_empty() {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(
                        chosen.into_iter().map(GameEvent::Attacking).collect(),
                    ))]);
                }
                Ok(())
            }
            (PendingDecision::DeclareBlockers { player: _, legal }, Decision::Blocks(pairs)) => {
                // [CR#509.1a]: each blocker is from the surfaced legal set, each
                // blocked creature is an attacker, and no creature blocks twice
                // (a creature blocks exactly one attacker).
                let distinct: HashSet<_> = pairs.iter().map(|&(b, _)| b).collect();
                let attackers = self.combat.attackers();
                if distinct.len() != pairs.len()
                    || !pairs
                        .iter()
                        .all(|(b, a)| legal.contains(b) && attackers.contains(a))
                {
                    return Err(DecisionError::Illegal {
                        reason: "each blocker (once) blocks an attacker from the legal set".into(),
                    });
                }
                self.pending = None;
                // [CR#509.1h]: the whole block declaration is one simultaneous
                // occurrence — a `Batch` (skipped when empty, which schedules
                // nothing observable).
                if !pairs.is_empty() {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(
                        pairs
                            .into_iter()
                            .map(|(blocker, attacker)| GameEvent::Blocked { blocker, attacker })
                            .collect(),
                    ))]);
                }
                Ok(())
            }
            (
                PendingDecision::AssignCombatDamage {
                    player: _,
                    source,
                    recipients,
                },
                Decision::Assignment(amounts),
            ) => {
                let source = *source;
                let recipients = recipients.clone();
                self.submit_assign_combat_damage(source, &recipients, amounts)
            }
            _ => Err(DecisionError::WrongKind),
        }
    }

    /// Shared by `DiscardToHandSize` ([CR#514.1]) and `DiscardCards`
    /// ([CR#701.9b]): validate that exactly `count` distinct in-hand cards were
    /// chosen, then emit one `Discarded` per card.
    ///
    /// # Errors
    ///
    /// `Illegal` when the chosen cards are not exactly `count` distinct cards
    /// from the player's hand.
    fn submit_discards(
        &mut self,
        player: PlayerId,
        count: Uint,
        objects: Vec<ObjectId>,
    ) -> Result<(), DecisionError> {
        let hand = &self.zones.hands[player.index()];
        let distinct: HashSet<_> = objects.iter().copied().collect();
        if objects.len() != count as usize
            || distinct.len() != objects.len()
            || !objects.iter().all(|o| hand.contains(o))
        {
            return Err(DecisionError::Illegal {
                reason: format!("discard exactly {count} distinct cards from hand"),
            });
        }
        self.pending = None;
        // The `Discarded` cause fact evolves into ZoneWillChange(Hand→Graveyard)
        // at apply ([CR#701.8], spec §5.6).
        self.schedule_front(
            objects
                .into_iter()
                .map(|object| {
                    WorkItem::Emit(Occurrence::single(GameEvent::Discarded { player, object }))
                })
                .collect(),
        );
        Ok(())
    }

    /// [CR#603.3b]: apply an `OrderTriggers` answer — validate `order` is a
    /// permutation of `0..triggers.len()`, reorder this player's noted
    /// triggers, and immediately place the FIRST one. Placement of subsequent
    /// triggers is deferred to future `PlaceTriggers` barrier passes (one per
    /// `step()`), each preceded by a `CheckSbas` re-sweep.
    ///
    /// Placing the first trigger here (instead of rescheduling `PlaceTriggers`
    /// and letting it see `mine.len() > 1` again) avoids the `OrderTriggers`
    /// re-surface loop.
    ///
    /// # Errors
    ///
    /// `Illegal` when `order` is not a permutation of the offered indices.
    fn submit_order_triggers(
        &mut self,
        player: PlayerId,
        triggers: &[crate::trigger::NotedTrigger],
        order: &[usize],
    ) -> Result<(), DecisionError> {
        let len = triggers.len();
        let distinct: HashSet<usize> = order.iter().copied().collect();
        if order.len() != len || distinct.len() != len || order.iter().any(|&i| i >= len) {
            return Err(DecisionError::Illegal {
                reason: format!("order must be a permutation of 0..{len}"),
            });
        }
        // Reorder this player's noted triggers to the chosen order, leaving
        // other players' notes untouched.
        let ordered: Vec<crate::trigger::NotedTrigger> =
            order.iter().map(|&i| triggers[i].clone()).collect();
        self.pending = None;
        self.reorder_pending_triggers(player, ordered);
        // Immediately place the FIRST trigger in the ordered sequence so the
        // next `PlaceTriggers` call sees one fewer trigger and does not
        // re-surface `OrderTriggers`. Schedule CheckSbas + PlaceTriggers so
        // subsequent triggers (and any new SBA state) are handled normally.
        let noted = self.take_first_trigger_of(player);
        let placed = self.place_one_trigger(noted);
        self.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
        let _ = placed; // placement success is tracked by CheckSbas/PlaceTriggers
        Ok(())
    }

    /// [CR#510.1c]: apply an `AssignCombatDamage` answer for the queue's front
    /// source. Validates that the amounts sum to the source's power and every
    /// named target is one of its recipients. Ordinary (non-trample) division
    /// is free — *any* such split is legal ([CR#510.1c]). For a **trample**
    /// source whose recipients include the defending player's proxy,
    /// [CR#702.19b] additionally requires that the player be assigned
    /// damage only once every blocker recipient has at least lethal —
    /// `lethal` is 1 when the source has deathtouch ([CR#702.2c]) and the
    /// blocker's toughness otherwise (less any damage already marked). On
    /// success it appends one `DamageDealt` per nonzero amount to the
    /// buffer, pops the source off the queue, and surfaces
    /// the next decision (or deals the batch when the queue empties) via
    /// `open_next_assignment`.
    ///
    /// # Errors
    ///
    /// `Illegal` when the amounts don't sum to the source's power, name a
    /// creature that isn't one of the source's recipients, repeat a recipient,
    /// or (trample) assign the player while a blocker recipient is below
    /// lethal.
    ///
    /// # Panics
    ///
    /// Panics if no combat-damage assignment is in flight, or the front source
    /// doesn't match `source` — engine invariants (the pending decision pins
    /// both), not caller input.
    fn submit_assign_combat_damage(
        &mut self,
        source: ObjectId,
        recipients: &[ObjectId],
        amounts: Vec<(ObjectId, Uint)>,
    ) -> Result<(), DecisionError> {
        let power = {
            let cd = self
                .combat_damage
                .as_ref()
                .expect("combat-damage in flight");
            let front = cd.queue.first().expect("a queued assignment");
            debug_assert_eq!(
                front.source, source,
                "front source matches the pending decision"
            );
            front.power
        };
        let total: Uint = amounts.iter().map(|&(_, n)| n).sum();
        if total != power {
            return Err(DecisionError::Illegal {
                reason: format!(
                    "assigned damage ({total}) must sum to the source's power ({power})"
                ),
            });
        }
        if !amounts.iter().all(|(t, _)| recipients.contains(t)) {
            return Err(DecisionError::Illegal {
                reason: "every assignment target must be one of the source's recipients".into(),
            });
        }
        let distinct: HashSet<ObjectId> = amounts.iter().map(|&(t, _)| t).collect();
        if distinct.len() != amounts.len() {
            return Err(DecisionError::Illegal {
                reason: "each recipient may appear at most once in a damage assignment".into(),
            });
        }
        // [CR#702.19b]: a trample source may assign damage to the defending
        // player only after every blocker recipient has lethal. Player proxies
        // among the recipients are identified by their `ObjectSource::Player`.
        let view = self.layers();
        if crate::combat::has_keyword(&view, source, &KeywordAbility::Trample) {
            let assigned = |id: ObjectId| {
                amounts
                    .iter()
                    .find(|&&(t, _)| t == id)
                    .map_or(0, |&(_, n)| n)
            };
            let to_player: Uint = recipients
                .iter()
                .filter(|&&r| matches!(self.objects.obj(r).source, ObjectSource::Player(_)))
                .map(|&r| assigned(r))
                .sum();
            if to_player > 0 {
                for &r in recipients {
                    if matches!(self.objects.obj(r).source, ObjectSource::Player(_)) {
                        continue; // the player, not a blocker
                    }
                    if assigned(r) < self.lethal_for(&view, source, r) {
                        return Err(DecisionError::Illegal {
                            reason: "trample: each blocker must be assigned lethal before the \
                                     defending player ([CR#702.19b])"
                                .into(),
                        });
                    }
                }
            }
        }
        self.pending = None;
        let cd = self
            .combat_damage
            .as_mut()
            .expect("combat-damage in flight");
        for (target, amount) in amounts {
            if amount > 0 {
                cd.buffer.push(GameEvent::DamageDealt {
                    source,
                    target,
                    amount,
                });
            }
        }
        cd.queue.remove(0);
        self.open_next_assignment();
        Ok(())
    }

    /// [CR#702.19b]: how much damage `source` must assign to the creature
    /// `blocker` before any excess may spill to the defending player. With
    /// deathtouch on the source, any nonzero amount is lethal, so `1`
    /// ([CR#702.2c]); otherwise the blocker's printed toughness, less any
    /// damage already marked on it (undamaged at assignment time in v1, but
    /// subtracted for correctness). A non-`Number` toughness (CDA / `*`) is
    /// treated as needing the source's whole power — effectively
    /// unsatisfiable below full assignment — by returning `Uint::MAX`;
    /// layers and `*`-toughness are a later stage.
    #[must_use]
    fn lethal_for(
        &self,
        view: &crate::layer::LayeredView,
        source: ObjectId,
        blocker: ObjectId,
    ) -> Uint {
        if crate::combat::has_keyword(view, source, &KeywordAbility::Deathtouch) {
            return 1; // [CR#702.2c]: any nonzero amount is lethal.
        }
        match view.toughness(blocker) {
            Some(t) if t > 0 => {
                #[expect(clippy::cast_sign_loss)]
                let toughness = t as Uint;
                toughness.saturating_sub(self.objects.obj(blocker).damage)
            }
            // toughness ≤ 0 is already a destroy SBA; None is a non-creature
            // or unmodeled case — require the full amount so the player can't
            // be reached.
            _ => Uint::MAX,
        }
    }

    /// The priority bookkeeping ([CR#117.3c,117.4]): a pass rotates or ends
    /// the round; an action emits, re-runs the barrier, and re-opens
    /// priority for the actor. Legality was checked by the caller.
    ///
    /// # Panics
    ///
    /// Panics if no priority round is open — engine invariant, not caller
    /// input.
    fn take_priority_action(&mut self, player: PlayerId, action: &Action) {
        match action {
            Action::Pass => {
                // Compute before borrowing the round mutably.
                let live = self.live_count();
                let next = self.next_live_after(player);
                let round = self.turn.priority.as_mut().expect("open priority round");
                round.consecutive_passes += 1;
                let all_passed = round.consecutive_passes >= live;
                if all_passed {
                    self.turn.priority = None;
                    if let Some(top) = self.stack.last() {
                        // [CR#608]: resolve the top; AP gets priority after
                        // ([CR#117.3b]). Keyed on `StackEntry.id` so a triggered
                        // ability (no backing object) resolves like a spell.
                        let id = top.id;
                        self.schedule_front(vec![
                            WorkItem::Resolve(id),
                            WorkItem::CheckSbas,
                            WorkItem::PlaceTriggers,
                            WorkItem::OpenPriority,
                        ]);
                    } else {
                        // [CR#117.4]: all-pass on an empty stack ends the step.
                        let items = self.end_of_step_items();
                        self.schedule_front(items);
                    }
                } else {
                    self.turn
                        .priority
                        .as_mut()
                        .expect("open priority round")
                        .holder = next;
                    self.schedule_front(vec![WorkItem::OpenPriority]);
                }
            }
            Action::PlayLand { object } => {
                self.reset_passes();
                // The `LandPlayed` cause fact bumps the land tally and evolves into
                // ZoneWillChange(Hand→Battlefield) at apply (spec §5.6).
                self.schedule_front(vec![
                    WorkItem::Emit(Occurrence::single(GameEvent::LandPlayed {
                        object: *object,
                    })),
                    WorkItem::CheckSbas,
                    WorkItem::PlaceTriggers,
                    WorkItem::OpenPriority,
                ]);
            }
            Action::ActivateAbility { object, ability } => {
                let abilities = derive::abilities(self, *object);
                let a = abilities.get(*ability).expect(
                    "ability index from the legal list is in bounds (state frozen by pending)",
                );
                self.reset_passes();
                if let Some((mana, amount)) = derive::tap_mana_ability(a) {
                    // [CR#605.3b]: mana abilities skip the stack entirely.
                    self.schedule_front(vec![
                        WorkItem::Emit(Occurrence::single(GameEvent::Tapped(*object))),
                        WorkItem::Emit(Occurrence::single(GameEvent::ManaAdded {
                            player,
                            mana,
                            amount,
                        })),
                        WorkItem::CheckSbas,
                        WorkItem::PlaceTriggers,
                        WorkItem::OpenPriority,
                    ]);
                } else {
                    // [CR#602.2b]: the casting steps, for an ability.
                    self.schedule_front(vec![
                        WorkItem::BeginActivate {
                            object: *object,
                            ability: *ability,
                        },
                        WorkItem::AnnounceTargets,
                        WorkItem::PayCost,
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityActivated {
                            source: *object,
                            ability: *ability,
                        })),
                        WorkItem::CheckSbas,
                        WorkItem::PlaceTriggers,
                        WorkItem::OpenPriority,
                    ]);
                }
            }
            Action::CastSpell { object } => {
                // [CR#601.2]: reify the announce procedure. Targets and cost are
                // chosen by the staged WorkItems (surfacing decisions when
                // there is a choice); `SpellCast` is the becomes-cast moment
                // ([CR#601.2i]) that promotes the announce onto the stack; the
                // caster then regains priority ([CR#117.3c]).
                self.reset_passes();
                self.schedule_front(vec![
                    WorkItem::BeginCast(*object),
                    WorkItem::AnnounceTargets,
                    WorkItem::PayCost,
                    WorkItem::Emit(Occurrence::single(GameEvent::SpellCast(*object))),
                    WorkItem::CheckSbas,
                    WorkItem::PlaceTriggers,
                    WorkItem::OpenPriority,
                ]);
            }
        }
    }

    /// [CR#117.3c]: taking an action restarts the pass count; the actor
    /// receives priority again afterward. `holder` is intentionally not
    /// touched — the scheduled `OpenPriority` reuses it as-is, and it is
    /// already the actor.
    ///
    /// # Panics
    ///
    /// Panics if no priority round is open — engine invariant, not caller
    /// input.
    fn reset_passes(&mut self) {
        self.turn
            .priority
            .as_mut()
            .expect("open priority round")
            .consecutive_passes = 0;
    }
}
