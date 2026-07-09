//! Permissions/prohibitions/requirements as sentences.

use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::StaticEffect;

use super::Ctx;
use super::fragment;

/// One sentence for a deontic clause.
pub(super) fn deontic(d: &Deontic, ctx: &Ctx) -> String {
    match d {
        Deontic::Expanded(exp) => deontic(&exp.value, ctx),
        Deontic::Must(a) => requirement(a, ctx),
        Deontic::Cant(a) => prohibition(a, ctx),
        other => format!("[unrendered: {other:?}]."),
    }
}

/// An adjacent `Cant(Attack)` + `Cant(Block)` pair over the SAME subject
/// prints as the single oracle clause — "Enchanted creature can't attack or
/// block." (Pacifism). `None` when the pair doesn't match.
pub(super) fn merged_cant_attack_block(
    first: &StaticEffect,
    second: &StaticEffect,
    ctx: &Ctx,
) -> Option<String> {
    let (Deontic::Cant(a), Deontic::Cant(b)) =
        (peel_static_deontic(first)?, peel_static_deontic(second)?)
    else {
        return None;
    };
    let (DeonticAction::Attack { by: attack_by, .. }, DeonticAction::Block { by: block_by, .. }) =
        (unwrap_action(a), unwrap_action(b))
    else {
        return None;
    };
    if attack_by != block_by {
        return None;
    }
    Some(format!(
        "{} can't attack or block.",
        deontic_subject(attack_by, ctx)
    ))
}

/// The deontic inside a static effect, seen through macro provenance.
fn peel_static_deontic(e: &StaticEffect) -> Option<&Deontic> {
    match e {
        StaticEffect::Deontic(d) => Some(peel_deontic(d)),
        StaticEffect::Expanded(exp) => peel_static_deontic(&exp.value),
        _ => None,
    }
}

fn peel_deontic(d: &Deontic) -> &Deontic {
    match d {
        Deontic::Expanded(e) => peel_deontic(&e.value),
        other => other,
    }
}

fn requirement(a: &DeonticAction, ctx: &Ctx) -> String {
    match unwrap_action(a) {
        DeonticAction::Attack { by, .. } => {
            format!("{} attacks each combat if able.", deontic_subject(by, ctx))
        }
        // "All creatures able to block ~ do so." — the block requirement
        // aimed at THIS ([CR#509.1c] if-able arbitration): the required
        // blockers read as the set-wide subject.
        DeonticAction::Block { by, on, .. } if is_this(on) => {
            format!(
                "All {}s able to block {} do so.",
                fragment::filter_noun(by),
                ctx.subject
            )
        }
        DeonticAction::Block { by, .. } => {
            format!("{} blocks each combat if able.", deontic_subject(by, ctx))
        }
        // The Flagbearer requirement ([CR#601.2c] choosing targets while
        // casting/activating): a Must(Target) whose agent is an
        // opponent-controlled stack object prints the printed while-choosing
        // template.
        DeonticAction::Target { by, on } => {
            let opponent_agent =
                by.source.is_none() && by.stack_object.as_ref().is_some_and(is_opponent_controlled);
            let noun = find_subtype_noun(on);
            match (opponent_agent, noun) {
                (true, Some(noun)) => format!(
                    "While an opponent is choosing targets as part of casting a spell they \
                     control or activating an ability they control, that player must choose at \
                     least one {noun} on the battlefield if able."
                ),
                _ => format!("[unrendered: Must(Target {{ by: {by:?}, on: {on:?} }})]."),
            }
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

fn prohibition(a: &DeonticAction, ctx: &Ctx) -> String {
    match unwrap_action(a) {
        DeonticAction::Attack { by, .. } => {
            format!("{} can't attack.", deontic_subject(by, ctx))
        }
        // The passive "can't be blocked" evasion clause ([CR#509.1b]) anchors
        // on `on`, leaving `by` at its default `Any` and no `count` rider —
        // checked BEFORE the active "can't block" arm below (mirrors the
        // parser's be-blocked-before-block guard: `by` being untouched is
        // what distinguishes this from the active form, which anchors `by`
        // instead). A `count`/`by`-filter rider (menace, "except by N or
        // more creatures", …) isn't this shape — deferred, falls through.
        DeonticAction::Block {
            by,
            on,
            count: None,
        } if is_any(by) => {
            format!("{} can't be blocked.", deontic_subject(on, ctx))
        }
        DeonticAction::Block { by, .. } => {
            format!("{} can't block.", deontic_subject(by, ctx))
        }
        // "This spell can't be countered." ([CR#701.6a]) — the countered
        // object (`on`) is the subject; the any-source agent isn't named.
        DeonticAction::Counter { on, .. } => {
            format!("{} can't be countered.", deontic_subject(on, ctx))
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

fn unwrap_action(a: &DeonticAction) -> &DeonticAction {
    match a {
        DeonticAction::Expanded(e) => unwrap_action(&e.value),
        other => other,
    }
}

fn is_this(f: &Predicate) -> bool {
    matches!(fragment::strip_expanded(f), Predicate::Ref(Reference::This))
}

/// The untouched `Predicate::any()` default — distinguishes an unanchored
/// `Deontic` slot (e.g. `Block`'s `by` in the passive "can't be blocked" form)
/// from a concrete anchor.
fn is_any(f: &Predicate) -> bool {
    matches!(fragment::strip_expanded(f), Predicate::Any)
}

/// `ControlledBy(OpponentOf(You))` — the "an opponent controls" agent.
fn is_opponent_controlled(f: &Predicate) -> bool {
    if let Predicate::Relation(RelationPredicate::ControlledBy(inner)) = fragment::strip_expanded(f)
        && let Predicate::Relation(RelationPredicate::OpponentOf(who)) =
            fragment::strip_expanded(inner)
    {
        return matches!(
            fragment::strip_expanded(who),
            Predicate::Ref(Reference::You)
        );
    }
    false
}

/// The subtype name among a filter's parts ("Flagbearer"), for deontics
/// keyed on a subtype noun.
fn find_subtype_noun(f: &Predicate) -> Option<String> {
    match fragment::strip_expanded(f) {
        Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) => {
            Some(name.as_str().to_string())
        }
        Predicate::And(parts) => parts.iter().find_map(find_subtype_noun),
        _ => None,
    }
}

/// A `Predicate` as the singular subject of a deontic — always a sentence
/// SUBJECT position ("X can't block.", "X can't be blocked."), so every
/// `Ref(<Reference>)` reads through the shared [`fragment::modify_subject`]
/// (the same sentence-start-capitalized noun phrase `Modify`'s subject uses):
/// `This` -> the host's name, `It` -> the announced target's phrase ("Target
/// creature") at a single-slot announce root, `AttachHostOf(This)` ->
/// "Enchanted creature", etc.
fn deontic_subject(f: &Predicate, ctx: &Ctx) -> String {
    match f {
        Predicate::Ref(r) => fragment::modify_subject(r, ctx),
        other => format!("[unrendered: {other:?}]"),
    }
}
