//! End-to-end stack/casting/targeting against fake testing-plugin data,
//! driven entirely through the public API (`step` / `submit_decision`).
//!
//! Each test builds a two-player game from testing cards, forces the relevant
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
    Action, Decision, DecisionError, GameConfig, GameEvent, GameState, ObjectId, Occurrence,
    Payment, PendingDecision, PlayerConfig, PlayerId, Progress, StackObject, StartingPlayer,
    StepOutcome,
};

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

/// A two-player game: player 0 holds Instant `DealDamage` `AnyTarget` and
/// Mountains, player 1 holds Vanilla Creature and Forests. `mountains`
/// Mountains are forced onto player 0's battlefield. Callers force a Vanilla
/// Creature onto player 1's battlefield (as a target) when they need one.
fn bolt_game(seed: u64, mountains: usize) -> GameState {
    let testing = testing();
    let bolt = Arc::new(testing.card("Instant DealDamage AnyTarget").unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
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
    // Mono-typed halves of each deck guarantee the opening seven holds both an
    // instant and a Mountain for player 0 and a Vanilla Creature for player 1.
    for _ in 0..mountains {
        force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    }
    state
}

/// A two-player game where the *casting* player (player 0) holds Vanilla
/// Creature and Forests. `forests` Forests are forced onto player 0's
/// battlefield.
fn bears_game(seed: u64, forests: usize) -> GameState {
    let bears = Arc::new(testing().card("Vanilla Creature").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
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
///
/// When a `PayMana` decision surfaces mid-cast for an all-colored cost
/// (generic == 0, so `Payment { generic: vec![] }` is the only valid answer),
/// this function auto-answers it and continues. Costs with a generic component
/// must be answered explicitly before calling this helper.
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
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                // Auto-answer all-colored costs (generic == 0). Callers that
                // cast spells with a generic component must answer PayMana
                // explicitly before invoking run_to_priority.
                state
                    .submit_decision(Decision::Pay(Payment { generic: vec![] }))
                    .unwrap_or_else(|e| panic!("auto-pay failed (cost has a generic component — answer PayMana explicitly before run_to_priority): {e}"));
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

/// Extracts the `GameEvent` from a `Progress::Applied(Occurrence::Single(_))`,
/// returning `None` for any other variant.
fn applied(p: &Progress) -> Option<&GameEvent> {
    match p {
        Progress::Applied(Occurrence::Single(e)) => Some(e),
        _ => None,
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
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    // P0's precombat main: an instant and an untapped Mountain in play.
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");

    // Cast the instant; answer the target choice with the creature.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(
        legal[0].contains(&bear),
        "the Vanilla Creature is a legal target"
    );
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();

    // Step to the caster's priority: the instant is on the stack (announce
    // done, not yet resolved).
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the instant sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(bolt));
    assert_eq!(state.stack[0].targets, vec![bear]);
    assert!(!state.zones.battlefield.contains(&bolt));

    // Both players pass: the instant resolves, deals 3, SBA destroys the creature.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), StepOrPhase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);

    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::DamageDealt { target, amount: 3, .. }) if *target == bear
        )),
        "3 damage dealt to the Vanilla Creature, trace: {trace:?}"
    );
    // [CR#400.7]: the old ObjectId is gone; check that the SBA fired and a
    // new object landed in P1's graveyard instead.
    assert!(
        state.objects.get(bear).is_none(),
        "old Vanilla Creature id must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "exactly one object (the reminted creature) in P1's graveyard ([CR#704.5g])"
    );
    // [CR#608.2m]/[CR#400.7]: the instant leaves the stack and remints — the
    // old bolt id is gone; a fresh object sits in P0's graveyard.
    assert!(
        state.objects.get(bolt).is_none(),
        "old instant id must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "the reminted instant lands in P0's graveyard ([CR#608.2m])"
    );
    assert!(
        !state.zones.graveyards[0].contains(&bolt),
        "the graveyard object carries a fresh id, not the old stack id"
    );
    assert!(state.stack.is_empty());
}

