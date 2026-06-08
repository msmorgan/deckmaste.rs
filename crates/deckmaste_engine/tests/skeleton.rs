//! The walking skeleton against real builtin-plugin data: a full game of
//! basic lands, stepped one event at a time.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{Card, Color, Filter, StepOrPhase, Type, Zone};
use deckmaste_engine::{
    Action, Decision, DecisionError, GameConfig, GameEvent, GameState, ObjectId, ObjectSource,
    Occurrence, PendingDecision, PlayerConfig, PlayerId, Progress, RunStop, Runner, StackEntry,
    StackObject, StartingPlayer, StepOutcome,
};

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn testing() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
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
        StepOutcome::Progress(Progress::Applied(Occurrence::Single(
            GameEvent::TurnBegan {
                player: PlayerId(0),
                turn: 1,
            }
        )))
    ));
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::StepBegan(
            StepOrPhase::Untap
        ))))
    ));

    // Empty battlefield: no untap events; untap grants no priority ([CR#502.4]),
    // so the next transition is straight into upkeep.
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Advanced(StepOrPhase::Upkeep))
    ));
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::StepBegan(
            StepOrPhase::Upkeep
        ))))
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

    // The land arrives; P0 retains priority ([CR#117.3c]).
    let (trace, stop) = step_to_stop(&mut state);
    assert!(trace.iter().any(|p| matches!(
        applied(p),
        Some(GameEvent::LandPlayed { object }) if *object == land
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

    // Tap it for mana: the conferred [CR#305.6] ability, through the data.
    let tap = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .expect("mana ability should be legal")
        .clone();
    state.submit_decision(Decision::Act(tap)).unwrap();
    let (trace, _stop) = step_to_stop(&mut state);
    assert!(trace.iter().any(|p| matches!(
        applied(p),
        Some(GameEvent::Tapped(id)) if *id == land
    )));
    assert_eq!(state.players[0].mana_pool.amount(Color::White.into()), 1);

    // Pass around: the step ends, the pool empties ([CR#500.4]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state); // P1's priority
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);
    assert!(
        trace
            .iter()
            .any(|p| matches!(applied(p), Some(GameEvent::ManaEmptied(PlayerId(0)))))
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
        applied(p),
        Some(GameEvent::Discarded { player: PlayerId(1), object }) if *object == chosen
    )));
    assert_eq!(state.zones.hands[1].len(), 7);
    assert_eq!(state.zones.graveyards[1], vec![chosen]);
}

#[test]
fn deck_out_ends_the_game() {
    // Seven-card decks: opening hands take the whole library. P1 draws on
    // turn 2 from nothing → [CR#704.5c] → P0 wins.
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
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::LandPlayed { object },
            ))) if state.objects.obj(object).controller == PlayerId(0) => {
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
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::Untapped(
                _
            ))))
        )
    });
    let StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::Untapped(a)))) =
        first
    else {
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
        StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::Untapped(id)))) if id == b
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

/// [CR#103.8a] (two-player): the starting player skips their first draw step.
/// Pinned directly — `cleanup_discards_to_hand_size` only implies it.
#[test]
fn starting_player_skips_the_first_draw() {
    let mut state = two_player_plains(42, 20);
    step_until(&mut state, |s, _| {
        s.turn.turn_number == 1 && s.turn.current == StepOrPhase::PrecombatMain
    });
    // Past turn 1's draw step: the opening seven, no draw.
    assert_eq!(state.zones.hands[0].len(), 7);
    assert_eq!(state.zones.libraries[0].len(), 13);
}

/// A two-player game; player 0's deck is Vanilla Creature, player 1's is
/// Forest. Returns the state plus a creature object forced onto the
/// battlefield.
fn bear_on_field() -> (GameState, ObjectId) {
    let bears = Arc::new(testing().card("Vanilla Creature").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![Arc::clone(&bears); 10],
            },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    // Force a Vanilla Creature from player 0's hand onto the battlefield.
    let bear = *state.zones.hands[0]
        .iter()
        .find(|&&o| {
            deckmaste_engine::matches(
                &state,
                o,
                &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(Type::Creature)),
            )
        })
        .expect("a Vanilla Creature in the opening hand (10-card mono deck)");
    state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
    state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(bear);
    (state, bear)
}

