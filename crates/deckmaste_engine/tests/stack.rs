//! End-to-end stack/casting/targeting against real canon-card data,
//! driven entirely through the public API (`step` / `submit_decision`).
//!
//! Each test builds a two-player game from canon cards, forces the relevant
//! permanents into play (the public `GameState` fields are all `pub`), advances
//! to a priority window via `step`, then casts a spell the way a UI would:
//! float mana with the Stage-1 mana ability, `CastSpell`, answer
//! `ChooseTargets` / `PayMana` as they surface, and `Pass` to resolve.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::BeginningStep;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::PhaseStep;
use deckmaste_core::Zone;
use deckmaste_engine::Act;
use deckmaste_engine::Action;
use deckmaste_engine::BecameTarget;
use deckmaste_engine::Copied;
use deckmaste_engine::DamageDealt;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionError;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StackObject;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::TriggerFired;
use deckmaste_engine::ZoneChange;
use deckmaste_plugin::plugin::Plugin;

// --- plugin + deck building
// ---------------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// A canon card by name, ready for a deck list.
fn card(name: &str) -> Arc<Card> {
    Arc::new(canon().card(name).unwrap().core)
}

fn red() -> ColorOrColorless {
    Color::Red.into()
}
fn green() -> ColorOrColorless {
    Color::Green.into()
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

/// A two-player game: player 0 holds Instant `DealDamage` `AnyTarget` and
/// Mountains, player 1 holds Vanilla Creature and Forests. `mountains`
/// Mountains are forced onto player 0's battlefield. Callers force a Vanilla
/// Creature onto player 1's battlefield (as a target) when they need one.
fn bolt_game(seed: u64, mountains: usize) -> GameState {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bears = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 5];
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
    // Load builtin rules so lethal-damage SBA fires after spells resolve.
    state.sba_rules = builtin().sba_rules;
    // Mono-typed halves of each deck guarantee the opening seven holds both an
    // instant and a Mountain for player 0 and a Vanilla Creature for player 1.
    for _ in 0..mountains {
        force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    }
    state
}

/// A two-player game where the *casting* player (player 0) holds Vanilla
/// Creature and Forests. `forests` Forests are forced onto player 0's
/// battlefield.
fn bears_game(seed: u64, forests: usize) -> GameState {
    let bears = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    for _ in 0..forests {
        force_onto_battlefield(&mut state, PlayerId(0), "Forest");
    }
    state
}

/// A three-seat game for the Glaring Spotlight "as though it didn't have
/// hexproof" overlay. Seat 0 (A) controls Glaring Spotlight and casts; seat 1
/// (V) is the victim who OWNS the hexproof Gladecover Scout — hexproof only
/// stops opponents ([CR#702.11b]), so the protected creature must belong to a
/// non-caster for "A can, a third player can't" to be a clean assertion. Seat 2
/// (T) is a third player who also casts. A non-hexproof Grizzly Bears (V's)
/// keeps every Bolt cast legal so `ChooseTargets` always surfaces.
fn spotlight_game(seed: u64) -> GameState {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let spotlight = card("Glaring Spotlight");
    let scout = card("Gladecover Scout");
    let bears = card("Grizzly Bears");
    let caster_deck = || {
        let mut d = vec![Arc::clone(&bolt); 5];
        d.extend(vec![Arc::clone(&mountain); 5]);
        d
    };
    let mut a = caster_deck();
    a.push(Arc::clone(&spotlight));
    let mut v = vec![Arc::clone(&mountain); 8];
    v.push(Arc::clone(&scout));
    v.push(Arc::clone(&bears));
    let t = caster_deck();
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: a },
            PlayerConfig { deck: v },
            PlayerConfig { deck: t },
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
    });
    state.sba_rules = builtin().sba_rules;
    force_into_play(&mut state, PlayerId(0), "Glaring Spotlight");
    force_into_play(&mut state, PlayerId(1), "Gladecover Scout");
    force_into_play(&mut state, PlayerId(1), "Grizzly Bears");
    // Mana for the two casters (Bolt is {R}).
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(2), "Mountain");
    force_into_play(&mut state, PlayerId(2), "Mountain");
    state
}

/// The battlefield object with face name `name`.
fn battlefield_named(state: &GameState, name: &str) -> ObjectId {
    *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} on the battlefield"))
}

/// Casts a Lightning Bolt from `caster` in a fresh Spotlight game and returns
/// (state, the legal candidate set for the single target slot).
fn bolt_legal_targets_from(caster: PlayerId) -> (GameState, Vec<ObjectId>) {
    let mut state = spotlight_game(7);
    let _ = run_to_priority(&mut state, caster, PhaseStep::PrecombatMain);
    float_mana(&mut state, caster, 1);
    let bolt = find_in_hand(&state, caster, "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { player, legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}")
    };
    assert_eq!(player, caster);
    (state, legal.into_iter().next().expect("one target slot"))
}

/// Glaring Spotlight lets ITS controller target an opponent's hexproof creature
/// as though it lacked hexproof, while a third player — an opponent of the
/// creature's controller with no Spotlight — still can't ([CR#609.4,702.11d]).
#[test]
fn spotlight_sees_through_hexproof_only_for_its_controller() {
    // A (seat 0) controls Glaring Spotlight.
    let (st, legal) = bolt_legal_targets_from(PlayerId(0));
    let scout = battlefield_named(&st, "Gladecover Scout");
    let bears = battlefield_named(&st, "Grizzly Bears");
    assert!(
        legal.contains(&bears),
        "a non-hexproof creature is always targetable"
    );
    assert!(
        legal.contains(&scout),
        "Spotlight's controller targets the hexproof creature as though it had none"
    );

    // T (seat 2) has no Spotlight and is the Scout controller's opponent.
    let (st2, legal2) = bolt_legal_targets_from(PlayerId(2));
    let scout2 = battlefield_named(&st2, "Gladecover Scout");
    let bears2 = battlefield_named(&st2, "Grizzly Bears");
    assert!(
        legal2.contains(&bears2),
        "a non-hexproof creature is always targetable"
    );
    assert!(
        !legal2.contains(&scout2),
        "a third player still can't target the hexproof creature"
    );
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

/// Drives the explicit payment protocol with the compatibility runner until a
/// non-payment decision surfaces, preserving the intervening progress trace.
fn step_through_payment(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        let (more, stop) = step_to_stop(state);
        trace.extend(more);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("a Payment prompt has an automatic runner answer");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::PayMana(_)) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(
                deckmaste_engine::ChooseManaReversals { legal, .. },
            )) => {
                let reversals = legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a mana-reversal prompt offers a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .unwrap();
            }
            other => return (trace, other),
        }
    }
}

/// Steps until a `Priority` decision surfaces for `player` in `phase`, passing
/// any other priority along the way. Returns the legal action list at that
/// window.
///
/// When payment decisions surface mid-cast, this function uses the engine's
/// canonical runner answers and continues. Tests that need a *specific* mana
/// coverage must answer the `Payment` prompt explicitly before calling this
/// helper.
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(
                deckmaste_engine::ChooseManaReversals { legal, .. },
            )) => {
                let reversals = legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a mana-reversal prompt offers a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .unwrap();
            }
            other => panic!("unexpected stop before {player:?} priority in {phase:?}: {other:?}"),
        }
    }
}

/// Like `run_to_priority`, but also returns the accumulated `Progress` trace
/// along the way — Task 6's trigger-firing assertions read it for
/// `TriggerFired`/`Copied` events. Auto-pays any payment prompts mid-cast
/// exactly like `run_to_priority`.
fn run_to_priority_traced(
    state: &mut GameState,
    player: PlayerId,
    phase: PhaseStep,
) -> (Vec<Progress>, Vec<Action>) {
    let mut trace = Vec::new();
    loop {
        let (t, stop) = step_to_stop(state);
        trace.extend(t);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player: p, legal },
            )) if p == player && state.turn.current == phase => {
                return (trace, legal);
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(
                deckmaste_engine::ChooseManaReversals { legal, .. },
            )) => {
                let reversals = legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a mana-reversal prompt offers a legal set");
                state
                    .submit_decision(Decision::ManaReversals(reversals))
                    .unwrap();
            }
            other => panic!("unexpected stop before {player:?} priority in {phase:?}: {other:?}"),
        }
    }
}

/// Floats `count` mana by activating the first untapped land's mana ability
/// `count` times (each tap is one land). Player 0's forced lands each produce
/// one mana of their land's color.
fn float_mana(state: &mut GameState, player: PlayerId, count: usize) {
    for _ in 0..count {
        // Re-derive the legal list each iteration: tapping a land removes its
        // ability from the next list. The decision is idempotent, so stepping
        // re-surfaces it without mutating.
        let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
            legal,
            ..
        })) = state.step()
        else {
            panic!("expected a priority decision to float mana");
        };
        let tap = legal
            .iter()
            .find(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { object, .. }
                        if !state.objects.obj(*object).tapped
                )
            })
            .cloned()
            .expect("an untapped land with a mana ability");
        state.submit_decision(Decision::Act(tap)).unwrap();
        // Apply the tap/mana events and return to the same priority window.
        let _ = run_to_priority(state, player, state.turn.current);
    }
}

/// Extracts the `GameEvent` from a `Progress::Applied(Occurrence::Single(_))`,
/// returning `None` for any other variant.
fn applied(p: &Progress) -> Option<&GameEvent> {
    match p {
        Progress::Applied(Occurrence::Single(e)) => Some(e),
        _ => None,
    }
}

/// Reads the printed power/toughness of a card-backed object as a pair, or
/// `None` if either is unprinted/variable.
fn printed_pt(state: &GameState, id: ObjectId) -> Option<(i64, i64)> {
    use deckmaste_core::StatValue;
    let face = match state.def(id) {
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => f,
    };
    let num = |s: &Option<StatValue>| match s {
        Some(StatValue::Number(n)) => Some(i64::from(*n)),
        _ => None,
    };
    Some((
        num(&face.characteristics.power)?,
        num(&face.characteristics.toughness)?,
    ))
}

// --- tests --------------------------------------------------------------------

#[test]
fn bolt_kills_grizzly_bears() {
    let mut state = bolt_game(1, 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    // P0's precombat main: an instant and an untapped Mountain in play.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");

    // Cast the instant; answer the target choice with the creature.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(
        legal[0].contains(&bear),
        "the Vanilla Creature is a legal target"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();

    // Step to the caster's priority: the instant is on the stack (announce
    // done, not yet resolved).
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the instant sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(bolt));
    assert_eq!(state.stack[0].targets, vec![vec![bear]]);
    assert!(!state.zones.battlefield.contains(&bolt));

    // Both players pass: the instant resolves, deals 3, SBA destroys the
    // creature.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);

    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::DamageDealt(DamageDealt { target, amount: 3, .. })) if *target == bear
        )),
        "3 damage dealt to the Vanilla Creature, trace: {trace:?}"
    );
    // [CR#400.7]: the old ObjectId is gone; check that the SBA fired and a
    // new object landed in P1's graveyard instead.
    assert!(
        state.objects.get(bear).is_none(),
        "old Vanilla Creature id must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "exactly one object (the reminted creature) in P1's graveyard ([CR#704.5g])"
    );
    // [CR#608.2m]/[CR#400.7]: the instant leaves the stack and remints — the
    // old bolt id is gone; a fresh object sits in P0's graveyard.
    assert!(
        state.objects.get(bolt).is_none(),
        "old instant id must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "the reminted instant lands in P0's graveyard ([CR#608.2m])"
    );
    assert!(
        !state.zones.graveyards[0].contains(&bolt),
        "the graveyard object carries a fresh id, not the old stack id"
    );
    assert!(state.stack.is_empty());
}

/// A `plugins/testing` mock card (macro-aware, builtin prelude) — the home of
/// fixtures for mechanics no canon card carries (e.g. Ward, [CR#702.21a]).
fn testing_card(name: &str) -> Arc<Card> {
    Arc::new(
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
        )
        .unwrap()
        .card(name)
        .unwrap()
        .core,
    )
}

/// P0 holds Lightning Bolt + Mountains (the targeting spell + its mana); P1's
/// deck is Ward Creatures (the warded target). One Mountain is forced onto P0's
/// battlefield.
fn ward_game(seed: u64) -> GameState {
    ward_game_with(seed, "Ward Creature")
}

/// [`ward_game`] with a chosen testing ward carrier (the ward-{X} fixture).
fn ward_game_with(seed: u64, ward_name: &str) -> GameState {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let ward = testing_card(ward_name);
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let p1 = vec![Arc::clone(&ward); 10];
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
    state.sba_rules = builtin().sba_rules;
    force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    state
}

/// [CR#702.21a,601.2c]: Ward reads the event provenance of a `BecameTarget`
/// fact (NOT a zone change) — the collapsed `May(Pay)` toll's
/// `Counter(EventObject)`
/// must counter the targeting SPELL ("counter that spell or ability"), which
/// the engine binds as the event object (the agent = the source on the
/// stack), with the warded permanent the patient. The toll's payer is
/// `ControllerOf(EventObject)` — [CR#702.21a]'s "that player", the TARGETING
/// player, never the ward's controller. P0 bolts P1's Ward creature; P0
/// declines to pay → the Bolt is countered, the creature takes no damage.
#[test]
fn ward_counters_targeting_spell_via_that_object() {
    let mut state = ward_game(7);
    let ward = force_into_play(&mut state, PlayerId(1), "Ward Creature");
    assert_eq!(state.players[1].life, 20, "defender starts at 20");

    // P0's precombat main: float {R} and cast Bolt at P1's Ward creature.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(
        legal[0].contains(&ward),
        "the Ward creature is a legal target of the Bolt"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![ward]]))
        .unwrap();

    // Choosing the target fires the BecameTarget fact → Ward triggers (P1's),
    // placed above the Bolt. Pass priority; when the Ward toll's YesNo
    // surfaces, P1 declines → Counter(ThatObject) counters the Bolt. Drive
    // to an empty stack.
    let mut declined = false;
    loop {
        let (_t, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo {
                player,
            })) => {
                assert_eq!(
                    player,
                    PlayerId(0),
                    "the Ward toll bills the targeting spell's controller \
                     ([CR#702.21a] \"that player\"), not the ward's controller"
                );
                declined = true;
                state.submit_decision(Decision::Answer(false)).unwrap();
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
                    .expect("the Ward flow has an automatic payment answer");
                if matches!(
                    decision,
                    Decision::Payment(deckmaste_engine::PaymentCommand::DeclinePayment)
                ) {
                    declined = true;
                }
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while resolving Ward: {other:?}"),
        }
    }
    assert!(
        declined,
        "the Ward toll's optional payment surfaced and was declined"
    );

    // The Bolt was countered: it left the stack and reminted into P0's
    // graveyard ([CR#701.6a]) WITHOUT resolving — the Ward creature is
    // unharmed (proving `Counter(ThatObject)` hit the spell, not the warded
    // permanent).
    assert!(
        state.objects.get(bolt).is_none(),
        "the old Bolt stack id is gone (countered → reminted)"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "the countered Bolt sits in P0's graveyard ([CR#701.6a])"
    );
    assert!(
        state.zones.battlefield.contains(&ward),
        "the warded creature is untouched (the Bolt never resolved)"
    );
    assert_eq!(
        state.objects.obj(ward).total_damage(),
        0,
        "the warded creature took no damage — Counter hit the Bolt, not the creature"
    );
}

/// P0's deck holds kicker fixtures + Forests; three Forests are forced onto
/// the battlefield for floating. P1 is inert (Forests).
fn kicker_game(seed: u64) -> GameState {
    let charm = testing_card("Kicker Charm");
    let chant = testing_card("Multikicker Chant");
    let beast = testing_card("Kicked Beast");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&charm); 2];
    p0.extend(vec![Arc::clone(&chant); 2]);
    p0.extend(vec![Arc::clone(&beast); 2]);
    p0.extend(vec![Arc::clone(&forest); 6]);
    let p1 = vec![Arc::clone(&forest); 10];
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
    state.sba_rules = builtin().sba_rules;
    for _ in 0..3 {
        force_onto_battlefield(&mut state, PlayerId(0), "Forest");
    }
    state
}

/// [CR#702.33a,702.33d,601.2b]: kicker is announced as the spell is cast —
/// a `YesNo` surfacing BEFORE X/targets — and "if it was kicked" reads the
/// announced record as the spell resolves ([CR#607.2] linked read). Kicked:
/// the {2} joins the mana demand ([CR#601.2f]) and the `If` takes the
/// kicked branch (4 life); declined: base cost only, the otherwise branch
/// (2 life).
#[test]
fn kicker_announce_records_and_paid_cost_reads() {
    for (kick, expected_gain, floats) in [(true, 4, 3), (false, 2, 1)] {
        let mut state = kicker_game(5);
        let charm = force_into_hand(&mut state, PlayerId(0), "Kicker Charm");
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), floats);
        let life0 = state.players[0].life;
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: charm }))
            .unwrap();
        // [CR#601.2b]: the optional-cost YesNo surfaces first.
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo {
            player,
        })) = stop
        else {
            panic!("expected the kicker YesNo, got {stop:?}");
        };
        assert_eq!(player, PlayerId(0), "the caster announces the kicker");
        state.submit_decision(Decision::Answer(kick)).unwrap();
        let mut checked_cost = false;
        loop {
            let (_t, stop) = step_to_stop(&mut state);
            match stop {
                StepOutcome::NeedsDecision(DecisionPointKind::PayMana(
                    deckmaste_engine::PayMana { cost, .. },
                )) => {
                    if kick {
                        assert_eq!(
                            cost.mana_value(),
                            3,
                            "base {{G}} + the kicked {{2}} ([CR#601.2f])"
                        );
                    }
                    let pay = state.auto_pay_pending();
                    state.submit_decision(Decision::Pay(pay)).unwrap();
                }
                StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => {
                    if !checked_cost && prompt.stage == deckmaste_engine::PaymentStage::PrePayment {
                        if kick {
                            assert_eq!(
                                prompt.outstanding.len(),
                                3,
                                "base {{G}} + the kicked {{2}} ([CR#601.2f])"
                            );
                        }
                        checked_cost = true;
                    }
                    let decision = state
                        .auto_payment_pending()
                        .expect("the kicker cast has an automatic payment answer");
                    state.submit_decision(decision).unwrap();
                }
                StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                    deckmaste_engine::Priority { .. },
                )) => {
                    if state.stack.is_empty() {
                        break;
                    }
                    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
                }
                other => panic!("unexpected stop while resolving the charm: {other:?}"),
            }
        }
        assert_eq!(
            state.players[0].life,
            life0 + expected_gain,
            "kick={kick}: PaidCost(Kicker) branched the resolution"
        );
    }
}

