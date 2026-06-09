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
use deckmaste_core::{BeginningStep, Card, CombatStep, Phase, Zone};
use deckmaste_engine::{
    Action, Decision, GameConfig, GameEvent, GameState, ObjectId, Occurrence, PendingDecision,
    PlayerConfig, PlayerId, Progress, StartingPlayer, StepOutcome, legal_attackers, legal_blockers,
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

/// Steps until the next decision or game end, returning the progress trace.
fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

/// Drives to the next non-priority decision (or game over), answering every
/// priority with Pass. Returns the trace accumulated and the stop.
fn pass_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        let (chunk, stop) = step_to_stop(state);
        trace.extend(chunk);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => return (trace, other),
        }
    }
}

/// [CR#508.1a], [CR#508.1f]: the Declare Attackers step surfaces a
/// `DeclareAttackers` decision for the active player; declaring an attacker
/// taps it, records it in `CombatState`, and fires an `Attacking` event.
#[test]
fn declare_attackers_taps_records_and_fires_attacking() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // A creature on P0's battlefield BEFORE the game runs: turn 1's begin clears
    // its summoning sickness, so it is a legal attacker this combat.
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    // Drive (passing priorities) to the Declare Attackers step's decision.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { player, legal }) = stop
    else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert_eq!(
        state.turn.current,
        Phase::Combat(CombatStep::DeclareAttackers)
    );
    assert!(
        legal.contains(&bear),
        "the de-sickened creature is a surfaced legal attacker"
    );

    // Declaring an attacker that isn't in `legal` is rejected.
    assert!(
        state
            .submit_decision(Decision::Attackers(vec![ObjectId(99999)]))
            .is_err(),
        "an id outside the legal set is rejected"
    );
    // A duplicate is rejected.
    assert!(
        state
            .submit_decision(Decision::Attackers(vec![bear, bear]))
            .is_err(),
        "duplicate attackers are rejected"
    );

    // Declare the bear.
    state
        .submit_decision(Decision::Attackers(vec![bear]))
        .unwrap();

    // The Attacking event fires; the bear ends up attacking and tapped.
    let (trace, _stop) = step_to_stop(&mut state);
    assert!(
        trace.iter().any(|p| matches!(
            p,
            Progress::Applied(Occurrence::Batch(events))
                if events.contains(&GameEvent::Attacking(bear))
        )),
        "an Attacking(bear) event appears in the step trace: {trace:?}"
    );
    assert!(
        state.combat.is_attacking(bear),
        "the bear is recorded as an attacker ([CR#508.1a])"
    );
    assert!(
        state.objects.obj(bear).tapped,
        "declaring the bear as an attacker taps it ([CR#508.1f])"
    );
}

/// With no legal attacker, the Declare Attackers step still surfaces the
/// decision; the active player declares no attackers with an empty vec.
#[test]
fn declare_attackers_with_no_legal_attacker_accepts_empty() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // No creature on the battlefield: an empty legal set.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { player, legal }) = stop
    else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert!(legal.is_empty(), "no legal attackers");
    // A nonempty vec is rejected; the empty vec is accepted.
    assert!(
        state
            .submit_decision(Decision::Attackers(vec![ObjectId(99999)]))
            .is_err()
    );
    state.submit_decision(Decision::Attackers(vec![])).unwrap();
    assert!(state.combat.attackers().is_empty());
}

/// Drives to the Declare Attackers decision, declares exactly `attackers`, and
/// then drives (passing priorities) to the Declare Blockers decision. Returns
/// the defending player and the surfaced legal-blocker set.
fn drive_to_declare_blockers(
    state: &mut GameState,
    attackers: Vec<ObjectId>,
) -> (PlayerId, Vec<ObjectId>) {
    let (_trace, stop) = pass_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(attackers))
        .unwrap();
    let (_trace, stop) = pass_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { player, legal }) = stop
    else {
        panic!("expected a DeclareBlockers decision, got {stop:?}");
    };
    (player, legal)
}

