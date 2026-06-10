//! The per-game driver: steps the engine, routes each pending decision to
//! the owning seat's pilot (built on a fresh honest Observation + Tactician),
//! and accumulates probes.

use std::sync::Arc;

use deckmaste_core::Card;
use deckmaste_core::Int;
use deckmaste_core::Uint;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;

use crate::lookahead::Tactician;
use crate::lookahead::events_of;
use crate::lookahead::pending_player;
use crate::observe::Observation;
use crate::observe::opponent;
use crate::pilot::Pilot;
use crate::probe::Probes;

pub struct Setup {
    pub decks: [Vec<Arc<Card>>; 2],
    pub seed: u64,
    pub starting_life: Int,
}

#[derive(Debug, Clone)]
pub struct GameRecord {
    pub outcome: GameOutcome,
    pub turns: Uint,
    pub life: [Int; 2],
    pub probes: Probes,
    pub loser_lost_for_real: bool,
    pub decked: bool,
}

/// Plays one game to completion. `p0` drives `PlayerId(0)`.
///
/// # Panics
///
/// Panics if the game livelocks, runs absurdly long, or a pilot submits a
/// decision the engine rejects — all engine-or-pilot bugs the suite must
/// surface loudly.
#[must_use]
pub fn play_game(setup: Setup, p0: &mut dyn Pilot, p1: &mut dyn Pilot) -> GameRecord {
    let [d0, d1] = setup.decks;
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: d0 }, PlayerConfig { deck: d1 }],
        seed: setup.seed,
        starting_life: setup.starting_life,
        starting_player: StartingPlayer::Random,
    });
    let proxies = [state.players[0].object, state.players[1].object];
    let mut probes = Probes::default();

    let mut guard = 0u32;
    let outcome = loop {
        guard += 1;
        assert!(guard < 1_000_000, "game did not terminate (livelock?)");
        assert!(
            state.turn.turn_number < 200,
            "game ran absurdly long ({} turns)",
            state.turn.turn_number
        );

        match state.step() {
            StepOutcome::Progress(p) => {
                probes.observe(&state, events_of(&p), proxies);
            }
            StepOutcome::GameOver(o) => break o,
            StepOutcome::NeedsDecision(pending) => {
                let who = pending_player(&pending);
                let obs = Observation::of(&state, who);
                let tac = Tactician::new(&state, who);
                let d = if who.index() == 0 {
                    p0.decide(&obs, &tac, &pending)
                } else {
                    p1.decide(&obs, &tac, &pending)
                };
                state
                    .submit_decision(d)
                    .expect("a pilot submits only legal decisions");
            }
        }
    };

    let (loser_lost_for_real, decked) = match outcome {
        GameOutcome::Win(w) => {
            let l = &state.players[opponent(w).index()];
            (
                l.lost && (l.life <= 0 || l.drew_from_empty),
                l.drew_from_empty,
            )
        }
        GameOutcome::Draw => (false, false),
    };

    GameRecord {
        outcome,
        turns: state.turn.turn_number,
        life: [state.players[0].life, state.players[1].life],
        probes,
        loser_lost_for_real,
        decked,
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::GameOutcome;

    use super::*;
    use crate::deck;
    use crate::pilots::PassBot;
    use crate::source::CardSource;
    use crate::wc99;

    /// Two draw-go pilots deck out: the loop, mechanical decisions, and
    /// probes survive a full game with zero proactive actions.
    #[test]
    fn passbots_deck_out() {
        let src = CardSource::load();
        for seed in 0..3u64 {
            let rec = play_game(
                Setup {
                    decks: [
                        deck::build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, &src),
                        deck::build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, &src),
                    ],
                    seed,
                    starting_life: 20,
                },
                &mut PassBot,
                &mut PassBot,
            );
            assert!(matches!(rec.outcome, GameOutcome::Win(_)), "{rec:?}");
            assert!(rec.decked, "draw-go must end in a deck-out: {rec:?}");
            assert!(rec.loser_lost_for_real);
            assert_eq!(rec.probes.spells_cast, 0);
        }
    }
}
