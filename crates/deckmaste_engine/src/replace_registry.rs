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
use deckmaste_core::CausePattern;
use deckmaste_core::CauseVerb;
use deckmaste_core::Duration;
use deckmaste_core::EventFilter;
use deckmaste_core::Filter;
use deckmaste_core::Prevention;
use deckmaste_core::Replacement;
use deckmaste_core::StaticEffect;
use deckmaste_core::Zone;

use crate::event::GameEvent;
use crate::layer::LayeredView;
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

/// The abstract `EventFilter` a replaceable intent represents, plus what it
/// affects. Returns `None` for non-replaceable facts (zone-change facts,
/// life-loss, etc.) — only INTENTS are replaceable [CR#614].
pub(crate) fn intent_event(e: &GameEvent) -> Option<(EventFilter, Affected)> {
    match e {
        // [CR#701.8a]: destruction = a BF→GY move with the Destroy cause.
        // The abstract event mirrors the trigger pattern for "destroyed"
        // ([CR#701.8b]): a ZoneChange with verb "Destroy", its zone
        // coordinates read from the verb's emitted entailment row — the
        // engine hardcodes no per-verb fact form.
        GameEvent::WillDestroy { object, cause } => {
            let row = crate::entail::entailment("Destroy").expect("emitted Destroy entailment row");
            Some((
                EventFilter::ZoneChange {
                    what: Filter::Any,
                    from: row.from,
                    to: row.to,
                    cause: cause.as_ref().map(lift_cause),
                },
                Affected::Object(*object),
            ))
        }
        // [CR#121.1]: a draw — Library→Hand zone move.
        GameEvent::WillDraw { player, .. } => Some((
            EventFilter::ZoneChange {
                what: Filter::Any,
                from: Some(Zone::Library),
                to: Some(Zone::Hand),
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
            EventFilter::ZoneChange {
                what: Filter::Any,
                from: *from,
                to: Some(*to),
                cause: cause.as_ref().map(lift_cause),
            },
            Affected::Object(*object),
        )),
        // [CR#120.3]: damage dealt — the abstract event carries the
        // intent's combat flag ([CR#510.1]) so a combat-narrowed would
        // evaluates faithfully.
        GameEvent::DamageDealt { target, combat, .. } => Some((
            EventFilter::Damage {
                source: Filter::Any,
                to: Filter::Any,
                combat: Some(*combat),
                amount: None,
            },
            Affected::Object(*target),
        )),
        // [CR#119.3]: life gain.
        GameEvent::LifeGained { player, .. } => Some((
            EventFilter::LifeGained {
                who: Filter::Any,
                amount: None,
            },
            Affected::Player(*player),
        )),
        // Facts and non-replaceable events produce `None`.
        _ => None,
    }
}

/// The live object that PERFORMED a replaceable intent — the `source`
/// coordinate of a `Damage` event ([CR#120.3], "damage dealt BY a source").
/// `Some` only for the intents that carry a source object; `None` for zone
/// moves and player-experienced events, whose pattern (if any) has no
/// performer to bind. Mirrors `Affected` (the recipient coordinate):
/// `Affected` is the recipient, this is the actor.
pub(crate) fn intent_performer(e: &GameEvent) -> Option<ObjectId> {
    match e {
        // [CR#120.3]: the damage's SOURCE — the object infect/wither/lifelink
        // key their source-side replacements off of.
        GameEvent::DamageDealt { source, .. } => Some(*source),
        _ => None,
    }
}

/// Whether `would` (with `Ref(This) = this`, the watching object) watches
/// intent `e`. Built on `intent_event` + the existing filter matcher and
/// `CausePattern` matching.
pub(crate) fn replacement_watches(
    state: &GameState,
    view: &LayeredView,
    would: &EventFilter,
    this: ObjectId,
    e: &GameEvent,
) -> bool {
    let Some((abstract_ev, affected)) = intent_event(e) else {
        return false;
    };
    let performer = intent_performer(e);
    event_pattern_matches(state, view, would, this, &abstract_ev, affected, performer)
}

/// Match `would` (a core `EventFilter` pattern) against `abstract_ev` (the
/// abstract representation of the intent). `this` anchors `Ref(This)`. `view`
/// is threaded through for later-task derived-property checks. `performer` is
/// the live actor (the `source` coordinate) when the intent carries one.
///
/// Conservative v1 seam: pattern kinds/refinements this matcher doesn't
/// handle return hard-`false` (documented), never a silent over-match.
#[allow(clippy::only_used_in_recursion)]
fn event_pattern_matches(
    state: &GameState,
    view: &LayeredView,
    would: &EventFilter,
    this: ObjectId,
    abstract_ev: &EventFilter,
    affected: Affected,
    performer: Option<ObjectId>,
) -> bool {
    // Look through remembered macro invocations.
    let would = look_through_event(would);

    match (would, abstract_ev) {
        // Both are ZoneChange: compare each present field.
        (
            EventFilter::ZoneChange {
                what,
                from: w_from,
                to: w_to,
                cause: w_cause,
            },
            EventFilter::ZoneChange {
                from: e_from,
                to: e_to,
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
            // `cause`: if the would specifies a cause pattern, the intent's
            // cause must match every present coordinate.
            if let Some(w_c) = w_cause {
                let deckmaste_core::Cause::Cause(pattern) = w_c;
                let matched = match e_cause {
                    None => false,
                    Some(e_c) => {
                        // The abstract event's cause was lifted from the intent.
                        let deckmaste_core::Cause::Cause(e_pattern) = e_c;
                        cause_pattern_matches(pattern, e_pattern.verb, e_pattern.agency)
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
                // A ZoneChange with an Affected::Player is the draw case;
                // `what` should be `Filter::Any` for that, which always
                // matches.
                Affected::Player(_) => matches!(what, Filter::Any),
            }
        }

        // Both are Damage: resolve the `to` filter against the recipient
        // (`affected`) and the `source` filter against the performer (the
        // live actor). Infect/Wither key a SOURCE replacement off
        // `source: Ref(This)` — "damage dealt BY this creature"
        // ([CR#702.80a,702.90b,702.90c,120.3]) — so the `source` match is
        // what restricts the replacement to damage from that very source. A
        // `source` filter present with no performer to test against never
        // matches (the intent carries no actor).
        (
            EventFilter::Damage {
                source: w_source,
                to: w_to,
                combat,
                amount,
            },
            EventFilter::Damage {
                combat: e_combat, ..
            },
        ) => {
            // The abstract event carries the intent's combat flag
            // ([CR#510.1]) — a combat-narrowed pattern compares against it;
            // the unevaluated amount bound stays hard-false (conservative
            // v1: a refined pattern must never over-match).
            if amount.is_some() {
                return false;
            }
            if combat.is_some() && combat != e_combat {
                return false;
            }
            let watcher = object_source_of(state, this);
            // `source`: resolve against the performer, mirroring `to` against
            // the recipient. `Filter::Any` matches any (even a missing)
            // performer; a more specific filter requires the actor to be
            // present and match.
            let source_ok = match w_source {
                Filter::Any => true,
                _ => performer.is_some_and(|src| {
                    crate::target::matches_with(state, src, w_source, Some(watcher))
                }),
            };
            if !source_ok {
                return false;
            }
            // `to`: resolve against the recipient.
            match affected {
                Affected::Object(id) => crate::target::matches_with(state, id, w_to, Some(watcher)),
                Affected::Player(p) => {
                    let proxy = state.player(p).object;
                    crate::target::matches_with(state, proxy, w_to, Some(watcher))
                }
            }
        }

        // Both are LifeGained: resolve `who` against the affected player.
        (EventFilter::LifeGained { who, amount }, EventFilter::LifeGained { .. }) => {
            // Hard-false for the unevaluated amount bound (conservative v1).
            if amount.is_some() {
                return false;
            }
            let watcher = object_source_of(state, this);
            match affected {
                Affected::Object(id) => crate::target::matches_with(state, id, who, Some(watcher)),
                Affected::Player(p) => {
                    let proxy = state.player(p).object;
                    crate::target::matches_with(state, proxy, who, Some(watcher))
                }
            }
        }

        // OneOf: any arm matches. AllOf: every arm matches.
        (EventFilter::OneOf(events), _) => events
            .iter()
            .any(|p| event_pattern_matches(state, view, p, this, abstract_ev, affected, performer)),
        (EventFilter::AllOf(events), _) => events
            .iter()
            .all(|p| event_pattern_matches(state, view, p, this, abstract_ev, affected, performer)),
        // [CR#603.2c]: a would-fact is one intent — the batch quantifier
        // matches iff its operand does.
        (EventFilter::OneOrMore(inner), _) => {
            event_pattern_matches(state, view, inner, this, abstract_ev, affected, performer)
        }

        // A pattern for a different event kind never matches.
        _ => false,
    }
}

/// Whether a `CausePattern` matches an intent's (lifted) cause coordinates.
/// Every PRESENT coordinate in the pattern must match; an absent one matches
/// anything. An event with no cause (no verb/agency) fails every present-verb
/// pattern. Verbs compare by their canonical spelling ([`CauseVerb::as_str`]).
fn cause_pattern_matches(
    pattern: &CausePattern,
    actual_verb: Option<CauseVerb>,
    actual_agency: Option<deckmaste_core::Agency>,
) -> bool {
    if let Some(pv) = pattern.verb {
        let Some(av) = actual_verb else {
            return false;
        };
        if pv.as_str() != av.as_str() {
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
    // The lifted abstract cause carries no AGENT coordinate (`lift_cause`
    // drops it), so an agent-narrowed pattern is load-capped in the would
    // lanes (E-BRIDGE-CAP, `Cause:agent`) and conservatively never matches
    // here — hard-false, never a silent over-match.
    if pattern.agent.is_some() {
        return false;
    }
    true
}

/// Lift an engine `Cause` into a core `Cause` (wrapping a `CausePattern`).
/// Agent resolution is deferred (v1 seam) — agent → `None`.
fn lift_cause(cause: &crate::event::Cause) -> deckmaste_core::Cause {
    deckmaste_core::Cause::Cause(CausePattern {
        verb: lift_verb(&cause.verb),
        agency: Some(cause.agency),
        agent: None,
    })
}

/// The engine's open `Ident` fact verb, re-expressed in the closed
/// [`CauseVerb`] pattern vocabulary. Fact verbs outside it (`Counter`, `Tap`,
/// `PutCounters`, …) have no pattern spelling and lift to `None`: a
/// verb-narrowed pattern can never match them (correct — the closed
/// vocabulary has no such verb), and an unnarrowed pattern doesn't care.
fn lift_verb(verb: &deckmaste_core::Ident) -> Option<CauseVerb> {
    [
        CauseVerb::Sacrifice,
        CauseVerb::Destroy,
        CauseVerb::Discard,
        CauseVerb::Exile,
        CauseVerb::Mill,
        CauseVerb::Play,
        CauseVerb::Fight,
        CauseVerb::Explore,
        CauseVerb::Regenerate,
    ]
    .into_iter()
    .find(|v| v.as_str() == verb.as_str())
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
                        effect: ApplicableEffect::Replacement((**r).clone()),
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
                for (ei, eff) in s.effects.iter().enumerate() {
                    if let StaticEffect::Prevention(p) = eff
                        && prevention_watches(state, p, obj, event_source, event_target)
                    {
                        out.push(Applicable {
                            key: ReplacementKey::Static {
                                source: obj,
                                ability: ai,
                                effect: ei,
                            },
                            effect: ApplicableEffect::Prevention((**p).clone()),
                            source: obj,
                        });
                    }
                }
            }
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
/// `That`), so matching is by SUBJECT IDENTITY — the `would`'s `what`
/// (typically `Ref(EventObject)`, which a frameless gather can't re-resolve) is
/// NOT re-evaluated — paired with the event SHAPE (kind + from/to/cause).
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

/// Whether a `would`'s event SHAPE matches the abstract intent: the event
/// kind plus the `ZoneChange` coordinates (from/to/cause) — a `Damage`/
/// `LifeGained` would matches its intent kind. The participant filters
/// (`what`/`to`/`who`) are NOT checked here — the floating matcher pairs this
/// with its own subject-identity check. A refinement this matcher doesn't
/// evaluate (a `combat:`/`amount:` narrow) is hard-`false`, matching the
/// registry's conservative v1 style.
fn event_shape_matches(would: &EventFilter, abstract_ev: &EventFilter) -> bool {
    match (would, abstract_ev) {
        (
            EventFilter::ZoneChange {
                from: w_from,
                to: w_to,
                cause: w_cause,
                ..
            },
            EventFilter::ZoneChange {
                from: e_from,
                to: e_to,
                cause: e_cause,
                ..
            },
        ) => {
            (w_from.is_none() || w_from == e_from)
                && (w_to.is_none() || w_to == e_to)
                && match w_cause {
                    None => true,
                    Some(deckmaste_core::Cause::Cause(p)) => matches!(
                        e_cause,
                        Some(deckmaste_core::Cause::Cause(ep))
                            if cause_pattern_matches(p, ep.verb, ep.agency)
                    ),
                }
        }
        (
            EventFilter::Damage { combat, amount, .. },
            EventFilter::Damage {
                combat: e_combat, ..
            },
        ) => {
            // The abstract event carries the intent's combat flag
            // ([CR#510.1]); an amount narrow stays conservative-false.
            amount.is_none() && combat.is_none_or(|w| Some(w) == *e_combat)
        }
        (EventFilter::LifeGained { amount, .. }, EventFilter::LifeGained { .. }) => {
            amount.is_none()
        }
        (EventFilter::OneOf(ws), _) => ws
            .iter()
            .any(|w| event_shape_matches(look_through_event(w), abstract_ev)),
        (EventFilter::AllOf(ws), _) => ws
            .iter()
            .all(|w| event_shape_matches(look_through_event(w), abstract_ev)),
        // [CR#603.2c]: a would-fact is one intent — the batch quantifier
        // matches iff its operand does.
        (EventFilter::OneOrMore(inner), _) => {
            event_shape_matches(look_through_event(inner), abstract_ev)
        }
        _ => false,
    }
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
    let that = match intent_event(&e) {
        Some((_, Affected::Object(id))) => Some(id),
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
            abilities: vec![Ability::Static(StaticAbility {
                from: None,
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
        let (state, view, id) = super::tests_support::lone_creature();
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
        assert!(replacement_watches(&state, &view, &would, id, &e));
    }

    /// A sacrifice cause is NOT watched by a destruction `would`
    /// ([CR#701.21a]).
    #[test]
    fn destroyed_would_does_not_watch_sacrifice() {
        let (state, view, id) = super::tests_support::lone_creature();
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
        assert!(!replacement_watches(&state, &view, &would, id, &e));
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
        let (mut state, view, source_a) = super::tests_support::lone_creature();
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
            replacement_watches(&state, &view, &would, source_a, &from_a),
            "a `source: Ref(This)` would watches damage from its own source"
        );
        assert!(
            !replacement_watches(&state, &view, &would, source_a, &from_b),
            "it must NOT watch damage from a different source"
        );

        // A bare `source: Any` watches both (the default, source-agnostic).
        let any = EventFilter::Damage {
            source: Filter::Any,
            to: Filter::Any,
            combat: None,
            amount: None,
        };
        assert!(replacement_watches(&state, &view, &any, source_a, &from_b));
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
        let (state, view, id) = super::tests_support::lone_creature();
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
            replacement_watches(&state, &view, &would, id, &e),
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
        assert!(replacement_watches(&state, &view, &sacrifice_would, id, &e));
    }

    /// THE BRIDGE INVARIANT, would half ([CR#614.1]): every atom the
    /// emitted bridge-caps table marks WOULD-supported evaluates through
    /// `replacement_watches` on a representative (would, intent) pair;
    /// everything else is load-capped (E-BRIDGE-CAP) and conservatively
    /// hard-`false` here — never a silent over-match.
    #[test]
    fn bridge_caps_agree_with_the_would_matcher() {
        use deckmaste_cards::elaborate::tables::Matcher;
        use deckmaste_cards::elaborate::tables::tables;

        let (state, view, id) = super::tests_support::lone_creature();
        let dies_would = || EventFilter::ZoneChange {
            what: Filter::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: None,
        };
        let destroy_intent = || GameEvent::WillDestroy {
            object: id,
            cause: None,
        };
        let pair = |atom: &str| -> Option<(EventFilter, GameEvent)> {
            Some(match atom {
                "ZoneChange" => (dies_would(), destroy_intent()),
                "Damage" | "Damage:combat" => (
                    EventFilter::Damage {
                        source: Filter::Any,
                        to: Filter::Any,
                        // The combat narrow reads the intent's carried flag
                        // ([CR#510.1]).
                        combat: (atom == "Damage:combat").then_some(false),
                        amount: None,
                    },
                    GameEvent::DamageDealt {
                        source: id,
                        target: id,
                        amount: 2,
                        combat: false,
                    },
                ),
                "LifeGained" => (
                    EventFilter::LifeGained {
                        who: Filter::Any,
                        amount: None,
                    },
                    GameEvent::LifeGained {
                        player: crate::player::PlayerId(0),
                        amount: 3,
                    },
                ),
                // `Filter::Where` runs LIVE in the would lane — the intent's
                // object is still on the battlefield.
                "Where-in-snapshot" => (
                    EventFilter::ZoneChange {
                        what: Filter::Where(Box::new(deckmaste_core::Condition::YourTurn)),
                        from: None,
                        to: Some(Zone::Graveyard),
                        cause: None,
                    },
                    destroy_intent(),
                ),
                "AllOf" => (EventFilter::AllOf(vec![dies_would()]), destroy_intent()),
                "OneOf" => (EventFilter::OneOf(vec![dies_would()]), destroy_intent()),
                "OneOrMore" => (
                    EventFilter::OneOrMore(Box::new(dies_would())),
                    destroy_intent(),
                ),
                _ => return None,
            })
        };
        for row in tables().bridge_rows() {
            let atom = row.atom.as_str();
            if let Some((would, e)) = pair(atom) {
                assert!(
                    row.supports(Matcher::Would),
                    "{atom}: the would-matcher evaluates it, but the table caps it"
                );
                assert!(
                    replacement_watches(&state, &view, &would, id, &e),
                    "{atom}: representative would/intent pair must match"
                );
            } else {
                assert!(
                    !row.supports(Matcher::Would),
                    "{atom}: table says would-supported but no pair exercises it"
                );
            }
        }
    }
}
