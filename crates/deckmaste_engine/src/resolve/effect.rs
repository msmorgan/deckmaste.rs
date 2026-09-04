//! `run_effect`: walk a `Instruction` AST — combinators, binders, riders,
//! continuous-effect minting — lowering each node to agenda work.

use std::sync::Arc;

use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Instruction;
use deckmaste_core::Modification;
use deckmaste_core::Normalize;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::StaticSpec;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use slotmap::Key;

use super::action::composite_body_group;
use super::action::composite_body_whose;
use super::action::composite_move_src;
use super::occurrence_of;
use crate::agenda::WorkItem;
use crate::event::Act;
use crate::event::Cause;
use crate::event::DamageDealt;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::TriggerFired;
use crate::event::ZoneChange;
use crate::layer::ContinuousEffect;
use crate::layer::ScopeResolved;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::ExecutionFrame;
use crate::state::GameState;

impl GameState {
    /// [CR#614.1]: register a floating replacement shield (regeneration, "the
    /// next time …") on `state.shields`. Mutates `&mut self`, so it can't ride
    /// `action_items` (`&self`); the `Action::CreateReplacement` arm of
    /// `run_effect` routes here.
    fn create_shield(
        &mut self,
        subject: &deckmaste_core::Reference,
        replacement: deckmaste_core::Replacement,
        duration: deckmaste_core::Duration,
        one_shot: bool,
        frame: &ExecutionFrame,
    ) {
        // Same canonical guard the continuous-effect mint uses
        // ([CR#611.2,614.3]): a shield may carry only a SWEEPABLE
        // duration. `ForThisEvent` is the one exception — an
        // instruction-scoped rider, never a stored shield;
        // stay LOUD rather than register a silently-forever shield.
        assert!(
            crate::state::duration_sweepable(&duration),
            "create_shield: non-sweepable duration {duration:?} — a ForThisEvent \
             shield would last forever (rider durations never mint instances)"
        );
        // [CR#614.1]: the protected permanent — whatever this is a shield
        // AROUND — is the register lowering declared for it. The shield freezes
        // THAT resolved identity at creation (`floating_watches` then matches
        // on this frozen subject). A vanished subject (a bound-but-departed
        // register) fizzles the mint — never a shield with a null subject,
        // never a panic ([CR#701.8a]).
        let id = self.eval_reference(subject, frame);
        if self.objects.get(id).is_none() {
            return;
        }
        let iid = crate::replace_registry::InstanceId(self.next_shield_id);
        self.next_shield_id += 1;
        let source = frame.source(self);
        self.shields
            .push(crate::replace_registry::ReplacementInstance {
                id: iid,
                replacement,
                subject: id,
                duration,
                one_shot,
                source,
            });
    }

    /// Resolve a granted `Deontic`'s SUBJECT `Reference`s at mint ([CR#611.2c]
    /// object lock) — the object set a resolved one-shot restriction affects is
    /// fixed at creation. Returns the resolved ids (for
    /// `ScopeResolved::Locked`) and a rewritten `Deontic` whose subject
    /// slot(s) now read `Ref(It)`, so a consumer interprets `It` as the
    /// locked scope members. A subject that is not a bare object reference
    /// (a static filter like "creatures you control") locks nothing and is
    /// left unchanged — the row still exists, evaluated against live
    /// objects by its (still-LOUD) reader.
    fn lock_deontic_subject(
        &self,
        deontic: &Deontic,
        frame: &ExecutionFrame,
    ) -> (Vec<ObjectId>, Deontic) {
        let mut locked = deontic.clone();
        let mut ids = Vec::new();
        for slot in deontic_subject_slots(deontic_action_mut(&mut locked)) {
            if let Predicate::Ref(r) = slot {
                ids.push(self.eval_reference(r, frame));
                *slot = Predicate::Any;
            }
        }
        (ids, locked)
    }

    /// Resolve a `ForThisEvent` rider clause's parts to the "can't be
    /// regenerated" subject ids for `WorkItem::InstallRiders`. Only
    /// `Cant(Regenerate(on))` is wired ([CR#701.19c] — a regeneration shield is
    /// not APPLIED to the destruction, and stays unconsumed); every OTHER rider
    /// kind is LOUD (its enforcement belongs to a future ticket), as is a
    /// `Regenerate` whose `on` is not a bare object reference.
    fn resolve_no_regen_riders(
        &self,
        parts: &[StaticSpec],
        frame: &ExecutionFrame,
    ) -> Vec<ObjectId> {
        let mut ids = Vec::new();
        for part in parts {
            let StaticSpec::Deontic(Deontic::Cant(action)) = part else {
                todo!(
                    "engine seam: ForThisEvent rider {part:?} ([CR#701.19c]) — only Cant(Regenerate) is wired; owner: engine-forthisevent-riders"
                );
            };
            let DeonticAction::Regenerate { on, .. } = action else {
                todo!(
                    "engine seam: ForThisEvent Cant rider {action:?} ([CR#701.19c]) — only Regenerate is wired; owner: engine-forthisevent-riders"
                );
            };
            match on {
                Predicate::Ref(r) => ids.push(self.eval_reference(r, frame)),
                other => todo!(
                    "engine seam: Cant(Regenerate(on: {other:?})) ([CR#701.19c]) — only a bare object Ref is wired; owner: engine-forthisevent-riders"
                ),
            }
        }
        ids
    }

    /// The element set a loop iterates ([CR#608.2]). A `Random` selection is
    /// SAMPLED here — once, by the mutating resolver — instead of by the pure
    /// selection evaluator. The sample is returned directly; each element is
    /// then written to its loop region parameter.
    fn iteration_selection(
        &mut self,
        selection: &deckmaste_core::Selection,
        frame: &ExecutionFrame,
    ) -> Vec<crate::object::ObjectId> {
        use deckmaste_core::Selection;

        let Selection::Random(quantity, filter) = selection else {
            return self.eval_selection_set(selection, frame);
        };

        if let Some((recorded, post_sample_rng)) = self.take_replay_random_outcome() {
            self.rng.set_stream(post_sample_rng.stream);
            self.rng.set_word_pos(post_sample_rng.word_pos);
            self.record_payment_random_outcome(&recorded);
            return recorded;
        }

        let watcher = Some(self.frame_watcher(frame));
        // The region's declared candidate domain gates the enumeration, so a
        // random sample never offers a player to an Object-domain filter or
        // the reverse ([CR#109.1,102.1]).
        let candidates = crate::target::candidates_region_with_activation(
            self,
            filter,
            watcher,
            frame.activation,
        );
        let (_, max) = self.choice_bounds(quantity, candidates.len(), frame);
        let n = usize::try_from(max).expect("sample count fits usize");
        let indices = rand::seq::index::sample(&mut self.rng, candidates.len(), n);
        let selected = indices
            .into_iter()
            .map(|i| candidates[i])
            .collect::<Vec<_>>();
        self.record_payment_random_outcome(&selected);
        selected
    }

