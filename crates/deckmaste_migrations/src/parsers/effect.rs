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
pub(super) fn parse_clause(line: &str) -> Option<ParsedEffect> {
    parse_deal_damage(line)
        .or_else(|| parse_draw(line))
        .or_else(|| parse_lose_life(line))
        .or_else(|| parse_gain_life(line))
}

/// `~ deals N damage to <target>.` or `it deals N damage to <target>.`
fn parse_deal_damage(line: &str) -> Option<ParsedEffect> {
    let body = line
        .strip_prefix("~ deals ")
        .or_else(|| line.strip_prefix("it deals "))?;
    let rest = body.strip_suffix('.')?;
    let (amount, tail) = rest.split_once(" damage to ")?;
    let amount: u32 = amount.parse().ok()?;
    let (targets, selection) = damage_target(tail)?;
    Some(ParsedEffect {
        targets,
        effect: format!("DealDamage({selection}, {amount})"),
    })
}

/// `Draw N card(s).` — no targets. Case-insensitive lead ("draw" or "Draw").
fn parse_draw(line: &str) -> Option<ParsedEffect> {
    let rest = strip_prefix_ci(line, "draw ")?.strip_suffix('.')?;
    // Plural first so "two cards" doesn't strip to "two card".
    let count = rest
        .strip_suffix(" cards")
        .or_else(|| rest.strip_suffix(" card"))?;
    let n = number_word(count)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("Draw({n})"),
    })
}

/// `You lose N life.` — the ability's controller loses N life. No targets.
fn parse_lose_life(line: &str) -> Option<ParsedEffect> {
    let n = life_count(strip_prefix_ci(line, "you lose ")?)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("LoseLife({n})"),
    })
}

/// `You gain N life.` — the ability's controller gains N life. No targets.
fn parse_gain_life(line: &str) -> Option<ParsedEffect> {
    let n = life_count(strip_prefix_ci(line, "you gain ")?)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("GainLife({n})"),
    })
}

/// `N life.` -> N ("life" is invariant — never pluralized). `None` if the
/// count word or the shape is off.
fn life_count(text: &str) -> Option<u32> {
    number_word(text.strip_suffix('.')?.strip_suffix(" life")?)
}

/// Case-insensitive ASCII prefix strip: the remainder after `prefix` if `s`
/// starts with it (ignoring ASCII case), else `None`.
fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let (head, rest) = s.split_at_checked(prefix.len())?;
    head.eq_ignore_ascii_case(prefix).then_some(rest)
}

/// A small spelled cardinal or a bare decimal -> its value. `None` for
/// anything else (e.g. "X", "that many").
fn number_word(word: &str) -> Option<u32> {
    match word {
        "a" | "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        digits => digits.parse().ok(),
    }
}

/// Maps the "to <X>" tail of a damage clause to its `(target declarations,
/// body selection)`. Targeted shapes declare a `TargetSpec` and the body reads
/// `Target(0)`; "each" shapes declare nothing and inline a `Filter(...)`
/// selection.
fn damage_target(text: &str) -> Option<(Vec<String>, String)> {
    Some(match text {
        "any target" => (vec!["AnyTarget".to_owned()], "Target(0)".to_owned()),
        "target creature" => (
            vec!["TargetOne(Creature)".to_owned()],
            "Target(0)".to_owned(),
        ),
        "target player" => (vec!["TargetOne(Player)".to_owned()], "Target(0)".to_owned()),
        "each creature" => (Vec::new(), "Filter(Creature)".to_owned()),
        "each player" => (Vec::new(), "Filter(Player)".to_owned()),
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
                "TargetOne(Creature)".to_owned(),
                "DealDamage(Target(0), 2)".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 4 damage to target player."),
            Some((
                "TargetOne(Player)".to_owned(),
                "DealDamage(Target(0), 4)".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_each_shapes() {
        assert_eq!(
            parsed("~ deals 2 damage to each creature."),
            Some((String::new(), "DealDamage(Filter(Creature), 2)".to_owned()))
        );
        assert_eq!(
            parsed("~ deals 20 damage to each player."),
            Some((String::new(), "DealDamage(Filter(Player), 20)".to_owned()))
        );
    }

    #[test]
    fn declines_unknown_damage_targets_and_non_effects() {
        assert!(parse_clause("~ deals 3 damage to each opponent.").is_none());
        assert!(parse_clause("Flying").is_none());
        assert!(parse_clause("~ deals X damage to any target.").is_none());
    }

    #[test]
    fn draw_counts_from_words_and_digits() {
        assert_eq!(
            parsed("Draw a card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
        assert_eq!(
            parsed("Draw one card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
        assert_eq!(
            parsed("Draw two cards."),
            Some((String::new(), "Draw(2)".to_owned()))
        );
        assert_eq!(
            parsed("Draw three cards."),
            Some((String::new(), "Draw(3)".to_owned()))
        );
        assert_eq!(
            parsed("Draw 5 cards."),
            Some((String::new(), "Draw(5)".to_owned()))
        );
    }

    #[test]
    fn draw_declines_unparseable_counts() {
        // "X" and "that many" aren't v1 productions.
        assert!(parse_clause("Draw X cards.").is_none());
        assert!(parse_clause("Draw that many cards.").is_none());
    }

    #[test]
    fn deal_damage_accepts_it_subject() {
        // Trigger surface: "it deals …" (the source), same RON as "~ deals …".
        assert_eq!(
            parsed("it deals 1 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 1)".to_owned()
            ))
        );
    }

    #[test]
    fn draw_is_case_insensitive() {
        // Trigger surface: lowercase "draw a card." (mid-sentence).
        assert_eq!(
            parsed("draw a card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
    }

    #[test]
    fn spell_surface_still_parses() {
        // Regression: the spell forms must keep working after generalization.
        assert_eq!(
            parsed("~ deals 3 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 3)".to_owned()
            ))
        );
        assert_eq!(
            parsed("Draw two cards."),
            Some((String::new(), "Draw(2)".to_owned()))
        );
    }

    #[test]
    fn lose_and_gain_life() {
        assert_eq!(
            parsed("You lose 1 life."),
            Some((String::new(), "LoseLife(1)".to_owned()))
        );
        assert_eq!(
            parsed("you lose 2 life."),
            Some((String::new(), "LoseLife(2)".to_owned()))
        );
        assert_eq!(
            parsed("You gain 3 life."),
            Some((String::new(), "GainLife(3)".to_owned()))
        );
        assert_eq!(
            parsed("you gain three life."),
            Some((String::new(), "GainLife(3)".to_owned()))
        );
    }

    #[test]
    fn life_declines_unparseable() {
        assert!(parse_clause("you lose life.").is_none());
        assert!(parse_clause("you gain X life.").is_none());
    }
}
