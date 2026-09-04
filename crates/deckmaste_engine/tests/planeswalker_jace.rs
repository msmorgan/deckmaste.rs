//! The canon planeswalker proof: Jace Beleren, loaded through the REAL
//! `Plugin` path so the builtin rules-as-data are wired, driven end-to-end
//! through the public engine API. Unlike the mock-planeswalker loyalty tests
//! in `activate.rs`/`combat.rs` (which `force_*_onto_battlefield` and write
//! `LoyaltyCounter`s directly with an empty `sba_rules`/`conferral_rules`),
//! this proof sources BOTH the committed builtin rules —
//! `rules/grant/planeswalker-loyalty.ron` (enters-with-loyalty, [CR#306.5b])
//! and `rules/sba/loyalty-zero.ron` (the 0-loyalty SBA, [CR#704.5i]) — so the
//! whole planeswalker stack fires organically on a real card:
//!
//!   enter with 3 loyalty → activate `[+2]` to 5 → attack it → it dies.
//!
//! The permanent enters through the engine's own zone-change machinery (a
//! front-scheduled future-form `ZoneChange` to the battlefield, the same public
//! `agenda`/`Emit` path `combat.rs` uses to drive a mid-game zone change), so
//! the conferral replacement is what puts the loyalty counters on — never a
//! hand-set counter map.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::PhaseStep;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_engine::ZoneChange;
use deckmaste_plugin::plugin::Plugin;

const JACE: &str = "Jace Beleren";
const CENTAUR: &str = "Centaur Courser"; // a 3/3 — 3 combat damage = Jace's loyalty
const FILLER: &str = "Grizzly Bears";

// --- plugin loading
// -----------------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// A two-player game with the builtin rules-as-data wired into `GameConfig` (so
/// the enters-with-loyalty conferral and the 0-loyalty SBA both fire), P0
/// playing `p0` and P1 playing `p1`.
fn game_with_rules(p0: Vec<Arc<Card>>, p1: Vec<Arc<Card>>, seed: u64) -> GameState {
    let builtin = builtin();
    let canon = canon();
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: builtin.sba_rules.clone(),
        conferral_rules: builtin.conferral_rules.clone(),
        damage_result_rules: builtin.damage_result_rules.clone(),
        // The counter/subtype/type registries the engine reads; canon
        // inherits the builtin `LoyaltyCounter` decl (and the builtin
        // `TypeDef`s) via its sibling prelude and adds the `Jace` subtype.
        counter_decls: canon.counters.clone(),
        subtypes: canon.subtypes.clone(),
        types: canon.types.clone(),
    })
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
    vec![Arc::clone(card); n]
}

// --- object helpers
// -----------------------------------------------------------

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.name,
    }
}

fn is_card(state: &GameState, id: ObjectId, name: &str) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == name)
}

/// The first `name` object `player` owns, searching hand then library.
fn find_owned(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    state.zones.hands[i]
        .iter()
        .copied()
        .find(|&o| is_card(state, o, name))
        .or_else(|| {
            state.zones.libraries[i]
                .iter()
                .copied()
                .find(|&o| is_card(state, o, name))
        })
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand or library", player.0))
}

/// The battlefield object whose face name is `name`, after a remint moved it
/// there. Panics if none — a silently-fizzled entry should fail the test.
fn on_battlefield_named(state: &GameState, name: &str) -> ObjectId {
    *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("{name} on the battlefield"))
}

fn on_battlefield(state: &GameState, id: ObjectId) -> bool {
    state.zones.battlefield.contains(&id)
}

/// A permanent's current loyalty = its `LoyaltyCounter` count ([CR#306.5c]).
fn loyalty(state: &GameState, pw: ObjectId) -> u32 {
    state
        .objects
        .obj(pw)
        .counters
        .get("LoyaltyCounter")
        .copied()
        .unwrap_or(0)
}

/// Force-place a permanent onto `player`'s battlefield straight from hand or
/// library (the raw zone-write setup the sibling combat/activate tests use for
/// bystanders and attackers — it deliberately SKIPS the enters-with-loyalty
/// replacement, which is only what the planeswalker itself must go through).
fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = find_owned(state, player, name);
    let i = player.index();
    state.zones.hands[i].retain(|&o| o != obj);
    state.zones.libraries[i].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// Front-schedule the object's entry through the engine's zone-change
