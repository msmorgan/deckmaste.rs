//! End-to-end non-mana activated abilities against testing-plugin mocks (the
//! activated permanents) and canon real cards (bystanders and removal),
//! driven entirely through the public API (`step` / `submit_decision`).
//!
//! Each test builds a two-player game from testing cards, forces the relevant
//! permanents into play, advances to a priority window via `step`, then
//! activates the way a UI would: pick the offered `ActivateAbility`, answer
//! `ChooseTargets` / `PayMana` as they surface, and `Pass` to resolve.

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Phase;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
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

const PINGER: &str = "Creature tap-activated DealDamage AnyTarget";
const MANA_DRAWER: &str = "Artifact mana-activated DrawCards";
const SORCERY_DRAWER: &str = "Artifact sorcery-speed DrawCards";
const TURN_DRAWER: &str = "Artifact once-per-turn DrawCards";
const GAME_DRAWER: &str = "Artifact once-per-game DrawCards";
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

/// Passes every decision (priorities, cleanup discards, combat declarations)
/// until player 0 holds priority in the precombat main of their own NEXT turn.
/// Returns the legal action list at that window.
fn advance_to_next_own_main(state: &mut GameState) -> Vec<Action> {
    let start_turn = state.turn.turn_number;
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::Priority { player, legal })
                if player == PlayerId(0)
                    && state.turn.active_player == PlayerId(0)
                    && state.turn.current == Phase::PrecombatMain
                    && state.turn.turn_number > start_turn =>
            {
                return legal;
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
            StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. }) => {
                state.submit_decision(Decision::Blocks(vec![])).unwrap();
            }
            other => panic!("unexpected stop while advancing a turn: {other:?}"),
        }
    }
}

/// The `ActivateAbility` action for `object` in `legal`, if offered.
fn activate_action(legal: &[Action], object: ObjectId) -> Option<Action> {
    legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object: o, .. } if *o == object))
        .cloned()
}

/// Extracts the `GameEvent` from a `Progress::Applied(Occurrence::Single(_))`,
/// returning `None` for any other variant.
fn applied(p: &Progress) -> Option<&GameEvent> {
    match p {
        Progress::Applied(Occurrence::Single(e)) => Some(e),
        _ => None,
    }
}

// --- tests --------------------------------------------------------------------

#[test]
fn tap_pinger_damages_target_through_stack() {
    let mut state = activation_game(1, PINGER, 0);
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    // Documents the precondition; the turn-start untap also clears the flag
    // for the active player's permanents ([CR#302.6]).
    state.objects.obj_mut(pinger).summoning_sick = false;
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);

    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let activate = activate_action(&legal, pinger).expect("the pinger's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // Announce: the target choice surfaces.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();

    // A mana-free cost: the very next stop is the activator's priority — no
    // PayMana surfaces; the queued Tapped event paid the {T}.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. }) = stop else {
        panic!("expected P0 priority with no PayMana for a mana-free cost, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert!(state.objects.obj(pinger).tapped, "the tap cost was paid");
    assert_eq!(state.stack.len(), 1, "the ability sits on the stack");
    let StackObject::Activated { source, .. } = &state.stack[0].object else {
        panic!(
            "expected an Activated entry, got {:?}",
            state.stack[0].object
        );
    };
    assert_eq!(*source, pinger);
    assert_eq!(state.stack[0].targets, vec![bear]);

    // Both players pass: the ability resolves and deals 1 to the bear.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);

    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::DamageDealt { target, amount: 1, .. }) if *target == bear
        )),
        "1 damage dealt to the bear, trace: {trace:?}"
    );
    assert_eq!(
        state.objects.obj(bear).damage,
        1,
        "1 marked on the 2/2 — it survives"
    );
    assert!(
        state.objects.obj(pinger).tapped,
        "still tapped after resolution"
    );
    assert!(
        state.stack.is_empty(),
        "the minted entry left the stack on resolution"
    );
}

