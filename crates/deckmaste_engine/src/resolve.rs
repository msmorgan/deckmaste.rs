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
use deckmaste_core::Normalize;
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
                        subject: None,
                        those: None,
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
                    subject: None,
                    those: None,
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
                        subject: None,
                        those: None,
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

    /// [CR#614.3]: register a floating replacement shield (regeneration, "the
    /// next time …") on `state.shields`. Mutates `&mut self`, so it can't ride
    /// `action_items` (`&self`); the `Action::CreateReplacement` arm of
    /// `run_effect` routes here.
    fn create_shield(
        &mut self,
        replacement: deckmaste_core::Replacement,
        subject: &deckmaste_core::Selection,
        duration: deckmaste_core::Duration,
        one_shot: bool,
        frame: &Frame,
    ) {
        let id = self
            .eval_selection_set(subject, frame)
            .into_iter()
            .next()
            .expect("CreateReplacement subject must resolve to one object");
        let iid = crate::replace_registry::InstanceId(self.next_shield_id);
        self.next_shield_id += 1;
        self.shields
            .push(crate::replace_registry::ReplacementInstance {
                id: iid,
                replacement,
                subject: id,
                duration,
                one_shot,
                source: frame.source,
            });
    }

    /// Interpret one `Effect` node ([CR#608.2]). `Act` becomes one or more
    /// `Emit` work items (via `action_items`); `Sequence` expands to one
    /// `RunEffect` per child.
    ///
    /// # Panics
    ///
    /// Panics on any `Effect` variant not wired for Stage 3.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per effect-frame variant; splitting would scatter the dispatch"
    )]
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
                            self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
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
                // `CreateReplacement` directly mutates `state.shields` — it
                // cannot go through `action_items` (which is `&self`). Handle
                // it here, mirroring how `Effect::Continuously` works.
                if let Action::CreateReplacement {
                    replacement,
                    subject,
                    duration,
                    one_shot,
                } = action
                {
                    self.create_shield(*replacement, &subject, duration, one_shot, frame);
                } else {
                    let items = self.action_items(&action, frame);
                    self.schedule_front(items);
                }
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
            // [CR#608.2]: "for each [over], [do]". The matched set is fixed
            // once — when this node resolves ([CR#608.2h]) — then the inner
            // effect runs once per match with that object bound as `ThatObject`
            // (so a body like `Destroy(ThatObject)` reads "it"). Per-object
            // `RunEffect`s scheduled at the front preserve their order and each
            // pauses independently if the body surfaces a choice.
            Effect::ForEach(fe) => {
                let matches = crate::target::candidates(self, &fe.over);
                let items: Vec<WorkItem> = matches
                    .into_iter()
                    .map(|obj| {
                        let mut next = frame.clone();
                        let mut bindings =
                            next.bindings
                                .clone()
                                .unwrap_or(crate::trigger::TriggerBindings {
                                    this: None,
                                    that_object: None,
                                    that_player: None,
                                });
                        bindings.that_object = Some(crate::lki::LkiSnapshot::capture(self, obj));
                        next.bindings = Some(bindings);
                        WorkItem::RunEffect {
                            effect: fe.effect.clone(),
                            frame: next,
                        }
                    })
                    .collect();
                self.schedule_front(items);
            }
            // Bind the plural anaphor: evaluate `with.selection` at this
            // moment (order preserved, top→down for a library window), then
            // schedule `with.body` under a frame carrying that group as `those`
            // so `Selection::Those` resolves inside the body.
            Effect::With(with) => {
                let group = self.eval_selection_set(&with.selection, frame);
                let mut next = frame.clone();
                next.those = Some(group);
                self.schedule_front(vec![WorkItem::RunEffect {
                    effect: with.body,
                    frame: next,
                }]);
            }
            // [CR#118.12]: "[A player] may [do]. If they do/don't, …". Surface a
            // yes/no to the controller; the chosen branch (effect + if_did on
            // yes, if_not on no) runs when the answer comes back — the `May`
            // continuation in `submit_decision`.
            Effect::May(may) => {
                self.pending = Some(crate::decide::PendingDecision::YesNo {
                    player: frame.controller,
                });
                self.choice = Some(crate::state::ChoiceContinuation::May {
                    may,
                    frame: frame.clone(),
                });
            }
            // [CR#700.2]: a modal effect — choose `count` modes (up to `count`
            // when `up_to`, with repetition when `repeats`), then apply each
            // chosen mode's effect. Per-mode targets/costs are announce-time
            // ([CR#601.2b,700.2c,700.2h]) and unbuilt, so a resolution-time
            // modal handles target/cost-free modes; a mode carrying either is a
            // loud seam.
            Effect::Modal(modal) => {
                if modal
                    .modes
                    .iter()
                    .any(|m| !top_targets(&m.effect).is_empty() || m.cost.is_some())
                {
                    todo!(
                        "engine-resolve-effects seam: modal per-mode targets/costs are \
                         announce-time ([CR#601.2b,700.2c,700.2h])"
                    );
                }
                let options = Uint::try_from(modal.modes.len()).expect("mode count fits Uint");
                let count = self.eval_count(&modal.choose.count, frame);
                let max = if modal.choose.repeats { count } else { count.min(options) };
                let min = if modal.choose.up_to { 0 } else { max };
                self.pending = Some(crate::decide::PendingDecision::ChooseModes {
                    player: frame.controller,
                    options,
                    min,
                    max,
                    repeats: modal.choose.repeats,
                });
                self.choice = Some(crate::state::ChoiceContinuation::Modal {
                    modes: modal.modes,
                    frame: frame.clone(),
                });
            }
            // [CR#118.12a,608.2d]: "[do] unless [who] pays [cost]". Surface a
            // yes/no to the paying player; on yes the cost is paid and `effect`
            // is skipped, on no `effect` happens — branched when the answer
            // returns (the `Unless` continuation). v1 does NOT gate the offer on
            // affordability (a refinement); the runner only offers "pay" when
            // able.
            Effect::Unless(u) => {
                let payer = self.acting_player(&u.who, frame);
                self.pending = Some(crate::decide::PendingDecision::YesNo { player: payer });
                self.choice = Some(crate::state::ChoiceContinuation::Unless {
                    effect: u.effect,
                    who: u.who,
                    // Normalize the authored cost list at this boundary: read is
                    // faithful, so a macro-spliced `unless` cost arrives lumpy
                    // (a nested `CostComponent::Cost`); splice it flat before the
                    // payment walk (`unless_cost_action`) consumes it.
                    unless: deckmaste_core::Cost(u.unless).normalize().0,
                    frame: frame.clone(),
                });
            }
            // [CR#115.1,601.2c]: a target-scoping wrapper. Targets were chosen
            // at announcement and already live in `frame.targets`; for a single
            // top-level wrapper (the only shape today) this node is transparent
            // — descend into the inner effect, exactly like `Expanded`. Nested
            // wrappers would need a per-scope target stack (left a loud seam by
            // falling through to the `other` arm if a wrapper nests).
            Effect::Targeted(te) => self.run_effect(*te.effect, frame),
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
            // `CreateReplacement` directly mutates `state.shields` — it is
            // intercepted in `run_effect` before `action_items` is called.
            // This arm is unreachable by design.
            Action::CreateReplacement { .. } => {
                unreachable!(
                    "CreateReplacement is handled in run_effect before action_items is called"
                )
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
            // [CR#122.1]: place/remove `n` counters of `kind` on each selected
            // object or player proxy. `n == 0` (or an empty selection) is a
            // no-op, so no event fires — a "counter is put on" trigger never
            // sees a zero placement. The cause carries the effect-instruction
            // agent so "you put a counter" reads ([CR#603.2e]-style transition
            // views) resolve to the right controller.
            PlayerAction::PutCounters(sel, kind, count) => {
                let n = self.eval_count(count, frame);
                if n == 0 {
                    return vec![];
                }
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::CounterPlaced {
                        object,
                        // The event carries the resolved Ident name (engine
                        // state is Ident-keyed); the authored ref is a `CounterRef`.
                        kind: kind.0,
                        count: n,
                        cause: Some(crate::event::Cause::put_counters(
                            deckmaste_core::Agency::EffectInstruction,
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
            PlayerAction::RemoveCounters(sel, kind, count) => {
                let n = self.eval_count(count, frame);
                if n == 0 {
                    return vec![];
                }
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::CounterRemoved {
                        object,
                        kind: kind.0,
                        count: n,
                        cause: Some(crate::event::Cause::remove_counters(
                            deckmaste_core::Agency::EffectInstruction,
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
            PlayerAction::Discard { count, what } => {
                // [CR#701.9b]: the actor chooses which cards — surfaced as a
                // decision when the work item applies (the hand may change
                // before then). A named `what` (discard a *specific* card) as a
                // resolution EFFECT is unbuilt; cycling's "discard this card"
                // ([CR#702.29a]) is a COST, paid in `activate.rs`, never here.
                if let Some(sel) = what {
                    todo!("discard a named selection as a resolution effect: {sel:?}");
                }
                let count = self.eval_count(count, frame);
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
                // carries the resolved inline definition; a `TokenSpec::Named`
                // predefined token ([CR#111.10]) resolves to its rules-defined
                // characteristics here.
                let token = match spec {
                    deckmaste_core::TokenSpec::Token(token) => token.clone(),
                    deckmaste_core::TokenSpec::Named(name) => name
                        .resolve()
                        .expect("a Named token in a card resolves to a builtin definition"),
                };
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
            // [CR#614.8,701.19a]: remove all marked damage from each selected
            // object and remove it from combat if it's attacking or blocking.
            // This is the regeneration "heal" clause — its apply zeroes damage
            // and calls `combat.remove_object`.
            PlayerAction::RemoveDamage(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::DamageRemoved { object })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            // Look through a remembered macro invocation.
            PlayerAction::Expanded(e) => self.player_action_items(&e.value, actor, frame),
            // [CR#701.22a]: Scry/Surveil put-back step — player distributes the
            // looked-at group into top/bottom bins.
            PlayerAction::Distribute { group, bins, name } => {
                let window = self.eval_selection_set(group, frame);
                if window.is_empty() {
                    // [CR#701.22b]: scry/surveil 0 — no-op, no decision, no event.
                    return vec![];
                }
                vec![WorkItem::OpenDistribute {
                    player: actor,
                    window,
                    bins: bins.clone(),
                    name: name.clone(),
                }]
            }
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
            // The ordered plural group bound by the enclosing `Effect::With`.
            // Order-preserved exactly as set by `With` (top→down for a library
            // window). Panics outside an enclosing `With` — always a bug.
            Selection::Those => frame
                .those
                .clone()
                .expect("Selection::Those outside an enclosing With"),
            // The top `count` cards of `of`'s library, front-to-back (top→down).
            // `of: You` resolves to `frame.controller`'s library. Other player
            // references (e.g. Fateseal's opponent) are deferred to Task 9.
            Selection::TopOfLibrary { count, of } => {
                let controller = match of {
                    deckmaste_core::Reference::You => frame.controller,
                    other => {
                        todo!("Fateseal opponent ref — Task 9: non-You TopOfLibrary.of ({other:?})")
                    }
                };
                let n = self.eval_count(count, frame) as usize;
                self.zones.libraries[controller.index()]
                    .iter()
                    .take(n)
                    .copied()
                    .collect()
            }
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
            // The candidate a `Filter::Where` is matching — bound by that arm
            // alone. Referenced anywhere else (no enclosing filter match) it is
            // a malformed read, like a frameless carrier reference.
            Reference::Subject => frame
                .subject
                .expect("Reference::Subject referenced outside a Filter::Where match"),
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
                    // [CR#122.1e,122.1g]: a planeswalker's loyalty IS its
                    // number of loyalty counters; a battle's defense IS its
                    // defense counters — read straight off the counter map (the
                    // counter machinery this ticket built). The PLACEMENT of
                    // those counters (a planeswalker/battle enters with its
                    // printed value, [CR#209.1,210.1]) is the separate
                    // planeswalker/battle-modeling work; until then this reads 0.
                    deckmaste_core::Stat::Loyalty => deckmaste_core::Int::try_from(
                        self.objects
                            .obj(id)
                            .counters
                            .get("LoyaltyCounter")
                            .copied()
                            .unwrap_or(0),
                    )
                    .expect("loyalty fits Int"),
                    deckmaste_core::Stat::Defense => deckmaste_core::Int::try_from(
                        self.objects
                            .obj(id)
                            .counters
                            .get("DefenseCounter")
                            .copied()
                            .unwrap_or(0),
                    )
                    .expect("defense fits Int"),
                };
                Uint::try_from(value.max(0)).expect("clamped stat fits Uint")
            }
            // [CR#122.1]: the count of a counter kind on the resolved
            // object/player proxy, read off the raw counter map (not the
            // derived view — counter quantities are base state, so no layers
            // recursion). An absent kind is zero.
            //
            // [CR#603.10a] LKI: a dies/leaves trigger reads "for each +1/+1
            // counter on this permanent" AFTER the object is gone (Modular,
            // [CR#702.43a]). `This`/`ThatObject` then resolves to the firing
            // object's now-stale id; the live object store no longer holds it,
            // so the count comes from the trigger's last-known snapshot instead.
            Count::CounterCount(reference, kind) => {
                let id = self.eval_reference(reference, frame);
                match self.objects.get(id) {
                    Some(o) => o.counters.get(kind.as_str()).copied().unwrap_or(0),
                    None => lki_counters(reference, frame)
                        .and_then(|c| c.get(kind.as_str()).copied())
                        .unwrap_or(0),
                }
            }
            // [CR#704.5q]: the lesser of two magnitudes (annihilation removes
            // the smaller of the two counter counts of each kind).
            Count::Min(a, b) => self.eval_count(a, frame).min(self.eval_count(b, frame)),
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
            // [CR#608.2i]: count history facts matching `event` within `within` —
            // the count-valued twin of `Condition::Happened`. Reuses the trigger
            // event-matcher, anchored to the frame's watcher (so `Ref(This)` in the
            // pattern resolves to the evaluating source).
            Count::EventCount(event, within) => {
                // Self/object-scoped ability-use count: resolve `by` to a
                // concrete `ObjectId` via the frame and count `AbilityUsed`
                // for that object — the generic matcher can't, since its
                // watcher is an `ObjectSource`, not the resolved id
                // ([CR#608.2i,603.2]).
                let n = if let deckmaste_core::Event::Used { by } = &**event {
                    let obj = self.eval_reference(by, frame);
                    self.history
                        .scan(*within, self.turn.turn_number)
                        .filter(|fact| {
                            matches!(fact, GameEvent::AbilityUsed { object, .. } if *object == obj)
                        })
                        .count()
                } else {
                    let watcher = self.frame_watcher(frame);
                    self.history
                        .scan(*within, self.turn.turn_number)
                        .filter(|fact| self.event_matches(event, fact, watcher))
                        .count()
                };
                Uint::try_from(n).expect("event count fits Uint")
            }
            // [CR#608.2i,119.3]: sum the carried amount of history facts that
            // match `event` within `within` — the sum-valued twin of
            // `EventCount`. Reuses the trigger event-matcher and extracts each
            // matching fact's magnitude via `game_event_amount`.
            Count::EventSum(event, within) => {
                let watcher = self.frame_watcher(frame);
                self.history
                    .scan(*within, self.turn.turn_number)
                    .filter(|fact| self.event_matches(event, fact, watcher))
                    .map(Self::game_event_amount)
                    .sum()
            }
            Count::Noted(key) => todo!("P0.W4: noted read {key:?} (slot store is P0.W5)"),
            Count::Expanded(e) => self.eval_count(&e.value, frame),
        }
    }

    /// The magnitude carried by an amount-bearing history fact ([CR#119.3]).
    /// Returns `0` for facts with no scalar amount.
    fn game_event_amount(fact: &GameEvent) -> Uint {
        match fact {
            GameEvent::LifeLost { amount, .. } | GameEvent::LifeGained { amount, .. } => *amount,
            _ => 0,
        }
    }

    /// Lands `player` has played this turn ([CR#305.2,608.2i]) — backing the
    /// one-land-per-turn rule. A play is a `ZoneChanged` to the battlefield
    /// whose cause verb is `Play`, by `player`.
    ///
    /// # Panics
    ///
    /// Panics only if the count exceeds `Uint` — unreachable in a real game.
    #[must_use]
    pub fn lands_played_this_turn(&self, player: crate::player::PlayerId) -> Uint {
        use deckmaste_core::Zone;

        use crate::event::GameEvent;
        let play = deckmaste_core::Ident::from("Play");
        let n = self
            .history
            .scan(deckmaste_core::Window::ThisTurn, self.turn.turn_number)
            .filter(|f| {
                matches!(f,
                    GameEvent::ZoneChanged { to: Zone::Battlefield, cause: Some(c), snapshot, .. }
                        if c.verb == play && snapshot.controller == player)
            })
            .count();
        Uint::try_from(n).expect("land count fits Uint")
    }

    /// Count of `AbilityUsed` events for the given `(object, ability)` pair
    /// within `within` — the primitive backing per-instance use-limit gates
    /// ([CR#602.5b,603.2h]) and history reads ([CR#608.2i]).
    ///
    /// `within` must be a history-lookback window (`ThisTurn` or `ThisGame`);
    /// timing windows produce 0 (same defensive contract as
    /// [`History::scan`]).
    pub(crate) fn ability_used_count(
        &self,
        object: crate::object::ObjectId,
        ability: Uint,
        within: deckmaste_core::Window,
    ) -> Uint {
        let n = self
            .history
            .scan(within, self.turn.turn_number)
            .filter(|f| {
                matches!(f,
                    GameEvent::AbilityUsed { object: o, ability: a }
                        if *o == object && *a == ability)
            })
            .count();
        Uint::try_from(n).expect("ability use count fits Uint")
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
            // (possibly gone, possibly changed) source. Targets live on a
            // top-level `Effect::Targeted` wrapper ([CR#115.1,601.2c]).
            StackObject::Activated { ability, .. } => top_targets(&ability.effect).to_vec(),
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

/// The last-known counter map for a `This`/`ThatObject` reference whose object
/// is gone ([CR#603.10a]): the matching snapshot the fired trigger carried.
/// `None` when the frame has no bindings (a spell frame — a spell's object is
/// always live as it resolves) or the reference isn't a trigger-bound self.
fn lki_counters<'f>(
    reference: &Reference,
    frame: &'f Frame,
) -> Option<&'f std::collections::HashMap<deckmaste_core::Ident, Uint>> {
    let bindings = frame.bindings.as_ref()?;
    let snapshot = match reference {
        Reference::This => bindings.this.as_ref(),
        Reference::ThatObject => bindings.that_object.as_ref(),
        _ => None,
    }?;
    Some(&snapshot.counters)
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
            | PlayerAction::PutInLibrary(s, _)
            | PlayerAction::RemoveDamage(s) => lift(s),
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
        // `CreateReplacement.subject` could be a Choose; inspect it.
        Action::CreateReplacement { subject, .. } => lift(subject),
    }
}

/// The `SpellAbility.targets` of the spell (empty for permanent spells).
/// Used by the cast checks and `targets_still_legal`. Reads the caller's
/// derived view — the legality loop checks every hand card against one
/// view instead of re-deriving the board per card.
#[must_use]
pub(crate) fn spell_targets(view: &crate::layer::LayeredView, id: ObjectId) -> Vec<TargetSpec> {
    // Targets live on a top-level `Effect::Targeted` wrapper in the spell
    // ability's effect ([CR#115.1,601.2c]).
    view.get(id)
        .abilities
        .iter()
        .find_map(spell_ability_effect)
        .map_or_else(Vec::new, |e| top_targets(e).to_vec())
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

/// The targets declared on a top-level `Targeted` wrapper (peeling
/// `Expanded`), or `&[]` when the effect isn't a wrapper — the announce-list
/// home after the migration ([CR#115.1,601.2c]). A single top-level wrapper is
/// the only shape today; a nested wrapper would need a per-scope target stack.
pub(crate) fn top_targets(effect: &Effect) -> &[TargetSpec] {
    match effect {
        Effect::Targeted(te) => &te.targets,
        Effect::Expanded(e) => top_targets(&e.value),
        _ => &[],
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

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
        vec![Arc::clone(card); n]
    }

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
            .find(|&&o| obj_matches(state, o, &Filter::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != theirs);
        state.objects.obj_mut(theirs).zone = Some(Zone::Battlefield);
        state.objects.obj_mut(theirs).controller = PlayerId(1);
        state.zones.battlefield.push(theirs);
        theirs
    }

    /// History tallies via `EventCount`/`EventSum` — the general primitives
    /// that subsume the old `Count::Query`/`eval_query` scalar family
    /// ([CR#608.2i]). Fixtures are the same events; assertions use the
    /// replacements.
    #[expect(
        clippy::too_many_lines,
        reason = "one fixture exercises all five history tallies (storm/draws/lands/life-lost/life-gained) end-to-end"
    )]
    #[test]
    fn history_tallies_via_event_count_sum() {
        use deckmaste_core::Agency;
        use deckmaste_core::Count;
        use deckmaste_core::Event;
        use deckmaste_core::Filter;
        use deckmaste_core::Reference;
        use deckmaste_core::Window;

        use crate::event::Cause;
        use crate::lki::LkiSnapshot;
        use crate::object::ObjectSource;

        let mut state = game();
        state.turn.turn_number = 1;
        let p = PlayerId(0);
        // A frame anchored on player p — Ref(You) resolves to p's proxy.
        let frame = frame_for(&state, p);

        // Three spells cast this turn (game-wide); `EventCount(Cast, ThisTurn)`
        // returns the FULL count (3). The storm "−1/before this one"
        // self-exclusion is deferred until a storm card exists — no card
        // consumes it yet ([CR#702.40a]).
        // Mint real objects for the Cast path (performed_matches reads their
        // controllers via objects.obj, which panics on stale IDs).
        let sp1 = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp2 = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp3 = state.objects.mint(ObjectSource::Player(p), p, None);
        state.history.record(1, GameEvent::SpellCast(sp1));
        state.history.record(1, GameEvent::SpellCast(sp2));
        state.history.record(1, GameEvent::SpellCast(sp3));
        let cast_event = Event::Performed {
            verb: "Cast".into(),
            by: Filter::Any,
            on: Filter::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(cast_event), Window::ThisTurn),
                &frame
            ),
            3,
            "storm: all casts this turn (full count; −1 self-exclusion deferred)"
        );

        // Two draws by p this turn → EventCount(Draw, by: Ref(You)) = 2.
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
        let draw_event = Event::Performed {
            verb: "Draw".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(draw_event), Window::ThisTurn),
                &frame
            ),
            2,
            "draws by p this turn"
        );

        // One land played by p (a Play-caused battlefield entry).
        // `lands_played_this_turn` is the direct helper; EventCount(Play,
        // by: Ref(You)) is the generic equivalent.
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
        assert_eq!(
            state.lands_played_this_turn(p),
            1,
            "lands played by p this turn (direct helper)"
        );
        let play_event = Event::Performed {
            verb: "Play".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(play_event), Window::ThisTurn),
                &frame
            ),
            1,
            "lands played by p via EventCount"
        );

        // Life: lost 3 then 2 (=5), gained 4.
        // EventSum(LoseLife, by: Ref(You)) sums the amounts; EventSum(GainLife)
        // likewise ([CR#119.3]).
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
        let lose_event = Event::Performed {
            verb: "LoseLife".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        let gain_event = Event::Performed {
            verb: "GainLife".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_event), Window::ThisTurn),
                &frame
            ),
            5,
            "life lost by p this turn"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(gain_event), Window::ThisTurn),
                &frame
            ),
            4,
            "life gained by p this turn"
        );

        // Prior-turn entries are excluded once the turn advances.
        state.turn.turn_number = 2;
        let cast_event2 = Event::Performed {
            verb: "Cast".into(),
            by: Filter::Any,
            on: Filter::Any,
        };
        let draw_event2 = Event::Performed {
            verb: "Draw".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(cast_event2), Window::ThisTurn),
                &frame
            ),
            0,
            "storm resets on new turn"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(draw_event2), Window::ThisTurn),
                &frame
            ),
            0,
            "draws reset on new turn"
        );
    }

    /// `ability_used_count` counts `AbilityUsed` events keyed by (object,
    /// ability) and respects the `Window` filter — `ThisTurn` excludes
    /// prior-turn entries, `ThisGame` includes them all.
    #[test]
    fn ability_used_count_keys_object_ability_window() {
        use deckmaste_core::Window;

        let mut state = game();
        state.turn.turn_number = 1;

        let obj_a = ObjectId::from_raw(10);
        let obj_b = ObjectId::from_raw(20);

        // Two uses of ability 0 on obj_a, one use of ability 1 on obj_a —
        // all on the current turn.
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 0,
            },
        );
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 0,
            },
        );
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 1,
            },
        );

        assert_eq!(state.ability_used_count(obj_a, 0, Window::ThisGame), 2);
        assert_eq!(state.ability_used_count(obj_a, 1, Window::ThisGame), 1);
        // obj_b has no uses recorded.
        assert_eq!(state.ability_used_count(obj_b, 0, Window::ThisGame), 0);

        // A use of (obj_a, 0) on a DIFFERENT turn.
        state.history.record(
            2,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 0,
            },
        );

        // ThisTurn (still turn 1) excludes the turn-2 entry.
        assert_eq!(state.ability_used_count(obj_a, 0, Window::ThisTurn), 2);
        // ThisGame includes it.
        assert_eq!(state.ability_used_count(obj_a, 0, Window::ThisGame), 3);
    }

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
            .find(|&&o| obj_matches(&state, o, &Filter::creature()))
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
            .find(|&&o| obj_matches(&state, o, &Filter::creature()))
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
                what: Selection::this(),
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
                what: Selection::this(),
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
                what: Selection::this(),
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
                what: Selection::this(),
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
        state.run_effect(Effect::Act(Action::Unattach(Selection::this())), &frame);
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
        state.run_effect(Effect::Act(Action::Unattach(Selection::this())), &frame);
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
            Filter::creature(),
        ]);
        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
            chosen: Some(vec![bear]),
            x: None,
            subject: None,
            those: None,
        };
        let sel = Selection::Choose(Quantity::one(), creatures);
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
            Filter::creature(),
        ]);
        let frame = frame_src(bear);
        let before = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(before, 2);

        state.run_effect(
            Effect::Act(Action::Destroy(Selection::Random(
                Quantity::one(),
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
            Filter::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            Effect::Act(Action::Destroy(Selection::Choose(
                Quantity::one(),
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
            subject: None,
            those: None,
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
    /// `Destroy` action's `WillDestroy` intent is suppressed by the
    /// event-side cant pass ([CR#614.17]) in `apply_occurrence`, so the
    /// Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_destroy_action() {
        let (mut state, myr) = myr_on_field();
        let frame = frame_src(myr);
        state.run_effect(Effect::Act(Action::Destroy(Selection::this())), &frame);
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
        state.run_effect(Effect::Act(Action::Destroy(Selection::this())), &frame);
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
            Effect::Act(Action::Move(Selection::this(), Zone::Graveyard)),
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
        let items = state.action_items(
            &Action::by_you(PlayerAction::Tap(Selection::this())),
            &frame,
        );
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
        let items = state.action_items(
            &Action::by_you(PlayerAction::Draw(Count::Literal(2))),
            &frame,
        );
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                player: PlayerId(0),
                ..
            }))
        )));

        // By(You, LoseLife(3)) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(
            &Action::by_you(PlayerAction::LoseLife(Count::Literal(3))),
            &frame,
        );
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
        let items = state.action_items(
            &Action::by_you(PlayerAction::Tap(Selection::this())),
            &frame,
        );
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
        let items = state.action_items(
            &Action::by_you(PlayerAction::Untap(Selection::this())),
            &frame,
        );
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
            Filter::creature(),
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
                Effect::act_by_you(PlayerAction::GainLife(Count::ThatMuch)),
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

    /// A `Targeted` wrapper is transparent at resolution — the inner
    /// instruction runs with `frame.targets` already bound, so `Target(0)`
    /// resolves and damage lands ([CR#115.1,608]).
    #[test]
    fn targeted_effect_resolves_its_inner_effect() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);
        state.run_effect(
            Effect::Targeted(deckmaste_core::Targeted::new(
                vec![],
                Effect::Act(Action::DealDamage(
                    Selection::Ref(Reference::Target(0)),
                    Count::Literal(3),
                )),
            )),
            &frame,
        );
        // RunEffect(Targeted) → RunEffect(DealDamage) → Emit(DamageDealt).
        for _ in 0..3 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).damage, 3);
    }

    /// `top_targets` reads the targets off a top-level `Targeted` (peeling
    /// `Expanded`) and returns empty for a non-wrapper effect
    /// ([CR#115.1,601.2c]).
    #[test]
    fn top_targets_reads_wrapper_and_peels_expanded() {
        let spec =
            deckmaste_core::TargetSpec::Target(deckmaste_core::Quantity::one(), Filter::creature());
        let wrapped = Effect::Targeted(deckmaste_core::Targeted::new(
            vec![spec.clone()],
            Effect::Act(Action::DealDamage(
                Selection::Ref(Reference::Target(0)),
                Count::Literal(3),
            )),
        ));
        assert_eq!(super::top_targets(&wrapped), std::slice::from_ref(&spec));
        let bare = Effect::Act(Action::DealDamage(
            Selection::Ref(Reference::Target(0)),
            Count::Literal(1),
        ));
        assert!(super::top_targets(&bare).is_empty());
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
            subject: None,
            those: None,
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
            Filter::creature(),
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
                Filter::creature(),
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
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Effect;
        use deckmaste_core::Filter;
        use deckmaste_core::Modification;
        use deckmaste_core::Scope;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        assert!(state.continuous.is_empty(), "no effects before resolve");

        let filter = Filter::creature();
        let effect = Effect::Continuously(Continuously {
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
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Effect;
        use deckmaste_core::Modification;
        use deckmaste_core::Reference;
        use deckmaste_core::Scope;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        let effect = Effect::Continuously(Continuously {
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

        let items = state.action_items(
            &Action::by_you(PlayerAction::GainLife(Count::Literal(3))),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 3,
            }))]
        );

        let items = state.action_items(
            &Action::by_you(PlayerAction::Untap(Selection::this())),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Untapped(src)))]
        );
    }

    /// [CR#122.1]: `PutCounters(This, P1P1Counter, 2)` emits one `CounterPlaced`
    /// per selected object, carrying the effect-instruction cause
    /// (events.md §3) — its agent is the resolving source's controller.
    /// Counter kinds are bare `CounterRef` idents, not symbolic strings.
    #[test]
    fn put_counters_emits_counter_placed() {
        use deckmaste_core::Agency;

        use crate::event::Cause;

        let (state, bear) = bear_on_field();
        let frame = frame_src(bear);
        let items = state.action_items(
            &Action::by_you(PlayerAction::PutCounters(
                Selection::this(),
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(
                GameEvent::CounterPlaced {
                    object: bear,
                    kind: "P1P1Counter".into(),
                    count: 2,
                    cause: Some(Cause::put_counters(
                        Agency::EffectInstruction,
                        Some((bear, PlayerId(0))),
                    )),
                }
            ))]
        );
    }

    /// [CR#122.1]: putting zero counters is a no-op — no event (so no
    /// "counter is put on" trigger fires for nothing).
    #[test]
    fn put_zero_counters_emits_nothing() {
        let (state, bear) = bear_on_field();
        let frame = frame_src(bear);
        let items = state.action_items(
            &Action::by_you(PlayerAction::PutCounters(
                Selection::this(),
                "P1P1Counter".into(),
                Count::Literal(0),
            )),
            &frame,
        );
        assert_eq!(items, vec![]);
    }

    /// Applying `CounterPlaced` adds to the object's counter map, and a second
    /// placement of the same kind sums ([CR#122.1] — counters are
    /// interchangeable).
    #[test]
    fn counter_placed_apply_is_additive() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 1);
        let frame = frame_src(bear);
        state.run_effect(
            Effect::act_by_you(PlayerAction::PutCounters(
                Selection::this(),
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterPlaced
        assert_eq!(
            state
                .objects
                .obj(bear)
                .counters
                .get(&deckmaste_core::Ident::from("P1P1Counter"))
                .copied(),
            Some(3)
        );
    }

    /// Removing more counters than present clamps to zero and DROPS the key,
    /// so `HasCounter` and the layer-7c P/T read both see absence ([CR#122.1]).
    #[test]
    fn counter_removed_clamps_and_drops_key() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 1);
        let frame = frame_src(bear);
        state.run_effect(
            Effect::act_by_you(PlayerAction::RemoveCounters(
                Selection::this(),
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterRemoved
        assert!(
            !state
                .objects
                .obj(bear)
                .counters
                .contains_key(&deckmaste_core::Ident::from("P1P1Counter")),
            "a counter kind dropped to zero leaves no key behind"
        );
    }

    /// [CR#122.1]: `Count::CounterCount(ref, kind)` reads how many `kind`
    /// counters sit on the resolved object/player proxy; an absent kind is 0.
    /// Counter kinds are rusty idents (`P1P1Counter`), not symbolic strings.
    #[test]
    fn counter_count_reads_the_objects_counter_map() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let frame = frame_src(bear);
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "P1P1Counter".into()),
                &frame
            ),
            3
        );
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "M1M1Counter".into()),
                &frame
            ),
            0,
            "an absent counter kind reads as zero"
        );
    }

    /// [CR#603.10a,702.43a]: when the object a `CounterCount(This, _)` names is
    /// GONE (a dies trigger — Modular's "for each +1/+1 counter on this
    /// permanent" resolves after the creature left the battlefield), the count
    /// comes from the trigger's last-known snapshot, not the stale id. Without
    /// the LKI bridge `eval_count` would panic dereferencing the dead object.
    #[test]
    fn counter_count_reads_lki_when_the_object_is_gone() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 2);
        // Snapshot the creature, then remove it — `bear` is now a stale id, the
        // exact state a dies trigger's `This` resolves to ([CR#603.10a]).
        let snapshot = crate::lki::LkiSnapshot::capture(&state, bear);
        state.objects.remove(bear);
        assert!(state.objects.get(bear).is_none(), "the object is gone");

        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: Vec::new(),
            bindings: Some(crate::trigger::TriggerBindings {
                this: Some(snapshot),
                that_object: None,
                that_player: None,
            }),
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };

        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "P1P1Counter".into()),
                &frame
            ),
            2,
            "the dying creature's last-known +1/+1 counter count"
        );
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "M1M1Counter".into()),
                &frame
            ),
            0,
            "an absent kind on the snapshot reads as zero"
        );
    }

    /// [CR#122.1e]: `StatOf(_, Loyalty)` reads the object's loyalty-counter
    /// count off the counter map this ticket built (closing the
    /// engine-resolve-count-x seam; placement on entry is planeswalker work).
    #[test]
    fn stat_of_loyalty_reads_loyalty_counters() {
        use deckmaste_core::Stat;

        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("LoyaltyCounter".into(), 4);
        let frame = frame_src(bear);
        assert_eq!(
            state.eval_count(&Count::StatOf(Reference::This, Stat::Loyalty), &frame),
            4
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
            Effect::act_by_you(PlayerAction::Sacrifice(Selection::this())),
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
            Effect::act_by_you(PlayerAction::Sacrifice(Selection::this())),
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
        state.run_effect(
            Effect::act_by_you(PlayerAction::Exile(Selection::this())),
            &frame,
        );
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
        state.run_effect(
            Effect::act_by_you(PlayerAction::Exile(Selection::this())),
            &frame,
        );
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
        state.run_effect(Effect::Act(Action::ReturnToHand(Selection::this())), &frame);
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
        state.run_effect(Effect::Act(Action::ReturnToHand(Selection::this())), &frame);
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
            Effect::act_by_you(PlayerAction::PutInLibrary(
                Selection::this(),
                Count::Literal(0),
            )),
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
            Effect::act_by_you(PlayerAction::PutInLibrary(
                Selection::this(),
                Count::Literal(99),
            )),
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
            Effect::act_by_you(PlayerAction::AddMana(
                Count::Literal(2),
                ManaSpec::Specific(green).into(),
            )),
            &frame,
        );
        let _ = state.step();
        assert_eq!(state.players[0].mana_pool.amount(green), 2);

        state.run_effect(
            Effect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::AnyColor.into(),
            )),
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
            Effect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaProduction::WithRiders {
                    mana: ManaSpec::Specific(red),
                    riders: vec![rider],
                },
            )),
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
            Effect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(2),
                what: None,
            }),
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
            Effect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(99),
                what: None,
            }),
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
            state.action_items(&Action::by_you(expanded), &frame),
            state.action_items(&Action::by_you(body), &frame),
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
            Effect::act_by_you(PlayerAction::Create(Count::Literal(2), token.into())),
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
                obj_matches(&state, t, &Filter::type_(Type::Artifact)),
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
                Effect::act_by_you(PlayerAction::Create(
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
                )),
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

    /// A `Create(_, Named(Treasure))` resolves the predefined token
    /// ([CR#111.10a]) from its rules-defined characteristics — no plugin handle
    /// needed — and puts a real Treasure-subtyped token onto the battlefield.
    #[test]
    fn create_named_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        state.run_effect(
            Effect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Named(deckmaste_core::TokenName::from("Treasure")),
            )),
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
        assert!(
            state
                .cards
                .get(card)
                .subtypes
                .iter()
                .any(|s| s.name == "Treasure"),
            "[CR#111.10a]: the resolved Named token carries the Treasure subtype"
        );
        assert!(state.cards.get(card).is_token, "[CR#111.6]");
    }

    /// The builtin predefined Treasure token ([CR#111.10a]) creates with its
    /// declared subtype and the [CR#111.4] default name (subtypes + "Token").
    #[test]
    fn create_builtin_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let treasure = builtin().token("Treasure").unwrap();
        state.run_effect(
            Effect::act_by_you(PlayerAction::Create(Count::Literal(1), treasure.into())),
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
                what: Selection::this(),
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
        state.run_effect(Effect::Act(Action::Destroy(Selection::this())), &frame);
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
                what: Selection::this(),
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
                what: Selection::this(),
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
        state.run_effect(Effect::Act(Action::Unattach(Selection::this())), &frame);
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
        use deckmaste_core::If;

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
            Effect::If(If {
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
            Effect::If(If {
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
            Effect::If(If {
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

    /// [CR#608.2]: `Effect::ForEach` evaluates `over` once at resolution and
    /// runs the inner effect once per matched object, binding each iterated
    /// object as `ThatObject` (a per-iteration `bindings.that_object`). Proven
    /// via `Destroy(ThatObject)` over the battlefield creatures: every creature
    /// dies, which can only happen if each iteration's `ThatObject` resolves to
    /// that iteration's object.
    #[test]
    fn run_effect_foreach_binds_each_match_as_that_object() {
        use deckmaste_core::ForEach;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let creatures = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            Effect::ForEach(ForEach {
                over: creatures,
                effect: Box::new(Effect::Act(Action::Destroy(Selection::Ref(
                    Reference::ThatObject,
                )))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 80);
        assert!(
            !state.zones.battlefield.contains(&bear) && !state.zones.battlefield.contains(&theirs),
            "every iterated creature is destroyed via its ThatObject binding"
        );
    }

    /// [CR#608.2]: `Effect::ForEach` runs the inner effect once per match — a
    /// non-binding body (gain 1 life) over two creatures gains 2 life.
    #[test]
    fn run_effect_foreach_runs_once_per_match() {
        use deckmaste_core::ForEach;

        let (mut state, bear) = bear_on_field();
        let _theirs = second_bear_to_player_1(&mut state);
        let creatures = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::creature(),
        ]);
        let frame = frame_src(bear);
        let life0 = state.player(PlayerId(0)).life;
        state.run_effect(
            Effect::ForEach(ForEach {
                over: creatures,
                effect: Box::new(Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::GainLife(Count::Literal(1)),
                ))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 80);
        assert_eq!(
            state.player(PlayerId(0)).life,
            life0 + 2,
            "two creatures → inner effect runs twice"
        );
    }

    /// [CR#118.12]: `Effect::May` surfaces a yes/no to the controller. Yes runs
    /// `effect` then `if_did`; no runs `if_not` (nothing when absent). Driven
    /// via `GainLife` so each branch reads as a clean life delta.
    #[test]
    fn run_effect_may_branches_on_the_answer() {
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let gain = |n| {
            Effect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            ))
        };
        let may = || May {
            effect: Box::new(gain(3)),
            if_did: Some(Box::new(gain(10))),
            if_not: Some(Box::new(gain(1))),
        };
        let p0 = PlayerId(0);

        // yes → effect (3) + if_did (10) = +13; surfaces YesNo to the controller.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Effect::May(may()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the controller decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 13, "yes → effect + if_did");

        // no → if_not (1) = +1.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Effect::May(may()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "no → if_not");

        // no + no if_not → nothing.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Effect::May(May {
                effect: Box::new(gain(3)),
                if_did: None,
                if_not: None,
            }),
            &frame,
        );
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0, "no + no if_not → no change");
    }

    /// [CR#700.2]: `Effect::Modal` surfaces `ChooseModes`; the chosen modes'
    /// effects run in written order. "Choose one" of three life-gain modes —
    /// picking index 1 gains 5; "choose two" runs both picks (+3+7); bad picks
    /// (too many, out of range) are rejected.
    #[test]
    fn run_effect_modal_runs_chosen_modes() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let gain_mode = |n| Mode {
            effect: Effect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            )),
            cost: None,
        };
        let modes = || vec![gain_mode(3), gain_mode(5), gain_mode(7)];
        let spec = |count, up_to| ChooseSpec {
            count: Count::Literal(count),
            up_to,
            repeats: false,
        };
        let p0 = PlayerId(0);

        // choose one → ChooseModes(options 3, [1,1]); pick mode 1 → +5.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Effect::Modal(Modal {
                choose: spec(1, false),
                modes: modes(),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(PendingDecision::ChooseModes {
            player,
            options,
            min,
            max,
            repeats,
        }) = state.step()
        else {
            panic!("expected ChooseModes, got {:?}", state.pending);
        };
        assert_eq!((player, options, min, max, repeats), (p0, 3, 1, 1, false));
        assert!(
            state.submit_decision(Decision::Modes(vec![0, 1])).is_err(),
            "too many modes"
        );
        assert!(
            state.submit_decision(Decision::Modes(vec![5])).is_err(),
            "mode index out of range"
        );
        state.submit_decision(Decision::Modes(vec![1])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 5,
            "chosen mode's effect runs"
        );

        // choose two → both picks run (+3 +7 = +10).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Effect::Modal(Modal {
                choose: spec(2, false),
                modes: modes(),
            }),
            &frame,
        );
        state.submit_decision(Decision::Modes(vec![0, 2])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "both chosen modes run");
    }

    /// [CR#118.12a,608.2d]: `Effect::Unless` lets the payer (`who`, default You)
    /// choose to pay the `unless` cost to avoid `effect`. Paying runs the cost
    /// and skips the effect; declining runs the effect. Driven via a `LoseLife`
    /// cost + `GainLife` effect so each branch is a clean life delta.
    #[test]
    fn run_effect_unless_pays_or_suffers_the_effect() {
        use deckmaste_core::CostComponent;
        use deckmaste_core::Unless;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let unless = || Unless {
            who: Reference::You,
            effect: Box::new(Effect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(10)),
            ))),
            unless: vec![CostComponent::do_(PlayerAction::LoseLife(Count::Literal(
                2,
            )))],
        };

        // "I'll pay" → lose 2 life, effect (gain 10) skipped; YesNo to the payer.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Effect::Unless(unless()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 - 2,
            "pay → cost paid, effect skipped"
        );

        // "won't pay" → effect runs (gain 10).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Effect::Unless(unless()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "decline → effect runs");
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
        use deckmaste_core::If;

        Effect::Sequence(vec![
            Effect::If(If {
                condition: ascend_gate(),
                then: Box::new(Effect::act_by_you(PlayerAction::GetDesignation(
                    "CitysBlessing".into(),
                ))),
                otherwise: None,
            }),
            Effect::If(If {
                condition: Condition::Is(
                    Reference::You,
                    Filter::State(StateFilter::Designated("CitysBlessing".into())),
                ),
                then: Box::new(Effect::act_by_you(PlayerAction::Draw(Count::Literal(3)))),
                otherwise: Some(Box::new(Effect::act_by_you(PlayerAction::Draw(
                    Count::Literal(2),
                )))),
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

    /// `Count::EventCount` is the count-valued twin of `Condition::Happened`:
    /// it scans the history log within the given window and returns how many
    /// facts match the `Event` pattern via `event_matches` ([CR#608.2i]).
    /// Two creature-death facts recorded this turn → count == 2; a non-matching
    /// pattern (zone-enter) or a turn with no facts → count == 0.
    #[test]
    fn event_count_counts_matching_history() {
        use deckmaste_core::Event;
        use deckmaste_core::Window;

        use crate::lki::LkiSnapshot;
        use crate::object::ObjectSource;

        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
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
        state.turn.turn_number = 1;

        // Build two creature-death GameEvents (same shape as the morbid test).
        let first_bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let first_bear = state.objects.mint(
            ObjectSource::Card(first_bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(first_bear);
        let death1 = GameEvent::ZoneChanged {
            snapshot: LkiSnapshot::capture(&state, first_bear),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            face: None,
            cause: None,
        };

        let second_bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let second_bear = state.objects.mint(
            ObjectSource::Card(second_bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(second_bear);
        let death2 = GameEvent::ZoneChanged {
            snapshot: LkiSnapshot::capture(&state, second_bear),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            face: None,
            cause: None,
        };

        // The creature-death event pattern (same as morbid Condition::Happened).
        let death_pattern = Event::ZoneMove {
            what: Filter::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            face: None,
            cause: None,
        };

        // A non-matching pattern: creatures entering the battlefield.
        let enter_pattern = Event::ZoneMove {
            what: Filter::creature(),
            from: None,
            to: Some(Zone::Battlefield),
            face: None,
            cause: None,
        };

        let frame = frame_for(&state, PlayerId(0));

        // No deaths recorded yet → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern.clone()), Window::ThisTurn),
                &frame
            ),
            0,
            "no deaths recorded yet"
        );

        // Record two deaths this turn.
        state.history.record(1, death1);
        state.history.record(1, death2);

        // Both deaths match → 2.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern.clone()), Window::ThisTurn),
                &frame
            ),
            2,
            "two creature deaths this turn"
        );

        // A non-matching pattern → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(enter_pattern), Window::ThisTurn),
                &frame
            ),
            0,
            "enter pattern does not match death facts"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame sees 2.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern.clone()), Window::ThisTurn),
                &frame
            ),
            0,
            "ThisTurn no longer sees last turn's deaths"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern), Window::ThisGame),
                &frame
            ),
            2,
            "ThisGame still sees last turn's deaths"
        );
    }

    /// `EventSum` sums the `amount` field of matching `LifeLost` facts within
    /// the window ([CR#608.2i,119.3]). Two losses of 2 and 3 by the same player
    /// total 5; a third loss by an opponent does not contribute. After a turn
    /// advance `ThisTurn` reads 0 while `ThisGame` still reads 5.
    #[test]
    fn event_sum_totals_amounts() {
        use deckmaste_core::Event;
        use deckmaste_core::Window;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        state.turn.turn_number = 1;

        let frame = frame_for(&state, PlayerId(0));

        let lose_life_pattern = Event::Performed {
            verb: "LoseLife".into(),
            by: deckmaste_core::Filter::Ref(deckmaste_core::Reference::You),
            on: deckmaste_core::Filter::Any,
        };

        // No facts yet → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern.clone()), Window::ThisTurn),
                &frame
            ),
            0,
            "no life-loss facts yet"
        );

        // Record two life-loss facts for player 0 (you) and one for player 1
        // (opponent).
        state.history.record(
            1,
            GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 2,
            },
        );
        state.history.record(
            1,
            GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            },
        );
        state.history.record(
            1,
            GameEvent::LifeLost {
                player: PlayerId(1),
                amount: 10,
            },
        );

        // Only player 0's losses sum → 5.
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern.clone()), Window::ThisTurn),
                &frame
            ),
            5,
            "two life-loss facts for you: 2 + 3 = 5"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame sees 5.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern.clone()), Window::ThisTurn),
                &frame
            ),
            0,
            "ThisTurn no longer sees last turn's life losses"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern), Window::ThisGame),
                &frame
            ),
            5,
            "ThisGame still sees 5 total life lost by you"
        );
    }

    /// `EventCount(Used(by: This))` is OBJECT-scoped: it resolves `by` to the
    /// frame's source `ObjectId` and counts that object's `AbilityUsed` facts,
    /// NOT a watcher-pattern match ([CR#608.2i,603.2]). Two uses by the frame
    /// object this turn → 2 (a third use by a DIFFERENT object is excluded);
    /// after a turn advance `ThisTurn` reads 0 while `ThisGame` still reads 2.
    #[test]
    fn event_count_used_counts_object_ability_uses() {
        use deckmaste_core::Event;
        use deckmaste_core::Window;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        state.turn.turn_number = 1;

        // The frame's source is `obj`; `Used(by: This)` resolves `This` to it.
        let obj = ObjectId::from_raw(1);
        let other = ObjectId::from_raw(2);
        let frame = frame_src(obj);

        let used = |n| {
            Count::EventCount(
                Box::new(Event::Used {
                    by: Reference::This,
                }),
                n,
            )
        };

        // No uses recorded yet → 0.
        assert_eq!(
            state.eval_count(&used(Window::ThisTurn), &frame),
            0,
            "no ability uses recorded yet"
        );

        // Two uses by `obj` this turn, and one by `other`.
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: other,
                ability: 0,
            },
        );

        // Only `obj`'s two uses count (`This` == frame.source == obj).
        assert_eq!(
            state.eval_count(&used(Window::ThisTurn), &frame),
            2,
            "two uses by the frame object; the other object's use is excluded"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame still sees the 2.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(&used(Window::ThisTurn), &frame),
            0,
            "ThisTurn no longer sees last turn's uses"
        );
        assert_eq!(
            state.eval_count(&used(Window::ThisGame), &frame),
            2,
            "ThisGame still sees last turn's two uses"
        );
    }

    /// The card-facing payoff: a self-use count drives a branching condition
    /// (`If(Compare(EventCount(Used(by: This), ThisTurn), Eq, 2), then,
    /// else)`). The `Compare` is FALSE after one recorded use of the frame
    /// object and TRUE after the second — the ability's own use-count keys
    /// the branch ([CR#608.2i]).
    #[test]
    fn event_count_self_drives_branching_condition() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Event;
        use deckmaste_core::Window;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        state.turn.turn_number = 1;

        let obj = ObjectId::from_raw(1);
        let frame = frame_src(obj);

        // "if this object's abilities have been used exactly twice this turn".
        let twice = Condition::Compare(
            Count::EventCount(
                Box::new(Event::Used {
                    by: Reference::This,
                }),
                Window::ThisTurn,
            ),
            Cmp::Eq,
            Count::Literal(2),
        );

        // One use → not yet two → branch is FALSE.
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        assert!(
            !state.condition_holds(&twice, &frame),
            "one self-use does not satisfy `== 2`"
        );

        // Second use → exactly two → branch is TRUE.
        state.history.record(
            1,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        assert!(
            state.condition_holds(&twice, &frame),
            "two self-uses satisfy `== 2`"
        );
    }

    /// `TopOfLibrary` returns the top N cards in order (front of library =
    /// top); `Effect::With` binds them so `Selection::Those` resolves to the
    /// same ordered vec inside the body frame.
    #[test]
    fn with_binds_those_and_top_of_library_is_ordered() {
        use deckmaste_core::CardFace;
        use deckmaste_core::With;

        use crate::object::ObjectSource;

        let mut state = game();
        let p0 = PlayerId(0);

        // Build three distinct library cards and mint them in order a→b→c
        // (a at front = top).
        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);

        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Library));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Library));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Library));

        // Push in top→bottom order: a at front (index 0) = top of library.
        state.zones.libraries[p0.index()].push_back(a);
        state.zones.libraries[p0.index()].push_back(b);
        state.zones.libraries[p0.index()].push_back(c);

        // A source object for the frame — use p0's proxy.
        let source = state.player(p0).object;
        let frame = Frame {
            source,
            controller: p0,
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };

        // TopOfLibrary(count:2, of:You) → top two in order.
        let top2 = state.eval_selection_set(
            &Selection::TopOfLibrary {
                count: Count::Literal(2),
                of: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top2, vec![a, b], "top 2 are a then b, top→down");

        // With binds them as Those; the body frame sees the same ordered group.
        let mut bound = frame.clone();
        bound.those = Some(top2.clone());
        assert_eq!(
            state.eval_selection_set(&Selection::Those, &bound),
            vec![a, b],
            "Those inside a With frame returns the bound group in order"
        );

        // Effect::With end-to-end: run_effect schedules a body that reads Those
        // and verifies the binding survives round-trip through the agenda.
        // We check indirectly by scheduling a no-op body and confirming no panic.
        state.run_effect(
            Effect::With(With {
                selection: Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    of: deckmaste_core::Reference::You,
                },
                body: Box::new(Effect::Sequence(vec![])),
            }),
            &frame,
        );
        // Drain the agenda — the empty Sequence body completes without a
        // decision, proving With schedules correctly.
        for _ in 0..10 {
            state.step();
        }
    }

    /// [CR#701.22a]: `Distribute` over a 3-card window surfaces a decision,
    /// and submitting the answer repositions the cards in the library in the
    /// exact authored order: top pile `[c, a]` (c on top) and bottom pile
    /// `[b]` → final library `[c, a, b]` top→down. ObjectIds are preserved
    /// (no remint) because the reposition is a direct VecDeque surgery.
    #[test]
    fn scry_partitions_top_three_in_order() {
        use deckmaste_core::Bin;
        use deckmaste_core::CardFace;
        use deckmaste_core::With;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::object::ObjectSource;
        use crate::step::StepOutcome;

        let mut state = game();
        let p0 = PlayerId(0);

        // Build three distinct library cards: a at top (front), then b, then c.
        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);

        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Library));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Library));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Library));

        // Push in top→bottom order: a at front (index 0) = top of library.
        state.zones.libraries[p0.index()].push_back(a);
        state.zones.libraries[p0.index()].push_back(b);
        state.zones.libraries[p0.index()].push_back(c);

        let source = state.player(p0).object;
        let frame = Frame {
            source,
            controller: p0,
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };

        // Run `Effect::With(TopOfLibrary(3), Distribute(Those, [Top, Bottom],
        // "Scry"))`.
        state.run_effect(
            Effect::With(With {
                selection: Selection::TopOfLibrary {
                    count: Count::Literal(3),
                    of: deckmaste_core::Reference::You,
                },
                body: Box::new(Effect::act_by_you(PlayerAction::Distribute {
                    group: Selection::Those,
                    bins: vec![Bin::Top, Bin::Bottom],
                    name: deckmaste_core::Ident::new("Scry"),
                })),
            }),
            &frame,
        );

        // Step until the Distribute decision surfaces.
        let pending = loop {
            match state.step() {
                StepOutcome::NeedsDecision(d) => break d,
                StepOutcome::GameOver(_) => panic!("game ended unexpectedly"),
                StepOutcome::Progress(_) => {}
            }
        };

        // The decision must carry the full ordered window [a, b, c] top→down.
        let (window, bins) = match &pending {
            PendingDecision::Distribute { window, bins, .. } => (window.clone(), bins.clone()),
            other => panic!("expected Distribute, got {other:?}"),
        };
        assert_eq!(
            window,
            vec![a, b, c],
            "window is the top-3 in library order"
        );
        assert_eq!(bins, vec![Bin::Top, Bin::Bottom]);

        // Submit: top pile [c, a] (c on top), bottom pile [b].
        state
            .submit_decision(Decision::Distribution(vec![vec![c, a], vec![b]]))
            .unwrap();

        // Step only through effect-induced work (Emit, RunEffect, OpenDistribute).
        // Stop as soon as the front item is a turn-structure item (BeginStep,
        // OpenPriority, etc.) — we don't need to advance the turn to check the
        // library state.
        for _ in 0..20 {
            match state.agenda.front() {
                Some(
                    WorkItem::Emit(_)
                    | WorkItem::RunEffect { .. }
                    | WorkItem::OpenDistribute { .. },
                ) => {
                    let _ = state.step();
                }
                _ => break,
            }
        }

        assert!(
            state.pending.is_none(),
            "no pending decision after distribution (got {:?})",
            state.pending
        );

        // Final library must be [c, a, b]: c on top, a in middle, b on bottom.
        let lib: Vec<_> = state.zones.libraries[p0.index()].iter().copied().collect();
        assert_eq!(lib, vec![c, a, b], "c top, a middle, b bottom");
    }

    /// [CR#701.22b]: Distribute over an empty window (scry/surveil 0) is a
    /// complete no-op — no pending decision, no library change, no event.
    #[test]
    fn scry_zero_is_a_noop() {
        use deckmaste_core::Bin;
        use deckmaste_core::CardFace;
        use deckmaste_core::With;

        use crate::object::ObjectSource;
        use crate::step::StepOutcome;

        let mut state = game();
        let p0 = PlayerId(0);

        // Build three library cards.
        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);

        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Library));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Library));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Library));

        state.zones.libraries[p0.index()].push_back(a);
        state.zones.libraries[p0.index()].push_back(b);
        state.zones.libraries[p0.index()].push_back(c);

        let source = state.player(p0).object;
        let frame = Frame {
            source,
            controller: p0,
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };

        // Scry 0: TopOfLibrary(0) → empty window → no decision, no move.
        state.run_effect(
            Effect::With(With {
                selection: Selection::TopOfLibrary {
                    count: Count::Literal(0),
                    of: deckmaste_core::Reference::You,
                },
                body: Box::new(Effect::act_by_you(PlayerAction::Distribute {
                    group: Selection::Those,
                    bins: vec![Bin::Top, Bin::Bottom],
                    name: deckmaste_core::Ident::new("Scry"),
                })),
            }),
            &frame,
        );

        // Drain injected work — must not surface a decision.
        for _ in 0..20 {
            match state.agenda.front() {
                Some(
                    WorkItem::Emit(_)
                    | WorkItem::RunEffect { .. }
                    | WorkItem::OpenDistribute { .. },
                ) => match state.step() {
                    StepOutcome::NeedsDecision(d) => {
                        panic!("scry-0 must not surface a decision, got {d:?}")
                    }
                    StepOutcome::GameOver(_) => panic!("game ended unexpectedly"),
                    StepOutcome::Progress(_) => {}
                },
                _ => break,
            }
        }

        assert!(state.pending.is_none(), "no pending decision after scry-0");
        let lib: Vec<_> = state.zones.libraries[p0.index()].iter().copied().collect();
        assert_eq!(lib, vec![a, b, c], "library unchanged after scry-0");
    }

    /// `GameEvent::Distributed` is recorded in history with the keyword name
    /// and count after a non-empty Distribute completes ([CR#701.22d]), and
    /// suppressed entirely when the window is empty ([CR#701.22b]).
    #[test]
    fn distribute_emits_named_event_and_suppresses_at_zero() {
        use deckmaste_core::Bin;
        use deckmaste_core::CardFace;
        use deckmaste_core::Window;
        use deckmaste_core::With;

        use crate::decide::Decision;
        use crate::object::ObjectSource;
        use crate::step::StepOutcome;

        let mut state = game();
        let p0 = PlayerId(0);

        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);

        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Library));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Library));

        state.zones.libraries[p0.index()].push_back(a);
        state.zones.libraries[p0.index()].push_back(b);

        let source = state.player(p0).object;
        let frame = Frame {
            source,
            controller: p0,
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };

        // — N=0 case: scry 0 emits no Distributed event.
        state.run_effect(
            Effect::With(With {
                selection: Selection::TopOfLibrary {
                    count: Count::Literal(0),
                    of: deckmaste_core::Reference::You,
                },
                body: Box::new(Effect::act_by_you(PlayerAction::Distribute {
                    group: Selection::Those,
                    bins: vec![Bin::Top, Bin::Bottom],
                    name: deckmaste_core::Ident::new("Scry"),
                })),
            }),
            &frame,
        );
        // Drain the injected work items.
        for _ in 0..20 {
            match state.agenda.front() {
                Some(
                    WorkItem::Emit(_)
                    | WorkItem::RunEffect { .. }
                    | WorkItem::OpenDistribute { .. },
                ) => match state.step() {
                    StepOutcome::NeedsDecision(d) => {
                        panic!("scry-0 must not surface a decision, got {d:?}")
                    }
                    StepOutcome::GameOver(_) => panic!("game ended unexpectedly"),
                    StepOutcome::Progress(_) => {}
                },
                _ => break,
            }
        }
        let has_distributed = state
            .history
            .scan(Window::ThisGame, state.turn.turn_number)
            .any(|e| matches!(e, GameEvent::Distributed { .. }));
        assert!(!has_distributed, "scry-0 must not emit Distributed");

        // — N=2 case: scry 2 records Distributed { name: "Scry", count: 2 }.
        state.run_effect(
            Effect::With(With {
                selection: Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    of: deckmaste_core::Reference::You,
                },
                body: Box::new(Effect::act_by_you(PlayerAction::Distribute {
                    group: Selection::Those,
                    bins: vec![Bin::Top, Bin::Bottom],
                    name: deckmaste_core::Ident::new("Scry"),
                })),
            }),
            &frame,
        );
        // Step until the Distribute decision surfaces.
        loop {
            match state.step() {
                StepOutcome::NeedsDecision(_) => break,
                StepOutcome::GameOver(_) => panic!("game ended unexpectedly"),
                StepOutcome::Progress(_) => {}
            }
        }
        // Submit: top=[a], bottom=[b].
        state
            .submit_decision(Decision::Distribution(vec![vec![a], vec![b]]))
            .unwrap();
        // Drain until the Distributed event fires.
        for _ in 0..20 {
            match state.agenda.front() {
                Some(
                    WorkItem::Emit(_)
                    | WorkItem::RunEffect { .. }
                    | WorkItem::OpenDistribute { .. },
                ) => {
                    let _ = state.step();
                }
                _ => break,
            }
        }
        let found = state
            .history
            .scan(Window::ThisGame, state.turn.turn_number)
            .any(|e| {
                matches!(e, GameEvent::Distributed { name, count, .. }
                    if name.as_str() == "Scry" && *count == 2)
            });
        assert!(
            found,
            "scry-2 must record Distributed {{ name: Scry, count: 2 }}"
        );
    }
}
