//! The steppable core: `step()` pops one agenda item and returns one
//! `Progress`. Decisions surface on the following call; the runner loops.

use std::sync::Arc;

use deckmaste_core::BeginningStep;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CombatStep;
use deckmaste_core::EndingStep;
use deckmaste_core::KeywordAbility;
use deckmaste_core::PhaseStep;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use rand::RngExt;

use crate::agenda::FinalizeMark;
use crate::agenda::FinalizeWatch;
use crate::agenda::MagnitudeSource;
use crate::agenda::WorkItem;
use crate::decide::DecisionPointKind;
use crate::event::Act;
use crate::event::Attached;
use crate::event::CoinFlipped;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageDealt;
use crate::event::DieRolled;
use crate::event::GameEvent;
use crate::event::ManaEmptied;
use crate::event::Occurrence;
use crate::event::PlayerLost;
use crate::event::PlayerWon;
use crate::event::TriggerFired;
use crate::event::TurnBegan;
use crate::event::ZoneChange;
use crate::legal::legal_actions;
use crate::legal::legal_attackers;
use crate::legal::legal_blockers;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::sba;
use crate::stack::StackEntry;
use crate::state::GameOutcome;
use crate::state::GameState;
use crate::turn::PriorityRound;
use crate::turn::successor;

mod act;
mod combat;
mod player;
mod stack;
mod trigger;
mod zone;

/// One non-`Act` `GameEvent` payload's apply-time effect on `GameState`.
///
/// Dispatched from [`GameState::apply`]'s incremental router. Takes `&self`
/// (the payload by ref) so the caller still owns the original event for the
/// unchanged path.
pub(crate) trait EventApply {
    /// Apply this event's effect to `g`. Returns `None` to leave the event
    /// unchanged (the common case — `apply` returns it as-is), or `Some(e)`
    /// to report that a DIFFERENT event actually occurred (e.g. a draw over
    /// an empty library → `GameEvent::DrewFromEmpty`).
    fn apply(&self, g: &mut GameState) -> Option<GameEvent>;
}

/// What one `step()` call produced.
#[allow(
    clippy::large_enum_variant,
    reason = "step outcomes are short-lived hot-path messages"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOutcome {
    /// One unit of work happened.
    Progress(Progress),
    /// No mutation; `submit_decision` to proceed.
    NeedsDecision(DecisionPointKind),
    GameOver(GameOutcome),
}

/// One unit of engine work, observed.
#[allow(
    clippy::large_enum_variant,
    reason = "progress variants encode event payloads needed in hot path"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progress {
    /// One or more events mutated the state (apply-time bindings filled in).
    Applied(Occurrence),
    /// A new step began.
    Advanced(PhaseStep),
    /// [CR#510.4]: a step the turn structure traverses was elided without
    /// opening (no `StepBegan`, no turn-based action, no priority) — today only
    /// the `FirstCombatDamage` step, skipped when no combat creature has first
    /// or double strike. `BeginStep` of the successor was scheduled instead.
    Skipped(PhaseStep),
    /// A [CR#704] sweep ran; `actions` lost-player events were scheduled.
    SbasChecked { actions: Uint },
    /// [CR#603.3]: the placement barrier ran; `placed` triggers went on the
    /// stack this step (0 when none were waiting, or when an `OrderTriggers` /
    /// `ChooseTargets` decision surfaced instead).
    TriggersPlaced { placed: Uint },
    /// Cleanup's hand-size check ran ([CR#514.1]).
    HandSizeChecked { discarding: Uint },
    /// [CR#705.1,705.2]: a resolving `FlipCoins` ran; `count` is the total
    /// requested — an uncalled flip drew all `count` coins immediately and
    /// front-scheduled their batch, while a called flip surfaced the first of
    /// `count` `CallFlip` decisions instead (Task 4 wires the submit side).
    CoinsFlipped { count: Uint },
    /// [CR#706.1]: a resolving `RollDice` drew `count` naturals and
    /// front-scheduled their `DieRolled` batch (0 = nothing scheduled).
    DiceRolled { count: Uint },
    /// [CR#106.1b]: a resolving `AddMana` surfaced its color choice.
    ManaColorOpened,
    /// [CR#106.1b]: a resolving `AddMana` surfaced its multi-symbol-run choice
    /// (the filterland cycle).
    ManaModeOpened,
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
    /// [CR#511.3]: the End of Combat step's turn-based action ran — every
    /// creature was removed from combat (the combat-state registry cleared).
    CombatEnded,
    /// A priority decision was surfaced for this player.
    PriorityOpened(PlayerId),
    /// [CR#601.2a,601.2b] / [CR#602.2a,602.2b]: a spell moved to the stack (or
    /// an activated ability was staged) and the announce slot opened.
    Announcing(crate::object::ObjectId),
    /// [CR#601.2b]: the X-announce step ran (a `ChooseXValue` may now be pending).
    XAnnounced,
    /// [CR#601.2b,700.2]: the modal announce step ran. `options` is zero for
    /// a nonmodal object; otherwise a `ChooseModes` decision is pending.
    ModesAnnounced { options: Uint },
    /// [CR#601.2c]: targets were announced for the in-flight spell (a
    /// `ChooseTargets` decision surfaces when `specs > 0`).
    TargetsAnnounced { specs: Uint },
    /// [CR#707.10c]: a `Retarget` work item ran, re-targeting a
    /// committed stack entry. `specs` is the entry's target-spec count (0 =
    /// no decision surfaced — the entry was already gone, or its ability has
    /// no targets; > 0 = a `Retarget` decision is now pending).
    NewTargetsOpened { specs: Uint },
    /// [CR#601.2b]: the in-flight cost's hybrid/Phyrexian symbols were
    /// concretized. `surfaced` is true when a `ChooseCostOptions` decision
    /// opened (the printed cost had a choosable symbol); false when the cost
    /// was plain and the stash was set directly (no decision).
    CostOptionsChosen { surfaced: bool },
    /// [CR#601.2f,601.2g,601.2h]: the in-flight cost was paid or a `PayMana` decision
    /// surfaced.
    CostPaid,
    /// The locked total cost became an explicit payment-obligation prompt.
    PaymentOpened,
    /// A suspended cost fulfillment completed and the payment prompt resumed.
    PaymentFulfilled(crate::payment::IouId),
    /// A submitted mana ability entered stackless resolution.
    ManaActionBegan(crate::player::ManaActionId),
    /// A stackless mana ability finished and restored its parent controller.
    ManaActionFinished(crate::player::ManaActionId),
    /// A resolution step ran (dispatch or one effect node) for this object.
    Resolving(crate::object::ObjectId),
    /// [CR#701.19c]: an instruction-scoped "can't be regenerated" rider was
    /// installed for the destroy running next; `subjects` is how many objects
    /// it covers.
    RidersInstalled { subjects: Uint },
    /// [CR#608.2c,608.2d]: a resolving `ChooseAndNote` surfaced its choice —
    /// a `ChooseNoteNumber` (number kind) or a `ChooseObjects` (objects kind)
    /// decision is now pending.
    NoteChoiceOpened,
    /// [CR#401.7]: a card was repositioned within its own library (no zone
    /// change — `ObjectId` preserved).
    Repositioned(crate::object::ObjectId),
    /// [CR#401.4]: the post-pick arrange finalizer ran; `deciding` is how many
    /// piles of more than one card still need an arrange decision (0 = every
    /// pile was ≤1 card, nothing surfaced).
    PilesArranged { deciding: Uint },
    /// [CR#616.1]: a `FinalizeAct` watcher observed its keyword action's
    /// outcome. `recorded` is true when the characteristic change committed and
    /// the PAST name-fact was scheduled; false when nothing committed (replaced
    /// away / regenerated / empty) and the fact died silently.
    ActFinalized { recorded: bool },
    /// A runtime-produced numeric instruction result was written to its
    /// region-local destination register.
    MagnitudeWritten { amount: Uint },
}

