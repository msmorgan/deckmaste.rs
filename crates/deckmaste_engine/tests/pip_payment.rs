//! Per-pip alternative payment (`StaticEffect::PayPips`) — convoke / delve /
//! improvise ([CR#702.51a,702.66a,702.126a]). Proves the cost-payment hook
//! ([CR#601.2g..601.2h]) satisfies an individual pip by tapping a creature /
//! exiling a graveyard card "rather than pay that mana" ([CR#702.51a]) while
//! the spell's total cost and mana value ([CR#202.3]) are never mutated — it
//! "isn't an additional or alternative cost" ([CR#702.51b]).

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Phase;
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
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;

// --- plugin loaders ----------------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

fn testing() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
}

fn blue() -> deckmaste_core::ColorOrColorless {
    deckmaste_core::Color::Blue.into()
}
fn green() -> deckmaste_core::ColorOrColorless {
    deckmaste_core::Color::Green.into()
}

// --- helpers (mirrors of x_costs.rs) -----------------------------------------

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.name,
    }
}

fn printed_mana_value(state: &GameState, id: ObjectId) -> u32 {
    match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => f.mana_cost.mana_value(),
    }
}

fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    *state.zones.hands[player.index()]
        .iter()
        .find(|&&o| state.objects.obj(o).card_id().is_some() && face_name(state, o) == name)
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand", player.0))
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

/// Re-derives the in-flight priority decision so a freshly injected pool /
/// board is reflected in the legal list (see `x_costs.rs`).
fn resurface_priority(state: &mut GameState) {
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
}

/// Forces `id` from player 0's hand onto the battlefield, untapped — the
/// direct-setup shape `activate.rs` / `x_costs.rs` use for board state.
fn force_onto_battlefield(state: &mut GameState, id: ObjectId) {
    state.zones.hands[0].retain(|&o| o != id);
    state.objects.obj_mut(id).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(id);
}

/// Forces `id` from player 0's hand into player 0's graveyard.
fn force_into_graveyard(state: &mut GameState, id: ObjectId) {
    state.zones.hands[0].retain(|&o| o != id);
    state.objects.obj_mut(id).zone = Some(Zone::Graveyard);
    state.zones.graveyards[0].push(id);
}

// --- convoke: tap a creature to pay a generic pip ----------------------------

fn convoke_game(seed: u64) -> GameState {
    let convoke = Arc::new(testing().card("Sorcery Convoke Draw").unwrap());
    let bear = Arc::new(canon().card("Grizzly Bears").unwrap());
    let forest = Arc::new(builtin().card("Forest").unwrap());
    let mut p0 = vec![Arc::clone(&convoke); 3];
    p0.extend(vec![Arc::clone(&bear); 3]);
    p0.extend(vec![Arc::clone(&forest); 10]);
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 15],
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
    })
}

#[test]
fn convoke_taps_a_creature_to_pay_a_pip_without_changing_mana_value() {
    let mut state = convoke_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // A creature you control to convoke with.
    let bear = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    force_onto_battlefield(&mut state, bear);

    // Float enough mana to pay the FULL {1}{G} so the cast is legal regardless
    // of convoke (the affordability gate is convoke-unaware; the hook still
    // reduces the actual payment). Convoke then pays the {1} by tapping.
    state.player_mut(PlayerId(0)).mana_pool.add(green(), 2);
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    let spell = find_in_hand(&state, PlayerId(0), "Sorcery Convoke Draw");
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "{{1}}{{G}} is mana value 2"
    );
    let library_before = state.zones.libraries[0].len();

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    // First decision after cast: PayMana for the REDUCED cost — convoke covered
    // the generic pip by tapping, so only {G} remains.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = &stop else {
        panic!("expected PayMana after convoke reduced the cost, got {stop:?}");
    };
    assert_eq!(
        cost.mana_value(),
        1,
        "one pip ({{1}}) was paid by tapping; only {{G}} is left to pay with mana"
    );
    // [CR#702.51b,202.3]: the spell's printed cost / mana value is untouched —
    // the pip is still IN the cost, just paid a different way.
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "convoke must not lower the spell's mana value"
    );
    // The tap is scheduled in the payment window, behind the mana decision.
    assert!(
        !state.objects.obj(bear).tapped,
        "the convoked creature taps as the cost is paid, not before"
    );

    // Pay {G}, then pass to resolution.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                if state.stack.is_empty() && !state.zones.hands[0].contains(&spell) {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }

    assert!(
        state.objects.obj(bear).tapped,
        "the convoked creature is tapped to pay the pip ([CR#702.51a])"
    );
    assert_eq!(
        state.zones.libraries[0].len(),
        library_before - 1,
        "the spell resolved and drew a card"
    );
}

// --- delve: exile a graveyard card to pay a generic pip ----------------------

fn delve_game(seed: u64) -> GameState {
    let delve = Arc::new(testing().card("Sorcery Delve Draw").unwrap());
    let island = Arc::new(builtin().card("Island").unwrap());
    let mut p0 = vec![Arc::clone(&delve); 3];
    p0.extend(vec![Arc::clone(&island); 12]);
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&island); 15],
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
    })
}

#[test]
fn delve_exiles_a_graveyard_card_to_pay_a_pip_without_changing_mana_value() {
    let mut state = delve_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    // A card in your graveyard to delve away.
    let fodder = find_in_hand(&state, PlayerId(0), "Island");
    force_into_graveyard(&mut state, fodder);

    // Float the full {2} so the cast is legal; delve then pays one {1} by
    // exiling the graveyard card.
    state.player_mut(PlayerId(0)).mana_pool.add(blue(), 2);
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    let spell = find_in_hand(&state, PlayerId(0), "Sorcery Delve Draw");
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "{{2}} is mana value 2"
    );
    let library_before = state.zones.libraries[0].len();

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::PayMana { cost, .. }) = &stop else {
        panic!("expected PayMana after delve reduced the cost, got {stop:?}");
    };
    assert_eq!(
        cost.mana_value(),
        1,
        "one generic pip was paid by exiling a card; only {{1}} is left to pay with mana"
    );
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "delve must not lower the spell's mana value"
    );
    assert_eq!(
        state.objects.obj(fodder).zone,
        Some(Zone::Graveyard),
        "the card exiles as the cost is paid, not before"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "the delve fodder is still in the graveyard until payment settles"
    );

    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                if state.stack.is_empty() && !state.zones.hands[0].contains(&spell) {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }

    // The exile is a zone move, so the card is reminted to a fresh id; assert
    // over the zone contents rather than the now-stale `fodder` id. The delved
    // Island is in exile; the graveyard holds only the resolved sorcery (a
    // sorcery goes to its owner's graveyard on resolution, [CR#608.2m]).
    let exiled: Vec<&str> = state
        .zones
        .exile
        .iter()
        .map(|&o| face_name(&state, o))
        .collect();
    assert_eq!(
        exiled,
        vec!["Island"],
        "the delved card is exiled to pay the pip ([CR#702.66a])"
    );
    assert!(
        !state.zones.graveyards[0]
            .iter()
            .any(|&o| face_name(&state, o) == "Island"),
        "the delved Island is no longer in the graveyard"
    );
    assert_eq!(
        state.zones.libraries[0].len(),
        library_before - 1,
        "the spell resolved and drew a card"
    );
}
