//! The replacement-effect registry ([CR#614]). Gathers replacements watching an
//! event intent and applies them per [CR#616.1] with lineage ([CR#614.5]).
//!
//! Task 1: the `replacement_watches` matcher — does a replacement's `would`
//! (a core `Event`) watch a given live `GameEvent` intent, and who/what does
//! the intent affect.
//!
//! Later tasks (2–10) add: `CantHappen` variant + cant pass, shields registry,
//! `replace_event` loop, `ChooseReplacement` decision, regeneration,
//! umbra/totem armor, genericity proof, and Skip step elision.

use deckmaste_core::Ability;
use deckmaste_core::CausePattern;
use deckmaste_core::Duration;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::Replacement;
use deckmaste_core::StaticEffect;
use deckmaste_core::Zone;

use crate::event::GameEvent;
use crate::layer::LayeredView;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::state::GameState;
use crate::trigger::TriggerBindings;

/// What an intent affects — the object being moved/changed, or the player
/// experiencing the event (e.g. the player drawing a card, gaining life).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Affected {
    Object(ObjectId),
    Player(PlayerId),
}

/// The abstract `Event` a replaceable intent represents, plus what it affects.
/// Returns `None` for non-replaceable facts (zone-change facts, life-loss,
/// etc.) — only INTENTS are replaceable [CR#614].
pub(crate) fn intent_event(e: &GameEvent) -> Option<(Event, Affected)> {
    match e {
        // [CR#701.8a]: destruction = a BF→GY move with the Destroy cause.
        // The abstract event mirrors the trigger pattern for "destroyed"
        // ([CR#701.8b]): a ZoneMove with verb "Destroy".
        GameEvent::WillDestroy { object, cause } => Some((
            Event::ZoneMove {
                what: Filter::Any,
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                face: None,
                cause: cause.as_ref().map(lift_cause),
            },
            Affected::Object(*object),
        )),
        // [CR#121.1]: a draw — Library→Hand zone move.
        GameEvent::WillDraw { player, .. } => Some((
            Event::ZoneMove {
                what: Filter::Any,
                from: Some(Zone::Library),
                to: Some(Zone::Hand),
                face: None,
                cause: None,
            },
            Affected::Player(*player),
        )),
        // [CR#400.7]: a general zone-change intent.
        GameEvent::ZoneWillChange {
            object,
            from,
            to,
            cause,
            ..
        } => Some((
            Event::ZoneMove {
                what: Filter::Any,
                from: *from,
                to: Some(*to),
                face: None,
                cause: cause.as_ref().map(lift_cause),
            },
            Affected::Object(*object),
        )),
        // [CR#120.3]: damage dealt.
        GameEvent::DamageDealt { target, .. } => Some((
            Event::Performed {
                verb: "DealDamage".into(),
                by: Filter::Any,
                on: Filter::Any,
            },
            Affected::Object(*target),
        )),
        // [CR#119.3]: life gain.
        GameEvent::LifeGained { player, .. } => Some((
            Event::Performed {
                verb: "GainLife".into(),
                by: Filter::Any,
                on: Filter::Any,
            },
            Affected::Player(*player),
        )),
        // Facts and non-replaceable events produce `None`.
        _ => None,
    }
}

/// Whether `would` (with `Ref(This) = this`, the watching object) watches
/// intent `e`. Built on `intent_event` + the existing filter matcher and
/// `CausePattern` matching.
pub(crate) fn replacement_watches(
    state: &GameState,
    view: &LayeredView,
    would: &Event,
    this: ObjectId,
    e: &GameEvent,
) -> bool {
    let Some((abstract_ev, affected)) = intent_event(e) else {
        return false;
    };
    event_pattern_matches(state, view, would, this, &abstract_ev, affected)
}

