//! Combat legality against fake testing-plugin data: summoning sickness
//! ([CR#302.6]) and the attacker/blocker legality helpers
//! ([CR#508.1a], [CR#509.1a]).
//!
//! The legality helpers are pure functions over a `GameState`, so most tests
//! build a game, force a creature onto the battlefield, toggle the per-object
//! flags directly (the `GameState` fields are all `pub`), and assert the
//! computed lists. The turn-start clear is driven through the public `step`
//! API.

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{BeginningStep, Card, CombatStep, KeywordAbility, Phase, Zone};
use deckmaste_engine::{
    Action, Decision, GameConfig, GameEvent, GameState, ObjectId, Occurrence, PendingDecision,
    PlayerConfig, PlayerId, Progress, StartingPlayer, StepOutcome, has_keyword, legal_attackers,
    legal_blockers,
};

// --- plugin + deck building
// ---------------------------------------------------

fn testing() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

/// A two-player game: both players play decks of `card`.
fn two_player_with(card: &str, seed: u64, deck_size: usize) -> GameState {
    two_player_decks(card, card, seed, deck_size)
}

/// A two-player game where P0 plays `p0_card` and P1 plays `p1_card`.
fn two_player_decks(p0_card: &str, p1_card: &str, seed: u64, deck_size: usize) -> GameState {
    let p0 = Arc::new(testing().card(p0_card).unwrap());
    let p1 = Arc::new(testing().card(p1_card).unwrap());
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&p0, deck_size),
            },
            PlayerConfig {
                deck: deck(&p1, deck_size),
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

// --- forcing onto the battlefield (copied from tests/stack.rs)
// -----------------

/// The face name of a card-backed object.
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

// --- tests --------------------------------------------------------------------

/// [CR#508.1a], [CR#302.6]: a creature is a legal attacker only when untapped
/// and not summoning-sick.
#[test]
fn legal_attackers_gates_on_sickness_and_tapped() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    // Summoning-sick: not a legal attacker.
    state.objects.obj_mut(bear).summoning_sick = true;
    state.objects.obj_mut(bear).tapped = false;
    assert!(
        !legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a summoning-sick creature can't attack ([CR#302.6])"
    );

    // Not sick, untapped: a legal attacker.
    state.objects.obj_mut(bear).summoning_sick = false;
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a non-sick untapped creature is a legal attacker ([CR#508.1a])"
    );

    // Tapped: not a legal attacker.
    state.objects.obj_mut(bear).tapped = true;
    assert!(
        !legal_attackers(&state, PlayerId(0)).contains(&bear),
        "a tapped creature can't be declared as an attacker ([CR#508.1a])"
    );
}

/// [CR#509.1a]: a summoning-sick creature can still block.
#[test]
fn legal_blockers_ignores_summoning_sickness() {
    let mut state = two_player_with("Vanilla Creature", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    state.objects.obj_mut(bear).summoning_sick = true;
    state.objects.obj_mut(bear).tapped = false;
    assert!(
        legal_blockers(&state, PlayerId(0)).contains(&bear),
        "a summoning-sick creature is still a legal blocker ([CR#509.1a])"
    );

    // But a tapped creature can't block.
    state.objects.obj_mut(bear).tapped = true;
    assert!(
        !legal_blockers(&state, PlayerId(0)).contains(&bear),
        "a tapped creature can't be declared as a blocker ([CR#509.1a])"
    );
}

/// [CR#302.6]: summoning sickness is cleared at the controller's turn start, and
/// **only** for the active player. Put a sick creature on each battlefield and
/// stop at turn 1's untap (P0 active): P0's sheds the flag, P1's keeps it. This
/// isolates the clear — if `begin_turn` cleared nobody, P0's would stay set; if
/// it cleared everybody, P1's would be reset; only the correct behavior passes.
#[test]
fn turn_start_clears_summoning_sickness_for_the_active_player_only() {
    let mut state = two_player_with("Vanilla Creature", 42, 20);
    let p0_bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let p1_bear = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
    state.objects.obj_mut(p0_bear).summoning_sick = true;
    state.objects.obj_mut(p1_bear).summoning_sick = true;

    assert_eq!(state.turn.turn_number, 0, "no turn has begun yet");

    // Step until turn 1's untap step has begun — `begin_turn` (and its clear)
    // runs inside `begin_step(Untap)` before this `Advanced` is returned, so the
    // post-clear state is what we observe. Turn 1's untap is the very first step,
    // before any draw/discard, so no discard decision arises.
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Advanced(Phase::Beginning(BeginningStep::Untap)))
                if state.turn.turn_number == 1 =>
            {
                break;
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }

    assert_eq!(state.turn.turn_number, 1);
    assert_eq!(state.turn.active_player, PlayerId(0));
    assert!(
        !state.objects.obj(p0_bear).summoning_sick,
        "P0's creature shed summoning sickness at P0's turn start ([CR#302.6])"
    );
    assert!(
        state.objects.obj(p1_bear).summoning_sick,
        "P1's creature is NOT cleared on P0's turn — the clear is active-player-only ([CR#302.6])"
    );
    // P0's de-sickened creature is now a legal attacker (untapped, not sick).
    assert!(
        legal_attackers(&state, PlayerId(0)).contains(&p0_bear),
        "the de-sickened creature is a legal attacker"
    );
}

/// [CR#702.19]: `has_keyword` reads the PRINTED face abilities. The "Keyword
/// probe" fixture carries `abilities: [Keyword(Trample)]` — proving the keyword
/// grammar parses as a known variant through the real plugin loader (macro
/// reader active) — while "Vanilla Creature" carries none.
#[test]
fn has_keyword_reads_printed_face_abilities() {
    let mut state = two_player_decks("Keyword probe", "Vanilla Creature", 1, 10);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Keyword probe");
    let vanilla = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    assert!(
        has_keyword(&state, trampler, KeywordAbility::Trample),
        "the probe's printed abilities carry Keyword(Trample)"
    );
    assert!(
        !has_keyword(&state, trampler, KeywordAbility::Deathtouch),
        "the probe carries only Trample, not Deathtouch"
    );
    assert!(
        !has_keyword(&state, vanilla, KeywordAbility::Trample),
        "a vanilla creature has no keywords"
    );
}

/// Steps until the next decision or game end, returning the progress trace.
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
/// priority with Pass. Returns the trace accumulated and the stop.
fn pass_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        let (chunk, stop) = step_to_stop(state);
        trace.extend(chunk);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => return (trace, other),
        }
    }
}