#[test]
fn bolt_to_the_face_costs_three_life() {
    let mut state = bolt_game(1, 1);

    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
    let face = state.players[1].object;

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&face), "P1's proxy is a legal target");
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
    let bears = find_in_hand(&state, PlayerId(0), "Vanilla Creature");

    // Sorcery-speed cast, no targets; PayMana surfaces for {1}{G} from G,G.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();
    // PayMana must be answered: {G} takes one green pip (forced by color),
    // {1} takes the other green — the only legal allocation from G,G.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{1}}{{G}}, got {stop:?}");
    };
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![green()],
        }))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "the Vanilla Creature spell is on the stack"
    );

    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), StepOrPhase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    // [CR#608.3]/[CR#400.7]: the permanent spell enters the battlefield via a
    // stack→battlefield ZoneWillChange that remints — the old stack id is gone
    // and a fresh object is on the battlefield under P0's control.
    assert!(
        state.objects.get(bears).is_none(),
        "old stack id must be gone after the permanent enters and remints"
    );
    assert!(
        !state.zones.battlefield.contains(&bears),
        "the old stack id must not remain on the battlefield"
    );
    // Find the reminted creature among the battlefield (which also holds the
    // forced Forests).
    let entered = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| {
            state
                .objects
                .obj(o)
                .card_id()
                .is_some_and(|_| matches!(state.def(o), Card::Normal(f) | Card::ModalDfc(f, _) if f.name == "Vanilla Creature"))
        })
        .expect("the reminted Vanilla Creature is on the battlefield");
    assert_ne!(entered, bears, "the entering object carries a fresh id");
    assert_eq!(state.objects.obj(entered).controller, PlayerId(0));
    assert_eq!(state.objects.obj(entered).zone, Some(Zone::Battlefield));
    assert_eq!(printed_pt(&state, entered), Some((2, 2)), "a printed 2/2");
    assert!(state.stack.is_empty());
}

