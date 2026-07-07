//! The steppable core: `step()` pops one agenda item and returns one
//! `Progress`. Decisions surface on the following call; the runner loops.

use deckmaste_core::BeginningStep;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CombatStep;
use deckmaste_core::EndingStep;
use deckmaste_core::KeywordAbility;
use deckmaste_core::PhaseStep;
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
#[expect(
    clippy::large_enum_variant,
    reason = "per-step return value, produced and matched immediately; boxing `Progress` would add allocation churn to the hot step() loop for no gain"
)]
pub enum StepOutcome {
    /// One unit of work happened.
    Progress(Progress),
    /// No mutation; `submit_decision` to proceed.
    NeedsDecision(PendingDecision),
    GameOver(GameOutcome),
}

/// One unit of engine work, observed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::large_enum_variant,
    reason = "the `Applied(Occurrence)` payload dominates but is the common case; boxing it would allocate on every progressing step"
)]
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
    /// A `Noting` collection window opened (`true`) or closed (`false`)
    /// ([CR#607.2a] — fact-backed product groups).
    NoteScoped { open: bool },
    /// [CR#701.22a]: a `Distribute` decision was surfaced (or skipped for an
    /// empty window — scry/surveil 0 no-op per [CR#701.22b]).
    DistributeOpened,
    /// [CR#401.7]: a card was repositioned within its own library (no zone
    /// change — `ObjectId` preserved).
    Repositioned(crate::object::ObjectId),
    /// [CR#401.4]: the post-pick arrange finalizer ran; `deciding` is how many
    /// piles of more than one card still need an arrange decision (0 = every
    /// pile was ≤1 card, nothing surfaced).
    PilesArranged { deciding: Uint },
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
                self.pending = Some(PendingDecision::PayMana {
                    player,
                    cost,
                    pool: self.player(player).mana_pool.clone(),
                    subject,
                });
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
            WorkItem::BeginNote { key } => {
                // [CR#607.2a]: a fresh window — the key holds THIS noting
                // run's product, never an earlier clause's leftovers.
                self.noted.insert(key, Vec::new());
                self.noting.push(key);
                Progress::NoteScoped { open: true }
            }
            WorkItem::EndNote => {
                self.noting
                    .pop()
                    .expect("EndNote pairs with a BeginNote (scheduled together)");
                Progress::NoteScoped { open: false }
            }
            WorkItem::OpenDistribute {
                player,
                window,
                bins,
                name,
            } => self.open_distribute(player, window, bins, name),
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
        };
        StepOutcome::Progress(progress)
    }

    /// Applies one event: the skeleton's whole pipeline (cant and
    /// replacement registries are empty). Returns the event as it occurred —
    /// apply-time bindings (a drawn card's identity) filled in, and a draw
    /// from an empty library occurring as `DrewFromEmpty` instead.
    #[expect(
        clippy::too_many_lines,
        reason = "decomposition by subsystem (stack / zone-change / player) \
                  tracked in refactor-oversized-fns"
    )]
    fn apply(&mut self, event: GameEvent) -> GameEvent {
        // Every amount-carrying event fixes the "that much" register
        // (`Count::ThatMuch`) as it actually happens — at the apply funnel,
        // after any replacement has rewritten the event. Cause-amount zone
        // changes (a discard, a mill) are fixed per OCCURRENCE instead
        // (`apply_occurrence` counts the batch — "discard two cards, then
        // draw that many" reads the batch size, not 1).
        match &event {
            GameEvent::DamageDealt { amount, .. }
            | GameEvent::LifeLost { amount, .. }
            | GameEvent::LifeGained { amount, .. }
            | GameEvent::CounterPlaced { amount, .. }
            | GameEvent::CounterRemoved { amount, .. } => self.that_much = Some(*amount),
            GameEvent::WillDraw { .. } => self.that_much = Some(1),
            _ => {}
        }
        #[expect(
            clippy::match_same_arms,
            reason = "large apply dispatch; the pure-fact arms and the `Distributed` arm both return `event` unchanged but sit hundreds of lines apart with distinct explanatory comments — merging would wreck the structure"
        )]
        match event {
            // Pure facts: nothing to mutate. `BecameTarget` ([CR#601.2c])
            // exists for the trigger scan (ward, [CR#702.21a]); the
            // targeting state itself lives in the announce slot / stack
            // entry. `AbilityUsed` is bookkeeping recorded directly via
            // `history.record` at the trigger-fire/activation apply sites
            // ([CR#608.2i]); it never enters the occurrence pipeline, so its
            // apply is inert here (defensive only).
            GameEvent::TurnBegan { .. }
            | GameEvent::StepBegan(_)
            | GameEvent::BecameTarget { .. }
            | GameEvent::AbilityUsed { .. } => event,
            // P0.W3 seam: grammar-complete events nothing emits yet — their
            // apply (RNG) is unbuilt.
            GameEvent::CoinFlipped { .. } | GameEvent::DieRolled { .. } => {
                todo!("P0.W3: random-event apply")
            }
            // [CR#122.1]: counters live in the object's (or player proxy's)
            // counter map. Placement sums by kind; removal saturates at zero
            // and DROPS the key, so an absent kind reads as zero everywhere
            // (the layer-7c P/T read, `HasCounter`). The occurred fact
            // carries the carrier's before/after TOTALS, apply-computed —
            // the [CR#714.2b] `Crossed` reads run off the fact, never a
            // post-hoc map read.
            GameEvent::CounterPlaced {
                object,
                kind,
                amount,
                cause,
                ..
            } => {
                let entry = self
                    .objects
                    .obj_mut(object)
                    .counters
                    .entry(kind)
                    .or_insert(0);
                let before = *entry;
                *entry += amount;
                let after = *entry;
                GameEvent::CounterPlaced {
                    object,
                    kind,
                    amount,
                    before,
                    after,
                    cause,
                }
            }
            GameEvent::CounterRemoved {
                object,
                ref kind,
                amount,
                ..
            } => {
                let counters = &mut self.objects.obj_mut(object).counters;
                if let Some(have) = counters.get_mut(kind) {
                    *have = have.saturating_sub(amount);
                    if *have == 0 {
                        counters.remove(kind);
                    }
                }
                event
            }
            GameEvent::Untapped(id) => {
                self.objects.obj_mut(id).tapped = false;
                event
            }
            GameEvent::WillDestroy { object, cause } => {
                // [CR#701.8a]: the destruction intent commits and evolves into
                // the committed Battlefield→Graveyard move, carrying the destroy
                // cause ([CR#701.8b]) for the "destroyed" view. Indestructible
                // (and other cant-happen statics) are handled upstream in
                // `apply_occurrence` via the cant pass ([CR#614.17]) — the
                // `WillDestroy` event never reaches `apply` for such objects.
                self.schedule_evolution(GameEvent::ZoneWillChange {
                    object,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard,
                    enters: None,
                    position: None,
                    face: None,
                    cause: cause.clone(),
                });
                GameEvent::WillDestroy { object, cause }
            }
            GameEvent::WillDraw { player, source } => {
                // [CR#121.1]: the draw intent commits. A card present → evolve
                // into the generic Library→Hand move (remint + LKI); the
                // returned `WillDraw` fact is what `CardsDrawnThisTurn` counts
                // in the history log. An empty library → DrewFromEmpty, the
                // failed-draw fact the loss SBA keys on ([CR#121.4,704.5b]).
                if let Some(&top) = self.zones.libraries[player.index()].front() {
                    self.schedule_evolution(GameEvent::ZoneWillChange {
                        object: top,
                        from: Some(Zone::Library),
                        to: Zone::Hand,
                        enters: None,
                        position: None,
                        face: None,
                        cause: None,
                    });
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
                // ([CR#405], `begin_activate`).
                let pending = self.promote_announce();
                debug_assert!(
                    matches!(
                        &pending.object,
                        StackObject::Activated { source: s, .. } if *s == source
                    ),
                    "AbilityActivated event matches the staged announce"
                );
                // Record the substantive "this ability was used" fact directly
                // so history reads (use-limit counts, EventCount) can find it.
                // Not routed through the occurrence pipeline — must not trigger
                // anything and must not be re-recorded ([CR#608.2i]).
                //
                // Use the announce-time LKI snapshot rather than the live
                // object — the source may have been removed (e.g. self-sacrifice
                // cost) before this event applies ([CR#602.2a]).
                // `begin_activate` always captures `bindings.this`, so the
                // expect below should never fire in practice.
                let used_object = match &pending.object {
                    StackObject::Activated { bindings, .. } => {
                        bindings
                            .this
                            .as_ref()
                            .expect("begin_activate always captures a this snapshot")
                            .object
                    }
                    _ => panic!("AbilityActivated with non-Activated stack object"),
                };
                self.record_history_fact(
                    self.turn.turn_number,
                    None,
                    GameEvent::AbilityUsed {
                        object: used_object,
                        ability: Uint::try_from(ability).expect("ability index fits in Uint"),
                    },
                );
                GameEvent::AbilityActivated { source, ability }
            }
            GameEvent::DamageDealt {
                source,
                target,
                amount,
                combat,
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
                    combat,
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
                    enters.clone(),
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
                // [CR#603.2h]: a "once each turn" / once-per-game triggered
                // ability is noted at most that often. The gate lives HERE, at
                // note time — NOT at scan-emit — because a single
                // multi-occurrence event ([CR#603.2c], e.g. two creatures
                // dying simultaneously) emits both `TriggerFired`s in one scan
                // pass before either applies; only at sequential apply-time
                // does the second see the first's recorded `AbilityUsed`. The
                // limit is per firing object ([CR#400.7]).
                let obj = bindings.this.as_ref().map(|t| t.object);
                // Collect the firing ability's limits into an owned vec BEFORE
                // the `pending_triggers`/`history` mutations below (the
                // `abilities_of_source` borrow must not overlap them).
                let limits: Vec<deckmaste_core::UseLimit> =
                    match crate::derive::abilities_of_source(self, source).get(ability as usize) {
                        Some(deckmaste_core::Ability::Triggered(t)) => t.limits.clone(),
                        _ => Vec::new(),
                    };
                if let Some(obj) = obj {
                    for limit in &limits {
                        let window = match limit {
                            deckmaste_core::UseLimit::OncePerTurn => {
                                deckmaste_core::Lookback::ThisTurn
                            }
                            deckmaste_core::UseLimit::OncePerGame => {
                                deckmaste_core::Lookback::ThisGame
                            }
                        };
                        if self.ability_used_count(obj, ability, window) >= 1 {
                            // The limit is spent: the trigger does NOT fire —
                            // note nothing, record nothing.
                            return event;
                        }
                    }
                }
                self.pending_triggers.push(crate::trigger::NotedTrigger {
                    source,
                    ability: ability as usize,
                    controller,
                    bindings: bindings.clone(),
                });
                // Record the substantive "this ability was used" fact directly
                // so history reads (use-limit counts, EventCount) can find it.
                // Not routed through the occurrence pipeline — must not trigger
                // anything and must not be re-recorded ([CR#608.2i]).
                if let Some(this) = &bindings.this {
                    let used = GameEvent::AbilityUsed {
                        object: this.object,
                        ability,
                    };
                    self.record_history_fact(self.turn.turn_number, None, used);
                }
                event
            }
            // [CR#608.2n]: the triggered or activated ability vanishes —
            // remove its stack entry and discard the minted token. No zone move; the
            // source (already gone for a dies-trigger) is untouched.
            GameEvent::AbilityCountered { id, .. } => {
                self.remove_stack_entry(id);
                self.objects.remove(id);
                event
            }
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
            // Notification only — no state mutation. The event fires once the
            // distribute has completed (top/bottom repositioned, graveyard items
            // scheduled), so triggers can observe it ([CR#701.22d]).
            GameEvent::Distributed { .. } => event,
            // Notification only — no state mutation. Fired once the keyword
            // action's body has completed ([CR#701.22d]), so triggers observe it.
            GameEvent::KeywordActionPerformed { .. } => event,
            GameEvent::DesignationChanged { .. } => {
                todo!("P0.W6: game-scope designation flip apply ([CR#731.1a])")
            }
            GameEvent::GotDesignation { player, name } => {
                // [CR#702.131c]: set the player-scope flag once; never removed.
                self.designations
                    .players
                    .entry((player, name))
                    .or_insert(crate::state::DesignationValue::Flag);
                GameEvent::GotDesignation { player, name }
            }
            // [CR#701.12b,613.1b]: a one-shot control TRANSITION — re-home
            // the object (a control change is never a zone move; the object
            // keeps its identity). The base controller moves; layer-2
            // continuous control effects still override on top. The new
            // controller has not controlled it continuously since their
            // last turn began, so it is summoning-sick for them
            // ([CR#302.6]).
            GameEvent::ControlChanged { object, to } => {
                if self.objects.get(object).is_some() {
                    self.objects.obj_mut(object).controller = to;
                    self.objects.obj_mut(object).summoning_sick = true;
                }
                event
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
            // [CR#614.8,701.19a]: the regeneration heal clause — zero damage
            // and remove from combat. If the object is already at 0 damage
            // this is still a no-op (removal from combat is still correct —
            // the shield fired, so the permanent would have been in combat
            // when the destroy was imminent). `remove_object` is idempotent
            // for non-combat objects.
            GameEvent::DamageRemoved { object } => {
                if self.objects.get(object).is_some() {
                    self.objects.obj_mut(object).damage = 0;
                }
                self.combat.remove_object(object);
                GameEvent::DamageRemoved { object }
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
            entering.counters.extend(as_enters.counters);
            // [CR#302.6]: a permanent entering the battlefield is summoning-sick
            // until its controller's turn begins with it under continuous control.
            self.objects.obj_mut(new).summoning_sick = true;
        }
        if entering.tapped {
            self.objects.obj_mut(new).tapped = true;
        }
        // [CR#122.6a,614.1c]: place enters-with counters atomically at mint,
        // before the `ZoneChanged` fact — the entering P/T already reflects
        // them, and no counterless window is observable.
        for (kind, n) in &entering.counters {
            *self.objects.obj_mut(new).counters.entry(*kind).or_insert(0) += n;
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
        // and cause coordinates ride through from the intent. Inside a batch
        // apply the fact joins the shared evolution collector instead, so a
        // simultaneous move-batch commits as ONE `ZoneChanged` occurrence
        // ([CR#603.3b,603.2c]). When the permanent entered attached
        // ([CR#303.4]), the `Attached` fact follows the entry fact (it
        // entered, then became attached) so "becomes attached / equipped"
        // can match it (breadth is a seam, §9); it stays its own occurrence
        // — the flushed evolution batch is front-scheduled after the member
        // loop, landing AHEAD of it.
        self.schedule_evolution(GameEvent::ZoneChanged {
            snapshot,
            from,
            to,
            face,
            cause,
        });
        if let Some(host) = attached_host {
            self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                GameEvent::Attached {
                    attachment: new,
                    host,
                },
            ))]);
        }
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
        // The entry fact joins the batch collector when N tokens are minted
        // as one simultaneous instruction ([CR#701.7a] — "one instruction,
        // simultaneous"): their enter-triggers see ONE occurrence.
        self.schedule_evolution(GameEvent::ZoneChanged {
            snapshot,
            from: None,
            to: Zone::Battlefield,
            face: None,
            cause: None,
        });
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

    /// Routes an intent's evolution product (`WillDestroy` →
    /// `ZoneWillChange` → `ZoneChanged`, a draw's move, a token's entry
    /// fact): inside a `Batch` apply it joins the shared evolution
    /// collector — the whole batch's products commit later as ONE follow-on
    /// occurrence ([CR#603.3b,603.2c] — a simultaneous set stays one
    /// occurrence through every stage) — and outside one it front-schedules
    /// the familiar `Single`.
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
    /// is evaluated as a whole.
    fn apply_occurrence(&mut self, occ: Occurrence) -> Occurrence {
        // Preserve whether the input was a Single or Batch — callers and tests
        // observe the shape ([CR#616.1]: a batch is a simultaneous set).
        let occurred = match occ {
            Occurrence::Single(e) => {
                // [CR#614.17]: can't-happen pass — suppressed before replacements.
                if crate::replace_registry::cant_event(self, &e) {
                    Occurrence::Batch(vec![]) // suppressed, nothing occurred
                } else {
                    // [CR#616.1]: replacement-effect loop.
                    match crate::replace_registry::replace_event(self, e) {
                        crate::replace_registry::ReplaceOutcome::Pass(e2) => {
                            Occurrence::Single(self.apply(e2))
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
                // follow-on occurrence ([CR#603.3b]).
                let live: Vec<GameEvent> = events
                    .into_iter()
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
                            facts.push(self.apply(e2));
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
                            }
                            return partial;
                        }
                    }
                }
                self.flush_evolving_batch();
                Occurrence::Batch(facts)
            }
        };
        self.record_history(&occurred);
        self.fix_occurrence_amount(&occurred);
        self.check_game_end();
        if self.outcome.is_none() {
            self.scan_triggers(&occurred);
        }
        occurred
    }

    /// Front-schedules the batch-evolution collector's contents as ONE
    /// occurrence ([CR#603.3b]) and deactivates it. A single-product batch
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

    /// The occurrence-level "that much" fix for cause-amount zone changes
    /// ([CR#107.3] magnitude anaphora): a discard/mill clause's amount is
    /// its CARD COUNT — "discards all the cards in their hand, then draws
    /// that many" reads the batch size, so the per-fact funnel (which would
    /// leave 1) defers to this count. Which cause verbs carry an amount is
    /// the emitted entailment table's `amount` column.
    fn fix_occurrence_amount(&mut self, occurred: &Occurrence) {
        let events: &[GameEvent] = match occurred {
            Occurrence::Single(e) => std::slice::from_ref(e),
            Occurrence::Batch(es) => es,
        };
        let moved = events
            .iter()
            .filter(|e| match e {
                GameEvent::ZoneChanged { cause: Some(c), .. } => {
                    crate::entail::entailment(c.verb.as_str()).is_some_and(|row| row.amount)
                }
                _ => false,
            })
            .count();
        if moved > 0 {
            self.that_much = Some(Uint::try_from(moved).expect("batch size fits in Uint"));
        }
    }

    /// Appends the substantive facts of `occurred` to the history log, tagged
    /// with the current turn ([CR#608.2i]). Skips the meta/intent facts:
    /// a `TriggerFired` is bookkeeping, `ZoneWillChange` is the replaceable
    /// intent above its committed `ZoneChanged`, and `TurnBegan` is read
    /// off `TurnState`. `StepBegan` IS recorded (with the active player on
    /// its view) so history reads see step onsets ([CR#603.2b] — the
    /// StepBegins-in-history lift; also the future sub-turn window
    /// markers). A `Batch`'s members share one fresh batch id ([CR#603.3b]
    /// — they were ONE occurrence); a `Single` records `None`. Every
    /// recorded `ZoneChanged` also feeds the open `Noting` collections
    /// ([CR#607.2a] — fact-backed product groups: the group is what the
    /// clause ACTUALLY moved, never its gathered input set).
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
                GameEvent::TriggerFired { .. }
                | GameEvent::AbilityResolved(_)
                | GameEvent::TurnBegan { .. }
                | GameEvent::ZoneWillChange { .. } => {}
                _ => {
                    self.note_enacted(event);
                    self.record_history_fact(turn, batch, event.clone());
                }
            }
        }
    }

    /// Records one fact to the history log with its per-fact LKI view
    /// ([CR#603.10a]): live card participants are snapshotted NOW, so later
    /// history matching reads them as they were — never the live store
    /// through a stale id.
    pub(crate) fn record_history_fact(
        &mut self,
        turn: Uint,
        batch: Option<Uint>,
        event: GameEvent,
    ) {
        // A pre-evolution intent is shadowed by its downstream fact
        // ([CR#603.6]) — recording a view for BOTH would double-count every
        // `Happened`/`EventCount` zone-move read.
        let view = if crate::eval::shadowed_by_fact(&event) {
            None
        } else {
            crate::eval::FactView::of(self, &event).map(|v| v.into_lki(self))
        };
        self.history.record(turn, batch, event, view);
    }

    /// Feeds one enacted fact to every OPEN `Noting` collection
    /// ([CR#607.2a]): a `ZoneChanged` fact contributes its moved object —
    /// the fact's snapshot plus the post-move (reminted, [CR#400.7])
    /// identity when the object still exists. Suppressed and
    /// replaced-to-nothing members never get here, so an indestructible
    /// survivor of a destroy-all is excluded from "destroyed this way" BY
    /// CONSTRUCTION.
    fn note_enacted(&mut self, event: &GameEvent) {
        if self.noting.is_empty() {
            return;
        }
        let GameEvent::ZoneChanged { snapshot, .. } = event else {
            return;
        };
        // The post-move object: the freshly minted id with the same backing
        // source (a card backs at most one live object).
        let now = self
            .objects
            .iter()
            .find(|o| o.source == snapshot.source)
            .map(|o| o.id);
        for key in &self.noting {
            self.noted
                .entry(*key)
                .or_default()
                .push(crate::state::NotedMember {
                    snapshot: snapshot.clone(),
                    now,
                });
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
    fn begin_step(&mut self, s: PhaseStep) -> Progress {
        // [CR#510.4]: the FirstCombatDamage step exists only when at least one
        // attacking or blocking creature has first/double strike. When none
        // does, elide it entirely — no StepBegan, no turn-based action, no
        // priority window — and schedule the regular CombatDamage step directly.
        // (`turn.current` is NOT advanced; the step never owns a turn.)
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
    fn turn_based_actions(&mut self, s: PhaseStep) -> Vec<WorkItem> {
        match s {
            // [CR#502.3]: the active player's tapped permanents untap.
            PhaseStep::Beginning(BeginningStep::Untap) => {
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
            PhaseStep::Beginning(BeginningStep::Draw) if self.turn.turn_number > 1 => {
                vec![WorkItem::Emit(Occurrence::single(GameEvent::WillDraw {
                    player: self.turn.active_player,
                    source: None,
                }))]
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
            let obj = self.objects.obj_mut(id);
            obj.damage = 0;
            obj.struck_by_deathtouch = false; // [CR#514.2]
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
                WorkItem::Emit(Occurrence::single(GameEvent::ManaEmptied {
                    player: p.id,
                    ending: self.turn.current,
                }))
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
        // schedule follow-on work items at the front (e.g. `Emit(ZoneWillChange)`
        // from a `WillDestroy.apply`). If we re-check immediately after, the
        // follow-ons haven't run yet so the board looks unchanged — a destructible
        // creature with lethal damage hasn't moved yet and the re-check re-emits
        // a WillDestroy, looping. By inserting the re-check AFTER the follow-on
        // slots the re-check runs once the ZoneWillChange and ZoneChanged facts
        // have settled (and the creature is gone), so the next sweep is clean.
        let n_before = self.agenda.len();
        let applied = self.apply_occurrence(Occurrence::Batch(events));
        let n_after = self.agenda.len();
        let changed = !matches!(&applied, Occurrence::Batch(facts) if facts.is_empty());
        if changed {
            // `n_after - n_before` items were prepended by apply_occurrence (the
            // follow-on Emit(ZoneWillChange) etc). Insert the re-check right
            // behind them so they settle before the next sweep.
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
            self.pending = Some(PendingDecision::LegendRule { player, candidates });
            return Progress::SbasChecked { actions: 0 };
        }
        let actions = sba::sweep(self);
        let count = Uint::try_from(actions.len()).expect("action count fits in Uint");
        if count > 0 {
            // Re-check is conditional on this batch actually changing state.
            self.schedule_front(vec![WorkItem::EmitSbaBatch(actions)]);
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
        let hand =
            Uint::try_from(self.zones.hands[active.index()].len()).expect("hand size fits in Uint");
        // [CR#402.2]: discard to the EFFECTIVE maximum hand size — `None` means
        // no maximum (Reliquary Tower), so nothing is discarded.
        let discarding = match self.effective_max_hand_size(active) {
            Some(max) => hand.saturating_sub(max),
            None => 0,
        };
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
    /// [CR#701.22a]: surfaces a `Distribute` decision so the player can sort the
    /// looked-at `window` into ordered `bins`. `name` (e.g. "Scry") is stashed
    /// for the Task-8 event via [`ChoiceContinuation::Distribute`].
    fn open_distribute(
        &mut self,
        player: PlayerId,
        window: Vec<ObjectId>,
        bins: Vec<deckmaste_core::Bin>,
        name: deckmaste_core::Ident,
    ) -> Progress {
        for &object in &window {
            self.look_grants.insert((player, object));
        }
        self.pending = Some(PendingDecision::Distribute {
            player,
            window,
            bins,
        });
        self.choice = Some(crate::state::ChoiceContinuation::Distribute { name });
        Progress::DistributeOpened
    }

    /// [CR#401.7]: reposition a card ALREADY in its owner's library to `end` of
    /// that library, `offset` cards in — direct `VecDeque` surgery keeping the
    /// `ObjectId` (no remint, no `ZoneChanged`, no zone-change trigger; scry
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
    /// [`ChoiceContinuation::ArrangePiles`]. Piles of ≤1 card need no order and
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
            if let Some(pile) = piles.iter_mut().find(|p| {
                p.library_owner == landing.library_owner && p.end == landing.end
            }) {
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
            LibraryEnd::Bottom => lib.iter().skip(lib.len().saturating_sub(count)).copied().collect(),
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
                let mut order = pile.objects.clone();
                use rand::seq::SliceRandom;
                order.shuffle(&mut self.rng);
                self.apply_arranged(&pile, &order);
                Progress::PilesArranged { deciding: 0 }
            }
            deckmaste_core::Arrangement::SameOrder => Progress::PilesArranged { deciding: 0 },
        }
    }

    /// Surface the next pending pile's arrange decision (or nothing when the
    /// walk is done), stashing the walk state in
    /// [`ChoiceContinuation::ArrangePiles`]. `arranger` is the ordering player.
    pub(crate) fn open_next_arrange(
        &mut self,
        arranger: PlayerId,
        mut piles: Vec<crate::state::ArrangePile>,
    ) {
        if piles.is_empty() {
            return;
        }
        let current = piles.remove(0);
        self.pending = Some(PendingDecision::ArrangePile {
            player: arranger,
            objects: current.objects.clone(),
        });
        self.choice = Some(crate::state::ChoiceContinuation::ArrangePiles {
            current,
            remaining: piles,
        });
    }

    /// [CR#401.4]: apply an `Arranged` answer — reorder the pile's cards within
    /// their library (removed then re-inserted contiguously at their end, top →
    /// down in the chosen order) keeping every `ObjectId`.
    pub(crate) fn apply_arranged(
        &mut self,
        pile: &crate::state::ArrangePile,
        order: &[ObjectId],
    ) {
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
            if self.turn.current == PhaseStep::Combat(CombatStep::FirstCombatDamage) {
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
                // The combat-damage step's assignment ([CR#510.1]).
                combat: true,
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
    pub(crate) fn power_of(view: &crate::layer::LayeredView, id: ObjectId) -> Uint {
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
            object: pending.object.clone(),
            controller: pending.controller,
            targets: pending.targets.clone(),
            x: pending.x,
            // [CR#601.2b,702.33d]: the announced optional-cost record rides
            // the committed entry for the linked reads.
            paid_costs: pending.paid_costs.clone(),
        });
        pending
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Lookback;
    use deckmaste_core::PhaseStep;
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
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
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
        state.record_history(&Occurrence::single(GameEvent::TurnBegan {
            player: PlayerId(0),
            turn: 1,
        }));
        assert_eq!(
            state
                .history
                .scan(Lookback::ThisGame, state.turn.turn_number)
                .count(),
            2,
            "TurnBegan is skipped"
        );
    }

    /// An applied `Batch`'s recorded facts share ONE fresh batch id
    /// ([CR#603.3b] — they were one occurrence); `Single`s record `None`,
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

        state.record_history(&Occurrence::single(GameEvent::Untapped(a)));
        state.record_history(&Occurrence::Batch(vec![
            GameEvent::Untapped(a),
            GameEvent::Untapped(b),
        ]));
        state.record_history(&Occurrence::Batch(vec![GameEvent::Untapped(b)]));

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

    /// A simultaneous batch of intents stays ONE occurrence through every
    /// evolution stage ([CR#603.3b,603.2c]): a destroy-all's `WillDestroy`
    /// batch evolves into one `ZoneWillChange` batch and then one committed
    /// `ZoneChanged` batch — never per-member `Single`s — and the two dies-
    /// facts share a history batch id.
    #[test]
    fn batch_of_intents_evolves_as_one_batch() {
        let (mut state, _view, a) = crate::replace_registry::tests_support::lone_creature();
        let b = crate::replace_registry::tests_support::mint_creature_on_battlefield(&mut state);

        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(vec![
            GameEvent::WillDestroy {
                object: a,
                cause: None,
            },
            GameEvent::WillDestroy {
                object: b,
                cause: None,
            },
        ]))]);

        // Stage 1: the WillDestroy batch applies.
        let crate::step::StepOutcome::Progress(crate::step::Progress::Applied(Occurrence::Batch(
            stage1,
        ))) = state.step()
        else {
            panic!("expected the WillDestroy batch to apply");
        };
        assert_eq!(stage1.len(), 2);

        // Stage 2: ONE ZoneWillChange batch (not two Singles).
        let crate::step::StepOutcome::Progress(crate::step::Progress::Applied(Occurrence::Batch(
            stage2,
        ))) = state.step()
        else {
            panic!("expected one ZoneWillChange batch");
        };
        assert_eq!(stage2.len(), 2);
        assert!(
            stage2
                .iter()
                .all(|e| matches!(e, GameEvent::ZoneWillChange { .. })),
            "stage 2 is the intent batch, got {stage2:?}"
        );

        // Stage 3: ONE committed ZoneChanged batch.
        let crate::step::StepOutcome::Progress(crate::step::Progress::Applied(Occurrence::Batch(
            stage3,
        ))) = state.step()
        else {
            panic!("expected one ZoneChanged batch");
        };
        assert_eq!(stage3.len(), 2);
        assert!(
            stage3
                .iter()
                .all(|e| matches!(e, GameEvent::ZoneChanged { .. })),
            "stage 3 is the committed fact batch, got {stage3:?}"
        );

        // The two dies-facts share one history batch id.
        let ids: Vec<Option<deckmaste_core::Uint>> = state
            .history
            .entries()
            .filter(|e| matches!(e.fact, GameEvent::ZoneChanged { .. }))
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
        let place = |n: deckmaste_core::Uint| GameEvent::CounterPlaced {
            object: id,
            kind: "P1P1Counter".into(),
            amount: n,
            before: 0,
            after: 0,
            cause: None,
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
                    Occurrence::Single(GameEvent::CounterPlaced {
                        amount,
                        before,
                        after,
                        ..
                    }),
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
    #[test]
    fn discard_batch_fixes_that_much_to_its_card_count() {
        let (mut state, _view, _id) = crate::replace_registry::tests_support::lone_creature();
        // Two cards in hand to discard.
        let mut in_hand = Vec::new();
        for name in ["Discard A", "Discard B"] {
            let card =
                std::sync::Arc::new(deckmaste_core::Card::Normal(deckmaste_core::CardFace {
                    name: name.into(),
                    types: vec![deckmaste_core::Type::Sorcery],
                    ..deckmaste_core::CardFace::default()
                }));
            let cid = state.cards.push(card, PlayerId(0));
            let id = state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Hand));
            state.zones.hands[0].push(id);
            in_hand.push(id);
        }
        let events: Vec<GameEvent> = in_hand
            .into_iter()
            .map(|object| GameEvent::ZoneWillChange {
                object,
                from: Some(Zone::Hand),
                to: Zone::Graveyard,
                enters: None,
                position: None,
                face: None,
                cause: Some(crate::event::Cause::discard(
                    deckmaste_core::Agency::EffectInstruction,
                    None,
                )),
            })
            .collect();
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(events))]);
        let _ = state.step(); // the intent batch
        let _ = state.step(); // the committed ZoneChanged batch
        assert_eq!(
            state.that_much,
            Some(2),
            "the discard clause's amount is its card count"
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
                WorkItem::AnnounceOptionalCosts { index: 0 },
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
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
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

    /// `open_distribute` grants the DECIDER (looker) visibility over every
    /// object in the window, not the owner. This is the invariant Fateseal
    /// relies on: player 0 distributes cards owned/controlled by player 1, but
    /// the `look_grant` goes to player 0, not player 1.
    #[test]
    fn distribute_grants_looker_visibility() {
        let mut state = game();
        // Mint two objects into player 1's library (owner is player 1).
        let x = state.objects.mint(
            ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Library),
        );
        let y = state.objects.mint(
            ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Library),
        );
        let window = vec![x, y];
        let bins = vec![deckmaste_core::Bin::Top, deckmaste_core::Bin::Bottom];
        // Player 0 is the looker/decider (fateseal scenario).
        state.open_distribute(PlayerId(0), window, bins, "Fateseal".into());
        // Looker (player 0) gets grants for both objects.
        assert!(
            state.look_grants.contains(&(PlayerId(0), x)),
            "looker gets grant for x"
        );
        assert!(
            state.look_grants.contains(&(PlayerId(0), y)),
            "looker gets grant for y"
        );
        // Owner (player 1) does NOT get a grant from player 0's look.
        assert!(
            !state.look_grants.contains(&(PlayerId(1), x)),
            "owner is NOT granted by looker's distribute"
        );
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
            GameEvent::GotDesignation { player: p0, name },
        ))]);
        let _ = state.step();
        assert!(
            state.designations.players.contains_key(&(p0, name)),
            "player 0 now holds the city's blessing"
        );

        // A second apply does not panic and leaves the entry present.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Single(
            GameEvent::GotDesignation { player: p0, name },
        ))]);
        let _ = state.step();
        assert!(state.designations.players.contains_key(&(p0, name)));
    }

    // --- Legend-rule wiring (Task C3) ---

    // --- SBA loop termination (EmitSbaBatch) ---

    mod sba_loop_termination {
        use std::path::Path;
        use std::sync::Arc;

        use deckmaste_cards::plugin::Plugin;
        use deckmaste_core::Card;
        use deckmaste_core::Predicate;
        use deckmaste_core::Type;
        use deckmaste_core::Zone;

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
            let myr = Arc::new(canon().card("Darksteel Myr").unwrap());
            let forest = Arc::new(builtin().card("Forest").unwrap());
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
                counter_decls: std::collections::HashMap::new(),
                subtypes: std::collections::HashMap::new(),
            });
            let m = *state.zones.hands[0]
                .iter()
                .find(|&&o| {
                    obj_matches(
                        &state,
                        o,
                        &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                            Type::Creature,
                        )),
                    )
                })
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
            state.objects.obj_mut(myr).damage = 1; // toughness 1 → lethal
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

        use deckmaste_cards::plugin::Plugin;
        use deckmaste_core::Card;
        use deckmaste_core::StatValue;
        use deckmaste_core::Supertype;
        use deckmaste_core::Type;
        use deckmaste_core::Zone;

        use crate::agenda::WorkItem;
        use crate::decide::PendingDecision;
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
            let forest = Arc::new(builtin().card("Forest").unwrap());
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
                counter_decls: std::collections::HashMap::new(),
                subtypes: std::collections::HashMap::new(),
            })
        }

        fn legendary_creature(state: &mut GameState, name: &str, controller: PlayerId) -> ObjectId {
            let card = Arc::new(Card::Normal(deckmaste_core::CardFace {
                name: name.into(),
                types: vec![Type::Creature],
                supertypes: vec![Supertype::Legendary],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..deckmaste_core::CardFace::default()
            }));
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
                    StepOutcome::NeedsDecision(PendingDecision::LegendRule {
                        player: PlayerId(0),
                        ..
                    })
                ),
                "got {outcome:?}"
            );
        }

        fn nonlegendary_creature(
            state: &mut GameState,
            name: &str,
            controller: PlayerId,
        ) -> ObjectId {
            let card = Arc::new(Card::Normal(deckmaste_core::CardFace {
                name: name.into(),
                types: vec![Type::Creature],
                supertypes: vec![],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..deckmaste_core::CardFace::default()
            }));
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
                StepOutcome::NeedsDecision(PendingDecision::LegendRule { .. })
            ));
            assert!(!matches!(
                state.step(),
                StepOutcome::NeedsDecision(PendingDecision::LegendRule { .. })
            ));
        }
    }
}
