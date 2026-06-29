//! Effects / actions render to imperative sentences (spell mood).

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::Duration;
use deckmaste_core::Effect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
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
        // A target-scoping wrapper ([CR#115.1,601.2c]): render the inner effect
        // with `ctx.targets` rebound to this node's targets, so the inner
        // `Reference::Target(n)` resolves to "target creature" etc.
        Effect::Targeted(t) => effect(
            &t.effect,
            &Ctx {
                subject: ctx.subject,
                targets: &t.targets,
            },
        ),
        // [CR#118.12a]: "[or_else] unless [actor] pays [cost]" — the resolution-
        // time punisher (Mana Leak). Starts with the rendered punisher effect
        // (already capitalized). Declines structurally if the cost has no symbol
        // rendering (e.g. a `Do(...)` verb cost).
        Effect::MustPay(m) => {
            let payer = fragment::reference(&m.actor, ctx);
            match super::template::render_cost(&m.cost.0) {
                Some(c) => format!(
                    "{} unless {payer} pays {c}.",
                    trim_period(&effect(&m.or_else, ctx))
                ),
                None => format!("[unrendered: {m:?}]."),
            }
        }
        // [CR#603,608]: "[actor] may pay [cost]. If [actor] does, [and_then];
        // if [actor] doesn't, [or_else]" — a resolution-time kicker.
        Effect::MayPay(m) => {
            let payer = fragment::reference(&m.actor, ctx);
            match super::template::render_cost(&m.cost.0) {
                Some(c) => {
                    let did = super::ability::lower_first(&trim_period(&effect(&m.and_then, ctx)));
                    let tail = m.or_else.as_ref().map_or_else(String::new, |or_else| {
                        let didnt =
                            super::ability::lower_first(&trim_period(&effect(or_else, ctx)));
                        format!("; if {payer} doesn't, {didnt}")
                    });
                    fragment::capitalize(&format!(
                        "{payer} may pay {c}. If {payer} does, {did}{tail}."
                    ))
                }
                None => format!("[unrendered: {m:?}]."),
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
        // Default source (`This`): the implicit "deal N damage to X". An
        // explicit non-`This` source names the dealer — "<source> deals N
        // damage to <target>" (the fight / redirected-damage surface).
        Action::DealDamage(target, amount, Reference::This) => format!(
            "Deal {} damage to {}.",
            fragment::count(amount),
            fragment::selection(target, ctx)
        ),
        Action::DealDamage(target, amount, source) => format!(
            "{} deals {} damage to {}.",
            capitalize_first(&fragment::reference(source, ctx)),
            fragment::count(amount),
            fragment::selection(target, ctx)
        ),
        Action::Destroy(sel) => format!("Destroy {}.", fragment::selection(sel, ctx)),
        Action::By(_who, pa) => player_action(pa, ctx),
        // [CR#701.19a]: a regeneration shield — rendered as "Regenerate <target>."
        // when the replacement body has the standard structure. The top-level
        // `Regenerate` keyword macro emits this via its template.
        Action::CreateReplacement { subject, .. } => {
            format!("Regenerate {}.", fragment::selection(subject, ctx))
        }
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
        // [CR#701.19a]: remove all damage as part of regeneration.
        PlayerAction::RemoveDamage(sel) => {
            format!("Remove all damage from {}.", fragment::selection(sel, ctx))
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

// ── Token creation
// ────────────────────────────────────────────────────────────

fn create_text(count: &Count, spec: &TokenSpec) -> String {
    match spec {
        TokenSpec::Token(t) => {
            let plural = count.literal_value() != Some(1);
            let count_word = token_count_word(count);
            let descriptor = token_descriptor(t);
            let noun = if plural { "tokens" } else { "token" };
            let abilities_suffix = token_abilities_suffix(&t.abilities);
            format!("Create {count_word} {descriptor} {noun}{abilities_suffix}.")
        }
        // A predefined token ([CR#111.10]) renders by its bare name —
        // "Create a Treasure token." — the bidirectional truth the parser's
        // `create a <Name> token` production routes back to.
        TokenSpec::Named(name) => {
            let plural = count.literal_value() != Some(1);
            let count_word = token_count_word(count);
            let noun = if plural { "tokens" } else { "token" };
            format!("Create {count_word} {} {noun}.", name.as_str())
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

/// Capitalize the first character (sentence-start use, e.g. a named damage
/// source: "Target creature deals …").
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn ensure_period(s: &str) -> String {
    if s.ends_with(['.', '!', '?']) { s.to_string() } else { format!("{s}.") }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Action;
    use deckmaste_core::Count;
    use deckmaste_core::Filter;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::TargetSpec;

    use super::Ctx;
    use super::action;

    /// The default `This` source renders the implicit "Deal N damage to X";
    /// an explicit non-`This` source names the dealer — "<dealer> deals N
    /// damage to <target>" (the fight / redirected-damage surface).
    #[test]
    fn deal_damage_source_renders_dealer_phrase() {
        let target = TargetSpec::Target(Quantity::Exactly(Count::Literal(1)), Filter::creature());
        let ctx = Ctx {
            subject: "Pouncer",
            targets: std::slice::from_ref(&target),
        };

        let default = Action::deal_damage(Selection::Ref(Reference::Target(0)), Count::Literal(3));
        assert_eq!(action(&default, &ctx), "Deal 3 damage to target creature.");

        let sourced = Action::DealDamage(
            Selection::Ref(Reference::Target(0)),
            Count::Literal(3),
            Reference::Target(0),
        );
        assert_eq!(
            action(&sourced, &ctx),
            "Target creature deals 3 damage to target creature."
        );
    }
}
