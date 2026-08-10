use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::AggregateOp;
use deckmaste_core::Anchor;
use deckmaste_core::Binder;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::ChooseSpec;
use deckmaste_core::Color;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Countable;
use deckmaste_core::Destination;
use deckmaste_core::EventFilter;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaAbility;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::Modal;
use deckmaste_core::Mode;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Predicate;
use deckmaste_core::Projection;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Replacement;
use deckmaste_core::Sort;
use deckmaste_core::StatValue;
use deckmaste_core::StatePredicate;
use deckmaste_core::StaticEffect;
use deckmaste_core::Supertype;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::EngineIncident;
use deckmaste_engine::FulfillmentWitness;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::IouKind;
use deckmaste_engine::ManaCoverage;
use deckmaste_engine::ManaPayment;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentPrompt;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::PriorityRound;
use deckmaste_engine::ReversalBarrier;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;

fn activation_fixture(
    cost: Vec<CostComponent>,
) -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    activation_fixture_with_extras(cost, Vec::new())
}

fn activation_fixture_with_extras(
    cost: Vec<CostComponent>,
    extras: Vec<Arc<Card>>,
) -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    let payer = PlayerId(0);
    let card = Arc::new(Card::Normal(CardFace {
        name: "Replay fixture".into(),
        abilities: vec![Ability::activated(ActivatedAbility {
            ability_word: None,
            cost: Cost(cost.into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: OneShotEffect::Sequentially(Arc::from([])),
        })],
        ..CardFace::default()
    }));
    let mut deck = vec![card];
    deck.extend(extras);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
        seed: 17,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let source = state.zones.hands[payer.index()]
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == "Replay fixture",
            Card::TwoFaced { front, .. } => front.name.as_ref() == "Replay fixture",
        })
        .unwrap();
    state.zones.hands[payer.index()].retain(|&object| object != source);
    state.objects.obj_mut(source).zone = Some(Zone::Battlefield);
    state.objects.obj_mut(source).summoning_sick = false;
    state.zones.battlefield.push(source);
    let action = Action::ActivateAbility {
        object: source,
        ability: 0,
    };
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![action],
    }));
    (state, payer, source)
}

fn run_to_payment(state: &mut GameState) -> PaymentPrompt {
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Payment(prompt)) => return prompt,
            other => panic!("expected payment, got {other:?}"),
        }
    }
}

fn announce(state: &mut GameState, source: deckmaste_engine::ObjectId) -> PaymentPrompt {
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: source,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(state)
}

fn fulfill(
    state: &mut GameState,
    iou: deckmaste_engine::IouId,
    witness: FulfillmentWitness,
) -> PaymentPrompt {
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill { iou, witness }))
        .unwrap();
    run_to_payment(state)
}

fn green_source(name: &str) -> Arc<Card> {
    mana_source(name, Cost(vec![CostComponent::Tap].into()), Color::Green)
}

fn mana_source(name: &str, cost: Cost, color: Color) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: name.into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                cost,
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::Specific(color.into()).into(),
                )),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn krark_clan_ironworks() -> Arc<Card> {
    let sacrifice_artifact = CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::ChooseOne {
            filter: Predicate::And(
                vec![
                    Predicate::r#type(Type::Artifact),
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                ]
                .into(),
            ),
            by: Reference::You,
        }),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Sacrifice(
                Reference::You,
                Reference::That(Sort::Permanent),
            ))]
            .into(),
        ),
    };
    Arc::new(Card::Normal(CardFace {
        name: "Krark-Clan Ironworks".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                cost: Cost(vec![sacrifice_artifact].into()),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(2),
                    ManaSpec::Specific(deckmaste_core::ColorOrColorless::Colorless).into(),
                )),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn wheel_of_sun_and_moon() -> Arc<Card> {
    let instead = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(CoreAction::Reveal {
                    what: Reference::EventObject,
                    to: None,
                }),
                OneShotEffect::Act(CoreAction::Move(
                    Reference::EventObject,
                    Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                    Arc::from([]),
                    None,
                )),
            ]
            .into(),
        ),
    };
    Arc::new(Card::Normal(CardFace {
        name: "Wheel of Sun and Moon".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            instead,
        )))],
        ..CardFace::default()
    }))
}

