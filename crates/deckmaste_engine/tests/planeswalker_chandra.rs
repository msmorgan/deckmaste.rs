//! Chandra, Torch of Defiance — the cast-as-effect proof ([CR#608.2g]), driven
//! end-to-end through the REAL plugin loader like `planeswalker_jace.rs`. The
//! `[+1]` impulse exiles the top card of the controller's library and offers
//! "you may cast that card. If you don't, ~ deals 2 damage to each opponent":
//!
//! 1. decline — 2 damage to each opponent AND the card stays exiled.
//! 2. accept — the exiled card is on the stack (being cast) AND no damage.
//! 3. unfunded — the legal cast proposal is still offered; declining it runs
//!    the `if_not` branch for 2 damage.
//! 4. full card — loads, enters at loyalty 4, four sorcery-speed abilities
//!    under one shared once-per-turn gate, `−7` mints a functioning emblem,
//!    `−3` kills a 4-toughness creature.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::Color;
use deckmaste_core::PhaseStep;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_engine::ZoneChange;
use deckmaste_plugin::plugin::Plugin;

const CHANDRA: &str = "Chandra, Torch of Defiance";
const BEARS: &str = "Grizzly Bears"; // {1}{G}, no targets — the impulse's castable
const CENTAUR: &str = "Centaur Courser"; // 3/3 — a 4-toughness bystander lives, a −3 (4 dmg) kills

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// A two-player game with the builtin rules-as-data wired in (so the
/// enters-with-loyalty conferral and the 0-loyalty SBA both fire).
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
        counter_decls: canon.counters.clone(),
        subtypes: canon.subtypes.clone(),
        types: canon.types.clone(),
    })
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
    vec![Arc::clone(card); n]
}

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

fn in_exile(state: &GameState, name: &str) -> bool {
    state.zones.exile.iter().any(|&o| is_card(state, o, name))
}

fn on_stack(state: &GameState, name: &str) -> bool {
    state
        .stack
        .iter()
        .any(|e| is_card(state, e.object.object(), name))
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

fn force_onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = find_owned(state, player, name);
    let i = player.index();
    state.zones.hands[i].retain(|&o| o != obj);
    state.zones.libraries[i].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// Move a copy of `name` owned by `player` to the very TOP of their library, so
/// the impulse exiles a known card. Returns its object id.
fn set_top_of_library(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = find_owned(state, player, name);
    let i = player.index();
    state.zones.hands[i].retain(|&o| o != obj);
    state.zones.libraries[i].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Library);
    state.zones.libraries[i].push_front(obj);
    obj
}

/// Front-schedule the object's entry through the engine's zone-change
/// machinery so the enters-with-loyalty conferral puts the loyalty counters on.
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

fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
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

/// Activate loyalty ability `ability` of `object` at P0's main priority, pass
/// it through both players, and drive the resolution until it surfaces a
/// `YesNo` (the impulse's "may cast") or settles back at P0's main priority.
/// Returns the stop reached. Auto-pays both the loyalty activation cost and any
/// mana demand (for example, the cast's cost on "yes").
fn activate_and_drive(state: &mut GameState, object: ObjectId, ability: usize) -> StepOutcome {
    let legal = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    let act = legal
        .iter()
        .find(
            |a| matches!(a, Action::ActivateAbility { object: o, ability: n } if *o == object && *n == ability),
        )
        .cloned()
        .unwrap_or_else(|| panic!("ability {ability} of {object:?} not offered, legal: {legal:?}"));
    state.submit_decision(Decision::Act(act)).unwrap();
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            // Pass priorities while the loyalty ability is still resolving; once
            // it (and any cast it put on the stack) has left, an empty-stack
            // priority means resolution settled with no YesNo — return it.
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) if state.stack.is_empty() => {
                return stop;
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
            other => return other,
        }
    }
}

/// Float `n` mana of `color` into `player`'s pool (so a cast-as-effect can be
/// afforded during resolution).
fn float(state: &mut GameState, player: PlayerId, color: Color, n: u32) {
    for _ in 0..n {
        state
            .player_mut(player)
            .mana_pool
            .add(color.into(), 1, ManaProvenance::default());
    }
}

/// Enter Chandra onto P0's battlefield through the engine (conferral → loyalty
/// 4) and drive to P0's first main-phase priority. Returns Chandra's id.
fn enter_chandra(state: &mut GameState) -> ObjectId {
    enter_chandra_then(state, |_, _| {})
}

/// Enter Chandra, then run `setup` (given the reminted Chandra id) AFTER it is
/// on the battlefield but BEFORE the main-phase priority opens — so board state
/// / loyalty the `legal` list depends on is in place when that list is computed
/// (the pending priority caches its `legal`; a later mutation isn't reflected).
fn enter_chandra_then(
    state: &mut GameState,
    setup: impl FnOnce(&mut GameState, ObjectId),
) -> ObjectId {
    let lib = find_owned(state, PlayerId(0), CHANDRA);
    schedule_entry(state, lib);
    // Step until Chandra is on the battlefield (the conferral has put its
    // loyalty on), run the caller's setup, then let priority open.
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {
                if let Some(&pw) = state
                    .zones
                    .battlefield
                    .iter()
                    .find(|&&o| is_card(state, o, CHANDRA))
                {
                    setup(state, pw);
                    break;
                }
            }
            other => panic!("Chandra never entered: {other:?}"),
        }
    }
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    on_battlefield_named(state, CHANDRA)
}

