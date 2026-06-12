//! The matchup seats, as engine `Strategy` policies.
//!
//! Each seat is a data-driven [`StrategyEvaluator`] over an authored RON play
//! policy (`strategies/*.ron`), wrapped only to answer the one decision the
//! engine's strategy fallback still `todo!()`s on and that this matchup
//! surfaces: the legend rule. Everything else — develop, cast, target, float,
//! attack, block, and every forced/mechanical decision — rides the evaluator.

use deckmaste_engine::Decision;
use deckmaste_engine::GameState;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerId;
use deckmaste_engine::StrategyEvaluator;
use deckmaste_engine::sim::Strategy;

/// A seat: a greedy [`StrategyEvaluator`] plus the unavoidable `LegendRule`
/// override.
pub struct MatchupStrategy {
    inner: StrategyEvaluator,
}

impl MatchupStrategy {
    fn from_ron(src: &str, seat: PlayerId) -> Self {
        Self {
            inner: StrategyEvaluator::from_ron(src, seat).expect("matchup strategy RON parses"),
        }
    }

    /// Sped Red (burn): cheapest spell to the face, land-only floats.
    #[must_use]
    pub fn sped_red(seat: PlayerId) -> Self {
        Self::from_ron(include_str!("../strategies/sped_red.ron"), seat)
    }

    /// Mono-G Stompy (creatures): biggest affordable creature, lands-first
    /// floats (Elves stay untapped to attack, tapping only when flooding).
    #[must_use]
    pub fn stompy(seat: PlayerId) -> Self {
        Self::from_ron(include_str!("../strategies/stompy.ron"), seat)
    }

    /// Draw-go baseline: passes everything.
    #[must_use]
    pub fn pass_bot(seat: PlayerId) -> Self {
        Self::from_ron(include_str!("../strategies/pass.ron"), seat)
    }
}

impl Strategy for MatchupStrategy {
    fn decide(&self, state: &GameState, pending: &PendingDecision) -> Decision {
        match pending {
            // [CR#704.5j]: the engine strategy fallback `todo!()`s on the legend
            // rule, which the full-proxied matchup surfaces (duplicate
            // legendaries). Keep the first same-name legendary — identical
            // copies here, so the pick is strategy-neutral.
            PendingDecision::LegendRule { candidates, .. } => Decision::Chosen(vec![candidates[0]]),
            // Aim targeted spells at the opponent's face. The data evaluator
            // CANNOT express this: its `among` frame is candidate-anchored, so
            // `Ref(You)` resolves to the candidate's own controller (excluding
            // every player) and `Ref(Opponent)` is an unimplemented engine seam.
            // Burn therefore has to be aimed here — the one decision pure RON
            // can't reach. Each slot takes the opponent's face when legal, else
            // its first legal candidate.
            PendingDecision::ChooseTargets { player, legal, .. } => {
                let opp = state.players[1 - player.index()].object;
                Decision::Targets(
                    legal
                        .iter()
                        .map(|slot| if slot.contains(&opp) { opp } else { slot[0] })
                        .collect(),
                )
            }
            _ => self.inner.decide(state, pending),
        }
    }
}
