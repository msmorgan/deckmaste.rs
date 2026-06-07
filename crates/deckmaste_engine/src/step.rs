//! The steppable core: `step()` pops one agenda item and returns one
//! `Progress`. Decisions surface on the following call; the runner loops.

use deckmaste_core::{StepOrPhase, Uint, Zone};

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::GameEvent;
use crate::legal::legal_actions;
use crate::player::PlayerId;
use crate::sba;
use crate::state::{GameOutcome, GameState};
use crate::turn::{PriorityRound, successor};

/// What one `step()` call produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOutcome {
    /// One unit of work happened.
    Progress(Progress),
    /// No mutation; `submit_decision` to proceed.
    NeedsDecision(PendingDecision),
    GameOver(GameOutcome),
}

/// One unit of engine work, observed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progress {
    /// One event mutated the state — carried as it occurred (apply-time
    /// bindings filled in).
    Applied(GameEvent),
    /// A new step began.
    Advanced(StepOrPhase),
    /// A CR 704 sweep ran; `actions` lost-player events were scheduled.
    SbasChecked { actions: Uint },
    /// Cleanup's hand-size check ran (CR 514.1).
    HandSizeChecked { discarding: Uint },
    /// A priority decision was surfaced for this player.
    PriorityOpened(PlayerId),
}

impl GameState {
    /// Performs exactly one unit of work. With a decision pending, returns
    /// it idempotently; once the game is over, returns the outcome forever.
    ///
    /// # Panics
    ///
    /// Panics if the agenda is empty while the game is on — an engine
    /// invariant (every handler schedules its successor), not caller input.
    pub fn step(&mut self) -> StepOutcome {
        if let Some(outcome) = self.outcome {
            return StepOutcome::GameOver(outcome);
        }
        if let Some(pending) = &self.pending {
            return StepOutcome::NeedsDecision(pending.clone());
        }
        let item = self
            .agenda
            .pop_front()
            .expect("agenda is never empty while the game is on");
        let progress = match item {
            WorkItem::Emit(event) => Progress::Applied(self.apply(event)),
            WorkItem::BeginStep(s) => self.begin_step(s),
            WorkItem::CheckSbas => self.check_sbas(),
            WorkItem::CheckHandSize => self.check_hand_size(),
            WorkItem::OpenPriority => self.open_priority(),
        };
        StepOutcome::Progress(progress)
    }

