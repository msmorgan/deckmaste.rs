//! End-to-end coverage of `ManaRider::SpendOnly` enforcement at payment
//! ([CR#106.6]): a unit carrying `SpendOnly(filter)` may pay only for an object
//! the filter matches. Modeled on `stack.rs`'s harness; the relevant pieces of
//! that file are reproduced here (a Grizzly Bears `{1}{G}` game, the
//! `step_to_*` helpers) so this suite stands alone.

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Filter;
use deckmaste_core::ManaRider;
use deckmaste_core::Phase;
use deckmaste_core::Supertype;
use deckmaste_core::TurnMarker;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StackObject;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;

// --- plugin + deck building --------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

fn card(name: &str) -> Arc<Card> {
    Arc::new(canon().card(name).unwrap())
}

/// A snow Forest: the builtin `Forest` (whose `Forest` subtype confers the
/// tap-for-green mana ability) with the `Snow` supertype added — the minimal
/// snow source. Built in-Rust rather than loaded so the subtype's conferred
/// ability is present (the generated snow basics leave their subtype's
/// `confers` empty), isolating the test on snow-source detection.
fn snow_forest() -> Card {
    let mut forest = builtin().card("Forest").unwrap();
    let Card::Normal(face) = &mut forest else {
        panic!("Forest is a Normal card");
    };
    face.name = "Snow-Covered Forest".into();
    if !face.supertypes.contains(&Supertype::Snow) {
        face.supertypes.push(Supertype::Snow);
    }
    forest
}

fn green() -> ColorOrColorless {
    Color::Green.into()
}
fn red() -> ColorOrColorless {
    Color::Red.into()
}

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.name,
    }
}

fn is_card(state: &GameState, id: ObjectId, name: &str) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == name)
}

fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    *state.zones.hands[player.index()]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand", player.0))
}

/// Player 0 holds Grizzly Bears `{1}{G}` (a creature) and Forests; player 1
/// holds Forests. `forests` Forests are forced onto player 0's battlefield.
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
        let obj = find_in_hand(&state, PlayerId(0), "Forest");
        state.zones.hands[0].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(obj);
    }
    state
}

// --- stepping helpers --------------------------------------------------------

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
/// any other priority and auto-paying any `PayMana` along the way. Returns the
/// legal action list at that window.
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

/// Re-derives the in-flight priority decision so a freshly injected pool is
/// reflected in the legal list. Nulls the frozen `Priority` and schedules an
/// `OpenPriority` — the same work item the engine uses to re-grant priority.
/// (The `pub` fields make this direct setup possible without widening the API.)
fn resurface_priority(state: &mut GameState) {
    assert!(
        matches!(state.pending, Some(PendingDecision::Priority { .. })),
        "resurface_priority expects a Priority decision in flight"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
}

// --- tests -------------------------------------------------------------------

/// A green carrying `SpendOnly(creature)`, plus a plain unit to cover the
/// generic `{1}`, funds Grizzly Bears (a creature): the restricted green is
/// spendable on the subject and the Bears spell reaches the stack.
#[test]
fn spend_only_creature_funds_a_creature_spell() {
    let mut state = bears_game(1, 0);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // Float a restricted green (creature-only) + a plain red for {1}.
    state.player_mut(PlayerId(0)).mana_pool.add_riders(
        green(),
        1,
        &[ManaRider::SpendOnly(Filter::creature())],
    );
    state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
    // Re-derive the frozen priority list with the freshly floated pool.
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();

    // The cast pays (PayMana surfaces and auto-pays) and the spell reaches the
    // stack: the SpendOnly(creature) green is spendable on the creature.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the Bears spell sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(bears));
    assert!(!state.zones.battlefield.contains(&bears));
}

/// Steps until a `Priority` decision surfaces for `player` in `phase`, passing
/// any other priority, auto-paying mana, and declaring no attackers/blockers.
fn run_to_priority_through_combat(
    state: &mut GameState,
    player: PlayerId,
    phase: Phase,
) -> Vec<Action> {
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
            StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) => {
                state.submit_decision(Decision::Attackers(vec![])).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. }) => {
                state.submit_decision(Decision::Blocks(vec![])).unwrap();
            }
            other => {
                panic!("unexpected stop before {player:?} priority in {phase:?}: {other:?}")
            }
        }
    }
}

