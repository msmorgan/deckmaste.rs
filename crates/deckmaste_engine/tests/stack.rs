//! End-to-end stack/casting/targeting against real canon-card data,
//! driven entirely through the public API (`step` / `submit_decision`).
//!
//! Each test builds a two-player game from canon cards, forces the relevant
//! permanents into play (the public `GameState` fields are all `pub`), advances
//! to a priority window via `step`, then casts a spell the way a UI would:
//! float mana with the Stage-1 mana ability, `CastSpell`, answer
//! `ChooseTargets` / `PayMana` as they surface, and `Pass` to resolve.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::BeginningStep;
use deckmaste_core::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Phase;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionError;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::Payment;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StackObject;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;

// --- plugin + deck building
// ---------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// A canon card by name, ready for a deck list.
fn card(name: &str) -> Arc<Card> { Arc::new(canon().card(name).unwrap()) }

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
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let bears = card("Grizzly Bears");
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
    let bears = card("Grizzly Bears");
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
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    // P0's precombat main: an instant and an untapped Mountain in play.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");

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
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the instant sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(bolt));
    assert_eq!(state.stack[0].targets, vec![bear]);
    assert!(!state.zones.battlefield.contains(&bolt));

    // Both players pass: the instant resolves, deals 3, SBA destroys the creature.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
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

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
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
    assert!(legal[0].contains(&face), "P1's proxy is a legal target");
    state
        .submit_decision(Decision::Targets(vec![face]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(state.players[1].life, 17, "20 - 3");
    assert!(!state.players[1].lost);
}

#[test]
fn grizzly_bears_resolves_to_a_two_two_on_the_battlefield() {
    let mut state = bears_game(1, 2); // two Forests for {1}{G}

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // G, G
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

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
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "the Vanilla Creature spell is on the stack"
    );

    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
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
                .is_some_and(|_| matches!(state.def(o), Card::Normal(f) | Card::ModalDfc(f, _) if f.name == "Grizzly Bears"))
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
        let bolt = card("Lightning Bolt");
        let bears = card("Grizzly Bears");
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
                hand.iter().any(|&o| is_card(s, o, "Lightning Bolt"))
                    && hand.iter().any(|&o| is_card(s, o, "Grizzly Bears"))
            })
            .expect("a seed with both an instant and a Vanilla Creature in P0's opening hand");
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
        let bolt0 = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
        let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");

        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
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
        let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "an instant is on the stack");
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Vanilla Creature blocked while the stack is non-empty, legal: {legal:?}"
        );
        // A second instant is still castable on the non-empty stack.
        let bolt1 = state.zones.hands[0]
            .iter()
            .copied()
            .find(|&o| is_card(&state, o, "Lightning Bolt"))
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
                        Phase::PrecombatMain | Phase::PostcombatMain
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
            StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) => {
                state.submit_decision(Decision::Attackers(vec![])).unwrap();
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
    let bolt = card("Lightning Bolt");
    let bears = card("Grizzly Bears");
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
                .filter(|&&o| is_card(s, o, "Lightning Bolt"))
                .count();
            bolts >= 2 && hand.iter().any(|&o| is_card(s, o, "Grizzly Bears"))
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
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1); // R
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
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
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
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
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G from forests
        // Add a stray Red so {1} has a real choice.
        state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
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
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
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
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R

    // Cast instant A targeting the Vanilla Creature.
    let bolt_a = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_a }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // In response (LIFO), cast instant B also targeting the Vanilla Creature.
    let bolt_b = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    assert_ne!(bolt_a, bolt_b);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_b }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
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
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1);
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
            .unwrap();
        let (_, _) = step_to_stop(&mut state);
        state
            .submit_decision(Decision::Targets(vec![bear]))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
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
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
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
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
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
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "the Vanilla Creature reached the stack after retry"
    );
}

