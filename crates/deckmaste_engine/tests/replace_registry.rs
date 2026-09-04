//! End-to-end tests for the replacement-effect registry ([CR#614]).
//! Task 4: `replace_event` loop + lineage + Instead/Also apply.
//! Task 5: `ChooseReplacement` decision ([CR#616.1]).
//! Task 6: Regeneration — `CreateReplacement`/`RemoveDamage` primitives.
//!
//! Pattern: synthetic `Card` scaffolding (in-Rust, no plugin), place a
//! `WorkItem::CheckSbas` on the agenda to trigger the SBA sweep, then
//! drive via `state.step()` until stable — similar to how the internal
//! `sba.rs` tests work, but through the public API.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::CausePattern;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Duration;
use deckmaste_core::EventFilter;
use deckmaste_core::Instruction;
use deckmaste_core::LifeOp;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::StatValue;
use deckmaste_core::StaticSpec;
use deckmaste_core::TurnMarker;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::CardId;
use deckmaste_engine::DamageDealt;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::LifeGained;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::ReplacementKey;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_plugin::plugin::Plugin;

/// Load the builtin plugin's `sba_rules` — used to populate `state.sba_rules`
/// so data-driven SBAs (lethal-damage destroy, toughness-0 move, etc.) fire.
fn builtin_sba_rules() -> Vec<deckmaste_core::SbaRule> {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"))
        .unwrap()
        .sba_rules
}

/// Load the builtin plugin's counter declarations — `M1M1Counter`'s
/// conferred -1/-1 rides them ([CR#122.1a]; consumer-injected like
/// `sba_rules`).
fn builtin_counter_decls()
-> std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::Counter> {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"))
        .unwrap()
        .counters
}