/// [CR#508.1a], [CR#508.1f]: the Declare Attackers step surfaces a
/// `DeclareAttackers` decision for the active player; declaring an attacker
/// taps it, records it in `CombatState`, and fires an `Attacking` event.
#[test]
fn declare_attackers_taps_records_and_fires_attacking() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // A creature on P0's battlefield BEFORE the game runs: turn 1's begin clears
    // its summoning sickness, so it is a legal attacker this combat.
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");

    // Drive (passing priorities) to the Declare Attackers step's decision.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { player, legal }) = stop
    else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert_eq!(
        state.turn.current,
        Phase::Combat(CombatStep::DeclareAttackers)
    );
    assert!(
        legal.contains(&bear),
        "the de-sickened creature is a surfaced legal attacker"
    );

    // Declaring an attacker that isn't in `legal` is rejected.
    assert!(
        state
            .submit_decision(Decision::Attackers(vec![ObjectId(99999)]))
            .is_err(),
        "an id outside the legal set is rejected"
    );
    // A duplicate is rejected.
    assert!(
        state
            .submit_decision(Decision::Attackers(vec![bear, bear]))
            .is_err(),
        "duplicate attackers are rejected"
    );

    // Declare the bear.
    state
        .submit_decision(Decision::Attackers(vec![bear]))
        .unwrap();

    // The Attacking event fires; the bear ends up attacking and tapped.
    let (trace, _stop) = step_to_stop(&mut state);
    assert!(
        trace.iter().any(|p| matches!(
            p,
            Progress::Applied(Occurrence::Batch(events))
                if events.contains(&GameEvent::Attacking(bear))
        )),
        "an Attacking(bear) event appears in the step trace: {trace:?}"
    );
    assert!(
        state.combat.is_attacking(bear),
        "the bear is recorded as an attacker ([CR#508.1a])"
    );
    assert!(
        state.objects.obj(bear).tapped,
        "declaring the bear as an attacker taps it ([CR#508.1f])"
    );
}

