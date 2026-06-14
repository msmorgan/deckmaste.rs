//! Renderer-facing enumeration queries: a UI can read every legal option at a
//! decision point — cost, ability, source, decider — without re-deriving
//! legality. Harness modeled on `tests/activate.rs`.

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Phase;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Payment;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;

const PINGER: &str = "Creature tap-activated DealDamage AnyTarget";
const INSTANT: &str = "Lightning Bolt";
const BEARS: &str = "Grizzly Bears";

// --- plugin + deck building
// ---------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn testing() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

fn red() -> ColorOrColorless { Color::Red.into() }

/// The face name of a card-backed object.
///
/// # Panics
/// Panics if `id` is a player proxy.
fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.name,
    }
}

/// True iff `id` is a card-backed object whose face name is `name`.
fn is_card(state: &GameState, id: ObjectId, name: &str) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == name)
}

/// True iff `id` is a card-backed object whose printed types include Land.
fn is_land(state: &GameState, id: ObjectId) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| match state.def(id) {
            Card::Normal(f) | Card::ModalDfc(f, _) => f.types.contains(&Type::Land),
        })
}

/// The first object in `player`'s hand whose face name is `name`.
fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    *state.zones.hands[player.index()]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand", player.0))
}

/// Moves the first `name` card from `player`'s hand straight onto the
/// battlefield (no land-drop limit, no turn loop) and returns its id.
fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = find_in_hand(state, player, name);
    state.zones.hands[player.index()].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// Moves the first `name` card from `player`'s library (else hand) straight
/// onto the battlefield and returns its id. Deterministic placement that
/// doesn't depend on whether the card was drawn into the opening hand.
fn force_into_play(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    if let Some(&obj) = state.zones.libraries[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
    {
        state.zones.libraries[i].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(obj);
        return obj;
    }
    force_onto_battlefield(state, player, name)
}

/// Two players; player 0's deck holds five copies of `name` plus Mountains;
/// player 1 holds Grizzly Bears (targets) and Forests. `mountains`
/// Mountains are forced onto player 0's battlefield. The mono-typed halves of
/// each deck guarantee the opening seven holds at least two Mountains for
/// player 0; the named card is pulled by `force_into_play` (library or hand).
fn activation_game(seed: u64, name: &str, mountains: usize) -> GameState {
    let testing = testing();
    let card = Arc::new(testing.card(name).unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let bears = Arc::new(canon().card(BEARS).unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(&card); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    for _ in 0..mountains {
        force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    }
    state
}

// --- stepping helpers
// ---------------------------------------------------------

/// Steps until the next decision or game-over, returning the progress trace and
/// the stop.
fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

/// Steps until a `Priority` decision surfaces for `player` in `phase`, passing
/// any other priority along the way. Returns the legal action list at that
/// window.
///
/// When a `PayMana` decision surfaces mid-announce for an all-colored cost
/// (generic == 0, so `Payment { generic: vec![] }` is the only valid answer),
/// this function auto-answers it and continues. Costs with a generic component
/// must be answered explicitly before calling this helper.
fn run_to_priority(state: &mut GameState, player: PlayerId, phase: Phase) -> Vec<Action> {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::Priority { player: p, legal })
                if p == player && state.turn.current == phase =>
            {
                return legal;
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                state
                    .submit_decision(Decision::Pay(Payment { generic: vec![] }))
                    .unwrap_or_else(|e| panic!("auto-pay failed (cost has a generic component — answer PayMana explicitly before run_to_priority): {e}"));
            }
            other => panic!("unexpected stop before {player:?} priority in {phase:?}: {other:?}"),
        }
    }
}