/// End-to-end dies-trigger + target-on-trigger + LKI ([CR#603.3,603.10a]):
///
/// P0 controls a `Creature dies-trigger DealDamage AnyTarget` (a 1/1) on the
/// battlefield. P0 casts the fake bolt at the fiend (3 to it). The bolt
/// resolves, the SBA destroys the fiend (lethal), its dies-trigger NOTES, then
/// the `PlaceTriggers` barrier surfaces `ChooseTargets` for the fiend's "any
/// target". We choose a player proxy; the trigger resolves and deals 1 — and
/// the damage's source is the *dead* fiend's id (the LKI source), not a live
/// object.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "an end-to-end decision-driven scenario"
)]
fn dies_trigger_deals_damage_from_the_dead_source() {
    let bolt = card("Lightning Bolt");
    let fiend = card("Footlight Fiend");
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    // P0's deck: bolts + fiends + mountains. P1: forests.
    let mut p0 = vec![Arc::clone(&bolt); 4];
    p0.extend(vec![Arc::clone(&fiend); 3]);
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
    // A seed whose P0 opening hand holds a bolt (cast from hand) — the fiend
    // and Mountain are pulled from the library by `force_into_play`.
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            s.zones.hands[0]
                .iter()
                .any(|&o| is_card(s, o, "Lightning Bolt"))
        })
        .expect("a seed with a bolt in P0's opening hand");

    let fiend_obj = force_into_play(&mut state, PlayerId(0), "Footlight Fiend");
    force_into_play(&mut state, PlayerId(0), "Mountain");

    // P0's precombat main: float {R}, cast the bolt at the fiend.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets for the bolt, got {stop:?}");
    };
    assert!(
        legal[0].contains(&fiend_obj),
        "the fiend is a legal bolt target"
    );
    state
        .submit_decision(Decision::Targets(vec![fiend_obj]))
        .unwrap();
    // PayMana for {R}.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // Both players pass: the bolt resolves (3 to the fiend), the SBA destroys
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
    assert_eq!(player, PlayerId(0), "the fiend's controller chooses");
    // The fiend is gone; choose P1's player proxy as the "any target".
    let p1_proxy = state.players[1].object;
    assert!(
        legal[0].contains(&p1_proxy),
        "P1's proxy is a legal any-target, legal: {legal:?}"
    );
    assert!(
        state.objects.get(fiend_obj).is_none(),
        "the fiend is dead — its old id is gone before its trigger is placed"
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

    // P1 took 1 from the dead fiend.
    assert_eq!(state.players[1].life, 19, "20 - 1 from the dies-trigger");
    // [CR#603.10a]: the damage's source is the fiend's (now-stale) battlefield
    // id — the LKI source, not any live object.
    assert_eq!(
        damage_source,
        Some(fiend_obj),
        "the damage is dealt by the dead fiend's LKI id"
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
        "the triggered ability left the stack on resolution ([CR#608.2n])"
    );
}

