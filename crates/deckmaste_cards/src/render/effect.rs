//! Effects / actions render to imperative sentences (spell mood).

use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Effect;
use deckmaste_core::PlayerAction;

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
        other => format!("[unrendered: {other:?}]."),
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
