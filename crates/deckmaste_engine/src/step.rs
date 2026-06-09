//! The steppable core: `step()` pops one agenda item and returns one
//! `Progress`. Decisions surface on the following call; the runner loops.

use deckmaste_core::{BeginningStep, CombatStep, EndingStep, Phase, Uint, Zone};

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::{GameEvent, Occurrence};
use crate::legal::{legal_actions, legal_attackers, legal_blockers};
use crate::object::{ObjectId, ObjectSource};
use crate::player::PlayerId;
use crate::sba;
use crate::stack::StackEntry;
use crate::state::{GameOutcome, GameState};
use crate::tally::Tally;
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
    /// One or more events mutated the state (apply-time bindings filled in).
    Applied(Occurrence),
    /// A new step began.
    Advanced(Phase),
    /// A [CR#704] sweep ran; `actions` lost-player events were scheduled.
    SbasChecked { actions: Uint },
    /// [CR#603.3]: the placement barrier ran; `placed` triggers went on the
    /// stack this step (0 when none were waiting, or when an `OrderTriggers` /
    /// `ChooseTargets` decision surfaced instead).
    TriggersPlaced { placed: Uint },
    /// Cleanup's hand-size check ran ([CR#514.1]).
    HandSizeChecked { discarding: Uint },
    /// [CR#508.1]: the Declare Attackers step surfaced its decision; `legal` is
    /// how many creatures the active player may declare.
    DeclareAttackersOpened { legal: Uint },
    /// [CR#509.1]: the Declare Blockers step surfaced its decision; `legal` is
    /// how many creatures the defending player may declare as blockers.
    DeclareBlockersOpened { legal: Uint },
    /// [CR#510.1]: the Combat Damage step assigned damage. `deciding` is how
    /// many sources need a free-division decision (0 when every source was
    /// forced — the batch was dealt immediately; otherwise the first
    /// `AssignCombatDamage` decision has surfaced).
    CombatDamageOpened { deciding: Uint },
    /// A priority decision was surfaced for this player.
    PriorityOpened(PlayerId),
    /// [CR#601.2a,601.2b]: a spell moved to the stack and the announce slot opened.
    Announcing(crate::object::ObjectId),
    /// [CR#601.2c]: targets were announced for the in-flight spell (a
    /// `ChooseTargets` decision surfaces when `specs > 0`).
    TargetsAnnounced { specs: Uint },
    /// [CR#601.2f,601.2g,601.2h]: the in-flight cost was paid or a `PayMana` decision
    /// surfaced.
    CostPaid,
    /// A resolution step ran (dispatch or one effect node) for this object.
    Resolving(crate::object::ObjectId),
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
            WorkItem::Emit(occ) => Progress::Applied(self.apply_occurrence(occ)),
            WorkItem::BeginStep(s) => self.begin_step(s),
            WorkItem::CheckSbas => self.check_sbas(),
            WorkItem::PlaceTriggers => self.place_triggers(),
            WorkItem::CheckHandSize => self.check_hand_size(),
            WorkItem::DeclareAttackers => self.declare_attackers(),
            WorkItem::DeclareBlockers => self.declare_blockers(),
            WorkItem::AssignCombatDamage => self.assign_combat_damage(),
            WorkItem::OpenPriority => self.open_priority(),
            WorkItem::BeginCast(object) => {
                self.begin_cast(object);
                Progress::Announcing(object)
            }
            WorkItem::AnnounceTargets => {
                let specs = self.announce_targets();
                Progress::TargetsAnnounced { specs }
            }
            WorkItem::PayCost => {
                self.pay_cost();
                Progress::CostPaid
            }
            WorkItem::Resolve(obj) => {
                self.resolve_object(obj);
                Progress::Resolving(obj)
            }
            WorkItem::RunEffect { effect, frame } => {
                let source = frame.source;
                self.run_effect(*effect, &frame);
                Progress::Resolving(source)
            }
        };
        StepOutcome::Progress(progress)
    }

    /// Applies one event: the skeleton's whole pipeline (cant and
    /// replacement registries are empty). Returns the event as it occurred —
    /// apply-time bindings (a drawn card's identity) filled in, and a draw
    /// from an empty library occurring as `DrewFromEmpty` instead.
    // TODO: split apply() by subsystem (stack / zone-change / player) as arms grow. The
    //   action-driven zone-change collapse (draw / land / discard → ZoneWillChange) is done.
    #[expect(clippy::too_many_lines)]
    fn apply(&mut self, event: GameEvent) -> GameEvent {
        match event {
            GameEvent::TurnBegan { .. } | GameEvent::StepBegan(_) => event,
            GameEvent::Untapped(id) => {
                self.objects.obj_mut(id).tapped = false;
                event
            }
            GameEvent::WillDraw { player, source } => {
                // [CR#120.1]: the draw intent commits. A card present → bump the
                // tally and evolve into the generic Library→Hand move (remint +
                // LKI); an empty library → DrewFromEmpty, the failed-draw fact
                // the loss SBA keys on ([CR#120.3,704.5c]).
                if let Some(&top) = self.zones.libraries[player.index()].front() {
                    self.player_mut(player).this_turn.bump(Tally::CardsDrawn);
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: top,
                            from: Some(Zone::Library),
                            to: Zone::Hand,
                            enters: None,
                        },
                    ))]);
                    GameEvent::WillDraw { player, source }
                } else {
                    self.player_mut(player).drew_from_empty = true;
                    GameEvent::DrewFromEmpty(player)
                }
            }
            GameEvent::DrewFromEmpty(player) => {
                // Today only `WillDraw`'s apply-time transform produces this
                // fact; the arm exists for future direct emitters (e.g. a
                // replacement effect rewriting a draw).
                self.player_mut(player).drew_from_empty = true;
                event
            }
            GameEvent::LandPlayed { object } => {
                // [CR#305]: playing a land is an unreplaceable special action;
                // its only side effect (the land-drop tally) stays with the
                // cause (§5.5), then it evolves into the generic Hand→Battlefield
                // move — remint + LKI + AsEnters (a tapland enters tapped).
                let owner = self.owner_of(object);
                self.player_mut(owner).this_turn.bump(Tally::LandsPlayed);
                self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::ZoneWillChange {
                        object,
                        from: Some(Zone::Hand),
                        to: Zone::Battlefield,
                        enters: None,
                    },
                ))]);
                GameEvent::LandPlayed { object }
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
                // [CR#701.8]: discard evolves into the generic Hand→Graveyard
                // move (remint + LKI). (Madness — a WillDiscard intent that
                // replaces this — is a future seam.)
                self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::ZoneWillChange {
                        object,
                        from: Some(Zone::Hand),
                        to: Zone::Graveyard,
                        enters: None,
                    },
                ))]);
                GameEvent::Discarded { player, object }
            }
            GameEvent::PlayerLost { player, .. } => {
                self.player_mut(player).lost = true;
                event
            }
            GameEvent::SpellCast(object) => {
                // [CR#601.2i]: promote the staged announce onto the stack.
                let pending = self.announcing.take().expect("an announce in flight");
                debug_assert_eq!(
                    pending.object.object(),
                    object,
                    "SpellCast event matches the staged announce"
                );
                self.stack.push(StackEntry {
                    // [CR#405]: a spell's stack identity is its own object id —
                    // unchanged from Stage 2, so existing Resolve(spell) keying
                    // by `StackEntry.id` still finds it.
                    id: pending.object.object(),
                    object: pending.object,
                    controller: pending.controller,
                    targets: pending.targets,
                });
                GameEvent::SpellCast(object)
            }
            GameEvent::DamageDealt {
                source,
                target,
                amount,
            } => {
                // [CR#119]: damage to a player is life loss; to a creature it is
                // marked damage. `Int` is `i32`; `Uint` is `u32` — `try_from`
                // is required because u32 does not fit into i32 via `From`.
                match self.objects.obj(target).source {
                    ObjectSource::Player(p) => {
                        self.player_mut(p).life -=
                            deckmaste_core::Int::try_from(amount).expect("damage fits in i32");
                    }
                    ObjectSource::Card(_) => {
                        self.objects.obj_mut(target).damage += amount;
                    }
                }
                GameEvent::DamageDealt {
                    source,
                    target,
                    amount,
                }
            }
            GameEvent::ZoneWillChange {
                object,
                from,
                to,
                enters,
            } => {
                self.apply_zone_will_change(object, from, to, enters);
                GameEvent::ZoneWillChange {
                    object,
                    from,
                    to,
                    enters,
                }
            }
            // [CR#603.6]: the FACT — the move already happened at the
            // will-change apply. A no-op; triggers (a later task) match here.
            // (Same body as the `TurnBegan`/`StepBegan` no-op, but kept its own
            // arm to carry the CR rationale and the future trigger-match seam.)
            #[expect(clippy::match_same_arms)]
            GameEvent::ZoneChanged { .. } => event,
            GameEvent::LifeLost { player, amount } => {
                self.player_mut(player).life -=
                    deckmaste_core::Int::try_from(amount).expect("life loss fits in i32");
                GameEvent::LifeLost { player, amount }
            }
            // [CR#508.1a]: record the attacker; [CR#508.1f]: declaring it as an
            // attacker taps it (not a cost — attacking simply taps). Vigilance,
            // which skips the tap, is a later task.
            GameEvent::Attacking(o) => {
                self.combat.declare_attacker(o);
                self.objects.obj_mut(o).tapped = true;
                GameEvent::Attacking(o)
            }
            // [CR#509.1a]: record the block; [CR#509.1h]: the attacker becomes a
            // blocked creature (sticky). Declaring a blocker does NOT tap it. The
            // "becomes blocked" trigger seam matches on this fact.
            GameEvent::Blocked { blocker, attacker } => {
                self.combat.declare_block(blocker, attacker);
                GameEvent::Blocked { blocker, attacker }
            }
            // [CR#603.2]: applying a `TriggerFired` *notes* the trigger. It is
            // inert until the `PlaceTriggers` barrier (a later task) puts it on
            // the stack. Nothing else happens here.
            GameEvent::TriggerFired {
                source,
                ability,
                controller,
                ref bindings,
            } => {
                self.pending_triggers.push(crate::trigger::NotedTrigger {
                    source,
                    ability: ability as usize,
                    controller,
                    bindings: bindings.clone(),
                });
                event
            }
            // [CR#603.8]: the triggered ability vanishes — remove its stack
            // entry and discard the minted token. No zone move; the source
            // (already gone for a dies-trigger) is untouched.
            GameEvent::TriggerResolved(id) => {
                self.remove_stack_entry(id);
                self.objects.remove(id);
                GameEvent::TriggerResolved(id)
            }
        }
    }

    /// Applies a `ZoneWillChange` ([CR#400.7]): the move+remint that every zone
    /// change goes through. Captures the live object's LKI, removes it from its
    /// `from` zone, remints a fresh object into `to` (new `ObjectId`, same
    /// `CardId`), applies the permanent's own `AsEnters` self-replacements into
    /// the `EnterStatus` (no observable untapped window), and schedules the
    /// `ZoneChanged` fact at the agenda front.
    fn apply_zone_will_change(
        &mut self,
        object: ObjectId,
        from: Option<Zone>,
        to: Zone,
        enters: Option<crate::event::EnterStatus>,
    ) {
        // 1. Snapshot while the object is still live in `from`.
        let snapshot = crate::lki::LkiSnapshot::capture(self, object);

        // 2. (replace stage — other-object and destination-rewriting replacements are
        //    Stage-4 seams; AsEnters self-replacement applied below at mint.)

        // 3. Move + remint. Remove the old object from its `from` zone's list, then
        //    from the store; mint a fresh object into `to`.
        match from {
            Some(Zone::Stack) => self.remove_stack_entry(object),
            Some(Zone::Battlefield) => self.remove_from_battlefield(object),
            Some(Zone::Hand) => {
                let owner = self.owner_of(object);
                self.remove_from_hand(owner, object);
            }
            Some(Zone::Library) => {
                let owner = self.owner_of(object);
                self.remove_from_library(owner, object);
            }
            other => unreachable!(
                "zone-change source {other:?} is not wired (Stack/Battlefield/Hand/Library only)"
            ),
        }
        self.objects.remove(object);

        let ObjectSource::Card(card) = snapshot.source else {
            unreachable!("only card-backed objects change zones")
        };
        let owner = self.cards.get(card).owner;
        // [CR#400.7]: a permanent keeps its caster as controller; elsewhere the
        // object is controlled by its owner.
        let controller = if to == Zone::Battlefield { snapshot.controller } else { owner };
        let new = self.objects.mint(snapshot.source, controller, Some(to));
        // [CR#614.12]: how it enters — emitted status (Stage 4 replacements) plus
        // the object's own AsEnters self-replacement (enters tapped).
        let mut entering = enters.unwrap_or_default();
        if to == Zone::Battlefield {
            entering.tapped |= self.as_enters_status(snapshot.source).tapped;
            // [CR#302.6]: a permanent entering the battlefield is summoning-sick
            // until its controller's turn begins with it under continuous control.
            self.objects.obj_mut(new).summoning_sick = true;
        }
        if entering.tapped {
            self.objects.obj_mut(new).tapped = true;
        }
        match to {
            Zone::Battlefield => self.zones.battlefield.push(new),
            Zone::Graveyard => self.zones.graveyards[owner.index()].push(new),
            Zone::Hand => self.zones.hands[owner.index()].push(new),
            other => unreachable!(
                "zone-change destination {other:?} is not wired (Battlefield/Graveyard/Hand only)"
            ),
        }

        // 4. Schedule the unreplaceable fact at the agenda front.
        self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneChanged { snapshot, from, to },
        ))]);
    }

    /// Applies an occurrence: each event through the pipe, returned as the
    /// facts that occurred. A `Batch` applies with no SBA/trigger interleaving.
    /// After applying, runs `check_game_end` so a simultaneous multi-loss batch
    /// is evaluated as a whole.
    fn apply_occurrence(&mut self, occ: Occurrence) -> Occurrence {
        let occurred = match occ {
            Occurrence::Single(e) => Occurrence::Single(self.apply(e)),
            Occurrence::Batch(events) => {
                Occurrence::Batch(events.into_iter().map(|e| self.apply(e)).collect())
            }
        };
        self.check_game_end();
        if self.outcome.is_none() {
            self.scan_triggers(&occurred);
        }
        occurred
    }

    /// [CR#104.2a,104.4a]: last player standing wins; zero remaining is a draw.
    /// Run AFTER an occurrence applies, so a simultaneous multi-loss batch is a
    /// draw, not a win for whoever was checked first.
    fn check_game_end(&mut self) {
        if self.outcome.is_some() {
            return;
        }
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
    }

    /// Begins a new turn: [CR#500.1]. Returns the `TurnBegan` event to emit.
    fn begin_turn(&mut self) -> GameEvent {
        self.turn.turn_number += 1;
        if self.turn.turn_number > 1 {
            self.turn.active_player = self.next_live_after(self.turn.active_player);
        }
        for player in &mut self.players {
            player.this_turn.reset();
        }
        // [CR#302.6]: a creature the active player has controlled continuously
        // since this turn began sheds summoning sickness. Collect ids first to
        // satisfy the borrow checker (mirrors `clear_marked_damage`).
        let active = self.turn.active_player;
        let ids: Vec<_> = self.zones.battlefield.clone();
        for id in ids {
            if self.objects.obj(id).controller == active {
                self.objects.obj_mut(id).summoning_sick = false;
            }
        }
        GameEvent::TurnBegan {
            player: self.turn.active_player,
            turn: self.turn.turn_number,
        }
    }

    /// The turn-structure transition: schedules the step's whole shape.
    fn begin_step(&mut self, s: Phase) -> Progress {
        let mut items = Vec::new();
        if s == Phase::Beginning(BeginningStep::Untap) {
            let turn_began = self.begin_turn();
            items.push(WorkItem::Emit(Occurrence::single(turn_began)));
        }
        self.turn.current = s;
        items.push(WorkItem::Emit(Occurrence::single(GameEvent::StepBegan(s))));
        items.extend(self.turn_based_actions(s));
        items.extend(self.step_tail(s));
        self.schedule_front(items);
        Progress::Advanced(s)
    }

    /// [CR#500]: this step's turn-based actions, as schedulable items.
    /// Computed at scheduling time — equivalent in the skeleton (nothing can
    /// intervene before they apply); a lazy item arrives with triggers.
    // Two arms produce vec![] for different reasons; keeping them separate
    // preserves the per-step CR references.
    #[expect(clippy::match_same_arms)]
    fn turn_based_actions(&mut self, s: Phase) -> Vec<WorkItem> {
        match s {
            // [CR#502.1]: the active player's tapped permanents untap.
            Phase::Beginning(BeginningStep::Untap) => {
                let active = self.turn.active_player;
                self.zones
                    .battlefield
                    .iter()
                    .filter(|&&id| {
                        let obj = self.objects.obj(id);
                        obj.controller == active && obj.tapped
                    })
                    .map(|&id| WorkItem::Emit(Occurrence::single(GameEvent::Untapped(id))))
                    .collect()
            }
            // [CR#504.1]; [CR#103.8a] (two-player): turn 1 is the starting
            // player's, who skips their first draw.
            Phase::Beginning(BeginningStep::Draw) if self.turn.turn_number > 1 => {
                vec![WorkItem::Emit(Occurrence::single(GameEvent::WillDraw {
                    player: self.turn.active_player,
                    source: None,
                }))]
            }
            Phase::Beginning(BeginningStep::Draw) => vec![],
            // [CR#508.1]: the active player declares attackers — surface the
            // decision as this step's turn-based action.
            Phase::Combat(CombatStep::DeclareAttackers) => vec![WorkItem::DeclareAttackers],
            // [CR#509.1]: the defending player declares blockers — but only when
            // there is something to block. [CR#508.8]: with no creatures
            // attacking, the Declare Blockers step is skipped (like
            // `check_hand_size` skipping the trivial discard).
            Phase::Combat(CombatStep::DeclareBlockers) if !self.combat.attackers().is_empty() => {
                vec![WorkItem::DeclareBlockers]
            }
            Phase::Combat(CombatStep::DeclareBlockers) => vec![],
            // [CR#510.1]: assign + deal combat damage — but only when something
            // is attacking. With no attackers there is no damage to assign
            // ([CR#508.8] already skipped blockers); skip the step's work like
            // the empty Declare Blockers case.
            Phase::Combat(CombatStep::CombatDamage) if !self.combat.attackers().is_empty() => {
                vec![WorkItem::AssignCombatDamage]
            }
            Phase::Combat(CombatStep::CombatDamage) => vec![],
            // [CR#514.1]: discard to hand size — checked after StepBegan.
            // [CR#514.2]: marked damage is removed from all permanents.
            Phase::Ending(EndingStep::Cleanup) => {
                self.clear_marked_damage();
                vec![WorkItem::CheckHandSize]
            }
            _ => vec![],
        }
    }

    /// [CR#514.2]: remove all marked damage from battlefield permanents when
    /// the Cleanup step begins.
    fn clear_marked_damage(&mut self) {
        // Collect ids first to satisfy the borrow checker (can't hold a
        // shared ref to `self.zones` while mutably borrowing `self.objects`).
        let ids: Vec<_> = self.zones.battlefield.clone();
        for id in ids {
            self.objects.obj_mut(id).damage = 0;
        }
    }

    /// What follows a step's turn-based actions: the priority barrier, or
    /// the step end for the no-priority steps ([CR#502.4,514.3] — cleanup's
    /// sweep runs per [CR#514.2] but can never act in the skeleton).
    fn step_tail(&self, s: Phase) -> Vec<WorkItem> {
        match s {
            Phase::Beginning(BeginningStep::Untap) => self.end_of_step_items(),
            Phase::Ending(EndingStep::Cleanup) => {
                // [CR#514.3a]: if the sweep acts (or triggers are waiting),
                // players DO get priority and cleanup repeats. Stage 3 must
                // detect that and insert OpenPriority + another cleanup
                // before the step end; in the skeleton the sweep can never
                // act here.
                let mut items = vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers];
                items.extend(self.end_of_step_items());
                items
            }
            // [CR#603.3]: the placement barrier sits between the SBA loop and
            // `OpenPriority` — noted triggers go on the stack before anyone
            // gets priority.
            _ => vec![
                WorkItem::CheckSbas,
                WorkItem::PlaceTriggers,
                WorkItem::OpenPriority,
            ],
        }
    }

    /// [CR#500.4]: pools empty at the end of every step; then the next step
    /// begins (wrapping into the next turn's untap).
    pub(crate) fn end_of_step_items(&self) -> Vec<WorkItem> {
        let mut items: Vec<WorkItem> = self
            .players
            .iter()
            // Lost players have left the game; nothing of theirs empties.
            .filter(|p| !p.lost && !p.mana_pool.is_empty())
            .map(|p| WorkItem::Emit(Occurrence::single(GameEvent::ManaEmptied(p.id))))
            .collect();
        items.push(WorkItem::BeginStep(
            successor(self.turn.current).unwrap_or(Phase::Beginning(BeginningStep::Untap)),
        ));
        items
    }

    /// [CR#704.3]: sweep; if anything acted, emit the whole sweep as ONE
    /// simultaneous batch and re-check before the queued `OpenPriority` runs.
    fn check_sbas(&mut self) -> Progress {
        let actions = sba::sweep(self);
        let count = Uint::try_from(actions.len()).expect("action count fits in Uint");
        if count > 0 {
            self.schedule_front(vec![
                WorkItem::Emit(crate::event::Occurrence::Batch(actions)),
                WorkItem::CheckSbas,
            ]);
        }
        Progress::SbasChecked { actions: count }
    }

    /// [CR#514.1]: the active player discards to maximum hand size.
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

    /// [CR#508.1a]: surfaces the Declare Attackers decision for the active
    /// player. Always surfaces (even with an empty legal set — the player
    /// declares no attackers with an empty vec); submission front-schedules
    /// the `Attacking` batch ahead of the already-queued step tail, mirroring
    /// `check_hand_size`.
    fn declare_attackers(&mut self) -> Progress {
        let active = self.turn.active_player;
        let legal = legal_attackers(self, active);
        let count = Uint::try_from(legal.len()).expect("attacker count fits in Uint");
        self.pending = Some(PendingDecision::DeclareAttackers {
            player: active,
            legal,
        });
        Progress::DeclareAttackersOpened { legal: count }
    }

    /// [CR#509.1a]: surfaces the Declare Blockers decision for the defending
    /// player — the non-active player in this two-player engine ([CR#506.2]
    /// defines the nonactive player as the defending player). Reached only when
    /// an attacker exists (the `turn_based_actions` guard skips the trivial
    /// case, [CR#508.8]); submission front-schedules the `Blocked` batch
    /// ahead of the already-queued step tail, mirroring
    /// `declare_attackers`.
    fn declare_blockers(&mut self) -> Progress {
        let defender = self.next_live_after(self.turn.active_player);
        let legal = legal_blockers(self, defender);
        let count = Uint::try_from(legal.len()).expect("blocker count fits in Uint");
        self.pending = Some(PendingDecision::DeclareBlockers {
            player: defender,
            legal,
        });
        Progress::DeclareBlockersOpened { legal: count }
    }

    /// [CR#510.1,510.2]: the Combat Damage step's turn-based action. Computes
    /// every attacker's and blocker's recipients, auto-resolves the forced
    /// sources (0 or 1 recipient) into a buffer, and either surfaces the first
    /// free-division decision (a multi-blocked attacker, ≥ 2 recipients) or —
    /// when nothing needs deciding — deals the whole buffer as one simultaneous
    /// `Batch` ([CR#510.2]) immediately.
    ///
    /// The buffer + the queue of deciding sources live on `self.combat_damage`
    /// across decisions (the trigger/cast-in-flight pattern); each
    /// `Decision::Assignment` appends to the buffer and pops the queue, and the
    /// final answer (or this handler, when the queue starts empty) schedules
    /// the batch and clears `combat_damage`.
    fn assign_combat_damage(&mut self) -> Progress {
        let mut buffer: Vec<GameEvent> = Vec::new();
        let mut queue: Vec<crate::state::PendingAssignment> = Vec::new();

        // Each attacker is a source. Recipients: unblocked → the defending
        // player's proxy ([CR#510.1b]); blocked → its live blockers
        // ([CR#510.1c]); blocked-but-no-live-blockers → nothing (plain block,
        // no trample).
        let defender_proxy = self
            .player(self.next_live_after(self.turn.active_player))
            .object;
        for &attacker in self.combat.attackers() {
            let recipients: Vec<ObjectId> = if self.combat.is_blocked(attacker) {
                self.combat.blockers_of(attacker).to_vec()
            } else {
                vec![defender_proxy]
            };
            self.assign_source(attacker, &recipients, &mut buffer, &mut queue);
        }
        // Each live blocker is a source dealing to the one attacker it blocks
        // ([CR#510.1d]) — exactly one recipient, always forced.
        for &attacker in self.combat.attackers() {
            for &blocker in self.combat.blockers_of(attacker) {
                self.assign_source(blocker, &[attacker], &mut buffer, &mut queue);
            }
        }

        let deciding = Uint::try_from(queue.len()).expect("deciding-source count fits in Uint");
        self.combat_damage = Some(crate::state::CombatDamage { buffer, queue });
        // Surface the first deciding source, or deal the batch now if none.
        self.open_next_assignment();
        Progress::CombatDamageOpened { deciding }
    }

    /// Assigns one source's combat damage. 0 power → nothing ([CR#510.1a]); 1
    /// recipient → all power to it (forced); ≥ 2 recipients → queue a
    /// free-division decision ([CR#510.1c]). `buffer` collects forced
    /// `DamageDealt`s; `queue` collects deciding sources.
    fn assign_source(
        &self,
        source: ObjectId,
        recipients: &[ObjectId],
        buffer: &mut Vec<GameEvent>,
        queue: &mut Vec<crate::state::PendingAssignment>,
    ) {
        let power = self.power_of(source);
        if power == 0 || recipients.is_empty() {
            return; // [CR#510.1a]: 0 power (or no recipient) assigns nothing.
        }
        if recipients.len() == 1 {
            buffer.push(GameEvent::DamageDealt {
                source,
                target: recipients[0],
                amount: power,
            });
        } else {
            queue.push(crate::state::PendingAssignment {
                source,
                power,
                recipients: recipients.to_vec(),
            });
        }
    }

    /// Surfaces the next queued `AssignCombatDamage` decision, or — when the
    /// queue is empty — schedules the accumulated buffer as one simultaneous
    /// `Batch` ([CR#510.2]) and clears the transient state. Called by the step
    /// handler and after each `Decision::Assignment`.
    pub(crate) fn open_next_assignment(&mut self) {
        let cd = self
            .combat_damage
            .as_ref()
            .expect("combat-damage in flight");
        if let Some(next) = cd.queue.first() {
            let source = next.source;
            let recipients = next.recipients.clone();
            // [CR#510.1c]: the divider is the source's (attacker's) controller.
            let player = self.objects.obj(source).controller;
            self.pending = Some(PendingDecision::AssignCombatDamage {
                player,
                source,
                recipients,
            });
        } else {
            // [CR#510.2]: all assigned damage is dealt simultaneously. An empty
            // buffer (everyone 0 power / no recipients) schedules nothing.
            let buffer = self
                .combat_damage
                .take()
                .expect("combat-damage in flight")
                .buffer;
            if !buffer.is_empty() {
                self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(buffer))]);
            }
        }
    }

    /// A creature's combat-damage output: its power as a non-negative number.
    /// Reads the printed power like `sba` reads toughness; a non-`Number`
    /// (CDA / Variable) or negative power assigns 0 in the skeleton (layers and
    /// `*`-power are later stages).
    #[must_use]
    fn power_of(&self, id: ObjectId) -> Uint {
        match crate::derive::face(self.def(id)).power {
            Some(deckmaste_core::StatValue::Number(p)) if p > 0 => {
                #[expect(clippy::cast_sign_loss)]
                let p = p as Uint;
                p
            }
            _ => 0,
        }
    }

    /// [CR#117]: surfaces priority for the round's holder (opening the round
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