/// A green carrying `Persistent(EndOfTurn)` survives every step boundary until
/// cleanup, while a plain red added at the same time empties at the first step
/// end. Exercises the persistence logic end-to-end through the engine's
/// `ManaEmptied { ending }` event and the `empty_after` retainer.
#[test]
fn persistent_end_of_turn_mana_survives_step_boundaries() {
    let mut state = bears_game(1, 0);
    // Reach precombat-main priority for player 0.
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // Float one plain red and one persistent green (EndOfTurn).
    state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
    state.player_mut(PlayerId(0)).mana_pool.add_riders(
        green(),
        1,
        &[ManaRider::Persistent(TurnMarker::EndOfTurn)],
    );

    // Both units are present before any step boundary.
    assert_eq!(state.player(PlayerId(0)).mana_pool.amount(red()), 1);
    assert_eq!(state.player(PlayerId(0)).mana_pool.amount(green()), 1);

    // Advance past the precombat-main → combat step boundary. The plain red
    // empties; the EndOfTurn green survives. We drive to postcombat-main
    // priority (passing all intermediate decisions automatically, including
    // declare attackers and blockers).
    let _ = run_to_priority_through_combat(&mut state, PlayerId(0), Phase::PostcombatMain);

    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(red()),
        0,
        "plain red must be gone after the step boundary"
    );
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(green()),
        1,
        "persistent EndOfTurn green must survive until cleanup"
    );
}

/// Player 0 holds one Snow-Covered Forest and one plain Forest, both forced
/// onto the battlefield untapped. Used to compare the riders on mana tapped
/// from a snow source vs a non-snow source.
fn snow_vs_plain_game(seed: u64) -> GameState {
    let snow = Arc::new(snow_forest());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![Arc::clone(&snow), Arc::clone(&forest)],
            },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 2],
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    for name in ["Snow-Covered Forest", "Forest"] {
        let obj = find_in_hand(&state, PlayerId(0), name);
        state.zones.hands[0].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(obj);
    }
    state
}

/// Activates `object`'s tap-for-mana ability at the current priority window,
/// then steps until priority returns so the produced mana is actually applied.
/// Mana abilities skip the stack ([CR#605.3b]), so the unit lands in the pool
/// immediately on the following `ManaAdded` apply.
fn tap_for_mana(state: &mut GameState, object: ObjectId) {
    let legal = run_to_priority(state, PlayerId(0), Phase::PrecombatMain);
    let action = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object: o, .. } if *o == object))
        .unwrap_or_else(|| panic!("a tap-for-mana ability on {object:?} at priority: {legal:?}"))
        .clone();
    state.submit_decision(Decision::Act(action)).unwrap();
    // Flush the scheduled `ManaAdded` (and its SBA/trigger checks) back to a
    // priority window so the unit is in the pool when we inspect it.
    let _ = run_to_priority(state, PlayerId(0), Phase::PrecombatMain);
}

/// [CR#107.4h]: any mana produced by a snow source (a permanent with the Snow
/// supertype) is snow mana — its pool unit carries `ManaRider::Snow`. Mana from
/// a non-snow source (a plain Forest) carries no such marker. The Snow marker
/// is read off the source's DERIVED supertypes at the emit site, so it tracks
/// the layered view, not the raw printed face.
#[test]
fn snow_source_mana_carries_snow_rider() {
    let mut state = snow_vs_plain_game(1);
    let find = |state: &GameState, name: &str| {
        *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| is_card(state, o, name))
            .unwrap_or_else(|| panic!("a {name} on the battlefield"))
    };
    let snow = find(&state, "Snow-Covered Forest");
    let plain = find(&state, "Forest");

    // Tap the snow source first, then the plain source. The pool ends with two
    // green units, in tap order: the snow one carries `Snow`, the plain one not.
    tap_for_mana(&mut state, snow);
    tap_for_mana(&mut state, plain);

    let units = state.player(PlayerId(0)).mana_pool.units();
    assert_eq!(units.len(), 2, "two greens floated: {units:?}");
    assert!(
        units[0].riders.contains(&ManaRider::Snow),
        "snow-source mana carries ManaRider::Snow: {:?}",
        units[0],
    );
    assert!(
        !units[1].riders.contains(&ManaRider::Snow),
        "non-snow-source mana carries no Snow rider: {:?}",
        units[1],
    );
}

/// Player 0's ONLY green is `SpendOnly(instant)` (a noncreature restriction);
/// a creature spell cannot be funded by it, so the cast is not offered at
/// precombat-main priority.
#[test]
fn spend_only_instant_cannot_fund_a_creature_spell() {
    let mut state = bears_game(1, 0);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // The only green is instant-only; a plain red covers {1}. No other green is
    // available (no Forests in play), so the creature spell's {G} pip cannot be
    // paid by a spendable unit.
    state.player_mut(PlayerId(0)).mana_pool.add_riders(
        green(),
        1,
        &[ManaRider::SpendOnly(Filter::type_(Type::Instant))],
    );
    state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
    // Re-derive the frozen priority list with the freshly floated pool.
    resurface_priority(&mut state);

    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    assert!(
        !legal.contains(&Action::CastSpell { object: bears }),
        "an instant-only green cannot fund a creature spell, so the cast is not offered: {legal:?}"
    );
}
