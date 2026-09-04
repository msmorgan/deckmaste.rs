//! Shared test-only fixtures. Construction boilerplate that more than one
//! module's `#[cfg(test)] mod tests` builds the same way lives here so the
//! call sites stay focused on the behavior under test.

use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::state::GameState;

/// A minimal player-anchored frame (no bindings, no targets) — the gate-time
/// shape, enough to evaluate the context-free conditions these unit tests
/// exercise.
pub(crate) fn frame_for(state: &GameState, player: PlayerId) -> ExecutionFrame {
    state.frame(state.player(player).object, player)
}

/// A plain resolution frame anchored on `source` (controlled by player 0) with
/// no targets, bindings, choice, or X — the shape most effect/action tests
/// build to drive `run_effect`/`eval_*`.
pub(crate) fn frame_src(state: &GameState, source: crate::object::ObjectId) -> ExecutionFrame {
    frame_src_targets(state, source, Vec::new())
}

/// Like [`frame_src`] but with explicit `targets` — compatibility scaffolding
/// for hand-built effects that read announced region registers.
pub(crate) fn frame_src_targets(
    state: &GameState,
    source: crate::object::ObjectId,
    targets: Vec<crate::object::ObjectId>,
) -> ExecutionFrame {
    let mut frame = state.frame(source, PlayerId(0));
    let slots: Vec<_> = targets.into_iter().map(|target| vec![target]).collect();
    state.frame_set_targets(&mut frame, &slots);
    let provenances = [
        deckmaste_core::Provenance::Source,
        deckmaste_core::Provenance::Controller,
        deckmaste_core::Provenance::EventObject,
        deckmaste_core::Provenance::EventPatient,
        deckmaste_core::Provenance::EventActor,
        deckmaste_core::Provenance::DefendingPlayer,
    ]
    .into_iter()
    .chain((0..slots.len()).map(|index| {
        deckmaste_core::Provenance::AnnouncedTarget(
            u32::try_from(index).expect("fixture target count fits u32"),
        )
    }))
    .chain(std::iter::once(deckmaste_core::Provenance::AnnouncedX));
    let params: std::sync::Arc<[deckmaste_core::Param]> = provenances
        .enumerate()
        .map(|(index, provenance)| deckmaste_core::Param {
            def: deckmaste_core::DefId(
                u32::try_from(index).expect("fixture parameter count fits u32"),
            ),
            kind: match provenance {
                deckmaste_core::Provenance::AnnouncedTarget(_) => deckmaste_core::Kind::Entities,
                deckmaste_core::Provenance::AnnouncedX => deckmaste_core::Kind::Number,
                _ => deckmaste_core::Kind::Entity,
            },
            provenance,
        })
        .collect::<Vec<_>>()
        .into();
    let region = deckmaste_core::Region::new(params, ());
    frame.activation = state.enter_region(&region, &frame);
    frame
}