fn mox_amber_fixture() -> Arc<Card> {
    let colored_legend = Predicate::And(
        vec![
            Predicate::Characteristic(CharacteristicPredicate::Supertype(Supertype::Legendary)),
            Predicate::Not(Arc::new(Predicate::Characteristic(
                CharacteristicPredicate::Colorless,
            ))),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    let effect = OneShotEffect::With(deckmaste_core::With {
        binder: Binder::ChooseOne {
            filter: colored_legend,
            by: Reference::You,
        },
        body: Arc::new(OneShotEffect::Act(CoreAction::AddMana(
            Reference::You,
            Count::Literal(1),
            ManaSpec::AmongColorsOf(Reference::That(Sort::Permanent)).into(),
        ))),
    });
    Arc::new(Card::Normal(CardFace {
        name: "Mox Amber".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                cost: Cost(vec![CostComponent::Tap].into()),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect,
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn colored_legendary_artifact() -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Colored legendary artifact".into(),
        mana_cost: "{G}".parse().unwrap(),
        supertypes: vec![Supertype::Legendary],
        types: vec![Type::Artifact.def(), Type::Creature.def()],
        abilities: vec![Ability::activated(ActivatedAbility {
            ability_word: None,
            cost: Cost(vec![CostComponent::Mana("{2}{G}".parse::<ManaCost>().unwrap())].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: OneShotEffect::Sequentially(Arc::from([])),
        })],
        ..CardFace::default()
    }))
}

fn omnath_fixture() -> Arc<Card> {
    let green = Count::ManaAvailableKind(Reference::You, Color::Green.into());
    Arc::new(Card::Normal(CardFace {
        name: "Omnath, Locus of Mana".into(),
        mana_cost: "{2}{G}".parse().unwrap(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Count(green.clone())),
        toughness: Some(StatValue::Count(green.clone())),
        abilities: vec![Ability::r#static(StaticEffect::Modify(
            Reference::This,
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Set(StatValue::Count(green.clone()))),
                    Modification::Toughness(NumericOp::Set(StatValue::Count(green))),
                ]
                .into(),
            ),
        ))],
        ..CardFace::default()
    }))
}

fn mana_cylix_fixture() -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Mana Cylix".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                cost: Cost(
                    vec![
                        CostComponent::Mana("{1}".parse::<ManaCost>().unwrap()),
                        CostComponent::Tap,
                    ]
                    .into(),
                ),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::AnyColor.into(),
                )),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn bighorner_rancher_fixture() -> Arc<Card> {
    let controlled_creature = Predicate::And(
        vec![
            Predicate::creature(),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    let greatest_power = Count::Aggregate(
        AggregateOp::MaxOf,
        Projection {
            of: Countable::Objects(Arc::new(controlled_creature)),
            by: Arc::new(Count::StatOf(Reference::It, deckmaste_core::Stat::Power)),
        },
    );
    Arc::new(Card::Normal(CardFace {
        name: "Bighorner Rancher".into(),
        mana_cost: "{3}{G}".parse().unwrap(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(1)),
        toughness: Some(StatValue::Number(1)),
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                cost: Cost(vec![CostComponent::Tap].into()),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    greatest_power,
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn put_named_in_play(
    state: &mut GameState,
    payer: PlayerId,
    name: &str,
) -> deckmaste_engine::ObjectId {
    let object = state.zones.hands[payer.index()]
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == name,
            Card::TwoFaced { front, .. } => front.name.as_ref() == name,
        })
        .unwrap();
    state.zones.hands[payer.index()].retain(|&candidate| candidate != object);
    state.objects.obj_mut(object).zone = Some(Zone::Battlefield);
    state.objects.obj_mut(object).summoning_sick = false;
    state.zones.battlefield.push(object);
    object
}

fn submit_tap_mana_action(
    state: &mut GameState,
    source: deckmaste_engine::ObjectId,
) -> deckmaste_engine::ManaActionId {
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(state);
    fulfill(state, child.outstanding[0].id, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let _ = run_to_payment(state);
    state
        .payment_records()
        .unwrap()
        .last()
        .and_then(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the completed child is recorded on its parent")
}

#[test]
fn rescind_replays_unrelated_later_fulfillment() {
    let pay_life = CostComponent::do_action(CoreAction::ChangeLife(
        Reference::You,
        LifeOp::Down(Count::Literal(2)),
    ));
    let (mut state, payer, source) = activation_fixture(vec![CostComponent::Tap, pay_life]);
    let prompt = announce(&mut state, source);
    let tap = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let life = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::PayLife(2)))
        .unwrap()
        .id;

    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    let prompt = fulfill(&mut state, life, FulfillmentWitness::PayLife);
    assert!(state.objects.obj(source).tapped);
    assert_eq!(state.player(payer).life, 18);
    assert_eq!(prompt.fulfilled, vec![tap, life]);

    state
        .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(tap)))
        .unwrap();

    let prompt = match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt,
        other => panic!("rescind should return to payment, got {other:?}"),
    };
    assert!(!state.objects.obj(source).tapped);
    assert_eq!(state.player(payer).life, 18);
    assert_eq!(prompt.fulfilled, vec![life]);
    assert_eq!(prompt.outstanding.len(), 1);
    assert_eq!(prompt.outstanding[0].id, tap);
}

#[test]
fn library_barrier_rejects_selective_rescind_without_mutation() {
    let put_on_bottom = CostComponent::Act(Arc::new(CoreAction::Move(
        Reference::This,
        Destination::Library(Anchor::FromBottom(Count::Literal(0))),
        Arc::from([]),
        Some(Zone::Battlefield),
    )));
    let (mut state, payer, source) = activation_fixture(vec![put_on_bottom]);
    let prompt = announce(&mut state, source);
    let move_iou = prompt.outstanding[0].id;
    let prompt = fulfill(&mut state, move_iou, FulfillmentWitness::Bound);
    assert_eq!(prompt.fulfilled, vec![move_iou]);
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    let records = state.payment_records().unwrap();
    assert_eq!(records.len(), 1);
    assert!(
        records[0]
            .reversal_barriers
            .contains(&ReversalBarrier::MovedToLibrary)
    );

    let before_library = state.zones.libraries[payer.index()].clone();
    let before_prompt = prompt.clone();
    let before_records = records.to_vec();
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(
                move_iou,
            )))
            .is_err()
    );

    assert_eq!(state.zones.libraries[payer.index()], before_library);
    assert_eq!(state.pending, Some(PendingDecision::Payment(before_prompt)));
    assert_eq!(state.payment_records().unwrap(), before_records);
}

