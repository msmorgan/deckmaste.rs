//! Always-on static abilities on permanents: "gets ±N/±N", "have/has/gain/gains
//! <keyword>", "<subject> can't attack/block". The ±N/±N + keyword-grant +
//! subject/scope grammar is shared via [`crate::parsers::modify`]; this module
//! renders `Static(effects: [...])` RON. Declines (`Ok(None)`) on spells,
//! durational clauses, targeted subjects, or anything its productions don't
//! fully cover.

use crate::parsers::modify;
use crate::resolve::CardKind;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, kind: CardKind) -> anyhow::Result<Option<String>> {
    Ok(parse(line, kind))
}

fn parse(line: &str, kind: CardKind) -> Option<String> {
    if kind == CardKind::Spell {
        return None;
    }
    let low = line.to_ascii_lowercase();
    if low.contains("until end of turn") || low.contains("this turn") {
        return None;
    }
    let body = line.strip_suffix('.').unwrap_or(line);

    if let Some((subj, pred)) = modify::split_marker(body, &[" can't ", " cannot "]) {
        return parse_restriction(subj, pred);
    }
    if let Some((subj, pred)) = modify::split_marker(body, &[" attacks ", " attack "]) {
        return parse_requirement(subj, pred);
    }
    if let Some((subj, pred)) = modify::split_marker(body, &[" gets ", " get "]) {
        return parse_pt(subj, pred);
    }
    if let Some((subj, pred)) =
        modify::split_marker(body, &[" have ", " has ", " gain ", " gains "])
    {
        return parse_grant(subj, pred);
    }
    None
}

fn parse_pt(subj: &str, pred: &str) -> Option<String> {
    let filter = modify::subject_to_filter(subj)?;
    // Optional combo tail: "+N/+M and have/has/gain/gains <kw…>" → the P/T changes
    // followed by one GainAbility per granted keyword.
    let (pt_part, grant_tail) = modify::split_grant_tail(pred);
    let mut changes = modify::parse_pt_changes(pt_part.trim())?;
    if let Some(tail) = grant_tail {
        changes.extend(modify::parse_keyword_changes(tail)?);
    }
    Some(format!(
        "Static(effects: [Modify(of: {}, changes: [{}])])",
        modify::filter_to_scope(&filter),
        changes.join(", ")
    ))
}

fn parse_grant(subj: &str, pred: &str) -> Option<String> {
    let filter = modify::subject_to_filter(subj)?;
    let changes = modify::parse_keyword_changes(pred)?;
    Some(format!(
        "Static(effects: [Modify(of: {}, changes: [{}])])",
        modify::filter_to_scope(&filter),
        changes.join(", ")
    ))
}

/// "<subject> can't <action[ or action…]>" → one `Cant(<verb>)` per action.
/// The subject is a bare `Filter` (Deontic actions carry `Filter`, not
/// `Scope`). Any action outside the known verb set declines the whole.
fn parse_restriction(subj: &str, pred: &str) -> Option<String> {
    let filter = modify::subject_to_filter(subj)?;
    let effects: Option<Vec<String>> = modify::split_list(pred)
        .iter()
        .map(|act| match act.to_ascii_lowercase().as_str() {
            "attack" => Some(format!("Cant(Attack(by: {filter}))")),
            "block" => Some(format!("Cant(Block(by: {filter}))")),
            _ => None,
        })
        .collect();
    let effects = effects?;
    Some(format!("Static(effects: [{}])", effects.join(", ")))
}

