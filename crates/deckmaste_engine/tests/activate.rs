//! End-to-end non-mana activated abilities against testing-plugin mocks (the
//! activated permanents) and canon real cards (bystanders and removal),
//! driven entirely through the public API (`step` / `submit_decision`).
//!
//! Each test builds a two-player game from testing cards, forces the relevant
//! permanents into play, advances to a priority window via `step`, then
//! activates the way a UI would: pick the offered `ActivateAbility`, answer
//! `ChooseTargets` / `Payment` as they surface, and `Pass` to resolve.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_card::Characteristics;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Instruction;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PhaseStep;
use deckmaste_core::Reference;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use deckmaste_engine::AbilityActivated;
use deckmaste_engine::Action;
use deckmaste_engine::CostOptionChoices;
use deckmaste_engine::DamageDealt;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::LifeLost;
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StackObject;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::SymbolChoice;
use deckmaste_engine::WorkItem;
use deckmaste_plugin::plugin::Plugin;

const PINGER: &str = "Creature tap-activated DealDamage AnyTarget";
const MANA_DRAWER: &str = "Artifact mana-activated DrawCards";
const SORCERY_DRAWER: &str = "Artifact sorcery-speed DrawCards";
const TURN_DRAWER: &str = "Artifact once-per-turn DrawCards";
const GAME_DRAWER: &str = "Artifact once-per-game DrawCards";
const INSTANT: &str = "Lightning Bolt";
const BEARS: &str = "Grizzly Bears";
const LOYALTY_PW: &str = "Planeswalker two loyalty abilities";

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
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.name,
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
            Card::Normal(f)
            | Card::DoubleFaced { front: f, .. }
            | Card::Split { left: f, .. }
            | Card::Flip { normal: f, .. }
            | Card::Adventurer { normal: f, .. } => f
                .characteristics
                .types
                .iter()
                .any(|t| t.name == Type::Land.name()),
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
    let card = Arc::new(testing.card(name).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&card); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
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

/// Drive an already-surfaced payment through the deterministic compatibility
/// path and stop at the next priority window, preserving the progress trace.
fn complete_pending_payment(state: &mut GameState) -> Vec<Progress> {
    let mut trace = Vec::new();
    loop {
        match state.pending.clone() {
            Some(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            Some(DecisionPointKind::ChooseManaReversals(choice)) => {
                let reversals = choice
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .unwrap();
            }
            Some(DecisionPointKind::Priority(_)) => return trace,
            other => panic!("expected payment or priority, got {other:?}"),
        }
        let (progress, stop) = step_to_stop(state);
        trace.extend(progress);
        if matches!(
            stop,
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_))
        ) {
            return trace;
        }
    }
}

/// Steps until a `Priority` decision surfaces for `player` in `phase`, passing
/// any other priority along the way. Returns the legal action list at that
/// window.
///
/// Payment prompts use the engine's deterministic monocolor compatibility
/// answer. Tests that need a specific transcript answer the protocol before
/// calling this helper.
fn run_to_priority(state: &mut GameState, player: PlayerId, phase: PhaseStep) -> Vec<Action> {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player: p, legal },
            )) if p == player && state.turn.current == phase => {
                return legal;
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::PayMana(deckmaste_engine::PayMana {
                ..
            })) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(choice)) => {
                let reversals = choice
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .unwrap();
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
        let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
            legal,
            ..
        })) = state.step()
        else {
            panic!("expected a priority decision to float mana");
        };
        let tap = legal
            .iter()
            .find(|a| {
                matches!(a, Action::ActivateAbility { object, .. }
                    if is_land(state, *object) && !state.objects.obj(*object).tapped)
            })
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
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player, legal },
            )) if player == PlayerId(0)
                && state.turn.active_player == PlayerId(0)
                && state.turn.current == PhaseStep::PrecombatMain
                && state.turn.turn_number > start_turn =>
            {
                return legal;
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::DiscardToHandSize(
                deckmaste_engine::DiscardToHandSize { player, count },
            )) => {
                let hand = state.zones.hands[player.index()].clone();
                let chosen: Vec<ObjectId> = hand.into_iter().take(count as usize).collect();
                state.submit_decision(Decision::Discard(chosen)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::DeclareAttackers(
                deckmaste_engine::DeclareAttackers { .. },
            )) => {
                state.submit_decision(Decision::Attackers(vec![])).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::DeclareBlockers(
                deckmaste_engine::DeclareBlockers { .. },
            )) => {
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

    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, pinger).expect("the pinger's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // Announce: the target choice surfaces.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();

    // The tap IOU is acknowledged through the explicit payment transaction;
    // the compatibility runner hides those micro-steps here.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(state.objects.obj(pinger).tapped, "the tap cost was paid");
    assert_eq!(state.stack.len(), 1, "the ability sits on the stack");
    let StackObject::Activated { source, .. } = &state.stack[0].object else {
        panic!(
            "expected an Activated entry, got {:?}",
            state.stack[0].object
        );
    };
    assert_eq!(*source, pinger);
    assert_eq!(state.stack[0].targets, vec![vec![bear]]);

    // Both players pass: the ability resolves and deals 1 to the bear.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);

    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::DamageDealt(DamageDealt { target, amount: 1, .. })) if *target == bear
        )),
        "1 damage dealt to the bear, trace: {trace:?}"
    );
    assert_eq!(
        state.objects.obj(bear).total_damage(),
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

    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, pinger).expect("the pinger's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
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
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.objects.obj_mut(pinger).summoning_sick = true;
    // The pending legal list predates the stamp — take the offered land drop
    // to reopen priority with a freshly computed list.
    let land = legal
        .iter()
        .find(|a| matches!(a, Action::PlayLand { .. }))
        .cloned()
        .expect("a Mountain in hand for the land drop");
    state.submit_decision(Decision::Act(land)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // Stamp sickness AFTER the turn-start untap (which clears the flag on the
    // active player's permanents, [CR#302.6]): the [CR#602.5a] gate is
    // creature-and-{T}/{Q}-only, so a summoning-sick artifact with a pure mana
    // cost must still be offered. The floats below reopen priority, so the
    // final legal list is computed with the stamp in place.
    state.objects.obj_mut(drawer).summoning_sick = true;
    float_mana(&mut state, PlayerId(0), 2); // R, R
    let hand_before = state.zones.hands[0].len();

    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, drawer)
        .expect("a summoning-sick artifact's mana-only ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // No targets, so Payment surfaces directly with two generic pips.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment for {{2}}, got {stop:?}");
    };
    assert_eq!(
        prompt.outstanding.len(),
        2,
        "the decision carries two generic-pip obligations"
    );
    let _ = complete_pending_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the ability is on the stack");
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
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
    let bolt = Arc::new(canon.card(INSTANT).unwrap().core);
    let drawer_card = Arc::new(testing().card(SORCERY_DRAWER).unwrap().core);
    let bears = Arc::new(canon.card(BEARS).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
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

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R

    // (a) Own precombat main, empty stack: the SorcerySpeed condition holds.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the instant, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    // run_to_priority advances the all-colored {R} payment.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R
    let hand_before = state.zones.hands[0].len();

    // First activation this turn: offered; pay {1} with one red; resolve.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, drawer).expect("offered before any use this turn");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(DecisionPointKind::Payment(_))
    ));
    let _ = complete_pending_payment(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert_eq!(
        state.zones.hands[0].len(),
        hand_before + 1,
        "the first use drew"
    );

    // Same turn, mana still floating: spent for the turn ([CR#602.5b]).
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        activate_action(&legal, drawer).is_some(),
        "the once-per-turn limit resets on a new turn, legal: {legal:?}"
    );
}

