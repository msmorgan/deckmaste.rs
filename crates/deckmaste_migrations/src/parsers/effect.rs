//! The reusable effect-clause sub-parser: one normalized oracle effect
//! sentence -> the target declarations + body RON that any ability frame
//! (`Spell` now; triggered/activated later) wraps. Frame-agnostic by design,
//! so every frame parser shares one effect grammar. Targeting lives in the
//! announce list [CR#115.1]; distributive "each" is a resolution-time
//! selection [CR#608.2d].

use crate::parsers::filter;
use crate::parsers::modify;

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
        .or_else(|| parse_destroy(line))
        .or_else(|| parse_pump(line))
}

/// `<subject> gets ±N/±N [and gain(s) <kw…>] until end of turn.` (and the
/// keyword-only `<subject> gain(s)/have/has <kw…> until end of turn.`) -> a
/// one-shot continuous effect ([CR#611.2]): `Continuously(effect: Modify(of:
/// <scope>, changes: [...]), duration: FixedUntil(EndOfTurn))`. The durational
/// marker is required — it's what makes this a one-shot continuous effect
/// rather than an always-on static anthem ([`crate::parsers::static_ability`],
/// which declines the marker). The ±N/±N + keyword-grant grammar is shared with
/// that anthem parser via [`modify`]; the changes are written inline
/// (`Modification` is not a macro kind, so no `AddPowerToughness` macro can
/// stand here). Subject: a target ("target creature" -> `Of(Target(0))` +
/// `TargetOne(<filter>)`), or a team/self class via the shared subject grammar
/// (`Matching`/`Of`).
fn parse_pump(line: &str) -> Option<ParsedEffect> {
    let body = line.strip_suffix('.')?.strip_suffix(" until end of turn")?;
    let changes = pump_changes(body)?;
    let (scope, targets) = pump_scope(pump_subject(body)?)?;
    Some(ParsedEffect {
        targets,
        effect: format!(
            "Continuously(effect: Modify(of: {scope}, changes: [{}]), duration: FixedUntil(EndOfTurn))",
            changes.join(", ")
        ),
    })
}

/// The subject phrase of a pump body — everything before the first modify
/// marker.
fn pump_subject(body: &str) -> Option<&str> {
    modify::split_marker(body, &MODIFY_MARKERS).map(|(subj, _)| subj)
}

/// The changes list of a pump body: "±N/±N [and gain <kw…>]" (the P/T form,
/// with an optional keyword tail) or a bare keyword grant.
fn pump_changes(body: &str) -> Option<Vec<String>> {
    if let Some((_, pred)) = modify::split_marker(body, &[" gets ", " get "]) {
        let (pt_part, grant_tail) = modify::split_grant_tail(pred);
        let mut changes = modify::parse_pt_changes(pt_part.trim())?;
        if let Some(tail) = grant_tail {
            changes.extend(modify::parse_keyword_changes(tail)?);
        }
        return Some(changes);
    }
    let (_, pred) = modify::split_marker(body, &[" gains ", " gain ", " have ", " has "])?;
    modify::parse_keyword_changes(pred)
}

/// Pump subject -> (`Modify` scope, target declarations). A "target <filter>"
/// subject scopes `Of(Target(0))` and declares `TargetOne(<filter>)`; a
/// team/self class scopes via the shared subject grammar with no target.
fn pump_scope(subj: &str) -> Option<(String, Vec<String>)> {
    if let Some(rest) = modify::strip_prefix_ci(subj.trim(), "target ") {
        let filter = filter::parse_phrase(rest)?;
        return Some((
            "Of(Target(0))".to_owned(),
            vec![format!("TargetOne({filter})")],
        ));
    }
    let filter = modify::subject_to_filter(subj)?;
    Some((modify::filter_to_scope(&filter), Vec::new()))
}

/// The markers that separate a pump subject from its predicate.
const MODIFY_MARKERS: [&str; 6] = [" gets ", " get ", " gains ", " gain ", " have ", " has "];

