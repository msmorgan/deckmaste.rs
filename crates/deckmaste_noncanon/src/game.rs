//! The per-game driver: steps the engine and routes each pending decision to
//! the owning seat's `Strategy`, accumulating probes.

use std::sync::Arc;

use deckmaste_core::Card;
use deckmaste_core::Int;
use deckmaste_core::Uint;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::sim::Strategy;

use crate::probe::Probes;

pub struct Setup {
    pub decks: [Vec<Arc<Card>>; 2],
    pub seed: u64,
    pub starting_life: Int,
    /// The engine registries (SBA rules, counter/subtype decls) to inject —
    /// see [`crate::source::EngineRules`]. Without the SBA rules creatures
    /// never die, so this is load-bearing, not optional.
    pub rules: crate::source::EngineRules,
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

/// Plays one game to completion. `p0` drives `PlayerId(0)`, `p1` drives
/// `PlayerId(1)`.
///
/// # Panics
///
/// Panics if the game livelocks, runs absurdly long, or a strategy submits a
/// decision the engine rejects — all engine-or-strategy bugs the suite must
/// surface loudly.
#[must_use]
pub fn play_game(setup: Setup, p0: &dyn Strategy, p1: &dyn Strategy) -> GameRecord {
    let [d0, d1] = setup.decks;
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: d0 }, PlayerConfig { deck: d1 }],
        seed: setup.seed,
        starting_life: setup.starting_life,
        starting_player: StartingPlayer::Random,
        sba_rules: setup.rules.sba_rules,
        conferral_rules: setup.rules.conferral_rules,
        damage_result_rules: setup.rules.damage_result_rules,
        counter_decls: setup.rules.counter_decls,
        subtypes: setup.rules.subtypes,
        types: setup.rules.types,
    });
    let proxies = [state.players[0].object, state.players[1].object];
    let strategies: [&dyn Strategy; 2] = [p0, p1];
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
                let d = strategies[who.index()].decide(&state, &pending);
                state
                    .submit_decision(d)
                    .expect("a strategy submits only legal decisions");
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

/// The seat owning a pending decision. (Relocated from the deleted lookahead
/// module; seam-agnostic.)
fn pending_player(pending: &PendingDecision) -> PlayerId {
    pending.decider_player()
}

/// The events of one step's progress (seam-agnostic; relocated from lookahead).
fn events_of(progress: &Progress) -> &[GameEvent] {
    match progress {
        Progress::Applied(Occurrence::Single(e)) => std::slice::from_ref(e),
        Progress::Applied(Occurrence::Batch(es)) => es,
        _ => &[],
    }
}

/// The other seat. Two-player games only: `p` must be seat 0 or 1.
fn opponent(p: PlayerId) -> PlayerId {
    PlayerId(1 - p.0)
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::GameOutcome;
    use deckmaste_engine::PlayerId;

    use super::*;
    use crate::deck;
    use crate::source::CardSource;
    use crate::strategy::MatchupStrategy;
    use crate::wc99;

    /// Two draw-go seats deck out: the loop, mechanical fallbacks, and probes
    /// survive a full game with zero proactive actions.
    #[test]
    #[cfg_attr(not(wizards_corpus), ignore = "needs generated plugins/wizards corpus")]
    fn passbots_deck_out() {
        let src = CardSource::load();
        let p0 = MatchupStrategy::pass_bot(PlayerId(0));
        let p1 = MatchupStrategy::pass_bot(PlayerId(1));
        for seed in 0..3u64 {
            let rec = play_game(
                Setup {
                    decks: [
                        deck::build_full(&wc99::SPED_RED, &src),
                        deck::build_full(&wc99::STOMPY, &src),
                    ],
                    seed,
                    starting_life: 20,
                    rules: src.engine_rules(),
                },
                &p0,
                &p1,
            );
            // Draw is structurally impossible here: each player draws on
            // their own turn, so the two deck-outs land in different SBA
            // sweeps — always a Win.
            assert!(matches!(rec.outcome, GameOutcome::Win(_)), "{rec:?}");
            assert!(rec.decked, "draw-go must end in a deck-out: {rec:?}");
            assert!(rec.loser_lost_for_real);
            assert_eq!(rec.probes.spells_cast, 0);
        }
    }

    /// One complete game over the exact Worlds 1999 Sped Red and Stompy
    /// maindecks.
    #[test]
    #[cfg_attr(not(wizards_corpus), ignore = "needs generated plugins/wizards corpus")]
    fn sped_red_vs_stompy99_completes() {
        let src = CardSource::load();
        let red = deck::build_full(&wc99::SPED_RED, &src);
        let green = deck::build_full(&wc99::STOMPY, &src);
        let rec = play_game(
            Setup {
                decks: [red, green],
                seed: 0,
                starting_life: 20,
                rules: src.engine_rules(),
            },
            &MatchupStrategy::sped_red(PlayerId(0)),
            &MatchupStrategy::stompy(PlayerId(1)),
        );
        assert!(matches!(rec.outcome, GameOutcome::Win(_)), "{rec:?}");
        assert!(rec.loser_lost_for_real, "{rec:?}");
        assert!(rec.turns < 200, "{rec:?}");
        assert!(rec.probes.spells_cast > 0, "{rec:?}");
    }
}
