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
    // Peel a trailing "Activate only …" rider sentence off the effect body so
    // the body proper parses on its own; the rider's fields ride on the
    // `Activated` frame beside it. An unrecognized rider clause (an "and only
    // if …" this parser can't ground, "Any player may activate this ability."
    // — no core slot for who may activate) leaves `effect_clause` untouched,
    // so the body parse declines below rather than silently dropping the
    // restriction.
    let (effect_clause, riders) = peel_activation_riders(effect_clause, ctx)?;
    let Some(parsed) = effect::parse_clause(&effect_clause, ctx)? else {
        return Ok(None);
    };
    Ok(Some(render(&cost, riders.as_ref(), &parsed)))
}

/// The fields an "Activate only …" rider sentence lowers onto the
/// `Activated` frame — the activation `window`, a use `limit`, an activation
/// `condition`, and (for a graveyard-functioning self-return) the `from`
/// zone.
#[derive(Default)]
struct Riders {
    from: Option<&'static str>,
    window: Option<&'static str>,
    condition: Option<String>,
    limit: Option<&'static str>,
}

/// Split the trailing "Activate only <clause>[ and only <clause>]*." rider
/// sentence off `effect_clause`, lowering each recognized clause onto a
/// [`Riders`] frame — the window ("as a sorcery", "as an instant", "during
/// your turn", "during your upkeep"), a use-limit ("once each turn"/"once
/// each game" — bare "once" declines, see [`apply_rider_clause`]), or a
/// state condition ("if <phrase>", routed through
/// the `Condition`-kind macro index, [`apply_rider_clause`]). Multiple
/// clauses join " and only " ("Activate only as a sorcery and only once each
/// turn."). Returns `(body, Some(riders))` when a trailing rider sentence was
/// found and every one of its clauses was recognized; `(effect_clause, None)`
/// both when there is no such sentence AND when one of its clauses isn't one
/// this grammar grounds — the whole rider then stays attached, unstripped, so
/// the body parse below declines rather than silently dropping a restriction.
fn peel_activation_riders(
    effect_clause: &str,
    ctx: &ResolveCtx,
) -> anyhow::Result<(String, Option<Riders>)> {
    let Some(rest) = effect_clause.strip_suffix('.') else {
        return Ok((effect_clause.to_owned(), None));
    };
    let Some((body, clause)) = rest.rsplit_once(". Activate only ") else {
        return Ok((effect_clause.to_owned(), None));
    };
    let mut riders = Riders::default();
    for part in clause.split(" and only ") {
        if !apply_rider_clause(part.trim(), &mut riders, ctx)? {
            return Ok((effect_clause.to_owned(), None));
        }
    }
    // A self-return-from-graveyard body ("Return ~ from your graveyard …",
    // self-ref) marks the ability as functioning from the graveyard; a
    // `target … from your graveyard` OBJECT description (a different card
    // the effect reaches into the graveyard for) must not trip this — the
    // self-ref substring is exact enough to tell them apart.
    if body.contains("~ from your graveyard") {
        riders.from = Some("Graveyard");
    }
    // `body` is `effect_clause` with its own trailing period consumed as
    // half of the ". Activate only " delimiter — reattach it, since the body
    // is a complete sentence in its own right and the downstream effect
    // grammar expects one.
    Ok((format!("{body}."), Some(riders)))
}

