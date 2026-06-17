//! End-to-end tests for the replacement-effect registry ([CR#614]).
//! Task 4: `replace_event` loop + lineage + Instead/Also apply.
//! Task 5: `ChooseReplacement` decision ([CR#616.1]).
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
use deckmaste_engine::Decision;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::ReplacementKey;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;

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

/// The face name of a card-backed object, if it is card-backed.
fn face_name(state: &GameState, id: ObjectId) -> Option<&str> {
    state
        .objects
        .get(id)
        .and_then(deckmaste_engine::GameObject::card_id)
        .map(|cid| match state.cards.get(cid).def.as_ref() {
            Card::Normal(f) | Card::ModalDfc(f, _) => f.name.as_str(),
        })
}

/// Find the first object in player 0's hand whose face name is `name`.
fn find_in_hand(state: &GameState, name: &str) -> ObjectId {
    *state.zones.hands[0]
        .iter()
        .find(|&&o| face_name(state, o) == Some(name))
        .unwrap_or_else(|| panic!("expected '{name}' in player 0's hand"))
}

/// Move `obj` from player 0's hand straight onto the battlefield (no
/// event loop, no land-drop limit). The public `GameState` fields make this
/// direct setup possible without widening the engine API.
fn force_onto_battlefield(state: &mut GameState, obj: ObjectId) {
    state.zones.hands[0].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
}

/// Build a `GameConfig` whose player-0 deck contains `card` and player-1
/// deck is empty. After `GameState::new`, the card (the only deck entry)
/// will be in player 0's opening hand. Then force it onto the battlefield.
/// Returns `(state, id)`.
fn creature_with_replacement(replacement: Replacement) -> (GameState, ObjectId) {
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
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: vec![card] },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let obj = find_in_hand(&state, "Test Creature");
    force_onto_battlefield(&mut state, obj);
    (state, obj)
}

/// Build a `GameState` with a single synthetic creature (given abilities) on
/// player 0's battlefield. Returns `(state, id)`.
fn creature_with_abilities(
    name: &str,
    power: i32,
    toughness: i32,
    abilities: Vec<Ability>,
) -> (GameState, ObjectId) {
    let card = Arc::new(Card::Normal(CardFace {
        name: name.to_owned(),
        types: vec![Type::Creature],
        power: Some(StatValue::Number(power)),
        toughness: Some(StatValue::Number(toughness)),
        abilities,
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: vec![card] },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let obj = find_in_hand(&state, name);
    force_onto_battlefield(&mut state, obj);
    (state, obj)
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
    let (mut state, id) = creature_with_abilities(
        "Indestructible Test",
        1,
        1,
        vec![Ability::Static(StaticAbility {
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
    );

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

/// Build a `GameState` with ONE creature that has TWO static abilities, each
/// carrying an `Instead(would: Destroyed(This), instead: Sequence([]))`.
/// When it receives lethal damage, the registry must gather both and surface a
/// `ChooseReplacement` decision.
fn creature_with_two_replacements() -> (GameState, ObjectId) {
    let instead = Replacement::Instead {
        would: destroyed_self(),
        instead: Effect::Sequence(vec![]),
    };
    let card = Arc::new(Card::Normal(CardFace {
        name: "Double Shield".into(),
        types: vec![Type::Creature],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        // Two SEPARATE static abilities so gather yields two different keys.
        abilities: vec![
            Ability::Static(StaticAbility {
                characteristic_defining: false,
                effects: vec![StaticEffect::Replacement(Box::new(instead.clone()))],
                condition: None,
            }),
            Ability::Static(StaticAbility {
                characteristic_defining: false,
                effects: vec![StaticEffect::Replacement(Box::new(instead))],
                condition: None,
            }),
        ],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: vec![card] },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let obj = find_in_hand(&state, "Double Shield");
    force_onto_battlefield(&mut state, obj);
    (state, obj)
}

/// Drive, stopping when a `NeedsDecision` is returned. Panics after 50 steps
/// without surfacing one.
fn drive_to_decision(state: &mut GameState) -> PendingDecision {
    state.agenda.push_front(WorkItem::CheckSbas);
    for _ in 0..50 {
        match state.step() {
            StepOutcome::NeedsDecision(dec) => return dec,
            StepOutcome::Progress(_) => {}
            StepOutcome::GameOver(_) => panic!("game ended before decision"),
        }
        if state.agenda.is_empty() {
            break;
        }
    }
    panic!("expected NeedsDecision but agenda drained first")
}

/// Drive until stable (no more progress) or game-over, ignoring decisions.
fn drive(state: &mut GameState) {
    for _ in 0..200 {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
        }
        if state.agenda.is_empty() {
            break;
        }
    }
}

/// [CR#616.1]: two applicable Instead-to-nothing replacements on one creature →
/// lethal damage → `WillDestroy` → `ChooseReplacement` surfaces.
/// Choosing either key cancels the event: creature survives, graveyard empty.
#[test]
fn two_applicable_replacements_surface_choice() {
    let (mut state, id) = creature_with_two_replacements();
    // Mark lethal damage (toughness 2).
    state.objects.obj_mut(id).damage = 2;

    // Drive to the ChooseReplacement decision.
    let dec = drive_to_decision(&mut state);
    let PendingDecision::ChooseReplacement { applicable, .. } = dec else {
        panic!("expected ChooseReplacement, got {dec:?}");
    };
    assert!(!applicable.is_empty(), "at least one key in choice");

    // Submit the first choice.
    state
        .submit_decision(Decision::ReplacementChoice(applicable[0]))
        .expect("submit should succeed");

    // Drive to stability.
    drive(&mut state);

    // Creature must survive (Instead-to-nothing replaced the destroy away).
    assert!(
        state.objects.get(id).is_some(),
        "creature should survive after replacement choice"
    );
    assert!(
        state.zones.graveyards[0].is_empty(),
        "graveyard must be empty after Instead-to-nothing replacement"
    );
}

/// Same setup but choosing the SECOND key — both branches must cancel the
/// event.
#[test]
fn two_applicable_replacements_second_choice_also_survives() {
    let (mut state, id) = creature_with_two_replacements();
    state.objects.obj_mut(id).damage = 2;

    let dec = drive_to_decision(&mut state);
    let PendingDecision::ChooseReplacement { applicable, .. } = dec else {
        panic!("expected ChooseReplacement, got {dec:?}");
    };
    let key1: ReplacementKey = applicable[0];
    let _ = key1; // verify type

    // Submit the second (last) key.
    let last_key = *applicable.last().expect("at least one key");
    state
        .submit_decision(Decision::ReplacementChoice(last_key))
        .expect("submit second key should succeed");

    drive(&mut state);

    assert!(
        state.objects.get(id).is_some(),
        "creature should survive after second replacement choice"
    );
    assert!(
        state.zones.graveyards[0].is_empty(),
        "graveyard must be empty"
    );
}

/// Ordinary destroy (no replacement) still sends the creature to the graveyard.
#[test]
fn ordinary_destroy_goes_to_graveyard() {
    let card = Arc::new(Card::Normal(CardFace {
        name: "Vanilla Creature".into(),
        types: vec![Type::Creature],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: vec![card] },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
    });
    let obj = find_in_hand(&state, "Vanilla Creature");
    force_onto_battlefield(&mut state, obj);
    let card_id = state.objects.obj(obj).card_id().expect("backed by a card");

    // Lethal damage.
    state.objects.obj_mut(obj).damage = 2;

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
