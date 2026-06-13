//! The layer system ([CR#613]): derived characteristics. Fixtures are real
//! cards from `plugins/canon`, plus `plugins/testing` mocks for the two layer
//! shapes no real card carries (see that plugin's cards/README.md).

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Color;
use deckmaste_core::Zone;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::legal_attackers;

fn plugin(name: &str) -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../plugins/{name}")),
    )
    .unwrap()
}

/// Looks `name` up in canon (real cards) first, then in the testing mocks.
fn card(name: &str) -> Arc<Card> {
    let card = plugin("canon")
        .card(name)
        .or_else(|_| plugin("testing").card(name))
        .unwrap();
    Arc::new(card)
}

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

/// P0's deck contains `names`, padded with Grizzly Bears so the opening
/// draw never empties the library; P1 plays a plain Grizzly Bears deck.
fn game_with_p0_cards(names: &[&str], seed: u64) -> GameState {
    let mut p0: Vec<Arc<Card>> = names.iter().map(|n| card(n)).collect();
    while p0.len() < 10 {
        p0.push(card("Grizzly Bears"));
    }
    let p1 = vec![card("Grizzly Bears"); 10];
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
    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    let view = state.layers();
    assert_eq!(view.power(bear), Some(2), "printed 2/2 derives power 2");
    assert_eq!(
        view.toughness(bear),
        Some(2),
        "printed 2/2 derives toughness 2"
    );
}

/// [CR#613.4c]: a static "+1/+1 to black creatures" (layer 7c) pumps a black
/// 2/2 to 3/3.
#[test]
fn anthem_pumps_power_and_toughness() {
    let mut state = game_with_p0_cards(&["Bad Moon", "Walking Corpse"], 1);
    let zombie = force_onto_battlefield(&mut state, PlayerId(0), "Walking Corpse");
    let _moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon");
    let view = state.layers();
    assert_eq!(view.power(zombie), Some(3), "Bad Moon +1/+1 → 3 power");
    assert_eq!(
        view.toughness(zombie),
        Some(3),
        "Bad Moon +1/+1 → 3 toughness"
    );
}

/// [CR#613.4]: a 7b base set applies before all 7c modification, so Humility's
/// "base 1/1" + Bad Moon's "+1/+1" = 2/2 regardless of timestamps.
#[test]
fn base_set_applies_before_modify() {
    let mut state = game_with_p0_cards(&["Humility", "Bad Moon", "Walking Corpse"], 1);
    let zombie = force_onto_battlefield(&mut state, PlayerId(0), "Walking Corpse");
    let _humility = force_onto_battlefield(&mut state, PlayerId(0), "Humility");
    let _moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon");
    let view = state.layers();
    assert_eq!(
        view.power(zombie),
        Some(2),
        "base 1/1 (7b), then +1/+1 (7c)"
    );
    assert_eq!(view.toughness(zombie), Some(2));
}

/// [CR#611.2c],[CR#514.2]: a one-shot "+3/+3 until end of turn" pumps a
/// creature, then wears off at Cleanup.
#[test]
fn one_shot_pump_expires_at_cleanup() {
    use deckmaste_core::Count;
    use deckmaste_core::Duration;
    use deckmaste_core::Modification;
    use deckmaste_engine::ContinuousEffect;
    use deckmaste_engine::ScopeResolved;
    use deckmaste_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(0),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![
            Modification::AddPower(Count::Literal(3)),
            Modification::AddToughness(Count::Literal(3)),
        ],
        duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
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
/// The granter is a mock — symmetric "all creatures have <native keyword>"
/// statics don't exist in real Magic.
#[test]
fn static_grants_keyword() {
    use deckmaste_core::KeywordAbility;
    use deckmaste_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Trample granter"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let _granter = force_onto_battlefield(&mut state, PlayerId(0), "Trample granter");
    assert!(
        has_keyword(&state.layers(), bear, &KeywordAbility::Trample),
        "the bear gains trample from the static ([CR#613.1f])"
    );
}

/// [CR#611.3a,613.8a]: a self-referential layer-6 grant — "creatures with
/// trample have trample" — applies exactly once. The affected set is whatever
/// has trample when the effect applies, so the grant can't feed itself: no
/// fixpoint loop, trample-less creatures stay out of the set, and a printed
/// trampler ends up with a redundant ([CR#702.19g]) second instance.
#[test]
fn self_referential_grant_applies_once() {
    use deckmaste_core::Ability;
    use deckmaste_core::KeywordAbility;
    use deckmaste_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Trample tautology", "Fangren Hunter"], 1);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let _tautology = force_onto_battlefield(&mut state, PlayerId(0), "Trample tautology");

    let view = state.layers();
    let instances = view
        .get(trampler)
        .abilities
        .iter()
        .filter(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample)))
        .count();
    assert_eq!(
        instances, 2,
        "printed trample + the granted (redundant, [CR#702.19g]) one"
    );
    assert!(
        !has_keyword(&view, bear, &KeywordAbility::Trample),
        "a creature without trample is never in the affected set ([CR#611.3a])"
    );
}

/// [CR#613.1f]: Humility's "creatures lose all abilities" blanks a printed
/// keyword.
#[test]
fn lose_all_abilities_blanks_keyword() {
    use deckmaste_core::KeywordAbility;
    use deckmaste_engine::has_keyword;

    let mut state = game_with_p0_cards(&["Fangren Hunter", "Humility"], 1);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let _humility = force_onto_battlefield(&mut state, PlayerId(0), "Humility");
    assert!(
        !has_keyword(&state.layers(), trampler, &KeywordAbility::Trample),
        "lose-all-abilities removes the printed trample ([CR#613.1f])"
    );
}

