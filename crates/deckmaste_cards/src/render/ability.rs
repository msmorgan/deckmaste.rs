//! Framing for triggered/activated/static abilities (the clause around the
//! effect).

use deckmaste_core::Ability;
use deckmaste_core::Color;
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
pub(super) fn event_clause(e: &Event, ctx: &Ctx) -> (&'static str, String) {
    match e {
        Event::Expanded(exp) => event_clause(&exp.value, ctx),
        Event::ZoneMove {
            what,
            to: Some(Zone::Battlefield),
            from: None,
            ..
        } => (lead_for(what), format!("{} enters", subject_of(what, ctx))),
        Event::ZoneMove {
            what,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            ..
        } => (lead_for(what), format!("{} dies", subject_of(what, ctx))),
        Event::StateBecomes { of, becomes, .. } => (
            "Whenever",
            format!("{} becomes {}", subject_of(of, ctx), state_word(becomes)),
        ),
        other => ("When", format!("[unrendered: {other:?}]")),
    }
}

/// One-shot enters/dies of THIS → "When"; a filtered (non-self) subject →
/// "Whenever".
fn lead_for(what: &Filter) -> &'static str {
    if matches!(
        super::fragment::strip_expanded(what),
        Filter::Ref(Reference::This)
    ) {
        "When"
    } else {
        "Whenever"
    }
}