    /// Interpret one `Instruction` node ([CR#608.2]). `Act` becomes one or
    /// more `Emit` work items (via `action_items`); `Sequentially` expands
    /// to one `RunEffect` per child.
    ///
    /// # Panics
    ///
    /// Panics on any `Instruction` variant not wired for Stage 3.
    /// [CR#608.2g]: if `may`'s body is a bare `Cast(<ref>)` verb, resolve the
    /// caster (`Cast`'s own agent slot) and the referent object it would
    /// cast — so the `May` arm can gate the "yes" offer on
    /// [`can_cast_as_effect`](Self::can_cast_as_effect). `None` for any other
    /// `May` body (the ordinary "you may [do]", offered unconditionally). The
    /// referent id may be null/stale; the gate treats that as uncastable.
    pub(crate) fn may_cast_referent(
        &self,
        may: &deckmaste_core::May,
        frame: &ExecutionFrame,
    ) -> Option<(
        crate::player::PlayerId,
        crate::object::ObjectId,
        Option<deckmaste_core::Cost>,
    )> {
        match &*may.effect {
            Instruction::Act {
                action: Action::Cast(actor, what, for_cost),
                ..
            } => Some((
                self.acting_player(actor, frame),
                self.eval_reference(what, frame),
                for_cost.clone(),
            )),
            _ => None,
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per effect-frame variant; splitting would scatter the dispatch"
    )]
    pub(crate) fn run_effect(&mut self, effect: Instruction, frame: &ExecutionFrame) {
        match effect {
            Instruction::Act { dest, action } => {
                // A verb acts on an already-bound `Reference` — choosing is a
                // separate preceding instruction
                // (`Choose`/`ChooseValue`/`Search`),
                // never the verb's, so an `Act` never surfaces a choice itself.
                // `CreateReplacement` directly mutates `state.shields` — it
                // cannot go through `action_items` (which is `&self`). Handle
                // it here, mirroring how `Instruction::Continuously` works.
                let magnitude = match &action {
                    Action::FlipCoins(_, _, called) => {
                        Some(crate::agenda::MagnitudeSource::CoinFlips { called: *called })
                    }
                    Action::RollDice(..) => Some(crate::agenda::MagnitudeSource::DiceRolls),
                    Action::Composite { name, .. }
                        if action.produces_runtime_magnitude()
                            && crate::entail::entailment(name.as_str())
                                .is_some_and(|row| row.kind == "ZoneChange" && row.amount) =>
                    {
                        Some(crate::agenda::MagnitudeSource::ZoneChanges(*name))
                    }
                    _ => None,
                };
                debug_assert_eq!(
                    action.produces_runtime_magnitude(),
                    magnitude.is_some(),
                    "core and engine runtime-magnitude classifications agree"
                );
                if let Some(dest) = dest
                    && magnitude.is_none()
                {
                    match &action {
                        // [CR#701.21a]: a sacrifice moves the permanent to its
                        // owner's graveyard, so — like any move — its product
                        // is that object, written at its pre-move identity and
                        // chased to its new one on read ([CR#400.7]). This is
                        // "the sacrificed creature" a cost block's body names.
                        Action::Move(subject, _, _, _) | Action::Sacrifice(_, subject) => {
                            let object = self.eval_reference(subject, frame);
                            self.activation_write_object(frame.activation, dest, object);
                        }
                        Action::MoveGroup { group, .. } => {
                            let objects = self.eval_selection_set(group, frame);
                            self.activation_write_objects(frame.activation, dest, &objects);
                        }
                        Action::DrawCard(who) => {
                            let player = self.acting_player(who, frame);
                            if let Some(&object) = self.zones.libraries[player.index()].front() {
                                self.activation_write_object(frame.activation, dest, object);
                            }
                        }
                        _ => {}
                    }
                }
                if let Action::CreateReplacement {
                    subject,
                    replacement,
                    duration,
                    one_shot,
                } = action
                {
                    self.create_shield(
                        &subject,
                        Arc::unwrap_or_clone(replacement),
                        duration,
                        one_shot,
                        frame,
                    );
                } else {
                    let mut items = self.action_items(&action, frame);
                    if let (Some(dest), Some(source)) = (dest, magnitude) {
                        items.push(WorkItem::WriteMagnitude {
                            activation: frame.activation,
                            dest,
                            source,
                            mark: self.resolution_events.len(),
                        });
                    }
                    self.schedule_front(items);
                }
            }
            Instruction::Let(binding) => match binding.expr {
                deckmaste_core::Expr::Object(reference) => {
                    let object = self.eval_reference(&reference, frame);
                    self.activation_write_object(frame.activation, binding.dest, object);
                }
                deckmaste_core::Expr::Objects(selection) => {
                    let objects = self.eval_selection_set(&selection, frame);
                    self.activation_write_objects(frame.activation, binding.dest, &objects);
                }
                deckmaste_core::Expr::Number(count) => {
                    let number = self.eval_count(&count, frame);
                    self.activation_write_number(frame.activation, binding.dest, number);
                }
            },
            Instruction::Choose(choice) => {
                let candidates = crate::target::candidates_region_with_activation(
                    self,
                    &choice.filter,
                    Some(self.frame_watcher(frame)),
                    frame.activation,
                );
                let (min, max) = self.choice_bounds(&choice.quantity, candidates.len(), frame);
                self.pending = Some(crate::decide::DecisionPointKind::ChooseObjects(
                    crate::decide::pending::ChooseObjects {
                        player: self.acting_player(&choice.by, frame),
                        candidates,
                        min,
                        max,
                    },
                ));
                self.choice = Some(crate::state::DecisionContinuation::BindChoice {
                    dest: choice.dest,
                    frame: frame.clone(),
                    if_none: deckmaste_core::Block::default(),
                });
            }
            Instruction::Search(search) => {
                let owner = self.acting_player(&search.whose, frame);
                let candidates: Vec<_> = self
                    .objects
                    .iter()
                    .filter(|object| object.zone.is_some_and(|zone| search.from.contains(&zone)))
                    .filter(|object| self.owner_of(object.id) == owner)
                    .filter(|object| {
                        crate::target::matches_region_with_activation(
                            self,
                            object.id,
                            &search.filter,
                            Some(self.frame_watcher(frame)),
                            frame.activation,
                        )
                    })
                    .map(|object| object.id)
                    .collect();
                let (lo, max) = self.choice_bounds(&search.quantity, candidates.len(), frame);
                // [CR#701.23d]: only a BARE quantity ("search your library for
                // a card") compels a find — and even that settles for "as many
                // as possible" once `choice_bounds` clamps to availability. A
                // STATED quality ([CR#701.23b]) never compels one, even with a
                // match sitting right there. An UNDEFINED quality
                // ([CR#701.23c]) needs no separate arm: it can never match a
                // candidate, so it floors to 0 through this same mechanism.
                let min = if crate::resolve::search_is_bare_quantity(&search.filter.body) {
                    lo
                } else {
                    0
                };
                self.pending = Some(crate::decide::DecisionPointKind::ChooseObjects(
                    crate::decide::pending::ChooseObjects {
                        player: self.acting_player(&search.by, frame),
                        candidates,
                        min,
                        max,
                    },
                ));
                self.choice = Some(crate::state::DecisionContinuation::BindChoice {
                    dest: search.dest,
                    frame: frame.clone(),
                    if_none: search.if_none,
                });
            }
            Instruction::ChooseValue(choice) => {
                let actor = self.acting_player(&choice.by, frame);
                match choice.domain {
                    deckmaste_core::ChosenValueKind::Number => {
                        self.pending = Some(crate::decide::DecisionPointKind::ChooseNoteNumber(
                            crate::decide::pending::ChooseNoteNumber {
                                player: actor,
                                key: "register".into(),
                            },
                        ));
                        self.choice = Some(crate::state::DecisionContinuation::BindNumber {
                            dest: choice.dest,
                            activation: frame.activation,
                        });
                    }
                    deckmaste_core::ChosenValueKind::CardName => {
                        self.pending = Some(crate::decide::DecisionPointKind::ChooseNoteCardName(
                            crate::decide::pending::ChooseNoteCardName {
                                player: actor,
                                key: "register".into(),
                            },
                        ));
                        self.choice = Some(crate::state::DecisionContinuation::BindSymbol {
                            dest: choice.dest,
                            activation: frame.activation,
                        });
                    }
                    deckmaste_core::ChosenValueKind::Color => {}
                }
            }
            Instruction::Sequentially(children) => {
                // Pre-scan for `ForThisEvent` riders ([CR#611.2a],
                // [CR#701.19c]): an `Until(ForThisEvent,
                // parts)` child NEVER mints its own instance —
                // its parts fold as instruction-scoped RIDERS onto
                // the immediately-preceding sibling's work. This shape is
                // forced by scheduling: a `Destroy`
                // front-schedules its `Emit`, which is
                // APPLIED before the next `RunEffect` child could run, so a
                // sibling-minted instance would arrive too late ([CR#611.2c]);
                // the rider must be armed BEFORE the destroy runs. We install
                // it just before that preceding sibling's
                // `RunEffect`.
                let mut items: Vec<WorkItem> = Vec::new();
                for child in children.iter().cloned() {
                    if let Some(parts) = for_this_event_rider(&child) {
                        let no_regen = self.resolve_no_regen_riders(parts, frame);
                        // A rider with no preceding sibling is a semantic-input
                        // mistake ([CR#611.2a] scopes it to a host event that
                        // isn't there): FIZZLE — drop it, never mint or panic.
                        if let Some(pos) = items
                            .iter()
                            .rposition(|w| matches!(w, WorkItem::RunEffect { .. }))
                        {
                            items.insert(pos, WorkItem::InstallRiders { no_regen });
                        }
                        continue;
                    }
                    items.push(WorkItem::RunEffect {
                        effect: Arc::new(child),
                        frame: frame.clone(),
                    });
                }
                self.schedule_front(items);
            }
            // The written `Simultaneously` spec. ONE SNAPSHOT: every member's
            // items are evaluated up front against the current state, before
            // any applies (an exchange works BECAUSE both halves read the
            // pre-state). ONE BATCH: the merged events land as one
            // occurrence, so triggers see one [CR#603.2c] simultaneous set,
            // its facts share a batch id, replacements run per member
            // ([CR#616.1]), and SBAs run after the whole batch (the next
            // `CheckSbas`, never between members). ALL-OR-NOTHING: a member
            // that evaluates to no events voids the whole set ([CR#701.12a]
            // — "if the entire exchange can't be completed, no part of the
            // exchange occurs"). The exchange family restricts members to
            // pure-verb bodies — a choice-bearing member is unrepresentable
            // in sound data and trips loudly.
            Instruction::Simultaneously(children) => {
                // Phase A (read-only): classify and evaluate every member
                // against the pre-state snapshot ([CR#611.2c]). An `Act` verb
                // lowers to events via `action_items`, unchanged. A
                // `Continuously(Modify(r, change))` member — the only
                // non-verb shape wired here (Avarice Totem's exchange) — has
                // no event to ride the batch, so it resolves its `Locked`
                // scope (and, for a `SetController` naming another object's
                // controller, that controller too) as a pure read instead;
                // row minting mutates (`self.objects.next_timestamp()`), so
                // it waits for phase B, once every member is known-nonempty.
                struct PendingStatic {
                    controller: crate::player::PlayerId,
                    ids: Vec<ObjectId>,
                    changes: Vec<Modification>,
                    grant_runtimes: Vec<crate::activation::AbilityRuntime>,
                    duration: deckmaste_core::Duration,
                    origin: Option<Box<ExecutionFrame>>,
                }
                enum Member {
                    Events(Vec<GameEvent>),
                    Static(Box<PendingStatic>),
                }

                let mut members: Vec<Member> = Vec::new();
                let mut void = false;
                for child in children.iter() {
                    match child {
                        Instruction::Act { action, .. } => {
                            let mut events = Vec::new();
                            for item in self.action_items(action, frame) {
                                match item {
                                    WorkItem::Emit(crate::event::Occurrence::Single(e)) => {
                                        events.push(e);
                                    }
                                    WorkItem::Emit(crate::event::Occurrence::Batch(es)) => {
                                        events.extend(es);
                                    }
                                    other => todo!(
                                        "engine seam: choice-bearing Simultaneously member, \
                                         scheduled {other:?} ([CR#701.12a]) — the verb lowers to \
                                         a decision rather than events, which the one-snapshot \
                                         batch has no way to await; owner: \
                                         engine-simultaneous-choice-members"
                                    ),
                                }
                            }
                            if events.is_empty() {
                                void = true;
                            }
                            members.push(Member::Events(events));
                        }
                        Instruction::Continuously(e) => {
                            let StaticSpec::Modify(r, change) = &*e.effect else {
                                todo!(
                                    "engine seam: Simultaneously(Continuously({e:?})) \
                                     ([CR#701.12a,611.2c]) — only a bare Modify(_, _) inner is \
                                     wired (its Locked scope is emptiness-testable at mint); \
                                     other Continuously bodies stay unbuilt inside \
                                     Simultaneously; owner: engine-simultaneous-member-breadth"
                                )
                            };
                            let subject = self.eval_reference(r, frame);
                            let ids = if subject.is_null() { Vec::new() } else { vec![subject] };
                            if ids.is_empty() {
                                void = true;
                            }
                            // The affected object is fixed at mint
                            // ([CR#611.2c]).
                            // A `SetController` naming another object's
                            // controller must be resolved here too: the game
                            // read it needs happens once, when the effect
                            // applies ([CR#608.2h]), and
                            // `resolve_new_controller`'s
                            // apply-time pass only understands `You` anyway (it
                            // has no ExecutionFrame to chase a live `Target`/
                            // `ControllerOf` read) — so Avarice Totem's
                            // exchange resolves and freezes the new controller
                            // here, against the pre-exchange state, rewriting
                            // the stored change to the `You` shape
                            // `resolve_new_controller` already handles. A bare
                            // `SetController(You)` keeps today's
                            // `frame.controller(self)` path untouched.
                            let (changes, controller) = match change {
                                Modification::SetController(value)
                                    if *value != Reference::controller_parameter() =>
                                {
                                    if let Some(p) = self.eval_player_ref(value, frame) {
                                        (
                                            vec![Modification::SetController(
                                                Reference::controller_parameter(),
                                            )],
                                            p,
                                        )
                                    } else {
                                        void = true;
                                        (vec![], frame.controller(self))
                                    }
                                }
                                _ => (
                                    Modification::flatten(std::slice::from_ref(change)).to_vec(),
                                    frame.controller(self),
                                ),
                            };
                            let origin = match &e.duration {
                                deckmaste_core::Duration::UntilEvent(_)
                                | deckmaste_core::Duration::ForAsLongAs(_) => {
                                    Some(Box::new(frame.clone()))
                                }
                                _ => None,
                            };
                            let grant_runtimes = self.capture_grant_runtimes(&changes, frame);
                            members.push(Member::Static(Box::new(PendingStatic {
                                controller,
                                ids,
                                changes,
                                grant_runtimes,
                                duration: e.duration.clone(),
                                origin,
                            })));
                        }
                        other => todo!(
                            "engine seam: non-verb Simultaneously member {other:?} \
                             ([CR#701.12a,608.2f]) — a Continuously member mints a static row \
                             instead of events, so it has nothing to contribute to the \
                             all-or-nothing batch; only Act and a Modify(_, _)-bodied \
                             Continuously are wired; Sequentially/If/May/nested Simultaneously \
                             stay unbuilt; owner: engine-simultaneous-member-breadth"
                        ),
                    }
                }
                if void {
                    return;
                }

                // Phase B (commit): timestamp allocation and row insertion
                // mutate, so they wait until every member is known-nonempty
                // ([CR#701.12a] — if the entire exchange can't be completed,
                // no part of it occurs).
                let mut events: Vec<GameEvent> = Vec::new();
                for member in members {
                    match member {
                        Member::Events(es) => events.extend(es),
                        Member::Static(pending) => {
                            let PendingStatic {
                                controller,
                                ids,
                                changes,
                                grant_runtimes,
                                duration,
                                origin,
                            } = *pending;
                            let timestamp = self.objects.next_timestamp();
                            self.continuous_grant_runtimes
                                .insert(timestamp, grant_runtimes);
                            self.continuous.push(ContinuousEffect {
                                timestamp,
                                controller,
                                scope: ScopeResolved::Locked(ids),
                                changes,
                                rows: vec![],
                                duration,
                                origin,
                                is_cda: false,
                            });
                        }
                    }
                }
                if !events.is_empty() {
                    // [CR#701.14c]: a creature that fights itself deals ONE
                    // instance equal to twice its power — not two. Within a
                    // simultaneous batch, damage from the same source to the
                    // same target (same combat-ness) is one instance;
                    // coalesce by summing amounts so a self-fight's two
                    // `X -> X` packets become one `2x` event. Fight is the
                    // only damage-bearing `Simultaneously` today; other
                    // member events (e.g. exchange `ControlChanged`) pass
                    // through untouched. An all-`Continuously` set (Avarice
                    // Totem) never reaches here — `events` stays empty and no
                    // batch is scheduled.
                    let events = coalesce_simultaneous_damage(events);
                    self.schedule_front(vec![WorkItem::Emit(crate::event::Occurrence::Batch(
                        events,
                    ))]);
                }
            }
            Instruction::Continuously(e) => {
                // [CR#611.2]/[CR#611.2c]: stamp at creation; lock the object set
                // for non-floating scopes, leave `Matching` floating.
                let timestamp = self.objects.next_timestamp();
                // Duration guard narrowed from the old catch-all: EVERY
                // duration is now sweepable EXCEPT
                // `ForThisEvent`, which is an
                // instruction-scoped rider ([CR#611.2a]) handled entirely by
                // `Sequentially` lowering and never a standalone instance.
                // Reaching this mint with `ForThisEvent` means a rider with no
                // host instruction (e.g. a top-level `Until(ForThisEvent, …)`):
                // a semantic-input error — FIZZLE (drop it), never mint a
                // forever-lasting instance or panic.
                if !crate::state::duration_sweepable(&e.duration) {
                    return;
                }
                // Route the granted static by shape. Characteristic
                // modifications feed the hot layer pass via `changes` (see
                // `gather`'s `static_effect_scope` in `layer.rs`); every other
                // kind is a static ROW consulted by the legality/cost/
                // can't-happen readers directly ([CR#613.11] — a resolved
                // one-shot's restriction is not an ability of the object).
                let (scope, changes, rows) = match &*e.effect {
                    // The single-object shape locks the one resolved reference
                    // ([CR#613.6]).
                    StaticSpec::Modify(r, change) => (
                        ScopeResolved::Locked(vec![self.eval_reference(r, frame)]),
                        Modification::flatten(std::slice::from_ref(change)),
                        vec![],
                    ),
                    // The distributor shape ("target creature and all creatures
                    // it shares a color with get +1/+1", or a plain anthem)
                    // stays `Floating(f)`: the filter is NOT expanded to objects
                    // here.
                    StaticSpec::Each(Selection::SelectAll(f), inner) => match &inner.body {
                        StaticSpec::Modify(Reference::Reg(reference), change)
                            if matches!(
                                inner.provenance_of(*reference),
                                Some(deckmaste_core::Provenance::Candidate(_))
                            ) =>
                        {
                            (
                                ScopeResolved::Floating(f.clone()),
                                Modification::flatten(std::slice::from_ref(change)),
                                vec![],
                            )
                        }
                        other => todo!(
                            "engine seam: Continuously(Each(SelectAll, {other:?})) — only a bare \
                             Modify(It, _) inner is wired; owner: engine-granted-static-rows"
                        ),
                    },
                    // A granted `Deontic` restriction ("target creature can't
                    // block this turn"): its subject `Reference`s resolve at
                    // mint ([CR#611.2c] object lock) — `scope = Locked(ids)`,
                    // the subject rewritten to `It`, so a consumer reads `It` =
                    // the scope members. Row EVALUATION stays LOUD at `legal.rs`
                    // (core-casting-restrictions / engine-combat-requirements
                    // own it); this only makes the row EXIST in the view.
                    StaticSpec::Deontic(deontic) => {
                        let (ids, locked) = self.lock_deontic_subject(deontic, frame);
                        (
                            ScopeResolved::Locked(ids),
                            [].into(),
                            vec![StaticSpec::Deontic(locked)],
                        )
                    }
                    // Self-filtered rows: `of` / the `CantHappen` filter carry
                    // their own subject predicate, so the scope is unused (an
                    // empty lock). `CostModifier` is wired into
                    // `cost_modifier_rows` (cast.rs), `CantHappen` into
                    // `cant_event` (replace_registry.rs).
                    row @ (StaticSpec::CostModifier { .. } | StaticSpec::CantHappen(_)) => {
                        (ScopeResolved::Locked(vec![]), [].into(), vec![row.clone()])
                    }
                    // Prevention shields/windows are engine-prevention's domain
                    // ([CR#615.1]); the PreventNext/PreventAll macros stay
                    // blocked until that ticket lands.
                    StaticSpec::Prevention(_) => todo!(
                        "engine seam: Continuously(Prevention) ([CR#615.1]) — granted prevention shields/windows unbuilt; owner: engine-granted-prevention-rows"
                    ),
                    // Narrower than the old catch-all: name the specific unbuilt
                    // granted static-row kind.
                    StaticSpec::Each(sel, _) => todo!(
                        "engine seam: Continuously(Each({sel:?}, _)) — only Each(SelectAll, Modify(It, _)) is wired; owner: engine-granted-static-rows"
                    ),
                    // Becomes-a-copy is a deferred-exec seam: its copiable-value install is
                    // owned by engine-layers-1-copy-facedown-text (base_values). Until that
                    // lands a resolved becomes-a-copy FIZZLES — installs nothing — per the
                    // never-panic contract, matching the early-return fizzles above (473/555).
                    StaticSpec::BecomesCopy(..) => return,
                    other => {
                        todo!(
                            "engine seam: Continuously({other:?}) — granted static-row kind unbuilt; owner: engine-granted-static-rows"
                        )
                    }
                };
                // Only the two data-re-evaluating durations keep the minting
                // frame: `UntilEvent`'s filter and `ForAsLongAs`'s condition
                // read `This`/`You` through it in their sweeps ([CR#603.10a]).
                let origin = match &e.duration {
                    deckmaste_core::Duration::UntilEvent(_)
                    | deckmaste_core::Duration::ForAsLongAs(_) => Some(Box::new(frame.clone())),
                    _ => None,
                };
                // [CR#611.2b]: a `ForAsLongAs` whose condition is already FALSE
                // at creation NEVER STARTS — push no instance at all.
                if let deckmaste_core::Duration::ForAsLongAs(cond) = &e.duration
                    && !self.condition_holds(cond, frame)
                {
                    return;
                }
                let controller = frame.controller(self);
                let changes = changes.to_vec();
                let grant_runtimes = self.capture_grant_runtimes(&changes, frame);
                self.continuous_grant_runtimes
                    .insert(timestamp, grant_runtimes);
                self.continuous.push(ContinuousEffect {
                    timestamp,
                    // The continuous effect's controller is the controller
                    // of the spell/ability that created it ([CR#611.2c]);
                    // it resolves the `You` in a layer-2 control change.
                    controller,
                    scope,
                    changes,
                    rows,
                    duration: e.duration.clone(),
                    origin,
                    is_cda: false,
                });
            }
            // The LIST spelling of `Continuously` ([CR#611.2c] — a single
            // continuous effect with parts, each part's affected set
            // determined independently): each part lowers to its own
            // continuous-effect instance sharing the duration.
            Instruction::Until(duration, parts) => {
                let items: Vec<WorkItem> = parts
                    .iter()
                    .map(|part| WorkItem::RunEffect {
                        effect: Arc::new(Instruction::Continuously(deckmaste_core::Continuously {
                            effect: Arc::new(part.clone()),
                            duration: duration.clone(),
                        })),
                        frame: frame.clone(),
                    })
                    .collect();
                self.schedule_front(items);
            }
            // A label names the clause's introductions (a compile-time
            // soundness concept); at runtime it is transparent.
            Instruction::If(if_effect) => {
                if self.condition_holds(&if_effect.condition, frame) {
                    self.run_effect(Arc::unwrap_or_clone(if_effect.then), frame);
                } else if let Some(otherwise) = if_effect.otherwise {
                    self.run_effect(Arc::unwrap_or_clone(otherwise), frame);
                }
            }
            // [CR#608.2,700.1]: "[do] to each [over]" — a *simultaneous*
            // distributive. The matched set is fixed once when this node
            // resolves ([CR#608.2h]); each element enters the body region with
            // its own `LoopElement` parameter. A single non-producing `Act`
            // body resolves for every element as one `Occurrence::Batch`, so
            // triggers and SBAs see the simultaneous instruction together. A
            // body that can pause, define a product, or otherwise needs the
            // ordinary effect interpreter keeps per-element scheduling.
            Instruction::Each(each) => {
                let matches = self.iteration_selection(&each.over, frame);
                // [CR#701.22a] look-visibility: the controller sees the cards a
                // top-of-library PEEK iterates (scry/surveil/fateseal look
                // before they arrange), keyed by object identity (expires free
                // on the next remint/shuffle).
                if is_top_of_library_peek(&each.over) {
                    let viewer = frame.controller(self);
                    for &obj in &matches {
                        self.grant_payment_look(viewer, obj);
                    }
                }
                // [CR#401.4]: if the body puts cards into ordered library
                // positions (scry's top/bottom picks), arm the post-pick
                // arrange collector so it orders each pile once every pick has
                // landed. The arranger is the effect's controller (for
                // fateseal, over the opponent's library).
                let can_reposition = body_repositions_ordered(&each.body.body);
                if can_reposition {
                    self.arrange_scope = Some(crate::state::ArrangeScope {
                        arranger: frame.controller(self),
                        landings: Vec::new(),
                    });
                }
                let frames = matches
                    .iter()
                    .map(|&object| {
                        let mut next = frame.clone();
                        next.activation = self.enter_loop_region(&each.body, frame, object, None);
                        next
                    })
                    .collect::<Vec<_>>();

                let (setup, action) = each.body.body.split_last().map_or(
                    (&[][..], None),
                    |(last, setup)| match last {
                        Instruction::Act { dest: None, action }
                            if setup.iter().all(|item| matches!(item, Instruction::Let(_))) =>
                        {
                            (setup, Some(action))
                        }
                        _ => (&[][..], None),
                    },
                );
                if let Some(action) = action {
                    let mut events = Vec::new();
                    let mut finalizers = Vec::new();
                    let mut batchable = true;
                    for next in &frames {
                        for item in setup {
                            let Instruction::Let(binding) = item else {
                                unreachable!(
                                    "the simultaneous Each fast path admits only Let setup"
                                )
                            };
                            match &binding.expr {
                                deckmaste_core::Expr::Object(reference) => {
                                    let object = self.eval_reference(reference, next);
                                    self.activation_write_object(
                                        next.activation,
                                        binding.dest,
                                        object,
                                    );
                                }
                                deckmaste_core::Expr::Objects(selection) => {
                                    let objects = self.eval_selection_set(selection, next);
                                    self.activation_write_objects(
                                        next.activation,
                                        binding.dest,
                                        &objects,
                                    );
                                }
                                deckmaste_core::Expr::Number(count) => {
                                    let number = self.eval_count(count, next);
                                    self.activation_write_number(
                                        next.activation,
                                        binding.dest,
                                        number,
                                    );
                                }
                            }
                        }
                        for item in self.action_items(action, next) {
                            match item {
                                WorkItem::Emit(Occurrence::Single(event)) => events.push(event),
                                WorkItem::Emit(Occurrence::Batch(batch)) => events.extend(batch),
                                item @ WorkItem::FinalizeAct { .. } => finalizers.push(item),
                                _ => batchable = false,
                            }
                        }
                    }
                    if batchable {
                        if !events.is_empty() {
                            finalizers.insert(0, WorkItem::Emit(occurrence_of(events)));
                        }
                        // The arrange finalizer rides LAST, so it orders each
                        // pile once every element's pick has landed
                        // ([CR#401.4]).
                        if can_reposition {
                            finalizers.push(WorkItem::ArrangePiles);
                        }
                        self.schedule_front(finalizers);
                        return;
                    }
                }

                let mut items: Vec<WorkItem> = frames
                    .into_iter()
                    .flat_map(|next| {
                        each.body
                            .body
                            .iter()
                            .cloned()
                            .map(move |effect| WorkItem::RunEffect {
                                effect: Arc::new(effect),
                                frame: next.clone(),
                            })
                    })
                    .collect();
                if can_reposition {
                    items.push(WorkItem::ArrangePiles);
                }
                self.schedule_front(items);
            }
            Instruction::Distribute(divide) => {
                let group = self.iteration_selection(&divide.over, frame);
                let total = self.eval_count(&divide.amount, frame);
                let shares = split_evenly(total, group.len());
                let items =
                    group
                        .into_iter()
                        .zip(shares)
                        .flat_map(|(object, share)| {
                            let mut next = frame.clone();
                            next.activation =
                                self.enter_loop_region(&divide.body, frame, object, Some(share));
                            divide.body.body.iter().cloned().map(move |effect| {
                                WorkItem::RunEffect {
                                    effect: Arc::new(effect),
                                    frame: next.clone(),
                                }
                            })
                        })
                        .collect();
                self.schedule_front(items);
            }
            Instruction::May(may) => {
                // [CR#118.12a,118.12]: `May(Pay(cost))` is a real optional
                // payment transaction. No preliminary YesNo answer or
                // affordability oracle can faithfully account for mana
                // abilities activated during payment; submission means the
                // payer chose to pay, while decline means they did not.
                if let Instruction::Act {
                    action: Action::Pay(cost),
                    ..
                } = &*may.effect
                {
                    let payer = self.acting_player(&may.who, frame);
                    // Normalize at this boundary: read is faithful, so a
                    // macro-spliced cost arrives lumpy (a nested
                    // `CostComponent::Cost`); splice it flat and price any
                    // `{X}` (a ward toll's where_x) before the payment walk.
                    let cost = cost.clone().normalize().0;
                    let cost = self.price_variable_cost(cost.to_vec(), frame);
                    self.begin_optional_payment(
                        payer,
                        deckmaste_core::Cost(cost.into()),
                        may.if_did,
                        may.if_not,
                        frame.clone(),
                    );
                    return;
                }
                // [CR#608.2g]: "you may cast that card. If you don't, …" — the
                // "yes" (cast) branch is offered when a legal cast proposal
                // exists; affordability is discovered by its explicit payment
                // frame. Otherwise the offer is empty and the
                // `if_not` branch runs (faithful even when the card is
                // uncastable — a land or a spell with no legal target).
                // Detected structurally: a `May` whose body is a bare `Cast`
                // verb, gated by `can_cast_as_effect` before surfacing YesNo.
                if let Some((caster, object, for_cost)) = self.may_cast_referent(&may, frame)
                    && !self.can_cast_as_effect(caster, object, for_cost.as_ref())
                {
                    let items = may
                        .if_not
                        .into_iter()
                        .map(|effect| WorkItem::RunEffect {
                            effect,
                            frame: frame.clone(),
                        })
                        .collect();
                    self.schedule_front(items);
                    return;
                }
                // [CR#608.2d]: the decider is NAMED on `who`, not derived —
                // Browbeat's "Any player may have Browbeat deal 5 damage to
                // them" proves the slot isn't always the controller. `who:
                // You` is the only spelling in canon today, so this is
                // behavior-preserving for every existing card.
                let player = self.acting_player(&may.who, frame);
                self.pending = Some(crate::decide::DecisionPointKind::YesNo(
                    crate::decide::pending::YesNo { player },
                ));
                self.choice = Some(crate::state::DecisionContinuation::May {
                    may,
                    frame: frame.clone(),
                });
            }
            // [CR#700.2]: a modal effect — choose `count` modes (up to `count`
            // when `up_to`, with repetition when `repeats`), then apply each
            // chosen mode's effect. Per-mode targets/costs are announce-time
            // ([CR#601.2b,700.2c,700.2h]) and unbuilt, so a resolution-time
            // modal handles target/cost-free modes; a mode carrying either is
            // a documented FIZZLE (never a panic on a live path — canon
            // witness: Collective Resistance, escalate + per-mode targets on
            // every mode). Building announce-time mode choice is tracked at
            // `docs/tickets/planned/engine-modal-announce-time.md`.
            Instruction::Modal(modal) => {
                if modal
                    .modes
                    .iter()
                    .any(|m| !m.targets.is_empty() || !m.cost.is_empty())
                {
                    // engine-resolve-effects seam: modal per-mode targets/costs
                    // are announce-time ([CR#601.2b,700.2c,700.2h]) — the mode
                    // choice would need to happen during casting, before
                    // targets are announced, which this resolution-time modal
                    // node can't do. Wrong-but-safe: no effect, rather than a
                    // panic on a live (grammar-covered, rendering) canon path.
                    return;
                }
                if modal.choose.rider.is_some() {
                    // engine-alt-costs seam: entwine/escalate riders are
                    // announce-time cost additions
                    // ([CR#702.42a,702.120a,601.2b,601.2f]) — the extra cost
                    // has to be paid as part of casting, before this node ever
                    // runs. Fizzle rather than panic; see the ticket above.
                    return;
                }
                let options = Uint::try_from(modal.modes.len()).expect("mode count fits Uint");
                // The choose-count is a Quantity ([CR#700.2]): its bounds cap
                // the pick — a missing lower bound floors at 0, a missing
                // upper bound is capped by the printed modes (escalate's "one
                // or more"); `repeats` lifts that ceiling ([CR#700.2d]).
                let (lo, hi) = modal.choose.count.bounds();
                let lo = lo.map_or(0, |c| self.eval_count(c, frame));
                let hi = hi.map_or(options, |c| self.eval_count(c, frame));
                let max = if modal.choose.repeats { hi } else { hi.min(options) };
                let min = if modal.choose.up_to { 0 } else { lo.min(max) };
                // [CR#700.2e]: the mode CHOOSER is named on the spec, not
                // derived — Fatal Lore's "An opponent chooses one —" is the
                // canonical non-controller case. Quantified deciders ("an
                // opponent [of your choice] chooses") have zero witnesses and
                // are not built: see the seam note on
                // `GameState::acting_player`.
                let player = self.acting_player(&modal.choose.chooser, frame);
                self.pending = Some(crate::decide::DecisionPointKind::ChooseModes(
                    crate::decide::pending::ChooseModes {
                        player,
                        options,
                        min,
                        max,
                        repeats: modal.choose.repeats,
                        entwine: false,
                    },
                ));
                self.choice = Some(crate::state::DecisionContinuation::Modal {
                    modes: modal.modes.to_vec(),
                    frame: frame.clone(),
                });
            }
            // [CR#601.2f,118.8]: "As an additional cost, [pay]; then [body]." The
            // NESTED (resolution-time) form — an extra cost paid mid-resolution,
            // after which `body` runs reading the paid object via the event
            // references. The payer is the controller (an additional cost is
            // never another player's, [CR#601.2b]), so each component runs as
            // `You`'s action, exactly like the `Unless`/`MayPay` payment walk.
            // When the cost moves a single resolvable object (`Sacrifice(This)`,
            // …) its last-known snapshot ([CR#603.10a]) is bound as the event
            // AGENT, so `body` reads "the sacrificed creature" via `EventObject`
            // (`StatOf(EventObject, Power)` = Fling). Cost items are
            // front-scheduled BEFORE the body so the payment precedes it.
            //
            // SEAMS (deferred, see the ticket): (1) ROOT-level HOISTING to
            // cast/activation time ([CR#601.2f]) — a resolution node can't reach
            // back into the announce-time cost pipeline; this arm interprets only
            // the nested form. (2) A cost whose moved object is an INTERACTIVE
            // choice (`Sacrifice(Choose …)`) — the paid object isn't known until
            // the choice resolves, so `EventObject` is left unbound here (it needs
            // a payment-time capture continuation).
            // "[body], [count] times": resolution of a `Repeat` node follows
            // the general spell/ability resolution walk ([CR#608.2]); there
            // is no dedicated CR rule for a "do N times" quantifier — this
            // is engine-side shorthand, sibling to `Each`/`Distribute` rather
            // than the manner-adverb family (`Simultaneously`/
            // `Continuously`). `count` is evaluated ONCE, up front, to a
            // concrete number (Storm/Replicate-style per-iteration semantics
            // are engine-side, per the Idris comment) — never re-evaluated
            // against the ORIGINAL count expression, so a body that changes
            // game state mid-loop (e.g. proliferating counters) can't skew
            // how many iterations remain.
            //
            // Scheduled as a LAZY self-rescheduling continuation, not an
            // eager `(0..n)` materialization: `eval_count` returns a `Uint`
            // with no clamp (arithmetic `Count`s *saturate* toward
            // `u32::MAX`), so a mistake or a hostile huge count (e.g. a
            // `Repeat(Literal(4_000_000_000), body)`) must never allocate
            // count-many work items — that would OOM/hang, violating the
            // CRITICAL never-crash ruling. Instead, for `n >= 1`, exactly
            // TWO work items are scheduled, in order: (1) one `RunEffect`
            // for `body` — this iteration, against the SAME frame the count
            // was evaluated against — then (2) one `RunEffect` whose effect
            // is `Repeat(Literal(n - 1), body)`, the remaining iterations,
            // carrying that same frame forward. `schedule_front` preserves
            // this order at the agenda's front, so body runs before the
            // tail continuation is even looked at. The tail re-enters this
            // arm and `eval_count`s the literal `n - 1` (cheap, exact,
            // unlike the general case) — memory stays O(1) per step no
            // matter how large `n` starts out, so a saturated count churns
            // bounded-memory-and-interruptibly instead of OOMing. A
            // choice-bearing body still pauses and resumes per iteration
            // rather than being auto-resolved (the engine-steppable
            // ruling) — unlike `Each`'s all-`Emit` fast path, a `Repeat`
            // body always gets its own `RunEffect`. `n == 0` schedules
            // nothing — a clean no-op.
            Instruction::Repeat(count, body) => {
                let n = self.eval_count(&count, frame);
                if n > 0 {
                    let items = vec![
                        WorkItem::RunEffect {
                            effect: body.clone(),
                            frame: frame.clone(),
                        },
                        WorkItem::RunEffect {
                            effect: Arc::new(Instruction::Repeat(Count::Literal(n - 1), body)),
                            frame: frame.clone(),
                        },
                    ];
                    self.schedule_front(items);
                }
            }
            // [CR#616.1g,121.2a]: the `Batch` AGGREGATE-count tier. When
            // `body` is a keyword-action `Composite` ([CR#701]), build ONE
            // future aggregate `Act` window (`batch: Some(n)` carrying the
            // cardinality) instead of `n` separate future events — a
            // count-multiplying replacement (Bruvac-style "mill twice that
            // many") then bites the ONE window BEFORE any of the `n`
            // contained per-entity futures exists ([CR#616.1g]: the outer
            // effect is chosen before the inner one), and an aggregate
            // trigger ("whenever you mill one or more cards") fires once per
            // batch, not once per card. `contents.body` carries the stored
            // PER-UNIT keyword action UNCHANGED — a PASSED aggregate's apply
            // (`step.rs`) replicates it `n` times via `Repeat`. An
            // unresolvable performer/patient, an empty Mill patient group
            // ([CR#701.17b]), a would-not-act Scry/Surveil/Fateseal/Fight
            // guard ([CR#701.22b,701.14b]), or a gone Destroy/Fight patient
            // all fizzle the WHOLE aggregate — no window at all — matching
            // `composite_items`'s own per-verb fizzle discipline
            // ([CR#701.8a,701.9a,701.17a]) exactly (`batch_act_head`).
            //
            // A non-Act body keeps the ORIGINAL shell's sequential behavior
            // below (Task 2): no aggregate window for a plain effect (YAGNI)
            // — the same lazy self-rescheduling continuation `Repeat` uses
            // (never `n`-many eagerly materialized `RunEffect` items, per
            // the CRITICAL never-crash ruling against a saturated/huge
            // count). `n == 0` schedules nothing either way — a clean no-op.
            Instruction::Batch(count, body) => {
                let n = self.eval_count(&count, frame);
                if n == 0 {
                    // Clean no-op — mirrors `Repeat`.
                } else if let Some(unit) = batch_act_unit(&body) {
                    if let Some(head) = self.batch_act_head(&unit, frame) {
                        let verb = deckmaste_core::VerbName::from(head.verb);
                        let act = GameEvent::Act(Act {
                            verb,
                            who: head.who,
                            on: head.on,
                            // [CR#603.6]: expose the source facet (Mill's
                            // `Library`) so a source-scoped cant bites the
                            // aggregate; the destination stays `None` so a
                            // `→Graveyard` replacement bites each contained
                            // per-card move, never the aggregate ([CR#616.1]).
                            from: head.from,
                            to: None,
                            cause: head.cause,
                            committed: false,
                            contents: Some(Box::new(crate::event::ActContents {
                                body: (*body).clone(),
                                frame: frame.clone(),
                            })),
                            batch: Some(n),
                            inherited: self.activation_inherited_replacements(frame.activation),
                            // The aggregate itself is never "contained" —
                            // only the n futures ITS PASSED apply schedules
                            // are ([CR#616.1g]).
                            contained: false,
                        });
                        // Only the ONE window is opened here — no
                        // `FinalizeAct` is planted yet. Unlike the move
                        // verbs (whose id-scoped `Patients` watch is safely
                        // redirect-tolerant even planted early), the
                        // aggregate's `AnyContained` watch has no patient
                        // ids to scope by — planting it here would let it
                        // see whatever an `Instead`'s OWN unrelated
                        // same-verb activity does later and spuriously
                        // finalize a REPLACED-TO-NOTHING aggregate
                        // ([CR#614.1]: a replaced event never happened).
                        // So — mirroring draw/the reorder verbs — the
                        // `FinalizeAct` is planted from the PASSED apply
                        // instead (`step.rs`), where `mark` is narrow: it
                        // covers only what THIS aggregate's own contained
                        // futures do, nothing upstream or unrelated.
                        self.schedule_front(vec![WorkItem::Emit(Occurrence::single(act))]);
                    }
                    // else: `batch_act_head` fizzled (unresolvable performer/
                    // patient, empty Mill group, would-not-act guard, gone
                    // patient) — no window at all.
                } else {
                    let items = vec![
                        WorkItem::RunEffect {
                            effect: body.clone(),
                            frame: frame.clone(),
                        },
                        WorkItem::RunEffect {
                            effect: Arc::new(Instruction::Batch(Count::Literal(n - 1), body)),
                            frame: frame.clone(),
                        },
                    ];
                    self.schedule_front(items);
                }
            }
            // [CR#702.85,701.57] dig-until (cascade/discover's shape): reveal
            // cards off the top of `whose`'s library one at a time until one
            // matches, binding the found card as `It` and the passed-over
            // prefix as `They` (the Idris `bindFound`/`bindIt`+`bindThat`),
            // then run `body`. GENUINELY ABSENT SUBSYSTEM — but NOT the
            // single-reveal seam, which is now built: `Action::Reveal` /
            // `GameEvent::Revealed` are live (the `Explore` keyword action
            // [CR#701.44a] reveals its top card through them), and the
            // predicate-match (`target::matches`) plus `It`/`They`
            // anaphora-binding machinery are built and reusable. What has no
            // engine home yet is the VARIABLE-LENGTH reveal-until-match LOOP:
            // minting and closing a reveal window per card ([CR#701.20a]) as
            // it walks the top of the library, accumulating the passed-over
            // prefix as `They`, and degrading gracefully when nothing matches.
            // That bounded iteration is a real subsystem of its own (cascade /
            // discover), out of scope here; this arm fizzles to a graceful
            // no-op instead (nothing revealed, `body` never runs) — never a
            // panic, matching the CRITICAL never-crash ruling.
            Instruction::RevealUntil(_) => {}
            // [CR#607.1]: write one of this card's linked memory cells — the
            // writer half of a linked pair (ADR law 8). The cell is keyed by
            // the object the two abilities are printed on, so the later
            // reading ability's `Provenance::Linked` parameter finds it. The
            // published value is the register's SNAPSHOT, taken here: like a
            // capture, it is not re-chased at the read site, so [CR#607.2a]'s
            // "cards in the exile zone that were put there as a result of"
            // stops naming a card that has since left ([CR#400.7]).
            Instruction::Remember(remember) => {
                let owner = frame.source(self);
                let value = self.snapshot_register(frame, remember.value, remember.kind);
                self.remember_cell(owner, remember.cell, value);
            }
            // [CR#603.7,603.12]: create a delayed triggered ability, unified
            // with the reflexive rule. It is printed on no permanent, so it
            // fires ONCE the next time its event occurs ([CR#603.7b]) — BUT if
            // that event ALREADY occurred earlier in THIS resolution (the exile
            // a `With(Produce(...))` just performed — madness's "when a card is
            // exiled this way"), it is reflexive-checked on the spot
            // ([CR#603.12]) and fires now rather than waiting for a future
            // occurrence. Only when no earlier event matches does it register in
            // the `delayed_triggers` registry for the future. Source/controller
            // follow [CR#603.7d,603.7e]; `~`/`This` is the creating object's
            // snapshot (the produced object when the source moved itself away).
            Instruction::Delayed(ability) => {
                if !self.scan_created_reflexive(&ability, frame) {
                    let (source, bindings) = self.created_trigger_context(&ability, frame);
                    let controller = frame.controller(self);
                    self.delayed_triggers.push(crate::trigger::CreatedTrigger {
                        source,
                        controller,
                        ability,
                        bindings,
                    });
                }
            }
            // [CR#603.12]: a reflexive triggered ability ("when you do") — the
            // same immediate resolution-window scan, but NEVER registered for a
            // future event.
            Instruction::Reflexive(ability) => {
                self.scan_created_reflexive(&ability, frame);
            }
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both. Named explicitly
            // so the seam below reports only genuinely unbuilt shapes.
            // What remains is the pile family: `SeparatePiles`/`ChoosePile`.
            other => todo!(
                "engine seam: stage 3 does not interpret effect {other:?} \
                 ([CR#700.3a,700.3b]) — the pile family has no resolution; owner: engine-piles"
            ),
        }
    }

