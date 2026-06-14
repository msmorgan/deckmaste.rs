//! Effects / actions render to imperative sentences (spell mood).

use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Duration;
use deckmaste_core::Effect;
use deckmaste_core::PlayerAction;
use deckmaste_core::TurnMarker;

use super::Ctx;
use super::fragment;

/// Render an `Effect` as one or more sentences joined into a single rules
/// string.
pub(super) fn effect(e: &Effect, ctx: &Ctx) -> String {
    match e {
        Effect::Act(a) => action(a, ctx),
        Effect::Sequence(parts) => {
            let mut out = String::new();
            for (i, p) in parts.iter().enumerate() {
                let s = trim_period(&effect(p, ctx));
                if i == 0 {
                    out.push_str(&s);
                } else {
                    out.push_str(", then ");
                    out.push_str(&super::ability::lower_first(&s));
                }
            }
            out.push('.');
            out
        }
        Effect::Expanded(e) => {
            if let Some(t) = e.template.as_deref()
                && let Some(s) = super::template::fill(t, ctx.subject, &e.args)
            {
                return ensure_period(&s);
            }
            effect(&e.value, ctx)
        }
        Effect::Continuously(c) => {
            let clause = super::ability::static_effect(&c.effect, ctx).map_or_else(
                || format!("[unrendered: {:?}]", c.effect),
                |s| trim_period(&s),
            );
            match duration_suffix(&c.duration) {
                Some(d) => format!("{clause} {d}."),
                None => format!("{clause}."),
            }
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

fn duration_suffix(d: &Duration) -> Option<String> {
    match d {
        Duration::FixedUntil(m) => Some(format!("until {}", turn_marker(*m))),
        Duration::EndOfGame => None,
        other => Some(format!("[unrendered: {other:?}]")),
    }
}

fn turn_marker(m: TurnMarker) -> &'static str {
    match m {
        TurnMarker::EndOfTurn => "end of turn",
        TurnMarker::EndOfCombat => "end of combat",
        TurnMarker::YourNextTurn => "your next turn",
    }
}

fn action(a: &Action, ctx: &Ctx) -> String {
    match a {
        Action::DealDamage(target, amount) => format!(
            "Deal {} damage to {}.",
            fragment::count(amount),
            fragment::selection(target, ctx)
        ),
        Action::Destroy(sel) => format!("Destroy {}.", fragment::selection(sel, ctx)),
        Action::By(_who, pa) => player_action(pa, ctx),
        other => format!("[unrendered: {other:?}]."),
    }
}

fn player_action(pa: &PlayerAction, ctx: &Ctx) -> String {
    match pa {
        PlayerAction::Draw(Count::Literal(1)) => "Draw a card.".to_string(),
        PlayerAction::Draw(c) => format!("Draw {} cards.", fragment::count(c)),
        PlayerAction::GainLife(c) => format!("Gain {} life.", fragment::count(c)),
        PlayerAction::LoseLife(c) => format!("Lose {} life.", fragment::count(c)),
        PlayerAction::PutInLibrary(sel, position) => format!(
            "Put {} on {} of your library.",
            fragment::selection_object(sel, ctx),
            fragment::library_position(position),
        ),
        other => format!("[unrendered: {other:?}]."),
    }
}

fn trim_period(s: &str) -> String { s.strip_suffix('.').unwrap_or(s).to_string() }

fn ensure_period(s: &str) -> String {
    if s.ends_with(['.', '!', '?']) { s.to_string() } else { format!("{s}.") }
}
