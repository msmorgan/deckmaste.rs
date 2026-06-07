//! The walking skeleton against real builtin-plugin data: a full game of
//! basic lands, stepped one event at a time.

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_engine::{GameConfig, GameState, PlayerConfig, PlayerId, StartingPlayer};

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

fn two_player_plains(seed: u64, deck_size: usize) -> GameState {
    let plains = Arc::new(builtin().card("Plains").unwrap());
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&plains, deck_size),
            },
            PlayerConfig {
                deck: deck(&plains, deck_size),
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

#[test]
fn opening_state() {
    let state = two_player_plains(42, 20);
    for p in 0..2 {
        assert_eq!(state.zones.hands[p].len(), 7);
        assert_eq!(state.zones.libraries[p].len(), 13);
        assert!(state.zones.graveyards[p].is_empty());
        assert_eq!(state.players[p].life, 20);
    }
    assert!(state.zones.battlefield.is_empty());
    assert_eq!(state.cards.len(), 40);
    assert_eq!(state.turn.turn_number, 0);
    assert!(state.pending.is_none());
    assert!(state.outcome.is_none());
    assert_eq!(state.agenda.len(), 1);
}

#[test]
fn shuffles_are_seeded() {
    let a = two_player_plains(1, 20);
    let b = two_player_plains(1, 20);
    let c = two_player_plains(2, 20);
    assert_eq!(
        a.zones.libraries, b.zones.libraries,
        "same seed, same order"
    );
    // 13 identical Plains cards can't prove different *order*; prove the rng
    // streams differ by comparing the object-id sequences of the libraries.
    assert!(
        a.zones.libraries != c.zones.libraries,
        "different seed, different order (vanishingly unlikely to collide)"
    );
}
