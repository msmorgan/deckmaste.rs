//! The steppable core: `step()` pops one agenda item and returns one
//! `Progress`. Decisions surface on the following call; the runner loops.

use deckmaste_core::BeginningStep;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CombatStep;
use deckmaste_core::EndingStep;
use deckmaste_core::KeywordAbility;
use deckmaste_core::Phase;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use rand::seq::SliceRandom;

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::legal::legal_actions;
use crate::legal::legal_attackers;
use crate::legal::legal_blockers;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::sba;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameOutcome;
use crate::state::GameState;
use crate::turn::PriorityRound;
use crate::turn::successor;

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
    /// [CR#510.4]: a step the turn structure traverses was elided without
    /// opening (no `StepBegan`, no turn-based action, no priority) — today only
    /// the `FirstCombatDamage` step, skipped when no combat creature has first
    /// or double strike. `BeginStep` of the successor was scheduled instead.
    Skipped(Phase),
    /// A [CR#704] sweep ran; `actions` lost-player events were scheduled.
    SbasChecked { actions: Uint },
    /// [CR#603.3]: the placement barrier ran; `placed` triggers went on the
    /// stack this step (0 when none were waiting, or when an `OrderTriggers` /
    /// `ChooseTargets` decision surfaced instead).
    TriggersPlaced { placed: Uint },
    /// Cleanup's hand-size check ran ([CR#514.1]).
    HandSizeChecked { discarding: Uint },
    /// [CR#701.9b]: a resolving discard surfaced its card choice; `count` is
    /// how many cards the player must choose (clamped to the hand size; 0 =
    /// an empty hand, nothing surfaced).
    DiscardOpened { count: Uint },
    /// [CR#106.1b]: a resolving `AddMana` surfaced its color choice.
    ManaColorOpened,
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
    /// [CR#601.2c]: targets were announced for the in-flight spell (a
    /// `ChooseTargets` decision surfaces when `specs > 0`).
    TargetsAnnounced { specs: Uint },
    /// [CR#601.2b]: the in-flight cost's hybrid/Phyrexian symbols were
    /// concretized. `surfaced` is true when a `ChooseCostOptions` decision
    /// opened (the printed cost had a choosable symbol); false when the cost
    /// was plain and the stash was set directly (no decision).
    CostOptionsChosen { surfaced: bool },
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
            WorkItem::EndOfCombat => self.end_of_combat(),
            WorkItem::OpenPriority => self.open_priority(),
            WorkItem::BeginCast(object) => {
                self.begin_cast(object);
                Progress::Announcing(object)
            }
            WorkItem::BeginActivate { object, ability } => {
                self.begin_activate(object, ability);
                Progress::Announcing(object)
            }
            WorkItem::AnnounceX => {
                self.announce_x();
                Progress::XAnnounced
            }
            WorkItem::AnnounceTargets => {
                let specs = self.announce_targets();
                Progress::TargetsAnnounced { specs }
            }
            WorkItem::ChooseCostOptions => {
                let surfaced = self.choose_cost_options();
                Progress::CostOptionsChosen { surfaced }
            }
            WorkItem::PayCost => {
                self.pay_cost();
                Progress::CostPaid
            }
            WorkItem::DiscardCards { player, count } => self.open_discard_cards(player, count),
            WorkItem::ChooseManaColor {
                player,
                options,
                amount,
                riders,
            } => self.open_choose_mana_color(player, options, amount, riders),
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
        // Every amount-carrying event fixes the "that much" register
        // (`Count::ThatMuch`) as it actually happens — at the apply funnel,
        // after any replacement has rewritten the event.
        match &event {
            GameEvent::DamageDealt { amount, .. }
            | GameEvent::LifeLost { amount, .. }
            | GameEvent::LifeGained { amount, .. } => self.that_much = Some(*amount),
            _ => {}
        }
        match event {
            // Pure facts: nothing to mutate. `BecameTarget` ([CR#601.2c])
            // exists for the trigger scan (ward, [CR#702.21a]); the
            // targeting state itself lives in the announce slot / stack
            // entry.
            GameEvent::TurnBegan { .. }
            | GameEvent::StepBegan(_)
            | GameEvent::BecameTarget { .. } => event,
            // P0.W3 seams: grammar-complete events nothing emits yet —
            // their apply (RNG, counter storage [P0.W5]) is unbuilt.
            GameEvent::CoinFlipped { .. } | GameEvent::DieRolled { .. } => {
                todo!("P0.W3: random-event apply")
            }
            GameEvent::CounterPlaced { .. } | GameEvent::CounterRemoved { .. } => {
                todo!("P0.W3: counter-event apply (storage is P0.W5)")
            }
            GameEvent::Untapped(id) => {
                self.objects.obj_mut(id).tapped = false;
                event
            }
            GameEvent::WillDestroy { object, cause } => {
                // [CR#701.8a]: the destruction intent commits. [CR#702.12b]: a
                // destruction-replacement static (indestructible; a
                // regeneration shield once those exist) replaces it to nothing
                // — the object is untouched, so no zone move is scheduled.
                // Otherwise it evolves into the committed Battlefield→Graveyard
                // move, carrying the destroy cause ([CR#701.8b]) for the
                // "destroyed" view.
                if crate::legal::replaced_from_destruction(&self.layers(), object) {
                    GameEvent::WillDestroy { object, cause }
                } else {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object,
                            from: Some(Zone::Battlefield),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: cause.clone(),
                        },
                    ))]);
                    GameEvent::WillDestroy { object, cause }
                }
            }
            GameEvent::WillDraw { player, source } => {
                // [CR#121.1]: the draw intent commits. A card present → evolve
                // into the generic Library→Hand move (remint + LKI); the
                // returned `WillDraw` fact is what `CardsDrawnThisTurn` counts
                // in the history log. An empty library → DrewFromEmpty, the
                // failed-draw fact the loss SBA keys on ([CR#121.4,704.5b]).
                if let Some(&top) = self.zones.libraries[player.index()].front() {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: top,
                            from: Some(Zone::Library),
                            to: Zone::Hand,
                            enters: None,
                            position: None,
                            face: None,
                            cause: None,
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
            GameEvent::Tapped { object, .. } => {
                self.objects.obj_mut(object).tapped = true;
                event
            }
            GameEvent::ManaAdded {
                player,
                mana,
                amount,
                ref riders,
            } => {
                self.player_mut(player)
                    .mana_pool
                    .add_riders(mana, amount, riders);
                event
            }
            GameEvent::ManaEmptied { player, ending } => {
                self.player_mut(player).mana_pool.empty_after(ending);
                event
            }
            GameEvent::TokenCreated { player, ref token } => {
                self.apply_token_created(player, token);
                event
            }
            GameEvent::TokenCeased(id) => {
                self.apply_token_ceased(id);
                event
            }
            GameEvent::PlayerLost { player, .. } => {
                self.player_mut(player).lost = true;
                event
            }
            GameEvent::SpellCast(object) => {
                // [CR#601.2i]: promote the staged announce onto the stack.
                // [CR#405]: a spell's stack identity is its own object id —
                // unchanged from Stage 2, so existing Resolve(spell) keying by
                // `StackEntry.id` still finds it.
                let pending = self.promote_announce();
                debug_assert_eq!(
                    pending.object.object(),
                    object,
                    "SpellCast event matches the staged announce"
                );
                GameEvent::SpellCast(object)
            }
            GameEvent::AbilityActivated { source, ability } => {
                // [CR#602.2a]: promote the staged activation onto the stack
                // under the stack identity minted when the announce opened
                // ([CR#405], `begin_activate`), and count it against
                // "activate only once" limits ([CR#602.5b]).
                let pending = self.promote_announce();
                debug_assert!(
                    matches!(
                        &pending.object,
                        StackObject::Activated { source: s, .. } if *s == source
                    ),
                    // The `ability` index keys the ledger only; the stack object
                    // carries the text. Source match is the only structural check here.
                    "AbilityActivated event matches the staged announce"
                );
                self.activations.bump((source, ability));
                GameEvent::AbilityActivated { source, ability }
            }
            GameEvent::DamageDealt {
                source,
                target,
                amount,
            } => {
                // [CR#120.3]: damage to a player is life loss; to a creature it is
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
                // One view for the lifelink + deathtouch keyword checks below
                // (built after the damage lands; neither keyword depends on it).
                let view = self.layers();
                // [CR#702.15]: if the source is a card-backed object with lifelink,
                // its controller gains life equal to the damage dealt. This applies
                // to combat damage and any other damage from a lifelink source.
                // Guard: use `get` (not `obj`) because a dies-trigger's source id
                // may be a stale (reminted) id that is no longer in the store.
                if self
                    .objects
                    .get(source)
                    .is_some_and(|o| o.card_id().is_some())
                    && crate::combat::has_keyword_named(&view, source, "Lifelink")
                {
                    let controller = self.objects.obj(source).controller;
                    self.player_mut(controller).life +=
                        deckmaste_core::Int::try_from(amount).expect("damage fits in i32");
                }
                // [CR#702.2]: if the source is a card-backed object with deathtouch
                // and the amount dealt is > 0, mark the TARGET as struck by a
                // deathtouch source. The SBA then destroys a creature with
                // toughness > 0 so marked ([CR#704.5h]). Same guard pattern as
                // lifelink above.
                if amount > 0
                    && matches!(
                        self.objects.get(target).map(|o| o.source),
                        Some(ObjectSource::Card(_))
                    )
                    && self
                        .objects
                        .get(source)
                        .is_some_and(|o| o.card_id().is_some())
                    && crate::combat::has_keyword(&view, source, &KeywordAbility::Deathtouch)
                {
                    self.objects.obj_mut(target).struck_by_deathtouch = true;
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
                position,
                face,
                cause,
            } => {
                self.apply_zone_will_change(
                    object,
                    from,
                    to,
                    enters,
                    position,
                    face,
                    cause.clone(),
                );
                GameEvent::ZoneWillChange {
                    object,
                    from,
                    to,
                    enters,
                    position,
                    face,
                    cause,
                }
            }
            // [CR#603.6]: the FACT — the move already happened at the
            // will-change apply. A no-op; triggers (a later task) match here.
            // (Same body as the `TurnBegan`/`StepBegan` no-op, but kept its own
            // arm to carry the CR rationale and the future trigger-match seam.)
            #[expect(clippy::match_same_arms)]
            GameEvent::ZoneChanged { .. } => event,
            // [CR#701.3a,701.3c]: commit the attachment→host relation — a new
            // timestamp is implicit (no remint; the relation edit IS the
            // transition). The verb builder (`Action::Attach`) already filtered
            // the no-ops; this fact is real, so set the link, then record it for
            // "becomes attached / equipped" triggers (breadth is a seam, §9).
            GameEvent::Attached { attachment, host } => {
                self.objects.obj_mut(attachment).attached_to = Some(host);
                GameEvent::Attached { attachment, host }
            }
            // [CR#701.3d]: commit the unattach — clear the link. The verb
            // builder filtered the not-attached no-op, so this fact is real.
            GameEvent::Unattached {
                attachment,
                former_host,
            } => {
                self.objects.obj_mut(attachment).attached_to = None;
                GameEvent::Unattached {
                    attachment,
                    former_host,
                }
            }
            GameEvent::LifeLost { player, amount } => {
                self.player_mut(player).life -=
                    deckmaste_core::Int::try_from(amount).expect("life loss fits in i32");
                GameEvent::LifeLost { player, amount }
            }
            // [CR#119.3]: a player gains life — the life total adjusts up.
            GameEvent::LifeGained { player, amount } => {
                self.player_mut(player).life +=
                    deckmaste_core::Int::try_from(amount).expect("life gain fits in i32");
                GameEvent::LifeGained { player, amount }
            }
            // [CR#508.1a]: record the attacker; [CR#508.1f]: declaring it as an
            // attacker taps it (not a cost — attacking simply taps).
            // [CR#702.20]: a creature with vigilance is NOT tapped when it attacks.
            GameEvent::Attacking(o) => {
                self.combat.declare_attacker(o);
                if !crate::combat::has_keyword(&self.layers(), o, &KeywordAbility::Vigilance)
                    && !self.objects.obj(o).tapped
                {
                    self.objects.obj_mut(o).tapped = true;
                    // The declaration's tap is a real "becomes tapped"
                    // transition ([CR#603.2e]), distinguishable by its cause
                    // ([CR#508.1f] — not a cost): emit the fact in the
                    // declaration's wake so becomes-tapped triggers see it.
                    // Re-applying it is an idempotent flip.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::Tapped {
                            object: o,
                            cause: Some(crate::event::Cause::tap(
                                deckmaste_core::Agency::AttackDeclaration,
                                None,
                            )),
                        },
                    ))]);
                }
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
            // [CR#608.2n]: the triggered or activated ability vanishes —
            // remove its stack entry and discard the minted token. No zone move; the
            // source (already gone for a dies-trigger) is untouched.
            GameEvent::AbilityResolved(id) => {
                self.remove_stack_entry(id);
                self.objects.remove(id);
                GameEvent::AbilityResolved(id)
            }
            // P0.W6 seams: shaped, nothing emits them yet. Revealed's apply
            // will open a reveal window ([CR#701.20a] lifetime);
            // DesignationChanged's will write the W5 registry's game scope;
            // ControlChanged's will re-home the object ([CR#603.2e] delta,
            // never a zone move).
            GameEvent::Revealed { .. } => todo!("P0.W6: reveal apply ([CR#701.20a])"),
            GameEvent::DesignationChanged { .. } => {
                todo!("P0.W6: game-scope designation flip apply ([CR#731.1a])")
            }
            GameEvent::GotDesignation { player, name } => {
                // [CR#702.131c]: set the player-scope flag once; never removed.
                self.designations
                    .players
                    .entry((player, name.clone()))
                    .or_insert(crate::state::DesignationValue::Flag);
                GameEvent::GotDesignation { player, name }
            }
            GameEvent::ControlChanged { .. } => {
                todo!("P0.W6: control-change apply (layers L2 seam)")
            }
            // [CR#701.24a]: randomize so NO player knows the order — the
            // seeded rng (UD-8). Revealed-state reset ([CR#701.20d]) is a
            // P0.W6 seam (no reveal windows exist yet).
            GameEvent::Shuffled(player) => {
                self.zones.libraries[player.index()]
                    .make_contiguous()
                    .shuffle(&mut self.rng);
                event
            }
        }
    }

    /// Applies a `ZoneWillChange` ([CR#400.7]): the move+remint that every zone
    /// change goes through. Captures the live object's LKI, removes it from its
    /// `from` zone, remints a fresh object into `to` (new `ObjectId`, same
    /// `CardId`), applies the permanent's own `AsEnters` self-replacements into
    /// the `EnterStatus` (no observable untapped window), and schedules the
    /// `ZoneChanged` fact at the agenda front. `position` places a card
    /// entering a library at that index from the top, clamped to the bottom
    /// ([CR#401.7]).
    #[expect(
        clippy::too_many_arguments,
        reason = "one parameter per ZoneWillChange coordinate"
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

        // 2. (replace stage — other-object and destination-rewriting replacements are
        //    Stage-4 seams; AsEnters self-replacement applied below at mint.)

        // 3. Move + remint. Remove the old object from its `from` zone's list, then
        //    from the store; mint a fresh object into `to`.
        match from {
            Some(Zone::Stack) => self.remove_stack_entry(object),
            Some(Zone::Battlefield) => {
                // [CR#506.4]: an object that leaves the battlefield is removed
                // from combat. Prune the OLD/leaving id (not a reminted one) from
                // the combat registry immediately, so a creature that dies/leaves
                // mid-combat stops being tracked as an attacker/blocker at once.
                self.combat.remove_object(object);
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
            other => unreachable!(
                "zone-change source {other:?} is not wired \
                 (Stack/Battlefield/Hand/Library/Graveyard only)"
            ),
        }
        self.objects.remove(object);

        let ObjectSource::Card(card) = snapshot.source else {
            unreachable!("only card-backed objects change zones")
        };
        let owner = self.cards.get(card).owner;
        // [CR#110.2,108.4]: a permanent keeps its caster as controller; elsewhere
        // the object is controlled by its owner.
        let controller = if to == Zone::Battlefield { snapshot.controller } else { owner };
        let new = self.objects.mint(snapshot.source, controller, Some(to));
        // [CR#614.12]: how it enters — emitted status (Stage 4 replacements) plus
        // the object's own AsEnters self-replacement (enters tapped / attached).
        let mut entering = enters.unwrap_or_default();
        if to == Zone::Battlefield {
            let as_enters = self.as_enters_status(snapshot.source, new);
            entering.tapped |= as_enters.tapped;
            entering.attach_to = entering.attach_to.or(as_enters.attach_to);
            // [CR#302.6]: a permanent entering the battlefield is summoning-sick
            // until its controller's turn begins with it under continuous control.
            self.objects.obj_mut(new).summoning_sick = true;
        }
        if entering.tapped {
            self.objects.obj_mut(new).tapped = true;
        }
        // [CR#303.4]: enters attached atomically — set the link on the freshly
        // minted object before the `ZoneChanged` fact, so no unattached window
        // is observable. The `Attached` fact is scheduled after the entry fact.
        let attached_host = entering.attach_to.filter(|&host| {
            to == Zone::Battlefield && self.objects.get(host).is_some() && host != new
        });
        if let Some(host) = attached_host {
            self.objects.obj_mut(new).attached_to = Some(host);
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

        // 4. Schedule the unreplaceable fact(s) at the agenda front — the face
        // and cause coordinates ride through from the intent. When the
        // permanent entered attached ([CR#303.4]), the `Attached` fact follows
        // the entry fact (it entered, then became attached) so "becomes
        // attached / equipped" can match it (breadth is a seam, §9).
        let mut facts = vec![WorkItem::Emit(Occurrence::single(GameEvent::ZoneChanged {
            snapshot,
            from,
            to,
            face,
            cause,
        }))];
        if let Some(host) = attached_host {
            facts.push(WorkItem::Emit(Occurrence::single(GameEvent::Attached {
                attachment: new,
                host,
            })));
        }
        self.schedule_front(facts);
    }

    /// Applies a `TokenCreated` ([CR#701.7a]): synthesizes the token's
    /// definition into the card table ([CR#111.2]: `player` is its owner and
    /// it enters under their control), mints the object straight onto the
    /// battlefield (summoning-sick, [CR#302.6]; its own `AsEnters`
    /// self-replacements folded, [CR#614.12]), and schedules the `ZoneChanged
    /// { from: None, to: Battlefield }` fact so enter-triggers fire
    /// ([CR#603.6]). There is no `ZoneWillChange` stage — the token existed
    /// nowhere to move *from*; its snapshot is captured from the freshly
    /// minted object.
    fn apply_token_created(&mut self, player: PlayerId, token: &deckmaste_core::Token) {
        let card = self.cards.push_token(token, player);
        let source = ObjectSource::Card(card);
        let new = self.objects.mint(source, player, Some(Zone::Battlefield));
        self.objects.obj_mut(new).summoning_sick = true;
        if self.as_enters_status(source, new).tapped {
            self.objects.obj_mut(new).tapped = true;
        }
        self.zones.battlefield.push(new);
        let snapshot = crate::lki::LkiSnapshot::capture(self, new);
        self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneChanged {
                snapshot,
                from: None,
                to: Zone::Battlefield,
                face: None,
                cause: None,
            },
        ))]);
    }

    /// Applies a `TokenCeased` ([CR#704.5d,111.7]): removes the token object
    /// from its zone and the store outright. No remint, no `ZoneChanged` —
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
        self.record_history(&occurred);
        self.check_game_end();
        if self.outcome.is_none() {
            self.scan_triggers(&occurred);
        }
        occurred
    }

    /// Appends the substantive facts of `occurred` to the history log, tagged
    /// with the current turn ([CR#608.2i]). Skips the same meta/intent facts
    /// the trigger scan skips (`scan_triggers`): a `TriggerFired` is
    /// bookkeeping, `ZoneWillChange` is the replaceable intent above its
    /// committed `ZoneChanged`, and `StepBegan`/`TurnBegan` are read off
    /// `TurnState` rather than the log.
    fn record_history(&mut self, occurred: &Occurrence) {
        let turn = self.turn.turn_number;
        let events: &[GameEvent] = match occurred {
            Occurrence::Single(e) => std::slice::from_ref(e),
            Occurrence::Batch(es) => es,
        };
        for event in events {
            match event {
                GameEvent::TriggerFired { .. }
                | GameEvent::AbilityResolved(_)
                | GameEvent::StepBegan(_)
                | GameEvent::TurnBegan { .. }
                | GameEvent::ZoneWillChange { .. } => {}
                _ => self.history.record(turn, event.clone()),
            }
        }
    }

    /// [CR#104.2a,104.4a]: last player standing wins; zero remaining is a draw.
    /// Run AFTER an occurrence applies, so a simultaneous multi-loss batch is a
    /// draw, not a win for whoever was checked first.
    ///
    /// P0.W6 seam (no trip point exists yet): the mandatory-loop draw
    /// ([CR#104.4b]) needs a loop MONITOR, and the monitor needs a
    /// game-state equality predicate — UD-11, still OPEN (no rule defines
    /// when two states are "the same"; see docs/engine-adrs.md).
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
        self.activations.reset_turn();
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
        // [CR#510.4]: the FirstCombatDamage step exists only when at least one
        // attacking or blocking creature has first/double strike. When none
        // does, elide it entirely — no StepBegan, no turn-based action, no
        // priority window — and schedule the regular CombatDamage step directly.
        // (`turn.current` is NOT advanced; the step never owns a turn.)
        if s == Phase::Combat(CombatStep::FirstCombatDamage)
            && !crate::combat::any_first_or_double_striker(self)
        {
            self.schedule_front(vec![WorkItem::BeginStep(Phase::Combat(
                CombatStep::CombatDamage,
            ))]);
            return Progress::Skipped(s);
        }
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
            // [CR#502.3]: the active player's tapped permanents untap.
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
            // [CR#510.1,510.4]: assign + deal combat damage — but only when
            // something is attacking. With no attackers there is no damage to
            // assign ([CR#508.8] already skipped blockers); skip the step's work
            // like the empty Declare Blockers case. Both combat-damage steps
            // surface the same turn-based action; `assign_combat_damage` filters
            // sources by which step is current (first/double strikers in the
            // first step, normal + double strikers in the regular one).
            Phase::Combat(CombatStep::FirstCombatDamage | CombatStep::CombatDamage)
                if !self.combat.attackers().is_empty() =>
            {
                vec![WorkItem::AssignCombatDamage]
            }
            Phase::Combat(CombatStep::FirstCombatDamage | CombatStep::CombatDamage) => vec![],
            // [CR#511.1]: the End of Combat step has NO turn-based actions — the
            // removal-from-combat ([CR#511.3]) happens as the step *ends*, after
            // its priority window, scheduled from `end_of_step_items` (so an
            // "at end of combat" trigger resolving during that priority can still
            // read combat state).
            // [CR#514.1]: discard to hand size — checked after StepBegan.
            // [CR#514.2]: marked damage is removed from all permanents;
            // "until end of turn" continuous effects expire ([CR#514.2]).
            Phase::Ending(EndingStep::Cleanup) => {
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
            let obj = self.objects.obj_mut(id);
            obj.damage = 0;
            obj.struck_by_deathtouch = false; // [CR#514.2]
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

    /// [CR#500.5]: pools empty at the end of every step; then the next step
    /// begins (wrapping into the next turn's untap).
    pub(crate) fn end_of_step_items(&self) -> Vec<WorkItem> {
        let mut items: Vec<WorkItem> = self
            .players
            .iter()
            // Lost players have left the game; nothing of theirs empties.
            .filter(|p| !p.lost && !p.mana_pool.is_empty())
            .map(|p| {
                WorkItem::Emit(Occurrence::single(GameEvent::ManaEmptied {
                    player: p.id,
                    ending: self.turn.current,
                }))
            })
            .collect();
        // [CR#511.3]: removal from combat happens as the End of Combat step ends
        // — after its priority window, before the next step begins.
        if self.turn.current == Phase::Combat(CombatStep::EndOfCombat) {
            items.push(WorkItem::EndOfCombat);
        }
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

    /// [CR#701.9b]: a resolving discard surfaces its card choice when the work
    /// item applies — the hand may have changed since the discard was
    /// scheduled. An instruction to discard more cards than the hand holds
    /// discards the whole hand (the excess is impossible and ignored,
    /// [CR#101.3]); an empty hand (count 0) surfaces nothing.
    fn open_discard_cards(&mut self, player: PlayerId, count: Uint) -> Progress {
        let hand =
            Uint::try_from(self.zones.hands[player.index()].len()).expect("hand size fits in Uint");
        let count = count.min(hand);
        if count > 0 {
            self.pending = Some(PendingDecision::DiscardCards { player, count });
        }
        Progress::DiscardOpened { count }
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
    ) -> Progress {
        debug_assert!(
            !options.is_empty(),
            "a mana choice offers at least one option"
        );
        self.pending = Some(PendingDecision::ChooseManaColor {
            player,
            options,
            amount,
            riders,
        });
        Progress::ManaColorOpened
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

        // One derived view serves every keyword/power read in this pass.
        let view = self.layers();

        // [CR#510.4]: which sources deal this step is a pure keyword filter on
        // the current combat-damage step (no "already dealt" bookkeeping). In
        // the FIRST step only first/double strikers deal; in the REGULAR step
        // everyone EXCEPT a plain first-striker deals (a double-striker deals in
        // both). The regular filter includes everyone when no first strike
        // exists, so a single-pass combat is unchanged.
        let deals_this_step: fn(&crate::layer::LayeredView, ObjectId) -> bool =
            if self.turn.current == Phase::Combat(CombatStep::FirstCombatDamage) {
                crate::combat::deals_first_strike
            } else {
                crate::combat::deals_regular_strike
            };

        // Each attacker is a source. Recipients: unblocked → the defending
        // player's proxy ([CR#510.1b]); blocked → its live blockers
        // ([CR#510.1c]); blocked-but-no-live-blockers → nothing (plain block,
        // no trample). Trample ([CR#702.19]) widens the blocked cases: a blocked
        // trampler's recipients are its live blockers followed by the defending
        // player's proxy ([CR#702.19b]), and with no live blockers all of its
        // damage goes to the player ([CR#702.19d]).
        let defender_proxy = self
            .player(self.next_live_after(self.turn.active_player))
            .object;
        for &attacker in self.combat.attackers() {
            if !deals_this_step(&view, attacker) {
                continue; // [CR#510.4]: not dealing in this step.
            }
            let recipients: Vec<ObjectId> = if self.combat.is_blocked(attacker) {
                let mut blockers = self.combat.blockers_of(attacker).to_vec();
                if crate::combat::has_keyword(&view, attacker, &KeywordAbility::Trample) {
                    // [CR#702.19b]: lethal to the blockers, excess to the player;
                    // [CR#702.19d]: no live blockers → everything to the player.
                    blockers.push(defender_proxy);
                }
                blockers
            } else {
                vec![defender_proxy]
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

    /// [CR#511.3]: removal from combat, run as the End of Combat step *ends*
    /// (scheduled from `end_of_step_items` after the step's priority window,
    /// not as a turn-based action — [CR#511.1] gives this step none).
    /// Clears the registry so a later combat phase (and any SBA/other
    /// reader) never sees stale (reminted/dead) attacker/blocker
    /// designations.
    fn end_of_combat(&mut self) -> Progress {
        self.combat.clear();
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

    /// A creature's combat-damage output: its derived power as a non-negative
    /// number ([CR#510.1c]). `None`/negative power assigns 0.
    #[must_use]
    fn power_of(view: &crate::layer::LayeredView, id: ObjectId) -> Uint {
        match view.power(id) {
            Some(p) if p > 0 => {
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

    /// [CR#601.2i,602.2a]: the becomes-cast moment — take the single in-flight
    /// announce and commit it to the stack as a `StackEntry`, carrying
    /// id / object / controller / targets / x across unchanged ([CR#405]: the
    /// stack identity was minted at announce). Shared by the `SpellCast` and
    /// `AbilityActivated` apply arms, which differ only in their own
    /// debug-assert and (for activations) the "activate only once" bump — so
    /// the moved-out `PendingStackEntry` is returned for the caller's check.
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight — an engine invariant (these events
    /// are only emitted while one is staged), not caller input.
    fn promote_announce(&mut self) -> crate::stack::PendingStackEntry {
        let pending = self.announcing.take().expect("an announce in flight");
        self.stack.push(StackEntry {
            id: pending.id,
            object: pending.object.clone(),
            controller: pending.controller,
            targets: pending.targets.clone(),
            x: pending.x,
        });
        pending
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Phase;
    use deckmaste_core::Window;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
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
        state.record_history(&Occurrence::single(GameEvent::Untapped(id)));
        assert_eq!(
            state
                .history
                .scan(Window::ThisGame, state.turn.turn_number)
                .count(),
            1
        );

        // A skipped (meta) fact is not.
        state.record_history(&Occurrence::single(GameEvent::StepBegan(
            Phase::PrecombatMain,
        )));
        assert_eq!(
            state
                .history
                .scan(Window::ThisGame, state.turn.turn_number)
                .count(),
            1,
            "StepBegan is skipped"
        );
    }

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
                WorkItem::AnnounceX,
                WorkItem::AnnounceTargets,
                WorkItem::ChooseCostOptions,
                WorkItem::PayCost,
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
            id,
            object: StackObject::Spell(id),
            controller: PlayerId(0),
            origin: Zone::Hand,
            targets: vec![],
            x: Some(3),
            concretized: None,
        });

        let pending = state.promote_announce();

        assert!(state.announcing.is_none(), "the announce slot is cleared");
        assert_eq!(state.stack.len(), 1, "one entry committed to the stack");
        let top = state.stack.last().expect("the promoted entry");
        assert_eq!(top.id, id);
        assert_eq!(top.object, StackObject::Spell(id));
        assert_eq!(top.controller, PlayerId(0));
        assert_eq!(top.targets, Vec::<crate::object::ObjectId>::new());
        assert_eq!(top.x, Some(3));
        // The returned pending lets callers run their own debug-asserts.
        assert_eq!(pending.id, id);
    }

    /// Applying `GotDesignation` writes the player-scope designation store, and a
    /// second apply is a no-op ([CR#702.131c] — set once; any number of players may
    /// hold it, none loses it).
    #[test]
    fn got_designation_applies_idempotently() {
        let mut state = game();
        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = crate::player::PlayerId(0);

        state.schedule_front(vec![WorkItem::Emit(Occurrence::Single(
            GameEvent::GotDesignation {
                player: p0,
                name: name.clone(),
            },
        ))]);
        let _ = state.step();
        assert!(
            state.designations.players.contains_key(&(p0, name.clone())),
            "player 0 now holds the city's blessing"
        );

        // A second apply does not panic and leaves the entry present.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Single(
            GameEvent::GotDesignation {
                player: p0,
                name: name.clone(),
            },
        ))]);
        let _ = state.step();
        assert!(state.designations.players.contains_key(&(p0, name)));
    }
}