#[test]
fn sorcery_speed_gate_blocks_bears_off_turn_and_on_a_nonempty_stack() {
    // The gate compares Vanilla Creature (sorcery speed) against Instant
    // DealDamage AnyTarget (instant) in two off-window spots. In both, P0 holds
    // instants + a creature + the mana to pay for either, so only the *timing*
    // differs — proving the sorcery-speed gate, not a payment or target gap.

    // (a) On the OPPONENT's turn, in their main phase: P0 has priority. Float
    //     R,G,G through the real mana abilities (so `legal` recomputes); the
    //     creature is timing-blocked while the instant is allowed.
    {
        let testing = testing();
        let bolt = Arc::new(testing.card("Instant DealDamage AnyTarget").unwrap());
        let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
        let mountain = Arc::new(builtin().card("Mountain").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        // 4 instant + 4 creature + 1 Mountain + 3 Forest, fattened so a
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
        // Pick a seed whose P0 opening hand holds both an instant and a creature.
        let mut state = (0u64..1000)
            .map(build)
            .find(|s| {
                let hand = &s.zones.hands[0];
                hand.iter()
                    .any(|&o| is_card(s, o, "Instant DealDamage AnyTarget"))
                    && hand.iter().any(|&o| is_card(s, o, "Vanilla Creature"))
            })
            .expect("a seed with both an instant and a Vanilla Creature in P0's opening hand");
        // P0's mana sources on the battlefield, pulled from the library so they
        // never depend on the opening hand: one Mountain + two Forests.
        force_into_play(&mut state, PlayerId(0), "Mountain");
        force_into_play(&mut state, PlayerId(0), "Forest");
        force_into_play(&mut state, PlayerId(0), "Forest");
        let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
        let bears = find_in_hand(&state, PlayerId(0), "Vanilla Creature");

        // Drive to P0 holding priority during P1's main, tapping all of P0's
        // lands on the way so the pool can pay either spell.
        let legal = drive_to_off_turn_priority(&mut state);
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Vanilla Creature is not castable on the opponent's turn, legal: {legal:?}"
        );
        assert!(
            legal.contains(&Action::CastSpell { object: bolt }),
            "instant is castable on the opponent's turn, legal: {legal:?}"
        );
    }

    // (b) During the active player's OWN main phase but with a non-empty stack
    //     (an instant already announced): creature blocked, a second instant
    //     allowed.
    {
        let mut state = bears_with_bolts();
        let bolt0 = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
        let bears = find_in_hand(&state, PlayerId(0), "Vanilla Creature");
        let bear = force_into_play(&mut state, PlayerId(1), "Vanilla Creature");

        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        // Float R,R,G,G: the first {R} instant leaves R,G,G for the gate
        // comparison (a second {R} instant and a {1}{G} creature are both payable).
        float_mana(&mut state, PlayerId(0), 4);
        // Cast the first instant onto the stack, targeting the creature.
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
        assert_eq!(state.stack.len(), 1, "an instant is on the stack");
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Vanilla Creature blocked while the stack is non-empty, legal: {legal:?}"
        );
        // A second instant is still castable on the non-empty stack.
        let bolt1 = state.zones.hands[0]
            .iter()
            .copied()
            .find(|&o| is_card(&state, o, "Instant DealDamage AnyTarget"))
            .expect("a second instant still in hand");
        assert!(
            legal.contains(&Action::CastSpell { object: bolt1 }),
            "a second instant is castable at instant speed on a non-empty stack, legal: {legal:?}"
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

/// A game where player 0 holds instants + a Vanilla Creature + Forests (a
/// seed with two instants and a creature in the opening hand), two Mountains +
/// two Forests forced onto player 0's battlefield, and player 1 holds a
/// Vanilla Creature (for use as a target).
fn bears_with_bolts() -> GameState {
    let testing = testing();
    let bolt = Arc::new(testing.card("Instant DealDamage AnyTarget").unwrap());
    let bears = Arc::new(testing.card("Vanilla Creature").unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
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
                .filter(|&&o| is_card(s, o, "Instant DealDamage AnyTarget"))
                .count();
            bolts >= 2 && hand.iter().any(|&o| is_card(s, o, "Vanilla Creature"))
        })
        .expect("a seed with two instants and a Vanilla Creature in P0's opening hand");
    // Two Mountains + two Forests → R,R,G,G when all tapped: a {R} instant
    // leaves R,G,G, enough for both a second {R} instant and a {1}{G} creature.
    for _ in 0..2 {
        force_into_play(&mut state, PlayerId(0), "Mountain");
        force_into_play(&mut state, PlayerId(0), "Forest");
    }
    state
}

#[test]
fn paymana_surfaces_for_every_cast() {
    // (a) All-colored cost: instant {R} from a R pool surfaces PayMana even
    //     though there is only one legal allocation (empty generic).
    {
        let mut state = bolt_game(1, 1);
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1); // R
        let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
            .unwrap();
        // ChooseTargets surfaces first (instant has targets).
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { .. }) = stop else {
            panic!("expected ChooseTargets, got {stop:?}");
        };
        state
            .submit_decision(Decision::Targets(vec![bear]))
            .unwrap();
        // PayMana MUST surface — the core never auto-pays, even for {R}.
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
            panic!("expected PayMana for {{R}} (always explicit), got {stop:?}");
        };
        // {R} has no generic: empty Payment is the only valid answer.
        state
            .submit_decision(Decision::Pay(Payment { generic: vec![] }))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "the instant reached the stack");
        assert!(
            state.player(PlayerId(0)).mana_pool.is_empty(),
            "the Red was spent"
        );
    }

    // (b) Mixed cost with a real choice: {1}{G} from a G,G,R pool surfaces
    //     PayMana with the {1} generic open to Green or Red.
    {
        let mut state = bears_game(2, 2);
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G from forests
        // Add a stray Red so {1} has a real choice.
        state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
        let bears = find_in_hand(&state, PlayerId(0), "Vanilla Creature");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
            .unwrap();
        // PayMana surfaces (Vanilla Creature has no targets, so we skip ChooseTargets).
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
            panic!("expected PayMana for {{1}}{{G}} from G,G,R, got {stop:?}");
        };
        let _ = cost;
        // Pay {1} with the Red (either Red or one Green is legal here).
        state
            .submit_decision(Decision::Pay(Payment {
                generic: vec![red()],
            }))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        assert_eq!(
            state.stack.len(),
            1,
            "the Vanilla Creature reached the stack"
        );
        // {G} took one Green, {1} took the Red → one Green remains.
        let pool = &state.player(PlayerId(0)).mana_pool;
        assert_eq!(pool.amount(green()), 1, "one Green left");
        assert_eq!(pool.amount(red()), 0, "the Red paid {{1}}");
    }
}

