//! The replacement-effect registry ([CR#614]). Gathers replacements watching an
//! event intent and applies them per [CR#616.1] with lineage ([CR#614.5]).
//!
//! Task 1: the `replacement_watches` matcher — does a replacement's `would`
//! (a core `EventFilter`) watch a given live `GameEvent` intent, and who/what
//! does the intent affect.
//!
//! Later tasks (2–10) add: `CantHappen` variant + cant pass, shields registry,
//! `replace_event` loop, `ChooseReplacement` decision, regeneration,
//! umbra/totem armor, genericity proof, and Skip step elision.

use deckmaste_core::Ability;
use deckmaste_core::Duration;
use deckmaste_core::EventFilter;
use deckmaste_core::Prevention;
use deckmaste_core::Replacement;
use deckmaste_core::StaticEffect;

use crate::event::GameEvent;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::stack::Endophora;
use crate::state::GameState;
use crate::trigger::EventPatient;

/// What an intent affects — the object being moved/changed, or the player
/// experiencing the event (e.g. the player drawing a card, gaining life).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Affected {
    Object(ObjectId),
    Player(PlayerId),
}

/// Whether the replacement pipeline intercepts `e` at all ([CR#614]: only
/// INTENTS are replaceable — an event whose apply is still ahead of it.
/// Post-hoc facts (`ZoneChanged`, loss records) and directly recorded facts
/// (`AbilityUsed`) are not). Extending this set is what lifts a master
/// form's would-lane bridge cap: the kinds outside it keep their
/// `would: false` rows — step onsets belong to the Skip elision pass,
/// attach facts follow the already-performed relation mutation, targeting
/// is announce-time legality, coin/die facts carry an already-decided
/// outcome, and tap/untap/attack/block prevention rides the `Cant` statics.
pub(crate) fn replaceable(e: &GameEvent) -> bool {
    matches!(
        e,
        GameEvent::WillDestroy { .. }
            | GameEvent::WillDraw { .. }
            | GameEvent::ZoneWillChange { .. }
            | GameEvent::DamageDealt { .. }
            | GameEvent::LifeGained { .. }
            | GameEvent::LifeLost { .. }
            | GameEvent::CounterPlaced { .. }
            | GameEvent::CounterRemoved { .. }
            | GameEvent::SpellCast(_)
            | GameEvent::AbilityActivated { .. }
            | GameEvent::TokenCreated { .. }
            | GameEvent::GotDesignation { .. }
            | GameEvent::DesignationChanged { .. }
    )
}

/// What a replaceable intent AFFECTS — the recipient the [CR#616.1] choice
/// keys on and the `That`/`EventObject` a replacement body reads
/// ([CR#608.2]). `None` for intents with no single recipient (a token spec
/// not yet minted, a game-scope designation flip).
pub(crate) fn affected(e: &GameEvent) -> Option<Affected> {
    match e {
        GameEvent::WillDestroy { object, .. }
        | GameEvent::ZoneWillChange { object, .. }
        | GameEvent::CounterPlaced { object, .. }
        | GameEvent::CounterRemoved { object, .. }
        | GameEvent::DamageDealt { target: object, .. } => Some(Affected::Object(*object)),
        GameEvent::WillDraw { player, .. }
        | GameEvent::LifeGained { player, .. }
        | GameEvent::LifeLost { player, .. }
        | GameEvent::GotDesignation { player, .. } => Some(Affected::Player(*player)),
        GameEvent::SpellCast(o) => Some(Affected::Object(*o)),
        GameEvent::AbilityActivated { source, .. } => Some(Affected::Object(*source)),
        _ => None,
    }
}

/// Whether `would` (with `Ref(This) = this`, the watching object) watches
/// intent `e` — the Replacement-lane entry into THE one evaluator
/// ([CR#614]): the intercepted intent becomes its fact record
/// ([`crate::eval::FactView`]) and evaluates with participants LIVE (a
/// would-fact's participants are still around by definition), the raw cause
/// triple intact (the `Cause:agent` lift — no lossy pattern re-lift), and
/// `Ref(This)`/`Ref(You)` anchored on the watcher.
pub(crate) fn replacement_watches(
    state: &GameState,
    would: &EventFilter,
    this: ObjectId,
    e: &GameEvent,
) -> bool {
    if !replaceable(e) {
        return false;
    }
    let Some(fact) = crate::eval::FactView::of(state, e) else {
        return false;
    };
    let watcher = object_source_of(state, this);
    state.eval(
        would,
        &fact,
        crate::eval::Lane::Replacement,
        &crate::eval::Bindings::watcher(watcher),
    )
}

/// Look through a remembered `EventFilter` macro invocation (`Expanded`) to
/// the underlying structural form.
pub(crate) fn look_through_event(event: &EventFilter) -> &EventFilter {
    match event {
        EventFilter::Expanded(e) => look_through_event(&e.value),
        other => other,
    }
}

