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
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_plugin::plugin::Plugin;

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
    Arc::new(canon().card(name).unwrap().core)
}

fn green() -> deckmaste_core::ColorOrColorless {
    deckmaste_core::Color::Green.into()
}
fn red() -> deckmaste_core::ColorOrColorless {
    deckmaste_core::Color::Red.into()
}

fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
    fn face_name(state: &GameState, id: ObjectId) -> &str {
        match state.def(id) {
            Card::Normal(f)
            | Card::DoubleFaced { front: f, .. }
            | Card::Split { left: f, .. }
            | Card::Flip { normal: f, .. }
            | Card::Adventurer { normal: f, .. } => &f.name,
        }
    }
    fn is_card(state: &GameState, id: ObjectId, name: &str) -> bool {
        state
            .objects
            .obj(id)
            .card_id()
            .is_some_and(|_| face_name(state, id) == name)
    }
    *state.zones.hands[player.index()]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} in player {}'s hand", player.0))
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
/// any other priority and advancing any payment prompt along the way. Returns
/// the legal action list at that window.
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(prompt)) => {
                let reversals = prompt
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .expect("automatic reversal succeeds");
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
        matches!(
            state.pending,
            Some(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. }
            ))
        ),
        "resurface_priority expects a Priority decision in flight"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
}

// --- testing plugin ----------------------------------------------------------

fn testing() -> Plugin {
    // `load_with_sibling_prelude` so builtin target macros (`AnyTarget`) resolve
    // in testing fixtures — the same loader activate.rs/enumerate.rs use.
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
    )
    .unwrap()
}

fn x_draw() -> Arc<Card> {
    Arc::new(testing().card("Sorcery X Draw").unwrap().core)
}

/// Player 0 holds `Sorcery X Draw` ({X}) and Forests; player 1 holds Forests.
fn x_game(seed: u64) -> GameState {
    let xdraw = x_draw();
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&xdraw); 5];
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

// --- tests -------------------------------------------------------------------

#[test]
fn cast_x_draw_announces_pays_and_draws_x() {
    let mut state = x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // Float two generic-payable mana (greens).
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(green(), 2, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    let library_before = state.zones.libraries[0].len();
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: xdraw }))
        .unwrap();

    // [CR#601.2b]: X is announced first.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { player },
    )) = stop
    else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(2)).unwrap();

    // Pay {2} (auto), then both players pass so the spell resolves.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
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
                state
                    .submit_decision(decision)
                    .expect("automatic payment succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(prompt)) => {
                let reversals = prompt
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .expect("automatic reversal succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() && !state.zones.hands[0].contains(&xdraw) {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }

    assert_eq!(
        state.zones.libraries[0].len(),
        library_before - 2,
        "drew X=2 cards"
    );
}

#[test]
fn unpayable_x_reaches_payment_and_can_be_declined() {
    let mut state = x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // Only one mana available; announcing X=5 (cost {5}) is unpayable.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(green(), 1, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: xdraw }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { .. },
    )) = stop
    else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    state.submit_decision(Decision::XValue(5)).unwrap();

    // Choosing X locks the announced value but does not run an affordability
    // oracle. The complete five-pip obligation reaches the ordinary payment
    // protocol, where the player may decline the proposal.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment after announcing X, got {stop:?}");
    };
    assert_eq!(prompt.stage, deckmaste_engine::PaymentStage::PrePayment);
    assert_eq!(prompt.outstanding.len(), 5);
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(green()),
        1,
        "announcing X did not spend the pool"
    );
    state
        .submit_decision(Decision::Payment(
            deckmaste_engine::PaymentCommand::DeclinePayment,
        ))
        .unwrap();

    // Declining returns the spell to hand and gives priority back to the
    // caster; the pool remains untouched.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = stop
    else {
        panic!("expected Priority after rewind, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    assert!(
        state.zones.hands[0].contains(&xdraw),
        "spell returned to hand"
    );
    assert!(state.stack.is_empty(), "nothing reached the stack");
    assert_eq!(
        state.player(PlayerId(0)).mana_pool.amount(green()),
        1,
        "pool untouched"
    );
    assert!(state.announcing.is_none(), "announce slot cleared");
}

#[test]
fn x_spell_is_offered_before_payment_is_proven() {
    let mut state = x_game(1);
    // No mana is floated, but proposal enumeration defers the eventual payment.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    assert!(
        legal.contains(&Action::CastSpell { object: xdraw }),
        "an {{X}} spell is a legal proposal with an empty pool: {legal:?}"
    );
}