/// With no legal attacker, the Declare Attackers step still surfaces the
/// decision; the active player declares no attackers with an empty vec.
#[test]
fn declare_attackers_with_no_legal_attacker_accepts_empty() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // No creature on the battlefield: an empty legal set.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { player, legal }) = stop
    else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert!(legal.is_empty(), "no legal attackers");
    // A nonempty vec is rejected; the empty vec is accepted.
    assert!(
        state
            .submit_decision(Decision::Attackers(vec![ObjectId(99999)]))
            .is_err()
    );
    state.submit_decision(Decision::Attackers(vec![])).unwrap();
    assert!(state.combat.attackers().is_empty());
}

/// Drives to the Declare Attackers decision, declares exactly `attackers`, and
/// then drives (passing priorities) to the Declare Blockers decision. Returns
/// the defending player and the surfaced legal-blocker set.
fn drive_to_declare_blockers(
    state: &mut GameState,
    attackers: Vec<ObjectId>,
) -> (PlayerId, Vec<ObjectId>) {
    let (_trace, stop) = pass_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(attackers))
        .unwrap();
    let (_trace, stop) = pass_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { player, legal }) = stop
    else {
        panic!("expected a DeclareBlockers decision, got {stop:?}");
    };
    (player, legal)
}

/// [CR#509.1a], [CR#509.1h]: after attackers are declared, the Declare Blockers
/// step surfaces a `DeclareBlockers` decision for the **defending**
/// (non-active) player; declaring blocks records them in `CombatState`, marks
/// the attacker blocked, and fires a `Blocked` event per pair. Two blockers may
/// gang one attacker (no ordering decision).
#[test]
fn declare_blockers_records_blocks_and_fires_blocked() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // P0 (active) gets one attacker; P1 (defender) gets two blockers. P0's
    // creature sheds summoning sickness at turn 1's start, so it can attack;
    // P1's are legal blockers regardless of sickness (untapped creatures).
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let (defender, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert_eq!(
        defender,
        PlayerId(1),
        "the defender is the non-active player"
    );
    assert_eq!(
        state.turn.current,
        Phase::Combat(CombatStep::DeclareBlockers)
    );
    assert!(
        legal.contains(&b1) && legal.contains(&b2),
        "both of P1's untapped creatures are surfaced legal blockers: {legal:?}"
    );

    // A blocker outside the legal set is rejected.
    assert!(
        state
            .submit_decision(Decision::Blocks(vec![(ObjectId(99999), attacker)]))
            .is_err(),
        "a blocker outside the legal set is rejected"
    );
    // Blocking a non-attacker is rejected.
    assert!(
        state
            .submit_decision(Decision::Blocks(vec![(b1, ObjectId(88888))]))
            .is_err(),
        "blocking a creature that isn't attacking is rejected"
    );
    // The same blocker blocking twice is rejected ([CR#509.1a]).
    assert!(
        state
            .submit_decision(Decision::Blocks(vec![(b1, attacker), (b1, attacker)]))
            .is_err(),
        "a blocker may block only one attacker ([CR#509.1a])"
    );

    // Gang-block: both b1 and b2 block the single attacker.
    state
        .submit_decision(Decision::Blocks(vec![(b1, attacker), (b2, attacker)]))
        .unwrap();

    let (trace, _stop) = step_to_stop(&mut state);
    let blocked_events: Vec<_> = trace
        .iter()
        .filter_map(|p| match p {
            Progress::Applied(Occurrence::Batch(events)) => Some(events),
            _ => None,
        })
        .flatten()
        .filter(|e| matches!(e, GameEvent::Blocked { .. }))
        .collect();
    assert_eq!(
        blocked_events.len(),
        2,
        "two Blocked events fire (one per pair): {trace:?}"
    );
    assert!(blocked_events.contains(&&GameEvent::Blocked {
        blocker: b1,
        attacker
    }));
    assert!(blocked_events.contains(&&GameEvent::Blocked {
        blocker: b2,
        attacker
    }));

    let blockers = state.combat.blockers_of(attacker);
    assert!(
        blockers.contains(&b1) && blockers.contains(&b2),
        "both blockers are recorded against the attacker: {blockers:?}"
    );
    assert!(
        state.combat.is_blocked(attacker),
        "the attacker is a blocked creature ([CR#509.1h])"
    );
}

