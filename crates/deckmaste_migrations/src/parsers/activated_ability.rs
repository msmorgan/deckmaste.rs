//! The `Activated` frame parser: a "<cost>: <effect>." line -> the bare
//! `Activated(...)` ability RON. An activated ability is written as
//! "[Cost]: [`OneShotEffect`.]"; the activation cost is everything before the
//! colon [CR#602.1,602.1a]. The effect grammar is shared via
//! [`crate::parsers::effect`], the cost grammar via [`crate::parsers::cost`].

use crate::parsers::cost::VariableMana;
use crate::parsers::cost::{self};
use crate::parsers::effect::ParsedEffect;
use crate::parsers::effect::{self};
#[cfg(test)]
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

/// A registry parser: a "<cost>: <effect>." line -> the bare `Activated(...)`
/// RON. Declines (`Ok(None)`) on lines without a cost colon or with
/// unrecognized cost components/effects. Self-identifying by the cost grammar
/// before the colon, so the card's `CardKind` is irrelevant.
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let Some((cost_clause, effect_clause)) = line.split_once(": ") else {
        return Ok(None);
    };
    // Variable activation costs parse now: the engine announces X onto the
    // activation slot and concretizes the cost (engine-x-costs, [CR#601.2b]).
    let Some(cost) = cost::parse_cost(cost_clause, VariableMana::Allow, Some(ctx.index))? else {
        return Ok(None);
    };
    // Peel a trailing "Activate only once each turn/game." use-limit sentence
    // ([CR#602.5b]) off the effect body so the body proper parses on its own;
    // the limit rides on the `Activated` frame as a `limits: [...]` field. A
    // bare `Activate only once …` with no trailing condition is required — an
    // "… and only if …" extension is a state predicate this parser does not
    // yet structure, so it stays attached and the body parse declines below.
    let (effect_clause, limits) = peel_use_limit(effect_clause);
    let (effect_clause, from, window) = peel_activation_context(effect_clause);
    let Some(parsed) = effect::parse_clause(effect_clause, ctx)? else {
        return Ok(None);
    };
    Ok(Some(render(&cost, from, window, limits, &parsed)))
}

fn peel_activation_context(
    effect_clause: &str,
) -> (&str, Option<&'static str>, Option<&'static str>) {
    let Some(effect) = effect_clause.strip_suffix(" Activate only during your upkeep.") else {
        return (effect_clause, None, None);
    };
    let from = effect
        .contains(" from your graveyard ")
        .then_some("Graveyard");
    (effect, from, Some("DuringStep(Beginning(Upkeep), Your)"))
}

/// Split a trailing "Activate only once each turn." / "Activate only once each
/// game." sentence ([CR#602.5b]) off `effect_clause`, returning the body before
/// it plus the `UseLimit` RON token (`OncePerTurn` / `OncePerGame`). With no
/// such sentence the body is returned unchanged and `None`. Only the exact bare
/// forms peel: an "… and only if …" tail (a state condition this parser can't
/// structure yet) is left in place so the body parse declines rather than
/// silently dropping the condition.
fn peel_use_limit(effect_clause: &str) -> (&str, Option<&'static str>) {
    for (sentence, token) in [
        ("Activate only once each turn.", "OncePerTurn"),
        ("Activate only once each game.", "OncePerGame"),
    ] {
        if let Some(head) = effect_clause.strip_suffix(sentence) {
            return (head.trim_end(), Some(token));
        }
    }
    (effect_clause, None)
}