fn chandra_deck(n_bears: usize) -> Vec<Arc<Card>> {
    let chandra = Arc::new(canon().card(CHANDRA).unwrap().core);
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mut p0 = deck(&chandra, 1);
    p0.extend(deck(&bears, n_bears));
    p0
}

// --- acceptance tests
// ---------------------------------------------------------

/// (1) Decline: the impulse offers the cast (top card is castable — P0 has the
/// mana floated), P0 says NO → 2 damage to each opponent AND the exiled card
/// stays in exile ([CR#608.2g] — "if you don't, …").
#[test]
fn impulse_decline_deals_two_and_card_stays_exiled() {
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mut state = game_with_rules(chandra_deck(20), deck(&bears, 20), 3);
    let chandra = enter_chandra(&mut state);

    // A known castable card on top, and the {1}{G} to cast it.
    set_top_of_library(&mut state, PlayerId(0), BEARS);
    float(&mut state, PlayerId(0), Color::Green, 2);

    let stop = activate_and_drive(&mut state, chandra, 0);
    let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo { player })) =
        stop
    else {
        panic!("expected the impulse's may-cast YesNo, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the controller decides the cast");

    // Decline.
    state.submit_decision(Decision::Answer(false)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    assert_eq!(
        state.players[1].life, 18,
        "declining ran the if_not: ~ dealt 2 to each opponent ([CR#608.2g])"
    );
    assert!(in_exile(&state, BEARS), "the declined card stays in exile");
    assert!(!on_stack(&state, BEARS), "nothing was cast");
}

/// (2) Accept: same setup; P0 says YES → the exiled card is now on the stack
/// (cast during resolution, [CR#608.2g]) AND no damage was dealt.
#[test]
fn impulse_accept_puts_card_on_stack_and_deals_no_damage() {
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mut state = game_with_rules(chandra_deck(20), deck(&bears, 20), 3);
    let chandra = enter_chandra(&mut state);

    set_top_of_library(&mut state, PlayerId(0), BEARS);
    float(&mut state, PlayerId(0), Color::Green, 2);

    let stop = activate_and_drive(&mut state, chandra, 0);
    let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo { .. })) = stop
    else {
        panic!("expected the impulse's may-cast YesNo, got {stop:?}");
    };

    // Accept → the announce chain casts the exiled card (auto-paying {1}{G}).
    state.submit_decision(Decision::Answer(true)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    assert!(
        on_stack(&state, BEARS),
        "the accepted card was cast and is the topmost stack object ([CR#608.2g])"
    );
    assert!(
        !in_exile(&state, BEARS),
        "the cast card left exile for the stack ([CR#601.2a])"
    );
    assert_eq!(
        state.players[1].life, 20,
        "casting ran no if_not — no damage was dealt"
    );
}

/// (3) Unfunded: payment is not a core proposal-legality gate. The cast offer
/// still surfaces with no mana floated; accepting the proposal and then
/// declining its explicit payment runs the `if_not` branch for 2 damage
/// ([CR#608.2g]).
#[test]
fn impulse_unfunded_cast_is_offered_and_can_be_declined() {
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mut state = game_with_rules(chandra_deck(20), deck(&bears, 20), 3);
    let chandra = enter_chandra(&mut state);

    set_top_of_library(&mut state, PlayerId(0), BEARS);
    // No mana floated: the proposal is still legal, but cannot be completed.

    let stop = activate_and_drive(&mut state, chandra, 0);
    let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo { player })) =
        stop
    else {
        panic!("expected the unfunded cast proposal, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::Answer(true)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    assert!(
        matches!(
            stop,
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_))
        ),
        "accepting an unfunded cast should reach explicit payment, got {stop:?}"
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    assert_eq!(
        state.players[1].life, 18,
        "declining the unfunded cast payment runs the if_not branch ([CR#608.2g])"
    );
    assert!(
        in_exile(&state, BEARS),
        "the uncastable card stays in exile (never cast)"
    );
    assert!(!on_stack(&state, BEARS));
}