/// [CR#602.2a]: an activated ability exists on the stack from the START of
/// its announcement — the announce slot carries a freshly minted
/// stack-zone identity (not the source standing in), and the committed
/// entry promotes under that same id. Announce-time deontic `by` rows
/// (hexproof-family, stack-zone-keyed shapes) evaluate against it.
#[test]
fn activation_announce_carries_a_minted_stack_identity() {
    let mut state = activation_game(3, PINGER, 0);
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    state.objects.obj_mut(pinger).summoning_sick = false;
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);

    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let activate = activate_action(&legal, pinger).expect("the pinger's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { .. }) = stop else {
        panic!("expected ChooseTargets, got {stop:?}");
    };

    let pending = state.announcing.as_ref().expect("an announce in flight");
    let minted = pending.id;
    assert_ne!(minted, pinger, "the stack identity is not the source");
    assert_eq!(
        state.objects.obj(minted).zone,
        Some(Zone::Stack),
        "the announce's identity is a stack-zone object from announcement"
    );

    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) = stop else {
        panic!("expected priority after the mana-free activation, got {stop:?}");
    };
    assert_eq!(
        state.stack[0].id, minted,
        "the committed entry promotes under the announce-time identity"
    );
}

#[test]
fn summoning_sick_pinger_not_offered() {
    let mut state = activation_game(1, PINGER, 0);
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);

    // Reach priority FIRST: the turn-start untap clears summoning sickness on
    // the active player's permanents ([CR#302.6]), which would wipe a
    // pre-step stamp on the forced pinger.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.objects.obj_mut(pinger).summoning_sick = true;
    // The pending legal list predates the stamp — take the offered land drop
    // to reopen priority with a freshly computed list.
    let land = legal
        .iter()
        .find(|a| matches!(a, Action::PlayLand { .. }))
        .cloned()
        .expect("a Mountain in hand for the land drop");
    state.submit_decision(Decision::Act(land)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, pinger).is_none(),
        "[CR#602.5a]: a summoning-sick creature cannot pay {{T}}, legal: {legal:?}"
    );
    // No false positives: the vanilla bear (no abilities, P1's anyway) never
    // shows up as activatable.
    assert!(
        activate_action(&legal, bear).is_none(),
        "the bear has nothing to activate, legal: {legal:?}"
    );
}

#[test]
fn artifact_pays_mana_ignores_sickness() {
    let mut state = activation_game(2, MANA_DRAWER, 2);
    let drawer = force_into_play(&mut state, PlayerId(0), MANA_DRAWER);

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // Stamp sickness AFTER the turn-start untap (which clears the flag on the
    // active player's permanents, [CR#302.6]): the [CR#602.5a] gate is
    // creature-and-{T}/{Q}-only, so a summoning-sick artifact with a pure mana
    // cost must still be offered. The floats below reopen priority, so the
    // final legal list is computed with the stamp in place.
    state.objects.obj_mut(drawer).summoning_sick = true;
    float_mana(&mut state, PlayerId(0), 2); // R, R
    let hand_before = state.zones.hands[0].len();

    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let activate = activate_action(&legal, drawer)
        .expect("a summoning-sick artifact's mana-only ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // No targets, so PayMana surfaces directly, carrying the {2} cost.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
        panic!("expected PayMana for {{2}}, got {stop:?}");
    };
    assert_eq!(
        cost,
        ManaCost::from(vec![ManaSymbol::Simple(SimpleManaSymbol::Generic(2))]),
        "the decision carries the ability's {{2}} cost"
    );
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![red(), red()],
        }))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the ability is on the stack");
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(
        state.zones.hands[0].len(),
        hand_before + 1,
        "the resolution drew a card"
    );
    assert!(state.stack.is_empty());
}

