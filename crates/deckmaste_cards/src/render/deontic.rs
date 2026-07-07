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

/// One sentence for a deontic clause. `subject` is the host object's display
/// name.
pub(super) fn deontic(d: &Deontic, subject: &str) -> String {
    match d {
        Deontic::Expanded(exp) => deontic(&exp.value, subject),
        Deontic::Must(a) => requirement(a, subject),
        Deontic::Cant(a) => prohibition(a, subject),
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
        deontic_subject(attack_by, ctx.subject)
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

fn requirement(a: &DeonticAction, subject: &str) -> String {
    match unwrap_action(a) {
        DeonticAction::Attack { by, .. } => {
            format!(
                "{} attacks each combat if able.",
                deontic_subject(by, subject)
            )
        }
        // "All creatures able to block ~ do so." — the block requirement
        // aimed at THIS ([CR#509.1c] if-able arbitration): the required
        // blockers read as the set-wide subject.
        DeonticAction::Block { by, on, .. } if is_this(on) => {
            format!(
                "All {}s able to block {subject} do so.",
                fragment::filter_noun(by)
            )
        }
        DeonticAction::Block { by, .. } => {
            format!(
                "{} blocks each combat if able.",
                deontic_subject(by, subject)
            )
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

fn prohibition(a: &DeonticAction, subject: &str) -> String {
    match unwrap_action(a) {
        DeonticAction::Attack { by, .. } => {
            format!("{} can't attack.", deontic_subject(by, subject))
        }
        DeonticAction::Block { by, .. } => {
            format!("{} can't block.", deontic_subject(by, subject))
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
        Predicate::AllOf(parts) => parts.iter().find_map(find_subtype_noun),
        _ => None,
    }
}

/// A `Predicate` as the singular subject of a deontic. `subject` is the host's
/// name. `Ref(This)` -> the host's name; `Ref(AttachHostOf(This))` ->
/// "Enchanted creature".
fn deontic_subject(f: &Predicate, subject: &str) -> String {
    match f {
        Predicate::Ref(Reference::This) => subject.to_string(),
        Predicate::Ref(Reference::AttachHostOf(inner)) if matches!(**inner, Reference::This) => {
            "Enchanted creature".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}