#[test]
fn cleanup_clears_marked_damage_on_battlefield_creatures() {
    let (mut state, bear) = bear_on_field();
    // Mark some damage on the creature.
    state.objects.obj_mut(bear).damage = 5;
    assert_eq!(state.objects.obj(bear).damage, 5);

    // Drive all-pass until the Cleanup step begins.  The cleanup step fires
    // `clear_marked_damage` as a turn-based action ([CR#514.2]) and THEN
    // checks hand size, so by the time we see `Advanced(Cleanup)` the damage
    // has already been cleared.
    step_until(&mut state, |s, o| {
        matches!(
            o,
            StepOutcome::Progress(Progress::Advanced(StepOrPhase::Cleanup))
        ) && s.turn.current == StepOrPhase::Cleanup
    });

    assert_eq!(
        state.objects.obj(bear).damage,
        0,
        "[CR#514.2]: marked damage removed at start of Cleanup"
    );
}

/// Drives a single Emit and returns the applied event.
fn apply_one(state: &mut GameState, event: GameEvent) -> GameEvent {
    state
        .agenda
        .push_front(deckmaste_engine::WorkItem::Emit(Occurrence::single(event)));
    match state.step() {
        StepOutcome::Progress(Progress::Applied(Occurrence::Single(e))) => e,
        other => panic!("expected Applied(Single(_)), got {other:?}"),
    }
}

/// Extracts the `GameEvent` from a `Progress::Applied(Occurrence::Single(_))`,
/// returning `None` for any other variant. Reduces assertion churn in tests
/// that only see single-event emits.
fn applied(p: &Progress) -> Option<&GameEvent> {
    match p {
        Progress::Applied(Occurrence::Single(e)) => Some(e),
        _ => None,
    }
}

#[test]
fn damage_to_a_player_is_life_loss_and_to_a_creature_is_marked() {
    let (mut state, bear) = bear_on_field();
    let victim = state.players[1].object;
    apply_one(
        &mut state,
        GameEvent::DamageDealt {
            source: bear,
            target: victim,
            amount: 3,
        },
    );
    assert_eq!(state.players[1].life, 17);
    apply_one(
        &mut state,
        GameEvent::DamageDealt {
            source: victim,
            target: bear,
            amount: 2,
        },
    );
    assert_eq!(state.objects.obj(bear).damage, 2);
}

#[test]
fn spell_resolved_moves_the_stack_object_to_its_owners_graveyard() {
    let (mut state, _bear) = bear_on_field();
    // Put a (fake) spell object on the stack owned by player 0.
    let spell = state.zones.hands[0][0];
    state.zones.hands[PlayerId(0).index()].retain(|&o| o != spell);
    state.objects.obj_mut(spell).zone = Some(Zone::Stack);
    state.stack.push(StackEntry {
        object: StackObject::Spell(spell),
        controller: PlayerId(0),
        targets: vec![],
    });
    apply_one(&mut state, GameEvent::SpellResolved(spell));
    assert!(state.stack.is_empty());
    assert!(state.zones.graveyards[0].contains(&spell));
    assert_eq!(state.objects.obj(spell).zone, Some(Zone::Graveyard));
}

#[test]
fn destroyed_moves_a_damaged_creature_to_its_owners_graveyard_clearing_damage() {
    let (mut state, bear) = bear_on_field();
    state.objects.obj_mut(bear).damage = 5;
    apply_one(&mut state, GameEvent::Destroyed(bear));
    assert!(!state.zones.battlefield.contains(&bear));
    let owner = state.owner_of(bear);
    assert!(state.zones.graveyards[owner.index()].contains(&bear));
    assert_eq!(
        state.objects.obj(bear).zone,
        Some(deckmaste_core::Zone::Graveyard)
    );
    assert_eq!(state.objects.obj(bear).damage, 0);
}

