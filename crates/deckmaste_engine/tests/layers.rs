//! The layer system ([CR#613]): derived characteristics. Fixtures are fake
//! cards from `plugins/testing` (no WOTC IP).

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{Card, Color, Zone};
use deckmaste_engine::{
    GameConfig, GameState, ObjectId, PlayerConfig, PlayerId, StartingPlayer, legal_attackers,
};

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

/// [CR#611.2c],[CR#514.2]: a one-shot "+3/+3 until end of turn" pumps a
/// creature, then wears off at Cleanup.
#[test]
fn one_shot_pump_expires_at_cleanup() {
    use deckmaste_core::{Count, Duration, Modification};
    use deckmaste_engine::{ContinuousEffect, ScopeResolved, Timestamp};

    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![
            Modification::AddPower(Count::Literal(3)),
            Modification::AddToughness(Count::Literal(3)),
        ],
        duration: Duration::UntilEndOfTurn,
        is_cda: false,
    });
    assert_eq!(state.layers().power(bear), Some(5), "2/2 +3/+3 → 5");

    state.expire_end_of_turn();
    assert_eq!(
        state.layers().power(bear),
        Some(2),
        "pump gone after cleanup"
    );
}

/// [CR#613.1f]: a static "creatures gain trample" (layer 6) grants the keyword.
#[test]
fn static_grants_keyword() {
    use deckmaste_core::KeywordAbility;
    use deckmaste_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Trample granter"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let _granter = force_onto_battlefield(&mut state, PlayerId(0), "Trample granter");
    assert!(
        has_keyword(&state, bear, KeywordAbility::Trample),
        "the bear gains trample from the static ([CR#613.1f])"
    );
}

/// [CR#613.1f]: "creatures lose all abilities" blanks a printed keyword.
#[test]
fn lose_all_abilities_blanks_keyword() {
    use deckmaste_core::KeywordAbility;
    use deckmaste_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Trample Creature", "Blanker"], 1);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Trample Creature");
    let _blanker = force_onto_battlefield(&mut state, PlayerId(0), "Blanker");
    assert!(
        !has_keyword(&state, trampler, KeywordAbility::Trample),
        "lose-all-abilities removes the printed trample ([CR#613.1f])"
    );
}

/// [CR#613.6]: an effect that REPLACES an enchantment's type with Creature (L4)
/// AND sets its P/T (L7b) applies the P/T to the now-creature — the target set
/// locks at L4 and the L7b part rides along, even though the object no longer
/// matches the effect's `Matching(Enchantment)` filter by L7b.
#[test]
fn type_change_then_set_pt_locks_in() {
    use deckmaste_core::Type;

    let mut state = game_with_p0_cards(&["Animate enchantments", "Vanilla Enchantment"], 1);
    let _animator = force_onto_battlefield(&mut state, PlayerId(0), "Animate enchantments");
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Enchantment");

    let view = state.layers();
    assert!(
        view.get(ench).card_types.contains(&Type::Creature),
        "the enchantment became a creature (layer 4)"
    );
    assert_eq!(
        view.power(ench),
        Some(4),
        "and its P/T was set to 4/4 (layer 7b, riding the layer-4-locked set, [CR#613.6])"
    );
}

/// [CR#613.1e] layer 5: a static "creatures are red" adds red to a
/// creature's derived colors.
#[test]
fn static_adds_color() {
    let mut state = game_with_p0_cards(&["Paint red"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let _painter = force_onto_battlefield(&mut state, PlayerId(0), "Paint red");

    let view = state.layers();
    assert!(
        view.get(bear).colors.contains(&Color::Red),
        "the creature is now red (layer 5)"
    );
}

/// [CR#613.6] for layer 5: a "+1/+1 to red creatures" anthem catches a creature
/// that is only red because another effect painted it red THIS pass —
/// `matches_derived` evaluates the scope against the derived color, not the
/// printed face.
#[test]
fn anthem_catches_creature_painted_red() {
    let mut state = game_with_p0_cards(&["Paint red", "Red anthem"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature"); // base green
    let _painter = force_onto_battlefield(&mut state, PlayerId(0), "Paint red"); // creatures are red (L5)
    let _anthem = force_onto_battlefield(&mut state, PlayerId(0), "Red anthem"); // red creatures +1/+1 (L7c)

    let view = state.layers();
    assert!(
        view.get(bear).colors.contains(&Color::Red),
        "painted red (L5)"
    );
    assert_eq!(
        view.power(bear),
        Some(3),
        "the red anthem caught the now-red creature via derived-color matching ([CR#613.6])"
    );
}

/// [CR#613.4c],[CR#122]: +1/+1 counters modify P/T in layer 7c.
#[test]
fn plus_one_counters_pump() {
    use deckmaste_core::Ident;

    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("+1/+1"), 2);

    let view = state.layers();
    assert_eq!(view.power(bear), Some(4), "2/2 + two +1/+1 → 4 power");
    assert_eq!(
        view.toughness(bear),
        Some(4),
        "2/2 + two +1/+1 → 4 toughness"
    );
}

/// [CR#613.4c]: -1/-1 counters reduce P/T in layer 7c (and stack with +1/+1
/// as a net delta).
#[test]
fn minus_one_counters_shrink() {
    use deckmaste_core::Ident;

    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("-1/-1"), 1);

    let view = state.layers();
    assert_eq!(view.power(bear), Some(1), "2/2 + one -1/-1 → 1 power");
    assert_eq!(
        view.toughness(bear),
        Some(1),
        "2/2 + one -1/-1 → 1 toughness"
    );
}

/// A +1/+1 counter on a permanent with no P/T (a non-creature) does nothing —
/// the 7c counter delta only applies where P/T is `Some`.
#[test]
fn counter_on_non_creature_does_nothing() {
    use deckmaste_core::Ident;

    let mut state = game_with_p0_cards(&["Vanilla Enchantment"], 1);
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Enchantment");
    state
        .objects
        .obj_mut(ench)
        .counters
        .insert(Ident::from("+1/+1"), 3);

    let view = state.layers();
    assert_eq!(
        view.power(ench),
        None,
        "a counter on a non-creature confers no P/T"
    );
    assert_eq!(view.toughness(ench), None);
}

/// [CR#613.4c]: +1/+1 and -1/-1 counters combine as a net delta in 7c.
#[test]
fn mixed_counters_net_delta() {
    use deckmaste_core::Ident;

    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("+1/+1"), 2);
    state
        .objects
        .obj_mut(bear)
        .counters
        .insert(Ident::from("-1/-1"), 1);

    let view = state.layers();
    assert_eq!(
        view.power(bear),
        Some(3),
        "2/2 + two +1/+1 - one -1/-1 → net +1 → 3"
    );
    assert_eq!(view.toughness(bear), Some(3));
}

/// [CR#508.1a]: a permanent animated into a creature by a layer-4 effect is a
/// legal attacker — combat legality reads the derived view, not the printed
/// type.
#[test]
fn animated_enchantment_can_attack() {
    let mut state = game_with_p0_cards(&["Animate enchantments", "Vanilla Enchantment"], 1);
    let _animator = force_onto_battlefield(&mut state, PlayerId(0), "Animate enchantments");
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Enchantment");
    // force_onto_battlefield leaves it untapped and not summoning-sick (mint
    // defaults).

    assert!(
        state
            .layers()
            .get(ench)
            .card_types
            .contains(&deckmaste_core::Type::Creature),
        "sanity: the enchantment derives as a creature"
    );
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&ench),
        "an animated-into-creature permanent can attack ([CR#508.1a] over the derived type)"
    );
}
