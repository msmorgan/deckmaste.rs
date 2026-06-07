//! The auto-stepping wrapper: the old draft engine's ergonomics, recovered.

use crate::decide::{Decision, DecisionError, PendingDecision};
use crate::state::{GameOutcome, GameState};
use crate::step::{Progress, StepOutcome};

/// Where a run stopped: the engine needs input, or the game is over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStop {
    Decision(PendingDecision),
    GameOver(GameOutcome),
}

/// Auto-steps a [`GameState`] to its decision points, collecting the
/// progress trace along the way.
#[derive(Debug)]
pub struct Runner<'a> {
    state: &'a mut GameState,
}

impl<'a> Runner<'a> {
    #[must_use]
    pub fn new(state: &'a mut GameState) -> Self { Self { state } }

    /// Steps until a decision or game over.
    pub fn run(&mut self) -> (Vec<Progress>, RunStop) {
        let mut trace = Vec::new();
        loop {
            match self.state.step() {
                StepOutcome::Progress(p) => trace.push(p),
                StepOutcome::NeedsDecision(d) => return (trace, RunStop::Decision(d)),
                StepOutcome::GameOver(o) => return (trace, RunStop::GameOver(o)),
            }
        }
    }

    /// Submits an answer, then runs to the next stop.
    ///
    /// # Errors
    ///
    /// Propagates [`DecisionError`] from the submission; the decision stays
    /// pending and no stepping happens.
    pub fn submit(&mut self, d: Decision) -> Result<(Vec<Progress>, RunStop), DecisionError> {
        self.state.submit_decision(d)?;
        Ok(self.run())
    }
}
