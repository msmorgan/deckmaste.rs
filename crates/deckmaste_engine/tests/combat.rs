//! Combat legality against real canon-card data: summoning sickness
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
use deckmaste_core::BeginningStep;
use deckmaste_core::Card;
use deckmaste_core::CombatStep;
use deckmaste_core::KeywordAbility;
use deckmaste_core::Phase;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionError;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_engine::has_keyword;
use deckmaste_engine::has_keyword_named;
use deckmaste_engine::legal_attackers;
use deckmaste_engine::legal_blockers;

// --- plugin + deck building
// ---------------------------------------------------

fn plugin(name: &str) -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../plugins/{name}")),
    )
    .unwrap()
}

/// Looks `name` up in canon (real cards) first, then in the testing mocks.
fn card(name: &str) -> Arc<Card> {
    let card = plugin("canon")
        .card(name)
        .or_else(|_| plugin("testing").card(name))
        .unwrap();
    Arc::new(card)
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

/// A two-player game: both players play decks of `card`.
fn two_player_with(card: &str, seed: u64, deck_size: usize) -> GameState {
    two_player_decks(card, card, seed, deck_size)
}

/// A two-player game where P0 plays `p0_card` and P1 plays `p1_card`.
fn two_player_decks(p0_card: &str, p1_card: &str, seed: u64, deck_size: usize) -> GameState {
    let p0 = card(p0_card);
    let p1 = card(p1_card);
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
    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

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
    let mut state = two_player_with("Grizzly Bears", 1, 10);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

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
    let mut state = two_player_with("Grizzly Bears", 42, 20);
    let p0_bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let p1_bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
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

/// [CR#702.19]: `has_keyword` reads the PRINTED face abilities. Fangren
/// Hunter carries `abilities: [Keyword(Trample)]` — proving the keyword
/// grammar parses as a known variant through the real plugin loader (macro
/// reader active) — while "Grizzly Bears" carries none.
#[test]
fn has_keyword_reads_printed_face_abilities() {
    let mut state = two_player_decks("Fangren Hunter", "Grizzly Bears", 1, 10);
    let trampler = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let vanilla = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    assert!(
        has_keyword(&state.layers(), trampler, &KeywordAbility::Trample),
        "Fangren Hunter's printed abilities carry Keyword(Trample)"
    );
    assert!(
        !has_keyword(&state.layers(), trampler, &KeywordAbility::Deathtouch),
        "Fangren Hunter carries only Trample, not Deathtouch"
    );
    assert!(
        !has_keyword(&state.layers(), vanilla, &KeywordAbility::Trample),
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
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    // A creature on P0's battlefield BEFORE the game runs: turn 1's begin clears
    // its summoning sickness, so it is a legal attacker this combat.
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

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

/// [CR#508.1f]: the declaration's tap is a real "becomes tapped" transition
/// ([CR#603.2e]), so a `Tapped` fact carrying the attack-declaration cause
/// rides the step trace for becomes-tapped triggers to match.
#[test]
fn declare_attackers_emits_tapped_fact_with_attack_cause() {
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    let (_trace, _stop) = pass_to_stop(&mut state);
    state
        .submit_decision(Decision::Attackers(vec![bear]))
        .unwrap();

    let (trace, _stop) = step_to_stop(&mut state);
    assert!(
        trace.iter().any(|p| matches!(
            p,
            Progress::Applied(Occurrence::Single(GameEvent::Tapped {
                object,
                cause: Some(c),
            })) if *object == bear
                && c.agency == deckmaste_core::Agency::AttackDeclaration
        )),
        "the attack tap rides the trace as a cause-tagged Tapped fact: {trace:?}"
    );
}

/// [CR#702.20b]: a vigilance attacker doesn't tap — no transition
/// ([CR#603.2e]), so no `Tapped` fact is emitted for it. (The untapped
/// status itself is `vigilance_attacker_is_not_tapped`'s concern.)
#[test]
fn vigilant_attacker_emits_no_tapped_fact() {
    let mut state = two_player_decks("Alaborn Grenadier", "Grizzly Bears", 7, 20);
    let vigilant = force_onto_battlefield(&mut state, PlayerId(0), "Alaborn Grenadier");

    let (_trace, _stop) = pass_to_stop(&mut state);
    state
        .submit_decision(Decision::Attackers(vec![vigilant]))
        .unwrap();

    let (trace, _stop) = step_to_stop(&mut state);
    assert!(
        !trace.iter().any(|p| matches!(
            p,
            Progress::Applied(Occurrence::Single(GameEvent::Tapped { object, .. }))
                if *object == vigilant
        )),
        "no Tapped fact for a vigilance attacker: {trace:?}"
    );
}

/// With no legal attacker, the Declare Attackers step still surfaces the
/// decision; the active player declares no attackers with an empty vec.
#[test]
fn declare_attackers_with_no_legal_attacker_accepts_empty() {
    let mut state = two_player_with("Grizzly Bears", 7, 20);
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
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    // P0 (active) gets one attacker; P1 (defender) gets two blockers. P0's
    // creature sheds summoning sickness at turn 1's start, so it can attack;
    // P1's are legal blockers regardless of sickness (untapped creatures).
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

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
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

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
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

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
    let mut state = two_player_decks("Grizzly Bears", "Willow Elf", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Willow Elf");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Willow Elf");

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
    let mut state = two_player_decks("Grizzly Bears", "Willow Elf", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Willow Elf");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Willow Elf");

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
    let mut state = two_player_decks("Centaur Courser", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Centaur Courser");

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
    let mut state = two_player_decks("Centaur Courser", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Centaur Courser");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

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
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

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

/// [CR#508.1a], [CR#508.1m]: a creature with a "whenever ~ attacks" trigger
/// (`StateBecomes(of: Ref(This), becomes: Attacking)`, Library Larcenist)
/// declared as an attacker fires its trigger — the `Attacking` event reached
/// the trigger stage — and, once it resolves, the controller draws a card.
#[test]
fn attacks_trigger_fires_and_resolves() {
    let mut state = two_player_decks("Library Larcenist", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Library Larcenist");

    // Drive to Declare Attackers and declare the triggered creature. Hand size
    // is captured at the decision, AFTER the turn's draw step has run, so the
    // only draw left to observe is the trigger's.
    let (_trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected a DeclareAttackers decision, got {stop:?}");
    };
    let p0_hand_before = state.zones.hands[0].len();
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
    // (P0) draws a card.
    pass_to_postcombat_main(&mut state);
    assert_eq!(
        state.zones.hands[0].len(),
        p0_hand_before + 1,
        "the resolved Draw(1) put a card in the controller's hand"
    );
}

/// [CR#509.3c]: "becomes blocked" fires once even when two creatures block.
/// Canon Deepwood Tantiv ("Whenever this creature becomes blocked, you gain
/// 2 life") double-blocked gains its controller exactly 2 life.
#[test]
fn becomes_blocked_trigger_fires_once_for_double_block() {
    let mut state = two_player_decks("Deepwood Tantiv", "Grizzly Bears", 7, 20);
    let tantiv = force_onto_battlefield(&mut state, PlayerId(0), "Deepwood Tantiv");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    let life_before = state.players[0].life;
    let _stop = drive_through_blocks(&mut state, vec![tantiv], vec![(b1, tantiv), (b2, tantiv)]);
    assert_eq!(
        state.players[0].life,
        life_before + 2,
        "GainLife(2) resolved exactly once ([CR#509.3c])"
    );
}

/// [CR#702.20]: a creature with vigilance is NOT tapped when it attacks.
/// Contrast: the `declare_attackers_taps_records_and_fires_attacking` test
/// above shows that a normal creature IS tapped — this test covers only the
/// vigilance exception to keep concerns separate.
#[test]
fn vigilance_attacker_is_not_tapped() {
    let mut state = two_player_decks("Alaborn Grenadier", "Grizzly Bears", 7, 20);
    let vigilant = force_onto_battlefield(&mut state, PlayerId(0), "Alaborn Grenadier");
    assert!(
        has_keyword(&state.layers(), vigilant, &KeywordAbility::Vigilance),
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

/// [CR#702.15]: a lifelink creature's controller gains life equal to the damage
/// it deals. An unblocked 2/2 with lifelink attacks: the defending player drops
/// from 20 → 18, AND the attacking player (the lifelink source's controller)
/// rises from 20 → 22.
#[test]
fn lifelink_unblocked_attacker_gains_life_for_controller() {
    let mut state = two_player_decks("Steadfast Paladin", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Steadfast Paladin");
    assert!(
        has_keyword_named(&state.layers(), attacker, "Lifelink"),
        "pre-condition: the fixture carries the Lifelink composite keyword"
    );

    assert_eq!(
        state.players[0].life, 20,
        "attacker's controller starts at 20"
    );
    assert_eq!(state.players[1].life, 20, "defender starts at 20");

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![]);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. })
        ),
        "unblocked attacker → forced (one recipient), no assignment decision: {stop:?}"
    );

    assert_eq!(
        state.players[1].life, 18,
        "the unblocked 2/2 lifelink dealt 2 to the defender (20 → 18, [CR#702.15])"
    );
    assert_eq!(
        state.players[0].life, 22,
        "the lifelink creature's controller gained 2 life (20 → 22, [CR#702.15])"
    );
    assert!(
        on_battlefield(&state, attacker),
        "the unblocked attacker took no damage and survives"
    );
}

/// [CR#702.2c,704.5h]: a 1/1 with deathtouch, blocked by a 5/5 vanilla. The
/// deathtouch creature deals 1 damage — normally sublethal against 5 toughness,
/// but any damage from a deathtouch source is lethal ([CR#702.2c]). The SBA
/// sweep destroys the 5/5 ([CR#704.5h]). The 1/1 takes 5 and dies normally too.
#[test]
fn deathtouch_one_damage_kills_five_five() {
    let mut state = two_player_decks("Typhoid Rats", "Grizzled Outrider", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Typhoid Rats");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzled Outrider");
    assert!(
        has_keyword(&state.layers(), attacker, &KeywordAbility::Deathtouch),
        "pre-condition: the fixture carries Keyword(Deathtouch)"
    );

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    // Forced assignment (one recipient each) — no assignment decision.
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. })
        ),
        "forced (one recipient each) → no assignment decision: {stop:?}"
    );

    assert!(
        !on_battlefield(&state, attacker),
        "the 1/1 took 5 from the blocker and is destroyed"
    );
    assert!(
        !on_battlefield(&state, blocker),
        "the 5/5 took 1 deathtouch damage — lethal despite toughness 5 ([CR#704.5h])"
    );
}

/// [CR#702.19b]: a blocked 4/4 trampler over one 2/2 blocker. The trampler's
/// recipients are now `[blocker, defenderProxy]`, so the controller's
/// free-division decision surfaces even for a single blocker (it's a real
/// choice — assign lethal to the blocker and spill the rest to the player). A
/// 2+2 split kills the 2/2 (lethal) and deals 2 to the defender (20 → 18). Two
/// illegal splits are rejected/accepted to pin the lethal-before-player
/// constraint.
#[test]
fn trample_over_one_blocker_spills_excess_to_player() {
    let mut state = two_player_decks("Fangren Hunter", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    assert!(
        has_keyword(&state.layers(), attacker, &KeywordAbility::Trample),
        "pre-condition: the fixture carries Keyword(Trample)"
    );
    let player_proxy = state.players[1].object;

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    // [CR#702.19b]: a single-blocked trampler is a real choice — the decision
    // surfaces with the blocker AND the defending player's proxy as recipients.
    let StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage {
        player,
        source,
        recipients,
    }) = stop
    else {
        panic!("expected an AssignCombatDamage decision for the blocked trampler, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the attacker's controller divides");
    assert_eq!(source, attacker);
    assert!(
        recipients.contains(&blocker)
            && recipients.contains(&player_proxy)
            && recipients.len() == 2,
        "recipients are the live blocker followed by the defender's proxy ([CR#702.19b]): {recipients:?}"
    );

    // [CR#702.19b]: assigning the player while the blocker got < lethal (2) is
    // rejected — 1 on the 2/2 (sublethal) + 3 to the player. The decision stays
    // pending on rejection, so we can still answer it below. (The legal all-to-
    // blocker split `[(blocker, 4), (player, 0)]` is covered by
    // `trample_all_to_blocker_is_legal_no_player_damage`.)
    assert!(
        state
            .submit_decision(Decision::Assignment(vec![(blocker, 1), (player_proxy, 3)]))
            .is_err(),
        "can't assign to the player until each blocker has lethal ([CR#702.19b])"
    );

    // The decision is still pending after the rejection; the canonical legal 2+2
    // split drives to the end state.
    let (_t, _stop) = pass_to_stop_after(
        &mut state,
        Decision::Assignment(vec![(blocker, 2), (player_proxy, 2)]),
    );

    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 blocker took lethal 2 and is destroyed"
    );
    assert_eq!(
        state.players[1].life, 18,
        "the trampler spilled its excess 2 to the defender (20 → 18, [CR#702.19b])"
    );
}

/// [CR#702.19b]: the all-to-blocker split `[(blocker, 4), (player, 0)]` is legal
/// — the controller need not trample over (no player damage means no lethal
/// requirement to satisfy first). The blocker dies; the defender is untouched.
#[test]
fn trample_all_to_blocker_is_legal_no_player_damage() {
    let mut state = two_player_decks("Fangren Hunter", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let player_proxy = state.players[1].object;

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    let StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. }) = stop else {
        panic!("expected an AssignCombatDamage decision, got {stop:?}");
    };

    let (_t, _stop) = pass_to_stop_after(
        &mut state,
        Decision::Assignment(vec![(blocker, 4), (player_proxy, 0)]),
    );

    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 blocker took 4 and is destroyed"
    );
    assert_eq!(
        state.players[1].life, 20,
        "no damage assigned to the player → defender's life is unchanged ([CR#702.19b])"
    );
}

