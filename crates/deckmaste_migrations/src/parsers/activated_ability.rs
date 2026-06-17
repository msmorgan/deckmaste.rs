//! The `Activated` frame parser: a "<cost>: <effect>." line -> the bare
//! `Activated(...)` ability RON. An activated ability is written as
//! "[Cost]: [Effect.]"; the activation cost is everything before the colon
//! [CR#602.1,602.1a]. The effect grammar is shared via
//! [`crate::parsers::effect`], the cost grammar via [`crate::parsers::cost`].

use crate::parsers::cost::VariableMana;
use crate::parsers::cost::{self};
use crate::parsers::effect::ParsedEffect;
use crate::parsers::effect::{self};
use crate::resolve::CardKind;

/// A registry parser: a "<cost>: <effect>." line -> the bare `Activated(...)`
/// RON. Declines (`Ok(None)`) on lines without a cost colon or with
/// unrecognized cost components/effects. Self-identifying by the cost grammar
/// before the colon, so the card's `CardKind` is irrelevant.
pub(crate) fn resolve_line(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
    let Some((cost_clause, effect_clause)) = line.split_once(": ") else {
        return Ok(None);
    };
    // Variable activation costs parse now: the engine announces X onto the
    // activation slot and concretizes the cost (engine-x-costs, [CR#601.2b]).
    let Some(cost) = cost::parse_cost(cost_clause, VariableMana::Allow)? else {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn act(line: &str) -> Option<String> {
        resolve_line(line, CardKind::Permanent).unwrap()
    }

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
    fn sacrifice_filter_cost_like_goblin_bombardment() {
        assert_eq!(
            act("Sacrifice a creature: ~ deals 1 damage to any target.").as_deref(),
            Some(
                "Activated(cost: [Do(Sacrifice(Choose(Exactly(Literal(1)), Creature)))], \
                 targets: [AnyTarget], effect: DealDamage(Target(0), 1))"
            )
        );
    }

    #[test]
    fn sacrifice_another_subtype_cost() {
        // "another Goblin" → the self-exclusion filter, count 1.
        assert_eq!(
            act("Sacrifice another Goblin: Draw a card.").as_deref(),
            Some(
                "Activated(cost: [Do(Sacrifice(Choose(Exactly(Literal(1)), \
                 AllOf([Permanent, Subtype(\"Goblin\"), Not(Ref(This))]))))], effect: Draw(1))"
            )
        );
    }

    #[test]
    fn sacrifice_count_cost() {
        // A spelled count word sacrifices that many of the filtered subject.
        assert_eq!(
            act("Sacrifice two creatures: Draw a card.").as_deref(),
            Some(
                "Activated(cost: [Do(Sacrifice(Choose(Exactly(Literal(2)), Creature)))], \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn declines_unknown_costs() {
        // Loyalty costs are a different frame (extraction keeps them bracketed).
        assert!(act("[0]: Draw a card.").is_none());
        // A sacrifice subject the filter grammar can't parse declines.
        assert!(act("Sacrifice a creature wearing hats: Draw a card.").is_none());
        // A sacrifice with no determiner/count leading the subject declines.
        assert!(act("Sacrifice creatures: Draw a card.").is_none());
        // Discard riders decline.
        assert!(act("Discard a card at random: ~ deals 1 damage to any target.").is_none());
        assert!(act("Discard your hand: Draw two cards.").is_none());
    }

    #[test]
    fn variable_activation_cost_parses() {
        // [CR#601.2b]: variable activation costs now parse (engine-x-costs); the
        // rendered frame carries the printed `{X}` as a `Variable` mana symbol.
        assert_eq!(
            act("{X}: Draw a card.").as_deref(),
            Some("Activated(cost: [Mana([Variable])], effect: Draw(1))")
        );
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