/// [CR#614.17]: whether any battlefield static makes `e` unable to happen.
/// Runs before the replacement registry — can't-happen events are suppressed
/// entirely; the replacement loop is skipped ([CR#614.17c]).
pub(crate) fn cant_event(state: &GameState, e: &GameEvent) -> bool {
    if !replaceable(e) {
        return false;
    }
    let view = state.layers();
    state.zones.battlefield.iter().any(|&obj| {
        crate::legal::object_has_static(&view, obj, &|s| {
            matches!(s, StaticEffect::CantHappen(would)
                if replacement_watches(state, look_through_event(would), obj, e))
        })
    })
}

/// The `ObjectSource` of a live object — used to anchor `Ref(This)`.
fn object_source_of(state: &GameState, id: ObjectId) -> ObjectSource {
    state.objects.obj(id).source
}

// ── Floating-replacement registry types (Task 3) ─────────────────────────────

/// Stable identity for a floating replacement instance (a regeneration shield
/// or other "the next time … instead" effect). Used as the lineage key so the
/// [CR#614.5] applied-set can track it across event rewrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceId(pub u32);

/// A floating one-shot/duration-bounded replacement effect ([CR#614.3]):
/// regeneration shields and "the next time …" replacements. Stored in
/// `GameState.shields`; swept at end of turn; a `one_shot` instance is removed
/// when it is the chosen replacement.
#[derive(Debug, Clone)]
pub struct ReplacementInstance {
    pub id: InstanceId,
    pub replacement: Replacement,
    /// The permanent the replacement protects / watches.
    pub subject: ObjectId,
    pub duration: Duration,
    /// If true, consumed on first application ([CR#614.3]).
    pub one_shot: bool,
    /// The object whose static/activated ability created this instance (used
    /// to build the body frame).
    pub source: ObjectId,
}

/// Stable identity for a replacement effect — either a static ability effect
/// slot or a floating instance. Used in the [CR#614.5] lineage set so the same
/// replacement can't be applied twice to the modified event. `pub` so
/// `PendingDecision::ChooseReplacement` (and its integration tests) can name
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplacementKey {
    /// A static `StaticEffect::Replacement` at a known ability/effect index.
    Static {
        source: ObjectId,
        ability: usize,
        effect: usize,
    },
    /// A floating one-shot instance in `GameState.shields`.
    Floating(InstanceId),
}

#[derive(Debug, Clone)]
pub(crate) enum ApplicableEffect {
    Replacement(Replacement),
    Prevention(Prevention),
}

/// One replacement or prevention effect that is applicable to the current event
/// — the key (for lineage), the effect itself, and the source object.
#[derive(Debug, Clone)]
pub(crate) struct Applicable {
    pub key: ReplacementKey,
    pub effect: ApplicableEffect,
    pub source: ObjectId,
}

/// Collect every replacement effect watching intent `e` from:
/// - static abilities on every battlefield object, and
/// - floating instances in `state.shields`.
///
/// Does NOT filter by lineage; callers apply the lineage set.
pub(crate) fn gather_applicable(state: &GameState, e: &GameEvent) -> Vec<Applicable> {
    let mut out = Vec::new();

    // Static replacements on every battlefield object (self- and other-watching).
    for &obj in &state.zones.battlefield {
        let abilities = crate::derive::abilities_of_source(state, state.objects.obj(obj).source);
        for (ai, ability) in abilities.iter().enumerate() {
            let Ability::Static(s) = ability else {
                continue;
            };
            // `Ability::Static` carries a single `StaticEffect` directly, so
            // the effect index is always 0 — kept in `ReplacementKey::Static`
            // for shape stability (multi-effect abilities used to
            // disambiguate by index).
            if let StaticEffect::Replacement(r) = s
                && replacement_would(state, r, obj, e)
            {
                out.push(Applicable {
                    key: ReplacementKey::Static {
                        source: obj,
                        ability: ai,
                        effect: 0,
                    },
                    effect: ApplicableEffect::Replacement((**r).clone()),
                    source: obj,
                });
            }
        }
    }

    // Floating instances (regeneration shields, etc.). A shield's `subject`
    // was resolved to a concrete object when the shield was created (its
    // captured `That`), so it matches by SUBJECT IDENTITY — independent of how
    // the source ability refers to it — which is why "regenerate target
    // creature" (source ≠ subject) works, not just "regenerate this creature".
    for inst in &state.shields {
        if floating_watches(state, &inst.replacement, inst.subject, e) {
            out.push(Applicable {
                key: ReplacementKey::Floating(inst.id),
                effect: ApplicableEffect::Replacement(inst.replacement.clone()),
                source: inst.source,
            });
        }
    }

    // Static preventions on every battlefield object (watching damage events).
    if let GameEvent::DamageDealt {
        source: event_source,
        target: event_target,
        ..
    } = *e
    {
        for &obj in &state.zones.battlefield {
            let abilities =
                crate::derive::abilities_of_source(state, state.objects.obj(obj).source);
            for (ai, ability) in abilities.iter().enumerate() {
                let Ability::Static(s) = ability else {
                    continue;
                };
                if let StaticEffect::Prevention(p) = s
                    && prevention_watches(state, p, obj, event_source, event_target)
                {
                    out.push(Applicable {
                        key: ReplacementKey::Static {
                            source: obj,
                            ability: ai,
                            effect: 0,
                        },
                        effect: ApplicableEffect::Prevention((**p).clone()),
                        source: obj,
                    });
                }
            }
        }
    }

    out
}

