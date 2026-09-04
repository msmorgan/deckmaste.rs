//! `DamageResultRule` mechanism ([CR#120.3c]): damage to a permanent matching a
//! rule's `recipient` removes that many counters of the rule's kind — the data
//! form of "damage to a planeswalker removes loyalty counters". Also pins the
//! additive-dispatch fix ([CR#120.3]: damage has "one or more results"): a
//! permanent that is BOTH a creature and a planeswalker takes marked damage AND
//! loyalty removal, no longer one-or-the-other.

use std::sync::Arc;

use deckmaste_card::CardFace;
use deckmaste_card::Characteristics;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::CounterRef;
use deckmaste_core::DamageResultRule;
use deckmaste_core::Ident;
use deckmaste_core::Predicate;
use deckmaste_core::StatValue;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::DamageDealt;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;

/// The builtin planeswalker-loyalty rule as data.
fn loyalty_rule() -> DamageResultRule {
    DamageResultRule {
        recipient: Predicate::r#type(Type::Planeswalker),
        remove: CounterRef::from("LoyaltyCounter"),
    }
}

/// A synthetic non-creature planeswalker (types `[Planeswalker]`).
fn walker_card() -> deckmaste_card::Card {
    deckmaste_card::Card::Normal(CardFace::from(Characteristics {
        name: "Test Walker".into(),
        types: vec![Type::Planeswalker.def()],
        loyalty: Some(StatValue::Number(5)),
        ..Characteristics::default()
    }))
}

/// A `Creature` `TypeDef` carrying the combat confers inline — the production
/// shape a plugin-loaded `Creature.ron` attaches, which a bare
/// `Type::Creature.def()` does NOT (empty confers by design). Combat-damage
/// marking now keys on the `May(Attack)` grant (a COMBATANT), so a creature
/// fixture asserting marked damage must confer it. Mirror of legal.rs's
/// `creature_typedef`.
fn combatant_creature_def() -> deckmaste_core::TypeDef {
    use deckmaste_core::Ability;
    use deckmaste_core::Condition;
    use deckmaste_core::CostPredicate;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Property;
    use deckmaste_core::Reference;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticSpec;
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

/// A synthetic creature-planeswalker (types `[Creature, Planeswalker]`) — a
/// "creature-Gideon". Small toughness so a modest damage amount is lethal. Its
/// `Creature` type CONFERS the combat capability (via
/// `combatant_creature_def`), so combat-damage marking sees it as a combatant
/// ([CR#120.3d]).
fn creature_walker_card() -> deckmaste_card::Card {
    deckmaste_card::Card::Normal(CardFace::from(Characteristics {
        name: "Test Creature Walker".into(),
        types: vec![combatant_creature_def(), Type::Planeswalker.def()],
        power: Some(StatValue::Number(4)),
        toughness: Some(StatValue::Number(4)),
        loyalty: Some(StatValue::Number(5)),
        ..Characteristics::default()
    }))
}

/// A two-player game whose player 0 decks `card`, with `damage_result_rules`
/// as given (the knob these tests turn). No SBA rules — the tests assert the
/// raw deal-time results, not the downstream 0-counter sweep.
fn game_with(card: deckmaste_card::Card, rules: Vec<DamageResultRule>) -> GameState {
    let card = Arc::new(card);
    GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![Arc::clone(&card); 6],
            },
            PlayerConfig {
                deck: vec![Arc::clone(&card); 6],
            },
        ],
        seed: 1,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: rules,
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    })
}

