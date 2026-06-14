//! Drives the engine's step/submit loop, auto-resolving decisions the UI does
//! not yet handle through a `sim::Strategy`.
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionError;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::sim::Strategy;

/// Step budget for a full headless auto-play (and the smoke test): generous
/// enough that a healthy game finishes well within it.
pub(crate) const HEADLESS_BUDGET: usize = 100_000;

/// Why the driver stopped stepping.
#[derive(Debug)]
pub enum Stop {
    /// A human-facing priority decision is pending (interactive mode only).
    #[allow(dead_code)] // payload consumed by the interactive loop in Task 5
    Priority(PendingDecision),
    /// The game ended.
    GameOver(GameOutcome),
    /// The step budget was exhausted (headless mode only).
    Budget,
}

/// Owns the game and the auto-decider used for non-interactive decisions.
pub struct Driver {
    pub state: GameState,
    strategy: Box<dyn Strategy>,
}

impl Driver {
    #[must_use]
    pub fn new(state: GameState, strategy: Box<dyn Strategy>) -> Self { Self { state, strategy } }

    /// Steps the engine, auto-resolving every decision except (when
    /// `auto_priority` is false) priority, which is left for the human.
    ///
    /// # Errors
    /// Propagates a `DecisionError` if an auto-submitted decision is rejected
    /// (a wiring bug — `Strategy` is expected to answer legally).
    fn drive(&mut self, auto_priority: bool, budget: usize) -> Result<Stop, DecisionError> {
        for _ in 0..budget {
            match self.state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::GameOver(outcome) => return Ok(Stop::GameOver(outcome)),
                StepOutcome::NeedsDecision(pending) => {
                    let is_priority = matches!(pending, PendingDecision::Priority { .. });
                    if is_priority && !auto_priority {
                        return Ok(Stop::Priority(pending));
                    }
                    let decision = self.strategy.decide(&self.state, &pending);
                    self.state.submit_decision(decision)?;
                }
            }
        }
        Ok(Stop::Budget)
    }

    /// Interactive: step until the human holds priority or the game ends.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    #[allow(dead_code)] // consumed by the interactive loop in Task 5
    pub fn run_to_priority(&mut self) -> Result<Stop, DecisionError> {
        /// Escape valve: if the engine ever fails to open a priority window,
        /// return `Stop::Budget` rather than hang the UI. Mirrors the engine's
        /// own sim step guard.
        const PRIORITY_BUDGET: usize = 1_000_000;
        self.drive(false, PRIORITY_BUDGET)
    }

    /// Headless: auto-play both seats until game over or the step budget.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn run_to_end(&mut self, budget: usize) -> Result<Stop, DecisionError> {
        self.drive(true, budget)
    }

    /// Pass priority for the current holder.
    ///
    /// # Errors
    /// If the engine rejects the pass (no priority pending).
    #[allow(dead_code)] // consumed by the interactive loop in Task 5
    pub fn pass(&mut self) -> Result<(), DecisionError> {
        self.state.submit_decision(Decision::Act(Action::Pass))
    }

    /// Let the strategy answer the given pending decision.
    ///
    /// # Errors
    /// If the strategy's answer is rejected.
    #[allow(dead_code)] // consumed by the interactive loop in Task 5
    pub fn auto(&mut self, pending: &PendingDecision) -> Result<(), DecisionError> {
        let decision = self.strategy.decide(&self.state, pending);
        self.state.submit_decision(decision)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::game;

    #[test]
    fn auto_play_produces_only_legal_decisions() {
        let state = game::build_game().expect("build demo game");
        let mut driver = Driver::new(state, Box::new(GreedyCreatures));
        // Auto-play both seats. The point is the loop only ever submits legal
        // decisions (no DecisionError); reaching game over is a bonus, so a
        // step budget keeps the test bounded.
        let stop = driver
            .run_to_end(HEADLESS_BUDGET)
            .expect("no decision error");
        assert!(matches!(stop, Stop::GameOver(_) | Stop::Budget));
    }
}
