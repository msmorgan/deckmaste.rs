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
        return parse_restriction(subj, pred); // later task
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
    if let Some(rest) = strip_prefix_ci(s, "this ") {
        let _ = rest;
        return Some("Ref(This)".into());
    }
    if let Some(rest) = strip_prefix_ci(s, "enchanted ") {
        let _ = rest; // noun dropped — enchant restriction enforces the type
        return Some("Ref(AttachHostOf(This))".into());
    }
    if strip_prefix_ci(s, "target ").is_some() {
        return None; // targeted/one-shot, not a static anthem
    }
    filter::parse_phrase(s)
}

/// `Ref(r)` filter → `Of(r)` scope; any class filter → `Matching(filter)`.
fn filter_to_scope(f: &str) -> String {
    if let Some(inner) = f.strip_prefix("Ref(").and_then(|x| x.strip_suffix(')')) {
        format!("Of({inner})")
    } else {
        format!("Matching({f})")
    }
}

fn parse_pt(subj: &str, pred: &str) -> Option<String> {
    let filter = subject_to_filter(subj)?;
    // Optional combo tail: "+1/+1 and have/has/gain/gains <kw…>" (a later task
    // fills parse_keyword_changes; for this task there is no tail).
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

// --- filled in later tasks ---
fn parse_grant(_subj: &str, _pred: &str) -> Option<String> {
    None // later task
}
fn parse_keyword_changes(_pred: &str) -> Option<Vec<String>> {
    None // later task
}
fn parse_restriction(_subj: &str, _pred: &str) -> Option<String> {
    None // later task
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
                "Static(effects: [Modify(of: Matching(AllOf([Creature, Controller(Ref(You))])), changes: [AddPower(Literal(1)), AddToughness(Literal(1))])])"
            )
        );
    }

    #[test]
    fn pt_negative_and_mixed() {
        assert_eq!(
            stat("Creatures your opponents control get -1/-1.").as_deref(),
            Some(
                "Static(effects: [Modify(of: Matching(AllOf([Creature, Controller(OpponentOf(Ref(You)))])), changes: [SubtractPower(Literal(1)), SubtractToughness(Literal(1))])])"
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
}