/// machinery (future-form `ZoneChange` to the battlefield, then an SBA sweep),
/// so the enters-with-loyalty conferral replacement is what puts the loyalty
/// counters on. Does not itself step — the caller drives to the next stop.
fn schedule_entry(state: &mut GameState, obj: ObjectId) {
    let from = state.objects.obj(obj).zone;
    state.agenda.push_front(WorkItem::CheckSbas);
    state
        .agenda
        .push_front(WorkItem::Emit(Occurrence::single(GameEvent::ZoneChange(
            ZoneChange {
                snapshot: None,
                object: obj,
                from,
                to: Zone::Battlefield,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))));
}

// --- stepping helpers (mirrors activate.rs / combat.rs)
// -----------------------

fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

/// Drives to the next non-priority decision (or game over), answering every
/// priority with Pass (mirrors `combat.rs`'s `pass_to_stop`).
fn pass_to_stop(state: &mut GameState) -> StepOutcome {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => return other,
        }
    }
}

/// Drives to the next `Priority` for `player` in `phase`, passing any other
/// priority and auto-paying any legacy or obligation-protocol payment along
/// the way. Returns the legal action list at that window.
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
                    .expect("a Payment prompt has an automatic runner answer");
                state.submit_decision(decision).unwrap();
            }
            other => panic!("unexpected stop before {player:?} priority in {phase:?}: {other:?}"),
        }
    }
}

/// True iff `legal` offers `Action::ActivateAbility { object, ability }`.
fn loyalty_offered(legal: &[Action], object: ObjectId, ability: usize) -> bool {
    legal.iter().any(
        |a| matches!(a, Action::ActivateAbility { object: o, ability: n } if *o == object && *n == ability),
    )
}

/// Activates ability `ability` of `object` at P0's current priority window and
/// pays its explicit loyalty obligation, then passes it through both players
/// to resolution (mirrors `activate.rs`'s `activate_loyalty_and_resolve`).
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

/// Steps to the first window where P0 holds priority during P1's turn (an
/// OFF-turn window for P0), passing/answering everything else. Used to prove a
/// loyalty ability is not offered off its controller's turn ([CR#606.3]).
fn first_p0_priority_on_opponent_turn(state: &mut GameState) -> Vec<Action> {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player, legal },
            )) if player == PlayerId(0) && state.turn.active_player == PlayerId(1) => {
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
            other => panic!("unexpected stop while advancing to P1's turn: {other:?}"),
        }
    }
}

// --- tests --------------------------------------------------------------------

/// Enter → activate: a real Jace Beleren entered through the engine gains 3
/// loyalty from the enters-with-loyalty conferral ([CR#306.5b]); its `[+2]`
/// ability is offered at sorcery speed on P0's own main with an empty stack and
/// raises it to 5 ([CR#606.4]); the shared `LoyaltyOncePerTurn` gate
/// ([CR#606.3,306.5d]) blocks a second activation that turn; and it is not
/// offered at all on the opponent's turn (sorcery-speed only).
#[test]
fn jace_enters_with_loyalty_and_activates_plus_two() {
    let jace = Arc::new(canon().card(JACE).unwrap().core);
    let filler = Arc::new(canon().card(FILLER).unwrap().core);
    let mut p0 = deck(&jace, 1);
    p0.extend(deck(&filler, 19));
    let mut state = game_with_rules(p0, deck(&filler, 20), 1);

    // Enter Jace through the engine's zone-change machinery; the conferral
    // replacement is what puts the loyalty counters on.
    let lib_jace = find_owned(&state, PlayerId(0), JACE);
    schedule_entry(&mut state, lib_jace);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let jace_pw = on_battlefield_named(&state, JACE);
    assert_eq!(
        loyalty(&state, jace_pw),
        3,
        "Jace entered with 3 loyalty via the enters-with-loyalty conferral ([CR#306.5b])"
    );

    // (1) [+2] is offered at sorcery speed on P0's own empty-stack main.
    assert!(
        loyalty_offered(&legal, jace_pw, 0),
        "the [+2] loyalty ability is offered at sorcery speed, legal: {legal:?}"
    );

    // (2) Activate + resolve → 3 + 2 = 5 loyalty.
    activate_loyalty_and_resolve(&mut state, jace_pw, 0);
    assert_eq!(
        loyalty(&state, jace_pw),
        5,
        "[+2] put 2 loyalty counters on Jace (3 → 5, [CR#606.4])"
    );

    // (3) The shared once-per-turn gate blocks a SECOND activation this turn —
    // both [+2] (already fired) and its sibling [−1] ([CR#606.3,306.5d]).
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        !loyalty_offered(&legal, jace_pw, 0),
        "[+2] is blocked — it already fired this turn, legal: {legal:?}"
    );
    assert!(
        !loyalty_offered(&legal, jace_pw, 1),
        "[−1] is blocked too — the once-per-turn limit is shared across Jace's loyalty \
         abilities, legal: {legal:?}"
    );

    // (4) Off-turn: on the opponent's turn no loyalty ability of Jace is
    // offered to P0 (loyalty is sorcery speed, own-main only, [CR#606.3]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = first_p0_priority_on_opponent_turn(&mut state);
    assert_eq!(
        state.turn.active_player,
        PlayerId(1),
        "reached the opponent's turn"
    );
    for ability in 0..3usize {
        assert!(
            !loyalty_offered(&legal, jace_pw, ability),
            "Jace's ability {ability} must not be offered off its controller's turn, \
             legal: {legal:?}"
        );
    }
}

