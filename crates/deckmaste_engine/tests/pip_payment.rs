//! Per-pip alternative payment (`StaticSpec::PayPips`) — convoke / delve /
//! improvise ([CR#702.51a,702.66a,702.126a]). Proves the cost-payment hook
//! ([CR#601.2g..601.2h]) satisfies an individual pip by tapping a creature /
//! exiling a graveyard card "rather than pay that mana" ([CR#702.51a]) while
//! the spell's total cost and mana value ([CR#202.3]) are never mutated — it
//! "isn't an additional or alternative cost" ([CR#702.51b]).

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::PhaseStep;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::IouKind;
use deckmaste_engine::ManaCoverage;
use deckmaste_engine::ManaPayment;
use deckmaste_engine::ManaPip;
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentStage;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_plugin::plugin::Plugin;

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
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.name,
    }
}

fn printed_mana_value(state: &GameState, id: ObjectId) -> u32 {
    match state.def(id) {
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => f.characteristics.mana_cost.mana_value(),
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
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
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

/// Whether player 0 is offered `Action::CastSpell` for `spell` at its
/// precombat main priority. Payment feasibility is deliberately deferred.
fn cast_is_offered(state: &mut GameState, spell: ObjectId) -> bool {
    resurface_priority(state);
    let legal = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    legal.contains(&Action::CastSpell { object: spell })
}

/// Locks exact coverage for a two-pip spell using one `PayPips` alternative
/// and one floating unit. Coverage selection itself does not enact either
/// payment; fulfillment remains explicit in `Paying`.
fn begin_payment_with_one_pip_alternative(state: &mut GameState, object: ObjectId) {
    let (_, stop) = step_to_stop(state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected the payment protocol after announcement, got {stop:?}");
    };
    assert_eq!(prompt.stage, PaymentStage::PrePayment);
    assert_eq!(
        prompt.outstanding.len(),
        2,
        "the printed two pips stay locked"
    );

    let alternative_iou = prompt
        .outstanding
        .iter()
        .find(|iou| {
            matches!(iou.kind, IouKind::ManaPip(ManaPip::Generic)) && !iou.alternatives.is_empty()
        })
        .expect("a generic pip with the static PayPips alternative")
        .id;
    let floating_iou = prompt
        .outstanding
        .iter()
        .find(|iou| iou.id != alternative_iou)
        .expect("the other printed pip")
        .id;
    let floating = prompt
        .floating_mana
        .first()
        .expect("one exact floating unit for the other pip")
        .id;

    let mut coverage = ManaCoverage::empty();
    coverage.insert(
        alternative_iou,
        ManaPayment::PayPips {
            object,
            alternative: 0,
        },
    );
    coverage.insert(floating_iou, ManaPayment::Floating(floating));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
}

// --- convoke: tap a creature to pay a generic pip ----------------------------

fn convoke_game(seed: u64) -> GameState {
    let convoke = Arc::new(testing().card("Sorcery Convoke Draw").unwrap().core);
    let bear = Arc::new(canon().card("Grizzly Bears").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

#[test]
fn convoke_taps_a_creature_to_pay_a_pip_without_changing_mana_value() {
    let mut state = convoke_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // A creature you control to convoke with.
    let bear = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    force_onto_battlefield(&mut state, bear);

    // Float {G} for the colored pip; convoke will cover {1} by tapping.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(green(), 1, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

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

    begin_payment_with_one_pip_alternative(&mut state, bear);
    // [CR#702.51b,202.3]: the spell's printed cost / mana value is untouched —
    // the pip is still IN the cost, just paid a different way.
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "convoke must not lower the spell's mana value"
    );
    // Coverage is locked before either covered pip is fulfilled.
    assert!(
        !state.objects.obj(bear).tapped,
        "the convoked creature taps as the cost is paid, not before"
    );

    // Fulfill both covered pips, submit, then pass to resolution.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
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
    let delve = Arc::new(testing().card("Sorcery Delve Draw").unwrap().core);
    let island = Arc::new(builtin().card("Island").unwrap().core);
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
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

#[test]
fn delve_exiles_a_graveyard_card_to_pay_a_pip_without_changing_mana_value() {
    let mut state = delve_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // A card in your graveyard to delve away.
    let fodder = find_in_hand(&state, PlayerId(0), "Island");
    force_into_graveyard(&mut state, fodder);

    // Float {1}; delve will cover the other generic pip by exiling the card.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(blue(), 1, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

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

    begin_payment_with_one_pip_alternative(&mut state, fodder);
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
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
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

// --- payment resources do not gate spell proposals -----------------------

#[test]
fn convoke_resources_do_not_gate_a_spell_proposal() {
    let mut state = convoke_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let spell = find_in_hand(&state, PlayerId(0), "Sorcery Convoke Draw");
    let bear = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

    // Float only {G} — one short of {1}{G}. With no creature to convoke, exact
    // payment cannot complete, but that does not gate announcement.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(green(), 1, ManaProvenance::default());
    assert!(
        cast_is_offered(&mut state, spell),
        "payment feasibility is not an announcement gate"
    );

    // Put a creature onto the battlefield: convoke can now cover the {1} by
    // tapping it ([CR#702.51a]), so mana ({G}) plus the pip resource together
    // cover the locked-in cost and the cast becomes legal ([CR#601.2g..601.2h]).
    force_onto_battlefield(&mut state, bear);
    assert!(
        cast_is_offered(&mut state, spell),
        "the spell remains available when convoke can cover the {{1}} pip"
    );
    // Payment / mana value are untouched — pip payment is not a reduction
    // ([CR#702.51b,202.3]).
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "convoke must not lower the spell's mana value"
    );
    assert!(
        !state.objects.obj(bear).tapped,
        "proposal enumeration is read-only: nothing is tapped until payment"
    );
}

#[test]
fn delve_resources_do_not_gate_a_spell_proposal() {
    let mut state = delve_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let spell = find_in_hand(&state, PlayerId(0), "Sorcery Delve Draw");
    let fodder = find_in_hand(&state, PlayerId(0), "Island");

    // Float only {1} — one short of {2}. With an empty graveyard, exact payment
    // cannot complete, but that does not gate announcement.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(blue(), 1, ManaProvenance::default());
    assert!(
        cast_is_offered(&mut state, spell),
        "payment feasibility is not an announcement gate"
    );

    // A card in the graveyard lets delve cover one generic pip by exiling it
    // ([CR#702.66a]); {1} of mana plus that pip resource then cover {2}.
    force_into_graveyard(&mut state, fodder);
    assert!(
        cast_is_offered(&mut state, spell),
        "the spell remains available when delve can cover one generic pip"
    );
    assert_eq!(
        state.objects.obj(fodder).zone,
        Some(Zone::Graveyard),
        "proposal enumeration is read-only: nothing is exiled until payment"
    );
}

// --- improvise: tap an artifact you control to pay a generic pip
// --------------

fn improvise_game(seed: u64) -> GameState {
    let improvise = Arc::new(testing().card("Sorcery Improvise Draw").unwrap().core);
    let myr = Arc::new(canon().card("Darksteel Myr").unwrap().core);
    let island = Arc::new(builtin().card("Island").unwrap().core);
    let mut p0 = vec![Arc::clone(&improvise); 3];
    p0.extend(vec![Arc::clone(&myr); 3]);
    p0.extend(vec![Arc::clone(&island); 10]);
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
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

#[test]
fn improvise_taps_an_artifact_to_pay_a_pip_without_changing_mana_value() {
    let mut state = improvise_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // An artifact you control to improvise with (Darksteel Myr is an Artifact).
    let myr = find_in_hand(&state, PlayerId(0), "Darksteel Myr");
    force_onto_battlefield(&mut state, myr);

    // Float {1}; improvise will cover the other generic pip by tapping the
    // artifact.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(blue(), 1, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let spell = find_in_hand(&state, PlayerId(0), "Sorcery Improvise Draw");
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "{{2}} is mana value 2"
    );
    let library_before = state.zones.libraries[0].len();

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    begin_payment_with_one_pip_alternative(&mut state, myr);
    // [CR#702.51b,202.3]: the spell's printed cost / mana value is untouched —
    // the pip is still IN the cost, just paid a different way.
    assert_eq!(
        printed_mana_value(&state, spell),
        2,
        "improvise must not lower the spell's mana value"
    );
    // Coverage is locked before either covered pip is fulfilled.
    assert!(
        !state.objects.obj(myr).tapped,
        "the improvised artifact taps as the cost is paid, not before"
    );

    // Fulfill both covered pips, submit, then pass to resolution.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("automatic payment decision");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() && !state.zones.hands[0].contains(&spell) {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }

    assert!(
        state.objects.obj(myr).tapped,
        "the improvised artifact is tapped to pay the pip ([CR#702.126a])"
    );
    assert_eq!(
        state.zones.libraries[0].len(),
        library_before - 1,
        "the spell resolved and drew a card"
    );
}

#[test]
fn improvise_resources_do_not_gate_a_spell_proposal() {
    let mut state = improvise_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let spell = find_in_hand(&state, PlayerId(0), "Sorcery Improvise Draw");
    let myr = find_in_hand(&state, PlayerId(0), "Darksteel Myr");

    // Float only {1} — one short of {2}. With no artifact to tap, exact payment
    // cannot complete, but that does not gate announcement.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(blue(), 1, ManaProvenance::default());
    assert!(
        cast_is_offered(&mut state, spell),
        "payment feasibility is not an announcement gate"
    );

    // Put an artifact onto the battlefield: improvise can now cover one generic
    // pip by tapping it ([CR#702.126a]); {1} of mana plus that pip resource then
    // cover {2} ([CR#601.2g..601.2h]).
    force_onto_battlefield(&mut state, myr);
    assert!(
        cast_is_offered(&mut state, spell),
        "the spell remains available when improvise can cover one generic pip"
    );
    assert!(
        !state.objects.obj(myr).tapped,
        "proposal enumeration is read-only: nothing is tapped until payment"
    );
}