/// The abstract `Event` for "this permanent would be destroyed"
/// (BF→GY with verb "Destroy").
fn destroyed_self() -> EventFilter {
    EventFilter::ZoneChange {
        what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
        from: Some(Zone::Battlefield),
        to: Some(Zone::Graveyard),
        cause: Some(deckmaste_core::Cause::Cause(CausePattern {
            verb: Some(deckmaste_core::VerbName::from("Destroy")),
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
            Card::Normal(f)
            | Card::DoubleFaced { front: f, .. }
            | Card::Split { left: f, .. }
            | Card::Flip { normal: f, .. }
            | Card::Adventurer { normal: f, .. } => &*f.name,
        })
}

/// Find the first object in player 0's hand whose face name is `name`.
fn find_in_hand(state: &GameState, name: &str) -> ObjectId {
    *state.zones.hands[0]
        .iter()
        .find(|&&o| face_name(state, o) == Some(name))
        .unwrap_or_else(|| panic!("expected '{name}' in player 0's hand"))
}

/// A `Creature` `TypeDef` carrying the combat confers inline — the production
/// shape a plugin-loaded `Creature.ron` attaches, which a bare
/// `Type::Creature.def()` does NOT (empty confers by design). Combat-damage
/// marking now keys on the `May(Attack)` grant (a COMBATANT, [CR#120.3d]), so
/// a creature fixture asserting marked damage must confer it. Mirror of
/// legal.rs's `creature_typedef` / `damage_result_rules.rs`'s
/// `combatant_creature_def`.
fn combatant_creature_def() -> deckmaste_core::TypeDef {
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Condition;
    use deckmaste_core::CostPredicate;
    use deckmaste_core::Property;
    use deckmaste_core::StatePredicate;
    let sick_not_hasty = || {
        Condition::And(
            vec![
                Condition::Matches(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    Predicate::State(StatePredicate::SummoningSick),
                ),
                Condition::Not(Arc::new(Condition::Matches(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    Predicate::Characteristic(CharacteristicPredicate::Has("Haste".into())),
                ))),
            ]
            .into(),
        )
    };
    let ability = |s: StaticSpec| Property::Ability(Arc::new(Ability::r#static(s)));
    deckmaste_core::TypeDef {
        name: "Creature".into(),
        permanent_type: true,
        confers: vec![
            ability(StaticSpec::Deontic(Deontic::May(DeonticAction::Attack {
                by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                on: Predicate::Any,
            }))),
            ability(StaticSpec::Deontic(Deontic::May(DeonticAction::Block {
                by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                on: Predicate::Any,
                count: None,
            }))),
            ability(StaticSpec::Conditionally(
                sick_not_hasty(),
                Arc::new(StaticSpec::Deontic(Deontic::Cant(DeonticAction::Attack {
                    by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    on: Predicate::Any,
                }))),
            )),
            ability(StaticSpec::Conditionally(
                sick_not_hasty(),
                Arc::new(StaticSpec::Deontic(Deontic::Cant(
                    DeonticAction::Activate {
                        what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                        by: Predicate::Any,
                        cost: Some(CostPredicate::IncludesTapSymbol),
                    },
                ))),
            )),
        ]
        .into(),
    }
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
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
            replacement,
        )))],
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
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so data-driven SBAs (lethal-damage destroy) fire.
    state.sba_rules = builtin_sba_rules();
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
        name: name.into(),
        types: vec![Type::Creature.def()],
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
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so data-driven SBAs (lethal-damage destroy) fire.
    state.sba_rules = builtin_sba_rules();
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
/// The registry intercepts the `Act(Destroy)` SBA and redirects to exile.
#[test]
fn instead_redirects_destruction_to_exile() {
    // The `instead` body: Move(This, Zone(Exile)) is agent-silent.
    let instead_body = Instruction::Act(Action::Move(
        Reference::Reg(deckmaste_core::RefId(0)),
        deckmaste_core::Destination::Zone(Zone::Exile),
        vec![].into(),
        None,
    ));
    let (mut state, id) = creature_with_replacement(Replacement::Instead {
        would: destroyed_self(),
        instead: instead_body,
    });

    // Remember the card id so we can find the reminted object after zone move.
    let card_id = state.objects.obj(id).card_id().expect("backed by a card");

    // Mark lethal damage (toughness = 2, so damage ≥ 2 is lethal).
    state.objects.obj_mut(id).set_marked_damage(2);

    // Drive SBAs: CheckSbas → sweep → Act(Destroy) → replace_event → exile
    // instead.
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
/// `Act(Destroy)` before the replacement registry runs.
#[test]
fn indestructible_still_survives_via_cant_pass() {
    let (mut state, id) = creature_with_abilities(
        "Indestructible Test",
        1,
        1,
        vec![Ability::r#static(StaticSpec::CantHappen(
            EventFilter::ZoneChange {
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            },
        ))],
    );

    // Lethal damage (toughness 1).
    state.objects.obj_mut(id).set_marked_damage(1);

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
/// carrying an `Instead(would: Destroyed(This), instead: Sequentially([]))`.
/// When it receives lethal damage, the registry must gather both and surface a
/// `ChooseReplacement` decision.
fn creature_with_two_replacements() -> (GameState, ObjectId) {
    let instead = Replacement::Instead {
        would: destroyed_self(),
        instead: Instruction::Sequentially(vec![].into()),
    };
    let card = Arc::new(Card::Normal(CardFace {
        name: "Double Shield".into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        // Two SEPARATE static abilities so gather yields two different keys.
        abilities: vec![
            Ability::r#static(StaticSpec::Replacement(Arc::new(instead.clone()))),
            Ability::r#static(StaticSpec::Replacement(Arc::new(instead))),
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
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so data-driven SBAs (lethal-damage destroy) fire.
    state.sba_rules = builtin_sba_rules();
    let obj = find_in_hand(&state, "Double Shield");
    force_onto_battlefield(&mut state, obj);
    (state, obj)
}

/// Drive, stopping when a `NeedsDecision` is returned. Panics after 50 steps
/// without surfacing one.
fn drive_to_decision(state: &mut GameState) -> DecisionPointKind {
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
/// lethal damage → `Act(Destroy)` → `ChooseReplacement` surfaces.
/// Choosing either key cancels the event: creature survives, graveyard empty.
#[test]
fn two_applicable_replacements_surface_choice() {
    let (mut state, id) = creature_with_two_replacements();
    // Mark lethal damage (toughness 2).
    state.objects.obj_mut(id).set_marked_damage(2);

    // Drive to the ChooseReplacement decision.
    let dec = drive_to_decision(&mut state);
    let DecisionPointKind::ChooseReplacement(deckmaste_engine::ChooseReplacement {
        applicable,
        ..
    }) = dec
    else {
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
    state.objects.obj_mut(id).set_marked_damage(2);

    let dec = drive_to_decision(&mut state);
    let DecisionPointKind::ChooseReplacement(deckmaste_engine::ChooseReplacement {
        applicable,
        ..
    }) = dec
    else {
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

// ── Task 6: Regeneration helpers ─────────────────────────────────────────────

/// Build a vanilla creature (no abilities) on the battlefield.
fn vanilla_creature(power: i32, toughness: i32) -> (GameState, ObjectId) {
    creature_with_abilities("Vanilla", power, toughness, vec![])
}

/// Build a vanilla creature (no printed abilities) on the battlefield whose
/// `Creature` type CONFERS the combat capability (`combatant_creature_def`),
/// unlike `vanilla_creature`'s empty-confer `Type::Creature.def()`. Needed by
/// fixtures that drive a `DamageDealt` event through the real engine pipeline
/// and assert the resulting marked damage — `step.rs` now gates that marking
/// on `is_combatant` ([CR#120.3d,120.3e]).
fn combatant_vanilla_creature(power: i32, toughness: i32) -> (GameState, ObjectId) {
    let card = Arc::new(Card::Normal(CardFace {
        name: "Vanilla".into(),
        types: vec![combatant_creature_def()],
        power: Some(StatValue::Number(power)),
        toughness: Some(StatValue::Number(toughness)),
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
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so data-driven SBAs (lethal-damage destroy) fire.
    state.sba_rules = builtin_sba_rules();
    let obj = find_in_hand(&state, "Vanilla");
    force_onto_battlefield(&mut state, obj);
    (state, obj)
}

/// Run `effect` as a `RunEffect` work item with `source` as the frame source,
/// then drive until stable. Used to simulate an ability resolving.
///
/// Clears the agenda and any pending decision first so that pre-existing
/// game-startup work items (the initial `BeginStep(Untap)`) do not advance
/// the game into a priority window and prevent subsequent `drive_sbas` calls
/// from running.
fn resolve_and_drive(state: &mut GameState, effect: Instruction, source: ObjectId) {
    // Flush game-startup items; tests that call this function only care about
    // the effect's immediate consequences, not full turn progression.
    state.agenda.clear();
    state.pending = None;
    let controller = state.objects.obj(source).controller;
    let frame = state.frame(source, controller);
    state.agenda.push_front(WorkItem::RunEffect {
        effect: Arc::new(effect),
        frame,
    });
    drive(state);
}

/// Find the live `ObjectId` of `original_id`'s card on the battlefield
/// (the same id if the object didn't move; panics if absent).
fn find_on_battlefield(state: &GameState, card_id: CardId) -> ObjectId {
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&o| {
            state
                .objects
                .get(o)
                .and_then(deckmaste_engine::GameObject::card_id)
                == Some(card_id)
        })
        .expect("creature expected on battlefield")
}

// ── Task 6: Regeneration tests
// ────────────────────────────────────────────────

/// The first instruction definition in a region declaring the
/// [`deckmaste_core::event_region_params`] prefix (registers 0..=6).
const REGEN_SUBJECT: deckmaste_core::DefId = deckmaste_core::DefId(7);

/// Build the effect that registers a regeneration shield on `subject_ref`:
/// "the next time [subject] would be destroyed this turn, instead remove all
/// damage marked on it and tap it." [CR#701.19a,614.8]
///
/// Re-spelled from the deleted `With(TheRef(subject_ref), CreateReplacement)`
/// form: a binder is now an explicit register definition, so the subject is
/// pinned by a `Let` and `create_shield` freezes that product as the shield's
/// subject.
fn regenerate_effect(subject_ref: Reference) -> Instruction {
    // The shield resolves `subject` to a concrete object and remembers it; the
    // watch and body refer to that captured permanent as `EventObject`
    // (`Reg(2)`), NOT `Reg(0)` — the source register stays the source ability.
    let would = EventFilter::ZoneChange {
        what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(2))),
        from: Some(Zone::Battlefield),
        to: Some(Zone::Graveyard),
        cause: Some(deckmaste_core::Cause::Cause(CausePattern {
            verb: Some(deckmaste_core::VerbName::from("Destroy")),
            agency: None,
            agent: None,
        })),
    };
    let instead = Instruction::Sequentially(
        vec![
            // [CR#701.19a]: remove all damage from the regenerated permanent.
            Instruction::Act(Action::RemoveDamage(Reference::Reg(deckmaste_core::RefId(
                2,
            )))),
            // [CR#701.19a]: its controller taps it.
            Instruction::Act(Action::Tap(Reference::Reg(deckmaste_core::RefId(2)))),
        ]
        .into(),
    );
    // [CR#614.1]: the shield's subject — whatever it is a shield AROUND — is
    // the region product the preceding `Let` pinned, named on the instruction;
    // `create_shield` freezes that resolved identity. Semantics has no
    // `subject:` field, so lowering resolves the anaphor and declares it.
    Instruction::Sequentially(
        vec![
            Instruction::Let(deckmaste_core::Let {
                dest: REGEN_SUBJECT,
                expr: deckmaste_core::Expr::Object(subject_ref),
            }),
            Instruction::Act(Action::CreateReplacement {
                // The shield reads the register the preceding `Let` pinned —
                // the declared subject lowering resolves the anaphor to
                // ([CR#614.1]), not a search of the register file.
                subject: Reference::Reg(REGEN_SUBJECT.into()),
                replacement: Arc::new(Replacement::Instead { would, instead }),
                duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
                one_shot: true,
            }),
        ]
        .into(),
    )
}

/// Resolve `effect` as an activated ability of `source` ([CR#602.2a]), then
/// drive until stable — the region-anchored twin of [`resolve_and_drive`].
///
/// A `Let` destination is a register WRITE and `state.frame` carries no stored
/// register file, so a body that pins a product has to run under a real region
/// entry. Putting the body on the stack as an activated ability is the
/// production path that mints one; `resolve_and_drive`'s bare frame would
/// silently register no shield at all.
fn resolve_region_and_drive(state: &mut GameState, effect: Instruction, source: ObjectId) {
    state.agenda.clear();
    state.pending = None;
    let controller = state.objects.obj(source).controller;
    let ability = deckmaste_core::ActivatedAbility {
        ability_word: None,
        cost: deckmaste_core::Cost(Arc::from([])),
        from: None,
        window: None,
        condition: None,
        limits: Arc::from([]),
        targets: Arc::from([]),
        effect: deckmaste_core::Region::new(deckmaste_core::event_region_params(), effect.into()),
    };
    let object_source = state.objects.obj(source).source;
    let id = state
        .objects
        .mint(object_source, controller, Some(Zone::Stack));
    state.stack.push(deckmaste_engine::StackEntry {
        id,
        activation: deckmaste_engine::ActivationId::NONE,
        object: deckmaste_engine::StackObject::Activated {
            source,
            ability: Box::new(ability),
            bindings: deckmaste_engine::TriggerBindings::default(),
        },
        controller,
        targets: Vec::new(),
        chosen_modes: Arc::from([]),
        x: None,
        paid_costs: Vec::new(),
        copy: false,
    });
    state.agenda.push_front(WorkItem::Resolve(id));
    drive(state);
}

/// [CR#704.5h,701.19a]: a creature that regenerates from *deathtouch* damage
/// must survive. Deathtouch provenance now rides the damage mark itself, so
/// regeneration's heal — which removes the marks — drops the deathtouch clause
/// with them; the post-heal re-check sees no deathtouch source and does not
/// re-destroy the creature. Regression for the bug where a deal-time flag,
/// decoupled from the damage, survived the heal and drove a *second*
/// `Act(Destroy)` on the very next check with no shield left, wrongly
/// destroying the creature (Drudge Skeletons vs a deathtouch attacker).
#[test]
fn regenerated_creature_survives_deathtouch_strike() {
    let (mut state, id) = vanilla_creature(2, 2);
    let card_id = state.objects.obj(id).card_id().expect("backed by a card");

    // Resolve "Regenerate ~" — creates a single one-shot shield on id.
    resolve_region_and_drive(
        &mut state,
        regenerate_effect(Reference::Reg(deckmaste_core::RefId(0))),
        id,
    );
    assert_eq!(state.shields.len(), 1, "shield registered after regenerate");

    // SUBLETHAL physical damage (1 < toughness 2) dealt by a DEATHTOUCH source:
    // only the deal-time deathtouch provenance makes this lethal ([CR#704.5h]).
    state.objects.obj_mut(id).mark_damage(
        None,
        vec![deckmaste_core::Ability::Keyword(
            deckmaste_core::KeywordAbility::Deathtouch,
        )],
        1,
    );

    // Drive SBAs: first check → Act(Destroy) → shield replaces → heal (marks
    // cleared, taking the deathtouch provenance with them) + tap; the re-check
    // then sees no deathtouch source and no lethal damage.
    drive_sbas(&mut state);

    let live = find_on_battlefield(&state, card_id);
    assert!(
        state.objects.get(live).is_some(),
        "creature regenerated from deathtouch damage must survive; bf={:?}",
        state.zones.battlefield
    );
    assert_eq!(
        state.objects.obj(live).total_damage(),
        0,
        "damage — and its deathtouch provenance — must be cleared by regeneration [CR#701.19a]"
    );
    assert!(
        state.objects.obj(live).damage.is_empty(),
        "no marks remain after the heal, so the healed creature is not re-destroyed ([CR#704.5h])"
    );
    assert!(
        state.shields.is_empty(),
        "only one shield existed and it was consumed by the single destroy [CR#614.3]"
    );
}

/// [CR#701.19a,614.8]: Resolve "Regenerate ~" on a 2/2 → a shield registers.
/// Then mark lethal damage → SBA → `Act(Destroy)` → the shield replaces it:
/// damage is removed, creature is tapped, shield is consumed. Creature
/// survives.
#[test]
fn regenerated_creature_survives_lethal_damage() {
    let (mut state, id) = vanilla_creature(2, 2);
    let card_id = state.objects.obj(id).card_id().expect("backed by a card");

    // Resolve "Regenerate ~" — creates a shield on id.
    resolve_region_and_drive(
        &mut state,
        regenerate_effect(Reference::Reg(deckmaste_core::RefId(0))),
        id,
    );
    assert_eq!(state.shields.len(), 1, "shield registered after regenerate");

    // Mark lethal damage (toughness = 2).
    state.objects.obj_mut(id).set_marked_damage(5);

    // Drive SBAs: sweep → Act(Destroy) → shield replaces → heal + tap.
    drive_sbas(&mut state);

    // Creature must still be on the battlefield (same id — it didn't move).
    let live = find_on_battlefield(&state, card_id);
    assert!(
        state.objects.get(live).is_some(),
        "regenerated creature must survive lethal damage; bf={:?}",
        state.zones.battlefield
    );
    assert_eq!(
        state.objects.obj(live).total_damage(),
        0,
        "damage must be cleared by regeneration [CR#701.19a]"
    );
    assert!(
        state.objects.obj(live).tapped,
        "creature must be tapped by regeneration [CR#701.19a]"
    );
    assert!(
        state.shields.is_empty(),
        "one-shot shield must be consumed after use [CR#614.3]"
    );
}

/// "Regenerate TARGET creature" — the shield's SOURCE (the regenerating spell/
/// ability) is a different object than its SUBJECT (the protected creature).
/// Because the shield matches by stored subject identity and its body reads
/// `EventObject` (the affected permanent), the SUBJECT is healed and tapped
/// while the source is untouched — and the source register never had to move
/// off the source. [CR#701.19a]
#[test]
fn regenerate_target_creature_heals_the_subject_not_the_source() {
    use deckmaste_engine::InstanceId;
    use deckmaste_engine::ReplacementInstance;

    let subj_card = Arc::new(Card::Normal(CardFace {
        name: "Subject".into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        ..CardFace::default()
    }));
    let src_card = Arc::new(Card::Normal(CardFace {
        name: "Source".into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(1)),
        toughness: Some(StatValue::Number(1)),
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![subj_card, src_card],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so data-driven SBAs (lethal-damage destroy) fire.
    state.sba_rules = builtin_sba_rules();
    let subject = find_in_hand(&state, "Subject");
    force_onto_battlefield(&mut state, subject);
    let source = find_in_hand(&state, "Source");
    force_onto_battlefield(&mut state, source);
    let subj_card_id = state
        .objects
        .obj(subject)
        .card_id()
        .expect("backed by a card");

    // A shield protecting the SUBJECT, created by a distinct SOURCE. The
    // effect pins the subject with a leading `Let`; this test builds the shield
    // instance directly, so it peels that instruction off to reach the
    // replacement (the shield's subject is set explicitly below).
    let Instruction::Sequentially(steps) =
        regenerate_effect(Reference::Reg(deckmaste_core::RefId(0)))
    else {
        unreachable!("regenerate_effect builds a Let + CreateReplacement sequence")
    };
    let [
        _,
        Instruction::Act {
            action: Action::CreateReplacement { replacement, .. },
            ..
        },
    ] = &*steps
    else {
        unreachable!("the pinned subject is followed by the CreateReplacement")
    };
    state.shields.push(ReplacementInstance {
        id: InstanceId(7),
        replacement: (**replacement).clone(),
        subject,
        source, // distinct from subject — the key of this test
        duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        one_shot: true,
    });

    // Clear the startup agenda so the cascade is just the SBA sweep + the
    // shield body (a leftover BeginStep(Untap) would untap the active player's
    // creature again after regeneration taps it).
    state.agenda.clear();
    state.pending = None;

    state.objects.obj_mut(subject).set_marked_damage(5); // lethal
    drive_sbas(&mut state);

    let live = find_on_battlefield(&state, subj_card_id);
    assert!(
        state.objects.get(live).is_some(),
        "the targeted subject survives, not the source"
    );
    assert_eq!(
        state.objects.obj(live).total_damage(),
        0,
        "the SUBJECT's damage is removed — the body healed EventObject, not the source"
    );
    assert!(state.objects.obj(live).tapped, "the SUBJECT is tapped");
    assert!(
        state.objects.get(source).is_some() && state.zones.battlefield.contains(&source),
        "the source is untouched"
    );
    assert!(
        state.shields.is_empty(),
        "one-shot shield consumed [CR#614.3]"
    );
}

/// [CR#614.3]: A regeneration shield registered at end of turn is swept by
/// `expire_end_of_turn`. After expiry, a fresh lethal hit destroys the
/// creature.
#[test]
fn regeneration_shield_expires_end_of_turn() {
    let (mut state, id) = vanilla_creature(2, 2);
    let card_id = state.objects.obj(id).card_id().expect("backed by a card");

    // Register a regen shield.
    resolve_region_and_drive(
        &mut state,
        regenerate_effect(Reference::Reg(deckmaste_core::RefId(0))),
        id,
    );
    assert_eq!(state.shields.len(), 1, "shield registered");

    // Simulate end-of-turn sweep (what cleanup calls).
    state.expire_end_of_turn();
    assert!(
        state.shields.is_empty(),
        "shield must expire at end of turn"
    );

    // Now a lethal hit must destroy the creature (no shield left).
    state.objects.obj_mut(id).set_marked_damage(5);
    drive_sbas(&mut state);

    assert!(
        find_in_graveyard(&state, PlayerId(0), card_id).is_some(),
        "creature must be destroyed after shield expired; graveyard={:?}",
        state.zones.graveyards[0]
    );
}

// ── Task 7: Umbra/totem armor ([CR#702.89a]) ─────────────────────────────────

/// Build a `GameState` with a 2/2 creature enchanted by an "Umbra Armor" Aura.
/// The Aura carries a static `Instead` that watches the enchanted permanent's
/// destruction ([CR#702.89a]): if the creature would be destroyed, instead
/// remove all damage from it and destroy the Aura.
///
/// Returns `(state, creature_card_id, aura_card_id)` — the creature and Aura
/// are both live on the battlefield; `aura.attached_to == Some(creature)`.
fn enchanted_with_umbra() -> (GameState, CardId, CardId) {
    // The `would.what` for "the enchanted permanent":
    // `Predicate::Ref(Reference::AttachHostOf(Arc::new(Reference::Reg(deckmaste_core::RefId(0)))))` —
    // "the object THIS (the Aura) is attached to" ([CR#702.89a]).
    let enchanted_perm = Predicate::Ref(Reference::AttachHostOf(Arc::new(Reference::Reg(
        deckmaste_core::RefId(0),
    ))));

    let instead_body = Instruction::Sequentially(
        vec![
            // [CR#701.19a,702.89a]: remove all damage from the enchanted permanent.
            Instruction::Act(Action::RemoveDamage(Reference::AttachHostOf(Arc::new(
                Reference::Reg(deckmaste_core::RefId(0)),
            )))),
            // [CR#702.89a]: destroy this Aura.
            Instruction::Act(Action::destroy(Reference::Reg(deckmaste_core::RefId(0)))),
        ]
        .into(),
    );

    let umbra_armor = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: enchanted_perm,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some(deckmaste_core::VerbName::from("Destroy")),
                agency: None,
                agent: None,
            })),
        },
        instead: instead_body,
    };

    // Creature card: a 2/2 with no abilities.
    let creature_card = Arc::new(Card::Normal(CardFace {
        name: "Host Creature".into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(2)),
        ..CardFace::default()
    }));

    // Aura card: Enchantment with the umbra-armor static PLUS the default-deny
    // `May(Attach to: Creature)` grant, without which the SBA sweep would treat
    // the manual attachment as illegal and unattach it.
    let aura_card = Arc::new(Card::Normal(CardFace {
        name: "Umbra Armor".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![
            Ability::r#static(StaticSpec::Deontic(Deontic::May(DeonticAction::Attach {
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                to: Predicate::creature(),
            }))),
            Ability::r#static(StaticSpec::Replacement(Arc::new(umbra_armor))),
        ],
        ..CardFace::default()
    }));

    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![creature_card, aura_card],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so data-driven SBAs (lethal-damage destroy) fire.
    state.sba_rules = builtin_sba_rules();

    // Both cards are in hand after `GameState::new`; force them to the
    // battlefield.
    let creature_obj = find_in_hand(&state, "Host Creature");
    let aura_obj = find_in_hand(&state, "Umbra Armor");

    let creature_card_id = state.objects.obj(creature_obj).card_id().unwrap();
    let aura_card_id = state.objects.obj(aura_obj).card_id().unwrap();

    force_onto_battlefield(&mut state, creature_obj);
    force_onto_battlefield(&mut state, aura_obj);

    // Manually attach the Aura to the creature (bypass the enters-attached
    // replacement for the test setup — we care about the static's behavior,
    // not the attachment process).
    state.objects.obj_mut(aura_obj).attached_to = Some(creature_obj);

    (state, creature_card_id, aura_card_id)
}

/// [CR#702.89a,614.1a]: an Aura with umbra-armor watches the enchanted
/// permanent's destruction. When the creature gets lethal damage:
/// - `Act(Destroy(creature))` is gathered (the Aura's static other-watches it).
/// - The `Instead` fires: remove damage from creature, destroy the Aura.
/// - The creature SURVIVES (damage cleared); the Aura goes to the graveyard.
#[test]
fn umbra_armor_redirects_host_destruction_to_aura() {
    let (mut state, creature_card_id, aura_card_id) = enchanted_with_umbra();

    // Find current object ids (the cards were force-moved onto the
    // battlefield).
    let creature = find_on_battlefield(&state, creature_card_id);

    // Mark lethal damage on the creature (toughness = 2).
    state.objects.obj_mut(creature).set_marked_damage(5);

    // Drive SBAs: SBA sweep → Act(Destroy(creature)) → Aura's static gathered
    // → Instead fires → RemoveDamage + Destroy(Aura).
    drive_sbas(&mut state);

    // The creature must survive on the battlefield.
    let live_creature = find_on_battlefield(&state, creature_card_id);
    assert!(
        state.objects.get(live_creature).is_some(),
        "host creature must survive (umbra armor replaced its destruction); \
         bf={:?}",
        state.zones.battlefield
    );
    // Damage must be cleared.
    assert_eq!(
        state.objects.obj(live_creature).total_damage(),
        0,
        "damage must be removed by umbra armor [CR#702.89a]"
    );
    // The creature must NOT be in the graveyard.
    assert!(
        find_in_graveyard(&state, PlayerId(0), creature_card_id).is_none(),
        "creature must not be in graveyard"
    );
    // The Aura must be in the graveyard (destroyed instead of the creature).
    assert!(
        find_in_graveyard(&state, PlayerId(0), aura_card_id).is_some(),
        "Aura must be in graveyard (destroyed instead of host creature) [CR#702.89a]; \
         graveyard={:?}",
        state.zones.graveyards[0]
    );
}

/// Ordinary destroy (no replacement) still sends the creature to the graveyard.
#[test]
fn ordinary_destroy_goes_to_graveyard() {
    let card = Arc::new(Card::Normal(CardFace {
        name: "Vanilla Creature".into(),
        types: vec![Type::Creature.def()],
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
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    // Load builtin rules so the lethal-damage SBA fires.
    state.sba_rules = builtin_sba_rules();
    let obj = find_in_hand(&state, "Vanilla Creature");
    force_onto_battlefield(&mut state, obj);
    let card_id = state.objects.obj(obj).card_id().expect("backed by a card");

    // Lethal damage.
    state.objects.obj_mut(obj).set_marked_damage(2);

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

// ── Task 8: Non-destroy genericity proof + lineage
// ────────────────────────────

/// Build a `GameState` with a creature (carrying the given `replacement`
/// static) on the battlefield for player 0. Returns `(state, id)`.
fn creature_with_non_destroy_replacement(replacement: Replacement) -> (GameState, ObjectId) {
    creature_with_replacement(replacement)
}

/// [CR#614,616.1]: a non-destroy replacement (`GainLife` → `LoseLife`) proves the
/// registry handles player-experienced intents, not just `Act(Destroy)`.
///
/// A creature carrying `Instead(would: LifeGained(who: Any), instead:
/// LoseLife(Literal(1)))` watches a `LifeGained` intent for player 0. When the
/// `LifeGained` intent fires, the Instead fires: the player loses 1 life
/// instead of gaining 3. This exercises the `Affected::Player` arm of
/// `affected` and proves the registry is not destroy-only.
///
/// `who: Predicate::Any` matches any object, which in the `Affected::Player`
/// case resolves against the player's proxy object (`matches_with` with `Any`
/// always returns true).
#[test]
fn lifegain_replaced_by_draw() {
    // Build the GainLife → LoseLife(1) instead. We use LoseLife rather than
    // Draw because player 0's library may be empty after the opening-hand draw,
    // and an empty-library draw would silently set `drew_from_empty` rather
    // than adding a card. LoseLife(1) is directly observable as a
    // life-total change.
    let would = EventFilter::LifeGained {
        who: Predicate::Any,
        amount: None,
    };
    let instead_body = Instruction::Act(deckmaste_core::Action::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
        LifeOp::Down(deckmaste_core::Count::Literal(1)),
    ));
    let (mut state, _src) = creature_with_non_destroy_replacement(Replacement::Instead {
        would,
        instead: instead_body,
    });

    // Clear the agenda and any pending decision so the manually-emitted event
    // is the only thing processed.
    state.agenda.clear();
    state.pending = None;

    let before_life = state.players[0].life;

    // Emit the LifeGained intent directly onto the agenda — the replacement
    // registry intercepts it before `apply` runs.
    state
        .agenda
        .push_front(WorkItem::Emit(deckmaste_engine::Occurrence::Single(
            deckmaste_engine::GameEvent::LifeGained(LifeGained {
                player: PlayerId(0),
                amount: 3,
                cause: None,
            }),
        )));

    // Drive until stable.
    drive(&mut state);

    // Player 0 should NOT have gained the 3 life (Instead: event replaced
    // away).
    assert!(
        state.players[0].life < before_life,
        "player 0 should lose 1 life from Instead body (not gain 3); \
         before={before_life}, after={}",
        state.players[0].life,
    );
    let expected_life = before_life - 1; // deckmaste_core::Int
    assert_eq!(
        state.players[0].life, expected_life,
        "player 0 should lose exactly 1 life from the Instead body"
    );
}

// ── T6: `ChangeLife(who, Set(n))` under a life-gain replacement ─────────────
//
// Recon verdict (pinning, not routing): the `Set` branch
// (`resolve/player_action.rs`) reads the current life, computes a delta, and
// emits a plain directional `LifeGained`/`LifeLost` through the SAME
// provenance-blind pipeline `lifegain_replaced_by_draw` above already pins
// for a hand-built event — no dedicated "set" event exists to route
// separately. These two tests exercise that pipeline from the REAL
// `Action::ChangeLife(_, Set(_))` resolution (`resolve_and_drive`, not a
// hand-pushed `GameEvent`), the Tainted-Remedy-over-Axis-of-Mortality shape.

/// [CR#119.5,119.10]: a `Set` target ABOVE current life synthesizes a
/// `LifeGained` delta — and that delta enters the would-replacement window
/// exactly like a dedicated `Up`. [CR#119.5]: "the player gains or loses the
/// necessary amount of life to end up with the new total." [CR#119.10]: "If
/// a source would cause [a player] to gain life... " — the synthesized gain
/// IS such a cause, so a `LifeGained`-watching Instead applies to it.
#[test]
fn set_life_above_current_enters_the_replacement_window() {
    let would = EventFilter::LifeGained {
        who: Predicate::Any,
        amount: None,
    };
    let instead_body = Instruction::Act(Action::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
        LifeOp::Down(deckmaste_core::Count::Literal(1)),
    ));
    let (mut state, src) = creature_with_non_destroy_replacement(Replacement::Instead {
        would,
        instead: instead_body,
    });

    let before_life = state.players[0].life;
    let target =
        deckmaste_core::Uint::try_from(before_life + 5).expect("target life total fits in u32");

    // The real production path: `Action::ChangeLife(You, Set(target))`
    // resolves, computes the +5 delta, and emits `LifeGained{amount: 5}`.
    resolve_and_drive(
        &mut state,
        Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Set(deckmaste_core::Count::Literal(target)),
        )),
        src,
    );

    // The synthesized gain was replaced: player 0 lost 1 life instead of
    // gaining 5 — the Set-derived delta never bypassed the Instead body.
    assert_eq!(
        state.players[0].life,
        before_life - 1,
        "the Set-derived +5 gain should have been replaced by the Instead \
         body (lose 1), not applied directly; before={before_life}, after={}",
        state.players[0].life,
    );
}

/// [CR#119.5,119.9,119.10]: setting life to its CURRENT total is a ZERO
/// delta — [CR#119.5] computes "the necessary amount", which is 0, and
/// [CR#119.9]/[CR#119.10] are explicit that a 0-life-gain "event has [not]
/// occurred" / "would [not] occur" — so a watching replacement never even
/// APPLIES (not "applies and does nothing"): life stays exactly where it
/// was, not reduced by the Instead body's own -1.
#[test]
fn set_life_equal_to_current_emits_nothing() {
    let would = EventFilter::LifeGained {
        who: Predicate::Any,
        amount: None,
    };
    let instead_body = Instruction::Act(Action::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
        LifeOp::Down(deckmaste_core::Count::Literal(1)),
    ));
    let (mut state, src) = creature_with_non_destroy_replacement(Replacement::Instead {
        would,
        instead: instead_body,
    });

    let before_life = state.players[0].life;
    let target = deckmaste_core::Uint::try_from(before_life).expect("starting life fits in u32");

    resolve_and_drive(
        &mut state,
        Instruction::Act(Action::ChangeLife(
            Reference::Reg(deckmaste_core::RefId(1)),
            LifeOp::Set(deckmaste_core::Count::Literal(target)),
        )),
        src,
    );

    assert_eq!(
        state.players[0].life, before_life,
        "Set-to-current-total is a zero delta: no LifeGained/LifeLost event \
         occurs, so the watching replacement's Instead body never fires \
         (life must be UNCHANGED, not before_life - 1)"
    );
}