#[test]
fn decline_unannounces_while_retaining_a_barred_root_fulfillment() {
    let put_on_bottom = CostComponent::Act(Arc::new(CoreAction::Move(
        Reference::This,
        Destination::Library(Anchor::FromBottom(Count::Literal(0))),
        Arc::from([]),
        Some(Zone::Battlefield),
    )));
    let (mut state, payer, source) = activation_fixture(vec![put_on_bottom]);
    let prompt = announce(&mut state, source);
    fulfill(
        &mut state,
        prompt.outstanding[0].id,
        FulfillmentWitness::Bound,
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert!(matches!(state.pending, Some(PendingDecision::Priority(_))));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records.len() == 1
                && incident.crossed_reversal_barrier
    ));
}

#[test]
fn unreplayable_dependency_rejects_without_repair_or_mutation() {
    let battlefield = Predicate::State(StatePredicate::InZone(Zone::Battlefield));
    let graveyard = Predicate::State(StatePredicate::InZone(Zone::Graveyard));
    let move_chosen = |filter, destination, from| CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::ChooseOne {
            filter,
            by: Reference::You,
        }),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Move(
                Reference::That(Sort::Permanent),
                Destination::Zone(destination),
                Arc::from([]),
                Some(from),
            ))]
            .into(),
        ),
    };
    let resource = Arc::new(Card::Normal(CardFace {
        name: "Replay resource".into(),
        ..CardFace::default()
    }));
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![
            move_chosen(battlefield, Zone::Graveyard, Zone::Battlefield),
            move_chosen(graveyard, Zone::Exile, Zone::Graveyard),
        ],
        vec![resource],
    );
    let resource = state.zones.hands[payer.index()][0];
    state.zones.hands[payer.index()].clear();
    state.objects.obj_mut(resource).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(resource);

    let prompt = announce(&mut state, source);
    let first = prompt.outstanding[0].id;
    let second = prompt.outstanding[1].id;
    fulfill(
        &mut state,
        first,
        FulfillmentWitness::Objects(vec![resource]),
    );
    let graveyard_resource = state.zones.graveyards[payer.index()][0];
    fulfill(
        &mut state,
        second,
        FulfillmentWitness::Objects(vec![graveyard_resource]),
    );
    assert!(state.zones.graveyards[payer.index()].is_empty());
    assert_eq!(state.zones.exile.len(), 1);

    let before_exile = state.zones.exile.clone();
    let before_prompt = state.pending.clone();
    let before_records = state.payment_records().unwrap().to_vec();
    assert!(
        state
            .submit_decision(Decision::Payment(
                PaymentCommand::RescindFulfillment(first,)
            ))
            .is_err()
    );

    assert_eq!(state.zones.exile, before_exile);
    assert_eq!(state.pending, before_prompt);
    assert_eq!(state.payment_records().unwrap(), before_records);
}

