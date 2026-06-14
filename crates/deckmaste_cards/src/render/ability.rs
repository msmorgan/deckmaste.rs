//! Framing for triggered/activated/static abilities (the clause around the
//! effect).

use deckmaste_core::Count;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::Modification;
use deckmaste_core::Reference;
use deckmaste_core::StateFilterEvent;
use deckmaste_core::StaticAbility;
use deckmaste_core::StaticEffect;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Zone;

use super::CardView;
use super::Ctx;
use super::effect;

/// "When/Whenever <event>, <effect>."
pub(super) fn triggered(t: &TriggeredAbility, view: &CardView) -> String {
    let ctx = Ctx {
        subject: view.name,
        targets: &t.targets,
    };
    let (lead, clause) = event_clause(&t.event, &ctx);
    let body = lower_first(&effect::effect(&t.effect, &ctx));
    format!("{lead} {clause}, {body}")
}

/// Returns (lead word, the event clause).
/// "When", "Baleful Strix enters" | "Whenever", "Goblin Medics becomes tapped".
fn event_clause(e: &Event, ctx: &Ctx) -> (&'static str, String) {
    match e {
        Event::Expanded(exp) => event_clause(&exp.value, ctx),
        Event::ZoneMove {
            what,
            to: Some(Zone::Battlefield),
            ..
        } => ("When", format!("{} enters", subject_of(what, ctx))),
        Event::StateBecomes { of, becomes, .. } => (
            "Whenever",
            format!("{} becomes {}", subject_of(of, ctx), state_word(becomes)),
        ),
        other => ("When", format!("[unrendered: {other:?}]")),
    }
}

/// A subject filter as a noun ("Baleful Strix" for the self filter).
fn subject_of(f: &Filter, ctx: &Ctx) -> String {
    match f {
        Filter::Expanded(exp) => subject_of(&exp.value, ctx),
        Filter::Ref(Reference::This) => ctx.subject.to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

fn state_word(s: &StateFilterEvent) -> &'static str {
    match s {
        StateFilterEvent::Tapped => "tapped",
        StateFilterEvent::Untapped => "untapped",
        StateFilterEvent::Attacking => "attacking",
        StateFilterEvent::Blocked => "blocked",
        _ => "[unrendered]",
    }
}

pub(super) fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_lowercase().chain(c).collect(),
        None => String::new(),
    }
}

// ── Static abilities ─────────────────────────────────────────────────────────

/// Render a `Static` ability's effects, one rules string each.
pub(super) fn static_ability(s: &StaticAbility, subject: &str) -> Vec<String> {
    s.effects
        .iter()
        .filter_map(|e| static_effect(e, subject))
        .collect()
}

fn static_effect(e: &StaticEffect, subject: &str) -> Option<String> {
    match e {
        StaticEffect::Expanded(exp) => static_effect(&exp.value, subject),
        StaticEffect::Modify { of, changes } => Some(format!(
            "{} get {}.",
            super::fragment::scope_subject(of),
            pt_delta(changes)
        )),
        StaticEffect::Deontic(d) => Some(super::deontic::deontic(d, subject)),
        other => Some(format!("[unrendered: {other:?}].")),
    }
}

/// "+1/+1", "+2/+2", "-2/-2" from Add/Subtract Power/Toughness modifications.
fn pt_delta(changes: &[Modification]) -> String {
    let mut p: i64 = 0;
    let mut t: i64 = 0;
    let mut ok = true;
    for c in changes {
        match c {
            Modification::AddPower(Count::Literal(n)) => p += i64::from(*n),
            Modification::AddToughness(Count::Literal(n)) => t += i64::from(*n),
            Modification::SubtractPower(Count::Literal(n)) => p -= i64::from(*n),
            Modification::SubtractToughness(Count::Literal(n)) => t -= i64::from(*n),
            _ => ok = false,
        }
    }
    if !ok {
        return format!("[unrendered: {changes:?}]");
    }
    format!("{p:+}/{t:+}")
}