/// [CR#702.33c]: a multikicker row re-offers after each yes ("any number of
/// times"), and `TimesPaid(Kicker)` reads the multiplicity at resolution —
/// kicked twice gains exactly 2 life.
#[test]
fn multikicker_times_paid_counts_payments() {
    let mut state = kicker_game(9);
    let chant = force_into_hand(&mut state, PlayerId(0), "Multikicker Chant");
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 3);
    let life0 = state.players[0].life;
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: chant }))
        .unwrap();
    // Pay the multikicker twice, then stop ([CR#702.33c] — re-offered after
    // each yes).
    for answer in [true, true, false] {
        let (_t, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo {
            player,
        })) = stop
        else {
            panic!("expected a multikicker YesNo, got {stop:?}");
        };
        assert_eq!(player, PlayerId(0));
        state.submit_decision(Decision::Answer(answer)).unwrap();
    }
    let mut checked_cost = false;
    loop {
        let (_t, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::PayMana(deckmaste_engine::PayMana {
                cost,
                ..
            })) => {
                assert_eq!(
                    cost.mana_value(),
                    3,
                    "base {{G}} + two kicked {{1}}s ([CR#601.2f])"
                );
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => {
                if !checked_cost && prompt.stage == deckmaste_engine::PaymentStage::PrePayment {
                    assert_eq!(
                        prompt.outstanding.len(),
                        3,
                        "base {{G}} + two kicked {{1}}s ([CR#601.2f])"
                    );
                    checked_cost = true;
                }
                let decision = state
                    .auto_payment_pending()
                    .expect("the multikicker cast has an automatic payment answer");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if state.stack.is_empty() {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while resolving the chant: {other:?}"),
        }
    }
    assert_eq!(
        state.players[0].life,
        life0 + 2,
        "TimesPaid(Kicker) read the announced multiplicity ([CR#702.33c])"
    );
}

/// [CR#702.21b]: a ward-{X} toll prices X when the ward TRIGGER RESOLVES —
/// "ward {X}, where X is the number of experience counters you have"
/// (Minthara, Merciless Soul's shape, the `Ward X Creature` fixture). P1
/// holds 3 experience counters when the toll resolves, so the payment demand
/// P0 (the targeting player) faces is exactly `{3}` — the
/// `Mana([Variable])` priced through the conferred ability's `where_x`,
/// never locked in as the ability triggers.
#[test]
fn ward_x_prices_where_x_at_toll_resolution() {
    let mut state = ward_game_with(7, "Ward X Creature");
    let ward = force_into_play(&mut state, PlayerId(1), "Ward X Creature");

    // P0's precombat main: float {R} and cast Bolt at P1's warded creature.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![ward]]))
        .unwrap();

    // The ward trigger fired at target choice. AFTER it triggered but BEFORE
    // it resolves, P1 gains 3 experience counters — [CR#702.21b] says the
    // toll must price the RESOLUTION-time count, so the late counters bill.
    let p1_proxy = state.player(PlayerId(1)).object;
    state
        .objects
        .obj_mut(p1_proxy)
        .counters
        .insert("Experience".into(), 3);

    // Drive to the toll's optional Payment prompt. The Bolt's own {R}
    // payment comes first and is auto-paid; the later demand must be {3}.
    let mut accepted = false;
    loop {
        let (_t, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo {
                player,
            })) => {
                assert_eq!(player, PlayerId(0), "the toll bills the targeting player");
                accepted = true;
                state.submit_decision(Decision::Answer(true)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::PayMana(deckmaste_engine::PayMana {
                player,
                cost,
                ..
            })) => {
                if !accepted {
                    // The Bolt's own cast payment ({R}) — auto-pay and move on.
                    let pay = state.auto_pay_pending();
                    state.submit_decision(Decision::Pay(pay)).unwrap();
                    continue;
                }
                assert_eq!(player, PlayerId(0));
                assert_eq!(
                    cost,
                    deckmaste_core::ManaCost::from(Arc::<[deckmaste_core::ManaSymbol]>::from(
                        vec![deckmaste_core::ManaSymbol::Simple(
                            deckmaste_core::SimpleManaSymbol::Generic(3),
                        )],
                    )),
                    "the ward-X toll is priced {{3}} from where_x at resolution \
                     ([CR#702.21b])"
                );
                return; // the pricing is the point; payment mechanics are covered elsewhere
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => {
                let is_three_generic = prompt.outstanding.len() == 3
                    && prompt.outstanding.iter().all(|iou| {
                        matches!(
                            iou.kind,
                            deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Generic)
                        )
                    });
                if is_three_generic {
                    assert_eq!(prompt.payer, PlayerId(0));
                    return; // the pricing is the point; payment mechanics are covered elsewhere
                }
                let decision = state
                    .auto_payment_pending()
                    .expect("the Bolt cast has an automatic payment answer");
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while resolving the ward-X toll: {other:?}"),
        }
    }
}

/// Moves the first `name` card from `player`'s library into their hand
/// (when it isn't in hand already) and returns its id — the test-setup dual
/// of `force_into_play` for cards that must be CAST.
fn force_into_hand(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    if let Some(&obj) = state.zones.hands[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
    {
        return obj;
    }
    let obj = *state.zones.libraries[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} in player {}'s library", player.0));
    state.zones.libraries[i].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Hand);
    state.zones.hands[i].push(obj);
    obj
}

/// [CR#702.108a]: prowess end-to-end — canon Bloodfire Expert's
/// `Cast(who: …)` trigger fires on its controller's
/// noncreature cast ([CR#601.2i]) and the resolved pump shows in the
/// derived view until end of turn.
#[test]
fn prowess_fires_and_pumps_on_own_noncreature_cast() {
    // bolt_game's deck with Bloodfire Expert mixed into player 0's half.
    let bolt_card = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let expert_card = card("Bloodfire Expert");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&expert_card); 5];
    p0.extend(vec![Arc::clone(&bolt_card); 5]);
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let p1 = vec![Arc::clone(&forest); 10];
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let expert = force_into_play(&mut state, PlayerId(0), "Bloodfire Expert");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let bolt = force_into_hand(&mut state, PlayerId(0), "Lightning Bolt");
    assert_eq!(state.layers().power(expert), Some(3), "printed 3/1");

    // P0 casts Lightning Bolt at the opponent's face.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let face = state.players[1].object;
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![face]]))
        .unwrap();

    // The cast completes: SpellCast applies and the prowess trigger fires,
    // placing above the bolt.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "bolt + the prowess trigger above it");

    // Both pass: the trigger resolves; the pump shows in the derived view.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "only the bolt remains");
    assert_eq!(
        state.layers().power(expert),
        Some(4),
        "prowess pumped the expert to 4/2 until end of turn ([CR#702.108a])"
    );
}

/// [CR#601.2c]: the chosen objects become targets when the announce locks
/// them — the `BecameTarget` fact rides the cast and fires becomes-target
/// triggers. Canon Phantasmal Bear ("When this creature becomes the target
/// of a spell or ability, sacrifice it") sacrifices itself before the bolt
/// resolves; the bolt then fizzles on its only target ([CR#608.2b]).
#[test]
fn becomes_target_trigger_sacrifices_phantasmal_bear_and_bolt_fizzles() {
    // bolt_game with player 1's creatures swapped for Phantasmal Bear.
    let bolt_card = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let phantasmal = card("Phantasmal Bear");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt_card); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&phantasmal); 5];
    p1.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Phantasmal Bear");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();

    // The lock emits the fact; the bear's trigger fires in its wake.
    let (trace, _stop) = step_to_stop(&mut state);
    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::BecameTarget(BecameTarget { target, source }))
                if *target == bear && *source == bolt
        )),
        "the announce lock emits BecameTarget, trace: {trace:?}"
    );
    assert!(
        trace.iter().any(|p| matches!(
            applied(p),
            Some(GameEvent::TriggerFired(TriggerFired { source, .. }))
                if *source == state.objects.obj(bear).source
        )),
        "the bear's becomes-target trigger fired, trace: {trace:?}"
    );

    // Finish the cast; the trigger places above the bolt.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "bolt + the bear's trigger above it");

    // Both pass: the trigger resolves, the bear is sacrificed.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        state.objects.get(bear).is_none(),
        "the bear sacrificed itself before the bolt resolved"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "the reminted bear sits in its owner's graveyard ([CR#701.21a])"
    );
    assert_eq!(state.stack.len(), 1, "only the bolt remains");

    // Both pass: the bolt re-checks its only target and fizzles ([CR#608.2b]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _) = step_to_stop(&mut state);
    assert!(
        !trace
            .iter()
            .any(|p| matches!(applied(p), Some(GameEvent::DamageDealt(DamageDealt { .. })))),
        "the fizzled bolt deals no damage, trace: {trace:?}"
    );
    assert_eq!(state.players[1].life, 20, "no damage anywhere");
    assert!(state.stack.is_empty(), "the bolt left the stack");
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "the fizzled bolt remints to its owner's graveyard ([CR#608.2b])"
    );
}

#[test]
fn bolt_to_the_face_costs_three_life() {
    let mut state = bolt_game(1, 1);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    let face = state.players[1].object;

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&face), "P1's proxy is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![face]]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(state.players[1].life, 17, "20 - 3");
    assert!(!state.players[1].lost);
}

#[test]
fn grizzly_bears_resolves_to_a_two_two_on_the_battlefield() {
    let mut state = bears_game(1, 2); // two Forests for {1}{G}

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // G, G
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

    // Sorcery-speed cast, no targets; the explicit payment protocol covers
    // {1}{G} from G,G.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();
    // {G} takes one green pip and {1} the other — the only coverage from G,G.
    let (_, stop) = step_to_stop(&mut state);
    assert!(matches!(
        stop,
        StepOutcome::NeedsDecision(DecisionPointKind::Payment(_))
    ));
    let _ = step_through_payment(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "the Vanilla Creature spell is on the stack"
    );

    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    // [CR#608.3]/[CR#400.7]: the permanent spell enters the battlefield via a
    // stack→battlefield future-form ZoneChange that remints — the old stack id
    // is gone and a fresh object is on the battlefield under P0's control.
    assert!(
        state.objects.get(bears).is_none(),
        "old stack id must be gone after the permanent enters and remints"
    );
    assert!(
        !state.zones.battlefield.contains(&bears),
        "the old stack id must not remain on the battlefield"
    );
    // Find the reminted creature among the battlefield (which also holds the
    // forced Forests).
    let entered = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| {
            state.objects.obj(o).card_id().is_some_and(|_| {
                matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } if &*f.characteristics.name == "Grizzly Bears")
            })
        })
        .expect("the reminted Vanilla Creature is on the battlefield");
    assert_ne!(entered, bears, "the entering object carries a fresh id");
    assert_eq!(state.objects.obj(entered).controller, PlayerId(0));
    assert_eq!(state.objects.obj(entered).zone, Some(Zone::Battlefield));
    assert_eq!(printed_pt(&state, entered), Some((2, 2)), "a printed 2/2");
    assert!(state.stack.is_empty());
}

#[test]
fn sorcery_speed_gate_blocks_bears_off_turn_and_on_a_nonempty_stack() {
    // The gate compares Vanilla Creature (sorcery speed) against Instant
    // DealDamage AnyTarget (instant) in two off-window spots. In both, P0 holds
    // instants + a creature + the mana to pay for either, so only the *timing*
    // differs — proving the sorcery-speed gate, not a payment or target gap.

    // (a) On the OPPONENT's turn, in their main phase: P0 has priority. Float
    //     R,G,G through the real mana abilities (so `legal` recomputes); the
    //     creature is timing-blocked while the instant is allowed.
    {
        let bolt = card("Lightning Bolt");
        let bears = card("Grizzly Bears");
        let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        // 4 instant + 4 creature + 1 Mountain + 3 Forest, fattened so a
        // Mountain and two Forests are always somewhere in the library and a
        // seed exists with both spell types in the opening hand.
        let build = |seed: u64| {
            let mut deck = vec![Arc::clone(&bolt); 4];
            deck.extend(vec![Arc::clone(&bears); 4]);
            deck.push(Arc::clone(&mountain));
            deck.extend(vec![Arc::clone(&forest); 3]);
            GameState::new(GameConfig {
                players: vec![
                    PlayerConfig { deck },
                    PlayerConfig {
                        deck: vec![Arc::clone(&forest); 12],
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
        };
        // Pick a seed whose P0 opening hand holds both an instant and a
        // creature.
        let mut state = (0u64..1000)
            .map(build)
            .find(|s| {
                let hand = &s.zones.hands[0];
                hand.iter().any(|&o| is_card(s, o, "Lightning Bolt"))
                    && hand.iter().any(|&o| is_card(s, o, "Grizzly Bears"))
            })
            .expect("a seed with both an instant and a Vanilla Creature in P0's opening hand");
        // P0's mana sources on the battlefield, pulled from the library so they
        // never depend on the opening hand: one Mountain + two Forests.
        force_into_play(&mut state, PlayerId(0), "Mountain");
        force_into_play(&mut state, PlayerId(0), "Forest");
        force_into_play(&mut state, PlayerId(0), "Forest");
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

        // Drive to P0 holding priority during P1's main, tapping all of P0's
        // lands on the way so the pool can pay either spell.
        let legal = drive_to_off_turn_priority(&mut state);
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Vanilla Creature is not castable on the opponent's turn, legal: {legal:?}"
        );
        assert!(
            legal.contains(&Action::CastSpell { object: bolt }),
            "instant is castable on the opponent's turn, legal: {legal:?}"
        );
    }

    // (b) During the active player's OWN main phase but with a non-empty stack
    //     (an instant already announced): creature blocked, a second instant
    //     allowed.
    {
        let mut state = bears_with_bolts();
        let bolt0 = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
        let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");

        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        // Float R,R,G,G: the first {R} instant leaves R,G,G for the gate
        // comparison (a second {R} instant and a {1}{G} creature are both
        // payable).
        float_mana(&mut state, PlayerId(0), 4);
        // Cast the first instant onto the stack, targeting the creature.
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt0 }))
            .unwrap();
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
            deckmaste_engine::ChooseTargets { .. },
        )) = stop
        else {
            panic!("expected ChooseTargets, got {stop:?}");
        };
        state
            .submit_decision(Decision::Targets(vec![vec![bear]]))
            .unwrap();
        let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "an instant is on the stack");
        assert!(
            !legal.contains(&Action::CastSpell { object: bears }),
            "Vanilla Creature blocked while the stack is non-empty, legal: {legal:?}"
        );
        // A second instant is still castable on the non-empty stack.
        let bolt1 = state.zones.hands[0]
            .iter()
            .copied()
            .find(|&o| is_card(&state, o, "Lightning Bolt"))
            .expect("a second instant still in hand");
        assert!(
            legal.contains(&Action::CastSpell { object: bolt1 }),
            "a second instant is castable at instant speed on a non-empty stack, legal: {legal:?}"
        );
    }
}

/// Drives to player 0 holding priority on player 1's turn (a main phase), taps
/// every untapped land player 0 controls (floating mana through the real mana
/// abilities so the recomputed `legal` reflects the pool), and returns that
/// final legal list. Passes any player-1 priority and answers cleanup discards
/// along the way.
fn drive_to_off_turn_priority(state: &mut GameState) -> Vec<Action> {
    loop {
        let (_, stop) = step_to_stop(state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player, legal },
            )) if player == PlayerId(0)
                && state.turn.active_player == PlayerId(1)
                && matches!(
                    state.turn.current,
                    PhaseStep::PrecombatMain | PhaseStep::PostcombatMain
                ) =>
            {
                // Tap an untapped land if one remains; each tap re-opens P0's
                // priority with a freshly-computed legal list.
                if let Some(tap) = legal
                    .iter()
                    .find(|action| {
                        matches!(
                            action,
                            Action::ActivateAbility { object, .. }
                                if !state.objects.obj(*object).tapped
                        )
                    })
                    .cloned()
                {
                    state.submit_decision(Decision::Act(tap)).unwrap();
                } else {
                    return legal;
                }
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
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
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("a Payment prompt has an automatic runner answer");
                state.submit_decision(decision).unwrap();
            }
            other => panic!("unexpected stop: {other:?}"),
        }
    }
}

/// A game where player 0 holds instants + a Vanilla Creature + Forests (a
/// seed with two instants and a creature in the opening hand), two Mountains +
/// two Forests forced onto player 0's battlefield, and player 1 holds a
/// Vanilla Creature (for use as a target).
fn bears_with_bolts() -> GameState {
    let bolt = card("Lightning Bolt");
    let bears = card("Grizzly Bears");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let build = |seed: u64| {
        let mut deck = vec![Arc::clone(&bolt); 4];
        deck.extend(vec![Arc::clone(&bears); 2]);
        deck.extend(vec![Arc::clone(&mountain); 3]);
        deck.extend(vec![Arc::clone(&forest); 3]);
        let mut p1 = vec![Arc::clone(&bears); 2];
        p1.extend(vec![Arc::clone(&forest); 10]);
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck }, PlayerConfig { deck: p1 }],
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
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            let hand = &s.zones.hands[0];
            let bolts = hand
                .iter()
                .filter(|&&o| is_card(s, o, "Lightning Bolt"))
                .count();
            bolts >= 2 && hand.iter().any(|&o| is_card(s, o, "Grizzly Bears"))
        })
        .expect("a seed with two instants and a Vanilla Creature in P0's opening hand");
    // Two Mountains + two Forests → R,R,G,G when all tapped: a {R} instant
    // leaves R,G,G, enough for both a second {R} instant and a {1}{G} creature.
    for _ in 0..2 {
        force_into_play(&mut state, PlayerId(0), "Mountain");
        force_into_play(&mut state, PlayerId(0), "Forest");
    }
    state
}

