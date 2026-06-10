//! Trigger event-matching ([CR#603.2,603.6]), the trigger *scan* (emit a
//! `TriggerFired` per match, whose apply notes a `NotedTrigger`), and the
//! `PlaceTriggers` barrier ([CR#603.3]) that puts noted triggers on the stack
//! in APNAP order with an `OrderTriggers` decision and a target choice at
//! placement.
//!
//! Matching is pure predicates (`event_matches`, `filter_matches_snapshot`,
//! `condition_holds`); `scan_triggers` and `place_triggers` are the
//! scheduling/agenda-touching functions.

use deckmaste_core::{
    Ability, CharacteristicFilter, Condition, Event, Filter, Reference, StateFilterEvent,
    TargetSpec, Type, Uint, Zone,
};

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::{GameEvent, Occurrence};
use crate::lki::LkiSnapshot;
use crate::object::{ObjectId, ObjectSource};
use crate::player::PlayerId;
use crate::stack::{StackEntry, StackObject};
use crate::state::GameState;
use crate::step::Progress;

/// The last-known information a fired trigger carries to its placement and
/// resolution ([CR#603.10a], [CR#608.2]): `~`/`This`/source, the moved object,
/// and the affected player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerBindings {
    /// The firing object's last-known self (`~`/`This`/source).
    pub this: Option<LkiSnapshot>,
    /// The moved object for a `ZoneMove` trigger.
    pub that_object: Option<LkiSnapshot>,
    pub that_player: Option<PlayerId>,
}

/// A trigger that has fired but is not yet on the stack ([CR#603.2]). Noted by
/// applying a `TriggerFired`; placed by the `PlaceTriggers` barrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotedTrigger {
    pub source: ObjectSource,
    pub ability: usize,
    pub controller: PlayerId,
    pub bindings: TriggerBindings,
}

/// A triggered ability whose placement is mid-flight: its stack id is minted
/// and the `ChooseTargets` decision is open ([CR#603.3d]). The trigger analogue
/// of `announcing` — but a trigger is *not* an announce (no cost, no priority
/// window), so it has its own staging slot. Answering `ChooseTargets` supplies
/// the targets and pushes the committed `StackEntry`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTrigger {
    /// The freshly minted stack identity ([CR#405]).
    pub id: ObjectId,
    pub source: ObjectSource,
    pub ability: usize,
    pub controller: PlayerId,
    pub bindings: TriggerBindings,
}

impl GameState {
    /// [CR#603.2,603.6]: does `pattern` match `event`, for an ability on
    /// `watcher`?
    ///
    /// `Dies`/`Enters` are `Event::Expanded` macros over `ZoneMove`; they are
    /// looked through transparently via the `Expanded` arm.
    pub(crate) fn event_matches(
        &self,
        pattern: &Event,
        event: &GameEvent,
        watcher: ObjectSource,
    ) -> bool {
        match pattern {
            // Look through a remembered macro invocation — `Dies`, `Enters`,
            // `Landfall`, … all expand to one of the structural forms below.
            Event::Expanded(e) => self.event_matches(&e.value, event, watcher),

            // [CR#603.6]: a `ZoneMove` pattern matches a `ZoneChanged` fact
            // when the zone constraints hold and the `what` filter matches the
            // moved object's last-known state.
            Event::ZoneMove { what, from, to } => match event {
                GameEvent::ZoneChanged {
                    snapshot,
                    from: ef,
                    to: et,
                } => {
                    zone_ok(*from, *ef)
                        && zone_ok(*to, Some(*et))
                        && self.filter_matches_snapshot(what, snapshot, watcher)
                }
                _ => false,
            },

            // [CR#603.2]: step/phase triggers match a `StepBegan`.
            // `_whose` filtering (Your/AnOpponents/EachPlayers) is not reached
            // by any Stage-3 fixture; the constraint is wired as a seam here.
            Event::BeginningOf(step, _whose) => {
                matches!(event, GameEvent::StepBegan(s) if s == step)
            }

            // [CR#603.2e]: a "becomes [state]" transition. The `Attacking`
            // designation ([CR#508.1a]) is a live event ([CR#603.6] — it reaches
            // the trigger stage like any occurrence): match `GameEvent::Attacking`
            // against the `becomes` state and run the `of` filter against the
            // still-live attacking object. ("Becomes blocked" [CR#509.3c] is
            // deliberately NOT wired here: a creature blocked by N blockers emits
            // N `Blocked` events, so a naive point-wise match would fire it N
            // times instead of once — it needs once-per-attacker dedup, deferred
            // until a fixture forces it.)
            Event::StateBecomes { of, becomes } => {
                let live = match (becomes, event) {
                    (StateFilterEvent::Attacking, GameEvent::Attacking(o)) => Some(*o),
                    _ => None,
                };
                live.is_some_and(|o| self.filter_matches_live(of, o, watcher))
            }

            other => todo!("stage 3 does not match trigger event {other:?}"),
        }
    }

