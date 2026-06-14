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

use crate::shortcuts::PassState;

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
    #[allow(dead_code)]
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

    /// Interactive with shortcuts: run to the next decision, then auto-resolve
    /// single-legal decisions and auto-pass priority for any armed `PassState`
    /// mode whose stop condition has not fired, until a genuine human decision
    /// (or game over / budget). Clears the surfaced decision's decider mode
    /// (clear-on-stop).
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn advance(&mut self, pass: &mut PassState) -> Result<Stop, DecisionError> {
        for _ in 0..Self::DECISION_BUDGET {
            let stop = self.run_to_decision()?;
            let Stop::Decision(pending) = &stop else {
                return Ok(stop);
            };
            // Feature 1: single-legal auto-resolve (never priority).
            if let Some(decision) = crate::shortcuts::auto_answer(pending) {
                self.state.submit_decision(decision)?;
                continue;
            }
            // Feature 2: per-player pass mode on a priority window.
            if let PendingDecision::Priority { player, .. } = pending {
                let player = *player;
                if let (Some(mode), Some(armed)) = (pass.mode(player), pass.armed(player)) {
                    let now = crate::shortcuts::Snapshot::of(&self.state);
                    if crate::shortcuts::keep_passing(mode, armed, &now, player) {
                        self.state.submit_decision(Decision::Act(Action::Pass))?;
                        continue;
                    }
                }
            }
            // Genuine human decision: clear the decider's mode, then surface it.
            let decider = pending.decider_player();
            pass.clear(decider);
            return Ok(stop);
        }
        Ok(Stop::Budget)
    }

    /// Submit a decision, then [`Driver::advance`].
    ///
    /// # Errors
    /// As [`Driver::submit`].
    pub fn submit_and_advance(
        &mut self,
        decision: Decision,
        pass: &mut PassState,
    ) -> Result<Stop, DecisionError> {
        self.state.submit_decision(decision)?;
        self.advance(pass)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::game;
    use crate::shortcuts::PassMode;

    /// First-legal answer to a surfaced non-priority decision, for driving a
    /// game in tests (mirrors the choices in interact.rs's integration
    /// test).
    fn answer(pending: &PendingDecision) -> Decision {
        match pending {
            PendingDecision::ChooseTargets { legal, .. } => {
                Decision::Targets(legal.iter().map(|c| c[0]).collect())
            }
            PendingDecision::DeclareAttackers { .. } => Decision::Attackers(vec![]),
            PendingDecision::DeclareBlockers { .. } => Decision::Blocks(vec![]),
            // Priority and every other interactive kind are handled by the caller.
            _ => Decision::Act(Action::Pass),
        }
    }

    #[test]
    fn advance_with_no_modes_surfaces_an_interactive_decision() {
        let state = game::build_game().expect("build demo game");
        let mut driver = Driver::new(state, Box::new(GreedyCreatures));
        let mut pass = PassState::new();
        match driver.advance(&mut pass).expect("no decision error") {
            Stop::Decision(p) => assert!(
                crate::interact::is_interactive(&p),
                "surfaced a non-interactive decision: {p:?}"
            ),
            other => panic!("expected an interactive decision at the opening, got {other:?}"),
        }
    }

    #[test]
    fn armed_pass_modes_stay_legal_and_terminate() {
        // Arm "Turn" for whoever holds priority on every priority window, and
        // make trivial-but-legal choices for combat/targets. With both seats
        // auto-passing, the game must still TERMINATE (each mode clears at its
        // own player's next precombat main — the mutual-pass guard — so turns
        // advance and the game ends, by decking if nothing else). No submission
        // may be rejected.
        let state = game::build_game().expect("build demo game");
        let mut driver = Driver::new(state, Box::new(GreedyCreatures));
        let mut pass = PassState::new();
        let mut stop = driver.advance(&mut pass).expect("no decision error");
        for _ in 0..10_000 {
            match stop {
                Stop::GameOver(_) | Stop::Budget => return, // terminated → guard works
                Stop::Decision(ref pending) => {
                    let decision = if let PendingDecision::Priority { player, .. } = pending {
                        pass.arm(*player, PassMode::Turn, &driver.state);
                        Decision::Act(Action::Pass)
                    } else {
                        answer(pending)
                    };
                    stop = driver
                        .submit_and_advance(decision, &mut pass)
                        .expect("no decision error");
                }
            }
        }
        panic!("game did not terminate — mutual-pass guard failed");
    }

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