/// [CR#702.2c,702.19b]: a 4/4 with BOTH trample and deathtouch, blocked by a
/// 2/2. Deathtouch makes any nonzero amount lethal for excess-damage purposes
/// ([CR#702.2c]), so lethal(blocker) = 1. The split `[(blocker, 1), (player,
/// 3)]` — REJECTED for a plain trampler — is now VALID: 1 is lethal to the
/// blocker (deathtouch SBA destroys it, [CR#704.5h]) and 3 spills to the
/// defender (20 → 17).
#[test]
fn deathtouch_trample_lethal_is_one_so_one_three_split_is_legal() {
    let mut state = two_player_decks("Trample Deathtouch Creature", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Trample Deathtouch Creature");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    assert!(
        has_keyword(&state.layers(), attacker, &KeywordAbility::Trample)
            && has_keyword(&state.layers(), attacker, &KeywordAbility::Deathtouch),
        "pre-condition: the fixture carries BOTH Keyword(Trample) and Keyword(Deathtouch)"
    );
    let player_proxy = state.players[1].object;

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    let StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { recipients, .. }) = stop
    else {
        panic!("expected an AssignCombatDamage decision, got {stop:?}");
    };
    assert!(
        recipients.contains(&blocker) && recipients.contains(&player_proxy),
        "recipients are the blocker and the defender's proxy: {recipients:?}"
    );

    // [CR#702.2c]: deathtouch makes lethal = 1; assigning 1 to the blocker frees
    // the other 3 to spill to the player. (Rejected without deathtouch — see
    // `trample_over_one_blocker_spills_excess_to_player`.)
    let (_t, _stop) = pass_to_stop_after(
        &mut state,
        Decision::Assignment(vec![(blocker, 1), (player_proxy, 3)]),
    );

    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 blocker took 1 deathtouch damage — lethal ([CR#704.5h])"
    );
    assert_eq!(
        state.players[1].life, 17,
        "the deathtouch trampler spilled 3 to the defender (20 → 17, [CR#702.2c,702.19b])"
    );
}