/// [CR#509.1a], [CR#509.1h]: after attackers are declared, the Declare Blockers
/// step surfaces a `DeclareBlockers` decision for the **defending**
/// (non-active) player; declaring blocks records them in `CombatState`, marks
/// the attacker blocked, and fires a `Blocked` event per pair. Two blockers may
/// gang one attacker (no ordering decision).
#[test]
fn declare_blockers_records_blocks_and_fires_blocked() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // P0 (active) gets one attacker; P1 (defender) gets two blockers. P0's
    // creature sheds summoning sickness at turn 1's start, so it can attack;
    // P1's are legal blockers regardless of sickness (untapped creatures).
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let (defender, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert_eq!(
        defender,
        PlayerId(1),
        "the defender is the non-active player"
    );
    assert_eq!(
        state.turn.current,
        Phase::Combat(CombatStep::DeclareBlockers)
    );
    assert!(
        legal.contains(&b1) && legal.contains(&b2),
        "both of P1's untapped creatures are surfaced legal blockers: {legal:?}"
    );

    // A blocker outside the legal set is rejected.
    assert!(
        state
            .submit_decision(Decision::Blocks(vec![(ObjectId(99999), attacker)]))
            .is_err(),
        "a blocker outside the legal set is rejected"
    );
    // Blocking a non-attacker is rejected.
    assert!(
        state
            .submit_decision(Decision::Blocks(vec![(b1, ObjectId(88888))]))
            .is_err(),
        "blocking a creature that isn't attacking is rejected"
    );
    // The same blocker blocking twice is rejected ([CR#509.1a]).
    assert!(
        state
            .submit_decision(Decision::Blocks(vec![(b1, attacker), (b1, attacker)]))
            .is_err(),
        "a blocker may block only one attacker ([CR#509.1a])"
    );

    // Gang-block: both b1 and b2 block the single attacker.
    state
        .submit_decision(Decision::Blocks(vec![(b1, attacker), (b2, attacker)]))
        .unwrap();

    let (trace, _stop) = step_to_stop(&mut state);
    let blocked_events: Vec<_> = trace
        .iter()
        .filter_map(|p| match p {
            Progress::Applied(Occurrence::Batch(events)) => Some(events),
            _ => None,
        })
        .flatten()
        .filter(|e| matches!(e, GameEvent::Blocked { .. }))
        .collect();
    assert_eq!(
        blocked_events.len(),
        2,
        "two Blocked events fire (one per pair): {trace:?}"
    );
    assert!(blocked_events.contains(&&GameEvent::Blocked {
        blocker: b1,
        attacker
    }));
    assert!(blocked_events.contains(&&GameEvent::Blocked {
        blocker: b2,
        attacker
    }));

    let blockers = state.combat.blockers_of(attacker);
    assert!(
        blockers.contains(&b1) && blockers.contains(&b2),
        "both blockers are recorded against the attacker: {blockers:?}"
    );
    assert!(
        state.combat.is_blocked(attacker),
        "the attacker is a blocked creature ([CR#509.1h])"
    );
}

/// A single blocker still makes the attacker a blocked creature ([CR#509.1h]).
#[test]
fn declare_blockers_single_blocker_blocks_attacker() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let (defender, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert_eq!(defender, PlayerId(1));
    assert!(legal.contains(&b1));

    state
        .submit_decision(Decision::Blocks(vec![(b1, attacker)]))
        .unwrap();
    let (_trace, _stop) = step_to_stop(&mut state);

    assert_eq!(state.combat.blockers_of(attacker), &[b1]);
    assert!(
        state.combat.is_blocked(attacker),
        "one blocker is enough to make the attacker blocked ([CR#509.1h])"
    );
    assert_eq!(state.combat.attacker_of(b1), Some(attacker));
}

/// [CR#508.8]: with no attackers declared, the Declare Blockers step is skipped
/// — no `DeclareBlockers` decision surfaces and play proceeds.
#[test]
fn declare_blockers_skipped_when_no_attackers() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // Drive to Declare Attackers and declare none.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    state.submit_decision(Decision::Attackers(vec![])).unwrap();

    // The next decision is NOT a DeclareBlockers — the step was skipped.
    let (_trace, stop) = pass_to_stop(&mut state);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. })
        ),
        "no blockers step when nothing attacked ([CR#508.8]): {stop:?}"
    );
}