#[test]
fn replay_restores_ordinary_triggers_caused_by_a_retained_fulfillment() {
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Life-loss watcher".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            event: EventFilter::LifeLost {
                who: Predicate::Ref(Reference::You),
                amount: None,
            },
            from: None,
            condition: None,
            limits: Arc::from([]),
            where_x: None,
            effect: OneShotEffect::Sequentially(Arc::from([])),
        })],
        ..CardFace::default()
    }));
    let pay_life = CostComponent::do_action(CoreAction::ChangeLife(
        Reference::You,
        LifeOp::Down(Count::Literal(2)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![pay_life, CostComponent::Tap], vec![watcher]);
    put_named_in_play(&mut state, payer, "Life-loss watcher");
    let prompt = announce(&mut state, source);
    let life = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::PayLife(2)))
        .unwrap()
        .id;
    let tap = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    fulfill(&mut state, life, FulfillmentWitness::PayLife);
    assert_eq!(state.pending_triggers.len(), 1);
    fulfill(&mut state, tap, FulfillmentWitness::Bound);

    state
        .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(tap)))
        .unwrap();

    assert_eq!(state.player(payer).life, 18);
    assert_eq!(state.pending_triggers.len(), 1);
}

#[test]
fn replay_rebinds_a_retained_card_after_an_earlier_zone_remint_is_omitted() {
    let battlefield = Predicate::State(StatePredicate::InZone(Zone::Battlefield));
    let graveyard = Predicate::State(StatePredicate::InZone(Zone::Graveyard));
    let move_chosen = |filter, destination, from| CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::ChooseOne {
            filter,
            by: Reference::You,
        }),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Move(
                Reference::That(Sort::Permanent),
                Destination::Zone(destination),
                Arc::from([]),
                Some(from),
            ))]
            .into(),
        ),
    };
    let first_card = Arc::new(Card::Normal(CardFace {
        name: "First replay card".into(),
        ..CardFace::default()
    }));
    let retained_card = Arc::new(Card::Normal(CardFace {
        name: "Retained replay card".into(),
        ..CardFace::default()
    }));
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![
            move_chosen(battlefield.clone(), Zone::Graveyard, Zone::Battlefield),
            move_chosen(battlefield, Zone::Graveyard, Zone::Battlefield),
            move_chosen(graveyard, Zone::Exile, Zone::Graveyard),
        ],
        vec![first_card, retained_card],
    );
    let first_card = put_named_in_play(&mut state, payer, "First replay card");
    let retained_card = put_named_in_play(&mut state, payer, "Retained replay card");
    let prompt = announce(&mut state, source);
    let first_iou = prompt.outstanding[0].id;
    let second_iou = prompt.outstanding[1].id;
    let third_iou = prompt.outstanding[2].id;

    fulfill(
        &mut state,
        first_iou,
        FulfillmentWitness::Objects(vec![first_card]),
    );
    fulfill(
        &mut state,
        second_iou,
        FulfillmentWitness::Objects(vec![retained_card]),
    );
    let retained_in_graveyard = state.zones.graveyards[payer.index()]
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == "Retained replay card",
            Card::TwoFaced { front, .. } => front.name.as_ref() == "Retained replay card",
        })
        .unwrap();
    fulfill(
        &mut state,
        third_iou,
        FulfillmentWitness::Objects(vec![retained_in_graveyard]),
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(
            first_iou,
        )))
        .unwrap();

    assert!(state.zones.battlefield.contains(&first_card));
    assert_eq!(state.zones.exile.len(), 1);
    let prompt = match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt,
        other => panic!("rescind should return to payment, got {other:?}"),
    };
    assert_eq!(prompt.fulfilled, vec![second_iou, third_iou]);
}

#[test]
fn decline_selectively_reverses_independent_mana_actions() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![green_source("First source"), green_source("Second source")],
    );
    let first = put_named_in_play(&mut state, payer, "First source");
    let second = put_named_in_play(&mut state, payer, "Second source");
    assert_eq!(announce(&mut state, parent).mana_abilities.len(), 2);

    let first_action = submit_tap_mana_action(&mut state, first);
    let second_action = submit_tap_mana_action(&mut state, second);
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose legal mana reversals");
    };
    assert!(
        choice.legal.contains(&vec![first_action]),
        "legal sets: {:?}",
        choice.legal
    );
    assert!(
        choice.legal.contains(&vec![second_action]),
        "legal sets: {:?}",
        choice.legal
    );
    assert!(
        choice.legal.contains(&vec![first_action, second_action]),
        "legal sets: {:?}",
        choice.legal
    );

    state
        .submit_decision(Decision::ManaReversals(vec![first_action]))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(matches!(state.pending, Some(PendingDecision::Priority(_))));
    assert!(!state.objects.obj(first).tapped);
    assert!(state.objects.obj(second).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
}