/// [CR#702.19d]: a blocked trampler whose only blocker leaves combat before
/// damage is assigned deals ALL its damage to the defending player (as though
/// the blocker had been assigned lethal). We declare the block, then — in the
/// post-block priority window — mark the 2/2 blocker with lethal damage so the
/// SBA destroys it (battlefield→graveyard prunes it from combat, [CR#506.4]).
/// The attacker stays sticky-blocked ([CR#509.1h]) but has no live blockers, so
/// its single recipient is the player proxy: the assignment is forced (no
/// decision) and the defender takes the full 4 (20 → 16).
#[test]
fn trample_no_live_blockers_assigns_all_to_player() {
    let mut state = two_player_decks("Fangren Hunter", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Fangren Hunter");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    // Declare the attack and the block; the `Blocked` event applies (sticky
    // blocked status + a live-blocker entry).
    let (_defender, _legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    state
        .submit_decision(Decision::Blocks(vec![(blocker, attacker)]))
        .unwrap();
    // Step to the first priority window after blocks apply, still in Declare
    // Blockers (before the Combat Damage step).
    loop {
        match state.step() {
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => break,
            StepOutcome::Progress(_) => {}
            other => panic!("unexpected stop before combat damage: {other:?}"),
        }
    }
    assert!(
        state.combat.is_blocked(attacker) && state.combat.blockers_of(attacker) == [blocker],
        "pre-condition: the attacker is blocked with one live blocker"
    );

    // Remove the blocker from combat BEFORE the Combat Damage step, through the
    // engine's own machinery: destroy it (a battlefield→graveyard
    // `ZoneWillChange`), which prunes it from combat as it leaves the
    // battlefield ([CR#506.4]). We clear the open priority, front-schedule the
    // destroy + an SBA pass + a fresh `OpenPriority` (re-surfacing the same
    // round), and step it through. This mirrors a removal spell resolving in the
    // post-block priority window, but is set up directly via the public agenda.
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
    state.agenda.push_front(WorkItem::CheckSbas);
    state.agenda.push_front(WorkItem::Emit(Occurrence::single(
        GameEvent::ZoneWillChange {
            object: blocker,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: None,
        },
    )));
    // Drive on: the destroy applies and prunes combat, then play passes through
    // to the Combat Damage step where the trampler sees no live blockers.
    let (_t, _stop) = pass_to_stop(&mut state);

    assert!(
        !on_battlefield(&state, blocker),
        "the blocker was destroyed and left combat before damage was assigned"
    );
    assert_eq!(
        state.players[1].life, 16,
        "all 4 spilled to the defender — no live blockers to satisfy first (20 → 16, [CR#702.19d])"
    );
    assert!(
        on_battlefield(&state, attacker),
        "the trampler took no damage (its blocker was gone) and survives"
    );
}

// --- first strike + double strike ([CR#510.4]) -------------------------------

/// [CR#510.4], [CR#702.7]: Youthful Knight (a 2/1 first-striker) attacks,
/// blocked by a 2/2 vanilla. There are TWO combat-damage steps. In the FIRST
/// one only the first-striker deals — its 2 is lethal to the 2/2 blocker,
/// which the SBA destroys BEFORE the regular step. By the regular step the
/// blocker is gone, so it never deals its 2 back: the first-striker survives
/// the trade, untapped of damage and still on the battlefield.
#[test]
fn first_strike_kills_before_taking_damage() {
    let mut state = two_player_decks("Youthful Knight", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Youthful Knight");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    assert!(
        has_keyword(&state.layers(), attacker, &KeywordAbility::FirstStrike),
        "pre-condition: the fixture carries Keyword(FirstStrike)"
    );

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    // Every source is forced (one recipient each) → no assignment decision in
    // either pass.
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. })
        ),
        "forced (one recipient each) → no assignment decision: {stop:?}"
    );

    // The blocker died to first-strike damage before it could deal any back.
    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 blocker took 2 first-strike damage and is destroyed before the regular step"
    );
    // The first-striker survives — its blocker was dead before the regular step.
    assert!(
        on_battlefield(&state, attacker),
        "the first-striker killed its blocker in the first step and took no damage back ([CR#510.4])"
    );
    assert_eq!(
        state.objects.obj(attacker).damage,
        0,
        "the first-striker has no marked damage — the blocker never struck it"
    );
}

