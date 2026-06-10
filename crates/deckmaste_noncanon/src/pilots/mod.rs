//! The archetype pilots, plus the draw-go baseline.

use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::PendingDecision;

use crate::lookahead::Tactician;
use crate::observe::Observation;
use crate::pilot::Pilot;
use crate::pilot::mechanical;

/// Draw-go: passes every priority, never attacks or blocks. The baseline
/// opponent for deterministic behavior tests, and a deck-out smoke fixture.
pub struct PassBot;

impl Pilot for PassBot {
    fn decide(
        &mut self,
        obs: &Observation,
        _tac: &Tactician<'_>,
        pending: &PendingDecision,
    ) -> Decision {
        match pending {
            PendingDecision::Priority { .. } => Decision::Act(Action::Pass),
            PendingDecision::DeclareAttackers { .. } => Decision::Attackers(vec![]),
            PendingDecision::DeclareBlockers { .. } => Decision::Blocks(vec![]),
            PendingDecision::ChooseTargets { legal, .. } => {
                Decision::Targets(legal.iter().map(|c| c[0]).collect())
            }
            other => mechanical(obs, other),
        }
    }
}