    /// Evaluate `filter` against a *live* object `o` for an ability on
    /// `watcher`.
    ///
    /// The live counterpart of
    /// [`filter_matches_snapshot`](Self::filter_matches_snapshot):
    /// the transitioning object (an attacker/blocked creature) is still on the
    /// battlefield, so characteristics come from the live object via
    /// [`crate::target::matches`]. The one arm `target::matches` can't evaluate
    /// is `Is(This)` ([CR#603.10a] self-reference, which needs the `watcher`);
    /// that is special-cased here (and threaded through the logical
    /// combinators).
    fn filter_matches_live(&self, filter: &Filter, o: ObjectId, watcher: ObjectSource) -> bool {
        match filter {
            // "this object": match only when `o` is the watching object.
            Filter::Is(Reference::This) => self.objects.obj(o).source == watcher,

            // Logical combinators: recurse so an `Is(This)` nested inside is
            // still resolved against the watcher.
            Filter::AllOf(fs) => fs.iter().all(|f| self.filter_matches_live(f, o, watcher)),
            Filter::OneOf(fs) => fs.iter().any(|f| self.filter_matches_live(f, o, watcher)),
            Filter::Not(f) => !self.filter_matches_live(f, o, watcher),
            Filter::Expanded(e) => self.filter_matches_live(&e.value, o, watcher),

            // Everything else (`Any`, "a creature", …) is a plain live-object
            // characteristic test.
            other => crate::target::matches(self, o, other),
        }
    }

    /// Evaluate `filter` against the last-known state of a moved object
    /// (captured in `snapshot`), for an ability on `watcher`.
    ///
    /// Mirrors `target::matches` but sources characteristics from the snapshot
    /// instead of a live object — necessary for leaves where the object is
    /// already reminted/gone.
    pub(crate) fn filter_matches_snapshot(
        &self,
        filter: &Filter,
        snapshot: &LkiSnapshot,
        watcher: ObjectSource,
    ) -> bool {
        match filter {
            // "this object": match only when the snapshot is the watching object
            // ([CR#603.10a] — self-dies / self-enters).
            Filter::Is(Reference::This) => snapshot.source == watcher,

            // "a creature" — check the snapshot's printed card types.
            Filter::Characteristic(CharacteristicFilter::Type(ty)) => {
                snapshot_has_type(self, snapshot, *ty)
            }

            // Logical combinators: recurse.
            Filter::AllOf(fs) => fs
                .iter()
                .all(|f| self.filter_matches_snapshot(f, snapshot, watcher)),
            Filter::OneOf(fs) => fs
                .iter()
                .any(|f| self.filter_matches_snapshot(f, snapshot, watcher)),
            Filter::Not(f) => !self.filter_matches_snapshot(f, snapshot, watcher),

            // Match-anything.
            Filter::Any => true,

            // Look through a remembered filter macro.
            Filter::Expanded(e) => self.filter_matches_snapshot(&e.value, snapshot, watcher),

            other => todo!("stage 3 does not evaluate snapshot filter {other:?}"),
        }
    }

    /// Evaluate an intervening-`if` condition ([CR#603.4]) against the current
    /// game state (the state at the moment of the occurrence). The trigger scan
    /// calls this only when the ability has a condition; a `None` condition is
    /// treated as "holds" by the caller.
    pub(crate) fn condition_holds(&self, cond: &Condition) -> bool {
        match cond {
            // "if you control a creature" / "if a creature is on the battlefield"
            Condition::Exists(filter) => !crate::target::candidates(self, filter).is_empty(),

            // "if it is a [filter]" — Is(ref, filter): look up the ref and test.
            // Not reached by any Stage-3 fixture; wired as a seam.
            Condition::Is(_, _) => todo!("stage 3 does not evaluate Condition::Is"),

            // Numeric comparison.
            Condition::Compare(_, _, _) => todo!("stage 3 does not evaluate Condition::Compare"),

            Condition::AllOf(cs) => cs.iter().all(|c| self.condition_holds(c)),
            Condition::OneOf(cs) => cs.iter().any(|c| self.condition_holds(c)),
            Condition::Not(c) => !self.condition_holds(c),

            // Look through a macro.
            Condition::Expanded(e) => self.condition_holds(&e.value),

            Condition::Happened { .. } => todo!("stage 3 does not evaluate Condition::Happened"),
        }
    }