/// Forces player 0's first planeswalker (from hand or library) onto the
/// battlefield and seeds it with `n` loyalty counters (the force path skips the
/// enters-with-loyalty replacement, so it would otherwise have none).
fn walker_on_field(state: &mut GameState, n: u32) -> ObjectId {
    let pw = state.zones.hands[0]
        .iter()
        .chain(state.zones.libraries[0].iter())
        .copied()
        .find(|&o| deckmaste_engine::matches(state, o, &Predicate::r#type(Type::Planeswalker)))
        .expect("a synthetic planeswalker in player 0's deck");
    state.zones.hands[0].retain(|&o| o != pw);
    state.zones.libraries[0].retain(|&o| o != pw);
    state.objects.obj_mut(pw).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(pw);
    state
        .objects
        .obj_mut(pw)
        .counters
        .insert(Ident::from("LoyaltyCounter"), n);
    pw
}

fn loyalty(state: &GameState, pw: ObjectId) -> u32 {
    state
        .objects
        .obj(pw)
        .counters
        .get("LoyaltyCounter")
        .copied()
        .unwrap_or(0)
}

/// Steps one queued `DamageDealt` event through the engine's apply funnel.
fn deal(state: &mut GameState, source: ObjectId, target: ObjectId, amount: u32) {
    state
        .agenda
        .push_front(deckmaste_engine::WorkItem::Emit(Occurrence::single(
            GameEvent::DamageDealt(DamageDealt {
                source,
                target,
                amount,
                combat: false,
            }),
        )));
    match state.step() {
        StepOutcome::Progress(Progress::Applied(_)) => {}
        other => panic!("expected Applied(_), got {other:?}"),
    }
}

/// With the loyalty rule present, a planeswalker dealt N damage loses N loyalty
/// counters ([CR#120.3c]) — and, being no creature, is NOT marked
/// ([CR#120.3e]).
#[test]
fn planeswalker_loses_loyalty_to_damage_via_the_rule() {
    let mut state = game_with(walker_card(), vec![loyalty_rule()]);
    let source = state.players[1].object;
    let pw = walker_on_field(&mut state, 5);
    deal(&mut state, source, pw, 3);
    assert_eq!(loyalty(&state, pw), 2, "5 loyalty − 3 damage = 2");
    assert_eq!(
        state.objects.obj(pw).total_damage(),
        0,
        "a non-creature planeswalker is not marked"
    );
}

/// With NO damage-result rule wired, the same planeswalker loses no loyalty —
/// proving the behavior is the DATA rule, not a residual hardcode.
#[test]
fn planeswalker_keeps_loyalty_when_the_rule_is_absent() {
    let mut state = game_with(walker_card(), vec![]);
    let source = state.players[1].object;
    let pw = walker_on_field(&mut state, 5);
    deal(&mut state, source, pw, 3);
    assert_eq!(
        loyalty(&state, pw),
        5,
        "no rule ⇒ no loyalty removal (not a hardcode)"
    );
}

/// The additive-dispatch fix ([CR#120.3] "one or more results"): a permanent
/// that is BOTH a creature and a planeswalker, dealt a non-lethal amount, ends
/// up with marked damage AND reduced loyalty — both results, not one.
#[test]
fn creature_planeswalker_takes_marked_damage_and_loyalty_loss() {
    let mut state = game_with(creature_walker_card(), vec![loyalty_rule()]);
    let source = state.players[1].object;
    let pw = walker_on_field(&mut state, 5);
    deal(&mut state, source, pw, 2);
    assert_eq!(
        state.objects.obj(pw).total_damage(),
        2,
        "creature result: 2 damage marked"
    );
    assert_eq!(loyalty(&state, pw), 3, "planeswalker result: 5 − 2 = 3");
}

/// A lethal amount to a creature-planeswalker feeds BOTH state-based inputs at
/// once: marked damage ≥ toughness (the lethal-damage SBA [CR#704.5g]) AND
/// loyalty driven to 0 (clamped) (the 0-loyalty SBA [CR#704.5i]).
#[test]
fn lethal_to_creature_planeswalker_feeds_both_sbas() {
    let mut state = game_with(creature_walker_card(), vec![loyalty_rule()]);
    let source = state.players[1].object;
    // 4 toughness, 5 loyalty. Deal 5: lethal to the creature, and clamps loyalty
    // to 0 (5 − 5).
    let pw = walker_on_field(&mut state, 5);
    deal(&mut state, source, pw, 5);
    assert!(
        state.objects.obj(pw).total_damage() >= 4,
        "marked damage ≥ toughness feeds the lethal-damage SBA"
    );
    assert_eq!(
        loyalty(&state, pw),
        0,
        "loyalty clamped to 0 feeds the 0-loyalty SBA"
    );
}