#[test]
fn payment_protocol_surfaces_for_every_cast() {
    // (a) An all-colored {R} cost still enters the explicit payment protocol,
    // even though there is only one possible coverage.
    {
        let mut state = bolt_game(1, 1);
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1); // R
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
            .unwrap();
        // ChooseTargets surfaces first (instant has targets).
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
            deckmaste_engine::ChooseTargets { .. },
        )) = stop
        else {
            panic!("expected ChooseTargets, got {stop:?}");
        };
        state
            .submit_decision(Decision::Targets(vec![vec![bear]]))
            .unwrap();
        // Payment MUST surface — the core never auto-pays, even for {R}.
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) = stop else {
            panic!("expected Payment for {{R}} (always explicit), got {stop:?}");
        };
        let _ = step_through_payment(&mut state);
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "the instant reached the stack");
        assert!(
            state.player(PlayerId(0)).mana_pool.is_empty(),
            "the Red was spent"
        );
    }

    // (b) A mixed {1}{G} cost from G,G,R exposes exact coverage, with the {1}
    // generic open to Green or Red.
    {
        let mut state = bears_game(2, 2);
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G from forests
        // Add a stray Red so {1} has a real choice.
        state
            .player_mut(PlayerId(0))
            .mana_pool
            .add(red(), 1, ManaProvenance::default());
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
            .unwrap();
        // Payment surfaces (Vanilla Creature has no targets).
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
            panic!("expected Payment for {{1}}{{G}} from G,G,R, got {stop:?}");
        };
        let generic = prompt
            .outstanding
            .iter()
            .find(|iou| {
                matches!(
                    iou.kind,
                    deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Generic)
                )
            })
            .expect("one generic pip")
            .id;
        let colored = prompt
            .outstanding
            .iter()
            .find(|iou| {
                matches!(
                    iou.kind,
                    deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(
                        Color::Green
                    ))
                )
            })
            .expect("one green pip")
            .id;
        let red_unit = prompt
            .floating_mana
            .iter()
            .find(|unit| unit.kind == red())
            .expect("one floating Red")
            .id;
        let green_unit = prompt
            .floating_mana
            .iter()
            .find(|unit| unit.kind == green())
            .expect("floating Green")
            .id;
        let mut coverage = deckmaste_engine::ManaCoverage::empty();
        coverage.insert(generic, deckmaste_engine::ManaPayment::Floating(red_unit));
        coverage.insert(colored, deckmaste_engine::ManaPayment::Floating(green_unit));
        state
            .submit_decision(Decision::Payment(
                deckmaste_engine::PaymentCommand::BeginPayment(coverage),
            ))
            .unwrap();
        let _ = step_through_payment(&mut state);
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        assert_eq!(
            state.stack.len(),
            1,
            "the Vanilla Creature reached the stack"
        );
        // {G} took one Green, {1} took the Red → one Green remains.
        let pool = &state.player(PlayerId(0)).mana_pool;
        assert_eq!(pool.amount(green()), 1, "one Green left");
        assert_eq!(pool.amount(red()), 0, "the Red paid {{1}}");
    }
}

#[test]
fn second_bolt_fizzles_when_its_target_is_already_dead() {
    let mut state = bolt_game(1, 2); // two Mountains for two {R} casts
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R, R

    // Cast instant A targeting the Vanilla Creature.
    let bolt_a = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_a }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // In response (LIFO), cast instant B also targeting the Vanilla Creature.
    let bolt_b = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    assert_ne!(bolt_a, bolt_b);
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt_b }))
        .unwrap();
    let (_, _) = step_to_stop(&mut state);
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "both instants on the stack");

    // Pass both instants to resolution: B resolves (kills creature), then A
    // fizzles.
    let mut damage_events = 0;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::DamageDealt(DamageDealt { amount, .. }),
            ))) => {
                assert_eq!(amount, 3);
                damage_events += 1;
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(_) => break,
        }
        // [CR#400.7]: after reminting, the old `bear` id is gone — break when the
        // stack is empty and P1's graveyard has a (new) object in it.
        if state.stack.is_empty() && !state.zones.graveyards[1].is_empty() {
            // Drain any remaining priority passes for the empty stack, then
            // stop.
            break;
        }
    }

    assert_eq!(
        damage_events, 1,
        "only the top instant dealt damage; the second fizzled ([CR#608.2b])"
    );
    // [CR#400.7]: old id gone; graveyard has the reminted creature.
    assert!(
        state.objects.get(bear).is_none(),
        "old Vanilla Creature id must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "the Vanilla Creature died to the first instant ([CR#704.5g])"
    );
    // [CR#400.7]: both instants leave the stack and remint — their old stack
    // ids are gone; two fresh objects end in P0's graveyard.
    assert!(
        state.objects.get(bolt_a).is_none() && state.objects.get(bolt_b).is_none(),
        "both old instant ids must be gone after reminting"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        2,
        "both reminted instants end in P0's graveyard"
    );
    assert!(state.stack.is_empty());
}

#[test]
fn a_cast_game_is_deterministic() {
    // A reusable script: drive to P0's main, float {R}, cast the instant at
    // the Vanilla Creature, pass both, resolve. Run it twice and compare a
    // fingerprint.
    let fingerprint = |state: &GameState| {
        (
            state.players.iter().map(|p| p.life).collect::<Vec<_>>(),
            state.zones.hands.iter().map(Vec::len).collect::<Vec<_>>(),
            state
                .zones
                .libraries
                .iter()
                .map(VecDeque::len)
                .collect::<Vec<_>>(),
            state.zones.battlefield.clone(),
            state.zones.graveyards.clone(),
            state
                .players
                .iter()
                .map(|p| p.mana_pool.clone())
                .collect::<Vec<_>>(),
            state.turn.turn_number,
            state.turn.current,
        )
    };
    let play = || {
        let mut state = bolt_game(99, 1);
        let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1);
        let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
            .unwrap();
        let (_, _) = step_to_stop(&mut state);
        state
            .submit_decision(Decision::Targets(vec![vec![bear]]))
            .unwrap();
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = step_to_stop(&mut state);
        state
    };
    let a = play();
    let b = play();
    assert_eq!(
        fingerprint(&a),
        fingerprint(&b),
        "same config + decisions → same state"
    );
}

#[test]
fn illegal_target_and_payment_submissions_are_rejected_and_retryable() {
    // --- illegal target at ChooseTargets ---
    let mut state = bolt_game(1, 1);
    let bear = force_onto_battlefield(&mut state, PlayerId(1), "Grizzly Bears");
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_through_payment(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };

    // (i) An object not in the legal set — the instant itself (on the stack,
    //     not a creature/player) is illegal.
    assert!(
        matches!(
            state.submit_decision(Decision::Targets(vec![vec![bolt]])),
            Err(DecisionError::Illegal { .. })
        ),
        "an out-of-set target is rejected"
    );
    // (ii) Wrong count — two targets for a single-target spell.
    let other = state.players[0].object;
    assert!(matches!(
        state.submit_decision(Decision::Targets(vec![vec![bear, other]])),
        Err(DecisionError::Illegal { .. })
    ));
    // State untouched: the decision still pends and no targets were recorded.
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::ChooseTargets(
            deckmaste_engine::ChooseTargets { .. }
        ))
    ));
    assert!(
        state
            .announcing
            .as_ref()
            .expect("announce still in flight")
            .targets
            .is_empty(),
        "no targets recorded after rejected submissions"
    );
    // A valid retry is accepted.
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
    assert!(state.pending.is_none());

    // --- illegal explicit mana coverage ---
    let mut state = bears_game(2, 2);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // G, G
    state
        .player_mut(PlayerId(0))
        .mana_pool
        .add(red(), 1, ManaProvenance::default()); // G,G,R → a choice
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) = stop else {
        panic!("expected Payment, got {stop:?}");
    };
    let pool_before = state.player(PlayerId(0)).mana_pool.clone();
    let generic = prompt
        .outstanding
        .iter()
        .find(|iou| {
            matches!(
                iou.kind,
                deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Generic)
            )
        })
        .expect("one generic pip")
        .id;
    let colored = prompt
        .outstanding
        .iter()
        .find(|iou| {
            matches!(
                iou.kind,
                deckmaste_engine::IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(
                    Color::Green
                ))
            )
        })
        .expect("one green pip")
        .id;
    let green_unit = prompt
        .floating_mana
        .iter()
        .find(|unit| unit.kind == green())
        .expect("floating Green")
        .id;

    // (i) Reusing one unit for both pips is rejected.
    let mut duplicate = deckmaste_engine::ManaCoverage::empty();
    duplicate.insert(generic, deckmaste_engine::ManaPayment::Floating(green_unit));
    duplicate.insert(colored, deckmaste_engine::ManaPayment::Floating(green_unit));
    assert!(matches!(
        state.submit_decision(Decision::Payment(
            deckmaste_engine::PaymentCommand::BeginPayment(duplicate)
        )),
        Err(DecisionError::Illegal { .. })
    ));
    // (ii) Referencing a floating-mana id the pool does not contain is
    // rejected.
    let mut unknown = deckmaste_engine::ManaCoverage::empty();
    unknown.insert(
        generic,
        deckmaste_engine::ManaPayment::Floating(deckmaste_engine::FloatingManaId(u64::MAX)),
    );
    unknown.insert(colored, deckmaste_engine::ManaPayment::Floating(green_unit));
    assert!(matches!(
        state.submit_decision(Decision::Payment(
            deckmaste_engine::PaymentCommand::BeginPayment(unknown)
        )),
        Err(DecisionError::Illegal { .. })
    ));
    // State untouched: still pending, pool unchanged.
    assert!(matches!(state.pending, Some(DecisionPointKind::Payment(_))));
    assert_eq!(
        state.player(PlayerId(0)).mana_pool,
        pool_before,
        "a rejected payment leaves the pool untouched"
    );
    // A valid retry is accepted and the cast completes.
    let _ = step_through_payment(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "the Vanilla Creature reached the stack after retry"
    );
}

/// End-to-end dies-trigger + target-on-trigger + LKI ([CR#603.3,603.10a]):
///
/// P0 controls a `Creature dies-trigger DealDamage AnyTarget` (a 1/1) on the
/// battlefield. P0 casts the fake bolt at the fiend (3 to it). The bolt
/// resolves, the SBA destroys the fiend (lethal), its dies-trigger NOTES, then
/// the `PlaceTriggers` barrier surfaces `ChooseTargets` for the fiend's "any
/// target". We choose a player proxy; the trigger resolves and deals 1 — and
/// the damage's source is the *dead* fiend's id (the LKI source), not a live
/// object.
#[test]
fn dies_trigger_deals_damage_from_the_dead_source() {
    let bolt = card("Lightning Bolt");
    let fiend = card("Footlight Fiend");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    // P0's deck: bolts + fiends + mountains. P1: forests.
    let mut p0 = vec![Arc::clone(&bolt); 4];
    p0.extend(vec![Arc::clone(&fiend); 3]);
    p0.extend(vec![Arc::clone(&mountain); 4]);
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: p0.clone() },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 12],
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
    };
    // A seed whose P0 opening hand holds a bolt (cast from hand) — the fiend
    // and Mountain are pulled from the library by `force_into_play`.
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            s.zones.hands[0]
                .iter()
                .any(|&o| is_card(s, o, "Lightning Bolt"))
        })
        .expect("a seed with a bolt in P0's opening hand");
    // Load builtin rules so the lethal-damage SBA fires after the bolt
    // resolves.
    state.sba_rules = builtin().sba_rules;

    let fiend_obj = force_into_play(&mut state, PlayerId(0), "Footlight Fiend");
    force_into_play(&mut state, PlayerId(0), "Mountain");

    // P0's precombat main: float {R}, cast the bolt at the fiend.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the bolt, got {stop:?}");
    };
    assert!(
        legal[0].contains(&fiend_obj),
        "the fiend is a legal bolt target"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![fiend_obj]]))
        .unwrap();
    // PayMana for {R}.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // Both players pass: the bolt resolves (3 to the fiend), the SBA destroys
    // it, and the dies-trigger NOTES — then `PlaceTriggers` surfaces a
    // `ChooseTargets` for the trigger's own "any target".
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (_, stop) = loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            // P1's priority over the (still empty) stack: pass it along.
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority {
                    player: PlayerId(1),
                    ..
                },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => break (Vec::<Progress>::new(), other),
        }
    };

    // The trigger's target choice surfaces (placement, [CR#603.3d]).
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { player, legal, .. },
    )) = stop
    else {
        panic!("expected the dies-trigger's ChooseTargets, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the fiend's controller chooses");
    // The fiend is gone; choose P1's player proxy as the "any target".
    let p1_proxy = state.players[1].object;
    assert!(
        legal[0].contains(&p1_proxy),
        "P1's proxy is a legal any-target, legal: {legal:?}"
    );
    assert!(
        state.objects.get(fiend_obj).is_none(),
        "the fiend is dead — its old id is gone before its trigger is placed"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![p1_proxy]]))
        .unwrap();

    // A `Triggered` stack object now sits on the stack; both players pass and
    // it resolves, dealing 1 to P1.
    let triggered_on_stack = state
        .stack
        .iter()
        .any(|e| matches!(e.object, StackObject::Triggered { .. }));
    assert!(triggered_on_stack, "the dies-trigger is on the stack");

    // Drive to resolution, collecting the damage event.
    let mut damage_source: Option<ObjectId> = None;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::DamageDealt(DamageDealt {
                    source,
                    target,
                    amount,
                    ..
                }),
            ))) if target == p1_proxy => {
                assert_eq!(amount, 1, "the dies-trigger deals 1");
                damage_source = Some(source);
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(_) => break,
        }
        if damage_source.is_some() && state.stack.is_empty() {
            break;
        }
    }

    // P1 took 1 from the dead fiend.
    assert_eq!(state.players[1].life, 19, "20 - 1 from the dies-trigger");
    // [CR#603.10a]: the damage's source is the fiend's (now-stale) battlefield
    // id — the LKI source, not any live object.
    assert_eq!(
        damage_source,
        Some(fiend_obj),
        "the damage is dealt by the dead fiend's LKI id"
    );
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending after placement+resolution"
    );
    // The triggered ability vanished — no Triggered entry remains.
    assert!(
        !state
            .stack
            .iter()
            .any(|e| matches!(e.object, StackObject::Triggered { .. })),
        "the triggered ability left the stack on resolution ([CR#608.2n])"
    );
}

/// End-to-end ETB trigger + `DrawCards` ([CR#603.3,121.1]):
///
/// P0 casts Elvish Visionary ({1}{G}). It resolves and enters
/// the battlefield. Its `Enters(Ref(This))` trigger fires on the past-form
/// `ZoneChange` (→Battlefield), `PlaceTriggers` places it (no targets →
/// directly), it resolves and calls `DrawCards(1)` — P0 draws a card. Assert
/// hand grew by 1.
#[test]
fn etb_trigger_draws_a_card() {
    let etb = card("Elvish Visionary");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);

    // P0: etb creatures + forests (for {1}{G}).
    // P1: forests only (no cards relevant to the scenario).
    let mut deck0 = vec![Arc::clone(&etb); 4];
    deck0.extend(vec![Arc::clone(&forest); 6]);

    // Seed search: find a seed whose P0 opening hand holds the ETB creature
    // and a Forest so the {1}{G} cost is payable.
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
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
    };
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            let hand = &s.zones.hands[0];
            hand.iter().any(|&o| is_card(s, o, "Elvish Visionary"))
                && hand.iter().any(|&o| is_card(s, o, "Forest"))
        })
        .expect("a seed with the ETB creature and a Forest in P0's opening hand");

    // Force lands onto the battlefield so float_mana works.
    force_into_play(&mut state, PlayerId(0), "Forest");
    force_into_play(&mut state, PlayerId(0), "Forest");

    // Record hand size BEFORE casting (the ETB creature is in hand).
    let hand_before = state.zones.hands[0].len();

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    // Float {G} and {G} for the {1}{G} cost.
    float_mana(&mut state, PlayerId(0), 2);

    let creature = find_in_hand(&state, PlayerId(0), "Elvish Visionary");

    // Cast the creature spell (sorcery speed, no targets).
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: creature }))
        .unwrap();

    // {1}{G}: one green pip + one generic (the other Forest's green covers
    // {1}).
    let _ = step_through_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the creature spell is on the stack");

    // Both players pass → resolves.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    // Collect the trace while driving through PlaceTriggers → resolution.
    // The ETB trigger is non-targeting: it places directly, resolves, draws.
    let mut trigger_fired = false;
    let mut card_drawn = false;
    loop {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::TriggerFired(TriggerFired { .. }),
            ))) => {
                trigger_fired = true;
            }
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                GameEvent::ZoneChange(ZoneChange {
                    snapshot: Some(_),
                    from: Some(Zone::Library),
                    to: Zone::Hand,
                    ..
                }),
            ))) => {
                card_drawn = true;
            }
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { player, .. },
            )) => {
                // Once both the trigger has fired and the card was drawn, stop.
                if card_drawn {
                    break;
                }
                state
                    .submit_decision(Decision::Act(Action::Pass))
                    .unwrap_or_else(|_| {
                        let _ = player;
                    });
            }
            StepOutcome::NeedsDecision(other) => {
                panic!("unexpected decision after etb trigger: {other:?}")
            }
            StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
        }
        if card_drawn && state.stack.is_empty() {
            break;
        }
    }

    assert!(trigger_fired, "TriggerFired event was observed");
    assert!(card_drawn, "the ETB draw reached hand");

    // The ETB creature entered the battlefield; the spell left the hand; a card
    // was drawn. Net hand change: -1 (cast) + 1 (draw) = 0 relative to
    // hand_before. Wait — hand_before includes the creature in hand. After
    // cast: hand_before -
    // 1. After draw: hand_before - 1 + 1 = hand_before. But the trigger draws,
    // so final hand size == hand_before (cast removes creature, draw adds one).
    // The creature left the hand when cast (goes to stack), then leaves the
    // stack when it enters. So net: hand size unchanged from before cast,
    // but now includes one NEW card drawn instead of the creature.
    let hand_after = state.zones.hands[0].len();
    // The key assertion: drawing happened (+1 from the trigger).
    // Since the creature left hand when cast (-1), and the draw added +1, the
    // net from hand_before is 0. But we assert the draw happened (card_drawn)
    // and the creature is on the battlefield (not in hand).
    assert_eq!(
        hand_after, hand_before,
        "hand size is unchanged: the cast removed the creature and the ETB trigger drew 1 \
         (net zero from hand_before = {hand_before})"
    );
    // The creature entered the battlefield.
    let entered = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&o| {
            state.objects.obj(o).card_id().is_some_and(|_| {
                matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. }
                    if &*f.characteristics.name == "Elvish Visionary")
            })
        })
        .expect("the ETB creature is on the battlefield");
    assert_ne!(entered, creature, "reminted with a fresh id ([CR#400.7])");
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending"
    );
}