    /// [CR#603.2,603.6]: after an occurrence applies, scan watching abilities
    /// for ones whose `event` pattern matches each occurred fact, and schedule
    /// a `TriggerFired` per match at the agenda front (so they apply in the
    /// occurrence's wake — [CR#603.3b]). Applying a `TriggerFired` is what
    /// *notes* the trigger; this only emits.
    ///
    /// Watchers ([CR#603.6]) are every live battlefield permanent, plus — for a
    /// `ZoneChanged` that LEFT the battlefield — the leaving object itself (via
    /// its snapshot), so its own dies-trigger is considered even though its
    /// abilities are gone from the battlefield ([CR#603.6d]). An *entering*
    /// object is already a live battlefield permanent, so it is not re-added.
    pub(crate) fn scan_triggers(&mut self, facts: &Occurrence) {
        let events: &[GameEvent] = match facts {
            Occurrence::Single(e) => std::slice::from_ref(e),
            Occurrence::Batch(es) => es,
        };
        let mut emits: Vec<WorkItem> = Vec::new();
        for event in events {
            // Skip facts no fixture trigger watches; never scan a `TriggerFired`
            // (avoids any chance of recursion). `ZoneWillChange` is skipped because
            // trigger-matching happens on the downstream `ZoneChanged` fact (already
            // queued by the will-change apply at the agenda front — [CR#603.6]);
            // matching on the intent would double-fire every zone-move trigger.
            match event {
                GameEvent::TriggerFired { .. }
                | GameEvent::TriggerResolved(_)
                | GameEvent::StepBegan(_)
                | GameEvent::TurnBegan { .. }
                | GameEvent::ZoneWillChange { .. } => continue,
                _ => {}
            }
            self.scan_event(event, &mut emits);
        }
        if !emits.is_empty() {
            self.schedule_front(emits);
        }
    }

    /// Scan one occurred fact against every watcher, pushing a `TriggerFired`
    /// emit per match onto `emits`.
    fn scan_event(&self, event: &GameEvent, emits: &mut Vec<WorkItem>) {
        // The moved object's snapshot for a zone change (the trigger's
        // `that_object`); `None` for any other fact.
        let subject: Option<&LkiSnapshot> = match event {
            GameEvent::ZoneChanged { snapshot, .. } => Some(snapshot),
            _ => None,
        };

        // The watcher set ([CR#603.6]): every live battlefield permanent, plus
        // the leaving object's snapshot for a battlefield-leave.
        let mut watchers: Vec<Watcher> = self
            .zones
            .battlefield
            .iter()
            .map(|&id| Watcher::Live(id))
            .collect();
        if let GameEvent::ZoneChanged {
            snapshot,
            from: Some(Zone::Battlefield),
            ..
        } = event
        {
            // The leaving object — its abilities are no longer on the
            // battlefield, so add it explicitly ([CR#603.6d]).
            watchers.push(Watcher::Leaving(snapshot.clone()));
        }

        for watcher in watchers {
            let (source, controller, this) = match watcher {
                Watcher::Live(id) => {
                    let o = self.objects.obj(id);
                    (o.source, o.controller, LkiSnapshot::capture(self, id))
                }
                Watcher::Leaving(s) => (s.source, s.controller, s.clone()),
            };
            for (idx, ability) in crate::derive::abilities_of_source(self, source)
                .iter()
                .enumerate()
            {
                let Ability::Triggered(t) = ability else {
                    continue;
                };
                if self.event_matches(&t.event, event, source)
                    && t.condition.as_ref().is_none_or(|c| self.condition_holds(c))
                {
                    emits.push(WorkItem::Emit(Occurrence::single(
                        GameEvent::TriggerFired {
                            source,
                            ability: Uint::try_from(idx).expect("ability index fits in Uint"),
                            controller,
                            bindings: TriggerBindings {
                                this: Some(this.clone()),
                                that_object: subject.cloned(),
                                that_player: None,
                            },
                        },
                    )));
                }
            }
        }
    }

    /// [CR#603.3]: the placement barrier. Puts noted triggers on the stack in
    /// APNAP order ([CR#603.3b]) — the active player's first (so they resolve
    /// last). One `step()` places at most one trigger (or surfaces a decision):
    /// it re-schedules itself to loop, with `CheckSbas` ahead, so a placement
    /// that produced new state re-sweeps before the next is placed.
    ///
    /// Returns `TriggersPlaced { placed }`: 1 when a trigger was placed, 0 when
    /// none were waiting or a decision (`OrderTriggers` / `ChooseTargets`) was
    /// surfaced instead.
    pub(crate) fn place_triggers(&mut self) -> Progress {
        if self.pending_triggers.is_empty() {
            return Progress::TriggersPlaced { placed: 0 };
        }

        // APNAP: the first player from the active player who still has a noted
        // trigger ([CR#603.3b]).
        let order = self.apnap_order();
        let Some(player) = order
            .into_iter()
            .find(|&p| self.pending_triggers.iter().any(|t| t.controller == p))
        else {
            // Triggers exist but none belong to a live APNAP player — drop them.
            self.pending_triggers.clear();
            return Progress::TriggersPlaced { placed: 0 };
        };

        let mine: Vec<NotedTrigger> = self
            .pending_triggers
            .iter()
            .filter(|t| t.controller == player)
            .cloned()
            .collect();

        if mine.len() > 1 {
            // [CR#603.3b]: this player orders their simultaneous triggers.
            self.pending = Some(PendingDecision::OrderTriggers {
                player,
                triggers: mine,
            });
            return Progress::TriggersPlaced { placed: 0 };
        }

        // Exactly one: place it now and loop (re-sweep, then place the next).
        let noted = self.take_first_trigger_of(player);
        let placed = self.place_one_trigger(noted);
        self.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
        Progress::TriggersPlaced {
            placed: u32::from(placed),
        }
    }