/// [CR#510.4], [CR#702.4]: Boros Swiftblade (a 1/2 double-striker) attacks,
/// blocked by a 2/2 vanilla. In the FIRST step the double-striker deals 1 to
/// the 2/2 — sublethal, so the 2/2 survives, marked 1. In the REGULAR step the
/// double-striker deals 1 MORE (total 2 ≥ 2 → the 2/2 dies) AND the 2/2 deals
/// its 2 back SIMULTANEOUSLY → the 1/2 double-striker dies too. Both die: the
/// second-pass simultaneity is the mutual kill, observable only because the
/// double-striker dealt in BOTH passes.
#[test]
fn double_strike_deals_twice() {
    let mut state = two_player_decks("Boros Swiftblade", "Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Boros Swiftblade");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    assert!(
        has_keyword(&state.layers(), attacker, &KeywordAbility::DoubleStrike),
        "pre-condition: the fixture carries Keyword(DoubleStrike)"
    );

    let stop = drive_through_blocks(&mut state, vec![attacker], vec![(blocker, attacker)]);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::AssignCombatDamage { .. })
        ),
        "forced (one recipient each) → no assignment decision: {stop:?}"
    );

    // 1 (first step) + 1 (regular step) = 2 ≥ 2 → the 2/2 dies.
    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 took 1+1=2 across both steps and is destroyed ([CR#702.4])"
    );
    // In the regular step the 2/2 dealt its 2 back simultaneously → the 1/2 dies.
    assert!(
        !on_battlefield(&state, attacker),
        "the double-striker took the 2/2's 2 back in the second pass and dies (mutual kill, [CR#510.4])"
    );
}