/// Recognize one " and only "-joined clause of an "Activate only …" rider,
/// folding it onto `riders`. An "if <phrase>" clause routes `<phrase>`
/// through the `Condition`-kind macro index — new condition phrasings are
/// added by authoring a macro under `plugins/builtin/macros/condition/`, not
/// by extending this match. Returns `false` for a clause this grammar
/// doesn't ground (the caller declines the whole rider then).
fn apply_rider_clause(part: &str, riders: &mut Riders, ctx: &ResolveCtx) -> anyhow::Result<bool> {
    match part {
        "as a sorcery" => riders.window = Some("SorcerySpeed"),
        "as an instant" => riders.window = Some("InstantSpeed"),
        "during your turn" => riders.window = Some("DuringTurn(Your)"),
        "during your upkeep" => riders.window = Some("DuringStep(Beginning(Upkeep), Your)"),
        "once each turn" => riders.limit = Some("OncePerTurn"),
        // Bare "once" (no "each game") is NOT modeled as `OncePerGame`: the
        // renderer has only one printed form for that limit ("once each
        // game"), so accepting the bare form here would parse fine but
        // render back to the WRONG oracle string ("once each game." for a
        // card that printed "once.") — a lossy round-trip. Decline rather
        // than model it lossily; the whole rider then stays attached and the
        // line stays `Unparsed`.
        "once each game" => riders.limit = Some("OncePerGame"),
        _ => {
            let Some(phrase) = part.strip_prefix("if ") else {
                return Ok(false);
            };
            let Some(m) = ctx.index.match_kind("Condition", phrase)? else {
                return Ok(false);
            };
            riders.condition = Some(m.macro_name.to_string());
        }
    }
    Ok(true)
}

