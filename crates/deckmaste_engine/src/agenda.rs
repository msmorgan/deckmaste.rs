use deckmaste_core::StepOrPhase;

use crate::event::GameEvent;

/// One unit of engine work. `step()` pops exactly one; handlers schedule
/// follow-ups at the agenda *front*, ahead of previously queued work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkItem {
    /// The interception seam: cant → replacements → apply. Both registries
    /// are empty in the skeleton, so an Emit applies in the same step.
    Emit(GameEvent),
    /// Turn-structure transition plus that step's schedule.
    BeginStep(StepOrPhase),
    /// CR 704.3: state-based actions, checked before anyone gets priority.
    CheckSbas,
    /// Cleanup's turn-based action (CR 514.1).
    CheckHandSize,
    /// Surface `pending = Priority { .. }`.
    OpenPriority,
}
