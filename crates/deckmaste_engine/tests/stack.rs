//! End-to-end stack/casting/targeting against real builtin-plugin data,
//! driven entirely through the public API (`step` / `submit_decision`).
//!
//! Each test builds a two-player game from corpus cards, forces the relevant
//! permanents into play (the public `GameState` fields are all `pub`), advances
//! to a priority window via `step`, then casts a spell the way a UI would:
//! float mana with the Stage-1 mana ability, `CastSpell`, answer
//! `ChooseTargets` / `PayMana` as they surface, and `Pass` to resolve.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{Card, Color, ColorOrColorless, StepOrPhase, Zone};
use deckmaste_engine::{
    Action, Decision, DecisionError, GameConfig, GameEvent, GameState, ObjectId, Payment,
    PendingDecision, PlayerConfig, PlayerId, Progress, StackObject, StartingPlayer, StepOutcome,
};

// --- plugin + deck building
// ---------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn red() -> ColorOrColorless { Color::Red.into() }
fn green() -> ColorOrColorless { Color::Green.into() }

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

/// A two-player game: player 0 holds Lightning Bolts and Mountains, player 1
/// holds Grizzly Bears and Forests. `mountains` Mountains are forced onto
/// player 0's battlefield. Callers force a Bears onto player 1's battlefield
/// (as a Bolt target) when they need one.
fn bolt_game(seed: u64, mountains: usize) -> GameState {
    let plugin = builtin();
    let bolt = Arc::new(plugin.card("Lightning Bolt").unwrap());
    let mountain = Arc::new(plugin.card("Mountain").unwrap());
    let bears = Arc::new(plugin.card("Grizzly Bears").unwrap());
    let forest = Arc::new(plugin.card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    // Mono-typed halves of each deck guarantee the opening seven holds both a
    // Bolt and a Mountain for player 0 and a Bears for player 1.
    for _ in 0..mountains {
        force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    }
    state
}

/// A two-player game where the *casting* player (player 0) holds Grizzly Bears
/// and Forests. `forests` Forests are forced onto player 0's battlefield.
fn bears_game(seed: u64, forests: usize) -> GameState {
    let plugin = builtin();
    let bears = Arc::new(plugin.card("Grizzly Bears").unwrap());
    let forest = Arc::new(plugin.card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(&bears); 5];
    p0.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    for _ in 0..forests {
        force_onto_battlefield(&mut state, PlayerId(0), "Forest");
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
fn run_to_priority(state: &mut GameState, player: PlayerId, phase: StepOrPhase) -> Vec<Action> {
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
            other => panic!("unexpected stop before {player:?} priority in {phase:?}: {other:?}"),
        }
    }
}

/// Floats `count` mana by activating the first untapped land's mana ability
/// `count` times (each tap is one land). Player 0's forced lands each produce
/// one mana of their land's color.
fn float_mana(state: &mut GameState, player: PlayerId, count: usize) {
    for _ in 0..count {
        // Re-derive the legal list each iteration: tapping a land removes its
        // ability from the next list. The decision is idempotent, so stepping
        // re-surfaces it without mutating.
        let StepOutcome::NeedsDecision(PendingDecision::Priority { legal, .. }) = state.step()
        else {
            panic!("expected a priority decision to float mana");
        };
        let tap = legal
            .iter()
            .find(|a| matches!(a, Action::ActivateAbility { .. }))
            .cloned()
            .expect("an untapped land with a mana ability");
        state.submit_decision(Decision::Act(tap)).unwrap();
        // Apply the tap/mana events and return to the same priority window.
        let _ = run_to_priority(state, player, state.turn.current);
    }
}

/// Reads the printed power/toughness of a card-backed object as a pair, or
/// `None` if either is unprinted/variable.
fn printed_pt(state: &GameState, id: ObjectId) -> Option<(i64, i64)> {
    use deckmaste_core::StatValue;
    let face = match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => f,
    };
    let num = |s: &Option<StatValue>| match s {
        Some(StatValue::Number(n)) => Some(i64::from(*n)),
        _ => None,
    };
    Some((num(&face.power)?, num(&face.toughness)?))
}

