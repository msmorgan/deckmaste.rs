//! Replacement-effect parser ([CR#614]): the closed template list, rendered as
//! a `Static(effects: [Replacement(...)])` ability — replacements are carried
//! by a static ([CR#614.1], a continuous effect that modifies how an event
//! happens). Two templates:
//!
//! * **"As ~ enters, <effect>."** — a self-replacement applied at entry
//!   ([CR#614.1c,614.12]): `Replacement(AsEnters(<effect>))`. The shared
//!   [`effect`] grammar supplies the effect; an effect that *targets* declines
//!   (a self-replacement has no announce list to declare targets on).
//! * **"If [subject] would [die|enter], [effect] instead."** — replace
//!   ([CR#614.1a]): `Replacement(Instead(would: <event>, instead: <effect>))`.
//!   The event clause shares [`triggered_ability::parse_event`]; the effect
//!   shares [`effect`].
//!
//! `~ enters tapped.` (the bare self case) is already structured by the
//! mana-ability parser; this module declines it so the two never both match.
//! Declines (`Ok(None)`) on spells and on anything its productions don't fully
//! cover — a wrong replacement would graduate a wrong card.

use crate::parsers::effect::{self};
use crate::parsers::triggered_ability;
use crate::resolve::CardKind;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, kind: CardKind) -> anyhow::Result<Option<String>> {
    Ok(parse(line, kind))
}

fn parse(line: &str, kind: CardKind) -> Option<String> {
    // [CR#614.1]: a replacement is a continuous effect of a permanent's static
    // ability (or a one-shot's during-resolution clause). The permanent-side
    // templates here decline on spells.
    if kind == CardKind::Spell {
        return None;
    }
    parse_as_enters(line).or_else(|| parse_instead(line))
}

/// "As <subject> enters, <effect>." → `Replacement(AsEnters(<effect>))`. Only
/// the self subject (`~`) is structured — `AsEnters` is a *self*-replacement
/// ([CR#614.12]), folded into this object's own entry. The effect clause keeps
/// its trailing period (the shared grammar requires it) and must declare no
/// targets (no announce list exists at a replacement).
fn parse_as_enters(line: &str) -> Option<String> {
    let effect_clause = line.strip_prefix("As ~ enters, ")?;
    let parsed = effect::parse_clause(effect_clause)?;
    if !parsed.targets.is_empty() {
        return None;
    }
    Some(format!(
        "Static(effects: [Replacement(AsEnters({}))])",
        parsed.effect
    ))
}

/// "If [subject] would [die|enter], [effect] instead." →
/// `Replacement(Instead(would: <event>, instead: <effect>))`. The event is the
/// subject plus its base verb mapped to the present-tense form the shared
/// [`triggered_ability::parse_event`] grammar reads ("die" → "dies", "enter"
/// → "enters"); the effect reuses the shared grammar and must not target (a
/// replacement declares no announce list). Declines unless both halves parse.
fn parse_instead(line: &str) -> Option<String> {
    let body = line.strip_suffix('.')?;
    let rest = body.strip_prefix("If ")?;
    // "[subject] would [pred], [effect] instead": the verb "would" splits the
    // conditional event from its tail; "instead" terminates the effect.
    let (subject, tail) = rest.split_once(" would ")?;
    let (would_pred, effect_clause) = tail.split_once(", ")?;
    let effect_clause = effect_clause.strip_suffix(" instead")?;
    // Map the base verb after "would" to the event grammar's present tense.
    let verb_clause = match would_pred {
        "die" => "dies",
        "enter" | "enter the battlefield" => "enters",
        _ => return None,
    };
    let event = triggered_ability::parse_event(&format!("{subject} {verb_clause}"))?;
    // The shared effect grammar requires the trailing period the "instead"
    // suffix consumed; restore it.
    let parsed = effect::parse_clause(&format!("{effect_clause}."))?;
    if !parsed.targets.is_empty() {
        return None;
    }
    Some(format!(
        "Static(effects: [Replacement(Instead(would: {event}, instead: {}))])",
        parsed.effect
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rep(line: &str) -> Option<String> {
        resolve_line(line, CardKind::Permanent).unwrap()
    }

    #[test]
    fn as_enters_self_pump_one_shot() {
        // "As ~ enters" with a durational pump body — a self-replacement that
        // augments the entry with a one-shot continuous effect.
        assert_eq!(
            rep("As ~ enters, ~ gets +1/+1 until end of turn.").as_deref(),
            Some(
                "Static(effects: [Replacement(AsEnters(Continuously(effect: \
                 Modify(of: Of(This), changes: [AddPower(Literal(1)), AddToughness(Literal(1))]), \
                 duration: FixedUntil(EndOfTurn))))])"
            )
        );
    }

    #[test]
    fn instead_self_dies_draw() {
        // "If ~ would die, draw a card instead" — the enters/dies event grammar
        // supplies `ThisDies`, the effect grammar supplies `Draw(1)`.
        assert_eq!(
            rep("If ~ would die, draw a card instead.").as_deref(),
            Some("Static(effects: [Replacement(Instead(would: ThisDies, instead: Draw(1)))])")
        );
    }

    #[test]
    fn instead_filtered_subject_dies() {
        assert_eq!(
            rep("If a creature you control would die, you gain 1 life instead.").as_deref(),
            Some(
                "Static(effects: [Replacement(Instead(would: \
                 Dies(AllOf([Creature, ControlledBy(Ref(You))])), instead: GainLife(1)))])"
            )
        );
    }

    #[test]
    fn declines_spells() {
        assert!(
            resolve_line("If ~ would die, draw a card instead.", CardKind::Spell)
                .unwrap()
                .is_none()
        );
        assert!(
            resolve_line(
                "As ~ enters, ~ gets +1/+1 until end of turn.",
                CardKind::Spell
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn declines_bare_enters_tapped() {
        // Already structured by the mana-ability parser; this module abstains so
        // the two never both match the same line.
        assert!(rep("~ enters tapped.").is_none());
    }

    #[test]
    fn declines_unparseable_event_or_effect() {
        // Unknown event verb ("would be exiled").
        assert!(rep("If ~ would be exiled, draw a card instead.").is_none());
        // Unknown effect ("exile it" is not in the effect grammar yet).
        assert!(rep("If ~ would die, exile it instead.").is_none());
        // A targeting effect has nowhere to declare its target in a replacement.
        assert!(rep("As ~ enters, destroy target creature.").is_none());
        // Not a replacement line at all.
        assert!(rep("Draw a card.").is_none());
        // "If … would …" present but missing the "instead" terminator.
        assert!(rep("If ~ would die, draw a card.").is_none());
    }
}
