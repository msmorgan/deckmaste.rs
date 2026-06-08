//! Replacement effects ([CR#614]) — the `ZoneWillChange` replace stage. Stage 3
//! wires self-replacement on enter (`AsEnters`) into the entering object's
//! `EnterStatus`; other replacement kinds are Stage-4 seams (§7.2).

use deckmaste_core::{Ability, Action, Effect, Replacement, Selection, StaticEffect};

use crate::event::EnterStatus;
use crate::object::ObjectSource;
use crate::state::GameState;

impl GameState {
    /// [CR#614.1c,614.12]: the entering status a permanent's own `AsEnters`
    /// self-replacements impose. Stage 3 recognises `AsEnters(Tap(This))` →
    /// enters tapped; other `AsEnters` effects are a `todo!` seam.
    pub(crate) fn as_enters_status(&self, source: ObjectSource) -> EnterStatus {
        let mut status = EnterStatus::default();
        for ability in crate::derive::abilities_of_source(self, source) {
            if let Ability::Static(s) = &ability {
                for eff in &s.effects {
                    if let StaticEffect::Replacement(Replacement::AsEnters { effect }) = eff {
                        apply_as_enters(effect, &mut status);
                    }
                }
            }
        }
        status
    }
}

/// Fold one `AsEnters` effect into the entering status. Stage 3: only
/// `Tap(This)` (→ tapped). Counters/face-down are Stage-4 seams.
fn apply_as_enters(effect: &Effect, status: &mut EnterStatus) {
    match effect {
        Effect::Act(Action::Tap(Selection::This)) => status.tapped = true,
        other => todo!("stage 3 does not interpret AsEnters effect {other:?}"),
    }
}