    /// Controllers in APNAP order ([CR#603.3b]): the active player, then around
    /// the table, skipping lost players.
    fn apnap_order(&self) -> Vec<PlayerId> {
        let mut order = vec![self.turn.active_player];
        let mut p = self.turn.active_player;
        while order.len() < self.players.iter().filter(|pl| !pl.lost).count() {
            p = self.next_live_after(p);
            if p == self.turn.active_player {
                break;
            }
            order.push(p);
        }
        order
    }

    /// Removes and returns the FIRST noted trigger controlled by `player`
    /// (preserving the rest's order — the `OrderTriggers` reorder, when used,
    /// already fixed it).
    ///
    /// # Panics
    ///
    /// Panics if `player` controls no noted trigger — the caller guards this.
    pub(crate) fn take_first_trigger_of(&mut self, player: PlayerId) -> NotedTrigger {
        let i = self
            .pending_triggers
            .iter()
            .position(|t| t.controller == player)
            .expect("a noted trigger for this player");
        self.pending_triggers.remove(i)
    }

    /// Reorders `player`'s noted triggers in place to match `ordered` (the
    /// permutation chosen via `OrderTriggers`), leaving other players' notes
    /// untouched. Used by the `OrderTriggers` submission.
    pub(crate) fn reorder_pending_triggers(
        &mut self,
        player: PlayerId,
        ordered: Vec<NotedTrigger>,
    ) {
        let mut ordered = ordered.into_iter();
        for slot in &mut self.pending_triggers {
            if slot.controller == player {
                *slot = ordered
                    .next()
                    .expect("ordered covers this player's triggers");
            }
        }
        debug_assert!(
            ordered.next().is_none(),
            "ordered had more entries than this player's noted triggers",
        );
    }

    /// [CR#603.3c,603.3d]: place one noted trigger on the stack. If its ability
    /// targets and at least one legal target exists, mint the stack id and open
    /// a `ChooseTargets` decision (returns `false` — not yet placed). If it
    /// targets but has NO legal target, drop it ([CR#603.3c], returns `false`).
    /// If it does not target, push the committed `StackEntry` directly (returns
    /// `true`).
    pub(crate) fn place_one_trigger(&mut self, noted: NotedTrigger) -> bool {
        let specs = self.trigger_targets(noted.source, noted.ability);
        if specs.is_empty() {
            // No targets — push directly with a freshly minted stack id.
            let id = self
                .objects
                .mint(noted.source, noted.controller, Some(Zone::Stack));
            self.stack.push(StackEntry {
                id,
                object: StackObject::Triggered {
                    source: noted.source,
                    ability: noted.ability,
                    bindings: noted.bindings,
                },
                controller: noted.controller,
                targets: vec![],
            });
            return true;
        }

        // It targets ([CR#603.3d]). Compute each spec's legal candidates.
        let legal: Vec<Vec<ObjectId>> = specs.iter().map(|s| self.legal_targets(s)).collect();
        if legal.iter().any(Vec::is_empty) {
            // [CR#603.3c]: a target with no legal choice — the trigger is
            // removed from the stack, never placed.
            return false;
        }

        // Stage the in-flight placement and surface the target choice.
        let controller = noted.controller;
        let id = self
            .objects
            .mint(noted.source, controller, Some(Zone::Stack));
        self.placing_trigger = Some(crate::trigger::PendingTrigger {
            id,
            source: noted.source,
            ability: noted.ability,
            controller,
            bindings: noted.bindings,
        });
        self.pending = Some(PendingDecision::ChooseTargets {
            player: controller,
            spec: specs,
            legal,
        });
        false
    }

    /// The `TriggeredAbility.targets` for ability `ability` of `source`'s
    /// printed face. Empty when the ability is non-targeting.
    fn trigger_targets(&self, source: ObjectSource, ability: usize) -> Vec<TargetSpec> {
        match &crate::derive::abilities_of_source(self, source)[ability] {
            Ability::Triggered(t) => t.targets.clone(),
            _ => unreachable!("a noted trigger indexes a Triggered ability"),
        }
    }

    /// [CR#603.3d]: a placing trigger's targets were chosen — push the
    /// committed `StackEntry` (using the already-minted stack id) and clear the
    /// staging slot. Called from the `ChooseTargets` submission.
    ///
    /// # Panics
    ///
    /// Panics if no trigger placement is in flight — an engine invariant (the
    /// staging slot is open across the decision), not caller input.
    pub(crate) fn commit_placing_trigger(&mut self, targets: Vec<ObjectId>) {
        let staged = self
            .placing_trigger
            .take()
            .expect("a trigger placement in flight");
        self.stack.push(StackEntry {
            id: staged.id,
            object: StackObject::Triggered {
                source: staged.source,
                ability: staged.ability,
                bindings: staged.bindings,
            },
            controller: staged.controller,
            targets,
        });
    }
}