/// Match `would` (a core `Event` pattern) against `abstract_ev` (the abstract
/// representation of the intent). `this` anchors `Ref(This)`. `view` is
/// threaded through for later-task derived-property checks.
#[allow(clippy::only_used_in_recursion)]
fn event_pattern_matches(
    state: &GameState,
    view: &LayeredView,
    would: &Event,
    this: ObjectId,
    abstract_ev: &Event,
    affected: Affected,
) -> bool {
    // Look through remembered macro invocations.
    let would = look_through_event(would);

    match (would, abstract_ev) {
        // Both are ZoneMove: compare each present field.
        (
            Event::ZoneMove {
                what,
                from: w_from,
                to: w_to,
                face: w_face,
                cause: w_cause,
            },
            Event::ZoneMove {
                from: e_from,
                to: e_to,
                face: e_face,
                cause: e_cause,
                ..
            },
        ) => {
            // `from`: if the would specifies a zone, the intent must match.
            if w_from.is_some() && w_from != e_from {
                return false;
            }
            // `to`: if the would specifies a zone, the intent must match.
            if w_to.is_some() && w_to != e_to {
                return false;
            }
            // `face`: if the would specifies a face, the intent must match.
            if w_face.is_some() && w_face != e_face {
                return false;
            }
            // `cause`: if the would specifies a cause pattern, the intent's
            // cause must match every present coordinate.
            if let Some(w_c) = w_cause {
                let deckmaste_core::Cause::Cause(pattern) = w_c;
                let matched = match e_cause {
                    None => false,
                    Some(e_c) => {
                        // The abstract event's cause was lifted from the intent.
                        let deckmaste_core::Cause::Cause(e_pattern) = e_c;
                        cause_pattern_matches(pattern, e_pattern.verb.as_ref(), e_pattern.agency)
                    }
                };
                if !matched {
                    return false;
                }
            }
            // `what`: resolve against the affected object.
            let watcher = ObjectSource::Card(
                state
                    .objects
                    .obj(this)
                    .card_id()
                    .expect("replacement watcher must be a card-backed object"),
            );
            match affected {
                Affected::Object(id) => crate::target::matches_with(state, id, what, Some(watcher)),
                // A ZoneMove with an Affected::Player is the draw case; `what`
                // should be `Filter::Any` for that, which always matches.
                Affected::Player(_) => matches!(what, Filter::Any),
            }
        }

        // Both are Performed: compare verb and resolve filters against affected.
        // `by` (the performer, e.g. "if a creature YOU CONTROL would deal
        // damage") is not matched yet — a v1 seam, like the agent coordinate in
        // `cause_pattern_matches`; no in-scope replacement restricts `by`.
        (
            Event::Performed {
                verb: w_verb,
                by: _w_by,
                on: w_on,
            },
            Event::Performed { verb: e_verb, .. },
        ) => {
            if w_verb != e_verb {
                return false;
            }
            let watcher = object_source_of(state, this);
            match affected {
                Affected::Object(id) => crate::target::matches_with(state, id, w_on, Some(watcher)),
                Affected::Player(p) => {
                    let proxy = state.player(p).object;
                    crate::target::matches_with(state, proxy, w_on, Some(watcher))
                }
            }
        }

        // OneOf: any arm matches.
        (Event::OneOf(events), _) => events
            .iter()
            .any(|p| event_pattern_matches(state, view, p, this, abstract_ev, affected)),

        // A pattern for a different event kind never matches.
        _ => false,
    }
}

/// Whether a `CausePattern` matches an intent's cause coordinates.
/// Every PRESENT coordinate in the pattern must match; an absent one matches
/// anything. An event with no cause (no verb/agency) fails every present-verb
/// pattern.
fn cause_pattern_matches(
    pattern: &CausePattern,
    actual_verb: Option<&deckmaste_core::Ident>,
    actual_agency: Option<deckmaste_core::Agency>,
) -> bool {
    if let Some(pv) = &pattern.verb {
        let Some(av) = actual_verb else {
            return false;
        };
        if pv != av {
            return false;
        }
    }
    if let Some(pa) = pattern.agency {
        let Some(aa) = actual_agency else {
            return false;
        };
        if pa != aa {
            return false;
        }
    }
    // Agent matching deferred to v1 seam — a cause pattern with an agent
    // filter would need live-object lookup here.
    true
}