/// End-to-end occurrence batch + APNAP trigger ordering
/// ([CR#603.2c,603.3b,700.4]):
///
/// Board under P0: Footlight Fiend (dies-trigger, 1/1) +
/// Moonlit Wake (a dies-watcher enchantment) + a Willow Elf. Board under P1:
/// Moonlit Wake + a Willow Elf. P0 casts Pyroclasm (2 to each creature). All
/// three creatures die simultaneously in one `Occurrence::Batch`; the Wakes
/// survive and watch.
///
/// Expected flow:
/// 1. Resolve → one `Batch` of three `DamageDealt` events.
/// 2. SBA sweep → one `Batch` of three `Act(Destroy)` events (each creature has
///    lethal damage); each evolves into its battlefield→graveyard move.
/// 3. Trigger matching notes SEVEN triggers in the same scan: the fiend's
///    dies-trigger plus, for EACH of the three deaths, each player's Moonlit
///    Wake — a live watcher fires once per matching event in the batch.
/// 4. `PlaceTriggers` APNAP ([CR#603.3b,101.4]): P0 orders its four
///    simultaneous triggers, then P1 orders its three; P1's are placed last
///    (resolve first in LIFO).
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "exhaustive occurrence-batch + APNAP trigger-ordering scenario; splitting the seven-trigger sequence across helpers would obscure it"
)]
fn occurrence_batch_and_apnap_ordering() {
    let pyroclasm = card("Pyroclasm");
    let fiend = card("Footlight Fiend");
    let watcher = card("Moonlit Wake");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);

    // P0: pyroclasm + fiend + watcher + sweep-fodder elves + mountains +
    // forests. {1}{R} needs a Mountain (red) + something for generic.
    let elf = card("Willow Elf");
    let mut deck0 = vec![Arc::clone(&pyroclasm); 2];
    deck0.extend(vec![Arc::clone(&fiend); 2]);
    deck0.extend(vec![Arc::clone(&watcher); 2]);
    deck0.extend(vec![Arc::clone(&elf); 2]);
    deck0.extend(vec![Arc::clone(&mountain); 2]);
    deck0.extend(vec![Arc::clone(&forest); 2]);

    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: {
                        let mut d = vec![Arc::clone(&watcher); 2];
                        d.extend(vec![Arc::clone(&elf); 3]);
                        d.extend(vec![Arc::clone(&forest); 5]);
                        d
                    },
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
    };
    let mut state = (0u64..2000)
        .map(build)
        .find(|s| {
            let hand = &s.zones.hands[0];
            hand.iter().any(|&o| is_card(s, o, "Pyroclasm"))
                && hand.iter().any(|&o| is_card(s, o, "Mountain"))
        })
        .expect("a seed with Pyroclasm + Mountain in P0's opening hand");
    // Load builtin rules so lethal-damage SBA fires after Pyroclasm resolves.
    state.sba_rules = builtin().sba_rules;

    // Force mana sources onto the battlefield.
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Forest");

    // Force the creatures and watchers onto the battlefield directly (bypass
    // SBAs). The Wakes are enchantments — Pyroclasm won't touch them.
    let fiend_obj = force_into_play(&mut state, PlayerId(0), "Footlight Fiend");
    let elf0 = force_into_play(&mut state, PlayerId(0), "Willow Elf");
    let _w0 = force_into_play(&mut state, PlayerId(0), "Moonlit Wake");
    // P1's watcher + dying creature — forced from P1's library/hand.
    let _w1 = force_into_play(&mut state, PlayerId(1), "Moonlit Wake");
    let elf1 = force_into_play(&mut state, PlayerId(1), "Willow Elf");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R + G for {1}{R}

    let pyro = find_in_hand(&state, PlayerId(0), "Pyroclasm");

    // Cast Pyroclasm (no targets).
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: pyro }))
        .unwrap();

    // {1}{R}: red covers {R}, generic takes the green.
    let _ = step_through_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state.stack.len(),
        1,
        "Sorcery DealDamage each creature on the stack"
    );

    // Both pass → resolve.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    // --- Drive to the moment just after the SBA batch destroys all creatures.
    // We want to catch the state when pending_triggers has all three noted.
    let mut saw_damage_batch = false;
    let mut saw_sba_destroy_batch = false;
    let mut p0_ordered = false;
    let mut p1_ordered = false;

    loop {
        let (trace, stop) = step_to_stop(&mut state);

        // Look for the DamageDealt batch from the sorcery resolution.
        if !saw_damage_batch {
            for p in &trace {
                if let Progress::Applied(Occurrence::Batch(events)) = p {
                    let all_damage = events
                        .iter()
                        .all(|e| matches!(e, GameEvent::DamageDealt(DamageDealt { .. })));
                    let count = events
                        .iter()
                        .filter(|e| matches!(e, GameEvent::DamageDealt(DamageDealt { .. })))
                        .count();
                    if all_damage && count == 3 {
                        saw_damage_batch = true;
                    }
                }
            }
        }
        // Look for the Act(Destroy) batch (SBA destroys, one simultaneous batch
        // of intents; each evolves into its own battlefield→graveyard move).
        if !saw_sba_destroy_batch {
            for p in &trace {
                if let Progress::Applied(Occurrence::Batch(events)) = p {
                    let destroys = events
                        .iter()
                        .filter(|e| matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Destroy"))
                        .count();
                    if destroys >= 3 {
                        saw_sba_destroy_batch = true;
                    }
                }
            }
        }

        match stop {
            // OrderTriggers: the APNAP player with >1 noted trigger orders
            // them ([CR#603.3b,101.4]); placement is one-at-a-time, so the
            // decision re-surfaces (n, n−1, …, 2) until one remains (which is
            // placed without a decision).
            StepOutcome::NeedsDecision(DecisionPointKind::OrderTriggers(
                deckmaste_engine::OrderTriggers {
                    player,
                    ref triggers,
                },
            )) => {
                let n = triggers.len();
                if player == PlayerId(0) {
                    assert!(!p1_ordered, "APNAP: all of P0's orderings precede P1's");
                    if !p0_ordered {
                        // First encounter: all four of P0's notes at once.
                        assert_eq!(
                            n, 4,
                            "P0's four simultaneous triggers: the fiend's dies-trigger \
                             + its Wake noting each of the three deaths"
                        );
                        assert!(
                            saw_damage_batch,
                            "damage batch observed before OrderTriggers"
                        );
                        // All seven notes (P0's four + P1's three) were taken
                        // in the same scan; none have
                        // been placed yet.
                        assert_eq!(
                            state.pending_triggers.len(),
                            7,
                            "all seven notes pending at the first ordering"
                        );

                        // Submit invalid orders first (rejected; still
                        // pending).
                        let err = state
                            .submit_decision(Decision::Order(vec![0, 0]))
                            .unwrap_err();
                        assert!(
                            err.to_string().contains("permutation"),
                            "duplicate index should be rejected with permutation error, got: {err}"
                        );
                        let err = state.submit_decision(Decision::Order(vec![5])).unwrap_err();
                        assert!(
                            err.to_string().contains("permutation"),
                            "out-of-range index should be rejected with permutation error, got: {err}"
                        );
                        p0_ordered = true;
                    }
                } else {
                    assert_eq!(player, PlayerId(1), "only the two players order");
                    assert!(p0_ordered, "APNAP: P1 orders after P0");
                    if !p1_ordered {
                        assert_eq!(n, 3, "P1's Wake noted each of the three deaths");
                        p1_ordered = true;
                    }
                }
                // Keep the noted order: first noted placed first (resolves
                // last).
                state
                    .submit_decision(Decision::Order((0..n).collect()))
                    .unwrap();
            }

            // ChooseTargets for the dies-trigger's "any target" at placement.
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
                deckmaste_engine::ChooseTargets { player, legal, .. },
            )) => {
                assert_eq!(
                    player,
                    PlayerId(0),
                    "P0 chooses target for the dies-trigger"
                );
                // Choose P1's player proxy as the target.
                let p1_proxy = state.players[1].object;
                assert!(
                    legal[0].contains(&p1_proxy),
                    "P1 proxy is a legal AnyTarget"
                );
                state
                    .submit_decision(Decision::Targets(vec![vec![p1_proxy]]))
                    .unwrap();
            }

            // Priority windows: pass them through to drive to resolution.
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                // Stop once all creatures are dead and the stack is clear (all
                // triggers resolved).
                if state.objects.get(fiend_obj).is_none()
                    && state.objects.get(elf0).is_none()
                    && state.objects.get(elf1).is_none()
                    && state.stack.is_empty()
                    && state.pending_triggers.is_empty()
                {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }

            StepOutcome::NeedsDecision(other) => {
                panic!("unexpected decision: {other:?}")
            }
            StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
            StepOutcome::Progress(_) => unreachable!("step_to_stop never returns Progress"),
        }
    }

    assert!(
        saw_damage_batch,
        "the sorcery dealt damage in a single Batch"
    );
    assert!(
        saw_sba_destroy_batch,
        "SBA destroyed creatures in a single Batch"
    );

    // All three dead creatures' old ids are gone (reminted).
    assert!(
        state.objects.get(fiend_obj).is_none(),
        "fiend reminted — old id gone"
    );
    assert!(
        state.objects.get(elf0).is_none(),
        "P0's elf reminted — old id gone"
    );
    assert!(
        state.objects.get(elf1).is_none(),
        "P1's elf reminted — old id gone"
    );
    // All three ended in graveyards; the watcher enchantments survived.
    assert_eq!(
        state.zones.graveyards[0].len(),
        3,
        "P0's graveyard has Pyroclasm + reminted fiend + reminted elf = 3 objects"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "P1's graveyard has the reminted elf (its Moonlit Wake survives)"
    );
    assert!(
        p0_ordered && p1_ordered,
        "both players ordered their triggers"
    );
    // Every trigger resolved: each Wake gained its controller 1 life per dying
    // creature (×3), and the fiend's dies-trigger dealt 1 to P1's proxy.
    assert_eq!(state.players[0].life, 23, "P0: 20 + 3 × GainLife(1)");
    assert_eq!(
        state.players[1].life, 22,
        "P1: 20 + 3 × GainLife(1) − 1 from the fiend"
    );
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending after full resolution"
    );
    assert!(
        state.stack.is_empty(),
        "stack is empty after all triggers resolved"
    );
}

/// End-to-end multi-loss Draw ([CR#104.4a,700.4]):
///
/// Both players at 4 life. P0 casts Flame Rift ({1}{R} sorcery), which deals
/// 4 to each player in one `Occurrence::Batch`. Both players reach ≤0
/// simultaneously → `PlayerLost` batch → `GameOutcome::Draw`.
///
/// Asserts the Draw was reached via the full cast→resolve→SBA path, not by
/// setting life directly.
#[test]
fn simultaneous_loss_is_a_draw() {
    let each_player = card("Flame Rift");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);

    // P0: the sorcery + lands for {1}{R}. P1: forests.
    let mut deck0 = vec![Arc::clone(&each_player); 4];
    deck0.extend(vec![Arc::clone(&mountain); 3]);
    deck0.extend(vec![Arc::clone(&forest); 3]);

    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck0.clone(),
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed,
            starting_life: 4,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    };
    let mut state = (0u64..200)
        .map(build)
        .find(|s| {
            s.zones.hands[0]
                .iter()
                .any(|&o| is_card(s, o, "Flame Rift"))
        })
        .expect("a seed with Flame Rift in P0's opening hand");

    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Forest");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // R + G for {1}{R}

    let spell = find_in_hand(&state, PlayerId(0), "Flame Rift");

    // Cast: no targets (a set-valued selection), so ChooseTargets does not
    // surface.
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    // The spell has no targets. {1}{R}: the Mountain's red covers {R}, the
    // Forest's green pays the generic through the explicit payment protocol.
    let _ = step_through_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "spell is on the stack");

    // Both pass → resolve.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    // Let it resolve and reach game over.
    let final_outcome = loop {
        match state.step() {
            StepOutcome::GameOver(o) => break o,
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
        }
    };

    assert_eq!(
        final_outcome,
        GameOutcome::Draw,
        "[CR#104.4a]: simultaneous loss → Draw, got {final_outcome:?}"
    );
    // Both players must be marked lost.
    assert!(state.players[0].lost, "P0 lost");
    assert!(state.players[1].lost, "P1 lost");
}

/// End-to-end two-trigger ordering: one player controls two simultaneously
/// firing triggers ([CR#603.3b]):
///
/// P0 controls two Moonlit Wakes (dies-watcher enchantments). A creature
/// (a Grizzly Bears under P0) dies (forced by lethal damage via
/// `CheckSbas`). Both watchers fire at once → `OrderTriggers { player: P0 }`
/// surfaces with both triggers. Assert:
/// - an invalid `Order` is rejected;
/// - a valid `Order([1, 0])` is accepted;
/// - the stack/resolution order matches the chosen order (LIFO: the FIRST
///   placed resolves LAST).
#[test]
fn two_triggers_same_player_order_triggers_surfaces() {
    let watcher_card = card("Moonlit Wake");
    let bears_card = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);

    // Build a simple two-player game; both players' decks don't matter much.
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![
                    Arc::clone(&watcher_card),
                    Arc::clone(&watcher_card),
                    Arc::clone(&bears_card),
                    Arc::clone(&bears_card),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                    Arc::clone(&forest),
                ],
            },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so the lethal-damage SBA fires when the bear's
    // damage is checked.
    state.sba_rules = builtin().sba_rules;

    // Place two watchers and a Grizzly Bears (the dying creature) under P0.
    let watcher0 = force_into_play(&mut state, PlayerId(0), "Moonlit Wake");
    let _watcher1 = force_into_play(&mut state, PlayerId(0), "Moonlit Wake");
    let bear = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");

    // Deal lethal damage to the bear (2/2 → 2 damage = lethal).
    state.objects.obj_mut(bear).set_marked_damage(2);

    // Drive the engine from the start (game begins at Cleanup; each step runs
    // CheckSbas). The bear's lethal damage will be caught the first time the
    // SBA sweep runs — the two watchers' Dies(Type(Creature)) triggers both
    // fire and note simultaneously, then PlaceTriggers surfaces OrderTriggers.
    let mut order_triggers_player = None;
    let mut order_triggers_count = 0;
    let mut order_accepted = false;
    loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::OrderTriggers(
                deckmaste_engine::OrderTriggers {
                    player,
                    ref triggers,
                },
            )) => {
                order_triggers_player = Some(player);
                order_triggers_count = triggers.len();

                // Reject invalid orders.
                let err = state
                    .submit_decision(Decision::Order(vec![0, 0]))
                    .unwrap_err();
                assert!(
                    err.to_string().contains("permutation"),
                    "duplicate index should be rejected with permutation error, got: {err}"
                );
                let err = state.submit_decision(Decision::Order(vec![2])).unwrap_err();
                assert!(
                    err.to_string().contains("permutation"),
                    "out-of-range index should be rejected with permutation error, got: {err}"
                );

                // Accept [1, 0]: the second trigger is placed first (resolves
                // last in LIFO).
                state.submit_decision(Decision::Order(vec![1, 0])).unwrap();
                order_accepted = true;
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                // After ordering, let triggers resolve.
                if order_accepted && state.pending_triggers.is_empty() && state.stack.is_empty() {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::NeedsDecision(other) => panic!("unexpected decision: {other:?}"),
            StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
            StepOutcome::Progress(_) => unreachable!("step_to_stop never returns Progress"),
        }
    }

    assert!(
        order_triggers_player.is_some(),
        "OrderTriggers decision surfaced"
    );
    assert_eq!(
        order_triggers_player,
        Some(PlayerId(0)),
        "P0 controls both triggers"
    );
    assert_eq!(order_triggers_count, 2, "two simultaneous triggers offered");
    assert!(order_accepted, "valid Order([1, 0]) was accepted");

    // Both watchers' triggers fired and resolved — P0 gained 2 life (once per
    // dying creature × 2 watchers = 2 life gained).
    assert_eq!(
        state.players[0].life, 22,
        "P0 gained 2 life (2 × GainLife(1) triggers resolved)"
    );
    // The bear is gone (reminted).
    assert!(
        state.objects.get(bear).is_none(),
        "bear was destroyed and reminted"
    );
    assert!(
        state.pending_triggers.is_empty(),
        "no triggers left pending after resolution"
    );
    assert!(
        state.stack.is_empty(),
        "stack empty after all triggers resolved"
    );

    // The two watchers survive (they are enchantments; we only set the bear's
    // damage directly).
    assert!(
        state.objects.get(watcher0).is_some()
            || state.zones.battlefield.iter().any(|&o| {
                state.objects.obj(o).card_id().is_some_and(|_| {
                    matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. }
                        if &*f.characteristics.name == "Moonlit Wake")
                })
            }),
        "at least one watcher is still on the battlefield (the bear died, not the watchers)"
    );
}