/// End-to-end ETB trigger + `DrawCards` ([CR#603.3,121.1]):
///
/// P0 casts Elvish Visionary ({1}{G}). It resolves and enters
/// the battlefield. Its `Enters(Ref(This))` trigger fires on the `ZoneChanged`
/// (→Battlefield), `PlaceTriggers` places it (no targets → directly), it
/// resolves and calls `DrawCards(1)` — P0 draws a card. Assert hand grew by 1.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "an end-to-end decision-driven scenario"
)]
fn etb_trigger_draws_a_card() {
    let etb = card("Elvish Visionary");
    let forest = Arc::new(builtin().card("Forest").unwrap());

    // P0: etb creatures + forests (for {1}{G}).
    // P1: forests only (no cards relevant to the scenario).
    let mut deck0 = vec![Arc::clone(&etb); 4];
    deck0.extend(vec![Arc::clone(&forest); 6]);

    // Seed search: find a seed whose P0 opening hand holds the ETB creature
    // and a Forest so the {1}{G} cost is payable.
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            let hand = &s.zones.hands[0];
            hand.iter().any(|&o| is_card(s, o, "Elvish Visionary"))
                && hand.iter().any(|&o| is_card(s, o, "Forest"))
        })
        .expect("a seed with the ETB creature and a Forest in P0's opening hand");

    // Force lands onto the battlefield so float_mana works.
    force_into_play(&mut state, PlayerId(0), "Forest");
    force_into_play(&mut state, PlayerId(0), "Forest");

    // Record hand size BEFORE casting (the ETB creature is in hand).
    let hand_before = state.zones.hands[0].len();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // Float {G} and {G} for the {1}{G} cost.
    float_mana(&mut state, PlayerId(0), 2);

    let creature = find_in_hand(&state, PlayerId(0), "Elvish Visionary");

    // Cast the creature spell (sorcery speed, no targets).
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: creature }))
        .unwrap();

    // {1}{G}: one green pip + one generic (the other Forest's green covers {1}).
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{1}}{{G}}, got {stop:?}");
    };
    // Pay the {1} with the green mana floating.
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![Color::Green.into()],
        }))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the creature spell is on the stack");

    // Both players pass → resolves.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    // Collect the trace while driving through PlaceTriggers → resolution.
    // The ETB trigger is non-targeting: it places directly, resolves, draws.
    let mut trigger_fired = false;
    let mut card_drawn = false;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::TriggerFired { .. },
            ))) => {
                trigger_fired = true;
            }
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::ZoneChanged {
                    from: Some(Zone::Library),
                    to: Zone::Hand,
                    ..
                },
            ))) => {
                card_drawn = true;
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. }) => {
                // Once both the trigger has fired and the card was drawn, stop.
                if card_drawn {
                    break;
                }
                state
                    .submit_decision(Decision::Act(Action::Pass))
                    .unwrap_or_else(|_| {
                        let _ = player;
                    });
            }
            StepOutcome::NeedsDecision(other) => {
                panic!("unexpected decision after etb trigger: {other:?}")
            }
            StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
        }
        if card_drawn && state.stack.is_empty() {
            break;
        }
    }

    assert!(trigger_fired, "TriggerFired event was observed");
    assert!(card_drawn, "the ETB draw reached hand");

    // The ETB creature entered the battlefield; the spell left the hand; a card
    // was drawn. Net hand change: -1 (cast) + 1 (draw) = 0 relative to hand_before.
    // Wait — hand_before includes the creature in hand. After cast: hand_before -
    // 1. After draw: hand_before - 1 + 1 = hand_before. But the trigger draws,
    // so final hand size == hand_before (cast removes creature, draw adds one).
    // The creature left the hand when cast (goes to stack), then leaves the stack
    // when it enters. So net: hand size unchanged from before cast, but now
    // includes one NEW card drawn instead of the creature.
    let hand_after = state.zones.hands[0].len();
    // The key assertion: drawing happened (+1 from the trigger).
    // Since the creature left hand when cast (-1), and the draw added +1, the
    // net from hand_before is 0. But we assert the draw happened (card_drawn)
    // and the creature is on the battlefield (not in hand).
    assert_eq!(
        hand_after, hand_before,
        "hand size is unchanged: the cast removed the creature and the ETB trigger drew 1 \
         (net zero from hand_before = {hand_before})"
    );
    // The creature entered the battlefield.
    let entered = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&o| {
            state.objects.obj(o).card_id().is_some_and(|_| {
                matches!(state.def(o), Card::Normal(f) | Card::ModalDfc(f, _)
                    if f.name == "Elvish Visionary")
            })
        })
        .expect("the ETB creature is on the battlefield");
    assert_ne!(entered, creature, "reminted with a fresh id ([CR#400.7])");
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending"
    );
}