/// [CR#614.5]: `double_damage_lineage_terminates` — a one-shot floating shield
/// watching `DamageDealt` to the creature fires once and is consumed. The body
/// schedules a fresh `DamageDealt` with a fixed larger amount. That fresh event
/// enters the pipeline with the shield already consumed (one-shot), so
/// `gather_applicable` finds nothing and the loop terminates.
///
/// This proves:
/// 1. The registry handles `DamageDealt` (`Damage`-form) intents.
/// 2. A one-shot shield is consumed after application ([CR#614.3]).
/// 3. The fresh re-emitted event does NOT re-fire the same replacement (shield
///    gone), so the pipeline terminates.
///
/// SCOPE / SEAM ([CR#614.5]): the `applied` lineage set in `replace_event` is
/// LOCAL to one call. It terminates the in-call loop (an `Also` re-gather, an
/// `apply_one` that returns a modified event) and the one-shot path above. It
/// does NOT persist across the agenda: a *static* `Instead` whose body RE-EMITS
/// the watched event type to the same subject (the canonical "deals double
/// damage instead" — Furnace of Rath / Gratuitous Violence) would
/// re-enter `replace_event` with a fresh empty `applied` set and would not
/// terminate. No in-scope card and no parser path produces such a replacement
/// (hand-RON only). The robust fix is event-borne lineage (the deckmaster
/// prototype's per-event `replacement_lineage` bitmask) — tracked as a
/// follow-up. Do not paper over with an iteration cap.
///
/// Body form chosen: `DealDamage(Ref(This), Literal(10))` — a fixed amount
/// rather than a computed double, which avoids needing `ThatMuch`/semantic X
/// machinery. The test asserts termination (no stack overflow) and that the
/// fixed 10 damage lands exactly once.
#[test]
fn double_damage_lineage_terminates() {
    use deckmaste_core::Count;
    use deckmaste_engine::InstanceId;
    use deckmaste_engine::ReplacementInstance;

    // Build a vanilla 2/2 creature on the battlefield — a COMBATANT (confers
    // `May(Attack)`/`May(Block)`) so the fresh re-emitted `DamageDealt` below
    // is actually marked ([CR#120.3d]).
    let (mut state, id) = combatant_vanilla_creature(2, 2);
    let card_id = state.objects.obj(id).card_id().expect("backed by a card");

    // The `would`: "this creature would be dealt damage"
    //   Damage(to: Ref(This))
    let would = EventFilter::Damage {
        source: Predicate::Any,
        to: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
        combat: None,
        amount: None,
    };

    // The `instead` body: deal 10 damage to this creature (a fixed amount
    // rather than a doubled one — see doc-comment above for rationale).
    let instead_body = Instruction::Act(deckmaste_core::Action::deal_damage(
        Reference::Reg(deckmaste_core::RefId(0)),
        Count::Literal(10),
    ));

    // Register a ONE-SHOT floating shield on the creature.
    // After it fires, `consume_shield` removes it, so the fresh re-emitted
    // `DamageDealt` finds no applicable replacement and applies cleanly.
    state.shields.push(ReplacementInstance {
        id: InstanceId(42),
        replacement: Replacement::Instead {
            would,
            instead: instead_body,
        },
        subject: id,
        duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        one_shot: true,
        source: id,
    });

    // Clear the agenda so the manually-emitted event is processed in isolation.
    state.agenda.clear();
    state.pending = None;

    // Emit a `DamageDealt` intent with 2 damage to the creature.
    state
        .agenda
        .push_front(WorkItem::Emit(deckmaste_engine::Occurrence::Single(
            deckmaste_engine::GameEvent::DamageDealt(DamageDealt {
                source: id, // source = the creature itself (arbitrary for the test)
                target: id,
                amount: 2,
                combat: false,
            }),
        )));

    // Drive to stability — must terminate (no stack overflow / infinite loop).
    drive(&mut state);

    // The shield must be consumed (one-shot).
    assert!(
        state.shields.is_empty(),
        "one-shot shield must be consumed after firing [CR#614.3]"
    );

    // The creature must still be on the battlefield (10 damage < lethal for a
    // 2/2 without SBA running here — drive() doesn't push CheckSbas).
    let live = find_on_battlefield(&state, card_id);

    // The original 2-damage event was replaced away (Instead).
    // The body's 10-damage event applied once (no replacement on the fresh
    // event).
    assert_eq!(
        state.objects.obj(live).total_damage(),
        10,
        "the fixed-amount Instead body's damage must land exactly once; \
         actual={}",
        state.objects.obj(live).total_damage(),
    );
}