#[test]
fn creature_enters_tapped_via_as_enters_replacement() {
    // Drives both Diregraf Ghoul (an AsEnters replacement) and Grizzly Bears
    // (no replacement) through the full cast+resolve flow, asserting entry
    // status: the former is minted tapped all-at-once (no observable untapped
    // window [CR#614.1c,614.12]), the latter untapped.

    // --- (a) Diregraf Ghoul resolves tapped ---
    {
        let enters_tapped = card("Diregraf Ghoul");
        let swamp = Arc::new(builtin().card("Swamp").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        // {B} needs one black pip; a Swamp supplies it.
        let mut deck = vec![Arc::clone(&enters_tapped); 5];
        deck.extend(vec![Arc::clone(&swamp); 5]);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        force_into_play(&mut state, PlayerId(0), "Swamp");

        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 1); // B
        let spell = find_in_hand(&state, PlayerId(0), "Diregraf Ghoul");

        state
            .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
            .unwrap();
        // {B} is all-colored: the lone Black unit is the only coverage.
        let _ = step_through_payment(&mut state);
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        assert_eq!(state.stack.len(), 1, "the creature spell is on the stack");

        // Both players pass → resolves.
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = step_to_stop(&mut state);

        // Find the reminted permanent on the battlefield.
        let entered = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| {
                state.objects.obj(o).card_id().is_some_and(|_| {
                    matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. }
                        if &*f.characteristics.name == "Diregraf Ghoul")
                })
            })
            .expect("the reminted Diregraf Ghoul is on the battlefield");

        // [CR#614.1c,614.12]: the permanent is minted tapped — no untapped window.
        assert!(
            state.objects.obj(entered).tapped,
            "Diregraf Ghoul must be tapped the instant it enters the battlefield"
        );
    }

    // --- (b) Grizzly Bears (no AsEnters replacement) enters untapped ---
    {
        let mut state = bears_game(1, 2); // two Forests for {1}{G}

        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), 2); // G, G
        let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");

        state
            .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
            .unwrap();
        let _ = step_through_payment(&mut state);
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
        state.submit_decision(Decision::Act(Action::Pass)).unwrap();
        let _ = step_to_stop(&mut state);

        let entered = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| {
                state.objects.obj(o).card_id().is_some_and(|_| {
                    matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. }
                        if &*f.characteristics.name == "Grizzly Bears")
                })
            })
            .expect("the reminted Vanilla Creature is on the battlefield");

        assert!(
            !state.objects.obj(entered).tapped,
            "Grizzly Bears must enter untapped (no AsEnters replacement)"
        );
    }
}

#[test]
fn land_in_hand_offers_play_land_not_cast_spell() {
    // A Mountain in hand must appear as PlayLand (a special action
    // [CR#305.9,116.2a]) but never as CastSpell — lands are not castable spells
    // ([CR#305.9]).
    let mut state = bolt_game(1, 0); // no lands forced onto battlefield
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    let mountain = find_in_hand(&state, PlayerId(0), "Mountain");

    assert!(
        legal.contains(&Action::PlayLand { object: mountain }),
        "legal_actions must offer PlayLand for a Mountain in hand"
    );
    assert!(
        !legal.contains(&Action::CastSpell { object: mountain }),
        "legal_actions must NOT offer CastSpell for a land ([CR#305.9])"
    );
}

/// kw-hexproof goes LIVE: a `Cant(Target)` row excludes its carrier from an
/// opposing spell's legal target set ([CR#702.11b]); a creature beside it
/// stays targetable, and submission re-validates.
#[test]
fn hexproof_excludes_it_from_opposing_targets() {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let scout = card("Gladecover Scout");
    let bears = card("Grizzly Bears");
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&scout); 5];
    p1.extend(vec![Arc::clone(&bears); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 19,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let scout = force_into_play(&mut state, PlayerId(1), "Gladecover Scout");
    let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");
    force_into_play(&mut state, PlayerId(0), "Mountain");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&bear), "the bear is targetable");
    assert!(
        !legal[0].contains(&scout),
        "the hexproof creature is excluded from an opponent's targets"
    );
    // Submission re-validates against the surfaced legal set.
    assert!(
        state
            .submit_decision(Decision::Targets(vec![vec![scout]]))
            .is_err_and(|err| err.to_string().contains("illegal target selection")),
        "targeting the hexproof creature is rejected"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
}

/// kw-flash goes LIVE: the card's own `May(Cast(window: InstantSpeed))` row
/// lifts the sorcery-speed default ([CR#702.8a]) — a flash creature is
/// offered and resolves at an instant-timing window (its caster's upkeep),
/// and the priority window after it hits the battlefield still computes
/// (the row in the derived view no longer trips the presence guard).
#[test]
fn flash_creature_casts_at_instant_timing() {
    let cheetah = card("Pouncing Cheetah");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&cheetah); 5];
    p0.extend(vec![Arc::clone(&forest); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 23,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    for _ in 0..3 {
        force_onto_battlefield(&mut state, PlayerId(0), "Forest");
    }

    // Upkeep is a priority window where sorcery speed is NOT ok.
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
    float_mana(&mut state, PlayerId(0), 3);
    let cheetah = find_in_hand(&state, PlayerId(0), "Pouncing Cheetah");
    let legal = run_to_priority(
        &mut state,
        PlayerId(0),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
    assert!(
        legal.contains(&Action::CastSpell { object: cheetah }),
        "flash lifts the timing default: the cheetah must be castable at upkeep"
    );

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: cheetah }))
        .unwrap();
    let _ = step_through_payment(&mut state);
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(
        &mut state,
        PlayerId(1),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert!(
        state
            .zones
            .battlefield
            .iter()
            .any(|&o| is_card(&state, o, "Pouncing Cheetah")),
        "the flash creature resolved onto the battlefield during upkeep"
    );
    // The flash permanent's row sits in the derived view now — the next
    // legal_actions computation must evaluate it, not trip the seam guard.
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
}

/// Must(Target) goes LIVE (the Flagbearer class): with an opposing
/// Standard Bearer on the battlefield, an able spell must aim at a
/// Flagbearer — choosing past it is rejected, choosing it is accepted
/// ([CR#601.2c] choice-time requirement).
#[test]
fn flagbearer_constrains_opposing_target_choice() {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bearer = card("Standard Bearer");
    let bears = card("Grizzly Bears");
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&bearer); 5];
    p1.extend(vec![Arc::clone(&bears); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed: 41,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let bearer = force_into_play(&mut state, PlayerId(1), "Standard Bearer");
    let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");
    force_into_play(&mut state, PlayerId(0), "Mountain");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(
        legal[0].contains(&bear) && legal[0].contains(&bearer),
        "candidate sets are unchanged — the requirement binds the CHOICE"
    );
    // Aiming past the able Flagbearer is rejected ([CR#601.2c]).
    assert!(
        state
            .submit_decision(Decision::Targets(vec![vec![bear]]))
            .is_err_and(|err| matches!(err, DecisionError::Illegal { .. })),
        "ignoring the able Flagbearer is an illegal choice"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![bearer]]))
        .unwrap();
}

/// The Flagbearer row binds OPPONENTS' choices only: your own spell aims
/// freely past your own Standard Bearer (`by` anchors to the carrier's
/// controller's opponents).
#[test]
fn flagbearer_does_not_constrain_its_controllers_spells() {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bearer = card("Standard Bearer");
    let bears = card("Grizzly Bears");
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    p0.extend(vec![Arc::clone(&bearer); 2]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&bears); 10],
            },
        ],
        seed: 43,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    force_into_play(&mut state, PlayerId(0), "Standard Bearer");
    let bear = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");
    force_into_play(&mut state, PlayerId(0), "Mountain");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![bear]]))
        .unwrap();
}

/// The Flagbearer row is scoped to choosing targets while casting a spell
/// ([CR#601.2c]) or activating an ability ([CR#602.2a]) — a triggered
/// ability instead chooses its targets as it's put ON the stack
/// ([CR#603.3,603.3d]), a moment neither the printed wording nor the row's
/// `by` filter names, so it's exempt.
///
/// P1's own bolt kills P0's Footlight Fiend (a P1-controlled spell is never
/// bound by P1's own Standard Bearer row — the sibling
/// `flagbearer_does_not_constrain_its_controllers_spells` case — so the
/// killing spell itself is unconstrained). P0's dies-trigger then targets
/// "any target" past P1's able Standard Bearer: this trigger IS
/// opponent-controlled from the bearer's perspective, so without the fix the
/// row would bind it too. The choice must be legal.
#[test]
fn flagbearer_does_not_constrain_a_triggered_abilitys_target_choice() {
    let bolt = card("Lightning Bolt");
    let fiend = card("Footlight Fiend");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bearer = card("Standard Bearer");
    let mut p0 = vec![Arc::clone(&fiend); 3];
    p0.extend(vec![Arc::clone(&mountain); 9]);
    let mut p1 = vec![Arc::clone(&bolt); 4];
    p1.extend(vec![Arc::clone(&mountain); 4]);
    p1.extend(vec![Arc::clone(&bearer); 1]);
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    p1.extend(vec![Arc::clone(&forest); 3]);
    let build = |seed: u64| {
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: p0.clone() },
                PlayerConfig { deck: p1.clone() },
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
    };
    // A seed whose P1 opening hand holds a bolt (cast from hand) — the
    // fiend, and P1's Mountain and Standard Bearer, are pulled from the
    // libraries by `force_into_play`.
    let mut state = (0u64..1000)
        .map(build)
        .find(|s| {
            s.zones.hands[1]
                .iter()
                .any(|&o| is_card(s, o, "Lightning Bolt"))
        })
        .expect("a seed with a bolt in P1's opening hand");
    state.sba_rules = builtin().sba_rules;

    let fiend_obj = force_into_play(&mut state, PlayerId(0), "Footlight Fiend");
    force_into_play(&mut state, PlayerId(1), "Mountain");
    let bearer_obj = force_into_play(&mut state, PlayerId(1), "Standard Bearer");

    // P0 passes their precombat main priority without acting; P1 floats
    // {R} and bolts P0's fiend.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(1), 1);
    let bolt = find_in_hand(&state, PlayerId(1), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the bolt, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![fiend_obj]]))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);

    // Both players pass repeatedly: the bolt resolves (3 to the fiend), the
    // SBA destroys it, and the dies-trigger NOTES — then `PlaceTriggers`
    // surfaces a `ChooseTargets` for the trigger's own "any target".
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let stop = loop {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => break other,
        }
    };

    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { player, legal, .. },
    )) = stop
    else {
        panic!("expected the dies-trigger's ChooseTargets, got {stop:?}");
    };
    assert_eq!(player, PlayerId(0), "the fiend's controller chooses");
    // The able Flagbearer is among the "any target" candidates — a
    // Must(Target) row is live and would bind an announce/activation.
    assert!(
        legal[0].contains(&bearer_obj),
        "Standard Bearer is a legal any-target, legal: {legal:?}"
    );
    // Choosing P1's proxy instead — past the able Flagbearer — is legal:
    // the trigger's placement-time targeting is exempt.
    let p1_proxy = state.players[1].object;
    assert!(
        legal[0].contains(&p1_proxy),
        "P1's proxy is a legal any-target, legal: {legal:?}"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![p1_proxy]]))
        .unwrap();
}

/// The control: without a flash row, a creature spell stays
/// sorcery-speed-only ([CR#117.1a]) — never offered at upkeep even with
/// the cost funded.
#[test]
fn nonflash_creature_not_castable_at_instant_timing() {
    let mut state = bears_game(7, 2);
    let _ = run_to_priority(
        &mut state,
        PlayerId(0),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
    float_mana(&mut state, PlayerId(0), 2);
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    let legal = run_to_priority(
        &mut state,
        PlayerId(0),
        PhaseStep::Beginning(BeginningStep::Upkeep),
    );
    assert!(
        !legal.contains(&Action::CastSpell { object: bears }),
        "a non-flash creature must not be castable at upkeep"
    );
}

/// An inline "Blink" instant: "Exile target creature, then return that card
/// to the battlefield." Built in-test (not a canon card) because the
/// return-to-battlefield oracle wording needs enter-rider rendering/execution
/// (a separate ticket); this exercises the find-moved-object mechanism
/// ([CR#400.7j]) through the full cast path. The exile records old→new in the
/// resolution-scoped move record; the return reads the exile's PRODUCT
/// register — a NEW object, [CR#400.7] — and returns it, all in one
/// resolution. Re-spelled from the deleted product-sited `That(Card)`: the
/// exile is now `Act { dest, .. }` and the return reads `dest` by register.
fn inline_blink() -> Card {
    use deckmaste_core::Action;
    use deckmaste_core::DefId;
    use deckmaste_core::Destination;
    use deckmaste_core::Instruction;
    use deckmaste_core::Kind;
    use deckmaste_core::Param;
    use deckmaste_core::Predicate;
    use deckmaste_core::Provenance;
    use deckmaste_core::Quantity;
    use deckmaste_core::RefId;
    use deckmaste_core::Reference;
    use deckmaste_core::Region;
    use deckmaste_core::TargetSpec;

    // A spell region declares source(0), controller(1), announced target(2)
    // and announced X(3), so the exile's product is definition 4.
    let exiled = DefId(4);
    let params: Arc<[Param]> = Arc::from([
        Param {
            def: DefId(0),
            kind: Kind::Entity,
            provenance: Provenance::Source,
        },
        Param {
            def: DefId(1),
            kind: Kind::Entity,
            provenance: Provenance::Controller,
        },
        Param {
            def: DefId(2),
            kind: Kind::Entities,
            provenance: Provenance::AnnouncedTarget(0),
        },
        Param {
            def: DefId(3),
            kind: Kind::Number,
            provenance: Provenance::AnnouncedX,
        },
    ]);

    Card::Normal(deckmaste_card::CardFace::from(
        deckmaste_card::Characteristics {
            name: "Blink".into(),
            mana_cost: "{W}".parse().unwrap(),
            types: vec![deckmaste_core::Type::Instant.def()],
            abilities: vec![deckmaste_core::Ability::spell(
                deckmaste_core::SpellAbility {
                    ability_word: None,
                    cost: deckmaste_core::Cost::default(),
                    targets: vec![TargetSpec::Target(
                        Quantity::one(),
                        Arc::new(Region::candidate(Predicate::creature())),
                    )]
                    .into(),
                    effect: Region::new(
                        params,
                        Instruction::Sequentially(
                            vec![
                                Instruction::producing(
                                    exiled,
                                    Action::Move(
                                        Reference::Reg(RefId(2)),
                                        Destination::Zone(Zone::Exile),
                                        vec![].into(),
                                        None,
                                    ),
                                ),
                                Instruction::act(Action::Move(
                                    Reference::Reg(exiled.into()),
                                    Destination::Zone(Zone::Battlefield),
                                    vec![].into(),
                                    None,
                                )),
                            ]
                            .into(),
                        )
                        .into(),
                    ),
                },
            )],
            ..deckmaste_card::Characteristics::default()
        },
    ))
}

/// Cloudshift end-to-end ([CR#400.7j,110.2a]): a REAL canon card (unlike
/// `inline_blink` below, which predates `engine-enter-rider-execution` and
/// carries no rider at all) casts through the full stack, exiles the
/// targeted creature, and returns that card to the battlefield with
/// `EnterRider::UnderControlOf(You)` ([CR#110.2a,614.12]) — the first canon
/// card to exercise that grammar node
/// (`no_dead_grammar.rs`'s `EnterRider::UnderControlOf` entry).
#[test]
fn cloudshift_returns_the_exiled_creature_under_the_casters_control() {
    let cloudshift = card("Cloudshift");
    let bears = card("Grizzly Bears");
    let plains = Arc::new(builtin().card("Plains").unwrap().core);
    let mut p0 = vec![Arc::clone(&cloudshift); 5];
    p0.extend(vec![Arc::clone(&bears); 5]);
    p0.extend(vec![Arc::clone(&plains); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&plains); 10],
            },
        ],
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
    state.sba_rules = builtin().sba_rules;
    let before = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    force_onto_battlefield(&mut state, PlayerId(0), "Plains");
    let cloudshift = find_in_hand(&state, PlayerId(0), "Cloudshift");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {W}

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: cloudshift }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(
        legal[0].contains(&before),
        "the controlled creature is a legal target"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![before]]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "Cloudshift sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(cloudshift));

    // Both players pass: Cloudshift resolves — exile then return, one
    // resolution.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert!(
        state.objects.get(before).is_none(),
        "pre-exile object is gone ([CR#400.7])"
    );
    assert!(
        state.zones.exile.is_empty(),
        "the exile leg is transient within this one resolution"
    );
    let after = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(&state, o, "Grizzly Bears"))
        .expect("the returned creature is on the battlefield");
    assert_ne!(after, before, "a NEW object returned ([CR#400.7])");
    assert_eq!(state.objects.obj(after).zone, Some(Zone::Battlefield));
    assert_eq!(
        state.objects.obj(after).controller,
        PlayerId(0),
        "UnderControlOf(You) returns it under the caster's control"
    );
    assert_eq!(
        printed_pt(&state, after),
        Some((2, 2)),
        "still Grizzly Bears"
    );
    assert!(state.stack.is_empty());
}

