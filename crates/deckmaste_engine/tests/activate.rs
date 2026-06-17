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
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Effect;
use deckmaste_core::Filter;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Phase;
use deckmaste_core::PlayerAction;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::StateFilter;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::CostOptionChoices;
use deckmaste_engine::Decision;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StackObject;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::SymbolChoice;
use deckmaste_engine::WorkItem;

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

fn red() -> ColorOrColorless {
    Color::Red.into()
}

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
/// When a `PayMana` decision surfaces mid-announce, this function auto-taps it
/// (via the engine's canonical `auto_pay_pending`) and continues. Tests that
/// need a *specific* allocation must answer that `PayMana` explicitly before
/// calling this helper.
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
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
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
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();

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
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();
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
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();
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

// --- non-mana cost payment ([CR#601.2h]) ------------------------------------

/// A nondescript no-target effect: gain 0 life. Resolving it mutates nothing,
/// so a test can isolate the *cost* being performed from the effect.
fn gain_zero() -> Effect {
    Effect::Act(CoreAction::By(
        Reference::You,
        PlayerAction::GainLife(Count::Literal(0)),
    ))
}

/// An artifact whose sole ability is an activated ability with the given cost
/// and a harmless no-target effect. Built in-Rust so a test pins the exact
/// cost components; `Cards::push` derives its printed abilities like any card.
fn artifact_with_cost(name: &str, cost: Vec<CostComponent>) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: name.into(),
        mana_cost: ManaCost::from(vec![]),
        color_indicator: vec![],
        supertypes: vec![],
        types: vec![Type::Artifact],
        subtypes: vec![],
        abilities: vec![Ability::Activated(ActivatedAbility {
            from: None,
            window: None,
            cost: cost.into(),
            condition: None,
            limits: vec![],
            effect: gain_zero(),
        })],
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    }))
}

/// Builds a two-player game whose player-0 deck is five copies of `card` plus
/// five Mountains (so the opening hand and library are well-formed), nothing
/// special for player 1. Mirrors `activation_game` but seeds an arbitrary
/// in-Rust card.
fn cost_game(seed: u64, card: &Arc<Card>) -> GameState {
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(card); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let p1 = vec![forest; 10];
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

/// Activates `object`'s only ability at the current P0 priority window, then
/// answers the `PayMana` that the `{0}` mana component surfaces. Leaves the
/// engine at whatever stop follows payment.
///
/// [CR#118.5,118.5a]: a `{0}` cost is a placeholder payable with nothing, but
/// the ability is NOT paid automatically — the action still requires the
/// player's acknowledgment. The engine models that acknowledgment as a real
/// `PayMana` decision carrying the (zero) cost, answered here via `auto_pay`.
/// Every `artifact_with_cost` test below relies on this `{0}`-surfaces-PayMana
/// contract, so this asserts it once for all of them.
fn activate_and_pay_zero(state: &mut GameState, object: ObjectId) {
    let legal = run_to_priority(state, PlayerId(0), Phase::PrecombatMain);
    let activate =
        activate_action(&legal, object).expect("the in-Rust ability is offered at priority");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // No targets: the next stop is the `{0}` PayMana — the acknowledgment that
    // the cost is being paid ([CR#118.5]), even though it requires no resources.
    let (_, stop) = step_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
        panic!("expected PayMana for the {{0}} cost, got {stop:?}");
    };
    assert_eq!(
        cost,
        "{0}".parse().unwrap(),
        "[CR#118.5]: a {{0}} cost still surfaces a (zero) PayMana — not auto-paid"
    );
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();
}

