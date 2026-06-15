//! Resolution ([CR#608]): dispatch a stack object, and walk its `Effect` AST as
//! reified agenda work. Stage 3 wires the corpus's arms; the rest are `todo!`.

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Count;
use deckmaste_core::Effect;
use deckmaste_core::ManaSpec;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::Scope;
use deckmaste_core::Selection;
use deckmaste_core::StaticEffect;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use rand::seq::SliceRandom;

use crate::agenda::WorkItem;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::layer::ContinuousEffect;
use crate::layer::ScopeResolved;
use crate::object::ObjectId;
use crate::stack::Frame;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameState;

/// "Any color" ([CR#106.1b]): the five colors ([CR#105.1]) — a player asked to
/// choose a color may not choose colorless ([CR#105.4]).
const ANY_COLOR: [ColorOrColorless; 5] = [
    ColorOrColorless::Color(Color::White),
    ColorOrColorless::Color(Color::Blue),
    ColorOrColorless::Color(Color::Black),
    ColorOrColorless::Color(Color::Red),
    ColorOrColorless::Color(Color::Green),
];

impl GameState {
    /// [CR#608]: resolve the committed stack entry whose `id` is `id`. Schedules
    /// the work and the trailing cleanup event.
    ///
    /// Keyed on `StackEntry.id` (not the backing object) so it resolves both
    /// spells and triggered abilities (which have no backing object).
    ///
    /// # Panics
    ///
    /// Panics if no entry has that id — engine invariant, not caller input.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per stack-object kind; splitting would scatter the dispatch"
    )]
    pub(crate) fn resolve_object(&mut self, id: ObjectId) {
        let entry = self
            .stack
            .iter()
            .find(|e| e.id == id)
            .expect("entry on stack")
            .clone();
        // A fresh resolution has no amount fixed yet: `Count::ThatMuch` may
        // only read an amount an earlier instruction of THIS resolution
        // fixed, never one leaking in from combat or a prior resolution.
        self.that_much = None;
        match &entry.object {
            StackObject::Spell(spell) => {
                let spell = *spell;
                if self.is_permanent_spell(spell) {
                    // [CR#608.3]: a permanent spell enters the battlefield.
                    // Host resolution by entry context (spec §4, [CR#303.4]): a
                    // permanent SPELL that enters attached (the Enchant
                    // `AsEnters`) attaches to its resolving spell's CHOSEN TARGET
                    // — the Aura's enchant target — not an arbitrary candidate.
                    // Carry that host in the `EnterStatus`; `apply_zone_will_change`
                    // prefers it over the candidate-set fallback (`.or`).
                    let enters = if self.enters_attached_self(self.objects.obj(spell).source)
                        && let Some(&host) = entry.targets.first()
                    {
                        Some(crate::event::EnterStatus {
                            attach_to: Some(host),
                            ..crate::event::EnterStatus::default()
                        })
                    } else {
                        None
                    };
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Battlefield,
                            enters,
                            position: None,
                            face: None,
                            cause: None,
                        },
                    ))]);
                } else if self.targets_still_legal(&entry) {
                    // Instant/sorcery with all targets still legal: run its effect.
                    let frame = Frame {
                        source: spell,
                        controller: entry.controller,
                        targets: entry.targets.clone(),
                        bindings: None,
                        chosen: None,
                        x: entry.x,
                    };
                    let effect = self
                        .spell_effect(spell)
                        .expect("an instant/sorcery has a Spell ability");
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Box::new(effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: None,
                        })),
                    ]);
                } else {
                    // [CR#608.2b]: all targets illegal — the spell fizzles.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: None,
                        },
                    ))]);
                }
            }
            // [CR#608.2n]: a triggered ability resolves its effect, then vanishes
            // — no zone move, the source untouched. The minted stack id is just
            // discarded when `AbilityResolved` removes the entry.
            StackObject::Triggered {
                source,
                ability,
                bindings,
            } => {
                let t = match &crate::derive::abilities_of_source(self, *source)[*ability] {
                    Ability::Triggered(t) => t.clone(),
                    other => unreachable!(
                        "a Triggered stack object indexes a Triggered ability, got {other:?}"
                    ),
                };
                let frame = Frame {
                    // [CR#608.2,603.10a]: `~`/`This` is the firing object's
                    // last-known self; the live source may be gone.
                    source: bindings.this.as_ref().map_or(entry.id, |s| s.object),
                    controller: entry.controller,
                    targets: entry.targets.clone(),
                    bindings: Some(bindings.clone()),
                    chosen: None,
                    x: None,
                };
                // [CR#603.4]: an intervening-if is rechecked as the ability
                // resolves. If it no longer holds, the ability is removed from
                // the stack and does nothing (the rule mirrors the illegal-target
                // fizzle) — schedule only the `AbilityResolved` that discards the
                // entry, never the effect.
                if t.condition
                    .as_ref()
                    .is_some_and(|c| !self.condition_holds(c, &frame))
                {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(entry.id),
                    ))]);
                } else {
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Box::new(t.effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                    ]);
                }
            }
            // [CR#602.2a]: an activated ability resolves its carried text,
            // then vanishes like a trigger — no zone move.
            StackObject::Activated {
                ability, bindings, ..
            } => {
                if self.targets_still_legal(&entry) {
                    let this = bindings
                        .this
                        .as_ref()
                        .expect("begin_activate captures the source snapshot unconditionally");
                    let frame = Frame {
                        // [CR#608.2]: `~` is the source's announce-time
                        // snapshot; the live object may be gone.
                        source: this.object,
                        controller: entry.controller,
                        targets: entry.targets.clone(),
                        bindings: Some(bindings.clone()),
                        chosen: None,
                        x: entry.x,
                    };
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Box::new(ability.effect.clone()),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                    ]);
                } else {
                    // [CR#608.2b]: every target illegal — fizzle, vanish.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(entry.id),
                    ))]);
                }
            }
        }
    }

    /// Interpret one `Effect` node ([CR#608.2]). `Act` becomes one or more
    /// `Emit` work items (via `action_items`); `Sequence` expands to one
    /// `RunEffect` per child.
    ///
    /// # Panics
    ///
    /// Panics on any `Effect` variant not wired for Stage 3.
    pub(crate) fn run_effect(&mut self, effect: Effect, frame: &Frame) {
        match effect {
            Effect::Act(action) => {
                if frame.chosen.is_none()
                    && let Some(choice) = unresolved_choice(&action)
                {
                    match choice {
                        PendingChoice::Choose(quantity, filter) => {
                            let candidates = crate::target::candidates(self, &filter);
                            let (min, max) = self.choice_bounds(&quantity, candidates.len(), frame);
                            self.pending = Some(crate::decide::PendingDecision::ChooseObjects {
                                player: frame.controller,
                                candidates,
                                min,
                                max,
                            });
                            self.choice = Some(crate::state::ChoiceContinuation {
                                effect: Box::new(Effect::Act(action)),
                                frame: frame.clone(),
                            });
                            return;
                        }
                        PendingChoice::Random(quantity, filter) => {
                            let mut pool = crate::target::candidates(self, &filter);
                            let k = self.random_count(&quantity, pool.len(), frame);
                            pool.shuffle(&mut self.rng);
                            pool.truncate(k);
                            let mut next = frame.clone();
                            next.chosen = Some(pool);
                            self.run_effect(Effect::Act(action), &next);
                            return;
                        }
                    }
                }
                let items = self.action_items(&action, frame);
                self.schedule_front(items);
            }
            Effect::Sequence(children) => {
                let items: Vec<WorkItem> = children
                    .into_iter()
                    .map(|e| WorkItem::RunEffect {
                        effect: Box::new(e),
                        frame: frame.clone(),
                    })
                    .collect();
                self.schedule_front(items);
            }
            Effect::Continuously(e) => {
                // [CR#611.2]/[CR#611.2c]: stamp at creation; lock the object set
                // for non-floating scopes, leave `Matching` floating.
                let timestamp = self.objects.next_timestamp();
                // P0.W1 seam: only the durations the engine can SWEEP may
                // create instances — a duration with no sweep/tracking would
                // silently last forever.
                match &e.duration {
                    deckmaste_core::Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
                    | deckmaste_core::Duration::EndOfGame => {}
                    other => todo!("P0.W1: duration {other:?} — sweep/tracking unbuilt"),
                }
                if let StaticEffect::Modify { of, changes } = &*e.effect {
                    let scope = match of {
                        Scope::Matching(f) => ScopeResolved::Floating(f.clone()),
                        Scope::Of(r) => ScopeResolved::Locked(vec![self.eval_reference(r, frame)]),
                        Scope::These(rs) => ScopeResolved::Locked(
                            rs.iter().map(|r| self.eval_reference(r, frame)).collect(),
                        ),
                    };
                    self.continuous.push(ContinuousEffect {
                        timestamp,
                        // The continuous effect's controller is the controller
                        // of the spell/ability that created it ([CR#611.2c]);
                        // it resolves the `You` in a layer-2 control change.
                        controller: frame.controller,
                        scope,
                        changes: changes.clone(),
                        duration: e.duration.clone(),
                        is_cda: false,
                    });
                } else {
                    // P0.W1 seam: a granted Deontic/CostModifier/… row would
                    // be silently inert — loud instead.
                    todo!(
                        "P0.W1: Continuously({:?}) — non-Modify grants unbuilt",
                        e.effect
                    );
                }
            }
            // A remembered macro expansion (e.g. an `Effect`-kind macro like
            // `PumpThisUntilEot`) is transparent to resolution — run its value,
            // matching how every other engine layer sees through `*::Expanded`.
            Effect::Expanded(e) => self.run_effect(*e.value, frame),
            // [CR#608.2c,608.2h]: a plain effect "if" has only its normal
            // English meaning ([CR#603.4]) — NOT the intervening-"if" rule, which
            // is only the clause directly after a triggered ability's condition.
            // The controller follows the instructions in written order
            // ([CR#608.2c]), so the condition is read when this node resolves
            // (an earlier sibling's effect — e.g. gaining the city's blessing —
            // is already applied) and the game-state read happens once at that
            // moment ([CR#608.2h]); then the taken branch runs. Direct recursion
            // schedules the branch's items at the front, ahead of any queued
            // sibling, preserving resolution order.
            Effect::If(if_effect) => {
                if self.condition_holds(&if_effect.condition, frame) {
                    self.run_effect(*if_effect.then, frame);
                } else if let Some(otherwise) = if_effect.otherwise {
                    self.run_effect(*otherwise, frame);
                }
            }
            other => todo!("stage 3 does not interpret effect {other:?} (the choice seam)"),
        }
    }

    /// A `ZoneWillChange` intent ([CR#400.7]) moving `object` to `to` from
    /// WHATEVER zone it currently occupies — the current-zone lookup
    /// ([CR#406.2] "from wherever it is") bound at schedule time, with the
    /// `enters` / `position` / `face` coordinates left default (the
    /// bare-relocation case; a battlefield entry's `enters`, a library
    /// insertion's `position`, and a face-down arrival's `face` each need
    /// their own builder). Centralizes the `self.objects.obj(object).zone.
    /// expect(…)` + `…: None` boilerplate the move verbs (`ReturnToHand`,
    /// `Exile`) otherwise repeat.
    pub(crate) fn relocate_from_current(
        &self,
        object: ObjectId,
        to: Zone,
        cause: Option<Cause>,
    ) -> GameEvent {
        GameEvent::ZoneWillChange {
            object,
            from: Some(
                self.objects
                    .obj(object)
                    .zone
                    .expect("relocate a zoned object"),
            ),
            to,
            enters: None,
            position: None,
            face: None,
            cause,
        }
    }

    /// The `Emit` work item(s) a single-instruction `Action` produces. The
    /// source verbs (`DealDamage`, …) act with the source object as agent; the
    /// player verbs live under `By(who, …)`, where `who` resolves to the acting
    /// player and replaces the previously hard-coded `frame.controller`. Damage
    /// to a multi-valued selection is one simultaneous `Batch` (a later task);
    /// drawing N is N sequential `Single`s ([CR#121.2] — drawn one at a time).
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per source verb; splitting would scatter the dispatch"
    )]
    pub(crate) fn action_items(&self, action: &Action, frame: &Frame) -> Vec<WorkItem> {
        match action {
            Action::DealDamage(sel, qty) => {
                let amount = self.eval_count(qty, frame);
                let targets = self.eval_selection_set(sel, frame);
                let events: Vec<GameEvent> = targets
                    .into_iter()
                    .map(|target| GameEvent::DamageDealt {
                        source: frame.source,
                        target,
                        amount,
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.8a]: destroy = battlefield → graveyard, through the
            // replaceable `WillDestroy` intent so indestructible /
            // regeneration can intercede ([CR#702.12b]); its apply commits
            // the zone move when nothing replaces it. The cause names the
            // verb so the "destroyed" named view can narrow by it ([CR#701.8b]
            // — this verb or the lethal-damage SBA are its only two causes).
            Action::Destroy(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::WillDestroy {
                        object,
                        cause: Some(Cause::destroy(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // The named player performs the verb: resolve `who` to the acting
            // player, then dispatch the `PlayerAction`. `By(You, …)` (the
            // implicit-you default) resolves to `frame.controller` — identical
            // to the previous hard-coded behavior.
            Action::By(who, pa) => {
                let actor = self.acting_player(who, frame);
                self.player_action_items(pa, actor, frame)
            }
            // [CR#400.7]: each selected object moves to its owner's hand from
            // whatever zone it's in (bound at schedule time, like `Exile`),
            // becoming a new object the apply pipeline remints. A relocation,
            // not a destruction — no cause-verb fact (the from/to zones the
            // resulting `ZoneChanged` carries are what leave/enter triggers
            // match on).
            Action::ReturnToHand(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| self.relocate_from_current(object, Zone::Hand, None))
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.6a]: countering cancels an object on the stack — it
            // never resolves. A countered SPELL is put into its owner's
            // graveyard (reminted off the stack, [CR#400.7]), cause-tagged
            // "Counter" so a "becomes countered" view can narrow by verb. A
            // countered ABILITY isn't a card and goes nowhere — it simply
            // ceases (remove from stack, no zone move); that arm is the
            // remaining seam. An object already gone from the stack
            // ([CR#608.2b]) is a no-op. Spell is the happy path (ward's verb).
            Action::Counter(sel) => {
                let mut events = Vec::new();
                for object in self.eval_selection_set(sel, frame) {
                    match self
                        .stack
                        .iter()
                        .find(|e| e.id == object)
                        .map(|e| &e.object)
                    {
                        Some(StackObject::Spell(spell)) => {
                            events.push(GameEvent::ZoneWillChange {
                                object: *spell,
                                from: Some(Zone::Stack),
                                to: Zone::Graveyard,
                                enters: None,
                                position: None,
                                face: None,
                                cause: Some(Cause::counter(
                                    Agency::EffectInstruction,
                                    Some((frame.source, frame.controller)),
                                )),
                            });
                        }
                        Some(StackObject::Triggered { .. } | StackObject::Activated { .. }) => {
                            todo!(
                                "kw-ward: countered ability ceases — remove from stack, \
                                 no zone move ([CR#701.6a])"
                            )
                        }
                        None => {}
                    }
                }
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.3a]: attach `what` to each resolved host. This builder is
            // pure (like every `action_items` arm — `Destroy`, `Tap`): it emits
            // the `Attached` fact, and the relation mutation (`attached_to`)
            // happens at that fact's apply ([CR#701.3c] gives the re-attach its
            // new timestamp). No-op — no fact, mirroring the Tap/Untap
            // transition-only idiom ([CR#603.2e]) — on the host it is already
            // on, or a host that is the attachment itself ([CR#303.4d]).
            Action::Attach { what, to } => {
                let hosts = self.eval_selection_set(to, frame);
                let mut events = Vec::new();
                for attachment in self.eval_selection_set(what, frame) {
                    for &host in &hosts {
                        if host == attachment {
                            continue; // [CR#303.4d]: can't attach to itself.
                        }
                        if self.objects.obj(attachment).attached_to == Some(host) {
                            continue; // [CR#701.3a]: already on that host.
                        }
                        // [CR#701.3b]: no-op on an illegal (what, host) pair —
                        // `attachment_legal` reads the `Cant(Attach)` statics
                        // (the attachment's own enchant/Innate restriction + the
                        // host's protection), generically, never the subtype.
                        if !crate::legal::attachment_legal(self, attachment, host) {
                            continue;
                        }
                        events.push(GameEvent::Attached { attachment, host });
                    }
                }
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.3d]: unattach each selected attachment from its host —
            // emit the `Unattached` fact (the `attached_to` clear happens at the
            // fact's apply). No-op (no fact) on an attachment that isn't
            // attached, mirroring the transition-only idiom ([CR#603.2e]).
            Action::Unattach(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .filter_map(|attachment| {
                        self.objects.obj(attachment).attached_to.map(|former_host| {
                            GameEvent::Unattached {
                                attachment,
                                former_host,
                            }
                        })
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#400.7]: a PLAIN zone move (no `WillDestroy` intent, no
            // cause-verb fact) — each selected object moves from whatever zone
            // it's in to `zone`. The apply remints into the owner's
            // graveyard/hand/library (or the shared exile), exactly like
            // `ReturnToHand`/`Exile`. NOT destruction (indestructible doesn't
            // apply) and NOT a sacrifice — the [CR#704.5m] Aura graveyard SBA's
            // mover.
            Action::Move(sel, zone) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::ZoneWillChange {
                        object,
                        from: Some(self.objects.obj(object).zone.expect("move a zoned object")),
                        to: *zone,
                        enters: None,
                        position: None,
                        face: None,
                        cause: None,
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
        }
    }

    /// The `Emit` work item(s) one `PlayerAction` produces, performed by
    /// `actor` (the agent the enclosing `By` resolved to).
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per player verb; splitting would scatter the dispatch"
    )]
    fn player_action_items(
        &self,
        action: &PlayerAction,
        actor: crate::player::PlayerId,
        frame: &Frame,
    ) -> Vec<WorkItem> {
        use crate::event::Occurrence;
        match action {
            PlayerAction::Tap(sel) => {
                // [CR#701.26a]: only an untapped permanent can be tapped — a
                // no-op is no event ([CR#603.2e] "becomes tapped" fires on
                // the transition only).
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .filter(|&object| !self.objects.obj(object).tapped)
                    .map(|object| GameEvent::Tapped {
                        object,
                        cause: Some(Cause::tap(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::Draw(qty) => {
                let n = self.eval_count(qty, frame);
                (0..n)
                    .map(|_| {
                        WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                            player: actor,
                            source: Some(frame.source),
                        }))
                    })
                    .collect()
            }
            PlayerAction::LoseLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                    player: actor,
                    amount,
                }))]
            }
            PlayerAction::GainLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                    player: actor,
                    amount,
                }))]
            }
            PlayerAction::Untap(sel) => {
                // [CR#701.26b]: the mirror of `Tap` above — only a tapped
                // permanent can be untapped, a no-op is no event.
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .filter(|&object| self.objects.obj(object).tapped)
                    .map(GameEvent::Untapped)
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::Sacrifice(sel) => {
                // [CR#701.21a]: the actor moves each selected permanent to its
                // owner's graveyard — the `Sacrificed` verb fact evolves into
                // the zone move at apply. That the selection names permanents
                // the actor controls is the grammar's contract; a legality
                // pass is a later seam.
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::ZoneWillChange {
                        object,
                        from: Some(Zone::Battlefield),
                        to: Zone::Graveyard,
                        enters: None,
                        position: None,
                        // [CR#701.21a]: never a destruction — regeneration
                        // can't replace it; the cause says so.
                        face: None,
                        cause: Some(Cause::sacrifice(
                            Agency::EffectInstruction,
                            Some((frame.source, actor)),
                        )),
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::Exile(sel) => {
                // [CR#701.13a]: move each selected object to exile from
                // whatever zone it's in ([CR#406.2]) — bound at schedule time,
                // like every selection here.
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| self.relocate_from_current(object, Zone::Exile, None))
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::PutInLibrary(sel, pos) => {
                // [CR#401.7]: `pos` indexes from the top (0 = top), clamped to
                // the bottom at apply. [CR#401.4]: the owner's arrangement
                // choice for a multi-card selection is a seam — cards move one
                // at a time in selection order, each inserting at the same
                // index (so the last placed ends up frontmost of the group).
                let position = self.eval_count(pos, frame);
                self.eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| {
                        WorkItem::Emit(Occurrence::Single(GameEvent::ZoneWillChange {
                            object,
                            from: Some(self.objects.obj(object).zone.expect("a zoned object")),
                            to: Zone::Library,
                            enters: None,
                            position: Some(position),
                            face: None,
                            cause: None,
                        }))
                    })
                    .collect()
            }
            // P0.W5 seam: emblem minting into the command zone.
            PlayerAction::GetEmblem(..) => todo!("P0.W5: emblems ([CR#114.1])"),
            // [CR#701.24a]: shuffle the actor's library; the Shuffled
            // apply randomizes via the seeded rng.
            PlayerAction::Shuffle => {
                vec![WorkItem::Emit(Occurrence::single(GameEvent::Shuffled(
                    actor,
                )))]
            }
            // [CR#119.5]: set-to-N resolves as a gain or loss of the
            // difference — triggers see the gain/loss, never a "set";
            // equal totals produce no event (transition-only).
            PlayerAction::SetLife(qty) => {
                let target = deckmaste_core::Int::try_from(self.eval_count(qty, frame))
                    .expect("life total fits in i32");
                let current = self.player(actor).life;
                let event = match target.cmp(&current) {
                    std::cmp::Ordering::Less => GameEvent::LifeLost {
                        player: actor,
                        amount: Uint::try_from(current - target).expect("positive difference"),
                    },
                    std::cmp::Ordering::Greater => GameEvent::LifeGained {
                        player: actor,
                        amount: Uint::try_from(target - current).expect("positive difference"),
                    },
                    std::cmp::Ordering::Equal => return vec![],
                };
                vec![WorkItem::Emit(Occurrence::single(event))]
            }
            // P0.W6 seams: outcome verbs (immediate, gate-checked at the
            // OUTCOME layer — never deontic rows) and reveal/look.
            PlayerAction::WinGame => {
                todo!("P0.W6: win outcome ([CR#104.2b]; CantWin gate check)")
            }
            PlayerAction::LoseGame => {
                todo!("P0.W6: lose outcome ([CR#104.3e]; CantLose gate check)")
            }
            PlayerAction::RestartGame => {
                todo!("P0.W6: restart ([CR#727.1] — a terminal with carryover, not a reset)")
            }
            PlayerAction::Reveal { .. } => {
                todo!("P0.W6: reveal/look (emit Revealed; window lifetime [CR#701.20a])")
            }
            // P0.W4 seams: noted slots (store is P0.W5) and spell copies.
            PlayerAction::ChooseAndNote(..) => {
                todo!("P0.W4: choose-and-note (slot store is P0.W5)")
            }
            PlayerAction::CopySpell(..) => todo!("P0.W4: copy-on-stack ([CR#707.10])"),
            // P0.W3 seams: grammar-complete verbs whose execution is unbuilt.
            PlayerAction::FlipCoins(..) => todo!("P0.W3: coin flips (emit CoinFlipped)"),
            PlayerAction::RollDice(..) => todo!("P0.W3: die rolls (emit DieRolled)"),
            PlayerAction::PutCounters(..) => {
                todo!("P0.W3: counters (emit CounterPlaced; storage is P0.W5)")
            }
            PlayerAction::RemoveCounters(..) => {
                todo!("P0.W3: counters (emit CounterRemoved; storage is P0.W5)")
            }
            PlayerAction::AddMana(qty, production) => {
                let amount = self.eval_count(qty, frame);
                let (spec, mut riders) = match production {
                    deckmaste_core::ManaProduction::Bare(spec) => (spec, Vec::new()),
                    deckmaste_core::ManaProduction::WithRiders { mana, riders } => {
                        (mana, riders.clone())
                    }
                };
                // [CR#107.4h]: mana from a snow source carries `Snow`
                // provenance regardless of the riders the ability text
                // declares — snow-ness is a property of the producing source,
                // not the effect.
                riders.extend(self.snow_provenance(frame.source));
                match spec {
                    // A fixed production needs no choice.
                    ManaSpec::Specific(mana) => {
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::ManaAdded {
                            player: actor,
                            mana: *mana,
                            amount,
                            riders,
                        }))]
                    }
                    // [CR#106.1b]: the actor chooses on resolution — surfaced
                    // explicitly even when only one option exists (engine
                    // policy: every choice surfaces).
                    ManaSpec::AnyColor => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: ANY_COLOR.to_vec(),
                        amount,
                        riders,
                    }],
                    ManaSpec::OneOf(options) => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: options.clone(),
                        amount,
                        riders,
                    }],
                }
            }
            PlayerAction::Discard(qty) => {
                // [CR#701.9b]: the actor chooses which cards — surfaced as a
                // decision when the work item applies (the hand may change
                // before then).
                let count = self.eval_count(qty, frame);
                vec![WorkItem::DiscardCards {
                    player: actor,
                    count,
                }]
            }
            PlayerAction::Create(qty, spec) => {
                // [CR#701.7a]: one instruction puts all N tokens onto the
                // battlefield — one simultaneous batch of `TokenCreated`
                // facts. (Token copies — `Create` of a copy-defined token —
                // wait on the copy grammar, `core-copy-grammar`.) The FACT
                // carries the resolved inline definition; a future
                // `TokenSpec::Named` resolves to one here.
                let deckmaste_core::TokenSpec::Token(token) = spec;
                let n = self.eval_count(qty, frame);
                let events: Vec<GameEvent> = (0..n)
                    .map(|_| GameEvent::TokenCreated {
                        player: actor,
                        token: token.clone(),
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::GetDesignation(name) => {
                // [CR#702.131c]: idempotent — a player who already holds the
                // designation gets no second grant and no fact (so the SBA
                // sweep converges and no spurious "got it" event is recorded).
                if self.designations.players.contains_key(&(actor, *name)) {
                    vec![]
                } else {
                    vec![WorkItem::Emit(Occurrence::Single(
                        GameEvent::GotDesignation {
                            player: actor,
                            name: *name,
                        },
                    ))]
                }
            }
            // Look through a remembered macro invocation.
            PlayerAction::Expanded(e) => self.player_action_items(&e.value, actor, frame),
        }
    }

    /// Resolves the agent of a `By(who, …)` to the acting `PlayerId`. `who` is
    /// a [`Reference`] that resolves (via `eval_reference`) to a player proxy
    /// object; this maps that proxy back to its `PlayerId`.
    ///
    /// # Panics
    ///
    /// Panics if `who` resolves to a non-player object — a player verb's agent
    /// must be a player ([CR#608.2]).
    fn acting_player(&self, who: &Reference, frame: &Frame) -> crate::player::PlayerId {
        use crate::object::ObjectSource;
        let object = self.eval_reference(who, frame);
        match self.objects.get(object).map(|o| o.source) {
            Some(ObjectSource::Player(p)) => p,
            other => panic!("a player verb's agent must be a player, got {other:?}"),
        }
    }

    /// (min, max) objects to choose for `quantity`, clamped to `n` available —
    /// choose as many as able when fewer exist ([CR#608.2d]). Also used by the
    /// cost-payability gate (`can_pay_verbs`) to read a selection's required
    /// floor.
    pub(crate) fn choice_bounds(
        &self,
        quantity: &deckmaste_core::Quantity,
        n: usize,
        frame: &Frame,
    ) -> (Uint, Uint) {
        use deckmaste_core::Quantity;
        let cap = Uint::try_from(n).expect("candidate count fits Uint");
        let ev = |c: &Count| self.eval_count(c, frame).min(cap);
        match quantity {
            Quantity::Exactly(c) => {
                let v = ev(c);
                (v, v)
            }
            Quantity::AtLeast(c) => (ev(c), cap),
            Quantity::AtMost(c) => (0, ev(c)),
            Quantity::Between(lo, hi) => (ev(lo), ev(hi)),
            Quantity::AnyNumber => (0, cap),
            Quantity::Expanded(e) => self.choice_bounds(&e.value, n, frame),
        }
    }

    /// How many objects a `Random` selection picks. v1 supports `Exactly`
    /// (clamped to the candidate count); ranged random is a loud seam.
    fn random_count(&self, quantity: &deckmaste_core::Quantity, n: usize, frame: &Frame) -> usize {
        use deckmaste_core::Quantity;
        match quantity {
            Quantity::Exactly(c) => usize::try_from(self.eval_count(c, frame))
                .expect("count fits usize")
                .min(n),
            Quantity::Expanded(e) => self.random_count(&e.value, n, frame),
            other => todo!(
                "engine-resolve-selections follow-up: ranged Random {other:?} (Exactly only in v1)"
            ),
        }
    }

    /// A selection resolved to its full set ([CR#608.2d]). `Each` is the
    /// distributive "for each matching object" and `Filter` is the same
    /// matching set taken at once — both enumerate here, since a per-object
    /// instruction (deal damage, destroy) applies to every member either way.
    /// Unary references resolve to a 1-element set.
    pub(crate) fn eval_selection_set(&self, sel: &Selection, frame: &Frame) -> Vec<ObjectId> {
        match sel {
            Selection::Each(f) | Selection::Filter(f) => crate::target::candidates(self, f),
            // The chooser's/RNG's picks, bound into the frame before the action
            // re-runs ([CR#608.2d]).
            Selection::Choose(..) | Selection::Random(..) => frame
                .chosen
                .clone()
                .expect("a Choose/Random selection is bound into the frame before its action runs"),
            Selection::Expanded(e) => self.eval_selection_set(&e.value, frame),
            other => vec![self.eval_selection(other, frame)],
        }
    }

    /// Resolve a unary `Selection` to an `ObjectId` ([CR#608.2d] / references).
    ///
    /// # Panics
    ///
    /// Panics on a `Selection` not wired for Stage 3, or an out-of-range
    /// `Target(n)` index.
    fn eval_selection(&self, sel: &Selection, frame: &Frame) -> ObjectId {
        match sel {
            // A bound reference lifted into a choice slot: funnel to the
            // reference resolver.
            Selection::Ref(reference) => self.eval_reference(reference, frame),
            other => todo!("stage 3 does not evaluate selection {other:?}"),
        }
    }

    /// Resolve a [`Reference`] to an `ObjectId` (the bound-object resolver
    /// `Selection::Ref` funnels through).
    ///
    /// # Panics
    ///
    /// Panics on a `Reference` not wired for Stage 3, an out-of-range
    /// `Target(n)` index, or an `AttachHostOf`/`AttachedTo` over an
    /// attachment/host with no live link (the reference is only well-defined
    /// where the relation is established).
    pub(crate) fn eval_reference(&self, reference: &Reference, frame: &Frame) -> ObjectId {
        match reference {
            Reference::Target(n) => *frame
                .targets
                .get(*n)
                .expect("announced target index in bounds"),
            // [CR#603.10a]: for a triggered ability, `~`/`This` is the firing
            // object's last-known self (the live source may be gone); for a
            // spell frame (no bindings) it is the live source.
            Reference::This => frame
                .bindings
                .as_ref()
                .and_then(|b| b.this.as_ref())
                .map_or(frame.source, |s| s.object),
            Reference::You => self.player(frame.controller).object,
            // [CR#603.10a]: the trigger's moved object / affected player,
            // read from the bindings the fired trigger carried.
            Reference::ThatObject => frame
                .bindings
                .as_ref()
                .and_then(|b| b.that_object.as_ref())
                .map(|s| s.object)
                .expect("ThatObject referenced where the trigger bound one"),
            Reference::ThatPlayer => {
                let p = frame
                    .bindings
                    .as_ref()
                    .and_then(|b| b.that_player)
                    .expect("ThatPlayer referenced where the trigger bound one");
                self.player(p).object
            }
            // [CR#109.5]: the derived controller of a referenced object.
            Reference::ControllerOf(inner) => {
                let id = self.eval_reference(inner, frame);
                self.player(self.layers().controller(id)).object
            }
            // [CR#108.3]: the owner of a referenced (card-backed) object.
            Reference::OwnerOf(inner) => {
                let id = self.eval_reference(inner, frame);
                self.player(self.owner_of(id)).object
            }
            // Look through a remembered macro invocation.
            Reference::Expanded(e) => self.eval_reference(&e.value, frame),
            // engine-resolve-selections follow-ups: these need stores that do
            // not exist yet (see the filed tickets).
            Reference::Bound(ident) => todo!(
                "engine-bound-references: Bound({ident:?}) needs a named-role binding store ([CR#608.2])"
            ),
            Reference::Linked(ident) => todo!(
                "engine-linked-abilities: Linked({ident:?}) needs a linked-ability store ([CR#607])"
            ),
            // [CR#301.5,303.4]: the host an attachment is attached to — read
            // the attachment→host relation directly off the resolved object.
            Reference::AttachHostOf(inner) => {
                let id = self.eval_reference(inner, frame);
                self.objects
                    .obj(id)
                    .attached_to
                    .expect("AttachHostOf referenced an unattached object")
            }
            // The inverse (host→attachment): scan for the object whose
            // `attached_to` points at the resolved host. v1 single attachment —
            // returns the first (deterministic id order, [CR#613.7]); the
            // multiple-attachment fan-out is `Filter::Attachment` territory.
            Reference::AttachedTo(inner) => {
                let host = self.eval_reference(inner, frame);
                self.objects
                    .iter()
                    .find(|o| o.attached_to == Some(host))
                    .map(|o| o.id)
                    .expect("AttachedTo referenced a host with no attachment")
            }
        }
    }

    /// Evaluate a `Count` to a concrete number.
    ///
    /// # Panics
    ///
    /// Panics on a `Count` not wired for Stage 3, on a `StatOf` whose object
    /// lacks the stat, and on a `ThatMuch` with no amount fixed in this
    /// resolution.
    pub(crate) fn eval_count(&self, qty: &Count, frame: &Frame) -> Uint {
        match qty {
            Count::Literal(n) => *n,
            // "For each …": the filter's live cardinality over every object
            // (card objects in all zones + player proxies) — canonical
            // filters are context-free-correct, so they carry their own
            // zone/kind narrowing. The watcher anchors `Ref(This)` to the
            // frame's announce-time self, the way `eval_reference` does.
            Count::CountOf(filter) => {
                let watcher = self.frame_watcher(frame);
                let n = self
                    .objects
                    .iter()
                    .filter(|ob| self.filter_matches_live(filter, ob.id, watcher))
                    .count();
                Uint::try_from(n).expect("object count fits Uint")
            }
            // "Equal to its power": resolve the reference, read the DERIVED
            // stat off the layer view ([CR#613]; per-call rebuild — the same
            // documented perf seam as `target::matches`'s `Has` arm). A
            // negative result counts as 0 ([CR#107.1b]).
            Count::StatOf(reference, stat) => {
                let id = self.eval_reference(reference, frame);
                let value = match stat {
                    deckmaste_core::Stat::Power => self
                        .layers()
                        .power(id)
                        .expect("StatOf(Power) on an object with a power"),
                    deckmaste_core::Stat::Toughness => self
                        .layers()
                        .toughness(id)
                        .expect("StatOf(Toughness) on an object with a toughness"),
                    // [CR#202.3]: the printed cost's total. The on-stack
                    // announced-X contribution ([CR#202.3e]) rides the
                    // announce-slot X work (see `Count::X` below).
                    deckmaste_core::Stat::ManaValue => {
                        let face = crate::derive::face(self.def(id));
                        deckmaste_core::Int::try_from(face.mana_cost.mana_value())
                            .expect("mana value fits Int")
                    }
                    deckmaste_core::Stat::Loyalty | deckmaste_core::Stat::Defense => todo!(
                        "engine-resolve-counts: {stat:?} reads (planeswalker/battle counter \
                         machinery unbuilt)"
                    ),
                };
                Uint::try_from(value.max(0)).expect("clamped stat fits Uint")
            }
            // The amount fixed by an earlier instruction of this resolution —
            // recorded at the apply funnel (so it reads what actually
            // happened, post-replacement), cleared by `resolve_object`. A
            // trigger-bound magnitude ("whenever you gain life, … that much")
            // must instead ride `TriggerBindings`, which the trigger-events
            // lane owns — loud until that lands.
            Count::ThatMuch => self.that_much.unwrap_or_else(|| {
                todo!(
                    "ThatMuch with no amount fixed this resolution — trigger-bound magnitudes \
                     are the engine-trigger-events bindings seam"
                )
            }),
            // [CR#107.3a]: while a spell/ability is on the stack, X equals the
            // value announced as it was cast (engine-x-costs threads it onto the
            // resolution frame). [CR#107.3f] text-X chosen at resolution is a
            // separate seam.
            Count::X => frame.x.expect(
                "Count::X on a frame with no announced X — a card referenced X without an {X} cost",
            ),
            // [CR#608.2i] history reads off the log — the evaluating player is
            // the frame's controller.
            Count::Query(key) => self.eval_query(*key, frame.controller),
            Count::Noted(key) => todo!("P0.W4: noted read {key:?} (slot store is P0.W5)"),
            Count::Expanded(e) => self.eval_count(&e.value, frame),
        }
    }

    /// Derive an engine-tracked history scalar from the log ([CR#608.2i]).
    /// `controller` is the evaluating player (frame controller, or the
    /// condition's "you"). Storm is game-wide; the rest are per-`controller`.
    ///
    /// # Panics
    ///
    /// Panics only if a this-turn count exceeds `Uint` — unreachable in a real
    /// game.
    #[must_use]
    pub fn eval_query(
        &self,
        key: deckmaste_core::QueryKey,
        controller: crate::player::PlayerId,
    ) -> Uint {
        use deckmaste_core::QueryKey;
        use deckmaste_core::Window;
        use deckmaste_core::Zone;

        use crate::event::GameEvent;
        let turn = self.turn.turn_number;
        match key {
            // Storm ([CR#702.40a]): spells cast before this one this turn =
            // all spells cast this turn (game-wide) minus the spell itself.
            // The minus-one assumes the storm spell's own `SpellCast` is in the
            // log (true when storm reads it on the stack); spells cast in
            // response to the storm trigger would over-count — exact "before
            // it" capture at cast time is a trigger-bindings follow-up.
            QueryKey::StormCount => {
                let total = self
                    .history
                    .scan(Window::ThisTurn, turn)
                    .filter(|f| matches!(f, GameEvent::SpellCast(_)))
                    .count();
                Uint::try_from(total.saturating_sub(1)).expect("storm count fits Uint")
            }
            QueryKey::CardsDrawnThisTurn => {
                let n = self
                    .history
                    .scan(Window::ThisTurn, turn)
                    .filter(|f| matches!(f, GameEvent::WillDraw { player, .. } if *player == controller))
                    .count();
                Uint::try_from(n).expect("draw count fits Uint")
            }
            QueryKey::LandsPlayedThisTurn => {
                let play = deckmaste_core::Ident::from("Play");
                let n = self
                    .history
                    .scan(Window::ThisTurn, turn)
                    .filter(|f| {
                        matches!(f,
                            GameEvent::ZoneChanged { to: Zone::Battlefield, cause: Some(c), snapshot, .. }
                                if c.verb == play && snapshot.controller == controller)
                    })
                    .count();
                Uint::try_from(n).expect("land count fits Uint")
            }
            QueryKey::LifeLostThisTurn => self
                .history
                .scan(Window::ThisTurn, turn)
                .filter_map(|f| match f {
                    GameEvent::LifeLost { player, amount } if *player == controller => {
                        Some(*amount)
                    }
                    _ => None,
                })
                .sum(),
            QueryKey::LifeGainedThisTurn => self
                .history
                .scan(Window::ThisTurn, turn)
                .filter_map(|f| match f {
                    GameEvent::LifeGained { player, amount } if *player == controller => {
                        Some(*amount)
                    }
                    _ => None,
                })
                .sum(),
        }
    }

    /// The `ObjectSource` that anchors `Ref(This)`/`Ref(You)` in live filter
    /// evaluation for `frame`: the announce-time snapshot's source when the
    /// frame carries bindings (the live object may be gone, [CR#603.10a]),
    /// else the live source object's.
    pub(crate) fn frame_watcher(&self, frame: &Frame) -> crate::object::ObjectSource {
        frame
            .bindings
            .as_ref()
            .and_then(|b| b.this.as_ref())
            .map_or_else(|| self.objects.obj(frame.source).source, |s| s.source)
    }

    /// True iff the card's printed types include a permanent type
    /// (Creature/Artifact/Enchantment/Land/Planeswalker/Battle) and NOT
    /// Instant or Sorcery.
    ///
    /// [CR#110.1]: a permanent spell is one that would enter the battlefield on
    /// resolution. Grizzly Bears → true; Instant `DealDamage` `AnyTarget` →
    /// false.
    #[must_use]
    pub(crate) fn is_permanent_spell(&self, id: ObjectId) -> bool {
        let types = &crate::derive::face(self.def(id)).types;
        let is_permanent_type = types.iter().any(|t| {
            matches!(
                t,
                Type::Creature
                    | Type::Artifact
                    | Type::Enchantment
                    | Type::Land
                    | Type::Planeswalker
                    | Type::Battle
            )
        });
        let is_non_permanent = types
            .iter()
            .any(|t| matches!(t, Type::Instant | Type::Sorcery));
        is_permanent_type && !is_non_permanent
    }

    /// Returns the effect of the spell's first `Ability::Spell(SpellAbility {
    /// effect, .. })`, cloned. Looks through `Ability::Expanded` the way
    /// `derive::tap_mana_ability` does. Returns `None` if there is no Spell
    /// ability.
    #[must_use]
    pub(crate) fn spell_effect(&self, id: ObjectId) -> Option<Effect> {
        crate::derive::abilities(self, id)
            .iter()
            .find_map(|a| spell_ability_effect(a))
            .cloned()
    }

    /// [CR#608.2b]: for each chosen target, it still matches its `TargetSpec`'s
    /// filter. Returns `true` if all chosen targets are still legal (or there
    /// are no targets). Stage 2: single target, so "all legal" == "the one
    /// target legal". A spell's specs derive from its `Spell` ability; an
    /// activated ability's ride the carried text ([CR#602.2b]).
    ///
    /// **Announce invariant**: the zip assumes one chosen target per
    /// `TargetSpec` — exactly what the Stage-2 announce flow guarantees. If
    /// you add multi-target targeting, update both sides of the zip.
    ///
    /// # Panics
    ///
    /// Panics on a `Triggered` entry (its resolve arm does not re-check
    /// target legality yet — the trigger-fizzle seam), and on `TargetSpec`
    /// variants other than `Target` or `Expanded` — only single-target
    /// `Target(_, _)` is wired (multi-target is Stage 4).
    #[must_use]
    pub(crate) fn targets_still_legal(&self, entry: &StackEntry) -> bool {
        let specs: Vec<TargetSpec> = match &entry.object {
            StackObject::Spell(o) => spell_targets(&self.layers(), *o),
            // The carried text is authoritative — never re-derive from the
            // (possibly gone, possibly changed) source.
            StackObject::Activated { ability, .. } => ability.targets.clone(),
            StackObject::Triggered { .. } => unreachable!(
                "the Triggered resolve arm does not re-check target legality (fizzle seam)"
            ),
        };
        debug_assert_eq!(
            specs.len(),
            entry.targets.len(),
            "announce fills exactly one chosen target per TargetSpec",
        );
        // [CR#608.2b] re-checks the same Cant(Target) rows the announce
        // evaluated — a hexproof granted after announce fizzles the spell.
        let view = self.layers();
        let rows = crate::legal::cant_target_rows(self, &view);
        specs.iter().zip(&entry.targets).all(|(spec, &chosen)| {
            // [CR#608.2b]: a target that no longer exists (reminted on zone
            // change) is trivially illegal — the filter can't be satisfied.
            if self.objects.get(chosen).is_none() {
                return false;
            }
            let filter = target_spec_filter(spec);
            crate::target::matches(self, chosen, filter)
                && crate::legal::target_forbidden_by(self, &rows, entry.id, chosen).is_none()
        })
    }
}