#[test]
fn second_bolt_fizzles_when_its_target_is_already_dead() {
    let mut state = bolt_game(1, 2); // two Mountains for two {R} casts
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R

    // Cast instant A targeting the Vanilla Creature.
    let bolt_a = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_a }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);

    // In response (LIFO), cast instant B also targeting the Vanilla Creature.
    let bolt_b = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
    assert_ne!(bolt_a, bolt_b);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_b }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "both instants on the stack");

    // Pass both instants to resolution: B resolves (kills creature), then A
    // fizzles.
    let mut damage_events = 0;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::DamageDealt { amount, .. },
            ))) => {
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
        // [CR#400.7]: after reminting, the old `bear` id is gone — break when the
        // stack is empty and P1's graveyard has a (new) object in it.
        if state.stack.is_empty() && !state.zones.graveyards[1].is_empty() {
            // Drain any remaining priority passes for the empty stack, then stop.
            break;
        }
    }

    assert_eq!(
        damage_events, 1,
        "only the top instant dealt damage; the second fizzled ([CR#608.2b])"
    );
    // [CR#400.7]: old id gone; graveyard has the reminted creature.
    assert!(
        state.objects.get(bear).is_none(),
        "old Vanilla Creature id must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "the Vanilla Creature died to the first instant ([CR#704.5g])"
    );
    // [CR#400.7]: both instants leave the stack and remint — their old stack
    // ids are gone; two fresh objects end in P0's graveyard.
    assert!(
        state.objects.get(bolt_a).is_none() && state.objects.get(bolt_b).is_none(),
        "both old instant ids must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        2,
        "both reminted instants end in P0's graveyard"
    );
    assert!(state.stack.is_empty());
}

#[test]
fn a_cast_game_is_deterministic() {
    // A reusable script: drive to P0's main, float {R}, cast the instant at
    // the Vanilla Creature, pass both, resolve. Run it twice and compare a
    // fingerprint.
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
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
        let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1);
        let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
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
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };

    // (i) An object not in the legal set — the instant itself (on the stack,
    //     not a creature/player) is illegal.
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
    let bears = find_in_hand(&state, PlayerId(0), "Vanilla Creature");
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
        "the Vanilla Creature reached the stack after retry"
    );
}

