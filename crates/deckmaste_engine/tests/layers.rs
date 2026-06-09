//! The layer system ([CR#613]): derived characteristics. Fixtures are fake
//! cards from `plugins/testing` (no WOTC IP).

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{Card, Zone};
use deckmaste_engine::{GameConfig, GameState, ObjectId, PlayerConfig, PlayerId, StartingPlayer};

fn testing() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

fn two_player_with(card: &str, seed: u64, deck_size: usize) -> GameState {
    let c = Arc::new(testing().card(card).unwrap());
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&c, deck_size),
            },
            PlayerConfig {
                deck: deck(&c, deck_size),
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.name,
    }
}

fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    *state.zones.hands[player.index()]
        .iter()
        .find(|&&o| state.objects.obj(o).card_id().is_some() && face_name(state, o) == name)
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand", player.0))
}

fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = find_in_hand(state, player, name);
    state.zones.hands[player.index()].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// Mints a card directly onto the battlefield — bypasses hand/library
/// (test-only setup for cards not in the shuffled deck).
fn mint_onto_battlefield(state: &mut GameState, player: PlayerId, def: Arc<Card>) -> ObjectId {
    let obj = state.mint_card(def, player, Some(Zone::Battlefield));
    state.zones.battlefield.push(obj);
    obj
}

/// [CR#613.1]: with no continuous effects, derived characteristics equal the
/// printed values — the layer system is behavior-preserving at the base.
#[test]
fn base_values_equal_printed() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    let view = state.layers();
    assert_eq!(view.power(bear), Some(2), "printed 2/2 derives power 2");
    assert_eq!(
        view.toughness(bear),
        Some(2),
        "printed 2/2 derives toughness 2"
    );
}

/// [CR#613.4c]: a static "+1/+1 to creatures" (layer 7c) pumps a 2/2 to 3/3.
#[test]
fn anthem_pumps_power_and_toughness() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let anthem_def = Arc::new(testing().card("Anthem").unwrap());
    let _anthem = mint_onto_battlefield(&mut state, PlayerId(0), anthem_def);
    let view = state.layers();
    assert_eq!(view.power(bear), Some(3), "anthem +1/+1 → 3 power");
    assert_eq!(view.toughness(bear), Some(3), "anthem +1/+1 → 3 toughness");
}

/// [CR#613.4]: a 7b "base 0/1" set applies before all 7c modification, so
/// "base 0/1" + anthem "+1/+1" = 1/2 regardless of timestamps.
#[test]
fn base_set_applies_before_modify() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let setter_def = Arc::new(testing().card("Becomes 0-1 anthem").unwrap());
    let _setter = mint_onto_battlefield(&mut state, PlayerId(0), setter_def);
    let view = state.layers();
    assert_eq!(view.power(bear), Some(1));
    assert_eq!(view.toughness(bear), Some(2));
}