// --- tests --------------------------------------------------------------------

#[test]
fn bolt_kills_grizzly_bears() {
    let mut state = bolt_game(1, 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    // P0's precombat main: a Bolt and an untapped Mountain in play.
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");

    // Cast Bolt; answer the target choice with the Bears.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the Bears is a legal Bolt target");
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();

    // Step to the caster's priority: the Bolt is on the stack (announce done,
    // not yet resolved).
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the Bolt sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(bolt));
    assert_eq!(state.stack[0].targets, vec![bear]);
    assert!(!state.zones.battlefield.contains(&bolt));

    // Both players pass: the Bolt resolves, deals 3, SBA destroys the Bears.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), StepOrPhase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);

    assert!(
        trace.iter().any(|p| matches!(p,
            Progress::Applied(GameEvent::DamageDealt { target, amount: 3, .. }) if *target == bear)),
        "3 damage dealt to the Bears, trace: {trace:?}"
    );
    assert!(
        state.zones.graveyards[1].contains(&bear),
        "Bears destroyed to P1's graveyard (CR 704.5g)"
    );
    assert!(
        state.zones.graveyards[0].contains(&bolt),
        "Bolt to P0's graveyard after resolution (CR 608.2m)"
    );
    assert!(state.stack.is_empty());
}

#[test]
fn bolt_to_the_face_costs_three_life() {
    let mut state = bolt_game(1, 1);

    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    let face = state.players[1].object;

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(
        legal[0].contains(&face),
        "P1's proxy is a legal Bolt target"
    );
    state
        .submit_decision(Decision::Targets(vec![face]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), StepOrPhase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(state.players[1].life, 17, "20 - 3");
    assert!(!state.players[1].lost);
}

#[test]
fn grizzly_bears_resolves_to_a_two_two_on_the_battlefield() {
    let mut state = bears_game(1, 2); // two Forests for {1}{G}

    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // G, G
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

    // Sorcery-speed cast, no targets, no payment choice (forced from G,G).
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the Bears spell is on the stack");

    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), StepOrPhase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert!(
        state.zones.battlefield.contains(&bears),
        "the Bears entered the battlefield (CR 608.3)"
    );
    assert_eq!(state.objects.obj(bears).controller, PlayerId(0));
    assert_eq!(state.objects.obj(bears).zone, Some(Zone::Battlefield));
    assert_eq!(printed_pt(&state, bears), Some((2, 2)), "a printed 2/2");
    assert!(state.stack.is_empty());
}