/// Whether replacement `r` (with watcher `source`) watches intent `e` — its
/// `would` (Instead/Also) matches per `replacement_watches`. Returns `false`
/// for `Skip` (handled by the step-elision pass, Task 9) and `Expanded`.
fn replacement_would(state: &GameState, r: &Replacement, source: ObjectId, e: &GameEvent) -> bool {
    match crate::replace::look_through_replacement(r) {
        Replacement::Instead { would, .. } | Replacement::Also { would, .. } => {
            replacement_watches(state, would, source, e)
        }
        Replacement::Skip { .. } => false, // handled in begin_step, Task 9
        Replacement::Expanded(_) => unreachable!("look_through_replacement strips Expanded"),
    }
}

fn prevention_watches(
    state: &GameState,
    p: &Prevention,
    obj: ObjectId,
    source: ObjectId,
    target: ObjectId,
) -> bool {
    let watcher = Some(state.objects.obj(obj).source);
    let (from, to) = match p {
        Prevention::PreventAll { from, to, .. } => (from, to),
        Prevention::PreventNext { from, to, .. } => (from, to),
        Prevention::PreventNextInstance { from, to } => (from, to),
    };
    crate::target::matches_with(state, source, from, watcher)
        && crate::target::matches_with(state, target, to, watcher)
}

/// Whether a FLOATING shield watches intent `e`. A shield's `subject` was
/// resolved to a concrete object when the shield was created (its captured
/// `That`), so matching is by SUBJECT IDENTITY — the `would`'s participant
/// filters (typically `Ref(EventObject)`, which a frameless gather can't
/// re-resolve) are NOT re-evaluated — paired with the event SHAPE (kind +
/// zones/cause/refinements), evaluated through the one evaluator in
/// `shape_only` bindings.
fn floating_watches(
    state: &GameState,
    replacement: &Replacement,
    subject: ObjectId,
    e: &GameEvent,
) -> bool {
    if !replaceable(e) {
        return false;
    }
    if affected(e) != Some(Affected::Object(subject)) {
        return false;
    }
    let would = match crate::replace::look_through_replacement(replacement) {
        Replacement::Instead { would, .. } | Replacement::Also { would, .. } => would,
        Replacement::Skip { .. } | Replacement::Expanded(_) => return false,
    };
    let Some(fact) = crate::eval::FactView::of(state, e) else {
        return false;
    };
    state.eval(
        would,
        &fact,
        crate::eval::Lane::Replacement,
        &crate::eval::Bindings {
            watcher: object_source_of(state, subject),
            frame: None,
            shape_only: true,
        },
    )
}

// ── Task 4: replace_event loop + lineage + apply Instead/Also ────────────────

/// The outcome of running the [CR#616.1] replacement loop for one event.
#[derive(Debug)]
pub(crate) enum ReplaceOutcome {
    /// No applicable replacement rewrote the event — apply `e` as-is.
    Pass(GameEvent),
    /// The event was replaced to nothing (`Instead` with a body that doesn't
    /// re-emit the event) — skip `apply`.
    Nothing,
    /// Multiple applicable replacements require a player choice — the
    /// `ChooseReplacement` decision has been surfaced and the loop is
    /// suspended. The pending decision will resume processing via Task 5.
    Suspend,
}

/// Run the [CR#616.1] replacement loop for intent `e` with the [CR#614.5]
/// lineage set. Returns the modified event to apply, nothing (replaced away),
/// or a suspension waiting for a `ChooseReplacement` decision.
///
/// The affected object IS threaded into the body frame as the event
/// `EventObject` (via `schedule_body`), so a body reads it with
/// `Ref(EventObject)` while `This` stays the source ability.
///
/// Seam: general [CR#614.15] self-replacement (resolution-time) is a
/// `todo!`-tagged future concern; APNAP multi-player 616 ordering is also
/// deferred.
pub(crate) fn replace_event(state: &mut GameState, e: GameEvent) -> ReplaceOutcome {
    use std::collections::HashSet;

    // Non-replaceable intents pass through immediately ([CR#614]: only
    // replaceable intents can be modified by replacement effects).
    if !replaceable(&e) {
        return ReplaceOutcome::Pass(e);
    }

    // [CR#614.5]: the lineage set — a replacement that has already been applied
    // to the current event chain cannot apply again, terminating loops.
    let mut applied: HashSet<ReplacementKey> = HashSet::new();
    let mut current = e;

    loop {
        let applicable: Vec<Applicable> = gather_applicable(state, &current)
            .into_iter()
            .filter(|a| !applied.contains(&a.key))
            .collect();

        match applicable.len() {
            // [CR#616.1]: no more replacements watch the (possibly modified)
            // event — apply it as-is.
            0 => return ReplaceOutcome::Pass(current),

            // [CR#616.1]: exactly one applicable — auto-apply (no choice).
            1 => {
                let a = applicable.into_iter().next().unwrap();
                applied.insert(a.key);
                match apply_one(state, current, &a) {
                    Some(modified) => {
                        // Keep looping — the modified event may be watched by
                        // further replacements ([CR#616.1f]).
                        current = modified;
                    }
                    None => {
                        // The event was replaced to nothing (Instead with no
                        // re-emitted event).
                        return ReplaceOutcome::Nothing;
                    }
                }
            }

            // [CR#616.1]: multiple applicable — surface a choice ([CR#616.1]).
            // Store the suspended state and surface the decision; the resume
            // is driven by `resume_replacements` after `submit_decision`.
            _ => {
                surface_choice(state, current, applied, &applicable);
                return ReplaceOutcome::Suspend;
            }
        }
    }
}