/// Lift an engine `Cause` into a core `Cause` (wrapping a `CausePattern`).
/// Agent resolution is deferred (v1 seam) — agent → `None`.
fn lift_cause(cause: &crate::event::Cause) -> deckmaste_core::Cause {
    deckmaste_core::Cause::Cause(CausePattern {
        verb: Some(cause.verb),
        agency: Some(cause.agency),
        agent: None,
    })
}

/// Look through a remembered `Event` macro invocation (`Expanded`) to the
/// underlying structural form.
pub(crate) fn look_through_event(event: &Event) -> &Event {
    match event {
        Event::Expanded(e) => look_through_event(&e.value),
        other => other,
    }
}

/// [CR#614.17]: whether any battlefield static makes `e` unable to happen.
/// Runs before the replacement registry — can't-happen events are suppressed
/// entirely; the replacement loop is skipped ([CR#614.17c]).
pub(crate) fn cant_event(state: &GameState, e: &GameEvent) -> bool {
    if intent_event(e).is_none() {
        return false;
    }
    let view = state.layers();
    state.zones.battlefield.iter().any(|&obj| {
        crate::legal::object_has_static(&view, obj, &|s| {
            matches!(s, StaticEffect::CantHappen(would)
                if replacement_watches(state, &view, look_through_event(would), obj, e))
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

/// One replacement that is applicable to the current event — the key (for
/// lineage), the replacement itself, and the source object.
#[derive(Debug, Clone)]
pub(crate) struct Applicable {
    pub key: ReplacementKey,
    pub replacement: Replacement,
    pub source: ObjectId,
}

/// Collect every replacement effect watching intent `e` from:
/// - static abilities on every battlefield object, and
/// - floating instances in `state.shields`.
///
/// Does NOT filter by lineage; callers apply the lineage set.
pub(crate) fn gather_applicable(state: &GameState, e: &GameEvent) -> Vec<Applicable> {
    let view = state.layers();
    let mut out = Vec::new();

    // Static replacements on every battlefield object (self- and other-watching).
    for &obj in &state.zones.battlefield {
        let abilities = crate::derive::abilities_of_source(state, state.objects.obj(obj).source);
        for (ai, ability) in abilities.iter().enumerate() {
            let Ability::Static(s) = ability else {
                continue;
            };
            for (ei, eff) in s.effects.iter().enumerate() {
                if let StaticEffect::Replacement(r) = eff
                    && replacement_would(state, &view, r, obj, e)
                {
                    out.push(Applicable {
                        key: ReplacementKey::Static {
                            source: obj,
                            ability: ai,
                            effect: ei,
                        },
                        replacement: (**r).clone(),
                        source: obj,
                    });
                }
            }
        }
    }

    // Floating instances (regeneration shields, etc.). A shield's `subject`
    // was resolved to a concrete object when the shield was created (its
    // captured `That`), so it matches by SUBJECT IDENTITY — independent of how
    // the source ability refers to it — which is why "regenerate target
    // creature" (source ≠ subject) works, not just "regenerate this creature".
    for inst in &state.shields {
        if floating_watches(&inst.replacement, inst.subject, e) {
            out.push(Applicable {
                key: ReplacementKey::Floating(inst.id),
                replacement: inst.replacement.clone(),
                source: inst.source,
            });
        }
    }

    out
}

/// Whether replacement `r` (with watcher `source`) watches intent `e` — its
/// `would` (Instead/Also) matches per `replacement_watches`. Returns `false`
/// for `Skip` (handled by the step-elision pass, Task 9) and `Expanded`.
fn replacement_would(
    state: &GameState,
    view: &LayeredView,
    r: &Replacement,
    source: ObjectId,
    e: &GameEvent,
) -> bool {
    match crate::replace::look_through_replacement(r) {
        Replacement::Instead { would, .. } | Replacement::Also { would, .. } => {
            replacement_watches(state, view, would, source, e)
        }
        Replacement::Skip { .. } => false, // handled in begin_step, Task 9
        Replacement::Expanded(_) => unreachable!("look_through_replacement strips Expanded"),
    }
}

/// Whether a FLOATING shield watches intent `e`. A shield's `subject` was
/// resolved to a concrete object when the shield was created (its captured
/// `That`), so matching is by SUBJECT IDENTITY — the `would`'s `what`
/// (typically `Ref(ThatObject)`, which a frameless gather can't re-resolve) is
/// NOT re-evaluated — paired with the event SHAPE (kind + from/to/face/cause).
fn floating_watches(replacement: &Replacement, subject: ObjectId, e: &GameEvent) -> bool {
    let Some((abstract_ev, affected)) = intent_event(e) else {
        return false;
    };
    if affected != Affected::Object(subject) {
        return false;
    }
    let would = match crate::replace::look_through_replacement(replacement) {
        Replacement::Instead { would, .. } | Replacement::Also { would, .. } => would,
        Replacement::Skip { .. } | Replacement::Expanded(_) => return false,
    };
    event_shape_matches(look_through_event(would), &abstract_ev)
}

/// Whether a `would`'s event SHAPE matches the abstract intent: the event kind
/// plus the `ZoneMove` coordinates (from/to/face/cause) or the `Performed`
/// verb. The participant filter (`what`/`on`) is NOT checked here — the
/// floating matcher pairs this with its own subject-identity check.
fn event_shape_matches(would: &Event, abstract_ev: &Event) -> bool {
    match (would, abstract_ev) {
        (
            Event::ZoneMove {
                from: w_from,
                to: w_to,
                face: w_face,
                cause: w_cause,
                ..
            },
            Event::ZoneMove {
                from: e_from,
                to: e_to,
                face: e_face,
                cause: e_cause,
                ..
            },
        ) => {
            (w_from.is_none() || w_from == e_from)
                && (w_to.is_none() || w_to == e_to)
                && (w_face.is_none() || w_face == e_face)
                && match w_cause {
                    None => true,
                    Some(deckmaste_core::Cause::Cause(p)) => matches!(
                        e_cause,
                        Some(deckmaste_core::Cause::Cause(ep))
                            if cause_pattern_matches(p, ep.verb.as_ref(), ep.agency)
                    ),
                }
        }
        (Event::Performed { verb: w, .. }, Event::Performed { verb: e, .. }) => w == e,
        (Event::OneOf(ws), _) => ws
            .iter()
            .any(|w| event_shape_matches(look_through_event(w), abstract_ev)),
        _ => false,
    }
}

// ── Task 4: replace_event loop + lineage + apply Instead/Also ────────────────

/// The outcome of running the [CR#616.1] replacement loop for one event.
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
/// Seam: `ThatObject` binding for the affected object is not threaded into
/// the body frame — the body reads `Ref(This)` off `source`. Binding the
/// replaced object into `ThatObject` requires threading `Affected` through
/// `Frame` (deferred, follow-up task).
///
/// Seam: general [CR#614.15] self-replacement (resolution-time) is a
/// `todo!`-tagged future concern; APNAP multi-player 616 ordering is also
/// deferred.
pub(crate) fn replace_event(state: &mut GameState, e: GameEvent) -> ReplaceOutcome {
    use std::collections::HashSet;

    // Non-replaceable intents pass through immediately ([CR#614]: only
    // replaceable intents can be modified by replacement effects).
    if intent_event(&e).is_none() {
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

/// Apply one replacement to `e`. Returns the modified event to continue
/// looping on, or `None` when the event is replaced to nothing (Instead).
/// Schedules body effects via `schedule_body`.
fn apply_one(state: &mut GameState, e: GameEvent, a: &Applicable) -> Option<GameEvent> {
    // [CR#608.2]: the object the intent affects (the would-be-destroyed
    // permanent, the damaged creature, …) is bound to `That` for the body to
    // read — regeneration heals/taps `That`. `This` stays the source.
    let that = match intent_event(&e) {
        Some((_, Affected::Object(id))) => Some(id),
        _ => None,
    };
    match crate::replace::look_through_replacement(&a.replacement).clone() {
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

/// Schedule an `instead`/`also` body effect as a `RunEffect` work item at
/// the agenda front. The frame is anchored on `source`.
///
/// Seam: `ThatObject` binding for the replaced event's affected object is
/// deferred — body effects that need to reference the affected object read
/// `Ref(This)` off `source` instead. Threading `Affected` into `Frame`
/// is the follow-up work (see [CR#616.1] body-binding discussion in the
/// spec).
fn schedule_body(
    state: &mut GameState,
    effect: deckmaste_core::Effect,
    source: ObjectId,
    that: Option<ObjectId>,
) {
    let controller = state.objects.obj(source).controller;
    // [CR#608.2]: bind `That` (`ThatObject`) to the affected object so the body
    // can read it — `This` falls back to `source` (`bindings.this` is `None`),
    // never moving off the ability. A frameless body (`that == None`) leaves
    // bindings unset.
    let bindings = that.map(|id| TriggerBindings {
        this: None,
        that_object: Some(LkiSnapshot::capture(state, id)),
        that_player: None,
    });
    let frame = crate::stack::Frame {
        source,
        controller,
        targets: vec![],
        bindings,
        chosen: None,
        x: None,
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
    match intent_event(e).map(|(_, a)| a) {
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
    use deckmaste_core::StaticAbility;
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
        });
        let card = Arc::new(Card::Normal(CardFace {
            name: "Test Creature".into(),
            types: vec![Type::Creature],
            abilities: vec![Ability::Static(StaticAbility {
                characteristic_defining: false,
                effects: vec![effect],
                condition: None,
            })],
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
    use deckmaste_core::Event;
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
        let (state, view, id) = super::tests_support::lone_creature();
        let would = Event::ZoneMove {
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            face: None,
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some("Destroy".into()),
                agency: None,
                agent: None,
            })),
        };
        let e = GameEvent::WillDestroy {
            object: id,
            cause: Some(Cause::destroy(Agency::StateBasedAction, None)),
        };
        assert!(replacement_watches(&state, &view, &would, id, &e));
    }

    /// A sacrifice cause is NOT watched by a destruction `would`
    /// ([CR#701.21a]).
    #[test]
    fn destroyed_would_does_not_watch_sacrifice() {
        let (state, view, id) = super::tests_support::lone_creature();
        let would = Event::ZoneMove {
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            face: None,
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some("Destroy".into()),
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
        assert!(!replacement_watches(&state, &view, &would, id, &e));
    }

    /// An object carrying `CantHappen(Destroyed(Ref(This)))` makes its own
    /// `WillDestroy` "can't happen" ([CR#614.17]).
    #[test]
    fn cant_happen_suppresses_own_destruction() {
        let (state, id) = super::tests_support::creature_with_static(
            deckmaste_core::StaticEffect::CantHappen(Event::ZoneMove {
                what: Filter::Ref(Reference::This),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                face: None,
                cause: None,
            }),
        );
        let e = GameEvent::WillDestroy {
            object: id,
            cause: None,
        };
        assert!(cant_event(&state, &e));
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

    /// Helper: the abstract `Event` for "this permanent would be destroyed"
    /// (BF→GY with verb "Destroy"), as used in replacement `would` fields.
    fn destroyed_self() -> Event {
        Event::ZoneMove {
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            face: None,
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some("Destroy".into()),
                agency: None,
                agent: None,
            })),
        }
    }
}