/// [CR#601.2h]: a `Do(Sacrifice(This))` cost is PERFORMED in the payment
/// window — after the mana payment, before the ability becomes activated. The
/// source permanent leaves the battlefield for its owner's graveyard.
#[test]
fn activated_ability_pays_self_sacrifice_cost() {
    const NAME: &str = "Sacrifice-cost test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::Do(Box::new(PlayerAction::Sacrifice(Selection::Ref(
                Reference::This,
            )))),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    activate_and_pay_zero(&mut state, obj);

    // Drive to the next priority: the sacrifice cost must have fired during
    // payment, so the source is gone from the battlefield.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        !state.zones.battlefield.contains(&obj),
        "the self-sacrifice cost removed the source from the battlefield"
    );
    assert!(
        state.zones.graveyards[0].iter().any(|&o| {
            state
                .objects
                .obj(o)
                .card_id()
                .is_some_and(|_| face_name(&state, o) == NAME)
        }),
        "the sacrificed permanent is in its owner's graveyard, gy: {:?}",
        state.zones.graveyards[0]
    );
}

/// [CR#601.2h,119.4]: a `Do(LoseLife(2))` cost is PERFORMED in the payment
/// window — the controller's life drops by exactly 2.
#[test]
fn activated_ability_pays_life_cost() {
    const NAME: &str = "Life-cost test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::Do(Box::new(PlayerAction::LoseLife(Count::Literal(2)))),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    let life_before = state.players[0].life;
    activate_and_pay_zero(&mut state, obj);
    // Capture the trace from just after payment to the next priority: the
    // `LifeLost` cost must occur BEFORE the `AbilityActivated` "becomes
    // activated" step ([CR#601.2h] precedes [CR#601.2i]).
    let (trace, _) = step_to_stop(&mut state);
    let life_idx = trace.iter().position(|p| {
        matches!(
            applied(p),
            Some(GameEvent::LifeLost { player, amount: 2 }) if *player == PlayerId(0)
        )
    });
    let activated_idx = trace.iter().position(|p| {
        matches!(applied(p), Some(GameEvent::AbilityActivated { source, .. }) if *source == obj)
    });
    let life_idx = life_idx.unwrap_or_else(|| panic!("a LifeLost(2) cost event, trace: {trace:?}"));
    let activated_idx =
        activated_idx.unwrap_or_else(|| panic!("an AbilityActivated event, trace: {trace:?}"));
    assert!(
        life_idx < activated_idx,
        "the LoseLife cost ([CR#601.2h]) runs before the ability becomes activated ([CR#601.2i]); \
         life@{life_idx} activated@{activated_idx}, trace: {trace:?}"
    );

    assert_eq!(
        state.players[0].life,
        life_before - 2,
        "the LoseLife(2) cost dropped the controller's life by exactly 2"
    );
}

