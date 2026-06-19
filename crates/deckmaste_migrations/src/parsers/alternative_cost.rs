//! Nonkeyword alternative-cost permissions whose payment is expressed by an
//! existing cost component. These lines are static abilities on the card in
//! hand, so they lower to the same `May(Cast(... cost: Components(...)))`
//! shape used by flashback and evoke.

use crate::resolve::ResolveCtx;

pub(crate) fn resolve_line(line: &str, _ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    Ok(parse_exile_colored_card(line))
}

/// "You may exile a green card from your hand rather than pay ~'s mana cost."
/// becomes an alternative base cost. The choice is part of the cost and the
/// bound card is exiled as its payment; ownership plus `InZone(Hand)` names
/// the caster's hand without introducing a card-specific primitive.
fn parse_exile_colored_card(line: &str) -> Option<String> {
    let body = line.strip_suffix('.').unwrap_or(line);
    let color = body
        .strip_prefix("You may exile a ")?
        .strip_suffix(" card from your hand rather than pay ~'s mana cost")?;
    let color = match color {
        "white" => "White",
        "blue" => "Blue",
        "black" => "Black",
        "red" => "Red",
        "green" => "Green",
        _ => return None,
    };
    Some(format!(
        "Static(May(Cast(what: Ref(This), cost: Components([With(binder: ChooseOne(filter: And([Kind(Card), InZone(Hand), Owner(Ref(You)), ColorIs({color})])), body: [Do(Move(It, Exile))])]))))"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::test_ctx;
    use crate::resolve::CardKind;

    #[test]
    fn exile_green_card_instead_of_mana_cost() {
        let got = resolve_line(
            "You may exile a green card from your hand rather than pay ~'s mana cost.",
            &test_ctx::ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            got.as_deref(),
            Some(
                "Static(May(Cast(what: Ref(This), cost: Components([With(binder: ChooseOne(filter: And([Kind(Card), InZone(Hand), Owner(Ref(You)), ColorIs(Green)])), body: [Do(Move(It, Exile))])]))))"
            )
        );

        let row = got.expect("Vine Dryad line resolves");
        let ability: deckmaste_core::Ability =
            deckmaste_core::ron::options().from_str(&row).unwrap();
        let rendered = deckmaste_core::ron::options().to_string(&ability).unwrap();
        let reread: deckmaste_core::Ability =
            deckmaste_core::ron::options().from_str(&rendered).unwrap();
        assert_eq!(reread, ability, "alternative cost survives RON round-trip");
    }

    #[test]
    fn declines_nonmatching_and_unknown_color() {
        let ctx = test_ctx::ctx(CardKind::Permanent);
        assert_eq!(resolve_line("Flying", &ctx).unwrap(), None);
        assert_eq!(
            resolve_line(
                "You may exile a purple card from your hand rather than pay ~'s mana cost.",
                &ctx,
            )
            .unwrap(),
            None
        );
    }
}
