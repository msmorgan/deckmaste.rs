//! The shared characteristic-modification grammar: the `±N/±N` power/toughness
//! changes, the "and gain/have <keyword…>" grant tail, and the subject→
//! [`Target`] mapping that a `Modify(<ref>, <change>)` /
//! `Each(SelectAll(<filter>), Modify(It, <change>))` consumes. Two positions
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

/// Subject phrase → `Predicate` RON, or `None` to decline. `~`/"this …" are the
/// self-ref; "enchanted …" the attach host; a class phrase parses via
/// [`filter::parse_phrase`]. "target …" declines here — a targeted subject is
/// the caller's concern (it declares a `TargetSpec` and scopes
/// `Of(It)`).
pub(super) fn subject_to_filter(subj: &str) -> Option<String> {
    let s = subj.trim();
    if s == "~" {
        return Some("Ref(This)".into());
    }
    if strip_prefix_ci(s, "this ").is_some() {
        return Some("Ref(This)".into());
    }
    if strip_prefix_ci(s, "enchanted ").is_some() || strip_prefix_ci(s, "equipped ").is_some() {
        // noun dropped — enchant/equip restriction enforces the type. Both the
        // aura's "Enchanted creature …" and the Equipment's "Equipped creature
        // …" name the single attach host ([CR#702.5a,301.5a]); the conferred
        // Modify lands on it via `Of(AttachHostOf(This))`.
        return Some("Ref(AttachHostOf(This))".into());
    }
    if strip_prefix_ci(s, "target ").is_some() {
        return None; // targeted/one-shot, not a class subject
    }
    filter::parse_phrase(s)
}

/// Where a `Modify` distributes: a bare object `Reference` (a self/attach-host
/// subject), spliced directly into `Modify(<ref>, <change>)`; or a class
/// filter, which has no single-object form and must be distributed via
/// `Each(SelectAll(<filter>), Modify(It, <change>))` — the ONLY way a static
/// reaches many objects (`StaticEffect::Each` in `deckmaste_core`).
pub(super) enum Target {
    /// A bare `Reference` string (`This`, `AttachHostOf(This)`, `It`, …).
    Ref(String),
    /// A class filter.
    Predicate(String),
}

impl Target {
    /// Wrap a single `Modification` value `change` into the full
    /// `Modify`/`Each` RON for this target.
    pub(super) fn wrap(&self, change: &str) -> String {
        match self {
            Target::Ref(r) => format!("Modify({r}, {change})"),
            Target::Predicate(f) => format!("Each(SelectAll({f}), Modify(It, {change}))"),
        }
    }
}

/// `Ref(r)` filter → a bare-reference `Target::Ref(r)`; any class filter →
/// `Target::Predicate(filter)`. `parse_phrase` always leads with a head-noun
/// atom, so a top-level `Ref(` here can only be our own `subject_to_filter`
/// self-refs (`~`/enchanted); class subjects take the `Predicate` branch.
pub(super) fn filter_to_target(f: &str) -> Target {
    if let Some(inner) = f.strip_prefix("Ref(").and_then(|x| x.strip_suffix(')')) {
        Target::Ref(inner.to_owned())
    } else {
        Target::Predicate(f.to_owned())
    }
}

/// Bundle a changes list into ONE `Modification`, as `Modify` now takes a
/// single `Modification` (bundle several ops with `Modification::Several`):
/// a singleton list is spliced bare, a plural list wraps `Several([...])`.
pub(super) fn changes_to_modification(changes: &[String]) -> String {
    match changes {
        [one] => one.clone(),
        many => format!("Several([{}])", many.join(", ")),
    }
}

/// "+N/+M" / "-N/-M" / mixed → the `changes` list. The both-positive case
/// (`+N/+M`) emits the single `AddPowerToughness(N, M)` bundling macro, which
/// expands to `Several([Power(Up(N)), Toughness(Up(M))])` and renders via its
/// `"gets +{0}/+{1}"` template — the DRY anthem/pump form. Any negative side
/// (the `-N` debuffs, mixed `+N/-M`) has no such macro, so it keeps the inline
/// `[Power(Up|Down(N)), Toughness(Up|Down(M))]` pair.
pub(super) fn parse_pt_changes(s: &str) -> Option<Vec<String>> {
    let (p, t) = s.split_once('/')?;
    let (pv, pn) = signed(p)?;
    let (tv, tn) = signed(t)?;
    if pv == "Up" && tv == "Up" {
        return Some(vec![format!("AddPowerToughness({pn}, {tn})")]);
    }
    Some(vec![
        format!("Power({pv}({pn}))"),
        format!("Toughness({tv}({tn}))"),
    ])
}

/// "+1/+1" / "+1/+0" with a dynamic `count` (a `CountOf(…)` RON) ->
/// [Power(Up(...)), Toughness(Up(...))]. Each side must be `+1` (scales to
/// `count`) or `+0` (bare `0`); any other magnitude or a negative sign declines
/// — the `Count` grammar has no product form, and a negative count is
/// meaningless (object counts are non-negative [CR#107.1b]).
pub(super) fn parse_pt_changes_scaled(s: &str, count: &str) -> Option<Vec<String>> {
    let (p, t) = s.split_once('/')?;
    Some(vec![
        format!("Power(Up({}))", scaled_side(p, count)?),
        format!("Toughness(Up({}))", scaled_side(t, count)?),
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

/// One signed P/T token → (`NumericOp` op name, magnitude): `+N` → `Up`, `-N` →
/// `Down`.
fn signed(tok: &str) -> Option<(&'static str, u32)> {
    let tok = tok.trim();
    // std u32::parse tolerates a leading '+', so "++1" would collapse to +1;
    // harmless — oracle text never produces it.
    if let Some(n) = tok.strip_prefix('+') {
        Some(("Up", n.parse().ok()?))
    } else if let Some(n) = tok.strip_prefix('-') {
        Some(("Down", n.parse().ok()?))
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
            keyword_ability::match_keyword_invocation(kw)
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
