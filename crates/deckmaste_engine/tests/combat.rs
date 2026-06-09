//! Combat legality against fake testing-plugin data: summoning sickness
//! ([CR#302.6]) and the attacker/blocker legality helpers
//! ([CR#508.1a], [CR#509.1a]).
//!
//! The legality helpers are pure functions over a `GameState`, so most tests
//! build a game, force a creature onto the battlefield, toggle the per-object
//! flags directly (the `GameState` fields are all `pub`), and assert the
//! computed lists. The turn-start clear is driven through the public `step`
//! API.

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{BeginningStep, Card, Phase, Zone};
use deckmaste_engine::{
    Action, Decision, GameConfig, GameState, ObjectId, PendingDecision, PlayerConfig, PlayerId,
    Progress, StartingPlayer, StepOutcome, legal_attackers, legal_blockers,
};

// --- plugin + deck building
// ---------------------------------------------------

fn testing() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

/// A two-player game: both players play decks of `card`.
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

// --- forcing onto the battlefield (copied from tests/stack.rs)
// -----------------

/// The face name of a card-backed object.
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

/// The first object in `player`'s hand whose face name is `name`.
fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    *state.zones.hands[player.index()]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand", player.0))
}

/// Moves the first `name` card from `player`'s hand straight onto the
/// battlefield (no land-drop limit, no turn loop) and returns its id. The
/// public `GameState` fields make this direct setup possible without widening
/// the engine API.
fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = find_in_hand(state, player, name);
    state.zones.hands[player.index()].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

// --- tests --------------------------------------------------------------------

/// [CR#508.1a], [CR#302.6]: a creature is a legal attacker only when untapped
/// and not summoning-sick.
#[test]
fn legal_attackers_gates_on_sickness_and_tapped() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    // Summoning-sick: not a legal attacker.
    state.objects.obj_mut(bear).summoning_sick = true;
    state.objects.obj_mut(bear).tapped = false;
    assert!(
        !legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a summoning-sick creature can't attack ([CR#302.6])"
    );

    // Not sick, untapped: a legal attacker.
    state.objects.obj_mut(bear).summoning_sick = false;
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a non-sick untapped creature is a legal attacker ([CR#508.1a])"
    );

    // Tapped: not a legal attacker.
    state.objects.obj_mut(bear).tapped = true;
    assert!(
        !legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a tapped creature can't be declared as an attacker ([CR#508.1a])"
    );
}

/// [CR#509.1a]: a summoning-sick creature can still block.
#[test]
fn legal_blockers_ignores_summoning_sickness() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    state.objects.obj_mut(bear).summoning_sick = true;
    state.objects.obj_mut(bear).tapped = false;
    assert!(
        legal_blockers(&state, PlayerId(0)).contains(&bear),
        "a summoning-sick creature is still a legal blocker ([CR#509.1a])"
    );

    // But a tapped creature can't block.
    state.objects.obj_mut(bear).tapped = true;
    assert!(
        !legal_blockers(&state, PlayerId(0)).contains(&bear),
        "a tapped creature can't be declared as a blocker ([CR#509.1a])"
    );
}

/// [CR#302.6]: summoning sickness is cleared at the controller's turn start, and
/// **only** for the active player. Put a sick creature on each battlefield and
/// stop at turn 1's untap (P0 active): P0's sheds the flag, P1's keeps it. This
/// isolates the clear — if `begin_turn` cleared nobody, P0's would stay set; if
/// it cleared everybody, P1's would be reset; only the correct behavior passes.
#[test]
fn turn_start_clears_summoning_sickness_for_the_active_player_only() {
    let mut state = two_player_with("Vanilla Creature", 42, 20);
    let p0_bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let p1_bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
    state.objects.obj_mut(p0_bear).summoning_sick = true;
    state.objects.obj_mut(p1_bear).summoning_sick = true;

    assert_eq!(state.turn.turn_number, 0, "no turn has begun yet");

    // Step until turn 1's untap step has begun — `begin_turn` (and its clear)
    // runs inside `begin_step(Untap)` before this `Advanced` is returned, so the
    // post-clear state is what we observe. Turn 1's untap is the very first step,
    // before any draw/discard, so no discard decision arises.
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Advanced(Phase::Beginning(BeginningStep::Untap)))
                if state.turn.turn_number == 1 =>
            {
                break;
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }

    assert_eq!(state.turn.turn_number, 1);
    assert_eq!(state.turn.active_player, PlayerId(0));
    assert!(
        !state.objects.obj(p0_bear).summoning_sick,
        "P0's creature shed summoning sickness at P0's turn start ([CR#302.6])"
    );
    assert!(
        state.objects.obj(p1_bear).summoning_sick,
        "P1's creature is NOT cleared on P0's turn — the clear is active-player-only ([CR#302.6])"
    );
    // P0's de-sickened creature is now a legal attacker (untapped, not sick).
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&p0_bear),
        "the de-sickened creature is a legal attacker"
    );
}