// ── Infect / Wither: source-keyed damage-as-counters ([CR#702.80,702.90]) ────
//
// Wither ([CR#702.80a]): damage dealt to a creature by a source with wither
// puts that many -1/-1 counters instead of being marked. Infect
// ([CR#702.90b,702.90c]): to a creature → -1/-1 counters; to a player → poison.
// Both are SOURCE abilities — the replacement's `would` keys on `source:
// Ref(This)` ("damage dealt BY this creature"), which exercises the
// `by`-matcher, and the body puts `Count::ThatMuch` counters on
// `Ref(ThatObject)` (the recipient).

/// Build a source creature with the given abilities plus a separate target
/// creature, both on player 0's battlefield. Returns `(state, source, target)`.
fn source_and_target(source_abilities: Vec<Ability>) -> (GameState, ObjectId, ObjectId) {
    let src_card = Arc::new(Card::Normal(CardFace {
        name: "Source".into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(3)),
        toughness: Some(StatValue::Number(3)),
        abilities: source_abilities,
        ..CardFace::default()
    }));
    let tgt_card = Arc::new(Card::Normal(CardFace {
        name: "Target".into(),
        // A COMBATANT (confers `May(Attack)`/`May(Block)`), unlike a bare
        // `Type::Creature.def()` — `by_matcher_fires_only_for_damage_from_its_own_source`
        // asserts this creature's OWN marked damage from a non-wither source
        // ([CR#120.3d,120.3e]).
        types: vec![combatant_creature_def()],
        power: Some(StatValue::Number(4)),
        toughness: Some(StatValue::Number(4)),
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![src_card, tgt_card],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let source = find_in_hand(&state, "Source");
    force_onto_battlefield(&mut state, source);
    let target = find_in_hand(&state, "Target");
    force_onto_battlefield(&mut state, target);
    state.agenda.clear();
    state.pending = None;
    (state, source, target)
}

/// Emit `amount` damage from `source` to `target` and drive to stability.
fn deal_damage(state: &mut GameState, source: ObjectId, target: ObjectId, amount: u32) {
    state
        .agenda
        .push_front(WorkItem::Emit(deckmaste_engine::Occurrence::Single(
            deckmaste_engine::GameEvent::DamageDealt(DamageDealt {
                source,
                target,
                amount,
                combat: false,
            }),
        )));
    drive(state);
}

/// A Static "damage by This to `on` → put `kind` counters on the recipient
/// instead" replacement — the shape both Wither and Infect expand to. The
/// recipient is read as `recipient` (`EventObject`/`Reg(2)` for a creature;
/// `EventPatient`/`Reg(3)` for a player — the kind-poly patient, [CR#119.9]'s
/// distinction: the affected player is never `EventActor`, the retired
/// compat alias).
///
/// The magnitude is `Count::Reg(RefId(6))` — the `EventAmount` register of the
/// static region's `event_region_params()` prefix. Re-spelled from the deleted
/// `Count::ThatMuch`, which read the same replaced-intent magnitude off a
/// `GameState` slot instead of a declared parameter.
fn damage_as_counters_static(on: Predicate, recipient: Reference, kind: &str) -> Ability {
    use deckmaste_core::Count;
    let would = EventFilter::Damage {
        source: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
        to: on,
        combat: None,
        amount: None,
    };
    let instead = Instruction::Act(Action::PutCounters(
        recipient,
        kind.into(),
        Count::Reg(deckmaste_core::RefId(6)),
    ));
    Ability::r#static(StaticSpec::Replacement(Arc::new(Replacement::Instead {
        would,
        instead,
    })))
}

