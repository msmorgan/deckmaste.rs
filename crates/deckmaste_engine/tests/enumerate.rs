//! Renderer-facing enumeration queries: a UI can read every legal option at a
//! decision point — cost, ability, source, decider — without re-deriving
//! legality. Harness modeled on `tests/activate.rs`.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::PhaseStep;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::ActionViewKind;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_plugin::plugin::Plugin;

const PINGER: &str = "Creature tap-activated DealDamage AnyTarget";
const INSTANT: &str = "Lightning Bolt";
const BEARS: &str = "Grizzly Bears";

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

/// A two-player game whose p0 deck is Lightning Bolt + Mountains and whose p1
/// deck is Grizzly Bears + Forests — the canon-spell counterpart of
/// `activation_game`. Callers force the specific permanents/hand cards they
/// need.
fn bolt_game(seed: u64) -> GameState {
    let bolt = Arc::new(canon().card(INSTANT).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
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

/// Steps until a `Priority` decision surfaces for `player` in `phase`, passing
/// any other priority along the way. Returns the legal action list at that
/// window.
///
/// When a payment decision surfaces mid-announce, this function uses the
/// engine's deterministic monocolor runner shim and continues. Tests that need
/// a specific payment transcript must answer the protocol explicitly before
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
            .find(
                |a| matches!(a, Action::ActivateAbility { object, .. } if is_land(state, *object)),
            )
            .cloned()
            .expect("an untapped land with a mana ability");
        state.submit_decision(Decision::Act(tap)).unwrap();
        let _ = run_to_priority(state, player, state.turn.current);
    }
}

/// Moves the first `name` card from `player`'s library into their hand and
/// returns its id; falls back to an existing copy already in hand. Lets a test
/// guarantee a specific castable/playable card without depending on the
/// shuffle.
fn force_into_hand(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    if let Some(&obj) = state.zones.libraries[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
    {
        state.zones.libraries[i].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Hand);
        state.zones.hands[i].push(obj);
        return obj;
    }
    find_in_hand(state, player, name)
}

// --- tests --------------------------------------------------------------------

#[test]
fn mana_cost_is_publicly_reachable_for_a_castable_spell() {
    let mut state = bolt_game(1);
    let bolt_id = force_into_hand(&mut state, PlayerId(0), INSTANT);

    // mana_cost is now public: a renderer can read a castable spell's cost.
    let cost = state
        .mana_cost(bolt_id)
        .expect("Lightning Bolt has a mana cost");
    // Lightning Bolt costs {R}: exactly one colored symbol, no generic.
    assert_eq!(cost.mana_value(), 1, "Bolt is mana value 1");
}

#[test]
fn abilities_index_matches_activate_ability_action() {
    // PINGER is a creature with a tap-activated non-mana ability; activation_game
    // forces one Mountain onto p0's battlefield and stocks the deck with PINGER.
    let mut state = activation_game(7, PINGER, 1);
    let pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    let _legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // Find the offered ActivateAbility for the pinger and read its ability back
    // by the SAME index — that round-trip is the public indexing contract.
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        legal,
        ..
    })) = state.step()
    else {
        panic!("expected priority");
    };
    let idx = legal
        .iter()
        .find_map(|a| match a {
            Action::ActivateAbility { object, ability } if *object == pinger => Some(*ability),
            _ => None,
        })
        .expect("pinger offers an activated ability");

    let abilities = state.abilities(pinger);
    assert!(
        idx < abilities.len(),
        "the Action index is in range of abilities()"
    );
    // The indexed ability resolves as an activated ability.
    assert!(
        state.activated_ability(pinger, idx).is_some(),
        "activated_ability returns the activated ability at the offered index"
    );
}

#[test]
fn mana_ability_identifies_a_mountains_tap_for_red() {
    let state = activation_game(7, PINGER, 1);
    // The Mountain forced onto the battlefield by activation_game.
    let mountain = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(&state, o, "Mountain"))
        .expect("a Mountain on the battlefield");

    // Its derived (basic-land-conferred) abilities include a tap-for-{R}.
    let abilities = state.abilities(mountain);
    let mana_idx = (0..abilities.len())
        .find(|&i| state.mana_ability(mountain, i).is_some())
        .expect("Mountain has a derived mana ability");
    assert_eq!(
        state.mana_ability(mountain, mana_idx),
        Some((red(), 1)),
        "Mountain taps for one red"
    );
    // And that same ability is an activated ability.
    assert!(state.activated_ability(mountain, mana_idx).is_some());
}