#[test]
fn x_zero_draws_nothing_and_resolves() {
    let mut state = x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    let library_before = state.zones.libraries[0].len();
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: xdraw }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { .. },
    )) = stop
    else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    state.submit_decision(Decision::XValue(0)).unwrap();

    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
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
                state
                    .submit_decision(decision)
                    .expect("automatic payment succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(prompt)) => {
                let reversals = prompt
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .expect("automatic reversal succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() && !state.zones.hands[0].contains(&xdraw) {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }
    assert_eq!(
        state.zones.libraries[0].len(),
        library_before,
        "X=0 drew nothing"
    );
}

// --- bolt_game: non-X regression fixture -------------------------------------

fn bolt_game(seed: u64) -> GameState {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 10]);
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&mountain); 15],
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
fn non_x_cast_surfaces_no_choose_x() {
    let mut state = bolt_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(red(), 1, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
                deckmaste_engine::ChooseXValue { .. }
            ))
        ),
        "a non-X cast must not surface ChooseXValue, got {stop:?}"
    );
}

// --- activated {X} ability: announce X on the activation slot too ------------

/// Player 0 holds `Artifact X-activated Draw` ({X}: draw X) and Mountains;
/// player 1 holds Mountains.
fn artifact_x_game(seed: u64) -> GameState {
    let art = Arc::new(testing().card("Artifact X-activated Draw").unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let mut p0 = vec![Arc::clone(&art); 5];
    p0.extend(vec![Arc::clone(&mountain); 10]);
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&mountain); 15],
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
fn activate_x_draw_announces_pays_and_draws_x() {
    let mut state = artifact_x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // Force the artifact onto the battlefield (no cast pipeline needed here).
    let art = find_in_hand(&state, PlayerId(0), "Artifact X-activated Draw");
    state.zones.hands[0].retain(|&o| o != art);
    state.objects.obj_mut(art).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(art);
    // Float two generic-payable mana, re-derive priority with the artifact in play.
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(red(), 2, ManaProvenance::default());
    resurface_priority(&mut state);
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // Activate ability index 0 of the artifact (the {X}: Draw X ability).
    let act = Action::ActivateAbility {
        object: art,
        ability: 0,
    };
    assert!(
        legal.contains(&act),
        "the {{X}} activated ability is offered: {legal:?}"
    );
    let library_before = state.zones.libraries[0].len();
    state.submit_decision(Decision::Act(act)).unwrap();

    // [CR#601.2b]: X is announced first — on the activation slot too.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { player },
    )) = stop
    else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(2)).unwrap();

    // Pay {2} (auto), then both players pass so the ability resolves.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
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
                state
                    .submit_decision(decision)
                    .expect("automatic payment succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(prompt)) => {
                let reversals = prompt
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .expect("automatic reversal succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }
    assert_eq!(
        state.zones.libraries[0].len(),
        library_before - 2,
        "the activated ability drew X=2 cards"
    );
}

// --- {X} spell that ALSO targets: X-before-targets ordering ------------------

/// Player 0 holds `Sorcery X DealDamage AnyTarget` ({X}: deal X to any target)
/// and Mountains; player 1 holds Mountains.
fn x_burn_game(seed: u64) -> GameState {
    let burn = Arc::new(
        testing()
            .card("Sorcery X DealDamage AnyTarget")
            .unwrap()
            .core,
    );
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let mut p0 = vec![Arc::clone(&burn); 5];
    p0.extend(vec![Arc::clone(&mountain); 10]);
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&mountain); 15],
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
fn cast_x_burn_announces_x_then_targets_then_deals_x() {
    let mut state = x_burn_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(red(), 3, ManaProvenance::default());
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let burn = find_in_hand(&state, PlayerId(0), "Sorcery X DealDamage AnyTarget");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: burn }))
        .unwrap();

    // [CR#601.2b]: X is announced FIRST...
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseXValue(
        deckmaste_engine::ChooseXValue { player },
    )) = stop
    else {
        panic!("expected ChooseXValue first, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(3)).unwrap();

    // [CR#601.2c]: ...then targets. The opponent's player proxy is a legal
    // `AnyTarget` candidate.
    let opp = state.players[1].object;
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets after X, got {stop:?}");
    };
    assert!(
        legal[0].contains(&opp),
        "opponent proxy is a legal AnyTarget: {legal:?}"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![opp]]))
        .unwrap();

    let life_before = state.players[1].life;
    // Pay {3} (auto), then both players pass so the spell resolves.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
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
                state
                    .submit_decision(decision)
                    .expect("automatic payment succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(prompt)) => {
                let reversals = prompt
                    .legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a reversal prompt has a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .expect("automatic reversal succeeds");
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() && !state.zones.hands[0].contains(&burn) {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }
    // The declared-X register read inside DealDamage dealt exactly 3 to the opponent.
    assert_eq!(
        state.players[1].life,
        life_before - 3,
        "dealt X=3 damage to opponent"
    );
}