#[test]
fn once_per_game_stays_spent() {
    let mut state = activation_game(3, GAME_DRAWER, 2);
    let drawer = force_into_play(&mut state, PlayerId(0), GAME_DRAWER);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R
    let hand_before = state.zones.hands[0].len();

    // First (and only) activation: offered; resolve it.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, drawer).expect("offered before any use this game");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(DecisionPointKind::Payment(_))
    ));
    let _ = complete_pending_payment(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert_eq!(
        state.zones.hands[0].len(),
        hand_before + 1,
        "the only use drew"
    );

    // Same turn: spent.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        activate_action(&legal, drawer).is_none(),
        "once-per-game exhausted, legal: {legal:?}"
    );

    // P0's next turn: the per-turn flush must NOT revive a per-GAME limit.
    let _ = advance_to_next_own_main(&mut state);
    float_mana(&mut state, PlayerId(0), 1);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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

// --- loyalty abilities ([CR#606.3,306.5d]) ---------------------------------

/// Two-player game whose player-0 deck is ten copies of `LOYALTY_PW`
/// (`LoyaltyPlus`/`LoyaltyMinus` carry no mana component, so no lands are
/// needed); player 1 holds bears + forests as inert bystanders. Mirrors
/// `activation_game`/`cost_game`.
fn loyalty_game(seed: u64) -> GameState {
    let card = Arc::new(testing().card(LOYALTY_PW).unwrap().core);
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let p0 = vec![Arc::clone(&card); 10];
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

/// Activates ability `ability` of `object` at the current P0 priority window
/// and passes it through to resolution (both players pass with the ability
/// as the lone stack object). `LoyaltyPlus`/`LoyaltyMinus` carry no `Mana`
/// cost component, so [`cast::pay_cost`]'s empty-payment path applies — but
/// `run_to_priority` already advances payment prompts transparently if one
/// ever does, so this works either way.
fn activate_loyalty_and_resolve(state: &mut GameState, object: ObjectId, ability: usize) {
    let legal = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = legal
        .iter()
        .find(
            |a| matches!(a, Action::ActivateAbility { object: o, ability: n } if *o == object && *n == ability),
        )
        .cloned()
        .unwrap_or_else(|| panic!("ability {ability} of {object:?} not offered, legal: {legal:?}"));
    state.submit_decision(Decision::Act(activate)).unwrap();
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(state);
}

/// True iff `legal` offers `Action::ActivateAbility { object, ability }`.
fn loyalty_offered(legal: &[Action], object: ObjectId, ability: usize) -> bool {
    legal.iter().any(
        |a| matches!(a, Action::ActivateAbility { object: o, ability: n } if *o == object && *n == ability),
    )
}

/// [CR#606.3]: "A player may activate a loyalty ability of a permanent they
/// control any time they have priority and the stack is empty during a main
/// phase of their turn" — the same `window: SorcerySpeed` machinery
/// `sorcery_speed_drawer_gated` pins for a plain activated ability, wired
/// through `LoyaltyPlus`. Casting an instant occupies the stack, gating the
/// loyalty ability until it resolves.
#[test]
fn loyalty_ability_gated_at_sorcery_speed() {
    let canon = canon();
    let bolt = Arc::new(canon.card(INSTANT).unwrap().core);
    let pw_card = Arc::new(testing().card(LOYALTY_PW).unwrap().core);
    let bears = Arc::new(canon.card(BEARS).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut deck0 = vec![Arc::clone(&bolt); 4];
    deck0.extend(vec![Arc::clone(&pw_card); 3]);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| s.zones.hands[0].iter().any(|&o| is_card(s, o, INSTANT)))
        .expect("a seed with an instant in P0's opening hand");
    let pw = force_into_play(&mut state, PlayerId(0), LOYALTY_PW);
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);
    let instant = find_in_hand(&state, PlayerId(0), INSTANT);
    state
        .objects
        .obj_mut(pw)
        .counters
        .insert("LoyaltyCounter".into(), 5);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R for the bolt

    // (a) own precombat main, empty stack: the `+1` is offered.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        loyalty_offered(&legal, pw, 0),
        "the +1 loyalty ability is offered at sorcery speed on an empty stack, legal: {legal:?}"
    );

    // (b) cast the bolt; while it's on the stack, the loyalty ability is
    // gated ([CR#606.3]'s "stack is empty" clause — the SorcerySpeed
    // machinery `sorcery_speed_drawer_gated` already pins).
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: instant }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the instant, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the instant is on the stack");
    assert!(
        !loyalty_offered(&legal, pw, 0),
        "gated while the stack is non-empty, legal: {legal:?}"
    );

    // Let the instant resolve; at the next clean main-phase priority the
    // loyalty ability is offered again.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(state.stack.is_empty(), "the instant resolved");
    assert!(
        loyalty_offered(&legal, pw, 0),
        "offered again once the stack is empty, legal: {legal:?}"
    );
}