/// [CR#601.2h,608.2d]: a `Do(Sacrifice(Choose(1, creature)))` cost surfaces a
/// `ChooseObjects` decision during payment; the chosen creature is sacrificed
/// (leaving the battlefield for its owner's graveyard) and the other is left
/// untouched.
#[test]
fn activated_ability_pays_choose_sacrifice_cost() {
    const ARTIFACT_NAME: &str = "Choose-sacrifice test artifact";
    // Creature filter: battlefield creatures (zone check + type check).
    let creature_filter = Filter::AllOf(vec![
        Filter::State(StateFilter::InZone(Zone::Battlefield)),
        Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
    ]);
    let card = artifact_with_cost(
        ARTIFACT_NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::Do(Box::new(PlayerAction::Sacrifice(Selection::Choose(
                Quantity::Exactly(Count::Literal(1)),
                creature_filter,
            )))),
        ],
    );

    // Build a game: player 0 gets the artifact + mountains + Grizzly Bears,
    // player 1 gets forests (no creatures so candidates are unambiguously P0's).
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
    let bears_card = Arc::new(canon().card(BEARS).unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(&card); 3];
    p0.extend(vec![Arc::clone(&bears_card); 4]);
    p0.extend(vec![Arc::clone(&mountain); 3]);
    let p1 = vec![forest; 10];
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });

    // Force the artifact and two distinct bears onto the battlefield.
    let artifact = force_into_play(&mut state, PlayerId(0), ARTIFACT_NAME);
    let bear_a = force_into_play(&mut state, PlayerId(0), BEARS);
    let bear_b = force_into_play(&mut state, PlayerId(0), BEARS);
    // The two bears must be distinct objects.
    assert_ne!(
        bear_a, bear_b,
        "two distinct Grizzly Bears are on the battlefield"
    );

    // Activate and pay the {0} mana component; leaves engine waiting after payment.
    activate_and_pay_zero(&mut state, artifact);

    // The next stop must be ChooseObjects for the sacrifice-a-creature choice.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects {
        player,
        ref candidates,
        min,
        max,
    }) = stop
    else {
        panic!("expected ChooseObjects for the sacrifice-creature cost, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the activating player chooses");
    assert_eq!((min, max), (1, 1), "exactly one creature must be chosen");
    assert!(
        candidates.contains(&bear_a),
        "bear_a is a candidate, candidates: {candidates:?}"
    );
    assert!(
        candidates.contains(&bear_b),
        "bear_b is a candidate, candidates: {candidates:?}"
    );

    // Choose bear_a to sacrifice.
    state
        .submit_decision(Decision::Chosen(vec![bear_a]))
        .unwrap();

    // Drive to the next priority window: the sacrifice cost must fire during
    // payment so bear_a is already gone before the ability is on the stack.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // bear_a was sacrificed: its original id is no longer on the battlefield.
    assert!(
        !state.zones.battlefield.contains(&bear_a),
        "bear_a was sacrificed and is no longer on the battlefield"
    );
    // At least one object with the Bear's name is in P0's graveyard (the reminted
    // id).
    assert!(
        state.zones.graveyards[0].iter().any(|&o| {
            state
                .objects
                .obj(o)
                .card_id()
                .is_some_and(|_| face_name(&state, o) == BEARS)
        }),
        "a Grizzly Bears is in P0's graveyard after the sacrifice, gy: {:?}",
        state.zones.graveyards[0]
    );
    // bear_b was NOT chosen: it must still be on the battlefield.
    assert!(
        state.zones.battlefield.contains(&bear_b),
        "bear_b was not chosen and must still be on the battlefield"
    );

    // Resolve the ability (it has a no-op gain_zero() effect).
    assert_eq!(
        state.stack.len(),
        1,
        "the activated ability is on the stack"
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), Phase::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert!(state.stack.is_empty(), "the ability resolved cleanly");
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

// --- hybrid / Phyrexian concretization ([CR#601.2b]) -------------------------
//
// These drive the announce flow for an activated ability whose printed mana
// cost carries a hybrid or Phyrexian symbol. The affordability gate that
// decides whether such an ability is *offered* (`can_activate` over the
// concretized readings) is a separate, later task, so these schedule the
// announce block directly onto the agenda — the same direct-scheduling
// technique `skeleton.rs` uses to exercise `BeginCast`/`Resolve` without the
// legality gate. What is under test is the WIRING: a `ChooseCostOptions`
// decision surfaces at [CR#601.2b] (between targets and payment), and
// `pay_cost` consumes the player's announced reading.

fn white() -> ColorOrColorless {
    Color::White.into()
}
fn blue() -> ColorOrColorless {
    Color::Blue.into()
}

/// Schedules the [CR#602.2b] activation announce block for `object`'s ability
/// `index` straight onto the agenda front, mirroring `take_priority_action`'s
/// `ActivateAbility` arm (minus the priority bookkeeping). Bypasses the
/// `can_activate` legality gate so a hybrid/Phyrexian cost — not yet affordable
/// to the offer gate (a later task) — still reaches the announce flow.
///
/// `float` is added to P0's pool AFTER reaching the precombat-main priority
/// window (the pool empties at every step end, [CR#500.5]) so the injected
/// payment window can spend it.
fn schedule_activation(
    state: &mut GameState,
    object: ObjectId,
    index: usize,
    float: &[(ColorOrColorless, Uint)],
) {
    // Advance the game naturally to P0's precombat-main priority (the same
    // window the legality gate would offer the activation in), so the trailing
    // `OpenPriority` re-surfaces priority in the expected phase.
    let _ = run_to_priority(state, PlayerId(0), Phase::PrecombatMain);
    // Float the cost's mana now — after the walk, in the window that pays it.
    for &(color, amount) in float {
        state.player_mut(PlayerId(0)).mana_pool.add(color, amount);
    }
    // Clear the Priority decision `run_to_priority` stopped at; the announce
    // block front-runs the next priority open. (`consecutive_passes` is NOT
    // reset.)
    state.pending = None;
    // The full [CR#602.2b] announce block, in order: BeginActivate →
    // AnnounceTargets → ChooseCostOptions → PayCost → AbilityActivated → SBAs
    // → triggers → priority. Pushed back-to-front so the front-of-agenda order
    // is left-to-right.
    let block = [
        WorkItem::BeginActivate {
            object,
            ability: index,
        },
        WorkItem::AnnounceTargets,
        WorkItem::ChooseCostOptions,
        WorkItem::PayCost,
        WorkItem::Emit(Occurrence::single(GameEvent::AbilityActivated {
            source: object,
            ability: index,
        })),
        WorkItem::CheckSbas,
        WorkItem::PlaceTriggers,
        WorkItem::OpenPriority,
    ];
    for item in block.into_iter().rev() {
        state.agenda.push_front(item);
    }
}

/// [CR#601.2b]: an ability with a hybrid `{W/U}` mana cost surfaces a
/// `ChooseCostOptions` decision between announce and payment; picking the blue
/// reading makes the subsequent `PayMana` cost `{U}`, and a blue unit is spent.
#[test]
fn activated_ability_hybrid_picks_a_color() {
    const NAME: &str = "Hybrid-cost test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{W/U}".parse().unwrap())]);
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    // Float one of each color so whichever reading is picked is payable.
    schedule_activation(&mut state, obj, 0, &[(white(), 1), (blue(), 1)]);

    // No targets: the next stop is the ChooseCostOptions decision.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseCostOptions {
        player,
        cost,
        options,
    }) = stop
    else {
        panic!("expected ChooseCostOptions for the {{W/U}} cost, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the activating player announces");
    assert_eq!(
        cost,
        "{W/U}".parse().unwrap(),
        "the decision carries the printed hybrid cost"
    );
    assert_eq!(options.options.len(), 1, "one choosable symbol, the hybrid");

    // Pick the blue reading.
    state
        .submit_decision(Decision::CostOptions(CostOptionChoices {
            picks: vec![SymbolChoice::Mana(SimpleManaSymbol::Specific(
                Color::Blue.into(),
            ))],
        }))
        .unwrap();

    // PayMana now carries the CONCRETE {U}, not the printed {W/U}.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
        panic!("expected PayMana for the concretized {{U}}, got {stop:?}");
    };
    assert_eq!(
        cost,
        "{U}".parse().unwrap(),
        "the concretized cost is {{U}}, not the printed {{W/U}}"
    );
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // The blue unit was spent on {U}; the white unit is untouched.
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(blue()),
        0,
        "the blue unit paid the concretized {{U}}"
    );
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(white()),
        1,
        "the white unit was not spent — the blue reading was chosen"
    );
    assert_eq!(state.stack.len(), 1, "the ability reached the stack");
}