/// The magnitude an amount-carrying intent fixes for the `Count::ThatMuch`
/// register ([CR#107.3], "that many"). An `Instead` replaces the intent away —
/// it never reaches the `apply` funnel that normally fixes `that_much` — so a
/// body that reads "that many" (infect's poison/-1/-1 counters,
/// [CR#702.90b,702.90c]) must have it set HERE, off the replaced intent.
/// Mirrors the `apply`-funnel set ([CR#120.3], damage/life amounts).
fn intent_magnitude(e: &GameEvent) -> Option<deckmaste_core::Uint> {
    match e {
        GameEvent::DamageDealt { amount, .. }
        | GameEvent::LifeLost { amount, .. }
        | GameEvent::LifeGained { amount, .. } => Some(*amount),
        _ => None,
    }
}

/// Apply one replacement to `e`. Returns the modified event to continue
/// looping on, or `None` when the event is replaced to nothing (Instead).
/// Schedules body effects via `schedule_body`.
fn apply_one(state: &mut GameState, e: GameEvent, a: &Applicable) -> Option<GameEvent> {
    // [CR#608.2]: the object the intent affects (the would-be-destroyed
    // permanent, the damaged creature, …) is bound to `That` for the body to
    // read — regeneration heals/taps `That`. `This` stays the source.
    let that = match affected(&e) {
        Some(Affected::Object(id)) => Some(id),
        _ => None,
    };
    // [CR#107.3]: fix the "that many" register off the replaced intent so the
    // body's `Count::ThatMuch` reads the original magnitude. The `apply` funnel
    // can't do it — an `Instead` body schedules BEFORE (and instead of) the
    // intent's apply. Set before `schedule_body` so the scheduled `RunEffect`
    // (front of agenda, runs next) sees it. No-op for amount-less intents.
    if let Some(amount) = intent_magnitude(&e) {
        state.that_much = Some(amount);
    }
    match &a.effect {
        ApplicableEffect::Replacement(replacement) => {
            match crate::replace::look_through_replacement(replacement).clone() {
                Replacement::Instead { instead, .. } => {
                    // [CR#614.1a,614.6]: the event is replaced — it does NOT happen.
                    // Schedule the `instead` body; consume a one-shot shield if present.
                    schedule_body(state, instead, a.source, that);
                    // [CR#614.3]: only consume a floating instance when it is one-shot
                    // (e.g. a regeneration shield). Duration-only floating replacements
                    // (one_shot: false) persist until their duration expires and must
                    // NOT be consumed on use.
                    if let ReplacementKey::Floating(iid) = a.key
                        && state.shields.iter().any(|s| s.id == iid && s.one_shot)
                    {
                        consume_shield(state, iid);
                    }
                    None // event replaced away
                }
                Replacement::Also { also, .. } => {
                    // [CR#614.1c]: the event still happens AND `also` happens.
                    // Schedule the body; the (unchanged) event continues.
                    schedule_body(state, also, a.source, that);
                    Some(e)
                }
                Replacement::Skip { .. } => {
                    // Skip is handled by the step-elision pass (Task 9), not here.
                    Some(e)
                }
                Replacement::Expanded(_) => {
                    unreachable!("look_through_replacement strips Expanded")
                }
            }
        }
        ApplicableEffect::Prevention(prevention) => {
            if let GameEvent::DamageDealt {
                source: event_source,
                target: event_target,
                amount,
                combat,
            } = e
            {
                match prevention {
                    Prevention::PreventAll { .. } => {
                        // [CR#615.6]: prevented damage never happens.
                        None
                    }
                    Prevention::PreventNext { n, .. } => {
                        let frame = crate::stack::Frame::bare(
                            a.source,
                            state.objects.obj(a.source).controller,
                        );
                        let n_val = state.eval_count(n, &frame);
                        if amount <= n_val {
                            None
                        } else {
                            Some(GameEvent::DamageDealt {
                                source: event_source,
                                target: event_target,
                                amount: amount - n_val,
                                combat,
                            })
                        }
                    }
                    Prevention::PreventNextInstance { .. } => {
                        // Prevents the entire instance of damage in this event
                        None
                    }
                }
            } else {
                Some(e)
            }
        }
    }
}