/// [CR#606.3,306.5d]: "only if none of that permanent's loyalty abilities
/// have been activated that turn" — SHARED across every loyalty ability of
/// the permanent, unlike a plain `OncePerTurn` (per-ability). Activating
/// `pw1`'s `+1` blocks `pw1`'s `−3` for the rest of the turn, but a
/// DIFFERENT permanent (`pw2`) is unaffected.
#[test]
fn loyalty_once_per_turn_is_shared_per_permanent() {
    let mut state = loyalty_game(11);
    let pw1 = force_into_play(&mut state, PlayerId(0), LOYALTY_PW);
    let pw2 = force_into_play(&mut state, PlayerId(0), LOYALTY_PW);
    for pw in [pw1, pw2] {
        state
            .objects
            .obj_mut(pw)
            .counters
            .insert("LoyaltyCounter".into(), 5);
    }

    // Before any activation: every loyalty ability of both permanents is
    // offered.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    for &pw in &[pw1, pw2] {
        for ability in 0..2usize {
            assert!(
                loyalty_offered(&legal, pw, ability),
                "ability {ability} of {pw:?} should be offered before any activation this turn, \
                 legal: {legal:?}"
            );
        }
    }

    // Activate pw1's `+1` (ability 0).
    activate_loyalty_and_resolve(&mut state, pw1, 0);

    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // pw1's OTHER loyalty ability (`−3`, ability 1) is now blocked — the
    // limit is SHARED across the permanent's loyalty abilities, not
    // per-ability.
    assert!(
        !loyalty_offered(&legal, pw1, 1),
        "pw1's OTHER loyalty ability should be blocked after pw1's +1 fired this turn, \
         legal: {legal:?}"
    );
    // pw1's own `+1` is (trivially) blocked too — it already fired.
    assert!(
        !loyalty_offered(&legal, pw1, 0),
        "pw1's +1 should be blocked — it already fired this turn, legal: {legal:?}"
    );
    // A loyalty ability of a DIFFERENT permanent (pw2) is still offered.
    for ability in 0..2usize {
        assert!(
            loyalty_offered(&legal, pw2, ability),
            "pw2's ability {ability} should be unaffected by pw1's activation, legal: {legal:?}"
        );
    }
}

/// [CR#606.6] regression: a `−3` loyalty proposal is visible below its counter
/// floor, but fulfilling its locked cost is rejected without mutation.
#[test]
fn loyalty_minus_below_its_counter_floor_fails_during_payment() {
    let mut state = loyalty_game(13);
    let pw = force_into_play(&mut state, PlayerId(0), LOYALTY_PW);
    state
        .objects
        .obj_mut(pw)
        .counters
        .insert("LoyaltyCounter".into(), 2);

    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        loyalty_offered(&legal, pw, 1),
        "the −3 proposal is offered before payment is proven, legal: {legal:?}"
    );
    assert!(
        loyalty_offered(&legal, pw, 0),
        "the +1 ability should still be offered even though −3 is unpayable, legal: {legal:?}"
    );

    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: pw,
            ability: 1,
        }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(DecisionPointKind::Payment(_))
    ));
    let before = state
        .objects
        .obj(pw)
        .counters
        .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
        .copied();
    assert!(
        state
            .submit_decision(
                state
                    .auto_payment_pending()
                    .expect("automatic payment decision"),
            )
            .is_err(),
        "−3 cannot be fulfilled with only two counters"
    );
    assert_eq!(
        state
            .objects
            .obj(pw)
            .counters
            .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
            .copied(),
        before
    );
}

#[test]
fn pinger_fizzles_when_target_dies() {
    // P0: pingers + Mountains (mono halves). P1: instants + bears + Mountains
    // — the seed search puts an instant in P1's opening hand; the bear and a
    // Mountain are pulled from the library.
    let canon = canon();
    let pinger_card = Arc::new(testing().card(PINGER).unwrap().core);
    let bolt = Arc::new(canon.card(INSTANT).unwrap().core);
    let bears = Arc::new(canon.card(BEARS).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| s.zones.hands[1].iter().any(|&o| is_card(s, o, INSTANT)))
        .expect("a seed with an instant in P1's opening hand");
    // Load builtin rules so the lethal-damage SBA fires (bear dies when the
    // instant deals 3 damage to a 2/2, fizzling the pinger [CR#608.2b]).
    state.sba_rules = builtin().sba_rules;
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    state.objects.obj_mut(pinger).summoning_sick = false;
    let bear = force_into_play(&mut state, PlayerId(1), BEARS);
    force_into_play(&mut state, PlayerId(1), "Mountain");

    // P0 activates the pinger at P1's bear.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, pinger).expect("the pinger is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the pinger, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the ability is on the stack");

    // P0 passes; P1 responds: float {R} and kill their own bear with the
    // instant (3 damage on the 2/2 is lethal).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(1), 1);
    let instant = find_in_hand(&state, PlayerId(1), INSTANT);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: instant }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the instant, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    // run_to_priority advances the all-colored {R} payment.
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "the instant sits atop the ability");

    // Resolve LIFO, collecting every damage event: the instant kills the
    // bear, then the pinger's ability finds its only target gone and fizzles
    // ([CR#608.2b]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap(); // P1
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap(); // P0 → resolve
    let mut damage = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::DamageDealt(DamageDealt { target, amount, .. }),
            ))) => damage.push((target, amount)),
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
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
fn gain_zero() -> Instruction {
    Instruction::act(CoreAction::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
        LifeOp::Up(Count::Literal(0)),
    ))
}

/// An artifact whose sole ability is an activated ability with the given cost
/// and a harmless no-target effect. Built in-Rust so a test pins the exact
/// cost components; `Cards::push` derives its printed abilities like any card.
fn artifact_with_cost(name: &str, cost: Vec<CostComponent>) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace::from(Characteristics {
        name: name.into(),
        mana_cost: ManaCost::from(Arc::<[ManaSymbol]>::from(vec![])),
        color_indicator: vec![],
        supertypes: vec![],
        types: vec![Type::Artifact.def()],
        subtypes: vec![],
        abilities: vec![Ability::activated(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            window: None,
            cost: Arc::<[deckmaste_core::CostComponent]>::from(cost).into(),
            condition: None,
            limits: vec![].into(),
            effect: deckmaste_core::Region::new(
                [
                    deckmaste_core::source_controller_params().as_ref(),
                    &[deckmaste_core::Param {
                        def: deckmaste_core::DefId(2),
                        kind: deckmaste_core::Kind::Number,
                        provenance: deckmaste_core::Provenance::AnnouncedX,
                    }],
                ]
                .concat()
                .into(),
                gain_zero().into(),
            ),
        })],
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    })))
}

