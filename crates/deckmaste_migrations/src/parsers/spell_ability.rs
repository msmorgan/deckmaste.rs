//! The `Spell` frame parser: an instant/sorcery's one-shot effect line ->
//! the bare `Spell(...)` ability RON [CR#608.2d]. The effect grammar lives in
//! [`crate::parsers::effect`]; this module only decides framing (gating on the
//! card being a spell) and renders the wrapper.

use crate::parsers::effect::ParsedEffect;
use crate::parsers::effect::{self};
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

/// A registry parser: a spell's effect line -> the bare `Spell(...)` RON.
/// Declines (`Ok(None)`) on non-spell cards or unrecognized effect lines.
///
/// Infallible today, but the `Result` is required by the `AbilityParser`
/// registry signature (sibling parsers render fallibly), and future effect
/// productions may render fallibly too.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    if ctx.kind != CardKind::Spell {
        return Ok(None);
    }
    Ok(effect::parse_clause(line).map(|parsed| render(&parsed)))
}

/// Wraps a [`ParsedEffect`] in the `Spell` frame, emitting `targets:` only
/// when the effect declares any.
fn render(parsed: &ParsedEffect) -> String {
    if parsed.targets.is_empty() {
        format!("Spell(effect: {})", parsed.effect)
    } else {
        format!(
            "Spell(effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            parsed.effect
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spell(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Spell)).unwrap()
    }

    #[test]
    fn frames_targeted_damage_like_lightning_bolt() {
        assert_eq!(
            spell("~ deals 3 damage to any target.").as_deref(),
            Some("Spell(effect: Targeted(targets: [AnyTarget], effect: DealDamage(Target(0), 3)))")
        );
    }

    #[test]
    fn frames_restricted_target_like_lava_spike() {
        assert_eq!(
            spell("~ deals 3 damage to target player or planeswalker.").as_deref(),
            Some(
                "Spell(effect: Targeted(targets: [TargetOne(OneOf([Player, Planeswalker]))], effect: DealDamage(Target(0), 3)))"
            )
        );
    }

    #[test]
    fn frames_untargeted_effects_without_a_targets_field() {
        assert_eq!(
            spell("~ deals 2 damage to each creature.").as_deref(),
            Some("Spell(effect: DealDamage(Filter(Creature), 2))")
        );
        assert_eq!(
            spell("Draw two cards.").as_deref(),
            Some("Spell(effect: Draw(2))")
        );
    }

    #[test]
    fn declines_on_permanents_and_on_unknown_lines() {
        // Same line that frames on a spell declines on a permanent.
        assert!(
            resolve_line(
                "~ deals 3 damage to any target.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
        // Unknown effect on a spell still declines (exile isn't a production).
        assert!(spell("Exile target creature.").is_none());
    }

    #[test]
    fn frames_destroy_target_like_doom_blade() {
        assert_eq!(
            spell("Destroy target creature.").as_deref(),
            Some(
                "Spell(effect: Targeted(targets: [TargetOne(Creature)], effect: Destroy(Target(0))))"
            )
        );
    }

    #[test]
    fn frames_durational_team_pump_like_overrun() {
        assert_eq!(
            spell("Creatures you control get +3/+3 and gain trample until end of turn.").as_deref(),
            Some(
                "Spell(effect: Continuously(effect: Modify(of: Matching(AllOf([Creature, \
                 ControlledBy(Ref(You))])), changes: [AddPower(Literal(3)), AddToughness(Literal(3)), \
                 GainAbility(Keyword(Trample))]), duration: FixedUntil(EndOfTurn)))"
            )
        );
    }
}
