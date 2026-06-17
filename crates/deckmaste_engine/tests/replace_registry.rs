//! End-to-end tests for the replacement-effect registry ([CR#614]).
//! Task 4: `replace_event` loop + lineage + Instead/Also apply.
//!
//! Pattern: synthetic `Card` scaffolding (in-Rust, no plugin), place a
//! `WorkItem::CheckSbas` on the agenda to trigger the SBA sweep, then
//! drive via `state.step()` until stable — similar to how the internal
//! `sba.rs` tests work, but through the public API.

use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::CausePattern;
use deckmaste_core::Effect;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::Selection;
use deckmaste_core::StatValue;
use deckmaste_core::StaticAbility;
use deckmaste_core::StaticEffect;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::CardId;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::ObjectSource;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;

fn base_state() -> GameState {
    GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    })
}

/// The abstract `Event` for "this permanent would be destroyed"
/// (BF→GY with verb "Destroy").
fn destroyed_self() -> Event {
    Event::ZoneMove {
        what: Filter::Ref(Reference::This),
        from: Some(Zone::Battlefield),
        to: Some(Zone::Graveyard),
        face: None,
        cause: Some(deckmaste_core::Cause::Cause(CausePattern {
            verb: Some("Destroy".into()),
            agency: None,
            agent: None,
        })),
    }
}

/// Mint a synthetic creature with the given replacement on player 0's
/// battlefield. P/T is 2/2, so lethal = 2 damage. Returns `(state, id)`.
fn creature_with_replacement(replacement: Replacement) -> (GameState, ObjectId) {
    let mut state = base_state();
    let card = Arc::new(Card::Normal(CardFace {
        name: "Test Creature".into(),
        types: vec![Type::Creature],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        abilities: vec![Ability::Static(StaticAbility {
            characteristic_defining: false,
            effects: vec![StaticEffect::Replacement(Box::new(replacement))],
            condition: None,
        })],
        ..CardFace::default()
    }));
    let card_id = state.cards.push(card, PlayerId(0));
    let id = state.objects.mint(
        ObjectSource::Card(card_id),
        PlayerId(0),
        Some(Zone::Battlefield),
    );
    state.zones.battlefield.push(id);
    (state, id)
}

/// Drive the SBA check by injecting a `CheckSbas` work item directly on the
/// agenda front, then stepping until the agenda is empty or a
/// decision/game-over surfaces. This avoids needing `sba::sweep` from the
/// integration test.
fn drive_sbas(state: &mut GameState) {
    state.agenda.push_front(WorkItem::CheckSbas);
    for _ in 0..50 {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
        }
        if state.agenda.is_empty() {
            break;
        }
    }
}

/// Find the live id of a card-backed object in exile by its card id.
/// The creature is reminted on zone change, so the original `ObjectId` may
/// be stale.
fn find_in_exile(state: &GameState, card_id: CardId) -> Option<ObjectId> {
    state.zones.exile.iter().copied().find(|&o| {
        state
            .objects
            .get(o)
            .and_then(deckmaste_engine::GameObject::card_id)
            == Some(card_id)
    })
}

/// Find the live id of a card-backed object in the given player's graveyard.
fn find_in_graveyard(state: &GameState, player: PlayerId, card_id: CardId) -> Option<ObjectId> {
    state.zones.graveyards[player.index()]
        .iter()
        .copied()
        .find(|&o| {
            state
                .objects
                .get(o)
                .and_then(deckmaste_engine::GameObject::card_id)
                == Some(card_id)
        })
}

/// [CR#614.1a,616.1]: a creature with `Instead(would: Destroyed(This), instead:
/// Exile(This))` given lethal damage is EXILED, not sent to the graveyard.
/// The registry intercepts the `WillDestroy` SBA and redirects to exile.
#[test]
fn instead_redirects_destruction_to_exile() {
    // The `instead` body: exile this object.
    // `Exile` is a `PlayerAction`; By(You, ...) is the implicit agent sugar.
    let instead_body = Effect::Act(Action::By(
        Reference::You,
        PlayerAction::Exile(Selection::Ref(Reference::This)),
    ));
    let (mut state, id) = creature_with_replacement(Replacement::Instead {
        would: destroyed_self(),
        instead: instead_body,
    });

    // Remember the card id so we can find the reminted object after zone move.
    let card_id = state.objects.obj(id).card_id().expect("backed by a card");

    // Mark lethal damage (toughness = 2, so damage ≥ 2 is lethal).
    state.objects.obj_mut(id).damage = 2;

    // Drive SBAs: CheckSbas → sweep → WillDestroy → replace_event → exile instead.
    drive_sbas(&mut state);

    // The creature should be in exile, NOT in the graveyard.
    assert!(
        find_in_exile(&state, card_id).is_some(),
        "creature should be in exile after Instead replacement; \
         exile={:?}, graveyard={:?}",
        state.zones.exile,
        state.zones.graveyards[0],
    );
    assert!(
        find_in_graveyard(&state, PlayerId(0), card_id).is_none(),
        "creature should NOT be in graveyard after Instead replacement"
    );
}

/// [CR#614.17,702.12b]: a creature with `CantHappen(Destroyed(This))`
/// (indestructible) survives lethal damage — the cant pass suppresses
/// `WillDestroy` before the replacement registry runs.
#[test]
fn indestructible_still_survives_via_cant_pass() {
    let mut state = base_state();
    let card = Arc::new(Card::Normal(CardFace {
        name: "Indestructible Test".into(),
        types: vec![Type::Creature],
        power: Some(StatValue::Number(1)),
        toughness: Some(StatValue::Number(1)),
        abilities: vec![Ability::Static(StaticAbility {
            characteristic_defining: false,
            effects: vec![StaticEffect::CantHappen(Event::ZoneMove {
                what: Filter::Ref(Reference::This),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                face: None,
                cause: None,
            })],
            condition: None,
        })],
        ..CardFace::default()
    }));
    let card_id_val = state.cards.push(card, PlayerId(0));
    let id: ObjectId = state.objects.mint(
        ObjectSource::Card(card_id_val),
        PlayerId(0),
        Some(Zone::Battlefield),
    );
    state.zones.battlefield.push(id);

    // Lethal damage (toughness 1).
    state.objects.obj_mut(id).damage = 1;

    drive_sbas(&mut state);

    // Must survive.
    assert!(
        state.objects.get(id).is_some(),
        "indestructible creature must survive lethal damage"
    );
    assert!(
        state.zones.battlefield.contains(&id),
        "must remain on battlefield"
    );
    assert!(
        state.zones.graveyards[0].is_empty(),
        "must not be in graveyard"
    );
}

/// Ordinary destroy (no replacement) still sends the creature to the graveyard.
#[test]
fn ordinary_destroy_goes_to_graveyard() {
    let mut state = base_state();
    let card = Arc::new(Card::Normal(CardFace {
        name: "Vanilla Creature".into(),
        types: vec![Type::Creature],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        ..CardFace::default()
    }));
    let card_id = state.cards.push(card, PlayerId(0));
    let id = state.objects.mint(
        ObjectSource::Card(card_id),
        PlayerId(0),
        Some(Zone::Battlefield),
    );
    state.zones.battlefield.push(id);

    // Lethal damage.
    state.objects.obj_mut(id).damage = 2;

    drive_sbas(&mut state);

    // Must be in graveyard.
    assert!(
        find_in_graveyard(&state, PlayerId(0), card_id).is_some(),
        "ordinary destroy should go to graveyard; graveyard={:?}",
        state.zones.graveyards[0],
    );
    assert!(
        find_in_exile(&state, card_id).is_none(),
        "must not be in exile"
    );
}