/// Builds a two-player game whose player-0 deck is five copies of `card` plus
/// five Mountains (so the opening hand and library are well-formed), nothing
/// special for player 1. Mirrors `activation_game` but seeds an arbitrary
/// in-Rust card.
fn cost_game(seed: u64, card: &Arc<Card>) -> GameState {
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(card); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let p1 = vec![forest; 10];
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

/// Activates `object`'s only ability and drives its deterministic payment to
/// the next priority window, returning every progress item observed along the
/// way.
///
/// [CR#118.5,118.5a]: a `{0}` cost is a placeholder payable with nothing, but
/// the ability is NOT paid automatically — the action still requires the
/// player's acknowledgment. The explicit payment transaction therefore still
/// requires `BeginPayment` with empty coverage, fulfillment of every nonmana
/// IOU, and `SubmitPayment`.
fn activate_and_pay_zero(state: &mut GameState, object: ObjectId) -> Vec<Progress> {
    let legal = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate =
        activate_action(&legal, object).expect("the in-Rust ability is offered at priority");
    state.submit_decision(Decision::Act(activate)).unwrap();

    let mut trace = Vec::new();
    loop {
        let (progress, stop) = step_to_stop(state);
        trace.extend(progress);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => return trace,
            other => panic!("unexpected stop while paying a {{0}} ability: {other:?}"),
        }
    }
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
            CostComponent::do_action(CoreAction::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(deckmaste_core::RefId(0)),
            )),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    let _ = activate_and_pay_zero(&mut state, obj);

    // Drive to the next priority: the sacrifice cost must have fired during
    // payment, so the source is gone from the battlefield.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
            CostComponent::do_action(CoreAction::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Down(Count::Literal(2)),
            )),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    let life_before = state.players[0].life;
    // The trace spans the full payment and activation. The `LifeLost` cost
    // must occur before `AbilityActivated` ([CR#601.2h] precedes [CR#601.2i]).
    let trace = activate_and_pay_zero(&mut state, obj);
    let life_idx = trace.iter().position(|p| {
        matches!(
            applied(p),
            Some(GameEvent::LifeLost(LifeLost { player, amount: 2, .. })) if *player == PlayerId(0)
        )
    });
    let activated_idx = trace.iter().position(|p| {
        matches!(applied(p), Some(GameEvent::AbilityActivated(AbilityActivated { source, .. })) if *source == obj)
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

/// [CR#606.4]: a loyalty `+N` ability's cost is to put that many loyalty
/// counters on the ability's source. Modeled as a `Do(PutCounters(This,
/// LoyaltyCounter, N))` cost (no dedicated loyalty-cost verb — see
/// `idris/src/Semantics.idr`'s `Cost` `Do` ruling). This exercises
/// `PutCounters` being cost-eligible: the counters are ADDED to whatever
/// loyalty is already present during the payment window.
#[test]
fn activated_ability_pays_loyalty_plus_cost() {
    const NAME: &str = "Loyalty-plus-cost test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::do_action(CoreAction::PutCounters(
                Reference::Reg(deckmaste_core::RefId(0)),
                "LoyaltyCounter".into(),
                Count::Literal(2),
            )),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    // Seed the source with 3 loyalty so the `+2` cost proves it ADDS
    // ([CR#606.4]).
    state
        .objects
        .obj_mut(obj)
        .counters
        .insert("LoyaltyCounter".into(), 3);

    let _ = activate_and_pay_zero(&mut state, obj);

    // Drive to the next priority: the `+2` cost must have fired during payment.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state
            .objects
            .obj(obj)
            .counters
            .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
            .copied(),
        Some(5),
        "the `+2` loyalty cost added two loyalty counters (3 -> 5)"
    );
}

/// [CR#606.4,606.6]: a loyalty `−N` ability's cost is to remove that many
/// loyalty counters from the source (which must have at least that many,
/// [CR#606.6]). Modeled as `Do(RemoveCounters(This, LoyaltyCounter, N))` —
/// `RemoveCounters` was already cost-eligible, so this pins that `−N` pays with
/// no code change.
#[test]
fn activated_ability_pays_loyalty_minus_cost() {
    const NAME: &str = "Loyalty-minus-cost test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::do_action(CoreAction::RemoveCounters(
                Reference::Reg(deckmaste_core::RefId(0)),
                "LoyaltyCounter".into(),
                Count::Literal(2),
            )),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    // Seed the source with 5 loyalty so the `−2` cost has counters to remove.
    state
        .objects
        .obj_mut(obj)
        .counters
        .insert("LoyaltyCounter".into(), 5);

    let _ = activate_and_pay_zero(&mut state, obj);

    // Drive to the next priority: the `−2` cost must have fired during payment.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state
            .objects
            .obj(obj)
            .counters
            .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
            .copied(),
        Some(3),
        "the `−2` loyalty cost removed two loyalty counters (5 -> 3)"
    );
}

/// [CR#601.2b,107.3a,606.6]: a loyalty `−X` cost —
/// `Do(RemoveCounters(This, LoyaltyCounter, X))` — carries a declared-X operand
/// but NO `{X}` mana symbol. The activate-time X-announce trigger must still
/// fire (a `ChooseXValue` surfaces), and the announced value must bind the cost
/// verb so it removes exactly X counters. This pins the non-mana X-cost
/// announce surface (engine-nonmana-x-cost-announce), extending the mana-only
/// `{X}` trigger from engine-x-costs.
#[test]
fn activated_ability_announces_and_pays_nonmana_x_cost() {
    const NAME: &str = "Loyalty-minus-X-cost test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::do_action(CoreAction::RemoveCounters(
                Reference::Reg(deckmaste_core::RefId(0)),
                "LoyaltyCounter".into(),
                Count::Reg(deckmaste_core::RefId(2)),
            )),
        ],
    );
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    // Seed 5 loyalty so a `−X` for X=3 has counters to remove.
    state
        .objects
        .obj_mut(obj)
        .counters
        .insert("LoyaltyCounter".into(), 5);

    // Activate the ability — even with no `{X}` mana, the cost's X operand
    // verb operand must trigger the X announcement.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate =
        activate_action(&legal, obj).expect("the loyalty −X ability is offered at priority");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // [CR#601.2b]: X is announced first — driven by the cost verb, not mana.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { player },
    )) = stop
    else {
        panic!("expected ChooseXValue for the non-mana X cost, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the activating player announces X");
    state.submit_decision(Decision::XValue(3)).unwrap();

    // The `{0}` mana component still opens PrePayment with empty coverage, and
    // the nonmana X action is the sole fulfillment IOU.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment after X, got {stop:?}");
    };
    assert_eq!(prompt.stage, deckmaste_engine::PaymentStage::PrePayment);
    assert!(matches!(
        prompt.outstanding.as_slice(),
        [deckmaste_engine::PaymentIou {
            kind: deckmaste_engine::IouKind::Act { .. },
            ..
        }]
    ));
    let _ = complete_pending_payment(&mut state);

    // Drive to the next priority: the `−X` cost fired during payment, reading
    // the announced X=3, so 3 loyalty counters were removed (5 -> 2).
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state
            .objects
            .obj(obj)
            .counters
            .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
            .copied(),
        Some(2),
        "the announced X=3 bound the cost verb: it removed three loyalty counters (5 -> 2)"
    );
}