/// The source-matcher: a source-keyed replacement (`source: Ref(Reg(0))`) fires
/// ONLY for damage from its own object. Damage from a DIFFERENT source must not
/// trigger it — the Wither source's counters appear only when IT deals the
/// damage, never when a third creature does.
#[test]
fn by_matcher_fires_only_for_damage_from_its_own_source() {
    let wither = damage_as_counters_static(
        Predicate::r#type(Type::Creature),
        Reference::Reg(deckmaste_core::RefId(2)),
        "M1M1Counter",
    );
    let (mut state, wither_src, target) = source_and_target(vec![wither]);
    let m1m1: deckmaste_core::Ident = "M1M1Counter".into();

    // `target` is a plain creature (no wither). Damage FROM it (a different
    // source than wither_src) must NOT trigger the wither replacement, whose
    // `would` is keyed `source: Ref(Reg(0))` = wither_src.
    deal_damage(&mut state, target, target, 4);
    assert_eq!(
        state.objects.obj(target).counters.get(&m1m1).copied(),
        None,
        "the wither replacement (source: Ref(Reg(0))=wither_src) must NOT fire for \
         damage from a different source [CR#702.80a]"
    );
    assert_eq!(
        state.objects.obj(target).total_damage(),
        4,
        "damage from a non-wither source is marked normally [CR#120.3e]"
    );

    // Now damage FROM the wither source: counters appear, no marked damage
    // added.
    deal_damage(&mut state, wither_src, target, 2);
    assert_eq!(
        state.objects.obj(target).counters.get(&m1m1).copied(),
        Some(2),
        "the wither replacement DOES fire for damage from its own source"
    );
}

