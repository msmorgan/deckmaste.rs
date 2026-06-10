//! The `Activated` frame parser: a "<cost>: <effect>." line -> the bare
//! `Activated(...)` ability RON. An activated ability is written as
//! "[Cost]: [Effect.]"; the activation cost is everything before the colon
//! [CR#602.1,602.1a]. The effect grammar is shared via
//! [`crate::parsers::effect`]; the cost grammar (mana runs, {T}/{Q},
//! sacrifice-self, pay-life, discard; comma-separated) lives here as private
//! helpers.

use deckmaste_core::{ManaCost, ManaSymbol};

use crate::parsers::effect::{self, ParsedEffect};
use crate::resolve::CardKind;
use crate::ron_output::ron_options;

/// A registry parser: a "<cost>: <effect>." line -> the bare `Activated(...)`
/// RON. Declines (`Ok(None)`) on lines without a cost colon or with
/// unrecognized cost components/effects. Self-identifying by the cost grammar
/// before the colon, so the card's `CardKind` is irrelevant.
pub(crate) fn resolve_line(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
    let Some((cost_clause, effect_clause)) = line.split_once(": ") else {
        return Ok(None);
    };
    let Some(cost) = parse_cost(cost_clause)? else {
        return Ok(None);
    };
    let Some(parsed) = effect::parse_clause(effect_clause) else {
        return Ok(None);
    };
    Ok(Some(render(&cost, &parsed)))
}

/// Wraps a cost list + [`ParsedEffect`] in the `Activated` frame, emitting
/// `targets:` only when the effect declares any.
fn render(cost: &[String], parsed: &ParsedEffect) -> String {
    let cost = cost.join(", ");
    if parsed.targets.is_empty() {
        format!("Activated(cost: [{cost}], effect: {})", parsed.effect)
    } else {
        format!(
            "Activated(cost: [{cost}], targets: [{}], effect: {})",
            parsed.targets.join(", "),
            parsed.effect
        )
    }
}

/// Parses an activation cost clause: every ", "-separated component must be
/// recognized, or the whole cost declines.
fn parse_cost(clause: &str) -> anyhow::Result<Option<Vec<String>>> {
    let mut components = Vec::new();
    for part in clause.split(", ") {
        let Some(component) = cost_component(part)? else {
            return Ok(None);
        };
        components.push(component);
    }
    Ok(Some(components))
}

/// One cost component -> its `CostComponent` RON, or `None`. The tap/untap
/// symbols [CR#107.5,107.6] and sacrifice-self (the `SacrificeThis` macro)
/// are exact matches; the rest are shape productions.
fn cost_component(text: &str) -> anyhow::Result<Option<String>> {
    Ok(match text {
        "{T}" => Some("Tap".to_owned()),
        "{Q}" => Some("Untap".to_owned()),
        "Sacrifice ~" => Some("SacrificeThis".to_owned()),
        _ => match pay_life(text).or_else(|| discard(text)) {
            Some(component) => Some(component),
            None => mana_component(text)?,
        },
    })
}

/// `Pay N life` -> `Do(LoseLife(N))`: paying life is losing that much life
/// [CR#119.4].
fn pay_life(text: &str) -> Option<String> {
    let n = effect::number_word(text.strip_prefix("Pay ")?.strip_suffix(" life")?)?;
    Some(format!("Do(LoseLife({n}))"))
}

/// `Discard a card` / `Discard N cards` -> `Do(Discard(N))` (cards of the
/// payer's choice). Riders ("at random", "your hand") decline.
fn discard(text: &str) -> Option<String> {
    let rest = text.strip_prefix("Discard ")?;
    let count = rest
        .strip_suffix(" cards")
        .or_else(|| rest.strip_suffix(" card"))?;
    let n = effect::number_word(count)?;
    Some(format!("Do(Discard({n}))"))
}

/// A run of mana symbols -> `Mana([...])`. Declines on non-mana text, the
/// empty cost, and `{X}` (a variable activation cost needs an announced X —
/// no representation yet).
fn mana_component(text: &str) -> anyhow::Result<Option<String>> {
    let Ok(mana) = text.parse::<ManaCost>() else {
        return Ok(None);
    };
    if mana.is_empty() || mana.iter().any(|s| matches!(s, ManaSymbol::Variable)) {
        return Ok(None);
    }
    Ok(Some(format!("Mana({})", ron_options().to_string(&mana)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn act(line: &str) -> Option<String> { resolve_line(line, CardKind::Permanent).unwrap() }

    #[test]
    fn tap_damage_like_prodigal_sorcerer() {
        assert_eq!(
            act("{T}: ~ deals 1 damage to any target.").as_deref(),
            Some("Activated(cost: [Tap], targets: [AnyTarget], effect: DealDamage(Target(0), 1))")
        );
    }

    #[test]
    fn mana_tap_sacrifice_cost() {
        assert_eq!(
            act("{1}{B}, {T}, Sacrifice ~: Draw a card.").as_deref(),
            Some(
                "Activated(cost: [Mana([Generic(1),Black]), Tap, SacrificeThis], effect: Draw(1))"
            )
        );
    }

    #[test]
    fn pay_life_cost() {
        assert_eq!(
            act("{1}{B}, Pay 2 life: Draw a card.").as_deref(),
            Some("Activated(cost: [Mana([Generic(1),Black]), Do(LoseLife(2))], effect: Draw(1))")
        );
    }

    #[test]
    fn discard_cost() {
        assert_eq!(
            act("{1}{R}, Discard a card: Draw a card.").as_deref(),
            Some("Activated(cost: [Mana([Generic(1),Red]), Do(Discard(1))], effect: Draw(1))")
        );
    }

    #[test]
    fn untap_symbol_cost() {
        assert_eq!(
            act("{Q}: Draw a card.").as_deref(),
            Some("Activated(cost: [Untap], effect: Draw(1))")
        );
    }

    #[test]
    fn capital_it_subject_after_sacrifice() {
        assert_eq!(
            act("{1}{R}, Sacrifice ~: It deals 1 damage to any target.").as_deref(),
            Some(
                "Activated(cost: [Mana([Generic(1),Red]), SacrificeThis], targets: [AnyTarget], \
                 effect: DealDamage(Target(0), 1))"
            )
        );
    }

    #[test]
    fn big_generic_cost_gains_life() {
        assert_eq!(
            act("{10}, {T}, Sacrifice ~: You gain 15 life.").as_deref(),
            Some(
                "Activated(cost: [Mana([Generic(10)]), Tap, SacrificeThis], effect: GainLife(15))"
            )
        );
    }

    #[test]
    fn declines_unknown_costs() {
        // Loyalty costs are a different frame (extraction keeps them bracketed).
        assert!(act("[0]: Draw a card.").is_none());
        // Sacrifice with a non-self selection isn't a v1 production.
        assert!(act("Sacrifice a creature: Draw a card.").is_none());
        // Discard riders decline.
        assert!(act("Discard a card at random: ~ deals 1 damage to any target.").is_none());
        assert!(act("Discard your hand: Draw two cards.").is_none());
        // Variable activation costs decline.
        assert!(act("{X}: Draw a card.").is_none());
    }

    #[test]
    fn declines_unknown_effects_and_non_activated_lines() {
        // The mana parser's domain: `Add` isn't an effect production here.
        assert!(act("{T}: Add {G}.").is_none());
        // Activation instructions after the effect sentence decline.
        assert!(act("{T}: Draw a card. Activate only as a sorcery.").is_none());
        // No cost colon at all.
        assert!(act("Flying").is_none());
        assert!(act("When ~ dies, draw a card.").is_none());
    }
}