#[test]
fn producer_cannot_reverse_while_a_retained_mana_action_spent_its_mana() {
    let filter_cost = Cost(
        vec![
            CostComponent::Mana("{1}".parse::<ManaCost>().unwrap()),
            CostComponent::Tap,
        ]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![
            green_source("Producer"),
            mana_source("Filter", filter_cost, Color::Green),
        ],
    );
    let producer = put_named_in_play(&mut state, payer, "Producer");
    let filter = put_named_in_play(&mut state, payer, "Filter");
    announce(&mut state, parent);
    let producer_action = submit_tap_mana_action(&mut state, producer);
    let produced_unit = state.player(payer).mana_pool.units()[0].id;

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: filter,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    let pip = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
        .unwrap()
        .id;
    let tap = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(produced_unit));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    let records = state.payment_records().unwrap();
    let producer_record = records
        .iter()
        .find(|record| {
            matches!(
                record.command,
                deckmaste_engine::ReplayCommand::ManaAbility { action, .. }
                    if action == producer_action
            )
        })
        .unwrap()
        .id;
    let filter_record = records.last().unwrap();
    let filter_action = match filter_record.command {
        deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
        deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
    }
    .unwrap();
    assert_eq!(filter_record.dependencies, vec![producer_record]);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose dependency-aware reversal sets");
    };
    assert!(!choice.legal.contains(&vec![producer_action]));
    assert!(choice.legal.contains(&vec![filter_action]));
    assert!(choice.legal.contains(&vec![producer_action, filter_action]));
}

#[test]
fn replay_rebinds_mana_after_an_unrelated_earlier_producer_is_omitted() {
    let filter_cost = Cost(
        vec![
            CostComponent::Mana("{1}".parse::<ManaCost>().unwrap()),
            CostComponent::Tap,
        ]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![
            green_source("Unrelated producer"),
            green_source("Required producer"),
            mana_source("Spender", filter_cost, Color::Green),
        ],
    );
    let unrelated = put_named_in_play(&mut state, payer, "Unrelated producer");
    let required = put_named_in_play(&mut state, payer, "Required producer");
    let spender = put_named_in_play(&mut state, payer, "Spender");
    announce(&mut state, parent);
    let unrelated_action = submit_tap_mana_action(&mut state, unrelated);
    let required_action = submit_tap_mana_action(&mut state, required);
    let required_mana = state.player(payer).mana_pool.units()[1].id;

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: spender,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    let pip = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
        .unwrap()
        .id;
    let tap = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(required_mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    let spender_action = state
        .payment_records()
        .unwrap()
        .last()
        .and_then(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .unwrap();

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose reversal sets");
    };
    assert!(choice.legal.contains(&vec![unrelated_action]));
    assert!(!choice.legal.contains(&vec![required_action]));
    assert!(choice.legal.contains(&vec![spender_action]));
}

#[test]
fn rescinding_every_fulfillment_returns_to_prepayment_with_mana_actions_intact() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![green_source("Prepayment source")],
    );
    let source = put_named_in_play(&mut state, payer, "Prepayment source");
    let prompt = announce(&mut state, parent);
    assert_eq!(prompt.stage, deckmaste_engine::PaymentStage::PrePayment);
    let action = submit_tap_mana_action(&mut state, source);
    let mana = state.player(payer).mana_pool.units()[0].id;
    let prompt = match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt.clone(),
        other => panic!("mana action should resume payment, got {other:?}"),
    };
    let pip = prompt.outstanding[0].id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);

    state
        .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(pip)))
        .unwrap();

    let PendingDecision::Payment(prompt) = state.pending.as_ref().unwrap() else {
        panic!("rescind should return to payment");
    };
    assert_eq!(prompt.stage, deckmaste_engine::PaymentStage::PrePayment);
    assert!(state.objects.obj(source).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
    assert!(
        state
            .payment_records()
            .unwrap()
            .iter()
            .any(|record| matches!(
                record.command,
                deckmaste_engine::ReplayCommand::ManaAbility { action: found, .. }
                    if found == action
            ))
    );
}