/// [CR#510.4]: with NO attacking or blocking creature having first/double
/// strike, there is only the single regular combat-damage step — the
/// `FirstCombatDamage` step is elided entirely (no `StepBegan`, no priority
/// window for it). The plain 2/2-vs-2/2 trade still resolves exactly as before,
/// and the trace shows the step was skipped, never begun.
#[test]
fn no_first_strike_elides_first_combat_damage_step() {
    let mut state = two_player_with("Grizzly Bears", 7, 20);
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    let blocker = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    // Accumulate the whole combat trace by driving past damage.
    let (trace, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { .. }) = stop else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    state
        .submit_decision(Decision::Attackers(vec![attacker]))
        .unwrap();
    let (trace2, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareBlockers { .. }) = stop else {
        panic!("expected DeclareBlockers, got {stop:?}");
    };
    state
        .submit_decision(Decision::Blocks(vec![(blocker, attacker)]))
        .unwrap();
    let (trace3, _stop) = pass_to_stop(&mut state);

    let full: Vec<Progress> = trace.into_iter().chain(trace2).chain(trace3).collect();

    // The FirstCombatDamage step was SKIPPED — never began (no StepBegan), and
    // the only combat-damage StepBegan is the regular one.
    let began_first = full.iter().any(|p| {
        matches!(
            p,
            Progress::Applied(Occurrence::Single(GameEvent::StepBegan(Phase::Combat(
                CombatStep::FirstCombatDamage
            ))))
        )
    });
    assert!(
        !began_first,
        "no StepBegan(FirstCombatDamage) when nothing has first/double strike ([CR#510.4]): {full:?}"
    );
    let skipped_first = full.iter().any(|p| {
        matches!(
            p,
            Progress::Skipped(Phase::Combat(CombatStep::FirstCombatDamage))
        )
    });
    assert!(
        skipped_first,
        "the FirstCombatDamage step is observably elided (Skipped) ([CR#510.4]): {full:?}"
    );
    // No priority window opened while current was FirstCombatDamage — the step
    // never owned a turn.
    let advanced_to_first = full.iter().any(|p| {
        matches!(
            p,
            Progress::Advanced(Phase::Combat(CombatStep::FirstCombatDamage))
        )
    });
    assert!(
        !advanced_to_first,
        "the FirstCombatDamage step never advanced (was never opened): {full:?}"
    );

    // The plain trade still resolves as before: both 2/2s die.
    assert!(
        !on_battlefield(&state, attacker),
        "the 2/2 attacker took 2 and is destroyed (unchanged single-pass trade)"
    );
    assert!(
        !on_battlefield(&state, blocker),
        "the 2/2 blocker took 2 and is destroyed (unchanged single-pass trade)"
    );
}