/// A `Choose`/`Random` selection lifted out of an action, owned so the action
/// can be moved into a continuation afterward. v1 assumes <=1 per effect node;
/// grammar gives one selection slot per action.
enum PendingChoice {
    Choose(deckmaste_core::Quantity, deckmaste_core::Filter),
    Random(deckmaste_core::Quantity, deckmaste_core::Filter),
}

/// The lone unresolved `Choose`/`Random` selection in `action` (looking through
/// `Expanded`), cloned out. `Attach`'s two slots are both refs/targets in the
/// keyword macros (never `Choose`/`Random`), so it stays `None`; `Unattach`'s
/// single selection lifts like the other one-slot verbs.
fn unresolved_choice(action: &Action) -> Option<PendingChoice> {
    fn lift(sel: &Selection) -> Option<PendingChoice> {
        match sel {
            Selection::Choose(q, f) => Some(PendingChoice::Choose(q.clone(), f.clone())),
            Selection::Random(q, f) => Some(PendingChoice::Random(q.clone(), f.clone())),
            Selection::Expanded(e) => lift(&e.value),
            _ => None,
        }
    }
    fn lift_pa(pa: &PlayerAction) -> Option<PendingChoice> {
        match pa {
            PlayerAction::Sacrifice(s)
            | PlayerAction::Exile(s)
            | PlayerAction::Tap(s)
            | PlayerAction::Untap(s)
            | PlayerAction::CopySpell(s)
            | PlayerAction::PutCounters(s, _, _)
            | PlayerAction::RemoveCounters(s, _, _)
            | PlayerAction::PutInLibrary(s, _) => lift(s),
            PlayerAction::Reveal { what, .. } => lift(what),
            PlayerAction::Expanded(e) => lift_pa(&e.value),
            _ => None,
        }
    }
    match action {
        Action::DealDamage(s, _)
        | Action::Destroy(s)
        | Action::ReturnToHand(s)
        | Action::Counter(s)
        | Action::Unattach(s)
        | Action::Move(s, _) => lift(s),
        Action::By(_, p) => lift_pa(p),
        Action::Attach { .. } => None,
    }
}