#[test]
fn nested_mana_action_is_an_independently_reversible_unit() {
    let outer_cost = Cost(
        vec![
            CostComponent::Mana("{1}".parse::<ManaCost>().unwrap()),
            CostComponent::Tap,
        ]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![
            mana_source("Outer filter", outer_cost, Color::Green),
            green_source("Nested producer"),
        ],
    );
    let outer = put_named_in_play(&mut state, payer, "Outer filter");
    let nested = put_named_in_play(&mut state, payer, "Nested producer");
    let initial = state.player_mut(payer).mana_pool.add(
        Color::Green.into(),
        1,
        deckmaste_engine::ManaProvenance::default(),
    )[0];
    announce(&mut state, parent);

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: outer,
            ability: 0,
        }))
        .unwrap();
    assert_eq!(
        run_to_payment(&mut state).stage,
        deckmaste_engine::PaymentStage::PrePayment
    );
    let nested_action = submit_tap_mana_action(&mut state, nested);
    let outer_prompt = match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt.clone(),
        other => panic!("nested action should resume outer payment, got {other:?}"),
    };
    let pip = outer_prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
        .unwrap()
        .id;
    let tap = outer_prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(initial));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    let outer_action = state
        .payment_records()
        .unwrap()
        .last()
        .and_then(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .unwrap();

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose nested and outer actions");
    };
    assert!(choice.legal.contains(&vec![nested_action]));
    assert!(choice.legal.contains(&vec![outer_action]));
    assert!(choice.legal.contains(&vec![nested_action, outer_action]));

    state
        .submit_decision(Decision::ManaReversals(vec![outer_action]))
        .unwrap();
    assert!(!state.objects.obj(outer).tapped);
    assert!(state.objects.obj(nested).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 2);
}

#[test]
fn declining_an_outer_mana_action_offers_reversal_of_its_completed_nested_action() {
    let outer_cost = Cost(
        vec![
            CostComponent::Mana("{1}".parse::<ManaCost>().unwrap()),
            CostComponent::Tap,
        ]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![
            mana_source("Declined outer", outer_cost, Color::Green),
            green_source("Nested action"),
        ],
    );
    let outer = put_named_in_play(&mut state, payer, "Declined outer");
    let nested = put_named_in_play(&mut state, payer, "Nested action");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: outer,
            ability: 0,
        }))
        .unwrap();
    assert_eq!(
        run_to_payment(&mut state).stage,
        deckmaste_engine::PaymentStage::PrePayment
    );
    let nested_action = submit_tap_mana_action(&mut state, nested);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("declining the outer action should expose its nested reversal");
    };
    assert!(choice.legal.contains(&Vec::new()));
    assert!(choice.legal.contains(&vec![nested_action]));

    state
        .submit_decision(Decision::ManaReversals(vec![nested_action]))
        .unwrap();
    assert_eq!(state.payment_depth(), 1);
    assert!(!state.objects.obj(outer).tapped);
    assert!(!state.objects.obj(nested).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    assert!(matches!(state.pending, Some(PendingDecision::Payment(_))));
}

#[test]
fn a_nested_producer_spent_by_a_barred_outer_cost_is_forced_to_remain() {
    let outer_cost = Cost(
        vec![
            CostComponent::Mana("{1}".parse::<ManaCost>().unwrap()),
            CostComponent::Act(Arc::new(CoreAction::Move(
                Reference::This,
                Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                Arc::from([]),
                Some(Zone::Battlefield),
            ))),
        ]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![
            mana_source("Barred outer", outer_cost, Color::Green),
            green_source("Nested payer"),
        ],
    );
    let outer = put_named_in_play(&mut state, payer, "Barred outer");
    let nested = put_named_in_play(&mut state, payer, "Nested payer");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: outer,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    assert_eq!(child.stage, deckmaste_engine::PaymentStage::PrePayment);
    let _nested_action = submit_tap_mana_action(&mut state, nested);
    let child = match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt.clone(),
        other => panic!("nested payer should resume outer payment, got {other:?}"),
    };
    let pip = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
        .unwrap()
        .id;
    let move_iou = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Act(_)))
        .unwrap()
        .id;
    let mana = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    fulfill(&mut state, move_iou, FulfillmentWitness::Bound);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 1);
    assert!(state.objects.obj(nested).tapped);
    assert_eq!(state.player(payer).mana_pool.units().len(), 1);
    assert!(!state.zones.battlefield.contains(&outer));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    let retained = state.payment_records().unwrap();
    assert_eq!(retained.len(), 2);
    assert!(matches!(
        retained[1].command,
        deckmaste_engine::ReplayCommand::ManaAbility {
            submitted: false,
            ..
        }
    ));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records.len() == 2
    ));
}