/// [CR#508.8]: with no attackers declared, the Declare Blockers step is skipped
/// — no `DeclareBlockers` decision surfaces and play proceeds.
#[test]
fn declare_blockers_skipped_when_no_attackers() {
    let mut state = two_player_with("Grizzly Bears", 7, 20);
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

/// kw-flying goes LIVE: blocking a flier takes flying or reach
/// ([CR#702.9b]) — the first deontic `Cant(Block)` row the engine
/// EVALUATES instead of tripping its presence guard. The ground bear's
/// block is rejected at submission; the reach spider's is accepted.
#[test]
fn flying_attacker_blockable_only_by_flying_or_reach() {
    let strix = card("Baleful Strix");
    let bears = card("Grizzly Bears");
    let spider = card("Giant Spider");
    let mut p1_deck = deck(&bears, 5);
    p1_deck.extend(deck(&spider, 5));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&strix, 10),
            },
            PlayerConfig { deck: p1_deck },
        ],
        seed: 11,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Baleful Strix");
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let spider = force_onto_battlefield(&mut state, PlayerId(1), "Giant Spider");

    let (_, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert!(legal.contains(&bear) && legal.contains(&spider));

    // A ground creature can't block the flier ([CR#702.9b]).
    assert!(matches!(
        state.submit_decision(Decision::Blocks(vec![(bear, attacker)])),
        Err(DecisionError::Illegal { .. })
    ));
    // Reach can — flying's own clause names it ([CR#702.9b,702.17b]).
    state
        .submit_decision(Decision::Blocks(vec![(spider, attacker)]))
        .unwrap();
}