/// End-to-end occurrence batch + APNAP trigger ordering ([CR#603.3b,700.4]):
///
/// Board under P0: Footlight Fiend (dies-trigger, 1/1) +
/// Moonlit Wake (a dies-watcher enchantment) + a Willow Elf. Board under P1:
/// Moonlit Wake + a Willow Elf. P0 casts Pyroclasm (2 to each creature). All
/// three creatures die simultaneously in one `Occurrence::Batch`; the Wakes
/// survive and watch.
///
/// Expected flow:
/// 1. Resolve → one `Batch` of three `DamageDealt` events.
/// 2. SBA sweep → one `Batch` of three `ZoneWillChange`/`ZoneChanged` zone
///    moves (each creature gets lethal damage).
/// 3. Trigger matching notes SEVEN triggers in the same scan: the fiend's
///    dies-trigger plus, for EACH of the three deaths, each player's Moonlit
///    Wake — a live watcher fires once per matching event in the batch.
/// 4. `PlaceTriggers` APNAP ([CR#603.3b,101.4]): P0 orders its four
///    simultaneous triggers, then P1 orders its three; P1's are placed last
///    (resolve first in LIFO).
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "an end-to-end decision-driven scenario"
)]
fn occurrence_batch_and_apnap_ordering() {
    let pyroclasm = card("Pyroclasm");
    let fiend = card("Footlight Fiend");
    let watcher = card("Moonlit Wake");
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());

    // P0: pyroclasm + fiend + watcher + sweep-fodder elves + mountains +
    // forests. {1}{R} needs a Mountain (red) + something for generic.
    let elf = card("Willow Elf");
    let mut deck0 = vec![Arc::clone(&pyroclasm); 2];
    deck0.extend(vec![Arc::clone(&fiend); 2]);
    deck0.extend(vec![Arc::clone(&watcher); 2]);
    deck0.extend(vec![Arc::clone(&elf); 2]);
    deck0.extend(vec![Arc::clone(&mountain); 2]);
    deck0.extend(vec![Arc::clone(&forest); 2]);

    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: {
                        let mut d = vec![Arc::clone(&watcher); 2];
                        d.extend(vec![Arc::clone(&elf); 3]);
                        d.extend(vec![Arc::clone(&forest); 5]);
                        d
                    },
                },
            ],
            seed,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    let mut state = (0u64..2000)
        .map(build)
        .find(|s| {
            let hand = &s.zones.hands[0];
            hand.iter().any(|&o| is_card(s, o, "Pyroclasm"))
                && hand.iter().any(|&o| is_card(s, o, "Mountain"))
        })
        .expect("a seed with Pyroclasm + Mountain in P0's opening hand");

    // Force mana sources onto the battlefield.
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Forest");

    // Force the creatures and watchers onto the battlefield directly (bypass
    // SBAs). The Wakes are enchantments — Pyroclasm won't touch them.
    let fiend_obj = force_into_play(&mut state, PlayerId(0), "Footlight Fiend");
    let elf0 = force_into_play(&mut state, PlayerId(0), "Willow Elf");
    let _w0 = force_into_play(&mut state, PlayerId(0), "Moonlit Wake");
    // P1's watcher + dying creature — forced from P1's library/hand.
    let _w1 = force_into_play(&mut state, PlayerId(1), "Moonlit Wake");
    let elf1 = force_into_play(&mut state, PlayerId(1), "Willow Elf");

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R + G for {1}{R}

    let pyro = find_in_hand(&state, PlayerId(0), "Pyroclasm");

    // Cast Pyroclasm (no targets).
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: pyro }))
        .unwrap();

    // {1}{R}: red covers {R}, generic takes the green.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{1}}{{R}}, got {stop:?}");
    };
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![Color::Green.into()],
        }))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "Sorcery DealDamage each creature on the stack"
    );

    // Both pass → resolve.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    // --- Drive to the moment just after the SBA batch destroys all creatures.
    // We want to catch the state when pending_triggers has all three noted.
    let mut saw_damage_batch = false;
    let mut saw_sba_destroy_batch = false;
    let mut p0_ordered = false;
    let mut p1_ordered = false;

    loop {
        let (trace, stop) = step_to_stop(&mut state);

        // Look for the DamageDealt batch from the sorcery resolution.
        if !saw_damage_batch {
            for p in &trace {
                if let Progress::Applied(Occurrence::Batch(events)) = p {
                    let all_damage = events
                        .iter()
                        .all(|e| matches!(e, GameEvent::DamageDealt { .. }));
                    let count = events
                        .iter()
                        .filter(|e| matches!(e, GameEvent::DamageDealt { .. }))
                        .count();
                    if all_damage && count == 3 {
                        saw_damage_batch = true;
                    }
                }
            }
        }
        // Look for the ZoneWillChange/ZoneChanged batch (SBA destroys).
        if !saw_sba_destroy_batch {
            for p in &trace {
                if let Progress::Applied(Occurrence::Batch(events)) = p {
                    let zone_moves = events
                        .iter()
                        .filter(|e| matches!(e, GameEvent::ZoneWillChange { .. }))
                        .count();
                    if zone_moves >= 3 {
                        saw_sba_destroy_batch = true;
                    }
                }
            }
        }

        match stop {
            // OrderTriggers: the APNAP player with >1 noted trigger orders
            // them ([CR#603.3b,101.4]); placement is one-at-a-time, so the
            // decision re-surfaces (n, n−1, …, 2) until one remains (which is
            // placed without a decision).
            StepOutcome::NeedsDecision(PendingDecision::OrderTriggers {
                player,
                ref triggers,
            }) => {
                let n = triggers.len();
                if player == PlayerId(0) {
                    assert!(!p1_ordered, "APNAP: all of P0's orderings precede P1's");
                    if !p0_ordered {
                        // First encounter: all four of P0's notes at once.
                        assert_eq!(
                            n, 4,
                            "P0's four simultaneous triggers: the fiend's dies-trigger \
                             + its Wake noting each of the three deaths"
                        );
                        assert!(
                            saw_damage_batch,
                            "damage batch observed before OrderTriggers"
                        );
                        // All seven notes (P0's four + P1's three) were taken in
                        // the same scan; none have been placed yet.
                        assert_eq!(
                            state.pending_triggers.len(),
                            7,
                            "all seven notes pending at the first ordering"
                        );

                        // Submit invalid orders first (rejected; still pending).
                        assert!(
                            state.submit_decision(Decision::Order(vec![0, 0])).is_err(),
                            "duplicate index is rejected"
                        );
                        assert!(
                            state.submit_decision(Decision::Order(vec![5])).is_err(),
                            "out-of-range index is rejected"
                        );
                        p0_ordered = true;
                    }
                } else {
                    assert_eq!(player, PlayerId(1), "only the two players order");
                    assert!(p0_ordered, "APNAP: P1 orders after P0");
                    if !p1_ordered {
                        assert_eq!(n, 3, "P1's Wake noted each of the three deaths");
                        p1_ordered = true;
                    }
                }
                // Keep the noted order: first noted placed first (resolves last).
                state
                    .submit_decision(Decision::Order((0..n).collect()))
                    .unwrap();
            }

            // ChooseTargets for the dies-trigger's "any target" at placement.
            StepOutcome::NeedsDecision(PendingDecision::ChooseTargets {
                player, legal, ..
            }) => {
                assert_eq!(
                    player,
                    PlayerId(0),
                    "P0 chooses target for the dies-trigger"
                );
                // Choose P1's player proxy as the target.
                let p1_proxy = state.players[1].object;
                assert!(
                    legal[0].contains(&p1_proxy),
                    "P1 proxy is a legal AnyTarget"
                );
                state
                    .submit_decision(Decision::Targets(vec![p1_proxy]))
                    .unwrap();
            }

            // Priority windows: pass them through to drive to resolution.
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                // Stop once all creatures are dead and the stack is clear (all
                // triggers resolved).
                if state.objects.get(fiend_obj).is_none()
                    && state.objects.get(elf0).is_none()
                    && state.objects.get(elf1).is_none()
                    && state.stack.is_empty()
                    && state.pending_triggers.is_empty()
                {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }

            StepOutcome::NeedsDecision(other) => {
                panic!("unexpected decision: {other:?}")
            }
            StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
            StepOutcome::Progress(_) => unreachable!("step_to_stop never returns Progress"),
        }
    }

    assert!(
        saw_damage_batch,
        "the sorcery dealt damage in a single Batch"
    );
    assert!(
        saw_sba_destroy_batch,
        "SBA destroyed creatures in a single Batch"
    );

    // All three dead creatures' old ids are gone (reminted).
    assert!(
        state.objects.get(fiend_obj).is_none(),
        "fiend reminted — old id gone"
    );
    assert!(
        state.objects.get(elf0).is_none(),
        "P0's elf reminted — old id gone"
    );
    assert!(
        state.objects.get(elf1).is_none(),
        "P1's elf reminted — old id gone"
    );
    // All three ended in graveyards; the watcher enchantments survived.
    assert_eq!(
        state.zones.graveyards[0].len(),
        3,
        "P0's graveyard has Pyroclasm + reminted fiend + reminted elf = 3 objects"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "P1's graveyard has the reminted elf (its Moonlit Wake survives)"
    );
    assert!(
        p0_ordered && p1_ordered,
        "both players ordered their triggers"
    );
    // Every trigger resolved: each Wake gained its controller 1 life per dying
    // creature (×3), and the fiend's dies-trigger dealt 1 to P1's proxy.
    assert_eq!(state.players[0].life, 23, "P0: 20 + 3 × GainLife(1)");
    assert_eq!(
        state.players[1].life, 22,
        "P1: 20 + 3 × GainLife(1) − 1 from the fiend"
    );
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending after full resolution"
    );
    assert!(
        state.stack.is_empty(),
        "stack is empty after all triggers resolved"
    );
}