/// [CR#107.4f]: an ability with a Phyrexian `{W/P}` mana cost, paid with life,
/// requires NO mana (no `PayMana` surfaces) and drops the controller's life by
/// 2 in the payment window.
#[test]
fn activated_ability_phyrexian_pays_life() {
    const NAME: &str = "Phyrexian-cost test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{W/P}".parse().unwrap())]);
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let life_before = state.players[0].life;

    schedule_activation(&mut state, obj, 0, &[]);

    // The ChooseCostOptions decision surfaces; the Phyrexian symbol offers
    // [Mana(W), Life].
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseCostOptions { options, .. }) = stop
    else {
        panic!("expected ChooseCostOptions for the {{W/P}} cost, got {stop:?}");
    };
    assert!(
        options.options[0].choices.contains(&SymbolChoice::Life),
        "the Phyrexian symbol offers the Life reading, options: {options:?}"
    );

    // Pay 2 life.
    state
        .submit_decision(Decision::CostOptions(CostOptionChoices {
            picks: vec![SymbolChoice::Life],
        }))
        .unwrap();

    // No mana is required: the concretized cost is empty, so NO PayMana
    // surfaces. The life-loss cost ([CR#601.2h]) fires before the ability
    // becomes activated ([CR#601.2i]).
    let (trace, _) = step_to_stop(&mut state);
    let life_idx = trace.iter().position(|p| {
        matches!(
            applied(p),
            Some(GameEvent::LifeLost { player, amount: 2 }) if *player == PlayerId(0)
        )
    });
    let activated_idx = trace.iter().position(|p| {
        matches!(applied(p), Some(GameEvent::AbilityActivated { source, .. }) if *source == obj)
    });
    assert!(
        !trace.iter().any(|p| matches!(p, Progress::CostPaid)
            && matches!(state.pending, Some(PendingDecision::PayMana { .. }))),
        "no PayMana decision should surface for a fully-life Phyrexian cost"
    );
    let life_idx = life_idx.unwrap_or_else(|| panic!("a LifeLost(2) cost event, trace: {trace:?}"));
    let activated_idx =
        activated_idx.unwrap_or_else(|| panic!("an AbilityActivated event, trace: {trace:?}"));
    assert!(
        life_idx < activated_idx,
        "the Phyrexian life cost runs before the ability becomes activated; \
         life@{life_idx} activated@{activated_idx}, trace: {trace:?}"
    );
    assert_eq!(
        state.players[0].life,
        life_before - 2,
        "the Phyrexian-life reading dropped the controller's life by 2"
    );
}