    /// Applies one event: the skeleton's whole pipeline (cant and
    /// replacement registries are empty). Returns the event as it occurred —
    /// apply-time bindings (a drawn card's identity) filled in, and a draw
    /// from an empty library occurring as `DrewFromEmpty` instead.
    fn apply(&mut self, event: GameEvent) -> GameEvent {
        match event {
            GameEvent::TurnBegan { .. } | GameEvent::StepBegan(_) => event,
            GameEvent::Untapped(id) => {
                self.objects.obj_mut(id).tapped = false;
                event
            }
            GameEvent::CardDrawn { player, object: _ } => {
                if let Some(top) = self.zones.libraries[player.index()].pop_front() {
                    self.objects.obj_mut(top).zone = Zone::Hand;
                    self.zones.hands[player.index()].push(top);
                    GameEvent::CardDrawn {
                        player,
                        object: Some(top),
                    }
                } else {
                    // CR 120.3, 704.5c: the draw fails; the loss is the SBA's.
                    self.player_mut(player).drew_from_empty = true;
                    GameEvent::DrewFromEmpty(player)
                }
            }
            GameEvent::DrewFromEmpty(player) => {
                // Today only `CardDrawn`'s apply-time transform produces this
                // fact; the arm exists for future direct emitters (e.g. a
                // replacement effect rewriting a draw).
                self.player_mut(player).drew_from_empty = true;
                event
            }
            GameEvent::LandPlayed { object } => {
                let owner = self.owner_of(object);
                self.remove_from_hand(owner, object);
                self.objects.obj_mut(object).zone = Zone::Battlefield;
                self.zones.battlefield.push(object);
                self.player_mut(owner).lands_played_this_turn += 1;
                event
            }
            GameEvent::Tapped(id) => {
                self.objects.obj_mut(id).tapped = true;
                event
            }
            GameEvent::ManaAdded {
                player,
                mana,
                amount,
            } => {
                self.player_mut(player).mana_pool.add(mana, amount);
                event
            }
            GameEvent::ManaEmptied(player) => {
                self.player_mut(player).mana_pool.clear();
                event
            }
            GameEvent::Discarded { player, object } => {
                self.remove_from_hand(player, object);
                self.objects.obj_mut(object).zone = Zone::Graveyard;
                self.zones.graveyards[player.index()].push(object);
                event
            }
            GameEvent::PlayerLost { player, .. } => {
                self.player_mut(player).lost = true;
                // CR 104.2a / 104.4: last player standing wins; nobody is a
                // draw. Game over clears the agenda for good.
                //
                // Stage-3 trap, documented: a CR 704.3 sweep is simultaneous,
                // but these events apply one per step() — if two players lost
                // in ONE sweep, the first apply would declare Win instead of
                // Draw and destroy the second event with the agenda. This is
                // unreachable in the skeleton (no damage exists, and draws
                // are sequential), but stage 3's simultaneous-event batching
                // must make a multi-loss sweep apply atomically.
                let live: Vec<PlayerId> = self
                    .players
                    .iter()
                    .filter(|p| !p.lost)
                    .map(|p| p.id)
                    .collect();
                match live.as_slice() {
                    [winner] => self.outcome = Some(GameOutcome::Win(*winner)),
                    [] => self.outcome = Some(GameOutcome::Draw),
                    _ => {}
                }
                if self.outcome.is_some() {
                    self.agenda.clear();
                }
                event
            }
        }
    }

    /// Begins a new turn: CR 500.1. Returns the `TurnBegan` event to emit.
    fn begin_turn(&mut self) -> GameEvent {
        self.turn.turn_number += 1;
        if self.turn.turn_number > 1 {
            self.turn.active_player = self.next_live_after(self.turn.active_player);
        }
        for player in &mut self.players {
            player.lands_played_this_turn = 0;
        }
        GameEvent::TurnBegan {
            player: self.turn.active_player,
            turn: self.turn.turn_number,
        }
    }

    /// The turn-structure transition: schedules the step's whole shape.
    fn begin_step(&mut self, s: StepOrPhase) -> Progress {
        let mut items = Vec::new();
        if s == StepOrPhase::Untap {
            let turn_began = self.begin_turn();
            items.push(WorkItem::Emit(turn_began));
        }
        self.turn.current = s;
        items.push(WorkItem::Emit(GameEvent::StepBegan(s)));
        items.extend(self.turn_based_actions(s));
        items.extend(self.step_tail(s));
        self.schedule_front(items);
        Progress::Advanced(s)
    }

    /// CR 500: this step's turn-based actions, as schedulable items.
    /// Computed at scheduling time — equivalent in the skeleton (nothing can
    /// intervene before they apply); a lazy item arrives with triggers.
    // Two arms produce vec![] for different reasons; keeping them separate
    // preserves the per-step CR references.
    #[expect(clippy::match_same_arms)]
    fn turn_based_actions(&self, s: StepOrPhase) -> Vec<WorkItem> {
        match s {
            // CR 502.1: the active player's tapped permanents untap.
            StepOrPhase::Untap => {
                let active = self.turn.active_player;
                self.zones
                    .battlefield
                    .iter()
                    .filter(|&&id| {
                        let obj = self.objects.obj(id);
                        obj.controller == active && obj.tapped
                    })
                    .map(|&id| WorkItem::Emit(GameEvent::Untapped(id)))
                    .collect()
            }
            // CR 504.1; CR 103.8a (two-player): turn 1 is the starting
            // player's, who skips their first draw.
            StepOrPhase::Draw if self.turn.turn_number > 1 => {
                vec![WorkItem::Emit(GameEvent::CardDrawn {
                    player: self.turn.active_player,
                    object: None,
                })]
            }
            StepOrPhase::Draw => vec![],
            // CR 514.1: discard to hand size — checked after StepBegan.
            StepOrPhase::Cleanup => vec![WorkItem::CheckHandSize],
            _ => vec![],
        }
    }