/// kw-menace goes LIVE: the arrangement-level `CountBound` on a
/// `Cant(Block)` row evaluates at block submission ([CR#702.111b] — can't
/// be blocked except by two or more). One bear alone is rejected; two
/// bears gang it legally; declining to block at all stays legal.
#[test]
fn menace_attacker_needs_two_blockers() {
    let brute = card("Boggart Brute");
    let bears = card("Grizzly Bears");
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&brute, 10),
            },
            PlayerConfig {
                deck: deck(&bears, 10),
            },
        ],
        seed: 13,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Boggart Brute");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    let (_, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert!(legal.contains(&b1) && legal.contains(&b2));

    // A lone blocker is a forbidden arrangement ([CR#702.111b]).
    assert!(matches!(
        state.submit_decision(Decision::Blocks(vec![(b1, attacker)])),
        Err(DecisionError::Illegal { .. })
    ));
    // Two or more is legal.
    state
        .submit_decision(Decision::Blocks(vec![(b1, attacker), (b2, attacker)]))
        .unwrap();
}

/// kw-defender goes LIVE: a `Cant(Attack)` row excludes its carrier from
/// the legal attacker set ([CR#702.3b]), and the submission validator
/// rejects it; an unencumbered creature beside it still attacks.
#[test]
fn defender_cannot_be_declared_as_an_attacker() {
    let wall = card("Wall of Stone");
    let bears = card("Grizzly Bears");
    let mut p0_deck = deck(&wall, 5);
    p0_deck.extend(deck(&bears, 5));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0_deck },
            PlayerConfig {
                deck: deck(&bears, 10),
            },
        ],
        seed: 17,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let wall = force_onto_battlefield(&mut state, PlayerId(0), "Wall of Stone");
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    // Shed summoning sickness: both entered before P0's turn begins.
    let (_, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { legal, .. }) = stop else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    assert!(legal.contains(&bear), "the bear attacks freely");
    assert!(
        !legal.contains(&wall),
        "the defender is excluded from the legal attacker set"
    );
    // Submission re-validates against the legal set ([CR#508.1a]).
    assert!(matches!(
        state.submit_decision(Decision::Attackers(vec![wall])),
        Err(DecisionError::Illegal { .. })
    ));
    state
        .submit_decision(Decision::Attackers(vec![bear]))
        .unwrap();
}