/// A single blocker still makes the attacker a blocked creature ([CR#509.1h]).
#[test]
fn declare_blockers_single_blocker_blocks_attacker() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let (defender, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert_eq!(defender, PlayerId(1));
    assert!(legal.contains(&b1));

    state
        .submit_decision(Decision::Blocks(vec![(b1, attacker)]))
        .unwrap();
    let (_trace, _stop) = step_to_stop(&mut state);

    assert_eq!(state.combat.blockers_of(attacker), &[b1]);
    assert!(
        state.combat.is_blocked(attacker),
        "one blocker is enough to make the attacker blocked ([CR#509.1h])"
    );
    assert_eq!(state.combat.attacker_of(b1), Some(attacker));
}

// --- combat damage ([CR#510]) ----------------------------------------------

/// True iff `id` is on the battlefield.
fn on_battlefield(state: &GameState, id: ObjectId) -> bool { state.zones.battlefield.contains(&id) }

/// Drives a freshly-built game to the Declare Attackers decision, declares
/// `attackers`, then declares `blocks` at the Declare Blockers step (skipped
/// when there are no attackers/blocks), and finally passes priority on through
/// to the first stop AT OR INSIDE the Combat Damage step — i.e. either an
/// `AssignCombatDamage` decision (a multi-blocked attacker) or, when every
/// source is forced, the priority decision that opens after damage is dealt.
fn drive_through_blocks(
    state: &mut GameState,
    attackers: Vec<ObjectId>,
    blocks: Vec<(ObjectId, ObjectId)>,
) -> StepOutcome {
    let (_t, stop) = pass_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(attackers))
        .unwrap();
    let (_t, stop) = pass_to_stop(state);
    // With attackers declared the blockers step surfaces; declare the blocks.
    match stop {
        StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. }) => {
            state.submit_decision(Decision::Blocks(blocks)).unwrap();
            let (_t, stop) = pass_to_stop(state);
            stop
        }
        other => other,
    }
}

/// [CR#510.1a..510.1d], [CR#510.2]: a 2/2 attacker blocked by one 2/2.
/// Each assigns 2 to the other — forced (one recipient each, no decision). The
/// damage batch + the lethal SBA then remove both from the battlefield.
#[test]
fn combat_damage_one_block_trades() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    // A forced assignment surfaces NO AssignCombatDamage decision.
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. })
        ),
        "forced (one recipient each) → no assignment decision: {stop:?}"
    );

    assert!(
        !on_battlefield(&state, attacker),
        "the 2/2 attacker took 2 and is destroyed"
    );
    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 blocker took 2 and is destroyed"
    );
}