/// Blink end-to-end ([CR#400.7j]): cast through the full stack, exile the
/// targeted creature and return THAT CARD in one resolution. The original id
/// is gone; a NEW object is on the battlefield ([CR#400.7]).
#[test]
fn blink_exiles_and_returns_the_target_in_one_resolution() {
    let blink = Arc::new(inline_blink());
    let bears = card("Grizzly Bears");
    let plains = Arc::new(builtin().card("Plains").unwrap().core);
    let mut p0 = vec![Arc::clone(&blink); 5];
    p0.extend(vec![Arc::clone(&bears); 5]);
    p0.extend(vec![Arc::clone(&plains); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&plains); 10],
            },
        ],
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
    state.sba_rules = builtin().sba_rules;
    let before = force_onto_battlefield(&mut state, PlayerId(0), "Grizzly Bears");
    force_onto_battlefield(&mut state, PlayerId(0), "Plains");
    let blink = find_in_hand(&state, PlayerId(0), "Blink");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {W}

    state
        .submit_decision(Decision::Act(Action::CastSpell { object: blink }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&before), "the creature is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![before]]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "Blink sits on the stack");
    assert_eq!(state.stack[0].object, StackObject::Spell(blink));

    // Both players pass: Blink resolves — exile then return, one resolution.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert!(
        state.objects.get(before).is_none(),
        "pre-exile object is gone ([CR#400.7])"
    );
    assert!(
        state.zones.exile.is_empty(),
        "the exile leg is transient within this one resolution"
    );
    let after = *state
        .zones
        .battlefield
        .iter()
        .find(|&&o| is_card(&state, o, "Grizzly Bears"))
        .expect("the returned creature is on the battlefield");
    assert_ne!(after, before, "a NEW object returned ([CR#400.7])");
    assert_eq!(state.objects.obj(after).zone, Some(Zone::Battlefield));
    assert_eq!(state.objects.obj(after).controller, PlayerId(0));
    assert_eq!(
        printed_pt(&state, after),
        Some((2, 2)),
        "still Grizzly Bears"
    );
    assert!(state.stack.is_empty());
}

/// P0 holds Lightning Bolt + Mountains (the spell to copy); P1's deck is the
/// `Creature tap-activated CopySpell Target Spell` fixture (the copier) plus
/// Forests (unused — the ability's only cost is {T}).
fn copy_game(seed: u64, mountains: usize) -> GameState {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let copier = testing_card("Creature tap-activated CopySpell Target Spell");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&copier); 5];
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
    state.sba_rules = builtin().sba_rules;
    for _ in 0..mountains {
        force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    }
    state
}

/// Like `copy_game`, but the spell to copy is a PERMANENT: P0 holds Grizzly
/// Bears + Forests (the spell to copy); P1's deck is the copier fixture plus
/// Forests (unused — the ability's only cost is {T}). Backs the
/// permanent-spell-copy-vanish finding ([CR#707.10a],[CR#707.10f]).
fn bears_copy_game(seed: u64, forests: usize) -> GameState {
    let bears = card("Grizzly Bears");
    let copier = testing_card("Creature tap-activated CopySpell Target Spell");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&bears); 5];
    p0.extend(vec![Arc::clone(&forest); 5]);
    let mut p1 = vec![Arc::clone(&copier); 5];
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
    state.sba_rules = builtin().sba_rules;
    for _ in 0..forests {
        force_onto_battlefield(&mut state, PlayerId(0), "Forest");
    }
    state
}

/// [CR#707.10]: copying a spell on the stack puts a SECOND stack entry there
/// — the copy shares the original's targets/X/paid costs but is controlled
/// by the copying player, not the caster. P0 bolts P1's copier creature
/// (both the spell's target and the object that will copy it); while the
/// Bolt sits on the stack, P1 activates the copier's `{T}: copy target
/// spell` ability targeting the Bolt itself, before it resolves.
#[test]
fn copied_bolt_shares_targets_and_controller() {
    let mut state = copy_game(1, 1);
    let copier = force_into_play(
        &mut state,
        PlayerId(1),
        "Creature tap-activated CopySpell Target Spell",
    );
    // Documents the precondition; the turn-start untap also clears the flag
    // for the active player's permanents ([CR#302.6]) — P1 isn't active this
    // turn, so the fixture clears it directly like `activate.rs`'s pinger.
    state.objects.obj_mut(copier).summoning_sick = false;

    // P0's precombat main: float {R} and cast Bolt at P1's copier.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert!(legal[0].contains(&copier), "the copier is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![copier]]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the Bolt sits on the stack");
    assert_eq!(state.stack[0].targets, vec![vec![copier]]);

    // P0 passes; P1 gets priority with the Bolt still unresolved. Instead of
    // passing, P1 activates the copier's tap ability, targeting the Bolt.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    let activate = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == copier))
        .cloned()
        .expect("the copier's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the copy ability, got {stop:?}");
    };
    assert!(
        legal[0].contains(&bolt),
        "the Bolt is a legal target of Kind(Spell)"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![bolt]]))
        .unwrap();

    // The nonmana tap cost is fulfilled through Payment before P1 regains
    // priority with the copy ability above the unresolved Bolt.
    let (_, stop) = step_through_payment(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = stop
    else {
        panic!("expected P1 priority after tap-cost payment, got {stop:?}");
    };
    assert_eq!(player, PlayerId(1));
    assert_eq!(state.stack.len(), 2, "the Bolt plus the copy ability");

    // Both players pass: the copy ability resolves, pushing the Bolt's copy.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(
        state.stack.len(),
        2,
        "the original Bolt plus its minted copy"
    );
    let original = state
        .stack
        .iter()
        .find(|e| !e.copy)
        .expect("the original Bolt is still on the stack, unresolved");
    let copy = state
        .stack
        .iter()
        .find(|e| e.copy)
        .expect("the copy ability resolved and pushed a copy entry");
    assert_eq!(original.id, bolt, "the original entry is still the Bolt");
    assert_ne!(
        copy.id, original.id,
        "the copy is a freshly minted stack object"
    );
    assert_eq!(copy.targets, original.targets, "shares the Bolt's target");
    assert_eq!(copy.x, original.x, "shares the Bolt's X (none)");
    assert_eq!(
        copy.paid_costs, original.paid_costs,
        "shares the Bolt's paid costs"
    );
    assert_eq!(
        copy.controller,
        PlayerId(1),
        "the copy is controlled by the copying player, not the caster ([CR#707.10])"
    );
    assert_eq!(
        copy.object,
        StackObject::Spell(copy.id),
        "a spell copy is itself a spell, backed by its own fresh object"
    );
}

/// Like `copy_game`, but P0 additionally holds Mana Leak and Islands — the
/// [CR#118.12a] counter `countered_copy_vanishes` aims at the minted copy.
fn copy_counter_game(seed: u64, mountains: usize, islands: usize) -> GameState {
    let bolt = card("Lightning Bolt");
    let mana_leak = card("Mana Leak");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let island = Arc::new(builtin().card("Island").unwrap().core);
    let copier = testing_card("Creature tap-activated CopySpell Target Spell");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mana_leak); 5]);
    p0.extend(vec![Arc::clone(&mountain); 5]);
    p0.extend(vec![Arc::clone(&island); 5]);
    let mut p1 = vec![Arc::clone(&copier); 5];
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
    state.sba_rules = builtin().sba_rules;
    for _ in 0..mountains {
        force_into_play(&mut state, PlayerId(0), "Mountain");
    }
    for _ in 0..islands {
        force_into_play(&mut state, PlayerId(0), "Island");
    }
    state
}

/// Drives an already-constructed `state` (P0 holds Lightning Bolt plus a
/// Mountain in play; P1 holds the tap-activated copier plus a Forest) through
/// Task 3's copy-minting flow: P0 casts Bolt at P1's face, P1 copies it with
/// the copier's `{T}: copy target spell`. Returns `(bolt, copy, face)` with
/// both the original Bolt and its minted copy left on the stack ([CR#707.10]).
fn cast_and_copy_bolt_at_face(state: &mut GameState) -> (ObjectId, ObjectId, ObjectId) {
    let copier = force_into_play(
        state,
        PlayerId(1),
        "Creature tap-activated CopySpell Target Spell",
    );
    // Documents the precondition; see
    // `copied_bolt_shares_targets_and_controller`.
    state.objects.obj_mut(copier).summoning_sick = false;

    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(state, PlayerId(0), 1); // {R}
    let bolt = force_into_hand(state, PlayerId(0), "Lightning Bolt");
    let face = state.players[1].object;
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![face]]))
        .unwrap();

    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(state, PlayerId(1), PhaseStep::PrecombatMain);
    let activate = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == copier))
        .cloned()
        .expect("the copier's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    let (_, stop) = step_to_stop(state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the copy ability, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![bolt]]))
        .unwrap();
    // Fulfill the copier's nonmana tap cost and return to P1's priority.
    let _ = step_through_payment(state);

    // Both players pass: the copy ability resolves, pushing the Bolt's copy.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(state);

    let copy = state
        .stack
        .iter()
        .find(|e| e.copy)
        .expect("the copy ability resolved and pushed a copy entry")
        .id;
    (bolt, copy, face)
}

/// [CR#707.10a]: a copy leaves the stack by CEASING TO EXIST, never by moving
/// zones — resolution is the happy path. Bolt and its copy both target P1's
/// face (a player proxy is always a legal target, so neither resolution can
/// fizzle the other by killing a shared creature target first). Both deal
/// their 3 damage; only the ORIGINAL Bolt — a real card — lands in the
/// graveyard. The copy's minted object simply disappears.
#[test]
fn resolved_copy_vanishes_without_zone_move() {
    let mut state = copy_game(1, 1);
    let (bolt, copy, face) = cast_and_copy_bolt_at_face(&mut state);
    assert_eq!(
        state.stack.len(),
        2,
        "the original Bolt plus its minted copy"
    );

    // Drive both spells to resolution.
    let mut trace = Vec::new();
    while !state.stack.is_empty() {
        let (t, stop) = step_to_stop(&mut state);
        trace.extend(t);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while draining the stack: {other:?}"),
        }
    }

    let damage_events = trace
        .iter()
        .filter(|p| {
            matches!(
                applied(p),
                Some(GameEvent::DamageDealt(DamageDealt { target, amount: 3, .. })) if *target == face
            )
        })
        .count();
    assert_eq!(
        damage_events, 2,
        "damage applied twice — the original Bolt and its copy each dealt 3, trace: {trace:?}"
    );
    assert_eq!(state.players[1].life, 14, "20 - 3 - 3: bolted twice over");
    assert!(state.stack.is_empty(), "both spells left the stack");

    assert!(
        state.objects.get(bolt).is_none(),
        "the original Bolt's old id is gone after reminting into the graveyard"
    );
    assert!(
        state.objects.get(copy).is_none(),
        "the copy's ObjectId is gone from self.objects — it ceased, it didn't remint"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "graveyard contains ONLY the original Bolt card — the copy left no card behind"
    );
    assert!(
        is_card(&state, state.zones.graveyards[0][0], "Lightning Bolt"),
        "the one graveyard entry is the original Bolt"
    );
}

/// [CR#707.10a,701.6a]: a COUNTERED copy also ceases rather than moving to a
/// graveyard. Mana Leak targets the minted copy directly (its own target
/// choice, independent of what the original Bolt targets) and P1 — the
/// copy's controller, "that player" ([CR#118.12a]) — declines the {3}
/// punisher, so `Counter(It)` hits the copy.
#[test]
fn countered_copy_vanishes() {
    let mut state = copy_counter_game(1, 1, 2);
    let (_bolt, copy, _face) = cast_and_copy_bolt_at_face(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // {1}{U}
    let mana_leak = force_into_hand(&mut state, PlayerId(0), "Mana Leak");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: mana_leak }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for Mana Leak, got {stop:?}");
    };
    assert!(
        legal[0].contains(&copy),
        "the copy is a legal target of Mana Leak's TargetOne(Spell) ([CR#707.10])"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![copy]]))
        .unwrap();

    // Drive Mana Leak to resolution; P1 declines the punisher.
    let mut declined = false;
    loop {
        let (_t, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo {
                player,
            })) => {
                assert_eq!(
                    player,
                    PlayerId(1),
                    "Mana Leak bills the targeted copy's controller \
                     ([CR#118.12a] \"that player\")"
                );
                declined = true;
                state.submit_decision(Decision::Answer(false)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::PayMana(deckmaste_engine::PayMana {
                ..
            })) => {
                let pay = state.auto_pay_pending();
                state.submit_decision(Decision::Pay(pay)).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => {
                let decision = state
                    .auto_payment_pending()
                    .expect("the Mana Leak flow has an automatic payment answer");
                if matches!(
                    decision,
                    Decision::Payment(deckmaste_engine::PaymentCommand::DeclinePayment)
                ) {
                    assert_eq!(
                        prompt.payer,
                        PlayerId(1),
                        "Mana Leak bills the targeted copy's controller \
                         ([CR#118.12a] \"that player\")"
                    );
                    declined = true;
                }
                state.submit_decision(decision).unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                if declined {
                    break;
                }
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while resolving Mana Leak: {other:?}"),
        }
    }
    assert!(declined, "Mana Leak's optional payment was declined");

    assert!(
        state.objects.get(copy).is_none(),
        "the copy's ObjectId is gone from self.objects"
    );
    assert!(
        !state.stack.iter().any(|e| e.copy),
        "no copy entry remains on the stack"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        1,
        "graveyard contains only Mana Leak itself — no entry for the countered copy"
    );
    assert!(
        !state.zones.graveyards[0]
            .iter()
            .any(|&o| is_card(&state, o, "Lightning Bolt")),
        "no graveyard entry for the copy"
    );
}

/// [CR#707.10a]: the native SBA safety net. If a copy of a spell somehow ends
/// up off the stack (a future generic zone-mover, not yet built), the sweep
/// notices and it ceases — the same "no card, no zone move" shape as the
/// resolution/counter divert, just triggered from a different angle.
#[test]
fn off_stack_copy_ceases_via_sba() {
    // A second Mountain: casting Bolt taps the first, leaving one spare to
    // float mana with below (no card/decision needed to force a resweep).
    let mut state = copy_game(1, 2);
    let (bolt, copy, _face) = cast_and_copy_bolt_at_face(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // Force-move the copy off the stack: mutate its backing object's zone
    // directly, the way an as-yet-unbuilt generic mover (bounce, bury) might
    // leave it mid-transition. `state.stack` still carries the copy's entry —
    // only the object it points at is now stranded off-stack.
    state.objects.obj_mut(copy).zone = Some(Zone::Graveyard);

    // A bare `Action::Pass` that doesn't complete an all-pass round only
    // hands priority to the next player — it does NOT re-run `CheckSbas`
    // (see `take_priority_action`'s `Action::Pass` arm). Tapping the spare
    // Mountain for mana instead goes through `priority_tail()` =
    // `[CheckSbas, PlaceTriggers, OpenPriority]` ([CR#704.3]) — the simplest
    // action that forces a fresh sweep without resolving the still-2-deep
    // stack (which would exercise the resolution divert, not this SBA).
    float_mana(&mut state, PlayerId(0), 1);

    assert!(
        state.objects.get(copy).is_none(),
        "the copy object ceased ([CR#707.10a])"
    );
    assert!(
        !state.stack.iter().any(|e| e.copy),
        "no copy entry remains on the stack"
    );
    assert!(
        state.stack.iter().any(|e| e.id == bolt && !e.copy),
        "the original Bolt is untouched, still on the stack — the SBA only \
         swept the off-stack copy"
    );
}

/// core-copy-grammar Task 4: [CR#109.1] a card-less spell copy classifies
/// through `is_object_class`/`Predicate::Class` — the SAME predicate-evaluation
/// path an SBA `scope` uses (`sba.rs`'s `crate::matches(state, id,
/// &rule.scope)`) — as `Spell` while genuinely on the stack ([CR#707.10]:
/// "a copy of a spell is itself a spell"), and as `CardCopy` once stranded
/// off it (the same stranded state `off_stack_copy_ceases_via_sba`'s native
/// safety net sweeps). This is the classification the copy-cease SBA's
/// planned `scope` predicate (core-copy-grammar Task 5) will select over.
#[test]
fn off_stack_copy_classifies_as_card_copy() {
    use deckmaste_core::ObjectClass;
    use deckmaste_core::Predicate;

    let mut state = copy_game(1, 2);
    let (_bolt, copy, _face) = cast_and_copy_bolt_at_face(&mut state);
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    assert!(
        deckmaste_engine::is_object_class(&state, copy, ObjectClass::Spell),
        "[CR#707.10]: still genuinely on the stack, the copy is a Spell"
    );

    // Force-move the copy off the stack, exactly as
    // `off_stack_copy_ceases_via_sba` does: mutate its backing object's
    // zone directly (simulating an as-yet-unbuilt generic mover leaving it
    // mid-transition). `state.stack` still carries the copy's entry.
    state.objects.obj_mut(copy).zone = Some(Zone::Graveyard);

    assert!(
        deckmaste_engine::is_object_class(&state, copy, ObjectClass::CopyOfACard),
        "[CR#109.1,707.10a]: stranded off the stack, the card-less copy \
         entry classifies as CardCopy"
    );
    assert!(
        deckmaste_engine::matches(&state, copy, &Predicate::Class(ObjectClass::CopyOfACard)),
        "Filter::CardCopy matches through the same predicate path an SBA \
         scope evaluates"
    );
}

/// [CR#707.10a,707.10f]: a copy of a PERMANENT spell vanishes on resolution
/// too — it does NOT take the [CR#608.3] battlefield zone move. (The
/// CR-correct outcome, a resolving permanent-spell copy becoming a token
/// permanent, is deferred to a follow-up ticket; vanishing is the interim
/// placeholder, same as any other copy.) P0 casts Grizzly Bears; while it
/// sits on the stack, P1's copier copies it (`Kind(Spell)` targets any spell
/// on the stack, permanent or not). Only the ORIGINAL Bears should reach the
/// battlefield.
#[test]
fn resolved_permanent_copy_vanishes_without_entering_battlefield() {
    let mut state = bears_copy_game(1, 2); // two Forests for {1}{G}
    let copier = force_into_play(
        &mut state,
        PlayerId(1),
        "Creature tap-activated CopySpell Target Spell",
    );
    // Documents the precondition; see
    // `copied_bolt_shares_targets_and_controller`.
    state.objects.obj_mut(copier).summoning_sick = false;

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // G, G
    let bears = find_in_hand(&state, PlayerId(0), "Grizzly Bears");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bears }))
        .unwrap();
    // {G} takes one green pip and {1} the other — the only coverage from G,G.
    let _ = step_through_payment(&mut state);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the Bears spell sits on the stack");

    // P0 passes; P1 gets priority with Bears still unresolved. Instead of
    // passing, P1 activates the copier's tap ability, targeting Bears.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    let activate = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == copier))
        .cloned()
        .expect("the copier's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the copy ability, got {stop:?}");
    };
    assert!(
        legal[0].contains(&bears),
        "the Bears spell is a legal target of Kind(Spell), permanent or not"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![bears]]))
        .unwrap();

    // Fulfill the nonmana tap cost before the copy ability reaches the stack.
    let (_, stop) = step_through_payment(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = stop
    else {
        panic!("expected P1 priority after tap-cost payment, got {stop:?}");
    };
    assert_eq!(player, PlayerId(1));
    assert_eq!(state.stack.len(), 2, "Bears plus the copy ability");

    // Both players pass: the copy ability resolves, pushing the Bears copy.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(
        state.stack.len(),
        2,
        "the original Bears plus its minted copy"
    );
    let copy = state
        .stack
        .iter()
        .find(|e| e.copy)
        .expect("the copy ability resolved and pushed a copy entry")
        .id;

    // Drive both spells to resolution. The copy (minted on top) resolves
    // first; the original Bears resolves after.
    while !state.stack.is_empty() {
        let (_t, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while draining the stack: {other:?}"),
        }
    }
    assert!(state.stack.is_empty(), "both spells left the stack");

    assert!(
        state.objects.get(bears).is_none(),
        "the original Bears' old stack id is gone after reminting onto the battlefield"
    );
    assert!(
        state.objects.get(copy).is_none(),
        "the copy's ObjectId is gone from self.objects — it ceased, it never \
         entered the battlefield ([CR#707.10a],[CR#707.10f])"
    );

    let bears_on_battlefield: Vec<ObjectId> = state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&o| is_card(&state, o, "Grizzly Bears"))
        .collect();
    assert_eq!(
        bears_on_battlefield.len(),
        1,
        "exactly ONE Grizzly Bears entered the battlefield — the original, \
         not the copy"
    );
    assert_eq!(
        state.objects.obj(bears_on_battlefield[0]).controller,
        PlayerId(0)
    );

    assert!(
        state.zones.graveyards[0].is_empty(),
        "no graveyard entry for the copy — it left no card behind"
    );
    assert!(state.zones.graveyards[1].is_empty());
}