#[test]
fn decline_is_available_during_an_in_flight_replacement_choice() {
    let replacement = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::Sequentially(Arc::from([])),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Double replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![
            Ability::r#static(StaticEffect::Replacement(Arc::new(replacement.clone()))),
            Ability::r#static(StaticEffect::Replacement(Arc::new(replacement))),
        ],
        ..CardFace::default()
    }));
    let sacrifice_self =
        CostComponent::do_action(CoreAction::Sacrifice(Reference::You, Reference::This));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![sacrifice_self], vec![shield]);
    put_named_in_play(&mut state, payer, "Double replacement");
    let prompt = announce(&mut state, source);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseReplacement(_)) => break,
            other => panic!("expected replacement choice, got {other:?}"),
        }
    }

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&source));
    assert!(matches!(state.pending, Some(PendingDecision::Priority(_))));
}

#[test]
fn decline_records_an_observation_made_before_an_in_flight_choice() {
    let modal = OneShotEffect::Modal(Modal {
        choose: ChooseSpec {
            count: Quantity::one(),
            up_to: false,
            repeats: false,
            chooser: Reference::You,
            rider: None,
        },
        modes: vec![
            Mode {
                effect: OneShotEffect::Sequentially(Arc::from([])),
                cost: None,
            },
            Mode {
                effect: OneShotEffect::Sequentially(Arc::from([])),
                cost: None,
            },
        ]
        .into(),
    });
    let replacement = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(CoreAction::Reveal {
                    what: Reference::EventObject,
                    to: None,
                }),
                modal,
            ]
            .into(),
        ),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Observe then choose".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let sacrifice_self =
        CostComponent::do_action(CoreAction::Sacrifice(Reference::You, Reference::This));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![sacrifice_self], vec![shield]);
    put_named_in_play(&mut state, payer, "Observe then choose");
    let prompt = announce(&mut state, source);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseModes(_)) => break,
            other => panic!("expected modal choice, got {other:?}"),
        }
    }

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert!(state.zones.battlefield.contains(&source));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.crossed_observation_barrier
                && !incident.crossed_reversal_barrier
    ));
}

#[test]
fn submitted_mana_action_with_a_library_barrier_is_forced_to_remain() {
    let put_self_on_bottom = Cost(
        vec![CostComponent::Act(Arc::new(CoreAction::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            Arc::from([]),
            Some(Zone::Battlefield),
        )))]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![mana_source(
            "Barred source",
            put_self_on_bottom,
            Color::Green,
        )],
    );
    let source = put_named_in_play(&mut state, payer, "Barred source");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    fulfill(
        &mut state,
        child.outstanding[0].id,
        FulfillmentWitness::Bound,
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    let record = state.payment_records().unwrap().last().unwrap().clone();
    assert!(
        record
            .reversal_barriers
            .contains(&ReversalBarrier::MovedToLibrary)
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records == vec![record.id]
                && incident.crossed_reversal_barrier
    ));
}

#[test]
fn kci_under_wheel_is_retained_as_one_submitted_mana_action() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{1}".parse::<ManaCost>().unwrap())],
        vec![krark_clan_ironworks(), wheel_of_sun_and_moon()],
    );
    let kci = put_named_in_play(&mut state, payer, "Krark-Clan Ironworks");
    put_named_in_play(&mut state, payer, "Wheel of Sun and Moon");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: kci,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    fulfill(
        &mut state,
        child.outstanding[0].id,
        FulfillmentWitness::Objects(vec![kci]),
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    let record = state.payment_records().unwrap().last().unwrap().clone();
    assert!(
        record
            .reversal_barriers
            .contains(&ReversalBarrier::MovedToLibrary)
    );
    assert_eq!(state.player(payer).mana_pool.units().len(), 2);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert_eq!(state.player(payer).mana_pool.units().len(), 2);
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records == vec![record.id]
    ));
}

