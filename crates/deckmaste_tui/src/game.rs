//! Builds the demo `GameState` from committed plugin data + decklist files.
#![allow(dead_code)]
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

/// Loads canon + builtin plugins and the two demo decklists, and assembles a
/// two-player Goblins-vs-Elves game.
///
/// # Errors
/// If a plugin or decklist fails to load, or a listed card can't be resolved.
pub fn build_game() -> Result<GameState> {
    let canon = Plugin::load_with_sibling_prelude(data("../../plugins/canon"))?;
    let builtin = Plugin::load(data("../../plugins/builtin"))?;

    let goblins = Deck::load(&data("../../plugins/demo/decks/goblins.txt"))?;
    let elves = Deck::load(&data("../../plugins/demo/decks/elves.txt"))?;

    let p0 = goblins.resolve(&[&canon, &builtin])?;
    let p1 = elves.resolve(&[&canon, &builtin])?;

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
}