    /// What follows a step's turn-based actions: the priority barrier, or
    /// the step end for the no-priority steps (CR 502.4, 514.3 — cleanup's
    /// sweep runs per CR 514.2 but can never act in the skeleton).
    fn step_tail(&self, s: StepOrPhase) -> Vec<WorkItem> {
        match s {
            StepOrPhase::Untap => self.end_of_step_items(),
            StepOrPhase::Cleanup => {
                // CR 514.3a: if the sweep acts (or triggers are waiting),
                // players DO get priority and cleanup repeats. Stage 3 must
                // detect that and insert OpenPriority + another cleanup
                // before the step end; in the skeleton the sweep can never
                // act here.
                let mut items = vec![WorkItem::CheckSbas];
                items.extend(self.end_of_step_items());
                items
            }
            _ => vec![WorkItem::CheckSbas, WorkItem::OpenPriority],
        }
    }

    /// CR 500.4: pools empty at the end of every step; then the next step
    /// begins (wrapping into the next turn's untap).
    pub(crate) fn end_of_step_items(&self) -> Vec<WorkItem> {
        let mut items: Vec<WorkItem> = self
            .players
            .iter()
            // Lost players have left the game; nothing of theirs empties.
            .filter(|p| !p.lost && !p.mana_pool.is_empty())
            .map(|p| WorkItem::Emit(GameEvent::ManaEmptied(p.id)))
            .collect();
        items.push(WorkItem::BeginStep(
            successor(self.turn.current).unwrap_or(StepOrPhase::Untap),
        ));
        items
    }

    /// CR 704.3: sweep; if anything acted, emit and re-check before the
    /// queued `OpenPriority` runs.
    fn check_sbas(&mut self) -> Progress {
        let actions = sba::sweep(self);
        let count = Uint::try_from(actions.len()).expect("action count fits in Uint");
        if count > 0 {
            let mut items: Vec<WorkItem> = actions.into_iter().map(WorkItem::Emit).collect();
            items.push(WorkItem::CheckSbas);
            self.schedule_front(items);
        }
        Progress::SbasChecked { actions: count }
    }

    /// CR 514.1: the active player discards to maximum hand size.
    ///
    /// Setting `pending` blocks the next `step()` before the already-queued
    /// `CheckSbas` is consumed; submission front-schedules the `Discarded`
    /// emits ahead of it, so the sweep still runs after the discards apply.
    fn check_hand_size(&mut self) -> Progress {
        let active = self.turn.active_player;
        let player = self.player(active);
        let hand =
            Uint::try_from(self.zones.hands[active.index()].len()).expect("hand size fits in Uint");
        let discarding = hand.saturating_sub(player.max_hand_size);
        if discarding > 0 {
            self.pending = Some(PendingDecision::DiscardToHandSize {
                player: active,
                count: discarding,
            });
        }
        Progress::HandSizeChecked { discarding }
    }

    /// CR 117: surfaces priority for the round's holder (opening the round
    /// at the active player if none is open — APNAP).
    fn open_priority(&mut self) -> Progress {
        let holder = if let Some(round) = &self.turn.priority {
            round.holder
        } else {
            let holder = self.turn.active_player;
            self.turn.priority = Some(PriorityRound {
                holder,
                consecutive_passes: 0,
            });
            holder
        };
        let legal = legal_actions(self, holder);
        self.pending = Some(PendingDecision::Priority {
            player: holder,
            legal,
        });
        Progress::PriorityOpened(holder)
    }
}