/// [CR#702.90b]: an Infect source dealing N to a PLAYER gives that player N
/// poison counters instead of losing life.
#[test]
fn infect_source_gives_player_poison_not_life_loss() {
    let infect_player = damage_as_counters_static(
        Predicate::Entity(deckmaste_core::EntityClass::Player),
        Reference::Reg(deckmaste_core::RefId(3)),
        "Poison",
    );
    let (mut state, source, _target) = source_and_target(vec![infect_player]);
    let poison: deckmaste_core::Ident = "Poison".into();
    let victim = PlayerId(1);
    let victim_proxy = state.player(victim).object;
    let life_before = state.players[victim.index()].life;

    deal_damage(&mut state, source, victim_proxy, 3);

    assert_eq!(
        state
            .objects
            .obj(victim_proxy)
            .counters
            .get(&poison)
            .copied(),
        Some(3),
        "infect damage to a player = that many poison counters [CR#702.90b]"
    );
    assert_eq!(
        state.players[victim.index()].life,
        life_before,
        "infect damage to a player does NOT cause life loss [CR#702.90b]"
    );
}

/// [CR#702.90c]: an Infect source dealing N to a creature places N -1/-1
/// counters (the creature branch of infect, same shape as wither).
#[test]
fn infect_source_puts_minus_counters_on_a_creature() {
    let infect_creature = damage_as_counters_static(
        Predicate::r#type(Type::Creature),
        Reference::Reg(deckmaste_core::RefId(2)),
        "M1M1Counter",
    );
    let (mut state, source, target) = source_and_target(vec![infect_creature]);
    let m1m1: deckmaste_core::Ident = "M1M1Counter".into();

    deal_damage(&mut state, source, target, 2);

    assert_eq!(
        state.objects.obj(target).counters.get(&m1m1).copied(),
        Some(2),
        "infect damage to a creature = that many -1/-1 counters [CR#702.90c]"
    );
    assert_eq!(
        state.objects.obj(target).total_damage(),
        0,
        "not marked [CR#702.90c]"
    );
}