/// The `SpellAbility.targets` of the spell (empty for permanent spells).
/// Used by the cast checks and `targets_still_legal`. Reads the caller's
/// derived view — the legality loop checks every hand card against one
/// view instead of re-deriving the board per card.
#[must_use]
pub(crate) fn spell_targets(view: &crate::layer::LayeredView, id: ObjectId) -> Vec<TargetSpec> {
    view.get(id)
        .abilities
        .iter()
        .find_map(|a| spell_targets_list(a))
        .cloned()
        .unwrap_or_default()
}

/// Extracts the `Effect` from the first `Ability::Spell` arm, looking through
/// `Ability::Expanded`.
fn spell_ability_effect(ability: &Ability) -> Option<&Effect> {
    match ability {
        Ability::Spell(s) => Some(&s.effect),
        Ability::Expanded(e) => spell_ability_effect(&e.value),
        _ => None,
    }
}

/// Extracts the `targets` list from the first `Ability::Spell` arm, looking
/// through `Ability::Expanded`.
fn spell_targets_list(ability: &Ability) -> Option<&Vec<TargetSpec>> {
    match ability {
        Ability::Spell(s) => Some(&s.targets),
        Ability::Expanded(e) => spell_targets_list(&e.value),
        _ => None,
    }
}

/// One event → `Single`; several → a simultaneous `Batch`.
fn occurrence_of(mut events: Vec<GameEvent>) -> crate::event::Occurrence {
    use crate::event::Occurrence;
    if events.len() == 1 {
        Occurrence::Single(events.pop().expect("len 1"))
    } else {
        Occurrence::Batch(events)
    }
}