#[test]
fn sorcery_speed_gate_blocks_bears_off_turn_and_on_a_nonempty_stack() {
    // The gate compares Bears (sorcery speed) against Bolt (instant) in two
    // off-window spots. In both, P0 holds Bolts + a Bears + the mana to pay for
    // either, so only the *timing* differs — proving the sorcery-speed gate,
    // not a payment or target gap.

    // (a) On the OPPONENT's turn, in their main phase: P0 has priority. Float
    //     R,G,G through the real mana abilities (so `legal` recomputes); the
    //     Bears is timing-blocked while the Bolt (instant) is allowed.
    {
        let plugin = builtin();
        let bolt = Arc::new(plugin.card("Lightning Bolt").unwrap());
        let bears = Arc::new(plugin.card("Grizzly Bears").unwrap());
        let mountain = Arc::new(plugin.card("Mountain").unwrap());
        let forest = Arc::new(plugin.card("Forest").unwrap());
        // 4 Bolt + 4 Bears + 1 Mountain + 1 Forest, fattened with Forests so a
        // Mountain and two Forests are always somewhere in the library and a
        // seed exists with both spell types in the opening hand.
        let build = |seed: u64| {
            let mut deck = vec![Arc::clone(&bolt); 4];
            deck.extend(vec![Arc::clone(&bears); 4]);
            deck.push(Arc::clone(&mountain));
            deck.extend(vec![Arc::clone(&forest); 3]);
            GameState::new(GameConfig {
                players: vec![
                    PlayerConfig { deck },
                    PlayerConfig {
                        deck: vec![Arc::clone(&forest); 12],
                    },
                ],
                seed,
                starting_life: 20,
                starting_player: StartingPlayer::Fixed(PlayerId(0)),
            })
        };
        // Pick a seed whose P0 opening hand holds both a Bolt and a Bears.
        let mut state = (0u64..1000)
            .map(build)
            .find(|s| {
                let hand = &s.zones.hands[0];
                hand.iter().any(|&o| is_card(s, o, "Lightning Bolt"))
                    && hand.iter().any(|&o| is_card(s, o, "Grizzly Bears"))
            })
            .expect("a seed with both a Bolt and a Bears in P0's opening hand");
        // P0's mana sources on the battlefield, pulled from the library so they
        // never depend on the opening hand: one Mountain + two Forests.
        force_into_play(&mut state, PlayerId(0), "Mountain");
        force_into_play(&mut state, PlayerId(0), "Forest");
        force_into_play(&mut state, PlayerId(0), "Forest");
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

        // Drive to P0 holding priority during P1's main, tapping all of P0's
        // lands on the way so the pool can pay either spell.
        let legal = drive_to_off_turn_priority(&mut state);
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Bears is not castable on the opponent's turn, legal: {legal:?}"
        );
        assert!(
            legal.contains(&Action::CastSpell { object: bolt }),
            "Bolt (instant) is castable on the opponent's turn, legal: {legal:?}"
        );
    }

    // (b) During the active player's OWN main phase but with a non-empty stack
    //     (a Bolt already announced): Bears blocked, a second Bolt allowed.
    {
        let mut state = bears_with_bolts();
        let bolt0 = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
        let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");

        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        // Float R,R,G,G: the first {R} Bolt leaves R,G,G for the gate comparison
        // (a second {R} Bolt and a {1}{G} Bears are both payable).
        float_mana(&mut state, PlayerId(0), 4);
        // Cast the first Bolt onto the stack, targeting the Bears.
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt0 }))
            .unwrap();
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { .. }) = stop else {
            panic!("expected ChooseTargets, got {stop:?}");
        };
        state
            .submit_decision(Decision::Targets(vec![bear]))
            .unwrap();
        let legal = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "a Bolt is on the stack");
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Bears blocked while the stack is non-empty, legal: {legal:?}"
        );
        // A second Bolt (instant) is still castable on the non-empty stack.
        let bolt1 = state.zones.hands[0]
            .iter()
            .copied()
            .find(|&o| is_card(&state, o, "Lightning Bolt"))
            .expect("a second Bolt still in hand");
        assert!(
            legal.contains(&Action::CastSpell { object: bolt1 }),
            "a second Bolt is castable at instant speed on a non-empty stack, legal: {legal:?}"
        );
    }
}

/// Drives to player 0 holding priority on player 1's turn (a main phase), taps
/// every untapped land player 0 controls (floating mana through the real mana
/// abilities so the recomputed `legal` reflects the pool), and returns that
/// final legal list. Passes any player-1 priority and answers cleanup discards
/// along the way.
fn drive_to_off_turn_priority(state: &mut GameState) -> Vec<Action> {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::Priority { player, legal })
                if player == PlayerId(0)
                    && state.turn.active_player == PlayerId(1)
                    && matches!(
                        state.turn.current,
                        StepOrPhase::PrecombatMain | StepOrPhase::PostcombatMain
                    ) =>
            {
                // Tap an untapped land if one remains; each tap re-opens P0's
                // priority with a freshly-computed legal list.
                if let Some(tap) = legal
                    .iter()
                    .find(|a| matches!(a, Action::ActivateAbility { .. }))
                    .cloned()
                {
                    state.submit_decision(Decision::Act(tap)).unwrap();
                } else {
                    return legal;
                }
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::DiscardToHandSize { player, count }) => {
                let hand = state.zones.hands[player.index()].clone();
                let chosen: Vec<ObjectId> = hand.into_iter().take(count as usize).collect();
                state.submit_decision(Decision::Discard(chosen)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }
}

