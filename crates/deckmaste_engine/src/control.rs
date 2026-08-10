use crate::decide::PendingDecision;
use crate::stack::PendingStackEntry;
use crate::state::ChoiceContinuation;
use crate::state::GameImage;
use crate::state::ReplaceState;
use crate::trigger::PendingTrigger;

/// Rules-control slots hidden while a nested payment frame runs. A child image
/// uses the ordinary engine state machine without overwriting its parent's
/// announcement, prompt, continuation, trigger placement, or replacement
/// loop.
#[derive(Debug, Clone)]
pub struct ControlSnapshot {
    pub announcing: Option<PendingStackEntry>,
    pub pending: Option<PendingDecision>,
    pub choice: Option<ChoiceContinuation>,
    pub placing_trigger: Option<PendingTrigger>,
    pub replace_state: Option<ReplaceState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ControlError {
    #[error("cannot resume suspended control while an active control slot is occupied")]
    ActiveSlotsOccupied,
    #[error("there is no suspended control snapshot to resume")]
    NoSuspendedControl,
}

/// One full runnable image. Payment metadata joins this shell in the next
/// implementation slice; keeping the image nested here establishes the final
/// ownership direction now.
#[derive(Debug, Clone)]
pub(crate) struct ImageFrame {
    pub working: GameImage,
}

/// The out-of-image controller. Checkpoints never sit inside `GameImage`, so
/// cloning an image cannot recursively clone the transaction that owns it.
#[derive(Debug, Clone, Default)]
pub(crate) struct PaymentController {
    pub frames: Vec<ImageFrame>,
}

impl GameImage {
    pub fn suspend_control(&mut self) {
        self.control_stack.push(ControlSnapshot {
            announcing: self.announcing.take(),
            pending: self.pending.take(),
            choice: self.choice.take(),
            placing_trigger: self.placing_trigger.take(),
            replace_state: self.replace_state.take(),
        });
    }

    pub fn resume_control(&mut self) -> Result<(), ControlError> {
        if self.announcing.is_some()
            || self.pending.is_some()
            || self.choice.is_some()
            || self.placing_trigger.is_some()
            || self.replace_state.is_some()
        {
            return Err(ControlError::ActiveSlotsOccupied);
        }
        let snapshot = self
            .control_stack
            .pop()
            .ok_or(ControlError::NoSuspendedControl)?;
        self.announcing = snapshot.announcing;
        self.pending = snapshot.pending;
        self.choice = snapshot.choice;
        self.placing_trigger = snapshot.placing_trigger;
        self.replace_state = snapshot.replace_state;
        Ok(())
    }
}