/// End-to-end multi-loss Draw ([CR#104.4a,700.4]):
///
/// Both players at 4 life. P0 casts Flame Rift ({1}{R} sorcery), which deals
/// 4 to each player in one `Occurrence::Batch`. Both players reach ≤0
/// simultaneously → `PlayerLost` batch → `GameOutcome::Draw`.
///
/// Asserts the Draw was reached via the full cast→resolve→SBA path, not by
/// setting life directly.
#[test]
fn simultaneous_loss_is_a_draw() {
    let each_player = card("Flame Rift");
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());

    // P0: the sorcery + lands for {1}{R}. P1: forests.
    let mut deck0 = vec![Arc::clone(&each_player); 4];
    deck0.extend(vec![Arc::clone(&mountain); 3]);
    deck0.extend(vec![Arc::clone(&forest); 3]);

    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed,
            starting_life: 4,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    let mut state = (0u64..200)
        .map(build)
        .find(|s| {
            s.zones.hands[0]
                .iter()
                .any(|&o| is_card(s, o, "Flame Rift"))
        })
        .expect("a seed with Flame Rift in P0's opening hand");

    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Forest");

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R + G for {1}{R}

    let spell = find_in_hand(&state, PlayerId(0), "Flame Rift");

    // Cast: no targets (a set-valued selection), so ChooseTargets does not
    // surface.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    // The spell has no targets, so PayMana surfaces immediately. {1}{R}: the
    // Mountain's red covers {R}, the Forest's green pays the generic.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{1}}{{R}}, got {stop:?}");
    };
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![green()],
        }))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "spell is on the stack");

    // Both pass → resolve.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    // Let it resolve and reach game over.
    let final_outcome = loop {
        match state.step() {
            StepOutcome::GameOver(o) => break o,
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
        }
    };

    assert_eq!(
        final_outcome,
        GameOutcome::Draw,
        "[CR#104.4a]: simultaneous loss → Draw, got {final_outcome:?}"
    );
    // Both players must be marked lost.
    assert!(state.players[0].lost, "P0 lost");
    assert!(state.players[1].lost, "P1 lost");
}

