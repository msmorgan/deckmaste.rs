//! The shared characteristic-modification grammar: the `±N/±N` power/toughness
//! changes, the "and gain/have <keyword…>" grant tail, and the subject→`Scope`
//! mapping that a `Modify(of: <scope>, changes: [...])` consumes. Two positions
//! speak it: `static_ability` (always-on anthems, wrapped in `Static`) and
//! `effect` (one-shot durational pumps, wrapped in `Continuously`). Kept here
//! so both share one grammar instead of duplicating it.

use crate::parsers::filter;
use crate::parsers::keyword_ability;

/// Split `body` at the first occurrence of any marker → (subject, predicate),
/// each trimmed. Earliest marker wins.
pub(super) fn split_marker<'a>(body: &'a str, markers: &[&str]) -> Option<(&'a str, &'a str)> {
    markers
        .iter()
        .filter_map(|m| body.find(m).map(|i| (i, m.len())))
        .min_by_key(|(i, _)| *i)
        .map(|(i, mlen)| (body[..i].trim(), body[i + mlen..].trim()))
}

/// Subject phrase → `Filter` RON, or `None` to decline. `~`/"this …" are the
/// self-ref; "enchanted …" the attach host; a class phrase parses via
/// [`filter::parse_phrase`]. "target …" declines here — a targeted subject is
/// the caller's concern (it declares a `TargetSpec` and scopes
/// `Of(Target(0))`).
pub(super) fn subject_to_filter(subj: &str) -> Option<String> {
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
        return None; // targeted/one-shot, not a class subject
    }
    filter::parse_phrase(s)
}

/// `Ref(r)` filter → `Of(r)` scope; any class filter → `Matching(filter)`.
// `parse_phrase` always leads with a head-noun atom, so a top-level `Ref(` here
// can only be our own `subject_to_filter` self-refs (`~`/enchanted); class
// subjects take the `Matching(...)` branch.
pub(super) fn filter_to_scope(f: &str) -> String {
    if let Some(inner) = f.strip_prefix("Ref(").and_then(|x| x.strip_suffix(')')) {
        format!("Of({inner})")
    } else {
        format!("Matching({f})")
    }
}

/// "+N/+M" / "-N/-M" / mixed → [<Add|Subtract>Power, <Add|Subtract>Toughness].
pub(super) fn parse_pt_changes(s: &str) -> Option<Vec<String>> {
    let (p, t) = s.split_once('/')?;
    let (pv, pn) = signed(p)?;
    let (tv, tn) = signed(t)?;
    Some(vec![
        format!("{pv}Power({pn})"),
        format!("{tv}Toughness({tn})"),
    ])
}

/// "+1/+1" / "+1/+0" with a dynamic `count` (a `CountOf(…)` RON) ->
/// [Add Power, Add Toughness]. Each side must be `+1` (scales to `count`) or
/// `+0` (bare `0`); any other magnitude or a negative sign declines — the
/// `Count` grammar has no product form, and a negative count is meaningless
/// (object counts are non-negative [CR#107.1b]).
pub(super) fn parse_pt_changes_scaled(s: &str, count: &str) -> Option<Vec<String>> {
    let (p, t) = s.split_once('/')?;
    Some(vec![
        format!("AddPower({})", scaled_side(p, count)?),
        format!("AddToughness({})", scaled_side(t, count)?),
    ])
}

/// One signed P/T token under a dynamic count: `+1` -> the count, `+0` ->
/// bare `0`, anything else -> `None`.
fn scaled_side(tok: &str, count: &str) -> Option<String> {
    match tok.trim() {
        "+1" => Some(count.to_owned()),
        "+0" => Some("0".to_owned()),
        _ => None,
    }
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
pub(super) fn split_grant_tail(pred: &str) -> (&str, Option<&str>) {
    for marker in [" and have ", " and has ", " and gain ", " and gains "] {
        if let Some(i) = pred.find(marker) {
            return (&pred[..i], Some(pred[i + marker.len()..].trim()));
        }
    }
    (pred, None)
}

pub(super) fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    s.get(..prefix.len())
        .filter(|p| p.eq_ignore_ascii_case(prefix))
        .map(|_| &s[prefix.len()..])
}

/// "flying", "flying and haste", "flying, vigilance, and trample" → one
/// `GainAbility` per keyword. Any unknown / parameterized word declines the
/// whole.
pub(super) fn parse_keyword_changes(pred: &str) -> Option<Vec<String>> {
    split_list(pred)
        .iter()
        .map(|kw| {
            keyword_ability::match_keyword_name(kw)
                .map(|name| format!("GainAbility(Keyword({name}))"))
        })
        .collect()
}

/// Split a comma/"and"/"or"-separated list: "a, b, and c" / "a and b" /
/// "a or b" → [a, b, c].
pub(super) fn split_list(s: &str) -> Vec<String> {
    s.replace(", and ", ", ")
        .replace(" and ", ", ")
        .replace(" or ", ", ")
        .split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}