/// [CR#601.2b,601.2h]: a payment-time choice obligation accepts the
/// complete chosen object set in its `Fulfill` witness. The chosen creature is
/// sacrificed and the other is left untouched. Re-spelled from the deleted
/// `CostComponent::ChooseAndPay`/`Binder::ChooseOne` form: the choice is now
/// its own cost instruction writing a register, and the sacrifice reads it.
#[test]
fn activated_ability_pays_choose_sacrifice_cost() {
    const ARTIFACT_NAME: &str = "Choose-sacrifice test artifact";
    // Registers: 0 source, 1 controller, 2 announced X; the cost's choice
    // therefore defines register 3, which the paying verb reads.
    let chosen = deckmaste_core::DefId(3);
    // Creature filter: battlefield creatures (zone check + type check).
    let creature_filter = deckmaste_core::Predicate::And(
        vec![
            deckmaste_core::Predicate::State(deckmaste_core::StatePredicate::InZone(
                Zone::Battlefield,
            )),
            deckmaste_core::Predicate::creature(),
        ]
        .into(),
    );
    let card = artifact_with_cost(
        ARTIFACT_NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            // "sacrifice a creature": the payment-time decision ([CR#601.2b])
            // followed by the verb that spends the register it wrote.
            CostComponent::Choose(deckmaste_core::Choose {
                dest: chosen,
                by: Reference::Reg(deckmaste_core::RefId(1)),
                quantity: deckmaste_core::Quantity::one(),
                filter: Arc::new(deckmaste_core::Region::candidate(creature_filter)),
            }),
            CostComponent::do_action(CoreAction::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(chosen.into()),
            )),
        ],
    );

    // Build a game: player 0 gets the artifact + mountains + Grizzly Bears,
    // player 1 gets forests (no creatures so candidates are unambiguously
    // P0's).
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bears_card = Arc::new(canon().card(BEARS).unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&card); 3];
    p0.extend(vec![Arc::clone(&bears_card); 4]);
    p0.extend(vec![Arc::clone(&mountain); 3]);
    let p1 = vec![forest; 10];
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
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

    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate = activate_action(&legal, artifact).expect("the ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected prepayment, got {stop:?}");
    };
    assert_eq!(prompt.stage, deckmaste_engine::PaymentStage::PrePayment);
    state
        .submit_decision(
            state
                .auto_payment_pending()
                .expect("automatic payment decision"),
        )
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected paying prompt, got {stop:?}");
    };
    let choose = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, deckmaste_engine::IouKind::Choose(_)))
        .expect("a payment-time choice IOU")
        .id;
    state
        .submit_decision(Decision::Payment(
            deckmaste_engine::PaymentCommand::Fulfill {
                iou: choose,
                witness: deckmaste_engine::FulfillmentWitness::Objects(vec![bear_a]),
            },
        ))
        .unwrap();

    // Drive the ready prompt through submission and back to priority.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // bear_a was sacrificed: its original id is no longer on the battlefield.
    assert!(
        !state.zones.battlefield.contains(&bear_a),
        "bear_a was sacrificed and is no longer on the battlefield"
    );
    // At least one object with the Bear's name is in P0's graveyard (the
    // reminted id).
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
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);
    assert!(state.stack.is_empty(), "the ability resolved cleanly");
}

#[test]
fn mana_ability_stays_stackless() {
    let mut state = activation_game(4, MANA_DRAWER, 1);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(state.player(PlayerId(0)).mana_pool.is_empty());

    // The only battlefield activatable is the Mountain's mana ability.
    let tap = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .cloned()
        .expect("the Mountain's mana ability is offered");
    state.submit_decision(Decision::Act(tap)).unwrap();

    // Drive to the next priority one step at a time. The payment prompts are
    // explicit, but the activation remains stackless throughout ([CR#605.3b]).
    let stop = loop {
        match state.step() {
            StepOutcome::Progress(_) => assert!(
                state.stack.is_empty(),
                "a mana ability never touches the stack"
            ),
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                assert!(state.stack.is_empty(), "payment never uses the stack");
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            stop => break stop,
        }
    };
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        ..
    })) = stop
    else {
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
// cost carries a hybrid or Phyrexian symbol. They schedule the announce block
// directly onto the agenda so the tests can isolate the WIRING: a
// `ChooseCostOptions` decision surfaces at [CR#601.2b] (between targets and
// payment), and `pay_cost` consumes the player's announced reading.

fn white() -> ColorOrColorless {
    Color::White.into()
}
fn blue() -> ColorOrColorless {
    Color::Blue.into()
}

/// Schedules the [CR#602.2b] activation announce block for `object`'s ability
/// `index` straight onto the agenda front, mirroring `take_priority_action`'s
/// `ActivateAbility` arm (minus the priority bookkeeping). This isolates the
/// announcement flow from priority handling.
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
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    // Float the cost's mana now — after the walk, in the window that pays it.
    for &(color, amount) in float {
        state
            .player_mut(PlayerId(0))
            .mana_pool
            .add(color, amount, ManaProvenance::default());
    }
    // Clear the Priority decision `run_to_priority` stopped at; the announce
    // block front-runs the next priority open. (`consecutive_passes` is NOT
    // reset.)
    state.pending = None;
    // The full [CR#602.2b] announce block, in order: BeginActivate → modes →
    // targets → cost options → payment → AbilityActivated → SBAs → triggers →
    // priority. Pushed back-to-front so the front-of-agenda order is
    // left-to-right.
    let block = [
        WorkItem::BeginActivate {
            object,
            ability: index,
        },
        WorkItem::AnnounceModes,
        WorkItem::AnnounceTargets,
        WorkItem::ChooseCostOptions,
        WorkItem::OpenPayment,
        WorkItem::Emit(Occurrence::single(GameEvent::AbilityActivated(
            AbilityActivated {
                source: object,
                ability: index,
            },
        ))),
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
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseCostOptions(
        deckmaste_engine::ChooseCostOptions {
            player,
            cost,
            options,
            additional: _,
        },
    )) = stop
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

    // The payment graph now carries one concrete blue pip, not the printed
    // hybrid symbol.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment for the concretized {{U}}, got {stop:?}");
    };
    assert!(matches!(
        prompt.outstanding.as_slice(),
        [deckmaste_engine::PaymentIou {
            kind: deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(
                Color::Blue
            )),
            ..
        }]
    ));
    let _ = complete_pending_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseCostOptions(
        deckmaste_engine::ChooseCostOptions { options, .. },
    )) = stop
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
    let (mut trace, stop) = step_to_stop(&mut state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(DecisionPointKind::Payment(_))
    ));
    trace.extend(complete_pending_payment(&mut state));
    let life_idx = trace.iter().position(|p| {
        matches!(
            applied(p),
            Some(GameEvent::LifeLost(LifeLost { player, amount: 2, .. })) if *player == PlayerId(0)
        )
    });
    let activated_idx = trace.iter().position(|p| {
        matches!(applied(p), Some(GameEvent::AbilityActivated(AbilityActivated { source, .. })) if *source == obj)
    });
    assert!(
        !trace.iter().any(|p| matches!(p, Progress::CostPaid)
            && matches!(
                state.pending,
                Some(DecisionPointKind::PayMana(deckmaste_engine::PayMana { .. }))
            )),
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
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseCostOptions(
        deckmaste_engine::ChooseCostOptions { options, .. },
    )) = stop
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
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment for the concretized {{2}}, got {stop:?}");
    };
    assert_eq!(
        prompt
            .outstanding
            .iter()
            .filter(|iou| matches!(iou.kind, deckmaste_engine::IouKind::ManaPip(_)))
            .count(),
        2,
        "the generic half concretizes to two pip IOUs"
    );
    let _ = complete_pending_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
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
    // stop is Payment with one generic pip.
    let (trace, stop) = step_to_stop(&mut state);
    assert!(
        !trace
            .iter()
            .any(|p| matches!(p, Progress::CostOptionsChosen { surfaced: true })),
        "a plain cost surfaces no ChooseCostOptions decision, trace: {trace:?}"
    );
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment for the plain {{1}} cost, got {stop:?}");
    };
    assert!(matches!(
        prompt.outstanding.as_slice(),
        [deckmaste_engine::PaymentIou {
            kind: deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Generic),
            ..
        }]
    ));
}