/// A subject filter as a noun ("Baleful Strix" for the self filter,
/// "a creature" for a Creature macro filter).
fn subject_of(f: &Filter, ctx: &Ctx) -> String {
    let f = super::fragment::strip_expanded(f);
    if matches!(f, Filter::Ref(Reference::This)) {
        return ctx.subject.to_string();
    }
    if let Some(t) = super::fragment::find_card_type(f) {
        return format!("a {}", super::card::type_str(t).to_lowercase());
    }
    format!("[unrendered: {f:?}]")
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
pub(super) fn static_ability(s: &StaticAbility, ctx: &Ctx) -> Vec<String> {
    s.effects
        .iter()
        .filter_map(|e| static_effect(e, ctx))
        .collect()
}

/// Render one `StaticEffect` as a period-terminated sentence, or `None` for
/// effects that produce no text on their own.
pub(super) fn static_effect(e: &StaticEffect, ctx: &Ctx) -> Option<String> {
    match e {
        StaticEffect::Expanded(exp) => static_effect(&exp.value, ctx),
        StaticEffect::Modify { of, changes } => {
            let (subj, plural) = super::fragment::scope_subject_agreed(of, ctx);
            Some(format!(
                "{subj} {}.",
                modifications_predicate(changes, plural)
            ))
        }
        StaticEffect::Deontic(d) => Some(super::deontic::deontic(d, ctx.subject)),
        StaticEffect::Replacement(r) => Some(super::replacement::replacement(r, ctx)),
        other => Some(format!("[unrendered: {other:?}].")),
    }
}

// ── Modification predicate builder ──────────────────────────────────────────

/// Build the predicate for a `Modify` effect:
/// "get +1/+1", "are black", "lose all abilities and have base power and
/// toughness 1/1".
///
/// Clauses are emitted in the order they appear in `changes`, with three
/// exceptions:
/// - All Add/SubtractPower/Toughness deltas are summed and emitted at the
///   position of the first such modification.
/// - `SetPower` + `SetToughness` are combined into one "base P/T N/M" clause at
///   the position of the first `SetPower`/`SetToughness` in the list.
fn modifications_predicate(changes: &[Modification], plural: bool) -> String {
    let mut clauses: Vec<String> = Vec::new();

    // Pre-scan to compute the combined values for the grouped cases.
    let delta = pt_delta_clause(changes, plural);
    let base = base_pt_clause(changes, plural);

    let mut delta_emitted = false;
    let mut base_emitted = false;

    for m in changes {
        match m {
            // P/T delta group: emit once at first occurrence.
            Modification::AddPower(_)
            | Modification::AddToughness(_)
            | Modification::SubtractPower(_)
            | Modification::SubtractToughness(_) => {
                if !delta_emitted {
                    if let Some(ref d) = delta {
                        clauses.push(d.clone());
                    }
                    delta_emitted = true;
                }
            }
            // Base P/T group: emit once at first SetPower/SetToughness.
            Modification::SetPower(_) | Modification::SetToughness(_) => {
                if !base_emitted {
                    if let Some(ref b) = base {
                        clauses.push(b.clone());
                    }
                    base_emitted = true;
                }
            }
            Modification::SetColors(cs) => {
                clauses.push(format!("{} {}", be(plural), colors_phrase(cs)));
            }
            Modification::GainAbility(a) => {
                clauses.push(format!("{} {}", have(plural), ability_noun(a)));
            }
            Modification::LoseAllAbilities => {
                clauses.push(format!("{} all abilities", lose(plural)));
            }
            other => clauses.push(format!("[unrendered: {other:?}]")),
        }
    }
    if clauses.is_empty() {
        return format!("[unrendered: {changes:?}]");
    }
    clauses.join(" and ")
}

/// "+N/+N" or "-N/-N" clause from Add/Subtract Power/Toughness, or `None` if
/// none present.
fn pt_delta_clause(changes: &[Modification], plural: bool) -> Option<String> {
    let mut p: i64 = 0;
    let mut t: i64 = 0;
    let mut found = false;
    for c in changes {
        match c {
            Modification::AddPower(Count::Literal(n)) => {
                p += i64::from(*n);
                found = true;
            }
            Modification::AddToughness(Count::Literal(n)) => {
                t += i64::from(*n);
                found = true;
            }
            Modification::SubtractPower(Count::Literal(n)) => {
                p -= i64::from(*n);
                found = true;
            }
            Modification::SubtractToughness(Count::Literal(n)) => {
                t -= i64::from(*n);
                found = true;
            }
            _ => {}
        }
    }
    if !found {
        return None;
    }
    Some(format!("{} {p:+}/{t:+}", get(plural)))
}

/// "have base power and toughness N/M" from `SetPower` + `SetToughness`.
fn base_pt_clause(changes: &[Modification], plural: bool) -> Option<String> {
    let mut sp: Option<i64> = None;
    let mut st: Option<i64> = None;
    for c in changes {
        match c {
            Modification::SetPower(Count::Literal(n)) => sp = Some(i64::from(*n)),
            Modification::SetToughness(Count::Literal(n)) => st = Some(i64::from(*n)),
            _ => {}
        }
    }
    match (sp, st) {
        (Some(p), Some(t)) => Some(format!("{} base power and toughness {p}/{t}", have(plural))),
        (Some(p), None) => Some(format!("{} base power {p}", have(plural))),
        (None, Some(t)) => Some(format!("{} base toughness {t}", have(plural))),
        (None, None) => None,
    }
}

// ── Verb helpers (plural/singular) ──────────────────────────────────────────

fn get(plural: bool) -> &'static str { if plural { "get" } else { "gets" } }
fn be(plural: bool) -> &'static str { if plural { "are" } else { "is" } }
fn have(plural: bool) -> &'static str { if plural { "have" } else { "has" } }
fn lose(plural: bool) -> &'static str { if plural { "lose" } else { "loses" } }

// ── Phrase helpers ───────────────────────────────────────────────────────────

/// A list of colors as a phrase: "black", "white and blue".
fn colors_phrase(cs: &[Color]) -> String {
    cs.iter()
        .map(|&c| color_name(c))
        .collect::<Vec<_>>()
        .join(" and ")
}

fn color_name(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Blue => "blue",
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
    }
}

/// An `Ability` as a noun phrase for "have <noun>".
fn ability_noun(a: &Ability) -> String {
    match a {
        Ability::Keyword(k) => super::keyword::keyword_name(k).to_lowercase(),
        other => format!("[unrendered: {other:?}]"),
    }
}