#[test]
fn decision_point_exposes_the_decider_player() {
    let mut state = activation_game(7, PINGER, 1);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = state.step()
    else {
        panic!("expected priority");
    };

    let dp = state.decision_point().expect("a pending decision");
    assert_eq!(
        dp.decider_player(),
        player,
        "DecisionPoint::decider_player matches the pending decision's player"
    );
}

#[test]
fn describe_action_bundles_cast_land_activate_pass_concede() {
    let bolt = Arc::new(canon().card(INSTANT).unwrap().core);
    let pinger = Arc::new(testing().card(PINGER).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 4];
    p0.extend(vec![Arc::clone(&pinger); 3]);
    p0.extend(vec![Arc::clone(&mountain); 6]);
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 3,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let pinger_id = force_into_play(&mut state, PlayerId(0), PINGER);
    let mountain_id = force_into_play(&mut state, PlayerId(0), "Mountain");
    let bolt_id = force_into_hand(&mut state, PlayerId(0), INSTANT);
    let land_id = force_into_hand(&mut state, PlayerId(0), "Mountain");

    // CastSpell view carries the name and cost.
    let v = state.describe_action(&Action::CastSpell { object: bolt_id });
    assert_eq!(v.source, Some(bolt_id));
    assert_eq!(v.name, Some("Lightning Bolt"));
    match v.kind {
        ActionViewKind::Cast { cost: Some(c) } => assert_eq!(c.mana_value(), 1),
        other => panic!("expected Cast with a cost, got {other:?}"),
    }

    // PlayLand view: name, PlayLand kind.
    let v = state.describe_action(&Action::PlayLand { object: land_id });
    assert_eq!(v.name, Some("Mountain"));
    assert!(matches!(v.kind, ActionViewKind::PlayLand));

    // A mana ability (Mountain) -> Activate { mana: true }.
    let mab = state.abilities(mountain_id);
    let m_idx = (0..mab.len())
        .find(|&i| state.mana_ability(mountain_id, i).is_some())
        .unwrap();
    let v = state.describe_action(&Action::ActivateAbility {
        object: mountain_id,
        ability: m_idx,
    });
    assert_eq!(v.name, Some("Mountain"));
    assert!(matches!(
        v.kind,
        ActionViewKind::Activate { mana: true, .. }
    ));

    // A non-mana ability (PINGER) -> Activate { mana: false }.
    let pab = state.abilities(pinger_id);
    let p_idx = (0..pab.len())
        .find(|&i| {
            state.activated_ability(pinger_id, i).is_some()
                && state.mana_ability(pinger_id, i).is_none()
        })
        .expect("pinger has a non-mana activated ability");
    let v = state.describe_action(&Action::ActivateAbility {
        object: pinger_id,
        ability: p_idx,
    });
    assert!(matches!(
        v.kind,
        ActionViewKind::Activate { mana: false, .. }
    ));

    // Pass / Concede carry no source/name.
    let v = state.describe_action(&Action::Pass);
    assert_eq!(v.source, None);
    assert!(matches!(v.kind, ActionViewKind::Pass));
    let v = state.describe_action(&Action::Concede);
    assert!(matches!(v.kind, ActionViewKind::Concede));
}

#[test]
fn priority_enumerates_pass_concede_activate_and_land() {
    let mut state = activation_game(7, PINGER, 1);
    let _pinger = force_into_play(&mut state, PlayerId(0), PINGER);
    let land = force_into_hand(&mut state, PlayerId(0), "Mountain");
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    assert!(legal.contains(&Action::Pass), "Pass is always offered");
    assert!(
        legal.contains(&Action::Concede),
        "Concede is always offered [CR#104.3a]"
    );
    assert!(
        legal.contains(&Action::PlayLand { object: land }),
        "the land in hand is playable at sorcery speed"
    );
    assert!(
        legal
            .iter()
            .any(|a| matches!(a, Action::ActivateAbility { .. })),
        "an activated ability (the pinger and/or the Mountain) is offered"
    );
}