/// `Destroy target <subject>.` -> a `TargetOne(<filter>)` declaration (the
/// subject parsed by the shared [`filter`] grammar) and the body
/// `Destroy(Target(0))` ([CR#701.8]). Only the single-target form; board wipes
/// ("destroy all/each …") are a later production. Declines when the subject
/// isn't filter-parseable. Case-insensitive lead, since the clause opens a
/// spell ("Destroy …") or follows a trigger comma ("…, destroy …").
fn parse_destroy(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "destroy ")?
        .strip_suffix('.')?
        .strip_prefix("target ")?;
    let filter = filter::parse_phrase(subject)?;
    Some(ParsedEffect {
        targets: vec![format!("TargetOne({filter})")],
        effect: "Destroy(Target(0))".to_owned(),
    })
}

/// `~ deals N damage to <target>.` or `it deals N damage to <target>.` —
/// "it" case-insensitively, since it opens the clause after a cost colon
/// ("Sacrifice ~: It deals …") but follows a comma in trigger clauses.
fn parse_deal_damage(line: &str) -> Option<ParsedEffect> {
    let body = line
        .strip_prefix("~ deals ")
        .or_else(|| strip_prefix_ci(line, "it deals "))?;
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
/// anything else (e.g. "X", "that many"). Shared with the sibling frame
/// parsers (cost counts spell the same way).
pub(super) fn number_word(word: &str) -> Option<u32> {
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
        // "each opponent" — the players who are opponents of you ([CR#102.2]).
        "each opponent" => (Vec::new(), "Filter(OpponentOf(Ref(You)))".to_owned()),
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
        // "each opponent" -> the player set "opponents of you".
        assert_eq!(
            parsed("~ deals 1 damage to each opponent."),
            Some((
                String::new(),
                "DealDamage(Filter(OpponentOf(Ref(You))), 1)".to_owned()
            ))
        );
    }

    #[test]
    fn destroy_target_shapes() {
        // The target subject parses via filter.rs into a `TargetOne(<filter>)`.
        assert_eq!(
            parsed("Destroy target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Destroy target artifact."),
            Some((
                "TargetOne(Type(Artifact))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Destroy target nonland permanent."),
            Some((
                "TargetOne(AllOf([Permanent, Not(Type(Land))]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // Lowercase lead (the clause after a trigger comma) parses too.
        assert_eq!(
            parsed("destroy target Goblin."),
            Some((
                "TargetOne(Subtype(\"Goblin\"))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_team_like_overrun() {
        // Overrun: a team P/T boost + keyword grant lasting until end of turn.
        assert_eq!(
            parsed("Creatures you control get +3/+3 and gain trample until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [AddPower(Literal(3)), AddToughness(Literal(3)), GainAbility(Keyword(Trample))]), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_self_and_target() {
        // Self pump ("~ gets …"): scope Of(This), no target.
        assert_eq!(
            parsed("~ gets +1/+1 until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Of(This), changes: [AddPower(Literal(1)), \
                 AddToughness(Literal(1))]), duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Single-target pump ("target creature gets …"): TargetOne + Of(Target(0)).
        assert_eq!(
            parsed("Target creature gets +3/+3 until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(of: Of(Target(0)), changes: [AddPower(Literal(3)), \
                 AddToughness(Literal(3))]), duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Keyword-only durational grant on a target.
        assert_eq!(
            parsed("Target creature gains flying until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(of: Of(Target(0)), changes: [GainAbility(Keyword(Flying))]), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn declines_unknown_damage_targets_and_non_effects() {
        // A damage target the grammar doesn't model still declines.
        assert!(parse_clause("~ deals 3 damage to each artifact.").is_none());
        assert!(parse_clause("Flying").is_none());
        assert!(parse_clause("~ deals X damage to any target.").is_none());
        // Destroy without the "target" form (board wipes) is a later follow-up.
        assert!(parse_clause("Destroy all creatures.").is_none());
        // A target subject the filter grammar can't parse declines.
        assert!(parse_clause("Destroy target creature with flying.").is_none());
        // A pump without the durational marker isn't an effect-grammar pump (it's
        // a static anthem's job on a permanent).
        assert!(parse_clause("Creatures you control get +1/+1.").is_none());
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
        // Activated surface: clause-initial "It deals …" after a cost colon.
        assert_eq!(
            parsed("It deals 2 damage to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(Target(0), 2)".to_owned()
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
