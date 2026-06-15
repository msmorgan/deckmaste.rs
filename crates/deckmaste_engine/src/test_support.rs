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
    Frame {
        source: state.player(player).object,
        controller: player,
        targets: Vec::new(),
        bindings: None,
        chosen: None,
        x: None,
    }
}

/// A plain resolution frame anchored on `source` (controlled by player 0) with
/// no targets, bindings, choice, or X — the shape most effect/action tests
/// build to drive `run_effect`/`eval_*`.
pub(crate) fn frame_src(source: crate::object::ObjectId) -> Frame {
    frame_src_targets(source, Vec::new())
}

/// Like [`frame_src`] but with explicit `targets` — for effects/references that
/// read `Reference::Target(_)`.
pub(crate) fn frame_src_targets(
    source: crate::object::ObjectId,
    targets: Vec<crate::object::ObjectId>,
) -> Frame {
    Frame {
        source,
        controller: PlayerId(0),
        targets,
        bindings: None,
        chosen: None,
        x: None,
    }
}
