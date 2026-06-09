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

fn card(name: &str) -> Arc<Card> { Arc::new(testing().card(name).unwrap()) }

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.name,
    }
}

fn two_player_with(card_name: &str, seed: u64, deck_size: usize) -> GameState {
    let c = card(card_name);
    let deck = vec![Arc::clone(&c); deck_size];
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: deck.clone() },
            PlayerConfig { deck: deck.clone() },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

/// True iff `o` is a card-backed object whose face name is `name`.
fn is_named(state: &GameState, o: ObjectId, name: &str) -> bool {
    state.objects.obj(o).card_id().is_some() && face_name(state, o) == name
}

/// P0's deck contains `names`, padded with Vanilla Creatures so the opening
/// draw never empties the library; P1 plays a plain Vanilla deck.
fn game_with_p0_cards(names: &[&str], seed: u64) -> GameState {
    let mut p0: Vec<Arc<Card>> = names.iter().map(|n| card(n)).collect();
    while p0.len() < 10 {
        p0.push(card("Vanilla Creature"));
    }
    let p1 = vec![card("Vanilla Creature"); 10];
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

/// Move the first object named `name` in `player`'s hand or library straight
/// onto the battlefield. Public-fields-only; no engine API widening.
fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let p = player.index();
    let obj = if let Some(i) = state.zones.hands[p]
        .iter()
        .position(|&o| is_named(state, o, name))
    {
        state.zones.hands[p].remove(i)
    } else {
        let i = state.zones.libraries[p]
            .iter()
            .position(|&o| is_named(state, o, name))
            .unwrap_or_else(|| panic!("no {name} in P{}'s hand or library", player.0));
        state.zones.libraries[p]
            .remove(i)
            .expect("index in library")
    };
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
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
    let mut state = game_with_p0_cards(&["Anthem"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let _anthem = force_onto_battlefield(&mut state, PlayerId(0), "Anthem");
    let view = state.layers();
    assert_eq!(view.power(bear), Some(3), "anthem +1/+1 → 3 power");
    assert_eq!(view.toughness(bear), Some(3), "anthem +1/+1 → 3 toughness");
}

/// [CR#613.4]: a 7b "base 0/1" set applies before all 7c modification, so
/// "base 0/1" + anthem "+1/+1" = 1/2 regardless of timestamps.
#[test]
fn base_set_applies_before_modify() {
    let mut state = game_with_p0_cards(&["Becomes 0-1 anthem"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let _setter = force_onto_battlefield(&mut state, PlayerId(0), "Becomes 0-1 anthem");
    let view = state.layers();
    assert_eq!(view.power(bear), Some(1));
    assert_eq!(view.toughness(bear), Some(2));
}
