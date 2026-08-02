//! `run_effect`: walk a `OneShotEffect` AST — combinators, binders, riders,
//! continuous-effect minting — lowering each node to agenda work.

use std::sync::Arc;

use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Destination;
use deckmaste_core::Modification;
use deckmaste_core::Normalize;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::StaticEffect;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::action::composite_body_group;
use super::action::composite_body_whose;
use super::action::composite_move_src;
use super::deref_quantity;
use super::occurrence_of;
use super::peel_binder;
use super::peel_effect;
use super::top_targets;
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
        // [CR#614.3]: the protected permanent is the `That` the enclosing `With`
        // bound — the shield freezes THAT resolved identity at creation
        // (`floating_watches` then matches on this frozen subject). An unbound /
        // vanished `That` (no `With`, degenerate reference) fizzles the mint —
        // never a shield with a null subject, never a panic ([CR#701.8a]).
        let id = self.eval_reference(&Reference::That(deckmaste_core::Sort::Card), frame);
        if self.objects.get(id).is_none() {
            return;
        }
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
        // The carrier watcher anchors a `Ref(This)`/`Ref(You)` inside the
        // binder's filter (e.g. `InHand(who)`'s `Owner(Ref(who))`,
        // [CR#701.9b]) — the frameless `candidates()` shorthand panics on
        // one ([`crate::target::matches_with`]'s frameless `Ref` arms), so a
        // chooser's filter reads through the same framed
        // `candidates_with`/`frame_watcher` pair `Selection::SelectAll`
        // already uses.
        let watcher = Some(self.frame_watcher(frame));
        match peel_binder(binder) {
            Binder::ChooseOne { filter, by } => Some((
                self.acting_player(by, frame),
                crate::target::candidates_with(self, filter, watcher),
                1,
                1,
            )),
            Binder::Choose {
                quantity,
                filter,
                by,
            } => {
                let candidates = crate::target::candidates_with(self, filter, watcher);
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

    /// [CR#701.9b] "at random": when `binder` (after macro-peeling) is
    /// `Existing(Selection::Random(quantity, filter))` and no pick is bound
    /// yet, sample uniformly via the seeded rng right here — no decision is
    /// surfaced (there is no choice, unlike `Choose`/`ChooseOne`) — and bind
    /// the picks into `frame.anaphora.chosen`, exactly where a
    /// `ChooseObjects` answer would leave them, so the shared
    /// `Existing`→`eval_selection_set` read finds a bound group either way
    /// (the retired `DiscardRandom` work item's `rand::seq::index::sample`
    /// logic, relocated onto the general binder spine so any `Random`
    /// selection — not just discard's — now actually samples). Any other
    /// binder shape, or an already-bound `chosen`, returns `frame` cloned
    /// as-is. Called by `Each`/`With`/`Distribute` before `resolve_binder`.
    fn sample_random_binder(&mut self, binder: &deckmaste_core::Binder, frame: &Frame) -> Frame {
        use deckmaste_core::Binder;
        use deckmaste_core::Selection;
        let mut next = frame.clone();
        if next.anaphora.chosen.is_some() {
            return next;
        }
        if let Binder::Existing(Selection::Random(quantity, filter)) = peel_binder(binder) {
            let watcher = Some(self.frame_watcher(frame));
            let candidates = crate::target::candidates_with(self, filter, watcher);
            let (_, max) = self.choice_bounds(quantity, candidates.len(), frame);
            let n = usize::try_from(max).expect("sample count fits usize");
            let idx = rand::seq::index::sample(&mut self.rng, candidates.len(), n);
            next.anaphora.chosen = Some(idx.into_iter().map(|i| candidates[i]).collect());
        }
        next
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
    /// caster (`Cast`'s own agent slot) and the referent object it would
    /// cast — so the `May` arm can gate the "yes" offer on
    /// [`can_cast_as_effect`](Self::can_cast_as_effect). `None` for any other
    /// `May` body (the ordinary "you may [do]", offered unconditionally). The
    /// referent id may be null/stale; the gate treats that as uncastable.
    fn may_cast_referent(
        &self,
        may: &deckmaste_core::May,
        frame: &Frame,
    ) -> Option<(
        crate::player::PlayerId,
        crate::object::ObjectId,
        Option<deckmaste_core::Cost>,
    )> {
        match peel_effect(&may.effect) {
            OneShotEffect::Act(Action::Cast(actor, what, for_cost)) => Some((
                self.acting_player(actor, frame),
                self.eval_reference(what, frame),
                for_cost.clone(),
            )),
            _ => None,
        }
    }

    /// [CR#608.2d]: whether `payer` (resolved from `who`) can make a legal
    /// "yes" choice for a `May(Pay(cost))` toll — the no-creatures/Standstill
    /// exiled-source gate the pre-collapse `MustPay` never kept ("v1 does NOT
    /// gate the offer on affordability" no longer holds once the offer can be
    /// forced to `if_not`). A lightweight per-component check, not a
    /// full-fidelity payability oracle (that's T4's `Agency::CostPayment`
    /// scope): verb costs (`Do(...)`, `{T}`/`{Q}`) reuse the activation-cost
    /// gate ([`Self::can_pay_verbs`]) via the same
    /// [`crate::decide::unless_cost_action`] rendering the payment walk
    /// itself uses. `Mana` is deliberately NOT gated here — unlike a verb
    /// cost, a mana toll can be answered by activating mana abilities in
    /// response to the offer ([CR#605.3a]), so an empty pool right now does
    /// not mean "can't pay"; the pre-collapse `MustPay`/`MayPay` always
    /// offered on a mana toll regardless of the live pool, and this keeps
    /// that. A cost-side `With`/`TapTotal`/nested `Cost` has no cheap
    /// feasibility check here either and is optimistically treated as
    /// payable — the runner is trusted not to answer "yes" when it can't
    /// actually follow through, same as the pre-collapse behavior.
    fn can_pay_may_cost(
        &self,
        cost: &[deckmaste_core::CostComponent],
        who: &Reference,
        payer: crate::player::PlayerId,
        frame: &Frame,
    ) -> bool {
        use deckmaste_core::CostComponent;
        let mut verbs = Vec::new();
        for component in cost {
            if let other @ (CostComponent::Do(_) | CostComponent::Tap | CostComponent::Untap) =
                component
            {
                verbs.push(crate::decide::unless_cost_action(other, who));
            }
        }
        self.can_pay_verbs(payer, &verbs, frame.source)
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
                    duration,
                    one_shot,
                } = action
                {
                    self.create_shield(
                        Arc::unwrap_or_clone(replacement),
                        duration,
                        one_shot,
                        frame,
                    );
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
                for child in children.iter().cloned() {
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
                for child in children.iter() {
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
                        Modification::flatten(std::slice::from_ref(change)),
                        vec![],
                    ),
                    // The distributor shape ("target creature and all creatures
                    // it shares a color with get +1/+1", or a plain anthem)
                    // stays `Floating(f)`: the filter is NOT expanded to objects
                    // here.
                    StaticEffect::Each(Selection::SelectAll(f), inner) => match inner.as_ref() {
                        StaticEffect::Modify(Reference::It, change) => (
                            ScopeResolved::Floating(f.clone()),
                            Modification::flatten(std::slice::from_ref(change)),
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
                            [].into(),
                            vec![StaticEffect::Deontic(locked)],
                        )
                    }
                    // Self-filtered rows: `of` / the `CantHappen` filter carry
                    // their own subject predicate, so the scope is unused (an
                    // empty lock). `CostModifier` is wired into
                    // `cost_modifier_rows` (cast.rs), `CantHappen` into
                    // `cant_event` (replace_registry.rs).
                    row @ (StaticEffect::CostModifier { .. } | StaticEffect::CantHappen(_)) => {
                        (ScopeResolved::Locked(vec![]), [].into(), vec![row.clone()])
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
                    // Becomes-a-copy is a deferred-exec seam: its copiable-value install is
                    // owned by engine-layers-1-copy-facedown-text (base_values). Until that
                    // lands a resolved becomes-a-copy FIZZLES — installs nothing — per the
                    // never-panic contract, matching the early-return fizzles above (473/555).
                    StaticEffect::BecomesCopy(..) => return,
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
                    changes: changes.to_vec(),
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
                    .iter()
                    .map(|part| WorkItem::RunEffect {
                        effect: Arc::new(OneShotEffect::Continuously(
                            deckmaste_core::Continuously {
                                effect: Arc::new(part.clone()),
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
            OneShotEffect::Label(label) => {
                self.run_effect(Arc::unwrap_or_clone(label.effect), frame);
            }
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
                    self.run_effect(Arc::unwrap_or_clone(if_effect.then), frame);
                } else if let Some(otherwise) = if_effect.otherwise {
                    self.run_effect(Arc::unwrap_or_clone(otherwise), frame);
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
                let sampled = self.sample_random_binder(&each.binder, frame);
                let frame = &sampled;
                if let Some((chooser, candidates, min, max)) =
                    self.binder_choice(&each.binder, frame)
                {
                    self.pending = Some(crate::decide::PendingDecision::ChooseObjects(
                        crate::decide::pending::ChooseObjects {
                            player: chooser,
                            candidates,
                            min,
                            max,
                        },
                    ));
                    self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
                        effect: Arc::new(OneShotEffect::Each(each)),
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
                let sampled = self.sample_random_binder(&with.binder, frame);
                let frame = &sampled;
                if let Some((chooser, candidates, min, max)) =
                    self.binder_choice(&with.binder, frame)
                {
                    self.pending = Some(crate::decide::PendingDecision::ChooseObjects(
                        crate::decide::pending::ChooseObjects {
                            player: chooser,
                            candidates,
                            min,
                            max,
                        },
                    ));
                    self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
                        effect: Arc::new(OneShotEffect::With(with)),
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
                let sampled = self.sample_random_binder(&divide.binder, frame);
                let frame = &sampled;
                if let Some((chooser, candidates, min, max)) =
                    self.binder_choice(&divide.binder, frame)
                {
                    self.pending = Some(crate::decide::PendingDecision::ChooseObjects(
                        crate::decide::pending::ChooseObjects {
                            player: chooser,
                            candidates,
                            min,
                            max,
                        },
                    ));
                    self.choice = Some(crate::state::ChoiceContinuation::BindChoice {
                        effect: Arc::new(OneShotEffect::Distribute(divide)),
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
                // [CR#118.12a,118.12,608.2d]: the collapsed `MayPay`/`MustPay`
                // shape — `effect` is `Pay(cost)`, so `if_did`/`if_not` invoke
                // cost semantics rather than plain optionality. Branch on
                // whether the payer CHOSE to pay, "regardless of what events
                // actually occurred" ([CR#118.12], the Dermoplasm clause) —
                // decided by the YesNo answer, never by inspecting what the
                // payment sub-effects produced. Inability to make a legal
                // "yes" choice forces `if_not` directly ([CR#608.2d]'s
                // no-creatures/Standstill-exiled-source cases), mirroring the
                // Cast-referent gate below. A branchless `May` (`if_did`/
                // `if_not` both `None`) rides the exact same path — one node
                // serves both, so there is no separate "plain Pay" arm.
                if let OneShotEffect::Act(Action::Pay(cost)) = peel_effect(&may.effect) {
                    let payer = self.acting_player(&may.who, frame);
                    // Normalize at this boundary: read is faithful, so a
                    // macro-spliced cost arrives lumpy (a nested
                    // `CostComponent::Cost`); splice it flat and price any
                    // `{X}` (a ward toll's where_x) before the payment walk.
                    let cost = cost.clone().normalize().0;
                    let cost = self.price_variable_cost(cost.to_vec(), frame);
                    if !self.can_pay_may_cost(&cost, &may.who, payer, frame) {
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
                    self.pending = Some(crate::decide::PendingDecision::YesNo(
                        crate::decide::pending::YesNo { player: payer },
                    ));
                    self.choice = Some(crate::state::ChoiceContinuation::MayPayCost {
                        who: may.who,
                        cost,
                        if_did: may.if_did,
                        if_not: may.if_not,
                        frame: frame.clone(),
                    });
                    return;
                }
                // [CR#608.2g]: "you may cast that card. If you don't, …" — the
                // "yes" (cast) branch is offered ONLY when a legal, payable cast
                // of the referent exists; otherwise the offer is empty and the
                // `if_not` branch runs (faithful even when the card is
                // uncastable — a land, an unaffordable cost, no legal target).
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
                self.pending = Some(crate::decide::PendingDecision::YesNo(
                    crate::decide::pending::YesNo {
                        player: frame.controller,
                    },
                ));
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
                self.pending = Some(crate::decide::PendingDecision::ChooseModes(
                    crate::decide::pending::ChooseModes {
                        player: frame.controller,
                        options,
                        min,
                        max,
                        repeats: modal.choose.repeats,
                    },
                ));
                self.choice = Some(crate::state::ChoiceContinuation::Modal {
                    modes: modal.modes.to_vec(),
                    frame: frame.clone(),
                });
            }
            // [CR#115.1,601.2c]: a target-scoping wrapper. Targets were chosen
            // at announcement and already live in `frame.anaphora.targets`;
            // this node is otherwise transparent — descend into the inner
            // effect, exactly like `Expanded`. The body reads announced
            // targets by position via `Reference::Target(n)`.
            OneShotEffect::Targeted(te) => {
                self.run_effect(Arc::unwrap_or_clone(te.effect), frame);
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
                            effect: Arc::new(OneShotEffect::Repeat(Count::Literal(n - 1), body)),
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
            OneShotEffect::Batch(count, body) => {
                let n = self.eval_count(&count, frame);
                if n == 0 {
                    // Clean no-op — mirrors `Repeat`.
                } else if let Some(unit) = batch_act_unit(peel_effect(&body)) {
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
                            inherited: frame.anaphora.inherited_replacements.clone(),
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
                            effect: Arc::new(OneShotEffect::Batch(Count::Literal(n - 1), body)),
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
            OneShotEffect::RevealUntil(_) => {}
            OneShotEffect::AdditionalCost(ac) => {
                let cost = ac.pay.normalize().0;
                // [CR#608.2k,118.12]: the payment's DECISION RECORD, captured
                // when the payer chooses to pay (here, at schedule time —
                // the cost hasn't run yet, but a directly-resolvable or
                // already-`That`-bound reference is already fixed) — object
                // and actor only (the payer, statically known: [CR#601.2b]
                // an additional cost is never another player's). Amount and
                // patient stay documented seams. `bindEvent`'s
                // `notEventRoleA` clears any stale outer event-role binding
                // FIRST ([CR#608.2k] — one antecedent set per body), so a
                // trigger body's own `that_player`/`that_patient` can't leak
                // into this payment's body.
                let mut body_frame = frame.clone();
                body_frame.anaphora.that_object = None;
                body_frame.anaphora.that_player = None;
                body_frame.anaphora.that_patient = None;
                if let Some(snapshot) = self.cost_paid_object(&cost, frame) {
                    body_frame.anaphora.that_object = Some(snapshot);
                }
                body_frame.anaphora.that_player = Some(frame.controller);
                // [CR#118.10]: one fresh payment id for this cost's whole
                // drain — every component below shares it; `body_frame`
                // (the consequence, not the payment) never carries it.
                let payment = self.mint_payment();
                let mut payment_frame = frame.clone();
                payment_frame.payment = Some(payment);
                let mut items: Vec<WorkItem> = cost
                    .iter()
                    .map(|c| WorkItem::RunEffect {
                        // Render each cost component as the controller's payment
                        // effect; a cost-side `With` becomes an `OneShotEffect::With`
                        // (its choice surfaced at payment), not a single action.
                        effect: Arc::new(crate::decide::unless_cost_effect(c, &Reference::You)),
                        frame: payment_frame.clone(),
                    })
                    .collect();
                items.push(WorkItem::RunEffect {
                    effect: ac.body,
                    frame: body_frame,
                });
                self.schedule_front(items);
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
            OneShotEffect::Delayed(ability) => {
                if !self.scan_created_reflexive(&ability, frame) {
                    let (source, bindings) = self.created_trigger_context(frame);
                    self.delayed_triggers.push(crate::trigger::CreatedTrigger {
                        source,
                        controller: frame.controller,
                        ability,
                        bindings,
                    });
                }
            }
            // [CR#603.12]: a reflexive triggered ability ("when you do") — the
            // same immediate resolution-window scan, but NEVER registered for a
            // future event.
            OneShotEffect::Reflexive(ability) => {
                self.scan_created_reflexive(&ability, frame);
            }
            other => todo!("stage 3 does not interpret effect {other:?} (the choice seam)"),
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
    fn batch_act_head(&self, unit: &BatchUnit<'_>, frame: &Frame) -> Option<BatchActHead> {
        use deckmaste_core::Agency;
        let agent = Some((frame.source, frame.controller));
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

    /// [CR#603.7d,603.7e]: the source and captured `~`/`This` context for a
    /// delayed/reflexive triggered ability created while `frame` resolves. The
    /// source is the creating object (the trigger/activated ability's own
    /// source snapshot when `frame` has one — [CR#603.7e]; otherwise the
    /// resolving spell — [CR#603.7d]). `~`/`This` is that same object's
    /// snapshot, so the created body reads it at its later resolution.
    ///
    /// [CR#400.7j]: when the creating source itself MOVED during the same
    /// resolution (madness exiles the very card whose ability is discarding
    /// it — `frame.source` is reminted and gone), `~`/`This` falls back to the
    /// `With(Produce(...))` product bound as `That`, so the delayed body's
    /// filter still anchors `Ref(This)` on the just-produced object and its
    /// watcher-source is that live card rather than a bare player proxy.
    fn created_trigger_context(
        &self,
        frame: &Frame,
    ) -> (ObjectSource, crate::trigger::TriggerBindings) {
        let this = frame
            .this
            .clone()
            .or_else(|| {
                self.objects
                    .get(frame.source)
                    .map(|_| crate::lki::LkiSnapshot::capture(self, frame.source))
            })
            .or_else(|| self.produced_that_snapshot(frame));
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

    /// [CR#400.7j]: the live snapshot of the `With(Produce(...))` product bound
    /// as a One `That` in `frame` — chased through the same-resolution move
    /// record to its current incarnation. `None` when there is no such binding
    /// or the product has left play. Anchors a created trigger whose own source
    /// moved itself away (madness).
    fn produced_that_snapshot(&self, frame: &Frame) -> Option<crate::lki::LkiSnapshot> {
        let that = frame.anaphora.that.as_ref()?;
        if that.cardinality != crate::stack::Cardinality::One {
            return None;
        }
        let &id = that.group.first()?;
        let product = self.chase_moved(id);
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
        frame: &Frame,
    ) -> bool {
        let (source, base) = self.created_trigger_context(frame);
        let mut emits = Vec::new();
        for event in self.resolution_events.clone() {
            if self.event_matches_delayed(&ability.event, &event, source) {
                let roles = self.event_roles(&event);
                let mut bindings = roles.bindings_over(base.clone());
                // [CR#400.7j,603.2e]: a trigger firing reflexively WITHIN the
                // resolution that produced its event reads the moved object at
                // its CURRENT identity. The event fact's `that_object` snapshot
                // holds the object's PRE-move id; the object was reminted on the
                // move (madness's Hand → Exile remint), so chase the live
                // same-resolution move record to the product and re-snapshot it
                // — the "…exiled this way" linkage. Left as-is when the object
                // did not move again or the chase leaves the store (LKI stands).
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
                        controller: frame.controller,
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

    /// The last-known snapshot of the object a cost payment moves
    /// ([CR#608.2k] cost→effect reference) — shared by the
    /// [`OneShotEffect::AdditionalCost`] arm and the `MayPayCost` yes-answer
    /// ([`crate::decide::pending::choice`]) — when the cost is a single
    /// object-moving verb (`Sacrifice`/`Move` — exile is `Move(_, Exile)` —
    /// or a `Discard` that names *what*) over a resolvable reference
    /// (`Sacrifice(This)`, …). Captured BEFORE the verb actually pays, so it
    /// is the object's last-known information ([CR#603.10a]) — exactly what
    /// the body's `EventObject` should read. Returns `None` for a cost that
    /// moves no object (mana/tap/life — nothing to bind).
    ///
    /// A `That(Sort)` reference — bound by an ENCLOSING cost
    /// `With(ChooseOne, …)` whose choice resolves before this cost's own
    /// verb runs — resolves fine too: `frame.anaphora.that` is already the
    /// choice's answer by the time we're called (`OneShotEffect::With` binds
    /// `that` before running its body, [CR#608.2]), so `eval_reference`
    /// reads a real value, not the unbound-`That` panic path. Only a
    /// genuinely unbound `That` (no enclosing `With` at all — malformed
    /// authoring) is still skipped, matching the historical `None` return.
    pub(crate) fn cost_paid_object(
        &self,
        cost: &[deckmaste_core::CostComponent],
        frame: &Frame,
    ) -> Option<crate::lki::LkiSnapshot> {
        for component in cost {
            let deckmaste_core::CostComponent::Do(pa) = component else {
                continue;
            };
            let reference = match pa.as_ref() {
                Action::Sacrifice(_, r) | Action::Move(r, _, _, _) => Some(r),
                // The bound discard form ("discard this card") names its
                // moved card in the body's single-move head.
                Action::Composite { name, body } if name.as_str() == "Discard" => {
                    deckmaste_core::discard_body_what(body)
                }
                _ => None,
            };
            let Some(reference) = reference else { continue };
            let resolvable =
                !matches!(reference, Reference::That(_)) || frame.anaphora.that.is_some();
            if resolvable {
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

    /// Whether a move verb relocates to an ordered [`Destination::Library`]
    /// position (the former player-agent `Move` twin was DELETED — merged
    /// into this one, agent-silent, verb); `Composite` looks through to its
    /// body.
    fn action_moves_to_library(a: &Action) -> bool {
        match a {
            Action::Move(_, Destination::Library(_), _, _) => true,
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
    Composite(&'a deckmaste_core::VerbName, &'a OneShotEffect),
    Draw(&'a Reference),
}

/// Classify a `Batch`'s per-unit body: `Some` iff it takes an aggregate window.
/// `None` keeps the plain sequential `Repeat`-style lane.
fn batch_act_unit(unit: &OneShotEffect) -> Option<BatchUnit<'_>> {
    match unit {
        OneShotEffect::Act(Action::Composite { name, body }) => {
            Some(BatchUnit::Composite(name, body))
        }
        OneShotEffect::Act(Action::DrawCard(who)) => Some(BatchUnit::Draw(who)),
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

    use std::sync::Arc;

    use deckmaste_core::Action;
    use deckmaste_core::Binder;
    use deckmaste_core::Card;
    use deckmaste_core::ChosenValueKind;
    use deckmaste_core::Count;
    use deckmaste_core::Countable;
    use deckmaste_core::LifeOp;
    use deckmaste_core::ObjectKind;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
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
                vec![].into(),
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
            .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let filter = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
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
            effect: Arc::new(OneShotEffect::Act(Action::deal_damage(
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
            .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::creature(),
                ]
                .into(),
            ))),
            effect: Arc::new(OneShotEffect::Act(Action::deal_damage(
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
        // Regression: the `Each` single-`Act` batch path must not silently
        // drop a choice-bearing body. Discard's `With(Choose(..), ..)` body's
        // carry future `Act` IS an ordinary `Emit` now (Task 8 — unlike the
        // retired `WorkItem::DiscardCards`), so both creatures' carry-Acts DO
        // collapse into one simultaneous `Emit(Batch)` — but each still
        // recurses into its OWN choose-then-discard `RunEffect` once that
        // batch passes, so TWO separate `ChooseObjects` decisions surface
        // (one per creature-triggered discard) and neither is dropped.
        // (Synthetic "for each creature, you discard a card" shape — chosen
        // to exercise the choice-bearing-body seam without standing up a
        // player-matching `over`.)
        let (mut state, a) = bear_on_field();
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);
        let hand_before = state.zones.hands[0].len();

        let frame = frame_src(a);
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::creature(),
                ]
                .into(),
            ))),
            effect: Arc::new(OneShotEffect::Act(Action::discard(
                Reference::You,
                Count::Literal(1),
                false,
            ))),
        });
        state.run_effect(effect, &frame);

        let mut decisions = 0;
        for _ in 0..60 {
            if state.zones.hands[0].len() + 2 <= hand_before {
                break;
            }
            if let crate::step::StepOutcome::NeedsDecision(
                crate::decide::PendingDecision::ChooseObjects(
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

    /// [CR#608.2] "[body], [count] times": a plain (non-choice) body runs the
    /// evaluated count total — `count` is read ONCE, up front (`eval_count`),
    /// never re-evaluated mid-loop.
    #[test]
    fn repeat_runs_a_plain_body_count_times() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let body = OneShotEffect::Act(Action::ChangeLife(
            Reference::You,
            LifeOp::Up(Count::Literal(2)),
        ));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(3), Arc::new(body)),
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

        let body = OneShotEffect::Act(Action::ChangeLife(
            Reference::You,
            LifeOp::Up(Count::Literal(2)),
        ));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(0), Arc::new(body)),
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
                who: Reference::You,
                effect: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Up(Count::Literal(3)),
                ))),
                if_did: None,
                if_not: None,
            })
        };
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(2), Arc::new(may_gain_3())),
            &frame,
        );

        // First iteration's decision.
        let PendingDecision::YesNo(crate::decide::pending::YesNo { player }) =
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
        let PendingDecision::YesNo(crate::decide::pending::YesNo { player }) =
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

        let body = OneShotEffect::Act(Action::ChangeLife(
            Reference::You,
            LifeOp::Up(Count::Literal(1)),
        ));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(1_000_000), Arc::new(body)),
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

        let body = OneShotEffect::Act(Action::ChangeLife(
            Reference::You,
            LifeOp::Up(Count::Literal(1)),
        ));
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(2), Arc::new(body)),
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

        let body = OneShotEffect::Act(Action::ChangeLife(
            Reference::You,
            LifeOp::Up(Count::Literal(2)),
        ));
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(0), Arc::new(body)),
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
            Arc::new(Card::Normal(deckmaste_core::CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                ..deckmaste_core::CardFace::default()
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
    #[test]
    fn bruvac_shape_batch_doubles_the_aggregate_and_no_card_moves_early() {
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
            instead: OneShotEffect::Batch(
                Count::Times(Arc::new(Count::Literal(2)), Arc::new(Count::ThatMany)),
                Arc::new(OneShotEffect::Act(deckmaste_core::Action::mill_one(
                    Reference::You,
                ))),
            ),
        };
        mint_on_field(
            &mut state,
            Card::Normal(deckmaste_core::CardFace {
                name: "Bruvac Stand-In".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![deckmaste_core::Ability::r#static(
                    deckmaste_core::StaticEffect::Replacement(Arc::new(bruvac)),
                )],
                ..deckmaste_core::CardFace::default()
            }),
        );

        let frame = frame_for(&state, p0);
        let mill_one = OneShotEffect::Act(deckmaste_core::Action::mill_one(Reference::You));
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(3), Arc::new(mill_one)),
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
            instead: OneShotEffect::Batch(
                Count::Literal(2),
                Arc::new(OneShotEffect::Act(deckmaste_core::Action::draw_one(
                    Reference::You,
                ))),
            ),
        };
        mint_on_field(
            &mut state,
            Card::Normal(deckmaste_core::CardFace {
                name: "Archive Stand-In".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![deckmaste_core::Ability::r#static(
                    deckmaste_core::StaticEffect::Replacement(Arc::new(archive)),
                )],
                ..deckmaste_core::CardFace::default()
            }),
        );

        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::draw(Reference::You, Count::Literal(1)),
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
            Card::Normal(deckmaste_core::CardFace {
                name: "Mill Watcher".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![deckmaste_core::Ability::triggered(
                    deckmaste_core::TriggeredAbility {
                        ability_word: None,
                        from: None,
                        condition: None,
                        limits: Vec::new().into(),
                        where_x: None,
                        event: deckmaste_core::EventFilter::Act {
                            verb: deckmaste_core::VerbName::from("Mill"),
                            who: Predicate::Any,
                            on: Predicate::Any,
                            cause: None,
                        },
                        effect: OneShotEffect::Act(Action::ChangeLife(
                            Reference::You,
                            LifeOp::Up(Count::Literal(1)),
                        )),
                    },
                )],
                ..deckmaste_core::CardFace::default()
            }),
        );

        let frame = frame_for(&state, p0);
        let mill_one = OneShotEffect::Act(deckmaste_core::Action::mill_one(Reference::You));
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(3), Arc::new(mill_one)),
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
            Card::Normal(deckmaste_core::CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(5)),
                ..deckmaste_core::CardFace::default()
            })
        };
        let a = mint_on_field(&mut state, fighter("Batch Fighter A"));
        let b = mint_on_field(&mut state, fighter("Batch Fighter B"));
        let frame = frame_src_targets(a, vec![a, b]);

        state.run_effect(
            OneShotEffect::Batch(
                Count::Literal(2),
                Arc::new(fight_effect(&Reference::Target(0), &Reference::Target(1))),
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
            Card::Normal(deckmaste_core::CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(0)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                ..deckmaste_core::CardFace::default()
            })
        };
        let a = mint_on_field(&mut state, fighter("Zero Fighter A"));
        let b = mint_on_field(&mut state, fighter("Zero Fighter B"));
        let frame = frame_src_targets(a, vec![a, b]);

        state.run_effect(
            OneShotEffect::Batch(
                Count::Literal(1),
                Arc::new(fight_effect(&Reference::Target(0), &Reference::Target(1))),
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
        let mill_one = OneShotEffect::Act(deckmaste_core::Action::mill_one(Reference::You));
        let agenda_before = state.agenda.len();
        state.run_effect(
            OneShotEffect::Batch(Count::Literal(2), Arc::new(mill_one)),
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

    /// [CR#702.85,701.57] `RevealUntil` fizzles to a graceful no-op — the
    /// Reveal seam (`Action::Reveal`/`GameEvent::Revealed`) is
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
            body: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(99)),
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
        let effect = OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(Action::ChooseValue(
                    Reference::You,
                    ChosenValueKind::Number,
                    key,
                )),
                OneShotEffect::mill(deckmaste_core::Reference::You, Count::Noted(key)),
            ]
            .into(),
        );
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        // The first child surfaced the number choice.
        let Some(PendingDecision::ChooseNoteNumber(crate::decide::pending::ChooseNoteNumber {
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
            effect: Arc::new(OneShotEffect::Act(Action::destroy(Reference::It))),
        });
        let frame = frame_src(a);
        state.run_effect(effect, &frame);

        let Some(PendingDecision::ChooseObjects(crate::decide::pending::ChooseObjects {
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
    #[should_panic(expected = "ChooseValue(Color)")]
    fn choose_value_color_is_a_loud_seam() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::Act(Action::ChooseValue(
                Reference::You,
                ChosenValueKind::Color,
                deckmaste_core::Ident::from("c"),
            )),
            &frame,
        );
    }

    #[test]
    fn choose_value_card_name_records_the_choice() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        let key = deckmaste_core::Ident::from("cn");
        state.run_effect(
            OneShotEffect::Act(Action::ChooseValue(
                Reference::You,
                ChosenValueKind::CardName,
                key,
            )),
            &frame,
        );
        run_injected(&mut state);
        assert!(matches!(
            state.pending,
            Some(PendingDecision::ChooseNoteCardName(crate::decide::pending::ChooseNoteCardName { key: pending, .. })) if pending == key
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

    /// P0.W3 seam filled for the new decision kind: the mechanical strategy
    /// notes the minimum (0, the X=0 default, via the reused `Decision::XValue`
    /// answer), and the seat resolver names the deciding player.
    #[test]
    fn strategy_and_seat_cover_the_note_number_choice() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (state, _a) = bear_on_field();
        let pending = PendingDecision::ChooseNoteNumber(crate::decide::pending::ChooseNoteNumber {
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
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
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
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
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
            "the two per-subject facts share one batch id ([CR#603.3b])"
        );
    }

    /// [CR#701.14c]: a self-fight has ONE subject — one committed fact.
    #[test]
    fn self_fight_commits_exactly_one_fact() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a, a]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
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
        // Darksteel Myr is canon's 0/1 (see myr_on_field, resolve/action.rs:971-1003).
        let (mut state, a, _b) = two_permanents_on_field();
        let myr_card = Arc::new(canon().card("Darksteel Myr").unwrap());
        let cid = state.cards.push(myr_card, PlayerId(0));
        let myr = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(myr);
        let frame = frame_src_targets(a, vec![myr, a]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
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
            effect: Arc::new(StaticEffect::Each(
                Selection::SelectAll(filter.clone()),
                Arc::new(StaticEffect::Modify(
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
            effect: Arc::new(StaticEffect::Modify(
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
            effect: Arc::new(StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block {
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
            effect: Arc::new(StaticEffect::CostModifier {
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
            effect: Arc::new(StaticEffect::CantHappen(filter)),
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
            effect: Arc::new(StaticEffect::Prevention(Arc::new(
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
        let cond = Condition::Exists(Predicate::Not(Arc::new(Predicate::Any)));
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Arc::new(StaticEffect::Modify(
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
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let spec = CopySpec {
            source: CopySource::SelfCard,
            exceptions: vec![],
        };

        let effect = OneShotEffect::Continuously(Continuously {
            effect: Arc::new(StaticEffect::BecomesCopy(Reference::This, spec.clone())),
            duration: Duration::EndOfGame,
        });
        state.run_effect(effect, &frame);
        assert!(
            state.continuous.is_empty(),
            "Continuously(BecomesCopy(..)) fizzles — installs no continuous instance"
        );

        let until = OneShotEffect::Until(
            Duration::EndOfGame,
            vec![StaticEffect::BecomesCopy(Reference::This, spec)].into(),
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
        use deckmaste_core::LifeOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let seq = OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Up(Count::Literal(1)),
                )),
                OneShotEffect::Until(
                    Duration::ForThisEvent,
                    vec![StaticEffect::Deontic(Deontic::Cant(
                        DeonticAction::Regenerate {
                            by: Predicate::Any,
                            on: Predicate::Ref(Reference::This),
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
        let seq = OneShotEffect::Sequentially(
            vec![OneShotEffect::Until(
                Duration::ForThisEvent,
                vec![StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Regenerate {
                        by: Predicate::Any,
                        on: Predicate::Ref(Reference::This),
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
        let frame = frame_src(src);
        // The sweepable-duration guard is LOUD before the `That` subject read.
        state.create_shield(
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
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::LifeGained(
            LifeGained {
                player: PlayerId(0),
                amount: 1,
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
    #[test]
    fn divide_among_splits_amount_and_binds_allotment() {
        use deckmaste_core::Distribute;
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(a);
        let effect = OneShotEffect::Distribute(Distribute {
            amount: Count::Literal(3),
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::creature(),
                ]
                .into(),
            ))),
            body: Arc::new(OneShotEffect::Act(Action::deal_damage(
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
            body: Arc::new(OneShotEffect::Act(Action::deal_damage(
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
                effect: Arc::new(OneShotEffect::Act(Action::deal_damage(
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
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(deckmaste_core::Each {
                binder: Binder::Choose {
                    quantity: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                    filter: creatures,
                    by: Reference::You,
                },
                effect: Arc::new(OneShotEffect::Act(Action::destroy(Reference::It))),
            }),
            &frame,
        );
        // The many-binder surfaces a choice for the WHOLE group before iterating.
        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects(
            crate::decide::pending::ChooseObjects {
                min,
                max,
                candidates,
                ..
            },
        )) = state.step()
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
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let effect = OneShotEffect::Distribute(deckmaste_core::Distribute {
            amount: Count::Literal(2),
            binder: Binder::Existing(Selection::SelectAll(creatures.clone())),
            // The outer share is in scope here, but the inner `Each` rebinds `It`
            // per inner element and clears it before the body runs.
            body: Arc::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Arc::new(OneShotEffect::Act(Action::deal_damage(
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
            OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(n)),
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
            OneShotEffect::If(If {
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
            OneShotEffect::If(If {
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
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Arc::new(OneShotEffect::Act(Action::destroy(Reference::It))),
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
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let frame = frame_src(bear);
        let life0 = state.player(PlayerId(0)).life;
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
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

    /// [CR#118.12]: `OneShotEffect::May` surfaces a yes/no to the controller. Yes runs
    /// `effect` then `if_did`; no runs `if_not` (nothing when absent). Driven
    /// via `GainLife` so each branch reads as a clean life delta.
    #[test]
    fn run_effect_may_branches_on_the_answer() {
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let gain = |n| {
            OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(n)),
            ))
        };
        let may = || May {
            who: Reference::You,
            effect: Arc::new(gain(3)),
            if_did: Some(Arc::new(gain(10))),
            if_not: Some(Arc::new(gain(1))),
        };
        let p0 = PlayerId(0);

        // yes → effect (3) + if_did (10) = +13; surfaces YesNo to the controller.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(may()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo(crate::decide::pending::YesNo {
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
                who: Reference::You,
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
            effect: OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(n)),
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
                modes: modes().into(),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(PendingDecision::ChooseModes(
            crate::decide::pending::ChooseModes {
                player,
                options,
                min,
                max,
                repeats,
            },
        )) = state.step()
        else {
            panic!("expected ChooseModes, got {:?}", state.pending);
        };
        assert_eq!((player, options, min, max, repeats), (p0, 3, 1, 1, false));
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
            OneShotEffect::Modal(Modal {
                choose: spec(2, false),
                modes: modes().into(),
            }),
            &frame,
        );
        state.submit_decision(Decision::Modes(vec![0, 2])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "both chosen modes run");
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
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let pay_cost = || {
            OneShotEffect::Act(Action::Pay(Cost(
                vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Down(Count::Literal(2)),
                ))]
                .into(),
            )))
        };
        let must_pay = || May {
            who: Reference::You,
            effect: Arc::new(pay_cost()),
            if_did: None,
            if_not: Some(Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(10)),
            )))),
        };

        // "I'll pay" → lose 2, the punisher (gain 10) skipped.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(must_pay()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo(crate::decide::pending::YesNo {
            player,
        })) = state.step()
        else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
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
        state.run_effect(OneShotEffect::May(must_pay()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
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
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let pay_cost = || {
            OneShotEffect::Act(Action::Pay(Cost(
                vec![CostComponent::do_action(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Down(Count::Literal(2)),
                ))]
                .into(),
            )))
        };
        let may_pay = || May {
            who: Reference::You,
            effect: Arc::new(pay_cost()),
            if_did: Some(Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(10)),
            )))),
            if_not: Some(Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1)),
            )))),
        };

        // "I'll pay" → lose 2 THEN gain 10 (net +8) — the kicker fires.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(may_pay()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo(crate::decide::pending::YesNo {
            player,
        })) = state.step()
        else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
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
        state.run_effect(OneShotEffect::May(may_pay()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "decline → if_not runs");
    }

    /// [CR#118.12]: the branch is decided by whether the payer CHOSE to pay,
    /// "regardless of what events actually occurred" (the Dermoplasm
    /// clause) — the engine schedules `if_did` immediately once the yes
    /// answer comes back, chained BEFORE the toll items even run (never by
    /// inspecting what those toll items go on to produce). A sacrifice
    /// payment — Dermoplasm's own cost verb — demonstrates it: `if_did`
    /// fires off the choice, not off any inspected sacrifice event.
    ///
    /// A test that only checks the AFTERMATH (creature gone + life gained)
    /// can't discriminate "branch on the choice" from "branch on the
    /// observed event" — both hypotheses predict the same aftermath here,
    /// since nothing in this engine can make a CHOSEN, cost-eligible,
    /// directly-resolvable `Sacrifice(This)` toll item produce no event
    /// (`can_pay_may_cost`'s pre-offer legality gate, [CR#608.2d], already
    /// forces `if_not` for anything that can't actually pay — see
    /// `run_effect_may_pay_forces_if_not_when_unable_to_pay` below — so a
    /// CHOSEN-but-silent toll is unreachable with current machinery). So
    /// this asserts the ordering the design actually rests on instead: right
    /// after the yes answer — BEFORE either work item has been stepped —
    /// `if_did`'s `RunEffect` already sits in the agenda immediately behind
    /// the toll item, proving it was scheduled in the same atomic step as
    /// the toll items ([CR#118.12] "regardless of what events actually
    /// occurred"), not appended later after inspecting what they produced.
    #[test]
    fn run_effect_may_pay_branches_on_the_choice_not_observed_events() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, bear) = bear_on_field();
        let p0 = PlayerId(0);
        let frame = frame_src(bear);
        let pay_cost = OneShotEffect::Act(Action::Pay(Cost(
            vec![CostComponent::do_action(Action::Sacrifice(
                Reference::You,
                Reference::This,
            ))]
            .into(),
        )));
        let may = May {
            who: Reference::You,
            effect: Arc::new(pay_cost),
            if_did: Some(Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(7)),
            )))),
            if_not: None,
        };
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(may), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo(crate::decide::pending::YesNo {
            ..
        })) = state.step()
        else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        state.submit_decision(Decision::Answer(true)).unwrap();

        // Ordering discriminator ([CR#118.12]): inspect the agenda BEFORE
        // stepping either item — `if_did` (ChangeLife) is already queued
        // right behind the toll item (Sacrifice), scheduled by the SAME
        // `schedule_front` call the yes-answer made, not contingent on the
        // toll item having run yet.
        assert!(
            matches!(
                state.agenda.front(),
                Some(WorkItem::RunEffect { effect, .. })
                    if matches!(effect.as_ref(), OneShotEffect::Act(Action::Sacrifice(..)))
            ),
            "the toll item (Sacrifice) is at the agenda front, unstepped: {:?}",
            state.agenda.front()
        );
        assert!(
            matches!(
                state.agenda.get(1),
                Some(WorkItem::RunEffect { effect, .. })
                    if matches!(effect.as_ref(), OneShotEffect::Act(Action::ChangeLife(..)))
            ),
            "if_did (ChangeLife) is already scheduled right behind it: {:?}",
            state.agenda.get(1)
        );

        let _ = drain_progress(&mut state, 40);
        assert!(
            !state.zones.battlefield.contains(&bear),
            "the sacrifice paid the cost"
        );
        assert_eq!(
            state.player(p0).life,
            life0 + 7,
            "if_did runs off the yes CHOICE, not a post-hoc events check"
        );
    }

    /// [CR#608.2d]: inability to make a legal "yes" choice forces `if_not`
    /// directly — no unanswerable `YesNo` is ever surfaced. An unaffordable
    /// mana cost (the payer's pool is empty) skips straight to `if_not`.
    #[test]
    fn run_effect_may_pay_forces_if_not_when_unable_to_pay() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::May;

        let p0 = PlayerId(0);
        let mut state = game();
        // [CR#119.4,119.4b]: paying life needs life >= the amount — 1 life
        // can't cover a 5-life toll, so the "yes" choice is illegal.
        state.player_mut(p0).life = 1;
        let frame = frame_for(&state, p0);
        let pay_cost = OneShotEffect::Act(Action::Pay(Cost(
            vec![CostComponent::do_action(Action::ChangeLife(
                Reference::You,
                LifeOp::Down(Count::Literal(5)),
            ))]
            .into(),
        )));
        let may = May {
            who: Reference::You,
            effect: Arc::new(pay_cost),
            if_did: Some(Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(10)),
            )))),
            if_not: Some(Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1)),
            )))),
        };
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(may), &frame);
        assert!(
            state.pending.is_none(),
            "unaffordable → no YesNo is ever surfaced"
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 1,
            "unaffordable → if_not runs directly, if_did never offered"
        );
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
                pay: Cost(
                    vec![CostComponent::do_action(Action::Sacrifice(
                        Reference::You,
                        Reference::This,
                    ))]
                    .into(),
                ),
                body: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Up(Count::CounterCount(
                        Arc::new(Reference::EventObject),
                        "P1P1Counter".into(),
                    )),
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

    /// [CR#608.2k]: `bindEvent`'s `notEventRoleA` hygiene, ported — a stale
    /// event-role binding an ENCLOSING scope carried (e.g. a triggered
    /// ability's own firing-event patient) must not ride along into an
    /// `AdditionalCost` body's frame just because the body's frame is a
    /// clone of the enclosing one. One antecedent set per body: the payment
    /// clears `that_patient` before landing its own (unpopulated —
    /// documented seam) roles, rather than leaving the outer value in place.
    #[test]
    fn additional_cost_body_clears_a_stale_outer_event_patient_binding() {
        use deckmaste_core::AdditionalCost;
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;

        let (mut state, bear) = bear_on_field();
        let mut frame = frame_src(bear);
        // Simulate an enclosing scope's own stale event-role binding — the
        // shape a triggered ability's frame carries when ITS firing event
        // named a patient (`resolve/mod.rs`'s trigger-frame construction).
        frame.anaphora.that_patient = Some(crate::trigger::EventPatient::Player(PlayerId(1)));

        state.run_effect(
            OneShotEffect::AdditionalCost(AdditionalCost {
                pay: Cost(
                    vec![CostComponent::do_action(Action::Sacrifice(
                        Reference::You,
                        Reference::This,
                    ))]
                    .into(),
                ),
                body: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Up(Count::Literal(1)),
                ))),
            }),
            &frame,
        );

        let body_frame = state
            .agenda
            .iter()
            .find_map(|item| match item {
                WorkItem::RunEffect { effect, frame }
                    if matches!(effect.as_ref(), OneShotEffect::Act(Action::ChangeLife(..))) =>
                {
                    Some(frame.clone())
                }
                _ => None,
            })
            .expect("the AdditionalCost body's RunEffect is scheduled");

        assert_eq!(
            body_frame.anaphora.that_patient, None,
            "a stale outer EventPatient binding must not leak into the payment body"
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

        Condition::And(
            vec![
                Condition::Compare(
                    Count::CountOf(Countable::Objects(Arc::new(Predicate::And(
                        vec![
                            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                                Predicate::Ref(Reference::You),
                            ))),
                        ]
                        .into(),
                    )))),
                    Cmp::AtLeast,
                    Count::Literal(10),
                ),
                Condition::Not(Arc::new(Condition::Matches(
                    Reference::You,
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
    fn secrets_effect() -> OneShotEffect {
        use deckmaste_core::Condition;
        use deckmaste_core::If;

        OneShotEffect::Sequentially(
            vec![
                OneShotEffect::If(If {
                    condition: ascend_gate(),
                    then: Arc::new(OneShotEffect::Act(Action::GetDesignation(
                        Reference::You,
                        "CitysBlessing".into(),
                    ))),
                    otherwise: None,
                }),
                OneShotEffect::If(If {
                    condition: Condition::Matches(
                        Reference::You,
                        Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
                    ),
                    then: Arc::new(OneShotEffect::draw(Reference::You, Count::Literal(3))),
                    otherwise: Some(Arc::new(OneShotEffect::draw(
                        Reference::You,
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
                    if let GameEvent::DamageDealt(DamageDealt { target, amount, .. }) = e {
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
            OneShotEffect::draw(Reference::You, Count::Literal(3)),
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
                body: Arc::new(OneShotEffect::Sequentially(vec![].into())),
            }),
            &frame,
        );
        // Drain the agenda — the empty Sequentially body completes without a
        // decision, proving With schedules correctly.
        for _ in 0..10 {
            state.step();
        }
    }

    /// Mint a single card of type `ty` onto the (empty) top of `owner`'s
    /// library and return it. `game()` starts with empty libraries, so the
    /// lone `push_back` object is the top card.
    fn mint_library_top(state: &mut GameState, owner: PlayerId, name: &str, ty: Type) -> ObjectId {
        let cid = state.cards.push(
            Arc::new(Card::Normal(deckmaste_core::CardFace {
                name: name.into(),
                types: vec![ty.def()],
                ..deckmaste_core::CardFace::default()
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
            Card::Normal(deckmaste_core::CardFace {
                name: "Explorer".into(),
                types: vec![Type::Creature.def()],
                ..deckmaste_core::CardFace::default()
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

        let effect: OneShotEffect = builtin().macros.read_str("Explore").unwrap();
        state.run_effect(effect, &frame_src(source));
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

        let effect: OneShotEffect = builtin().macros.read_str("Explore").unwrap();
        state.run_effect(effect, &frame_src(source));
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