#[test]
fn mox_then_kci_can_pay_for_the_sacrificed_colored_legends_ability() {
    let payer = PlayerId(0);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![
                    colored_legendary_artifact(),
                    mox_amber_fixture(),
                    krark_clan_ironworks(),
                ],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 23,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let legend = put_named_in_play(&mut state, payer, "Colored legendary artifact");
    let mox = put_named_in_play(&mut state, payer, "Mox Amber");
    let kci = put_named_in_play(&mut state, payer, "Krark-Clan Ironworks");
    let action = Action::ActivateAbility {
        object: legend,
        ability: 0,
    };
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![action.clone()],
    }));
    state.submit_decision(Decision::Act(action)).unwrap();
    assert_eq!(
        run_to_payment(&mut state).stage,
        deckmaste_engine::PaymentStage::PrePayment
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: mox,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    fulfill(
        &mut state,
        child.outstanding[0].id,
        FulfillmentWitness::Bound,
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseObjects(_)) => {
                state
                    .submit_decision(Decision::Chosen(vec![legend]))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor(_)) => {
                state
                    .submit_decision(Decision::ManaColor(Color::Green.into()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Payment(_)) => break,
            other => panic!("expected Mox to resume parent payment, got {other:?}"),
        }
    }
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: kci,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    fulfill(
        &mut state,
        child.outstanding[0].id,
        FulfillmentWitness::Objects(vec![legend]),
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let prompt = run_to_payment(&mut state);
    assert!(!state.zones.battlefield.contains(&legend));
    assert_eq!(state.player(payer).mana_pool.units().len(), 3);

    let green = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .find(|unit| unit.kind == Color::Green.into())
        .unwrap()
        .id;
    let colorless: Vec<_> = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .filter(|unit| unit.kind == deckmaste_core::ColorOrColorless::Colorless)
        .map(|unit| unit.id)
        .collect();
    let mut generic = colorless.into_iter();
    let mut coverage = ManaCoverage::empty();
    for iou in &prompt.outstanding {
        let mana = match iou.kind {
            IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(Color::Green)) => green,
            IouKind::ManaPip(deckmaste_engine::ManaPip::Generic) => generic.next().unwrap(),
            ref other => panic!("unexpected IOU {other:?}"),
        };
        coverage.insert(iou.id, ManaPayment::Floating(mana));
    }
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    for iou in prompt.outstanding {
        fulfill(&mut state, iou.id, FulfillmentWitness::CoveredMana);
    }
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    assert_eq!(state.payment_depth(), 0);
}

#[test]
fn filter_mana_spent_in_a_child_reduces_omnath_before_rancher_produces() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana(
            "{5}{B}{G}".parse::<ManaCost>().unwrap(),
        )],
        vec![
            omnath_fixture(),
            mana_cylix_fixture(),
            bighorner_rancher_fixture(),
        ],
    );
    put_named_in_play(&mut state, payer, "Omnath, Locus of Mana");
    let cylix = put_named_in_play(&mut state, payer, "Mana Cylix");
    let rancher = put_named_in_play(&mut state, payer, "Bighorner Rancher");
    state.player_mut(payer).mana_pool.add(
        Color::Green.into(),
        3,
        deckmaste_engine::ManaProvenance::default(),
    );
    announce(&mut state, parent);

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: cylix,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    let pip = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
        .unwrap()
        .id;
    let tap = child
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let spent_green = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(spent_green));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor(_)) => {
                state
                    .submit_decision(Decision::ManaColor(Color::Black.into()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::Payment(_)) => break,
            other => panic!("expected Cylix to resume parent payment, got {other:?}"),
        }
    }
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 2);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Black.into()), 1);

    submit_tap_mana_action(&mut state, rancher);
    let prompt = match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt.clone(),
        other => panic!("Rancher should resume parent payment, got {other:?}"),
    };
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 4);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Black.into()), 1);

    let mut incomplete = ManaCoverage::empty();
    for (iou, mana) in prompt
        .outstanding
        .iter()
        .zip(state.player(payer).mana_pool.units())
    {
        incomplete.insert(iou.id, ManaPayment::Floating(mana.id));
    }
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(incomplete,)))
            .is_err()
    );
    assert_eq!(state.player(payer).mana_pool.units().len(), 5);
}

#[test]
fn declining_unsubmitted_mana_child_retains_only_its_barred_cost() {
    let put_self_on_bottom = Cost(
        vec![CostComponent::Act(Arc::new(CoreAction::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            Arc::from([]),
            Some(Zone::Battlefield),
        )))]
        .into(),
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![mana_source(
            "Unsubmitted barred source",
            put_self_on_bottom,
            Color::Green,
        )],
    );
    let source = put_named_in_play(&mut state, payer, "Unsubmitted barred source");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    fulfill(
        &mut state,
        child.outstanding[0].id,
        FulfillmentWitness::Bound,
    );
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 1);
    assert!(matches!(state.pending, Some(PendingDecision::Payment(_))));
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    let retained = state.payment_records().unwrap().last().unwrap();
    assert!(matches!(
        retained.command,
        deckmaste_engine::ReplayCommand::ManaAbility {
            submitted: false,
            ..
        }
    ));
    assert!(
        retained
            .reversal_barriers
            .contains(&ReversalBarrier::MovedToLibrary)
    );
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records == vec![retained.id]
                && incident.crossed_reversal_barrier
    ));

    let retained_id = retained.id;
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    assert_eq!(state.payment_depth(), 0);
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    assert!(matches!(
        state.incidents(),
        [_, EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records == vec![retained_id]
                && incident.crossed_reversal_barrier
    ));
}