// --- proposal enumeration is independent of affordability ------------------

fn green() -> ColorOrColorless {
    Color::Green.into()
}

/// Floats `pool` mana into P0's pool at a freshly re-surfaced precombat-main
/// priority and returns the recomputed legal list (with the float reflected).
/// The pool empties each step end ([CR#500.5]), so the float happens in the
/// same window the gate is evaluated.
fn legal_with_float(state: &mut GameState, pool: &[(ColorOrColorless, Uint)]) -> Vec<Action> {
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    for &(color, amount) in pool {
        state
            .player_mut(PlayerId(0))
            .mana_pool
            .add(color, amount, ManaProvenance::default());
    }
    // Re-derive priority so the freshly floated pool is reflected in the list
    // (mirrors x_costs.rs's resurface_priority).
    assert!(
        matches!(
            state.pending,
            Some(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. }
            ))
        ),
        "expected a Priority decision to resurface"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain)
}

/// [CR#107.4e,601.2g]: a hybrid `{W/U}` ability is offered before the player
/// proves which reading they will pay.
#[test]
fn hybrid_ability_proposal_is_offered_without_affordability_proof() {
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

    // No mana at all: payment may later fail, but the proposal remains legal.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let legal = legal_with_float(&mut state, &[]);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/U}} is offered before payment is proven, legal: {legal:?}"
    );
}

/// [CR#107.4f,601.2g]: a Phyrexian `{W/P}` ability is offered before the
/// player proves which reading they will pay.
#[test]
fn phyrexian_ability_proposal_is_offered_even_when_current_resources_fail() {
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

    // One life and no white cannot complete either reading, but affordability
    // is discovered only after the reading is announced and payment opens.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 1;
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/P}} remains a legal proposal at 1 life with no white, legal: {legal:?}"
    );
}

/// [CR#107.4f]: two Phyrexian `{W/P}{W/P}` symbols remain a legal proposal
/// even though two life and no white cannot complete both readings.
#[test]
fn two_phyrexian_proposal_does_not_precompute_shared_life() {
    const NAME: &str = "Double-Phyrexian-gate test artifact";
    let card = artifact_with_cost(
        NAME,
        vec![CostComponent::Mana("{W/P}{W/P}".parse().unwrap())],
    );

    // Two life and no white cannot complete both symbols, but the proposal is
    // still enumerated before the player chooses the readings.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 2;
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/P}}{{W/P}} is offered without a shared-life precheck, legal: {legal:?}"
    );

    // 2 life + ONE white: pay one symbol with the white, the other with 2 life
    // -> offered.
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 2;
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(white(), 1, ManaProvenance::default());
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        activate_action(&legal, obj).is_some(),
        "{{W/P}}{{W/P}} at 2 life + one white IS activatable (one life, one white), legal: {legal:?}"
    );
}

/// A plain `{1}` ability is offered before its payment is proven, regardless of
/// whether mana is already floating.
#[test]
fn plain_cost_proposal_is_offered_with_an_empty_pool() {
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
        activate_action(&legal, obj).is_some(),
        "plain {{1}} is offered before payment is proven, legal: {legal:?}"
    );
}

/// A cost bearing both `{X}` and a hybrid (`{X}{W/U}`) is offered before either
/// part is paid.
#[test]
fn x_plus_hybrid_proposal_is_offered_before_either_part_is_payable() {
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
        activate_action(&legal, obj).is_some(),
        "{{X}}{{W/U}} is offered before a hybrid reading is paid, legal: {legal:?}"
    );
}