/// A game where player 0 holds Bolts + a Bears + Forests (a seed with both a
/// Bolt and a Bears in the opening hand), three Forests forced onto player 0's
/// battlefield, and player 1 holds a Bears (for use as a Bolt target).
fn bears_with_bolts() -> GameState {
    let plugin = builtin();
    let bolt = Arc::new(plugin.card("Lightning Bolt").unwrap());
    let bears = Arc::new(plugin.card("Grizzly Bears").unwrap());
    let mountain = Arc::new(plugin.card("Mountain").unwrap());
    let forest = Arc::new(plugin.card("Forest").unwrap());
    let build = |seed: u64| {
        let mut deck = vec![Arc::clone(&bolt); 4];
        deck.extend(vec![Arc::clone(&bears); 2]);
        deck.extend(vec![Arc::clone(&mountain); 3]);
        deck.extend(vec![Arc::clone(&forest); 3]);
        let mut p1 = vec![Arc::clone(&bears); 2];
        p1.extend(vec![Arc::clone(&forest); 10]);
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck }, PlayerConfig { deck: p1 }],
            seed,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            let hand = &s.zones.hands[0];
            let bolts = hand
                .iter()
                .filter(|&&o| is_card(s, o, "Lightning Bolt"))
                .count();
            bolts >= 2 && hand.iter().any(|&o| is_card(s, o, "Grizzly Bears"))
        })
        .expect("a seed with two Bolts and a Bears in P0's opening hand");
    // Two Mountains + two Forests → R,R,G,G when all tapped: a {R} Bolt leaves
    // R,G,G, enough for both a second {R} Bolt and a {1}{G} Bears.
    for _ in 0..2 {
        force_into_play(&mut state, PlayerId(0), "Mountain");
        force_into_play(&mut state, PlayerId(0), "Forest");
    }
    state
}

#[test]
fn paymana_surfaces_with_a_choice_and_auto_pays_when_forced() {
    // (a) Forced: {1}{G} from a G,G pool auto-pays — no PayMana surfaces.
    {
        let mut state = bears_game(1, 2);
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
            .unwrap();
        // The cast proceeds straight to the caster's priority — no PayMana.
        let (_, stop) = step_to_stop(&mut state);
        assert!(
            matches!(
                stop,
                StepOutcome::NeedsDecision(PendingDecision::Priority { .. })
            ),
            "G,G auto-pays {{1}}{{G}}; no PayMana should surface, got {stop:?}"
        );
        assert_eq!(state.stack.len(), 1, "the Bears reached the stack");
        assert!(state.player(PlayerId(0)).mana_pool.is_empty(), "pool spent");
    }

    // (b) Choice: {1}{G} from a G,G,R pool surfaces PayMana ({1} <- G or R).
    {
        let mut state = bears_game(2, 2);
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G from forests
        // Add a stray Red so {1} has a real choice.
        state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
            .unwrap();
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
            panic!("G,G,R should surface a PayMana decision, got {stop:?}");
        };
        let _ = cost;
        // Pay {1} with the Red (either Red or one Green is legal here).
        state
            .submit_decision(Decision::Pay(Payment {
                generic: vec![red()],
            }))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "the Bears reached the stack");
        // {G} took one Green, {1} took the Red → one Green remains.
        let pool = &state.player(PlayerId(0)).mana_pool;
        assert_eq!(pool.amount(green()), 1, "one Green left");
        assert_eq!(pool.amount(red()), 0, "the Red paid {{1}}");
    }
}