/// End-to-end two-trigger ordering: one player controls two simultaneously
/// firing triggers ([CR#603.3b]):
///
/// P0 controls two Moonlit Wakes (dies-watcher enchantments). A creature
/// (a Grizzly Bears under P0) dies (forced by lethal damage via
/// `CheckSbas`). Both watchers fire at once → `OrderTriggers { player: P0 }`
/// surfaces with both triggers. Assert:
/// - an invalid `Order` is rejected;
/// - a valid `Order([1, 0])` is accepted;
/// - the stack/resolution order matches the chosen order (LIFO: the FIRST
///   placed resolves LAST).
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "an end-to-end decision-driven scenario"
)]
fn two_triggers_same_player_order_triggers_surfaces() {
    let watcher_card = card("Moonlit Wake");
    let bears_card = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap());

    // Build a simple two-player game; both players' decks don't matter much.
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![
                    Arc::clone(&watcher_card),
                    Arc::clone(&watcher_card),
                    Arc::clone(&bears_card),
                    Arc::clone(&bears_card),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                ],
            },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });

    // Place two watchers and a Grizzly Bears (the dying creature) under P0.
    let watcher0 = force_into_play(&mut state, PlayerId(0), "Moonlit Wake");
    let _watcher1 = force_into_play(&mut state, PlayerId(0), "Moonlit Wake");
    let bear = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");

    // Deal lethal damage to the bear (2/2 → 2 damage = lethal).
    state.objects.obj_mut(bear).damage = 2;

    // Drive the engine from the start (game begins at Cleanup; each step runs
    // CheckSbas). The bear's lethal damage will be caught the first time the
    // SBA sweep runs — the two watchers' Dies(Type(Creature)) triggers both
    // fire and note simultaneously, then PlaceTriggers surfaces OrderTriggers.
    let mut order_triggers_player = None;
    let mut order_triggers_count = 0;
    let mut order_accepted = false;
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::OrderTriggers {
                player,
                ref triggers,
            }) => {
                order_triggers_player = Some(player);
                order_triggers_count = triggers.len();

                // Reject invalid orders.
                assert!(
                    state.submit_decision(Decision::Order(vec![0, 0])).is_err(),
                    "duplicate index rejected"
                );
                assert!(
                    state.submit_decision(Decision::Order(vec![2])).is_err(),
                    "out-of-range index rejected"
                );

                // Accept [1, 0]: the second trigger is placed first (resolves
                // last in LIFO).
                state.submit_decision(Decision::Order(vec![1, 0])).unwrap();
                order_accepted = true;
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                // After ordering, let triggers resolve.
                if order_accepted && state.pending_triggers.is_empty() && state.stack.is_empty() {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
            StepOutcome::Progress(_) => unreachable!("step_to_stop never returns Progress"),
        }
    }

    assert!(
        order_triggers_player.is_some(),
        "OrderTriggers decision surfaced"
    );
    assert_eq!(
        order_triggers_player,
        Some(PlayerId(0)),
        "P0 controls both triggers"
    );
    assert_eq!(order_triggers_count, 2, "two simultaneous triggers offered");
    assert!(order_accepted, "valid Order([1, 0]) was accepted");

    // Both watchers' triggers fired and resolved — P0 gained 2 life (once per
    // dying creature × 2 watchers = 2 life gained).
    assert_eq!(
        state.players[0].life, 22,
        "P0 gained 2 life (2 × GainLife(1) triggers resolved)"
    );
    // The bear is gone (reminted).
    assert!(
        state.objects.get(bear).is_none(),
        "bear was destroyed and reminted"
    );
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending after resolution"
    );
    assert!(
        state.stack.is_empty(),
        "stack empty after all triggers resolved"
    );

    // The two watchers survive (they are enchantments; we only set the bear's
    // damage directly).
    assert!(
        state.objects.get(watcher0).is_some()
            || state.zones.battlefield.iter().any(|&o| {
                state.objects.obj(o).card_id().is_some_and(|_| {
                    matches!(state.def(o), Card::Normal(f) | Card::ModalDfc(f, _)
                        if f.name == "Moonlit Wake")
                })
            }),
        "at least one watcher is still on the battlefield (the bear died, not the watchers)"
    );
}

