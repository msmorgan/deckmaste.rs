//! The walking skeleton against real builtin-plugin data: a full game of
//! basic lands, stepped one event at a time.

use std::collections::VecDeque;
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
    // 13 identical Plains cards can't distinguish permutations by value.
    // ObjectIds are minted in deck-loop order before the shuffle, so every
    // construction assigns the same ids; the VecDeque sequence captures the
    // permutation the shuffle chose.
    assert!(
        a.zones.libraries != c.zones.libraries,
        "different seed, different order (vanishingly unlikely to collide)"
    );
}

use deckmaste_core::StepOrPhase;
use deckmaste_engine::{
    Action, Decision, DecisionError, GameEvent, PendingDecision, Progress, StepOutcome,
};

/// Steps until the next decision or game end, returning the progress trace.
/// (The Runner wraps exactly this; tests that predate it drive manually.)
fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

#[test]
fn turn_one_walks_to_upkeep_priority_one_event_at_a_time() {
    let mut state = two_player_plains(42, 20);

    // Turn structure: BeginStep(Untap) begins turn 1.
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Advanced(StepOrPhase::Untap))
    ));
    assert_eq!(state.turn.turn_number, 1);
    assert_eq!(state.turn.active_player, PlayerId(0));

    // One event per step: TurnBegan, then StepBegan(Untap).
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Applied(GameEvent::TurnBegan {
            player: PlayerId(0),
            turn: 1,
        }))
    ));
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Applied(GameEvent::StepBegan(StepOrPhase::Untap)))
    ));

    // Empty battlefield: no untap events; untap grants no priority (CR 502.4),
    // so the next transition is straight into upkeep.
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Advanced(StepOrPhase::Upkeep))
    ));
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Applied(GameEvent::StepBegan(StepOrPhase::Upkeep)))
    ));

    // The pre-priority barrier: a clean SBA sweep, then priority opens.
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::SbasChecked { actions: 0 })
    ));
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::PriorityOpened(PlayerId(0)))
    ));

    // The decision surfaces on the NEXT call, idempotently, without mutating.
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. }) = state.step() else {
        panic!("expected priority");
    };
    assert_eq!(player, PlayerId(0));
    assert!(matches!(
        state.step(),
        StepOutcome::NeedsDecision(PendingDecision::Priority { .. })
    ));
}

/// Drives to the next decision, answering every priority with Pass.
/// Returns the non-priority stop (other decision kind, or game over).
fn pass_to_stop(state: &mut GameState) -> StepOutcome {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => return other,
        }
    }
}

/// Steps until the predicate matches a just-returned outcome. The predicate
/// also receives the state (taking it as a parameter rather than capturing
/// it — the closure can't borrow `state` while `&mut state` is in use).
fn step_until(
    state: &mut GameState,
    mut pred: impl FnMut(&GameState, &StepOutcome) -> bool,
) -> StepOutcome {
    loop {
        let outcome = state.step();
        if pred(state, &outcome) {
            return outcome;
        }
        if let StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) = outcome {
            state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        } else if matches!(
            outcome,
            StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_)
        ) {
            panic!("unexpected stop: {outcome:?}");
        }
    }
}

#[test]
fn submission_errors() {
    let mut state = two_player_plains(42, 20);
    // Nothing pending yet.
    assert_eq!(
        state.submit_decision(Decision::Act(Action::Pass)),
        Err(DecisionError::NothingPending)
    );
    // Wrong kind at a priority decision.
    let (_, stop) = step_to_stop(&mut state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(PendingDecision::Priority { .. })
    ));
    assert_eq!(
        state.submit_decision(Decision::Discard(vec![])),
        Err(DecisionError::WrongKind)
    );
    // Illegal action: playing a land during upkeep.
    let object = state.zones.hands[0][0];
    assert!(matches!(
        state.submit_decision(Decision::Act(Action::PlayLand { object })),
        Err(DecisionError::Illegal { .. })
    ));
    // The decision is still pending and answerable after errors.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
}

#[test]
fn a_full_pass_around_advances_the_step() {
    let mut state = two_player_plains(42, 20);
    let (_, stop) = step_to_stop(&mut state);
    assert!(matches!(stop, StepOutcome::NeedsDecision(_)));
    assert_eq!(state.turn.current, StepOrPhase::Upkeep);
    // P0 passes; priority rotates to P1 (same step).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. }) = stop else {
        panic!("expected P1 priority");
    };
    assert_eq!(player, PlayerId(1));
    assert_eq!(state.turn.current, StepOrPhase::Upkeep);
    // P1 passes too: all-pass on an empty stack ends the step.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert_eq!(state.turn.current, StepOrPhase::Draw);
}