/// Must(Attack) goes LIVE: an attack requirement ("attacks each combat if
/// able", [CR#508.1d]) makes any declaration omitting the able carrier
/// illegal; the creature beside it stays free to stay home.
#[test]
fn must_attack_requires_the_able_creature() {
    let brigand = card("Goblin Brigand");
    let bears = card("Grizzly Bears");
    let mut p0_deck = deck(&brigand, 5);
    p0_deck.extend(deck(&bears, 5));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0_deck },
            PlayerConfig {
                deck: deck(&bears, 10),
            },
        ],
        seed: 29,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let brigand = force_onto_battlefield(&mut state, PlayerId(0), "Goblin Brigand");
    let bear = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");

    let (_, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { legal, .. }) = stop else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    assert!(legal.contains(&brigand) && legal.contains(&bear));

    // Omitting the required creature is an illegal declaration ([CR#508.1d]).
    assert!(matches!(
        state.submit_decision(Decision::Attackers(vec![])),
        Err(DecisionError::Illegal { .. })
    ));
    assert!(matches!(
        state.submit_decision(Decision::Attackers(vec![bear])),
        Err(DecisionError::Illegal { .. })
    ));
    // The requirement binds only its carrier — the bear may stay home.
    state
        .submit_decision(Decision::Attackers(vec![brigand]))
        .unwrap();
}

/// "If able" ([CR#508.1d]): a tapped carrier isn't an able attacker, so
/// the requirement is waived and the empty declaration stays legal.
#[test]
fn must_attack_waived_when_unable() {
    let brigand = card("Goblin Brigand");
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&brigand, 10),
            },
            PlayerConfig {
                deck: deck(&brigand, 10),
            },
        ],
        seed: 31,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let brigand = force_onto_battlefield(&mut state, PlayerId(0), "Goblin Brigand");

    // Tap the carrier AFTER the untap step (at the first priority window),
    // so it is still tapped when attackers are declared.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) = stop else {
        panic!("expected a priority window before combat, got {stop:?}");
    };
    state.objects.obj_mut(brigand).tapped = true;

    let (_, stop) = pass_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::DeclareAttackers { legal, .. }) = stop else {
        panic!("expected DeclareAttackers, got {stop:?}");
    };
    assert!(!legal.contains(&brigand), "tapped: not an able attacker");
    state.submit_decision(Decision::Attackers(vec![])).unwrap();
}

/// Must(Block) goes LIVE: "All creatures able to block this creature do
/// so" ([CR#509.1c]) — with the Taunting Elf attacking, every able
/// defender must block it; arrangements leaving one home are illegal.
#[test]
fn must_block_requires_every_able_blocker() {
    let elf = card("Taunting Elf");
    let bears = card("Grizzly Bears");
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: deck(&elf, 10),
            },
            PlayerConfig {
                deck: deck(&bears, 10),
            },
        ],
        seed: 37,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let attacker = force_onto_battlefield(&mut state, PlayerId(0), "Taunting Elf");
    let b1 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let b2 = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    let (_, legal) = drive_to_declare_blockers(&mut state, vec![attacker]);
    assert!(legal.contains(&b1) && legal.contains(&b2));

    // Leaving any able blocker home is an illegal declaration ([CR#509.1c]).
    assert!(matches!(
        state.submit_decision(Decision::Blocks(vec![])),
        Err(DecisionError::Illegal { .. })
    ));
    assert!(matches!(
        state.submit_decision(Decision::Blocks(vec![(b1, attacker)])),
        Err(DecisionError::Illegal { .. })
    ));
    state
        .submit_decision(Decision::Blocks(vec![(b1, attacker), (b2, attacker)]))
        .unwrap();
}
