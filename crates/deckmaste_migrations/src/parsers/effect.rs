//! The reusable effect-clause sub-parser: one normalized oracle effect
//! sentence -> the target declarations + body RON that any ability frame
//! (`Spell` now; triggered/activated later) wraps. Frame-agnostic by design,
//! so every frame parser shares one effect grammar. Targeting lives in the
//! announce list [CR#115.1]; distributive "each" is a resolution-time
//! selection [CR#608.2d].

/// One parsed effect clause: `TargetSpec` RON fragments to declare on the
/// frame (empty when the effect targets nothing), and the `Effect`/`Action`
/// body RON, which references any declared targets as `Target(0)`, `Target(1)`…
pub(super) struct ParsedEffect {
    pub(super) targets: Vec<String>,
    pub(super) effect: String,
}

/// Parses one normalized effect line into a [`ParsedEffect`], or `None` to
/// decline. Productions are tried in order; the first match wins.
pub(super) fn parse_clause(line: &str) -> Option<ParsedEffect> { parse_deal_damage(line) }

/// `~ deals N damage to <target>.`
fn parse_deal_damage(line: &str) -> Option<ParsedEffect> {
    let rest = line.strip_prefix("~ deals ")?.strip_suffix('.')?;
    let (amount, tail) = rest.split_once(" damage to ")?;
    let amount: u32 = amount.parse().ok()?;
    let (targets, selection) = damage_target(tail)?;
    Some(ParsedEffect {
        targets,
        effect: format!("DealDamage({selection}, {amount})"),
    })
}

/// Maps the "to <X>" tail of a damage clause to its `(target declarations,
/// body selection)`. Targeted shapes declare a `TargetSpec` and the body reads
/// `Target(0)`; "each" shapes declare nothing and inline an `Each(...)`
/// selection.
fn damage_target(text: &str) -> Option<(Vec<String>, String)> {
    Some(match text {
        "any target" => (vec!["AnyTarget".to_owned()], "Target(0)".to_owned()),
        "target creature" => (
            vec!["Target(Exactly(Literal(1)), Type(Creature))".to_owned()],
            "Target(0)".to_owned(),
        ),
        "target player" => (
            vec!["Target(Exactly(Literal(1)), Kind(Player))".to_owned()],
            "Target(0)".to_owned(),
        ),
        "each creature" => (
            Vec::new(),
            "Each(AllOf([InZone(Battlefield), Type(Creature)]))".to_owned(),
        ),
        "each player" => (Vec::new(), "Each(Kind(Player))".to_owned()),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(targets joined by ", ", effect)` for terse assertions.
    fn parsed(line: &str) -> Option<(String, String)> {
        parse_clause(line).map(|p| (p.targets.join(", "), p.effect))
    }

    #[test]
    fn deal_damage_targeted_shapes() {
        assert_eq!(
            parsed("~ deals 3 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 3)".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 2 damage to target creature."),
            Some((
                "Target(Exactly(Literal(1)), Type(Creature))".to_owned(),
                "DealDamage(Target(0), 2)".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 4 damage to target player."),
            Some((
                "Target(Exactly(Literal(1)), Kind(Player))".to_owned(),
                "DealDamage(Target(0), 4)".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_each_shapes() {
        assert_eq!(
            parsed("~ deals 2 damage to each creature."),
            Some((
                String::new(),
                "DealDamage(Each(AllOf([InZone(Battlefield), Type(Creature)])), 2)".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 20 damage to each player."),
            Some((
                String::new(),
                "DealDamage(Each(Kind(Player)), 20)".to_owned()
            ))
        );
    }

    #[test]
    fn declines_unknown_damage_targets_and_non_effects() {
        assert!(parse_clause("~ deals 3 damage to each opponent.").is_none());
        assert!(parse_clause("Flying").is_none());
        assert!(parse_clause("~ deals X damage to any target.").is_none());
    }
}