#[test]
fn cast_spell_is_enumerated_before_payment_is_proven() {
    // Build a deck with Bolt + Mountains so the same spell can be observed
    // before and after floating {R}.
    let mut state = bolt_game(2);
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let bolt_id = force_into_hand(&mut state, PlayerId(0), INSTANT);

    // Announcing a spell no longer asks an affordability oracle to prove the
    // eventual payment, so the empty pool does not hide Bolt.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        legal.contains(&Action::CastSpell { object: bolt_id }),
        "Bolt is a legal proposal with an empty pool"
    );

    // Floating {R} changes how the later payment can be completed, not whether
    // the proposal appears in a priority window.
    float_mana(&mut state, PlayerId(0), 1);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        legal,
        ..
    })) = state.step()
    else {
        panic!("expected priority");
    };
    assert!(
        legal.contains(&Action::CastSpell { object: bolt_id }),
        "Bolt is castable once {{R}} is floated"
    );
}

#[test]
fn priority_enumerates_all_action_kinds_at_one_window() {
    // One board: a castable instant in hand, two tap-mana lands, a non-mana
    // activated-ability creature (pinger), and a land in hand. After floating
    // one red, the legal list at p0's main-phase priority offers every action
    // kind in a SINGLE window — CastSpell, the pinger's ActivateAbility, an
    // untapped Mountain's mana ActivateAbility, PlayLand, Pass, and Concede.
    let bolt = Arc::new(canon().card(INSTANT).unwrap().core);
    let pinger = Arc::new(testing().card(PINGER).unwrap().core);
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let bears = Arc::new(canon().card(BEARS).unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 4];
    p0.extend(vec![Arc::clone(&pinger); 3]);
    p0.extend(vec![Arc::clone(&mountain); 8]);
    let mut p1 = vec![Arc::clone(&bears); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 5,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let pinger_id = force_into_play(&mut state, PlayerId(0), PINGER);
    force_into_play(&mut state, PlayerId(0), "Mountain"); // Mountain A (will be tapped)
    force_into_play(&mut state, PlayerId(0), "Mountain"); // a second Mountain stays untapped
    let bolt_id = force_into_hand(&mut state, PlayerId(0), INSTANT);
    let land = force_into_hand(&mut state, PlayerId(0), "Mountain");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // Float one red: taps the first untapped Mountain, leaving the other untapped.
    float_mana(&mut state, PlayerId(0), 1);

    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        legal,
        ..
    })) = state.step()
    else {
        panic!("expected priority");
    };

    assert!(legal.contains(&Action::Pass), "Pass is offered");
    assert!(
        legal.contains(&Action::Concede),
        "Concede is offered [CR#104.3a]"
    );
    assert!(
        legal.contains(&Action::PlayLand { object: land }),
        "the land in hand is playable"
    );
    assert!(
        legal.contains(&Action::CastSpell { object: bolt_id }),
        "the instant is castable once {{R}} is floated"
    );
    // The pinger's non-mana activated ability.
    assert!(
        legal
            .iter()
            .any(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == pinger_id)),
        "the pinger's activated ability is offered"
    );
    // An untapped Mountain's tap-mana ability. `float_mana` taps the first
    // untapped land, so find a Mountain that is still untapped (by content, not
    // by a remembered id) and assert ITS mana ability is offered.
    let untapped_mountain = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(&state, o, "Mountain") && !state.objects.obj(o).tapped)
        .expect("an untapped Mountain remains on the battlefield");
    assert!(
        legal.iter().any(
            |a| matches!(a, Action::ActivateAbility { object, .. } if *object == untapped_mountain)
        ),
        "the remaining untapped Mountain's mana ability is offered"
    );
}

#[test]
fn choose_targets_candidates_resolve_to_names() {
    // Cast Lightning Bolt; the ChooseTargets candidates are object ids a renderer
    // resolves to names via def()/face_name — confirming that decision kind is
    // already renderable with no engine change.
    let mut state = bolt_game(4);
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let target = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");
    let bolt_id = force_into_hand(&mut state, PlayerId(0), INSTANT);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    // Drain to the priority where Bolt is castable, then cast it.
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        ..
    })) = state.step()
    else {
        panic!("expected priority");
    };
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_id }))
        .unwrap();

    // Step until ChooseTargets surfaces; its candidate ids include the bears,
    // and each id resolves to a renderable name.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    let candidates: Vec<ObjectId> = legal.into_iter().flatten().collect();
    assert!(
        candidates.contains(&target),
        "the bears are a legal Bolt target"
    );
    assert_eq!(
        face_name(&state, target),
        "Grizzly Bears",
        "a candidate id resolves to its name for rendering"
    );
}