#[test]
fn land_drop_tap_for_mana_and_pool_emptying() {
    let mut state = two_player_plains(42, 20);
    // Drive to P0's precombat main.
    let stop = step_until(&mut state, |s, o| {
        matches!(o, StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. })
            if *player == PlayerId(0))
            && s.turn.current == StepOrPhase::PrecombatMain
    });
    let StepOutcome::NeedsDecision(PendingDecision::Priority { legal, .. }) = stop else {
        unreachable!()
    };
    // Land drop is legal; take the first.
    let land = legal
        .iter()
        .find_map(|a| match a {
            Action::PlayLand { object } => Some(*object),
            _ => None,
        })
        .expect("a land drop should be legal");
    state
        .submit_decision(Decision::Act(Action::PlayLand { object: land }))
        .unwrap();

    // The land arrives; P0 retains priority (CR 117.3c).
    let (trace, stop) = step_to_stop(&mut state);
    assert!(trace.iter().any(|p| matches!(
        p,
        Progress::Applied(GameEvent::LandPlayed { object }) if *object == land
    )));
    assert_eq!(state.zones.battlefield, vec![land]);
    assert_eq!(state.players[0].lands_played_this_turn, 1);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, legal }) = stop else {
        panic!("expected priority back");
    };
    assert_eq!(player, PlayerId(0));

    // Second land this turn: not offered, and rejected if forced.
    assert!(!legal.iter().any(|a| matches!(a, Action::PlayLand { .. })));
    let another = state.zones.hands[0][0];
    assert!(matches!(
        state.submit_decision(Decision::Act(Action::PlayLand { object: another })),
        Err(DecisionError::Illegal { .. })
    ));

    // Tap it for mana: the conferred CR 305.6 ability, through the data.
    let tap = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .expect("mana ability should be legal")
        .clone();
    state.submit_decision(Decision::Act(tap)).unwrap();
    let (trace, _stop) = step_to_stop(&mut state);
    assert!(trace.iter().any(|p| matches!(
        p,
        Progress::Applied(GameEvent::Tapped(id)) if *id == land
    )));
    use deckmaste_core::Color;
    assert_eq!(state.players[0].mana_pool.amount(Color::White.into()), 1);

    // Pass around: the step ends, the pool empties (CR 500.4).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state); // P1's priority
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);
    assert!(
        trace
            .iter()
            .any(|p| matches!(p, Progress::Applied(GameEvent::ManaEmptied(PlayerId(0)))))
    );
    assert!(state.players[0].mana_pool.is_empty());
}

#[test]
fn cleanup_discards_to_hand_size() {
    let mut state = two_player_plains(42, 20);
    // All-pass: P1 draws on turn 2 (8 cards) and must discard at cleanup.
    let stop = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DiscardToHandSize { player, count }) = stop
    else {
        panic!("expected a cleanup discard, got {stop:?}");
    };
    assert_eq!(player, PlayerId(1));
    assert_eq!(count, 1);
    assert_eq!(state.turn.turn_number, 2);

    // Wrong count rejected; then a legal discard.
    assert!(matches!(
        state.submit_decision(Decision::Discard(vec![])),
        Err(DecisionError::Illegal { .. })
    ));
    let chosen = state.zones.hands[1][0];
    state
        .submit_decision(Decision::Discard(vec![chosen]))
        .unwrap();
    let (trace, _) = step_to_stop(&mut state);
    assert!(trace.iter().any(|p| matches!(
        p,
        Progress::Applied(GameEvent::Discarded { player: PlayerId(1), object }) if *object == chosen
    )));
    assert_eq!(state.zones.hands[1].len(), 7);
    assert_eq!(state.zones.graveyards[1], vec![chosen]);
}

#[test]
fn deck_out_ends_the_game() {
    // Seven-card decks: opening hands take the whole library. P1 draws on
    // turn 2 from nothing → CR 704.5c → P0 wins.
    let mut state = two_player_plains(7, 7);
    let stop = pass_to_stop(&mut state);
    assert_eq!(
        stop,
        StepOutcome::GameOver(deckmaste_engine::GameOutcome::Win(PlayerId(0)))
    );
    assert!(state.players[1].lost);
    // Game over is sticky.
    assert!(matches!(state.step(), StepOutcome::GameOver(_)));
}

