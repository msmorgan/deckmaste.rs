//! Builds the demo `GameState` from committed plugin data + decklist files.
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use deckmaste_cards::Deck;
use deckmaste_cards::plugin::Plugin;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::StartingPlayer;

/// Fixed RNG seed so the demo game is reproducible.
const SEED: u64 = 0xD00D;

fn data(rel: &str) -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join(rel) }

/// Loads canon + builtin + the generated `wizards` corpus and the two demo
/// decklists, and assembles a two-player Goblins-vs-Elves game.
///
/// The demo's cards are produced locally (not shipped): `wizards` is the
/// gitignored generated corpus (`cargo xtask generate plugins/wizards`), so the
/// decklists resolve against canon staples (Lightning Bolt, basics) plus the
/// rest of each card materialized from bulk data.
///
/// # Errors
/// If a plugin or decklist fails to load, or a listed card can't be resolved.
pub fn build_game() -> Result<GameState> {
    let canon = Plugin::load_with_sibling_prelude(data("../../plugins/canon"))?;
    let builtin = Plugin::load(data("../../plugins/builtin"))?;
    let wizards = Plugin::load_with_prelude(&builtin, data("../../plugins/wizards"))?;

    let goblins = Deck::load(&data("../../plugins/demo/decks/goblins.txt"))?;
    let elves = Deck::load(&data("../../plugins/demo/decks/elves.txt"))?;

    let p0 = goblins.resolve(&[&canon, &builtin, &wizards])?;
    let p1 = elves.resolve(&[&canon, &builtin, &wizards])?;

    Ok(GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: SEED,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_two_player_twenty_life_game() {
        let state = build_game().expect("build demo game");
        assert_eq!(state.players.len(), 2);
        assert_eq!(state.players[0].life, 20);
        assert_eq!(state.players[1].life, 20);
    }

    /// End-to-end: both seats are auto-developed by `GreedyDemo` (which, unlike
    /// `GreedyCreatures`, also chooses legal targets for the burn / sac-outlet
    /// pings these decks cast), so the tribal lords, Krenko's token scaling,
    /// sac outlets, and burn all reach the battlefield. A lord-filled board
    /// exercises the continuous-effect carrier (engine-static-scope-carrier)
    /// that used to panic every layer rebuild — so this finishing at all is the
    /// demo's real proof of life.
    #[test]
    fn demo_auto_plays_to_completion() {
        use deckmaste_engine::sim::GreedyDemo;

        use crate::driver::Driver;
        use crate::driver::HEADLESS_BUDGET;
        use crate::driver::Stop;

        let state = build_game().expect("build demo game");
        let mut driver = Driver::new(state, Box::new(GreedyDemo));
        match driver
            .run_to_end(HEADLESS_BUDGET)
            .expect("no decision error")
        {
            Stop::GameOver(_) => {}
            other => panic!("demo did not play to completion: {other:?}"),
        }
    }
}