/// [CR#704.5c,122.1f]: ten or more poison counters → that player loses. Drives
/// the infect player branch up to 10, then runs the SBA sweep.
#[test]
fn ten_poison_counters_lose_the_game() {
    let infect_player = damage_as_counters_static(
        Predicate::Entity(deckmaste_core::EntityClass::Player),
        Reference::Reg(deckmaste_core::RefId(3)),
        "Poison",
    );
    let (mut state, source, _target) = source_and_target(vec![infect_player]);
    let victim = PlayerId(1);
    let victim_proxy = state.player(victim).object;

    // Two 5-damage infect hits → 10 poison counters.
    deal_damage(&mut state, source, victim_proxy, 5);
    deal_damage(&mut state, source, victim_proxy, 5);
    let poison: deckmaste_core::Ident = "Poison".into();
    assert_eq!(
        state
            .objects
            .obj(victim_proxy)
            .counters
            .get(&poison)
            .copied(),
        Some(10),
        "two 5-poison hits accumulate to 10"
    );

    // The SBA sweep must register the loss ([CR#704.5c]).
    drive_sbas(&mut state);
    assert!(
        state.players[victim.index()].lost,
        "a player with 10 poison counters loses the game [CR#704.5c]"
    );
}

/// A wither source damaging two creatures as one simultaneous batch: each
/// member is replaced independently ([CR#616.1]) into -1/-1 counters
/// ([CR#702.80a]), the counters land in the batch's wake with NO SBA
/// between members, and the SBA sweep runs only AFTER the whole batch —
/// both creatures then go to their graveyards together.
#[test]
fn wither_batch_places_counters_for_every_member_and_sbas_run_after() {
    let wither = damage_as_counters_static(
        Predicate::r#type(Type::Creature),
        Reference::Reg(deckmaste_core::RefId(2)),
        "M1M1Counter",
    );
    let four_four = |name: &str| {
        Arc::new(Card::Normal(CardFace {
            name: name.into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(4)),
            toughness: Some(StatValue::Number(4)),
            ..CardFace::default()
        }))
    };
    let src_card = Arc::new(Card::Normal(CardFace {
        name: "Source".into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(3)),
        toughness: Some(StatValue::Number(3)),
        abilities: vec![wither],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![src_card, four_four("Target"), four_four("Target Two")],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let source = find_in_hand(&state, "Source");
    force_onto_battlefield(&mut state, source);
    let t1 = find_in_hand(&state, "Target");
    force_onto_battlefield(&mut state, t1);
    let t2 = find_in_hand(&state, "Target Two");
    force_onto_battlefield(&mut state, t2);
    state.agenda.clear();
    state.pending = None;
    state.sba_rules = builtin_sba_rules();
    state.counter_decls = builtin_counter_decls();
    let m1m1: deckmaste_core::Ident = "M1M1Counter".into();

    // ONE batch of two damage packets (a "deals 4 damage to each of two
    // target creatures" shape).
    state
        .agenda
        .push_front(WorkItem::Emit(deckmaste_engine::Occurrence::Batch(vec![
            deckmaste_engine::GameEvent::DamageDealt(DamageDealt {
                source,
                target: t1,
                amount: 4,
                combat: false,
            }),
            deckmaste_engine::GameEvent::DamageDealt(DamageDealt {
                source,
                target: t2,
                amount: 4,
                combat: false,
            }),
        ])));
    drive(&mut state);

    // Counters landed for BOTH members; nothing was marked; and no SBA has
    // run between/after the members yet — both creatures still stand at
    // 0/0 until the next SBA boundary.
    for &t in &[t1, t2] {
        assert_eq!(
            state.objects.obj(t).counters.get(&m1m1).copied(),
            Some(4),
            "each batch member's damage became counters ([CR#702.80a])"
        );
        assert_eq!(state.objects.obj(t).total_damage(), 0, "no marked damage");
        assert!(
            state.zones.battlefield.contains(&t),
            "SBAs never run between a batch's members — only at the next \
             CheckSbas boundary"
        );
    }

    // The SBA boundary: both 0-toughness creatures leave TOGETHER.
    drive_sbas(&mut state);
    assert!(
        !state.zones.battlefield.contains(&t1) && !state.zones.battlefield.contains(&t2),
        "the toughness-0 SBA swept both after the whole batch ([CR#704.5f])"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        2,
        "both creatures in the graveyard"
    );
}

/// [CR#702.80a,120.3d]: a Wither source dealing N damage to a creature places N
/// -1/-1 counters on it instead of marking damage.
#[test]
fn wither_source_puts_minus_counters_not_marked_damage() {
    let wither = damage_as_counters_static(
        Predicate::r#type(Type::Creature),
        Reference::Reg(deckmaste_core::RefId(2)),
        "M1M1Counter",
    );
    let (mut state, source, target) = source_and_target(vec![wither]);
    let m1m1: deckmaste_core::Ident = "M1M1Counter".into();

    deal_damage(&mut state, source, target, 3);

    assert_eq!(
        state.objects.obj(target).counters.get(&m1m1).copied(),
        Some(3),
        "wither damage = that many -1/-1 counters [CR#702.80a]"
    );
    assert_eq!(
        state.objects.obj(target).total_damage(),
        0,
        "wither damage is NOT marked [CR#702.80a]"
    );
}

// ── EventPatient: the recipient read through the provenance-explicit role ────
//
// The damage recipient IS the event PATIENT ([CR#608.2k,120.3]) — the
// acted-upon thing, distinct from the agent (the source, reachable as `This`).
// The replacement frame binder now binds it as `EventPatient` (kind-poly), so a
// body reads the recipient through the role instead of the flat `ThatObject`/
// `ThatPlayer` aliases. These drive the same wither/infect shape through the
// new reference, proving the binder populates the patient slot end-to-end.

/// [CR#702.80a,608.2k]: a Wither source reading its creature recipient via the
/// kind-poly `EventPatient` (an OBJECT patient) places the -1/-1 counters on
/// it.
#[test]
fn event_patient_object_reads_the_damage_recipient_creature() {
    let wither = damage_as_counters_static(
        Predicate::r#type(Type::Creature),
        Reference::Reg(deckmaste_core::RefId(3)),
        "M1M1Counter",
    );
    let (mut state, source, target) = source_and_target(vec![wither]);
    let m1m1: deckmaste_core::Ident = "M1M1Counter".into();

    deal_damage(&mut state, source, target, 3);

    assert_eq!(
        state.objects.obj(target).counters.get(&m1m1).copied(),
        Some(3),
        "EventPatient resolves to the creature recipient (object patient)"
    );
    assert_eq!(
        state.objects.obj(target).total_damage(),
        0,
        "the damage is replaced, not marked"
    );
}

/// [CR#702.90b,608.2k,120.3]: an Infect source reading its PLAYER recipient via
/// `EventPatient` (a PLAYER patient — the proxy is zoneless) gives that player
/// the poison counters. The same role spells both kinds of recipient.
#[test]
fn event_patient_player_reads_the_damage_recipient_player() {
    let infect_player = damage_as_counters_static(
        Predicate::Entity(deckmaste_core::EntityClass::Player),
        Reference::Reg(deckmaste_core::RefId(3)),
        "Poison",
    );
    let (mut state, source, _target) = source_and_target(vec![infect_player]);
    let poison: deckmaste_core::Ident = "Poison".into();
    let victim = PlayerId(1);
    let victim_proxy = state.player(victim).object;
    let life_before = state.players[victim.index()].life;

    deal_damage(&mut state, source, victim_proxy, 3);

    assert_eq!(
        state
            .objects
            .obj(victim_proxy)
            .counters
            .get(&poison)
            .copied(),
        Some(3),
        "EventPatient resolves to the player recipient (player patient)"
    );
    assert_eq!(
        state.players[victim.index()].life,
        life_before,
        "the damage is replaced, not life loss"
    );
}