use deckmaste_engine::{RunStop, Runner};

#[test]
fn runner_recovers_the_auto_stepping_ergonomics() {
    let mut state = two_player_plains(7, 7);
    let mut runner = Runner::new(&mut state);
    let (_, mut stop) = runner.run();
    loop {
        match stop {
            RunStop::Decision(PendingDecision::Priority { .. }) => {
                (_, stop) = runner.submit(Decision::Act(Action::Pass)).unwrap();
            }
            RunStop::Decision(other) => panic!("unexpected decision: {other:?}"),
            RunStop::GameOver(outcome) => {
                assert_eq!(outcome, deckmaste_engine::GameOutcome::Win(PlayerId(0)));
                break;
            }
        }
    }
}

/// The step-grain showcase: two tapped lands untap one event at a time, and
/// the state between the two events is assertable.
#[test]
fn state_is_assertable_between_two_untap_events() {
    let mut state = two_player_plains(42, 20);

    // Each player's script at priority: play a land if allowed, tap every
    // untapped land, then pass.
    let script = |legal: &[Action]| -> Action {
        legal
            .iter()
            .find(|a| matches!(a, Action::PlayLand { .. }))
            .or_else(|| {
                legal
                    .iter()
                    .find(|a| matches!(a, Action::ActivateAbility { .. }))
            })
            .unwrap_or(&Action::Pass)
            .clone()
    };

    // Drive turns 1–4 with the script; collect P0's lands.
    let mut p0_lands = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(GameEvent::LandPlayed { object }))
                if state.objects.obj(object).controller == PlayerId(0) =>
            {
                p0_lands.push(object);
            }
            StepOutcome::Progress(Progress::Advanced(StepOrPhase::Untap))
                if state.turn.turn_number == 5 =>
            {
                break; // turn 5 has begun; its untap events are next.
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { legal, .. }) => {
                let action = script(&legal);
                state.submit_decision(Decision::Act(action)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
    assert_eq!(p0_lands.len(), 2, "turns 1 and 3 each played a land");
    assert!(p0_lands.iter().all(|&l| state.objects.obj(l).tapped));

    // Step into the untap events: after the FIRST, exactly one of the two
    // is untapped — the in-between state the old engine could never show.
    let first = step_until(&mut state, |_, o| {
        matches!(
            o,
            StepOutcome::Progress(Progress::Applied(GameEvent::Untapped(_)))
        )
    });
    let StepOutcome::Progress(Progress::Applied(GameEvent::Untapped(a))) = first else {
        unreachable!()
    };
    let b = *p0_lands.iter().find(|&&l| l != a).expect("the other land");
    assert!(!state.objects.obj(a).tapped, "first land untapped");
    assert!(
        state.objects.obj(b).tapped,
        "second land still tapped in between"
    );

    // One more step: the second untap.
    let second = state.step();
    assert!(matches!(
        second,
        StepOutcome::Progress(Progress::Applied(GameEvent::Untapped(id))) if id == b
    ));
    assert!(!state.objects.obj(b).tapped);
}

/// Replay: the same config and the same decisions reach the same state.
#[test]
fn replay_is_deterministic() {
    let fingerprint = |state: &GameState| {
        (
            state.players.iter().map(|p| p.life).collect::<Vec<_>>(),
            state.zones.hands.iter().map(Vec::len).collect::<Vec<_>>(),
            state
                .zones
                .libraries
                .iter()
                .map(VecDeque::len)
                .collect::<Vec<_>>(),
            state.zones.battlefield.clone(),
            // The script taps lands and floats mana: pin those too.
            state
                .zones
                .battlefield
                .iter()
                .map(|&id| state.objects.obj(id).tapped)
                .collect::<Vec<_>>(),
            state
                .players
                .iter()
                .map(|p| p.mana_pool.clone())
                .collect::<Vec<_>>(),
            state.turn.turn_number,
            state.turn.current,
        )
    };
    let play = || {
        let mut state = two_player_plains(123, 20);
        for _ in 0..40 {
            match state.step() {
                StepOutcome::NeedsDecision(PendingDecision::Priority { legal, .. }) => {
                    let action = legal
                        .iter()
                        .find(|a| !matches!(a, Action::Pass))
                        .unwrap_or(&Action::Pass)
                        .clone();
                    state.submit_decision(Decision::Act(action)).unwrap();
                }
                StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
                StepOutcome::Progress(_) => {}
            }
        }
        state
    };
    assert_eq!(fingerprint(&play()), fingerprint(&play()));
}
