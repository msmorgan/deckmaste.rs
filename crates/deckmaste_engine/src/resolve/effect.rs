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
    pub(crate) fn resolve_binder(
        &self,
        binder: &deckmaste_core::Binder,
        frame: &Frame,
    ) -> Vec<ObjectId> {
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
    /// [CR#608.2g]: if `may`'s body is a bare `Cast(<ref>)` verb, resolve the
    /// caster (the `By` agent, `You` by default) and the referent object it
    /// would cast — so the `May` arm can gate the "yes" offer on
    /// [`can_cast_as_effect`](Self::can_cast_as_effect). `None` for any other
    /// `May` body (the ordinary "you may [do]", offered unconditionally). The
    /// referent id may be null/stale; the gate treats that as uncastable.
    fn may_cast_referent(
        &self,
        may: &deckmaste_core::May,
        frame: &Frame,
    ) -> Option<(crate::player::PlayerId, crate::object::ObjectId)> {
        match peel_effect(&may.effect) {
            OneShotEffect::Act(Action::By(actor, PlayerAction::Cast(what))) => Some((
                self.acting_player(actor, frame),
                self.eval_reference(what, frame),
            )),
            _ => None,
        }
    }

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
            // effect between a BeginNote/EndNote pair; every past-form
            // `ZoneChange` fact the clause ACTUALLY enacts (its whole apply cascade sits
            // between the markers) joins `noted[key]`, never the gathered
            // input set: a destroy-all's indestructible survivor is
            // excluded by construction (its `Act(Destroy)` was canted and no
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
                        // Resolve every element's work items up front. A
                        // pure keyword-action body — one whose items are all
                        // `Emit` (the event windows) plus their `FinalizeAct`
                        // watchers — collapses its windows into a single
                        // simultaneous batch ([CR#700.1]), the watchers riding
                        // AFTER so each still finalizes on its own patient. A
                        // choice-bearing body (e.g. `By(player, Discard)`, whose
                        // `action_items` yield `DiscardCards` /
                        // `ChooseManaColor` / `OpenDistribute`) must NOT be
                        // batched: those items pause per element, and keeping
                        // only the `Emit`s would silently drop them (each player
                        // would no-op). `action_items` is pure, so this probe is
                        // free of side effects.
                        let per_element: Vec<Vec<WorkItem>> = matches
                            .into_iter()
                            .map(|obj| self.action_items(action, &bind_it(self, obj)))
                            .collect();
                        let batchable = per_element.iter().flatten().all(|item| {
                            matches!(item, WorkItem::Emit(_) | WorkItem::FinalizeAct { .. })
                        });
                        if batchable {
                            let mut events: Vec<GameEvent> = Vec::new();
                            let mut finalizers: Vec<WorkItem> = Vec::new();
                            for item in per_element.into_iter().flatten() {
                                match item {
                                    WorkItem::Emit(crate::event::Occurrence::Single(e)) => {
                                        events.push(e);
                                    }
                                    WorkItem::Emit(crate::event::Occurrence::Batch(v)) => {
                                        events.extend(v);
                                    }
                                    other => finalizers.push(other),
                                }
                            }
                            let mut items = Vec::new();
                            if !events.is_empty() {
                                items.push(WorkItem::Emit(occurrence_of(events)));
                            }
                            // The watchers finalize AFTER the batch commits.
                            items.extend(finalizers);
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
                    let deckmaste_core::Action::Move(subject, _, _, _) = action.as_ref() else {
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
                // [CR#608.2g]: "you may cast that card. If you don't, …" — the
                // "yes" (cast) branch is offered ONLY when a legal, payable cast
                // of the referent exists; otherwise the offer is empty and the
                // `if_not` branch runs (faithful even when the card is
                // uncastable — a land, an unaffordable cost, no legal target).
                // Detected structurally: a `May` whose body is a bare `Cast`
                // verb, gated by `can_cast_as_effect` before surfacing YesNo.
                if let Some((caster, object)) = self.may_cast_referent(&may, frame)
                    && !self.can_cast_as_effect(caster, object)
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
            // SHELL arm ([CR#616.1g] aggregate-count tier is a later pass's
            // job): `Batch` resolves EXACTLY like `Repeat` above — the same
            // lazy self-rescheduling continuation (never `n`-many eagerly
            // materialized `RunEffect` items, per the CRITICAL never-crash
            // ruling against a saturated/huge count), just recursing into
            // `Batch` rather than `Repeat` on the tail so the shape survives
            // intact for the later pass to rebuild into ONE aggregate
            // containing event. `n == 0` schedules nothing — a clean no-op.
            OneShotEffect::Batch(count, body) => {
                let n = self.eval_count(&count, frame);
                if n > 0 {
                    let items = vec![
                        WorkItem::RunEffect {
                            effect: body.clone(),
                            frame: frame.clone(),
                        },
                        WorkItem::RunEffect {
                            effect: Box::new(OneShotEffect::Batch(Count::Literal(n - 1), body)),
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
                Action::By(_, PlayerAction::Sacrifice(r) | PlayerAction::Move(r, _, _)) => Some(r),
                // The bound discard form ("discard this card") names its
                // moved card in the body's single-move head.
                Action::Composite(deckmaste_core::KeywordAction::Discard(..), body) => {
                    deckmaste_core::discard_body_what(body)
                }
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
        GameEvent::ZoneChange {
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
        }
    }

    /// Whether an [`Action::Composite`]'s reorder/guarded `body` will actually
    /// do something this resolution — the fizzle gate for the verbs whose
    /// "did it happen" can't be read off flat coordinates ([CR#701.22b]: scry 0
    /// fires no "you scried" trigger; [CR#701.14b]: a fight whose guard fails
    /// does nothing). An `Each` over a peek is a no-op when the peek is empty;
    /// an `If`-guarded body looks through to the branch its condition selects.
    /// The move verbs (destroy/discard/mill) vet their coordinates directly in
    /// `composite_items` instead.
    pub(crate) fn composite_body_would_act(&self, body: &OneShotEffect, frame: &Frame) -> bool {
        match peel_effect(body) {
            OneShotEffect::Each(each) => match peel_binder(&each.binder) {
                deckmaste_core::Binder::Existing(_) | deckmaste_core::Binder::TheRef(_) => {
                    !self.resolve_binder(&each.binder, frame).is_empty()
                }
                _ => true,
            },
            OneShotEffect::If(i) => {
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
            Action::Move(_, Destination::Library(_), _, _)
            | Action::By(_, PlayerAction::Move(_, Destination::Library(_), _)) => true,
            Action::Composite(_, body) => Self::body_repositions_ordered(body),
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

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use deckmaste_core::Action;
    use deckmaste_core::Binder;
    use deckmaste_core::Card;
    use deckmaste_core::Count;
    use deckmaste_core::Countable;
    use deckmaste_core::ObjectKind;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::stack::Cardinality;
    use crate::stack::Frame;
    use crate::stack::RefKind;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::stack::ThatBinding;
    use crate::state::GameState;
    use crate::step::Progress;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    /// A `Targeted` wrapper is transparent at resolution — the inner
    /// instruction runs with `frame.anaphora.targets` already bound, so
    /// `Target(0)` resolves and damage lands ([CR#115.1,608]).
    #[test]
    fn targeted_effect_resolves_its_inner_effect() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);
        state.run_effect(
            OneShotEffect::Targeted(deckmaste_core::Targeted::new(
                vec![],
                OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3))),
            )),
            &frame,
        );
        // RunEffect(Targeted) → RunEffect(DealDamage) → Emit(DamageDealt).
        for _ in 0..3 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).total_damage(), 3);
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
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let filter = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let mut got = state.eval_selection_set(&Selection::SelectAll(filter), &frame);
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want);
    }

    /// `Each(Kind(Player), DealDamage(This, 20, It))` deals 20 damage to
    /// each of the two players. A verb's patient is a single `Reference`, so
    /// the spread over the player set is the enclosing `Each` — one
    /// `DamageDealt` per element rather than the old single multi-target
    /// `Batch` ([CR#608.2,120.1]).
    #[test]
    fn each_player_deal_damage_hits_both_players() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::Kind(ObjectKind::Player))),
            effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
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

    /// `Each(And([InZone(Battlefield), Type(Creature)]), DealDamage(
    /// It, 2))` deals 2 damage to each of the two battlefield creatures
    /// — one `DamageDealt` per iterated element (the verb's patient is a single
    /// `Reference`).
    #[test]
    fn each_creature_deal_damage_hits_both_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second creature onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]))),
            effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Literal(2),
            ))),
        });
        state.run_effect(effect, &frame);

        // Simultaneity ([CR#700.1]): a single-verb `Each` body resolves for
        // every element at once — both `DamageDealt`s ride ONE `Occurrence::Batch`
        // (not two sequential singles), so death triggers / SBAs see them
        // together. This is the "to each" simultaneity the old verb-over-`Predicate`
        // carried, restored after the verb→`Reference` split.
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

    #[test]
    fn each_over_choice_bearing_body_schedules_its_work_items() {
        // Regression: the `Each` single-`Act` batch path must NOT batch a
        // choice-bearing body. `By(You, Discard)` yields a `DiscardCards` work
        // item (not an `Emit`); the old code kept only `Emit`s, so the discards
        // were silently dropped ("each player discards a card" would no-op). Here
        // the body must instead be scheduled, one item per element. (Synthetic
        // "for each creature, you discard a card" shape — chosen to exercise the
        // non-`Emit` seam without standing up a player-matching `over`.)
        let (mut state, a) = bear_on_field();
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]))),
            effect: Box::new(OneShotEffect::Act(Action::discard(
                Reference::You,
                Count::Literal(1),
                false,
            ))),
        });
        state.run_effect(effect, &frame);

        // Each element's choice-bearing item is scheduled — not folded into (and
        // lost by) a simultaneous `Emit` batch.
        let discards = state
            .agenda
            .iter()
            .filter(|item| matches!(item, WorkItem::DiscardCards { .. }))
            .count();
        assert_eq!(
            discards, 2,
            "each element's discard work item is scheduled, not dropped"
        );
        assert!(
            !matches!(state.agenda.front(), Some(WorkItem::Emit(_))),
            "a choice-bearing Each body must not collapse to a simultaneous Emit batch"
        );
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

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(2)));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(3), Box::new(body)),
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

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(2)));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(0), Box::new(body)),
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
        use crate::decide::PendingDecision;

        // Pump steps until a decision surfaces (a `RunEffect` work item's own
        // `step()` call only *sets* `self.pending` as a side effect and
        // returns `Progress::Resolving` for that step; `NeedsDecision` is
        // reported on the NEXT `step()` call, which sees `pending` already
        // set — so this may take more than one `step()`).
        fn step_to_decision(state: &mut GameState) -> crate::decide::PendingDecision {
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
            OneShotEffect::May(deckmaste_core::May {
                effect: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                    Count::Literal(3),
                ))),
                if_did: None,
                if_not: None,
            })
        };
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(2), Box::new(may_gain_3())),
            &frame,
        );

        // First iteration's decision.
        let PendingDecision::YesNo { player } = step_to_decision(&mut state) else {
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
        let PendingDecision::YesNo { player } = step_to_decision(&mut state) else {
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

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(1)));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(1_000_000), Box::new(body)),
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
                    matches!(**first, OneShotEffect::Act(_)),
                    "the front item is this iteration's own body, not another Repeat layer"
                );
                match &**second {
                    OneShotEffect::Repeat(Count::Literal(n), _) => {
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

    /// `Batch` shell: in THIS task purely sequential-equivalent to `Repeat`
    /// ([CR#616.1g] aggregate-count semantics are a later pass's job) — two
    /// `Batch(2, GainLife(1))` iterations record as TWO separate
    /// `LifeGained` facts, exactly like `Repeat`, not one combined
    /// aggregate fact.
    #[test]
    fn batch_runs_sequentially_recording_one_fact_per_iteration() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(1)));
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(2), Box::new(body)),
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
            .filter(|e| matches!(e, GameEvent::LifeGained { .. }))
            .count();
        assert_eq!(
            life_gained_facts, 2,
            "shell Batch resolves sequentially — TWO separate LifeGained facts, not \
             one combined aggregate fact (the aggregate-count tier is a later pass's job)"
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

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(2)));
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(0), Box::new(body)),
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

    /// [CR#702.85,701.57] `RevealUntil` fizzles to a graceful no-op — the
    /// Reveal seam (`PlayerAction::Reveal`/`GameEvent::Revealed`) is
    /// genuinely unbuilt (see the doc comment on this arm in `run_effect`),
    /// so `body` never runs and nothing is scheduled — never a panic,
    /// matching the CRITICAL never-crash ruling.
    #[test]
    fn reveal_until_fizzles_to_a_no_op() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        let life0 = state.player(PlayerId(0)).life;
        let agenda_before = state.agenda.len();

        let effect = OneShotEffect::RevealUntil(deckmaste_core::RevealUntil {
            whose: Reference::You,
            matches: Predicate::creature(),
            body: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                Count::Literal(99),
            ))),
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

    /// [CR#608.2c,607.2] the resolution note store round-trips a NUMBER choice:
    /// `ChooseAndNote(Number)` surfaces a resolution-time number decision; the
    /// submitted value lands in `resolution_notes`; a LATER clause of the SAME
    /// resolution reads it back via `Count::Noted`. The two clauses run as
    /// separate `Sequentially` children — each carrying its OWN frame clone
    /// (`Sequentially` clones the frame per child up front) — so this asserts
    /// the note rides the resolution-scoped STORE, not the frame (the
    /// cloned-frame trap the store exists to avoid).
    #[test]
    fn choose_and_note_number_round_trips_through_the_store() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a) = bear_on_field();
        let key = deckmaste_core::Ident::from("n");
        let effect = OneShotEffect::Sequentially(vec![
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                key,
                deckmaste_core::NotedKind::Number,
            )),
            OneShotEffect::Act(deckmaste_core::Action::mill(
                deckmaste_core::Reference::You,
                Count::Noted(key),
            )),
        ]);
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        // The first child surfaced the number choice.
        let Some(PendingDecision::ChooseNoteNumber { player, key: pk }) = state.pending.clone()
        else {
            panic!("expected ChooseNoteNumber, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(0),
            "the effect's controller notes the number"
        );
        assert_eq!(pk, key);

        state
            .submit_decision(Decision::XValue(2))
            .expect("a non-negative note number is always legal");
        assert_eq!(
            state.resolution_notes.get(&key),
            Some(&crate::state::NotedValue::Number(2)),
            "the submitted value is stored under the note key"
        );

        // The SECOND child reads it back: Mill(Noted(key)) mills exactly 2.
        run_injected(&mut state);
        assert_eq!(
            state.zones.graveyards[0].len(),
            2,
            "Count::Noted read 2 from the store in the second child's frame clone"
        );
    }

    /// [CR#608.2c] scope: a resolution note lives ONLY within its resolution.
    /// A note written in one resolution is GONE when the next begins —
    /// `resolve_object` clears `resolution_notes` at the fresh-resolution
    /// boundary — so a `Count::Noted` read in the next resolution finds nothing
    /// and fizzles to 0 (never a stale value, never a panic). Mirrors
    /// `moved_chain_resets_when_a_resolution_begins`.
    #[test]
    fn resolution_notes_clear_when_a_fresh_resolution_begins() {
        use deckmaste_core::CardFace;
        use deckmaste_core::StatValue;

        let (mut state, _a) = bear_on_field();
        let key = deckmaste_core::Ident::from("n");
        state
            .resolution_notes
            .insert(key, crate::state::NotedValue::Number(5));

        // Drive a REAL resolution: mint a vanilla creature spell, push its
        // `StackEntry`, and resolve it.
        let card = Card::Normal(CardFace {
            name: "Test Bear".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            paid_costs: vec![],
            copy: false,
        });
        state.resolve_object(spell);

        assert!(
            state.resolution_notes.is_empty(),
            "a fresh resolution clears the note store ([CR#608.2c])"
        );
        // The consuming read fizzles to 0 (not the stale 5), never panics.
        let frame = frame_src(spell);
        assert_eq!(
            state.eval_count(&Count::Noted(key), &frame),
            0,
            "Count::Noted on a cleared note fizzles to 0"
        );
    }

    /// [CR#607.2a,608.2d] `ChooseAndNote(Objects)`: the picks are recorded into
    /// the fact-backed `noted` product group (live members), and
    /// `Selection::AmongNoted` reads them back. The grammar carries no
    /// narrowing predicate, so the chooser's domain is the battlefield-wide
    /// default (choose any number).
    #[test]
    fn choose_and_note_objects_writes_group_read_by_among_noted() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a, _b) = two_permanents_on_field();
        let key = deckmaste_core::Ident::from("chosen");
        let effect = OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
            key,
            deckmaste_core::NotedKind::Objects,
        ));
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        let Some(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(
            (min, max),
            (0, 2),
            "battlefield-wide domain, choose any number of the two permanents"
        );
        assert!(candidates.contains(&a));

        state
            .submit_decision(Decision::Chosen(vec![a]))
            .expect("a is a battlefield candidate");

        // The pick is recorded into the `noted` group, live.
        let group = &state.noted[&key];
        assert_eq!(group.len(), 1);
        assert_eq!(group[0].now, Some(a));

        // AmongNoted (unconstrained) reads the whole live group back.
        assert_eq!(
            state.eval_selection_set(
                &Selection::AmongNoted(key, deckmaste_core::Quantity::Range(None, None)),
                &frame,
            ),
            vec![a],
            "AmongNoted reads the noted objects back"
        );
    }

    /// [CR#607.2a,608.2d] a CONSTRAINING `AmongNoted` quantity ("destroy one of
    /// them") surfaces a `ChooseObjects` chooser over the noted group's LIVE
    /// members, honors the quantity's bounds, and binds the picks into the
    /// re-run body — exactly the chooser the unconstrained full-group read does
    /// not need.
    #[test]
    fn among_noted_constrained_quantity_surfaces_and_binds_chooser() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a, b) = two_permanents_on_field();
        let key = deckmaste_core::Ident::from("grp");
        // Seed the noted product group with both live permanents.
        let ma = crate::state::NotedMember {
            snapshot: crate::lki::LkiSnapshot::capture(&state, a),
            now: Some(a),
        };
        let mb = crate::state::NotedMember {
            snapshot: crate::lki::LkiSnapshot::capture(&state, b),
            now: Some(b),
        };
        state.noted.insert(key, vec![ma, mb]);

        // "Destroy exactly one of them" — a constraining AmongNoted quantity.
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::AmongNoted(
                key,
                deckmaste_core::Quantity::Range(Some(Count::Literal(1)), Some(Count::Literal(1))),
            )),
            effect: Box::new(OneShotEffect::Act(Action::destroy(Reference::It))),
        });
        let frame = frame_src(a);
        state.run_effect(effect, &frame);

        let Some(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(0),
            "the controller chooses (AmongNoted has no `by`)"
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

    /// [CR#607.2] note kinds without readers stay loud per-kind.
    #[test]
    #[should_panic(expected = "ChooseAndNote(Color)")]
    fn choose_and_note_color_is_a_loud_seam() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                deckmaste_core::Ident::from("c"),
                deckmaste_core::NotedKind::Color,
            )),
            &frame,
        );
    }

    #[test]
    fn choose_and_note_card_name_records_the_choice() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        let key = deckmaste_core::Ident::from("cn");
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                key,
                deckmaste_core::NotedKind::CardName,
            )),
            &frame,
        );
        run_injected(&mut state);
        assert!(matches!(
            state.pending,
            Some(PendingDecision::ChooseNoteCardName { key: pending, .. }) if pending == key
        ));
        state
            .submit_decision(Decision::CardName("Grizzly Bears".to_owned()))
            .unwrap();
        assert_eq!(
            state.resolution_notes.get(&key),
            Some(&crate::state::NotedValue::CardName(
                "Grizzly Bears".to_owned()
            ))
        );
    }

    #[test]
    #[should_panic(expected = "ChooseAndNote(Piles)")]
    fn choose_and_note_piles_is_a_loud_seam() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                deckmaste_core::Ident::from("p"),
                deckmaste_core::NotedKind::Piles,
            )),
            &frame,
        );
    }

    /// P0.W3 seam filled for the new decision kind: the mechanical strategy
    /// notes the minimum (0, the X=0 default, via the reused `Decision::XValue`
    /// answer), and the seat resolver names the deciding player.
    #[test]
    fn strategy_and_seat_cover_the_note_number_choice() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (state, _a) = bear_on_field();
        let pending = PendingDecision::ChooseNoteNumber {
            player: PlayerId(1),
            key: deckmaste_core::Ident::from("n"),
        };
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

        let effect: OneShotEffect = builtin()
            .macros
            .read_str(r"ExchangeControl(Target(0), Target(1))")
            .unwrap();
        let frame = frame_src_targets(mine, vec![mine, other]);
        state.run_effect(effect, &frame);

        // ONE batch of two ControlChanged facts.
        let front = state.agenda.front().cloned();
        let Some(WorkItem::Emit(Occurrence::Batch(events))) = front else {
            panic!("expected one ControlChanged batch, got {front:?}");
        };
        assert_eq!(events.len(), 2, "both halves in one occurrence");
        assert!(
            events
                .iter()
                .all(|e| matches!(e, GameEvent::ControlChanged { .. })),
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
        let effect: OneShotEffect = builtin()
            .macros
            .read_str(r"ExchangeControl(Target(0), Target(1))")
            .unwrap();
        let frame = frame_src_targets(mine, vec![mine, other]);
        state.run_effect(effect, &frame);
        assert!(
            !state.agenda.iter().any(|w| matches!(w, WorkItem::Emit(_))),
            "a same-controller exchange emits nothing ([CR#701.12b])"
        );
    }

    /// The `Fight` grammar macro's expansion ([CR#701.14a]): `Composite Fight`
    /// wrapping `If (both fighters are creatures on the battlefield —
    /// [CR#701.14b]) (Simultaneously [each deals its power to the OTHER, source
    /// = itself])`. Slots `x`/`y` are the two fighters. Mirrors
    /// `plugins/builtin/macros/effect/Fight.ron` (the guard's `Permanent` is
    /// spelled here as `InZone(Battlefield)`, an equivalent for the test).
    fn fight_effect(x: &Reference, y: &Reference) -> OneShotEffect {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::Condition;
        use deckmaste_core::Predicate;
        use deckmaste_core::Stat;
        use deckmaste_core::StatePredicate;
        let is_creature = |r: &Reference| {
            Condition::Matches(
                r.clone(),
                Predicate::And(vec![
                    Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature.name())),
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                ]),
            )
        };
        let half = |tgt: &Reference, src: &Reference| {
            OneShotEffect::Act(Action::DealDamage(
                src.clone(),
                Count::StatOf(src.clone(), Stat::Power),
                tgt.clone(),
            ))
        };
        OneShotEffect::Act(Action::Composite(
            deckmaste_core::KeywordAction::Fight(x.clone(), y.clone()),
            Box::new(OneShotEffect::If(deckmaste_core::If {
                condition: Condition::And(vec![is_creature(x), is_creature(y)]),
                then: Box::new(OneShotEffect::Simultaneously(vec![half(y, x), half(x, y)])),
                otherwise: None,
            })),
        ))
    }

    /// [CR#701.14a]: a fight — each creature deals damage equal to its power to
    /// the other, as ONE simultaneous batch of noncombat ([CR#701.14d]) damage
    /// facts; SBAs run after the whole batch. The `Composite Fight` fires its
    /// keyword-action fact once the guarded body acts.
    #[test]
    fn fight_deals_each_others_power_as_one_noncombat_batch() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
            &frame,
        );
        // Both packets land as ONE applied batch occurrence.
        let batch = drain_progress(&mut state, 30)
            .into_iter()
            .find_map(|p| match p {
                Progress::Applied(Occurrence::Batch(evs))
                    if evs
                        .iter()
                        .all(|e| matches!(e, GameEvent::DamageDealt { .. })) =>
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
                .all(|e| matches!(e, GameEvent::DamageDealt { combat: false, .. })),
            "fight damage is noncombat damage ([CR#701.14d]), got {batch:?}"
        );
        assert_eq!(state.objects.obj(a).total_damage(), 2, "a took b's power");
        assert_eq!(state.objects.obj(b).total_damage(), 2, "b took a's power");
        assert!(
            logged(&state, |e| matches!(e, GameEvent::Act { .. })),
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
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::DamageDealt { .. })),
            "neither creature deals damage ([CR#701.14b])"
        );
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Act { .. })),
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
        let frame = frame_src_targets(a, vec![a, a]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
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
                GameEvent::DamageDealt {
                    amount: 4,
                    combat: false,
                    ..
                }
            )),
            "one coalesced damage instance of twice its power"
        );
    }

    /// [CR#611.2]/[CR#611.2c]: `OneShotEffect::Continuously(Modify(Matching(...), ...),
    /// UntilEndOfTurn)` — the resolve arm pushes one `ContinuousEffect` with a
    /// `ScopeResolved::Floating` scope and the right duration/changes.
    #[test]
    fn continuously_matching_registers_floating_scope() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::Selection;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        assert!(state.continuous.is_empty(), "no effects before resolve");

        let filter = Predicate::creature();
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Each(
                Selection::SelectAll(filter.clone()),
                Box::new(StaticEffect::Modify(
                    Reference::It,
                    Modification::Power(NumericOp::Up(Count::Literal(1))),
                )),
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

    /// [CR#611.2c]: `OneShotEffect::Continuously(Modify(Of(This), ...), ...)` locks
    /// the id at creation — `ScopeResolved::Locked(vec![src])`.
    #[test]
    fn continuously_of_this_registers_locked_scope() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Modify(
                Reference::This,
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

    /// [CR#611.2c]: a granted `Deontic` ("target creature can't block this
    /// turn") mints a static ROW, not a layer change — its subject `Target(0)`
    /// resolves to the locked object at mint and is rewritten to `Ref(It)`.
    #[test]
    fn continuously_deontic_mints_locked_row_rewriting_subject_to_it() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        // The bear is the lone announced target — the restriction's subject.
        let frame = frame_src_targets(src, vec![src]);
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block {
                by: Predicate::Ref(Reference::Target(0)),
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
            "subject Target(0) locked to the bear at mint"
        );
        assert!(
            ce.changes.is_empty(),
            "a Deontic grant carries no layer changes"
        );
        assert!(
            matches!(
                &ce.rows[..],
                [StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block { by, .. }))]
                    if *by == Predicate::Ref(Reference::It)
            ),
            "subject rewritten to Ref(It), got {:?}",
            ce.rows
        );
    }

    /// A granted `CostModifier` is self-filtered (empty lock) and lands as a
    /// row.
    #[test]
    fn continuously_cost_modifier_mints_self_filtered_row() {
        use deckmaste_core::Continuously;
        use deckmaste_core::CostChange;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::CostModifier {
                of: Predicate::creature(),
                change: CostChange::Increase(vec![]),
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
        assert!(matches!(&ce.rows[..], [StaticEffect::CostModifier { .. }]));
    }

    /// A granted `CantHappen` is self-filtered and lands as a row.
    #[test]
    fn continuously_cant_happen_mints_self_filtered_row() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::EventFilter;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let filter = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::CantHappen(filter)),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
        let ce = &state.continuous[0];
        assert!(ce.changes.is_empty());
        assert!(matches!(&ce.rows[..], [StaticEffect::CantHappen(_)]));
    }

    /// A granted `Prevention` is LOUD — engine-prevention owns the machinery.
    #[test]
    #[should_panic(expected = "engine-prevention")]
    fn continuously_prevention_is_loud() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Prevention;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Prevention(Box::new(
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
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        // "for as long as an object matching not-anything exists" — always false.
        let cond = Condition::Exists(Predicate::Not(Box::new(Predicate::Any)));
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Modify(
                Reference::This,
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
        let cond = Condition::Exists(Predicate::Not(Box::new(Predicate::Any)));
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::ForAsLongAs(cond),
            origin: Some(Box::new(frame_src(src))),
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
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let seq = OneShotEffect::Sequentially(vec![
            OneShotEffect::Act(Action::by_you(PlayerAction::GainLife(Count::Literal(1)))),
            OneShotEffect::Until(
                Duration::ForThisEvent,
                vec![StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Regenerate {
                        by: Predicate::Any,
                        on: Predicate::Ref(Reference::This),
                    },
                ))],
            ),
        ]);
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
    /// authoring mistake — it fizzles (dropped), never mints or panics.
    #[test]
    fn for_this_event_rider_without_preceding_sibling_fizzles() {
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let seq = OneShotEffect::Sequentially(vec![OneShotEffect::Until(
            Duration::ForThisEvent,
            vec![StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Regenerate {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::This),
                },
            ))],
        )]);
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
        use deckmaste_core::Reference;
        use deckmaste_core::Replacement;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        state.create_shield(
            Replacement::Skip {
                what: PhaseStep::Beginning(BeginningStep::Untap),
            },
            &Reference::This,
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
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::UntilEvent(filter),
            origin: Some(Box::new(frame_src(src))),
            is_cda: false,
        });
        // A non-matching fact (a life gain) leaves it in place.
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::LifeGained {
            player: PlayerId(0),
            amount: 1,
        }));
        assert_eq!(
            state.continuous.len(),
            1,
            "survives a non-matching occurrence"
        );
        // The awaited damage fact ends it.
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::DamageDealt {
            source: src,
            target: src,
            amount: 1,
            combat: false,
        }));
        assert!(
            state.continuous.is_empty(),
            "removed once the awaited event happens"
        );
    }

    /// [CR#601.2d]: `Distribute` splits the amount across the binder's group and
    /// binds each element's `Allotment` share in scope for its body — divided
    /// damage deals the split shares, summing to the total, ≥1 to each.
    #[test]
    fn divide_among_splits_amount_and_binds_allotment() {
        use deckmaste_core::Distribute;
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(a);
        let effect = OneShotEffect::Distribute(Distribute {
            amount: Count::Literal(3),
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]))),
            body: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Allotment,
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

    /// [CR#601.2d,120.3]: dividing damage among a MIXED group — a creature AND a
    /// player (Arc Lightning's `CreatureOrPlayer`) — must not panic. The player
    /// element is a zoneless proxy with no LKI snapshot, so its `It` binding is
    /// a player; the body's `It` reads it kind-poly: the creature takes its
    /// share as marked damage, the player loses life by its share.
    #[test]
    fn divide_among_handles_a_player_element_without_panicking() {
        use deckmaste_core::Distribute;

        let (mut state, creature) = bear_on_field();
        let player = state.players[1].object;
        let life0 = state.player(PlayerId(1)).life;
        // The group is pre-bound as the many-binder `That` (Arc Lightning's
        // chosen `CreatureOrPlayer` set, bound by an enclosing `With`);
        // `split_evenly(3, 2)` is [2, 1], so the first-listed creature takes 2,
        // the player takes 1.
        let mut frame = frame_src(creature);
        frame.anaphora.that = Some(ThatBinding {
            cardinality: Cardinality::Many,
            kind: RefKind::Object,
            group: vec![creature, player],
        });
        let effect = OneShotEffect::Distribute(Distribute {
            amount: Count::Literal(3),
            binder: Binder::Existing(Selection::They),
            body: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Allotment,
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

    /// [CR#608.2,120.3]: `Each` over the players binds each zoneless player
    /// proxy as the iteration anaphor `It` — the body's `DealDamage(This, 1,
    /// It)` resolves to the player and each loses 1 life, with no panic on
    /// the snapshotless element.
    #[test]
    fn foreach_over_players_binds_each_player_as_it() {
        use deckmaste_core::Each;

        let (mut state, bear) = bear_on_field();
        let life0 = [
            state.player(PlayerId(0)).life,
            state.player(PlayerId(1)).life,
        ];
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(Predicate::Kind(ObjectKind::Player))),
                effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                    Reference::It,
                    Count::Literal(1),
                ))),
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(state.player(PlayerId(0)).life, life0[0] - 1);
        assert_eq!(state.player(PlayerId(1)).life, life0[1] - 1);
    }

    /// Ticket (the first-of-many fix): a many-binder iterated by `Each` acts on
    /// EVERY element. The Brainstorm shape `Each(Choose(2, …), Destroy(It))`
    /// chooses two creatures and destroys BOTH — the dropped-cardinality bug
    /// (which acted on only the first) is unrepresentable now that the binder
    /// surfaces its choice and `Each` iterates the whole group ([CR#608.2]).
    #[test]
    fn each_over_choose_many_acts_on_all_elements() {
        use deckmaste_core::Quantity;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(deckmaste_core::Each {
                binder: Binder::Choose {
                    quantity: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                    filter: creatures,
                    by: Reference::You,
                },
                effect: Box::new(OneShotEffect::Act(Action::destroy(Reference::It))),
            }),
            &frame,
        );
        // The many-binder surfaces a choice for the WHOLE group before iterating.
        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects {
            min,
            max,
            candidates,
            ..
        }) = state.step()
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

    /// Ticket: a nested `Each` CLEARS the outer `Distribute` allotment (the
    /// Idris allotment-clearing `bindIt`), so an outer per-element share cannot
    /// leak into the inner loop ([CR#601.2d]). Once the inner `Each` rebinds
    /// `It`, the share is gone, and the inner body's `Count::Allotment` read
    /// has nothing in scope and is rejected — proving threading is
    /// add-AND-clear.
    #[test]
    #[should_panic(expected = "Allotment outside a Distribute body")]
    fn nested_each_clears_outer_divide_among_allotment() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src(a);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let effect = OneShotEffect::Distribute(deckmaste_core::Distribute {
            amount: Count::Literal(2),
            binder: Binder::Existing(Selection::SelectAll(creatures.clone())),
            // The outer share is in scope here, but the inner `Each` rebinds `It`
            // per inner element and clears it before the body runs.
            body: Box::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                    Reference::It,
                    Count::Allotment,
                ))),
            })),
        });
        state.run_effect(effect, &frame);
        // Driving the inner `Each` reads the (now-cleared) `Allotment` and panics.
        run_injected(&mut state);
    }

    /// [CR#608.2c]: `OneShotEffect::If` evaluates its condition WHEN it resolves and
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
            OneShotEffect::Act(Action::By(
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
            OneShotEffect::If(If {
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
            OneShotEffect::If(If {
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
            OneShotEffect::If(If {
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

    /// [CR#608.2]: `OneShotEffect::Each` evaluates its binder once at resolution and
    /// runs the inner effect once per matched object, binding each iterated
    /// object as the anaphor `It` (a per-iteration `frame.anaphora.it`).
    /// Proven via `Destroy(It)` over the battlefield creatures: every
    /// creature dies, which can only happen if each iteration's `It`
    /// resolves to that iteration's object.
    #[test]
    fn run_effect_foreach_binds_each_match_as_it() {
        use deckmaste_core::Each;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Box::new(OneShotEffect::Act(Action::destroy(Reference::It))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 80);
        assert!(
            !state.zones.battlefield.contains(&bear) && !state.zones.battlefield.contains(&theirs),
            "every iterated creature is destroyed via its It binding"
        );
    }

    /// [CR#608.2]: `OneShotEffect::Each` runs the inner effect once per match — a
    /// non-binding body (gain 1 life) over two creatures gains 2 life.
    #[test]
    fn run_effect_foreach_runs_once_per_match() {
        use deckmaste_core::Each;

        let (mut state, bear) = bear_on_field();
        let _theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        let life0 = state.player(PlayerId(0)).life;
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Box::new(OneShotEffect::Act(Action::By(
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

    /// [CR#118.12]: `OneShotEffect::May` surfaces a yes/no to the controller. Yes runs
    /// `effect` then `if_did`; no runs `if_not` (nothing when absent). Driven
    /// via `GainLife` so each branch reads as a clean life delta.
    #[test]
    fn run_effect_may_branches_on_the_answer() {
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let gain = |n| {
            OneShotEffect::Act(Action::By(
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
        state.run_effect(OneShotEffect::May(may()), &frame);
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
        state.run_effect(OneShotEffect::May(may()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "no → if_not");

        // no + no if_not → nothing.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::May(May {
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

    /// [CR#700.2]: `OneShotEffect::Modal` surfaces `ChooseModes`; the chosen modes'
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
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            )),
            cost: None,
        };
        let modes = || vec![gain_mode(3), gain_mode(5), gain_mode(7)];
        let spec = |count, up_to| ChooseSpec {
            count: deckmaste_core::Quantity::Range(
                Some(Count::Literal(count)),
                Some(Count::Literal(count)),
            ),
            up_to,
            repeats: false,
            chooser: Reference::You,
            rider: None,
        };
        let p0 = PlayerId(0);

        // choose one → ChooseModes(options 3, [1,1]); pick mode 1 → +5.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::Modal(Modal {
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
            OneShotEffect::Modal(Modal {
                choose: spec(2, false),
                modes: modes(),
            }),
            &frame,
        );
        state.submit_decision(Decision::Modes(vec![0, 2])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "both chosen modes run");
    }

    /// [CR#118.12a]: `OneShotEffect::MustPay` — the Mana Leak punisher over the full
    /// `Cost` (the English "unless" order is the `Unless` macro over this
    /// node). Pay → the cost runs and `or_else` is skipped; decline →
    /// `or_else` runs.
    #[test]
    fn run_effect_must_pay_pays_or_suffers_or_else() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::MustPay;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let must_pay = || MustPay {
            actor: Reference::You,
            cost: Cost(vec![CostComponent::do_(PlayerAction::LoseLife(
                Count::Literal(2),
            ))]),
            or_else: Box::new(OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(10)),
            ))),
        };

        // "I'll pay" → lose 2, the punisher (gain 10) skipped.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MustPay(must_pay()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 - 2,
            "pay → cost paid, or_else skipped"
        );

        // "won't pay" → the punisher runs (gain 10).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MustPay(must_pay()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "decline → or_else runs");
    }

    /// [CR#603,608]: `OneShotEffect::MayPay` — a resolution-time kicker. Pay → the cost
    /// runs THEN `and_then`; decline → `or_else`. The PAID branch running a
    /// follow-up effect is what `Unless`/`MustPay` cannot express.
    #[test]
    fn run_effect_may_pay_runs_and_then_on_pay_or_else_on_decline() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::MayPay;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let may_pay = || MayPay {
            actor: Reference::You,
            cost: Cost(vec![CostComponent::do_(PlayerAction::LoseLife(
                Count::Literal(2),
            ))]),
            and_then: Box::new(OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(10)),
            ))),
            or_else: Some(Box::new(OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(1)),
            )))),
        };

        // "I'll pay" → lose 2 THEN gain 10 (net +8) — the kicker fires.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MayPay(may_pay()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 8,
            "pay → cost (−2) then and_then (+10)"
        );

        // "won't pay" → or_else runs (gain 1).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MayPay(may_pay()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "decline → or_else runs");
    }

    /// [CR#601.2f,118.8]: `OneShotEffect::AdditionalCost` (nested, resolution-time) —
    /// the cost is PAID (the source is sacrificed) and the body then reads the
    /// paid object through the event reference `EventObject`. The bear is
    /// sacrificed carrying three +1/+1 counters; the body gains life equal to
    /// the SACRIFICED creature's counter count, read via its last-known
    /// snapshot ([CR#603.10a]) — proving the paid object is bound for the
    /// body even after it has left the battlefield.
    #[test]
    fn run_effect_additional_cost_binds_paid_object_for_body() {
        use deckmaste_core::AdditionalCost;
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;

        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let p0 = PlayerId(0);
        let life0 = state.player(p0).life;
        let frame = frame_src(bear);

        state.run_effect(
            OneShotEffect::AdditionalCost(AdditionalCost {
                pay: Cost(vec![CostComponent::do_(PlayerAction::Sacrifice(
                    Reference::This,
                ))]),
                body: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                    Count::CounterCount(Box::new(Reference::EventObject), "P1P1Counter".into()),
                ))),
            }),
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
            "the body read the sacrificed object's counters via EventObject"
        );
    }

    // --- Ascend (spell form) e2e ([CR#702.131a]) -------------------------------
    //
    // The spell form of Ascend folds into `Sequentially([If(<gate>,
    // GetDesignation), If(Is(You,Designated), Draw(3), otherwise: Draw(2))])`
    // (Task 7). The `OneShotEffect::If` interpreter is now live (see the
    // `OneShotEffect::If` arm in `run_effect` — it evaluates `condition_holds`,
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

        Condition::And(vec![
            Condition::Compare(
                Count::CountOf(Countable::Objects(Box::new(Predicate::And(vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                        Reference::You,
                    )))),
                ])))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Box::new(Condition::Matches(
                Reference::You,
                Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
            ))),
        ])
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
    fn secrets_effect() -> OneShotEffect {
        use deckmaste_core::Condition;
        use deckmaste_core::If;

        OneShotEffect::Sequentially(vec![
            OneShotEffect::If(If {
                condition: ascend_gate(),
                then: Box::new(OneShotEffect::act_by_you(PlayerAction::GetDesignation(
                    "CitysBlessing".into(),
                ))),
                otherwise: None,
            }),
            OneShotEffect::If(If {
                condition: Condition::Matches(
                    Reference::You,
                    Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
                ),
                then: Box::new(OneShotEffect::Act(Action::draw(
                    Reference::You,
                    Count::Literal(3),
                ))),
                otherwise: Some(Box::new(OneShotEffect::Act(Action::draw(
                    Reference::You,
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
            abilities: vec![Ability::Spell(SpellAbility {
                ability_word: None,
                effect: secrets_effect(),
            })],
            ..CardFace::default()
        });
        let spell_card_id = state.cards.push(Arc::new(spell_card), p0);
        let spell = state
            .objects
            .mint(ObjectSource::Card(spell_card_id), p0, Some(Zone::Stack));
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: p0,
            targets: vec![],
            x: None,
            copy: false,
        });

        (state, p0, library_before)
    }

    /// Pump up to `n` steps, collecting every `(target, amount)` of the
    /// `DamageDealt` events applied along the way — the per-element emissions a
    /// `Each(.., DealDamage(This, .., It))` produces (a verb deals to a
    /// single `Reference`, so the spread is the iterator).
    fn collect_damage_dealt(state: &mut GameState, n: usize) -> Vec<(ObjectId, u32)> {
        let mut got = Vec::new();
        for p in drain_progress(state, n) {
            if let Progress::Applied(occ) = p {
                let events = match occ {
                    Occurrence::Single(e) => vec![e],
                    Occurrence::Batch(es) => es,
                };
                for e in events {
                    if let GameEvent::DamageDealt { target, amount, .. } = e {
                        got.push((target, amount));
                    }
                }
            }
        }
        got
    }

    /// Isolation guard: proves the spell-form fixture is sound independent of
    /// the `OneShotEffect::If` interpreter — the gate reads true at ten / false
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
            OneShotEffect::Act(Action::draw(Reference::You, Count::Literal(3))),
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

    /// `TopOfLibrary` returns the top N cards in order (front of library =
    /// top); `OneShotEffect::With` binds them so `Selection::That` resolves to
    /// the same ordered vec inside the body frame.
    #[test]
    fn with_binds_those_and_top_of_library_is_ordered() {
        use deckmaste_core::CardFace;
        use deckmaste_core::With;

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
        let frame = Frame::bare(source, p0);

        // TopOfLibrary(count:2, whose:You) → top two in order.
        let top2 = state.eval_selection_set(
            &Selection::TopOfLibrary {
                count: Count::Literal(2),
                whose: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top2, vec![a, b], "top 2 are a then b, top→down");

        // With binds them as the many-binder `That`; the body frame sees the same
        // ordered group.
        let mut bound = frame.clone();
        bound.anaphora.that = Some(ThatBinding {
            cardinality: Cardinality::Many,
            kind: RefKind::Object,
            group: top2.clone(),
        });
        assert_eq!(
            state.eval_selection_set(&Selection::They, &bound),
            vec![a, b],
            "Selection::They inside a With frame returns the bound group in order"
        );

        // OneShotEffect::With end-to-end: run_effect schedules a body that reads
        // Selection::That and verifies the binding survives round-trip through
        // the agenda.
        // We check indirectly by scheduling a no-op body and confirming no panic.
        state.run_effect(
            OneShotEffect::With(With {
                binder: deckmaste_core::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::You,
                }),
                body: Box::new(OneShotEffect::Sequentially(vec![])),
            }),
            &frame,
        );
        // Drain the agenda — the empty Sequentially body completes without a
        // decision, proving With schedules correctly.
        for _ in 0..10 {
            state.step();
        }
    }

    /// `BottomOfLibrary` mirrors `TopOfLibrary` from the other end (bottom→up
    /// order), and `Union` concatenates member groups order-preserved with an
    /// object in more than one member appearing once (the Idris `Union` /
    /// `BottomOfLibrary` constructors).
    #[test]
    fn bottom_of_library_and_union_resolve_as_groups() {
        use deckmaste_core::CardFace;

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
        let frame = Frame::bare(source, p0);

        // BottomOfLibrary(count:2) → the bottom two, nearest-to-bottom first.
        let bottom2 = state.eval_selection_set(
            &Selection::BottomOfLibrary {
                count: Count::Literal(2),
                whose: deckmaste_core::Reference::You,
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
                    whose: deckmaste_core::Reference::You,
                },
                Selection::BottomOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::You,
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
        use deckmaste_core::CardFace;

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
        let frame = Frame::bare(source, p0);

        let top1 = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(1),
                of: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top1, vec![c], "the top card is the most recently put one");

        let top2 = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(2),
                of: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top2, vec![c, b], "top 2, top→down");

        // `of` resolving to a non-player (a card, not a player proxy) fizzles
        // to the empty group rather than panicking.
        let bad = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(1),
                of: deckmaste_core::Reference::This,
            },
            &Frame::bare(a, p0),
        );
        assert_eq!(
            bad,
            Vec::<ObjectId>::new(),
            "non-player `of` fizzles, never panics"
        );
    }
}
