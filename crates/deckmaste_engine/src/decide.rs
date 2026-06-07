use std::collections::HashSet;
use std::fmt;

use deckmaste_core::Uint;

use crate::object::ObjectId;
use crate::player::PlayerId;

/// What the engine is waiting on. `step()` returns `NeedsDecision` (without
/// mutating) until `submit_decision` answers it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingDecision {
    /// CR 117: the holder may act or pass. `legal` is advisory UI data —
    /// submission re-validates.
    Priority {
        player: PlayerId,
        legal: Vec<Action>,
    },
    /// CR 514.1: discard down to maximum hand size.
    DiscardToHandSize { player: PlayerId, count: Uint },
}

/// An answer to the pending decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Answers `Priority`.
    Act(Action),
    /// Answers `DiscardToHandSize`: which cards to discard.
    Discard(Vec<ObjectId>),
}

/// What a priority holder can do in the skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Pass,
    /// Special action, no stack (CR 116.2a, 305).
    PlayLand {
        object: ObjectId,
    },
    /// Skeleton: mana abilities only — no stack (CR 605.3a). `ability`
    /// indexes the object's derived ability list.
    ActivateAbility {
        object: ObjectId,
        ability: usize,
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
            _ => Err(DecisionError::WrongKind),
        }
    }

    /// The priority bookkeeping (CR 117.3c, 117.4): a pass rotates or ends
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
                    // CR 117.4: all-pass on an empty stack ends the step.
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
        }
    }

    /// CR 117.3c: taking an action restarts the pass count; the actor
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