/// [CR#107.3a,107.4e,601.2b]: a cost bearing both `{X}` and a hybrid drives
/// the full announce flow. The two concretizers run at different
/// steps — `AnnounceX` records X, `ChooseCostOptions` picks the hybrid reading
/// (passing `{X}` through untouched), then `PayCost` applies `concretize_x` to
/// the residual `{X}`. With X=2 and the blue reading picked, the payment
/// obligation contains `{2}{U}` (X→`Generic(2)`, `{W/U}`→`{U}`), and exactly
/// the right two units (one blue + two generic-payable) are spent.
#[test]
fn x_plus_hybrid_announces_x_concretizes_hybrid_pays_composed_cost() {
    const NAME: &str = "X-plus-hybrid e2e test artifact";
    let card = artifact_with_cost(NAME, vec![CostComponent::Mana("{X}{W/U}".parse().unwrap())]);
    let mut state = cost_game(7, &card);
    let obj = force_into_play(&mut state, PlayerId(0), NAME);

    // Float one blue (for the {W/U}→{U} reading) plus two greens (generic-
    // payable, to fund X=2). `legal_with_float` floats AFTER reaching the
    // precombat-main priority window (the pool empties at every step end,
    // [CR#500.5]) and re-derives the legal list with the float reflected.
    let legal = legal_with_float(&mut state, &[(blue(), 1), (green(), 2)]);
    let activate = activate_action(&legal, obj)
        .expect("the {X}{W/U} ability is OFFERED when the hybrid reading is payable");
    state.submit_decision(Decision::Act(activate)).unwrap();

    // [CR#601.2b]: X is announced FIRST (engine-x-costs' `AnnounceX` step).
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { player },
    )) = stop
    else {
        panic!("expected ChooseXValue first, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(2)).unwrap();

    // [CR#601.2b]: ...then ChooseCostOptions for the hybrid. The carried cost is
    // the printed {X}{W/U}; only the hybrid symbol is choosable (X passes
    // through). Pick the blue reading.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseCostOptions(
        deckmaste_engine::ChooseCostOptions {
            player,
            cost,
            options,
            additional: _,
        },
    )) = stop
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

    // Payment composes the two concretizers into two generic pip IOUs and one
    // blue pip IOU.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment for the composed {{2}}{{U}}, got {stop:?}");
    };
    assert_eq!(prompt.outstanding.len(), 3);
    assert_eq!(
        prompt
            .outstanding
            .iter()
            .filter(|iou| matches!(
                iou.kind,
                deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Generic)
            ))
            .count(),
        2
    );
    assert_eq!(
        prompt
            .outstanding
            .iter()
            .filter(|iou| matches!(
                iou.kind,
                deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(Color::Blue))
            ))
            .count(),
        1
    );
    let _ = complete_pending_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // The blue unit paid {U}; the two greens paid the {2}. Nothing is left
    // over.
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

// --- hybrid/Phyrexian spell proposal enumeration ----------------------------

/// An instant whose only distinction is its printed (hybrid/Phyrexian) mana
/// cost and a harmless no-target effect. Instant timing keeps it castable at
/// any priority.
fn instant_with_cost(name: &str, cost: ManaCost) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace::from(Characteristics {
        name: name.into(),
        mana_cost: cost,
        color_indicator: vec![],
        supertypes: vec![],
        types: vec![Type::Instant.def()],
        subtypes: vec![],
        abilities: vec![],
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    })))
}

/// The `CastSpell` action for `object` in `legal`, if offered.
fn cast_action(legal: &[Action], object: ObjectId) -> Option<Action> {
    legal
        .iter()
        .find(|a| matches!(a, Action::CastSpell { object: o } if *o == object))
        .cloned()
}

/// [CR#601.2g,107.4e]: a hybrid `{W/U}` instant is offered before its reading
/// is paid.
#[test]
fn hybrid_spell_proposal_is_offered_without_affordability_proof() {
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
        cast_action(&legal, spell).is_some(),
        "{{W/U}} instant is offered before payment is proven, legal: {legal:?}"
    );
}

/// [CR#601.2g,107.4f]: a Phyrexian `{W/P}` instant is offered before payment
/// is proven.
#[test]
fn phyrexian_spell_proposal_is_offered_even_when_current_resources_fail() {
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
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.player_mut(PlayerId(0)).life = 1;
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        cast_action(&legal, spell).is_some(),
        "{{W/P}} instant remains a legal proposal at 1 life with no white, legal: {legal:?}"
    );
}

// --- announcement cost blocks ([CR#118.8,601.2b])
// -----------------------------

/// An artifact whose sole ability carries the given announcement cost block and
/// effect region. The cost's instructions define into the ability's own region
/// ahead of the body, so `effect` may read a payment product by register.
fn artifact_with_cost_and_effect(
    name: &str,
    cost: Vec<CostComponent>,
    effect: Instruction,
) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace::from(Characteristics {
        name: name.into(),
        mana_cost: ManaCost::from(Arc::<[ManaSymbol]>::from(vec![])),
        color_indicator: vec![],
        supertypes: vec![],
        types: vec![Type::Artifact.def()],
        subtypes: vec![],
        abilities: vec![Ability::activated(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            window: None,
            cost: Arc::<[deckmaste_core::CostComponent]>::from(cost).into(),
            condition: None,
            limits: vec![].into(),
            effect: effect.into(),
        })],
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    })))
}

/// A candidate region matching "another creature" — register 0 is the candidate
/// under test, register 1 the ability's source.
fn another_creature() -> Arc<deckmaste_core::Region<deckmaste_core::Predicate>> {
    use deckmaste_core::Predicate;

    Arc::new(deckmaste_core::Region::new(
        Arc::from([
            deckmaste_core::Param {
                def: deckmaste_core::DefId(0),
                kind: deckmaste_core::Kind::Entity,
                provenance: deckmaste_core::Provenance::Candidate(deckmaste_core::Domain::Entity),
            },
            deckmaste_core::Param {
                def: deckmaste_core::DefId(1),
                kind: deckmaste_core::Kind::Entity,
                provenance: deckmaste_core::Provenance::Source,
            },
        ]),
        Predicate::And(Arc::from(vec![
            Predicate::r#type(Type::Creature),
            Predicate::State(deckmaste_core::StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(
                deckmaste_core::RefId(1),
            )))),
        ])),
    ))
}