/// Attack → die: a 3/3 attacks a freshly-entered 3-loyalty Jace; the combat
/// damage removes 3 loyalty counters ([CR#120.3c]) and the 0-loyalty
/// state-based action puts Jace into its owner's graveyard ([CR#704.5i]). The
/// defending player takes no damage (damage to a planeswalker is not life
/// loss). Uses the Task 8-10 attack-target path
/// (`Decision::Attackers(vec![(attacker, jace)])`).
#[test]
fn jace_dies_to_combat_damage_via_zero_loyalty_sba() {
    let centaur = Arc::new(canon().card(CENTAUR).unwrap().core);
    let jace = Arc::new(canon().card(JACE).unwrap().core);
    let filler = Arc::new(canon().card(FILLER).unwrap().core);
    let mut p1 = deck(&jace, 1);
    p1.extend(deck(&filler, 19));
    let mut state = game_with_rules(deck(&centaur, 20), p1, 7);

    // P0's 3/3 is force-placed before the turn's untap so it is a legal
    // attacker (the sibling combat tests rely on the same turn-start clear).
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), CENTAUR);

    // P1's Jace enters through the engine (conferral → 3 loyalty); drive to the
    // first stop (P0's precombat-main priority) so its reminted id is live.
    let lib_jace = find_owned(&state, PlayerId(1), JACE);
    schedule_entry(&mut state, lib_jace);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let jace_pw = on_battlefield_named(&state, JACE);
    assert_eq!(
        loyalty(&state, jace_pw),
        3,
        "Jace entered with 3 loyalty via the conferral ([CR#306.5b])"
    );
    assert_eq!(state.players[1].life, 20);

    // Pass the open P0 priority on into combat's Declare Attackers.
    let stop = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::DeclareAttackers(
        deckmaste_engine::DeclareAttackers { .. },
    )) = stop
    else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(vec![(attacker, jace_pw)]))
        .unwrap();
    // P1 has no creatures — declare no blocks, then drive priorities on through
    // the combat-damage step (which removes Jace's loyalty) and the SBA sweep.
    if let StepOutcome::NeedsDecision(DecisionPointKind::DeclareBlockers(
        deckmaste_engine::DeclareBlockers { .. },
    )) = pass_to_stop(&mut state)
    {
        state.submit_decision(Decision::Blocks(vec![])).unwrap();
        let _ = pass_to_stop(&mut state);
    }

    // 3 combat damage → 0 loyalty → the 0-loyalty SBA sends Jace to the
    // graveyard ([CR#120.3c,704.5i]).
    assert!(
        !on_battlefield(&state, jace_pw),
        "0 loyalty → the SBA put Jace into the graveyard ([CR#704.5i])"
    );
    assert!(
        state.zones.graveyards[1]
            .iter()
            .any(|&o| is_card(&state, o, JACE)),
        "Jace is in its owner's graveyard (reminted on the zone change, [CR#704.5i])"
    );
    assert_eq!(
        state.players[1].life, 20,
        "damage to the planeswalker never touched the defending player ([CR#120.3c])"
    );
}