impl GameState {
    /// Performs exactly one unit of work. With a decision pending, returns
    /// it idempotently; once the game is over, returns the outcome forever.
    ///
    /// # Panics
    ///
    /// Panics if the agenda is empty while the game is on — an engine
    /// invariant (every handler schedules its successor), not caller input.
    #[expect(
        clippy::too_many_lines,
        reason = "the step dispatcher keeps every WorkItem transition in one exhaustive match"
    )]
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
            WorkItem::EmitSbaBatch(events) => self.emit_sba_batch(events),
            WorkItem::BeginStep(s) => self.begin_step(s),
            WorkItem::CheckSbas => self.check_sbas(),
            WorkItem::PlaceTriggers => self.place_triggers(),
            WorkItem::CheckHandSize => self.check_hand_size(),
            WorkItem::DeclareAttackers => self.declare_attackers(),
            WorkItem::DeclareBlockers => self.declare_blockers(),
            WorkItem::AssignCombatDamage => self.assign_combat_damage(),
            WorkItem::EndOfCombat => self.end_of_combat(),
            WorkItem::OpenPriority => self.open_priority(),
            WorkItem::BeginCast(object) => {
                self.begin_cast(object);
                Progress::Announcing(object)
            }
            WorkItem::BeginCastFromResolution {
                object,
                origin,
                caster,
                alternative_cost,
                resume,
                if_not,
            } => {
                // [CR#608.2g]: cast the referenced card from the zone it's in.
                // A resolution cast owns its own speculative announcement,
                // even when it occurs inside an enclosing payment/mana action.
                self.begin_payment_proposal(caster);
                self.begin_cast_from(object, origin, caster, alternative_cost);
                self.configure_resolution_cast_decline(&resume, if_not);
                Progress::Announcing(object)
            }
            WorkItem::BeginActivate { object, ability } => {
                self.begin_activate(object, ability);
                Progress::Announcing(object)
            }
            WorkItem::AnnounceModes => {
                let options = self.announce_modes();
                Progress::ModesAnnounced { options }
            }
            WorkItem::AnnounceX => {
                self.announce_x();
                Progress::XAnnounced
            }
            WorkItem::AnnounceTargets => {
                let specs = self.announce_targets();
                Progress::TargetsAnnounced { specs }
            }
            WorkItem::Retarget { player, entry } => self.open_choose_new_targets(player, entry),
            WorkItem::ChooseCostOptions => {
                let surfaced = self.choose_cost_options();
                Progress::CostOptionsChosen { surfaced }
            }
            WorkItem::OpenPayment => {
                self.open_payment();
                Progress::PaymentOpened
            }
            WorkItem::FinishPaymentFulfillment(iou) => {
                self.finish_payment_fulfillment(iou);
                Progress::PaymentFulfilled(iou)
            }
            WorkItem::BeginManaAction(action) => {
                self.begin_mana_action(action);
                Progress::ManaActionBegan(action)
            }
            WorkItem::FinishManaAction(action) => {
                self.finish_mana_action(action);
                Progress::ManaActionFinished(action)
            }
            WorkItem::ResolveTriggeredMana {
                source,
                ability,
                triggered,
                controller,
                bindings,
            } => {
                let (action, began) = self
                    .begin_triggered_mana_action(source, ability, triggered, controller, bindings);
                if began {
                    Progress::ManaActionBegan(action)
                } else {
                    Progress::ManaActionFinished(action)
                }
            }
            WorkItem::FinishTriggeredMana {
                action,
                source,
                controller,
            } => {
                self.finish_triggered_mana_action(action, source, controller);
                Progress::ManaActionFinished(action)
            }
            WorkItem::CompleteTriggeredMana(action) => {
                self.complete_triggered_mana_action(action);
                Progress::ManaActionFinished(action)
            }
            WorkItem::FlipCoins {
                player,
                count,
                called,
            } => self.flip_coins(player, count, called),
            WorkItem::RollDice {
                player,
                count,
                sides,
            } => self.roll_dice(player, count, sides),
            WorkItem::ChooseManaColor {
                player,
                options,
                amount,
                riders,
                provenance,
            } => self.open_choose_mana_color(player, options, amount, riders, provenance),
            WorkItem::ChooseManaMode {
                player,
                options,
                amount,
                riders,
                provenance,
            } => self.open_choose_mana_mode(player, options, amount, riders, provenance),
            WorkItem::AnnounceOptionalCosts { index } => {
                let surfaced = self.announce_optional_costs(index);
                Progress::CostOptionsChosen { surfaced }
            }
            WorkItem::TollMana {
                player,
                cost,
                subject,
            } => {
                // [CR#118.12a]: surface the toll's mana demand; the `PayMana`
                // answer validates coverage and drains the pool, then the
                // agenda continues (payment is continuation-free).
                self.pending = Some(DecisionPointKind::PayMana(
                    crate::decide::pending::PayMana {
                        player,
                        cost,
                        pool: self.player(player).mana_pool.clone(),
                        subject,
                    },
                ));
                Progress::CostPaid
            }
            WorkItem::ChooseNoteNumber { player, key } => self.open_choose_note_number(player, key),
            WorkItem::ChooseNoteCardName { player, key } => {
                self.pending = Some(DecisionPointKind::ChooseNoteCardName(
                    crate::decide::pending::ChooseNoteCardName { player, key },
                ));
                Progress::NoteChoiceOpened
            }
            WorkItem::Resolve(obj) => {
                self.resolve_object(obj);
                Progress::Resolving(obj)
            }
            WorkItem::RunEffect { effect, frame } => {
                let source = frame.source(self);
                self.run_effect(Arc::unwrap_or_clone(effect), &frame);
                Progress::Resolving(source)
            }
            WorkItem::InstallRiders { no_regen } => {
                // [CR#701.19c]: arm the no-regen subjects for the destroy that
                // runs next (the immediately-following `RunEffect`). Cleared at
                // the end of the next `apply_occurrence`.
                let subjects = Uint::try_from(no_regen.len()).unwrap_or(Uint::MAX);
                self.no_regen_subjects = no_regen;
                Progress::RidersInstalled { subjects }
            }
            WorkItem::RepositionLibrary {
                object,
                end,
                offset,
            } => self.reposition_library(object, end, offset),
            WorkItem::ArrangePiles => self.arrange_piles(),
            WorkItem::ArrangeGroupLanding {
                arranger,
                arrangement,
                library_owner,
                end,
                count,
            } => self.arrange_group_landing(arranger, &arrangement, library_owner, end, count),
            WorkItem::FinalizeAct { act, watch, mark } => self.finalize_act(act, &watch, mark),
            WorkItem::WriteMagnitude {
                activation,
                dest,
                source,
                mark,
            } => self.write_magnitude(activation, dest, source, mark),
        };
        StepOutcome::Progress(progress)
    }

    /// Applies one event: the skeleton's whole pipeline (cant and
    /// replacement registries are empty). Returns the event as it occurred —
    /// apply-time bindings (a drawn card's identity) filled in, and a draw
    /// from an empty library occurring as `DrewFromEmpty` instead.
    fn apply(&mut self, event: GameEvent) -> GameEvent {
        // Non-`Act` variants dispatch through `EventApply` (by ref) and report
        // via `transformed`; `Act` is routed separately below — it takes its
        // payload BY VALUE (`apply_act` moves the owned `Box<ActContents>`), so
        // it is deliberately absent from this by-ref match, folding into the
        // wildcard (an explicit `GameEvent::Act(_) => None,` arm here would be
        // clippy::match_same_arms against that wildcard). `None` = unchanged.
        let transformed = match &event {
            GameEvent::Copied(e) => e.apply(self),
            GameEvent::AbilityActivated(e) => e.apply(self),
            GameEvent::ManaAbilityActivated(e) => e.apply(self),
            GameEvent::SpellCast(object) => stack::handle_spell_cast(self, *object),
            GameEvent::DamageDealt(e) => e.apply(self),
            GameEvent::ZoneChange(e) => e.apply(self),
            GameEvent::Attached(e) => e.apply(self),
            GameEvent::Unattached(e) => e.apply(self),
            GameEvent::LifeLost(e) => e.apply(self),
            GameEvent::LifeGained(e) => e.apply(self),
            GameEvent::Attacking(e) => e.apply(self),
            GameEvent::Blocked(e) => e.apply(self),
            GameEvent::CounterPlaced(e) => e.apply(self),
            GameEvent::CounterRemoved(e) => e.apply(self),
            GameEvent::TriggerFired(e) => e.apply(self),
            GameEvent::AbilityCountered(e) => e.apply(self),
            GameEvent::AbilityResolved(id) => stack::handle_ability_resolved(self, *id),
            GameEvent::DesignationChanged(e) => e.apply(self),
            GameEvent::GotDesignation(e) => e.apply(self),
            GameEvent::ControlChanged(e) => e.apply(self),
            GameEvent::Shuffled(player) => zone::handle_shuffled(self, *player),
            GameEvent::DamageRemoved(e) => e.apply(self),
            GameEvent::Untapped(id, _) => player::handle_untapped(self, *id),
            GameEvent::Transformed(id) => player::handle_transformed(self, *id),
            GameEvent::DrewFromEmpty(p) => player::handle_drew_from_empty(self, *p),
            GameEvent::Tapped(e) => e.apply(self),
            GameEvent::ManaAdded(e) => e.apply(self),
            GameEvent::ManaEmptied(e) => e.apply(self),
            GameEvent::TokenCreated(e) => e.apply(self),
            GameEvent::EmblemCreated(e) => e.apply(self),
            GameEvent::TokenCeased(id) => zone::handle_token_ceased(self, *id),
            GameEvent::PlayerLost(e) => e.apply(self),
            GameEvent::PlayerWon(e) => e.apply(self),
            GameEvent::Revealed(e) => {
                let viewers = e.to.clone().unwrap_or_else(|| {
                    (0..self.players.len())
                        .map(|index| {
                            crate::player::PlayerId(
                                deckmaste_core::Uint::try_from(index)
                                    .expect("player count fits in Uint"),
                            )
                        })
                        .collect()
                });
                for &viewer in &viewers {
                    for &object in &e.objects {
                        self.grant_payment_look(viewer, object);
                    }
                }
                None
            }
            // Pure facts (`TurnBegan`, `StepBegan`, `BecameTarget`,
            // `AbilityUsed`, `CoinFlipped`, `DieRolled`) have no real apply
            // body — nothing to dispatch, unchanged.
            _ => None,
        };
        // `Act` consumes its owned contents (it moves the `Box<ActContents>`),
        // so — unlike the `&self` `EventApply` handlers — it takes the payload
        // BY VALUE through its own `apply_act` dispatch (`step/act.rs`); every
        // other variant reports its transform (or `None` = unchanged) through
        // `transformed`.
        match event {
            GameEvent::Act(a) => self.apply_act(a),
            other => transformed.unwrap_or(other),
        }
    }

    /// Applies the future-form `ZoneChange` ([CR#400.7]): the move+remint that
    /// every zone change goes through. Captures the live object's LKI, removes
    /// it from its `from` zone, remints a fresh object into `to` (new
    /// `ObjectId`, same `CardId`), applies the permanent's own `AsEnters`
    /// self-replacements into the `EnterStatus` (no observable untapped
    /// window), and schedules the past-form `ZoneChange` fact (`snapshot:
    /// Some(..)`) at the agenda front. `position` places a card entering a
    /// library at that index from the top, clamped to the bottom
    /// ([CR#401.7]).
    #[expect(
        clippy::too_many_arguments,
        reason = "one parameter per ZoneChange coordinate"
    )]
    fn apply_zone_will_change(
        &mut self,
        object: ObjectId,
        from: Option<Zone>,
        to: Zone,
        enters: Option<crate::event::EnterStatus>,
        position: Option<Uint>,
        face: Option<deckmaste_core::Face>,
        cause: Option<crate::event::Cause>,
    ) {
        // 1. Snapshot while the object is still live in `from`.
        let snapshot = crate::lki::LkiSnapshot::capture(self, object);
        self.activation_departed(object, &snapshot);

        // 2. (replace stage — other-object and destination-rewriting
        //    replacements are Stage-4 seams; AsEnters self-replacement applied
        //    below at mint.)

        // [CR#400.7d,702.33e]: read the announced optional-cost record off the
        // stack entry BEFORE the entry is removed, so a kicked permanent
        // spell's record can cross onto the permanent it becomes and the
        // linked "if it was kicked" ETB read still answers after the spell has
        // left the stack.
        let carried_paid_costs = (from == Some(Zone::Stack) && to == Zone::Battlefield)
            .then(|| {
                self.stack
                    .iter()
                    .find(|entry| entry.id == object)
                    .map(|entry| entry.paid_costs.clone())
            })
            .flatten()
            .filter(|paid| !paid.is_empty());
        // A permanent that leaves the battlefield is a new object when it comes
        // back ([CR#400.7]); its inherited record dies with the old id.
        self.paid_costs_by_object.remove(&object);

        // 3. Move + remint. Remove the old object from its `from` zone's list,
        //    then from the store; mint a fresh object into `to`.
        match from {
            Some(Zone::Stack) => self.remove_stack_entry(object),
            Some(Zone::Battlefield) => {
                // [CR#506.4]: an object that leaves the battlefield is removed
                // from combat. Prune the OLD/leaving id (not a reminted one)
                // from the combat registry immediately, so a
                // creature that dies/leaves mid-combat stops
                // being tracked as an attacker/blocker at once.
                self.combat.remove_object(object);
                // [CR#506.4c]: if `object` was an attacked planeswalker, the
                // creatures attacking it are NOT removed from combat — each
                // stays an attacking creature, just no longer attacking
                // anyone (its `attack_target` entry is cleared). A no-op when
                // nothing was attacking `object`.
                self.combat.clear_attack_target(object);
                self.remove_from_battlefield(object);
            }
            Some(Zone::Hand) => {
                let owner = self.owner_of(object);
                self.remove_from_hand(owner, object);
            }
            Some(Zone::Library) => {
                let owner = self.owner_of(object);
                self.remove_from_library(owner, object);
            }
            Some(Zone::Graveyard) => {
                let owner = self.owner_of(object);
                self.remove_from_graveyard(owner, object);
            }
            Some(Zone::Exile) => self.remove_from_exile(object),
            other => unreachable!(
                "zone-change source {other:?} is not wired \
                 (Stack/Battlefield/Hand/Library/Graveyard/Exile only)"
            ),
        }
        self.objects.remove(object);

        let ObjectSource::Card(card) = snapshot.source else {
            unreachable!("only card-backed objects change zones")
        };
        let owner = self.cards.get(card).owner;
        // [CR#614.12]: how it enters — emitted status (Stage 4 replacements)
        // plus the object's own AsEnters self-replacement (enters tapped /
        // attached) folded in below. Cloned (not moved) so the original
        // `enters` is still available below to ride through unchanged on the
        // scheduled past-form `ZoneChange`.
        let mut entering = enters.clone().unwrap_or_default();
        // [CR#110.2,108.4,110.2a]: a permanent keeps its caster as controller
        // by default; an `EnterRider::UnderControlOf`/`UnderOwnersControl`
        // rider (`entering.controller`) overrides that default. Elsewhere the
        // object is controlled by its owner.
        let controller = if to == Zone::Battlefield {
            entering.controller.unwrap_or(snapshot.controller)
        } else {
            owner
        };
        let new = self.objects.mint(snapshot.source, controller, Some(to));
        // [CR#400.7j,400.7e]: the same effect (and a zone-change trigger) can
        // find the object it became — but only in a PUBLIC zone. Hidden
        // destinations record nothing: the old id simply goes stale.
        if !to.is_hidden() {
            self.moved_chain.push((object, new));
        }
        if let Some(paid) = carried_paid_costs {
            self.paid_costs_by_object.insert(new, paid);
        }
        if to == Zone::Battlefield {
            let as_enters = self.as_enters_status(snapshot.source, new);
            entering.tapped |= as_enters.tapped;
            entering.attach_to = entering.attach_to.or(as_enters.attach_to);
            entering.counters.extend(as_enters.counters);
            // [CR#302.6]: a permanent entering the battlefield is summoning-sick
            // until its controller's turn begins with it under continuous
            // control.
            self.objects.obj_mut(new).summoning_sick = true;
        }
        if entering.tapped {
            self.objects.obj_mut(new).tapped = true;
        }
        // [CR#122.6a,614.1c]: place enters-with counters atomically at mint,
        // before the past-form `ZoneChange` fact — the entering P/T already
        // reflects them, and no counterless window is observable.
        for (kind, n) in &entering.counters {
            *self.objects.obj_mut(new).counters.entry(*kind).or_insert(0) += n;
        }
        // [CR#303.4]: enters attached atomically — set the link on the freshly
        // minted object before the past-form `ZoneChange` fact, so no
        // unattached window is observable. The `Attached` fact is
        // scheduled after the entry fact.
        let attached_host = entering.attach_to.filter(|&host| {
            to == Zone::Battlefield && self.objects.get(host).is_some() && host != new
        });
        if let Some(host) = attached_host {
            self.objects.obj_mut(new).attached_to = Some(host);
        }
        // [CR#508.4,508.4a]: an `EnterRider::Attacking` target that's
        // stopped existing since it was resolved (or a non-battlefield
        // destination — riders are ETB-only) is never considered an
        // attacking creature. `declare_attacker` mutates `CombatState`
        // directly — no `GameEvent::Attacking` fires (see `EnterStatus::
        // attacking`'s doc comment for why).
        let attacking_target = entering
            .attacking
            .filter(|&target| to == Zone::Battlefield && self.objects.get(target).is_some());
        if let Some(target) = attacking_target {
            self.combat.declare_attacker(new, target);
        }
        match to {
            Zone::Battlefield => self.zones.battlefield.push(new),
            Zone::Graveyard => self.zones.graveyards[owner.index()].push(new),
            Zone::Hand => self.zones.hands[owner.index()].push(new),
            Zone::Exile => self.zones.exile.push(new),
            Zone::Library => {
                // [CR#401.7]: an index past the bottom places the card on the
                // bottom; `None` (a non-positional move) means the top.
                let lib = &mut self.zones.libraries[owner.index()];
                let i = (position.unwrap_or(0) as usize).min(lib.len());
                lib.insert(i, new);
            }
            other => unreachable!(
                "zone-change destination {other:?} is not wired \
                 (Battlefield/Graveyard/Hand/Exile/Library only)"
            ),
        }

        // 4. Schedule the unreplaceable fact(s) at the agenda front — the face,
        // cause, enters and position coordinates ride through unchanged from
        // the intent; only `snapshot` flips from `None` to `Some`, turning the
        // future form into the past form of the SAME `ZoneChange` variant.
        // Inside a batch apply the fact joins the shared evolution collector
        // instead, so a simultaneous move-batch commits as ONE past-form
        // `ZoneChange` occurrence ([CR#603.2c]). When the permanent
        // entered attached ([CR#303.4]), the `Attached` fact follows the entry
        // fact (it entered, then became attached) so "becomes attached /
        // equipped" can match it (breadth is a seam, §9); it stays its own
        // occurrence — the flushed evolution batch is front-scheduled after
        // the member loop, landing AHEAD of it.
        self.schedule_evolution(GameEvent::ZoneChange(ZoneChange {
            object,
            snapshot: Some(Box::new(snapshot)),
            from,
            to,
            enters,
            position,
            face,
            cause,
        }));
        if let Some(host) = attached_host {
            self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                GameEvent::Attached(Attached {
                    attachment: new,
                    host,
                }),
            ))]);
        }
    }

    /// Applies a `TokenCreated` ([CR#701.7a]): synthesizes the token's
    /// definition into the card table ([CR#111.2]: `player` is its owner and
    /// it enters under their control, unless `enters.controller` overrides
    /// it — [CR#110.2a]), mints the object straight onto the battlefield
    /// (summoning-sick, [CR#302.6]; its own `AsEnters` self-replacements
    /// folded in with any `Action::Create` rider list, [CR#614.12,508.4]),
    /// and schedules the past-form `ZoneChange { from: None, to:
    /// Battlefield, .. }` fact so enter-triggers fire ([CR#603.6]). There is
    /// no future/replaceable form here — the token existed nowhere to move
    /// *from*; its snapshot is captured from the freshly minted object.
    fn apply_token_created(
        &mut self,
        player: PlayerId,
        token: &deckmaste_core::Token,
        enters: Option<crate::event::EnterStatus>,
    ) {
        let mut entering = enters.clone().unwrap_or_default();
        let card = self.cards.push_token(token, player);
        let source = ObjectSource::Card(card);
        let controller = entering.controller.unwrap_or(player);
        let new = self
            .objects
            .mint(source, controller, Some(Zone::Battlefield));
        self.objects.obj_mut(new).summoning_sick = true;
        let as_enters = self.as_enters_status(source, new);
        entering.tapped |= as_enters.tapped;
        entering.counters.extend(as_enters.counters);
        if entering.tapped {
            self.objects.obj_mut(new).tapped = true;
        }
        // [CR#122.6a,614.1c]: place enters-with counters atomically at mint —
        // same no-counterless-window guarantee as `apply_zone_will_change`.
        for (kind, n) in &entering.counters {
            *self.objects.obj_mut(new).counters.entry(*kind).or_insert(0) += n;
        }
        // [CR#508.4,508.4a]: see `apply_zone_will_change`'s twin block — no
        // `GameEvent::Attacking` fires here either.
        if let Some(target) = entering
            .attacking
            .filter(|&t| self.objects.get(t).is_some())
        {
            self.combat.declare_attacker(new, target);
        }
        self.zones.battlefield.push(new);
        let snapshot = crate::lki::LkiSnapshot::capture(self, new);
        // The entry fact joins the batch collector when N tokens are minted
        // as one simultaneous instruction ([CR#701.7a] — "one instruction,
        // simultaneous"): their enter-triggers see ONE occurrence.
        self.schedule_evolution(GameEvent::ZoneChange(ZoneChange {
            object: new,
            snapshot: Some(Box::new(snapshot)),
            from: None,
            to: Zone::Battlefield,
            enters,
            position: None,
            face: None,
            cause: None,
        }));
    }

    /// Applies an `EmblemCreated` ([CR#114.1]): synthesizes an abilities-only
    /// def ([CR#114.3]) into the card table (owner = `player`, [CR#114.2]) and
    /// mints the object straight into the command zone (controller = `player`).
    /// Unlike a token, an emblem does NOT enter the battlefield — it's never a
    /// permanent ([CR#114.5]) and never summoning-sick — so there is no
    /// `ZoneChange` fact and no enter-trigger scan. Its abilities function
    /// from the command zone ([CR#114.4]); no SBA ever removes it (it isn't
    /// a permanent and a command-zone object can't be destroyed —
    /// [CR#114.5,408.1]).
    pub(crate) fn apply_emblem_created(
        &mut self,
        player: PlayerId,
        abilities: Vec<deckmaste_core::Ability>,
    ) {
        let card = self.cards.push_emblem(abilities, player);
        let source = ObjectSource::Card(card);
        let new = self.objects.mint(source, player, Some(Zone::Command));
        self.zones.command.push(new);
    }

    /// Applies a `TokenCeased` ([CR#704.5d,111.7]): removes the token object
    /// from its zone and the store outright. No remint, no `ZoneChange` —
    /// ceasing to exist is not a zone change. The card-table entry stays as
    /// inert history (`Cards` never shrinks).
    fn apply_token_ceased(&mut self, id: ObjectId) {
        let owner = self.owner_of(id);
        match self.objects.obj(id).zone {
            Some(Zone::Graveyard) => self.remove_from_graveyard(owner, id),
            Some(Zone::Hand) => self.remove_from_hand(owner, id),
            Some(Zone::Library) => self.remove_from_library(owner, id),
            Some(Zone::Exile) => self.remove_from_exile(id),
            other => unreachable!(
                "a token ceases only from a non-battlefield, non-stack zone, got {other:?}"
            ),
        }
        self.objects.remove(id);
    }

    /// Routes an intent's evolution product (`Act(Destroy)` →
    /// future-form `ZoneChange` → past-form `ZoneChange`, a draw's move, a
    /// token's entry fact): inside a `Batch` apply it joins the shared
    /// evolution collector — the whole batch's products commit later as ONE
    /// follow-on occurrence ([CR#603.2c] — a simultaneous set stays
    /// one occurrence through every stage) — and outside one it
    /// front-schedules the familiar `Single`.
    fn schedule_evolution(&mut self, event: GameEvent) {
        match &mut self.evolving_batch {
            Some(collector) => collector.push(event),
            None => {
                self.schedule_front(vec![WorkItem::Emit(Occurrence::single(event))]);
            }
        }
    }

    /// Applies an occurrence: each event through the pipe, returned as the
    /// facts that occurred. A `Batch` applies with no SBA/trigger interleaving.
    /// After applying, runs `check_game_end` so a simultaneous multi-loss batch
    /// is evaluated as a whole. A damage/counter event whose current-only
    /// recipient departed after the event was queued is discarded before cant,
    /// replacement, mutation, history, or trigger processing ([CR#608.2b]); a
    /// stale damage source remains valid LKI ([CR#608.2h,113.7a]).
    fn apply_occurrence(&mut self, mut occ: Occurrence) -> Occurrence {
        // [CR#104.3f]: a player who would simultaneously win and lose, loses
        // — drop any `PlayerWon{p}` that the same batch also carries a
        // `PlayerLost{p}` for, before the batch is applied. Guarded so the
        // common batch (no win at all) skips the scan entirely.
        if let Occurrence::Batch(ref mut facts) = occ
            && facts
                .iter()
                .any(|e| matches!(e, GameEvent::PlayerWon(PlayerWon { .. })))
        {
            let losers: std::collections::HashSet<PlayerId> = facts
                .iter()
                .filter_map(|e| match e {
                    GameEvent::PlayerLost(PlayerLost { player, .. }) => Some(*player),
                    _ => None,
                })
                .collect();
            facts.retain(
                |e| !matches!(e, GameEvent::PlayerWon(PlayerWon { player }) if losers.contains(player)),
            );
        }
        // Preserve whether the input was a Single or Batch — callers and tests
        // observe the shape ([CR#616.1]: a batch is a simultaneous set).
        let occurred = match occ {
            Occurrence::Single(e) => {
                // [CR#614.17]: can't-happen pass — suppressed before replacements.
                if !self.event_current_recipient_is_live(&e)
                    || crate::replace_registry::cant_event(self, &e)
                {
                    Occurrence::Batch(vec![]) // suppressed, nothing occurred
                } else {
                    // [CR#616.1]: replacement-effect loop.
                    match crate::replace_registry::replace_event(self, e) {
                        crate::replace_registry::ReplaceOutcome::Pass(e2) => {
                            if self.event_current_recipient_is_live(&e2) {
                                Occurrence::Single(self.apply(e2))
                            } else {
                                Occurrence::Batch(vec![])
                            }
                        }
                        crate::replace_registry::ReplaceOutcome::Nothing => {
                            Occurrence::Batch(vec![]) // replaced to nothing
                        }
                        crate::replace_registry::ReplaceOutcome::Suspend => {
                            // [CR#616.1]: ChooseReplacement surfaced for a
                            // Single event. The decision and ReplaceState are
                            // already set; remaining is empty (no batch tail).
                            // Produce an empty Batch — nothing happened yet;
                            // the event will be applied (or not) after the
                            // decision is answered via resume_replacements.
                            Occurrence::Batch(vec![])
                        }
                    }
                }
            }
            Occurrence::Batch(events) => {
                // Predicate out suppressed events first (cant pass borrows self
                // immutably), then run the replacement loop + apply on each —
                // each member is replaceable ON ITS OWN ([CR#616.1]);
                // replacing one member never unapplies the others. Member
                // evolutions collect into `evolving_batch` and flush as ONE
                // follow-on occurrence ([CR#603.2c]).
                let live: Vec<GameEvent> = events
                    .into_iter()
                    .filter(|e| self.event_current_recipient_is_live(e))
                    .filter(|e| !crate::replace_registry::cant_event(self, e))
                    .collect();
                debug_assert!(
                    self.evolving_batch.is_none(),
                    "batch applies never nest — apply schedules, it does not recurse"
                );
                self.evolving_batch = Some(Vec::new());
                let mut facts = Vec::new();
                let mut iter = live.into_iter();
                while let Some(e) = iter.next() {
                    match crate::replace_registry::replace_event(self, e) {
                        crate::replace_registry::ReplaceOutcome::Pass(e2) => {
                            if self.event_current_recipient_is_live(&e2) {
                                facts.push(self.apply(e2));
                            }
                        }
                        crate::replace_registry::ReplaceOutcome::Nothing => {}
                        crate::replace_registry::ReplaceOutcome::Suspend => {
                            // [CR#616.1]: a ChooseReplacement decision was
                            // surfaced. Store the not-yet-processed tail of
                            // the batch into the suspended replace_state so
                            // `resume_replacements` can finish the batch after
                            // the decision is answered. (The resumed tail's
                            // evolutions run outside this collector — an
                            // interactively split batch commits its halves as
                            // separate occurrences.)
                            let remaining: Vec<GameEvent> = iter
                                .filter(|ev| self.event_current_recipient_is_live(ev))
                                .filter(|ev| !crate::replace_registry::cant_event(self, ev))
                                .collect();
                            if let Some(rs) = self.replace_state.as_mut() {
                                rs.remaining = remaining;
                            }
                            // Record and scan the partial facts that DID happen
                            // before the suspension, then return — the decision
                            // will drive the rest of the batch via resume.
                            self.flush_evolving_batch();
                            let partial = Occurrence::Batch(facts);
                            self.record_history(&partial);
                            self.check_game_end();
                            if self.outcome.is_none() {
                                self.scan_triggers(&partial);
                                self.sweep_event_durations(&partial);
                                self.sweep_condition_durations();
                            }
                            self.no_regen_subjects.clear();
                            return partial;
                        }
                    }
                }
                self.flush_evolving_batch();
                Occurrence::Batch(facts)
            }
        };
        self.record_history(&occurred);
        self.check_game_end();
        if self.outcome.is_none() {
            self.scan_triggers(&occurred);
            // [CR#610.3,611.2b]: beside the trigger scan, end any floating
            // one-shot whose ending EVENT has now happened (`UntilEvent`) or
            // whose CONDITION has just lapsed (`ForAsLongAs`). Post-apply, so
            // an effect lasts through the very event/state-change
            // that ends it.
            self.sweep_event_durations(&occurred);
            self.sweep_condition_durations();
        }
        // The instruction-scoped "can't be regenerated" rider (if any) is in
        // force ONLY for the occurrence just applied ([CR#701.19c]) — clear it
        // so it never bleeds onto a later (e.g. SBA) destruction. Cheap: the
        // vec is empty except across a `ForThisEvent` destroy.
        self.no_regen_subjects.clear();
        occurred
    }

    /// Whether the event's current-only recipient still exists immediately
    /// before application. Builders perform the same check at resolution time;
    /// this closes the queue window in which an earlier event can remint or
    /// remove the recipient before this event reaches the apply funnel.
    fn event_current_recipient_is_live(&self, event: &GameEvent) -> bool {
        let recipient = match event {
            GameEvent::DamageDealt(DamageDealt { target, .. }) => Some(*target),
            GameEvent::CounterPlaced(CounterPlaced { object, .. })
            | GameEvent::CounterRemoved(CounterRemoved { object, .. }) => Some(*object),
            _ => None,
        };
        recipient.is_none_or(|object| self.objects.get(object).is_some())
    }

    /// Front-schedules the batch-evolution collector's contents as ONE
    /// occurrence ([CR#603.2c]) and deactivates it. A single-product batch
    /// stays a `Batch` — it was one simultaneous instruction, and its
    /// follow-ons must keep collecting (a destroy-all that reached one
    /// creature still evolves batch-wise).
    fn flush_evolving_batch(&mut self) {
        let collected = self
            .evolving_batch
            .take()
            .expect("flush pairs with an active collector");
        if !collected.is_empty() {
            self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(collected))]);
        }
    }

    /// [CR#616.1]: observe a keyword action's outcome and record its PAST
    /// name-fact iff the characteristic change committed since `mark`. The
    /// move verbs watch their patients zone-changing to ANY destination — a
    /// redirected card is still discarded/milled ([CR#701.9c,701.17c],
    /// Megrim-under-madness/Leyline); draw watches a `Draw`-caused move for the
    /// drawing player (an empty library commits none, so no draw fact); a
    /// reorder/fight `FinalizeAct` is scheduled only by a PASSED apply (after
    /// the arrange/damage body), so it records unconditionally ([CR#701.22d]).
    /// A miss records nothing — a replaced-away mill, a regenerated destroy, an
    /// empty draw. The committed fact is front-scheduled as an `Emit`: it
    /// records + fires its "whenever you …" triggers, and `replaceable()`
    /// refuses it so it opens no fresh window. [CR#616.1g,121.2a]: a
    /// CONTAINED window (one of an aggregate `Batch`'s `n` per-entity
    /// futures, `act.contained`) still runs this whole computation — its
    /// characteristic result still needs to happen and `committed` still
    /// feeds the RETURNED `Progress` plus the private contained-success
    /// ledger — but its fact/trigger emission is suppressed; the aggregate's
    /// own (non-contained) `FinalizeAct` is the one ACT-level commit for the
    /// batch.
    fn finalize_act(
        &mut self,
        act: GameEvent,
        watch: &FinalizeWatch,
        mark: FinalizeMark,
    ) -> Progress {
        let committed = match watch {
            FinalizeWatch::BodyRan => true,
            FinalizeWatch::Patients(ids) => self.resolution_events[mark.events..].iter().any(|e| {
                matches!(
                    e,
                    GameEvent::ZoneChange(ZoneChange { snapshot: Some(_), object, .. }) if ids.contains(object)
                )
            }),
            FinalizeWatch::Performer(player) => self.resolution_events[mark.events..].iter().any(|e| {
                let GameEvent::ZoneChange(ZoneChange {
                    snapshot: Some(snap),
                    cause: Some(c),
                    ..
                }) = e
                else {
                    return false;
                };
                c.verb.as_str() == "Draw"
                    && match snap.source {
                        crate::object::ObjectSource::Card(card) => {
                            self.cards.get(card).owner == *player
                        }
                        crate::object::ObjectSource::Player(_) => false,
                    }
            }),
            // [CR#616.1g,121.2a]: the aggregate finalizes iff ≥1 of the `n`
            // contained per-entity futures its own PASSED apply scheduled
            // itself committed since `mark`. Mill's direct batch lane reports
            // through its GROUND-TRUTH cause-tagged `ZoneChange`; every
            // repeated `Act` lane reports through the contained future's OWN
            // verb-appropriate finalizer. Neither exposes a contained `Act`
            // fact: `finalize_act` suppresses that public emission below so a
            // "whenever you Verb" trigger fires once per batch, not once per
            // entity (see `GameEvent::Act::contained`).
            FinalizeWatch::AnyContained(verb) => {
                let direct_event_committed = self.resolution_events[mark.events..].iter().any(|e| {
                    matches!(
                        e,
                        GameEvent::ZoneChange(ZoneChange { snapshot: Some(_), cause: Some(c), .. })
                            if c.verb.as_str() == verb.as_str()
                    )
                });
                let contained_act_committed = self
                    .resolution_contained_act_commits
                    .get(verb)
                    .is_some_and(|&serial| serial > mark.contained_act_serial);
                direct_event_committed || contained_act_committed
            }
        };
        // [CR#616.1g,121.2a]: a CONTAINED per-entity future (one of an
        // aggregate `Batch` window's `n` contents) commits its own
        // `ZoneChange`/`Drawn` fact normally (RESULT-side "whenever a card
        // is milled/drawn" triggers still see it, once per card) but never
        // its OWN `Act` name-fact — the aggregate's `FinalizeAct` is the
        // ONE ACT-level ("whenever you Verb") commit for the whole batch.
        let contained = matches!(
            act,
            GameEvent::Act(Act {
                contained: true,
                ..
            })
        );
        // A contained future still runs its OWN verb-appropriate watcher. Its
        // success is private resolution plumbing: record a marker for the
        // enclosing aggregate, never a public `Act` fact that would trigger
        // once per element. This preserves BodyRan successes (Fight/reorder)
        // even when the body produces no ground-truth zone-change event.
        if committed
            && contained
            && let GameEvent::Act(Act { verb, .. }) = &act
        {
            self.resolution_contained_act_serial += 1;
            let serial = self.resolution_contained_act_serial;
            self.resolution_contained_act_commits.insert(*verb, serial);
        }
        if committed && !contained {
            let done: Vec<GameEvent> = match act {
                GameEvent::Act(Act {
                    verb,
                    who,
                    on,
                    from,
                    to,
                    cause,
                    ..
                }) => {
                    // ONE committed past fact PER SUBJECT ([CR#701.14a] — a
                    // symmetric verb's "fights" trigger fires once per
                    // combatant, and `event_roles` binds "it" per fact). A
                    // subjectless or single-subject verb splits into itself —
                    // the identity for every verb but Fight.
                    let per_subject: Vec<Vec<ObjectId>> = if on.len() <= 1 {
                        vec![on]
                    } else {
                        on.into_iter().map(|s| vec![s]).collect()
                    };
                    per_subject
                        .into_iter()
                        .map(|on| {
                            GameEvent::Act(Act {
                                verb,
                                who,
                                on,
                                from,
                                to,
                                cause: cause.clone(),
                                committed: true,
                                contents: None,
                                batch: None,
                                inherited: std::collections::HashSet::new(),
                                contained: false,
                            })
                        })
                        .collect()
                }
                other => vec![other],
            };
            // >1 subject → ONE Batch occurrence: the members share a history
            // batch id ([CR#603.2c] — they were one fight; the per-FIGHT
            // dedup hook for any future fight-counting consumer).
            let occ = match done.len() {
                1 => Occurrence::single(done.into_iter().next().expect("one fact")),
                _ => Occurrence::Batch(done),
            };
            self.schedule_front(vec![WorkItem::Emit(occ)]);
        }
        Progress::ActFinalized {
            recorded: committed,
        }
    }

    fn write_magnitude(
        &self,
        activation: crate::activation::ActivationId,
        dest: deckmaste_core::DefId,
        source: MagnitudeSource,
        mark: usize,
    ) -> Progress {
        let facts = &self.resolution_events[mark..];
        let amount = match source {
            MagnitudeSource::CoinFlips { called } => facts
                .iter()
                .filter_map(|event| match event {
                    GameEvent::CoinFlipped(CoinFlipped { heads, won, .. }) => {
                        Some(Uint::from(if called { *won == Some(true) } else { *heads }))
                    }
                    _ => None,
                })
                .sum(),
            MagnitudeSource::DiceRolls => facts
                .iter()
                .filter_map(|event| match event {
                    GameEvent::DieRolled(DieRolled { result, .. }) => Some(*result),
                    _ => None,
                })
                .sum(),
            MagnitudeSource::ZoneChanges(verb) => Uint::try_from(
                facts
                    .iter()
                    .filter(|event| {
                        matches!(
                            event,
                            GameEvent::ZoneChange(ZoneChange {
                                snapshot: Some(_),
                                cause: Some(cause),
                                ..
                            }) if cause.verb.as_str() == verb.as_str()
                        )
                    })
                    .count(),
            )
            .expect("one instruction's moved-card count fits Uint"),
        };
        self.activation_write_number(activation, dest, amount);
        Progress::MagnitudeWritten { amount }
    }

    /// Appends the substantive facts of `occurred` to the history log, tagged
    /// with the current turn ([CR#608.2i]). Skips the meta/intent facts:
    /// a `TriggerFired` is bookkeeping, the future-form `ZoneChange`
    /// (`snapshot: None`) is the replaceable intent above its committed
    /// past-form fact, and `TurnBegan` is read off `TurnState`. `StepBegan`
    /// IS recorded (with the active player on its view) so history reads see
    /// step onsets ([CR#603.2b] — the StepBegins-in-history lift; also the
    /// future sub-turn window markers). A `Batch`'s members share one fresh
    /// batch id ([CR#603.2c] — they were ONE occurrence); a `Single` records
    /// `None`. Every recorded past-form `ZoneChange` also feeds the open
    /// `Noting` collections ([CR#607.2a] — fact-backed product groups: the
    /// group is what the clause ACTUALLY moved, never its gathered input
    /// set).
    fn record_history(&mut self, occurred: &Occurrence) {
        let turn = self.turn.turn_number;
        let (events, batch): (&[GameEvent], Option<Uint>) = match occurred {
            Occurrence::Single(e) => (std::slice::from_ref(e), None),
            Occurrence::Batch(es) => {
                let id = self.next_batch;
                self.next_batch += 1;
                (es, Some(id))
            }
        };
        for event in events {
            match event {
                GameEvent::TriggerFired(TriggerFired { .. })
                | GameEvent::AbilityResolved(_)
                | GameEvent::TurnBegan(TurnBegan { .. })
                | GameEvent::ZoneChange(ZoneChange { snapshot: None, .. })
                // The FUTURE keyword-action window is not a recorded fact — its
                // committed PAST form (recorded by `FinalizeAct`) is what
                // history/triggers read ([CR#603.6]); recording the future too
                // would double-count every keyword-action query.
                | GameEvent::Act(Act { committed: false, .. }) => {}
                _ => {
                    // [CR#603.12]: the resolution-scoped window a reflexive
                    // triggered ability looks back over. Reset per resolution
                    // (`resolve_object`); the same non-meta facts history keeps.
                    self.resolution_events.push(event.clone());
                    self.record_history_fact(turn, batch, event.clone());
                }
            }
        }
    }

    /// Records one fact to the history log with its per-fact LKI view
    /// ([CR#603.10a]): live card participants are snapshotted NOW, so later
    /// history matching reads them as they were — never the live store
    /// through a stale id.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "history recording accepts the event ownership transferred by its caller"
    )]
    pub(crate) fn record_history_fact(
        &mut self,
        turn: Uint,
        batch: Option<Uint>,
        event: GameEvent,
    ) {
        let (reversal_barriers, observation_barriers) =
            crate::payment::contextual_barriers_for(self, &event);
        let observed_rng = observation_barriers
            .contains(&crate::payment::ObservationBarrier::RandomOutcome)
            .then(|| crate::payment::RecordedRngState {
                stream: self.rng.get_stream(),
                word_pos: self.rng.get_word_pos(),
            });
        // A pre-evolution intent is shadowed by its downstream fact
        // ([CR#603.6]) — recording a view for BOTH would double-count every
        // `Happened`/`EventCount` zone-move read.
        let view = if crate::eval::shadowed_by_fact(&event) {
            None
        } else {
            crate::eval::FactView::of(self, &event).map(|v| v.into_lki(self))
        };
        // [CR#118.10]: mirror the fact's cause-carried payment id (if any)
        // onto the entry — see `HistEntry::payment`.
        let payment = view
            .as_ref()
            .and_then(|v| v.cause.as_ref())
            .and_then(|c| c.payment);
        self.history
            .record(turn, batch, payment, event.clone(), view);
        self.note_payment_created_object(&event);
        if let Some(observed_rng) = observed_rng
            && let Some(controller) = self.payment.as_mut()
        {
            for frame in &mut controller.frames {
                frame.observed_rng = Some(observed_rng);
            }
        }
        if let Some(action) = self
            .payment
            .as_mut()
            .and_then(|controller| controller.mana_actions.last_mut())
        {
            action.facts.push(event.clone());
            action
                .reversal_barriers
                .extend(reversal_barriers.iter().copied());
            action
                .observation_barriers
                .extend(observation_barriers.iter().copied());
        }
        if let Some(recording) = self
            .payment
            .as_mut()
            .and_then(|controller| controller.frames.last_mut())
            .and_then(|frame| frame.recording.as_mut())
        {
            recording.facts.push(event.clone());
            recording.reversal_barriers.extend(reversal_barriers);
            recording.observation_barriers.extend(observation_barriers);
        }
    }

    /// [CR#104.2a,104.4a]: last player standing wins; zero remaining is a draw.
    /// Run AFTER an occurrence applies, so a simultaneous multi-loss batch is a
    /// draw, not a win for whoever was checked first.
    ///
    /// Seam (no trip point exists yet): the mandatory-loop draw
    /// ([CR#104.4b]) needs a loop MONITOR, and the monitor needs a
    /// game-state equality predicate — UD-11, still OPEN (no rule defines
    /// when two states are "the same"; see docs/engine-adrs.md).
    ///
    /// [CR#104.2a]: the last-player-standing win OVERRIDES all effects that
    /// would preclude winning — it pierces `CantWin`. This derived win must
    /// therefore NEVER consult `gate_suppresses`; only the effect-driven
    /// `WinGame` verb ([CR#104.2b]) is gate-checked.
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
        // [CR#611.2a]: "until your next turn" effects/shields whose controller
        // is the now-active player end as this turn begins.
        self.expire_your_next_turn(active);
        GameEvent::TurnBegan(TurnBegan {
            player: self.turn.active_player,
            turn: self.turn.turn_number,
        })
    }

    /// The turn-structure transition: schedules the step's whole shape.
    fn begin_step(&mut self, s: PhaseStep) -> Progress {
        // [CR#510.4]: the FirstCombatDamage step exists only when at least one
        // attacking or blocking creature has first/double strike. When none
        // does, elide it entirely — no StepBegan, no turn-based action, no
        // priority window — and schedule the regular CombatDamage step
        // directly. (`turn.current` is NOT advanced; the step never
        // owns a turn.)
        if s == PhaseStep::Combat(CombatStep::FirstCombatDamage)
            && !crate::combat::any_first_or_double_striker(self)
        {
            self.schedule_front(vec![WorkItem::BeginStep(PhaseStep::Combat(
                CombatStep::CombatDamage,
            ))]);
            return Progress::Skipped(s);
        }
        let mut items = Vec::new();
        if s == PhaseStep::Beginning(BeginningStep::Untap) {
            let turn_began = self.begin_turn();
            items.push(WorkItem::Emit(Occurrence::single(turn_began)));
        }
        self.turn.current = s;
        // [CR#611.2b]: a step transition can end a `ForAsLongAs` effect (its
        // condition may read the phase/step, or a between-steps state change
        // may have lapsed it) — re-check with the new step current.
        // Safe here: this is outside `layer::gather`, so
        // `condition_holds` may derive the board.
        self.sweep_condition_durations();
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
    #[expect(
        clippy::match_same_arms,
        reason = "separate arms keep per-step CR refs"
    )]
    fn turn_based_actions(&mut self, s: PhaseStep) -> Vec<WorkItem> {
        match s {
            // [CR#502.3]: the active player determines which of their
            // permanents untap, then untaps them all. Normally all untap, but
            // two families of effect keep one from untapping: a continuous
            // `Cant(Untap)` restriction (the aura's "enchanted creature
            // doesn't untap …", "~ doesn't untap during your untap step"), and
            // the one-shot `skip_next_untap` rider ([CR#701.43a] exert; the
            // temple/painland "~ doesn't untap during your next untap step").
            // The rider is consumed at this untap step whether or not the
            // permanent is tapped — its scope is exactly this one untap step.
            PhaseStep::Beginning(BeginningStep::Untap) => {
                let active = self.turn.active_player;
                let view = self.layers();
                let rows = crate::legal::cant_untap_rows(self, &view);
                let ids: Vec<_> = self.zones.battlefield.clone();
                let mut to_untap = Vec::new();
                let mut consume_rider = Vec::new();
                for id in ids {
                    let obj = self.objects.obj(id);
                    if obj.controller != active {
                        continue;
                    }
                    if obj.skip_next_untap {
                        // [CR#701.43a]: the one-shot rider fires once and is
                        // spent — collect it to clear below (it suppresses this
                        // untap regardless of the tapped state).
                        consume_rider.push(id);
                        continue;
                    }
                    // [CR#502.3]: a continuous "doesn't untap" restriction
                    // keeps a tapped permanent tapped.
                    if crate::legal::untap_forbidden_by(self, &rows, id) {
                        continue;
                    }
                    if obj.tapped {
                        to_untap.push(id);
                    }
                }
                drop(view);
                for id in consume_rider {
                    self.objects.obj_mut(id).skip_next_untap = false;
                }
                to_untap
                    .into_iter()
                    .map(|id| {
                        WorkItem::Emit(Occurrence::single(GameEvent::Untapped(
                            id,
                            // [CR#502.3]: the untap step's turn-based action —
                            // no source object.
                            Some(crate::event::Cause::untap(
                                deckmaste_core::Agency::TurnBasedAction,
                                None,
                            )),
                        )))
                    })
                    .collect()
            }
            // [CR#504.1]; [CR#103.8a] (two-player): turn 1 is the starting
            // player's, who skips their first draw.
            PhaseStep::Beginning(BeginningStep::Draw) if self.turn.turn_number > 1 => {
                // [CR#121.1,504.1]: the turn-based draw is the same atomic
                // `Act(Draw)` keyword action as an effect draw — the active
                // player draws one card (`on: vec![]`, the drawn card binds at
                // apply). A turn-based action has no source object.
                vec![WorkItem::Emit(Occurrence::single(GameEvent::Act(Act {
                    verb: deckmaste_core::VerbName::from("Draw"),
                    who: Some(self.turn.active_player),
                    on: vec![],
                    from: None,
                    to: None,
                    // [CR#703.4d]: agency distinguishes the draw-step draw
                    // from an effect-instructed one in recorded history.
                    cause: Some(crate::event::Cause::draw(
                        deckmaste_core::Agency::TurnBasedAction,
                        None,
                    )),
                    // The FUTURE window; its apply late-binds the top card and
                    // schedules the draw's `FinalizeAct`.
                    committed: false,
                    contents: None,
                    batch: None,
                    inherited: std::collections::HashSet::new(),
                    contained: false,
                })))]
            }
            PhaseStep::Beginning(BeginningStep::Draw) => vec![],
            // [CR#508.1]: the active player declares attackers — surface the
            // decision as this step's turn-based action.
            PhaseStep::Combat(CombatStep::DeclareAttackers) => vec![WorkItem::DeclareAttackers],
            // [CR#509.1]: the defending player declares blockers — but only when
            // there is something to block. [CR#508.8]: with no creatures
            // attacking, the Declare Blockers step is skipped (like
            // `check_hand_size` skipping the trivial discard).
            PhaseStep::Combat(CombatStep::DeclareBlockers)
                if !self.combat.attackers().is_empty() =>
            {
                vec![WorkItem::DeclareBlockers]
            }
            PhaseStep::Combat(CombatStep::DeclareBlockers) => vec![],
            // [CR#510.1,510.4]: assign + deal combat damage — but only when
            // something is attacking. With no attackers there is no damage to
            // assign ([CR#508.8] already skipped blockers); skip the step's work
            // like the empty Declare Blockers case. Both combat-damage steps
            // surface the same turn-based action; `assign_combat_damage` filters
            // sources by which step is current (first/double strikers in the
            // first step, normal + double strikers in the regular one).
            PhaseStep::Combat(CombatStep::FirstCombatDamage | CombatStep::CombatDamage)
                if !self.combat.attackers().is_empty() =>
            {
                vec![WorkItem::AssignCombatDamage]
            }
            PhaseStep::Combat(CombatStep::FirstCombatDamage | CombatStep::CombatDamage) => vec![],
            // [CR#511.1]: the End of Combat step has NO turn-based actions — the
            // removal-from-combat ([CR#511.3]) happens as the step *ends*, after
            // its priority window, scheduled from `end_of_step_items` (so an
            // "at end of combat" trigger resolving during that priority can still
            // read combat state).
            // [CR#514.1]: discard to hand size — checked after StepBegan.
            // [CR#514.2]: marked damage is removed from all permanents;
            // "until end of turn" continuous effects expire ([CR#514.2]).
            PhaseStep::Ending(EndingStep::Cleanup) => {
                self.clear_marked_damage();
                self.expire_end_of_turn();
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
            self.objects.obj_mut(id).clear_damage(); // [CR#514.2]
        }
    }

    /// What follows a step's turn-based actions: the priority barrier, or
    /// the step end for the no-priority steps ([CR#502.4,514.3] — cleanup's
    /// sweep runs per [CR#514.2] but can never act in the skeleton).
    fn step_tail(&self, s: PhaseStep) -> Vec<WorkItem> {
        match s {
            PhaseStep::Beginning(BeginningStep::Untap) => self.end_of_step_items(),
            PhaseStep::Ending(EndingStep::Cleanup) => {
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

    /// [CR#500.5]: pools empty at the end of every step; then the next step
    /// begins (wrapping into the next turn's untap).
    pub(crate) fn end_of_step_items(&self) -> Vec<WorkItem> {
        let mut items: Vec<WorkItem> = self
            .players
            .iter()
            // Lost players have left the game; nothing of theirs empties.
            .filter(|p| !p.lost && !p.mana_pool.is_empty())
            .map(|p| {
                WorkItem::Emit(Occurrence::single(GameEvent::ManaEmptied(ManaEmptied {
                    player: p.id,
                    ending: self.turn.current,
                })))
            })
            .collect();
        // [CR#511.3]: removal from combat happens as the End of Combat step ends
        // — after its priority window, before the next step begins.
        if self.turn.current == PhaseStep::Combat(CombatStep::EndOfCombat) {
            items.push(WorkItem::EndOfCombat);
        }
        items.push(WorkItem::BeginStep(
            successor(self.turn.current).unwrap_or(PhaseStep::Beginning(BeginningStep::Untap)),
        ));
        items
    }

    /// [CR#704.3]: apply a state-based-action batch, then re-check ONLY if it
    /// actually performed something. A batch whose events were all suppressed
    /// (e.g. a destroy canted by indestructible) applies to an empty result and
    /// must NOT re-trigger the check — otherwise a persistent condition (lethal
    /// damage on an indestructible creature) loops forever.
    fn emit_sba_batch(&mut self, events: Vec<GameEvent>) -> Progress {
        // Snapshot the agenda length before applying: `apply_occurrence` may
        // schedule follow-on work items at the front (e.g. `Emit(ZoneChange)`
        // future-form from an `Act(Destroy).apply`). If we re-check immediately
        // after, the follow-ons haven't run yet so the board looks unchanged —
        // a destructible creature with lethal damage hasn't moved yet
        // and the re-check re-emits an Act(Destroy), looping. By
        // inserting the re-check AFTER the follow-on slots the re-check
        // runs once the future-form and past-form `ZoneChange` facts
        // have settled (and the creature is gone), so the next sweep is
        // clean.
        let n_before = self.agenda.len();
        let applied = self.apply_occurrence(Occurrence::Batch(events));
        let n_after = self.agenda.len();
        let changed = !matches!(&applied, Occurrence::Batch(facts) if facts.is_empty());
        if changed {
            // `n_after - n_before` items were prepended by apply_occurrence
            // (the follow-on Emit(ZoneChange) etc). Insert the
            // re-check right behind them so they settle before the
            // next sweep.
            let added = n_after.saturating_sub(n_before);
            self.agenda.insert(added, WorkItem::CheckSbas);
        }
        Progress::Applied(applied)
    }

    /// [CR#704.3]: sweep; if anything acted, emit the whole sweep as ONE
    /// simultaneous batch and re-check before the queued `OpenPriority` runs.
    fn check_sbas(&mut self) -> Progress {
        // [CR#704.5j]: the legend rule needs a choice — surface it before the
        // mechanical, choice-free SBAs. One group per decision cycle; the
        // submission re-checks, so remaining groups and the mechanical sweep
        // follow.
        if let Some((player, candidates)) = sba::legend_rule_groups(self).into_iter().next() {
            self.pending = Some(DecisionPointKind::LegendRule(
                crate::decide::pending::LegendRule { player, candidates },
            ));
            return Progress::SbasChecked { actions: 0 };
        }
        let actions = sba::sweep(self);
        // [CR#704.5h]: deathtouch provenance now rides the damage mark itself,
        // so a creature that regenerates (or otherwise has its damage removed)
        // loses the deathtouch clause with the marks — no separate per-check
        // window to close. The mark persists exactly as long as the damage
        // does ([CR#514.2] cleanup / [CR#701.19a] heal), like lethal marked
        // damage, and the `EmitSbaBatch` state-change guard keeps an
        // indestructible creature's persistent destroy from looping.
        self.close_gated_draw_windows();
        let count = Uint::try_from(actions.len()).expect("action count fits in Uint");
        if count > 0 {
            // Re-check is conditional on this batch actually changing state.
            self.schedule_front(vec![WorkItem::EmitSbaBatch(actions)]);
        }
        Progress::SbasChecked { actions: count }
    }

    /// [CR#704.5b] is a WINDOWED predicate ("since the last time SBAs were
    /// checked"). When a `CantLose` gate ([CR#101.1], ADR U5) suppresses an
    /// empty-draw loss, close that player's window so the loss does NOT
    /// retroactively fire once the gate leaves — unlike the standing life/
    /// poison predicates, which re-fire at the first ungated check.
    fn close_gated_draw_windows(&mut self) {
        // Fast path: the overwhelmingly common check has no pending empty-draw,
        // so skip the fixpoint `layers()` build entirely.
        if !self.players.iter().any(|p| p.drew_from_empty && !p.lost) {
            return;
        }
        let view = self.layers();
        let gated: Vec<PlayerId> = self
            .players
            .iter()
            .filter(|p| {
                p.drew_from_empty
                    && !p.lost
                    && self.gate_suppresses(&view, p.id, deckmaste_core::OutcomeGateKind::CantLose)
            })
            .map(|p| p.id)
            .collect();
        for p in gated {
            self.player_mut(p).drew_from_empty = false;
        }
    }

    /// [CR#514.1]: the active player discards to maximum hand size.
    ///
    /// Setting `pending` blocks the next `step()` before the already-queued
    /// `CheckSbas` is consumed; submission front-schedules the `Discarded`
    /// emits ahead of it, so the sweep still runs after the discards apply.
    fn check_hand_size(&mut self) -> Progress {
        let active = self.turn.active_player;
        let hand =
            Uint::try_from(self.zones.hands[active.index()].len()).expect("hand size fits in Uint");
        // [CR#402.2]: discard to the EFFECTIVE maximum hand size — `None` means
        // no maximum (Reliquary Tower), so nothing is discarded.
        let discarding = match self.effective_max_hand_size(active) {
            Some(max) => hand.saturating_sub(max),
            None => 0,
        };
        if discarding > 0 {
            self.pending = Some(DecisionPointKind::DiscardToHandSize(
                crate::decide::pending::DiscardToHandSize {
                    player: active,
                    count: discarding,
                },
            ));
        }
        Progress::HandSizeChecked { discarding }
    }

    /// [CR#701.9b]: a resolving discard surfaces its card choice when the work
    /// item applies — the hand may have changed since the discard was
    /// scheduled. An instruction to discard more cards than the hand holds
    /// discards the whole hand (the excess is impossible and ignored,
    /// [CR#101.3]); an empty hand (count 0) surfaces nothing.
    /// [CR#401.7]: reposition a card ALREADY in its owner's library to `end` of
    /// that library, `offset` cards in — direct `VecDeque` surgery keeping the
    /// `ObjectId` (no remint, no `ZoneChange`, no zone-change trigger; scry
    /// never removes a card from the library, [CR#701.22a]). The offset is
    /// resolved against the library AFTER the card is pulled out, so a
    /// same-library move lands where the anchor names it. When a post-pick
    /// arrange scope is armed the landing is recorded for the finalizer.
    fn reposition_library(
        &mut self,
        object: ObjectId,
        end: crate::agenda::LibraryEnd,
        offset: Uint,
    ) -> Progress {
        use crate::agenda::LibraryEnd;
        let owner = self.owner_of(object);
        self.remove_from_library(owner, object);
        let lib = &mut self.zones.libraries[owner.index()];
        let offset = offset as usize;
        let index = match end {
            LibraryEnd::Top => offset.min(lib.len()),
            LibraryEnd::Bottom => lib.len().saturating_sub(offset),
        };
        lib.insert(index, object);
        if let Some(scope) = self.arrange_scope.as_mut() {
            scope.landings.push(crate::state::ArrangeLanding {
                library_owner: owner,
                end,
                object,
            });
        }
        Progress::Repositioned(object)
    }

    /// [CR#401.4]: the post-pick arrange finalizer. Drains the armed arrange
    /// scope, groups its landings into piles by (library, end), and for each
    /// pile of more than one card surfaces one arrange decision (the arranger
    /// orders it) — walked one pile at a time via
    /// [`DecisionContinuation::ArrangePiles`]. Piles of ≤1 card need no order and
    /// surface nothing (the reposition already placed them).
    fn arrange_piles(&mut self) -> Progress {
        let Some(scope) = self.arrange_scope.take() else {
            return Progress::PilesArranged { deciding: 0 };
        };
        let arranger = scope.arranger;
        // Group landings into piles preserving first-seen order, keyed by the
        // (library owner, end) axis.
        let mut piles: Vec<crate::state::ArrangePile> = Vec::new();
        for landing in scope.landings {
            if let Some(pile) = piles
                .iter_mut()
                .find(|p| p.library_owner == landing.library_owner && p.end == landing.end)
            {
                pile.objects.push(landing.object);
            } else {
                piles.push(crate::state::ArrangePile {
                    library_owner: landing.library_owner,
                    end: landing.end,
                    objects: vec![landing.object],
                });
            }
        }
        // Only piles of more than one card carry an order choice ([CR#401.4]).
        let deciding: Vec<crate::state::ArrangePile> =
            piles.into_iter().filter(|p| p.objects.len() > 1).collect();
        let count = Uint::try_from(deciding.len()).unwrap_or(Uint::MAX);
        self.open_next_arrange(arranger, deciding);
        Progress::PilesArranged { deciding: count }
    }

    /// [CR#401.4]: arrange a `MoveGroup`'s ordered landing — the `count` cards
    /// now at `end` of `library_owner`'s library. `AnyOrder`/`ChosenOrder`
    /// surface one arrange decision for `arranger`; `RandomOrder` shuffles the
    /// pile in place ([MTR 3.10] — no decision, no reveal); `SameOrder` leaves
    /// the group's landed order untouched (a minor seam: the reminted order is
    /// the batch's, not re-projected to selection order).
    fn arrange_group_landing(
        &mut self,
        arranger: PlayerId,
        arrangement: &deckmaste_core::Arrangement,
        library_owner: PlayerId,
        end: crate::agenda::LibraryEnd,
        count: Uint,
    ) -> Progress {
        use crate::agenda::LibraryEnd;
        let count = count as usize;
        if count <= 1 {
            return Progress::PilesArranged { deciding: 0 };
        }
        let lib = &self.zones.libraries[library_owner.index()];
        let pile_objects: Vec<ObjectId> = match end {
            LibraryEnd::Top => lib.iter().take(count).copied().collect(),
            LibraryEnd::Bottom => lib
                .iter()
                .skip(lib.len().saturating_sub(count))
                .copied()
                .collect(),
        };
        let pile = crate::state::ArrangePile {
            library_owner,
            end,
            objects: pile_objects,
        };
        match arrangement {
            deckmaste_core::Arrangement::AnyOrder | deckmaste_core::Arrangement::ChosenOrder(_) => {
                self.open_next_arrange(arranger, vec![pile]);
                Progress::PilesArranged { deciding: 1 }
            }
            deckmaste_core::Arrangement::RandomOrder => {
                use rand::seq::SliceRandom;
                let mut order = pile.objects.clone();
                order.shuffle(&mut self.rng);
                self.apply_arranged(&pile, &order);
                Progress::PilesArranged { deciding: 0 }
            }
            deckmaste_core::Arrangement::SameOrder => Progress::PilesArranged { deciding: 0 },
        }
    }

    /// Surface the next pending pile's arrange decision (or nothing when the
    /// walk is done), stashing the walk state in
    /// [`DecisionContinuation::ArrangePiles`]. `arranger` is the ordering player.
    pub(crate) fn open_next_arrange(
        &mut self,
        arranger: PlayerId,
        mut piles: Vec<crate::state::ArrangePile>,
    ) {
        if piles.is_empty() {
            return;
        }
        let current = piles.remove(0);
        self.pending = Some(DecisionPointKind::ArrangePile(
            crate::decide::pending::ArrangePile {
                player: arranger,
                objects: current.objects.clone(),
            },
        ));
        self.choice = Some(crate::state::DecisionContinuation::ArrangePiles {
            current,
            remaining: piles,
        });
    }

    /// [CR#401.4]: apply an `Arranged` answer — reorder the pile's cards within
    /// their library (removed then re-inserted contiguously at their end, top →
    /// down in the chosen order) keeping every `ObjectId`.
    pub(crate) fn apply_arranged(&mut self, pile: &crate::state::ArrangePile, order: &[ObjectId]) {
        use crate::agenda::LibraryEnd;
        let owner = pile.library_owner;
        for &object in order {
            self.remove_from_library(owner, object);
        }
        let lib = &mut self.zones.libraries[owner.index()];
        match pile.end {
            // Top pile: `order[0]` is the very top. Re-insert front-most last so
            // it ends up at the front of the deque (= top of library).
            LibraryEnd::Top => {
                for &object in order.iter().rev() {
                    lib.push_front(object);
                }
            }
            // Bottom pile: `order[0]` sits above the rest of the bottom cards;
            // push_back in order lands them bottom-most, top → down.
            LibraryEnd::Bottom => {
                for &object in order {
                    lib.push_back(object);
                }
            }
        }
    }

    /// [CR#707.10c,115.7d]: surface a `Retarget` decision re-targeting
    /// the COMMITTED stack entry `entry`. Re-derives the entry's target specs
    /// via `stack_object_target_specs` (shared with the announce slot) and
    /// their fresh legal candidates via `legal_targets_for_specs` (shared
    /// with `surface_target_choice`), then unions in the entry's CURRENT
    /// target per slot — leaving a slot unchanged is always allowed, even
    /// when the current target is no longer fresh-legal (it left play,
    /// gained hexproof, …); only a CHANGED slot must land on a fresh-legal
    /// candidate ([CR#707.10c]).
    ///
    /// A no-op (no decision surfaced) when `entry` has already left the stack
    /// — it may vanish between this work item being scheduled and running
    /// (e.g. countered in response) — or when its ability has no targets.
    /// Never crashes: an unresolvable retarget fizzles like any invalid
    /// semantic input.
    fn open_choose_new_targets(&mut self, player: PlayerId, entry: ObjectId) -> Progress {
        let Some(found) = self.stack.iter().find(|e| e.id == entry) else {
            return Progress::NewTargetsOpened { specs: 0 };
        };
        let view = self.layers();
        let specs =
            self.stack_object_target_specs(&view, &found.object, found.chosen_modes.as_ref());
        let current = found.targets.clone();
        let activation = found.activation;
        if specs.is_empty() {
            return Progress::NewTargetsOpened { specs: 0 };
        }
        let mut legal = self.retarget_candidates_for_specs(&specs, entry, activation, &current);
        // [CR#707.10c]: the union rule — every current target of a slot is a
        // keepable choice, even when it didn't make the fresh legal cut
        // (keeping the ENTIRE current set is always legal, final-set rule).
        for (slot, cur_set) in legal.iter_mut().zip(current.iter()) {
            for &cur in cur_set {
                if !slot.contains(&cur) {
                    slot.push(cur);
                }
            }
        }
        let count = Uint::try_from(specs.len()).expect("target-spec count fits in Uint");
        self.pending = Some(DecisionPointKind::Retarget(
            crate::decide::pending::Retarget {
                player,
                entry,
                spec: specs,
                legal,
            },
        ));
        Progress::NewTargetsOpened { specs: count }
    }

    /// [CR#608.2c,608.2d]: surface the resolution-time NUMBER choice for a
    /// `ChooseAndNote(key, NotedKind::Number)` ("choose a number"). Always
    /// surfaces (engine policy: every choice is explicit); the submit stores
    /// the answer in the armed activation register.
    fn open_choose_note_number(
        &mut self,
        player: PlayerId,
        key: deckmaste_core::Ident,
    ) -> Progress {
        self.pending = Some(DecisionPointKind::ChooseNoteNumber(
            crate::decide::pending::ChooseNoteNumber { player, key },
        ));
        Progress::NoteChoiceOpened
    }

    /// [CR#705.1]: draw `count` coins and front-schedule ONE simultaneous
    /// batch ([CR#603.2c] — the multi-discard precedent). A CALLED flip
    /// ([CR#705.2]) instead surfaces a `CallFlip` decision per coin; the
    /// draw happens as each call is submitted.
    fn flip_coins(&mut self, player: PlayerId, count: Uint, called: bool) -> Progress {
        if count == 0 {
            return Progress::CoinsFlipped { count: 0 };
        }
        if called {
            self.pending = Some(DecisionPointKind::CallFlip(
                crate::decide::pending::CallFlip { player },
            ));
            self.choice = Some(crate::state::DecisionContinuation::CallFlip {
                player,
                remaining: count,
                events: Vec::new(),
            });
            return Progress::CoinsFlipped { count };
        }
        let events: Vec<GameEvent> = (0..count)
            .map(|_| {
                GameEvent::CoinFlipped(CoinFlipped {
                    player,
                    heads: self.rng.random(),
                    won: None,
                })
            })
            .collect();
        self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(events))]);
        Progress::CoinsFlipped { count }
    }

    /// [CR#706.1..706.2]: draw `count` naturals in `1..=sides`; `result =
    /// natural` until the modifier pipeline lands (engine-replace-roll).
    fn roll_dice(&mut self, player: PlayerId, count: Uint, sides: Uint) -> Progress {
        let events: Vec<GameEvent> = (0..count)
            .map(|_| {
                let natural = self.rng.random_range(1..=sides);
                GameEvent::DieRolled(DieRolled {
                    player,
                    sides,
                    natural,
                    result: natural,
                })
            })
            .collect();
        if !events.is_empty() {
            self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(events))]);
        }
        Progress::DiceRolled { count }
    }

    /// [CR#106.1b]: surfaces the color choice for a resolving `AddMana` whose
    /// production is not fixed. Always surfaces — engine policy: every choice
    /// is explicit, even a single-option one.
    fn open_choose_mana_color(
        &mut self,
        player: PlayerId,
        options: Vec<ColorOrColorless>,
        amount: Uint,
        riders: Vec<deckmaste_core::ManaRider>,
        provenance: crate::player::ManaProvenance,
    ) -> Progress {
        debug_assert!(
            !options.is_empty(),
            "a mana choice offers at least one option"
        );
        self.pending = Some(DecisionPointKind::ChooseManaColor(
            crate::decide::pending::ChooseManaColor {
                player,
                options,
                amount,
                riders,
                provenance,
            },
        ));
        Progress::ManaColorOpened
    }

    /// [CR#106.1b]: surfaces the multi-symbol-run choice for a resolving
    /// `AddMana` whose production is the filterland "{W}{W}, {W}{U}, or
    /// {U}{U}". Always surfaces (engine policy: every choice is explicit).
    fn open_choose_mana_mode(
        &mut self,
        player: PlayerId,
        options: Vec<Vec<ColorOrColorless>>,
        amount: Uint,
        riders: Vec<deckmaste_core::ManaRider>,
        provenance: crate::player::ManaProvenance,
    ) -> Progress {
        debug_assert!(
            !options.is_empty() && options.iter().all(|run| !run.is_empty()),
            "a mana-run choice offers at least one non-empty run"
        );
        self.pending = Some(DecisionPointKind::ChooseManaMode(
            crate::decide::pending::ChooseManaMode {
                player,
                options,
                amount,
                riders,
                provenance,
            },
        ));
        Progress::ManaModeOpened
    }

    /// [CR#508.1a]: surfaces the Declare Attackers decision for the active
    /// player. Always surfaces (even with an empty legal set — the player
    /// declares no attackers with an empty vec); submission front-schedules
    /// the `Attacking` batch ahead of the already-queued step tail, mirroring
    /// `check_hand_size`.
    fn declare_attackers(&mut self) -> Progress {
        let active = self.turn.active_player;
        let legal = legal_attackers(self, active);
        // [CR#508.1b]: in the two-player game the sole defender is the
        // non-active player; the legal attack targets are their proxy plus
        // every planeswalker they control ([CR#506.3]).
        let defender = self.next_live_after(active);
        let legal_targets = crate::legal::legal_attack_targets(self, defender);
        let count = Uint::try_from(legal.len()).expect("attacker count fits in Uint");
        self.pending = Some(DecisionPointKind::DeclareAttackers(
            crate::decide::pending::DeclareAttackers {
                player: active,
                legal,
                legal_targets,
            },
        ));
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
        self.pending = Some(DecisionPointKind::DeclareBlockers(
            crate::decide::pending::DeclareBlockers {
                player: defender,
                legal,
            },
        ));
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

        // One derived view serves every keyword/power read in this pass.
        let view = self.layers();

        // [CR#510.4]: which sources deal this step is a pure keyword filter on
        // the current combat-damage step (no "already dealt" bookkeeping). In
        // the FIRST step only first/double strikers deal; in the REGULAR step
        // everyone EXCEPT a plain first-striker deals (a double-striker deals
        // in both). The regular filter includes everyone when no first
        // strike exists, so a single-pass combat is unchanged.
        let deals_this_step: fn(&crate::layer::LayeredView, ObjectId) -> bool =
            if self.turn.current == PhaseStep::Combat(CombatStep::FirstCombatDamage) {
                crate::combat::deals_first_strike
            } else {
                crate::combat::deals_regular_strike
            };

        // Each attacker is a source. Recipients route to WHAT IT IS ATTACKING
        // ([CR#508.1b]) — a defending player's proxy or a planeswalker they
        // control — not a fixed defender. Unblocked, target live → that target
        // ([CR#510.1b]); unblocked, target gone → NO damage (the attacker
        // "isn't attacking anything", [CR#510.1b]); blocked → its live blockers
        // ([CR#510.1c]); blocked-but-no-live-blockers → nothing (plain block,
        // no trample). Trample ([CR#702.19]) widens the blocked cases: a
        // blocked trampler's recipients are its live blockers followed
        // by the thing it's attacking ([CR#702.19b]) — spilling to that
        // planeswalker, NEVER past it to the defending player
        // ([CR#702.19f]; "trample over planeswalkers", [CR#702.19c], is
        // a separate keyword not modeled here).
        for &attacker in self.combat.attackers() {
            if !deals_this_step(&view, attacker) {
                continue; // [CR#510.4]: not dealing in this step.
            }
            // `target_of` is `None` only for an undeclared attacker
            // (unreachable in this loop); a live target still
            // on-side is validated below.
            let target = self.combat.target_of(attacker);
            let recipients: Vec<ObjectId> = if self.combat.is_blocked(attacker) {
                let mut blockers = self.combat.blockers_of(attacker).to_vec();
                if crate::combat::has_keyword(&view, attacker, &KeywordAbility::Trample)
                    && let Some(t) = target
                    && self.combat_target_live(&view, t)
                {
                    // [CR#702.19b]: lethal to the blockers, excess to the thing
                    // it's attacking; a gone target takes no spill
                    // ([CR#510.1b]).
                    blockers.push(t);
                }
                blockers
            } else {
                // [CR#510.1b]: unblocked → all damage to what it's attacking,
                // but only while that target is still there.
                match target {
                    Some(t) if self.combat_target_live(&view, t) => vec![t],
                    _ => Vec::new(),
                }
            };
            Self::assign_source(&view, attacker, &recipients, &mut buffer, &mut queue);
        }
        // Each live blocker is a source dealing to the one attacker it blocks
        // ([CR#510.1d]) — exactly one recipient, always forced.
        for &attacker in self.combat.attackers() {
            for &blocker in self.combat.blockers_of(attacker) {
                if !deals_this_step(&view, blocker) {
                    continue; // [CR#510.4]: not dealing in this step.
                }
                Self::assign_source(&view, blocker, &[attacker], &mut buffer, &mut queue);
            }
        }

        let deciding = Uint::try_from(queue.len()).expect("deciding-source count fits in Uint");
        self.combat_damage = Some(crate::state::CombatDamage { buffer, queue });
        // Surface the first deciding source, or deal the batch now if none.
        self.open_next_assignment();
        Progress::CombatDamageOpened { deciding }
    }

    /// Removes up to `amount` counters of `kind` from `object`, clamping at 0
    /// (removing more than are present leaves none) and dropping the key at 0
    /// so an absent kind reads as zero everywhere. Shared by the
    /// `CounterRemoved` apply and planeswalker damage-to-loyalty
    /// ([CR#120.3c]).
    fn remove_counters_clamped(
        &mut self,
        object: ObjectId,
        kind: &deckmaste_core::Ident,
        amount: Uint,
    ) {
        let counters = &mut self.objects.obj_mut(object).counters;
        if let Some(have) = counters.get_mut(kind) {
            *have = have.saturating_sub(amount);
            if *have == 0 {
                counters.remove(kind);
            }
        }
    }

    /// [CR#508.1b,510.1b]: whether `target` is still a legal recipient of the
    /// attack it was declared against. A defending player's proxy always is; a
    /// planeswalker only while it is still on the battlefield under a defending
    /// player's control. A target that fails this "isn't attacking anything"
    /// ([CR#510.1b]), so its attacker assigns no combat damage. (Two-player
    /// only; multi-defender is a separate ticket. Removal-from-combat
    /// bookkeeping is a later task — this is the assignment-time safety net.)
    fn combat_target_live(&self, view: &crate::layer::LayeredView, target: ObjectId) -> bool {
        match self.objects.get(target).map(|o| o.source) {
            Some(ObjectSource::Player(_)) => true,
            Some(ObjectSource::Card(_)) => {
                let defender = self.next_live_after(self.turn.active_player);
                self.zones.battlefield.contains(&target)
                    && view.controller(target) == defender
                    && view.get(target).has_type(Type::Planeswalker)
            }
            None => false,
        }
    }

    /// [CR#511.3]: removal from combat, run as the End of Combat step *ends*
    /// (scheduled from `end_of_step_items` after the step's priority window,
    /// not as a turn-based action — [CR#511.1] gives this step none).
    /// Clears the registry so a later combat phase (and any SBA/other
    /// reader) never sees stale (reminted/dead) attacker/blocker
    /// designations.
    fn end_of_combat(&mut self) -> Progress {
        self.combat.clear();
        // [CR#511.2]: "until end of combat" floating effects/shields end here.
        self.expire_end_of_combat();
        Progress::CombatEnded
    }

    /// Assigns one source's combat damage. 0 power → nothing ([CR#510.1a]); 1
    /// recipient → all power to it (forced); ≥ 2 recipients → queue a
    /// free-division decision ([CR#510.1c]). `buffer` collects forced
    /// `DamageDealt`s; `queue` collects deciding sources.
    fn assign_source(
        view: &crate::layer::LayeredView,
        source: ObjectId,
        recipients: &[ObjectId],
        buffer: &mut Vec<GameEvent>,
        queue: &mut Vec<crate::state::PendingAssignment>,
    ) {
        let power = Self::power_of(view, source);
        if power == 0 || recipients.is_empty() {
            return; // [CR#510.1a]: 0 power (or no recipient) assigns nothing.
        }
        if recipients.len() == 1 {
            buffer.push(GameEvent::DamageDealt(DamageDealt {
                source,
                target: recipients[0],
                amount: power,
                // The combat-damage step's assignment ([CR#510.1]).
                combat: true,
            }));
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
            self.pending = Some(DecisionPointKind::AssignCombatDamage(
                crate::decide::pending::AssignCombatDamage {
                    player,
                    source,
                    recipients,
                },
            ));
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

    /// A creature's combat-damage output: its derived power as a non-negative
    /// number ([CR#510.1c]). `None`/negative power assigns 0.
    #[must_use]
    pub(crate) fn power_of(view: &crate::layer::LayeredView, id: ObjectId) -> Uint {
        match view.power(id) {
            Some(p) if p > 0 => {
                #[expect(clippy::cast_sign_loss, reason = "p > 0 in this arm")]
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
        self.pending = Some(DecisionPointKind::Priority(
            crate::decide::pending::Priority {
                player: holder,
                legal,
            },
        ));
        Progress::PriorityOpened(holder)
    }

    /// [CR#601.2i,602.2a]: the becomes-cast moment — take the single in-flight
    /// announce and commit it to the stack as a `StackEntry`, carrying
    /// id / object / controller / targets / x across unchanged ([CR#405]: the
    /// stack identity was minted at announce). Shared by the `SpellCast` and
    /// `AbilityActivated` apply arms, which differ only in their own
    /// debug-assert — so the moved-out `PendingStackEntry` is returned for the
    /// caller's check.
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight — an engine invariant (these events
    /// are only emitted while one is staged), not caller input.
    fn promote_announce(&mut self) -> crate::stack::PendingStackEntry {
        let pending = self.announcing.take().expect("an announce in flight");
        self.stack.push(StackEntry {
            id: pending.id,
            activation: pending.activation,
            object: pending.object.clone(),
            controller: pending.controller,
            targets: pending.targets.clone(),
            chosen_modes: pending.chosen_modes.clone(),
            x: pending.x,
            // [CR#601.2b,702.33d]: the announced optional-cost record rides
            // the committed entry AND the register file announcement and
            // resolution share, which is where the linked reads look
            // ([CR#607.2i]).
            paid_costs: pending.paid_costs.clone(),
            // A cast/activate promote is never a copy ([CR#707.10]) — copies
            // mint their own `StackEntry` directly.
            copy: false,
        });
        self.activation_set_paid_costs(pending.activation, &pending.paid_costs);
        pending
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]
    use std::sync::Arc;

    use deckmaste_core::Lookback;
    use deckmaste_core::PhaseStep;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::Act;
    use crate::event::Copied;
    use crate::event::CounterPlaced;
    use crate::event::GameEvent;
    use crate::event::GotDesignation;
    use crate::event::Occurrence;
    use crate::event::TurnBegan;
    use crate::event::ZoneChange;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// `record_history` logs substantive facts and skips the meta/intent set
    /// the trigger scan also skips ([CR#608.2i] mirrors trigger.rs).
    #[test]
    fn record_history_logs_facts_skips_meta() {
        let mut state = game();
        let id = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Battlefield),
        );

        // A substantive fact is logged.
        state.record_history(&Occurrence::single(GameEvent::Untapped(id, None)));
        assert_eq!(
            state
                .history
                .scan(Lookback::ThisGame, state.turn.turn_number)
                .count(),
            1
        );

        // A step onset IS recorded ([CR#603.2b] — the StepBegins-in-history
        // lift), with the active player on its view.
        state.record_history(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::PrecombatMain,
        )));
        assert_eq!(
            state
                .history
                .scan(Lookback::ThisGame, state.turn.turn_number)
                .count(),
            2,
            "StepBegan is recorded"
        );

        // A skipped (meta) fact is not.
        state.record_history(&Occurrence::single(GameEvent::TurnBegan(TurnBegan {
            player: PlayerId(0),
            turn: 1,
        })));
        assert_eq!(
            state
                .history
                .scan(Lookback::ThisGame, state.turn.turn_number)
                .count(),
            2,
            "TurnBegan is skipped"
        );
    }

    /// Mints a card-backed object onto the battlefield — the idiom
    /// `apply_zone_will_change` requires, since it reads the card's owner
    /// ([CR#108.3]) via `ObjectSource::Card` on the way out.
    fn mint_card_backed(state: &mut GameState, controller: PlayerId) -> ObjectId {
        let card = Arc::new(deckmaste_card::Card::Normal(
            deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                name: "Test Card".into(),
                ..deckmaste_card::Characteristics::default()
            }),
        ));
        let card_id = state.cards.push(card, controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// [CR#400.7j]: applying a zone change to a PUBLIC destination records
    /// old→new in the move record; a HIDDEN destination records nothing
    /// ([CR#400.7] — the object is lost).
    #[test]
    fn apply_zone_change_records_public_moves_only() {
        let mut state = game();
        let old = mint_card_backed(&mut state, PlayerId(0));

        state.apply_zone_will_change(
            old,
            Some(Zone::Battlefield),
            Zone::Exile,
            None,
            None,
            None,
            None,
        );
        let new = state.chase_moved(old);
        assert_ne!(new, old, "public move recorded");
        assert_eq!(state.objects.get(new).unwrap().zone, Some(Zone::Exile));

        // Move the NEW object to a hidden zone: no entry — chase dead-ends at
        // `new`.
        state.apply_zone_will_change(new, Some(Zone::Exile), Zone::Hand, None, None, None, None);
        assert_eq!(state.chase_moved(old), new, "hidden move NOT recorded");
        assert!(
            state.objects.get(new).is_none(),
            "new id is stale after leaving"
        );
    }

    /// An applied `Batch`'s recorded facts share ONE fresh batch id
    /// ([CR#603.2c] — they were one occurrence); `Single`s record `None`,
    /// and distinct batches get distinct ids.
    #[test]
    fn batch_members_share_a_batch_id_singles_record_none() {
        let mut state = game();
        let a = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        let b = state.objects.mint(
            ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Battlefield),
        );

        state.record_history(&Occurrence::single(GameEvent::Untapped(a, None)));
        state.record_history(&Occurrence::Batch(vec![
            GameEvent::Untapped(a, None),
            GameEvent::Untapped(b, None),
        ]));
        state.record_history(&Occurrence::Batch(vec![GameEvent::Untapped(b, None)]));

        let batches: Vec<Option<deckmaste_core::Uint>> =
            state.history.entries().map(|e| e.batch).collect();
        assert_eq!(batches[0], None, "a Single records no batch id");
        assert!(batches[1].is_some(), "batch members carry an id");
        assert_eq!(batches[1], batches[2], "members of ONE batch share it");
        assert!(
            batches[3].is_some() && batches[3] != batches[1],
            "a distinct batch gets a distinct id"
        );
    }

    /// Atomic dual-facet apply ([CR#616.1,603.3b]): a destroy-all's
    /// `Act(Destroy)` batch commits DIRECTLY into one past-form `ZoneChange`
    /// batch — there is NO intermediate future-form `ZoneChange` stage (the
    /// single-replace invariant: exactly one cant/replace opportunity, on the
    /// `Act`, none downstream). The batch stays ONE occurrence through
    /// every stage ([CR#603.2c]) — never per-member `Single`s — and the two
    /// dies-facts share a history batch id.
    #[test]
    fn batch_of_intents_evolves_as_one_batch() {
        let (mut state, _view, a) = crate::replace_registry::tests_support::lone_creature();
        let b = crate::replace_registry::tests_support::mint_creature_on_battlefield(&mut state);

        let destroy = |object| {
            GameEvent::Act(Act {
                verb: deckmaste_core::VerbName::from("Destroy"),
                who: None,
                on: vec![object],
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: Some(crate::event::Cause::destroy(
                    deckmaste_core::Agency::EffectInstruction,
                    None,
                )),
                committed: false,
                contents: None,
                batch: None,
                inherited: std::collections::HashSet::new(),
                contained: false,
            })
        };
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(vec![
            destroy(a),
            destroy(b),
        ]))]);

        // Stage 1: the Act(Destroy) batch applies.
        let crate::step::StepOutcome::Progress(crate::step::Progress::Applied(Occurrence::Batch(
            stage1,
        ))) = state.step()
        else {
            panic!("expected the Act(Destroy) batch to apply");
        };
        assert_eq!(stage1.len(), 2);

        // Stage 2: ONE committed past-form ZoneChange batch, applied DIRECTLY
        // from the Act batch — no intervening future-form ZoneChange stage.
        let crate::step::StepOutcome::Progress(crate::step::Progress::Applied(Occurrence::Batch(
            stage2,
        ))) = state.step()
        else {
            panic!("expected one past-form ZoneChange batch");
        };
        assert_eq!(stage2.len(), 2);
        assert!(
            stage2.iter().all(|e| matches!(
                e,
                GameEvent::ZoneChange(ZoneChange {
                    snapshot: Some(_),
                    ..
                })
            )),
            "stage 2 is the committed fact batch (no future-form ZoneChange), got {stage2:?}"
        );

        // Single-replace evidence: no replaceable future-form `ZoneChange` ever
        // entered the history for the destroy — the `Act` committed the move
        // itself.
        assert!(
            !state.history.entries().any(|e| matches!(
                e.fact,
                GameEvent::ZoneChange(ZoneChange { snapshot: None, .. })
            )),
            "a composite destroy emits NO separate replaceable future-form ZoneChange"
        );

        // The two dies-facts share one history batch id.
        let ids: Vec<Option<deckmaste_core::Uint>> = state
            .history
            .entries()
            .filter(|e| {
                matches!(
                    e.fact,
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(_),
                        ..
                    })
                )
            })
            .map(|e| e.batch)
            .collect();
        assert_eq!(ids.len(), 2);
        assert!(ids[0].is_some() && ids[0] == ids[1], "one shared batch id");
    }

    /// `CounterPlaced`'s apply fills the carrier's before/after TOTALS onto
    /// the occurred fact ([CR#714.2b] — `Crossed` reads them off the fact):
    /// a second placement sees the first's total as its `before`.
    #[test]
    fn counter_placed_apply_fills_before_and_after_totals() {
        let (mut state, _view, id) = crate::replace_registry::tests_support::lone_creature();
        let place = |n: deckmaste_core::Uint| {
            GameEvent::CounterPlaced(CounterPlaced {
                object: id,
                kind: "P1P1Counter".into(),
                amount: n,
                before: 0,
                after: 0,
                cause: None,
            })
        };

        state.schedule_front(vec![
            WorkItem::Emit(Occurrence::single(place(2))),
            WorkItem::Emit(Occurrence::single(place(3))),
        ]);
        let facts: Vec<(
            deckmaste_core::Uint,
            deckmaste_core::Uint,
            deckmaste_core::Uint,
        )> = (0..2)
            .map(|_| match state.step() {
                crate::step::StepOutcome::Progress(crate::step::Progress::Applied(
                    Occurrence::Single(GameEvent::CounterPlaced(CounterPlaced {
                        amount,
                        before,
                        after,
                        ..
                    })),
                )) => (amount, before, after),
                other => panic!("expected an applied CounterPlaced, got {other:?}"),
            })
            .collect();
        assert_eq!(facts[0], (2, 0, 2), "first placement: 0 -> 2");
        assert_eq!(facts[1], (3, 2, 5), "second placement: 2 -> 5");
    }

    /// A cause-amount zone-change batch (a discard) fixes "that much" to
    /// its CARD COUNT ([CR#107.3,701.9a]) — "discards all the cards in
    /// their hand, then draws that many" reads the batch size, not 1. The
    /// per-verb admission is the entailment table's `amount` column.

    /// `priority_tail` is the shared `[CheckSbas, PlaceTriggers, OpenPriority]`
    /// trailer reused by every action that emits and then re-opens priority.
    #[test]
    fn priority_tail_is_check_place_open() {
        assert_eq!(
            GameState::priority_tail(),
            vec![
                WorkItem::CheckSbas,
                WorkItem::PlaceTriggers,
                WorkItem::OpenPriority,
            ],
        );
    }

    /// `announce_schedule` reifies the full announce procedure shared by cast
    /// and activate — only the `begin` shell and the becomes-cast event differ.
    #[test]
    fn announce_schedule_matches_cast_and_activate_shape() {
        let begin = WorkItem::BeginCast(crate::object::ObjectId::from_raw(1));
        let event = GameEvent::SpellCast(crate::object::ObjectId::from_raw(1));
        assert_eq!(
            GameState::announce_schedule(begin.clone(), event.clone()),
            vec![
                begin,
                WorkItem::AnnounceModes,
                WorkItem::AnnounceOptionalCosts { index: 0 },
                WorkItem::AnnounceX,
                WorkItem::AnnounceTargets,
                WorkItem::ChooseCostOptions,
                WorkItem::OpenPayment,
                WorkItem::Emit(Occurrence::single(event)),
                WorkItem::CheckSbas,
                WorkItem::PlaceTriggers,
                WorkItem::OpenPriority,
            ],
        );
    }

    /// `promote_announce` takes the single in-flight announce and pushes the
    /// committed `StackEntry`, carrying id / object / controller / targets / x
    /// across unchanged. It is the shared body of the `SpellCast` and
    /// `AbilityActivated` apply arms.
    #[test]
    fn promote_announce_pushes_committed_entry() {
        use crate::stack::PendingStackEntry;
        use crate::stack::StackObject;

        let mut state = game();
        let id = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Stack),
        );
        state.announcing = Some(PendingStackEntry {
            activation: crate::ActivationId::NONE,
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
            id,
            object: StackObject::Spell(id),
            controller: PlayerId(0),
            origin: Zone::Hand,
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: Some(3),
            concretized: None,
            alternative_cost: None,
        });

        let pending = state.promote_announce();

        assert!(state.announcing.is_none(), "the announce slot is cleared");
        assert_eq!(state.stack.len(), 1, "one entry committed to the stack");
        let top = state.stack.last().expect("the promoted entry");
        assert_eq!(top.id, id);
        assert_eq!(top.object, StackObject::Spell(id));
        assert_eq!(top.controller, PlayerId(0));
        assert_eq!(top.targets, Vec::<Vec<crate::object::ObjectId>>::new());
        assert_eq!(top.x, Some(3));
        // The returned pending lets callers run their own debug-asserts.
        assert_eq!(pending.id, id);
    }

    /// Applying `GotDesignation` writes the player-scope designation store, and
    /// a second apply is a no-op ([CR#702.131c] — set once; any number of
    /// players may hold it, none loses it).
    #[test]
    fn got_designation_applies_idempotently() {
        let mut state = game();
        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = crate::player::PlayerId(0);

        state.schedule_front(vec![WorkItem::Emit(Occurrence::Single(
            GameEvent::GotDesignation(GotDesignation { player: p0, name }),
        ))]);
        let _ = state.step();
        assert!(
            state.designations.players.contains_key(&(p0, name)),
            "player 0 now holds the city's blessing"
        );

        // A second apply does not panic and leaves the entry present.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Single(
            GameEvent::GotDesignation(GotDesignation { player: p0, name }),
        ))]);
        let _ = state.step();
        assert!(state.designations.players.contains_key(&(p0, name)));
    }

    // --- Legend-rule wiring (Task C3) ---

    // --- SBA loop termination (EmitSbaBatch) ---

    mod sba_loop_termination {
        use std::path::Path;
        use std::sync::Arc;

        use deckmaste_card::Card;
        use deckmaste_core::Predicate;
        use deckmaste_core::Type;
        use deckmaste_core::Zone;
        use deckmaste_plugin::plugin::Plugin;

        use crate::agenda::WorkItem;
        use crate::matches as obj_matches;
        use crate::player::PlayerId;
        use crate::state::GameConfig;
        use crate::state::GameState;
        use crate::state::PlayerConfig;
        use crate::state::StartingPlayer;
        use crate::step::StepOutcome;

        fn builtin() -> Plugin {
            Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"))
                .unwrap()
        }

        fn canon() -> Plugin {
            Plugin::load_with_sibling_prelude(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
            )
            .unwrap()
        }

        fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
            vec![Arc::clone(card); n]
        }

        /// Player 0's deck = Darksteel Myr (indestructible 0/1), one on the
        /// field.
        fn myr_on_field() -> (GameState, crate::object::ObjectId) {
            let myr = Arc::new(canon().card("Darksteel Myr").unwrap().core);
            let forest = Arc::new(builtin().card("Forest").unwrap().core);
            let mut state = GameState::new(GameConfig {
                players: vec![
                    PlayerConfig {
                        deck: deck(&myr, 10),
                    },
                    PlayerConfig {
                        deck: deck(&forest, 10),
                    },
                ],
                seed: 1,
                starting_life: 20,
                starting_player: StartingPlayer::Fixed(PlayerId(0)),
                sba_rules: vec![],
                conferral_rules: vec![],
                damage_result_rules: vec![],
                counter_decls: std::collections::HashMap::new(),
                subtypes: std::collections::HashMap::new(),
                types: std::collections::HashMap::new(),
            });
            let m = *state.zones.hands[0]
                .iter()
                .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
                .expect("a Darksteel Myr in the opening hand");
            state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
            state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
            state.zones.battlefield.push(m);
            (state, m)
        }

        /// [CR#704.3,704.5g,702.12b]: an indestructible creature with lethal
        /// marked damage holds a persistent SBA condition (destroy is canted).
        /// `check_sbas` must TERMINATE — the `EmitSbaBatch` handler re-checks
        /// only when the batch actually changed state; a fully-suppressed batch
        /// (the cant reduces it to `Batch(vec![])`) must NOT re-trigger the
        /// check, or the loop runs forever.
        #[test]
        fn indestructible_lethal_damage_sba_check_terminates() {
            let (mut state, myr) = myr_on_field();
            state.objects.obj_mut(myr).set_marked_damage(1); // toughness 1 → lethal
            state.schedule_front(vec![WorkItem::CheckSbas]);
            // Bounded drive: a correct SBA loop settles in a handful of steps.
            // If it hasn't settled in 50, it's the infinite loop.
            let mut settled = false;
            for _ in 0..50 {
                if let StepOutcome::NeedsDecision(_) = state.step() {
                    settled = true;
                    break;
                }
            }
            assert!(
                settled,
                "the SBA check must terminate, not loop on the canted destroy"
            );
            assert!(
                state.zones.battlefield.contains(&myr),
                "indestructible creature survives"
            );
        }
    }

    mod legend_choice {
        use std::path::Path;
        use std::sync::Arc;

        use deckmaste_card::Card;
        use deckmaste_core::StatValue;
        use deckmaste_core::Supertype;
        use deckmaste_core::Type;
        use deckmaste_core::Zone;
        use deckmaste_plugin::plugin::Plugin;

        use crate::agenda::WorkItem;
        use crate::decide::DecisionPointKind;
        use crate::object::ObjectId;
        use crate::object::ObjectSource;
        use crate::player::PlayerId;
        use crate::state::GameConfig;
        use crate::state::GameState;
        use crate::state::PlayerConfig;
        use crate::state::StartingPlayer;
        use crate::step::StepOutcome;

        fn builtin() -> Plugin {
            Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"))
                .unwrap()
        }

        /// A two-player game with player 0's deck being Grizzly Bears (loaded
        /// from canon so the bear ends up on the battlefield).
        fn empty_game() -> GameState {
            let forest = Arc::new(builtin().card("Forest").unwrap().core);
            GameState::new(GameConfig {
                players: vec![
                    PlayerConfig {
                        deck: vec![Arc::clone(&forest)],
                    },
                    PlayerConfig {
                        deck: vec![Arc::clone(&forest)],
                    },
                ],
                seed: 42,
                starting_life: 20,
                starting_player: StartingPlayer::Fixed(PlayerId(0)),
                sba_rules: vec![],
                conferral_rules: vec![],
                damage_result_rules: vec![],
                counter_decls: std::collections::HashMap::new(),
                subtypes: std::collections::HashMap::new(),
                types: std::collections::HashMap::new(),
            })
        }

        fn legendary_creature(state: &mut GameState, name: &str, controller: PlayerId) -> ObjectId {
            let card = Arc::new(Card::Normal(deckmaste_card::CardFace::from(
                deckmaste_card::Characteristics {
                    name: name.into(),
                    types: vec![Type::Creature.def()],
                    supertypes: vec![Supertype::Legendary],
                    power: Some(StatValue::Number(2)),
                    toughness: Some(StatValue::Number(2)),
                    ..deckmaste_card::Characteristics::default()
                },
            )));
            let card_id = state.cards.push(Arc::clone(&card), controller);
            let id = state.objects.mint(
                ObjectSource::Card(card_id),
                controller,
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        }

        /// [CR#704.5j]: two same-name legendaries controlled by one player →
        /// `check_sbas` surfaces `NeedsDecision(LegendRule)` before the
        /// mechanical sweep. The decision surfaces on the call after
        /// `CheckSbas` sets pending (the runner's step loop pattern:
        /// Progress* then `NeedsDecision`).
        #[test]
        fn legend_rule_surfaces_a_choice() {
            let mut state = empty_game();
            state.sba_rules = builtin().sba_rules;
            legendary_creature(&mut state, "Bob", PlayerId(0));
            legendary_creature(&mut state, "Bob", PlayerId(0));
            state.schedule_front(vec![WorkItem::CheckSbas]);
            // First step processes CheckSbas and sets pending.
            let _ = state.step();
            // Second step sees pending and returns NeedsDecision.
            let outcome = state.step();
            assert!(
                matches!(
                    outcome,
                    StepOutcome::NeedsDecision(DecisionPointKind::LegendRule(
                        crate::decide::pending::LegendRule {
                            player: PlayerId(0),
                            ..
                        }
                    ))
                ),
                "got {outcome:?}"
            );
        }

        fn nonlegendary_creature(
            state: &mut GameState,
            name: &str,
            controller: PlayerId,
        ) -> ObjectId {
            let card = Arc::new(Card::Normal(deckmaste_card::CardFace::from(
                deckmaste_card::Characteristics {
                    name: name.into(),
                    types: vec![Type::Creature.def()],
                    supertypes: vec![],
                    power: Some(StatValue::Number(2)),
                    toughness: Some(StatValue::Number(2)),
                    ..deckmaste_card::Characteristics::default()
                },
            )));
            let card_id = state.cards.push(Arc::clone(&card), controller);
            let id = state.objects.mint(
                ObjectSource::Card(card_id),
                controller,
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        }

        /// [CR#704.5j]: keeping one of three same-name legendaries puts the
        /// other two into their owners' graveyards.
        #[test]
        fn legend_rule_keeps_one_graveyards_the_rest() {
            use crate::decide::Decision;
            let mut state = empty_game();
            state.sba_rules = builtin().sba_rules;
            let a = legendary_creature(&mut state, "Bob", PlayerId(0));
            let b = legendary_creature(&mut state, "Bob", PlayerId(0));
            let c = legendary_creature(&mut state, "Bob", PlayerId(0));
            state.schedule_front(vec![WorkItem::CheckSbas]);
            // Step until LegendRule surfaces.
            let _ = state.step(); // CheckSbas sets pending
            let _ = state.step(); // NeedsDecision
            // Keep `a`; the other two go to the graveyard.
            state
                .submit_decision(Decision::Chosen(vec![a]))
                .expect("valid legend-rule decision");
            // Drive the scheduled graveyard moves + recheck to completion.
            while matches!(state.step(), StepOutcome::Progress(_)) {}
            assert!(
                state.zones.battlefield.contains(&a),
                "kept legend should still be on the battlefield"
            );
            assert!(
                !state.zones.battlefield.contains(&b) && !state.zones.battlefield.contains(&c),
                "the two non-kept Bobs should be off the battlefield"
            );
            assert_eq!(
                state.zones.graveyards[0].len(),
                2,
                "the two non-kept Bobs should be in player 0's graveyard"
            );
        }

        /// Non-legendary duplicates must not trigger the legend rule.
        #[test]
        fn nonlegendary_duplicates_do_not_trigger_legend_rule() {
            let mut state = empty_game();
            state.sba_rules = builtin().sba_rules;
            nonlegendary_creature(&mut state, "Mox", PlayerId(0));
            nonlegendary_creature(&mut state, "Mox", PlayerId(0));
            state.schedule_front(vec![WorkItem::CheckSbas]);
            assert!(!matches!(
                state.step(),
                StepOutcome::NeedsDecision(DecisionPointKind::LegendRule(
                    crate::decide::pending::LegendRule { .. }
                ))
            ));
            assert!(!matches!(
                state.step(),
                StepOutcome::NeedsDecision(DecisionPointKind::LegendRule(
                    crate::decide::pending::LegendRule { .. }
                ))
            ));
        }
    }

    // --- [CR#502.3,701.43a]: untap-skip (doesn't-untap) primitive ----------

    /// A card-backed battlefield permanent controlled by `controller`,
    /// carrying `abilities`, entering tapped — the untap-step fixtures.
    fn tapped_perm(
        state: &mut GameState,
        controller: PlayerId,
        abilities: Vec<deckmaste_core::Ability>,
    ) -> ObjectId {
        let card = Arc::new(deckmaste_card::Card::Normal(
            deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                name: "Untap Fixture".into(),
                abilities,
                ..deckmaste_card::Characteristics::default()
            }),
        ));
        let card_id = state.cards.push(card, controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        state.objects.obj_mut(id).tapped = true;
        id
    }

    /// Run the untap step's turn-based action and APPLY its untap events —
    /// exactly what `begin_step` schedules — returning the ids that untapped.
    fn run_untap_step(state: &mut GameState) -> Vec<ObjectId> {
        use deckmaste_core::BeginningStep;
        let items = state.turn_based_actions(PhaseStep::Beginning(BeginningStep::Untap));
        let mut untapped = Vec::new();
        for item in items {
            if let WorkItem::Emit(Occurrence::Single(GameEvent::Untapped(id, cause))) = item {
                state.apply(GameEvent::Untapped(id, cause));
                untapped.push(id);
            }
        }
        untapped
    }

    /// A `Cant(Untap(what: Ref(This)))` continuous restriction ([CR#502.3])
    /// keeps its carrier tapped through every untap step, while an
    /// unrestricted permanent untaps normally.
    #[test]
    fn cant_untap_static_keeps_permanent_tapped() {
        use deckmaste_core::Ability;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticSpec;

        let mut state = game();
        state.turn.active_player = PlayerId(0);
        let restricted = tapped_perm(
            &mut state,
            PlayerId(0),
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Untap {
                    what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                },
            )))],
        );
        let plain = tapped_perm(&mut state, PlayerId(0), vec![]);

        let untapped = run_untap_step(&mut state);
        assert!(
            untapped.contains(&plain) && !untapped.contains(&restricted),
            "only the unrestricted permanent untaps ([CR#502.3])"
        );
        assert!(
            state.objects.obj(restricted).tapped,
            "the Cant(Untap) carrier stays tapped"
        );
        assert!(
            !state.objects.obj(plain).tapped,
            "the plain permanent untapped"
        );

        // The restriction is continuous: it holds on a later untap step too.
        state.objects.obj_mut(restricted).tapped = true;
        let untapped = run_untap_step(&mut state);
        assert!(
            !untapped.contains(&restricted) && state.objects.obj(restricted).tapped,
            "the continuous restriction still keeps it tapped next untap step"
        );
    }

    /// A one-shot `skip_next_untap` rider ([CR#701.43a] exert; the
    /// temple/painland mana riders) keeps its permanent tapped for exactly
    /// ONE untap step, is consumed, then the permanent untaps normally at the
    /// following untap step.
    #[test]
    fn skip_next_untap_rider_stays_tapped_once_then_untaps() {
        let mut state = game();
        state.turn.active_player = PlayerId(0);
        let perm = tapped_perm(&mut state, PlayerId(0), vec![]);
        state.objects.obj_mut(perm).skip_next_untap = true;

        // First untap step: the rider suppresses the untap and is consumed.
        let untapped = run_untap_step(&mut state);
        assert!(!untapped.contains(&perm), "the rider suppresses this untap");
        assert!(
            state.objects.obj(perm).tapped,
            "the permanent stays tapped once"
        );
        assert!(
            !state.objects.obj(perm).skip_next_untap,
            "the one-shot rider was consumed ([CR#701.43a])"
        );

        // Next untap step: no rider left — it untaps normally.
        let untapped = run_untap_step(&mut state);
        assert!(
            untapped.contains(&perm),
            "it untaps the following untap step"
        );
        assert!(
            !state.objects.obj(perm).tapped,
            "the permanent is now untapped"
        );
    }

    // --- [CR#707.10]: Copied — stale activated-ability source fizzles ------

    /// Copying an activated ability whose source object has already left the
    /// object store (a zone change removes an id from the store, [CR#400.7])
    /// must fizzle silently, never panic — `StackObject::Activated`'s source
    /// is carried by id only and is documented as "possibly gone, possibly
    /// changed" (`stack.rs`). Card-semantics / timing situations that leave
    /// a stale reference must never crash the engine (the Invalid semantic
    /// input fizzles decision,
    /// `docs/decisions/invalid-semantic-input-fizzles.md`); this
    /// pins the `Copied` apply arm's Activated-branch lookup against that
    /// invariant.
    #[test]
    fn copy_activated_ability_with_gone_source_fizzles() {
        use deckmaste_core::ActivatedAbility;
        use deckmaste_core::Cost;
        use deckmaste_core::Instruction;

        use crate::stack::StackEntry;
        use crate::stack::StackObject;
        use crate::trigger::TriggerBindings;

        let mut state = game();
        let source = mint_card_backed(&mut state, PlayerId(0));
        let ability_src = state.objects.obj(source).source;
        let entry_id = state
            .objects
            .mint(ability_src, PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            id: entry_id,
            object: StackObject::Activated {
                source,
                ability: Box::new(ActivatedAbility {
                    ability_word: None,
                    targets: [].into(),
                    cost: Cost(vec![].into()),
                    from: None,
                    window: None,
                    condition: None,
                    limits: vec![].into(),
                    effect: Instruction::Sequentially(vec![].into()).into(),
                }),
                bindings: TriggerBindings::default(),
            },
            controller: PlayerId(0),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: vec![],
            copy: false,
        });

        // The source permanent leaves play — its id is gone from the store
        // ([CR#400.7]) — while the ability it printed stays on the stack
        // ([CR#602.2a]: "It has the text of the ability that created it").
        state.objects.remove(source);

        let event = state.apply(GameEvent::Copied(Copied {
            original: entry_id,
            copy: None,
            controller: PlayerId(1),
        }));

        assert_eq!(
            event,
            GameEvent::Copied(Copied {
                original: entry_id,
                copy: None,
                controller: PlayerId(1),
            }),
            "fizzles: the event comes back unchanged, no copy minted"
        );
        assert_eq!(
            state.stack.len(),
            1,
            "no new stack entry was pushed — only the original ability remains"
        );
    }

    /// engine-transform: applying `Transformed` toggles a transforming DFC's
    /// `Side` in place, front↔back, identity preserved ([CR#712.18]).
    #[test]
    fn transformed_event_toggles_side() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_card::DoubleFacedLayout;

        use crate::object::Side;

        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front: CardFace::from(Characteristics {
                name: "Delverish".into(),
                ..Characteristics::default()
            }),
            back: CardFace::from(Characteristics {
                name: "Insectile Aberration".into(),
                ..Characteristics::default()
            }),
        };
        let mut state = game();
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);

        assert_eq!(state.objects.obj(id).side, Side::Front);
        state.apply(GameEvent::Transformed(id));
        assert_eq!(
            state.objects.obj(id).side,
            Side::Back,
            "transform flips to back [CR#712.18]"
        );
        state.apply(GameEvent::Transformed(id));
        assert_eq!(
            state.objects.obj(id).side,
            Side::Front,
            "transform again flips back to front"
        );
    }

    /// A cause-amount zone-change batch (a discard) fixes the magnitude
    /// anaphor to its CARD COUNT ([CR#107.3,701.9a]) — "discards all the cards
    /// in their hand, then draws that many" reads the batch size, not 1.
    #[test]
    fn discard_batch_fixes_the_magnitude_anaphor_to_its_card_count() {
        const TALLY: deckmaste_core::RefId = deckmaste_core::RefId(8);

        let (mut state, _view, source) = crate::replace_registry::tests_support::lone_creature();
        let frame = crate::test_support::frame_src(&state, source);
        // Two cards in hand to discard.
        for name in ["Discard A", "Discard B"] {
            let card = Arc::new(deckmaste_card::Card::Normal(
                deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                    name: name.into(),
                    types: vec![deckmaste_core::Type::Sorcery.def()],
                    ..deckmaste_card::Characteristics::default()
                }),
            ));
            let cid = state.cards.push(card, PlayerId(0));
            let id = state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Hand));
            state.zones.hands[0].push(id);
        }
        let initial_hand = state.zones.hands[0].len();
        state.run_effect(
            deckmaste_core::Instruction::producing(
                deckmaste_core::DefId(TALLY.0),
                deckmaste_core::Action::discard(
                    deckmaste_core::Reference::controller_parameter(),
                    deckmaste_core::Count::Literal(2),
                    true,
                ),
            ),
            &frame,
        );
        for _ in 0..30 {
            if state.activation_number(frame.activation, TALLY).is_some() {
                break;
            }
            assert!(
                matches!(state.step(), crate::step::StepOutcome::Progress(_)),
                "the random discard needs no player decision"
            );
        }
        assert_eq!(
            state.activation_number(frame.activation, TALLY),
            Some(2),
            "the discard clause's amount is its card count"
        );
        assert_eq!(
            state.zones.hands[0].len(),
            initial_hand - 2,
            "exactly two cards moved"
        );
    }
}
