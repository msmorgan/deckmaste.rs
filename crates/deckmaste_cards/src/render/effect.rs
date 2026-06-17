//! Effects / actions render to imperative sentences (spell mood).

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::Duration;
use deckmaste_core::Effect;
use deckmaste_core::PlayerAction;
use deckmaste_core::StatValue;
use deckmaste_core::Token;
use deckmaste_core::TokenSpec;
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
        Effect::Expanded(e) => match super::template::expanded(e, ctx.subject) {
            Some(s) => ensure_period(&s),
            None => effect(&e.value, ctx),
        },
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
        PlayerAction::Create(count, spec) => create_text(count, spec),
        PlayerAction::Tap(sel) => format!("Tap {}.", fragment::selection(sel, ctx)),
        PlayerAction::Untap(sel) => format!("Untap {}.", fragment::selection(sel, ctx)),
        PlayerAction::GetDesignation(name) if name.as_ref() == "CitysBlessing" => {
            "You get the city's blessing.".to_string()
        }
        PlayerAction::GetDesignation(name) => format!("You get {name}."),
        other => format!("[unrendered: {other:?}]."),
    }
}

// ── Token creation
// ────────────────────────────────────────────────────────────

fn create_text(count: &Count, spec: &TokenSpec) -> String {
    match spec {
        TokenSpec::Token(t) => {
            let plural = !matches!(count, Count::Literal(1));
            let count_word = token_count_word(count);
            let descriptor = token_descriptor(t);
            let noun = if plural { "tokens" } else { "token" };
            let abilities_suffix = token_abilities_suffix(&t.abilities);
            format!("Create {count_word} {descriptor} {noun}{abilities_suffix}.")
        }
    }
}

fn token_count_word(count: &Count) -> &'static str {
    match count {
        Count::Literal(1) => "a",
        Count::Literal(2) => "two",
        Count::Literal(3) => "three",
        Count::Literal(4) => "four",
        Count::Literal(5) => "five",
        Count::Literal(6) => "six",
        Count::Literal(7) => "seven",
        Count::Literal(8) => "eight",
        Count::Literal(9) => "nine",
        Count::X => "X",
        _ => "some",
    }
}

fn token_descriptor(t: &Token) -> String {
    let mut parts: Vec<String> = Vec::new();

    // P/T
    if let (Some(p), Some(toughness)) = (&t.power, &t.toughness) {
        let ps = stat_value_str(p);
        let ts = stat_value_str(toughness);
        parts.push(format!("{ps}/{ts}"));
    }

    // Colors
    for color in &t.color_indicator {
        parts.push(color_word(*color).to_string());
    }

    // Supertypes
    for s in &t.supertypes {
        parts.push(super::card::supertype_str(*s).to_lowercase());
    }

    // Subtypes (proper-cased names)
    for s in &t.subtypes {
        parts.push(s.name.to_string());
    }

    // Types
    for ty in &t.types {
        parts.push(super::card::type_str(*ty).to_lowercase());
    }

    parts.join(" ")
}

fn stat_value_str(v: &StatValue) -> String {
    match v {
        StatValue::Number(n) => n.to_string(),
        _ => "*".to_string(),
    }
}

pub(super) fn color_word(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Blue => "blue",
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
    }
}

fn token_abilities_suffix(abilities: &[Ability]) -> String {
    if abilities.is_empty() {
        return String::new();
    }
    let mut kw_names: Vec<String> = Vec::new();
    for ability in abilities {
        if let Ability::Keyword(k) = ability {
            kw_names.push(super::keyword::keyword_name(k).to_lowercase());
        }
    }
    if kw_names.is_empty() {
        return String::new();
    }
    let joined = match kw_names.len() {
        1 => kw_names.into_iter().next().unwrap(),
        2 => format!("{} and {}", kw_names[0], kw_names[1]),
        _ => {
            let (last, rest) = kw_names.split_last().unwrap();
            format!("{}, and {}", rest.join(", "), last)
        }
    };
    format!(" with {joined}")
}

fn trim_period(s: &str) -> String {
    s.strip_suffix('.').unwrap_or(s).to_string()
}

fn ensure_period(s: &str) -> String {
    if s.ends_with(['.', '!', '?']) { s.to_string() } else { format!("{s}.") }
}