/// Schedule an `instead`/`also` body effect as a `RunEffect` work item at
/// the agenda front. The frame is anchored on `source`, with the replaced
/// intent's affected recipient bound for the body to read while `This` stays
/// the source ability. A card-backed recipient binds as `EventObject` (a body
/// reads `Ref(EventObject)` — regeneration heals it, wither/infect put -1/-1
/// counters on it, [CR#702.80a,702.90c]); a PLAYER recipient (the proxy is
/// zoneless, so it has no LKI snapshot) binds as `EventActor` instead, read as
/// `Ref(EventActor)` — infect gives that player poison counters ([CR#702.90b]).
fn schedule_body(
    state: &mut GameState,
    effect: deckmaste_core::Effect,
    source: ObjectId,
    that: Option<ObjectId>,
) {
    let controller = state.objects.obj(source).controller;
    // [CR#608.2,608.2k]: bind the affected recipient — the event PATIENT
    // ([CR#120.3]) — so the body can read it while `This` falls back to
    // `source` (`bindings.this` is `None`, the agent never moving off the
    // ability). A frameless body (`that == None`) leaves bindings unset. The
    // recipient is the provenance-explicit `EventPatient`; it ALSO mirrors into
    // `that_object`/`that_player` so `EventObject`/`EventActor` bodies keep
    // reading it (a player proxy is zoneless, so it has no LKI snapshot — it
    // binds as the player patient; a card/token recipient binds as the object
    // patient).
    let endophora = that.map_or_else(Endophora::empty, |id| match state.objects.obj(id).source {
        ObjectSource::Player(p) => Endophora {
            that_player: Some(p),
            that_patient: Some(EventPatient::Player(p)),
            ..Endophora::empty()
        },
        ObjectSource::Card(_) => {
            let snapshot = LkiSnapshot::capture(state, id);
            Endophora {
                that_object: Some(snapshot.clone()),
                that_patient: Some(EventPatient::Object(snapshot)),
                ..Endophora::empty()
            }
        }
    });
    let frame = crate::stack::Frame {
        endophora,
        ..crate::stack::Frame::bare(source, controller)
    };
    state.schedule_front(vec![crate::agenda::WorkItem::RunEffect {
        effect: Box::new(effect),
        frame,
    }]);
}

/// Remove a one-shot floating replacement instance from the shields list
/// ([CR#614.3]: one-shot instances are consumed on first application).
fn consume_shield(state: &mut GameState, iid: InstanceId) {
    state.shields.retain(|s| s.id != iid);
}

// ── Task 5: ChooseReplacement decision ([CR#616.1]) ──────────────────────────

/// The player who experiences the event — they choose which replacement applies
/// first when multiple are applicable ([CR#616.1]).
pub(crate) fn affected_player(state: &GameState, e: &GameEvent) -> PlayerId {
    match affected(e) {
        Some(Affected::Player(p)) => p,
        Some(Affected::Object(o)) => state
            .objects
            .get(o)
            .map_or(state.turn.active_player, |x| x.controller),
        None => state.turn.active_player,
    }
}

/// Store a suspended replacement-loop state into `GameState.replace_state` and
/// surface a `PendingDecision::ChooseReplacement` to the pending slot.
///
/// Called when ≥ 2 applicable replacements are found for the same event.
/// The `remaining` field is set to `vec![]` here because `replace_event` is
/// called per-event from `apply_occurrence`; batch remainders are stored by
/// the `Suspend` handler in `apply_occurrence` itself (see `step.rs`).
pub(crate) fn surface_choice(
    state: &mut GameState,
    current: GameEvent,
    applied: std::collections::HashSet<ReplacementKey>,
    applicable: &[Applicable],
) {
    let chooser = affected_player(state, &current);
    let keys: Vec<ReplacementKey> = applicable.iter().map(|a| a.key).collect();
    state.replace_state = Some(crate::state::ReplaceState {
        current,
        applied,
        remaining: vec![], // batch remainders written by apply_occurrence after Suspend
    });
    state.pending = Some(crate::decide::PendingDecision::ChooseReplacement {
        chooser,
        applicable: keys,
    });
}

