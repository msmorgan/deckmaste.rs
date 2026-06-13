//! Always-on static abilities on permanents: "gets ±N/±N", "have/has/gain/gains
//! <keyword>", "<subject> can't attack/block". Subjects parse via
//! `filter::parse_phrase`; renders `Static(effects: [...])` RON. Declines
//! (`Ok(None)`) on spells, durational clauses, targeted subjects, or anything
//! its productions don't fully cover.

use crate::parsers::filter;
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

    if let Some((subj, pred)) = split_marker(body, &[" can't ", " cannot "]) {
        return parse_restriction(subj, pred);
    }
    if let Some((subj, pred)) = split_marker(body, &[" gets ", " get "]) {
        return parse_pt(subj, pred);
    }
    if let Some((subj, pred)) = split_marker(body, &[" have ", " has ", " gain ", " gains "]) {
        return parse_grant(subj, pred); // later task
    }
    None
}

/// Split `body` at the first occurrence of any marker → (subject, predicate).
fn split_marker<'a>(body: &'a str, markers: &[&str]) -> Option<(&'a str, &'a str)> {
    markers
        .iter()
        .filter_map(|m| body.find(m).map(|i| (i, m.len())))
        .min_by_key(|(i, _)| *i)
        .map(|(i, mlen)| (body[..i].trim(), body[i + mlen..].trim()))
}

/// Subject phrase → `Filter` RON, or `None` to decline.
fn subject_to_filter(subj: &str) -> Option<String> {
    let s = subj.trim();
    if s == "~" {
        return Some("Ref(This)".into());
    }
    if strip_prefix_ci(s, "this ").is_some() {
        return Some("Ref(This)".into());
    }
    if strip_prefix_ci(s, "enchanted ").is_some() {
        // noun dropped — enchant restriction enforces the type
        return Some("Ref(AttachHostOf(This))".into());
    }
    if strip_prefix_ci(s, "target ").is_some() {
        return None; // targeted/one-shot, not a static anthem
    }
    filter::parse_phrase(s)
}

/// `Ref(r)` filter → `Of(r)` scope; any class filter → `Matching(filter)`.
// `parse_phrase` always leads with a head-noun atom, so a top-level `Ref(` here
// can only be our own `subject_to_filter` self-refs (`~`/enchanted); class
// subjects take the `Matching(...)` branch.
fn filter_to_scope(f: &str) -> String {
    if let Some(inner) = f.strip_prefix("Ref(").and_then(|x| x.strip_suffix(')')) {
        format!("Of({inner})")
    } else {
        format!("Matching({f})")
    }
}

fn parse_pt(subj: &str, pred: &str) -> Option<String> {
    let filter = subject_to_filter(subj)?;
    // Optional combo tail: "+N/+M and have/has/gain/gains <kw…>" → the P/T changes
    // followed by one GainAbility per granted keyword.
    let (pt_part, grant_tail) = split_grant_tail(pred);
    let mut changes = parse_pt_changes(pt_part.trim())?;
    if let Some(tail) = grant_tail {
        changes.extend(parse_keyword_changes(tail)?);
    }
    Some(format!(
        "Static(effects: [Modify(of: {}, changes: [{}])])",
        filter_to_scope(&filter),
        changes.join(", ")
    ))
}

/// "+N/+M" / "-N/-M" / mixed → [<Add|Subtract>Power, <Add|Subtract>Toughness].
fn parse_pt_changes(s: &str) -> Option<Vec<String>> {
    let (p, t) = s.split_once('/')?;
    let (pv, pn) = signed(p)?;
    let (tv, tn) = signed(t)?;
    Some(vec![
        format!("{pv}Power(Literal({pn}))"),
        format!("{tv}Toughness(Literal({tn}))"),
    ])
}

fn signed(tok: &str) -> Option<(&'static str, u32)> {
    let tok = tok.trim();
    // std u32::parse tolerates a leading '+', so "++1" would collapse to +1;
    // harmless — oracle text never produces it.
    if let Some(n) = tok.strip_prefix('+') {
        Some(("Add", n.parse().ok()?))
    } else if let Some(n) = tok.strip_prefix('-') {
        Some(("Subtract", n.parse().ok()?))
    } else {
        None
    }
}

/// Split a trailing " and have/has/gain/gains <…>" off a predicate.
fn split_grant_tail(pred: &str) -> (&str, Option<&str>) {
    for marker in [" and have ", " and has ", " and gain ", " and gains "] {
        if let Some(i) = pred.find(marker) {
            return (&pred[..i], Some(pred[i + marker.len()..].trim()));
        }
    }
    (pred, None)
}

fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    s.get(..prefix.len())
        .filter(|p| p.eq_ignore_ascii_case(prefix))
        .map(|_| &s[prefix.len()..])
}

fn parse_grant(subj: &str, pred: &str) -> Option<String> {
    let filter = subject_to_filter(subj)?;
    let changes = parse_keyword_changes(pred)?;
    Some(format!(
        "Static(effects: [Modify(of: {}, changes: [{}])])",
        filter_to_scope(&filter),
        changes.join(", ")
    ))
}

/// "flying", "flying and haste", "flying, vigilance, and trample" → one
/// `GainAbility` per keyword. Any unknown / parameterized word declines the
/// whole.
fn parse_keyword_changes(pred: &str) -> Option<Vec<String>> {
    split_list(pred)
        .iter()
        .map(|kw| {
            crate::parsers::keyword_ability::match_keyword_name(kw)
                .map(|name| format!("GainAbility(Keyword({name}))"))
        })
        .collect()
}

/// Split a comma/"and"/"or"-separated list: "a, b, and c" / "a and b" /
/// "a or b" → [a, b, c].
fn split_list(s: &str) -> Vec<String> {
    s.replace(", and ", ", ")
        .replace(" and ", ", ")
        .replace(" or ", ", ")
        .split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

/// "<subject> can't <action[ or action…]>" → one `Cant(<verb>)` per action.
/// The subject is a bare `Filter` (Deontic actions carry `Filter`, not
/// `Scope`). Any action outside the known verb set declines the whole.
fn parse_restriction(subj: &str, pred: &str) -> Option<String> {
    let filter = subject_to_filter(subj)?;
    let effects: Option<Vec<String>> = split_list(pred)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(line: &str) -> Option<String> { resolve_line(line, CardKind::Permanent).unwrap() }

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
                "Static(effects: [Modify(of: Matching(AllOf([Subtype(\"Goblin\"), Not(Ref(This))])), changes: [GainAbility(Keyword(Haste))])])"
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
    fn grant_combo_with_pt() {
        assert_eq!(
            stat("Other Goblins get +1/+1 and have mountainwalk.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Subtype(\"Goblin\"), Not(Ref(This))])), changes: [AddPower(Literal(1)), AddToughness(Literal(1)), GainAbility(Keyword(Mountainwalk))])])"
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