/// [CR#510.1c], [CR#510.2]: a 2/2 attacker blocked by two 1/1s. The active
/// player (the attacker's controller) is asked to divide the attacker's 2
/// power among the two blockers (free division). A 1+1 split kills both
/// blockers; the attacker takes 1+1 = 2 and dies too.
#[test]
fn combat_damage_two_blockers_split_one_one() {
    let mut state = two_player_decks("Vanilla Creature", "Vanilla 1/1", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla 1/1");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla 1/1");

    let stop = drive_through_blocks(
        &mut state,
        vec![attacker],
        vec![(b1, attacker), (b2, attacker)],
    );
    let StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage {
        player,
        source,
        recipients,
    }) = stop
    else {
        panic!(
            "expected an AssignCombatDamage decision for the multi-blocked attacker, got {stop:?}"
        );
    };
    assert_eq!(player, PlayerId(0), "the attacker's controller divides");
    assert_eq!(source, attacker);
    assert!(
        recipients.contains(&b1) && recipients.contains(&b2) && recipients.len() == 2,
        "the recipients are the two live blockers: {recipients:?}"
    );

    // Wrong total (sums to 3, not the attacker's power 2) is rejected.
    assert!(
        state
            .submit_decision(Decision::Assignment(vec![(b1, 2), (b2, 1)]))
            .is_err(),
        "an assignment whose amounts don't sum to power is rejected"
    );
    // An out-of-recipient target is rejected.
    assert!(
        state
            .submit_decision(Decision::Assignment(vec![(b1, 1), (ObjectId(99999), 1)]))
            .is_err(),
        "an amount on a creature that isn't a recipient is rejected"
    );

    // The legal 1+1 split.
    let (_t, _stop) = pass_to_stop_after(&mut state, Decision::Assignment(vec![(b1, 1), (b2, 1)]));

    assert!(!on_battlefield(&state, b1), "b1 took 1 (lethal for a 1/1)");
    assert!(!on_battlefield(&state, b2), "b2 took 1 (lethal for a 1/1)");
    assert!(
        !on_battlefield(&state, attacker),
        "the attacker took 1+1=2 and dies"
    );
}

/// [CR#510.1c]: free division — a DIFFERENT legal split (2+0) is also valid.
/// All 2 power on b1 kills it; b2 takes nothing and survives; the attacker
/// still takes 1+1 = 2 (both 1/1s dealt their power) and dies.
#[test]
fn combat_damage_two_blockers_split_two_zero() {
    let mut state = two_player_decks("Vanilla Creature", "Vanilla 1/1", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla 1/1");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla 1/1");

    let stop = drive_through_blocks(
        &mut state,
        vec![attacker],
        vec![(b1, attacker), (b2, attacker)],
    );
    let StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. }) = stop else {
        panic!("expected an AssignCombatDamage decision, got {stop:?}");
    };

    // 2 on b1, 0 on b2 — a legal free division ([CR#510.1c]).
    let (_t, _stop) = pass_to_stop_after(&mut state, Decision::Assignment(vec![(b1, 2), (b2, 0)]));

    assert!(!on_battlefield(&state, b1), "b1 took the full 2 and dies");
    assert!(
        on_battlefield(&state, b2),
        "b2 was assigned 0 and survives (free division)"
    );
    assert!(
        !on_battlefield(&state, attacker),
        "the attacker still takes 1+1=2 from the two blockers and dies"
    );
}

/// [CR#510.1b], [CR#510.2]: an unblocked 3/3 deals 3 to the defending player —
/// forced (one recipient, the defender's proxy). Their life goes 20 → 17.
#[test]
fn combat_damage_unblocked_hits_defender() {
    let mut state = two_player_decks("Vanilla 3/3", "Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla 3/3");

    assert_eq!(state.players[1].life, 20);
    let stop = drive_through_blocks(&mut state, vec![attacker], vec![]);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. })
        ),
        "an unblocked attacker is forced (one recipient) → no decision: {stop:?}"
    );
    assert_eq!(
        state.players[1].life, 17,
        "the unblocked 3/3 dealt 3 to the defender (20 → 17, [CR#510.1b])"
    );
    assert!(
        on_battlefield(&state, attacker),
        "the unblocked attacker took no damage and survives"
    );
}

/// Submits `decision`, then passes priority through to the next stop.
fn pass_to_stop_after(state: &mut GameState, decision: Decision) -> (Vec<Progress>, StepOutcome) {
    state.submit_decision(decision).unwrap();
    pass_to_stop(state)
}

// --- end of combat + mid-combat prune + attacks trigger ----------------------