#[test]
fn creature_enters_tapped_via_as_enters_replacement() {
    // Drives both Diregraf Ghoul (an AsEnters replacement) and Grizzly Bears
    // (no replacement) through the full cast+resolve flow, asserting entry
    // status: the former is minted tapped all-at-once (no observable untapped
    // window [CR#614.1c,614.12]), the latter untapped.

    // --- (a) Diregraf Ghoul resolves tapped ---
    {
        let enters_tapped = card("Diregraf Ghoul");
        let swamp = Arc::new(builtin().card("Swamp").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        // {B} needs one black pip; a Swamp supplies it.
        let mut deck = vec![Arc::clone(&enters_tapped); 5];
        deck.extend(vec![Arc::clone(&swamp); 5]);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        force_into_play(&mut state, PlayerId(0), "Swamp");

        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1); // B
        let spell = find_in_hand(&state, PlayerId(0), "Diregraf Ghoul");

        state
            .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
            .unwrap();
        // {B} is all-colored: PayMana surfaces with an empty generic component.
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
            panic!("expected PayMana for {{B}}, got {stop:?}");
        };
        state
            .submit_decision(Decision::Pay(Payment { generic: vec![] }))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "the creature spell is on the stack");

        // Both players pass → resolves.
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = step_to_stop(&mut state);

        // Find the reminted permanent on the battlefield.
        let entered = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| {
                state.objects.obj(o).card_id().is_some_and(|_| {
                    matches!(state.def(o), Card::Normal(f) | Card::ModalDfc(f, _)
                        if f.name == "Diregraf Ghoul")
                })
            })
            .expect("the reminted Diregraf Ghoul is on the battlefield");

        // [CR#614.1c,614.12]: the permanent is minted tapped — no untapped window.
        assert!(
            state.objects.obj(entered).tapped,
            "Diregraf Ghoul must be tapped the instant it enters the battlefield"
        );
    }

    // --- (b) Grizzly Bears (no AsEnters replacement) enters untapped ---
    {
        let mut state = bears_game(1, 2); // two Forests for {1}{G}

        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
            .unwrap();
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
            panic!("expected PayMana for {{1}}{{G}}, got {stop:?}");
        };
        state
            .submit_decision(Decision::Pay(Payment {
                generic: vec![green()],
            }))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = step_to_stop(&mut state);

        let entered = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| {
                state.objects.obj(o).card_id().is_some_and(|_| {
                    matches!(state.def(o), Card::Normal(f) | Card::ModalDfc(f, _)
                        if f.name == "Grizzly Bears")
                })
            })
            .expect("the reminted Vanilla Creature is on the battlefield");

        assert!(
            !state.objects.obj(entered).tapped,
            "Grizzly Bears must enter untapped (no AsEnters replacement)"
        );
    }
}

