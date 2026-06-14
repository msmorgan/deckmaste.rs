//! Permissions/prohibitions/requirements as sentences.

use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Filter;
use deckmaste_core::Reference;

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

fn requirement(a: &DeonticAction, subject: &str) -> String {
    match unwrap_action(a) {
        DeonticAction::Attack { by, .. } => {
            format!(
                "{} attacks each combat if able.",
                deontic_subject(by, subject)
            )
        }
        DeonticAction::Block { by, .. } => {
            format!(
                "{} blocks each combat if able.",
                deontic_subject(by, subject)
            )
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

/// A `Filter` as the singular subject of a deontic. `subject` is the host's
/// name. `Ref(This)` -> the host's name; `Ref(AttachHostOf(This))` ->
/// "Enchanted creature".
fn deontic_subject(f: &Filter, subject: &str) -> String {
    match f {
        Filter::Ref(Reference::This) => subject.to_string(),
        Filter::Ref(Reference::AttachHostOf(inner)) if matches!(**inner, Reference::This) => {
            "Enchanted creature".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}