/// "<subject> attack[s] each combat if able" → a `Must(Attack(by: <subject>))`
/// requirement static ([CR#508.1d]). The predicate must be exactly the
/// "each combat if able" requirement tail — any other prose after the verb
/// (e.g. "this turn", "a player") declines, so the bare verb marker never
/// swallows a durational or targeted clause.
fn parse_requirement(subj: &str, pred: &str) -> Option<String> {
    if pred.trim() != "each combat if able" {
        return None;
    }
    let filter = modify::subject_to_filter(subj)?;
    Some(format!("Static(effects: [Must(Attack(by: {filter}))])"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(line: &str) -> Option<String> {
        resolve_line(line, CardKind::Permanent).unwrap()
    }

    #[test]
    fn pt_anthem_you_control() {
        assert_eq!(
            stat("Creatures you control get +1/+1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), changes: [AddPower(Literal(1)), AddToughness(Literal(1))])])"
            )
        );
    }

    #[test]
    fn pt_negative_and_mixed() {
        assert_eq!(
            stat("Creatures your opponents control get -1/-1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(OpponentOf(Ref(You)))])), changes: [SubtractPower(Literal(1)), SubtractToughness(Literal(1))])])"
            )
        );
        assert_eq!(
            stat("~ gets +1/-1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Of(This), changes: [AddPower(Literal(1)), SubtractToughness(Literal(1))])])"
            )
        );
    }

    #[test]
    fn pt_declines() {
        assert!(stat("Creatures you control get +1/+1.").is_some());
        assert!(
            resolve_line("Target creature gets +2/+2.", CardKind::Permanent)
                .unwrap()
                .is_none()
        );
        assert!(
            resolve_line(
                "Creatures you control get +1/+1 until end of turn.",
                CardKind::Permanent
            )
            .unwrap()
            .is_none()
        );
        assert!(
            resolve_line("Creatures you control get +1/+1.", CardKind::Spell)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn grant_single_and_list() {
        assert_eq!(
            stat("Other Goblins have haste.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Permanent, Subtype(\"Goblin\"), Not(Ref(This))])), changes: [GainAbility(Keyword(Haste))])])"
            )
        );
        assert_eq!(
            stat("Creatures you control have flying and vigilance.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), changes: [GainAbility(Keyword(Flying)), GainAbility(Keyword(Vigilance))])])"
            )
        );
        assert_eq!(
            stat("Creatures you control have flying, vigilance, and trample.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), changes: [GainAbility(Keyword(Flying)), GainAbility(Keyword(Vigilance)), GainAbility(Keyword(Trample))])])"
            )
        );
    }

    #[test]
    fn restriction_attack_block() {
        assert_eq!(
            stat("Enchanted creature can't attack or block.").as_deref(),
            Some(
                "Static(effects: [Cant(Attack(by: Ref(AttachHostOf(This)))), Cant(Block(by: Ref(AttachHostOf(This))))])"
            )
        );
        assert_eq!(
            stat("Creatures you control can't attack.").as_deref(),
            Some("Static(effects: [Cant(Attack(by: AllOf([Creature, ControlledBy(Ref(You))])))])")
        );
    }

    #[test]
    fn restriction_declines_unknown_action() {
        assert!(stat("Enchanted creature can't transform.").is_none());
    }

    #[test]
    fn requirement_attack_each_combat() {
        // Goblin Rabblemaster's requirement clause → a Must(Attack) static over
        // the subject filter ([CR#508.1d]).
        assert_eq!(
            stat("Other Goblin creatures you control attack each combat if able.").as_deref(),
            Some(
                "Static(effects: [Must(Attack(by: AllOf([Creature, Not(Ref(This)), Subtype(\"Goblin\"), ControlledBy(Ref(You))])))])"
            )
        );
        // A self-ref subject ("~ attacks each combat if able") → Must over This.
        assert_eq!(
            stat("~ attacks each combat if able.").as_deref(),
            Some("Static(effects: [Must(Attack(by: Ref(This)))])")
        );
    }

    #[test]
    fn requirement_declines_partial_phrase() {
        // The bare " attack " marker must not swallow non-requirement prose; only
        // the exact "each combat if able" tail (singular or plural verb) qualifies.
        assert!(stat("Creatures you control attack this turn if able.").is_none());
        assert!(stat("Creatures you control attack a player.").is_none());
        // A targeted subject still declines (no class filter).
        assert!(stat("Target creature attacks each combat if able.").is_none());
    }

    #[test]
    fn pt_anthem_subtype_adjective() {
        // Elvish Archdruid's anthem: a subtype-adjective subject ("Elf creatures").
        assert_eq!(
            stat("Other Elf creatures you control get +1/+1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, Not(Ref(This)), Subtype(\"Elf\"), ControlledBy(Ref(You))])), changes: [AddPower(Literal(1)), AddToughness(Literal(1))])])"
            )
        );
    }

    #[test]
    fn grant_combo_with_pt() {
        assert_eq!(
            stat("Other Goblins get +1/+1 and have mountainwalk.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Permanent, Subtype(\"Goblin\"), Not(Ref(This))])), changes: [AddPower(Literal(1)), AddToughness(Literal(1)), GainAbility(Keyword(Mountainwalk))])])"
            )
        );
    }

    #[test]
    fn grant_declines() {
        // unknown keyword
        assert!(stat("Creatures you control have wibble.").is_none());
        // parameterized keyword (leftover) — deferred to macro-keyword-templates
        assert!(stat("Creatures you control have protection from red.").is_none());
        // durational gain
        assert!(
            resolve_line(
                "Creatures you control gain trample until end of turn.",
                CardKind::Permanent
            )
            .unwrap()
            .is_none()
        );
    }
}
