//! The loyalty-ability frame parser: a bracketed "[+N]: <effect>." /
//! "[−N]: …" / "[0]: …" / "[−X]: …" planeswalker loyalty line -> the loyalty
//! ability macro invocation (`LoyaltyPlus`/`LoyaltyMinus`/`LoyaltyZero`,
//! `plugins/builtin/macros/ability/`). A loyalty ability is an activated
//! ability whose cost puts or removes loyalty counters [CR#606.2,606.4]; the
//! macros carry the shared loyalty frame (sorcery speed, the shared
//! once-per-turn gate), so the parser only picks the macro by cost sign and
//! hands the body to the shared effect grammar — exactly how the canon
//! walkers author these abilities. Extraction keeps the printed bracket
//! notation (`[+1]:`, mtgjson's cost delimiter), and the minus is U+2212
//! MINUS SIGN (the printed glyph), never an ASCII hyphen — the render
//! direction (`render/effect.rs`'s `loyalty_cost_prefix`) emits the same
//! bracketed prefix.

use crate::parsers::effect::ParsedEffect;
use crate::parsers::effect::{self};
#[cfg(test)]
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

/// A recognized bracketed loyalty cost: the macro-name choice plus the count
/// token (a bare integer, or `X` for a variable "−X" cost [CR#601.2b]). The
/// sign is a macro-NAME choice, not a signed param — macros can't branch on a
/// param's sign, so `LoyaltyPlus`/`LoyaltyMinus`/`LoyaltyZero` are distinct
/// macros (see the `LoyaltyPlus` macro's rationale comment).
enum LoyaltyCost<'a> {
    Plus(&'a str),
    Minus(&'a str),
    Zero,
}

/// A registry parser: a "[<loyalty cost>]: <effect>." line -> a
/// `LoyaltyPlus(n: N, effect: …)` / `LoyaltyMinus(n: N, effect: …)` /
/// `LoyaltyZero(effect: …)` macro invocation. Declines (`Ok(None)`) on lines
/// without the bracketed-cost prefix, on cost shapes outside
/// `[+N]`/`[−N]`/`[0]`/`[−X]`, and on effect bodies the shared grammar can't
/// structure. Self-identifying by the bracket frame, so the card's `CardKind`
/// is irrelevant.
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let Some(rest) = line.strip_prefix('[') else {
        return Ok(None);
    };
    let Some((cost_token, effect_clause)) = rest.split_once("]: ") else {
        return Ok(None);
    };
    let Some(cost) = parse_loyalty_cost(cost_token) else {
        return Ok(None);
    };
    // A mana-adding body first: a loyalty ability is never a mana ability
    // ([CR#605.1a] excludes them — it uses the stack), but "Add …" is not an
    // effect-grammar production, so the body borrows the mana-ability
    // module's shared `Add` reader (canon Chandra, Torch of Defiance's
    // `LoyaltyPlus(n: 1, effect: AddMana(You, 2, Red))` is this shape).
    if let Some(add) = crate::parsers::mana_ability::parse_add_effect(effect_clause)? {
        let parsed = ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: add,
        };
        return Ok(Some(render(&cost, &parsed)));
    }
    let Some(parsed) = effect::parse_clause(effect_clause, ctx)? else {
        return Ok(None);
    };
    Ok(Some(render(&cost, &parsed)))
}

/// The bracket's interior -> a [`LoyaltyCost`], or `None` for any shape the
/// printed corpus doesn't use: the exact inventory is `+N` (N ≥ 1), `−N`
/// (N ≥ 1, U+2212), `0`, and `−X`. An ASCII-hyphen "-N" is NOT a loyalty
/// cost (printed cards use the minus glyph; a hyphen here would be a
/// normalization bug worth surfacing as a decline, not absorbing), and
/// signed zeros (`+0`/`−0`) don't exist on any card.
fn parse_loyalty_cost(token: &str) -> Option<LoyaltyCost<'_>> {
    let nonzero_int = |n: &str| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) && n != "0";
    if token == "0" {
        return Some(LoyaltyCost::Zero);
    }
    if let Some(n) = token.strip_prefix('+') {
        return nonzero_int(n).then_some(LoyaltyCost::Plus(n));
    }
    if let Some(n) = token.strip_prefix('\u{2212}') {
        if n == "X" {
            return Some(LoyaltyCost::Minus(n));
        }
        return nonzero_int(n).then_some(LoyaltyCost::Minus(n));
    }
    None
}