/// A candidate watching object ([CR#603.6]) for the trigger scan: a live
/// battlefield permanent, or a just-left object carried by its snapshot.
enum Watcher {
    Live(ObjectId),
    Leaving(LkiSnapshot),
}

/// Whether `zone_constraint` (from the trigger pattern) is satisfied by
/// `actual` (from the `ZoneChanged` event).
///
/// `None` in the pattern means "any zone" (open constraint); `Some(z)` requires
/// an exact match.
fn zone_ok(constraint: Option<Zone>, actual: Option<Zone>) -> bool {
    match constraint {
        None => true,
        Some(z) => actual == Some(z),
    }
}

/// Whether the snapshot's card has the given type in its printed face.
fn snapshot_has_type(state: &GameState, snapshot: &LkiSnapshot, ty: Type) -> bool {
    match snapshot.source {
        ObjectSource::Card(card_id) => {
            let card = &state.cards.get(card_id).def;
            crate::derive::face(card).types.contains(&ty)
        }
        // Player proxies have no card types.
        ObjectSource::Player(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::{CharacteristicFilter, Condition, Event, Filter, Reference, Type, Zone};

    use crate::event::GameEvent;
    use crate::lki::LkiSnapshot;
    use crate::object::{ObjectId, ObjectSource};
    use crate::player::PlayerId;
    use crate::state::{GameConfig, GameState, PlayerConfig, StartingPlayer};
    use crate::target::matches;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn testing() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
        )
        .unwrap()
    }

    /// Build a two-player game with one Vanilla Creature forced onto the
    /// battlefield, mirroring the `bear_on_field` helper in other test modules.
    fn bear_on_field() -> (GameState, ObjectId) {
        let bears = Arc::new(testing().card("Vanilla Creature").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("a Vanilla Creature in the opening hand");
        state.zones.hands[0].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// Capture a snapshot of `id` while it's still live, then build a fake
    /// `ZoneChanged` as if it moved `from → to`.
    fn zone_changed_event(state: &GameState, id: ObjectId, from: Zone, to: Zone) -> GameEvent {
        let snapshot = LkiSnapshot::capture(state, id);
        GameEvent::ZoneChanged {
            snapshot,
            from: Some(from),
            to,
        }
    }

    // -------------------------------------------------------------------------
    // Dies(Type(Creature))
    // -------------------------------------------------------------------------

    /// `Dies(Type(Creature))` expands to
    /// `ZoneMove { what: Type(Creature), from: Battlefield, to: Graveyard }`.
    /// It must match a creature dying (battlefield → graveyard) …
    #[test]
    fn dies_type_creature_matches_creature_dying() {
        let testing = testing();
        // The dies-watcher card uses Dies(Type(Creature)) in its event.
        let watcher_card = Arc::new(testing.card("Creature dies-watcher LoseLife").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&watcher_card); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        // Place the watcher on the battlefield so its ObjectSource is accessible.
        let watcher = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("watcher in opening hand");
        state.zones.hands[0].retain(|&o| o != watcher);
        state.objects.obj_mut(watcher).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(watcher);
        let watcher_source = state.objects.obj(watcher).source;

        // Separately put a Vanilla Creature on the battlefield.
        let bear = {
            let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
            let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
            let bid = state.objects.mint(
                ObjectSource::Card(bear_card),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(bid);
            bid
        };

        // Build the `ZoneChanged` for the Vanilla Creature dying.
        let event = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);

        // The pattern from Dies(Type(Creature)) — built directly.
        let pattern = Event::ZoneMove {
            what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "Dies(Type(Creature)) must match a creature dying"
        );
    }

    /// … and must NOT match a creature entering (wrong direction).
    #[test]
    fn dies_type_creature_does_not_match_creature_entering() {
        let testing = testing();
        let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .unwrap();
        state.zones.hands[0].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        let watcher_source = state.objects.obj(bear).source;

        // Entering event (hand → battlefield).
        let snapshot = LkiSnapshot {
            object: bear,
            source: state.objects.obj(bear).source,
            controller: PlayerId(0),
            tapped: false,
            damage: 0,
            left: Zone::Hand,
        };
        let enter_event = GameEvent::ZoneChanged {
            snapshot,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
        };

        let dies_pattern = Event::ZoneMove {
            what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            !state.event_matches(&dies_pattern, &enter_event, watcher_source),
            "Dies(Type(Creature)) must NOT match an entering event"
        );
    }

    /// Must NOT match a non-creature dying.
    #[test]
    fn dies_type_creature_does_not_match_non_creature_dying() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;

        // Find a Forest in hand (not a creature).
        let land = *state.zones.hands[1]
            .first()
            .expect("player 1 has cards in hand");
        let land_source = state.objects.obj(land).source;

        // Simulate a non-creature leaving the battlefield (battlefield →
        // graveyard). The zone constraints DO match Dies, so the *filter* must
        // do the rejecting — the land lacks the Creature type.
        let snapshot = LkiSnapshot {
            object: land,
            source: land_source,
            controller: PlayerId(1),
            tapped: false,
            damage: 0,
            left: Zone::Battlefield,
        };
        let event = GameEvent::ZoneChanged {
            snapshot,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
        };

        let dies_pattern = Event::ZoneMove {
            what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            !state.event_matches(&dies_pattern, &event, watcher_source),
            "Dies(Type(Creature)) must NOT match a non-creature"
        );
    }

    // -------------------------------------------------------------------------
    // Dies(Is(This)) — self-dies
    // -------------------------------------------------------------------------

    /// `Dies(Is(This))` matches only when the dying object IS the watcher.
    #[test]
    fn dies_this_matches_only_self_death() {
        let testing = testing();
        let dies_card = Arc::new(
            testing
                .card("Creature dies-trigger DealDamage AnyTarget")
                .unwrap(),
        );
        let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&dies_card); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });

        // Place the dies-trigger creature on the battlefield.
        let trigger_obj = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("trigger creature in hand");
        state.zones.hands[0].retain(|&o| o != trigger_obj);
        state.objects.obj_mut(trigger_obj).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(trigger_obj);
        let trigger_source = state.objects.obj(trigger_obj).source;

        // Place a Vanilla Creature beside it.
        let bear_card_id = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let other = state.objects.mint(
            ObjectSource::Card(bear_card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(other);
        let other_source = state.objects.obj(other).source;

        // Pattern: Dies(Is(This))
        let self_dies = Event::ZoneMove {
            what: Filter::Is(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        // Self-death: snapshot.source == watcher
        let self_death_event =
            zone_changed_event(&state, trigger_obj, Zone::Battlefield, Zone::Graveyard);
        assert!(
            state.event_matches(&self_dies, &self_death_event, trigger_source),
            "Dies(Is(This)) must match when the dying object is the watcher"
        );

        // Other-death: snapshot.source != watcher
        let other_death_event =
            zone_changed_event(&state, other, Zone::Battlefield, Zone::Graveyard);
        assert!(
            !state.event_matches(&self_dies, &other_death_event, trigger_source),
            "Dies(Is(This)) must NOT match when a different creature dies"
        );

        // Watcher's own death should NOT match from watcher's OWN perspective
        // when the other creature dies (wrong watcher).
        assert!(
            !state.event_matches(&self_dies, &self_death_event, other_source),
            "Dies(Is(This)) must NOT match when the dying object is a different watcher"
        );
    }

    // -------------------------------------------------------------------------
    // Enters(Is(This)) — self-enters
    // -------------------------------------------------------------------------

    /// `Enters(Is(This))` matches when the object entering is the watcher.
    #[test]
    fn enters_this_matches_own_entry() {
        let testing = testing();
        let etb_card = Arc::new(testing.card("Creature etb-trigger DrawCards").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&etb_card); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });

        // The ETB creature starts in hand; we want to simulate it entering.
        let etb_obj = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("ETB creature in hand");
        let etb_source = state.objects.obj(etb_obj).source;

        // Pattern: Enters(Is(This))
        let self_enters = Event::ZoneMove {
            what: Filter::Is(Reference::This),
            from: None,
            to: Some(Zone::Battlefield),
        };

        // Build a ZoneChanged snapshot for the ETB creature entering.
        let enters_snapshot = LkiSnapshot {
            object: etb_obj,
            source: etb_source,
            controller: PlayerId(0),
            tapped: false,
            damage: 0,
            // For an enter, the snapshot's `left` represents what zone it came
            // from — Hand in this case.
            left: Zone::Hand,
        };
        let enters_event = GameEvent::ZoneChanged {
            snapshot: enters_snapshot,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
        };

        assert!(
            state.event_matches(&self_enters, &enters_event, etb_source),
            "Enters(Is(This)) must match when the entering object is the watcher"
        );

        // A different (placeholder) ObjectSource must not match.
        let forest_obj = *state.zones.hands[1].first().expect("player 1 hand");
        let forest_source = state.objects.obj(forest_obj).source;
        assert!(
            !state.event_matches(&self_enters, &enters_event, forest_source),
            "Enters(Is(This)) must NOT match when the entering object is not the watcher"
        );

        // A ZoneWillChange (not ZoneChanged) must not match.
        let will_change_event = GameEvent::ZoneWillChange {
            object: etb_obj,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            enters: None,
            position: None,
        };
        assert!(
            !state.event_matches(&self_enters, &will_change_event, etb_source),
            "Enters triggers must not fire on ZoneWillChange (only on ZoneChanged)"
        );
    }

    // -------------------------------------------------------------------------
    // Intervening-if: condition_holds
    // -------------------------------------------------------------------------

    /// `Condition::Exists(Type(Creature))` is true when a creature is on the
    /// battlefield, false when none is.
    #[test]
    fn condition_holds_exists_creature_true_when_creature_present() {
        let (state, _bear) = bear_on_field();
        let cond = Condition::Exists(Filter::Characteristic(CharacteristicFilter::Type(
            Type::Creature,
        )));
        assert!(
            state.condition_holds(&cond),
            "Exists(Type(Creature)) should hold when a creature is on the battlefield"
        );
    }

    #[test]
    fn condition_holds_exists_creature_false_when_no_creature() {
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let cond = Condition::Exists(Filter::Characteristic(CharacteristicFilter::Type(
            Type::Creature,
        )));
        assert!(
            !state.condition_holds(&cond),
            "Exists(Type(Creature)) should NOT hold when no creatures are present"
        );
    }

    // -------------------------------------------------------------------------
    // Event macro round-trip: Dies / Enters expand correctly
    // -------------------------------------------------------------------------

    /// Confirm that `Dies(Type(Creature))` parses and produces the expected
    /// `Event::Expanded` shape wrapping `ZoneMove`.
    #[test]
    fn dies_macro_expands_to_zone_move() {
        use deckmaste_core::Event;

        let event: Event = testing().macros.read_str("Dies(Type(Creature))").unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded, got {event:?}");
        };
        assert_eq!(expanded.name.as_str(), "Dies");
        assert_eq!(
            *expanded.value,
            Event::ZoneMove {
                what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
            }
        );
    }

    /// Confirm that `Enters(Is(This))` expands to `ZoneMove { to: Battlefield,
    /// from: None }`.
    #[test]
    fn enters_macro_expands_to_zone_move() {
        use deckmaste_core::Event;

        let event: Event = testing().macros.read_str("Enters(Is(This))").unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded, got {event:?}");
        };
        assert_eq!(expanded.name.as_str(), "Enters");
        assert_eq!(
            *expanded.value,
            Event::ZoneMove {
                what: Filter::Is(Reference::This),
                from: None,
                to: Some(Zone::Battlefield),
            }
        );
    }

    /// Confirm that reading `Dies(Is(This))` yields
    /// `Filter::Is(Reference::This)` in the `what` position — the "this
    /// object" form.
    #[test]
    fn dies_this_filter_is_reference_this() {
        use deckmaste_core::Event;

        let event: Event = testing().macros.read_str("Dies(Is(This))").unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded");
        };
        let Event::ZoneMove { what, .. } = expanded.value.as_ref() else {
            panic!("expected ZoneMove inner, got {:?}", expanded.value);
        };
        assert_eq!(
            what,
            &Filter::Is(Reference::This),
            "Dies(Is(This)) must use Filter::Is(Reference::This)"
        );
    }

    // -------------------------------------------------------------------------
    // The trigger scan: noting into pending_triggers
    // -------------------------------------------------------------------------

    /// Force a single card from a named fixture onto the battlefield (as
    /// player 0's, freshly minted) and return the new id. Player 1's deck is
    /// Forest fodder so the game is well-formed.
    fn fixture_on_field(card_name: &str) -> (GameState, ObjectId) {
        use crate::object::ObjectSource;

        let card = Arc::new(testing().card(card_name).unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// A `Creature dies-trigger DealDamage AnyTarget` on the battlefield with
    /// lethal damage: stepping past the SBA destroy (`CheckSbas` →
    /// `ZoneWillChange` → `ZoneChanged` → `TriggerFired` apply) notes exactly
    /// one trigger, whose `this` binding is the LKI snapshot of the (now-gone)
    /// battlefield id.
    #[test]
    fn dies_trigger_notes_into_pending_triggers() {
        use crate::agenda::WorkItem;

        let (mut state, goblin) = fixture_on_field("Creature dies-trigger DealDamage AnyTarget");
        // toughness 1 → 1 damage is lethal.
        state.objects.obj_mut(goblin).damage = 1;

        state.schedule_front(vec![WorkItem::CheckSbas]);
        for _ in 0..30 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }

        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the self-dies trigger must be noted exactly once"
        );
        let noted = &state.pending_triggers[0];
        assert_eq!(noted.ability, 0);
        assert_eq!(noted.controller, PlayerId(0));
        assert!(
            noted.bindings.this.is_some(),
            "LKI snapshot of the dead goblin"
        );
        // The snapshot's object id is the (now-gone) battlefield id.
        assert_eq!(noted.bindings.this.as_ref().unwrap().object, goblin);
        // The dead object is truly gone from the store.
        assert!(state.objects.get(goblin).is_none());
        // `that_object` for a zone-move trigger is the moved object's snapshot.
        assert_eq!(
            noted.bindings.that_object.as_ref().unwrap().object,
            goblin,
            "the moved object's snapshot rides as that_object"
        );
    }

    /// A non-watching board: a `Vanilla Creature` dying notes NOTHING (it has
    /// no triggered abilities, and no other watcher cares).
    #[test]
    fn vanilla_creature_dying_notes_nothing() {
        use crate::agenda::WorkItem;

        let (mut state, bear) = fixture_on_field("Vanilla Creature");
        // Vanilla Creature has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).damage = 2;

        state.schedule_front(vec![WorkItem::CheckSbas]);
        for _ in 0..30 {
            // Stop once the bear is gone (the death has been fully processed).
            if state.objects.get(bear).is_none() && state.agenda.is_empty() {
                break;
            }
            let _ = state.step();
        }

        assert!(
            state.objects.get(bear).is_none(),
            "the bear should have died and reminted to the graveyard"
        );
        assert!(
            state.pending_triggers.is_empty(),
            "a vanilla creature dying watches nothing — no trigger noted"
        );
    }

    // -------------------------------------------------------------------------
    // PlaceTriggers: placement on the stack ([CR#603.3])
    // -------------------------------------------------------------------------

    /// A single non-targeting noted trigger places DIRECTLY onto the stack as a
    /// `Triggered` object with a fresh id, no decision surfaced.
    #[test]
    fn non_targeting_trigger_places_directly() {
        use crate::stack::StackObject;

        let (mut state, etb) = fixture_on_field("Creature etb-trigger DrawCards");
        let source = state.objects.obj(etb).source;
        let controller = state.objects.obj(etb).controller;
        // Note one trigger by hand (ability 0 = the DrawCards etb).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller,
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 1 },
            "the non-targeting trigger places without a decision"
        );
        assert!(state.pending.is_none(), "no decision surfaced");
        assert!(state.pending_triggers.is_empty(), "the note was consumed");
        assert_eq!(state.stack.len(), 1, "one Triggered object on the stack");
        let entry = &state.stack[0];
        assert!(
            matches!(entry.object, StackObject::Triggered { ability: 0, .. }),
            "the entry is the etb trigger"
        );
        assert_ne!(entry.id, etb, "the stack id is a freshly minted token");
        assert!(entry.targets.is_empty(), "a non-targeting trigger has none");
    }

    /// A targeting noted trigger surfaces a `ChooseTargets` at placement
    /// ([CR#603.3d]): the stack id is minted and staged in `placing_trigger`,
    /// and nothing is on the stack until the target is chosen. (The
    /// no-legal-target drop, [CR#603.3c], can't be hit here — "any target"
    /// always admits the two player proxies.)
    #[test]
    fn targeting_trigger_surfaces_choose_targets_at_placement() {
        use crate::decide::PendingDecision;

        let (mut state, gob) = fixture_on_field("Creature dies-trigger DealDamage AnyTarget");
        let source = state.objects.obj(gob).source;
        let controller = state.objects.obj(gob).controller;
        // Note the dies-trigger (ability 0, targets [AnyTarget]).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller,
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "a target choice surfaces instead of an immediate placement"
        );
        let Some(PendingDecision::ChooseTargets { player, legal, .. }) = &state.pending else {
            panic!("expected ChooseTargets, got {:?}", state.pending);
        };
        assert_eq!(*player, controller);
        assert!(
            !legal[0].is_empty(),
            "AnyTarget admits the player proxies (and the goblin)"
        );
        assert!(
            state.placing_trigger.is_some(),
            "the placement is staged across the decision"
        );
        assert!(
            state.stack.is_empty(),
            "nothing placed until the target is chosen"
        );
    }

    /// [CR#603.3b]: a player controlling TWO simultaneous triggers surfaces an
    /// `OrderTriggers` decision; the submitted permutation becomes the
    /// placement order (last placed resolves first).
    #[test]
    fn two_triggers_one_player_surface_order_triggers() {
        use crate::decide::{Decision, PendingDecision};

        // Two dies-watchers under player 0 (non-targeting `LoseLife`).
        let (mut state, w0) = fixture_on_field("Creature dies-watcher LoseLife");
        let source = state.objects.obj(w0).source;
        let bindings = || super::TriggerBindings {
            this: None,
            that_object: None,
            that_player: None,
        };
        // Two notes from the same controller (as if two creatures died at once).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller: PlayerId(0),
            bindings: bindings(),
        });
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller: PlayerId(0),
            bindings: bindings(),
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "ordering is needed first — nothing placed yet"
        );
        let Some(PendingDecision::OrderTriggers { player, triggers }) = &state.pending else {
            panic!("expected OrderTriggers, got {:?}", state.pending);
        };
        assert_eq!(*player, PlayerId(0));
        assert_eq!(triggers.len(), 2, "both of player 0's triggers offered");

        // A non-permutation is rejected.
        assert!(state.submit_decision(Decision::Order(vec![0, 0])).is_err());
        assert!(state.submit_decision(Decision::Order(vec![5])).is_err());
        // The valid permutation is accepted; placement resumes (re-scheduled).
        state.submit_decision(Decision::Order(vec![1, 0])).unwrap();
        assert!(state.pending.is_none());
        assert!(
            state
                .agenda
                .iter()
                .any(|w| matches!(w, crate::agenda::WorkItem::PlaceTriggers)),
            "placement is re-scheduled after ordering"
        );
    }
}