/// Wraps a cost list + optional `UseLimit` + [`ParsedEffect`] in the
/// `Activated` frame, emitting `limits:` only when a use-limit rider was peeled
/// and `targets:` only when the effect declares any. The limit sits on the
/// outer frame, beside (not inside) any `Targeted` wrapper.
fn render(
    cost: &[String],
    from: Option<&str>,
    window: Option<&str>,
    limits: Option<&str>,
    parsed: &ParsedEffect,
) -> String {
    let cost = cost.join(", ");
    let from = from.map_or(String::new(), |z| format!(", from: {z}"));
    let window = window.map_or(String::new(), |w| format!(", window: {w}"));
    let limits = limits.map_or(String::new(), |l| format!(", limits: [{l}]"));
    if parsed.targets.is_empty() {
        format!(
            "Activated(cost: [{cost}]{from}{window}{limits}, effect: {})",
            parsed.effect
        )
    } else {
        format!(
            "Activated(cost: [{cost}]{from}{window}{limits}, effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            parsed.effect
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn act(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    /// An activated ability whose effect is a keyword-action macro
    /// ("{4}, {T}: Investigate.") resolves through the macro-template
    /// fallthrough in the shared effect grammar.
    #[test]
    fn activated_keyword_action_macro_like_investigate() {
        let out = resolve_line(
            "{4}, {T}: Investigate.",
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            out.as_deref(),
            Some("Activated(cost: [Mana([Generic(4)]), Tap], effect: Investigate)")
        );
    }

    #[test]
    fn tap_damage_like_prodigal_sorcerer() {
        assert_eq!(
            act("{T}: ~ deals 1 damage to any target.").as_deref(),
            Some(
                "Activated(cost: [Tap], effect: Targeted(targets: [AnyTarget], effect: DealDamage(This, 1, It)))"
            )
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

    /// "Pay {E}" / "Pay {E}{E}" activation costs route through the reverse
    /// `CostComponent` template index to the `PayEnergy` macro — the repeat
    /// construct folds the `{E}` run into the count. Needs the real builtin
    /// index (the empty test ctx has no macros).
    #[test]
    fn pay_energy_cost() {
        let one = resolve_line(
            "{T}, Pay {E}: Draw a card.",
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            one.as_deref(),
            Some("Activated(cost: [Tap, PayEnergy(1)], effect: Draw(1))")
        );

        let two = resolve_line(
            "Pay {E}{E}: Draw a card.",
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            two.as_deref(),
            Some("Activated(cost: [PayEnergy(2)], effect: Draw(1))")
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
            Some(
                "Activated(cost: [Mana([Generic(1),Red]), Do(Discard(count: 1))], effect: Draw(1))"
            )
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
                "Activated(cost: [Mana([Generic(1),Red]), SacrificeThis], effect: \
                 Targeted(targets: [AnyTarget], effect: DealDamage(This, 1, It)))"
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
                "Activated(cost: [With(binder: ChooseOne(filter: Creature), \
                 body: [Do(Sacrifice(That(Permanent)))])], \
                 effect: Targeted(targets: [AnyTarget], effect: DealDamage(This, 1, It)))"
            )
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn sacrifice_another_subtype_cost() {
        // "another Goblin" → the self-exclusion filter, count 1.
        assert_eq!(
            act("Sacrifice another Goblin: Draw a card.").as_deref(),
            Some(
                "Activated(cost: [With(binder: ChooseOne(filter: \
                 And([Permanent, Subtype(\"Goblin\"), Not(Ref(This))])), \
                 body: [Do(Sacrifice(That(Permanent)))])], effect: Draw(1))"
            )
        );
    }

    #[test]
    fn sacrifice_count_cost() {
        // A spelled count word sacrifices that many of the filtered subject.
        assert_eq!(
            act("Sacrifice two creatures: Draw a card.").as_deref(),
            Some(
                "Activated(cost: [With(binder: Choose(quantity: Exactly(2), filter: Creature), \
                 body: [Do(Sacrifice(That(Permanent)))])], \
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

    /// "Activate only once each turn." [CR#602.5b] is split off the effect body
    /// and lifted into a `OncePerTurn` use-limit, the pump body parsing as
    /// usual. This is the dominant rider in the wizards one-away set. The
    /// emitted frame is the rider-less parse with a `limits: [OncePerTurn]`
    /// field spliced after `cost:`.
    #[test]
    fn once_per_turn_pump() {
        let with_rider =
            act("{R}: ~ gets +1/+0 until end of turn. Activate only once each turn.").unwrap();
        let bare = act("{R}: ~ gets +1/+0 until end of turn.").unwrap();
        let spliced = bare.replacen("], effect:", "], limits: [OncePerTurn], effect:", 1);
        assert_eq!(with_rider, spliced);
        assert!(with_rider.contains("limits: [OncePerTurn]"));
    }

    /// The double-mana pump shape with the rider.
    #[test]
    fn once_per_turn_double_mana_pump() {
        let with_rider =
            act("{1}{G}: ~ gets +2/+2 until end of turn. Activate only once each turn.").unwrap();
        let bare = act("{1}{G}: ~ gets +2/+2 until end of turn.").unwrap();
        let spliced = bare.replacen("], effect:", "], limits: [OncePerTurn], effect:", 1);
        assert_eq!(with_rider, spliced);
    }

    /// A targeted body keeps the limit on the outer `Activated` frame, beside
    /// (not inside) the `Targeted` effect wrapper.
    #[test]
    fn once_per_turn_with_targets() {
        let with_rider = act("{T}: ~ deals 1 damage to any target. \
                              Activate only once each turn.")
        .unwrap();
        let bare = act("{T}: ~ deals 1 damage to any target.").unwrap();
        let spliced = bare.replacen("], effect:", "], limits: [OncePerTurn], effect:", 1);
        assert_eq!(with_rider, spliced);
        // The limit sits on the outer frame, not inside the Targeted wrapper.
        assert!(with_rider.contains("], limits: [OncePerTurn], effect: Targeted("));
    }

    /// "Activate only once each game." [CR#702.177a] maps to `OncePerGame`.
    #[test]
    fn once_per_game() {
        let with_rider =
            act("{R}: ~ gets +1/+0 until end of turn. Activate only once each game.").unwrap();
        let bare = act("{R}: ~ gets +1/+0 until end of turn.").unwrap();
        let spliced = bare.replacen("], effect:", "], limits: [OncePerGame], effect:", 1);
        assert_eq!(with_rider, spliced);
    }

    /// A `... and only if ...` extension on the rider is a state condition this
    /// parser does not yet structure, so the whole line declines rather than
    /// silently dropping the condition.
    #[test]
    fn declines_rider_with_trailing_condition() {
        assert!(
            act("{0}: ~ gets +3/+3 until end of turn. \
                 Activate only once each turn and only if ~ is a creature.")
            .is_none()
        );
    }

    #[test]
    fn graveyard_activation_during_your_upkeep() {
        assert_eq!(
            act("{2}{R}{R}{R}: Return ~ from your graveyard to your hand. Activate only during your upkeep.").as_deref(),
            Some("Activated(cost: [Mana([Generic(2),Red,Red,Red])], from: Graveyard, window: DuringStep(Beginning(Upkeep), Your), effect: Move(This, Hand))")
        );
    }
}
