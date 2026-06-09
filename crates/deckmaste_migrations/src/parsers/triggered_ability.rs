//! The `Triggered` frame parser: a "When/Whenever <event>, <effect>." line ->
//! the bare `Triggered(...)` ability RON. A triggered ability is written as
//! "[trigger condition], [effect]" divided by the trigger word [CR#603.1]. The
//! effect grammar is shared via [`crate::parsers::effect`]; the event grammar
//! (ETB / dies; self or "a creature" subject) lives here as private helpers.

use crate::parsers::effect::{self, ParsedEffect};
use crate::resolve::CardKind;

/// A registry parser: a "When/Whenever <event>, <effect>." line -> the bare
/// `Triggered(...)` RON. Declines (`Ok(None)`) on non-trigger lines or
/// unrecognized events/effects. Self-identifying by the trigger word, so the
/// card's `CardKind` is irrelevant.
///
/// Infallible today, but the `Result` is required by the `AbilityParser`
/// registry signature (sibling parsers render fallibly).
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
    let Some(rest) = line
        .strip_prefix("When ")
        .or_else(|| line.strip_prefix("Whenever "))
    else {
        return Ok(None);
    };
    let Some((event_clause, effect_clause)) = rest.split_once(", ") else {
        return Ok(None);
    };
    let Some(event) = parse_event(event_clause) else {
        return Ok(None);
    };
    let Some(parsed) = effect::parse_clause(effect_clause) else {
        return Ok(None);
    };
    Ok(Some(render(&event, &parsed)))
}

/// Wraps an event + [`ParsedEffect`] in the `Triggered` frame, emitting
/// `targets:` only when the effect declares any.
fn render(event: &str, parsed: &ParsedEffect) -> String {
    if parsed.targets.is_empty() {
        format!("Triggered(event: {event}, effect: {})", parsed.effect)
    } else {
        format!(
            "Triggered(event: {event}, targets: [{}], effect: {})",
            parsed.targets.join(", "),
            parsed.effect
        )
    }
}

/// Parses a trigger's event clause (the text between the trigger word and the
/// comma) into the event RON, or `None`. v1: ETB + dies, subject self (`~`) or
/// "a creature". Self uses the `This{Verb}` shorthand macro; other subjects
/// apply the event macro to the subject filter.
fn parse_event(clause: &str) -> Option<String> {
    // Tolerate the older "enters the battlefield" wording (current oracle:
    // "enters").
    let clause = clause.strip_suffix(" the battlefield").unwrap_or(clause);
    let (subject, verb) = if let Some(subject) = clause.strip_suffix(" enters") {
        (subject, "Enters")
    } else if let Some(subject) = clause.strip_suffix(" dies") {
        (subject, "Dies")
    } else {
        return None;
    };
    if subject == "~" {
        Some(format!("This{verb}"))
    } else {
        Some(format!("{verb}({})", event_subject(subject)?))
    }
}

/// A trigger subject phrase -> its `Filter` RON. `~` is handled in
/// `parse_event` via the `This{Verb}` shorthand; "a creature" is the `Creature`
/// macro (a creature permanent).
fn event_subject(subject: &str) -> Option<&'static str> {
    match subject {
        "a creature" => Some("Creature"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trig(line: &str) -> Option<String> { resolve_line(line, CardKind::Permanent).unwrap() }

    #[test]
    fn etb_self_draw() {
        assert_eq!(
            trig("When ~ enters, draw a card.").as_deref(),
            Some("Triggered(event: ThisEnters, effect: Draw(1))")
        );
    }

    #[test]
    fn dies_self_targeted_damage() {
        assert_eq!(
            trig("When ~ dies, it deals 1 damage to any target.").as_deref(),
            Some(
                "Triggered(event: ThisDies, targets: [AnyTarget], effect: DealDamage(Target(0), 1))"
            )
        );
    }

    #[test]
    fn whenever_a_creature_dies_lose_life() {
        assert_eq!(
            trig("Whenever a creature dies, you lose 1 life.").as_deref(),
            Some("Triggered(event: Dies(Creature), effect: LoseLife(1))")
        );
    }

    #[test]
    fn whenever_a_creature_enters_draw() {
        assert_eq!(
            trig("Whenever a creature enters, draw a card.").as_deref(),
            Some("Triggered(event: Enters(Creature), effect: Draw(1))")
        );
    }

    #[test]
    fn tolerates_enters_the_battlefield_wording() {
        assert_eq!(
            trig("When ~ enters the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: ThisEnters, effect: Draw(1))")
        );
    }

    #[test]
    fn declines_non_triggers_unknown_events_and_effects() {
        // Not a trigger line.
        assert!(trig("Draw a card.").is_none());
        // Unknown event (cast trigger not in v1).
        assert!(trig("When you cast ~, draw a card.").is_none());
        // Unknown effect.
        assert!(trig("When ~ dies, destroy target creature.").is_none());
        // Trigger word present but no ", " separator (no effect clause).
        assert!(trig("When ~ dies").is_none());
    }
}
