use std::collections::HashSet;
use std::fmt;

use deckmaste_core::Uint;

use crate::object::ObjectId;
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
    /// CR 601.2c / 115: choose targets for the in-flight announce. `legal[i]`
    /// is the candidate set for `spec[i]`; `submit_decision` re-validates.
    ChooseTargets {
        player: PlayerId,
        spec: Vec<deckmaste_core::TargetSpec>,
        legal: Vec<Vec<ObjectId>>,
    },
    /// CR 601.2g: allocate pool mana to the in-flight cost.
    PayMana {
        player: PlayerId,
        cost: deckmaste_core::ManaCost,
        pool: crate::player::ManaPool,
    },
}

/// An answer to the pending decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Answers `Priority`.
    Act(Action),
    /// Answers `DiscardToHandSize`: which cards to discard.
    Discard(Vec<ObjectId>),
    /// Answers `ChooseTargets`: one chosen object per `TargetSpec`.
    Targets(Vec<ObjectId>),
    /// Answers `PayMana`: how the pool covers the cost.
    Pay(crate::cast::Payment),
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
    /// Cast a spell from hand (CR 601). The announce block (targets, cost) is
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
use crate::event::GameEvent;
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
            (PendingDecision::DiscardToHandSize { player, count }, Decision::Discard(objects)) => {
                let (player, count) = (*player, *count);
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
                self.schedule_front(
                    objects
                        .into_iter()
                        .map(|object| WorkItem::Emit(GameEvent::Discarded { player, object }))
                        .collect(),
                );
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
                // CR 601.2c / 115: one chosen object per spec, each drawn from
                // that spec's legal candidate set.
                if chosen.len() != spec.len()
                    || chosen.iter().zip(legal).any(|(c, set)| !set.contains(c))
                {
                    return Err(DecisionError::Illegal {
                        reason: "illegal target selection".into(),
                    });
                }
                self.pending = None;
                self.announcing
                    .as_mut()
                    .expect("an announce in flight")
                    .targets = chosen;
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
            _ => Err(DecisionError::WrongKind),
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
                    // [CR#117.4]: all-pass on an empty stack ends the step.
                    self.turn.priority = None;
                    let items = self.end_of_step_items();
                    self.schedule_front(items);
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
                self.schedule_front(vec![
                    WorkItem::Emit(GameEvent::LandPlayed { object: *object }),
                    WorkItem::CheckSbas,
                    WorkItem::OpenPriority,
                ]);
            }
            Action::ActivateAbility { object, ability } => {
                let abilities = derive::abilities(self, *object);
                let ability = abilities.get(*ability).expect(
                    "ability index from the legal list is in bounds (state frozen by pending)",
                );
                let (mana, amount) = derive::tap_mana_ability(ability)
                    .expect("legality check admitted only tap-mana abilities");
                self.reset_passes();
                self.schedule_front(vec![
                    WorkItem::Emit(GameEvent::Tapped(*object)),
                    WorkItem::Emit(GameEvent::ManaAdded {
                        player,
                        mana,
                        amount,
                    }),
                    WorkItem::CheckSbas,
                    WorkItem::OpenPriority,
                ]);
            }
            Action::CastSpell { object } => {
                // CR 601.2: reify the announce procedure. Targets and cost are
                // chosen by the staged WorkItems (surfacing decisions when
                // there is a choice); `SpellCast` is the becomes-cast moment
                // (601.2i) that promotes the announce onto the stack; the
                // caster then regains priority (CR 117.3c).
                self.reset_passes();
                self.schedule_front(vec![
                    WorkItem::BeginCast(*object),
                    WorkItem::AnnounceTargets,
                    WorkItem::PayCost,
                    WorkItem::Emit(GameEvent::SpellCast(*object)),
                    WorkItem::CheckSbas,
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