/// Extracts the `Filter` from a `TargetSpec`. Stage 3 only handles
/// `TargetSpec::Target(Exactly(Literal(1)), filter)` (and `Expanded` wrappers
/// around it).
///
/// This is the single authoritative site for TargetSpec→Filter extraction;
/// both `cast::legal_targets` (announce time) and `targets_still_legal`
/// (resolution time) funnel through here so they stay in sync.
///
/// # Panics
///
/// Panics on `TargetSpec` quantities not wired for Stage 3.
pub(crate) fn target_spec_filter(spec: &TargetSpec) -> &deckmaste_core::Filter {
    match spec {
        TargetSpec::Target(_quantity, f) => {
            // TODO(stage-4): enforce quantity; for now, Stage 3 only exercises
            // single targets and callers expect exactly one target slot.
            f
        }
        // P0.W7 seam: the distinctness CONSTRAINT is unenforced — a spec
        // carrying one must trip loudly, not silently target-overlap
        // (final-set semantics [CR#115.7e]; checked at announce [CR#601.2c]
        // and at the [CR#608.2b] re-check).
        TargetSpec::Distinct(..) => {
            todo!("P0.W7: co-target distinctness enforcement ([CR#115.7e])")
        }
        TargetSpec::Expanded(e) => target_spec_filter(&e.value),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Action;
    use deckmaste_core::Card;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Count;
    use deckmaste_core::Effect;
    use deckmaste_core::Filter;
    use deckmaste_core::ObjectKind;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StateFilter;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::player::PlayerId;
    use crate::stack::Frame;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::Progress;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    }

    /// Pulls a second creature out of player 0's opening hand, drops it onto
    /// the battlefield, and hands it to player 1 — owner stays player 0,
    /// controller becomes player 1. Returns the object so tests can read
    /// both sides.
    fn second_bear_to_player_1(state: &mut GameState) -> ObjectId {
        let theirs = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != theirs);
        state.objects.obj_mut(theirs).zone = Some(Zone::Battlefield);
        state.objects.obj_mut(theirs).controller = PlayerId(1);
        state.zones.battlefield.push(theirs);
        theirs
    }

    /// `eval_query` derives every history scalar off the log: storm is the
    /// game-wide spell count this turn minus the spell itself ([CR#702.40a]);
    /// lands by the `Play` cause; draws by the `WillDraw` fact; life by summed
    /// amounts ([CR#119.3]).
    #[test]
    fn eval_query_reads_history_scalars() {
        use deckmaste_core::Agency;
        use deckmaste_core::QueryKey;

        use crate::event::Cause;
        use crate::lki::LkiSnapshot;
        use crate::object::ObjectSource;

        let mut state = game();
        state.turn.turn_number = 1;
        let p = PlayerId(0);

        // Three spells cast this turn (game-wide) → storm = 3 - 1 = 2.
        state
            .history
            .record(1, GameEvent::SpellCast(ObjectId::from_raw(1)));
        state
            .history
            .record(1, GameEvent::SpellCast(ObjectId::from_raw(2)));
        state
            .history
            .record(1, GameEvent::SpellCast(ObjectId::from_raw(3)));
        assert_eq!(state.eval_query(QueryKey::StormCount, p), 2);

        // Two draws by p this turn → CardsDrawn = 2.
        state.history.record(
            1,
            GameEvent::WillDraw {
                player: p,
                source: None,
            },
        );
        state.history.record(
            1,
            GameEvent::WillDraw {
                player: p,
                source: None,
            },
        );
        assert_eq!(state.eval_query(QueryKey::CardsDrawnThisTurn, p), 2);

        // One land played by p (a Play-caused battlefield entry) → Lands = 1.
        let land = state
            .objects
            .mint(ObjectSource::Player(p), p, Some(Zone::Battlefield));
        state.history.record(
            1,
            GameEvent::ZoneChanged {
                snapshot: LkiSnapshot::capture(&state, land),
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                face: None,
                cause: Some(Cause {
                    verb: "Play".into(),
                    agency: Agency::SpecialAction,
                    agent: None,
                }),
            },
        );
        assert_eq!(state.eval_query(QueryKey::LandsPlayedThisTurn, p), 1);

        // Life: lost 3 then 2 (=5), gained 4.
        state.history.record(
            1,
            GameEvent::LifeLost {
                player: p,
                amount: 3,
            },
        );
        state.history.record(
            1,
            GameEvent::LifeLost {
                player: p,
                amount: 2,
            },
        );
        state.history.record(
            1,
            GameEvent::LifeGained {
                player: p,
                amount: 4,
            },
        );
        assert_eq!(state.eval_query(QueryKey::LifeLostThisTurn, p), 5);
        assert_eq!(state.eval_query(QueryKey::LifeGainedThisTurn, p), 4);

        // Prior-turn entries are excluded once the turn advances.
        state.turn.turn_number = 2;
        assert_eq!(state.eval_query(QueryKey::StormCount, p), 0);
        assert_eq!(state.eval_query(QueryKey::CardsDrawnThisTurn, p), 0);
    }

    /// `Action::By(You, pa)` — the implicit-you default a bare player verb
    /// reads as.
    fn by_you(pa: PlayerAction) -> Action { Action::By(Reference::You, pa) }

    /// `Selection::Ref(This)` — the common "this object" selection.
    fn sel_this() -> Selection { Selection::Ref(Reference::This) }

    /// A two-player game; player 0's deck is Grizzly Bears.
    /// Returns the state plus a creature object forced onto the battlefield.
    fn bear_on_field() -> (GameState, ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&bears, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// A two-player game with player 0's deck = Darksteel Myr (an
    /// indestructible 0/1), one forced onto the battlefield.
    fn myr_on_field() -> (GameState, ObjectId) {
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
        });
        let m = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("a Darksteel Myr in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
        state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(m);
        (state, m)
    }

    /// Two Grizzly Bears (player 0) forced onto the battlefield — `(a, b)`.
    /// The attachment subsystem is type-agnostic in Stage 1 ([CR#701.3b]'s
    /// type-based attachability is Task 4.x), so two bare permanents exercise
    /// the relation/verb/event mechanism directly.
    fn two_permanents_on_field() -> (GameState, ObjectId, ObjectId) {
        let (mut state, a) = bear_on_field();
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);
        (state, a, b)
    }

    /// Drives the agenda forward a bounded number of steps — assert on the
    /// post-condition, not the iteration count. A pending decision stops it.
    fn drain(state: &mut GameState) {
        for _ in 0..30 {
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
    }

    /// Process exactly a test-injected effect's front-scheduled work, WITHOUT
    /// advancing the turn structure. `run_effect` schedules its work
    /// (`Emit`/`RunEffect`/…) at the front of the agenda; this steps while the
    /// front item is such injected work and stops the moment a turn-structure
    /// item (`BeginStep`/`CheckSbas`/`OpenPriority`/…) or a decision would run.
    /// A test-only driver for sequentially-injected effects: it never parks a
    /// priority (so the next `run_effect`'s front work isn't blocked) and never
    /// drains the turn loop dry.
    fn run_injected(state: &mut GameState) {
        for _ in 0..30 {
            let injected = matches!(
                state.agenda.front(),
                Some(WorkItem::Emit(_) | WorkItem::RunEffect { .. } | WorkItem::Resolve(_))
            );
            if !injected || state.pending.is_some() {
                return;
            }
            let _ = state.step();
        }
    }

    /// Whether the history log holds a fact matching `pred` (game-wide).
    fn logged(state: &GameState, pred: impl Fn(&GameEvent) -> bool) -> bool {
        state
            .history
            .scan(deckmaste_core::Window::ThisGame, state.turn.turn_number)
            .any(pred)
    }

    /// [CR#701.3a]: `Attach` sets the attachment→host relation and records the
    /// `Attached` fact.
    #[test]
    fn attach_sets_the_relation_and_emits_attached() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(a).attached_to,
            Some(b),
            "a is attached to b"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Attached { attachment, host }
                if *attachment == a && *host == b)
            ),
            "Attached fact recorded"
        );
    }

    /// [CR#701.3a]: attaching to the host it is already on is a no-op — no
    /// second `Attached` fact (transition-only, [CR#603.2e]).
    #[test]
    fn attach_to_current_host_is_a_noop() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, Some(b));
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact for a re-attach to the current host"
        );
    }

    /// [CR#303.4d]: an attachment can't be attached to itself — a no-op.
    #[test]
    fn attach_to_self_is_a_noop() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, None, "host == what no-op");
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact for a self-attach"
        );
    }

    /// [CR#701.3b]: `Attach` no-ops on an illegal host — the attachment carries
    /// a conferred `Innate(Cant(Attach(what: Ref(This), to: Not(Creature))))`
    /// (the Equipment-subtype shape) and the host is a non-creature, so the
    /// link stays `None` and no `Attached` fact is recorded.
    #[test]
    fn attach_illegal_noop() {
        use deckmaste_core::Ability;
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::StaticAbility;
        use deckmaste_core::StaticEffect;

        use crate::object::ObjectSource;

        let mut state = game();
        // The attachment: an artifact whose Innate rule forbids non-creature
        // hosts (mirrors the Equipment subtype confer).
        let equip_card = Card::Normal(CardFace {
            name: "Test Equipment".into(),
            types: vec![Type::Artifact],
            abilities: vec![Ability::Innate(Box::new(Ability::Static(StaticAbility {
                condition: None,
                effects: vec![StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Attach {
                        what: Filter::Ref(Reference::This),
                        to: Filter::Not(Box::new(Filter::Characteristic(
                            CharacteristicFilter::Type(Type::Creature),
                        ))),
                    },
                ))],
                characteristic_defining: false,
            })))],
            ..CardFace::default()
        });
        let equip_id = state.cards.push(Arc::new(equip_card), PlayerId(0));
        let equip = state.objects.mint(
            ObjectSource::Card(equip_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(equip);

        // The host: a non-creature artifact "Rock".
        let rock_card = Card::Normal(CardFace {
            name: "Rock".into(),
            types: vec![Type::Artifact],
            ..CardFace::default()
        });
        let rock_id = state.cards.push(Arc::new(rock_card), PlayerId(0));
        let rock = state.objects.mint(
            ObjectSource::Card(rock_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(rock);

        let frame = frame_src_targets(equip, vec![rock]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(equip).attached_to,
            None,
            "illegal attach no-ops ([CR#701.3b])"
        );
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact for an illegal host"
        );
    }

    /// [CR#701.3d]: `Unattach` clears the relation and records the `Unattached`
    /// fact carrying the former host.
    #[test]
    fn unattach_clears_the_relation_and_emits_unattached() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);
        let frame = frame_src(a);
        state.run_effect(
            Effect::Act(Action::Unattach(Selection::Ref(Reference::This))),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(a).attached_to,
            None,
            "a is now unattached"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Unattached { attachment, former_host }
                if *attachment == a && *former_host == b)
            ),
            "Unattached fact records the former host"
        );
    }

    /// [CR#701.3d]: unattaching an attachment that isn't attached is a no-op —
    /// no `Unattached` fact (transition-only, [CR#603.2e]).
    #[test]
    fn unattach_of_an_unattached_object_is_a_noop() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src(a);
        state.run_effect(
            Effect::Act(Action::Unattach(Selection::Ref(Reference::This))),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, None);
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Unattached { .. })),
            "no Unattached fact for an already-unattached object"
        );
    }

    /// [CR#301.5]: `AttachHostOf(This)` from the attachment resolves to its
    /// host; `AttachedTo(This)` from the host resolves to the attachment (the
    /// derived inverse).
    #[test]
    fn eval_reference_attach_host_and_inverse() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);

        let frame_a = frame_src(a);
        assert_eq!(
            state.eval_reference(
                &Reference::AttachHostOf(Box::new(Reference::This)),
                &frame_a
            ),
            b,
            "AttachHostOf(This) from a is its host b"
        );

        let frame_b = frame_src(b);
        assert_eq!(
            state.eval_reference(&Reference::AttachedTo(Box::new(Reference::This)), &frame_b),
            a,
            "AttachedTo(This) from b is the attachment a (inverse by scan)"
        );
    }

    /// `eval_selection_set` returns the bound set for a `Choose`/`Random` slot
    /// (the value the decision/RNG wrote into the frame), instead of surfacing.
    #[test]
    fn eval_selection_set_reads_bound_choice() {
        use deckmaste_core::Quantity;

        let (state, bear) = bear_on_field();
        let creatures = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        ]);
        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
            chosen: Some(vec![bear]),
            x: None,
        };
        let sel = Selection::Choose(Quantity::Exactly(Count::Literal(1)), creatures);
        assert_eq!(state.eval_selection_set(&sel, &frame), vec![bear]);
    }

    /// `Destroy(Random(Exactly 1, creature))` resolves via the seeded RNG —
    /// exactly one creature is destroyed and NO decision is surfaced.
    #[test]
    fn destroy_random_destroys_one_without_a_decision() {
        use deckmaste_core::Quantity;

        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        let creatures = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        ]);
        let frame = frame_src(bear);
        let before = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(before, 2);

        state.run_effect(
            Effect::Act(Action::Destroy(Selection::Random(
                Quantity::Exactly(Count::Literal(1)),
                creatures,
            ))),
            &frame,
        );
        // No decision: Random is resolved inline by the RNG.
        assert!(
            !matches!(state.step(), StepOutcome::NeedsDecision(_)),
            "Random surfaces no decision"
        );
        // Pump the agenda to completion (bounded safety cap; assert on the
        // post-condition, not the iteration count).
        for _ in 0..30 {
            let alive = [bear, theirs]
                .iter()
                .filter(|o| state.zones.battlefield.contains(o))
                .count();
            if alive == 1 {
                break;
            }
            let _ = state.step();
        }
        let alive = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(alive, 1, "exactly one creature destroyed at random");
    }

    /// `Destroy(Choose(Exactly 1, creature))` surfaces `ChooseObjects`; an
    /// out-of-range count and an out-of-pool object are rejected; a legal pick
    /// destroys exactly that creature ([CR#608.2d]).
    #[test]
    fn destroy_choose_surfaces_decision_validates_and_destroys() {
        use deckmaste_core::Quantity;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        let creatures = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            Effect::Act(Action::Destroy(Selection::Choose(
                Quantity::Exactly(Count::Literal(1)),
                creatures,
            ))),
            &frame,
        );

        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.step()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!((min, max), (1, 1));
        assert_eq!(
            candidates.len(),
            2,
            "both battlefield creatures are candidates"
        );

        // Too many (count 2 > max 1).
        assert!(
            state
                .submit_decision(Decision::Chosen(candidates.clone()))
                .is_err(),
            "count must be within [min, max]"
        );
        // Out of pool (a player proxy is not a creature).
        assert!(
            state
                .submit_decision(Decision::Chosen(vec![state.player(PlayerId(0)).object]))
                .is_err(),
            "every chosen object must be a candidate"
        );

        // Legal: destroy player 1's creature.
        state
            .submit_decision(Decision::Chosen(vec![theirs]))
            .unwrap();
        // Pump the agenda to completion (bounded safety cap; we assert on the
        // post-condition, not the iteration count).
        for _ in 0..30 {
            if !state.zones.battlefield.contains(&theirs) {
                break;
            }
            let _ = state.step();
        }
        assert!(
            !state.zones.battlefield.contains(&theirs),
            "the chosen creature is destroyed"
        );
        assert!(
            state.zones.battlefield.contains(&bear),
            "the unchosen creature survives"
        );
    }

    /// The buildable-now references: `ControllerOf`/`OwnerOf` distinguish
    /// control from ownership ([CR#109.5,108.3]); `ThatObject`/`ThatPlayer`
    /// read the trigger bindings ([CR#603.10a]).
    #[test]
    fn references_resolve_controller_owner_and_trigger_bindings() {
        let (mut state, bear) = bear_on_field();
        // A second Grizzly Bears from player 0's hand onto the battlefield, then
        // handed to player 1: owner stays player 0, controller becomes player 1.
        let theirs = second_bear_to_player_1(&mut state);

        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: vec![theirs],
            bindings: Some(crate::trigger::TriggerBindings {
                this: Some(crate::lki::LkiSnapshot::capture(&state, bear)),
                that_object: Some(crate::lki::LkiSnapshot::capture(&state, theirs)),
                that_player: Some(PlayerId(1)),
            }),
            chosen: None,
            x: None,
        };

        assert_eq!(
            state.eval_reference(
                &Reference::ControllerOf(Box::new(Reference::Target(0))),
                &frame
            ),
            state.player(PlayerId(1)).object,
            "controller of player 1's creature is player 1"
        );
        assert_eq!(
            state.eval_reference(&Reference::OwnerOf(Box::new(Reference::Target(0))), &frame),
            state.player(PlayerId(0)).object,
            "owner is still player 0"
        );
        assert_eq!(
            state.eval_reference(&Reference::ThatObject, &frame),
            theirs,
            "ThatObject is the bound snapshot's object"
        );
        assert_eq!(
            state.eval_reference(&Reference::ThatPlayer, &frame),
            state.player(PlayerId(1)).object,
            "ThatPlayer is the bound player's proxy"
        );
    }

    /// [CR#702.12b]: an indestructible permanent can't be destroyed — the
    /// `Destroy` action's `WillDestroy` intent finds the
    /// destruction-replacement static and is replaced to nothing, so the
    /// Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_destroy_action() {
        let (mut state, myr) = myr_on_field();
        let frame = frame_src(myr);
        state.run_effect(Effect::Act(Action::Destroy(sel_this())), &frame);
        // WillDestroy applies and schedules no zone move (replaced to nothing).
        let _ = state.step();
        assert!(
            state.objects.get(myr).is_some(),
            "indestructible object still exists"
        );
        assert!(
            state.zones.battlefield.contains(&myr),
            "still on the battlefield"
        );
        assert!(state.zones.graveyards[0].is_empty(), "not destroyed");
    }

    /// A destructible creature still dies: `Destroy` → `WillDestroy` (nothing
    /// replaces it) → `ZoneWillChange(Battlefield → Graveyard)` →
    /// `ZoneChanged`, reminting it into its owner's graveyard.
    #[test]
    fn destroy_action_sends_a_normal_creature_to_its_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(Effect::Act(Action::Destroy(sel_this())), &frame);
        // WillDestroy → ZoneWillChange → ZoneChanged.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
    }

    /// [CR#400.7]: `Move(This, Graveyard)` is a PLAIN relocation — no
    /// `WillDestroy` intent, so it's a direct `ZoneWillChange(Battlefield →
    /// Graveyard)` → `ZoneChanged`, reminting the object into its OWNER's
    /// graveyard. (Indestructible would not save it — but a plain Grizzly Bears
    /// exercises the move path.)
    #[test]
    fn move_sends_this_to_owner_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            Effect::Act(Action::Move(sel_this(), Zone::Graveyard)),
            &frame,
        );
        // ZoneWillChange → ZoneChanged (one fewer step than Destroy — no
        // WillDestroy replace stage).
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "moved into owner's graveyard"
        );
    }

    #[test]
    fn action_items_for_tap_draw_loselife() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);

        // By(You, Tap(This)) -> one Single(Tapped(src)) carrying the
        // effect-instruction cause triple (events.md §3).
        let items = state.action_items(&by_you(PlayerAction::Tap(sel_this())), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Tapped {
                object: src,
                cause: Some(crate::event::Cause {
                    verb: "Tap".into(),
                    agency: deckmaste_core::Agency::EffectInstruction,
                    agent: Some((src, PlayerId(0))),
                }),
            }))]
        );

        // By(You, Draw(2)) -> two sequential Single(WillDraw) for the controller
        let items = state.action_items(&by_you(PlayerAction::Draw(Count::Literal(2))), &frame);
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                player: PlayerId(0),
                ..
            }))
        )));

        // By(You, LoseLife(3)) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(&by_you(PlayerAction::LoseLife(Count::Literal(3))), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            }))]
        );
    }

    /// [CR#701.26a]: only an untapped permanent can be tapped — a tap
    /// instruction on an already-tapped object is a no-op, and a no-op is
    /// no event ([CR#603.2e] "becomes tapped" fires on the transition only).
    #[test]
    fn tap_effect_skips_already_tapped() {
        let (mut state, src) = bear_on_field();
        state.objects.obj_mut(src).tapped = true;
        let frame = frame_src(src);
        let items = state.action_items(&by_you(PlayerAction::Tap(sel_this())), &frame);
        assert_eq!(
            items,
            vec![],
            "tapping an already-tapped object emits nothing"
        );
    }

    /// [CR#701.26b]: the untap mirror — untapping an untapped object is a
    /// no-op, no event.
    #[test]
    fn untap_effect_skips_already_untapped() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);
        let items = state.action_items(&by_you(PlayerAction::Untap(sel_this())), &frame);
        assert_eq!(
            items,
            vec![],
            "untapping an already-untapped object emits nothing"
        );
    }

    /// An explicit agent: `By(Target(0), Draw(2))` draws for the targeted
    /// player, not the controller. Targets player 1's proxy.
    #[test]
    fn action_items_explicit_agent_draws_for_target() {
        let (state, src) = bear_on_field();
        let p1_proxy = state.players[1].object;
        let frame = frame_src_targets(src, vec![p1_proxy]);
        let items = state.action_items(
            &Action::By(Reference::Target(0), PlayerAction::Draw(Count::Literal(2))),
            &frame,
        );
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                player: PlayerId(1),
                ..
            }))
        )));
    }

    /// `CountOf` is the filter's live cardinality; a `ControlledBy(Ref(You))`
    /// relation anchors to the frame's side via the watcher.
    #[test]
    fn count_of_counts_live_matching_objects() {
        let (mut state, bear) = bear_on_field();
        // A second bear onto the battlefield, then handed to player 1.
        let _ = second_bear_to_player_1(&mut state);

        let frame = frame_src(bear);
        let creatures = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        ]);
        assert_eq!(
            state.eval_count(&Count::CountOf(Box::new(creatures.clone())), &frame),
            2
        );

        // "Creatures you control": only the frame side's bear.
        let yours = Filter::AllOf(vec![
            creatures,
            Filter::Relation(deckmaste_core::RelationFilter::ControlledBy(Box::new(
                Filter::Ref(Reference::You),
            ))),
        ]);
        assert_eq!(
            state.eval_count(&Count::CountOf(Box::new(yours)), &frame),
            1
        );
    }

    /// `StatOf` reads the DERIVED stat (a pump shows through) and the
    /// printed mana value ([CR#202.3]).
    #[test]
    fn stat_of_reads_derived_stats() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);

        let power = Count::StatOf(Reference::Target(0), deckmaste_core::Stat::Power);
        assert_eq!(state.eval_count(&power, &frame), 2);
        assert_eq!(
            state.eval_count(
                &Count::StatOf(Reference::This, deckmaste_core::Stat::ManaValue),
                &frame
            ),
            2,
            "Grizzly Bears costs {{1}}{{G}}"
        );

        // A +1/+0 continuous effect shows the read rides the layer view.
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![bear]),
            changes: vec![deckmaste_core::Modification::AddPower(Count::Literal(1))],
            duration: deckmaste_core::Duration::EndOfGame,
            is_cda: false,
        });
        assert_eq!(state.eval_count(&power, &frame), 3);
    }

    /// "That much" reads the amount the damage instruction fixed: the two
    /// instructions run through the agenda, the `DamageDealt` apply records
    /// 3, and the later `GainLife(ThatMuch)` evaluation reads it back.
    #[test]
    fn that_much_gains_life_equal_to_damage_dealt() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);
        state.run_effect(
            Effect::Sequence(vec![
                Effect::Act(Action::DealDamage(
                    Selection::Ref(Reference::Target(0)),
                    Count::Literal(3),
                )),
                Effect::Act(by_you(PlayerAction::GainLife(Count::ThatMuch))),
            ]),
            &frame,
        );
        // RunEffect(damage) → Emit(DamageDealt) → RunEffect(gain) → Emit(LifeGained).
        for _ in 0..4 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).damage, 3);
        assert_eq!(state.players[0].life, 23);
    }

    #[test]
    fn count_x_reads_announced_value() {
        let (state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
            chosen: None,
            x: Some(3),
        };
        assert_eq!(state.eval_count(&Count::X, &frame), 3);
    }

    #[test]
    fn each_creature_yields_all_battlefield_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second Grizzly Bears from player 0's hand onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let filter = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        ]);
        let mut got = state.eval_selection_set(&Selection::Each(filter), &frame);
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want);
    }

    /// `Each(Kind(Player))` yields exactly the two player proxies (no card
    /// objects), and `DealDamage` wraps them in ONE simultaneous `Batch`.
    #[test]
    fn each_player_deal_damage_emits_one_batch() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        // Build the effect directly: DealDamage(Each(Kind(Player)), 20)
        let effect = Effect::Act(Action::DealDamage(
            Selection::Each(Filter::Kind(ObjectKind::Player)),
            Count::Literal(20),
        ));
        state.run_effect(effect, &frame);

        // The agenda front should now have a single Emit(Batch([...])) item.
        let outcome = state.step();
        let Progress::Applied(Occurrence::Batch(events)) = (match outcome {
            StepOutcome::Progress(p) => p,
            other => panic!("expected Progress, got {other:?}"),
        }) else {
            panic!("expected Applied(Batch(…))");
        };

        // Both players took 20 damage, order-independent.
        let p0_obj = state.players[0].object;
        let p1_obj = state.players[1].object;
        let mut got: Vec<_> = events
            .iter()
            .map(|e| match e {
                GameEvent::DamageDealt { target, amount, .. } => (*target, *amount),
                other => panic!("unexpected event {other:?}"),
            })
            .collect();
        got.sort();
        let mut want = vec![(p0_obj, 20u32), (p1_obj, 20u32)];
        want.sort();
        assert_eq!(got, want);
    }

    /// `DealDamage(Each(AllOf([InZone(Battlefield), Type(Creature)])), 2)` with
    /// two creatures on the field emits ONE `Batch` of two `DamageDealt`
    /// events — the sweep fixture drives simultaneous deaths later.
    #[test]
    fn each_creature_deal_damage_emits_one_batch() {
        let (mut state, a) = bear_on_field();
        // Force a second creature onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let effect = Effect::Act(Action::DealDamage(
            Selection::Each(Filter::AllOf(vec![
                Filter::State(StateFilter::InZone(Zone::Battlefield)),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ])),
            Count::Literal(2),
        ));
        state.run_effect(effect, &frame);

        let outcome = state.step();
        let Progress::Applied(Occurrence::Batch(events)) = (match outcome {
            StepOutcome::Progress(p) => p,
            other => panic!("expected Progress, got {other:?}"),
        }) else {
            panic!("expected Applied(Batch(…))");
        };

        // Both creatures took 2 damage.
        let mut got: Vec<_> = events
            .iter()
            .map(|e| match e {
                GameEvent::DamageDealt { target, amount, .. } => (*target, *amount),
                other => panic!("unexpected event {other:?}"),
            })
            .collect();
        got.sort();
        let mut want = vec![(a, 2u32), (b, 2u32)];
        want.sort();
        assert_eq!(got, want);
    }

    /// [CR#611.2]/[CR#611.2c]: `Effect::Continuously(Modify(Matching(...), ...),
    /// UntilEndOfTurn)` — the resolve arm pushes one `ContinuousEffect` with a
    /// `ScopeResolved::Floating` scope and the right duration/changes.
    #[test]
    fn continuously_matching_registers_floating_scope() {
        use deckmaste_core::CharacteristicFilter;
        use deckmaste_core::ContinuouslyEffect;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Effect;
        use deckmaste_core::Filter;
        use deckmaste_core::Modification;
        use deckmaste_core::Scope;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::Type;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        assert!(state.continuous.is_empty(), "no effects before resolve");

        let filter = Filter::Characteristic(CharacteristicFilter::Type(Type::Creature));
        let effect = Effect::Continuously(ContinuouslyEffect {
            effect: Box::new(StaticEffect::Modify {
                of: Scope::Matching(filter.clone()),
                changes: vec![Modification::AddPower(Count::Literal(1))],
            }),
            duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1, "one effect registered");
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Floating(f) if f == &filter),
            "scope is Floating(creature filter)"
        );
        assert_eq!(
            ce.duration,
            Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
        );
        assert_eq!(ce.changes, vec![Modification::AddPower(Count::Literal(1))]);
        assert!(!ce.is_cda);
    }

    /// [CR#611.2c]: `Effect::Continuously(Modify(Of(This), ...), ...)` locks
    /// the id at creation — `ScopeResolved::Locked(vec![src])`.
    #[test]
    fn continuously_of_this_registers_locked_scope() {
        use deckmaste_core::ContinuouslyEffect;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Effect;
        use deckmaste_core::Modification;
        use deckmaste_core::Reference;
        use deckmaste_core::Scope;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        let effect = Effect::Continuously(ContinuouslyEffect {
            effect: Box::new(StaticEffect::Modify {
                of: Scope::Of(Reference::This),
                changes: vec![Modification::AddToughness(Count::Literal(2))],
            }),
            duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1, "one effect registered");
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Locked(ids) if ids == &vec![src]),
            "scope is Locked([src])"
        );
        assert_eq!(
            ce.changes,
            vec![Modification::AddToughness(Count::Literal(2))]
        );
    }

    /// `By(You, GainLife(3))` → one `LifeGained`; `By(You, Untap(This))` → one
    /// `Untapped` — the mirrors of `LoseLife`/`Tap` above. The bear is tapped
    /// first: untapping is transition-only ([CR#701.26b]).
    #[test]
    fn action_items_for_gainlife_untap() {
        let (mut state, src) = bear_on_field();
        state.objects.obj_mut(src).tapped = true;
        let frame = frame_src(src);

        let items = state.action_items(&by_you(PlayerAction::GainLife(Count::Literal(3))), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 3,
            }))]
        );

        let items = state.action_items(&by_you(PlayerAction::Untap(sel_this())), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Untapped(src)))]
        );
    }

    /// [CR#701.21a]: `Sacrifice(This)` emits the verb fact, which evolves into
    /// the Battlefield→Graveyard move — old id gone, fresh object in the
    /// owner's graveyard.
    #[test]
    fn sacrifice_this_remints_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Sacrifice(sel_this()))),
            &frame,
        );
        // Sacrificed → ZoneWillChange → ZoneChanged.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
        assert_ne!(state.zones.graveyards[0][0], bear, "reminted");
    }

    /// A sacrifice rides the same death pipeline as a destroy: the sacrificed
    /// creature's own dies-trigger fires ([CR#603.6c] — the leaving object
    /// watches its own departure).
    #[test]
    fn sacrifice_fires_the_dying_objects_dies_trigger() {
        use crate::object::ObjectSource;

        let card = Arc::new(canon().card("Footlight Fiend").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let card_id = state.cards.push(card, PlayerId(0));
        let gob = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(gob);

        let frame = frame_src(gob);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Sacrifice(sel_this()))),
            &frame,
        );
        for _ in 0..10 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the self-dies trigger must be noted"
        );
        assert!(state.objects.get(gob).is_none(), "the sacrifice happened");
    }

    /// [CR#701.13a,406.2]: exile moves an object to the shared exile zone —
    /// from the battlefield, and (via the graveyard source arm) from a
    /// graveyard.
    #[test]
    fn exile_moves_objects_from_battlefield_and_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(Effect::Act(by_you(PlayerAction::Exile(sel_this()))), &frame);
        // ZoneWillChange → ZoneChanged.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old id gone");
        assert_eq!(state.zones.exile.len(), 1);
        let exiled = state.zones.exile[0];
        assert_eq!(state.objects.obj(exiled).zone, Some(Zone::Exile));

        // From the graveyard: force a hand card into the graveyard, exile it.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let frame = frame_src(card);
        state.run_effect(Effect::Act(by_you(PlayerAction::Exile(sel_this()))), &frame);
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.exile.len(), 2);
    }

    /// [CR#400.7]: `ReturnToHand(This)` moves the source to its owner's hand,
    /// reminting it — the old id is gone and a fresh object sits in hand. The
    /// graveyard arm proves the move reads each object's current zone (like
    /// `Exile`), not a hard-coded battlefield source.
    #[test]
    fn return_to_hand_from_battlefield_and_graveyard() {
        let (mut state, bear) = bear_on_field();
        let hand_before = state.zones.hands[0].len();
        let frame = frame_src(bear);
        state.run_effect(Effect::Act(Action::ReturnToHand(sel_this())), &frame);
        // ZoneWillChange → ZoneChanged.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.hands[0].len(), hand_before + 1);
        let returned = *state.zones.hands[0].last().expect("a returned card");
        assert_eq!(state.objects.obj(returned).zone, Some(Zone::Hand));

        // From the graveyard ([CR#400.7] reads the current zone): force a hand
        // card into the graveyard, then return it to hand.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let gy_hand_before = state.zones.hands[0].len();
        let frame = frame_src(card);
        state.run_effect(Effect::Act(Action::ReturnToHand(sel_this())), &frame);
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.hands[0].len(), gy_hand_before + 1);
    }

    /// [CR#701.6a]: countering a spell removes it from the stack and puts it
    /// into its owner's graveyard, reminted ([CR#400.7]) and cause-tagged
    /// "Counter" — the spell never resolves.
    #[test]
    fn counter_spell_goes_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        // Stand a hand card up as a spell on the stack, owned by player 0.
        let spell = state.zones.hands[0][0];
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != spell);
        state.objects.obj_mut(spell).zone = Some(Zone::Stack);
        state.stack.push(StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
        });
        let gy_before = state.zones.graveyards[0].len();

        // The source's effect counters that spell (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![spell]);
        state.run_effect(
            Effect::Act(Action::Counter(Selection::Ref(Reference::Target(0)))),
            &frame,
        );
        // ZoneWillChange → ZoneChanged.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.stack.is_empty(), "spell removed from the stack");
        assert!(state.objects.get(spell).is_none(), "old stack id gone");
        assert_eq!(state.zones.graveyards[0].len(), gy_before + 1);
        let countered = *state.zones.graveyards[0].last().expect("a countered spell");
        assert_eq!(state.objects.obj(countered).zone, Some(Zone::Graveyard));
    }

    /// [CR#401.7]: `PutInLibrary(This, 0)` puts the card on top; an index past
    /// the bottom clamps to the bottom.
    #[test]
    fn put_in_library_top_and_clamped_bottom() {
        let (mut state, bear) = bear_on_field();
        let bear_card = state.objects.obj(bear).card_id().expect("card-backed");
        let lib_before = state.zones.libraries[0].len();
        let frame = frame_src(bear);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::PutInLibrary(
                sel_this(),
                Count::Literal(0),
            ))),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old id gone");
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let top = *state.zones.libraries[0].front().expect("non-empty library");
        assert_eq!(state.objects.obj(top).card_id(), Some(bear_card));
        assert_eq!(state.objects.obj(top).zone, Some(Zone::Library));

        // Past the bottom ([CR#401.7]): index 99 places it on the bottom.
        let frame = frame_src(top);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::PutInLibrary(
                sel_this(),
                Count::Literal(99),
            ))),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let bottom = *state.zones.libraries[0].back().expect("non-empty library");
        assert_eq!(state.objects.obj(bottom).card_id(), Some(bear_card));
    }

    /// `AddMana(2, Green)` needs no choice and lands in the pool ([CR#106.4]);
    /// `AddMana(1, AnyColor)` surfaces `ChooseManaColor` with the five colors
    /// — colorless is not a color ([CR#105.4]) and is rejected.
    #[test]
    fn add_mana_specific_and_any_color() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let green = ColorOrColorless::Color(Color::Green);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::AddMana(
                Count::Literal(2),
                ManaSpec::Specific(green).into(),
            ))),
            &frame,
        );
        let _ = state.step();
        assert_eq!(state.players[0].mana_pool.amount(green), 2);

        state.run_effect(
            Effect::Act(by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::AnyColor.into(),
            ))),
            &frame,
        );
        let _ = state.step(); // ManaColorOpened
        let StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor {
            player,
            options,
            amount,
            ..
        }) = state.step()
        else {
            panic!("expected ChooseManaColor, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(options.len(), 5, "the five colors");
        assert_eq!(amount, 1);
        assert!(
            state
                .submit_decision(Decision::ManaColor(ColorOrColorless::Colorless))
                .is_err(),
            "colorless is not a color"
        );
        let blue = ColorOrColorless::Color(Color::Blue);
        state.submit_decision(Decision::ManaColor(blue)).unwrap();
        let _ = state.step(); // ManaAdded applies
        assert_eq!(state.players[0].mana_pool.amount(blue), 1);
    }

    /// `AddMana(1, WithRiders{ mana: Red, riders: [SpendOnly(Any)] })` lands
    /// one red unit in the pool whose riders vec is non-empty ([CR#106.6]).
    #[test]
    fn add_mana_with_riders_lands_unit_carrying_riders() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::Filter;
        use deckmaste_core::ManaProduction;
        use deckmaste_core::ManaRider;
        use deckmaste_core::ManaSpec;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let red = ColorOrColorless::Color(Color::Red);
        let rider = ManaRider::SpendOnly(Filter::Any);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaProduction::WithRiders {
                    mana: ManaSpec::Specific(red),
                    riders: vec![rider],
                },
            ))),
            &frame,
        );
        let _ = state.step(); // ManaAdded applies
        assert_eq!(state.players[0].mana_pool.amount(red), 1);
        let units_with_riders = state.players[0]
            .mana_pool
            .units()
            .iter()
            .filter(|u| !u.riders.is_empty())
            .count();
        assert_eq!(units_with_riders, 1, "one unit should carry riders");
    }

    /// [CR#701.9b]: `Discard(2)` surfaces the card choice; a wrong-sized answer
    /// is rejected; the right answer discards through the Hand→Graveyard
    /// pipeline. Discarding more than the hand holds clamps to the whole hand
    /// ([CR#101.3]).
    #[test]
    fn discard_surfaces_choice_validates_and_clamps() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let hand_before = state.zones.hands[0].len();
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Discard(Count::Literal(2)))),
            &frame,
        );
        let _ = state.step(); // DiscardOpened
        let StepOutcome::NeedsDecision(PendingDecision::DiscardCards { player, count }) =
            state.step()
        else {
            panic!("expected DiscardCards, got {:?}", state.pending);
        };
        assert_eq!((player, count), (PlayerId(0), 2));
        let one = vec![state.zones.hands[0][0]];
        assert!(
            state.submit_decision(Decision::Discard(one)).is_err(),
            "exactly `count` cards must be chosen"
        );
        let two = state.zones.hands[0][..2].to_vec();
        state.submit_decision(Decision::Discard(two)).unwrap();
        for _ in 0..30 {
            if state.zones.graveyards[0].len() == 2 {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(state.zones.hands[0].len(), hand_before - 2);
        assert_eq!(state.zones.graveyards[0].len(), 2);

        // Clamp: an instruction to discard far more than the hand holds
        // discards the whole hand.
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Discard(Count::Literal(99)))),
            &frame,
        );
        let _ = state.step();
        let StepOutcome::NeedsDecision(PendingDecision::DiscardCards { count, .. }) = state.step()
        else {
            panic!("expected DiscardCards, got {:?}", state.pending);
        };
        assert_eq!(count as usize, hand_before - 2, "clamped to the hand size");
        let rest = state.zones.hands[0].clone();
        state.submit_decision(Decision::Discard(rest)).unwrap();
        for _ in 0..30 {
            if state.zones.hands[0].is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert!(state.zones.hands[0].is_empty());
        assert_eq!(state.zones.graveyards[0].len(), hand_before);
    }

    /// A remembered `PlayerAction` macro invocation resolves through its
    /// expanded body.
    #[test]
    fn expanded_player_action_resolves_through_body() {
        use deckmaste_core::Expansion;
        use deckmaste_core::ExpansionArgs;

        let (state, src) = bear_on_field();
        let frame = frame_src(src);
        let body = PlayerAction::GainLife(Count::Literal(2));
        let expanded = PlayerAction::Expanded(Expansion {
            name: "GainTwo".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(body.clone()),
        });
        assert_eq!(
            state.action_items(&by_you(expanded), &frame),
            state.action_items(&by_you(body), &frame),
        );
    }

    /// [CR#701.7a,111.2]: `Create(2, token)` puts two token permanents onto
    /// the battlefield under the creator — owned by them, summoning-sick,
    /// kind `Token` (not `Card`, [CR#111.6]) — as ONE simultaneous batch of
    /// `TokenCreated` facts, each followed by its `ZoneChanged { from: None,
    /// to: Battlefield }` fact.
    #[test]
    fn create_tokens_enter_battlefield() {
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let token = Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![],
            power: None,
            toughness: None,
        };
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Create(
                Count::Literal(2),
                token.into(),
            ))),
            &frame,
        );
        // One simultaneous batch of two TokenCreated facts.
        let made = match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Batch(events))) => events,
            other => panic!("expected Applied(Batch), got {other:?}"),
        };
        assert_eq!(made.len(), 2);
        assert!(made.iter().all(|e| matches!(
            e,
            GameEvent::TokenCreated {
                player: PlayerId(0),
                ..
            }
        )));

        let tokens: Vec<ObjectId> = state
            .zones
            .battlefield
            .iter()
            .copied()
            .filter(|&id| id != src)
            .collect();
        assert_eq!(tokens.len(), 2, "two tokens on the battlefield");
        for &t in &tokens {
            assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
            assert_eq!(
                state.owner_of(t),
                PlayerId(0),
                "[CR#111.2]: creator owns it"
            );
            assert_eq!(state.objects.obj(t).controller, PlayerId(0));
            assert!(state.objects.obj(t).summoning_sick, "[CR#302.6]");
            assert!(
                obj_matches(
                    &state,
                    t,
                    &Filter::Characteristic(CharacteristicFilter::Type(Type::Artifact))
                ),
                "the creating effect's characteristics stick ([CR#111.3])"
            );
        }
        // Each token's enter fact follows — from: None (created, not moved).
        for _ in 0..2 {
            match state.step() {
                StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                    GameEvent::ZoneChanged {
                        from: None,
                        to: Zone::Battlefield,
                        ..
                    },
                ))) => {}
                other => panic!("expected the token's ZoneChanged fact, got {other:?}"),
            }
        }
    }

    /// [CR#109.2]: an activated ability that counts "Goblins you control" — a
    /// subtype description with no zone qualifier — means Goblin PERMANENTS on
    /// the battlefield. The canonical (`Permanent`-scoped) filter counts
    /// exactly the battlefield Goblins; the bare-subtype filter (no zone
    /// scope) ALSO matches the ability's own freshly-minted on-stack
    /// identity — which reuses the source's card id — so it over-counts by
    /// one. With three controlled Goblins (incl. the source) Krenko makes 3
    /// tokens, not 4. This pins the engine semantics the parser fix relies
    /// on (see `parsers::filter::head_noun`'s `Permanent` scope).
    #[test]
    fn count_you_control_excludes_the_activations_own_stack_copy() {
        use crate::object::ObjectSource;

        // A Goblin permanent on the battlefield, player 0.
        fn goblin(state: &mut GameState, name: &str) -> ObjectId {
            mint_on_field(
                state,
                Card::Normal(CardFace {
                    name: name.into(),
                    types: vec![Type::Creature],
                    subtypes: vec![subtype("Goblin")],
                    power: Some(deckmaste_core::StatValue::Number(1)),
                    toughness: Some(deckmaste_core::StatValue::Number(1)),
                    ..CardFace::default()
                }),
            )
        }

        // Builds the Krenko scenario fresh (three controlled Goblins, incl. the
        // source, plus the activation's own Stack-zone copy of the source),
        // runs `Create(CountOf(filter), 1/1 Goblin)` once, and returns how many
        // tokens entered. A fresh state per call keeps the two filters'
        // token batches from feeding each other's count. `filter` is parsed
        // (and its `Permanent` macro expanded) through the live plugin macros.
        fn tokens_made(filter: &str) -> usize {
            let mut state = game();
            let source = goblin(&mut state, "Krenko, Mob Boss");
            let _g2 = goblin(&mut state, "Goblin Two");
            let _g3 = goblin(&mut state, "Goblin Three");

            // The activation mints a Stack-zone identity that REUSES the
            // source's card id ([CR#602.2a]) — the LKI copy that drives the
            // over-count. `eval_count` enumerates every object in the store, so
            // minting it into the Stack zone is enough for the unzoned filter to
            // reach it.
            let src_card = state.objects.obj(source).card_id().unwrap();
            let _stack_copy =
                state
                    .objects
                    .mint(ObjectSource::Card(src_card), PlayerId(0), Some(Zone::Stack));

            let parsed: Filter = builtin().macros.read_str(filter).unwrap();
            let frame = frame_src(source);
            let before = state.zones.battlefield.len();
            state.run_effect(
                Effect::Act(by_you(PlayerAction::Create(
                    Count::CountOf(Box::new(parsed)),
                    deckmaste_core::Token {
                        color_indicator: vec![],
                        supertypes: vec![],
                        types: vec![Type::Creature],
                        subtypes: vec![subtype("Goblin")],
                        abilities: vec![],
                        power: Some(deckmaste_core::StatValue::Number(1)),
                        toughness: Some(deckmaste_core::StatValue::Number(1)),
                    }
                    .into(),
                ))),
                &frame,
            );
            // Drain the queued work (the TokenCreated batch + per-token enters).
            while let StepOutcome::Progress(_) = state.step() {}
            state.zones.battlefield.len() - before
        }

        // Bare subtype (the pre-fix parser output): the Stack-zone copy is a
        // Goblin you control too, so it over-counts → 4.
        assert_eq!(
            tokens_made("AllOf([Subtype(\"Goblin\"), ControlledBy(Ref(You))])"),
            4,
            "the unzoned filter wrongly counts the on-stack copy"
        );

        // The canonical battlefield-scoped filter (the post-fix parser output):
        // the Stack-zone copy is excluded → exactly the three battlefield
        // Goblins.
        assert_eq!(
            tokens_made("AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])"),
            3,
            "[CR#109.2]: the Permanent scope counts only battlefield Goblins"
        );
    }

    /// The builtin predefined Treasure token ([CR#111.10a]) creates with its
    /// declared subtype and the [CR#111.4] default name (subtypes + "Token").
    #[test]
    fn create_builtin_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let treasure = builtin().token("Treasure").unwrap();
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Create(
                Count::Literal(1),
                treasure.into(),
            ))),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
        let card = state.objects.obj(t).card_id().expect("card-backed");
        // Subtype asserted on the card entry directly — `Filter::Subtype`
        // evaluation is the `engine-filter-breadth` item.
        assert!(
            state
                .cards
                .get(card)
                .subtypes
                .iter()
                .any(|s| s.name == "Treasure"),
            "declared subtype sticks"
        );
        assert!(state.cards.get(card).is_token, "[CR#111.6]");
        assert_eq!(
            crate::derive::face(&state.cards.get(card).def).name,
            "Treasure Token",
            "[CR#111.4]: unnamed token defaults to subtypes + \"Token\""
        );
    }

    // ====================================================================
    // Task 4.6 — end-to-end (Enchant + Equip + Fortify + Reconfigure)
    // ====================================================================
    //
    // These drive a real `GameState` and assert real state, exercising the
    // ACTUAL keyword-macro (builtin) + subtype-confer (canon) paths where
    // feasible — the integration coverage that caught the composite-flatten
    // prerequisite. Helpers below build cards from the live plugin macros.

    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Modification;
    use deckmaste_core::Scope;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Subtype;

    use crate::object::ObjectSource;

    /// Expand a builtin keyword macro invocation to an `Ability::Keyword`.
    fn keyword(invocation: &str) -> Ability {
        Ability::Keyword(builtin().macros.read_str(invocation).unwrap())
    }

    /// A canon subtype value (with its `confers:` list) by printed name.
    fn subtype(name: &str) -> Subtype {
        canon()
            .subtypes
            .get(&deckmaste_core::Ident::from(name))
            .unwrap_or_else(|| panic!("canon defines the {name} subtype"))
            .clone()
    }

    /// A "host gets +n/+n" static targeting this attachment's host
    /// (`Of(AttachHostOf(This))`) — the equipped/enchanted-creature bonus.
    fn host_pump(n: u32) -> Ability {
        Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Modify {
                of: Scope::Of(Reference::AttachHostOf(Box::new(Reference::This))),
                changes: vec![
                    Modification::AddPower(Count::Literal(n)),
                    Modification::AddToughness(Count::Literal(n)),
                ],
            }],
            characteristic_defining: false,
        })
    }

    /// Mint a card-backed object directly onto the battlefield (player 0).
    fn mint_on_field(state: &mut GameState, card: Card) -> ObjectId {
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// A vanilla 2/2 creature on the battlefield.
    fn vanilla_creature(state: &mut GameState, name: &str) -> ObjectId {
        mint_on_field(
            state,
            Card::Normal(CardFace {
                name: name.into(),
                types: vec![Type::Creature],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(2)),
                ..CardFace::default()
            }),
        )
    }

    /// [CR#702.6a]: activate the Equipment's equip ability (sorcery speed)
    /// targeting a creature you control → the host's derived P/T includes the
    /// Equipment's "+1/+1" bonus (via the `Of(AttachHostOf(This))` path).
    #[test]
    fn equip_e2e() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Bear Host");
        // A real Equipment: the Equipment subtype confer (Innate Cant(Attach)) +
        // the `equip {T}` keyword + "+1/+1 to the equipped creature".
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Sword".into(),
                types: vec![Type::Artifact],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])"), host_pump(1)],
                ..CardFace::default()
            }),
        );
        // Base host is 2/2.
        assert_eq!(state.layers().power(host), Some(2));

        // Drive the equip activated ability: the keyword + host_pump → the
        // activated ability is at filtered index 0 (no Innate to skew it here,
        // but resolve via the offered legal action to be faithful).
        let frame = frame_src_targets(equipment, vec![host]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        drain(&mut state);

        assert_eq!(
            state.objects.obj(equipment).attached_to,
            Some(host),
            "equip attached the Equipment to the host ([CR#701.3a])"
        );
        assert_eq!(
            state.layers().power(host),
            Some(3),
            "the equipped creature gets +1/+1 (host-targeting static landed)"
        );
        assert_eq!(state.layers().toughness(host), Some(3));
    }

    /// [CR#303.4,704.5m]: a CAST Aura resolves attached to the SPELL'S CHOSEN
    /// TARGET (the cast-path host wiring), buffs it +2/+2, and is sent to its
    /// owner's graveyard by the SBA when the host leaves.
    #[test]
    fn aura_cast_e2e() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Enchanted Bear");
        // A real Aura: Enchant(creature) keyword (targeting Spell + Cant(Attach)
        // + AsEnters) + the Aura subtype's Innate graveyard SBA + "+2/+2".
        let aura_card = Card::Normal(CardFace {
            name: "Test Aura".into(),
            types: vec![Type::Enchantment],
            subtypes: vec![subtype("Aura")],
            abilities: vec![keyword("Enchant(Type(Creature))"), host_pump(2)],
            ..CardFace::default()
        });
        // Stand the Aura up as a spell on the stack, target = the host.
        let cid = state.cards.push(Arc::new(aura_card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![host],
            x: None,
        });
        // Resolve the Aura spell — it enters attached to its chosen target.
        // (`resolve_object` schedules the entering ZoneMove at the agenda front;
        // `run_injected` processes just that, without parking priority.)
        state.resolve_object(spell);
        run_injected(&mut state);

        let aura = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(cid))
            .expect("the Aura entered the battlefield");
        assert_eq!(
            state.objects.obj(aura).attached_to,
            Some(host),
            "cast Aura enters attached to its chosen target ([CR#303.4])"
        );
        assert_eq!(
            state.layers().power(host),
            Some(4),
            "the enchanted creature gets +2/+2"
        );

        // Destroy the host (source = host, `This` = the dying creature); the SBA
        // sweep then sends the now-unattached Aura to the graveyard ([CR#704.5m]).
        let frame = frame_src(host);
        state.run_effect(Effect::Act(Action::Destroy(sel_this())), &frame);
        run_injected(&mut state);
        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            run_injected(&mut state);
        }
        let aura_gy = *state.zones.graveyards[PlayerId(0).index()]
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(cid))
            .expect("the orphaned Aura was put into its owner's graveyard ([CR#704.5m])");
        assert_eq!(state.objects.obj(aura_gy).zone, Some(Zone::Graveyard));
    }

    /// [CR#704.5n]: when an equipped creature dies, the Equipment becomes
    /// unattached and STAYS on the battlefield (no graveyard SBA — that's
    /// Auras).
    #[test]
    fn equipment_host_dies_unattaches() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Doomed Bear");
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Sticky Sword".into(),
                types: vec![Type::Artifact],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equipment).attached_to = Some(host);

        // Host dies.
        let frame = frame_src_targets(equipment, vec![host]);
        state.run_effect(
            Effect::Act(Action::Destroy(Selection::Ref(Reference::Target(0)))),
            &frame,
        );
        drain(&mut state);
        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            drain(&mut state);
        }
        assert_eq!(
            state.objects.obj(equipment).attached_to,
            None,
            "the Equipment became unattached when its host died ([CR#704.5n])"
        );
        assert!(
            state.zones.battlefield.contains(&equipment),
            "the Equipment STAYS on the battlefield (not graveyarded)"
        );
    }

    /// [CR#702.16d]: a creature that gains protection from a color drops a
    /// colored Equipment attached to it — the SBA re-runs `attachment_legal`
    /// (host-side protection `Cant(Attach)`) and unattaches.
    #[test]
    fn protection_drops_equipment() {
        use deckmaste_core::Color;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;

        let mut state = game();
        // The host gains protection from red: a host-side `Cant(Attach(what:
        // red, to: This))` (the Protection-conferred shape, [CR#702.16d]).
        let host = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Protected Bear".into(),
                types: vec![Type::Creature],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(2)),
                abilities: vec![Ability::Static(StaticAbility {
                    condition: None,
                    effects: vec![StaticEffect::Deontic(Deontic::Cant(
                        DeonticAction::Attach {
                            what: Filter::Characteristic(CharacteristicFilter::ColorIs(Color::Red)),
                            to: Filter::Ref(Reference::This),
                        },
                    ))],
                    characteristic_defining: false,
                })],
                ..CardFace::default()
            }),
        );
        // A RED Equipment attached to the host.
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Red Sword".into(),
                types: vec![Type::Artifact],
                color_indicator: vec![Color::Red],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equipment).attached_to = Some(host);
        // Sanity: it is currently illegal (protection) — the SBA will catch it.
        assert!(!crate::legal::attachment_legal(&state, equipment, host));

        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            drain(&mut state);
        }
        assert_eq!(
            state.objects.obj(equipment).attached_to,
            None,
            "the colored Equipment fell off the protected creature ([CR#702.16d])"
        );
    }

    /// [CR#702.67a]: a Fortification with `fortify` activated, targeting a land
    /// you control → attached to that land.
    #[test]
    fn fortify_attaches_to_land() {
        let mut state = game();
        let land = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Target Land".into(),
                types: vec![Type::Land],
                ..CardFace::default()
            }),
        );
        let fortification = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Banner".into(),
                types: vec![Type::Artifact],
                subtypes: vec![subtype("Fortification")],
                abilities: vec![keyword("Fortify([Tap])")],
                ..CardFace::default()
            }),
        );
        let frame = frame_src_targets(fortification, vec![land]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(fortification).attached_to,
            Some(land),
            "fortify attached the Fortification to the land ([CR#702.67a])"
        );
    }

    /// [CR#702.151b]: a reconfigured Equipment attached to a creature stops
    /// being a creature; unattaching restores it. SEAM: the
    /// creature-suppression static needs condition-gated layer-4 type
    /// removal the engine doesn't have yet (see Reconfigure.ron) — so the
    /// suppression assertion is `#[ignore]`d; the attach/unattach mechanics
    /// are exercised here unignored.
    #[test]
    fn reconfigure_attaches_and_unattaches() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Recon Host");
        // A reconfigure Equipment creature (it IS a creature when unattached).
        let equip_creature = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Living Weapon".into(),
                types: vec![Type::Artifact, Type::Creature],
                subtypes: vec![subtype("Equipment")],
                power: Some(deckmaste_core::StatValue::Number(1)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                abilities: vec![keyword("Reconfigure([Tap])")],
                ..CardFace::default()
            }),
        );
        // Attach via reconfigure's first ability shape (Attach to a creature).
        let frame = frame_src_targets(equip_creature, vec![host]);
        state.run_effect(
            Effect::Act(Action::Attach {
                what: Selection::Ref(Reference::This),
                to: Selection::Ref(Reference::Target(0)),
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(equip_creature).attached_to,
            Some(host),
            "reconfigure attached the Equipment to the creature ([CR#702.151a])"
        );

        // Unattach (reconfigure's second ability).
        let frame = frame_src(equip_creature);
        state.run_effect(
            Effect::Act(Action::Unattach(Selection::Ref(Reference::This))),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(equip_creature).attached_to,
            None,
            "reconfigure unattached the Equipment ([CR#702.151a])"
        );
    }

    /// [CR#702.151b]: SEAM — the creature-suppression static (a reconfigured
    /// Equipment isn't a creature while attached) needs condition-gated layer-4
    /// type removal the layer pipeline doesn't have yet (Reconfigure.ron seam).
    /// Ignored until that engine support lands.
    #[test]
    #[ignore = "engine-attach seam: conditional layer-4 type removal not built ([CR#702.151b]) — see Reconfigure.ron"]
    fn reconfigure_suppresses_creature() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Recon Host");
        let equip_creature = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Living Weapon".into(),
                types: vec![Type::Artifact, Type::Creature],
                subtypes: vec![subtype("Equipment")],
                power: Some(deckmaste_core::StatValue::Number(1)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                abilities: vec![keyword("Reconfigure([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equip_creature).attached_to = Some(host);
        // Would-be: attached → not a creature.
        let view = state.layers();
        assert!(
            !view
                .get(equip_creature)
                .card_types
                .contains(&Type::Creature),
            "attached reconfigure Equipment is not a creature ([CR#702.151b])"
        );
    }

    /// [CR#702.131c]: the grant verb emits one `GotDesignation` for a player
    /// who lacks the designation, and nothing for one who already holds it
    /// (idempotent — keeps the SBA sweep convergent and avoids spurious facts).
    #[test]
    fn get_designation_emits_once_then_nothing() {
        use crate::state::DesignationValue;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let pa = deckmaste_core::PlayerAction::GetDesignation("CitysBlessing".into());

        let items = state.player_action_items(&pa, p0, &frame);
        assert_eq!(items.len(), 1, "first grant emits exactly one fact");

        // Grant it for real, then re-run: no event.
        state
            .designations
            .players
            .insert((p0, "CitysBlessing".into()), DesignationValue::Flag);
        let items = state.player_action_items(&pa, p0, &frame);
        assert!(items.is_empty(), "already-held designation emits nothing");
    }

    /// [CR#608.2c]: `Effect::If` evaluates its condition WHEN it resolves and
    /// runs the taken branch — `then` on true, `otherwise` on false, and
    /// nothing when false with no `otherwise`. Driven via `GainLife` (a
    /// choice-free, library-free player action) so the assertion is a clean
    /// life delta.
    #[test]
    fn run_effect_if_takes_the_right_branch() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::IfEffect;

        // Trivially-true and trivially-false comparisons over literals.
        let yes = Condition::Compare(Count::Literal(1), Cmp::AtLeast, Count::Literal(0));
        let no = Condition::Compare(Count::Literal(0), Cmp::AtLeast, Count::Literal(1));
        let gain = |n| {
            Effect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            ))
        };

        let p0 = PlayerId(0);

        // true → then (gain 3), otherwise NOT taken.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Effect::If(IfEffect {
                condition: yes.clone(),
                then: Box::new(gain(3)),
                otherwise: Some(Box::new(gain(5))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 3, "true → then branch");

        // false → otherwise (gain 5), then NOT taken.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Effect::If(IfEffect {
                condition: no.clone(),
                then: Box::new(gain(3)),
                otherwise: Some(Box::new(gain(5))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 5, "false → otherwise branch");

        // false + no otherwise → nothing runs (life unchanged).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Effect::If(IfEffect {
                condition: no.clone(),
                then: Box::new(gain(3)),
                otherwise: None,
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0,
            "false + no otherwise → no change"
        );
    }

    // --- Ascend (spell form) e2e ([CR#702.131a]) -------------------------------
    //
    // The spell form of Ascend folds into `Sequence([If(<gate>,
    // GetDesignation), If(Is(You,Designated), Draw(3), otherwise: Draw(2))])`
    // (Task 7). The `Effect::If` interpreter is now live (see the `Effect::If`
    // arm in `run_effect` — it evaluates `condition_holds`, then schedules
    // `then`/`otherwise`), so these run unignored. They prove the grant-then-read
    // ordering ([CR#608.2c]): draws 3 at ten, 2 at nine, and 2 at ten-then-nine
    // (no high-water mark). The fixture is isolated by `diag_setup_is_sound`,
    // which proves the gate reads 10/9 correctly and a bare `Draw(3)` lands three.

    /// The folded Ascend gate ([CR#702.131a]) built typed — the exact shape
    /// `deckmaste_migrations::resolve::fold_spell_ascend` prepends to a spell's
    /// effect: "ten battlefield permanents you control AND you don't already
    /// have the city's blessing".
    fn ascend_gate() -> deckmaste_core::Condition {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::RelationFilter;

        Condition::AllOf(vec![
            Condition::Compare(
                Count::CountOf(Box::new(Filter::AllOf(vec![
                    Filter::State(StateFilter::InZone(Zone::Battlefield)),
                    Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                        Reference::You,
                    )))),
                ]))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Box::new(Condition::Is(
                Reference::You,
                Filter::State(StateFilter::Designated("CitysBlessing".into())),
            ))),
        ])
    }

    /// Secrets of the Golden City's resolved shape — the folded grant followed
    /// by the blessing-conditioned draw ("Ascend. Draw two cards. If you have
    /// the city's blessing, draw three instead."):
    ///
    /// ```text
    /// Sequence([
    ///   If(gate, then: GetDesignation("CitysBlessing")),          // folded Ascend
    ///   If(Is(You, Designated), then: Draw(3), otherwise: Draw(2)),
    /// ])
    /// ```
    fn secrets_effect() -> Effect {
        use deckmaste_core::Condition;
        use deckmaste_core::IfEffect;

        let by_you = |pa| Effect::Act(Action::By(Reference::You, pa));
        Effect::Sequence(vec![
            Effect::If(IfEffect {
                condition: ascend_gate(),
                then: Box::new(by_you(PlayerAction::GetDesignation("CitysBlessing".into()))),
                otherwise: None,
            }),
            Effect::If(IfEffect {
                condition: Condition::Is(
                    Reference::You,
                    Filter::State(StateFilter::Designated("CitysBlessing".into())),
                ),
                then: Box::new(by_you(PlayerAction::Draw(Count::Literal(3)))),
                otherwise: Some(Box::new(by_you(PlayerAction::Draw(Count::Literal(2))))),
            }),
        ])
    }

    /// Builds a game where p0 controls `permanents` battlefield objects, has a
    /// fat library to draw from, an EMPTY hand (so the draw delta is the
    /// post-resolution hand size), and the synthetic Secrets-of-the-Golden-City
    /// spell on the stack (its first/only ability the `secrets` effect).
    /// Returns `(state, p0, library_before)`.
    fn secrets_on_stack(permanents: usize) -> (GameState, PlayerId, usize) {
        use deckmaste_core::Ability;
        use deckmaste_core::CardFace;
        use deckmaste_core::SpellAbility;

        use crate::object::ObjectSource;

        let mut state = game();
        let p0 = PlayerId(0);

        // A stocked library and an empty hand, so the post-resolution hand size
        // IS the number of cards drawn. Mint plain library objects under p0;
        // their identity is irrelevant — a draw just remints the top.
        let dummy = Card::Normal(CardFace {
            name: "Library Filler".into(),
            ..CardFace::default()
        });
        let dummy_card = state.cards.push(Arc::new(dummy), p0);
        for _ in 0..10 {
            let id = state
                .objects
                .mint(ObjectSource::Card(dummy_card), p0, Some(Zone::Library));
            state.zones.libraries[p0.index()].push_back(id);
        }
        let library_before = state.zones.libraries[p0.index()].len();
        assert!(
            state.zones.hands[p0.index()].is_empty(),
            "empty starting hand"
        );

        // p0's battlefield: `permanents` plain artifacts. Card-backed (mirrors
        // a real board), all controlled by p0 — the gate counts these.
        for i in 0..permanents {
            let perm = Card::Normal(CardFace {
                name: format!("Permanent {i}"),
                types: vec![Type::Artifact],
                ..CardFace::default()
            });
            let card_id = state.cards.push(Arc::new(perm), p0);
            let id = state
                .objects
                .mint(ObjectSource::Card(card_id), p0, Some(Zone::Battlefield));
            state.zones.battlefield.push(id);
        }
        assert_eq!(state.zones.battlefield.len(), permanents);

        // The synthetic Secrets-of-the-Golden-City spell on the stack.
        let spell_card = Card::Normal(CardFace {
            name: "Secrets of the Golden City".into(),
            types: vec![Type::Sorcery],
            abilities: vec![Ability::Spell(SpellAbility {
                targets: vec![],
                effect: secrets_effect(),
            })],
            ..CardFace::default()
        });
        let spell_card_id = state.cards.push(Arc::new(spell_card), p0);
        let spell = state
            .objects
            .mint(ObjectSource::Card(spell_card_id), p0, Some(Zone::Stack));
        state.stack.push(StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: p0,
            targets: vec![],
            x: None,
        });

        (state, p0, library_before)
    }

    /// Steps the agenda to a stop (decision / game-over) or until `n` steps
    /// elapse, returning the `Progress` trace — the in-crate analogue of
    /// `skeleton::drain_progress`.
    fn drain_progress(state: &mut GameState, n: usize) -> Vec<Progress> {
        let mut out = Vec::new();
        for _ in 0..n {
            match state.step() {
                StepOutcome::Progress(p) => out.push(p),
                StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
            }
        }
        out
    }

    /// Isolation guard: proves the spell-form fixture is sound independent of
    /// the `Effect::If` interpreter — the gate reads true at ten / false at
    /// nine for these minted battlefield objects, and a bare `Draw(3)` from
    /// the stocked library lands three cards in hand. So any failure of the
    /// three behavioral cases below points at the interpreter, not the
    /// fixture.
    #[test]
    fn diag_setup_is_sound() {
        // Gate at ten: true.
        let (state, p0, _lib) = secrets_on_stack(10);
        let frame = frame_for(&state, p0);
        assert!(
            state.condition_holds(&ascend_gate(), &frame),
            "gate true at ten permanents"
        );

        // Gate at nine: false.
        let (state9, p0, _lib) = secrets_on_stack(9);
        let frame9 = frame_for(&state9, p0);
        assert!(
            !state9.condition_holds(&ascend_gate(), &frame9),
            "gate false at nine permanents"
        );

        // A bare Draw(3) lands three cards in hand from the stocked library.
        let (mut sd, p0, lib_before) = secrets_on_stack(10);
        let dframe = frame_for(&sd, p0);
        sd.run_effect(
            Effect::Act(Action::By(
                Reference::You,
                PlayerAction::Draw(Count::Literal(3)),
            )),
            &dframe,
        );
        let _ = drain_progress(&mut sd, 40);
        assert_eq!(
            sd.zones.hands[p0.index()].len(),
            3,
            "bare Draw(3) drew three"
        );
        assert_eq!(sd.zones.libraries[p0.index()].len(), lib_before - 3);
    }

    /// [CR#702.131a,608.2c]: on a SPELL, the folded Ascend grant ([CR#702.131a])
    /// fires DURING resolution, and because the controller follows the spell's
    /// instructions in written order ([CR#608.2c]), the DOWNSTREAM "if you have
    /// the city's blessing" read sees that fresh grant — at ten permanents the
    /// player gets the blessing AND draws three (not two). This is the crux:
    /// the grant must be applied before the later read. No high-water mark
    /// — only the count at resolution matters (see the sibling cases).
    #[test]
    fn ascend_spell_grants_then_reads_at_ten() {
        let (mut state, p0, lib_before) = secrets_on_stack(10);
        let name: deckmaste_core::Ident = "CitysBlessing".into();

        state
            .agenda
            .push_front(WorkItem::Resolve(state.stack[0].id));
        let _trace = drain_progress(&mut state, 40);

        assert!(
            state.designations.players.contains_key(&(p0, name)),
            "the folded Ascend grant fired during resolution ([CR#702.131a])"
        );
        let drawn = state.zones.hands[p0.index()].len();
        assert_eq!(
            drawn, 3,
            "the downstream read saw the fresh blessing → drew three ([CR#608.2c]); drew {drawn}"
        );
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            lib_before - 3,
            "three cards left the library"
        );
    }

    /// At NINE permanents the gate is false: no grant, the downstream read is
    /// false, the player draws two and never holds the blessing.
    #[test]
    fn ascend_spell_no_blessing_below_ten() {
        let (mut state, p0, lib_before) = secrets_on_stack(9);
        let name: deckmaste_core::Ident = "CitysBlessing".into();

        state
            .agenda
            .push_front(WorkItem::Resolve(state.stack[0].id));
        let _trace = drain_progress(&mut state, 40);

        assert!(
            !state.designations.players.contains_key(&(p0, name)),
            "no blessing below ten permanents"
        );
        let drawn = state.zones.hands[p0.index()].len();
        assert_eq!(drawn, 2, "no blessing → drew two; drew {drawn}");
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            lib_before - 2,
            "two cards left the library"
        );
    }

    /// [CR#702.131a]: NO high-water mark. Reach ten permanents, then drop one
    /// back to nine BEFORE the spell resolves: the gate reads nine at
    /// resolution, so no blessing and a two-card draw. A momentary ten does not
    /// count.
    #[test]
    fn ascend_spell_no_high_water_mark() {
        let (mut state, p0, lib_before) = secrets_on_stack(10);
        let name: deckmaste_core::Ident = "CitysBlessing".into();

        // Drop one permanent (10 → 9) before resolution.
        let dropped = state.zones.battlefield.pop().expect("a permanent to drop");
        state.objects.obj_mut(dropped).zone = None;
        assert_eq!(
            state.zones.battlefield.len(),
            9,
            "back to nine at resolution"
        );

        state
            .agenda
            .push_front(WorkItem::Resolve(state.stack[0].id));
        let _trace = drain_progress(&mut state, 40);

        assert!(
            !state.designations.players.contains_key(&(p0, name)),
            "a momentary ten doesn't grant — only the resolution count matters ([CR#702.131a])"
        );
        let drawn = state.zones.hands[p0.index()].len();
        assert_eq!(drawn, 2, "nine at resolution → drew two; drew {drawn}");
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            lib_before - 2,
            "two cards left the library"
        );
    }
}