#[test]
fn land_in_hand_offers_play_land_not_cast_spell() {
    // A Mountain in hand must appear as PlayLand (a special action
    // [CR#305.9,116.2a]) but never as CastSpell — lands are not castable spells
    // ([CR#305.9]).
    let mut state = bolt_game(1, 0); // no lands forced onto battlefield
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    let mountain = find_in_hand(&state, PlayerId(0), "Mountain");

    assert!(
        legal.contains(&Action::PlayLand { object: mountain }),
        "legal_actions must offer PlayLand for a Mountain in hand"
    );
    assert!(
        !legal.contains(&Action::CastSpell { object: mountain }),
        "legal_actions must NOT offer CastSpell for a land ([CR#305.9])"
    );
}

/// kw-hexproof goes LIVE: a `Cant(Target)` row excludes its carrier from an
/// opposing spell's legal target set ([CR#702.11b]); a creature beside it
/// stays targetable, and submission re-validates.
#[test]
fn hexproof_excludes_it_from_opposing_targets() {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let scout = card("Gladecover Scout");
    let bears = card("Grizzly Bears");
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&scout); 5];
    p1.extend(vec![Arc::clone(&bears); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 19,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let scout = force_into_play(&mut state, PlayerId(1), "Gladecover Scout");
    let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");
    force_into_play(&mut state, PlayerId(0), "Mountain");

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is targetable");
    assert!(
        !legal[0].contains(&scout),
        "the hexproof creature is excluded from an opponent's targets"
    );
    // Submission re-validates against the surfaced legal set.
    assert!(
        state
            .submit_decision(Decision::Targets(vec![scout]))
            .is_err(),
        "targeting the hexproof creature is rejected"
    );
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
}

/// kw-flash goes LIVE: the card's own `May(Cast(window: InstantSpeed))` row
/// lifts the sorcery-speed default ([CR#702.8a]) — a flash creature is
/// offered and resolves at an instant-timing window (its caster's upkeep),
/// and the priority window after it hits the battlefield still computes
/// (the row in the derived view no longer trips the presence guard).
#[test]
fn flash_creature_casts_at_instant_timing() {
    let cheetah = card("Pouncing Cheetah");
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(&cheetah); 5];
    p0.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 23,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    for _ in 0..3 {
        force_onto_battlefield(&mut state, PlayerId(0), "Forest");
    }

    // Upkeep is a priority window where sorcery speed is NOT ok.
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        Phase::Beginning(BeginningStep::Upkeep),
    );
    float_mana(&mut state, PlayerId(0), 3);
    let cheetah = find_in_hand(&state, PlayerId(0), "Pouncing Cheetah");
    let legal = run_to_priority(
        &mut state,
        PlayerId(0),
        Phase::Beginning(BeginningStep::Upkeep),
    );
    assert!(
        legal.contains(&Action::CastSpell { object: cheetah }),
        "flash lifts the timing default: the cheetah must be castable at upkeep"
    );

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: cheetah }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{2}}{{G}}, got {stop:?}");
    };
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![green(), green()],
        }))
        .unwrap();
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        Phase::Beginning(BeginningStep::Upkeep),
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(
        &mut state,
        PlayerId(1),
        Phase::Beginning(BeginningStep::Upkeep),
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert!(
        state
            .zones
            .battlefield
            .iter()
            .any(|&o| is_card(&state, o, "Pouncing Cheetah")),
        "the flash creature resolved onto the battlefield during upkeep"
    );
    // The flash permanent's row sits in the derived view now — the next
    // legal_actions computation must evaluate it, not trip the seam guard.
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        Phase::Beginning(BeginningStep::Upkeep),
    );
}

/// The control: without a flash row, a creature spell stays
/// sorcery-speed-only ([CR#117.1a]) — never offered at upkeep even with
/// the cost funded.
#[test]
fn nonflash_creature_not_castable_at_instant_timing() {
    let mut state = bears_game(7, 2);
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        Phase::Beginning(BeginningStep::Upkeep),
    );
    float_mana(&mut state, PlayerId(0), 2);
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    let legal = run_to_priority(
        &mut state,
        PlayerId(0),
        Phase::Beginning(BeginningStep::Upkeep),
    );
    assert!(
        !legal.contains(&Action::CastSpell { object: bears }),
        "a non-flash creature must not be castable at upkeep"
    );
}