#[test]
fn sorcery_speed_drawer_gated() {
    // P0: instants (cast from hand) + sorcery-speed drawers + Mountains; the
    // drawer and two Mountains are pulled from the library, the instant from a
    // seed-searched opening hand. P1: bears (the instant's target) + Forests.
    let canon = canon();
    let bolt = Arc::new(canon.card(INSTANT).unwrap());
    let drawer_card = Arc::new(testing().card(SORCERY_DRAWER).unwrap());
    let bears = Arc::new(canon.card(BEARS).unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut deck0 = vec![Arc::clone(&bolt); 4];
    deck0.extend(vec![Arc::clone(&drawer_card); 3]);
    deck0.extend(vec![Arc::clone(&mountain); 5]);
    let mut deck1 = vec![Arc::clone(&bears); 4];
    deck1.extend(vec![Arc::clone(&forest); 8]);
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: deck1.clone(),
                },
            ],
            seed,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| s.zones.hands[0].iter().any(|&o| is_card(s, o, INSTANT)))
        .expect("a seed with an instant in P0's opening hand");
    let drawer = force_into_play(&mut state, PlayerId(0), SORCERY_DRAWER);
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);
    let instant = find_in_hand(&state, PlayerId(0), INSTANT);

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R

    // (a) Own precombat main, empty stack: the SorcerySpeed condition holds.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, drawer).is_some(),
        "offered at sorcery speed on an empty stack, legal: {legal:?}"
    );

    // (b) Cast the instant; while it sits on the stack, P0 regains priority
    //     and the drawer is gated — the SorcerySpeed stack census is nonzero.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: instant }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets for the instant, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    // run_to_priority auto-pays the all-colored {R}.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the instant is on the stack");
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(red()),
        1,
        "the {{1}} stays payable — only the timing differs"
    );
    assert!(
        activate_action(&legal, drawer).is_none(),
        "gated while the stack is non-empty (SorcerySpeed census != 0), legal: {legal:?}"
    );

    // Let the instant resolve; at the next clean main-phase priority the
    // drawer is offered again.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(state.stack.is_empty(), "the instant resolved");
    assert!(
        activate_action(&legal, drawer).is_some(),
        "offered again once the stack is empty, legal: {legal:?}"
    );
}

#[test]
fn once_per_turn_resets_next_turn() {
    let mut state = activation_game(3, TURN_DRAWER, 2);
    let drawer = force_into_play(&mut state, PlayerId(0), TURN_DRAWER);

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R
    let hand_before = state.zones.hands[0].len();

    // First activation this turn: offered; pay {1} with one red; resolve.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let activate = activate_action(&legal, drawer).expect("offered before any use this turn");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{1}}, got {stop:?}");
    };
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![red()],
        }))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert_eq!(
        state.zones.hands[0].len(),
        hand_before + 1,
        "the first use drew"
    );

    // Same turn, mana still floating: spent for the turn ([CR#602.5b]).
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(red()),
        1,
        "mana is not the gap"
    );
    assert!(
        activate_action(&legal, drawer).is_none(),
        "once-per-turn exhausted this turn, legal: {legal:?}"
    );

    // P0's next turn: the per-turn ledger flushed; offered again.
    let _ = advance_to_next_own_main(&mut state);
    float_mana(&mut state, PlayerId(0), 1);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, drawer).is_some(),
        "the once-per-turn limit resets on a new turn, legal: {legal:?}"
    );
}