/// Wraps a cost list + optional [`Riders`] + [`ParsedEffect`] in the
/// `Activated` frame, emitting each rider field only when peeled and
/// `targets:` only when the effect declares any. The rider fields sit on the
/// outer frame, beside (not inside) any `Targeted` wrapper, in the
/// `ActivatedAbility` struct's declared order (`from`, `window`,
/// `condition`, `limits`).
fn render(cost: &[String], riders: Option<&Riders>, parsed: &ParsedEffect) -> String {
    let cost = cost.join(", ");
    let from = riders
        .and_then(|r| r.from)
        .map_or(String::new(), |z| format!(", from: {z}"));
    let window = riders
        .and_then(|r| r.window)
        .map_or(String::new(), |w| format!(", window: {w}"));
    let condition = riders
        .and_then(|r| r.condition.as_deref())
        .map_or(String::new(), |c| format!(", condition: {c}"));
    let limits = riders
        .and_then(|r| r.limit)
        .map_or(String::new(), |l| format!(", limits: [{l}]"));
    if parsed.targets.is_empty() {
        format!(
            "Activated(cost: [{cost}]{from}{window}{condition}{limits}, effect: {})",
            parsed.effect
        )
    } else {
        format!(
            "Activated(cost: [{cost}]{from}{window}{condition}{limits}, effect: Targeted(targets: [{}], effect: {}))",
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

    /// Bare "Activate only once." (no "each game") declines: the renderer
    /// has only one printed form for `OncePerGame` ("once each game"), so
    /// modeling the bare form here would round-trip lossily (parse "once."
    /// but render "once each game."). "Activate only once each game." still
    /// parses — only the bare form is affected.
    #[test]
    fn declines_bare_once_but_parses_once_each_game() {
        assert!(act("{1}: Draw a card. Activate only once.").is_none());
        assert!(act("{1}: Draw a card. Activate only once each game.").is_some());
    }

    /// A `... and only if ...` extension whose condition phrase has no
    /// matching `Condition` macro is a state predicate this parser can't
    /// ground, so the whole line declines rather than silently dropping it.
    /// (The empty test ctx used by `act` also has no macro index at all, so
    /// this declines the same way even a real condition phrase would.)
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

    /// A `target … from your graveyard` OBJECT description (a different card
    /// the effect reaches into the graveyard for, not the ability's own
    /// source) must NOT trip the graveyard-functioning `from` detection —
    /// only a self-referential `~ from your graveyard` does.
    #[test]
    fn graveyard_target_object_does_not_set_from() {
        let out = act(
            "{U}{U}: Return target creature card from your graveyard to your hand. \
             Activate only during your upkeep.",
        )
        .unwrap();
        assert!(!out.contains("from: Graveyard"), "no self-ref: {out}");
        assert!(out.contains("window: DuringStep(Beginning(Upkeep), Your)"));
    }

    /// "Activate only as a sorcery." — the dominant rider in the wizards
    /// one-away set — maps to `window: SorcerySpeed`.
    #[test]
    fn sorcery_speed_rider() {
        let with_rider = act("{1}: Draw a card. Activate only as a sorcery.").unwrap();
        let bare = act("{1}: Draw a card.").unwrap();
        let spliced = bare.replacen("], effect:", "], window: SorcerySpeed, effect:", 1);
        assert_eq!(with_rider, spliced);
    }

    /// "Activate only as an instant." maps to the (explicit, otherwise
    /// implicit) `window: InstantSpeed`.
    #[test]
    fn instant_speed_rider() {
        let with_rider = act("{1}: Draw a card. Activate only as an instant.").unwrap();
        let bare = act("{1}: Draw a card.").unwrap();
        let spliced = bare.replacen("], effect:", "], window: InstantSpeed, effect:", 1);
        assert_eq!(with_rider, spliced);
    }

    /// "Activate only during your turn." (bare — no "before attackers are
    /// declared" tail) maps to `window: DuringTurn(Your)`.
    #[test]
    fn during_your_turn_rider() {
        let with_rider = act("{1}: Draw a card. Activate only during your turn.").unwrap();
        let bare = act("{1}: Draw a card.").unwrap();
        let spliced = bare.replacen("], effect:", "], window: DuringTurn(Your), effect:", 1);
        assert_eq!(with_rider, spliced);
    }

    /// The "…, before attackers are declared" compound has no core `Timing`
    /// slot (a window bounded ABOVE by a step, rather than pinned to one, isn't
    /// modeled) — the whole line declines rather than dropping the qualifier.
    #[test]
    fn declines_before_attackers_are_declared_compound() {
        assert!(
            act("{T}: ~ deals 1 damage to any target. \
                 Activate only during your turn, before attackers are declared.")
            .is_none()
        );
    }

    /// "Any player may activate this ability." has no core slot for WHO may
    /// activate an ability — the whole line declines.
    #[test]
    fn declines_any_player_may_activate() {
        assert!(
            act("{1}: ~ gets +1/+1 until end of turn. Any player may activate this ability.")
                .is_none()
        );
    }

    /// "Activate only as a sorcery and only once each turn." — two clauses
    /// joined " and only ", both folding onto the same `Activated` frame
    /// (window and limits, in the struct's declared order).
    #[test]
    fn sorcery_and_once_per_turn_combo() {
        let with_rider = act("{1}: ~ gets +1/+1 until end of turn. \
             Activate only as a sorcery and only once each turn.")
        .unwrap();
        let bare = act("{1}: ~ gets +1/+1 until end of turn.").unwrap();
        let spliced = bare.replacen(
            "], effect:",
            "], window: SorcerySpeed, limits: [OncePerTurn], effect:",
            1,
        );
        assert_eq!(with_rider, spliced);
    }

    /// "Activate only if <phrase>." routes `<phrase>` through the
    /// `Condition`-kind macro index ([`crate::parsers::effect::parse_if`]'s
    /// sibling path) — needs the real builtin index (the empty test ctx has
    /// no macros).
    #[test]
    fn if_condition_rider_via_macro() {
        let with_rider = resolve_line(
            "{2}, {T}: Draw a card. \
             Activate only if there are seven or more cards in your graveyard.",
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            with_rider.as_deref(),
            Some(
                "Activated(cost: [Mana([Generic(2)]), Tap], \
                 condition: SevenOrMoreCardsInYourGraveyard, effect: Draw(1))"
            )
        );
    }

    /// An "if <phrase>" whose phrase has no matching `Condition` macro
    /// declines the whole line, even with the real builtin macro index.
    #[test]
    fn declines_if_condition_with_no_macro() {
        assert!(
            resolve_line(
                "{2}, {T}: Draw a card. Activate only if ~ is a creature.",
                &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
            )
            .unwrap()
            .is_none()
        );
    }
}