/// End-to-end dies-trigger + target-on-trigger + LKI ([CR#603.3,603.10a]):
///
/// P0 controls a `Creature dies-trigger DealDamage AnyTarget` (a 1/1) on the
/// battlefield. P0 casts the fake bolt at the goblin (3 to it). The bolt
/// resolves, the SBA destroys the goblin (lethal), its dies-trigger NOTES, then
/// the `PlaceTriggers` barrier surfaces `ChooseTargets` for the goblin's "any
/// target". We choose a player proxy; the trigger resolves and deals 1 — and
/// the damage's source is the *dead* goblin's id (the LKI source), not a live
/// object.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "an end-to-end decision-driven scenario"
)]
fn dies_trigger_deals_damage_from_the_dead_source() {
    let testing = testing();
    let bolt = Arc::new(testing.card("Instant DealDamage AnyTarget").unwrap());
    let goblin = Arc::new(
        testing
            .card("Creature dies-trigger DealDamage AnyTarget")
            .unwrap(),
    );
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    // P0's deck: bolts + goblins + mountains. P1: forests.
    let mut p0 = vec![Arc::clone(&bolt); 4];
    p0.extend(vec![Arc::clone(&goblin); 3]);
    p0.extend(vec![Arc::clone(&mountain); 4]);
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: p0.clone() },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 12],
                },
            ],
            seed,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    // A seed whose P0 opening hand holds a bolt (cast from hand) — the goblin
    // and Mountain are pulled from the library by `force_into_play`.
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            s.zones.hands[0]
                .iter()
                .any(|&o| is_card(s, o, "Instant DealDamage AnyTarget"))
        })
        .expect("a seed with a bolt in P0's opening hand");

    let gob = force_into_play(
        &mut state,
        PlayerId(0),
        "Creature dies-trigger DealDamage AnyTarget",
    );
    force_into_play(&mut state, PlayerId(0), "Mountain");

    // P0's precombat main: float {R}, cast the bolt at the goblin.
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Instant DealDamage AnyTarget");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets for the bolt, got {stop:?}");
    };
    assert!(legal[0].contains(&gob), "the goblin is a legal bolt target");
    state.submit_decision(Decision::Targets(vec![gob])).unwrap();
    // PayMana for {R}.
    let _ = run_to_priority(&mut state, PlayerId(0), StepOrPhase::PrecombatMain);

    // Both players pass: the bolt resolves (3 to the goblin), the SBA destroys
    // it, and the dies-trigger NOTES — then `PlaceTriggers` surfaces a
    // `ChooseTargets` for the trigger's own "any target".
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (_, stop) = loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            // P1's priority over the (still empty) stack: pass it along.
            StepOutcome::NeedsDecision(PendingDecision::Priority {
                player: PlayerId(1),
                ..
            }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => break (Vec::<Progress>::new(), other),
        }
    };

    // The trigger's target choice surfaces (placement, [CR#603.3d]).
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { player, legal, .. }) = stop
    else {
        panic!("expected the dies-trigger's ChooseTargets, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the goblin's controller chooses");
    // The goblin is gone; choose P1's player proxy as the "any target".
    let p1_proxy = state.players[1].object;
    assert!(
        legal[0].contains(&p1_proxy),
        "P1's proxy is a legal any-target, legal: {legal:?}"
    );
    assert!(
        state.objects.get(gob).is_none(),
        "the goblin is dead — its old id is gone before its trigger is placed"
    );
    state
        .submit_decision(Decision::Targets(vec![p1_proxy]))
        .unwrap();

    // A `Triggered` stack object now sits on the stack; both players pass and it
    // resolves, dealing 1 to P1.
    let triggered_on_stack = state
        .stack
        .iter()
        .any(|e| matches!(e.object, StackObject::Triggered { .. }));
    assert!(triggered_on_stack, "the dies-trigger is on the stack");

    // Drive to resolution, collecting the damage event.
    let mut damage_source: Option<ObjectId> = None;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::DamageDealt {
                    source,
                    target,
                    amount,
                },
            ))) if target == p1_proxy => {
                assert_eq!(amount, 1, "the dies-trigger deals 1");
                damage_source = Some(source);
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(_) => break,
        }
        if damage_source.is_some() && state.stack.is_empty() {
            break;
        }
    }

    // P1 took 1 from the dead goblin.
    assert_eq!(state.players[1].life, 19, "20 - 1 from the dies-trigger");
    // [CR#603.10a]: the damage's source is the goblin's (now-stale) battlefield
    // id — the LKI source, not any live object.
    assert_eq!(
        damage_source,
        Some(gob),
        "the damage is dealt by the dead goblin's LKI id"
    );
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending after placement+resolution"
    );
    // The triggered ability vanished — no Triggered entry remains.
    assert!(
        !state
            .stack
            .iter()
            .any(|e| matches!(e.object, StackObject::Triggered { .. })),
        "the triggered ability left the stack on resolution ([CR#603.8])"
    );
}