/// [CR#107.4e]: a monohybrid `{2/W}` cost, paid via the generic half, requires
/// 2 generic mana; two units are spent.
#[test]
fn activated_ability_monohybrid_picks_generic() {
    const NAME: &str = "Monohybrid-cost test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{2/W}".parse().unwrap())]);
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    // Float two red units to pay the generic half.
    schedule_activation(&mut state, obj, 0, &[(red(), 2)]);

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseCostOptions { options, .. }) = stop
    else {
        panic!("expected ChooseCostOptions for the {{2/W}} cost, got {stop:?}");
    };
    assert_eq!(
        options.options[0].choices,
        vec![
            SymbolChoice::Mana(SimpleManaSymbol::Generic(2)),
            SymbolChoice::Mana(SimpleManaSymbol::Specific(Color::White.into())),
        ],
        "the monohybrid offers its generic and white halves"
    );

    // Pick the generic-2 half.
    state
        .submit_decision(Decision::CostOptions(CostOptionChoices {
            picks: vec![SymbolChoice::Mana(SimpleManaSymbol::Generic(2))],
        }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
        panic!("expected PayMana for the concretized {{2}}, got {stop:?}");
    };
    assert_eq!(
        cost,
        "{2}".parse().unwrap(),
        "the generic half concretizes to {{2}}"
    );
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(red()),
        0,
        "both red units paid the concretized {{2}}"
    );
    assert_eq!(state.stack.len(), 1, "the ability reached the stack");
}

/// A no-choice (plain) cost still flows through the new `ChooseCostOptions`
/// step transparently: no decision surfaces, and the printed cost reaches
/// `PayMana` unchanged.
#[test]
fn activated_ability_plain_cost_skips_choose_cost_options() {
    const NAME: &str = "Plain-cost test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{1}".parse().unwrap())]);
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    schedule_activation(&mut state, obj, 0, &[(red(), 1)]);

    // No hybrid/Phyrexian symbol: ChooseCostOptions surfaces NOTHING; the next
    // stop is PayMana with the printed {1}.
    let (trace, stop) = step_to_stop(&mut state);
    assert!(
        !trace
            .iter()
            .any(|p| matches!(p, Progress::CostOptionsChosen { surfaced: true })),
        "a plain cost surfaces no ChooseCostOptions decision, trace: {trace:?}"
    );
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
        panic!("expected PayMana for the plain {{1}} cost, got {stop:?}");
    };
    assert_eq!(
        cost,
        "{1}".parse().unwrap(),
        "the printed {{1}} is unchanged"
    );
}

