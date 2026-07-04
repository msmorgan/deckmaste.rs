//! Shared test-only fixtures. Construction boilerplate that more than one
//! module's `#[cfg(test)] mod tests` builds the same way lives here so the
//! call sites stay focused on the behavior under test.

use crate::player::PlayerId;
use crate::stack::Frame;
use crate::state::GameState;

/// A minimal player-anchored frame (no bindings, no targets) — the gate-time
/// shape, enough to evaluate the context-free conditions these unit tests
/// exercise.
pub(crate) fn frame_for(state: &GameState, player: PlayerId) -> Frame {
    Frame::bare(state.player(player).object, player)
}

/// A plain resolution frame anchored on `source` (controlled by player 0) with
/// no targets, bindings, choice, or X — the shape most effect/action tests
/// build to drive `run_effect`/`eval_*`.
pub(crate) fn frame_src(source: crate::object::ObjectId) -> Frame {
    frame_src_targets(source, Vec::new())
}

/// Like [`frame_src`] but with explicit `targets` — for effects/references
/// that read the announced slot (`It` over a lone target, `They` over a
/// plural slot).
pub(crate) fn frame_src_targets(
    source: crate::object::ObjectId,
    targets: Vec<crate::object::ObjectId>,
) -> Frame {
    Frame {
        endophora: crate::stack::Endophora {
            targets,
            ..crate::stack::Endophora::empty()
        },
        ..Frame::bare(source, PlayerId(0))
    }
}

/// The labeled announce-slot read `The(label)` ([CR#608.2d]) — test
/// shorthand pairing with [`frame_src_labeled`].
pub(crate) fn the(label: &str) -> deckmaste_core::Reference {
    deckmaste_core::Reference::The(deckmaste_core::Ident::new(label))
}

/// Like [`frame_src_targets`] but with each slot `As`-NAMED, one object per
/// slot — for bodies that read multiple announce slots via
/// `Reference::The(label)` ([CR#608.2d]; the fight family's shape).
pub(crate) fn frame_src_labeled(
    source: crate::object::ObjectId,
    slots: Vec<(&str, crate::object::ObjectId)>,
) -> Frame {
    Frame {
        endophora: crate::stack::Endophora {
            targets: slots.iter().map(|(_, id)| *id).collect(),
            labeled: slots
                .into_iter()
                .map(|(label, id)| (deckmaste_core::Ident::new(label), vec![id]))
                .collect(),
            ..crate::stack::Endophora::empty()
        },
        ..Frame::bare(source, PlayerId(0))
    }
}