/// Resume the replacement loop after a `ChooseReplacement` decision. Called
/// from `submit_decision` with the chosen key and the suspended `ReplaceState`.
///
/// Applies the chosen replacement to `rs.current`, continues the replacement
/// loop on the (possibly modified) event, and then runs each `rs.remaining`
/// event through the full cant→replace→apply pipeline. The results are
/// scheduled as `WorkItem::Emit` occurrences at the agenda front.
pub(crate) fn resume_replacements(
    state: &mut GameState,
    mut rs: crate::state::ReplaceState,
    chosen_key: ReplacementKey,
) {
    // Find the chosen applicable from a fresh gather (state may have changed),
    // filtered to the key the player picked.
    let gathered = gather_applicable(state, &rs.current);
    let chosen_applicable = gathered
        .into_iter()
        .find(|a| a.key == chosen_key)
        .expect("chosen replacement key must still be applicable");

    // Apply the chosen replacement.
    rs.applied.insert(chosen_key);
    let next_event = apply_one(state, rs.current, &chosen_applicable);

    // Continue the replacement loop on the (possibly modified) event.
    let mut facts: Vec<GameEvent> = Vec::new();
    if let Some(modified) = next_event {
        // Re-enter the replacement loop: the modified event may be watched by
        // further replacements ([CR#616.1f]).
        match resume_replace_loop(state, modified, rs.applied) {
            ResumeOutcome::Fact(e) => facts.push(e),
            ResumeOutcome::Nothing => {}
            ResumeOutcome::Suspended => {
                // Another choice needed: the new surface_choice stored the
                // remaining batch in replace_state. We must not process the
                // batch remainder (rs.remaining) yet; store it back.
                if let Some(new_rs) = state.replace_state.as_mut() {
                    new_rs.remaining = rs.remaining;
                }
                return;
            }
        }
    }
    // else: event replaced to nothing — no fact.

    // Process each remaining event from the batch.
    for e in rs.remaining {
        if crate::replace_registry::cant_event(state, &e) {
            continue;
        }
        match replace_event(state, e.clone()) {
            ReplaceOutcome::Pass(e2) => {
                // Schedule apply as an Emit work item.
                state.schedule_front(vec![crate::agenda::WorkItem::Emit(
                    crate::event::Occurrence::Single(e2),
                )]);
            }
            ReplaceOutcome::Nothing => {}
            ReplaceOutcome::Suspend => {
                // Another replacement choice surfaced mid-batch. The
                // remaining events after this one need to be stored.
                // `surface_choice` already wrote replace_state for `e`;
                // we don't have the tail here, so just return — the rest
                // will be processed after that nested choice resolves.
                // (For the simple two-shield test this branch isn't hit.)
                return;
            }
        }
    }

    // Emit the facts from the chosen replacement path.
    if !facts.is_empty() {
        let occ = if facts.len() == 1 {
            crate::event::Occurrence::Single(facts.pop().unwrap())
        } else {
            crate::event::Occurrence::Batch(facts)
        };
        state.schedule_front(vec![crate::agenda::WorkItem::Emit(occ)]);
    }
}

/// Outcome of re-entering the replacement loop on an already-partially-applied
/// event during a `resume_replacements` call.
enum ResumeOutcome {
    /// The event survived the loop — apply it.
    Fact(GameEvent),
    /// The event was replaced to nothing.
    Nothing,
    /// Another `ChooseReplacement` was surfaced for this event.
    Suspended,
}

/// Continue the replacement loop on `event` with the given lineage `applied`.
/// Does NOT call `state.apply` — returns the final event for the caller to
/// schedule.
fn resume_replace_loop(
    state: &mut GameState,
    mut event: GameEvent,
    mut applied: std::collections::HashSet<ReplacementKey>,
) -> ResumeOutcome {
    loop {
        let applicable: Vec<Applicable> = gather_applicable(state, &event)
            .into_iter()
            .filter(|a| !applied.contains(&a.key))
            .collect();
        match applicable.len() {
            0 => return ResumeOutcome::Fact(event),
            1 => {
                let a = applicable.into_iter().next().unwrap();
                applied.insert(a.key);
                match apply_one(state, event, &a) {
                    Some(modified) => {
                        event = modified;
                    }
                    None => return ResumeOutcome::Nothing,
                }
            }
            _ => {
                surface_choice(state, event, applied, &applicable);
                return ResumeOutcome::Suspended;
            }
        }
    }
}

// ── End Task 5 ───────────────────────────────────────────────────────────────

#[cfg(test)]
pub(crate) mod tests_support {
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::Card;
    use deckmaste_core::CardFace;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::layer::LayeredView;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    /// A minimal two-player game state with a single vanilla 2/2 creature
    /// (player 0's) on the battlefield. No plugins loaded — the card is
    /// synthesised in Rust.
    ///
    /// Returns `(state, view, id)`. Because `LayeredView` is owned, returning
    /// it alongside the state is safe (it holds computed data, not a
    /// borrow).
    pub(crate) fn lone_creature() -> (GameState, LayeredView, ObjectId) {
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let id = mint_creature_on_battlefield(&mut state);
        let view = state.layers();
        (state, view, id)
    }