    /// The TAG-facet coordinates a `Batch` aggregate window carries when its
    /// per-unit body is a keyword action ([CR#701]) — narrowed to what a
    /// replacement/trigger's `would`/pattern reads off the aggregate (verb +
    /// performer/patient), NOT the body-move resolution itself (that's each
    /// contained per-entity future's own job, once the aggregate passes).
    /// `body` is the composite's own per-unit body (`Action::Composite`'s
    /// second field — the same value [`Self::composite_items`] reads), so
    /// the two functions can share the exact same body-shape queries.
    /// Mirrors BOTH the per-verb coordinate resolution AND the fizzle
    /// discipline [`Self::composite_items`](crate::resolve::action) performs
    /// for the ordinary (non-aggregate) lane: an unresolvable performer
    /// (every verb), an empty Mill patient group ([CR#701.17b]), a
    /// would-not-act Scry/Surveil/Fateseal/Fight guard
    /// ([CR#701.22b,701.14b]), or a gone Destroy/Fight patient all fizzle the
    /// WHOLE aggregate here too — no window at all, matching
    /// `composite_items`'s discipline exactly ([CR#701.8a,701.9a,701.17a]) —
    /// never a panic.
    fn batch_act_head(&self, unit: &BatchUnit<'_>, frame: &ExecutionFrame) -> Option<BatchActHead> {
        use deckmaste_core::Agency;
        let agent = Some((frame.source(self), frame.controller(self)));
        let (name, body) = match unit {
            // Draw ([CR#121.1]) is a game action, NOT a keyword action
            // ([CR#701]) — it has no composite body, so there are no stored
            // coordinates to read: the performer comes straight off `By`'s
            // agent slot. The aggregate window still opens, because that is
            // what a count-referring replacement bites (Alhammarret's Archive,
            // [CR#121.2a,616.1g]) BEFORE any of the individual card draws
            // ([CR#121.2]) exists.
            BatchUnit::Draw(who) => {
                return Some(BatchActHead {
                    verb: "Draw",
                    who: Some(self.eval_player_ref(who, frame)?),
                    on: vec![],
                    cause: Some(Cause::draw(Agency::EffectInstruction, agent)),
                    from: None,
                });
            }
            BatchUnit::Composite(name, body) => (*name, *body),
        };
        // The performer a slice/reorder body names, resolved to a player.
        let performer =
            || composite_body_whose(body).and_then(|who| self.eval_player_ref(who, frame));
        Some(match name.as_str() {
            "Destroy" => {
                let on = self.eval_reference(composite_move_src(body)?, frame);
                // gone / zoneless patient — fizzle [CR#701.8a]
                self.objects.get(on).and_then(|o| o.zone)?;
                BatchActHead {
                    verb: "Destroy",
                    who: None,
                    on: vec![on],
                    cause: Some(Cause::destroy(Agency::EffectInstruction, agent)),
                    from: None,
                }
            }
            "Discard" => BatchActHead {
                verb: "Discard",
                who: Some(performer()?),
                on: vec![],
                cause: Some(Cause::discard(Agency::EffectInstruction, agent)),
                from: None,
            },
            "Mill" => {
                let who = Some(performer()?);
                // The top-slice group, exactly as `composite_items` resolves
                // it — an empty result (empty library / count 0) fizzles the
                // whole aggregate before any window opens [CR#701.17b].
                let patients: Vec<ObjectId> = composite_body_group(body)
                    .map(|(group, _)| self.eval_selection_set(&group, frame))
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|&o| self.objects.get(o).and_then(|x| x.zone) == Some(Zone::Library))
                    .collect();
                if patients.is_empty() {
                    return None;
                }
                BatchActHead {
                    verb: "Mill",
                    who,
                    on: vec![],
                    cause: Some(Cause::mill(Agency::EffectInstruction, agent)),
                    // Mill reads the library — a "can't leave the library" cant
                    // suppresses the whole aggregate ([CR#614.17,701.17a]).
                    from: Some(Zone::Library),
                }
            }
            verb @ ("Scry" | "Surveil" | "Fateseal") => {
                let who = Some(performer()?);
                if !self.composite_body_would_act(body, frame) {
                    return None; // scry 0 / empty peek — fizzle [CR#701.22b]
                }
                BatchActHead {
                    verb,
                    who,
                    on: vec![],
                    cause: None,
                    from: None,
                }
            }
            "Fight" => {
                let (a, b) = deckmaste_core::fight_body_fighters(body)?;
                let first = self.eval_reference(a, frame);
                let second = self.eval_reference(b, frame);
                // [CR#701.14c]: a self-fight has ONE subject — dedup so the
                // committed fact (and its trigger) fires once, not twice.
                let mut on = vec![first];
                if second != first {
                    on.push(second);
                }
                if on.iter().any(|&f| self.objects.get(f).is_none())
                    || !self.composite_body_would_act(body, frame)
                {
                    return None; // gone fighter / guard fails — fizzle [CR#701.14b]
                }
                BatchActHead {
                    verb: "Fight",
                    who: None,
                    on,
                    cause: None,
                    from: None,
                }
            }
            // An unknown verb name aggregates nothing — fizzle the whole batch.
            _ => return None,
        })
    }

    /// [CR#603.7d,603.7e]: the source and the DECLARED captures for a
    /// delayed/reflexive triggered ability created while `frame` resolves. The
    /// source is the creating object (the trigger/activated ability's own
    /// source snapshot when `frame` has one — [CR#603.7e]; otherwise the
    /// resolving spell — [CR#603.7d]). `~`/`This` is that same object's
    /// snapshot, which the created body reads through its OWN `Source`
    /// parameter — a region's source is intrinsic, not a crossing.
    ///
    /// Everything else the created body may read from the region that created
    /// it comes through `ability.effect`'s declared capture parameters (ADR
    /// law 7), snapshotted here, once, at creation ([CR#603.7a]). Nothing is
    /// hand-carried: the defending player this used to copy across is supplied
    /// by the FIRING event's own roles ([CR#608.2k] — an ability's effect
    /// refers to what its trigger condition referred to) when the body
    /// declares it, and by a declared capture when the body means the
    /// creating combat's.
    ///
    /// [CR#400.7j]: when the creating source itself MOVED during the same
    /// resolution (madness exiles the very card whose ability is discarding
    /// it — `frame.source(self)` is reminted and gone), `~`/`This` follows the
    /// source register through the move record to the object's new
    /// incarnation, so the delayed body's filter still anchors `Ref(This)` on
    /// that card and its watcher-source is a live card rather than a bare
    /// player proxy.
    fn created_trigger_context(
        &self,
        ability: &deckmaste_core::TriggeredAbility,
        frame: &ExecutionFrame,
    ) -> (ObjectSource, crate::trigger::TriggerBindings) {
        let this = frame
            .source_lki(self)
            .or_else(|| {
                self.objects
                    .get(frame.source(self))
                    .map(|_| crate::lki::LkiSnapshot::capture(self, frame.source(self)))
            })
            .or_else(|| self.moved_source_snapshot(frame));
        let source = this.as_ref().map_or_else(
            || ObjectSource::Player(frame.controller(self)),
            |s| s.source,
        );
        let bindings = crate::trigger::TriggerBindings {
            this,
            captures: self.capture_snapshot(&ability.effect, frame),
            ..crate::trigger::TriggerBindings::default()
        };
        (source, bindings)
    }

    /// [CR#400.7j]: the live snapshot of the frame's own SOURCE register,
    /// chased through the same-resolution move record to its current
    /// incarnation. `None` when the source has left play for good. Anchors a
    /// created trigger whose own source moved ITSELF away during this
    /// resolution (madness exiles the very card whose ability is discarding
    /// it, so the source id was reminted). The declared source register is the
    /// binding to chase — never the newest object in the register file, which
    /// would just as happily hand back an unrelated token the body created.
    fn moved_source_snapshot(&self, frame: &ExecutionFrame) -> Option<crate::lki::LkiSnapshot> {
        let product = self.chase_moved(frame.source(self));
        self.objects
            .get(product)
            .map(|_| crate::lki::LkiSnapshot::capture(self, product))
    }

    /// [CR#603.12]: scan the resolution-scoped window ([`Self::resolution_events`])
    /// for events that already occurred and match `ability`'s trigger event,
    /// emitting a `TriggerFired` (carrying the body by value, [CR#603.7c]) per
    /// match — the shared immediate-fire spine of a reflexive trigger and of a
    /// delayed trigger unified with it. Returns whether it fired (so the
    /// delayed arm knows to skip registering for the future). The firing
    /// event's roles bind `EventObject`/`ThatMuch`/… for the fired body
    /// ([CR#603.2e]).
    fn scan_created_reflexive(
        &mut self,
        ability: &deckmaste_core::TriggeredAbility,
        frame: &ExecutionFrame,
    ) -> bool {
        let (source, base) = self.created_trigger_context(ability, frame);
        let mut emits = Vec::new();
        for event in self.resolution_events.clone() {
            if self.event_matches_delayed(&ability.event, &event, source) {
                let roles = self.event_roles(&event);
                let mut bindings = roles.bindings_over(base.clone());
                // [CR#400.7j,603.2e]: a trigger firing reflexively WITHIN the
                // resolution that produced its event reads the moved object at
                // its CURRENT identity. The event fact's `that_object` snapshot
                // holds the object's PRE-move id; the object was reminted on
                // the move (madness's Hand → Exile remint), so
                // chase the live same-resolution move record to
                // the product and re-snapshot it — the "…exiled
                // this way" linkage. Left as-is when the object
                // did not move again or the chase leaves the store (LKI
                // stands).
                if let Some(snap) = &bindings.that_object {
                    let chased = self.chase_moved(snap.object);
                    if chased != snap.object && self.objects.get(chased).is_some() {
                        bindings.that_object = Some(crate::lki::LkiSnapshot::capture(self, chased));
                    }
                }
                emits.push(WorkItem::Emit(Occurrence::single(GameEvent::TriggerFired(
                    TriggerFired {
                        source,
                        ability: 0,
                        controller: frame.controller(self),
                        created: Some(Arc::new(ability.clone())),
                        bindings: Box::new(bindings),
                    },
                ))));
            }
        }
        let fired = !emits.is_empty();
        if fired {
            self.schedule_front(emits);
        }
        fired
    }

    /// A future-form `ZoneChange` intent ([CR#400.7]) moving `object` to `to`
    /// from WHATEVER zone it currently occupies — the current-zone lookup
    /// ([CR#406.2] "from wherever it is") bound at schedule time, with the
    /// `enters` / `position` / `face` coordinates left default (the
    /// bare-relocation case; a battlefield entry's `enters`, a library
    /// insertion's `position`, and a face-down arrival's `face` each need
    /// their own builder). Centralizes the `self.objects.obj(object).zone.
    /// expect(…)` + `…: None` boilerplate a cost-payment exile (`cast.rs`)
    /// otherwise repeats — the retired `ReturnToHand` verb's other caller;
    /// `Action::Move`'s own bounce/exile paths go through `move_items`
    /// instead, which additionally no-ops a gone reference rather than
    /// panicking.
    pub(crate) fn relocate_from_current(
        &self,
        object: ObjectId,
        to: Zone,
        cause: Option<Cause>,
    ) -> GameEvent {
        GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
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
        })
    }

    /// Whether an [`Action::Composite`]'s reorder/guarded `body` will actually
    /// do something this resolution — the fizzle gate for the verbs whose
    /// "did it happen" can't be read off flat coordinates ([CR#701.22b]: scry 0
    /// fires no "you scried" trigger; [CR#701.14b]: a fight whose guard fails
    /// does nothing). An `Each` over a peek is a no-op when the peek is empty;
    /// an `If`-guarded body looks through to the branch its condition selects.
    /// The move verbs (destroy/discard/mill) vet their coordinates directly in
    /// `composite_items` instead.
    pub(crate) fn composite_body_would_act(
        &self,
        body: &Instruction,
        frame: &ExecutionFrame,
    ) -> bool {
        match body {
            Instruction::Each(each) => !self.eval_selection_set(&each.over, frame).is_empty(),
            Instruction::If(i) => {
                if self.condition_holds(&i.condition, frame) {
                    self.composite_body_would_act(&i.then, frame)
                } else {
                    i.otherwise
                        .as_ref()
                        .is_some_and(|o| self.composite_body_would_act(o, frame))
                }
            }
            _ => true,
        }
    }

    /// Price the `{X}` symbols of a resolution-time cost ([CR#702.21b] — a
    /// ward-{X} toll's X "is determined at the time the ability resolves,
    /// not locked in as the ability triggers"): every `Variable` mana symbol
    /// becomes `Generic(X)` read from the region's DECLARED announced-X
    /// parameter, which `enter_triggered_region` filled from the ability's
    /// `where_x`. A region with no X leaves the cost unchanged.
    fn price_variable_cost(
        &self,
        cost: Vec<deckmaste_core::CostComponent>,
        frame: &ExecutionFrame,
    ) -> Vec<deckmaste_core::CostComponent> {
        use deckmaste_core::CostComponent;
        use deckmaste_core::ManaSymbol;
        use deckmaste_core::SimpleManaSymbol;
        let Some(x) = self.activation_x(frame.activation) else {
            return cost;
        };
        cost.into_iter()
            .map(|component| match component {
                CostComponent::Mana(m) => {
                    let symbols: Vec<ManaSymbol> = m
                        .iter()
                        .copied()
                        .map(|s| match s {
                            ManaSymbol::Variable => {
                                ManaSymbol::Simple(SimpleManaSymbol::Generic(x))
                            }
                            other => other,
                        })
                        .collect();
                    CostComponent::Mana(Arc::<[ManaSymbol]>::from(symbols).into())
                }
                other => other,
            })
            .collect()
    }
}

/// Split `total` among `n` recipients as evenly as possible ([CR#601.2d] — the
/// resolution-time division, summing to `total`): the first `total % n` get one
/// extra. v1 placeholder for the "as you choose" player decision; an empty
/// group yields no shares.
/// Whether an iteration's `over` selection peeks the top of a library
/// ([CR#701.22a]) — the peek whose cards the controller is granted visibility
/// over.
fn is_top_of_library_peek(over: &deckmaste_core::Selection) -> bool {
    matches!(over, deckmaste_core::Selection::TopOfLibrary { .. })
}

/// Whether a loop body puts cards into ORDERED library positions
/// ([CR#401.7]) — the signal to arm the post-pick arrange collector
/// ([CR#401.4]). True when any reachable move verb targets a
/// [`Destination::Library`] anchor (scry/surveil's top pick, fateseal); false
/// for a graveyard-only body (mill — the graveyard is unordered).
fn body_repositions_ordered(block: &[Instruction]) -> bool {
    block.iter().any(instruction_repositions_ordered)
}

fn instruction_repositions_ordered(effect: &Instruction) -> bool {
    match effect {
        Instruction::Act { action, .. } => action_moves_to_library(action),
        Instruction::Sequentially(v) | Instruction::Simultaneously(v) => {
            body_repositions_ordered(v)
        }
        Instruction::Modal(m) => m
            .modes
            .iter()
            .any(|mode| body_repositions_ordered(&mode.effect.body)),
        Instruction::Each(e) => body_repositions_ordered(&e.body.body),
        Instruction::Distribute(d) => body_repositions_ordered(&d.body.body),
        Instruction::RevealUntil(r) => body_repositions_ordered(&r.body.body),
        Instruction::Search(s) => body_repositions_ordered(&s.if_none),
        Instruction::If(i) => {
            instruction_repositions_ordered(&i.then)
                || i.otherwise
                    .as_ref()
                    .is_some_and(|o| instruction_repositions_ordered(o))
        }
        Instruction::May(m) => {
            instruction_repositions_ordered(&m.effect)
                || m.if_did
                    .as_ref()
                    .is_some_and(|e| instruction_repositions_ordered(e))
                || m.if_not
                    .as_ref()
                    .is_some_and(|e| instruction_repositions_ordered(e))
        }
        Instruction::ChoosePile(c) => instruction_repositions_ordered(&c.then),
        Instruction::SeparatePiles(p) => p
            .then
            .as_ref()
            .is_some_and(|e| instruction_repositions_ordered(e)),
        Instruction::Repeat(_, body) | Instruction::Batch(_, body) => {
            instruction_repositions_ordered(body)
        }
        _ => false,
    }
}

/// Whether a move verb relocates to an ordered [`Destination::Library`]
/// position; `Composite` looks through to its body.
fn action_moves_to_library(a: &Action) -> bool {
    match a {
        Action::Move(_, deckmaste_core::Destination::Library(_), _, _) => true,
        Action::Composite { body, .. } => instruction_repositions_ordered(body),
        _ => false,
    }
}

fn split_evenly(total: Uint, n: usize) -> Vec<Uint> {
    let Ok(n_u) = Uint::try_from(n) else {
        return vec![];
    };
    if n_u == 0 {
        return vec![];
    }
    let base = total / n_u;
    let rem = total % n_u;
    (0..n_u).map(|i| base + Uint::from(i < rem)).collect()
}

/// If `effect` is a `ForThisEvent` rider clause — `Until(ForThisEvent, parts)`
/// — return its `parts`: the instruction-scoped statics to
/// fold onto the preceding sibling in `Sequentially` lowering ([CR#611.2a]).
/// `None` for every other effect.
fn for_this_event_rider(effect: &Instruction) -> Option<&[StaticSpec]> {
    match effect {
        Instruction::Until(deckmaste_core::Duration::ForThisEvent, parts) => Some(parts),
        _ => None,
    }
}

/// A `Batch`'s per-unit body when it is one that takes an AGGREGATE `Act`
/// window ([CR#616.1g]) — the two shapes are not one type because drawing is
/// not a keyword action:
///
/// - [`Composite`](BatchUnit::Composite) — a keyword action ([CR#701]), whose
///   coordinates are read off its stored body.
/// - [`Draw`](BatchUnit::Draw) — a game action ([CR#121.1]) carried by
///   [`Action::DrawCard`], whose performer is its own agent slot. Drawing is
///   irreducible ([CR#121.5]: a Library → Hand move made without the word
///   "draw" is not a draw), so it has no body to read.
enum BatchUnit<'a> {
    Composite(&'a deckmaste_core::VerbName, &'a Instruction),
    Draw(&'a Reference),
}

/// Classify a `Batch`'s per-unit body: `Some` iff it takes an aggregate window.
/// `None` keeps the plain sequential `Repeat`-style lane.
fn batch_act_unit(unit: &Instruction) -> Option<BatchUnit<'_>> {
    match unit {
        Instruction::Act {
            action: Action::Composite { name, body },
            ..
        } => Some(BatchUnit::Composite(name, body)),
        Instruction::Act {
            action: Action::DrawCard(who),
            ..
        } => Some(BatchUnit::Draw(who)),
        _ => None,
    }
}

/// The TAG-facet coordinates [`GameState::batch_act_head`] resolves — a
/// named struct rather than a positional tuple, per [CR#616.1g]'s `Batch`
/// aggregate window needing all four independently.
struct BatchActHead {
    verb: &'static str,
    who: Option<crate::player::PlayerId>,
    on: Vec<ObjectId>,
    cause: Option<Cause>,
    // The SOURCE facet ([CR#603.6]): a slice-verb aggregate exposes the zone
    // its cards leave (`Mill`'s `Library`) so a source-scoped `CantHappen`
    // ("cards can't leave your library") suppresses the WHOLE aggregate before
    // any card moves ([CR#614.17]), exactly as it bit the per-move facet. The
    // DESTINATION stays `None`: a destination-scoped replacement (Rest in
    // Peace's `→Graveyard`) must bite each contained per-card move
    // INDIVIDUALLY, never the aggregate ([CR#616.1]).
    from: Option<Zone>,
}

/// The mutable structural [`DeonticAction`] of a `Deontic`
/// (May/Cant/Must/Gate).
fn deontic_action_mut(d: &mut Deontic) -> &mut DeonticAction {
    match d {
        Deontic::May(a) | Deontic::Cant(a) | Deontic::Must(a) | Deontic::Gate(a, _) => a,
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    }
}

/// The candidate SUBJECT predicate slots of a `DeonticAction` — the slots a
/// one-shot restriction locks its affected object into ([CR#611.2c]). BOTH a
/// two-slot deed's `by`/`on` are candidates because either can be the subject
/// (active "X can't block" locks `by`; passive "X can't be blocked" locks
/// `on`); the lock rewrites whichever is a bare object `Ref`. `Target`'s agent
/// is a [`DeedAgent`](deckmaste_core::DeedAgent), not a `Predicate`, so only
/// its `on` is a slot.
fn deontic_subject_slots(a: &mut DeonticAction) -> Vec<&mut Predicate> {
    match a {
        DeonticAction::Attack { by, on } | DeonticAction::Block { by, on, .. } => vec![by, on],
        DeonticAction::Regenerate { by, on } | DeonticAction::Counter { by, on } => vec![on, by],
        DeonticAction::Target { on, .. } => vec![on],
        DeonticAction::Attach { what, to } => vec![what, to],
        DeonticAction::Cast { what, by, .. }
        | DeonticAction::Play { what, by, .. }
        | DeonticAction::Activate { what, by, .. } => vec![by, what],
        DeonticAction::Untap { what } => vec![what],
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    }
}