#[test]
fn second_bolt_fizzles_when_its_target_is_already_dead() {
    let mut state = bolt_game(1, 2); // two Mountains for two {R} casts
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R

    // Cast Bolt A targeting the Bears.
    let bolt_a = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_a }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);

    // In response (LIFO), cast Bolt B also targeting the Bears.
    let bolt_b = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    assert_ne!(bolt_a, bolt_b);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_b }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "both Bolts on the stack");

    // Pass both Bolts to resolution: B resolves (kills Bears), then A fizzles.
    let mut damage_events = 0;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(GameEvent::DamageDealt { amount, .. })) => {
                assert_eq!(amount, 3);
                damage_events += 1;
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(_) => break,
        }
        if state.stack.is_empty() && state.zones.graveyards[1].contains(&bear) {
            // Drain any remaining priority passes for the empty stack, then stop.
            break;
        }
    }

    assert_eq!(
        damage_events, 1,
        "only the top Bolt dealt damage; the second fizzled (CR 608.2b)"
    );
    assert!(
        state.zones.graveyards[1].contains(&bear),
        "the Bears died to the first Bolt"
    );
    assert!(
        state.zones.graveyards[0].contains(&bolt_a) && state.zones.graveyards[0].contains(&bolt_b),
        "both Bolts end in P0's graveyard"
    );
    assert!(state.stack.is_empty());
}

#[test]
fn a_cast_game_is_deterministic() {
    // A reusable script: drive to P0's main, float {R}, cast Bolt at the Bears,
    // pass both, resolve. Run it twice and compare a fingerprint.
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
            state.zones.graveyards.clone(),
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
        let mut state = bolt_game(99, 1);
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1);
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
            .unwrap();
        let (_, _) = step_to_stop(&mut state);
        state
            .submit_decision(Decision::Targets(vec![bear]))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), StepOrPhase::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = step_to_stop(&mut state);
        state
    };
    let a = play();
    let b = play();
    assert_eq!(
        fingerprint(&a),
        fingerprint(&b),
        "same config + decisions → same state"
    );
}

#[test]
fn illegal_target_and_payment_submissions_are_rejected_and_retryable() {
    // --- illegal target at ChooseTargets ---
    let mut state = bolt_game(1, 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };

    // (i) An object not in the legal set — the Bolt itself (on the stack, not a
    //     creature/player) is illegal.
    assert!(
        matches!(
            state.submit_decision(Decision::Targets(vec![bolt])),
            Err(DecisionError::Illegal { .. })
        ),
        "an out-of-set target is rejected"
    );
    // (ii) Wrong count — two targets for a single-target spell.
    let other = state.players[0].object;
    assert!(matches!(
        state.submit_decision(Decision::Targets(vec![bear, other])),
        Err(DecisionError::Illegal { .. })
    ));
    // State untouched: the decision still pends and no targets were recorded.
    assert!(matches!(
        state.pending,
        Some(PendingDecision::ChooseTargets { .. })
    ));
    assert!(
        state
            .announcing
            .as_ref()
            .expect("announce still in flight")
            .targets
            .is_empty(),
        "no targets recorded after rejected submissions"
    );
    // A valid retry is accepted.
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    assert!(state.pending.is_none());

    // --- illegal payment at PayMana ---
    let mut state = bears_game(2, 2);
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // G, G
    state.player_mut(PlayerId(0)).mana_pool.add(red(), 1); // G,G,R → a choice
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana, got {stop:?}");
    };
    let pool_before = state.player(PlayerId(0)).mana_pool.clone();

    // (i) Over-spend: {1} needs exactly one generic, two is rejected.
    assert!(matches!(
        state.submit_decision(Decision::Pay(Payment {
            generic: vec![red(), green()]
        })),
        Err(DecisionError::Illegal { .. })
    ));
    // (ii) Spend a color the pool can't cover after the colored pip: paying {1}
    //      with a Blue the pool never had.
    assert!(matches!(
        state.submit_decision(Decision::Pay(Payment {
            generic: vec![Color::Blue.into()]
        })),
        Err(DecisionError::Illegal { .. })
    ));
    // State untouched: still pending, pool unchanged.
    assert!(matches!(
        state.pending,
        Some(PendingDecision::PayMana { .. })
    ));
    assert_eq!(
        state.player(PlayerId(0)).mana_pool,
        pool_before,
        "a rejected payment leaves the pool untouched"
    );
    // A valid retry is accepted and the cast completes.
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![red()],
        }))
        .unwrap();
    assert!(state.pending.is_none());
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "the Bears reached the stack after retry"
    );
}
