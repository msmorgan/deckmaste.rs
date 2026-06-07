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
use deckmaste_engine::{GameEvent, PendingDecision, Progress, StepOutcome};

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