/// Drives on to the post-combat main phase, passing every priority and
/// declaring no further attackers/blockers. Reaching `PostcombatMain` means the
/// End of Combat step (and its turn-based clear, [CR#511.3]) has run.
fn pass_to_postcombat_main(state: &mut GameState) {
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Advanced(Phase::PostcombatMain)) => return,
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. }) => {
                state.submit_decision(Decision::Blocks(vec![])).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
}

/// Drives a freshly-built game to the Declare Attackers decision, declares
/// `attackers`, then declares `blocks` at the Declare Blockers step, stopping
/// the instant the **Combat Damage** step's turn-based action is reached — i.e.
/// after damage has been dealt but before the End of Combat step runs. (Like
/// `drive_through_blocks`, but it stops at the post-damage priority instead of
/// passing it onward.)
fn drive_to_combat_damage_done(
    state: &mut GameState,
    attackers: Vec<ObjectId>,
    blocks: Vec<(ObjectId, ObjectId)>,
) {
    let (_t, stop) = pass_to_stop(state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(attackers))
        .unwrap();
    // Reach the Declare Blockers decision and declare the blocks.
    let mut blocks = Some(blocks);
    loop {
        // Stop at the first priority decision once we're in the Combat Damage
        // step — by then damage has been dealt but End of Combat hasn't run.
        if state.turn.current == Phase::Combat(CombatStep::CombatDamage)
            && matches!(state.pending, Some(PendingDecision::Priority { .. }))
        {
            return;
        }
        match state.step() {
            StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. }) => {
                state
                    .submit_decision(Decision::Blocks(blocks.take().unwrap()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("game ended early: {o:?}"),
        }
    }
}

/// [CR#511.3]: when the End of Combat step ends, all creatures are removed from
/// combat. An unblocked attacker survives the phase; once play reaches the
/// post-combat main phase, `state.combat` is empty — no stale attacker/blocked
/// designation lingers.
#[test]
fn end_of_combat_clears_combat_state() {
    let mut state = two_player_decks("Vanilla 3/3", "Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla 3/3");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    // Attack with the 3/3; the 2/2 blocks it. The 3/3 deals 3 (lethal to the
    // 2/2) and takes 2 — it survives the phase. Drive just past damage, with
    // combat still live.
    drive_to_combat_damage_done(&mut state, vec![attacker], vec![(blocker, attacker)]);
    // While combat is live the surviving attacker is still recorded.
    assert!(
        on_battlefield(&state, attacker),
        "the 3/3 took only 2 and survives to End of Combat"
    );
    assert!(
        state.combat.is_attacking(attacker),
        "the attacker is recorded during combat"
    );
    assert!(
        state.combat.is_blocked(attacker),
        "the blocked designation is set during combat"
    );

    // Run the End of Combat step's clear by passing priority into the post-combat
    // main phase.
    pass_to_postcombat_main(&mut state);

    // [CR#511.3]: every designation is gone.
    assert!(
        state.combat.attackers().is_empty(),
        "no attackers remain after End of Combat ([CR#511.3])"
    );
    assert!(
        !state.combat.is_attacking(attacker),
        "the surviving attacker is no longer designated as attacking"
    );
    assert!(
        !state.combat.is_blocked(attacker),
        "the blocked designation is cleared too"
    );
}

/// [CR#506.4]: a creature that leaves the battlefield is removed from combat
/// immediately. A 2/2 attacker and 2/2 blocker trade in the combat-damage SBA;
/// once both are dead their (now-reminted-away) ids are pruned from the combat
/// registry the instant they leave — they are not in `attackers()` and the
/// blocker is no longer recorded against the attacker.
#[test]
fn mid_combat_death_prunes_combat_state() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Vanilla Creature");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Vanilla Creature");

    let _stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);

    // Both 2/2s took 2 and died in the damage SBA.
    assert!(
        !on_battlefield(&state, attacker),
        "the attacker traded away"
    );
    assert!(!on_battlefield(&state, blocker), "the blocker traded away");

    // [CR#506.4]: each was pruned from combat the moment it left the battlefield,
    // even before End of Combat. The dead ids are stale (death remints), so check
    // by id that they are absent.
    assert!(
        !state.combat.attackers().contains(&attacker),
        "the dead attacker is pruned from the attacker list ([CR#506.4])"
    );
    assert!(
        state.combat.blockers_of(attacker).is_empty(),
        "the dead blocker is pruned from the attacker's live blockers ([CR#506.4])"
    );
    assert_eq!(
        state.combat.attacker_of(blocker),
        None,
        "the dead blocker is gone from the blocks map ([CR#506.4])"
    );
}