// --- hybrid / Phyrexian AFFORDABILITY GATE ([CR#601.2b,601.2g,107.4e,107.4f])
//
// These pin Task 2.4: a hybrid/Phyrexian cost is OFFERED by `can_activate`
// (→ `legal_actions`) iff SOME legal reading is fully payable. Unlike the
// 2.2/2.3 wiring tests above (which bypass the gate via `schedule_activation`),
// these drive the real gate: reach a priority window, float a pool, and assert
// whether the ability is in the legal list.

fn green() -> ColorOrColorless {
    Color::Green.into()
}

/// Floats `pool` mana into P0's pool at a freshly re-surfaced precombat-main
/// priority and returns the recomputed legal list (with the float reflected).
/// The pool empties each step end ([CR#500.5]), so the float happens in the
/// same window the gate is evaluated.
fn legal_with_float(state: &mut GameState, pool: &[(ColorOrColorless, Uint)]) -> Vec<Action> {
    let _ = run_to_priority(state, PlayerId(0), Phase::PrecombatMain);
    for &(color, amount) in pool {
        state.player_mut(PlayerId(0)).mana_pool.add(color, amount);
    }
    // Re-derive priority so the freshly floated pool is reflected in the list
    // (mirrors x_costs.rs's resurface_priority).
    assert!(
        matches!(state.pending, Some(PendingDecision::Priority { .. })),
        "expected a Priority decision to resurface"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    run_to_priority(state, PlayerId(0), Phase::PrecombatMain)
}

/// [CR#107.4e,601.2g]: a hybrid `{W/U}` ability is OFFERED when only the blue
/// reading is payable (one blue unit, no white) — the gate must find the
/// affordable reading — and NOT offered when neither reading is payable.
#[test]
fn hybrid_ability_offered_when_one_reading_payable() {
    const NAME: &str = "Hybrid-gate test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{W/U}".parse().unwrap())]);

    // Only blue available: the {U} reading is payable -> offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[(blue(), 1)]);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/U}} is activatable with only blue mana (pick U), legal: {legal:?}"
    );

    // No mana at all: neither {W} nor {U} reading is payable -> not offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        activate_action(&legal, obj).is_none(),
        "{{W/U}} is NOT activatable with no payable reading, legal: {legal:?}"
    );
}

/// [CR#107.4f,601.2g]: a Phyrexian `{W/P}` ability is OFFERED with NO mana but
/// life ≥ 2 (pay 2 life), and NOT offered at 1 life with no white (neither
/// reading payable).
#[test]
fn phyrexian_ability_offered_via_life() {
    const NAME: &str = "Phyrexian-gate test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{W/P}".parse().unwrap())]);

    // No mana, plenty of life: the Life reading is payable -> offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        state.players[0].life >= 2,
        "the starting life funds the 2-life reading"
    );
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/P}} is activatable via the 2-life reading with no mana, legal: {legal:?}"
    );

    // 1 life, no white: the Life reading needs 2 life and the {W} reading needs
    // white -> neither payable -> not offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 1;
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, obj).is_none(),
        "{{W/P}} is NOT activatable at 1 life with no white, legal: {legal:?}"
    );
}

/// SHARED-RESOURCE correctness ([CR#107.4f]): two Phyrexian `{W/P}{W/P}` with
/// exactly 2 life and no white is NOT castable — only ONE symbol can be paid by
/// life (2 of the 2 life), the other needs white, which is absent. But 2 life
/// plus ONE white IS payable (one life, one white). A naive per-symbol greedy
/// that lets each Phyrexian independently "see" the full 2 life would wrongly
/// offer the no-white case.
#[test]
fn two_phyrexian_share_life_correctly() {
    const NAME: &str = "Double-Phyrexian-gate test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![CostComponent::Mana("{W/P}{W/P}".parse().unwrap())],
    );

    // 2 life, no white: cannot pay both via life (needs 4); cannot pay either
    // via {W} (no white) -> not offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 2;
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, obj).is_none(),
        "{{W/P}}{{W/P}} at 2 life + no white is NOT activatable (only one life payment fits), \
         legal: {legal:?}"
    );

    // 2 life + ONE white: pay one symbol with the white, the other with 2 life
    // -> offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 2;
    state.player_mut(PlayerId(0)).mana_pool.add(white(), 1);
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/P}}{{W/P}} at 2 life + one white IS activatable (one life, one white), legal: {legal:?}"
    );
}