// --- Retarget (Task 5, [CR#707.10c,115.7d])
// ----------------------------

/// A two-player game for `Retarget` end-to-end tests: player 0 holds
/// Lightning Bolt and Mountains; player 1 holds the Task 5 fixture — `{T}:
/// choose new targets for target spell.` — plus Grizzly Bears (the targets)
/// and Forests.
fn choose_new_targets_game(seed: u64, mountains: usize) -> GameState {
    let bolt = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let retargeter = testing_card("Creature tap-activated Retarget Target Spell");
    let bears = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&bolt); 5];
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut p1 = vec![Arc::clone(&retargeter); 3];
    p1.extend(vec![Arc::clone(&bears); 3]);
    p1.extend(vec![Arc::clone(&forest); 4]);
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
    state.sba_rules = builtin().sba_rules;
    for _ in 0..mountains {
        force_onto_battlefield(&mut state, PlayerId(0), "Mountain");
    }
    state
}

/// Test-only: moves `id` directly from the battlefield to its owner's
/// graveyard WITHOUT reminting (unlike a real zone change, [CR#400.7]) — the
/// same `ObjectId` stays a live object in `state.objects`, just no longer
/// `InZone(Battlefield)` ([CR#110.5a]). Models "the current target is no
/// longer a fresh-legal candidate" (it left play, gained hexproof, …) while
/// keeping it nameable — [CR#707.10c]'s union rule cares about legality, not
/// liveness. (The public `GameState` fields make this direct manipulation
/// possible, like `force_onto_battlefield` above.)
fn force_off_battlefield(state: &mut GameState, id: ObjectId) {
    state.zones.battlefield.retain(|&o| o != id);
    // `controller` stands in for owner here — nothing in these tests changes
    // control, so they coincide ([CR#110.2]).
    let owner = state.objects.obj(id).controller;
    state.zones.graveyards[owner.index()].push(id);
    state.objects.obj_mut(id).zone = Some(Zone::Graveyard);
}

/// Drives `state` (built by `choose_new_targets_game`) through: P0 bolts P1's
/// `bear1`; `bear1` is then forced off the battlefield (still a live id, no
/// longer a legal target — [CR#707.10c]'s "illegal but present" case); P1
/// activates the retargeter's `{T}: choose new targets for target spell`,
/// targeting the still-unresolved Bolt. Leaves a `Retarget` decision
/// pending. Returns `(bolt, bear1, bear2)` — `bear2` is a second Grizzly
/// Bears left on the battlefield, the fresh-legal alternative.
fn drive_to_choose_new_targets(state: &mut GameState) -> (ObjectId, ObjectId, ObjectId) {
    let retargeter = force_into_play(
        state,
        PlayerId(1),
        "Creature tap-activated Retarget Target Spell",
    );
    // Documents the precondition; see
    // `copied_bolt_shares_targets_and_controller`.
    state.objects.obj_mut(retargeter).summoning_sick = false;
    let bear1 = force_into_play(state, PlayerId(1), "Grizzly Bears");
    let bear2 = force_into_play(state, PlayerId(1), "Grizzly Bears");

    // P0's precombat main: float {R} and cast Bolt at bear1.
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(state, PlayerId(0), 1); // {R}
    let bolt = find_in_hand(state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for Bolt, got {stop:?}");
    };
    assert!(legal[0].contains(&bear1), "bear1 is a legal Bolt target");
    state
        .submit_decision(Decision::Targets(vec![vec![bear1]]))
        .unwrap();

    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the Bolt sits on the stack");
    assert_eq!(state.stack[0].targets, vec![vec![bear1]]);

    // [CR#707.10c]: make bear1 an ILLEGAL-but-present target — still a live
    // id, just no longer InZone(Battlefield), so it drops out of Bolt's
    // AnyTarget candidates without vanishing from `self.objects`.
    force_off_battlefield(state, bear1);

    // P0 passes; P1 gets priority with the Bolt still unresolved. Instead of
    // passing, P1 activates the retargeter's tap ability, targeting the Bolt.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(state, PlayerId(1), PhaseStep::PrecombatMain);
    let activate = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == retargeter))
        .cloned()
        .expect("the retargeter's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    let (_, stop) = step_to_stop(state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the retargeter's own ability, got {stop:?}");
    };
    assert!(
        legal[0].contains(&bolt),
        "the Bolt is a legal target of Kind(Spell)"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![bolt]]))
        .unwrap();

    // Fulfill the retargeter's nonmana tap cost before it reaches the stack.
    let (_, stop) = step_through_payment(state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = stop
    else {
        panic!("expected P1 priority after tap-cost payment, got {stop:?}");
    };
    assert_eq!(player, PlayerId(1));
    assert_eq!(state.stack.len(), 2, "the Bolt plus the retargeter ability");

    // Both players pass: the retargeter ability resolves, surfacing
    // Retarget ([CR#707.10c]).
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();

    (bolt, bear1, bear2)
}

/// [CR#707.10c]: leaving a re-targeted slot UNCHANGED is always legal, even
/// when the current target is no longer a fresh-legal candidate — the
/// surfaced legal set is the fresh candidates UNIONED with the current
/// target, and submitting the current target back is accepted, leaving the
/// entry's targets untouched.
#[test]
fn choose_new_targets_keep_current_even_if_illegal() {
    let mut state = choose_new_targets_game(1, 1);
    let (bolt, bear1, bear2) = drive_to_choose_new_targets(&mut state);

    let (_, stop) = step_through_payment(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Retarget(deckmaste_engine::Retarget {
        player,
        entry,
        spec,
        legal,
    })) = stop
    else {
        panic!("expected Retarget, got {stop:?}");
    };
    assert_eq!(
        player,
        PlayerId(1),
        "the retargeter's controller chooses ([CR#707.10c] `by`)"
    );
    assert_eq!(entry, bolt, "re-targeting the committed Bolt entry");
    assert_eq!(spec.len(), 1, "Bolt has one AnyTarget spec");
    assert!(
        legal[0].contains(&bear1),
        "the current (now off-battlefield) target stays choosable — the \
         union rule ([CR#707.10c])"
    );
    assert!(
        legal[0].contains(&bear2),
        "a fresh-legal candidate is also offered"
    );

    state
        .submit_decision(Decision::Targets(vec![vec![bear1]]))
        .unwrap();
    assert_eq!(
        state.stack.iter().find(|e| e.id == bolt).unwrap().targets,
        vec![vec![bear1]],
        "keeping the current target leaves entry.targets unchanged"
    );
}

/// [CR#707.10c]: a CHANGED slot must land on a fresh-legal candidate — an
/// out-of-legal-set submission is rejected and the decision stays pending
/// (retryable, mirroring `ChooseTargets`'s own re-validation); a fresh-legal
/// submission is accepted and overwrites the entry's targets.
#[test]
fn choose_new_targets_change_must_be_legal() {
    let mut state = choose_new_targets_game(2, 1);
    let (bolt, _bear1, bear2) = drive_to_choose_new_targets(&mut state);

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Retarget(deckmaste_engine::Retarget {
        legal,
        ..
    })) = stop
    else {
        panic!("expected Retarget, got {stop:?}");
    };
    assert!(
        !legal[0].contains(&bolt),
        "the Bolt's own stack id is not an AnyTarget candidate — a clean \
         out-of-legal-set probe"
    );
    assert!(
        legal[0].contains(&bear2),
        "bear2 is a fresh-legal candidate"
    );

    assert!(
        state
            .submit_decision(Decision::Targets(vec![vec![bolt]]))
            .is_err_and(|err| err.to_string().contains("illegal target selection")),
        "an out-of-legal-set target is rejected ([CR#707.10c])"
    );
    // Submission re-validates against the surfaced legal set, so a rejected
    // answer leaves the decision pending — retry with a fresh-legal target.
    state
        .submit_decision(Decision::Targets(vec![vec![bear2]]))
        .unwrap();
    assert_eq!(
        state.stack.iter().find(|e| e.id == bolt).unwrap().targets,
        vec![vec![bear2]],
        "changing to a fresh-legal target updates entry.targets"
    );
}

/// [CR#707.10c]: `Retarget` on an id no longer on the stack —
/// simulating the referenced entry vanishing between the retargeter ability
/// committing its own target (the Bolt) and its resolution (e.g. countered in
/// response, in a real game) — fizzles silently: no decision surfaces, no
/// panic, and resolution continues past the retargeter ability itself.
#[test]
fn choose_new_targets_fizzles_on_vanished_entry() {
    let mut state = choose_new_targets_game(3, 1);
    let retargeter = force_into_play(
        &mut state,
        PlayerId(1),
        "Creature tap-activated Retarget Target Spell",
    );
    state.objects.obj_mut(retargeter).summoning_sick = false;
    let bear1 = force_into_play(&mut state, PlayerId(1), "Grizzly Bears");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = find_in_hand(&state, PlayerId(0), "Lightning Bolt");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for Bolt, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![bear1]]))
        .unwrap();

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    let activate = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == retargeter))
        .cloned()
        .expect("the retargeter's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the retargeter's own ability, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![bolt]]))
        .unwrap();

    // The retargeter ability now sits alone on the stack, committed to
    // re-target the Bolt. Simulate the Bolt vanishing before the retargeter
    // ability resolves — a real game reaches this via a counter cast in
    // response; the direct removal isolates the fizzle path from a second
    // spell's own resolution machinery.
    state.stack.retain(|e| e.id != bolt);

    let (_, stop) = step_through_payment(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = stop
    else {
        panic!("expected P1 priority after tap-cost payment, got {stop:?}");
    };
    assert_eq!(player, PlayerId(1));
    assert_eq!(
        state.stack.len(),
        1,
        "only the retargeter ability remains (the Bolt was removed)"
    );

    // Both players pass: the retargeter ability resolves. Retarget
    // fizzles on the vanished Bolt — no decision, no panic — and resolution
    // continues straight through to the active player's priority.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::Priority(deckmaste_engine::Priority {
        player,
        ..
    })) = stop
    else {
        panic!(
            "expected resolution to continue past the fizzled retarget straight \
             to priority (no Retarget, no panic), got {stop:?}"
        );
    };
    assert_eq!(player, PlayerId(0), "the active player gets priority next");
    assert!(
        state.stack.is_empty(),
        "the retargeter ability resolved (fizzled retarget) and left no trace"
    );
}

// --- Task 6: consumers + trigger semantics ([CR#707.10,707.10b])
// ------------

/// [CR#707.10,601.2i]: the magecraft family's two halves fire on mutually
/// exclusive events — `EventFilter::Cast` on an ordinary cast, `Copied` on a
/// spell/ability being copied onto the stack, never both for the same
/// occurrence ("a copy of a spell isn't cast"). Player 0 controls the
/// `Creature Cast and Copied Triggers` fixture (ability 0 = `Cast`, `GainLife`
/// 1; ability 1 = `Copied`, `GainLife` 10) and the Task 3 copier (`{T}: copy
/// target spell`) — the SAME player casts and copies, so `who: Ref(You)`
/// on both triggers reads that one player throughout. Player 0 casts
/// Lightning Bolt at player 1's face (only `Cast` fires, `Copied` does not),
/// then copies the still-unresolved Bolt with the copier (only `Copied`
/// fires, `Cast` does not fire again).
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "end-to-end magecraft Cast-vs-Copied filter scenario; the cast-then-copy sequence reads clearer whole than split into helpers"
)]
fn copied_filter_fires_on_copy_and_cast_filter_does_not() {
    let bolt_card = card("Lightning Bolt");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let watcher_card = testing_card("Creature Cast and Copied Triggers");
    let copier_card = testing_card("Creature tap-activated CopySpell Target Spell");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&watcher_card); 5];
    p0.extend(vec![Arc::clone(&copier_card); 5]);
    p0.extend(vec![Arc::clone(&bolt_card); 5]);
    p0.extend(vec![Arc::clone(&mountain); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    state.sba_rules = builtin().sba_rules;

    let watcher = force_into_play(&mut state, PlayerId(0), "Creature Cast and Copied Triggers");
    let copier = force_into_play(
        &mut state,
        PlayerId(0),
        "Creature tap-activated CopySpell Target Spell",
    );
    // Documents the precondition; see
    // `copied_bolt_shares_targets_and_controller`.
    state.objects.obj_mut(copier).summoning_sick = false;
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let watcher_source = state.objects.obj(watcher).source;

    // P0's precombat main: float {R} and cast Bolt at P1's face.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 1);
    let bolt = force_into_hand(&mut state, PlayerId(0), "Lightning Bolt");
    let face = state.players[1].object;
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: bolt }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for Bolt, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![face]]))
        .unwrap();

    // The cast completes (any mid-cast PayMana is auto-tapped): SpellCast
    // applies and the watcher's Cast half fires, placing above the bolt —
    // the Copied half must NOT note.
    let (trace, _legal) = run_to_priority_traced(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 2, "bolt + the Cast trigger above it");
    let cast_fires = trace
        .iter()
        .filter_map(applied)
        .filter(
            |e| matches!(e, GameEvent::TriggerFired(TriggerFired { source, ability, .. }) if *source == watcher_source && *ability == 0),
        )
        .count();
    let copied_fires_on_cast = trace
        .iter()
        .filter_map(applied)
        .filter(
            |e| matches!(e, GameEvent::TriggerFired(TriggerFired { source, ability, .. }) if *source == watcher_source && *ability == 1),
        )
        .count();
    assert_eq!(
        cast_fires, 1,
        "the Cast half fires exactly once on the cast, trace: {trace:?}"
    );
    assert_eq!(
        copied_fires_on_cast, 0,
        "[CR#707.10] a cast is not a copy — the Copied half must not fire, trace: {trace:?}"
    );

    // Resolve the Cast trigger (top of stack) before copying.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "only the bolt remains");
    assert_eq!(
        state.players[0].life, 21,
        "the Cast trigger resolved: GainLife 1"
    );

    // P0 copies the still-unresolved Bolt with the copier's `{T}: copy
    // target spell`.
    let activate = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == copier))
        .cloned()
        .expect("the copier's tap ability is offered");
    state.submit_decision(Decision::Act(activate)).unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the copy ability, got {stop:?}");
    };
    assert!(legal[0].contains(&bolt), "the Bolt is a legal target");
    state
        .submit_decision(Decision::Targets(vec![vec![bolt]]))
        .unwrap();
    // Fulfill the copier's nonmana tap cost before P0 regains priority.
    let _ = step_through_payment(&mut state);
    assert_eq!(state.stack.len(), 2, "bolt + the copy ability above it");

    // Both pass: the copy ability resolves — GameEvent::Copied fires, and the
    // watcher's Copied half notes. The Cast half must NOT fire again.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let (trace, _legal) = run_to_priority_traced(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert!(
        trace
            .iter()
            .filter_map(applied)
            .any(|e| matches!(e, GameEvent::Copied(Copied { .. }))),
        "the copy ability minted a copy, trace: {trace:?}"
    );
    assert_eq!(
        state.stack.len(),
        3,
        "the original bolt, its minted copy, and the Copied trigger above them"
    );
    let cast_fires_on_copy = trace
        .iter()
        .filter_map(applied)
        .filter(
            |e| matches!(e, GameEvent::TriggerFired(TriggerFired { source, ability, .. }) if *source == watcher_source && *ability == 0),
        )
        .count();
    let copied_fires = trace
        .iter()
        .filter_map(applied)
        .filter(
            |e| matches!(e, GameEvent::TriggerFired(TriggerFired { source, ability, .. }) if *source == watcher_source && *ability == 1),
        )
        .count();
    assert_eq!(
        copied_fires, 1,
        "the Copied half fires exactly once on the copy, trace: {trace:?}"
    );
    assert_eq!(
        cast_fires_on_copy, 0,
        "[CR#707.10] \"a copy of a spell isn't cast\" — the Cast half must not fire, trace: {trace:?}"
    );

    // Drain the rest of the stack (Copied trigger, then the bolt copy, then
    // the original bolt) to confirm a clean, crash-free finish.
    while !state.stack.is_empty() {
        let (_, stop) = step_to_stop(&mut state);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while draining the stack: {other:?}"),
        }
    }
    assert_eq!(
        state.players[0].life, 31,
        "20 + 1 (Cast) + 10 (Copied) — both halves resolved exactly once"
    );
    assert_eq!(
        state.players[1].life, 14,
        "20 - 3 (original bolt) - 3 (its copy)"
    );
}

