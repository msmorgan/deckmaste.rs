//! Always-on static abilities on permanents: "gets ±N/±N", "have/has/gain/gains
//! <keyword>", "<subject> can't attack/block". The ±N/±N + keyword-grant +
//! subject/scope grammar is shared via [`crate::parsers::modify`]; this module
//! renders `Static(effects: [...])` RON. Declines (`Ok(None)`) on spells,
//! durational clauses, targeted subjects, or anything its productions don't
//! fully cover.

use crate::parsers::modify;
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    Ok(parse(line, ctx.kind))
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
    if let Some((subj, pred)) = modify::split_marker(body, &[" can block "]) {
        return parse_block_permission(subj, pred);
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
/// `Scope`). The active verbs ("attack"/"block") anchor the subject on the
/// actor side (`by`); the passive "be blocked …" evasion forms anchor it on
/// the blocked side (`on`) and read a blocker-quality clause. Any action
/// outside the known set declines the whole.
fn parse_restriction(subj: &str, pred: &str) -> Option<String> {
    let filter = modify::subject_to_filter(subj)?;
    // The passive "be blocked …" evasion clause is a single clause whose tail
    // ("by creatures with power 2 or less") contains its own "or" — handle it
    // BEFORE the active-verb list split, which would shred that tail.
    let low = pred.to_ascii_lowercase();
    if low.starts_with("be blocked") {
        let row = parse_cant_be_blocked(&filter, &low)?;
        return Some(format!("Static(effects: [{row}])"));
    }
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

/// The passive "be blocked …" evasion clause ([CR#509.1b], [CR#702]) — the
/// subject (`on`) is the creature being blocked. `clause` is the lowercased
/// predicate after "can't " (e.g. "be blocked", "be blocked by creatures with
/// power 2 or less", "be blocked by more than one creature"). Returns one
/// `Cant(Block(on: <subj>, …))` row, or `None` for an unrecognized tail.
fn parse_cant_be_blocked(on: &str, clause: &str) -> Option<String> {
    let tail = clause.strip_prefix("be blocked")?.trim();
    // "~ can't be blocked." — unblockable: no blocker may block it.
    if tail.is_empty() {
        return Some(format!("Cant(Block(on: {on}))"));
    }
    // "… by more than one creature." — an arrangement bound: a blocker set
    // larger than one is forbidden ([CR#509.1b]).
    if tail == "by more than one creature" {
        return Some(format!("Cant(Block(on: {on}, count: Greater(1)))"));
    }
    // "… except by N or more creatures." — menace generalized ([CR#702.111b]
    // is the N=2 case): a blocker set of fewer than N is forbidden. N is a
    // spelled cardinal ("three") or a digit.
    if let Some(n) = tail
        .strip_prefix("except by ")
        .and_then(|t| t.strip_suffix(" or more creatures"))
        .and_then(crate::parsers::effect::number_word)
    {
        return Some(format!("Cant(Block(on: {on}, count: Less({n})))"));
    }
    // "… by creatures with power N or less/greater." — a blocker-quality
    // restriction (the candidate blocker's power).
    let by = tail
        .strip_prefix("by ")
        .and_then(crate::parsers::filter::parse_phrase)?;
    Some(format!("Cant(Block(on: {on}, by: {by}))"))
}

/// "<subject> can block <predicate>" → a blocking restriction/permission.
///   - "only creatures with flying" → `Cant(Block(by: <subj>, on: Not(<X>)))`:
///     the subject can't block anything that isn't `<X>` ([CR#509.1a]).
///   - "an additional creature each combat" → a multi-block permission
///     ([CR#509.1a] default = one). Not yet engine-evaluated (the May/Gate
///     Block seam), but representable, so it graduates.
fn parse_block_permission(subj: &str, pred: &str) -> Option<String> {
    let by = modify::subject_to_filter(subj)?;
    if let Some(only) = pred.strip_prefix("only ") {
        let on = crate::parsers::filter::parse_phrase(only.trim())?;
        return Some(format!(
            "Static(effects: [Cant(Block(by: {by}, on: Not({on})))])"
        ));
    }
    if pred == "an additional creature each combat" {
        // The default per-blocker cap is one creature ([CR#509.1a]); this row
        // raises it to two. A May permission over the second-block slot.
        return Some(format!(
            "Static(effects: [May(Block(by: {by}, count: AtMost(2)))])"
        ));
    }
    None
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
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    #[test]
    fn pt_anthem_you_control() {
        assert_eq!(
            stat("Creatures you control get +1/+1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), changes: [AddPowerToughness(1, 1)])])"
            )
        );
    }

    #[test]
    fn pt_negative_and_mixed() {
        assert_eq!(
            stat("Creatures your opponents control get -1/-1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, ControlledBy(OpponentOf(Ref(You)))])), changes: [SubtractPower(1), SubtractToughness(1)])])"
            )
        );
        assert_eq!(
            stat("~ gets +1/-1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Of(This), changes: [AddPower(1), SubtractToughness(1)])])"
            )
        );
    }

    #[test]
    fn pt_declines() {
        assert!(stat("Creatures you control get +1/+1.").is_some());
        assert!(
            resolve_line(
                "Target creature gets +2/+2.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
        assert!(
            resolve_line(
                "Creatures you control get +1/+1 until end of turn.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
        assert!(
            resolve_line(
                "Creatures you control get +1/+1.",
                &crate::parsers::test_ctx::ctx(CardKind::Spell)
            )
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
    fn cant_be_blocked_unblockable() {
        // Unblockable: no blocker may block This ([CR#509.1b]).
        assert_eq!(
            stat("~ can't be blocked.").as_deref(),
            Some("Static(effects: [Cant(Block(on: Ref(This)))])")
        );
        // Equipment/aura conferral: the host can't be blocked.
        assert_eq!(
            stat("Equipped creature can't be blocked.").as_deref(),
            Some("Static(effects: [Cant(Block(on: Ref(AttachHostOf(This))))])")
        );
        assert_eq!(
            stat("Enchanted creature can't be blocked.").as_deref(),
            Some("Static(effects: [Cant(Block(on: Ref(AttachHostOf(This))))])")
        );
    }

    #[test]
    fn cant_be_blocked_by_power() {
        // Blocker-quality power restriction (the candidate blocker's power).
        assert_eq!(
            stat("~ can't be blocked by creatures with power 2 or less.").as_deref(),
            Some(
                "Static(effects: [Cant(Block(on: Ref(This), by: AllOf([Creature, Stat(Power, AtMost, 2)])))])"
            )
        );
        assert_eq!(
            stat("~ can't be blocked by creatures with power 3 or greater.").as_deref(),
            Some(
                "Static(effects: [Cant(Block(on: Ref(This), by: AllOf([Creature, Stat(Power, AtLeast, 3)])))])"
            )
        );
    }

    #[test]
    fn cant_be_blocked_by_more_than_one() {
        // An arrangement bound: a blocker set larger than one is forbidden.
        assert_eq!(
            stat("~ can't be blocked by more than one creature.").as_deref(),
            Some("Static(effects: [Cant(Block(on: Ref(This), count: Greater(1)))])")
        );
        // Conferred form on the equip host.
        assert_eq!(
            stat("Each creature you control can't be blocked by more than one creature.")
                .as_deref(),
            Some(
                "Static(effects: [Cant(Block(on: AllOf([Creature, ControlledBy(Ref(You))]), count: Greater(1)))])"
            )
        );
    }

    #[test]
    fn cant_be_blocked_except_by_n_or_more() {
        // Menace generalized to N=3 — a spelled cardinal in the corpus.
        assert_eq!(
            stat("~ can't be blocked except by three or more creatures.").as_deref(),
            Some("Static(effects: [Cant(Block(on: Ref(This), count: Less(3)))])")
        );
    }

    #[test]
    fn can_block_only_flying() {
        // "can block only creatures with flying" → can't block non-flying
        // ([CR#509.1a]).
        assert_eq!(
            stat("~ can block only creatures with flying.").as_deref(),
            Some(
                "Static(effects: [Cant(Block(by: Ref(This), on: Not(AllOf([Creature, Has(Flying)]))))])"
            )
        );
    }

    #[test]
    fn can_block_additional_creature() {
        // "can block an additional creature each combat" raises the per-blocker
        // cap to two ([CR#509.1a]) — a May permission.
        assert_eq!(
            stat("~ can block an additional creature each combat.").as_deref(),
            Some("Static(effects: [May(Block(by: Ref(This), count: AtMost(2)))])")
        );
    }

    #[test]
    fn equipped_creature_gets() {
        // "Equipped creature gets +N/+N" → Modify on the attach host, the same
        // shape as the already-wired "Enchanted creature".
        assert_eq!(
            stat("Equipped creature gets +2/+0.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Of(AttachHostOf(This)), changes: [AddPowerToughness(2, 0)])])"
            )
        );
        assert_eq!(
            stat("Equipped creature gets +1/+1 and has trample.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Of(AttachHostOf(This)), changes: [AddPowerToughness(1, 1), GainAbility(Keyword(Trample))])])"
            )
        );
    }

    #[test]
    fn cant_be_blocked_declines_unknown_tail() {
        // An unrecognized "be blocked …" tail (a phrase the filter parser can't
        // resolve) declines — no wrong Block row.
        assert!(stat("~ can't be blocked by creatures wearing hats.").is_none());
        assert!(stat("~ can block only creatures wearing hats.").is_none());
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
                "Static(effects: [Modify(of: Matching(AllOf([Creature, Not(Ref(This)), Subtype(\"Elf\"), ControlledBy(Ref(You))])), changes: [AddPowerToughness(1, 1)])])"
            )
        );
    }

    #[test]
    fn grant_combo_with_pt() {
        assert_eq!(
            stat("Other Goblins get +1/+1 and have mountainwalk.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Permanent, Subtype(\"Goblin\"), Not(Ref(This))])), changes: [AddPowerToughness(1, 1), GainAbility(Keyword(Mountainwalk))])])"
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
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
    }
}