    /// Mint a synthetic vanilla 2/2 creature on the battlefield for player 0
    /// and return its `ObjectId`.
    pub(crate) fn mint_creature_on_battlefield(state: &mut GameState) -> ObjectId {
        let card = Arc::new(Card::Normal(CardFace {
            name: "Test Creature".into(),
            types: vec![Type::Creature],
            ..CardFace::default()
        }));
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// Mint a synthetic creature carrying a single `StaticEffect` on the
    /// battlefield for player 0. Returns `(state, id)`.
    pub(crate) fn creature_with_static(effect: StaticEffect) -> (GameState, ObjectId) {
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let card = Arc::new(Card::Normal(CardFace {
            name: "Test Creature".into(),
            types: vec![Type::Creature],
            abilities: vec![Ability::Static(effect)],
            ..CardFace::default()
        }));
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Agency;
    use deckmaste_core::CausePattern;
    use deckmaste_core::EventFilter;
    use deckmaste_core::Filter;
    use deckmaste_core::Reference;
    use deckmaste_core::Zone;

    use super::*;
    use crate::event::Cause;
    use crate::event::GameEvent;

    /// A `WillDestroy` intent is watched by a `Destroyed(Ref(This))` would
    /// when `this` is the dying object.
    #[test]
    fn destroyed_would_watches_will_destroy_of_self() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some(deckmaste_core::CauseVerb::Destroy),
                agency: None,
                agent: None,
            })),
        };
        let e = GameEvent::WillDestroy {
            object: id,
            cause: Some(Cause::destroy(Agency::StateBasedAction, None)),
        };
        assert!(replacement_watches(&state, &would, id, &e));
    }

    /// A sacrifice cause is NOT watched by a destruction `would`
    /// ([CR#701.21a]).
    #[test]
    fn destroyed_would_does_not_watch_sacrifice() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some(deckmaste_core::CauseVerb::Destroy),
                agency: None,
                agent: None,
            })),
        };
        let e = GameEvent::ZoneWillChange {
            object: id,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: Some(Cause::sacrifice(Agency::EffectInstruction, None)),
        };
        assert!(!replacement_watches(&state, &would, id, &e));
    }

    /// An object carrying `CantHappen(Destroyed(Ref(This)))` makes its own
    /// `WillDestroy` "can't happen" ([CR#614.17]).
    #[test]
    fn cant_happen_suppresses_own_destruction() {
        let (state, id) = super::tests_support::creature_with_static(
            deckmaste_core::StaticEffect::CantHappen(EventFilter::ZoneChange {
                what: Filter::Ref(Reference::This),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            }),
        );
        let e = GameEvent::WillDestroy {
            object: id,
            cause: None,
        };
        assert!(cant_event(&state, &e));
    }

    /// The `Cause:agent` lift in would lanes ([CR#603.2]): would-facts keep
    /// the raw engine cause triple, so an agent-narrowed pattern evaluates
    /// against the LIVE causing object — and an agentless cause (a
    /// turn-based/state-based action) still fails it.
    #[test]
    fn would_cause_agent_narrows_by_the_live_agent() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Filter::Any,
            from: None,
            to: None,
            cause: Some(deckmaste_core::Cause::Cause(deckmaste_core::CausePattern {
                verb: None,
                agency: None,
                agent: Some(Filter::creature()),
            })),
        };
        let intent = |agent| GameEvent::WillDestroy {
            object: id,
            cause: Some(crate::event::Cause::destroy(
                deckmaste_core::Agency::EffectInstruction,
                agent,
            )),
        };
        assert!(
            replacement_watches(
                &state,
                &would,
                id,
                &intent(Some((id, crate::player::PlayerId(0))))
            ),
            "a creature agent satisfies the agent narrow"
        );
        assert!(
            !replacement_watches(&state, &would, id, &intent(None)),
            "an agentless cause fails an agent-narrowed pattern"
        );
    }

    /// The non-intent-master-form lift in would lanes ([CR#601.2i,614]): a
    /// `CantHappen(Cast(who: Ref(You)))` static suppresses its controller's
    /// own cast intent and leaves an opponent's alone.
    #[test]
    fn cant_happen_cast_suppresses_matching_casts() {
        let (mut state, _watcher) = super::tests_support::creature_with_static(
            deckmaste_core::StaticEffect::CantHappen(EventFilter::Cast {
                who: Filter::Ref(Reference::You),
                what: Filter::Any,
            }),
        );
        // A spell object per caster (the fact record's actor is its
        // controller).
        let mut spell = |controller: crate::player::PlayerId| {
            let card =
                std::sync::Arc::new(deckmaste_core::Card::Normal(deckmaste_core::CardFace {
                    name: "Test Spell".into(),
                    types: vec![deckmaste_core::Type::Sorcery],
                    ..deckmaste_core::CardFace::default()
                }));
            let cid = state.cards.push(card, controller);
            state
                .objects
                .mint(ObjectSource::Card(cid), controller, Some(Zone::Stack))
        };
        let own = spell(crate::player::PlayerId(0));
        let opponents = spell(crate::player::PlayerId(1));
        assert!(
            cant_event(&state, &GameEvent::SpellCast(own)),
            "the controller's own cast can't happen"
        );
        assert!(
            !cant_event(&state, &GameEvent::SpellCast(opponents)),
            "an opponent's cast is not watched by Ref(You)"
        );
    }

    /// A creature with a static umbra-style `Instead` on itself, plus a
    /// floating shield on it → both gathered for its `WillDestroy`.
    #[test]
    fn gather_collects_static_and_floating_for_will_destroy() {
        use deckmaste_core::Duration;
        use deckmaste_core::Effect;
        use deckmaste_core::TurnMarker;

        let instead = deckmaste_core::Replacement::Instead {
            would: destroyed_self(),
            instead: Effect::Sequence(vec![]),
        };
        let (mut state, id) = tests_support::creature_with_static(StaticEffect::Replacement(
            Box::new(instead.clone()),
        ));
        state.shields.push(ReplacementInstance {
            id: InstanceId(0),
            replacement: instead,
            subject: id,
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
            one_shot: true,
            source: id,
        });
        let e = GameEvent::WillDestroy {
            object: id,
            cause: Some(Cause::destroy(Agency::StateBasedAction, None)),
        };
        let app = gather_applicable(&state, &e);
        assert_eq!(app.len(), 2);
    }

    /// The `by`-matcher ([CR#120.3], "damage dealt BY a source"): a `would`
    /// keyed `source: Ref(This)` watches a `DamageDealt` whose SOURCE is
    /// `this`, but NOT one whose source is a different object. This is the
    /// source-side filter infect/wither rely on
    /// ([CR#702.80a,702.90b,702.90c]).
    #[test]
    fn by_matcher_distinguishes_damage_source() {
        let (mut state, _view, source_a) = super::tests_support::lone_creature();
        // A second creature — the "other source".
        let source_b = super::tests_support::mint_creature_on_battlefield(&mut state);
        let target = super::tests_support::mint_creature_on_battlefield(&mut state);

        // "damage dealt BY this creature" — keyed to the watching object.
        let would = EventFilter::Damage {
            source: Filter::Ref(Reference::This),
            to: Filter::Any,
            combat: None,
            amount: None,
        };

        // `this == source_a`: damage from A fires the watch; damage from B does not.
        let from_a = GameEvent::DamageDealt {
            source: source_a,
            target,
            amount: 2,
            combat: false,
        };
        let from_b = GameEvent::DamageDealt {
            source: source_b,
            target,
            amount: 2,
            combat: false,
        };
        assert!(
            replacement_watches(&state, &would, source_a, &from_a),
            "a `source: Ref(This)` would watches damage from its own source"
        );
        assert!(
            !replacement_watches(&state, &would, source_a, &from_b),
            "it must NOT watch damage from a different source"
        );

        // A bare `source: Any` watches both (the default, source-agnostic).
        let any = EventFilter::Damage {
            source: Filter::Any,
            to: Filter::Any,
            combat: None,
            amount: None,
        };
        assert!(replacement_watches(&state, &any, source_a, &from_b));
    }

    /// Helper: the abstract `EventFilter` for "this permanent would be
    /// destroyed" (BF→GY with verb "Destroy"), as used in replacement `would`
    /// fields.
    fn destroyed_self() -> EventFilter {
        EventFilter::ZoneChange {
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some(deckmaste_core::CauseVerb::Destroy),
                agency: None,
                agent: None,
            })),
        }
    }

    /// A damage-prevention static effect prevents damage dealt to the creature.
    #[test]
    fn prevention_effect_prevents_damage() {
        use deckmaste_core::Filter;
        use deckmaste_core::Prevention;
        use deckmaste_core::Reference;

        let (mut state, id) = super::tests_support::creature_with_static(StaticEffect::Prevention(
            Box::new(Prevention::PreventAll {
                from: Filter::Any,
                to: Filter::Ref(Reference::This),
                duration: None,
            }),
        ));
        let source = super::tests_support::mint_creature_on_battlefield(&mut state);
        let e = GameEvent::DamageDealt {
            source,
            target: id,
            amount: 3,
            combat: false,
        };
        let app = gather_applicable(&state, &e);
        assert_eq!(app.len(), 1, "prevention is applicable to the damage event");

        let outcome = replace_event(&mut state, e);
        assert!(
            matches!(outcome, ReplaceOutcome::Nothing),
            "damage is fully prevented"
        );
    }

    /// A damage-prevention static effect prevents next N damage.
    #[test]
    fn prevention_effect_prevents_next_n_damage() {
        use deckmaste_core::Count;
        use deckmaste_core::Filter;
        use deckmaste_core::Prevention;
        use deckmaste_core::Reference;

        let (mut state, id) = super::tests_support::creature_with_static(StaticEffect::Prevention(
            Box::new(Prevention::PreventNext {
                n: Count::Literal(2),
                from: Filter::Any,
                to: Filter::Ref(Reference::This),
                duration: None,
            }),
        ));
        let source = super::tests_support::mint_creature_on_battlefield(&mut state);
        let e = GameEvent::DamageDealt {
            source,
            target: id,
            amount: 3,
            combat: false,
        };
        let app = gather_applicable(&state, &e);
        assert_eq!(app.len(), 1, "prevention is applicable");

        let outcome = replace_event(&mut state, e);
        if let ReplaceOutcome::Pass(GameEvent::DamageDealt { amount, .. }) = outcome {
            assert_eq!(amount, 1, "3 damage is reduced by 2 to 1");
        } else {
            panic!("expected Pass with reduced amount, got {:?}", outcome);
        }
    }

    /// The [CR#701.21a] entailment, replacement half: a PLAIN to-graveyard
    /// `would` (no cause narrow) intercepts a SACRIFICE intent — the
    /// sacrifice is structurally the entailed Battlefield→Graveyard move,
    /// no per-verb engine arm — while the Destroy-narrowed `would` does not
    /// (see `destroyed_would_does_not_watch_sacrifice`).
    #[test]
    fn graveyard_would_intercepts_a_sacrifice() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Filter::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: None,
        };
        let e = GameEvent::ZoneWillChange {
            object: id,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: Some(Cause::sacrifice(Agency::EffectInstruction, None)),
        };
        assert!(
            replacement_watches(&state, &would, id, &e),
            "a graveyard replacement intercepts a sacrifice [CR#701.21a]"
        );
        // And the SACRIFICE-narrowed would matches it too, by its verb.
        let sacrifice_would = EventFilter::ZoneChange {
            what: Filter::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some(deckmaste_core::CauseVerb::Sacrifice),
                agency: None,
                agent: None,
            })),
        };
        assert!(replacement_watches(&state, &sacrifice_would, id, &e));
    }
}