#[test]
fn once_per_game_stays_spent() {
    let mut state = activation_game(3, GAME_DRAWER, 2);
    let drawer = force_into_play(&mut state, PlayerId(0), GAME_DRAWER);

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R
    let hand_before = state.zones.hands[0].len();

    // First (and only) activation: offered; resolve it.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let activate = activate_action(&legal, drawer).expect("offered before any use this game");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) = stop else {
        panic!("expected PayMana for {{1}}, got {stop:?}");
    };
    state
        .submit_decision(Decision::Pay(Payment {
            generic: vec![red()],
        }))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert_eq!(
        state.zones.hands[0].len(),
        hand_before + 1,
        "the only use drew"
    );

    // Same turn: spent.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, drawer).is_none(),
        "once-per-game exhausted, legal: {legal:?}"
    );

    // P0's next turn: the per-turn flush must NOT revive a per-GAME limit.
    let _ = advance_to_next_own_main(&mut state);
    float_mana(&mut state, PlayerId(0), 1);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(red()),
        1,
        "mana is not the gap"
    );
    assert!(
        activate_action(&legal, drawer).is_none(),
        "the game count survives the turn flush, legal: {legal:?}"
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "an end-to-end decision-driven scenario"
)]
fn pinger_fizzles_when_target_dies() {
    // P0: pingers + Mountains (mono halves). P1: instants + bears + Mountains
    // — the seed search puts an instant in P1's opening hand; the bear and a
    // Mountain are pulled from the library.
    let canon = canon();
    let pinger_card = Arc::new(testing().card(PINGER).unwrap());
    let bolt = Arc::new(canon.card(INSTANT).unwrap());
    let bears = Arc::new(canon.card(BEARS).unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let mut deck0 = vec![Arc::clone(&pinger_card); 5];
    deck0.extend(vec![Arc::clone(&mountain); 5]);
    let mut deck1 = vec![Arc::clone(&bolt); 4];
    deck1.extend(vec![Arc::clone(&bears); 3]);
    deck1.extend(vec![Arc::clone(&mountain); 5]);
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: deck1.clone(),
                },
            ],
            seed,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| s.zones.hands[1].iter().any(|&o| is_card(s, o, INSTANT)))
        .expect("a seed with an instant in P1's opening hand");
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    state.objects.obj_mut(pinger).summoning_sick = false;
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);
    force_into_play(&mut state, PlayerId(1), "Mountain");

    // P0 activates the pinger at P1's bear.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let activate = activate_action(&legal, pinger).expect("the pinger is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { .. }) = stop else {
        panic!("expected ChooseTargets for the pinger, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the ability is on the stack");

    // P0 passes; P1 responds: float {R} and kill their own bear with the
    // instant (3 damage on the 2/2 is lethal).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    float_mana(&mut state, PlayerId(1), 1);
    let instant = find_in_hand(&state, PlayerId(1), INSTANT);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: instant }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseTargets { legal, .. }) = stop else {
        panic!("expected ChooseTargets for the instant, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![bear]))
        .unwrap();
    // run_to_priority auto-pays the all-colored {R}.
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "the instant sits atop the ability");

    // Resolve LIFO, collecting every damage event: the instant kills the
    // bear, then the pinger's ability finds its only target gone and fizzles
    // ([CR#608.2b]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap(); // P1
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap(); // P0 → resolve
    let mut damage = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::DamageDealt { target, amount, .. },
            ))) => damage.push((target, amount)),
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                if state.stack.is_empty() {
                    // The game continues: a further priority decision arrived.
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while draining the stack: {other:?}"),
        }
    }

    // Exactly one damage event landed: the instant's 3 to the bear. The
    // fizzled ability dealt nothing to anyone.
    assert_eq!(
        damage,
        vec![(bear, 3)],
        "only the instant dealt damage; the fizzled ability dealt none"
    );
    assert_eq!(state.players[0].life, 20);
    assert_eq!(state.players[1].life, 20, "the fizzled ping never landed");
    assert!(
        state.objects.get(bear).is_none(),
        "the bear died and reminted away ([CR#400.7])"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        2,
        "P1's graveyard holds the reminted bear and P1's spent instant"
    );
    assert!(state.stack.is_empty());
    assert!(
        state.objects.obj(pinger).tapped,
        "the tap cost stays paid on a fizzle"
    );
}

#[test]
fn mana_ability_stays_stackless() {
    let mut state = activation_game(4, MANA_DRAWER, 1);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(state.player(PlayerId(0)).mana_pool.is_empty());

    // The only battlefield activatable is the Mountain's mana ability.
    let tap = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .cloned()
        .expect("the Mountain's mana ability is offered");
    state.submit_decision(Decision::Act(tap)).unwrap();

    // Drive to the next decision one step at a time: the whole activation is
    // stackless ([CR#605.3b]) — no ChooseTargets, no PayMana, no stack entry.
    let stop = loop {
        match state.step() {
            StepOutcome::Progress(_) => assert!(
                state.stack.is_empty(),
                "a mana ability never touches the stack"
            ),
            stop => break stop,
        }
    };
    let StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) = stop else {
        panic!("expected to return straight to priority, got {stop:?}");
    };
    assert!(state.stack.is_empty());
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(red()),
        1,
        "the pool gained one red"
    );
}