/// Regression: a plain `{1}` cost is gated exactly as before — offered with one
/// mana, not offered with none. The no-choosable-symbol path must keep the
/// existing `can_pay` behavior unchanged.
#[test]
fn plain_cost_gate_unchanged() {
    const NAME: &str = "Plain-gate test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{1}".parse().unwrap())]);

    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[(green(), 1)]);
    assert!(
        activate_action(&legal, obj).is_some(),
        "plain {{1}} is offered with one mana, legal: {legal:?}"
    );

    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        activate_action(&legal, obj).is_none(),
        "plain {{1}} is NOT offered with an empty pool, legal: {legal:?}"
    );
}

/// Composition with engine-x-costs: a cost bearing BOTH `{X}` and a hybrid
/// (`{X}{W/U}`) is still gated on the non-X part — `{X}` reduces to 0 (X never
/// blocks, [CR#107.3a]), and the hybrid's affordable reading decides. Offered
/// with one blue; not offered with an empty pool.
#[test]
fn x_plus_hybrid_gated_on_hybrid() {
    const NAME: &str = "X-plus-hybrid gate test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{X}{W/U}".parse().unwrap())]);

    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[(blue(), 1)]);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{X}}{{W/U}} is offered at X=0 with blue (pick U), legal: {legal:?}"
    );

    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        activate_action(&legal, obj).is_none(),
        "{{X}}{{W/U}} is NOT offered with no payable hybrid reading, legal: {legal:?}"
    );
}

/// MERGE-COMPOSITION ([CR#107.3a,107.4e,601.2b]): a cost bearing BOTH `{X}` and
/// a hybrid (`{X}{W/U}`) drives the full announce flow `engine-x-costs` and the
/// hybrid concretization compose through. The two concretizers run at different
/// steps — `AnnounceX` records X, `ChooseCostOptions` picks the hybrid reading
/// (passing `{X}` through untouched), then `PayCost` applies `concretize_x` to
/// the residual `{X}`. With X=2 and the blue reading picked, the `PayMana` cost
/// must be the composed `{2}{U}` (X→`Generic(2)`, `{W/U}`→`{U}`), and exactly
/// the right two units (one blue + two generic-payable) are spent. The gate
/// also OFFERS the ability when the hybrid is affordable ([CR#601.2g]).
#[test]
fn x_plus_hybrid_announces_x_concretizes_hybrid_pays_composed_cost() {
    const NAME: &str = "X-plus-hybrid e2e test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{X}{W/U}".parse().unwrap())]);
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    // Float one blue (for the {W/U}→{U} reading) plus two greens (generic-
    // payable, to fund X=2). `legal_with_float` floats AFTER reaching the
    // precombat-main priority window (the pool empties at every step end,
    // [CR#500.5]) and re-derives the legal list with the float reflected. The
    // gate is X-reduced ([CR#107.3a]), so the offer hangs on the hybrid alone.
    let legal = legal_with_float(&mut state, &[(blue(), 1), (green(), 2)]);
    let activate = activate_action(&legal, obj)
        .expect("the {X}{W/U} ability is OFFERED when the hybrid reading is payable");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // [CR#601.2b]: X is announced FIRST (engine-x-costs' `AnnounceX` step).
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseXValue { player }) = stop else {
        panic!("expected ChooseXValue first, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(2)).unwrap();

    // [CR#601.2b]: ...then ChooseCostOptions for the hybrid. The carried cost is
    // the printed {X}{W/U}; only the hybrid symbol is choosable (X passes
    // through). Pick the blue reading.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseCostOptions {
        player,
        cost,
        options,
    }) = stop
    else {
        panic!("expected ChooseCostOptions after X, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert_eq!(
        cost,
        "{X}{W/U}".parse().unwrap(),
        "the decision carries the printed {{X}}{{W/U}} cost"
    );
    assert_eq!(
        options.options.len(),
        1,
        "only the hybrid symbol is choosable; {{X}} is announced separately"
    );
    state
        .submit_decision(Decision::CostOptions(CostOptionChoices {
            picks: vec![SymbolChoice::Mana(SimpleManaSymbol::Specific(
                Color::Blue.into(),
            ))],
        }))
        .unwrap();

    // PayCost composes the two concretizers: the hybrid is already {U}, and
    // `concretize_x` turns the residual {X} into Generic(2). The PayMana cost
    // must be the composed {2}{U}.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = stop else {
        panic!("expected PayMana for the composed {{2}}{{U}}, got {stop:?}");
    };
    assert_eq!(
        cost,
        "{2}{U}".parse().unwrap(),
        "X→Generic(2) ∘ hybrid→{{U}} composes to {{2}}{{U}}, not the printed {{X}}{{W/U}}"
    );
    let pay = state.auto_pay_pending();
    state.submit_decision(Decision::Pay(pay)).unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // The blue unit paid {U}; the two greens paid the {2}. Nothing is left over.
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(blue()),
        0,
        "the blue unit paid the concretized {{U}}"
    );
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(green()),
        0,
        "the two green units paid the X=2 generic"
    );
    assert_eq!(
        state.stack.len(),
        1,
        "the ability reached the stack after the composed cost was paid"
    );
}