/// Merge simultaneous `DamageDealt` packets sharing `(source, target, combat)`
/// into one instance by summing amounts, preserving first-seen order; every
/// other event passes through unchanged. This is what makes a self-fight
/// (`X` fights `X` → two `X -> X` packets) deal ONE instance of twice its power
/// ([CR#701.14c]) rather than two, once `Fight` is a `Simultaneously` macro
/// over two `DealDamage`s. Distinct-target fights (the common case) are
/// untouched.
fn coalesce_simultaneous_damage(events: Vec<GameEvent>) -> Vec<GameEvent> {
    let mut out: Vec<GameEvent> = Vec::with_capacity(events.len());
    for ev in events {
        if let GameEvent::DamageDealt(DamageDealt {
            source,
            target,
            amount,
            combat,
        }) = ev
        {
            let existing = out.iter_mut().find_map(|e| match e {
                GameEvent::DamageDealt(DamageDealt {
                    source: s,
                    target: t,
                    amount: a,
                    combat: c,
                }) if *s == source && *t == target && *c == combat => Some(a),
                _ => None,
            });
            match existing {
                Some(a) => *a += amount,
                None => out.push(GameEvent::DamageDealt(DamageDealt {
                    source,
                    target,
                    amount,
                    combat,
                })),
            }
        } else {
            out.push(ev);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]

    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::ChosenValueKind;
    use deckmaste_core::Count;
    use deckmaste_core::Countable;
    use deckmaste_core::Instruction;
    use deckmaste_core::LifeOp;
    use deckmaste_core::ObjectClass;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::Act;
    use crate::event::ControlChanged;
    use crate::event::DamageDealt;
    use crate::event::GameEvent;
    use crate::event::LifeGained;
    use crate::event::Occurrence;
    use crate::event::TriggerFired;
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::stack::ExecutionFrame;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::state::GameState;
    use crate::step::Progress;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    /// Ability target declarations are outside the instruction block; the
    /// instruction reads its already-bound target directly.
    #[test]
    fn targeted_effect_resolves_its_inner_effect() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(&state, bear, vec![bear]);
        state.run_effect(
            Instruction::Act(Action::deal_damage(
                Reference::Reg(deckmaste_core::RefId(6)),
                Count::Literal(3),
            )),
            &frame,
        );
        // RunEffect(DealDamage) → Emit(DamageDealt).
        for _ in 0..3 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).total_damage(), 3);
    }

    /// The real Do or Die card compiles through the register-backed pile
    /// shape and reaches the deliberately unimplemented resolution boundary;
    /// lowering must not reject it first.
    #[test]
    #[should_panic(expected = "owner: engine-piles")]
    fn do_or_die_reaches_the_engine_piles_seam() {
        let Card::Normal(face) = canon().card("Do or Die").unwrap().core else {
            panic!("Do or Die should be single-faced");
        };
        let [Ability::Spell(spell)] = face.abilities.as_slice() else {
            panic!("expected one spell ability");
        };
        let [effect] = spell.effect.body.as_ref() else {
            panic!("expected one pile instruction");
        };
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(&state, bear);
        state.run_effect(effect.clone(), &frame);
    }

    #[test]
    fn each_creature_yields_all_battlefield_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second Grizzly Bears from player 0's hand onto the
        // battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(&state, a);
        let filter = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let mut got = state.eval_selection_set(
            &Selection::SelectAll(Arc::new(deckmaste_core::Region::candidate(filter))),
            &frame,
        );
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want);
    }

    /// [CR#608.2] "[body], [count] times": a plain (non-choice) body runs the
    /// evaluated count total — `count` is read ONCE, up front (`eval_count`),
    /// never re-evaluated mid-loop.
    #[test]
    fn repeat_runs_a_plain_body_count_times() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let body = Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::Literal(2)),
        ));
        state.run_effect(
            Instruction::Repeat(Count::Literal(3), Arc::new(body)),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);

        assert_eq!(
            state.player(p0).life,
            life0 + 6,
            "three iterations of +2 life = +6 total"
        );
    }

    /// A count of zero schedules nothing — never a panic, never a hang; the
    /// CRITICAL never-crash ruling applies to a degenerate `Repeat` exactly
    /// as it does to every other new arm.
    #[test]
    fn repeat_zero_is_a_no_op() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        let agenda_before = state.agenda.len();

        let body = Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::Literal(2)),
        ));
        state.run_effect(
            Instruction::Repeat(Count::Literal(0), Arc::new(body)),
            &frame,
        );

        assert_eq!(
            state.agenda.len(),
            agenda_before,
            "count=0 schedules no ADDITIONAL work (a fresh game already has its own \
             turn-structure agenda queued, so this compares the delta, not raw emptiness)"
        );
        let _ = drain_progress(&mut state, 5);
        assert_eq!(state.player(p0).life, life0, "no iterations ran");
    }

    /// A choice-bearing body (`May`) surfaces its OWN decision on EVERY
    /// repetition — never auto-resolved (the engine-steppable ruling): the
    /// second iteration's yes/no is not decided until the first iteration's
    /// answer has actually been submitted and applied.
    #[test]
    fn repeat_over_a_choice_bearing_body_steps_each_iteration_independently() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        // Pump steps until a decision surfaces (a `RunEffect` work item's own
        // `step()` call only *sets* `self.pending` as a side effect and
        // returns `Progress::Resolving` for that step; `NeedsDecision` is
        // reported on the NEXT `step()` call, which sees `pending` already
        // set — so this may take more than one `step()`).
        fn step_to_decision(state: &mut GameState) -> crate::decide::DecisionPointKind {
            loop {
                match state.step() {
                    StepOutcome::NeedsDecision(d) => return d,
                    StepOutcome::Progress(_) => {}
                    StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
                }
            }
        }

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let may_gain_3 = || {
            Instruction::May(deckmaste_core::May {
                who: Reference::Reg(deckmaste_core::RefId(1)),
                effect: Arc::new(Instruction::Act(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Up(Count::Literal(3)),
                ))),
                if_did: None,
                if_not: None,
            })
        };
        state.run_effect(
            Instruction::Repeat(Count::Literal(2), Arc::new(may_gain_3())),
            &frame,
        );

        // First iteration's decision.
        let DecisionPointKind::YesNo(crate::decide::pending::YesNo { player }) =
            step_to_decision(&mut state)
        else {
            panic!("expected the first iteration's YesNo");
        };
        assert_eq!(player, p0);
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 10);
        assert_eq!(
            state.player(p0).life,
            life0 + 3,
            "first iteration applied alone"
        );

        // Second iteration's OWN decision — not skipped, not pre-answered.
        let DecisionPointKind::YesNo(crate::decide::pending::YesNo { player }) =
            step_to_decision(&mut state)
        else {
            panic!("expected the second iteration's own YesNo");
        };
        assert_eq!(player, p0);
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 10);
        assert_eq!(state.player(p0).life, life0 + 6, "both iterations applied");
    }

    /// A saturated/huge count must never eagerly allocate — the CRITICAL
    /// never-crash/never-hang ruling applies exactly as much to a mistaken
    /// `Repeat(Literal(4_000_000_000), body)` as to a panic: `eval_count`
    /// has no clamp (arithmetic `Count`s *saturate* toward `u32::MAX`), so
    /// the lazy self-rescheduling continuation must schedule exactly TWO
    /// work items per step — this iteration's `body` plus a
    /// `Repeat(Literal(n - 1), body)` tail — never `n` of them.
    #[test]
    fn repeat_with_huge_count_does_not_eagerly_allocate() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let agenda_before = state.agenda.len();

        let body = Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::Literal(1)),
        ));
        state.run_effect(
            Instruction::Repeat(Count::Literal(1_000_000), Arc::new(body)),
            &frame,
        );

        assert_eq!(
            state.agenda.len(),
            agenda_before + 2,
            "a single Repeat step schedules exactly two work items — this iteration's body \
             plus a Repeat(n-1, body) tail continuation — never count-many eagerly \
             materialized RunEffect items"
        );
        match (&state.agenda[0], &state.agenda[1]) {
            (
                WorkItem::RunEffect { effect: first, .. },
                WorkItem::RunEffect { effect: second, .. },
            ) => {
                assert!(
                    matches!(**first, Instruction::Act { .. }),
                    "the front item is this iteration's own body, not another Repeat layer"
                );
                match &**second {
                    Instruction::Repeat(Count::Literal(n), _) => {
                        assert_eq!(
                            *n, 999_999,
                            "the tail carries the DECREMENTED remaining count"
                        );
                    }
                    other => panic!("expected a Repeat(Literal(n - 1), body) tail, got {other:?}"),
                }
            }
            other => panic!("expected two RunEffect work items at the agenda front, got {other:?}"),
        }
    }

    /// `Batch` shell: for a non-`Act` body, purely sequential-equivalent to
    /// `Repeat` (the [CR#616.1g] aggregate-count tier only exists for an
    /// `Act`-shaped body — see `bruvac_shape_batch_doubles_the_aggregate_...`
    /// and friends below for that path) — two `Batch(2, GainLife(1))`
    /// iterations record as TWO separate `LifeGained` facts, exactly like
    /// `Repeat`, not one combined aggregate fact.
    #[test]
    fn batch_runs_sequentially_recording_one_fact_per_iteration() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let body = Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::Literal(1)),
        ));
        state.run_effect(
            Instruction::Batch(Count::Literal(2), Arc::new(body)),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);

        assert_eq!(
            state.player(p0).life,
            life0 + 2,
            "two iterations of +1 life = +2 total"
        );
        let life_gained_facts = state
            .history
            .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
            .filter(|e| matches!(e, GameEvent::LifeGained(LifeGained { .. })))
            .count();
        assert_eq!(
            life_gained_facts, 2,
            "shell Batch resolves sequentially — TWO separate LifeGained facts, not \
             one combined aggregate fact (the aggregate-count tier only applies to an \
             Act-shaped body)"
        );
    }

    /// A count of zero schedules nothing for `Batch` either — the CRITICAL
    /// never-crash ruling applies to a degenerate `Batch` exactly as it does
    /// to `Repeat`.
    #[test]
    fn batch_zero_is_a_no_op() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        let agenda_before = state.agenda.len();

        let body = Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::Literal(2)),
        ));
        state.run_effect(
            Instruction::Batch(Count::Literal(0), Arc::new(body)),
            &frame,
        );

        assert_eq!(
            state.agenda.len(),
            agenda_before,
            "count=0 schedules no ADDITIONAL work"
        );
        let _ = drain_progress(&mut state, 5);
        assert_eq!(state.player(p0).life, life0, "no iterations ran");
    }

    /// Mint a fresh card-backed object into `owner`'s library (bottom —
    /// identity/order is irrelevant to the mill/draw tests that use this;
    /// only the COUNT of cards moved matters). Mirrors `fixtures.rs`'s
    /// `mint_in_hand`, but for the library zone the `Batch` aggregate tests
    /// need to mill/draw from.
    fn mint_in_library(state: &mut GameState, owner: PlayerId, name: &str) -> ObjectId {
        let cid = state.cards.push(
            Arc::new(Card::Normal(deckmaste_card::CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                ..deckmaste_card::CardFace::default()
            })),
            owner,
        );
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Library));
        state.zones.libraries[owner.index()].push_back(id);
        id
    }

    /// How many `GameEvent::TriggerFired` occurrences got APPLIED while
    /// draining up to `n` steps — the direct fire-count signal (mirrors
    /// `trigger.rs`'s own fire-counting helpers, but counts occurrences
    /// actually applied during a drain rather than snapshotting the agenda
    /// once). `scan_triggers` always emits `TriggerFired` as an
    /// `Occurrence::Single`, never batched with other emits.
    fn count_trigger_fired(state: &mut GameState, n: usize) -> usize {
        drain_progress(state, n)
            .into_iter()
            .filter(|p| {
                matches!(
                    p,
                    Progress::Applied(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                )
            })
            .count()
    }

    /// [CR#616.1g,121.2a] Bruvac shape: `Instead(would: Act(Mill(Any)),
    /// instead: Batch(2×ThatMany, Act(Mill(You,1))))` layered over an
    /// ORIGINAL `Batch(3, Act(Mill(You,1)))` — mill 3 becomes mill 6, via
    /// ONE replacement decision made against ONE aggregate window (never
    /// three independently-doubled per-card windows), and that decision is
    /// made BEFORE any card physically moves.

    /// [CR#614.5] Archive shape: `Instead(would: Act(Draw(You)), instead:
    /// Batch(2, Act(Draw(You,1))))` — a plain draw-1 becomes draw 2, and the
    /// SAME replacement does NOT re-apply to its own products. Without the
    /// [CR#614.5] tree-scoping, the doubled aggregate — and each of its two
    /// contained per-card draws — would ALSO match `Act(Draw(You))` and get
    /// replaced again: an unbounded "draw 2, which becomes draw 2, which
    /// becomes draw 2, …" loop that never commits a single card, so the
    /// hand size would stay at its STARTING count instead of growing by 2.
    #[test]
    fn archive_shape_replacement_does_not_reapply_to_its_own_products() {
        let mut state = game();
        let p0 = PlayerId(0);
        for i in 0..6 {
            mint_in_library(&mut state, p0, &format!("Card {i}"));
        }
        let hand0 = state.zones.hands[p0.index()].len();

        let archive = deckmaste_core::Replacement::Instead {
            would: deckmaste_core::EventFilter::Act {
                verb: deckmaste_core::VerbName::from("Draw"),
                who: Predicate::Any,
                on: Predicate::Any,
                cause: None,
            },
            instead: Instruction::Batch(
                Count::Literal(2),
                Arc::new(Instruction::Act(deckmaste_core::Action::draw_one(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
            ),
        };
        mint_on_field(
            &mut state,
            Card::Normal(deckmaste_card::CardFace {
                name: "Archive Stand-In".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![deckmaste_core::Ability::r#static(
                    deckmaste_core::StaticSpec::Replacement(Arc::new(archive)),
                )],
                ..deckmaste_card::CardFace::default()
            }),
        );

        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::draw(Reference::Reg(deckmaste_core::RefId(1)), Count::Literal(1)),
            &frame,
        );
        let _ = drain_progress(&mut state, 60);

        assert_eq!(
            state.zones.hands[p0.index()].len(),
            hand0 + 2,
            "draw 1 became draw 2 — the replacement fired exactly once against the \
             aggregate, not repeatedly against its own products ([CR#614.5])"
        );
    }

    /// [CR#616.1g,121.2a,701.17a]: an Act-level "whenever you mill one or
    /// more cards" trigger fires ONCE per `Batch` aggregate, not once per
    /// contained card — `Batch(3, Act(Mill(You,1)))` mills 3 cards as ONE
    /// instruction, so the watcher fires exactly once (not four times: once
    /// for the aggregate plus once per of its three contained per-card
    /// futures).
    #[test]
    fn batch_aggregate_trigger_fires_once_per_batch_not_once_per_card() {
        let mut state = game();
        let p0 = PlayerId(0);
        for i in 0..5 {
            mint_in_library(&mut state, p0, &format!("Card {i}"));
        }
        mint_on_field(
            &mut state,
            Card::Normal(deckmaste_card::CardFace {
                name: "Mill Watcher".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![deckmaste_core::Ability::triggered(
                    deckmaste_core::TriggeredAbility {
                        ability_word: None,
                        where_x: None,
                        targets: [].into(),
                        from: None,
                        condition: None,
                        limits: Vec::new().into(),
                        event: deckmaste_core::EventFilter::Act {
                            verb: deckmaste_core::VerbName::from("Mill"),
                            who: Predicate::Any,
                            on: Predicate::Any,
                            cause: None,
                        },
                        effect: Instruction::Act(Action::ChangeLife(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            LifeOp::Up(Count::Literal(1)),
                        ))
                        .into(),
                    },
                )],
                ..deckmaste_card::CardFace::default()
            }),
        );

        let frame = frame_for(&state, p0);
        let mill_one = Instruction::Act(deckmaste_core::Action::mill_one(Reference::Reg(
            deckmaste_core::RefId(1),
        )));
        state.run_effect(
            Instruction::Batch(Count::Literal(3), Arc::new(mill_one)),
            &frame,
        );
        let fired = count_trigger_fired(&mut state, 60);

        assert_eq!(
            fired, 1,
            "the aggregate trigger fires ONCE for the whole batch, not once per \
             contained card"
        );
        assert_eq!(
            state.zones.graveyards[p0.index()].len(),
            3,
            "all three cards still milled individually"
        );
    }

    /// [CR#616.1g,701.14a]: a `Batch(2, Fight(a, b))` records ONE aggregate
    /// fight fact per fighter, not one fact per contained iteration. Fight has
    /// no zone-change result for `AnyContained` to observe; the aggregate must
    /// consume the contained fights' own successful finalization decisions.
    #[test]
    fn batch_fight_records_one_aggregate_fact_per_fighter() {
        let mut state = game();
        let fighter = |name: &str| {
            Card::Normal(deckmaste_card::CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(5)),
                ..deckmaste_card::CardFace::default()
            })
        };
        let a = mint_on_field(&mut state, fighter("Batch Fighter A"));
        let b = mint_on_field(&mut state, fighter("Batch Fighter B"));
        let frame = frame_src_targets(&state, a, vec![a, b]);

        state.run_effect(
            Instruction::Batch(
                Count::Literal(2),
                Arc::new(fight_effect(
                    &Reference::Reg(deckmaste_core::RefId(6)),
                    &Reference::Reg(deckmaste_core::RefId(7)),
                )),
            ),
            &frame,
        );
        let _ = drain_progress(&mut state, 100);

        let fights: Vec<_> = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, GameEvent::Act(Act { verb, committed: true, .. })
                    if verb.as_str() == "Fight")
            })
            .collect();
        assert_eq!(
            fights.len(),
            2,
            "the aggregate records one fact per fighter, not per contained fight"
        );
        let subjects: Vec<_> = fights
            .iter()
            .map(|e| match &e.fact {
                GameEvent::Act(Act { on, .. }) => on.clone(),
                _ => unreachable!(),
            })
            .collect();
        assert!(
            subjects.iter().any(|on| on.as_slice() == [a])
                && subjects.iter().any(|on| on.as_slice() == [b])
        );
        assert!(
            fights[0].batch.is_some() && fights[0].batch == fights[1].batch,
            "the aggregate's per-fighter facts share one history batch id"
        );
    }

    /// [CR#120.8,701.14a]: an all-zero-power fight still happened even though
    /// zero damage ultimately produces no `DamageDealt` fact. Aggregate Fight
    /// finalization therefore cannot use damage (or zone movement) as its
    /// success signal; it must inherit the contained Fight finalizer's
    /// `BodyRan` result.
    #[test]
    fn batch_fight_with_zero_power_still_records_its_aggregate_fact() {
        let mut state = game();
        let fighter = |name: &str| {
            Card::Normal(deckmaste_card::CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(0)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                ..deckmaste_card::CardFace::default()
            })
        };
        let a = mint_on_field(&mut state, fighter("Zero Fighter A"));
        let b = mint_on_field(&mut state, fighter("Zero Fighter B"));
        let frame = frame_src_targets(&state, a, vec![a, b]);

        state.run_effect(
            Instruction::Batch(
                Count::Literal(1),
                Arc::new(fight_effect(
                    &Reference::Reg(deckmaste_core::RefId(6)),
                    &Reference::Reg(deckmaste_core::RefId(7)),
                )),
            ),
            &frame,
        );
        let _ = drain_progress(&mut state, 60);

        let subjects: Vec<_> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::Act(Act {
                    verb,
                    on,
                    committed: true,
                    ..
                }) if verb.as_str() == "Fight" => Some(on.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(subjects.len(), 2, "both zero-power fighters still fought");
        assert!(
            subjects.iter().any(|on| on.as_slice() == [a])
                && subjects.iter().any(|on| on.as_slice() == [b])
        );
    }

    /// [CR#616.1g,701.17b]: `Batch(2, Act(Mill(You,1)))` over an EMPTY
    /// library fizzles the WHOLE aggregate — `batch_act_head` must resolve
    /// the Mill patient group (mirroring `composite_items`'s own
    /// `[CR#701.17b]` empty-library fizzle) BEFORE opening the aggregate
    /// window, not after. Asserted directly at the strongest point: no
    /// ADDITIONAL agenda item exists immediately after `run_effect` returns
    /// — no window is ever scheduled, so no replacement/trigger could ever
    /// observe one — plus (after a drain) no card moved and no committed
    /// `Act(Mill)` fact exists.
    #[test]
    fn batch_mill_over_empty_library_fizzles_the_whole_aggregate() {
        let mut state = game();
        let p0 = PlayerId(0);
        // `game()`'s players start with an empty deck — no `mint_in_library`
        // calls here, unlike the other `Batch`/Mill tests — the degenerate
        // case under test.
        assert!(
            state.zones.libraries[p0.index()].is_empty(),
            "empty-library precondition"
        );

        let frame = frame_for(&state, p0);
        let mill_one = Instruction::Act(deckmaste_core::Action::mill_one(Reference::Reg(
            deckmaste_core::RefId(1),
        )));
        let agenda_before = state.agenda.len();
        state.run_effect(
            Instruction::Batch(Count::Literal(2), Arc::new(mill_one)),
            &frame,
        );

        assert_eq!(
            state.agenda.len(),
            agenda_before,
            "an empty-library Batch(Mill) schedules no ADDITIONAL work — the whole \
             aggregate fizzles before any window opens, matching composite_items's own \
             [CR#701.17b] empty-library Mill fizzle"
        );

        let _ = drain_progress(&mut state, 10);

        assert!(
            state.zones.graveyards[p0.index()].is_empty(),
            "nothing milled — the library was empty"
        );
        let mill_commits = state
            .history
            .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
            .filter(|e| {
                matches!(e, GameEvent::Act(Act { verb, committed: true, .. }) if verb.as_str() == "Mill")
            })
            .count();
        assert_eq!(
            mill_commits, 0,
            "no committed Act(Mill) fact — the aggregate never opened a window at all"
        );
    }

    /// [CR#608.2c,607.2] an activation register round-trips a NUMBER choice:
    /// `ChooseAndNote(Number)` surfaces a resolution-time number decision; the
    /// submitted value lands in a register; a LATER clause of the SAME
    /// resolution reads that register. The two clauses run as
    /// separate `Sequentially` children — each carrying its OWN frame clone
    /// (`Sequentially` clones the frame per child up front) — so this asserts
    /// the value rides the shared activation record, not either frame clone.
    #[test]
    fn choose_and_note_number_round_trips_through_the_register() {
        const NOTE: deckmaste_core::DefId = deckmaste_core::DefId(7);

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, a) = bear_on_field();
        let effect = Instruction::Sequentially(
            vec![
                Instruction::ChooseValue(deckmaste_core::ChooseValue {
                    dest: NOTE,
                    by: Reference::Reg(deckmaste_core::RefId(1)),
                    domain: ChosenValueKind::Number,
                }),
                Instruction::mill(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Reg(NOTE.into()),
                ),
            ]
            .into(),
        );
        let frame = frame_src(&state, a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        // The first child surfaced the number choice.
        let Some(DecisionPointKind::ChooseNoteNumber(crate::decide::pending::ChooseNoteNumber {
            player,
            key: pk,
        })) = state.pending.clone()
        else {
            panic!("expected ChooseNoteNumber, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(0),
            "the effect's controller notes the number"
        );
        assert_eq!(pk, deckmaste_core::Ident::from("register"));

        state
            .submit_decision(Decision::XValue(2))
            .expect("a non-negative note number is always legal");
        assert_eq!(
            state.activation_number(frame.activation, NOTE.into()),
            Some(2)
        );

        // The SECOND child reads it back by index and mills exactly 2.
        run_injected(&mut state);
        assert_eq!(
            state.zones.graveyards[0].len(),
            2,
            "the second child reads 2 from the shared activation register"
        );
    }

    /// [CR#608.2c] scope: activation registers do not leak between activations.
    #[test]
    fn resolution_registers_do_not_leak_into_a_fresh_activation() {
        let (state, a) = bear_on_field();
        let first = frame_src(&state, a);
        state.activation_write_number(first.activation, deckmaste_core::DefId(7), 5);
        let second = frame_src(&state, a);
        assert_eq!(
            state.activation_number(second.activation, deckmaste_core::RefId(7)),
            None,
            "a fresh activation cannot observe the prior activation's register"
        );
    }

    /// [CR#607.2a,608.2d] a CONSTRAINING `AmongNoted` quantity ("destroy one of
    /// them") surfaces a `ChooseObjects` chooser over the noted group's LIVE
    /// members, honors the quantity's bounds, and binds the picks into the
    /// re-run body — exactly the chooser the unconstrained full-group read does
    /// not need.

    /// [CR#607.2] `ChooseValue(Color)` has no reader grammar yet — it's
    /// grammar-spellable, so a card reaching it is a live path, and the
    /// engine's never-crash doctrine means that fizzles (no work item, no
    /// note written) rather than panicking. `resolve::player_action::tests::
    /// choose_value_color_fizzles_no_reader_grammar_yet` pins the same
    /// behavior directly on `player_action_items`; this pins it through the
    /// full `run_effect` dispatch path.
    #[test]
    fn choose_value_color_fizzles_without_panic() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(&state, a);
        state.run_effect(
            Instruction::Act(Action::ChooseValue(
                Reference::Reg(deckmaste_core::RefId(1)),
                ChosenValueKind::Color,
                deckmaste_core::Ident::from("c"),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert!(state.pending.is_none(), "no decision surfaces — fizzles");
    }

    #[test]
    fn choose_value_card_name_reaches_the_printed_name_filter() {
        const NAME: deckmaste_core::DefId = deckmaste_core::DefId(7);

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, a) = bear_on_field();
        let frame = frame_src(&state, a);
        state.run_effect(
            Instruction::ChooseValue(deckmaste_core::ChooseValue {
                dest: NAME,
                by: Reference::Reg(deckmaste_core::RefId(1)),
                domain: ChosenValueKind::CardName,
            }),
            &frame,
        );
        run_injected(&mut state);
        assert!(matches!(
            state.pending,
            Some(DecisionPointKind::ChooseNoteCardName(crate::decide::pending::ChooseNoteCardName { key: pending, .. })) if pending == deckmaste_core::Ident::from("register")
        ));
        state
            .submit_decision(Decision::CardName("Grizzly Bears".to_owned()))
            .unwrap();
        assert_eq!(
            state
                .activation_symbol(frame.activation, NAME.into())
                .as_deref(),
            Some("Grizzly Bears")
        );
        assert!(crate::target::matches_with_activation(
            &state,
            a,
            &deckmaste_core::Predicate::Characteristic(
                deckmaste_core::CharacteristicPredicate::NamedReg(NAME.into()),
            ),
            Some(state.objects.obj(a).source),
            frame.activation,
        ));
    }

    /// Seam filled for the new decision kind: the mechanical strategy
    /// notes the minimum (0, the X=0 default, via the reused `Decision::XValue`
    /// answer), and the seat resolver names the deciding player.
    #[test]
    fn strategy_and_seat_cover_the_note_number_choice() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (state, _a) = bear_on_field();
        let pending =
            DecisionPointKind::ChooseNoteNumber(crate::decide::pending::ChooseNoteNumber {
                player: PlayerId(1),
                key: deckmaste_core::Ident::from("n"),
            });
        assert_eq!(
            crate::sim::mechanical(&state, &pending),
            Decision::XValue(0)
        );
        assert_eq!(crate::sim::pending_player(&pending), PlayerId(1));
    }

    /// The exchange-control card, through `Simultaneously`
    /// ([CR#701.12a..701.12b]): the `ExchangeControl` macro's two halves
    /// read ONE pre-application snapshot, land as one `ControlChanged`
    /// batch, and the two creatures swap controllers — each
    /// summoning-sick for its new controller ([CR#302.6]).
    #[test]
    fn exchange_control_swaps_controllers_through_one_simultaneous_batch() {
        let (mut state, mine, other) = two_permanents_on_field();
        // Re-home `other` to player 1 so the exchange crosses seats.
        state.objects.obj_mut(other).controller = PlayerId(1);

        // Parsed through the SEMANTICS path (`semantics::Instruction` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::OneShotEffect = builtin()
            .macros
            .read_str(r"ExchangeControl(Target(0), Target(1))")
            .unwrap();
        let frame = frame_src_targets(&state, mine, vec![mine, other]);
        schedule_lowered_effect(&mut state, semantic, 2, &frame);
        let _ = state.step();

        // ONE batch of two ControlChanged facts.
        let front = state.agenda.front().cloned();
        let Some(WorkItem::Emit(Occurrence::Batch(events))) = front else {
            panic!("expected one ControlChanged batch, got {front:?}");
        };
        assert_eq!(events.len(), 2, "both halves in one occurrence");
        assert!(
            events
                .iter()
                .all(|e| matches!(e, GameEvent::ControlChanged(ControlChanged { .. }))),
            "the exchange is a batch of control transitions, got {events:?}"
        );

        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(mine).controller,
            PlayerId(1),
            "player 1 gained control of player 0's creature"
        );
        assert_eq!(
            state.objects.obj(other).controller,
            PlayerId(0),
            "player 0 gained control of player 1's creature — both halves read \
             the PRE-exchange controllers (one snapshot)"
        );
        assert!(
            state.objects.obj(mine).summoning_sick && state.objects.obj(other).summoning_sick,
            "newly controlled permanents are summoning-sick ([CR#302.6])"
        );
    }

    /// [CR#701.12b]: exchanging control of two permanents the SAME player
    /// controls does nothing — each half is a no-transition no-op, and the
    /// all-or-nothing rule ([CR#701.12a]) voids the empty set.
    #[test]
    fn same_controller_exchange_does_nothing() {
        let (mut state, mine, other) = two_permanents_on_field();
        // Parsed through the SEMANTICS path (`semantics::Instruction` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::OneShotEffect = builtin()
            .macros
            .read_str(r"ExchangeControl(Target(0), Target(1))")
            .unwrap();
        let frame = frame_src_targets(&state, mine, vec![mine, other]);
        schedule_lowered_effect(&mut state, semantic, 2, &frame);
        let _ = state.step();
        assert!(
            !state.agenda.iter().any(|w| matches!(w, WorkItem::Emit(_))),
            "a same-controller exchange emits nothing ([CR#701.12b])"
        );
    }

    /// Avarice Totem's own body ([CR#701.12a,701.12b]): a `Simultaneously` of
    /// two mirrored `Continuously(EndOfGame, Modify(_, SetController(
    /// ControllerOf(_))))` halves, the Idris `exchangeControl` macro's exact
    /// shape — deliberately NOT the `Action::GainControl`-based
    /// `ExchangeControl` builtin macro `same_controller_exchange_does_nothing`
    /// above exercises.
    fn avarice_totem_body(this: &Reference, target: &Reference) -> Instruction {
        let half = |subject: &Reference, other: &Reference| {
            Instruction::Continuously(deckmaste_core::Continuously {
                effect: Arc::new(deckmaste_core::StaticSpec::Modify(
                    subject.clone(),
                    deckmaste_core::Modification::SetController(
                        deckmaste_core::Reference::ControllerOf(Arc::new(other.clone())),
                    ),
                )),
                duration: deckmaste_core::Duration::EndOfGame,
            })
        };
        Instruction::Simultaneously(vec![half(this, target), half(target, this)].into())
    }

    /// Avarice Totem's ability ([CR#701.12a,701.12b]): the two `Continuously`
    /// halves mint as ONE simultaneous all-or-nothing swap — a `Continuously`
    /// member has no event to ride the batch, so it must mint its static row
    /// through the same read-only-then-commit split an `Act` member's
    /// events do, and both halves must read the PRE-exchange controllers (one
    /// snapshot, [CR#611.2c]).
    #[test]
    fn avarice_totem_swaps_control_through_two_continuously_members() {
        let (mut state, mine, other) = two_permanents_on_field();
        state.objects.obj_mut(other).controller = PlayerId(1);

        let effect = avarice_totem_body(
            &Reference::Reg(deckmaste_core::RefId(0)),
            &Reference::Reg(deckmaste_core::RefId(6)),
        );
        let frame = frame_src_targets(&state, mine, vec![other]);
        state.run_effect(effect, &frame);

        assert_eq!(
            state.layers().controller(mine),
            PlayerId(1),
            "player 1 gained control of player 0's totem"
        );
        assert_eq!(
            state.layers().controller(other),
            PlayerId(0),
            "player 0 gained control of player 1's permanent — both halves read \
             the PRE-exchange controllers (one snapshot)"
        );
    }

    /// [CR#701.12b]: exchanging control of two permanents the SAME player
    /// controls does nothing. Unlike the `Action::GainControl`-based
    /// `same_controller_exchange_does_nothing` above, a `Continuously` member
    /// mints a static row rather than an event, so there is no "empty batch"
    /// to test — the two rows each set the controller to the value it
    /// already holds, which is an observably inert no-op ([CR#701.12b]) even
    /// though the rows themselves exist.
    #[test]
    fn avarice_totem_same_controller_exchange_is_a_no_op() {
        let (mut state, mine, other) = two_permanents_on_field();

        let effect = avarice_totem_body(
            &Reference::Reg(deckmaste_core::RefId(0)),
            &Reference::Reg(deckmaste_core::RefId(6)),
        );
        let frame = frame_src_targets(&state, mine, vec![other]);
        state.run_effect(effect, &frame);

        assert_eq!(
            state.layers().controller(mine),
            PlayerId(0),
            "control does not change when both permanents share a controller"
        );
        assert_eq!(
            state.layers().controller(other),
            PlayerId(0),
            "control does not change when both permanents share a controller"
        );
    }

    /// [CR#701.14a]: a fight — each creature deals damage equal to its power to
    /// the other, as ONE simultaneous batch of noncombat ([CR#701.14d]) damage
    /// facts; SBAs run after the whole batch. The `Composite Fight` fires its
    /// keyword-action fact once the guarded body acts.
    #[test]
    fn fight_deals_each_others_power_as_one_noncombat_batch() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(&state, a, vec![a, b]);
        state.run_effect(
            fight_effect(
                &Reference::Reg(deckmaste_core::RefId(6)),
                &Reference::Reg(deckmaste_core::RefId(7)),
            ),
            &frame,
        );
        // Both packets land as ONE applied batch occurrence.
        let batch = drain_progress(&mut state, 30)
            .into_iter()
            .find_map(|p| match p {
                Progress::Applied(Occurrence::Batch(evs))
                    if evs
                        .iter()
                        .all(|e| matches!(e, GameEvent::DamageDealt(DamageDealt { .. }))) =>
                {
                    Some(evs)
                }
                _ => None,
            })
            .expect("one batch of fight damage");
        assert_eq!(batch.len(), 2, "both damage packets in one occurrence");
        assert!(
            batch
                .iter()
                .all(|e| matches!(e, GameEvent::DamageDealt(DamageDealt { combat: false, .. }))),
            "fight damage is noncombat damage ([CR#701.14d]), got {batch:?}"
        );
        assert_eq!(state.objects.obj(a).total_damage(), 2, "a took b's power");
        assert_eq!(state.objects.obj(b).total_damage(), 2, "b took a's power");
        assert!(
            logged(&state, |e| matches!(e, GameEvent::Act(Act { .. }))),
            "the fight fired its 'fights' keyword-action fact"
        );
    }

    /// [CR#701.14b]: both-or-neither — a fighter that is no longer a creature on
    /// the battlefield when the fight would occur means NEITHER deals damage,
    /// and no "fights" fact fires (the `If` guard is false).
    #[test]
    fn fight_with_a_gone_fighter_deals_no_damage_at_all() {
        let (mut state, a, b) = two_permanents_on_field();
        // b leaves before the fight resolves.
        state.zones.battlefield.retain(|&o| o != b);
        state.objects.remove(b);
        let frame = frame_src_targets(&state, a, vec![a, b]);
        state.run_effect(
            fight_effect(
                &Reference::Reg(deckmaste_core::RefId(6)),
                &Reference::Reg(deckmaste_core::RefId(7)),
            ),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            !logged(&state, |e| matches!(
                e,
                GameEvent::DamageDealt(DamageDealt { .. })
            )),
            "neither creature deals damage ([CR#701.14b])"
        );
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Act(Act { .. }))),
            "no fight occurred, so no 'fights' fact"
        );
        assert_eq!(state.objects.obj(a).total_damage(), 0);
    }

    /// [CR#701.14c]: a creature fighting itself deals damage to itself equal to
    /// TWICE its power — the macro's two `X -> X` packets coalesce to one
    /// instance in the simultaneous batch.
    #[test]
    fn self_fight_deals_twice_its_power_to_itself() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(&state, a, vec![a, a]);
        state.run_effect(
            fight_effect(
                &Reference::Reg(deckmaste_core::RefId(6)),
                &Reference::Reg(deckmaste_core::RefId(7)),
            ),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(a).total_damage(),
            4,
            "a 2/2 fighting itself takes 2 x 2 = 4 ([CR#701.14c])"
        );
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::DamageDealt(DamageDealt {
                    amount: 4,
                    combat: false,
                    ..
                })
            )),
            "one coalesced damage instance of twice its power"
        );
    }

    /// [CR#701.14a]: the committed fight fact is PER SUBJECT — one committed
    /// `Act(Fight)` per combatant, members of ONE batch occurrence (shared
    /// history batch id: they were one fight).
    #[test]
    fn fight_commits_one_fact_per_fighter_sharing_a_batch_id() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(&state, a, vec![a, b]);
        state.run_effect(
            fight_effect(
                &Reference::Reg(deckmaste_core::RefId(6)),
                &Reference::Reg(deckmaste_core::RefId(7)),
            ),
            &frame,
        );
        run_injected(&mut state);
        let fights: Vec<_> = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, GameEvent::Act(Act { verb, committed: true, .. })
                    if verb.as_str() == "Fight")
            })
            .collect();
        assert_eq!(fights.len(), 2, "one committed fact per combatant");
        let subjects: Vec<_> = fights
            .iter()
            .map(|e| match &e.fact {
                GameEvent::Act(Act { on, .. }) => on.clone(),
                _ => unreachable!(),
            })
            .collect();
        assert!(subjects.contains(&vec![a]) && subjects.contains(&vec![b]));
        assert!(
            fights[0].batch.is_some() && fights[0].batch == fights[1].batch,
            "the two per-subject facts share one batch id ([CR#603.2c])"
        );
    }

    /// [CR#701.14c]: a self-fight has ONE subject — one committed fact.
    #[test]
    fn self_fight_commits_exactly_one_fact() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(&state, a, vec![a, a]);
        state.run_effect(
            fight_effect(
                &Reference::Reg(deckmaste_core::RefId(6)),
                &Reference::Reg(deckmaste_core::RefId(7)),
            ),
            &frame,
        );
        run_injected(&mut state);
        let count = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, GameEvent::Act(Act { verb, committed: true, .. })
                    if verb.as_str() == "Fight")
            })
            .count();
        assert_eq!(count, 1, "self-fight fires once ([CR#701.14c])");
    }

    /// [CR#120.8,701.14b]: a 0-power fighter still FOUGHT — its per-subject
    /// fact records regardless of any damage event (the fact derives from the
    /// body instructions, never from `DamageDealt`).
    #[test]
    fn zero_power_fighter_still_records_its_fight_fact() {
        // Darksteel Myr is canon's 0/1 (see myr_on_field,
        // resolve/action.rs:971-1003).
        let (mut state, a, _b) = two_permanents_on_field();
        let myr_card = Arc::new(canon().card("Darksteel Myr").unwrap().core);
        let cid = state.cards.push(myr_card, PlayerId(0));
        let myr = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(myr);
        let frame = frame_src_targets(&state, a, vec![myr, a]);
        state.run_effect(
            fight_effect(
                &Reference::Reg(deckmaste_core::RefId(6)),
                &Reference::Reg(deckmaste_core::RefId(7)),
            ),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            state.history.entries().any(|e| matches!(&e.fact,
                GameEvent::Act(Act { verb, on, committed: true, .. })
                    if verb.as_str() == "Fight" && on.as_slice() == [myr])),
            "the 0-power fighter's own fact recorded"
        );
    }

    /// [CR#611.2]/[CR#611.2c]: `Instruction::Continuously(Modify(Matching(...), ...),
    /// UntilEndOfTurn)` — the resolve arm pushes one `ContinuousEffect` with a
    /// `ScopeResolved::Floating` scope and the right duration/changes.

    /// [CR#611.2c]: `Instruction::Continuously(Modify(Of(This), ...), ...)` locks
    /// the id at creation — `ScopeResolved::Locked(vec![src])`.
    #[test]
    fn continuously_of_this_registers_locked_scope() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticSpec;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);

        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::Toughness(NumericOp::Up(Count::Literal(2))),
            )),
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
            vec![Modification::Toughness(NumericOp::Up(Count::Literal(2)))]
        );
    }

    /// FIXTURE — a resolved effect grants an activated ability whose body
    /// reads the granting region's controller. The granted object changes
    /// controller before activation, proving the later body reads the frozen
    /// grant-time capture rather than its new bare frame ([CR#611.2,613.1f]).
    #[test]
    fn a_granted_ability_body_reads_its_grant_time_capture() {
        use deckmaste_core::Ability;
        use deckmaste_core::ActivatedAbility;
        use deckmaste_core::Continuously;
        use deckmaste_core::Cost;
        use deckmaste_core::DefId;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Kind;
        use deckmaste_core::LifeOp;
        use deckmaste_core::Modification;
        use deckmaste_core::Param;
        use deckmaste_core::Provenance;
        use deckmaste_core::RefId;
        use deckmaste_core::Reference;
        use deckmaste_core::Region;
        use deckmaste_core::StaticSpec;

        let (mut state, _) = bear_on_field();
        let captured_controller = RefId(2);
        let granted = Ability::activated(ActivatedAbility {
            ability_word: None,
            cost: Cost::default(),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            targets: Arc::from([]),
            effect: Region::new(
                Arc::from([
                    Param {
                        def: DefId(0),
                        kind: Kind::Entity,
                        provenance: Provenance::Source,
                    },
                    Param {
                        def: DefId(1),
                        kind: Kind::Entity,
                        provenance: Provenance::Controller,
                    },
                    Param {
                        def: DefId(captured_controller.0),
                        kind: Kind::Entity,
                        provenance: Provenance::Capture(RefId(1)),
                    },
                ]),
                Instruction::Act(deckmaste_core::Action::ChangeLife(
                    Reference::Reg(captured_controller),
                    LifeOp::Down(deckmaste_core::Count::Literal(1)),
                ))
                .into(),
            ),
        });
        let grant_effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::Modify(
                Reference::Reg(RefId(0)),
                Modification::GainAbility(Arc::new(granted)),
            )),
            duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        });
        let grant_region = Region::new(
            Arc::from([
                Param {
                    def: DefId(0),
                    kind: Kind::Entity,
                    provenance: Provenance::Source,
                },
                Param {
                    def: DefId(1),
                    kind: Kind::Entity,
                    provenance: Provenance::Controller,
                },
            ]),
            grant_effect.clone().into(),
        );
        let fixture = mint_on_field(
            &mut state,
            Card::Normal(deckmaste_card::CardFace {
                name: "Grant Capture Fixture".into(),
                types: vec![Type::Creature.def()],
                abilities: vec![Ability::activated(ActivatedAbility {
                    ability_word: None,
                    cost: Cost::default(),
                    from: None,
                    window: None,
                    condition: None,
                    limits: Arc::from([]),
                    targets: Arc::from([]),
                    effect: grant_region.clone(),
                })],
                ..deckmaste_card::CardFace::default()
            }),
        );
        let bare = frame_src(&state, fixture);
        let activation = state.enter_region(&grant_region, &bare);
        let grant_frame = ExecutionFrame {
            activation,
            payment: None,
        };
        state.run_effect(grant_effect, &grant_frame);
        state.remove_activation_family(activation);

        // The ability is activated under a different controller. Its own
        // Controller parameter is now P1, while capture 2 must remain P0.
        state.objects.obj_mut(fixture).controller = PlayerId(1);
        let abilities = crate::derive::usable_abilities(&state, fixture);
        let index = abilities
            .iter()
            .rposition(|ability| ability.as_activated().is_some())
            .expect("the layer-6 grant is usable");
        state.begin_activate(fixture, index);
        let pending = state
            .announcing
            .as_ref()
            .expect("activation announcement opened");
        let StackObject::Activated { ability, .. } = &pending.object else {
            panic!("the granted ability is the pending activation")
        };
        let body = ability.effect.body[0].clone();
        let frame = ExecutionFrame {
            activation: pending.activation,
            payment: None,
        };
        let p0_before = state.player(PlayerId(0)).life;
        let p1_before = state.player(PlayerId(1)).life;
        state.run_effect(body, &frame);
        run_injected(&mut state);

        assert_eq!(state.player(PlayerId(0)).life, p0_before - 1);
        assert_eq!(
            state.player(PlayerId(1)).life,
            p1_before,
            "the later bare activation controller did not replace the grant-time capture"
        );
    }

    /// [CR#611.2c]: a granted `Deontic` ("target creature can't block this
    /// turn") mints a static ROW, not a layer change — its subject `Target(0)`
    /// resolves to the locked object at mint and is rewritten to `Ref(It)`.

    /// A granted `CostModifier` is self-filtered (empty lock) and lands as a
    /// row.
    #[test]
    fn continuously_cost_modifier_mints_self_filtered_row() {
        use deckmaste_core::Continuously;
        use deckmaste_core::CostChange;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticSpec;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::CostModifier {
                of: Predicate::creature(),
                change: CostChange::Increase(vec![].into()),
            }),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Locked(ids) if ids.is_empty()),
            "self-filtered → empty lock"
        );
        assert!(ce.changes.is_empty());
        assert!(matches!(&ce.rows[..], [StaticSpec::CostModifier { .. }]));
    }

    /// A granted `CantHappen` is self-filtered and lands as a row.
    #[test]
    fn continuously_cant_happen_mints_self_filtered_row() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticSpec;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let filter = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::CantHappen(filter)),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
        let ce = &state.continuous[0];
        assert!(ce.changes.is_empty());
        assert!(matches!(&ce.rows[..], [StaticSpec::CantHappen(_)]));
    }

    /// A granted `Prevention` is LOUD — `engine-granted-prevention-rows` owns
    /// the machinery. (`engine-prevention` shipped only the one-shot
    /// shields; it never unblocked this granted-continuous arm.)
    #[test]
    #[should_panic(expected = "Continuously(Prevention)")]
    fn continuously_prevention_is_loud() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Predicate;
        use deckmaste_core::Prevention;
        use deckmaste_core::StaticSpec;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::Prevention(Arc::new(
                Prevention::PreventNextInstance {
                    from: Predicate::Any,
                    to: Predicate::Any,
                },
            ))),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
    }

    /// [CR#611.2b]: a `ForAsLongAs` whose condition is already false at creation
    /// never starts — no instance is pushed.
    #[test]
    fn for_as_long_as_false_at_mint_never_starts() {
        use deckmaste_core::Condition;
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticSpec;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        // "for as long as an object matching not-anything exists" — always
        // false.
        let cond = Condition::Exists(Predicate::Not(Arc::new(Predicate::Any)));
        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::Power(NumericOp::Up(Count::Literal(1))),
            )),
            duration: Duration::ForAsLongAs(cond),
        });
        state.run_effect(effect, &frame);
        assert!(
            state.continuous.is_empty(),
            "a false-at-mint ForAsLongAs pushes no instance"
        );
    }

    /// A resolved `BecomesCopy` is a deferred-exec seam
    /// (engine-layers-1-copy-facedown-text owns the layer-1a copiable-value
    /// install): `Continuously(BecomesCopy(..))` and its `Until(..)`-list
    /// spelling must both FIZZLE — no panic, no continuous instance minted —
    /// never hit the `Continuously` match's `other => todo!(...)` catch-all.
    #[test]
    fn becomes_copy_continuously_fizzles_without_panic() {
        use deckmaste_core::Continuously;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticSpec;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let spec = CopySpec {
            source: CopySource::SelfCard,
            exceptions: vec![],
        };

        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::BecomesCopy(
                Reference::Reg(deckmaste_core::RefId(0)),
                spec.clone(),
            )),
            duration: Duration::EndOfGame,
        });
        state.run_effect(effect, &frame);
        assert!(
            state.continuous.is_empty(),
            "Continuously(BecomesCopy(..)) fizzles — installs no continuous instance"
        );

        let until = Instruction::Until(
            Duration::EndOfGame,
            vec![StaticSpec::BecomesCopy(
                Reference::Reg(deckmaste_core::RefId(0)),
                spec,
            )]
            .into(),
        );
        state.run_effect(until, &frame);
        // `Until` lowers its one part to a scheduled
        // `WorkItem::RunEffect { effect: Continuously(...), .. }`
        // (pushed to the agenda FRONT) rather than resolving it inline —
        // one `step()` pops and runs exactly that item.
        let _ = state.step();
        assert!(
            state.continuous.is_empty(),
            "Until(duration, [BecomesCopy(..)]) fizzles — installs no continuous instance"
        );
    }

    /// [CR#611.2b]: `sweep_condition_durations` removes an instance whose
    /// condition has lapsed — the once-stopped-never-resumes latch is the
    /// removal itself.
    #[test]
    fn sweep_condition_durations_removes_lapsed_for_as_long_as() {
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Predicate;

        let (mut state, src) = bear_on_field();
        let cond = Condition::Exists(Predicate::Not(Arc::new(Predicate::Any)));
        let origin = frame_src(&state, src);
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::ForAsLongAs(cond),
            origin: Some(Box::new(origin)),
            is_cda: false,
        });
        state.sweep_condition_durations();
        assert!(
            state.continuous.is_empty(),
            "a lapsed ForAsLongAs is removed"
        );
    }

    /// [CR#611.2a],[CR#701.19c]: a `Sequentially` folds an `Until(ForThisEvent,
    /// [Cant(Regenerate)])` child onto the IMMEDIATELY-preceding sibling — an
    /// `InstallRiders` armed with the destroy subject is scheduled just before
    /// that sibling's `RunEffect`, and the rider mints no continuous instance.
    #[test]
    fn sequentially_folds_for_this_event_rider_before_preceding_sibling() {
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::LifeOp;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticSpec;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let seq = Instruction::Sequentially(
            vec![
                Instruction::Act(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Up(Count::Literal(1)),
                )),
                Instruction::Until(
                    Duration::ForThisEvent,
                    vec![StaticSpec::Deontic(Deontic::Cant(
                        DeonticAction::Regenerate {
                            by: Predicate::Any,
                            on: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                        },
                    ))]
                    .into(),
                ),
            ]
            .into(),
        );
        state.run_effect(seq, &frame);

        let front: Vec<&WorkItem> = state.agenda.iter().take(2).collect();
        assert!(
            matches!(front[0], WorkItem::InstallRiders { no_regen } if no_regen == &vec![src]),
            "InstallRiders armed with the destroy subject, got {:?}",
            front[0]
        );
        assert!(
            matches!(front[1], WorkItem::RunEffect { .. }),
            "the preceding sibling runs next"
        );
        assert!(
            state.continuous.is_empty(),
            "the rider child mints no continuous instance"
        );
    }

    /// [CR#611.2a]: a `ForThisEvent` rider with NO preceding sibling is an
    /// semantic-input error — it fizzles (dropped), never mints or panics.
    #[test]
    fn for_this_event_rider_without_preceding_sibling_fizzles() {
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticSpec;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let seq = Instruction::Sequentially(
            vec![Instruction::Until(
                Duration::ForThisEvent,
                vec![StaticSpec::Deontic(Deontic::Cant(
                    DeonticAction::Regenerate {
                        by: Predicate::Any,
                        on: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    },
                ))]
                .into(),
            )]
            .into(),
        );
        state.run_effect(seq, &frame);
        assert!(
            !state
                .agenda
                .iter()
                .any(|w| matches!(w, WorkItem::InstallRiders { .. })),
            "a rider with no host instruction is dropped"
        );
    }

    /// [CR#611.2,614.3]: the shared sweepable-duration guard makes a
    /// non-sweepable (`ForThisEvent`) shield LOUD at `create_shield` — a rider
    /// duration never mints a stored instance.
    #[test]
    #[should_panic(expected = "non-sweepable duration")]
    fn create_shield_rejects_non_sweepable_duration() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::Duration;
        use deckmaste_core::PhaseStep;
        use deckmaste_core::Replacement;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        // The sweepable-duration guard is LOUD before the subject register
        // read.
        state.create_shield(
            &deckmaste_core::Reference::source_parameter(),
            Replacement::Skip {
                what: PhaseStep::Beginning(BeginningStep::Untap),
            },
            Duration::ForThisEvent,
            false,
            &frame,
        );
    }

    /// [CR#511.2]: an "until end of combat" instance is swept by
    /// `expire_end_of_combat`, and by the cleanup catch-all when combat is
    /// skipped.
    #[test]
    fn until_end_of_combat_expires_at_end_of_combat_and_via_cleanup_catch_all() {
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let mint = |state: &mut GameState| {
            state.continuous.push(crate::layer::ContinuousEffect {
                timestamp: crate::object::Timestamp(1),
                controller: PlayerId(0),
                scope: crate::layer::ScopeResolved::Locked(vec![src]),
                changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
                rows: vec![],
                duration: Duration::FixedUntil(TurnMarker::EndOfCombat),
                origin: None,
                is_cda: false,
            });
        };
        mint(&mut state);
        state.expire_end_of_combat();
        assert!(state.continuous.is_empty(), "swept at end of combat");
        // Combat-skipped turn: cleanup's catch-all still removes it.
        mint(&mut state);
        state.expire_end_of_turn();
        assert!(
            state.continuous.is_empty(),
            "cleanup catch-all sweeps a leaked EndOfCombat effect"
        );
    }

    /// [CR#611.2a]: an "until your next turn" instance survives an opponent's
    /// turn and expires as its controller's next turn begins.
    #[test]
    fn until_your_next_turn_survives_opponent_expires_at_controller() {
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::FixedUntil(TurnMarker::YourNextTurn),
            origin: None,
            is_cda: false,
        });
        state.expire_your_next_turn(PlayerId(1));
        assert_eq!(state.continuous.len(), 1, "survives the opponent's turn");
        state.expire_your_next_turn(PlayerId(0));
        assert!(
            state.continuous.is_empty(),
            "expires as the controller's turn begins"
        );
    }

    /// [CR#610.3]: an `UntilEvent` instance is removed once its awaited event is
    /// applied, and survives a non-matching one.
    #[test]
    fn until_event_removed_on_match_survives_nonmatch() {
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Predicate;

        let (mut state, src) = bear_on_field();
        let filter = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        let origin = frame_src(&state, src);
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::UntilEvent(filter),
            origin: Some(Box::new(origin)),
            is_cda: false,
        });
        // A non-matching fact (a life gain) leaves it in place.
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::LifeGained(
            LifeGained {
                player: PlayerId(0),
                amount: 1,
                cause: None,
            },
        )));
        assert_eq!(
            state.continuous.len(),
            1,
            "survives a non-matching occurrence"
        );
        // The awaited damage fact ends it.
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::DamageDealt(
            DamageDealt {
                source: src,
                target: src,
                amount: 1,
                combat: false,
            },
        )));
        assert!(
            state.continuous.is_empty(),
            "removed once the awaited event happens"
        );
    }

    /// [CR#601.2d]: `Distribute` splits the amount across the binder's group and
    /// binds each element's `Allotment` share in scope for its body — divided
    /// damage deals the split shares, summing to the total, ≥1 to each.

    /// [CR#601.2d,120.3]: dividing damage among a MIXED group — a creature AND a
    /// player (Arc Lightning's `CreatureOrPlayer`) — must not panic. The player
    /// element is a zoneless proxy with no LKI snapshot, so its `It` binding is
    /// a player; the body's `It` reads it kind-poly: the creature takes its
    /// share as marked damage, the player loses life by its share.

    /// [CR#608.2,120.3]: `Each` over the players binds each zoneless player
    /// proxy as the iteration anaphor `It` — the body's `DealDamage(This, 1,
    /// It)` resolves to the player and each loses 1 life, with no panic on
    /// the snapshotless element.

    /// Ticket (the first-of-many fix): a many-binder iterated by `Each` acts on
    /// EVERY element. The Brainstorm shape `Each(Choose(2, …), Destroy(It))`
    /// chooses two creatures and destroys BOTH — the dropped-cardinality bug
    /// (which acted on only the first) is unrepresentable now that the binder
    /// surfaces its choice and `Each` iterates the whole group ([CR#608.2]).

    /// Ticket: a nested `Each` CLEARS the outer `Distribute` allotment (the
    /// Idris allotment-clearing `bindIt`), so an outer per-element share cannot
    /// leak into the inner loop ([CR#601.2d]). Once the inner `Each` rebinds
    /// `It`, the share is gone, and the inner body's `Count::Allotment` read
    /// has nothing in scope and is rejected — proving threading is
    /// add-AND-clear.

    /// [CR#608.2c]: `Instruction::If` evaluates its condition WHEN it resolves and
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
            Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(n)),
            ))
        };

        let p0 = PlayerId(0);

        // true → then (gain 3), otherwise NOT taken.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Instruction::If(If {
                condition: yes.clone(),
                then: Arc::new(gain(3)),
                otherwise: Some(Arc::new(gain(5))),
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
            Instruction::If(If {
                condition: no.clone(),
                then: Arc::new(gain(3)),
                otherwise: Some(Arc::new(gain(5))),
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
            Instruction::If(If {
                condition: no.clone(),
                then: Arc::new(gain(3)),
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

    /// [CR#118.12]: `Instruction::May` surfaces a yes/no to the controller. Yes runs
    /// `effect` then `if_did`; no runs `if_not` (nothing when absent). Driven
    /// via `GainLife` so each branch reads as a clean life delta.
    #[test]
    fn run_effect_may_branches_on_the_answer() {
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let gain = |n| {
            Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(n)),
            ))
        };
        let may = || May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(gain(3)),
            if_did: Some(Arc::new(gain(10))),
            if_not: Some(Arc::new(gain(1))),
        };
        let p0 = PlayerId(0);

        // yes → effect (3) + if_did (10) = +13; surfaces YesNo to the
        // controller.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(may()), &frame);
        let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(crate::decide::pending::YesNo {
            player,
        })) = state.step()
        else {
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
        state.run_effect(Instruction::May(may()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "no → if_not");

        // no + no if_not → nothing.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Instruction::May(May {
                who: Reference::Reg(deckmaste_core::RefId(1)),
                effect: Arc::new(gain(3)),
                if_did: None,
                if_not: None,
            }),
            &frame,
        );
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0, "no + no if_not → no change");
    }

    /// [CR#700.2]: `Instruction::Modal` surfaces `ChooseModes`; the chosen modes'
    /// effects run in written order. "Choose one" of three life-gain modes —
    /// picking index 1 gains 5; "choose two" runs both picks (+3+7); bad picks
    /// (too many, out of range) are rejected.
    #[test]
    fn run_effect_modal_runs_chosen_modes() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let gain_mode = |n| Mode {
            targets: [].into(),
            effect: Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(n)),
            ))
            .into(),
            cost: deckmaste_core::Cost::default(),
        };
        let modes = || vec![gain_mode(3), gain_mode(5), gain_mode(7)];
        let spec = |count, up_to| ChooseSpec {
            count: deckmaste_core::Quantity::Range(
                Some(Count::Literal(count)),
                Some(Count::Literal(count)),
            ),
            up_to,
            repeats: false,
            chooser: Reference::Reg(deckmaste_core::RefId(1)),
            rider: None,
        };
        let p0 = PlayerId(0);

        // choose one → ChooseModes(options 3, [1,1]); pick mode 1 → +5.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Instruction::Modal(Modal {
                choose: spec(1, false),
                modes: modes().into(),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(DecisionPointKind::ChooseModes(
            crate::decide::pending::ChooseModes {
                player,
                options,
                min,
                max,
                repeats,
                entwine,
            },
        )) = state.step()
        else {
            panic!("expected ChooseModes, got {:?}", state.pending);
        };
        assert_eq!(
            (player, options, min, max, repeats, entwine),
            (p0, 3, 1, 1, false, false)
        );
        assert!(
            state
                .submit_decision(Decision::Modes(vec![0, 1]))
                .is_err_and(|err| err.to_string().contains("illegal mode selection")),
            "too many modes"
        );
        assert!(
            state
                .submit_decision(Decision::Modes(vec![5]))
                .is_err_and(|err| err.to_string().contains("illegal mode selection")),
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
            Instruction::Modal(Modal {
                choose: spec(2, false),
                modes: modes().into(),
            }),
            &frame,
        );
        state.submit_decision(Decision::Modes(vec![0, 2])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "both chosen modes run");

        // A lowered `Sequentially` is a multi-instruction mode block. Every
        // instruction runs; resolution must not truncate the block to its
        // first element.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            Instruction::Modal(Modal {
                choose: spec(1, false),
                modes: vec![Mode {
                    targets: [].into(),
                    effect: Instruction::Sequentially(
                        vec![
                            Instruction::Act(Action::ChangeLife(
                                Reference::Reg(deckmaste_core::RefId(1)),
                                LifeOp::Up(Count::Literal(2)),
                            )),
                            Instruction::Act(Action::ChangeLife(
                                Reference::Reg(deckmaste_core::RefId(1)),
                                LifeOp::Up(Count::Literal(4)),
                            )),
                        ]
                        .into(),
                    )
                    .into(),
                    cost: deckmaste_core::Cost::default(),
                }]
                .into(),
            }),
            &frame,
        );
        state.submit_decision(Decision::Modes(vec![0])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 6,
            "the whole mode block runs"
        );
    }

    /// Regression guard for the crash this task fixed: canon's Collective
    /// Resistance ("Escalate {G} … Choose one or more —" with a target on
    /// every mode) used to hit `todo!()` on resolution — a modal whose modes
    /// carry targets, and separately a modal whose `choose` carries a cost
    /// rider, both used to panic ([CR#601.2b,700.2c,700.2h,702.120a]). Both
    /// must now fizzle (no pending decision, no continuation, no panic)
    /// instead — the announce-time feature they need is unbuilt and tracked
    /// at `docs/tickets/planned/engine-modal-announce-time.md`.
    #[test]
    fn modal_with_per_mode_targets_or_rider_fizzles_without_panic() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::Modal;
        use deckmaste_core::ModalCostRider;
        use deckmaste_core::Mode;
        use deckmaste_core::Predicate;
        use deckmaste_core::Quantity;
        use deckmaste_core::TargetSpec;

        let p0 = PlayerId(0);
        // The verb doesn't matter — only that the mode carries a top-level
        // `Targeted` wrapper (`top_targets` non-empty), matching Collective
        // Resistance's "Destroy target artifact" / "target creature gains…"
        // modes. A harmless life-gain stands in for the verb.
        let destroy_target = |predicate: Predicate| Mode {
            targets: vec![TargetSpec::Target(
                Quantity::one(),
                Arc::new(deckmaste_core::Region::candidate(predicate)),
            )]
            .into(),
            effect: Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(0)),
            ))
            .into(),
            cost: deckmaste_core::Cost::default(),
        };
        let gain_mode = |n| Mode {
            targets: [].into(),
            effect: Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(n)),
            ))
            .into(),
            cost: deckmaste_core::Cost::default(),
        };
        let escalate = || {
            Some(ModalCostRider::Escalate(Cost(
                vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(1)),
                ))]
                .into(),
            )))
        };

        // The Collective Resistance shape: every mode targeted, AND an
        // escalate rider — the per-mode-targets check fires first.
        let mut state = game();
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::Modal(Modal {
                choose: ChooseSpec {
                    count: Quantity::Range(Some(Count::Literal(1)), None),
                    up_to: false,
                    repeats: false,
                    chooser: Reference::Reg(deckmaste_core::RefId(1)),
                    rider: escalate(),
                },
                modes: vec![
                    destroy_target(Predicate::Any),
                    destroy_target(Predicate::Any),
                    destroy_target(Predicate::Any),
                ]
                .into(),
            }),
            &frame,
        );
        assert!(
            state.pending.is_none(),
            "per-mode targets + rider fizzles — no decision opened, no panic"
        );
        assert!(state.choice.is_none(), "no continuation left behind");

        // Rider alone (no per-mode targets) exercises the SECOND todo!() site
        // in isolation.
        let mut state = game();
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::Modal(Modal {
                choose: ChooseSpec {
                    count: Quantity::one(),
                    up_to: false,
                    repeats: false,
                    chooser: Reference::Reg(deckmaste_core::RefId(1)),
                    rider: escalate(),
                },
                modes: vec![gain_mode(3), gain_mode(5)].into(),
            }),
            &frame,
        );
        assert!(
            state.pending.is_none(),
            "rider alone fizzles — no decision opened, no panic"
        );
        assert!(state.choice.is_none(), "no continuation left behind");
    }

    /// [CR#608.2d,700.2e]: `Modal.choose.chooser` is the decider ([CR#700.2e],
    /// Fatal Lore's "An opponent chooses one —") — surfaced via
    /// `acting_player`, not hardcoded to the controller. `chooser: You` is
    /// the only spelling in canon today, so this pins the new routing for a
    /// non-`You` decider without changing any existing card's behavior.
    #[test]
    fn run_effect_modal_surfaces_choose_modes_to_the_named_chooser() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;

        use crate::decide::DecisionPointKind;

        let gain_mode = |n| Mode {
            targets: [].into(),
            effect: Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(n)),
            ))
            .into(),
            cost: deckmaste_core::Cost::default(),
        };
        let p0 = PlayerId(0);
        let p1 = PlayerId(1);

        let mut state = game();
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::Modal(Modal {
                choose: ChooseSpec {
                    count: Quantity::one(),
                    up_to: false,
                    repeats: false,
                    chooser: Reference::OpponentOf(std::sync::Arc::new(Reference::Reg(
                        deckmaste_core::RefId(1),
                    ))),
                    rider: None,
                },
                modes: vec![gain_mode(3), gain_mode(5)].into(),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(DecisionPointKind::ChooseModes(
            crate::decide::pending::ChooseModes { player, .. },
        )) = state.step()
        else {
            panic!("expected ChooseModes, got {:?}", state.pending);
        };
        assert_eq!(player, p1, "the NAMED chooser decides, not the controller");
    }

    /// [CR#118.12,608.2d]: `May.who` is the decider — Browbeat's "Any
    /// player may have Browbeat deal 5 damage to them" proves it isn't always
    /// the controller. Surfaced via `acting_player`, not hardcoded to
    /// `frame.controller(self)`. `who: You` is the only spelling in canon
    /// today, so this pins the new routing for a non-`You` decider without
    /// changing any existing card's behavior.
    #[test]
    fn run_effect_may_surfaces_yes_no_to_the_named_decider() {
        use deckmaste_core::May;

        use crate::decide::DecisionPointKind;

        let p0 = PlayerId(0);
        let p1 = PlayerId(1);
        let mut state = game();
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::May(May {
                who: Reference::OpponentOf(std::sync::Arc::new(Reference::Reg(
                    deckmaste_core::RefId(1),
                ))),
                effect: Arc::new(Instruction::Act(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Up(Count::Literal(3)),
                ))),
                if_did: None,
                if_not: None,
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(crate::decide::pending::YesNo {
            player,
        })) = state.step()
        else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p1, "the NAMED decider decides, not the controller");
    }

    /// [CR#118.12a]: the collapsed `May(Pay(cost))` `MustPay` shape (`if_did`
    /// absent) — the Mana Leak punisher over the full `Cost` (the English
    /// "unless" order is the `Unless` macro over this node). Pay → the cost
    /// runs and `if_not` is skipped; decline → `if_not` runs.
    #[test]
    fn run_effect_may_pay_shape_pays_or_suffers_if_not() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::payment::FulfillmentWitness;
        use crate::payment::PaymentCommand;

        let p0 = PlayerId(0);
        let pay_cost = || {
            Instruction::Act(Action::Pay(Cost(
                vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(2)),
                ))]
                .into(),
            )))
        };
        let must_pay = || May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(pay_cost()),
            if_did: None,
            if_not: Some(Arc::new(Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(10)),
            )))),
        };

        // "I'll pay" → lose 2, the punisher (gain 10) skipped.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(must_pay()), &frame);
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("expected optional payment, got {:?}", state.pending);
        };
        assert_eq!(prompt.payer, p0, "the payer decides");
        let iou = prompt.outstanding[0].id;
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::PayLife,
            }))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        state
            .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 - 2,
            "pay → cost paid, if_not skipped"
        );

        // "won't pay" → the punisher runs (gain 10).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(must_pay()), &frame);
        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "decline → if_not runs");
    }

    /// [CR#603,608]: the collapsed `May(Pay(cost))` `MayPay` shape (`if_did`
    /// present) — a resolution-time kicker. Pay → the cost runs THEN
    /// `if_did`; decline → `if_not`. The PAID branch running a follow-up
    /// effect is what the punisher (`if_did` absent) shape cannot express.
    #[test]
    fn run_effect_may_pay_shape_runs_if_did_on_pay_or_if_not_on_decline() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::payment::FulfillmentWitness;
        use crate::payment::PaymentCommand;

        let p0 = PlayerId(0);
        let pay_cost = || {
            Instruction::Act(Action::Pay(Cost(
                vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(2)),
                ))]
                .into(),
            )))
        };
        let may_pay = || May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(pay_cost()),
            if_did: Some(Arc::new(Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(10)),
            )))),
            if_not: Some(Arc::new(Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(1)),
            )))),
        };

        // "I'll pay" → lose 2 THEN gain 10 (net +8) — the kicker fires.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(may_pay()), &frame);
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("expected optional payment, got {:?}", state.pending);
        };
        assert_eq!(prompt.payer, p0, "the payer decides");
        let iou = prompt.outstanding[0].id;
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::PayLife,
            }))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        state
            .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 8,
            "pay → cost (−2) then if_did (+10)"
        );

        // "won't pay" → if_not runs (gain 1).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(may_pay()), &frame);
        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "decline → if_not runs");
    }

    /// [CR#118.12]: the branch is decided by whether the payer submitted the
    /// complete payment, "regardless of what events actually occurred." A
    /// sacrifice cost demonstrates the boundary: fulfilling the IOU performs
    /// the sacrifice, but `if_did` is not scheduled until `SubmitPayment`.
    #[test]
    fn run_effect_may_pay_branches_on_submission_not_observed_events() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::payment::FulfillmentWitness;
        use crate::payment::PaymentCommand;

        let (mut state, bear) = bear_on_field();
        let p0 = PlayerId(0);
        let frame = frame_src(&state, bear);
        let pay_cost = Instruction::Act(Action::Pay(Cost(
            vec![CostComponent::do_action(Action::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(deckmaste_core::RefId(0)),
            ))]
            .into(),
        )));
        let may = May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(pay_cost),
            if_did: Some(Arc::new(Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(7)),
            )))),
            if_not: None,
        };
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(may), &frame);
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("expected optional payment, got {:?}", state.pending);
        };
        let iou = prompt.outstanding[0].id;
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Bound,
            }))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        assert!(
            !state.zones.battlefield.contains(&bear),
            "the sacrifice paid the cost"
        );
        assert_eq!(state.player(p0).life, life0, "if_did waits for submission");
        state
            .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
            .unwrap();
        assert!(
            matches!(
                state.agenda.front(),
                Some(WorkItem::RunEffect { effect, .. })
                    if matches!(effect.as_ref(), Instruction::Act { action: Action::ChangeLife(..), .. })
            ),
            "submission schedules if_did independently of payment events"
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 7,
            "if_did runs off submission, not a post-hoc events check"
        );
    }

    /// Payability is determined by the actual payment protocol, not a
    /// preliminary approximation. An unaffordable optional cost still opens;
    /// its invalid fulfillment is rejected atomically and decline runs
    /// `if_not`.
    #[test]
    fn run_effect_may_pay_unaffordable_offer_can_be_declined() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::payment::FulfillmentWitness;
        use crate::payment::PaymentCommand;

        let p0 = PlayerId(0);
        let mut state = game();
        // [CR#119.4,119.4b]: paying life needs life >= the amount — 1 life
        // can't cover a 5-life toll, so the "yes" choice is illegal.
        state.player_mut(p0).life = 1;
        let frame = frame_for(&state, p0);
        let pay_cost = Instruction::Act(Action::Pay(Cost(
            vec![CostComponent::do_action(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Down(Count::Literal(5)),
            ))]
            .into(),
        )));
        let may = May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(pay_cost),
            if_did: Some(Arc::new(Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(10)),
            )))),
            if_not: Some(Arc::new(Instruction::Act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(1)),
            )))),
        };
        let life0 = state.player(p0).life;
        state.run_effect(Instruction::May(may), &frame);
        let Some(DecisionPointKind::Payment(prompt)) = state.pending.as_ref() else {
            panic!("unaffordable cost should still open payment");
        };
        let iou = prompt.outstanding[0].id;
        let before = state.pending.clone();
        assert!(
            state
                .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                    iou,
                    witness: FulfillmentWitness::PayLife,
                }))
                .is_err()
        );
        assert_eq!(state.pending, before, "rejection is atomic");
        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 1,
            "decline runs if_not; if_did never runs"
        );
    }

    /// An announcement's cost block followed by the ability body it pays for,
    /// as one ordered run against a single activation — the shape
    /// `cast::cost_step_items` schedules, with the body's instructions after
    /// it ([CR#601.2b,601.2h]).
    fn announced_run(block: &[deckmaste_core::CostComponent], body: Instruction) -> Instruction {
        let mut instructions: Vec<Instruction> = block
            .iter()
            .map(|component| {
                crate::decide::unless_cost_effect(
                    component,
                    &deckmaste_core::Reference::controller_parameter(),
                )
            })
            .collect();
        instructions.push(body);
        Instruction::Sequentially(instructions.into())
    }

    /// "Sacrifice another creature", as the cost instruction the announcement
    /// runs: the paid product goes to `dest` for the body to read.
    fn sacrifice_this_for(dest: u32) -> deckmaste_core::CostComponent {
        deckmaste_core::CostComponent::producing(
            deckmaste_core::DefId(dest),
            Action::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(deckmaste_core::RefId(0)),
            ),
        )
    }

    /// "You gain life equal to the sacrificed creature's [counters]" — the
    /// ability body reading the paid product's register.
    fn gain_life_from_counters_of(register: u32) -> Instruction {
        Instruction::Act {
            dest: None,
            action: Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::CounterCount(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(register))),
                    "P1P1Counter".into(),
                )),
            ),
        }
    }

    /// [CR#118.8,601.2b]: an ability's cost block is PAID (the source is
    /// sacrificed) and the body then reads the paid object through the
    /// register the payment wrote — Ayli's "you gain life equal to the
    /// sacrificed creature's …". The bear is sacrificed carrying three +1/+1
    /// counters; the body gains life equal to the SACRIFICED creature's
    /// counter count, read via its last-known snapshot ([CR#603.10a]) —
    /// proving the paid product is bound for the body even after it has left
    /// the battlefield.
    ///
    /// Re-spelled from the deleted nested-`AdditionalCost` node: an additional
    /// cost is announced and paid with the mana or activation cost
    /// ([CR#118.8,118.8a]), never mid-resolution, so the same card and the same
    /// asserted outcome are pinned against the announcement block instead.
    #[test]
    fn a_cost_block_binds_its_paid_product_for_the_ability_body() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let p0 = PlayerId(0);
        let life0 = state.player(p0).life;
        let frame = frame_src(&state, bear);

        state.run_effect(
            announced_run(&[sacrifice_this_for(2)], gain_life_from_counters_of(2)),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);

        assert!(
            state.objects.get(bear).is_none(),
            "the additional cost was paid: the source is sacrificed (old id gone)"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "the sacrificed creature is in its owner's graveyard"
        );
        assert_eq!(
            state.player(p0).life,
            life0 + 3,
            "the body read the sacrificed object's counters through the paid product's register"
        );
    }

    /// [CR#608.2k]: one antecedent set per body — a stale event-role binding an
    /// ENCLOSING scope carried (a triggered ability's own firing-event patient)
    /// must never be mistaken for the payment's product.
    ///
    /// Re-spelled from the deleted nested-`AdditionalCost` node's frame-hygiene
    /// test. The old shape forked a body frame and cleared its event roles; the
    /// announcement shape has no fork at all — the paid product is a REGISTER,
    /// so an outer event role is structurally incapable of standing in for it.
    /// Same card, same asserted outcome: the body reads the sacrificed
    /// creature, not the stale patient.
    #[test]
    fn a_paid_product_is_read_by_register_not_a_stale_outer_event_role() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let p0 = PlayerId(0);
        let life0 = state.player(p0).life;
        let mut frame = frame_src(&state, bear);
        // An enclosing scope's own stale event-role binding — the shape a
        // triggered ability's frame carries when ITS firing event named a
        // patient (`resolve/mod.rs`'s trigger-frame construction). The patient
        // is a PLAYER, which carries no +1/+1 counters at all.
        state.frame_set_event_bindings(
            &mut frame,
            None,
            None,
            Some(crate::trigger::EventPatient::Player(PlayerId(1))),
        );

        // The paid product goes to register 7 — past this region's whole
        // parameter prefix, so it can only be the payment's own definition and
        // never an event role.
        state.run_effect(
            announced_run(&[sacrifice_this_for(7)], gain_life_from_counters_of(7)),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);

        assert_eq!(
            state.player(p0).life,
            life0 + 3,
            "the paid product's register holds the sacrificed creature, never the stale patient"
        );
        assert!(
            state
                .activation_product(frame.activation, deckmaste_core::RefId(3))
                .is_some(),
            "the outer event role is still there — it simply is not what the body reads"
        );
    }

    // --- Ascend (spell form) e2e ([CR#702.131a])
    // -------------------------------
    //
    // The spell form of Ascend folds into `Sequentially([If(<gate>,
    // GetDesignation), If(Is(You,Designated), Draw(3), otherwise: Draw(2))])`
    // (Task 7). The `Instruction::If` interpreter is now live (see the
    // `Instruction::If` arm in `run_effect` — it evaluates `condition_holds`,
    // then schedules `then`/`otherwise`), so these run unignored. They prove
    // the grant-then-read ordering ([CR#608.2c]): draws 3 at ten, 2 at nine,
    // and 2 at ten-then-nine (no high-water mark). The fixture is isolated by
    // `diag_setup_is_sound`, which proves the gate reads 10/9 correctly and a
    // bare `Draw(3)` lands three.

    /// The folded Ascend gate ([CR#702.131a]) built typed — the exact shape
    /// `deckmaste_migrations::resolve::fold_spell_ascend` prepends to a spell's
    /// effect: "ten battlefield permanents you control AND you don't already
    /// have the city's blessing".
    fn ascend_gate() -> deckmaste_core::Condition {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::RelationPredicate;

        Condition::And(
            vec![
                Condition::Compare(
                    Count::CountOf(Countable::Objects(Arc::new(
                        deckmaste_core::Region::candidate(Predicate::And(
                            vec![
                                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                                    Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                                ))),
                            ]
                            .into(),
                        )),
                    ))),
                    Cmp::AtLeast,
                    Count::Literal(10),
                ),
                Condition::Not(Arc::new(Condition::Matches(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
                ))),
            ]
            .into(),
        )
    }

    /// Secrets of the Golden City's resolved shape — the folded grant followed
    /// by the blessing-conditioned draw ("Ascend. Draw two cards. If you have
    /// the city's blessing, draw three instead."):
    ///
    /// ```text
    /// Sequentially([
    ///   If(gate, then: GetDesignation("CitysBlessing")),          // folded Ascend
    ///   If(Is(You, Designated), then: Draw(3), otherwise: Draw(2)),
    /// ])
    /// ```
    fn secrets_effect() -> Instruction {
        use deckmaste_core::Condition;
        use deckmaste_core::If;

        Instruction::Sequentially(
            vec![
                Instruction::If(If {
                    condition: ascend_gate(),
                    then: Arc::new(Instruction::Act(Action::GetDesignation(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        "CitysBlessing".into(),
                    ))),
                    otherwise: None,
                }),
                Instruction::If(If {
                    condition: Condition::Matches(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
                    ),
                    then: Arc::new(Instruction::draw(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(3),
                    )),
                    otherwise: Some(Arc::new(Instruction::draw(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(2),
                    ))),
                }),
            ]
            .into(),
        )
    }

    /// Builds a game where p0 controls `permanents` battlefield objects, has a
    /// fat library to draw from, an EMPTY hand (so the draw delta is the
    /// post-resolution hand size), and the synthetic Secrets-of-the-Golden-City
    /// spell on the stack (its first/only ability the `secrets` effect).
    /// Returns `(state, p0, library_before)`.
    fn secrets_on_stack(permanents: usize) -> (GameState, PlayerId, usize) {
        use deckmaste_card::CardFace;
        use deckmaste_core::Ability;
        use deckmaste_core::SpellAbility;

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
                name: format!("Permanent {i}").into(),
                types: vec![Type::Artifact.def()],
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
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: secrets_effect().into(),
            })],
            ..CardFace::default()
        });
        let spell_card_id = state.cards.push(Arc::new(spell_card), p0);
        let spell = state
            .objects
            .mint(ObjectSource::Card(spell_card_id), p0, Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: p0,
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            copy: false,
        });

        (state, p0, library_before)
    }

    /// Pump up to `n` steps, collecting every `(target, amount)` of the
    /// `DamageDealt` events applied along the way — the per-element emissions a
    /// `Each(.., DealDamage(This, .., It))` produces (a verb deals to a
    /// single `Reference`, so the spread is the iterator).
    #[allow(
        dead_code,
        reason = "shared fixture retained for neighboring effect cases"
    )]
    fn collect_damage_dealt(state: &mut GameState, n: usize) -> Vec<(ObjectId, u32)> {
        let mut got = Vec::new();
        for p in drain_progress(state, n) {
            if let Progress::Applied(occ) = p {
                let events = match occ {
                    Occurrence::Single(e) => vec![e],
                    Occurrence::Batch(es) => es,
                };
                for e in events {
                    if let GameEvent::DamageDealt(DamageDealt { target, amount, .. }) = e {
                        got.push((target, amount));
                    }
                }
            }
        }
        got
    }

    /// Isolation guard: proves the spell-form fixture is sound independent of
    /// the `Instruction::If` interpreter — the gate reads true at ten / false
    /// at nine for these minted battlefield objects, and a bare `Draw(3)`
    /// from the stocked library lands three cards in hand. So any failure
    /// of the three behavioral cases below points at the interpreter, not
    /// the fixture.
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
            Instruction::draw(Reference::Reg(deckmaste_core::RefId(1)), Count::Literal(3)),
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

        let spell = state.stack[0].id;
        state.agenda.push_front(WorkItem::Resolve(spell));
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

        let spell = state.stack[0].id;
        state.agenda.push_front(WorkItem::Resolve(spell));
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

        let spell = state.stack[0].id;
        state.agenda.push_front(WorkItem::Resolve(spell));
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

    /// `TopOfLibrary` returns the top N cards in order (front of library =
    /// top); `Instruction::With` binds them so `Selection::That` resolves to
    /// the same ordered vec inside the body frame.

    /// [CR#701.23a,701.23b]: a stated-quality search (Rampant Growth: "search
    /// your library for a basic land card, put it onto the battlefield
    /// tapped, then shuffle") end-to-end — the engine surfaces the hidden
    /// library as `ChooseObjects` candidates (a non-match never offered), the
    /// player finds the land, and the BODY (not the binder) moves/shuffles
    /// it. No reveal step in this shape, so no `Revealed` fact.

    /// [CR#701.23e]: reveal happens ONLY when the effect says to — a body
    /// that opens with `Reveal(That)` produces a `Revealed` fact naming the
    /// found card.

    /// [CR#701.23d]: a BARE quantity ("search your library for a card") — no
    /// stated quality — compels the find whenever the zone has one, unlike
    /// the stated-quality floor the test above pins.

    /// [CR#701.23d]: "or as many as possible" — a bare-quantity search over
    /// an EMPTY zone degrades to a legal zero-find, never an impossible
    /// decision, and the body (which shuffles) still runs.

    /// [CR#701.23b]: a STATED quality never compels a find — the player may
    /// decline even with a match sitting right there; the body still runs
    /// (and shuffles) on the decline.

    /// [CR#701.23b..701.23d]: `if_none` runs INSTEAD of the body on a failed
    /// find, with no `That` bound. The body's own Shuffle step never fires on
    /// this path, so a card whose printed "then shuffle" must survive a failed
    /// find has to spell that Shuffle inside `if_none` too. The shuffle is
    /// printed text, not a rules guarantee, so this is an authoring
    /// constraint rather than an engine gap; the corpus never populates
    /// `if_none` today.

    /// A plural `Search` ([CR#701.23]) binds the found set as a GROUP `That`
    /// — the body's `Each(Existing(They), …)` iterates every found card, not
    /// just one, proving the many-binder's group binding (not just
    /// `SearchOne`'s singular `That`) actually works.

    /// Mint a single card of type `ty` onto the (empty) top of `owner`'s
    /// library and return it. `game()` starts with empty libraries, so the
    /// lone `push_back` object is the top card.
    fn mint_library_top(state: &mut GameState, owner: PlayerId, name: &str, ty: Type) -> ObjectId {
        let cid = state.cards.push(
            Arc::new(Card::Normal(deckmaste_card::CardFace {
                name: name.into(),
                types: vec![ty.def()],
                ..deckmaste_card::CardFace::default()
            })),
            owner,
        );
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Library));
        state.zones.libraries[owner.index()].push_back(id);
        id
    }

    /// A bare creature permanent for `p0` to be the "exploring permanent".
    fn explorer_on_field(state: &mut GameState) -> ObjectId {
        mint_on_field(
            state,
            Card::Normal(deckmaste_card::CardFace {
                name: "Explorer".into(),
                types: vec![Type::Creature.def()],
                ..deckmaste_card::CardFace::default()
            }),
        )
    }

    /// Explore [CR#701.44a]: the controller reveals the top card of their
    /// library; a revealed LAND goes to hand — no +1/+1 counter, no
    /// may-to-graveyard. The reveal is public ([CR#701.20a]).
    #[test]
    fn explore_reveals_land_into_hand() {
        let mut state = game();
        let p0 = PlayerId(0);
        let source = explorer_on_field(&mut state);
        let land = mint_library_top(&mut state, p0, "Forest", Type::Land);

        // Parsed through the SEMANTICS path (`semantics::Instruction` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::OneShotEffect =
            builtin().macros.read_str("Explore").unwrap();
        let frame = frame_src(&state, source);
        schedule_lowered_effect(&mut state, semantic, 0, &frame);
        let _ = drain_progress(&mut state, 80);

        assert!(
            logged(&state, |e| matches!(e, GameEvent::Revealed(_))),
            "explore reveals the top card to all players ([CR#701.20a])"
        );
        // A zone change remints the object under a fresh id, so match the moved
        // card by identity, not the pre-move `land` id.
        assert!(
            state.zones.hands[p0.index()]
                .iter()
                .any(|&o| matches!(state.def(o), Card::Normal(f) if &*f.name == "Forest")),
            "a revealed land card is put into hand ([CR#701.44a])"
        );
        assert!(
            !state.zones.libraries[p0.index()].contains(&land),
            "the revealed land left the top of the library for the hand"
        );
        assert!(
            !logged(
                &state,
                |e| matches!(e, GameEvent::CounterPlaced(cp) if cp.object == source)
            ),
            "the land branch places no +1/+1 counter ([CR#701.44a])"
        );
    }

    /// Explore [CR#701.44a]: a revealed NON-land puts a +1/+1 counter on the
    /// exploring permanent and offers a may-put-into-graveyard. The revealed
    /// card stays on top until decided ([CR#701.20b] — revealing never moves
    /// it).
    #[test]
    fn explore_reveals_nonland_counter_then_may_graveyard() {
        let mut state = game();
        let p0 = PlayerId(0);
        let source = explorer_on_field(&mut state);
        let spell = mint_library_top(&mut state, p0, "Shock", Type::Instant);

        // Parsed through the SEMANTICS path (`semantics::Instruction` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::OneShotEffect =
            builtin().macros.read_str("Explore").unwrap();
        let frame = frame_src(&state, source);
        schedule_lowered_effect(&mut state, semantic, 0, &frame);
        // Drains through the reveal + counter and STOPS at the may-to-graveyard
        // decision ([CR#701.44a]).
        let _ = drain_progress(&mut state, 80);

        assert!(
            logged(&state, |e| matches!(e, GameEvent::Revealed(_))),
            "explore reveals the top card ([CR#701.20a])"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::CounterPlaced(cp) if cp.object == source)
            ),
            "a non-land puts a +1/+1 counter on the exploring permanent ([CR#701.44a])"
        );
        assert!(
            state.pending.is_some(),
            "and then offers a may-put-into-graveyard decision ([CR#701.44a])"
        );
        assert!(
            state.zones.libraries[p0.index()].contains(&spell),
            "the revealed non-land stays on top pending the may ([CR#701.20b])"
        );
    }

    /// `BottomOfLibrary` mirrors `TopOfLibrary` from the other end (bottom→up
    /// order), and `Union` concatenates member groups order-preserved with an
    /// object in more than one member appearing once (the Idris `Union` /
    /// `BottomOfLibrary` constructors).
    #[test]
    fn bottom_of_library_and_union_resolve_as_groups() {
        use deckmaste_card::CardFace;

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
        let frame = state.frame(source, p0);

        // BottomOfLibrary(count:2) → the bottom two, nearest-to-bottom first.
        let bottom2 = state.eval_selection_set(
            &Selection::BottomOfLibrary {
                count: Count::Literal(2),
                whose: deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
            },
            &frame,
        );
        assert_eq!(bottom2, vec![c, b], "bottom 2 are c then b, bottom→up");

        // Union of the top-2 and bottom-2 windows: b appears in both members
        // and is kept once, at its first position.
        let union = state.eval_selection_set(
            &Selection::Union(vec![
                Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                },
                Selection::BottomOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                },
            ]),
            &frame,
        );
        assert_eq!(union, vec![a, b, c], "order-preserving union, b deduped");
    }

    /// `TopOfGraveyard` reads the top `count` cards of a graveyard, top→down
    /// ([CR#404.2]): `zones.graveyards` is push-appended, so the LAST-pushed
    /// card (the most recent addition) is physically on top. A non-player
    /// `of` fizzles to the empty group — never a panic, unlike
    /// `TopOfLibrary`/`BottomOfLibrary`'s established (and here deliberately
    /// NOT repeated) panic baseline.
    #[test]
    fn top_of_graveyard_resolves_top_down_and_fizzles_on_bad_of() {
        use deckmaste_card::CardFace;

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
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);
        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Graveyard));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Graveyard));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Graveyard));
        // Put in bottom→top order: a first (bottom), c last (top).
        state.zones.graveyards[p0.index()].push(a);
        state.zones.graveyards[p0.index()].push(b);
        state.zones.graveyards[p0.index()].push(c);
        let source = state.player(p0).object;
        let frame = state.frame(source, p0);

        let top1 = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(1),
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
            },
            &frame,
        );
        assert_eq!(top1, vec![c], "the top card is the most recently put one");

        let top2 = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(2),
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
            },
            &frame,
        );
        assert_eq!(top2, vec![c, b], "top 2, top→down");

        // `of` resolving to a non-player (a card, not a player proxy) fizzles
        // to the empty group rather than panicking.
        let bad = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(1),
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
            },
            &state.frame(a, p0),
        );
        assert_eq!(
            bad,
            Vec::<ObjectId>::new(),
            "non-player `of` fizzles, never panics"
        );
    }

    // ---- Region-form fixtures for the restored iteration tests ----

    /// The loop element inside a [`loop_region`]/[`allot_region`] — the
    /// register the retired `Reference::It` named.
    const ELEMENT: deckmaste_core::RefId = deckmaste_core::RefId(0);
    /// The enclosing controller as seen from inside a [`loop_region`]: the
    /// element takes register 0, so the captured source and controller shift
    /// to 1 and 2.
    const LOOP_CONTROLLER: deckmaste_core::RefId = deckmaste_core::RefId(2);
    /// A distribution body's allotted share ([CR#601.2d]) — the register the
    /// retired `Count::Allotment` named.
    const SHARE: deckmaste_core::RefId = deckmaste_core::RefId(1);
    /// The first instruction definition of a `frame_src`-shaped activation:
    /// source(0), controller(1), the four event roles(2..=5), announced X(6).
    const FIRST_DEF: deckmaste_core::DefId = deckmaste_core::DefId(7);

    fn param(
        index: u32,
        kind: deckmaste_core::Kind,
        provenance: deckmaste_core::Provenance,
    ) -> deckmaste_core::Param {
        deckmaste_core::Param {
            def: deckmaste_core::DefId(index),
            kind,
            provenance,
        }
    }

    /// A loop body region: the element at parameter zero, then the enclosing
    /// source and controller — the prefix lowering's `in_child` builds for an
    /// `Each` body.
    fn loop_region(body: Instruction) -> deckmaste_core::Region {
        deckmaste_core::Region::new(
            Arc::from([
                param(
                    0,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::LoopElement,
                ),
                param(
                    1,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Source,
                ),
                param(
                    2,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Controller,
                ),
            ]),
            body.into(),
        )
    }

    /// A distribution body region: the recipient at parameter zero and its
    /// allotted share at parameter one ([CR#601.2d]).
    fn allot_region(body: Instruction) -> deckmaste_core::Region {
        deckmaste_core::Region::new(
            Arc::from([
                param(
                    0,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::LoopElement,
                ),
                param(
                    1,
                    deckmaste_core::Kind::Number,
                    deckmaste_core::Provenance::Allotment,
                ),
                param(
                    2,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Source,
                ),
                param(
                    3,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Controller,
                ),
            ]),
            body.into(),
        )
    }

    /// A per-candidate predicate region.
    fn candidate_region<T: deckmaste_core::CandidateBody>(
        body: T,
    ) -> Arc<deckmaste_core::Region<T>> {
        Arc::new(deckmaste_core::Region::candidate(body))
    }

    fn creatures_on_the_battlefield() -> Predicate {
        Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        )
    }

    /// A player-anchored frame with a STORED activation (unlike
    /// `frame_for`'s bare one), so a deciding instruction has a register file
    /// to write its product into.
    fn player_frame(state: &GameState, player: PlayerId) -> crate::stack::ExecutionFrame {
        frame_src(state, state.player(player).object)
    }

    /// [CR#611.2c]: a granted `Deontic` ("target creature can't block this
    /// turn") mints a static ROW, not a layer change — its subject register
    /// resolves to the locked object at mint and the row's own slot is
    /// rewritten to `Any`, so the identity lives in the scope rather than in
    /// the row. Re-spelled from
    /// `continuously_deontic_mints_locked_row_rewriting_subject_to_it`: the
    /// subject is an announced-target register, and the rewrite target is
    /// `Predicate::Any` now that `Reference::It` has left core.
    #[test]
    fn continuously_deontic_mints_locked_row_rewriting_subject_to_any() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::TurnMarker;

        const TARGET: deckmaste_core::RefId = deckmaste_core::RefId(6);

        let (mut state, src) = bear_on_field();
        // The bear is the lone announced target — the restriction's subject.
        let frame = frame_src_targets(&state, src, vec![src]);
        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::Deontic(Deontic::Cant(DeonticAction::Block {
                by: Predicate::Ref(Reference::Reg(TARGET)),
                on: Predicate::Any,
                count: None,
            }))),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1);
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Locked(ids) if ids == &vec![src]),
            "the announced-target subject locked to the bear at mint"
        );
        assert!(
            ce.changes.is_empty(),
            "a Deontic grant carries no layer changes"
        );
        assert!(
            matches!(
                &ce.rows[..],
                [StaticSpec::Deontic(Deontic::Cant(DeonticAction::Block { by, .. }))]
                    if *by == Predicate::Any
            ),
            "the subject slot is rewritten to Any — the scope carries the identity, got {:?}",
            ce.rows
        );
    }

    /// [CR#611.2]/[CR#611.2c]: `Continuously(Each(SelectAll(...), Modify(...)),
    /// UntilEndOfTurn)` — the resolve arm pushes one `ContinuousEffect` with a
    /// `ScopeResolved::Floating` scope and the right duration/changes.
    #[test]
    fn continuously_matching_registers_floating_scope() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);

        assert!(state.continuous.is_empty(), "no effects before resolve");

        let filter = candidate_region(Predicate::creature());
        let effect = Instruction::Continuously(Continuously {
            effect: Arc::new(StaticSpec::Each(
                Selection::SelectAll(Arc::clone(&filter)),
                Arc::new(deckmaste_core::Region::candidate(StaticSpec::Modify(
                    Reference::Reg(ELEMENT),
                    Modification::Power(NumericOp::Up(Count::Literal(1))),
                ))),
            )),
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
        assert_eq!(
            ce.changes,
            vec![Modification::Power(NumericOp::Up(Count::Literal(1)))]
        );
        assert!(!ce.is_cda);
    }

    /// [CR#601.2d,120.3]: dividing damage among a MIXED group — a creature AND a
    /// player (Arc Lightning's `CreatureOrPlayer`) — must not panic. The player
    /// element is a zoneless proxy with no LKI snapshot, so the loop's element
    /// register holds a player; the body reads it kind-poly: the creature takes
    /// its share as marked damage, the player loses life by its share.
    #[test]
    fn divide_among_handles_a_player_element_without_panicking() {
        use deckmaste_core::Distribute;

        let (mut state, creature) = bear_on_field();
        let player = state.players[1].object;
        let life0 = state.player(PlayerId(1)).life;
        // The group is pinned in a register (Arc Lightning's chosen
        // `CreatureOrPlayer` set); `split_evenly(3, 2)` is [2, 1], so the
        // first-listed creature takes 2, the player takes 1.
        let frame = frame_src(&state, creature);
        state.activation_write_objects(frame.activation, FIRST_DEF, &[creature, player]);
        let effect = Instruction::Distribute(Distribute {
            amount: Count::Literal(3),
            over: Selection::Reg(FIRST_DEF.into()),
            body: allot_region(Instruction::Act(Action::deal_damage(
                Reference::Reg(ELEMENT),
                Count::Reg(SHARE),
            ))),
        });
        state.run_effect(effect, &frame);
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(creature).total_damage(),
            2,
            "the creature took its 2-damage share as marked damage"
        );
        assert_eq!(
            state.player(PlayerId(1)).life,
            life0 - 1,
            "the player lost life equal to its 1-damage share"
        );
    }

    /// [CR#601.2d]: `Distribute` splits the amount across the group and supplies
    /// each element's share through the body region's allotment parameter —
    /// divided damage deals the split shares, summing to the total, ≥1 to each.
    #[test]
    fn divide_among_splits_amount_and_binds_allotment() {
        use deckmaste_core::Distribute;
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        let effect = Instruction::Distribute(Distribute {
            amount: Count::Literal(3),
            over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
            body: allot_region(Instruction::Act(Action::deal_damage(
                Reference::Reg(ELEMENT),
                Count::Reg(SHARE),
            ))),
        });
        state.run_effect(effect, &frame);
        run_injected(&mut state);
        let da = state.objects.obj(a).total_damage();
        let db = state.objects.obj(b).total_damage();
        assert_eq!(
            da + db,
            3,
            "the 3 damage was divided across the two creatures"
        );
        assert!(
            da >= 1 && db >= 1,
            "each creature got at least 1 ({da}, {db})"
        );
    }

    /// `Each(SelectAll(battlefield creature), DealDamage(<element>, 2))` deals
    /// 2 damage to each of the two battlefield creatures — one `DamageDealt`
    /// per iterated element (the verb's patient is a single `Reference`).
    #[test]
    fn each_creature_deal_damage_hits_both_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second creature onto the battlefield.
        let b = second_creature_on_field(&mut state);

        let frame = frame_src(&state, a);
        let effect = Instruction::Each(deckmaste_core::Each {
            over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
            body: loop_region(Instruction::Act(Action::deal_damage(
                Reference::Reg(ELEMENT),
                Count::Literal(2),
            ))),
        });
        state.run_effect(effect, &frame);

        // Simultaneity ([CR#700.1]): a single-verb `Each` body resolves for
        // every element at once — both `DamageDealt`s ride ONE
        // `Occurrence::Batch` (not two sequential singles), so death triggers /
        // SBAs see them together.
        match state.agenda.front() {
            Some(WorkItem::Emit(crate::event::Occurrence::Batch(evs))) => {
                assert_eq!(evs.len(), 2, "both hits land in one simultaneous batch");
            }
            other => panic!("expected one simultaneous Emit(Batch), got {other:?}"),
        }

        // Both creatures took 2 damage — one DamageDealt per iterated element.
        let mut got = collect_damage_dealt(&mut state, 40);
        got.sort();
        let mut want = vec![(a, 2u32), (b, 2u32)];
        want.sort();
        assert_eq!(got, want);
    }

    /// Force a second Grizzly Bears from P0's opening hand onto the
    /// battlefield.
    fn second_creature_on_field(state: &mut GameState) -> ObjectId {
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(state, o, &Predicate::r#type(Type::Creature)))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);
        b
    }

    /// A per-candidate filter region nested inside a loop body: the candidate
    /// at parameter zero, then the loop's own parameters as captures — the
    /// prefix lowering's `in_child` builds. The enclosing controller is
    /// therefore register 3.
    fn nested_filter_region(body: Predicate) -> Arc<deckmaste_core::Region<Predicate>> {
        Arc::new(deckmaste_core::Region::new(
            Arc::from([
                param(
                    0,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Candidate(deckmaste_core::Domain::Entity),
                ),
                param(
                    1,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Capture(deckmaste_core::RefId(0)),
                ),
                param(
                    2,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Source,
                ),
                param(
                    3,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Controller,
                ),
            ]),
            body,
        ))
    }

    /// The enclosing controller as seen from inside a [`nested_filter_region`].
    const FILTER_CONTROLLER: deckmaste_core::RefId = deckmaste_core::RefId(3);

    #[test]
    fn each_over_choice_bearing_body_schedules_its_work_items() {
        // Regression: the `Each` single-`Act` batch path must not silently
        // drop a choice-bearing body. Discard's carry future `Act` IS an
        // ordinary `Emit`, so both creatures' carry-Acts DO collapse into one
        // simultaneous `Emit(Batch)` — but each still recurses into its OWN
        // choose-then-discard `RunEffect` once that batch passes, so TWO
        // separate `ChooseObjects` decisions surface (one per
        // creature-triggered discard) and neither is dropped.
        // (Synthetic "for each creature, you discard a card" shape — chosen to
        // exercise the choice-bearing-body seam without standing up a
        // player-matching `over`.)
        // "You discard a card", spelled as the region form: the chooser is its
        // own instruction inside the keyword-action boundary, and the discard
        // verb reads the register it wrote ([CR#601.2b,701.9a]).
        const CHOSEN: deckmaste_core::DefId = deckmaste_core::DefId(3);

        let (mut state, a) = bear_on_field();
        let _b = second_creature_on_field(&mut state);
        let hand_before = state.zones.hands[0].len();
        let in_your_hand = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Hand)),
                Predicate::Relation(deckmaste_core::RelationPredicate::Owner(Arc::new(
                    Predicate::Ref(Reference::Reg(FILTER_CONTROLLER)),
                ))),
            ]
            .into(),
        );
        let discard_one = Instruction::Act(Action::Composite {
            name: deckmaste_core::VerbName::from("Discard"),
            body: Arc::new(Instruction::Sequentially(
                vec![
                    Instruction::Choose(deckmaste_core::Choose {
                        dest: CHOSEN,
                        by: Reference::Reg(LOOP_CONTROLLER),
                        quantity: deckmaste_core::Quantity::one(),
                        filter: nested_filter_region(in_your_hand),
                    }),
                    Instruction::Act(Action::discard_what(Reference::Reg(CHOSEN.into()))),
                ]
                .into(),
            )),
        });

        let frame = frame_src(&state, a);
        let effect = Instruction::Each(deckmaste_core::Each {
            over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
            body: loop_region(discard_one),
        });
        state.run_effect(effect, &frame);

        let mut decisions = 0;
        for _ in 0..60 {
            if state.zones.hands[0].len() + 2 <= hand_before {
                break;
            }
            if let crate::step::StepOutcome::NeedsDecision(
                crate::decide::DecisionPointKind::ChooseObjects(
                    crate::decide::pending::ChooseObjects {
                        candidates,
                        min,
                        max,
                        ..
                    },
                ),
            ) = state.step()
            {
                decisions += 1;
                assert_eq!((min, max), (1, 1), "one choice of one card each time");
                state
                    .submit_decision(crate::decide::Decision::Chosen(vec![candidates[0]]))
                    .unwrap();
            }
        }
        assert_eq!(
            decisions, 2,
            "each creature's discard surfaces its own choice — neither is dropped"
        );
        assert_eq!(
            state.zones.hands[0].len(),
            hand_before - 2,
            "both discards actually happened"
        );
    }

    /// A group chosen by a deciding instruction is iterated in FULL: the
    /// Brainstorm shape `Choose(2, …)` then `Each(<that register>,
    /// Destroy(<element>))` destroys BOTH — the dropped-cardinality bug (which
    /// acted on only the first) is unrepresentable now that the choice writes
    /// a group register and `Each` iterates the whole group ([CR#608.2]).
    #[test]
    fn each_over_choose_many_acts_on_all_elements() {
        use deckmaste_core::Quantity;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let frame = frame_src(&state, bear);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Choose(deckmaste_core::Choose {
                        dest: FIRST_DEF,
                        by: Reference::controller_parameter(),
                        quantity: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                        filter: candidate_region(creatures_on_the_battlefield()),
                    }),
                    Instruction::Each(deckmaste_core::Each {
                        over: Selection::Reg(FIRST_DEF.into()),
                        body: loop_region(Instruction::Act(Action::destroy(Reference::Reg(
                            ELEMENT,
                        )))),
                    }),
                ]
                .into(),
            ),
            &frame,
        );
        // The choice is made for the WHOLE group before iterating.
        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            min,
            max,
            candidates,
            ..
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!((min, max), (2, 2), "Choose(2) asks for exactly two");
        assert_eq!(
            candidates.len(),
            2,
            "both battlefield creatures are candidates"
        );
        state
            .submit_decision(Decision::Chosen(vec![bear, theirs]))
            .unwrap();
        for _ in 0..30 {
            if !state.zones.battlefield.contains(&bear)
                && !state.zones.battlefield.contains(&theirs)
            {
                break;
            }
            let _ = state.step();
        }
        assert!(
            !state.zones.battlefield.contains(&bear) && !state.zones.battlefield.contains(&theirs),
            "BOTH chosen creatures are destroyed — every element acted on, not just the first"
        );
    }

    /// `Each(SelectAll(Kind(Player)), DealDamage(<element>, 20))` deals 20
    /// damage to each of the two players. A verb's patient is a single
    /// `Reference`, so the spread over the player set is the enclosing `Each` —
    /// one `DamageDealt` per element ([CR#608.2,120.1]).
    #[test]
    fn each_player_deal_damage_hits_both_players() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);

        let effect = Instruction::Each(deckmaste_core::Each {
            over: Selection::SelectAll(candidate_region(Predicate::Entity(
                deckmaste_core::EntityClass::Player,
            ))),
            body: loop_region(Instruction::Act(Action::deal_damage(
                Reference::Reg(ELEMENT),
                Count::Literal(20),
            ))),
        });
        state.run_effect(effect, &frame);

        // Collect every DamageDealt across the per-element runs.
        let p0_obj = state.players[0].object;
        let p1_obj = state.players[1].object;
        let mut got = collect_damage_dealt(&mut state, 40);
        got.sort();
        let mut want = vec![(p0_obj, 20u32), (p1_obj, 20u32)];
        want.sort();
        assert_eq!(got, want, "each player takes 20 damage, one event apiece");
    }

    /// [CR#608.2,120.3]: `Each` over the players supplies each zoneless player
    /// proxy through the loop region's element parameter — the body's
    /// `DealDamage(<element>, 1)` resolves to the player and each loses 1 life,
    /// with no panic on the snapshotless element. Re-spelled from
    /// `foreach_over_players_binds_each_player_as_it`.
    #[test]
    fn foreach_over_players_binds_each_player_as_the_loop_element() {
        let (mut state, bear) = bear_on_field();
        let life0 = [
            state.player(PlayerId(0)).life,
            state.player(PlayerId(1)).life,
        ];
        let frame = frame_src(&state, bear);
        state.run_effect(
            Instruction::Each(deckmaste_core::Each {
                over: Selection::SelectAll(candidate_region(Predicate::Entity(
                    deckmaste_core::EntityClass::Player,
                ))),
                body: loop_region(Instruction::Act(Action::deal_damage(
                    Reference::Reg(ELEMENT),
                    Count::Literal(1),
                ))),
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(state.player(PlayerId(0)).life, life0[0] - 1);
        assert_eq!(state.player(PlayerId(1)).life, life0[1] - 1);
    }

    /// A nested `Each` inside a `Distribute` body reads the enclosing
    /// distribution's share through the CAPTURE its region declares
    /// ([CR#601.2d], ADR laws 6 and 7): the inner loop's own element parameter
    /// takes register 0 and the outer allotment rides a declared capture, so
    /// the share neither leaks by accident nor is silently cleared.
    ///
    /// Re-spelled from `nested_each_clears_outer_divide_among_allotment`,
    /// whose subject — a frame slot the inner loop had to CLEAR so a stale
    /// share could not be read — the region model retires: nothing crosses a
    /// region boundary except a declared parameter, so there is no slot to
    /// clear and the visible outer share is the one the body named.
    #[test]
    fn a_nested_loop_reads_the_outer_share_through_its_declared_capture() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        // The inner loop declares its own element at 0, then captures the
        // enclosing body's element(1) and allotment(2).
        let inner = deckmaste_core::Region::new(
            Arc::from([
                param(
                    0,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::LoopElement,
                ),
                param(
                    1,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Capture(deckmaste_core::RefId(0)),
                ),
                param(
                    2,
                    deckmaste_core::Kind::Number,
                    deckmaste_core::Provenance::Capture(deckmaste_core::RefId(1)),
                ),
            ]),
            Instruction::Act(Action::deal_damage(
                Reference::Reg(ELEMENT),
                Count::Reg(deckmaste_core::RefId(2)),
            ))
            .into(),
        );
        let effect = Instruction::Distribute(deckmaste_core::Distribute {
            amount: Count::Literal(2),
            over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
            body: allot_region(Instruction::Each(deckmaste_core::Each {
                over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
                body: inner,
            })),
        });
        state.run_effect(effect, &frame);
        run_injected(&mut state);
        // Two recipients × one share of 1 each × two inner elements = every
        // creature takes 1 from each of the two outer iterations.
        assert_eq!(
            state.objects.obj(a).total_damage(),
            2,
            "the inner loop read a live share, never a cleared or stale one"
        );
    }

    /// [CR#702.85,701.57] `RevealUntil` fizzles to a graceful no-op — the
    /// Reveal seam (`Action::Reveal`/`GameEvent::Revealed`) is
    /// genuinely unbuilt (see the doc comment on this arm in `run_effect`),
    /// so `body` never runs and nothing is scheduled — never a panic,
    /// matching the CRITICAL never-crash ruling.
    #[test]
    fn reveal_until_fizzles_to_a_no_op() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(&state, a);
        let life0 = state.player(PlayerId(0)).life;
        let agenda_before = state.agenda.len();

        let effect = Instruction::RevealUntil(deckmaste_core::RevealUntil {
            found: FIRST_DEF,
            passed: deckmaste_core::DefId(FIRST_DEF.0 + 1),
            whose: Reference::controller_parameter(),
            matches: candidate_region(Predicate::creature()),
            body: deckmaste_core::Region::closed(
                Instruction::Act(Action::ChangeLife(
                    Reference::controller_parameter(),
                    LifeOp::Up(Count::Literal(99)),
                ))
                .into(),
            ),
        });
        state.run_effect(effect, &frame);

        assert_eq!(
            state.agenda.len(),
            agenda_before,
            "RevealUntil schedules no ADDITIONAL work — the absent-subsystem no-op (compares \
             the delta, since a live game already has its own turn-structure agenda queued)"
        );
        let _ = drain_progress(&mut state, 5);
        assert_eq!(state.player(PlayerId(0)).life, life0, "body never runs");
    }

    /// [CR#608.2]: `Each` evaluates its selection once at resolution and runs
    /// the body once per matched object, supplying each iterated object as the
    /// loop region's element. Proven via `Destroy(<element>)` over the
    /// battlefield creatures: every creature dies, which can only happen if
    /// each iteration's element register holds that iteration's object.
    #[test]
    fn run_effect_foreach_binds_each_match_as_the_loop_element() {
        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let frame = frame_src(&state, bear);
        state.run_effect(
            Instruction::Each(deckmaste_core::Each {
                over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
                body: loop_region(Instruction::Act(Action::destroy(Reference::Reg(ELEMENT)))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 80);
        assert!(
            !state.zones.battlefield.contains(&bear) && !state.zones.battlefield.contains(&theirs),
            "every iterated creature is destroyed via its element register"
        );
    }

    /// [CR#608.2]: `Each` runs the body once per match — a non-binding body
    /// (gain 1 life) over two creatures gains 2 life.
    #[test]
    fn run_effect_foreach_runs_once_per_match() {
        let (mut state, bear) = bear_on_field();
        let _theirs = second_bear_to_player_1(&mut state);
        let frame = frame_src(&state, bear);
        let life0 = state.player(PlayerId(0)).life;
        state.run_effect(
            Instruction::Each(deckmaste_core::Each {
                over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
                body: loop_region(Instruction::Act(Action::ChangeLife(
                    Reference::Reg(LOOP_CONTROLLER),
                    LifeOp::Up(Count::Literal(1)),
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

    /// One library search instruction ([CR#701.23]) writing its found group to
    /// [`FIRST_DEF`].
    fn search_library(
        quantity: deckmaste_core::Quantity,
        filter: Predicate,
        if_none: deckmaste_core::Block,
    ) -> Instruction {
        Instruction::Search(deckmaste_core::Search {
            dest: FIRST_DEF,
            by: Reference::controller_parameter(),
            whose: Reference::controller_parameter(),
            from: vec![Zone::Library].into(),
            quantity,
            filter: candidate_region(filter),
            if_none,
        })
    }

    fn shuffle_your_library() -> Instruction {
        Instruction::Act(Action::Shuffle(Selection::LibraryOf(
            Reference::controller_parameter(),
        )))
    }

    /// [CR#701.23b..701.23d]: `if_none` is the WHIFF branch — it runs when the
    /// search finds nothing, and only then. Re-spelled from
    /// `search_if_none_runs_instead_of_body_on_a_failed_find`: the search
    /// instruction no longer carries a body (the clauses that read the found
    /// card are its siblings), so `if_none` replaces nothing but itself. That
    /// retires the old authoring constraint — a printed "then shuffle" now
    /// survives a failed find without being spelled twice.
    #[test]
    fn search_if_none_runs_only_on_a_failed_find() {
        use crate::decide::Decision;

        let whiff_then_shuffle = |library: &[(&str, Type)]| {
            let mut state = game();
            let p0 = PlayerId(0);
            for (name, ty) in library {
                mint_library_top(&mut state, p0, name, *ty);
            }
            let frame = player_frame(&state, p0);
            let life0 = state.player(p0).life;
            state.run_effect(
                Instruction::Sequentially(
                    vec![
                        search_library(
                            deckmaste_core::Quantity::one(),
                            Predicate::r#type(Type::Land),
                            Instruction::Act(Action::ChangeLife(
                                Reference::controller_parameter(),
                                LifeOp::Down(Count::Literal(1)),
                            ))
                            .into(),
                        ),
                        shuffle_your_library(),
                    ]
                    .into(),
                ),
                &frame,
            );
            drain_progress(&mut state, 20);
            let found: Vec<_> = match state.pending.clone() {
                Some(crate::decide::DecisionPointKind::ChooseObjects(
                    crate::decide::pending::ChooseObjects { candidates, .. },
                )) => candidates,
                other => panic!("expected ChooseObjects, got {other:?}"),
            };
            state
                .submit_decision(Decision::Chosen(found.clone()))
                .expect("the offered candidates are a legal answer");
            run_injected(&mut state);
            (life0 - state.player(p0).life, found.len(), state)
        };

        // No land in the library — the stated-quality filter can't match, so
        // the whiff branch runs.
        let (lost, found, state) = whiff_then_shuffle(&[("Bear", Type::Creature)]);
        assert_eq!(found, 0, "nothing matches the land filter");
        assert_eq!(lost, 1, "if_none ran on the failed find");
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Shuffled(p) if *p == PlayerId(0))
            ),
            "the following clause still runs — a search instruction has no body to replace"
        );

        // A land IS present and found — the whiff branch must NOT run.
        let (lost, found, _state) = whiff_then_shuffle(&[("Forest", Type::Land)]);
        assert_eq!(found, 1, "the land is found");
        assert_eq!(lost, 0, "if_none never runs on a successful find");
    }

    /// A plural `Search` ([CR#701.23]) writes the found set to its destination
    /// as a GROUP — a following `Each` over that register iterates every found
    /// card, not just one. Re-spelled from `search_many_binds_the_found_group_
    /// as_they`, whose `They` read is the register now.
    #[test]
    fn search_many_binds_the_found_group_in_its_register() {
        use deckmaste_core::Destination;
        use deckmaste_core::Quantity;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let a = mint_library_top(&mut state, p0, "Forest A", Type::Land);
        let b = mint_library_top(&mut state, p0, "Forest B", Type::Land);
        let bear = mint_library_top(&mut state, p0, "Bear", Type::Creature);
        let frame = player_frame(&state, p0);

        state.run_effect(
            Instruction::Sequentially(
                vec![
                    search_library(
                        Quantity::Range(Some(Count::Literal(0)), Some(Count::Literal(2))),
                        Predicate::r#type(Type::Land),
                        deckmaste_core::Block::default(),
                    ),
                    Instruction::Each(deckmaste_core::Each {
                        over: Selection::Reg(FIRST_DEF.into()),
                        body: loop_region(Instruction::Act(Action::Move(
                            Reference::Reg(ELEMENT),
                            Destination::Zone(Zone::Hand),
                            vec![].into(),
                            None,
                        ))),
                    }),
                    shuffle_your_library(),
                ]
                .into(),
            ),
            &frame,
        );

        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            candidates,
            min,
            max,
            ..
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        let mut got = candidates.clone();
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want, "the bear doesn't match; both lands do");
        assert_eq!(
            (min, max),
            (0, 2),
            "up to two, never compelled ([CR#701.23b])"
        );

        state
            .submit_decision(Decision::Chosen(vec![a, b]))
            .expect("both lands are legal finds");
        run_injected(&mut state);
        // A zone change remints a fresh id ([CR#400.7]), so the moved
        // objects aren't `a`/`b` themselves anymore — check counts and the
        // untouched non-match's identity instead.
        assert_eq!(
            state.zones.hands[p0.index()].len(),
            2,
            "the following Each iterated BOTH found cards — the register held the group"
        );
        assert_eq!(
            state.zones.libraries[p0.index()],
            vec![bear],
            "only the non-match is left in the library"
        );
    }

    /// [CR#701.23d]: a BARE quantity ("search your library for a card") — no
    /// stated quality — compels the find whenever the zone has one, unlike
    /// the stated-quality floor the tests above pin.
    #[test]
    fn search_one_bare_quantity_compels_a_find_when_present() {
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let card = mint_library_top(&mut state, p0, "Anything", Type::Creature);
        let frame = player_frame(&state, p0);

        state.run_effect(
            search_library(
                deckmaste_core::Quantity::one(),
                Predicate::Class(ObjectClass::Card),
                deckmaste_core::Block::default(),
            ),
            &frame,
        );

        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            candidates,
            min,
            max,
            ..
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(candidates, vec![card]);
        assert_eq!(
            (min, max),
            (1, 1),
            "a bare quantity compels the find ([CR#701.23d])"
        );
    }

    /// [CR#701.23d]: "or as many as possible" — a bare-quantity search over
    /// an EMPTY zone degrades to a legal zero-find, never an impossible
    /// decision, and the following clause (which shuffles) still runs.
    #[test]
    fn search_one_bare_quantity_finds_none_from_an_empty_library() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = player_frame(&state, p0);

        state.run_effect(
            Instruction::Sequentially(
                vec![
                    search_library(
                        deckmaste_core::Quantity::one(),
                        Predicate::Class(ObjectClass::Card),
                        deckmaste_core::Block::default(),
                    ),
                    shuffle_your_library(),
                ]
                .into(),
            ),
            &frame,
        );

        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            candidates,
            min,
            max,
            ..
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert!(candidates.is_empty());
        assert_eq!((min, max), (0, 0), "an empty zone can't force a find");

        state
            .submit_decision(Decision::Chosen(vec![]))
            .expect("zero is the only legal answer");
        run_injected(&mut state);
        assert!(
            logged(&state, |e| matches!(e, GameEvent::Shuffled(p) if *p == p0)),
            "the rest of the block still runs (and shuffles) on a failed find ([CR#608.2c])"
        );
    }

    /// [CR#701.23e]: reveal happens ONLY when the effect says to — a following
    /// `Reveal` instruction produces a `Revealed` fact naming the found card.
    #[test]
    fn search_one_reveal_step_in_body_reveals_the_found_card() {
        use deckmaste_core::Destination;

        use crate::decide::Decision;
        use crate::event::Revealed;

        let mut state = game();
        let p0 = PlayerId(0);
        let forest = mint_library_top(&mut state, p0, "Forest", Type::Land);
        let frame = player_frame(&state, p0);

        state.run_effect(
            Instruction::Sequentially(
                vec![
                    search_library(
                        deckmaste_core::Quantity::one(),
                        Predicate::r#type(Type::Land),
                        deckmaste_core::Block::default(),
                    ),
                    Instruction::Act(Action::Reveal {
                        what: Reference::Reg(FIRST_DEF.into()),
                        to: None,
                    }),
                    Instruction::Act(Action::Move(
                        Reference::Reg(FIRST_DEF.into()),
                        Destination::Zone(Zone::Hand),
                        vec![].into(),
                        None,
                    )),
                    shuffle_your_library(),
                ]
                .into(),
            ),
            &frame,
        );
        drain_progress(&mut state, 20);
        state
            .submit_decision(Decision::Chosen(vec![forest]))
            .expect("the land is a legal find");
        run_injected(&mut state);

        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::Revealed(Revealed { objects, .. }) if objects == &vec![forest]
            )),
            "the Reveal step ran and named the found card"
        );
    }

    /// [CR#701.23a,701.23b]: a stated-quality search (Rampant Growth: "search
    /// your library for a basic land card, put it onto the battlefield
    /// tapped, then shuffle") end-to-end — the engine surfaces the hidden
    /// library as `ChooseObjects` candidates (a non-match never offered), the
    /// player finds the land, and the FOLLOWING instructions (not the search)
    /// move/shuffle it. No reveal step in this shape, so no `Revealed` fact.
    #[test]
    fn search_one_semantic_land_tutor_finds_moves_and_shuffles() {
        use deckmaste_core::Destination;
        use deckmaste_core::EnterRider;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let forest = mint_library_top(&mut state, p0, "Forest", Type::Land);
        let bear = mint_library_top(&mut state, p0, "Bear", Type::Creature);
        let frame = player_frame(&state, p0);

        state.run_effect(
            Instruction::Sequentially(
                vec![
                    search_library(
                        deckmaste_core::Quantity::one(),
                        Predicate::r#type(Type::Land),
                        deckmaste_core::Block::default(),
                    ),
                    Instruction::Act(Action::Move(
                        Reference::Reg(FIRST_DEF.into()),
                        Destination::Zone(Zone::Battlefield),
                        vec![EnterRider::Tapped].into(),
                        None,
                    )),
                    shuffle_your_library(),
                ]
                .into(),
            ),
            &frame,
        );

        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            player,
            candidates,
            min,
            max,
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(player, p0);
        assert_eq!(
            candidates,
            vec![forest],
            "the bear doesn't match the land filter — only the land is offered"
        );
        assert_eq!(
            (min, max),
            (0, 1),
            "a STATED quality never compels a find ([CR#701.23b])"
        );

        state
            .submit_decision(Decision::Chosen(vec![forest]))
            .expect("the land is a legal find");
        run_injected(&mut state);

        // A zone change remints a fresh id ([CR#400.7]) — chase the move.
        let landed = state.chase_moved(forest);
        assert!(
            state.zones.battlefield.contains(&landed),
            "the found land landed on the battlefield"
        );
        assert!(
            state.objects.obj(landed).tapped,
            "the Tapped enter-rider applied"
        );
        assert!(
            state.zones.libraries[p0.index()].contains(&bear),
            "the non-match stayed in the library"
        );
        assert!(
            logged(&state, |e| matches!(e, GameEvent::Shuffled(p) if *p == p0)),
            "the following Shuffle step ran ([CR#701.24a])"
        );
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Revealed(_))),
            "no reveal step in this shape ([CR#701.23e]) — nothing revealed"
        );
    }

    /// [CR#701.23b]: a STATED quality never compels a find — the player may
    /// decline even with a match sitting right there; the rest of the block
    /// still runs (and shuffles) on the decline.
    #[test]
    fn search_one_stated_quality_may_decline_a_present_match() {
        use crate::decide::Decision;

        let mut state = game();
        let p0 = PlayerId(0);
        let forest = mint_library_top(&mut state, p0, "Forest", Type::Land);
        let frame = player_frame(&state, p0);

        state.run_effect(
            Instruction::Sequentially(
                vec![
                    search_library(
                        deckmaste_core::Quantity::one(),
                        Predicate::r#type(Type::Land),
                        deckmaste_core::Block::default(),
                    ),
                    shuffle_your_library(),
                ]
                .into(),
            ),
            &frame,
        );
        drain_progress(&mut state, 20);
        state
            .submit_decision(Decision::Chosen(vec![]))
            .expect("declining is legal even though the land matches");
        run_injected(&mut state);

        assert!(
            state.zones.libraries[p0.index()].contains(&forest),
            "the declined land stays in the library"
        );
        assert!(
            logged(&state, |e| matches!(e, GameEvent::Shuffled(p) if *p == p0)),
            "the block still shuffles even on a decline"
        );
    }

    /// `TopOfLibrary` returns the top N cards in order (front of library =
    /// top), and a `Let` pinning that selection makes the same ordered group
    /// readable by register ([CR#608.2h]). Re-spelled from
    /// `with_binds_those_and_top_of_library_is_ordered`: the `With` binder is
    /// a `Let` instruction and `Selection::They` is its register.
    #[test]
    fn a_let_pins_the_ordered_top_of_library_group() {
        use deckmaste_card::CardFace;

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
        let mint = |state: &mut GameState, name: &str| {
            let cid = state.cards.push(Arc::new(make_card(name)), p0);
            let id = state
                .objects
                .mint(ObjectSource::Card(cid), p0, Some(Zone::Library));
            state.zones.libraries[p0.index()].push_back(id);
            id
        };
        let a = mint(&mut state, "Alpha");
        let b = mint(&mut state, "Beta");
        let _c = mint(&mut state, "Gamma");

        let frame = player_frame(&state, p0);
        let top_two = Selection::TopOfLibrary {
            count: Count::Literal(2),
            whose: Reference::controller_parameter(),
        };

        // The selection itself reads top→down.
        assert_eq!(
            state.eval_selection_set(&top_two, &frame),
            vec![a, b],
            "top 2 are a then b, top→down"
        );

        // Pinned into a register, the same ordered group reads back by
        // register — the binding a later clause names.
        state.run_effect(
            Instruction::Let(deckmaste_core::Let {
                dest: FIRST_DEF,
                expr: deckmaste_core::Expr::Objects(top_two),
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.eval_selection_set(&Selection::Reg(FIRST_DEF.into()), &frame),
            vec![a, b],
            "the pinned register returns the bound group in order"
        );
    }

    /// [CR#616.1g,121.2a] Bruvac shape: a replacement that doubles a mill's
    /// aggregate count, layered over an ORIGINAL `Batch(3, mill one)` — mill 3
    /// becomes mill 6, via ONE replacement decision made against ONE aggregate
    /// window (never three independently-doubled per-card windows), and that
    /// decision is made BEFORE any card physically moves.
    #[test]
    fn bruvac_shape_batch_doubles_the_aggregate_and_no_card_moves_early() {
        // The replaced event's magnitude is the replacement region's
        // `EventAmount` parameter ([CR#107.3]).
        const EVENT_AMOUNT: deckmaste_core::RefId = deckmaste_core::RefId(6);

        let mut state = game();
        let p0 = PlayerId(0);
        for i in 0..8 {
            mint_in_library(&mut state, p0, &format!("Card {i}"));
        }

        // "If a player would mill one or more cards, that player mills
        // twice that many cards instead."
        let bruvac = deckmaste_core::Replacement::Instead {
            would: deckmaste_core::EventFilter::Act {
                verb: deckmaste_core::VerbName::from("Mill"),
                who: Predicate::Any,
                on: Predicate::Any,
                cause: None,
            },
            instead: Instruction::Batch(
                Count::Times(
                    Arc::new(Count::Literal(2)),
                    Arc::new(Count::Reg(EVENT_AMOUNT)),
                ),
                Arc::new(Instruction::Act(deckmaste_core::Action::mill_one(
                    Reference::controller_parameter(),
                ))),
            ),
        };
        mint_on_field(
            &mut state,
            Card::Normal(deckmaste_card::CardFace {
                name: "Bruvac Stand-In".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![deckmaste_core::Ability::r#static(
                    deckmaste_core::StaticSpec::Replacement(Arc::new(bruvac)),
                )],
                ..deckmaste_card::CardFace::default()
            }),
        );

        let frame = player_frame(&state, p0);
        let mill_one = Instruction::Act(deckmaste_core::Action::mill_one(
            Reference::controller_parameter(),
        ));
        state.run_effect(
            Instruction::Batch(Count::Literal(3), Arc::new(mill_one)),
            &frame,
        );

        // Pop exactly the aggregate's own `Emit` — `apply_occurrence` makes
        // the ENTIRE replacement decision synchronously within this one
        // `step()` (finding Bruvac, doubling 3 → 6, scheduling the
        // doubled aggregate's `RunEffect`) — but no contained per-card
        // future has run yet, so NO card has moved.
        let _ = state.step();
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            8,
            "the replacement decision is made before any card physically moves"
        );

        let _ = drain_progress(&mut state, 60);

        assert_eq!(
            state.zones.graveyards[p0.index()].len(),
            6,
            "mill 3 doubled to mill 6 — one replacement decision on the aggregate"
        );
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            2,
            "the other two cards stay in the library"
        );

        let mill_commits = state
            .history
            .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
            .filter(|e| {
                matches!(e, GameEvent::Act(Act { verb, committed: true, .. }) if verb.as_str() == "Mill")
            })
            .count();
        assert_eq!(
            mill_commits, 1,
            "exactly ONE committed Act(Mill) fact — the doubled aggregate's own; the \
             replaced-away mill-3 never independently finalizes ([CR#614.1]), and \
             none of the six contained per-card mills is independently trigger-visible"
        );
    }

    /// [CR#607.2a,608.2d] a CONSTRAINING quantity over a noted product group
    /// ("destroy one of them") surfaces a `ChooseObjects` chooser over the
    /// group's LIVE members, honors the quantity's bounds, and binds the picks
    /// for the following clause — exactly the chooser the unconstrained
    /// full-group read does not need.
    #[test]
    fn among_noted_constrained_quantity_surfaces_and_binds_chooser() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, a, b) = two_permanents_on_field();
        // The group this reads is "them" — the members a preceding clause
        // produced. Its core spelling is the successor's: a register holding
        // the group, read by the constraining chooser. Where the group has to
        // outlive the clause that produced it, the register is published as a
        // linked memory cell instead ([CR#607.1], ADR law 8) — covered by
        // `a_noted_product_group_can_be_acted_on`.
        //
        // "Destroy exactly one of them" — a constraining quantity over the
        // group.
        let frame = frame_src(&state, a);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Choose(deckmaste_core::Choose {
                        dest: FIRST_DEF,
                        by: Reference::controller_parameter(),
                        quantity: deckmaste_core::Quantity::Range(
                            Some(Count::Literal(1)),
                            Some(Count::Literal(1)),
                        ),
                        filter: candidate_region(creatures_on_the_battlefield()),
                    }),
                    Instruction::Each(deckmaste_core::Each {
                        over: Selection::Reg(FIRST_DEF.into()),
                        body: loop_region(Instruction::Act(Action::destroy(Reference::Reg(
                            ELEMENT,
                        )))),
                    }),
                ]
                .into(),
            ),
            &frame,
        );

        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            player,
            candidates,
            min,
            max,
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(0),
            "the controller chooses (a noted read names no other decider)"
        );
        assert_eq!((min, max), (1, 1), "exactly one, from the quantity bounds");
        let mut got = candidates.clone();
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want, "candidates = the noted group's live members");

        // Bind the pick and run the body: exactly `a` is destroyed.
        state
            .submit_decision(Decision::Chosen(vec![a]))
            .expect("a is a live member");
        run_injected(&mut state);
        assert!(
            !state.zones.battlefield.contains(&a),
            "the chosen member was destroyed"
        );
        assert!(
            state.zones.battlefield.contains(&b),
            "the un-chosen member is untouched"
        );
    }

    // ---- The named fixtures the discourse stage's ticket required ----

    /// FIXTURE — a `Let`-pinned "that many" is NOT re-evaluated after the game
    /// state changes ([CR#608.2h]). "Count the creatures, destroy one of them,
    /// then gain that much life": the count is pinned at its own program point,
    /// so the destroy that follows cannot shrink it.
    #[test]
    fn a_let_pinned_magnitude_is_not_re_evaluated_after_the_state_changes() {
        // The victim rides its own register so the frame's source survives the
        // destroy and the post-state count stays evaluable.
        const VICTIM: deckmaste_core::DefId = deckmaste_core::DefId(8);

        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        state.activation_write_object(frame.activation, VICTIM, b);
        let life0 = state.player(PlayerId(0)).life;
        let creature_count = Count::CountOf(Countable::Objects(candidate_region(
            creatures_on_the_battlefield(),
        )));
        assert_eq!(
            state.eval_count(&creature_count, &frame),
            2,
            "two creatures before the destroy"
        );

        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Let(deckmaste_core::Let {
                        dest: FIRST_DEF,
                        expr: deckmaste_core::Expr::Number(creature_count.clone()),
                    }),
                    Instruction::Act(Action::destroy(Reference::Reg(VICTIM.into()))),
                    Instruction::Act(Action::ChangeLife(
                        Reference::controller_parameter(),
                        LifeOp::Up(Count::Reg(FIRST_DEF.into())),
                    )),
                ]
                .into(),
            ),
            &frame,
        );
        run_injected(&mut state);
        let _ = drain_progress(&mut state, 40);

        assert!(
            !state.zones.battlefield.contains(&b),
            "the destroy happened between the pin and the read"
        );
        assert_eq!(
            state.eval_count(&creature_count, &frame),
            1,
            "one creature is left, so a re-evaluated count would read 1"
        );
        assert_eq!(
            state.player(PlayerId(0)).life,
            life0 + 2,
            "the gain reads the PINNED 2, not the post-destroy 1"
        );
        assert!(state.zones.battlefield.contains(&a));
    }

    /// FIXTURE — a chooser nested under another chooser resolves its OWN picks
    /// ([CR#608.2d], ADR law 3). Each decision writes its own destination
    /// register, so there is no shared slot for the inner pick to clobber and
    /// no clear to omit: the inner chooser's victim dies and the outer pick
    /// survives.
    #[test]
    fn a_chooser_nested_under_a_chooser_resolves_its_own_picks() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        // The loop body declares element(0), source(1), controller(2), so its
        // own first definition is register 3.
        const INNER_DEF: deckmaste_core::DefId = deckmaste_core::DefId(3);

        let (mut state, outer_pick) = bear_on_field();
        let inner_pick = second_bear_to_player_1(&mut state);

        let frame = frame_src(&state, outer_pick);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Choose(deckmaste_core::Choose {
                        dest: FIRST_DEF,
                        by: Reference::controller_parameter(),
                        quantity: deckmaste_core::Quantity::one(),
                        filter: candidate_region(creatures_on_the_battlefield()),
                    }),
                    Instruction::Each(deckmaste_core::Each {
                        over: Selection::Reg(FIRST_DEF.into()),
                        body: loop_region(Instruction::Sequentially(
                            vec![
                                Instruction::Choose(deckmaste_core::Choose {
                                    dest: INNER_DEF,
                                    by: Reference::Reg(LOOP_CONTROLLER),
                                    quantity: deckmaste_core::Quantity::one(),
                                    filter: candidate_region(creatures_on_the_battlefield()),
                                }),
                                Instruction::Act(Action::destroy(Reference::Reg(INNER_DEF.into()))),
                            ]
                            .into(),
                        )),
                    }),
                ]
                .into(),
            ),
            &frame,
        );

        let answer = |state: &mut GameState, pick| {
            drain_progress(state, 20);
            let Some(DecisionPointKind::ChooseObjects(_)) = state.pending.clone() else {
                panic!("expected ChooseObjects, got {:?}", state.pending);
            };
            state.submit_decision(Decision::Chosen(vec![pick])).unwrap();
        };
        answer(&mut state, outer_pick);
        answer(&mut state, inner_pick);
        let _ = drain_progress(&mut state, 40);

        assert!(
            !state.zones.battlefield.contains(&inner_pick),
            "the INNER chooser's own pick is what the verb destroyed"
        );
        assert!(
            state.zones.battlefield.contains(&outer_pick),
            "the outer pick was never clobbered into the verb"
        );
    }

    /// FIXTURE — an `Each` over players followed by "that much" reads the
    /// PER-ELEMENT amount ([CR#601.2d,608.2h]). Each iteration enters its own
    /// activation, so the magnitude one element pins can never leak into the
    /// next: two players on different life totals each gain their own.
    #[test]
    fn an_each_over_players_reads_the_per_element_amount() {
        use deckmaste_core::PlayerAttr;

        // The loop body declares element(0), source(1), controller(2), so its
        // own first definition is register 3.
        const PER_ELEMENT: deckmaste_core::DefId = deckmaste_core::DefId(3);

        let (mut state, src) = bear_on_field();
        state.player_mut(PlayerId(0)).life = 12;
        state.player_mut(PlayerId(1)).life = 20;
        let frame = frame_src(&state, src);

        state.run_effect(
            Instruction::Each(deckmaste_core::Each {
                over: Selection::SelectAll(candidate_region(Predicate::Entity(
                    deckmaste_core::EntityClass::Player,
                ))),
                body: loop_region(Instruction::Sequentially(
                    vec![
                        Instruction::Let(deckmaste_core::Let {
                            dest: PER_ELEMENT,
                            expr: deckmaste_core::Expr::Number(Count::PlayerStatOf(
                                Reference::Reg(ELEMENT),
                                PlayerAttr::Life,
                            )),
                        }),
                        Instruction::Act(Action::ChangeLife(
                            Reference::Reg(ELEMENT),
                            LifeOp::Up(Count::Reg(PER_ELEMENT.into())),
                        )),
                    ]
                    .into(),
                )),
            }),
            &frame,
        );
        run_injected(&mut state);
        let _ = drain_progress(&mut state, 60);

        assert_eq!(
            state.player(PlayerId(0)).life,
            24,
            "player 0 gained its OWN 12"
        );
        assert_eq!(
            state.player(PlayerId(1)).life,
            40,
            "player 1 gained its OWN 20 — not the previous element's 12"
        );
    }

    // ---- The named fixtures of `core-regions-captures-and-memory` ----
    // ADR laws 7 (captures) and 8 (linked memory).

    /// The event-role prefix a lowered TRIGGERED region declares, followed by
    /// its announced X: source(0), controller(1), event object(2), patient(3),
    /// actor(4), defending player(5), amount(6), X(7). A created body's
    /// captures follow it ([Core is explicit regions] law 2).
    fn created_body_params(
        captures: &[(deckmaste_core::RefId, deckmaste_core::Kind)],
    ) -> Arc<[deckmaste_core::Param]> {
        let mut params: Vec<deckmaste_core::Param> = deckmaste_core::event_region_params().to_vec();
        params.push(param(
            7,
            deckmaste_core::Kind::Number,
            deckmaste_core::Provenance::AnnouncedX,
        ));
        for (index, (outer, kind)) in captures.iter().enumerate() {
            params.push(param(
                8 + u32::try_from(index).expect("fixture capture count fits u32"),
                *kind,
                deckmaste_core::Provenance::Capture(*outer),
            ));
        }
        params.into()
    }

    /// The first instruction definition of a `frame_src_targets` activation
    /// carrying ONE announced target: source(0), controller(1), the four event
    /// roles(2..=5), the target(6), announced X(7).
    const FIRST_DEF_ONE_TARGET: deckmaste_core::DefId = deckmaste_core::DefId(8);
    /// That activation's announced-target register.
    const TARGET_0: deckmaste_core::RefId = deckmaste_core::RefId(6);

    fn next_end_step() -> deckmaste_core::EventFilter {
        deckmaste_core::EventFilter::StepBegins {
            at: deckmaste_core::PhaseStep::Ending(deckmaste_core::EndingStep::End),
            whose: deckmaste_core::WhoseTurn::EachPlayers,
        }
    }

    fn delayed(params: Arc<[deckmaste_core::Param]>, body: Instruction) -> Instruction {
        Instruction::Delayed(Arc::new(deckmaste_core::TriggeredAbility {
            ability_word: None,
            event: next_end_step(),
            from: None,
            condition: None,
            limits: Vec::new().into(),
            where_x: None,
            targets: Vec::new().into(),
            effect: deckmaste_core::Region::new(params, body.into()),
        }))
    }

    /// Fire the delayed registry on the end-step onset and run the trigger all
    /// the way through placement to resolution.
    fn fire_end_step(state: &mut GameState) {
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            deckmaste_core::PhaseStep::Ending(deckmaste_core::EndingStep::End),
        )));
        run_injected(state);
        drain_passing_priority(state, 400);
    }

    /// FIXTURE — an exile-then-return-at-end-step card ("exile another target
    /// permanent; return that card to the battlefield at the beginning of the
    /// next end step"). The delayed trigger CAPTURES the exiled object, and
    /// the object it captures is the card in exile — the new object the move
    /// produced ([CR#400.7]), chased once at capture time because the creating
    /// effect is what moved it ([CR#400.7j]). The capture is the ONLY channel:
    /// the delayed body has no activation of its own to read the creating
    /// region's registers from.
    #[test]
    fn a_delayed_trigger_captures_the_exiled_object_and_returns_it() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::producing(
                        FIRST_DEF_ONE_TARGET,
                        Action::Move(
                            Reference::Reg(TARGET_0),
                            deckmaste_core::Destination::Zone(Zone::Exile),
                            vec![].into(),
                            None,
                        ),
                    ),
                    delayed(
                        created_body_params(&[(
                            FIRST_DEF_ONE_TARGET.into(),
                            deckmaste_core::Kind::Entities,
                        )]),
                        Instruction::Act(Action::Move(
                            Reference::Reg(deckmaste_core::RefId(8)),
                            deckmaste_core::Destination::Zone(Zone::Battlefield),
                            vec![].into(),
                            None,
                        )),
                    ),
                ]
                .into(),
            ),
            &frame_src_targets(&state, a, vec![b]),
        );
        let _ = frame;
        run_injected(&mut state);

        assert!(
            !state.zones.battlefield.contains(&b),
            "the targeted permanent was exiled"
        );
        assert_eq!(state.zones.exile.len(), 1, "exactly one card in exile");
        let exiled = state.zones.exile[0];
        assert_ne!(
            exiled, b,
            "[CR#400.7]: the card in exile is a NEW object, not the permanent that left"
        );
        let registered = state
            .delayed_triggers
            .first()
            .expect("[CR#603.7b]: the delayed trigger registered for the end step");
        assert_eq!(
            registered.bindings.captures.len(),
            1,
            "the created body's one declared capture is supplied at creation"
        );

        fire_end_step(&mut state);
        assert!(
            state.zones.exile.is_empty(),
            "the captured card left exile when the delayed trigger resolved"
        );
        assert_eq!(
            state.zones.battlefield.len(),
            2,
            "the exiled card returned to the battlefield alongside the untouched source"
        );
    }

    /// [CR#603.7c]: "if that object is no longer in the zone it's expected to
    /// be in at the time the delayed triggered ability resolves, the ability
    /// won't affect it." A capture is a SNAPSHOT, never a chase: once the
    /// captured card leaves exile on its own, the delayed body finds nothing
    /// and returns nothing — it does not follow the card to its successor.
    #[test]
    fn a_capture_is_not_chased_after_the_object_leaves_the_zone_it_was_captured_in() {
        let (mut state, a, b) = two_permanents_on_field();
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::producing(
                        FIRST_DEF_ONE_TARGET,
                        Action::Move(
                            Reference::Reg(TARGET_0),
                            deckmaste_core::Destination::Zone(Zone::Exile),
                            vec![].into(),
                            None,
                        ),
                    ),
                    delayed(
                        created_body_params(&[(
                            FIRST_DEF_ONE_TARGET.into(),
                            deckmaste_core::Kind::Entities,
                        )]),
                        Instruction::Act(Action::Move(
                            Reference::Reg(deckmaste_core::RefId(8)),
                            deckmaste_core::Destination::Zone(Zone::Battlefield),
                            vec![].into(),
                            None,
                        )),
                    ),
                ]
                .into(),
            ),
            &frame_src_targets(&state, a, vec![b]),
        );
        run_injected(&mut state);
        let exiled = state.zones.exile[0];

        // A later, unrelated effect moves the captured card out of exile. The
        // object it becomes in the graveyard is a new object ([CR#400.7]).
        let frame = frame_src(&state, a);
        state.run_effect(
            Instruction::Act(Action::Move(
                Reference::Reg(TARGET_0),
                deckmaste_core::Destination::Zone(Zone::Graveyard),
                vec![].into(),
                None,
            )),
            &frame_src_targets(&state, a, vec![exiled]),
        );
        let _ = frame;
        run_injected(&mut state);
        assert!(state.zones.exile.is_empty(), "the card left exile");
        let before = state.zones.battlefield.len();

        fire_end_step(&mut state);
        assert_eq!(
            state.zones.battlefield.len(),
            before,
            "[CR#603.7c]: the captured object is gone from the zone it was captured in, \
             so the delayed body affects nothing — it does not chase the successor"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "the successor object stayed where it was"
        );
    }

    /// FIXTURE — a carried body with a CAPTURE LIST. A delayed trigger created
    /// inside an `Each` body captures that body's own per-element register:
    /// the loop element is region-local to the loop body, so the created body
    /// can only see it by declaring it ([Core is explicit regions] law 7 —
    /// nothing else crosses a region boundary). Each iteration's capture is
    /// its own snapshot, so two elements produce two triggers that name two
    /// different objects.
    #[test]
    fn a_carried_body_captures_its_enclosing_loop_element_per_iteration() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        // The loop body's registers: element(0), source(1), controller(2).
        state.run_effect(
            Instruction::Each(deckmaste_core::Each {
                over: Selection::SelectAll(candidate_region(creatures_on_the_battlefield())),
                body: deckmaste_core::Region::new(
                    Arc::from([
                        param(
                            0,
                            deckmaste_core::Kind::Entity,
                            deckmaste_core::Provenance::LoopElement,
                        ),
                        param(
                            1,
                            deckmaste_core::Kind::Entity,
                            deckmaste_core::Provenance::Source,
                        ),
                        param(
                            2,
                            deckmaste_core::Kind::Entity,
                            deckmaste_core::Provenance::Controller,
                        ),
                    ]),
                    delayed(
                        created_body_params(&[(ELEMENT, deckmaste_core::Kind::Entity)]),
                        Instruction::Act(Action::destroy(Reference::Reg(deckmaste_core::RefId(8)))),
                    )
                    .into(),
                ),
            }),
            &frame,
        );
        run_injected(&mut state);

        let captured: Vec<ObjectId> = state
            .delayed_triggers
            .iter()
            .filter_map(|trigger| {
                trigger
                    .bindings
                    .captures
                    .first()
                    .and_then(|(_, value)| value.captured_object())
            })
            .collect();
        let mut got = captured.clone();
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(
            got, want,
            "each iteration's created body captured ITS OWN element, not the last one"
        );

        fire_end_step(&mut state);
        assert!(
            state.zones.battlefield.is_empty(),
            "both delayed bodies acted on the object they captured"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            2,
            "each created body moved ITS OWN captured element"
        );
    }

    /// FIXTURE — a linked pair through a DECLARED CELL ([CR#607.1], law 8):
    /// one ability exiles a card and REMEMBERS it, a second ability of the
    /// same object acts on "the exiled card" by declaring that cell as a
    /// `Provenance::Linked` parameter. The two abilities never share an
    /// activation; the cell is the whole channel.
    #[test]
    fn a_linked_pair_acts_on_the_exiled_card_through_a_declared_cell() {
        let (mut state, a, b) = two_permanents_on_field();
        // Ability one: "Exile target creature. (Remember it as `exiled`.)"
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::producing(
                        FIRST_DEF_ONE_TARGET,
                        Action::Move(
                            Reference::Reg(TARGET_0),
                            deckmaste_core::Destination::Zone(Zone::Exile),
                            vec![].into(),
                            None,
                        ),
                    ),
                    Instruction::Remember(deckmaste_core::Remember {
                        cell: deckmaste_core::Ident::from("exiled"),
                        kind: deckmaste_core::Kind::Entities,
                        value: FIRST_DEF_ONE_TARGET.into(),
                    }),
                ]
                .into(),
            ),
            &frame_src_targets(&state, a, vec![b]),
        );
        run_injected(&mut state);
        assert_eq!(state.zones.exile.len(), 1, "ability one exiled the card");

        // Ability two: "Put the card exiled with this permanent onto the
        // battlefield." Its own region — a fresh activation with no relation
        // to ability one's — declares the cell.
        let reader: deckmaste_core::Region = deckmaste_core::Region::new(
            Arc::from([
                param(
                    0,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Source,
                ),
                param(
                    1,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Controller,
                ),
                param(
                    2,
                    deckmaste_core::Kind::Entities,
                    deckmaste_core::Provenance::Linked(deckmaste_core::Ident::from("exiled")),
                ),
            ]),
            Instruction::Act(Action::Move(
                Reference::Reg(deckmaste_core::RefId(2)),
                deckmaste_core::Destination::Zone(Zone::Battlefield),
                vec![].into(),
                None,
            ))
            .into(),
        );
        let mut reading = state.frame(a, PlayerId(0));
        reading.activation = state.enter_region(&reader, &reading);
        state.run_effect(Instruction::Sequentially(reader.body.0.clone()), &reading);
        run_injected(&mut state);

        assert!(
            state.zones.exile.is_empty(),
            "[CR#607.2a]: the second ability found the card the first one exiled"
        );
        assert_eq!(
            state.zones.battlefield.len(),
            2,
            "the exiled card came back alongside the untouched source"
        );
    }

    /// [CR#607.1]: a cell is per-OBJECT. A different object's ability reading
    /// the same cell name finds nothing — the two abilities are not linked.
    #[test]
    fn a_linked_cell_is_not_readable_from_another_object() {
        let (mut state, a, b) = two_permanents_on_field();
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::producing(
                        FIRST_DEF_ONE_TARGET,
                        Action::Move(
                            Reference::Reg(TARGET_0),
                            deckmaste_core::Destination::Zone(Zone::Exile),
                            vec![].into(),
                            None,
                        ),
                    ),
                    Instruction::Remember(deckmaste_core::Remember {
                        cell: deckmaste_core::Ident::from("exiled"),
                        kind: deckmaste_core::Kind::Entities,
                        value: FIRST_DEF_ONE_TARGET.into(),
                    }),
                ]
                .into(),
            ),
            &frame_src_targets(&state, a, vec![b]),
        );
        run_injected(&mut state);
        let exiled = state.zones.exile.clone();
        assert_eq!(exiled.len(), 1);

        // The SAME cell name, read by an ability whose source is the OTHER
        // object (player 1's proxy stands in for a second permanent).
        let other = state.player(PlayerId(1)).object;
        let reader: deckmaste_core::Region = deckmaste_core::Region::new(
            Arc::from([
                param(
                    0,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Source,
                ),
                param(
                    1,
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::Controller,
                ),
                param(
                    2,
                    deckmaste_core::Kind::Entities,
                    deckmaste_core::Provenance::Linked(deckmaste_core::Ident::from("exiled")),
                ),
            ]),
            Instruction::Act(Action::Move(
                Reference::Reg(deckmaste_core::RefId(2)),
                deckmaste_core::Destination::Zone(Zone::Battlefield),
                vec![].into(),
                None,
            ))
            .into(),
        );
        let mut reading = state.frame(other, PlayerId(1));
        reading.activation = state.enter_region(&reader, &reading);
        state.run_effect(Instruction::Sequentially(reader.body.0.clone()), &reading);
        run_injected(&mut state);

        assert_eq!(
            state.zones.exile, exiled,
            "[CR#607.1]: an unlinked ability reads no cell of another object"
        );
    }
}