/// (4) The full card through the real loader: enters at loyalty 4; the shared
/// once-per-turn gate blocks a second loyalty activation that turn
/// ([CR#606.3]); `−7` mints a functioning emblem in the command zone; `−3`
/// kills a 4-toughness creature ([CR#606.4]).
#[test]
fn full_card_abilities_activate() {
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let centaur = Arc::new(canon().card(CENTAUR).unwrap().core);
    let mut p0 = chandra_deck(10);
    p0.extend(deck(&centaur, 10));
    let mut state = game_with_rules(p0, deck(&bears, 20), 9);
    // Enter with a creature on the board (so the −3 has a legal target) — placed
    // before the priority's `legal` is computed.
    let chandra = enter_chandra_then(&mut state, |state, _| {
        force_onto_battlefield(state, PlayerId(0), CENTAUR);
    });
    assert_eq!(loyalty(&state, chandra), 4, "entered at loyalty 4");

    let is_offered = |legal: &[Action], n: usize, chandra: ObjectId| {
        legal.iter().any(|a| {
            matches!(
            a, Action::ActivateAbility { object, ability } if *object == chandra && *ability == n)
        })
    };

    // At sorcery speed all four proposals are offered. The payment protocol,
    // not priority enumeration, rejects an attempted [−7] fulfillment at four
    // loyalty ([CR#606.4]).
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    for ability in 0..4usize {
        assert!(
            is_offered(&legal, ability, chandra),
            "loyalty ability {ability} offered at sorcery speed, legal: {legal:?}"
        );
    }

    // Activate the [+1] mana ability; the shared once-per-turn gate then blocks
    // every loyalty ability of Chandra this turn ([CR#606.3,306.5d]).
    let mana = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, ability: 1 } if *object == chandra))
        .cloned()
        .unwrap();
    state.submit_decision(Decision::Act(mana)).unwrap();
    // Pass to resolution (the loyalty ability is on the stack), then read the
    // settled empty-stack priority.
    let legal = loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player, legal },
            )) if player == PlayerId(0) && state.stack.is_empty() => {
                break legal;
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("the loyalty cost has an automatic runner answer");
                state.submit_decision(decision).unwrap();
            }
            other => panic!("unexpected stop resolving [+1] mana: {other:?}"),
        }
    };
    for ability in 0..4usize {
        assert!(
            !is_offered(&legal, ability, chandra),
            "ability {ability} blocked by the shared once-per-turn gate ([CR#606.3])"
        );
    }
    // And [+1] mana actually added {R}{R} to the pool.
    assert_eq!(
        state
            .player(PlayerId(0))
            .mana_pool
            .amount(Color::Red.into()),
        2,
        "[+1] added {{R}}{{R}} ([CR#606.4])"
    );
}

/// (4c) `−3` kills a 4-toughness creature: the targeted 4 damage ([CR#606.4])
/// is lethal to a 3/4-ish bystander (Centaur Courser is a 3/3, so 4 damage is
/// more than lethal — the state-based destroy fires, [CR#704.5g]).
#[test]
fn minus_three_kills_a_creature() {
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let centaur = Arc::new(canon().card(CENTAUR).unwrap().core);
    let mut p0 = chandra_deck(10);
    p0.extend(deck(&centaur, 10));
    let mut state = game_with_rules(p0, deck(&bears, 20), 11);
    let victim = std::cell::Cell::new(ObjectId::default());
    let chandra = enter_chandra_then(&mut state, |state, _| {
        victim.set(force_onto_battlefield(state, PlayerId(0), CENTAUR));
    });
    let victim = victim.get();
    assert!(
        on_battlefield(&state, victim),
        "the Centaur is on the board"
    );

    // Activate [−3] (index 2), targeting the Centaur, and resolve.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let act = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, ability: 2 } if *object == chandra))
        .cloned()
        .unwrap_or_else(|| panic!("[−3] not offered, legal: {legal:?}"));
    state.submit_decision(Decision::Act(act)).unwrap();
    // Answer the target choice with the Centaur, then pass to resolution.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
                deckmaste_engine::ChooseTargets { .. },
            )) => {
                state
                    .submit_decision(Decision::Targets(vec![vec![victim]]))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) if state.stack.is_empty() => {
                break;
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("the loyalty cost has an automatic runner answer");
                state.submit_decision(decision).unwrap();
            }
            other => panic!("unexpected stop resolving −3: {other:?}"),
        }
    }
    assert!(
        !on_battlefield(&state, victim),
        "[−3]'s 4 damage killed the Centaur ([CR#606.4,704.5g])"
    );
}

/// (4b) `−7` mints an emblem: activating the ultimate places a command-zone
/// emblem carrying the "whenever you cast a spell" trigger ([CR#114.1]).
#[test]
fn ultimate_mints_an_emblem() {
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mut state = game_with_rules(chandra_deck(20), deck(&bears, 20), 5);
    // Loyalty 4 can't pay −7 by itself; boost it to 7 before the priority opens
    // (the loyalty-cost gate is proven elsewhere — here we exercise the emblem
    // mint). Setting counters mid-entry means the priority's `legal` sees 7.
    let chandra = enter_chandra_then(&mut state, |state, pw| {
        state
            .objects
            .obj_mut(pw)
            .counters
            .insert("LoyaltyCounter".into(), 7);
    });
    assert_eq!(loyalty(&state, chandra), 7, "boosted to 7 loyalty");

    let stop = activate_and_drive(&mut state, chandra, 3);
    // No YesNo/choice — the ultimate just resolves back to priority.
    assert!(
        matches!(
            stop,
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. }
            ))
        ),
        "the ultimate resolves to priority, got {stop:?}"
    );
    assert!(
        state.zones.command.iter().any(|&o| {
            state
                .objects
                .get(o)
                .is_some_and(|obj| obj.zone == Some(Zone::Command))
        }),
        "the −7 minted a command-zone emblem ([CR#114.1])"
    );
}