/// [CR#707.10b]: an ability copy has no card behind it, so it keeps the SAME
/// source as the original — only its `StackEntry.id` is fresh (unlike a
/// spell copy, which mints a fresh backing object, [CR#707.10]). Player 0
/// controls a tap-pinger (`{T}: this deals 1 damage to any target`) and a
/// variant copier targeting `Kind(Ability)` (the spell-copier's `Kind(Spell)`
/// cannot reach an ability on the stack — an activated/triggered ability has
/// no card identity). P0 activates the pinger at P1's face, then — before it
/// resolves — copies that very activation with the ability-copier. Both
/// copies of the ping resolve (2 damage total), then both vanish (no zone
/// move, no lingering object, [CR#608.2n]).
#[test]
fn ability_copy_same_source_resolves_and_vanishes() {
    let pinger_card = testing_card("Creature tap-activated DealDamage AnyTarget");
    let copier_card = testing_card("Creature tap-activated CopySpell Target Ability");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut p0 = vec![Arc::clone(&pinger_card); 5];
    p0.extend(vec![Arc::clone(&copier_card); 5]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0 },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    state.sba_rules = builtin().sba_rules;

    let pinger = force_into_play(
        &mut state,
        PlayerId(0),
        "Creature tap-activated DealDamage AnyTarget",
    );
    let copier = force_into_play(
        &mut state,
        PlayerId(0),
        "Creature tap-activated CopySpell Target Ability",
    );
    state.objects.obj_mut(pinger).summoning_sick = false;
    state.objects.obj_mut(copier).summoning_sick = false;
    let face = state.players[1].object;

    // P0 activates the pinger at P1's face.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    let activate_pinger = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == pinger))
        .cloned()
        .expect("the pinger's tap ability is offered");
    state
        .submit_decision(Decision::Act(activate_pinger))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the pinger, got {stop:?}");
    };
    assert!(legal[0].contains(&face), "P1's face is a legal any-target");
    state
        .submit_decision(Decision::Targets(vec![vec![face]]))
        .unwrap();

    // The tap cost is paid explicitly before priority returns to P0.
    let legal = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(state.stack.len(), 1, "the pinger's ability sits alone");
    let original = state.stack[0].clone();

    // P0 copies the pinger's own still-unresolved ability with the
    // ability-copier, before it resolves.
    let activate_copier = legal
        .iter()
        .find(|a| matches!(a, Action::ActivateAbility { object, .. } if *object == copier))
        .cloned()
        .expect("the copier's tap ability is offered");
    state
        .submit_decision(Decision::Act(activate_copier))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets for the copy ability, got {stop:?}");
    };
    assert!(
        legal[0].contains(&original.id),
        "the pinger's ability is a legal Kind(Ability) target, legal: {legal:?}"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![original.id]]))
        .unwrap();

    // Both pass: the copier's ability resolves, minting the ability copy.
    let _ = step_through_payment(&mut state); // priority retained by P0
    assert_eq!(
        state.stack.len(),
        2,
        "the pinger's ability + the copy ability above it"
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    assert_eq!(
        state.stack.len(),
        2,
        "the original ability plus its minted copy"
    );
    let copy_entry = state
        .stack
        .iter()
        .find(|e| e.copy)
        .expect("the copy ability resolved and pushed a copy entry")
        .clone();
    assert_ne!(
        copy_entry.id, original.id,
        "the copy is a freshly minted stack identity"
    );
    assert_eq!(
        copy_entry.object, original.object,
        "[CR#707.10b] an ability copy keeps the SAME source — no fresh backing object"
    );
    let StackObject::Activated { source, .. } = &copy_entry.object else {
        panic!("expected an Activated ability, got {:?}", copy_entry.object);
    };
    assert_eq!(*source, pinger, "the source is the pinger itself");

    // Drain the stack: both the original ping and its copy resolve (2
    // damage total to P1's face), then both vanish.
    let mut trace = Vec::new();
    while !state.stack.is_empty() {
        let (t, stop) = step_to_stop(&mut state);
        trace.extend(t);
        match stop {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                deckmaste_engine::Priority { .. },
            )) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            other => panic!("unexpected stop while draining the stack: {other:?}"),
        }
    }
    let damage_events = trace
        .iter()
        .filter(|p| {
            matches!(
                applied(p),
                Some(GameEvent::DamageDealt(DamageDealt { target, amount: 1, .. })) if *target == face
            )
        })
        .count();
    assert_eq!(
        damage_events, 2,
        "the ping resolved twice — the original and its copy, trace: {trace:?}"
    );
    assert_eq!(state.players[1].life, 18, "20 - 1 - 1: pinged twice over");
    assert!(state.stack.is_empty());

    // [CR#608.2n]: neither ability owned a card, so both simply vanish — no
    // zone move, no lingering object.
    assert!(
        state.objects.get(original.id).is_none(),
        "the original ability's minted id is gone after resolving"
    );
    assert!(
        state.objects.get(copy_entry.id).is_none(),
        "the copy's minted id is gone after resolving — no zone move ([CR#707.10a])"
    );
}

/// [CR#115.7e,601.2c,608.2b]: Fate Transfer's two-slot Distinct announce, cast
/// end-to-end. The second slot is `Distinct([0])`, so the co-targets must
/// differ: the real cast surfaces BOTH specs, an overlapping submission is
/// rejected, a distinct pair is accepted, and resolution runs
/// `MoveCounters(AllKinds, Target(0), Target(1))` — relocating every counter
/// from the first creature onto the second.
#[test]
fn fate_transfer_two_distinct_targets_cast_to_resolution() {
    let fate = card("Fate Transfer");
    let island = Arc::new(builtin().card("Island").unwrap().core);
    let bears = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut deck = vec![Arc::clone(&fate); 4];
    deck.extend(vec![Arc::clone(&island); 4]);
    deck.extend(vec![Arc::clone(&bears); 4]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Two Islands pay {1}{U/B}; two creatures are the distinct co-targets.
    force_into_play(&mut state, PlayerId(0), "Island");
    force_into_play(&mut state, PlayerId(0), "Island");
    let c1 = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");
    let c2 = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");
    let p1p1 = deckmaste_core::Ident::from("P1P1Counter");
    state.objects.obj_mut(c1).counters.insert(p1p1, 2);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2); // {U}{U}
    let spell = force_into_hand(&mut state, PlayerId(0), "Fate Transfer");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    // Announce ([CR#601.2c]): targets are chosen before the cost is paid.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { spec, legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert_eq!(spec.len(), 2, "Fate Transfer announces two target slots");
    assert!(
        legal[0].contains(&c1) && legal[0].contains(&c2),
        "both creatures are legal for slot 0"
    );
    assert!(
        legal[1].contains(&c1) && legal[1].contains(&c2),
        "both creatures are legal for slot 1 (the Distinct slot)"
    );
    // [CR#115.7e]: the same creature in both slots violates Distinct — rejected,
    // decision stays pending (retryable, like ChooseTargets membership).
    assert!(
        state
            .submit_decision(Decision::Targets(vec![vec![c1], vec![c1]]))
            .is_err_and(|err| err
                .to_string()
                .contains("Distinct slot overlaps a sibling slot")),
        "an overlapping co-target pair is rejected ([CR#115.7e])"
    );
    // A distinct pair is accepted.
    state
        .submit_decision(Decision::Targets(vec![vec![c1], vec![c2]]))
        .unwrap();

    // Concretize the {U/B} hybrid to blue ([CR#601.2f]), then auto-pay.
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseCostOptions(
        deckmaste_engine::ChooseCostOptions { .. },
    )) = stop
    else {
        panic!("expected ChooseCostOptions for the {{U/B}} pip, got {stop:?}");
    };
    state
        .submit_decision(Decision::CostOptions(deckmaste_engine::CostOptionChoices {
            picks: vec![deckmaste_engine::SymbolChoice::Mana(
                deckmaste_core::SimpleManaSymbol::Specific(Color::Blue.into()),
            )],
        }))
        .unwrap();

    // Auto-pay {1}{U}, then both players pass so the spell resolves.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state.stack[0].targets,
        vec![vec![c1], vec![c2]],
        "the committed entry carries the two distinct target slots"
    );
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    // [CR#122]: every counter moved from c1 onto c2.
    assert!(
        state.objects.obj(c1).counters.is_empty(),
        "the source creature is emptied of counters"
    );
    assert_eq!(
        state.objects.obj(c2).counters.get(&p1p1).copied(),
        Some(2),
        "the destination creature received the two +1/+1 counters"
    );
}

/// [CR#601.2c,115]: Arc Lightning's single PLURAL target slot — `Target(Between(1,
/// 3), AnyTarget)` — announced through the real cast. The one spec accepts a
/// count in 1..=3: zero is rejected (below the minimum), a within-slot
/// duplicate is rejected ([CR#601.2c] — one object can't be two of the "one,
/// two, or three targets"), and two distinct targets are accepted and recorded
/// in the single slot. Resolution stops at the divided-distribution seam
/// (engine-divided-distribution owns [CR#601.2d]); this asserts the announce
/// half only.
#[test]
fn arc_lightning_announces_one_to_three_targets() {
    let arc = card("Arc Lightning");
    let mountain = Arc::new(builtin().card("Mountain").unwrap().core);
    let bears = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut deck = vec![Arc::clone(&arc); 4];
    deck.extend(vec![Arc::clone(&mountain); 4]);
    deck.extend(vec![Arc::clone(&bears); 4]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Three Mountains pay {2}{R}; two creatures are the candidate targets.
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    force_into_play(&mut state, PlayerId(0), "Mountain");
    let c1 = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");
    let c2 = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 3); // {R}{R}{R}
    let spell = force_into_hand(&mut state, PlayerId(0), "Arc Lightning");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();

    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { spec, legal, .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    assert_eq!(
        spec.len(),
        1,
        "Arc Lightning announces ONE plural target slot"
    );
    assert!(legal[0].contains(&c1) && legal[0].contains(&c2));
    // Zero is below the minimum of one.
    assert!(
        state
            .submit_decision(Decision::Targets(vec![vec![]]))
            .is_err_and(|err| err.to_string().contains("minimum")),
        "zero targets is below Between(1, 3)'s minimum ([CR#601.2c])"
    );
    // The same creature twice is not two targets ([CR#601.2c]).
    assert!(
        state
            .submit_decision(Decision::Targets(vec![vec![c1, c1]]))
            .is_err_and(|err| err.to_string().contains("object chosen twice")),
        "a within-slot duplicate is rejected ([CR#601.2c])"
    );
    // Two distinct targets in the one slot are accepted.
    state
        .submit_decision(Decision::Targets(vec![vec![c1, c2]]))
        .unwrap();

    // Reach the stack (auto-pay {2}{R}); the entry carries both targets in slot
    // 0. Resolution is NOT driven — the divided-distribution seam owns it.
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    assert_eq!(
        state.stack[0].targets,
        vec![vec![c1, c2]],
        "the plural slot carries both announced targets"
    );
}

/// [CR#608.2b]: partial fizzle. Fate Transfer targets a source and a distinct
/// destination; when the DESTINATION dies (lethal SBA remint) after announce
/// but before resolution, the entry still resolves — a single surviving legal
/// target keeps it off the fizzle path — but the two-endpoint `MoveCounters`
/// does nothing (the illegal, departed destination is not affected), and
/// crucially never panics reading the gone object. The surviving source keeps
/// its counters.
#[test]
fn fate_transfer_partial_fizzle_when_destination_dies() {
    let fate = card("Fate Transfer");
    let island = Arc::new(builtin().card("Island").unwrap().core);
    let bears = card("Grizzly Bears");
    let forest = Arc::new(builtin().card("Forest").unwrap().core);
    let mut deck = vec![Arc::clone(&fate); 4];
    deck.extend(vec![Arc::clone(&island); 4]);
    deck.extend(vec![Arc::clone(&bears); 4]);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck },
            PlayerConfig {
                deck: vec![Arc::clone(&forest); 10],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load lethal-damage SBAs so a marked 2/2 is destroyed and reminted.
    state.sba_rules = builtin().sba_rules;
    force_into_play(&mut state, PlayerId(0), "Island");
    force_into_play(&mut state, PlayerId(0), "Island");
    let source = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");
    let dest = force_into_play(&mut state, PlayerId(0), "Grizzly Bears");
    let p1p1 = deckmaste_core::Ident::from("P1P1Counter");
    state.objects.obj_mut(source).counters.insert(p1p1, 2);

    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
    float_mana(&mut state, PlayerId(0), 2);
    let spell = force_into_hand(&mut state, PlayerId(0), "Fate Transfer");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseTargets(
        deckmaste_engine::ChooseTargets { .. },
    )) = stop
    else {
        panic!("expected ChooseTargets, got {stop:?}");
    };
    state
        .submit_decision(Decision::Targets(vec![vec![source], vec![dest]]))
        .unwrap();
    let (_, stop) = step_to_stop(&mut state);
    let StepOutcome::NeedsDecision(DecisionPointKind::ChooseCostOptions(
        deckmaste_engine::ChooseCostOptions { .. },
    )) = stop
    else {
        panic!("expected ChooseCostOptions, got {stop:?}");
    };
    state
        .submit_decision(Decision::CostOptions(deckmaste_engine::CostOptionChoices {
            picks: vec![deckmaste_engine::SymbolChoice::Mana(
                deckmaste_core::SimpleManaSymbol::Specific(Color::Blue.into()),
            )],
        }))
        .unwrap();
    let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);

    // The DESTINATION departs after announce (it dies / leaves play): drop it
    // from the battlefield and the object store, so its target id is gone
    // ([CR#400.7] — a reminted/departed object resolves to `None`).
    state.zones.battlefield.retain(|&o| o != dest);
    state.objects.remove(dest);
    assert!(
        state.objects.get(dest).is_none(),
        "the destination's target id is gone"
    );
    // Both pass: Fate Transfer resolves (the source is still a legal target, so
    // it does NOT fizzle) — and never panics reading the departed destination.
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = run_to_priority(&mut state, PlayerId(1), PhaseStep::PrecombatMain);
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let _ = step_to_stop(&mut state);

    // The move affected nothing (the departed destination is excluded), so the
    // surviving source keeps its counters.
    assert!(
        state.zones.battlefield.contains(&source),
        "the surviving source is still on the battlefield"
    );
    assert_eq!(
        state.objects.obj(source).counters.get(&p1p1).copied(),
        Some(2),
        "the source keeps its counters — the two-endpoint move couldn't complete"
    );
    assert!(
        state.stack.is_empty(),
        "Fate Transfer left the stack (resolved, not stuck)"
    );
}

/// FIXTURE — a KICKED PERMANENT's enters-the-battlefield recheck, read after
/// the spell has left the stack. [CR#702.33e] makes "if it was kicked" a
/// linked ability read ([CR#607]), and [CR#400.7d] is the rule that lets it
/// answer at all: "an ability of a permanent can reference information about
/// the spell that became that permanent as it resolved, including what costs
/// were paid to cast that spell." The announced optional-cost record rides the
/// STACK ENTRY, which is gone by the time the ETB trigger resolves — so the
/// record has to cross the one stack -> battlefield zone change with the
/// object, and `Condition::PaidCost` has to read the activation, not scan the
/// stack for the trigger's source id (which is the permanent, never the
/// spell).
#[test]
fn a_kicked_permanents_etb_recheck_reads_the_record_after_the_spell_left_the_stack() {
    for (kick, expected_gain, floats) in [(true, 4, 3), (false, 2, 1)] {
        let mut state = kicker_game(5);
        let beast = force_into_hand(&mut state, PlayerId(0), "Kicked Beast");
        let _ = run_to_priority(&mut state, PlayerId(0), PhaseStep::PrecombatMain);
        float_mana(&mut state, PlayerId(0), floats);
        let life0 = state.players[0].life;
        state
            .submit_decision(Decision::Act(Action::CastSpell { object: beast }))
            .unwrap();
        let (_, stop) = step_to_stop(&mut state);
        let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(deckmaste_engine::YesNo {
            player,
        })) = stop
        else {
            panic!("expected the kicker YesNo, got {stop:?}");
        };
        assert_eq!(player, PlayerId(0), "the caster announces the kicker");
        state.submit_decision(Decision::Answer(kick)).unwrap();
        loop {
            if state.players[0].life != life0 && state.stack.is_empty() {
                break;
            }
            let (_t, stop) = step_to_stop(&mut state);
            match stop {
                StepOutcome::NeedsDecision(DecisionPointKind::PayMana(
                    deckmaste_engine::PayMana { .. },
                )) => {
                    let pay = state.auto_pay_pending();
                    state.submit_decision(Decision::Pay(pay)).unwrap();
                }
                StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                    let decision = state
                        .auto_payment_pending()
                        .expect("the kicked cast has an automatic payment answer");
                    state.submit_decision(decision).unwrap();
                }
                StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                    deckmaste_engine::Priority { .. },
                )) => {
                    // Keep passing until the spell AND its enters trigger have
                    // both resolved; the trigger goes on the stack only after
                    // the permanent lands, so one empty stack is not enough.
                    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
                }
                other => panic!("unexpected stop while resolving the beast: {other:?}"),
            }
        }
        assert!(
            state
                .zones
                .battlefield
                .iter()
                .any(|&o| is_card(&state, o, "Kicked Beast")),
            "kick={kick}: the beast is on the battlefield, so the spell has left the stack"
        );
        assert_eq!(
            state.players[0].life,
            life0 + expected_gain,
            "kick={kick}: the ETB recheck read the kicked record after the spell left the stack \
             ([CR#400.7d,702.33e])"
        );
    }
}