// --- the cast gate (`can_cast`) over hybrid/Phyrexian -----------------------

/// An instant whose only distinction is its printed (hybrid/Phyrexian) mana
/// cost and a harmless no-target effect — so `can_cast`'s affordability gate is
/// what decides whether `CastSpell` is offered. Instant timing keeps it
/// castable at any priority.
fn instant_with_cost(name: &str, cost: ManaCost) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: name.into(),
        mana_cost: cost,
        color_indicator: vec![],
        supertypes: vec![],
        types: vec![Type::Instant],
        subtypes: vec![],
        abilities: vec![],
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    }))
}

/// The `CastSpell` action for `object` in `legal`, if offered.
fn cast_action(legal: &[Action], object: ObjectId) -> Option<Action> {
    legal
        .iter()
        .find(|a| matches!(a, Action::CastSpell { object: o } if *o == object))
        .cloned()
}

/// [CR#601.2g,107.4e]: `can_cast` offers a hybrid `{W/U}` instant with only
/// blue mana (the {U} reading is payable) and withholds it with an empty pool.
#[test]
fn cast_gate_hybrid_offered_when_one_reading_payable() {
    const NAME: &str = "Hybrid-cost instant";
    let card = instant_with_cost(NAME, "{W/U}".parse().unwrap());

    let mut state = cost_game(7, &card);
    let spell = find_in_hand(&state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[(blue(), 1)]);
    assert!(
        cast_action(&legal, spell).is_some(),
        "{{W/U}} instant is castable with only blue mana (pick U), legal: {legal:?}"
    );

    let mut state = cost_game(7, &card);
    let spell = find_in_hand(&state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        cast_action(&legal, spell).is_none(),
        "{{W/U}} instant is NOT castable with no payable reading, legal: {legal:?}"
    );
}

/// [CR#601.2g,107.4f]: `can_cast` offers a Phyrexian `{W/P}` instant with NO
/// mana but life ≥ 2 (the 2-life reading), and withholds it at 1 life with no
/// white.
#[test]
fn cast_gate_phyrexian_offered_via_life() {
    const NAME: &str = "Phyrexian-cost instant";
    let card = instant_with_cost(NAME, "{W/P}".parse().unwrap());

    let mut state = cost_game(7, &card);
    let spell = find_in_hand(&state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        cast_action(&legal, spell).is_some(),
        "{{W/P}} instant is castable via 2 life with no mana, legal: {legal:?}"
    );

    let mut state = cost_game(7, &card);
    let spell = find_in_hand(&state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 1;
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        cast_action(&legal, spell).is_none(),
        "{{W/P}} instant is NOT castable at 1 life with no white, legal: {legal:?}"
    );
}