#[test]
fn each_player_has_a_proxy_object() {
    let state = two_player_plains(7, 20);
    for p in 0..2 {
        let proxy = state.players[p].object;
        let obj = state.objects.obj(proxy);
        assert_eq!(
            obj.source,
            ObjectSource::Player(PlayerId(u32::try_from(p).unwrap()))
        );
        assert_eq!(
            obj.controller,
            PlayerId(u32::try_from(p).unwrap()),
            "a player controls itself"
        );
        assert_eq!(obj.zone, None, "a player proxy is in no zone");
        assert_eq!(obj.damage, 0);
    }
}

/// A two-player game with Instant `DealDamage` `AnyTarget` in player 0's deck
/// and Vanilla Creature in player 1's deck; also forces a creature onto the
/// battlefield from player 1's hand. Returns `(state, bear)`.
fn decks_bolt_vs_bears_with_bear_on_field() -> (GameState, ObjectId) {
    let testing = testing();
    let bolt = Arc::new(testing.card("Instant DealDamage AnyTarget").unwrap());
    let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![Arc::clone(&bolt); 10],
            },
            PlayerConfig {
                deck: vec![Arc::clone(&bears); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    // Force a Vanilla Creature from player 1's hand onto the battlefield.
    let bear = *state.zones.hands[1]
        .iter()
        .find(|&&o| state_is_bears(&state, o))
        .expect("a Vanilla Creature in player 1's opening hand (10-card mono deck)");
    state.zones.hands[PlayerId(1).index()].retain(|&o| o != bear);
    state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(bear);
    (state, bear)
}

/// Returns the face name of the card-backed object `id`.
///
/// # Panics
/// Panics if `id` is a player proxy.
fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.name,
    }
}

/// True iff `id`'s card face name is "Instant `DealDamage` `AnyTarget`".
fn state_is_bolt(state: &GameState, id: ObjectId) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == "Instant DealDamage AnyTarget")
}

/// True iff `id`'s card face name is "Vanilla Creature".
fn state_is_bears(state: &GameState, id: ObjectId) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == "Vanilla Creature")
}

/// Steps up to `n` times, collecting `Progress` values. Stops early on a
/// decision or game-over.
fn drain_progress(state: &mut GameState, n: usize) -> Vec<Progress> {
    let mut out = Vec::new();
    for _ in 0..n {
        match state.step() {
            StepOutcome::Progress(p) => out.push(p),
            StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
        }
    }
    out
}

/// Builds a game with Instant `DealDamage` `AnyTarget` in player 0's deck and
/// a Vanilla Creature on the field (player 1's), puts the instant onto the
/// stack targeting the creature. Returns `(state, bolt, bear)`.
fn bolt_on_stack_targeting_bear() -> (GameState, ObjectId, ObjectId) {
    let (mut state, bear) = decks_bolt_vs_bears_with_bear_on_field();
    let bolt = *state.zones.hands[0]
        .iter()
        .find(|&&o| state_is_bolt(&state, o))
        .expect("an Instant DealDamage AnyTarget in player 0's opening hand");
    state.zones.hands[PlayerId(0).index()].retain(|&o| o != bolt);
    state.objects.obj_mut(bolt).zone = Some(Zone::Stack);
    state.stack.push(StackEntry {
        object: StackObject::Spell(bolt),
        controller: PlayerId(0),
        targets: vec![bear],
    });
    (state, bolt, bear)
}

#[test]
fn resolving_bolt_deals_three_then_leaves_for_graveyard() {
    let (mut state, bolt, bear) = bolt_on_stack_targeting_bear();
    state
        .agenda
        .push_front(deckmaste_engine::WorkItem::Resolve(bolt));
    // Resolve → RunEffect(DealDamage) → Emit(DamageDealt) → Emit(SpellResolved).
    let trace = drain_progress(&mut state, 8);
    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::DamageDealt { target, amount: 3, .. }) if *target == bear
        )),
        "expected DamageDealt{{target: bear, amount: 3}}, trace: {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|p| matches!(applied(p), Some(GameEvent::SpellResolved(o)) if *o == bolt)),
        "expected SpellResolved(bolt), trace: {trace:?}"
    );
    assert_eq!(state.objects.obj(bear).damage, 3);
    assert!(
        state.zones.graveyards[0].contains(&bolt),
        "bolt should be in player 0's graveyard"
    );
}