/// Wraps the parsed body in the macro invocation the cost sign picks. A
/// targeting body rides inside the `effect` param as the same `Targeted`
/// wrapper the `Activated` frame uses (the param is `OneShotEffect`-kind, and
/// the canon walkers author exactly this shape); the macro expansion at
/// graduation supplies the `Activated` frame itself.
fn render(cost: &LoyaltyCost, parsed: &ParsedEffect) -> String {
    let effect = if parsed.targets.is_empty() {
        parsed.effect.clone()
    } else {
        format!(
            "Targeted(targets: [{}], effect: {})",
            parsed.targets.join(", "),
            parsed.effect
        )
    };
    match cost {
        LoyaltyCost::Plus(n) => format!("LoyaltyPlus(n: {n}, effect: {effect})"),
        LoyaltyCost::Minus(n) => format!("LoyaltyMinus(n: {n}, effect: {effect})"),
        LoyaltyCost::Zero => format!("LoyaltyZero(effect: {effect})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loyalty(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    #[test]
    fn plus_gains_life_like_ajani_goldmane() {
        assert_eq!(
            loyalty("[+1]: You gain 2 life.").as_deref(),
            Some("LoyaltyPlus(n: 1, effect: ChangeLife(You, Up(2)))")
        );
    }

    #[test]
    fn minus_targeted_damage() {
        assert_eq!(
            loyalty("[\u{2212}2]: ~ deals 3 damage to any target.").as_deref(),
            Some(
                "LoyaltyMinus(n: 2, effect: Targeted(targets: [AnyTarget], \
                 effect: DealDamage(This, 3, Target(0))))"
            )
        );
    }

    #[test]
    fn zero_draws() {
        assert_eq!(
            loyalty("[0]: Draw a card.").as_deref(),
            Some("LoyaltyZero(effect: Draw(1))")
        );
    }

    /// A multi-digit "ultimate" cost stays one literal.
    #[test]
    fn minus_ten_like_jace_beleren() {
        assert_eq!(
            loyalty("[\u{2212}10]: Draw three cards.").as_deref(),
            Some("LoyaltyMinus(n: 10, effect: Draw(3))")
        );
    }

    /// A variable "−X" loyalty cost carries the `X` count token — the engine
    /// announces X onto the activation slot and concretizes the cost verb
    /// ([CR#601.2b], `activate.rs`'s semantic-X gate).
    #[test]
    fn minus_x_variable_cost() {
        assert_eq!(
            loyalty("[\u{2212}X]: Draw a card.").as_deref(),
            Some("LoyaltyMinus(n: X, effect: Draw(1))")
        );
    }

    /// A mana-adding loyalty body routes through the mana-ability module's
    /// shared `Add` production reader — the emitted invocation matches canon
    /// Chandra, Torch of Defiance's authored `LoyaltyPlus(n: 1, effect:
    /// AddMana(You, 2, Red))` byte for byte.
    #[test]
    fn mana_adding_body_like_chandra_torch() {
        assert_eq!(
            loyalty("[+1]: Add {R}{R}.").as_deref(),
            Some("LoyaltyPlus(n: 1, effect: AddMana(You, 2, Red))")
        );
        assert_eq!(
            loyalty("[0]: Add {C}{C}{C}.").as_deref(),
            Some("LoyaltyZero(effect: AddMana(You, 3, Colorless))")
        );
        // A trailing clause that isn't the painland rider (here a targeted
        // damage tail) declines the `Add` reader, and "Add …" is no effect
        // production either — the whole line declines.
        assert!(loyalty("[+1]: Add {R}{R}. ~ deals 2 damage to target player.").is_none());
    }

    #[test]
    fn declines_non_loyalty_shapes() {
        // An ASCII hyphen is not the printed minus glyph.
        assert!(loyalty("[-2]: Draw a card.").is_none());
        // Signed zeros don't exist on any card.
        assert!(loyalty("[+0]: Draw a card.").is_none());
        assert!(loyalty("[\u{2212}0]: Draw a card.").is_none());
        // No bracket frame at all — the `Activated` frame's domain.
        assert!(loyalty("{T}: Draw a card.").is_none());
        assert!(loyalty("+1: Draw a card.").is_none());
        // A bracketed non-cost.
        assert!(loyalty("[foo]: Draw a card.").is_none());
        // An unstructurable effect body declines the whole line.
        assert!(loyalty("[+1]: Do something inscrutable.").is_none());
    }
}