/// Floats `count` mana by activating the first untapped land's mana ability
/// `count` times (each tap is one land). Filters to LAND sources so a non-mana
/// activated ability on the same battlefield is never picked up by mistake.
fn float_mana(state: &mut GameState, player: PlayerId, count: usize) {
    for _ in 0..count {
        // Re-derive the legal list each iteration: tapping a land removes its
        // ability from the next list.
        let StepOutcome::NeedsDecision(PendingDecision::Priority { legal, .. }) = state.step()
        else {
            panic!("expected a priority decision to float mana");
        };
        let tap = legal
            .iter()
            .find(
                |a| matches!(a, Action::ActivateAbility { object, .. } if is_land(state, *object)),
            )
            .cloned()
            .expect("an untapped land with a mana ability");
        state.submit_decision(Decision::Act(tap)).unwrap();
        let _ = run_to_priority(state, player, state.turn.current);
    }
}

/// Moves the first `name` card from `player`'s library into their hand and
/// returns its id; falls back to an existing copy already in hand. Lets a test
/// guarantee a specific castable/playable card without depending on the
/// shuffle.
fn force_into_hand(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    if let Some(&obj) = state.zones.libraries[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
    {
        state.zones.libraries[i].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Hand);
        state.zones.hands[i].push(obj);
        return obj;
    }
    find_in_hand(state, player, name)
}

// --- tests --------------------------------------------------------------------

#[test]
fn mana_cost_is_publicly_reachable_for_a_castable_spell() {
    // A game whose p0 deck holds Lightning Bolt + Mountains (testing plugin
    // supplies nothing here, so build a bespoke deck like activation_game does).
    let bolt = Arc::new(canon().card(INSTANT).unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let bolt_id = force_into_hand(&mut state, PlayerId(0), INSTANT);

    // mana_cost is now public: a renderer can read a castable spell's cost.
    let cost = state
        .mana_cost(bolt_id)
        .expect("Lightning Bolt has a mana cost");
    // Lightning Bolt costs {R}: exactly one colored symbol, no generic.
    assert_eq!(cost.mana_value(), 1, "Bolt is mana value 1");
}

#[test]
fn abilities_index_matches_activate_ability_action() {
    // PINGER is a creature with a tap-activated non-mana ability; activation_game
    // forces one Mountain onto p0's battlefield and stocks the deck with PINGER.
    let mut state = activation_game(7, PINGER, 1);
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    let _legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // Find the offered ActivateAbility for the pinger and read its ability back
    // by the SAME index — that round-trip is the public indexing contract.
    let StepOutcome::NeedsDecision(PendingDecision::Priority { legal, .. }) = state.step() else {
        panic!("expected priority");
    };
    let idx = legal
        .iter()
        .find_map(|a| match a {
            Action::ActivateAbility { object, ability } if *object == pinger => Some(*ability),
            _ => None,
        })
        .expect("pinger offers an activated ability");

    let abilities = state.abilities(pinger);
    assert!(
        idx < abilities.len(),
        "the Action index is in range of abilities()"
    );
    // The indexed ability resolves as an activated ability.
    assert!(
        state.activated_ability(pinger, idx).is_some(),
        "activated_ability returns the activated ability at the offered index"
    );
}

#[test]
fn mana_ability_identifies_a_mountains_tap_for_red() {
    let mut state = activation_game(7, PINGER, 1);
    // The Mountain forced onto the battlefield by activation_game.
    let mountain = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(&state, o, "Mountain"))
        .expect("a Mountain on the battlefield");

    // Its derived (basic-land-conferred) abilities include a tap-for-{R}.
    let abilities = state.abilities(mountain);
    let mana_idx = (0..abilities.len())
        .find(|&i| state.mana_ability(mountain, i).is_some())
        .expect("Mountain has a derived mana ability");
    assert_eq!(
        state.mana_ability(mountain, mana_idx),
        Some((red(), 1)),
        "Mountain taps for one red"
    );
    // And that same ability is an activated ability.
    assert!(state.activated_ability(mountain, mana_idx).is_some());
}

#[test]
fn decision_point_exposes_the_decider_player() {
    let mut state = activation_game(7, PINGER, 1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. }) = state.step() else {
        panic!("expected priority");
    };

    let dp = state.decision_point().expect("a pending decision");
    assert_eq!(
        dp.decider_player(),
        player,
        "DecisionPoint::decider_player matches the pending decision's player"
    );
}