#[test]
fn all_pass_on_a_nonempty_stack_resolves_the_top() {
    let (mut state, bolt, bear) = bolt_on_stack_targeting_bear();
    // Open a fresh priority round at the active player and pass twice.
    state
        .agenda
        .push_front(deckmaste_engine::WorkItem::OpenPriority);
    let _ = step_to_stop(&mut state); // surfaces Priority(P0)
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state); // surfaces Priority(P1)
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);
    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::DamageDealt { target, amount: 3, .. }) if *target == bear
        )),
        "expected DamageDealt{{target: bear, amount: 3}}, trace: {trace:?}"
    );
    assert!(
        state.zones.graveyards[0].contains(&bolt),
        "bolt should be in player 0's graveyard after resolution"
    );
}

#[test]
fn casting_a_spell_schedules_the_announce_block_and_begin_cast_stages_it() {
    use deckmaste_engine::WorkItem;

    let (mut state, _bear) = decks_bolt_vs_bears_with_bear_on_field();
    // Float player 0 the {R} cost so `can_cast` admits it.
    let red: deckmaste_core::ColorOrColorless = Color::Red.into();
    state.player_mut(PlayerId(0)).mana_pool.add(red, 1);
    let bolt = *state.zones.hands[0]
        .iter()
        .find(|&&o| state_is_bolt(&state, o))
        .expect("an Instant DealDamage AnyTarget in player 0's opening hand");

    // Open a priority round at the active player and surface the decision.
    // `OpenPriority` runs on the first step (returning Progress and setting
    // `pending`); the decision surfaces on the next step (Stage 1 invariant).
    state
        .agenda
        .push_front(deckmaste_engine::WorkItem::OpenPriority);
    assert!(matches!(
        state.step(),
        StepOutcome::Progress(Progress::PriorityOpened(PlayerId(0)))
    ));
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, legal }) = state.step()
    else {
        panic!("expected a Priority decision");
    };
    assert_eq!(player, PlayerId(0));
    // The real `can_cast` gate offers the instant (instant timing, payable, a
    // legal target exists — the Vanilla Creature on the field).
    assert!(
        legal.contains(&Action::CastSpell { object: bolt }),
        "Bolt should be a legal CastSpell, legal: {legal:?}"
    );

    // Submitting CastSpell reifies the [CR#601.2] announce block at the front.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let front: Vec<WorkItem> = state.agenda.iter().take(6).cloned().collect();
    assert_eq!(
        front,
        vec![
            WorkItem::BeginCast(bolt),
            WorkItem::AnnounceTargets,
            WorkItem::PayCost,
            WorkItem::Emit(Occurrence::single(GameEvent::SpellCast(bolt))),
            WorkItem::CheckSbas,
            WorkItem::OpenPriority,
        ],
        "CastSpell must front-schedule the announce block in order"
    );

    // Stepping BeginCast stages the spell: out of hand, zone Stack, announcing.
    assert_eq!(
        state.step(),
        StepOutcome::Progress(Progress::Announcing(bolt))
    );
    assert!(
        !state.zones.hands[0].contains(&bolt),
        "BeginCast removes the spell from hand"
    );
    assert_eq!(state.objects.obj(bolt).zone, Some(Zone::Stack));
    let pending = state.announcing.as_ref().expect("an announce in flight");
    assert_eq!(pending.object, StackObject::Spell(bolt));
    assert_eq!(pending.controller, PlayerId(0));
    assert_eq!(pending.origin, Zone::Hand);
    assert!(pending.targets.is_empty(), "targets not yet announced");
    // The spell is staged, not yet on the stack (SpellCast hasn't run).
    assert!(state.stack.is_empty());
}
