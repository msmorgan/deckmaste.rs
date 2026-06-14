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
    /// An interactive decision is pending (interactive mode only). Carries the
    /// pending decision so the UI can build an `Interaction` for it.
    Decision(PendingDecision),
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

    /// Escape valve: if the engine ever fails to open a decision window, return
    /// `Stop::Budget` rather than hang the UI.
    const DECISION_BUDGET: usize = 1_000_000;

    /// Steps the engine, auto-resolving (via `Strategy`) every decision for
    /// which `stop_pred` is false, and stopping on the first one for which
    /// it is true.
    ///
    /// # Errors
    /// Propagates a `DecisionError` if an auto-submitted decision is rejected
    /// (a wiring bug — `Strategy` is expected to answer legally).
    fn drive(
        &mut self,
        stop_pred: impl Fn(&PendingDecision) -> bool,
        budget: usize,
    ) -> Result<Stop, DecisionError> {
        for _ in 0..budget {
            match self.state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::GameOver(outcome) => return Ok(Stop::GameOver(outcome)),
                StepOutcome::NeedsDecision(pending) => {
                    if stop_pred(&pending) {
                        return Ok(Stop::Decision(pending));
                    }
                    let decision = self.strategy.decide(&self.state, &pending);
                    self.state.submit_decision(decision)?;
                }
            }
        }
        Ok(Stop::Budget)
    }

    /// Interactive: step until a human-driven decision is pending or the game
    /// ends.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn run_to_decision(&mut self) -> Result<Stop, DecisionError> {
        self.drive(crate::interact::is_interactive, Self::DECISION_BUDGET)
    }

    /// Interactive (priority only): used by the existing render/board tests.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn run_to_priority(&mut self) -> Result<Stop, DecisionError> {
        self.drive(
            |p| matches!(p, PendingDecision::Priority { .. }),
            Self::DECISION_BUDGET,
        )
    }

    /// Headless: auto-play both seats until game over or the step budget.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn run_to_end(&mut self, budget: usize) -> Result<Stop, DecisionError> {
        self.drive(|_| false, budget)
    }

    /// Pass priority for the current holder.
    ///
    /// # Errors
    /// If the engine rejects the pass (no priority pending).
    pub fn pass(&mut self) -> Result<(), DecisionError> {
        self.state.submit_decision(Decision::Act(Action::Pass))
    }

    /// Let the strategy answer the given pending decision.
    ///
    /// # Errors
    /// If the strategy's answer is rejected.
    pub fn auto(&mut self, pending: &PendingDecision) -> Result<(), DecisionError> {
        let decision = self.strategy.decide(&self.state, pending);
        self.state.submit_decision(decision)
    }

    /// Submit a decision, then run to the next interactive stop.
    ///
    /// # Errors
    /// Returns the `DecisionError` if the engine rejects `decision` (e.g. an
    /// illegal selection); the caller keeps the current interaction and
    /// re-prompts.
    pub fn submit(&mut self, decision: Decision) -> Result<Stop, DecisionError> {
        self.state.submit_decision(decision)?;
        self.run_to_decision()
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

    #[test]
    fn run_to_decision_stops_on_an_interactive_kind() {
        let state = game::build_game().expect("build demo game");
        let mut driver = Driver::new(state, Box::new(GreedyCreatures));
        let stop = driver.run_to_decision().expect("no decision error");
        match stop {
            Stop::Decision(p) => assert!(
                crate::interact::is_interactive(&p),
                "stopped on a non-interactive decision: {p:?}"
            ),
            other => panic!("expected an interactive decision at the opening, got {other:?}"),
        }
    }
}
