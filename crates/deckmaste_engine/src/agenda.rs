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
    /// [CR#704.3]: state-based actions, checked before anyone gets priority.
    CheckSbas,
    /// Cleanup's turn-based action ([CR#514.1]).
    CheckHandSize,
    /// Surface `pending = Priority { .. }`.
    OpenPriority,
    /// CR 601.2a–b: move the spell to the stack and open the announce slot.
    BeginCast(crate::object::ObjectId),
    /// CR 601.2c: surface `ChooseTargets` if the in-flight spell targets.
    AnnounceTargets,
    /// CR 601.2f–h: pay the in-flight spell's cost (surface `PayMana` if there
    /// is a choice; auto-pay when forced).
    PayCost,
    /// Resolve the named committed stack object (CR 608). Reads `self.stack`.
    Resolve(crate::object::ObjectId),
    /// Interpret one `Effect` node against a resolution frame (CR 608.2).
    RunEffect {
        effect: Box<deckmaste_core::Effect>,
        frame: crate::stack::Frame,
    },
}