/// [CR#508.1a], [CR#603.6]: a creature with a "whenever ~ attacks" trigger
/// (`StateBecomes(of: Is(This), becomes: Attacking)`) declared as an attacker
/// fires its trigger — the `Attacking` event reached the trigger stage — and,
/// once it resolves, the controller loses 1 life.
#[test]
fn attacks_trigger_fires_and_resolves() {
    let mut state = two_player_decks("Attacks trigger", "Vanilla Creature", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Attacks trigger");

    let p0_life_before = state.players[0].life;

    // Drive to Declare Attackers and declare the triggered creature.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(vec![attacker]))
        .unwrap();

    // The Attacking event reaches the trigger stage: a TriggerFired for this
    // creature's ability appears in the step trace.
    let (trace, _stop) = step_to_stop(&mut state);
    let fired = trace.iter().any(|p| {
        matches!(
            p,
            Progress::Applied(Occurrence::Single(GameEvent::TriggerFired { source, .. }))
                if *source == state.objects.obj(attacker).source
        )
    });
    assert!(
        fired,
        "the attacks-trigger fired (the Attacking event reached the trigger stage): {trace:?}"
    );

    // Pass priority through the trigger's placement + resolution; the controller
    // (P0) loses 1 life.
    pass_to_postcombat_main(&mut state);
    assert_eq!(
        state.players[0].life,
        p0_life_before - 1,
        "the resolved LoseLife(1) cost the controller 1 life"
    );
}

/// [CR#702.20]: a creature with vigilance is NOT tapped when it attacks.
/// Contrast: the `declare_attackers_taps_records_and_fires_attacking` test
/// above shows that a normal creature IS tapped — this test covers only the
/// vigilance exception to keep concerns separate.
#[test]
fn vigilance_attacker_is_not_tapped() {
    let mut state = two_player_decks("Vigilance Creature", "Vanilla Creature", 7, 20);
    let vigilant = force_onto_battlefield(&mut state, PlayerId(0), "Vigilance Creature");
    assert!(
        has_keyword(&state, vigilant, KeywordAbility::Vigilance),
        "pre-condition: the fixture carries Keyword(Vigilance)"
    );

    // Drive (passing priorities) to the Declare Attackers step's decision.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { player, legal }) = stop
    else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert!(
        legal.contains(&vigilant),
        "the de-sickened creature is a legal attacker"
    );

    // Declare the vigilance creature as the sole attacker.
    state
        .submit_decision(Decision::Attackers(vec![vigilant]))
        .unwrap();
    let (_trace, _stop) = step_to_stop(&mut state);

    // [CR#702.20]: it IS recorded as attacking but is NOT tapped.
    assert!(
        state.combat.is_attacking(vigilant),
        "the creature is recorded as an attacker ([CR#508.1a])"
    );
    assert!(
        !state.objects.obj(vigilant).tapped,
        "a creature with vigilance is not tapped when it attacks ([CR#702.20])"
    );
}

/// [CR#508.8]: with no attackers declared, the Declare Blockers step is skipped
/// — no `DeclareBlockers` decision surfaces and play proceeds.
#[test]
fn declare_blockers_skipped_when_no_attackers() {
    let mut state = two_player_with("Vanilla Creature", 7, 20);
    // Drive to Declare Attackers and declare none.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    state.submit_decision(Decision::Attackers(vec![])).unwrap();

    // The next decision is NOT a DeclareBlockers — the step was skipped.
    let (_trace, stop) = pass_to_stop(&mut state);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. })
        ),
        "no blockers step when nothing attacked ([CR#508.8]): {stop:?}"
    );
}