/// [CR#613.6]: an effect that REPLACES an enchantment's type with Creature (L4)
/// AND sets its P/T (L7b) applies the P/T to the now-creature — the target set
/// locks at L4 and the L7b part rides along, even though the object no longer
/// matches the effect's `Matching(Enchantment)` filter by L7b. The animator is
/// a mock — no real static type-replaces with a literal P/T set.
#[test]
fn type_change_then_set_pt_locks_in() {
    use deckmaste_core::Type;

    let mut state = game_with_p0_cards(&["Animate enchantments", "Moonlit Wake"], 1);
    let _animator = force_onto_battlefield(&mut state, PlayerId(0), "Animate enchantments");
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Moonlit Wake");

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

/// [CR#613.1e] layer 5: Darkest Hour's "all creatures are black" replaces a
/// creature's derived colors.
#[test]
fn static_sets_color() {
    let mut state = game_with_p0_cards(&["Darkest Hour"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let _hour = force_onto_battlefield(&mut state, PlayerId(0), "Darkest Hour");

    let view = state.layers();
    assert!(
        view.get(bear).colors.contains(&Color::Black),
        "the creature is now black (layer 5)"
    );
    assert!(
        !view.get(bear).colors.contains(&Color::Green),
        "\"are black\" replaces the printed green (layer 5)"
    );
}

/// [CR#613.6] for layer 5: Bad Moon ("+1/+1 to black creatures") catches a
/// creature that is only black because Darkest Hour painted it black THIS pass
/// — `matches_derived` evaluates the scope against the derived color, not the
/// printed face.
#[test]
fn anthem_catches_creature_painted_black() {
    let mut state = game_with_p0_cards(&["Darkest Hour", "Bad Moon"], 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears"); // base green
    let _hour = force_onto_battlefield(&mut state, PlayerId(0), "Darkest Hour"); // creatures are black (L5)
    let _moon = force_onto_battlefield(&mut state, PlayerId(0), "Bad Moon"); // black creatures +1/+1 (L7c)

    let view = state.layers();
    assert!(
        view.get(bear).colors.contains(&Color::Black),
        "painted black (L5)"
    );
    assert_eq!(
        view.power(bear),
        Some(3),
        "Bad Moon caught the now-black creature via derived-color matching ([CR#613.6])"
    );
}

/// [CR#613.4c],[CR#122]: +1/+1 counters modify P/T in layer 7c.
#[test]
fn plus_one_counters_pump() {
    use deckmaste_core::Ident;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
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

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
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

    let mut state = game_with_p0_cards(&["Moonlit Wake"], 1);
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Moonlit Wake");
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

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
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
    let mut state = game_with_p0_cards(&["Animate enchantments", "Moonlit Wake"], 1);
    let _animator = force_onto_battlefield(&mut state, PlayerId(0), "Animate enchantments");
    let ench = force_onto_battlefield(&mut state, PlayerId(0), "Moonlit Wake");
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

/// [CR#613.1b]: a layer-2 control-change effect ("you gain control") makes the
/// derived controller the effect's controller. The object's base controller is
/// unchanged — control change is a continuous effect, not a mutation.
#[test]
fn gain_control_changes_derived_controller() {
    use deckmaste_core::Duration;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_engine::ContinuousEffect;
    use deckmaste_engine::ScopeResolved;
    use deckmaste_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    assert_eq!(
        state.layers().controller(bear),
        PlayerId(0),
        "base controller is P0"
    );

    // P1 "gains control of" the bear (Threaten-style). Per [CR#611.2c] the
    // effect's `You` is locked to its controller (P1) at creation.
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(1),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::SetController(Reference::You)],
        duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        is_cda: false,
    });
    assert_eq!(
        state.layers().controller(bear),
        PlayerId(1),
        "layer 2 derives the new controller ([CR#613.1b])"
    );
}

/// [CR#613.1b],[CR#613.5]: control change is continuous — when the effect
/// expires the derived controller reverts to the base controller.
#[test]
fn gained_control_reverts_when_effect_expires() {
    use deckmaste_core::Duration;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_engine::ContinuousEffect;
    use deckmaste_engine::ScopeResolved;
    use deckmaste_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(1),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::SetController(Reference::You)],
        duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        is_cda: false,
    });
    assert_eq!(state.layers().controller(bear), PlayerId(1), "stolen");

    state.expire_end_of_turn();
    assert_eq!(
        state.layers().controller(bear),
        PlayerId(0),
        "control reverts after the effect expires ([CR#613.5])"
    );
}

/// [CR#508.1a],[CR#613.1b]: combat eligibility reads the DERIVED controller — a
/// stolen creature is a legal attacker for its new controller, not its owner.
#[test]
fn stolen_creature_attacks_for_new_controller() {
    use deckmaste_core::Duration;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_engine::ContinuousEffect;
    use deckmaste_engine::ScopeResolved;
    use deckmaste_engine::Timestamp;

    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&bear),
        "sanity: P0's bear can attack before the steal"
    );

    state.continuous.push(ContinuousEffect {
        timestamp: Timestamp(1_000),
        controller: PlayerId(1),
        scope: ScopeResolved::Locked(vec![bear]),
        changes: vec![Modification::SetController(Reference::You)],
        duration: Duration::EndOfGame,
        is_cda: false,
    });
    assert!(
        !legal_attackers(&state, PlayerId(0)).contains(&bear),
        "P0 no longer controls the stolen creature"
    );
    assert!(
        legal_attackers(&state, PlayerId(1)).contains(&bear),
        "P1 gained control and can attack with it ([CR#613.1b])"
    );
}
