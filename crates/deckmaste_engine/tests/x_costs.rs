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

fn card(name: &str) -> Arc<Card> { Arc::new(canon().card(name).unwrap()) }

fn green() -> deckmaste_core::ColorOrColorless { deckmaste_core::Color::Green.into() }
fn red() -> deckmaste_core::ColorOrColorless { deckmaste_core::Color::Red.into() }

fn find_in_hand(state: &GameState, player: PlayerId, name: &str) -> ObjectId {
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

// --- testing plugin ----------------------------------------------------------

fn testing() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing")).unwrap()
}

fn x_draw() -> Arc<Card> { Arc::new(testing().card("Sorcery X Draw").unwrap()) }

/// Player 0 holds `Sorcery X Draw` ({X}) and Forests; player 1 holds Forests.
fn x_game(seed: u64) -> GameState {
    let xdraw = x_draw();
    let forest = Arc::new(builtin().card("Forest").unwrap());
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
    })
}

// --- tests -------------------------------------------------------------------

#[test]
fn cast_x_draw_announces_pays_and_draws_x() {
    let mut state = x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // Float two generic-payable mana (greens).
    state.player_mut(PlayerId(0)).mana_pool.add(green(), 2);
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    let library_before = state.zones.libraries[0].len();
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: xdraw }))
        .unwrap();

    // [CR#601.2b]: X is announced first.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseXValue { player }) = stop else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(2)).unwrap();

    // Pay {2} (auto), then both players pass so the spell resolves.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
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
fn unpayable_x_rewinds_the_cast() {
    let mut state = x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // Only one mana available; announcing X=5 (cost {5}) is unpayable.
    state.player_mut(PlayerId(0)).mana_pool.add(green(), 1);
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: xdraw }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseXValue { .. }) = stop else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    state.submit_decision(Decision::XValue(5)).unwrap();

    // [CR#733.1]: the spell returned to hand; [CR#733.2]: priority is back with
    // the caster; the pool is untouched.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::Priority { player, .. }) = stop else {
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
fn x_spell_is_offered_when_x_zero_is_affordable() {
    let mut state = x_game(1);
    // No mana floated: {X} at its floor X=0 is {0}, payable with nothing.
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    assert!(
        legal.contains(&Action::CastSpell { object: xdraw }),
        "an {{X}} spell is castable at X=0 with an empty pool: {legal:?}"
    );
}

#[test]
fn x_zero_draws_nothing_and_resolves() {
    let mut state = x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let xdraw = find_in_hand(&state, PlayerId(0), "Sorcery X Draw");
    let library_before = state.zones.libraries[0].len();
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: xdraw }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(PendingDecision::ChooseXValue { .. }) = stop else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    state.submit_decision(Decision::XValue(0)).unwrap();

    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
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
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
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
    })
}

#[test]
fn non_x_cast_surfaces_no_choose_x() {
    let mut state = bolt_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    state.player_mut(PlayerId(0)).mana_pool.add(red(), 1);
    resurface_priority(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    assert!(
        !matches!(
            stop,
            StepOutcome::NeedsDecision(PendingDecision::ChooseXValue { .. })
        ),
        "a non-X cast must not surface ChooseXValue, got {stop:?}"
    );
}

// --- activated {X} ability: announce X on the activation slot too ------------

/// Player 0 holds `Artifact X-activated Draw` ({X}: draw X) and Mountains;
/// player 1 holds Mountains.
fn artifact_x_game(seed: u64) -> GameState {
    let art = Arc::new(testing().card("Artifact X-activated Draw").unwrap());
    let mountain = Arc::new(builtin().card("Mountain").unwrap());
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
    })
}

#[test]
fn activate_x_draw_announces_pays_and_draws_x() {
    let mut state = artifact_x_game(1);
    let _ = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);
    // Force the artifact onto the battlefield (no cast pipeline needed here).
    let art = find_in_hand(&state, PlayerId(0), "Artifact X-activated Draw");
    state.zones.hands[0].retain(|&o| o != art);
    state.objects.obj_mut(art).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(art);
    // Float two generic-payable mana, re-derive priority with the artifact in play.
    state.player_mut(PlayerId(0)).mana_pool.add(red(), 2);
    resurface_priority(&mut state);
    let legal = run_to_priority(&mut state, PlayerId(0), Phase::PrecombatMain);

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
    let StepOutcome::NeedsDecision(PendingDecision::ChooseXValue { player }) = stop else {
        panic!("expected ChooseXValue, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0));
    state.submit_decision(Decision::XValue(2)).unwrap();

    // Pay {2} (auto), then both players pass so the ability resolves.
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(PendingDecision::PayMana { .. }) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
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
