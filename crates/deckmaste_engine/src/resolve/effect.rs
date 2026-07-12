//! `run_effect`: walk a `OneShotEffect` AST — combinators, binders, riders,
//! continuous-effect minting — lowering each node to agenda work.

use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Destination;
use deckmaste_core::Modification;
use deckmaste_core::Normalize;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::StaticEffect;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::deref_quantity;
use super::occurrence_of;
use super::peel_binder;
use super::peel_effect;
use super::top_targets;
use crate::agenda::WorkItem;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::layer::ContinuousEffect;
use crate::layer::ScopeResolved;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::Frame;
use crate::state::GameState;

impl GameState {
    /// [CR#614.3]: register a floating replacement shield (regeneration, "the
    /// next time …") on `state.shields`. Mutates `&mut self`, so it can't ride
    /// `action_items` (`&self`); the `Action::CreateReplacement` arm of
    /// `run_effect` routes here.
    fn create_shield(
        &mut self,
        replacement: deckmaste_core::Replacement,
        subject: &deckmaste_core::Reference,
        duration: deckmaste_core::Duration,
        one_shot: bool,
        frame: &Frame,
    ) {
        // Same canonical guard the continuous-effect mint uses ([CR#611.2,614.3]):
        // a shield may carry only a SWEEPABLE duration. `ForThisEvent` is the
        // one exception — an instruction-scoped rider, never a stored shield;
        // stay LOUD rather than register a silently-forever shield.
        assert!(
            crate::state::duration_sweepable(&duration),
            "create_shield: non-sweepable duration {duration:?} — a ForThisEvent \
             shield would last forever (rider durations never mint instances)"
        );
        let id = self.eval_reference(subject, frame);
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

    /// Resolve a granted `Deontic`'s SUBJECT `Reference`s at mint ([CR#611.2c]
    /// object lock) — the object set a resolved one-shot restriction affects is
    /// fixed at creation. Returns the resolved ids (for
    /// `ScopeResolved::Locked`) and a rewritten `Deontic` whose subject
    /// slot(s) now read `Ref(It)`, so a consumer interprets `It` as the
    /// locked scope members. A subject that is not a bare object reference
    /// (a static filter like "creatures you control") locks nothing and is
    /// left as authored — the row still exists, evaluated against live
    /// objects by its (still-LOUD) reader.
    fn lock_deontic_subject(&self, deontic: &Deontic, frame: &Frame) -> (Vec<ObjectId>, Deontic) {
        let mut locked = deontic.clone();
        let mut ids = Vec::new();
        if let Some(action) = deontic_action_mut(&mut locked) {
            for slot in deontic_subject_slots(action) {
                if let Predicate::Ref(r) = slot {
                    ids.push(self.eval_reference(r, frame));
                    *slot = Predicate::Ref(Reference::It);
                }
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
    fn resolve_no_regen_riders(&self, parts: &[StaticEffect], frame: &Frame) -> Vec<ObjectId> {
        let mut ids = Vec::new();
        for part in parts {
            let StaticEffect::Deontic(Deontic::Cant(action)) = part else {
                todo!(
                    "P0.W1: ForThisEvent rider {part:?} — only Cant(Regenerate) is wired [CR#701.19c]"
                );
            };
            let DeonticAction::Regenerate { on, .. } = action else {
                todo!(
                    "P0.W1: ForThisEvent Cant rider {action:?} — only Regenerate is wired [CR#701.19c]"
                );
            };
            match on {
                Predicate::Ref(r) => ids.push(self.eval_reference(r, frame)),
                other => todo!(
                    "P0.W1: Cant(Regenerate(on: {other:?})) — only a bare object Ref is wired"
                ),
            }
        }
        ids
    }

    /// Resolve a [`Binder`](deckmaste_core::Binder) to its bound group of
    /// element ids ([CR#608.2]) — the shared spine of `With`/`Each`/
    /// `Distribute`. `TheRef` is a singleton, `Existing` evaluates its
    /// `Selection`, and a chooser (`ChooseOne`/`Choose`) reads the picks that a
    /// prior [`Self::binder_choice`] surfaced into `frame.anaphora.chosen`.
    fn resolve_binder(&self, binder: &deckmaste_core::Binder, frame: &Frame) -> Vec<ObjectId> {
        use deckmaste_core::Binder;
        match peel_binder(binder) {
            Binder::TheRef(reference) => vec![self.eval_reference(reference, frame)],
            Binder::Existing(selection) => self.eval_selection_set(selection, frame),
            Binder::ChooseOne { .. } | Binder::Choose { .. } => frame
                .anaphora
                .chosen
                .clone()
                .expect("a chooser binder re-runs with its picks bound"),
            // SEAM: `OneShotEffect::With` special-cases `Binder::Produce` directly
            // (before this spine runs) — it needs a mutating run-action-and-
            // capture step this read-only (`&self`) resolver can't provide.
            // Unreachable from `With`; still hit if `Each`/`Distribute` ever
            // took a `Produce` binder (they don't — `Produce`/`SearchOne` are
            // One-binders, not the many-binder those expect).
            Binder::Produce(_) => unimplemented!(
                "Binder::Produce: resolved outside OneShotEffect::With, which is the only binder \
                 consumer wired to run a producer's Action and capture its product"
            ),
            // SEAM: search/tutor binders ([CR#701.23a]). Selecting from hidden
            // zones (library/graveyard) with the reveal + shuffle (and
            // fail-to-find) discipline has no runtime primitive; resolving
            // these via the plain `ChooseObjects` chooser would silently skip
            // reveal/shuffle, so this stays an explicit labeled seam.
            Binder::Search { .. } | Binder::SearchOne { .. } => unimplemented!(
                "Binder::Search/SearchOne: no library-search/tutor primitive — searching hidden \
                 zones (reveal + shuffle) is not yet wired"
            ),
            Binder::Expanded(_) => unreachable!("peeled above"),
        }
    }

    /// A many-binder's [`Cardinality`](crate::stack::Cardinality): `One` for a
    /// one-binder (`TheRef`/`ChooseOne`), `Many` for a many-binder
    /// (`Choose`/`Existing`). The bound-group kind comes from the resolved ids.
    fn binder_cardinality(binder: &deckmaste_core::Binder) -> crate::stack::Cardinality {
        use deckmaste_core::Binder;

        use crate::stack::Cardinality;
        match peel_binder(binder) {
            // `Produce`/`SearchOne` are One-binders (Idris `Bindable b One …`);
            // `Search` is Many. This cardinality is a pure structural fact from
            // the Idris constructors, valid independent of the resolution seam.
            Binder::TheRef(_)
            | Binder::ChooseOne { .. }
            | Binder::Produce(_)
            | Binder::SearchOne { .. } => Cardinality::One,
            Binder::Choose { .. } | Binder::Existing(_) | Binder::Search { .. } => {
                Cardinality::Many
            }
            Binder::Expanded(_) => unreachable!("peeled above"),
        }
    }

    /// If `binder` is a chooser (`ChooseOne`/`Choose`) whose pick has not yet
    /// been made, the `(chooser, candidates, min, max)` to surface as a
    /// `ChooseObjects` decision ([CR#601.2d]); else `None` (a
    /// `TheRef`/`Existing` binder, or a chooser already resolved into
    /// `frame.anaphora.chosen`). The chooser is the binder's `by` resolved
    /// to a player ([CR#608.2d] — default `You` = the controller; "that
    /// player sacrifices a creature of their choice" routes to the foreign
    /// actor). Shared by `With`/`Each`/`Distribute` so all three iterate a
    /// player-chosen group identically.
    fn binder_choice(
        &self,
        binder: &deckmaste_core::Binder,
        frame: &Frame,
    ) -> Option<(crate::player::PlayerId, Vec<ObjectId>, Uint, Uint)> {
        use deckmaste_core::Binder;
        if frame.anaphora.chosen.is_some() {
            return None;
        }
        match peel_binder(binder) {
            Binder::ChooseOne { filter, by } => Some((
                self.acting_player(by, frame),
                crate::target::candidates(self, filter),
                1,
                1,
            )),
            Binder::Choose {
                quantity,
                filter,
                by,
            } => {
                let candidates = crate::target::candidates(self, filter);
                let (min, max) = self.choice_bounds(quantity, candidates.len(), frame);
                Some((self.acting_player(by, frame), candidates, min, max))
            }
            // [CR#607.2a,608.2d]: a CONSTRAINING `AmongNoted` inside `Existing`
            // ("exile two of THEM") is a chooser over the noted group's LIVE
            // members — surface the same `ChooseObjects` a `Choose` binder does,
            // re-running the binder with the picks in `chosen`. The
            // UNCONSTRAINED group (the whole set) is not a choice, so
            // `among_noted_choice` returns `None` and it flows through
            // `resolve_binder`→`eval_selection_set` unchanged. `AmongNoted`
            // carries no `by`, so the controller chooses ([CR#608.2d] default).
            Binder::Existing(selection) => {
                let (label, quantity) = among_noted_choice(selection)?;
                let live = self.live_noted_members(label);
                let (min, max) = self.choice_bounds(quantity, live.len(), frame);
                Some((frame.controller, live, min, max))
            }
            _ => None,
        }
    }

    /// The LIVE members of the fact-backed `noted` product group under `label`
    /// ([CR#607.2a]): each read through its post-move identity, dropping any
    /// that has since left play. Shared by `Selection::AmongNoted` and the
    /// constrained-`AmongNoted` chooser in [`Self::binder_choice`].
    pub(super) fn live_noted_members(&self, label: &deckmaste_core::Ident) -> Vec<ObjectId> {
        self.noted
            .get(label)
            .into_iter()
            .flatten()
            .filter_map(|m| m.now)
            .filter(|&id| self.objects.get(id).is_some())
            .collect()
    }

    /// The [`RefKind`](crate::stack::RefKind) of a resolved id — object vs.
    /// player ([CR#120.3]).
    fn ref_kind_of(&self, id: ObjectId) -> crate::stack::RefKind {
        match self.objects.obj(id).source {
            ObjectSource::Player(_) => crate::stack::RefKind::Player,
            ObjectSource::Card(_) => crate::stack::RefKind::Object,
        }
    }

    /// Interpret one `OneShotEffect` node ([CR#608.2]). `Act` becomes one or
    /// more `Emit` work items (via `action_items`); `Sequentially` expands
    /// to one `RunEffect` per child.
    ///
    /// # Panics
    ///
    /// Panics on any `OneShotEffect` variant not wired for Stage 3.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per effect-frame variant; splitting would scatter the dispatch"
    )]
    pub(crate) fn run_effect(&mut self, effect: OneShotEffect, frame: &Frame) {
        match effect {
            OneShotEffect::Act(action) => {
                // A verb acts on an already-bound `Reference` — choosing is a
                // separate preceding step (`OneShotEffect::With(ChooseOne/Choose, …)`),
                // never the verb's, so an `Act` never surfaces a choice itself.
                // `CreateReplacement` directly mutates `state.shields` — it
                // cannot go through `action_items` (which is `&self`). Handle
                // it here, mirroring how `OneShotEffect::Continuously` works.
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
            OneShotEffect::Sequentially(children) => {
                // Pre-scan for `ForThisEvent` riders ([CR#611.2a], [CR#701.19c]):
                // an `Until(ForThisEvent, parts)` child NEVER mints its own
                // instance — its parts fold as instruction-scoped RIDERS onto
                // the immediately-preceding sibling's work. This shape is forced
                // by scheduling: a `Destroy` front-schedules its `Emit`, which is
                // APPLIED before the next `RunEffect` child could run, so a
                // sibling-minted instance would arrive too late ([CR#611.2c]);
                // the rider must be armed BEFORE the destroy runs. We install it
                // just before that preceding sibling's `RunEffect`.
                let mut items: Vec<WorkItem> = Vec::new();
                for child in children {
                    if let Some(parts) = for_this_event_rider(&child) {
                        let no_regen = self.resolve_no_regen_riders(parts, frame);
                        // A rider with no preceding sibling is an authoring
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
                        effect: Box::new(child),
                        frame: frame.clone(),
                    });
                }
                self.schedule_front(items);
            }
            // The written `Simultaneously` spec. ONE SNAPSHOT: every member's
            // items are evaluated up front against the current state, before
            // any applies (an exchange works BECAUSE both halves read the
            // pre-state). ONE BATCH: the merged events land as one
            // occurrence, so triggers see one [CR#603.3b] simultaneous set,
            // its facts share a batch id, replacements run per member
            // ([CR#616.1]), and SBAs run after the whole batch (the next
            // `CheckSbas`, never between members). ALL-OR-NOTHING: a member
            // that evaluates to no events voids the whole set ([CR#701.12a]
            // — "if the entire exchange can't be completed, no part of the
            // exchange occurs"). The exchange family restricts members to
            // pure-verb bodies — a choice-bearing member is unrepresentable
            // in sound data and trips loudly.
            OneShotEffect::Simultaneously(children) => {
                let mut member_events: Vec<Vec<GameEvent>> = Vec::new();
                for child in &children {
                    let OneShotEffect::Act(action) = peel_effect(child) else {
                        todo!("a non-verb Simultaneously member is not yet supported: {child:?}")
                    };
                    let mut events = Vec::new();
                    for item in self.action_items(action, frame) {
                        match item {
                            WorkItem::Emit(crate::event::Occurrence::Single(e)) => events.push(e),
                            WorkItem::Emit(crate::event::Occurrence::Batch(es)) => {
                                events.extend(es);
                            }
                            other => todo!(
                                "a choice-bearing Simultaneously member is not yet supported: scheduled {other:?}"
                            ),
                        }
                    }
                    member_events.push(events);
                }
                if member_events.iter().any(Vec::is_empty) {
                    return;
                }
                let events: Vec<GameEvent> = member_events.into_iter().flatten().collect();
                // [CR#701.14c]: a creature that fights itself deals ONE instance
                // equal to twice its power — not two. Within a simultaneous
                // batch, damage from the same source to the same target (same
                // combat-ness) is one instance; coalesce by summing amounts so a
                // self-fight's two `X -> X` packets become one `2x` event. Fight
                // is the only damage-bearing `Simultaneously` today; other member
                // events (e.g. exchange `ControlChanged`) pass through untouched.
                let events = coalesce_simultaneous_damage(events);
                self.schedule_front(vec![WorkItem::Emit(crate::event::Occurrence::Batch(
                    events,
                ))]);
            }
            OneShotEffect::Continuously(e) => {
                // [CR#611.2]/[CR#611.2c]: stamp at creation; lock the object set
                // for non-floating scopes, leave `Matching` floating.
                let timestamp = self.objects.next_timestamp();
                // Duration guard narrowed from the old catch-all: EVERY duration
                // is now sweepable EXCEPT `ForThisEvent`, which is an
                // instruction-scoped rider ([CR#611.2a]) handled entirely by
                // `Sequentially` lowering and never a standalone instance.
                // Reaching this mint with `ForThisEvent` means a rider with no
                // host instruction (e.g. a top-level `Until(ForThisEvent, …)`):
                // an authoring mistake — FIZZLE (drop it), never mint a
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
                    StaticEffect::Modify(r, change) => (
                        ScopeResolved::Locked(vec![self.eval_reference(r, frame)]),
                        Modification::flatten(vec![change.clone()]),
                        vec![],
                    ),
                    // The distributor shape ("target creature and all creatures
                    // it shares a color with get +1/+1", or a plain anthem)
                    // stays `Floating(f)`: the filter is NOT expanded to objects
                    // here.
                    StaticEffect::Each(Selection::SelectAll(f), inner) => match inner.as_ref() {
                        StaticEffect::Modify(Reference::It, change) => (
                            ScopeResolved::Floating(f.clone()),
                            Modification::flatten(vec![change.clone()]),
                            vec![],
                        ),
                        other => todo!(
                            "P0.W1: Continuously(Each(SelectAll, {other:?})) — only a bare \
                             Modify(It, _) inner is wired"
                        ),
                    },
                    // A granted `Deontic` restriction ("target creature can't
                    // block this turn"): its subject `Reference`s resolve at
                    // mint ([CR#611.2c] object lock) — `scope = Locked(ids)`,
                    // the subject rewritten to `It`, so a consumer reads `It` =
                    // the scope members. Row EVALUATION stays LOUD at `legal.rs`
                    // (core-casting-restrictions / engine-combat-requirements
                    // own it); this only makes the row EXIST in the view.
                    StaticEffect::Deontic(deontic) => {
                        let (ids, locked) = self.lock_deontic_subject(deontic, frame);
                        (
                            ScopeResolved::Locked(ids),
                            vec![],
                            vec![StaticEffect::Deontic(locked)],
                        )
                    }
                    // Self-filtered rows: `of` / the `CantHappen` filter carry
                    // their own subject predicate, so the scope is unused (an
                    // empty lock). `CostModifier` is wired into
                    // `cost_modifier_rows` (cast.rs), `CantHappen` into
                    // `cant_event` (replace_registry.rs).
                    row @ (StaticEffect::CostModifier { .. } | StaticEffect::CantHappen(_)) => {
                        (ScopeResolved::Locked(vec![]), vec![], vec![row.clone()])
                    }
                    // Prevention shields/windows are engine-prevention's domain
                    // ([CR#615.1]); the PreventNext/PreventAll macros stay
                    // blocked until that ticket lands.
                    StaticEffect::Prevention(_) => todo!(
                        "P0.W1: Continuously(Prevention) — engine-prevention owns shields/windows [CR#615.1]"
                    ),
                    // Narrower than the old catch-all: name the specific unbuilt
                    // granted static-row kind.
                    StaticEffect::Each(sel, _) => todo!(
                        "P0.W1: Continuously(Each({sel:?}, _)) — only Each(SelectAll, Modify(It, _)) is wired"
                    ),
                    other => {
                        todo!("P0.W1: Continuously({other:?}) — granted static-row kind unbuilt")
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
                self.continuous.push(ContinuousEffect {
                    timestamp,
                    // The continuous effect's controller is the controller
                    // of the spell/ability that created it ([CR#611.2c]);
                    // it resolves the `You` in a layer-2 control change.
                    controller: frame.controller,
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
            OneShotEffect::Until(duration, parts) => {
                let items: Vec<WorkItem> = parts
                    .into_iter()
                    .map(|part| WorkItem::RunEffect {
                        effect: Box::new(OneShotEffect::Continuously(
                            deckmaste_core::Continuously {
                                effect: Box::new(part),
                                duration: duration.clone(),
                            },
                        )),
                        frame: frame.clone(),
                    })
                    .collect();
                self.schedule_front(items);
            }
            // A label names the clause's introductions (a compile-time
            // soundness concept); at runtime it is transparent.
            OneShotEffect::Label(label) => self.run_effect(*label.effect, frame),
            // A remembered macro expansion (e.g. an `OneShotEffect`-kind macro like
            // `PumpThisUntilEot`) is transparent to resolution — run its value,
            // matching how every other engine layer sees through `*::Expanded`.
            OneShotEffect::Expanded(e) => self.run_effect(*e.value, frame),
            // [CR#607.2a]: fact-backed product groups — run the inner
            // effect between a BeginNote/EndNote pair; every `ZoneChanged`
            // fact the clause ACTUALLY enacts (its whole apply cascade sits
            // between the markers) joins `noted[key]`, never the gathered
            // input set: a destroy-all's indestructible survivor is
            // excluded by construction (its `WillDestroy` was canted and no
            // move fact exists). Later clauses read the group via
            // `Selection::AmongNoted` ("this way" anaphora).
            OneShotEffect::Noting(noting) => {
                self.schedule_front(vec![
                    WorkItem::BeginNote { key: noting.key },
                    WorkItem::RunEffect {
                        effect: noting.effect,
                        frame: frame.clone(),
                    },
                    WorkItem::EndNote,
                ]);
            }
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
            OneShotEffect::If(if_effect) => {
                if self.condition_holds(&if_effect.condition, frame) {
                    self.run_effect(*if_effect.then, frame);
                } else if let Some(otherwise) = if_effect.otherwise {
                    self.run_effect(*otherwise, frame);
                }
            }
            // [CR#608.2,700.1]: "[do] to each [over]" — a *simultaneous*
            // distributive. The matched set is fixed once when this node
            // resolves ([CR#608.2h]); each element binds as the iteration anaphor
            // `Reference::It` (so a body like `Destroy(It)` reads "it"), via
            // `bind_it` which also CLEARS any inherited `Distribute` allotment so
            // an outer share can't leak in. The many-binder is resolved through
            // the shared binder spine: a `ChooseOne`/`Choose` first surfaces the
            // controller's choice and re-runs this node with the picks in
            // `frame.anaphora.chosen` (the Brainstorm shape `Each(Choose(2, …), …)` then
            // iterates BOTH picks); `Existing` evaluates its `Selection`; `TheRef`
            // is a singleton. Because a verb takes a single `Reference` and never
            // pauses for a choice, a single-`Act` body resolves for EVERY element
            // at once — one `Occurrence::Batch`, so death triggers / SBAs / the
            // loss-is-a-draw check see them together (the simultaneity the old
            // verb-over-`Predicate` carried). A body that can pause (Sequentially/With/If,
            // or `CreateReplacement` which bypasses `action_items`) keeps
            // per-element scheduling, where each element runs and pauses
            // independently.
            OneShotEffect::Each(each) => {
                if let Some((chooser, candidates, min, max)) =
                    self.binder_choice(&each.binder, frame)
                {
                    self.pending = Some(crate::decide::PendingDecision::ChooseObjects {
                        player: chooser,
                        candidates,
                        min,
                        max,
                    });
                    self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
                        effect: Box::new(OneShotEffect::Each(each)),
                        frame: frame.clone(),
                    });
                    return;
                }
                let matches = self.resolve_binder(&each.binder, frame);
                // [CR#701.22a] look-visibility: the controller sees the cards a
                // top-of-library PEEK binds (scry/surveil/fateseal look before
                // they arrange), keyed by object identity (expires free on the
                // next remint/shuffle).
                if Self::is_top_of_library_peek(&each.binder) {
                    for &obj in &matches {
                        self.look_grants.insert((frame.controller, obj));
                    }
                }
                // [CR#401.4]: if the body puts cards into ordered library
                // positions (scry's top/bottom picks), arm the post-pick arrange
                // collector and schedule the finalizer AFTER the elements so it
                // orders each pile once every pick has landed. The arranger is
                // the effect's controller (for fateseal, over the opponent's
                // library).
                let can_reposition = Self::body_repositions_ordered(&each.effect);
                if can_reposition {
                    self.arrange_scope = Some(crate::state::ArrangeScope {
                        arranger: frame.controller,
                        landings: Vec::new(),
                    });
                }
                // `bind_it` per element: set `It`, clear the allotment, and open a
                // fresh choice scope so a nested chooser doesn't read this node's
                // picks.
                let bind_it = |me: &Self, obj: ObjectId| {
                    let mut next = frame.clone();
                    next.anaphora.it = Some(me.it_binding(obj));
                    next.anaphora.allotment = None;
                    next.anaphora.chosen = None;
                    next
                };
                match peel_effect(&each.effect) {
                    OneShotEffect::Act(action)
                        if !matches!(action, Action::CreateReplacement { .. }) =>
                    {
                        // Resolve every element's work items up front. Only a
                        // pure-event body — one whose items are all `Emit` —
                        // collapses to a single simultaneous batch ([CR#700.1]).
                        // A choice-bearing body (e.g. `By(player, Discard)`,
                        // whose `action_items` yield `DiscardCards` /
                        // `ChooseManaColor` / `OpenDistribute`) must NOT be
                        // batched: those items pause per element, and keeping
                        // only the `Emit`s would silently drop them (each player
                        // would no-op). `action_items` is pure, so this probe is
                        // free of side effects.
                        let per_element: Vec<Vec<WorkItem>> = matches
                            .into_iter()
                            .map(|obj| self.action_items(action, &bind_it(self, obj)))
                            .collect();
                        let all_emit = per_element
                            .iter()
                            .flatten()
                            .all(|item| matches!(item, WorkItem::Emit(_)));
                        if all_emit {
                            let mut events: Vec<GameEvent> = Vec::new();
                            for item in per_element.into_iter().flatten() {
                                if let WorkItem::Emit(occ) = item {
                                    match occ {
                                        crate::event::Occurrence::Single(e) => events.push(e),
                                        crate::event::Occurrence::Batch(v) => events.extend(v),
                                    }
                                }
                            }
                            let mut items = Vec::new();
                            if !events.is_empty() {
                                items.push(WorkItem::Emit(occurrence_of(events)));
                            }
                            if can_reposition {
                                items.push(WorkItem::ArrangePiles);
                            }
                            self.schedule_front(items);
                        } else {
                            // Not batchable — schedule each element's items in
                            // order so the choice-bearing ones actually run.
                            let mut items: Vec<WorkItem> =
                                per_element.into_iter().flatten().collect();
                            if can_reposition {
                                items.push(WorkItem::ArrangePiles);
                            }
                            self.schedule_front(items);
                        }
                    }
                    _ => {
                        let mut items: Vec<WorkItem> = matches
                            .into_iter()
                            .map(|obj| WorkItem::RunEffect {
                                effect: each.effect.clone(),
                                frame: bind_it(self, obj),
                            })
                            .collect();
                        if can_reposition {
                            items.push(WorkItem::ArrangePiles);
                        }
                        self.schedule_front(items);
                    }
                }
            }
            // Bind the With anaphor BEFORE the body ([CR#608.2]) — choosing is a
            // separate step, never part of the verb. A one-binder
            // (`TheRef`/`ChooseOne`) binds a single object read as singular
            // `Reference::That` (a `(One, k)` slot); a many-binder
            // (`Choose`/`Existing`) binds a group read as `Selection::That` (a
            // `(Many, k)` slot, order preserved, top→down for a library window).
            // The cardinality rides the `that` slot so the singular and group
            // reads resolve by slot — a `(Many, k)` binding has no singular read,
            // making the first-of-many bug unrepresentable. A chooser binder
            // (`ChooseOne`/`Choose`) first surfaces its `by`-player's choice
            // ([CR#608.2d]) and re-runs this node with the picks in
            // `frame.anaphora.chosen`.
            OneShotEffect::With(with) => {
                // [CR#400.7j]: a producer binder runs its action and binds the
                // PRE-move id as a One `That`; the bound-role reads chase the
                // move record, so after the action's zone change applies the
                // body's `That` resolves to the product. Only `Move` produces
                // in this cut — other actions stay a labeled seam.
                if let deckmaste_core::Binder::Produce(action) = peel_binder(&with.binder) {
                    let deckmaste_core::Action::Move(subject, _, _) = action.as_ref() else {
                        unimplemented!(
                            "Binder::Produce over a non-Move action: only zone-moves \
                             produce-and-capture in this cut ({action:?})"
                        );
                    };
                    let id = self.eval_reference(subject, frame);
                    let mut next = frame.clone();
                    next.anaphora.that = Some(crate::stack::ThatBinding {
                        cardinality: crate::stack::Cardinality::One,
                        kind: crate::stack::RefKind::Object,
                        group: vec![id],
                    });
                    next.anaphora.chosen = None;
                    let mut items = self.action_items(action, frame);
                    items.push(WorkItem::RunEffect {
                        effect: with.body,
                        frame: next,
                    });
                    self.schedule_front(items);
                    return;
                }
                if let Some((chooser, candidates, min, max)) =
                    self.binder_choice(&with.binder, frame)
                {
                    self.pending = Some(crate::decide::PendingDecision::ChooseObjects {
                        player: chooser,
                        candidates,
                        min,
                        max,
                    });
                    self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
                        effect: Box::new(OneShotEffect::With(with)),
                        frame: frame.clone(),
                    });
                    return;
                }
                let group = self.resolve_binder(&with.binder, frame);
                let cardinality = Self::binder_cardinality(&with.binder);
                let kind = group
                    .first()
                    .map_or(crate::stack::RefKind::Object, |&id| self.ref_kind_of(id));
                let mut next = frame.clone();
                next.anaphora.that = Some(crate::stack::ThatBinding {
                    cardinality,
                    kind,
                    group,
                });
                // The body opens a fresh choice scope: a nested `With` surfaces
                // its own choice rather than reading this one's picks.
                next.anaphora.chosen = None;
                self.schedule_front(vec![WorkItem::RunEffect {
                    effect: with.body,
                    frame: next,
                }]);
            }
            // [CR#601.2d]: divide `amount` among the many-binder's group "as you
            // choose" — bind each element as the iteration anaphor `Reference::It`
            // (like `Each`) with its `Count::Allotment` share in the `allotment`
            // slot (the Idris `bindAllot`), then schedule one `RunEffect` per
            // element (order preserved, each pausing independently if its body
            // surfaces a choice). The binder is resolved through the shared spine,
            // so a `ChooseOne`/`Choose` group surfaces its choice first. v1 splits
            // the amount as evenly as possible; surfacing the "as you choose"
            // division as a player decision is a seam.
            OneShotEffect::Distribute(divide) => {
                if let Some((chooser, candidates, min, max)) =
                    self.binder_choice(&divide.binder, frame)
                {
                    self.pending = Some(crate::decide::PendingDecision::ChooseObjects {
                        player: chooser,
                        candidates,
                        min,
                        max,
                    });
                    self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
                        effect: Box::new(OneShotEffect::Distribute(divide)),
                        frame: frame.clone(),
                    });
                    return;
                }
                let group = self.resolve_binder(&divide.binder, frame);
                let total = self.eval_count(&divide.amount, frame);
                let shares = split_evenly(total, group.len());
                let items: Vec<WorkItem> = group
                    .into_iter()
                    .zip(shares)
                    .map(|(obj, share)| {
                        let mut next = frame.clone();
                        // `bindAllot`: bind `It` and put this element's share in
                        // scope for `Count::Allotment`.
                        next.anaphora.it = Some(self.it_binding(obj));
                        next.anaphora.allotment = Some(share);
                        // A fresh choice scope per element, like `Each`.
                        next.anaphora.chosen = None;
                        WorkItem::RunEffect {
                            effect: divide.body.clone(),
                            frame: next,
                        }
                    })
                    .collect();
                self.schedule_front(items);
            }
            // [CR#118.12]: "[A player] may [do]. If they do/don't, …". Surface a
            // yes/no to the controller; the chosen branch (effect + if_did on
            // yes, if_not on no) runs when the answer comes back — the `May`
            // continuation in `submit_decision`.
            OneShotEffect::May(may) => {
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
            OneShotEffect::Modal(modal) => {
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
                if modal.choose.rider.is_some() {
                    // Entwine/escalate costs are announce-time additions to the
                    // total ([CR#702.42a,702.120a,601.2b,601.2f]).
                    todo!(
                        "engine-alt-costs seam: entwine/escalate riders are announce-time \
                         cost additions ([CR#601.2b])"
                    );
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
            // [CR#118.12a]: "[actor] must pay [cost], or else [or_else]" — the
            // Mana Leak punisher over the full `Cost` (the English "unless"
            // order is the `Unless` macro over this node). Surface a yes/no to
            // the paying player; on yes the cost is paid and `or_else` is
            // skipped, on no `or_else` happens — branched when the answer
            // returns (the `Unless` continuation). v1 does NOT gate the offer
            // on affordability (a refinement); the runner only offers "pay"
            // when able. A ward toll's `Mana([Variable])` is priced by the
            // resolving triggered ability's `where_x`, evaluated NOW
            // ([CR#702.21b] — at resolution, never locked in at trigger).
            OneShotEffect::MustPay(m) => {
                let payer = self.acting_player(&m.actor, frame);
                self.pending = Some(crate::decide::PendingDecision::YesNo { player: payer });
                // Normalize the authored cost at this boundary: read is
                // faithful, so a macro-spliced cost arrives lumpy (a nested
                // `CostComponent::Cost`); splice it flat before the payment
                // walk (`unless_cost_action`) consumes it.
                let cost = m.cost.normalize().0;
                let cost = self.price_variable_cost(cost, frame);
                self.choice = Some(crate::state::ChoiceContinuation::Unless {
                    effect: m.or_else,
                    who: m.actor,
                    unless: cost,
                    frame: frame.clone(),
                });
            }
            // [CR#603,608]: "[actor] may pay [cost]; if they do → and_then,
            // else → or_else" — a resolution-time kicker. Unlike `MustPay`, the
            // PAID branch also runs an effect, so it carries its own
            // continuation.
            OneShotEffect::MayPay(m) => {
                let payer = self.acting_player(&m.actor, frame);
                self.pending = Some(crate::decide::PendingDecision::YesNo { player: payer });
                self.choice = Some(crate::state::ChoiceContinuation::MayPay {
                    actor: m.actor,
                    cost: m.cost.normalize().0,
                    and_then: m.and_then,
                    or_else: m.or_else,
                    frame: frame.clone(),
                });
            }
            // [CR#115.1,601.2c]: a target-scoping wrapper. Targets were chosen
            // at announcement and already live in `frame.anaphora.targets`;
            // this node is otherwise transparent — descend into the inner
            // effect, exactly like `Expanded`. The body reads announced
            // targets by position via `Reference::Target(n)`.
            OneShotEffect::Targeted(te) => {
                self.run_effect(*te.effect, frame);
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
            OneShotEffect::Repeat(count, body) => {
                let n = self.eval_count(&count, frame);
                if n > 0 {
                    let items = vec![
                        WorkItem::RunEffect {
                            effect: body.clone(),
                            frame: frame.clone(),
                        },
                        WorkItem::RunEffect {
                            effect: Box::new(OneShotEffect::Repeat(Count::Literal(n - 1), body)),
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
            // then run `body`. GENUINELY ABSENT SUBSYSTEM — the identical
            // seam the sibling `engine-explore` ticket
            // (`docs/tickets/planned/engine-explore.md`, itself split out of
            // `engine-scry-surveil-explore` for exactly this reason) is
            // blocked on: `PlayerAction::Reveal` / `GameEvent::Revealed` are
            // SHAPED but unbuilt (`todo!("P0.W6: reveal apply")` at
            // `step.rs`'s `GameEvent::Revealed` apply arm;
            // `todo!("P0.W6: reveal/look")` at this file's own
            // `PlayerAction::Reveal` arm) — the reveal WINDOW lifetime
            // machinery ([CR#701.20a]) they need does not exist. The
            // predicate-match (`target::matches`) and `It`/`They`
            // anaphora-binding machinery this needs ARE built and reusable,
            // but walking the library and binding anaphora WITHOUT ever
            // emitting a reveal fact would silently skip the one thing that
            // makes "reveal" a real game action (a future "whenever a card
            // is revealed"/"as long as it's revealed" consumer would see
            // nothing happen) — a convenient-but-wrong shortcut, not a fix.
            // Building the seam is a real subsystem, out of scope for a
            // grammar-mirror task; this arm fizzles to a graceful no-op
            // instead (nothing revealed, `body` never runs) — never a
            // panic, matching the CRITICAL never-crash ruling.
            OneShotEffect::RevealUntil(_) => {}
            OneShotEffect::AdditionalCost(ac) => {
                let cost = ac.pay.normalize().0;
                let mut body_frame = frame.clone();
                if let Some(snapshot) = self.additional_cost_paid_object(&cost, frame) {
                    body_frame.anaphora.that_object = Some(snapshot);
                }
                let mut items: Vec<WorkItem> = cost
                    .iter()
                    .map(|c| WorkItem::RunEffect {
                        // Render each cost component as the controller's payment
                        // effect; a cost-side `With` becomes an `OneShotEffect::With`
                        // (its choice surfaced at payment), not a single action.
                        effect: Box::new(crate::decide::unless_cost_effect(c, &Reference::You)),
                        frame: frame.clone(),
                    })
                    .collect();
                items.push(WorkItem::RunEffect {
                    effect: ac.body,
                    frame: body_frame,
                });
                self.schedule_front(items);
            }
            // [CR#603.7]: create a delayed triggered ability. It is printed on
            // no permanent, so it goes into the `delayed_triggers` registry
            // (the trigger scan consults it alongside live permanents) and
            // fires ONCE, the next time its event occurs ([CR#603.7b]). Source
            // and controller follow [CR#603.7d,603.7e] — the creating spell/ability
            // and the player who controlled it as it resolved (`frame`); `~`/
            // `This` is the creating object's snapshot, carried so the delayed
            // body reads it at the later resolution ([CR#603.7c]).
            OneShotEffect::Delayed(ability) => {
                let (source, bindings) = self.created_trigger_context(frame);
                self.delayed_triggers.push(crate::trigger::CreatedTrigger {
                    source,
                    controller: frame.controller,
                    ability,
                    bindings,
                });
            }
            // [CR#603.12]: create a reflexive triggered ability ("when you
            // do"). It follows the delayed rules EXCEPT it is checked
            // immediately against events that occurred EARLIER in THIS
            // resolution — never persisted, never firing on a future event. So
            // it is not registered: scan the resolution-scoped window now and
            // emit a `TriggerFired` (carrying the body by value) per match
            // ([CR#603.12a] — once per occurrence of the trigger event).
            OneShotEffect::Reflexive(ability) => {
                let (source, base) = self.created_trigger_context(frame);
                let mut emits = Vec::new();
                for event in self.resolution_events.clone() {
                    if self.event_matches_delayed(&ability.event, &event, source) {
                        let roles = self.event_roles(&event);
                        emits.push(WorkItem::Emit(Occurrence::single(
                            GameEvent::TriggerFired {
                                source,
                                ability: 0,
                                controller: frame.controller,
                                created: Some(ability.clone()),
                                bindings: roles.bindings_over(base.clone()),
                            },
                        )));
                    }
                }
                if !emits.is_empty() {
                    self.schedule_front(emits);
                }
            }
            other => todo!("stage 3 does not interpret effect {other:?} (the choice seam)"),
        }
    }

    /// [CR#603.7d,603.7e]: the source and captured `~`/`This` context for a
    /// delayed/reflexive triggered ability created while `frame` resolves. The
    /// source is the creating object (the trigger/activated ability's own
    /// source snapshot when `frame` has one — [CR#603.7e]; otherwise the
    /// resolving spell — [CR#603.7d]). `~`/`This` is that same object's
    /// snapshot, so the created body reads it at its later resolution.
    fn created_trigger_context(
        &self,
        frame: &Frame,
    ) -> (ObjectSource, crate::trigger::TriggerBindings) {
        let this = frame.this.clone().or_else(|| {
            self.objects
                .get(frame.source)
                .map(|_| crate::lki::LkiSnapshot::capture(self, frame.source))
        });
        let source = this
            .as_ref()
            .map_or_else(|| ObjectSource::Player(frame.controller), |s| s.source);
        let bindings = crate::trigger::TriggerBindings {
            this,
            defending_player: frame.defending_player,
            ..crate::trigger::TriggerBindings::default()
        };
        (source, bindings)
    }

    /// The last-known snapshot of the object an
    /// [`OneShotEffect::AdditionalCost`] moves, when its cost is a single
    /// object-moving verb (`Sacrifice`/`Move` — exile is `Move(_, Exile)` —
    /// or a `Discard` that names *what*) over a directly-resolvable
    /// reference (`Sacrifice(This)`, …). Captured BEFORE payment, so it is
    /// the object's last-known information ([CR#603.10a]) — exactly what
    /// the body's `EventObject` should read. Returns `None` for a cost that
    /// moves no object (mana/tap/life — nothing to bind), or one
    /// whose moved object is bound by an enclosing cost `With(ChooseOne, …)`
    /// (a `That` not fixed until the choice resolves — a documented
    /// payment-time capture seam, see the `AdditionalCost` arm).
    fn additional_cost_paid_object(
        &self,
        cost: &[deckmaste_core::CostComponent],
        frame: &Frame,
    ) -> Option<crate::lki::LkiSnapshot> {
        for component in cost {
            let deckmaste_core::CostComponent::Do(pa) = component else {
                continue;
            };
            let reference = match pa.as_ref() {
                PlayerAction::Sacrifice(r)
                | PlayerAction::Move(r, _, _)
                | PlayerAction::Discard { what: Some(r), .. } => Some(r),
                _ => None,
            };
            // Only a directly-resolvable reference is captured here; a
            // `That(Sort)` bound by an enclosing cost `With(ChooseOne, …)`
            // has no fixed object until the choice resolves (seam).
            if let Some(reference) = reference
                && !matches!(reference, Reference::That(_))
            {
                let object = self.eval_reference(reference, frame);
                return Some(crate::lki::LkiSnapshot::capture(self, object));
            }
        }
        None
    }

    /// A `ZoneWillChange` intent ([CR#400.7]) moving `object` to `to` from
    /// WHATEVER zone it currently occupies — the current-zone lookup
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

    /// Whether an [`Action::Composite`]'s `body` will actually do something
    /// this resolution — the gate on emitting the keyword-action fact
    /// ([CR#701.22b]: scry 0 is a no-op and fires no "you scried" trigger). The
    /// keyword-action macros all wrap an `Each` over a peek selection, so an
    /// empty peek (count 0, or an empty library) means nothing happened; any
    /// other body is assumed to act.
    pub(super) fn composite_body_acts(&self, body: &OneShotEffect, frame: &Frame) -> bool {
        match peel_effect(body) {
            OneShotEffect::Each(each) => match peel_binder(&each.binder) {
                deckmaste_core::Binder::Existing(_) | deckmaste_core::Binder::TheRef(_) => {
                    !self.resolve_binder(&each.binder, frame).is_empty()
                }
                _ => true,
            },
            // A guarded keyword action — e.g. fight fires no "fights" event when
            // either creature is no longer a creature on the battlefield
            // ([CR#701.14b], the `If (both are creatures) …` guard). Look
            // through to whichever branch the condition selects.
            OneShotEffect::If(i) => {
                if self.condition_holds(&i.condition, frame) {
                    self.composite_body_acts(&i.then, frame)
                } else {
                    i.otherwise
                        .as_ref()
                        .is_some_and(|o| self.composite_body_acts(o, frame))
                }
            }
            _ => true,
        }
    }

    /// Whether `binder` peeks the top of a library ([CR#701.22a]) — the peek
    /// whose cards the controller is granted visibility over.
    fn is_top_of_library_peek(binder: &deckmaste_core::Binder) -> bool {
        matches!(
            peel_binder(binder),
            deckmaste_core::Binder::Existing(Selection::TopOfLibrary { .. })
        )
    }

    /// Whether an `Each`/distributor body puts cards into ORDERED library
    /// positions ([CR#401.7]) — the signal to arm the post-pick arrange
    /// collector ([CR#401.4]). True when any reachable move verb targets a
    /// [`Destination::Library`] anchor (scry/surveil's top pick, fateseal);
    /// false for a graveyard-only body (mill — the graveyard is unordered).
    fn body_repositions_ordered(effect: &OneShotEffect) -> bool {
        match peel_effect(effect) {
            OneShotEffect::Act(a) => Self::action_moves_to_library(a),
            OneShotEffect::Sequentially(v) | OneShotEffect::Simultaneously(v) => {
                v.iter().any(Self::body_repositions_ordered)
            }
            OneShotEffect::Modal(m) => m
                .modes
                .iter()
                .any(|mode| Self::body_repositions_ordered(&mode.effect)),
            OneShotEffect::With(w) => Self::body_repositions_ordered(&w.body),
            OneShotEffect::Each(e) => Self::body_repositions_ordered(&e.effect),
            OneShotEffect::Distribute(d) => Self::body_repositions_ordered(&d.body),
            OneShotEffect::If(i) => {
                Self::body_repositions_ordered(&i.then)
                    || i.otherwise
                        .as_ref()
                        .is_some_and(|o| Self::body_repositions_ordered(o))
            }
            OneShotEffect::Targeted(t) => Self::body_repositions_ordered(&t.effect),
            OneShotEffect::Label(l) => Self::body_repositions_ordered(&l.effect),
            OneShotEffect::Expanded(e) => Self::body_repositions_ordered(&e.value),
            _ => false,
        }
    }

    /// Whether a move verb (source- or player-agent) relocates to an ordered
    /// [`Destination::Library`] position; `Composite` looks through to its
    /// body.
    fn action_moves_to_library(a: &Action) -> bool {
        match a {
            Action::Move(_, Destination::Library(_), _)
            | Action::By(_, PlayerAction::Move(_, Destination::Library(_), _)) => true,
            Action::Composite { body, .. } => Self::body_repositions_ordered(body),
            _ => false,
        }
    }

    /// Price the `{X}` symbols of a resolution-time cost ([CR#702.21b] — a
    /// ward-{X} toll's X "is determined at the time the ability resolves,
    /// not locked in as the ability triggers"): with a `where_x` definition
    /// on the resolving frame, every `Variable` mana symbol becomes
    /// `Generic(X)` evaluated NOW; without one, the cost passes through
    /// unchanged.
    fn price_variable_cost(
        &self,
        cost: Vec<deckmaste_core::CostComponent>,
        frame: &Frame,
    ) -> Vec<deckmaste_core::CostComponent> {
        use deckmaste_core::CostComponent;
        use deckmaste_core::ManaSymbol;
        use deckmaste_core::SimpleManaSymbol;
        let Some(def) = &frame.anaphora.where_x else {
            return cost;
        };
        let x = self.eval_count(def, frame);
        cost.into_iter()
            .map(|component| match component {
                CostComponent::Mana(m) => {
                    let symbols: Vec<ManaSymbol> = Vec::from(m)
                        .into_iter()
                        .map(|s| match s {
                            ManaSymbol::Variable => {
                                ManaSymbol::Simple(SimpleManaSymbol::Generic(x))
                            }
                            other => other,
                        })
                        .collect();
                    CostComponent::Mana(symbols.into())
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

/// A CONSTRAINING `AmongNoted` inside a selection ("exile TWO of them"): the
/// `(label, quantity)` to surface as a chooser ([CR#608.2d]), or `None` for an
/// UNCONSTRAINED `AmongNoted` (the whole group — not a choice) or any other
/// selection. Peels `Selection::Expanded`.
fn among_noted_choice(
    selection: &Selection,
) -> Option<(&deckmaste_core::Ident, &deckmaste_core::Quantity)> {
    match selection {
        Selection::Expanded(e) => among_noted_choice(&e.value),
        Selection::AmongNoted(label, quantity)
            if !matches!(
                deref_quantity(quantity),
                deckmaste_core::Quantity::Range(None, None)
            ) =>
        {
            Some((label, quantity))
        }
        _ => None,
    }
}

/// If `effect` is a `ForThisEvent` rider clause — `Until(ForThisEvent, parts)`,
/// peeling `Expanded` — return its `parts`: the instruction-scoped statics to
/// fold onto the preceding sibling in `Sequentially` lowering ([CR#611.2a]).
/// `None` for every other effect.
fn for_this_event_rider(effect: &OneShotEffect) -> Option<&[StaticEffect]> {
    match peel_effect(effect) {
        OneShotEffect::Until(deckmaste_core::Duration::ForThisEvent, parts) => Some(parts),
        _ => None,
    }
}

/// The mutable structural [`DeonticAction`] of a `Deontic`
/// (May/Cant/Must/Gate). `None` for an unexpanded `Expanded` macro — by
/// resolution time deontics are expanded, so this is only a defensive floor
/// (lock nothing, leave it).
fn deontic_action_mut(d: &mut Deontic) -> Option<&mut DeonticAction> {
    match d {
        Deontic::May(a) | Deontic::Cant(a) | Deontic::Must(a) | Deontic::Gate(a, _) => Some(a),
        Deontic::Expanded(_) => None,
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
        DeonticAction::Expanded(_) => vec![],
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
        if let GameEvent::DamageDealt {
            source,
            target,
            amount,
            combat,
        } = ev
        {
            let existing = out.iter_mut().find_map(|e| match e {
                GameEvent::DamageDealt {
                    source: s,
                    target: t,
                    amount: a,
                    combat: c,
                } if *s == source && *t == target && *c == combat => Some(a),
                _ => None,
            });
            match existing {
                Some(a) => *a += amount,
                None => out.push(GameEvent::DamageDealt {
                    source,
                    target,
                    amount,
                    combat,
                }),
            }
        } else {
            out.push(ev);
        }
    }
    out
}