/// Ayli, Eternal Pilgrim's first activated ability ([CR#118.8,601.2b,601.2h]):
/// "{1}, Sacrifice another creature: You gain life equal to the sacrificed
/// creature's toughness."
///
/// The witness the `engine-root-additional-cost-hoist` ticket named. Three
/// claims, all about ANNOUNCEMENT rather than resolution:
///
/// 1. the creature is chosen and sacrificed while the ability is being
///    activated — it is already in the graveyard before the ability resolves
///    ([CR#601.2h] precedes [CR#601.2i]);
/// 2. every player's response window opens only after that payment, with the
///    ability already on the stack;
/// 3. resolution gains life equal to that creature's toughness, read through
///    the register the payment wrote — not through any re-derived anaphor.
#[test]
fn sacrifice_cost_is_paid_at_activation_and_its_product_is_read_at_resolution() {
    const NAME: &str = "Ayli-shaped life gainer";
    // Registers: 0 source, 1 controller, 2 announced X; the cost's choice
    // therefore defines register 3, which the body reads.
    let chosen = deckmaste_core::DefId(3);
    let card = artifact_with_cost_and_effect(
        NAME,
        vec![
            CostComponent::Mana("{0}".parse().unwrap()),
            CostComponent::Choose(deckmaste_core::Choose {
                dest: chosen,
                by: Reference::Reg(deckmaste_core::RefId(1)),
                quantity: deckmaste_core::Quantity::one(),
                filter: another_creature(),
            }),
            CostComponent::do_action(CoreAction::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(chosen.into()),
            )),
        ],
        Instruction::act(CoreAction::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Up(Count::StatOf(
                Reference::Reg(chosen.into()),
                deckmaste_core::Stat::Toughness,
            )),
        )),
    );

    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut deck0 = vec![Arc::clone(&card); 4];
    deck0.extend(vec![Arc::clone(&bears); 4]);
    deck0.extend(vec![mountain; 4]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: deck0 },
            PlayerConfig {
                deck: vec![forest; 10],
            },
        ],
        seed: 11,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let source = force_into_play(&mut state, PlayerId(0), NAME);
    let bear = force_into_play(&mut state, PlayerId(0), BEARS);
    let life_before = state.player(PlayerId(0)).life;
    let toughness = state
        .layers()
        .toughness(bear)
        .expect("the bear has a toughness");

    let trace = activate_and_pay_zero(&mut state, source);

    // (1) The chosen creature is sacrificed during the activation, before the
    //     ability becomes activated ([CR#601.2h] then [CR#601.2i]).
    let sacrificed = trace
        .iter()
        .position(|p| {
            matches!(
                applied(p),
                Some(GameEvent::ZoneChange(change)) if change.object == bear
            )
        })
        .unwrap_or_else(|| panic!("the chosen creature is sacrificed, trace: {trace:?}"));
    let activated = trace
        .iter()
        .position(|p| {
            matches!(
                applied(p),
                Some(GameEvent::AbilityActivated(AbilityActivated { source: s, .. })) if *s == source
            )
        })
        .unwrap_or_else(|| panic!("the ability becomes activated, trace: {trace:?}"));
    assert!(
        sacrificed < activated,
        "the sacrifice is a COST paid during activation, not part of resolution; \
         sacrificed@{sacrificed} activated@{activated}"
    );

    // (2) Opponents respond only after payment: the first priority window has
    //     the ability on the stack and the creature already gone.
    assert_eq!(state.stack.len(), 1, "the ability is on the stack");
    assert!(
        state.objects.get(bear).is_none(),
        "the sacrificed creature left the battlefield before anyone could respond"
    );
    assert_eq!(
        state.player(PlayerId(0)).life,
        life_before,
        "no life is gained until the ability resolves ([CR#608.2])"
    );

    // (3) Resolution reads the paid product's register.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    // Both players pass, so the ability resolves before the next P0 window.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(state.stack.is_empty(), "the ability resolved");
    assert_eq!(
        state.player(PlayerId(0)).life,
        life_before + toughness,
        "resolution gains life equal to the sacrificed creature's toughness"
    );
}

/// Fling ([CR#118.8,601.2b,601.2h]): "As an additional cost to cast this
/// spell, sacrifice a creature. Fling deals damage equal to the sacrificed
/// creature's power to any target."
///
/// The witness the `engine-bound-references` ticket named. The printed
/// additional cost is HOISTED onto the spell's announcement, so the creature is
/// chosen and sacrificed while Fling is being cast, and the resolving spell
/// reads that creature as the payment's product — never through a trigger's
/// event-object slot, and never by re-deriving an anaphor at resolution.
#[test]
fn fling_reads_the_sacrificed_creature_as_its_paid_product() {
    const FLING: &str = "Fling";
    let canon = canon();
    let fling = Arc::new(canon.card(FLING).unwrap().core);
    let bears = Arc::new(canon.card(BEARS).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut deck0 = vec![Arc::clone(&fling); 4];
    deck0.extend(vec![Arc::clone(&bears); 4]);
    deck0.extend(vec![Arc::clone(&mountain); 6]);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    };
    let mut state = (0u64..2000)
        .map(build)
        .find(|s| s.zones.hands[0].iter().any(|&o| is_card(s, o, FLING)))
        .expect("a seed with Fling in P0's opening hand");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let bear = force_into_play(&mut state, PlayerId(0), BEARS);
    let spell = find_in_hand(&state, PlayerId(0), FLING);
    let power = state.layers().power(bear).expect("the bear has a power");
    let opponent = state.player(PlayerId(1)).object;
    let life_before = state.player(PlayerId(1)).life;

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let cast = legal
        .iter()
        .find(|a| matches!(a, Action::CastSpell { object } if *object == spell))
        .cloned()
        .unwrap_or_else(|| panic!("Fling is castable, legal: {legal:?}"));
    state.submit_decision(Decision::Act(cast)).unwrap();

    // Announcement: target first ([CR#601.2c]), then the additional cost is
    // paid ([CR#601.2h]) — the deterministic payer takes the only creature.
    let mut targeted = false;
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(_)) => {
                state
                    .submit_decision(Decision::Targets(vec![vec![opponent]]))
                    .unwrap();
                targeted = true;
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::PayMana(_)) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => break,
            other => panic!("unexpected stop while casting Fling: {other:?}"),
        }
    }
    assert!(targeted, "Fling announced its target");

    // The additional cost was paid as part of casting: the creature is gone
    // while the spell is still on the stack.
    assert_eq!(state.stack.len(), 1, "Fling is on the stack");
    assert!(
        state.objects.get(bear).is_none(),
        "the sacrificed creature left the battlefield during the cast ([CR#118.8a])"
    );
    assert_eq!(
        state.player(PlayerId(1)).life,
        life_before,
        "no damage until Fling resolves ([CR#608.2])"
    );

    // Resolution reads the paid product's last-known power ([CR#608.2h]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(state.stack.is_empty(), "Fling resolved");
    assert_eq!(
        state.player(PlayerId(1)).life,
        life_before - power,
        "Fling dealt damage equal to the sacrificed creature's power"
    );
}
