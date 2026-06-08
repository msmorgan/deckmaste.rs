//! Replacement effects ([CR#614]) — the `ZoneWillChange` replace stage. Stage 3
//! wires self-replacement on enter (`Also(would: Enters(This), …)`) into the
//! entering object's `EnterStatus`; other replacement kinds are Stage-4 seams
//! (§7.2).

use deckmaste_core::{
    Ability, Action, Effect, Event, Filter, Reference, Replacement, Selection, StaticEffect, Zone,
};

use crate::event::EnterStatus;
use crate::object::ObjectSource;
use crate::state::GameState;

impl GameState {
    /// [CR#614.1c,614.12]: the entering status a permanent's own augment
    /// replacements (`Also(would: Enters(This), also: …)`) impose. Stage 3
    /// recognises `Also(would: Enters(This), also: Tap(This))` → enters tapped;
    /// other `also` effects are a `todo!` seam.
    pub(crate) fn as_enters_status(&self, source: ObjectSource) -> EnterStatus {
        let mut status = EnterStatus::default();
        for ability in crate::derive::abilities_of_source(self, source) {
            if let Ability::Static(s) = &ability {
                for eff in &s.effects {
                    if let StaticEffect::Replacement(replacement) = eff
                        && let Replacement::Also { would, also } = look_through(replacement)
                        && would_is_self_enter(would)
                    {
                        apply_as_enters(also, &mut status);
                    }
                }
            }
        }
        status
    }
}

/// Look through a remembered `Replacement` macro invocation (`AsEnters`, …) to
/// the form it expanded to.
fn look_through(replacement: &Replacement) -> &Replacement {
    match replacement {
        Replacement::Expanded(e) => look_through(&e.value),
        other => other,
    }
}

/// Whether `would` is an enter-the-battlefield event for the watching object
/// itself — the `Enters(This)`/`Enters(Is(This))` shape, looked through any
/// remembered macro invocation. Such a `would` on a static replacement is the
/// object's own self-enter (the watcher in `as_enters_status` is always self),
/// so a `Is(This)`/`Any` `what` both qualify.
fn would_is_self_enter(would: &Event) -> bool {
    match would {
        // Look through `Enters(…)` and any other remembered Event macro.
        Event::Expanded(e) => would_is_self_enter(&e.value),
        // A move *to* the battlefield, of this object (or match-anything).
        Event::ZoneMove { what, to, .. } => {
            *to == Some(Zone::Battlefield)
                && matches!(what, Filter::Is(Reference::This) | Filter::Any)
        }
        _ => false,
    }
}

/// Fold one `also` effect into the entering status. Stage 3: only `Tap(This)`
/// (→ tapped). Counters/face-down are Stage-4 seams.
fn apply_as_enters(effect: &Effect, status: &mut EnterStatus) {
    match effect {
        Effect::Act(Action::Tap(Selection::Ref(Reference::This))) => status.tapped = true,
        other => todo!("stage 3 does not interpret enters-replacement effect {other:?}"),
    }
}
