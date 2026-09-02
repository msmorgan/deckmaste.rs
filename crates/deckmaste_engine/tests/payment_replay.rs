use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Anchor;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::ChooseSpec;
use deckmaste_core::Color;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Destination;
use deckmaste_core::EventFilter;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaAbility;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::May;
use deckmaste_core::Modal;
use deckmaste_core::Mode;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Predicate;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Replacement;
use deckmaste_core::StatValue;
use deckmaste_core::StatePredicate;
use deckmaste_core::StaticEffect;
use deckmaste_core::Supertype;
use deckmaste_core::Token;
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
            targets: [].into(),
            cost: Cost(cost.into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: OneShotEffect::Sequentially(Arc::from([])).into(),
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
    mana_source_for(name, cost, color, Reference::Reg(deckmaste_core::RefId(1)))
}

fn mana_source_for(name: &str, cost: Cost, color: Color, recipient: Reference) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: name.into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                cost,
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    recipient,
                    Count::Literal(1),
                    ManaSpec::Specific(color.into()).into(),
                ))
                .into(),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn suspending_barred_mana_source() -> Arc<Card> {
    let choose = OneShotEffect::Modal(Modal {
        choose: ChooseSpec {
            count: Quantity::one(),
            up_to: false,
            repeats: false,
            chooser: Reference::Reg(deckmaste_core::RefId(1)),
            rider: None,
        },
        modes: vec![
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
        ]
        .into(),
    });
    Arc::new(Card::Normal(CardFace {
        name: "Suspending barred mana source".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                cost: Cost(Arc::from([])),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Sequentially(
                    vec![
                        OneShotEffect::Act(CoreAction::Move(
                            Reference::Reg(deckmaste_core::RefId(0)),
                            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                            Arc::from([]),
                            Some(Zone::Battlefield),
                        )),
                        choose,
                        OneShotEffect::Act(CoreAction::AddMana(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            Count::Literal(1),
                            ManaSpec::Specific(Color::Green.into()).into(),
                        )),
                    ]
                    .into(),
                )
                .into(),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn token_creating_barred_mana_source() -> Arc<Card> {
    let token = Token {
        name: None,
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: Arc::from([]),
        power: None,
        toughness: None,
    };
    Arc::new(Card::Normal(CardFace {
        name: "Token-creating barred source".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                cost: Cost(
                    vec![CostComponent::do_action(CoreAction::Move(
                        Reference::Reg(deckmaste_core::RefId(0)),
                        Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                        Arc::from([]),
                        Some(Zone::Battlefield),
                    ))]
                    .into(),
                ),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Sequentially(
                    vec![
                        OneShotEffect::Act(CoreAction::Create {
                            agent: Reference::Reg(deckmaste_core::RefId(1)),
                            count: Count::Literal(1),
                            token: token.into(),
                            riders: Arc::from([]),
                        }),
                        OneShotEffect::Act(CoreAction::AddMana(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            Count::Literal(1),
                            ManaSpec::Specific(Color::Green.into()).into(),
                        )),
                    ]
                    .into(),
                )
                .into(),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn fulfillment_created_mana_source_replacements() -> (Arc<Card>, Arc<Card>) {
    let token_ability = Ability::Mana(ManaAbility::Activated {
        ability: Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(vec![CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: OneShotEffect::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            ))
            .into(),
        }),
        profile: deckmaste_core::ActivatedManaProfile::Always,
    });
    let token = Token {
        name: Some("Fulfillment-created helper".into()),
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: vec![token_ability].into(),
        power: None,
        toughness: None,
    };
    let creator_replacement = Replacement::Instead {
        would: EventFilter::LifeLost {
            who: Predicate::Any,
            amount: None,
        },
        instead: OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(CoreAction::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: token.into(),
                    riders: Arc::from([]),
                }),
                OneShotEffect::Act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
            ]
            .into(),
        ),
    };
    let creator = Arc::new(Card::Normal(CardFace {
        name: "Fulfillment source creator".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            creator_replacement,
        )))],
        ..CardFace::default()
    }));
    let optional_replacement = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(OneShotEffect::Act(CoreAction::Pay(Cost(
                vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
            )))),
            if_did: None,
            if_not: None,
        }),
    };
    let optional = Arc::new(Card::Normal(CardFace {
        name: "Fulfillment source consumer".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            optional_replacement,
        )))],
        ..CardFace::default()
    }));
    (creator, optional)
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
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
                    what: Reference::Reg(deckmaste_core::RefId(2)),
                    to: None,
                }),
                OneShotEffect::Act(CoreAction::Move(
                    Reference::Reg(deckmaste_core::RefId(2)),
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

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn mox_amber_fixture() -> Arc<Card> {
    let colored_legend = Predicate::And(
        vec![
            Predicate::Characteristic(CharacteristicPredicate::Supertype(Supertype::Legendary)),
            Predicate::Or(
                vec![
                    Predicate::r#type(Type::Creature),
                    Predicate::r#type(Type::Planeswalker),
                ]
                .into(),
            ),
            Predicate::Not(Arc::new(Predicate::Characteristic(
                CharacteristicPredicate::Colorless,
            ))),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::Reg(deckmaste_core::RefId(1)),
            )))),
        ]
        .into(),
    );
    let sole_eligible_legend = Reference::Single(Arc::new(deckmaste_core::Selection::SelectAll(
        Arc::new(deckmaste_core::Region::candidate(colored_legend)),
    )));
    let effect = OneShotEffect::Act(CoreAction::AddMana(
        Reference::Reg(deckmaste_core::RefId(1)),
        Count::Literal(1),
        ManaSpec::AmongColorsOf(sole_eligible_legend).into(),
    ));
    Arc::new(Card::Normal(CardFace {
        name: "Mox Amber".into(),
        mana_cost: "{0}".parse().unwrap(),
        supertypes: vec![Supertype::Legendary],
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                cost: Cost(vec![CostComponent::Tap].into()),
                from: None,
                window: None,
                condition: None,
                limits: Arc::from([]),
                effect: effect.into(),
            }),
            profile: deckmaste_core::ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn colored_legendary_artifact() -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Colored legendary artifact".into(),
        mana_cost: "{G}".parse().unwrap(),
        supertypes: vec![Supertype::Legendary],
        types: vec![Type::Artifact.def(), Type::Creature.def()],
        abilities: vec![Ability::activated(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(vec![CostComponent::Mana("{2}{G}".parse::<ManaCost>().unwrap())].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: OneShotEffect::Sequentially(Arc::from([])).into(),
        })],
        ..CardFace::default()
    }))
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn omnath_fixture() -> Arc<Card> {
    let green = Count::ManaAvailableKind(
        Reference::Reg(deckmaste_core::RefId(1)),
        Color::Green.into(),
    );
    Arc::new(Card::Normal(CardFace {
        name: "Omnath, Locus of Mana".into(),
        mana_cost: "{2}{G}".parse().unwrap(),
        supertypes: vec![Supertype::Legendary],
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(1)),
        toughness: Some(StatValue::Number(1)),
        abilities: vec![Ability::r#static(StaticEffect::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(green.clone())),
                    Modification::Toughness(NumericOp::Up(green)),
                ]
                .into(),
            ),
        ))],
        ..CardFace::default()
    }))
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn mana_cylix_fixture() -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Mana Cylix".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
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
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::AnyColor.into(),
                ))
                .into(),
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
fn mana_dependencies_qualify_pool_local_ids_by_player() {
    let opponent_source = mana_source_for(
        "Opponent producer",
        Cost(vec![CostComponent::Tap].into()),
        Color::Green,
        Reference::OpponentOf(std::sync::Arc::new(Reference::Reg(deckmaste_core::RefId(
            1,
        )))),
    );
    let self_source = green_source("Self producer");
    let costed_source = mana_source(
        "Costed producer",
        Cost(vec![CostComponent::Mana("{1}".parse::<ManaCost>().unwrap())].into()),
        Color::Green,
    );
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![opponent_source, self_source, costed_source],
    );
    let opponent = put_named_in_play(&mut state, payer, "Opponent producer");
    let own = put_named_in_play(&mut state, payer, "Self producer");
    let costed = put_named_in_play(&mut state, payer, "Costed producer");
    announce(&mut state, parent);
    submit_tap_mana_action(&mut state, opponent);
    let opponent_record = state.payment_records().unwrap().last().unwrap().id;
    submit_tap_mana_action(&mut state, own);
    let own_record = state.payment_records().unwrap().last().unwrap().id;
    assert_eq!(state.player(PlayerId(1)).mana_pool.units()[0].id.0, 0);
    assert_eq!(state.player(payer).mana_pool.units()[0].id.0, 0);

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: costed,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    let pip = child.outstanding[0].id;
    let mana = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    let record = state.payment_records().unwrap().last().unwrap();
    assert_eq!(record.dependencies, vec![own_record]);
    assert_ne!(record.dependencies, vec![opponent_record]);
}

#[test]
fn rescind_replays_unrelated_later_fulfillment() {
    let pay_life = CostComponent::do_action(CoreAction::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
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
    let put_on_bottom = CostComponent::do_action(CoreAction::Move(
        Reference::Reg(deckmaste_core::RefId(0)),
        Destination::Library(Anchor::FromBottom(Count::Literal(0))),
        Arc::from([]),
        Some(Zone::Battlefield),
    ));
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
    let put_on_bottom = CostComponent::do_action(CoreAction::Move(
        Reference::Reg(deckmaste_core::RefId(0)),
        Destination::Library(Anchor::FromBottom(Count::Literal(0))),
        Arc::from([]),
        Some(Zone::Battlefield),
    ));
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
    // The activation reverses only as far as the library barrier permits,
    // then returns priority so the player may take another action
    // ([CR#733.1,733.2]).
    assert!(matches!(state.pending, Some(PendingDecision::Priority(_))));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records.len() == 1
                && incident.crossed_reversal_barrier
    ));
}

#[test]
fn replay_restores_ordinary_triggers_caused_by_a_retained_fulfillment() {
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Life-loss watcher".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            targets: [].into(),
            event: EventFilter::LifeLost {
                who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                amount: None,
            },
            from: None,
            condition: None,
            limits: Arc::from([]),
            effect: OneShotEffect::Sequentially(Arc::from([])).into(),
        })],
        ..CardFace::default()
    }));
    let pay_life = CostComponent::do_action(CoreAction::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
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
            CostComponent::do_action(CoreAction::Move(
                Reference::Reg(deckmaste_core::RefId(0)),
                Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                Arc::from([]),
                Some(Zone::Battlefield),
            )),
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
        .find(|iou| matches!(iou.kind, IouKind::Act { .. }))
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
    let sacrifice_self = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
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
fn rescind_replays_an_optional_payment_nested_inside_a_fulfillment() {
    let replacement = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(OneShotEffect::Act(CoreAction::Pay(Cost(
                vec![CostComponent::do_action(CoreAction::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(1)),
                ))]
                .into(),
            )))),
            if_did: None,
            if_not: None,
        }),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Optional replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let sacrifice = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![CostComponent::Tap, sacrifice], vec![shield]);
    put_named_in_play(&mut state, payer, "Optional replacement");
    let prompt = announce(&mut state, source);
    let tap = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let sacrifice = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Act { .. }))
        .unwrap()
        .id;
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: sacrifice,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    let optional = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 2);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: optional.outstanding[0].id,
            witness: FulfillmentWitness::PayLife,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let ready = run_to_payment(&mut state);
    assert_eq!(
        state.payment_depth(),
        1,
        "unexpected nested prompt: {ready:?}"
    );
    assert_eq!(ready.stage, deckmaste_engine::PaymentStage::Ready);
    assert_eq!(state.player(payer).life, 19);
    assert!(state.zones.battlefield.contains(&source));

    state
        .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(tap)))
        .unwrap();

    let PendingDecision::Payment(rebuilt) = state.pending.as_ref().unwrap() else {
        panic!("rescind should resume the reconstructed parent payment");
    };
    assert_eq!(rebuilt.fulfilled, vec![sacrifice]);
    assert_eq!(state.player(payer).life, 19);
    assert!(state.zones.battlefield.contains(&source));
}

#[test]
fn fulfillment_owned_mana_child_remains_a_separate_reversal_unit() {
    let replacement = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(OneShotEffect::Act(CoreAction::Pay(Cost(
                vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
            )))),
            if_did: None,
            if_not: None,
        }),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Optional mana replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let helper_card = green_source("Fulfillment-owned helper");
    let sacrifice = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![sacrifice], vec![shield, helper_card]);
    put_named_in_play(&mut state, payer, "Optional mana replacement");
    let helper = put_named_in_play(&mut state, payer, "Fulfillment-owned helper");

    let prompt = announce(&mut state, source);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    let optional = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 2);
    assert!(optional.mana_abilities.contains(&(helper, 0)));
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
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
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let ready = run_to_payment(&mut state);
    assert_eq!(ready.stage, deckmaste_engine::PaymentStage::Ready);

    let [fulfillment] = state.payment_records().unwrap() else {
        panic!("the completed fulfillment should own one nested mana child");
    };
    let child_action = fulfillment
        .children
        .iter()
        .find_map(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the nested mana action remains recorded under the fulfillment");

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose the fulfillment-owned mana action");
    };
    assert!(choice.legal.contains(&Vec::new()));
    assert!(choice.legal.contains(&vec![child_action]));
    let mut reversed_state = state.clone();
    state
        .submit_decision(Decision::ManaReversals(Vec::new()))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&source));
    assert!(state.objects.obj(helper).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);

    reversed_state
        .submit_decision(Decision::ManaReversals(vec![child_action]))
        .unwrap();
    assert_eq!(reversed_state.payment_depth(), 0);
    assert!(reversed_state.zones.battlefield.contains(&source));
    assert!(!reversed_state.objects.obj(helper).tapped);
    assert_eq!(
        reversed_state
            .player(payer)
            .mana_pool
            .amount(Color::Green.into()),
        0,
    );
}

#[test]
fn fulfillment_owned_mana_child_can_reverse_under_a_retained_parent() {
    let replacement = Replacement::Instead {
        would: EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        },
        instead: OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
                OneShotEffect::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(OneShotEffect::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: None,
                    if_not: None,
                }),
            ]
            .into(),
        ),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Barred optional mana replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let helper_card = green_source("Barred fulfillment helper");
    let sacrifice = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![sacrifice], vec![shield, helper_card]);
    put_named_in_play(&mut state, payer, "Barred optional mana replacement");
    let helper = put_named_in_play(&mut state, payer, "Barred fulfillment helper");

    let prompt = announce(&mut state, source);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
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
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    run_to_payment(&mut state);

    let [fulfillment] = state.payment_records().unwrap() else {
        panic!("the retained fulfillment should own one nested mana child");
    };
    let fulfillment_id = fulfillment.id;
    assert!(
        fulfillment
            .reversal_barriers
            .contains(&ReversalBarrier::ShuffledLibrary)
    );
    let child_action = fulfillment
        .children
        .iter()
        .find_map(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the fulfillment retains its nested mana child");

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("the retained fulfillment should still expose its child reversal");
    };
    assert!(choice.legal.contains(&vec![child_action]));
    state
        .submit_decision(Decision::ManaReversals(vec![child_action]))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&source));
    assert!(!state.objects.obj(helper).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.crossed_reversal_barrier
                && incident.forced_retained_records == vec![fulfillment_id]
    ));
}

#[test]
fn omitted_fulfillment_mana_child_replays_after_earlier_created_source() {
    let (creator, optional) = fulfillment_created_mana_source_replacements();
    let pay_life = CostComponent::do_action(CoreAction::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
        LifeOp::Down(Count::Literal(1)),
    ));
    let sacrifice = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![pay_life, sacrifice], vec![creator, optional]);
    put_named_in_play(&mut state, payer, "Fulfillment source creator");
    put_named_in_play(&mut state, payer, "Fulfillment source consumer");

    let prompt = announce(&mut state, source);
    let life = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::PayLife(1)))
        .unwrap()
        .id;
    let sacrifice = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Act { .. }))
        .unwrap()
        .id;
    fulfill(&mut state, life, FulfillmentWitness::PayLife);
    let helper = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| {
            matches!(
                state.def(object),
                Card::Normal(face) if face.name.as_ref() == "Fulfillment-created helper"
            )
        })
        .expect("the retained fulfillment creates the later mana source");
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: sacrifice,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
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
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    run_to_payment(&mut state);

    let [creator_record, consumer_record] = state.payment_records().unwrap() else {
        panic!("the two fulfillments should remain distinct root records");
    };
    assert!(
        creator_record
            .reversal_barriers
            .contains(&ReversalBarrier::ShuffledLibrary)
    );
    let child_action = consumer_record
        .children
        .iter()
        .find_map(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the omitted fulfillment owns its nested mana action");

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose the fulfillment-owned mana action");
    };
    assert!(choice.legal.contains(&Vec::new()));
    assert!(choice.legal.contains(&vec![child_action]));
    let mut reversed_state = state.clone();
    state
        .submit_decision(Decision::ManaReversals(Vec::new()))
        .unwrap();
    reversed_state
        .submit_decision(Decision::ManaReversals(vec![child_action]))
        .unwrap();

    let retained_helper = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| {
            matches!(
                state.def(object),
                Card::Normal(face) if face.name.as_ref() == "Fulfillment-created helper"
            )
        })
        .expect("replay remints the retained child's source");
    assert!(state.objects.obj(retained_helper).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);

    let reversed_helper = reversed_state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| {
            matches!(
                reversed_state.def(object),
                Card::Normal(face) if face.name.as_ref() == "Fulfillment-created helper"
            )
        })
        .expect("reversing only the child retains its source creator");
    assert!(!reversed_state.objects.obj(reversed_helper).tapped);
    assert_eq!(
        reversed_state
            .player(payer)
            .mana_pool
            .amount(Color::Green.into()),
        0,
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the suspended fulfillment regression exercises two causal mana children and both reversal outcomes"
)]
fn suspended_fulfillment_keeps_dependent_mana_children_separately_reversible() {
    let modal = OneShotEffect::Modal(Modal {
        choose: ChooseSpec {
            count: Quantity::one(),
            up_to: false,
            repeats: false,
            chooser: Reference::Reg(deckmaste_core::RefId(1)),
            rider: None,
        },
        modes: vec![
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
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
                OneShotEffect::Act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
                OneShotEffect::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(OneShotEffect::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: None,
                    if_not: Some(Arc::new(modal)),
                }),
            ]
            .into(),
        ),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Suspended optional mana replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let helper_card = green_source("Suspended fulfillment helper");
    let filter_card = mana_source(
        "Suspended fulfillment filter",
        Cost(
            vec![
                CostComponent::Mana("{G}".parse().unwrap()),
                CostComponent::Tap,
            ]
            .into(),
        ),
        Color::Black,
    );
    let sacrifice = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![sacrifice], vec![shield, helper_card, filter_card]);
    put_named_in_play(&mut state, payer, "Suspended optional mana replacement");
    let helper = put_named_in_play(&mut state, payer, "Suspended fulfillment helper");
    let filter = put_named_in_play(&mut state, payer, "Suspended fulfillment filter");

    let prompt = announce(&mut state, source);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
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

    let helper_mana = state.player(payer).mana_pool.units()[0].id;
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: filter,
            ability: 0,
        }))
        .unwrap();
    let filter_payment = run_to_payment(&mut state);
    let pip = filter_payment
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
        .unwrap()
        .id;
    let tap = filter_payment
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(pip, ManaPayment::Floating(helper_mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseModes(_)) => break,
            other => panic!("the fulfillment should suspend after Optional decline, got {other:?}"),
        }
    }
    let helper_action = deckmaste_engine::ManaActionId(0);
    let filter_action = deckmaste_engine::ManaActionId(1);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let PendingDecision::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("suspended decline should expose the nested child reversal");
    };
    assert!(choice.legal.contains(&Vec::new()));
    assert!(!choice.legal.contains(&vec![helper_action]));
    assert!(choice.legal.contains(&vec![filter_action]));
    assert!(choice.legal.contains(&vec![helper_action, filter_action]));
    let mut filter_reversed = state.clone();
    let mut both_reversed = state.clone();
    state
        .submit_decision(Decision::ManaReversals(Vec::new()))
        .unwrap();
    filter_reversed
        .submit_decision(Decision::ManaReversals(vec![filter_action]))
        .unwrap();
    both_reversed
        .submit_decision(Decision::ManaReversals(vec![helper_action, filter_action]))
        .unwrap();

    assert!(state.objects.obj(helper).tapped);
    assert!(state.objects.obj(filter).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Black.into()), 1);

    assert!(filter_reversed.objects.obj(helper).tapped);
    assert!(!filter_reversed.objects.obj(filter).tapped);
    assert_eq!(
        filter_reversed
            .player(payer)
            .mana_pool
            .amount(Color::Green.into()),
        1
    );

    assert!(!both_reversed.objects.obj(helper).tapped);
    assert!(!both_reversed.objects.obj(filter).tapped);
    assert_eq!(
        both_reversed
            .player(payer)
            .mana_pool
            .amount(Color::Green.into()),
        0
    );
    assert_eq!(
        both_reversed
            .player(payer)
            .mana_pool
            .amount(Color::Black.into()),
        0
    );
}

#[test]
fn decline_retains_a_library_move_from_an_in_flight_fulfillment() {
    let modal = OneShotEffect::Modal(Modal {
        choose: ChooseSpec {
            count: Quantity::one(),
            up_to: false,
            repeats: false,
            chooser: Reference::Reg(deckmaste_core::RefId(1)),
            rider: None,
        },
        modes: vec![
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
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
                OneShotEffect::Act(CoreAction::Move(
                    Reference::Reg(deckmaste_core::RefId(2)),
                    Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                    Arc::from([]),
                    Some(Zone::Battlefield),
                )),
                modal,
            ]
            .into(),
        ),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Barred replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let sacrifice_self = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![sacrifice_self], vec![shield]);
    put_named_in_play(&mut state, payer, "Barred replacement");
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
            other => panic!("expected the cost's modal choice, got {other:?}"),
        }
    }
    assert!(!state.zones.battlefield.contains(&source));

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.crossed_reversal_barrier
                && incident.forced_retained_records.len() == 1
    ));
}

#[test]
fn decline_replays_a_retained_fulfillment_that_creates_then_remints_a_token() {
    let token = Token {
        name: Some("Transient replay token".into()),
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: Arc::from([]),
        power: None,
        toughness: None,
    };
    let token_on_battlefield = Predicate::And(
        vec![
            Predicate::Characteristic(CharacteristicPredicate::Named(
                "Transient replay token".into(),
            )),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
        ]
        .into(),
    );
    let replacement = Replacement::Instead {
        would: EventFilter::LifeLost {
            who: Predicate::Any,
            amount: None,
        },
        instead: OneShotEffect::Sequentially(
            vec![
                OneShotEffect::Act(CoreAction::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: token.into(),
                    riders: Arc::from([]),
                }),
                OneShotEffect::Act(CoreAction::Move(
                    Reference::Single(
                        deckmaste_core::Selection::SelectAll(Arc::new(
                            deckmaste_core::Region::candidate(token_on_battlefield.clone()),
                        ))
                        .into(),
                    ),
                    Destination::Zone(Zone::Battlefield),
                    Arc::from([]),
                    Some(Zone::Battlefield),
                )),
                OneShotEffect::Act(CoreAction::Move(
                    Reference::Single(
                        deckmaste_core::Selection::SelectAll(Arc::new(
                            deckmaste_core::Region::candidate(token_on_battlefield),
                        ))
                        .into(),
                    ),
                    Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                    Arc::from([]),
                    Some(Zone::Battlefield),
                )),
            ]
            .into(),
        ),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Create-remint replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            replacement,
        )))],
        ..CardFace::default()
    }));
    let pay_life = CostComponent::do_action(CoreAction::ChangeLife(
        Reference::Reg(deckmaste_core::RefId(1)),
        LifeOp::Down(Count::Literal(1)),
    ));
    let (mut state, payer, source) = activation_fixture_with_extras(vec![pay_life], vec![shield]);
    put_named_in_play(&mut state, payer, "Create-remint replacement");
    let prompt = announce(&mut state, source);
    fulfill(
        &mut state,
        prompt.outstanding[0].id,
        FulfillmentWitness::PayLife,
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&source));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)] if incident.crossed_reversal_barrier
    ));
}

#[test]
fn decline_preserves_a_public_reveal_without_crossing_an_observation_barrier() {
    let modal = OneShotEffect::Modal(Modal {
        choose: ChooseSpec {
            count: Quantity::one(),
            up_to: false,
            repeats: false,
            chooser: Reference::Reg(deckmaste_core::RefId(1)),
            rider: None,
        },
        modes: vec![
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: OneShotEffect::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
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
                    what: Reference::Reg(deckmaste_core::RefId(2)),
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
    let sacrifice_self = CostComponent::do_action(CoreAction::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ));
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
    assert!(state.look_grants.contains(&(payer, source)));
    assert!(state.look_grants.contains(&(PlayerId(1), source)));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if !incident.crossed_observation_barrier
                && !incident.crossed_reversal_barrier
    ));
}

#[test]
fn submitted_mana_action_with_a_library_barrier_is_forced_to_remain() {
    let put_self_on_bottom = Cost(
        vec![CostComponent::do_action(CoreAction::Move(
            Reference::Reg(deckmaste_core::RefId(0)),
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            Arc::from([]),
            Some(Zone::Battlefield),
        ))]
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
fn retained_mana_action_rebinds_objects_it_created_during_replay() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![token_creating_barred_mana_source()],
    );
    let source = put_named_in_play(&mut state, payer, "Token-creating barred source");
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
    let record = state.payment_records().unwrap().last().unwrap();
    assert!(
        record
            .object_inputs
            .values()
            .any(|logical| matches!(logical, deckmaste_engine::LogicalObject::Created { .. }))
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
    assert_eq!(state.zones.battlefield.len(), 2);
    assert!(state.zones.battlefield.contains(&parent));
}

#[test]
fn submitted_mana_child_stays_isolated_until_its_effect_and_record_finish() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![green_source("Child lifetime source")],
    );
    let source = put_named_in_play(&mut state, payer, "Child lifetime source");
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

    assert_eq!(
        state.payment_depth(),
        2,
        "the submitted child remains the active isolated image until stackless resolution ends"
    );
    assert!(state.payment_records().unwrap().is_empty());

    run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 1);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
    assert!(matches!(
        state
            .payment_records()
            .unwrap()
            .last()
            .map(|record| &record.command),
        Some(deckmaste_engine::ReplayCommand::ManaAbility {
            submitted: true,
            ..
        })
    ));
}

#[test]
fn declining_a_suspended_submitted_child_retains_its_in_flight_library_barrier() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![suspending_barred_mana_source()],
    );
    let source = put_named_in_play(&mut state, payer, "Suspending barred mana source");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    assert_eq!(child.stage, deckmaste_engine::PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseModes(_)) => break,
            other => panic!("expected the submitted mana effect to suspend, got {other:?}"),
        }
    }
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 1);
    assert!(!state.zones.battlefield.contains(&source));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    let retained = state.payment_records().unwrap().last().unwrap();
    assert!(matches!(
        retained.command,
        deckmaste_engine::ReplayCommand::ManaAbility {
            submitted: true,
            ..
        }
    ));
    assert!(
        retained
            .reversal_barriers
            .contains(&ReversalBarrier::MovedToLibrary)
    );
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn completed_mana_action(state: &GameState) -> deckmaste_engine::ManaActionId {
    state
        .payment_records()
        .unwrap()
        .last()
        .and_then(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the completed mana action is recorded on the parent")
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn submit_mox_action(
    state: &mut GameState,
    source: deckmaste_engine::ObjectId,
    color: Option<Color>,
) -> deckmaste_engine::ManaActionId {
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(state);
    assert_eq!(child.outstanding.len(), 1);
    assert!(matches!(child.outstanding[0].kind, IouKind::Tap));
    fulfill(state, child.outstanding[0].id, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor(choice)) => {
                let selected = color.expect("a colorless Mox state must not open a choice");
                assert!(choice.options.contains(&selected.into()));
                state
                    .submit_decision(Decision::ManaColor(selected.into()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(PendingDecision::ChooseObjects(_)) => {
                panic!("Mox chooses a color, never a legendary permanent")
            }
            StepOutcome::NeedsDecision(PendingDecision::Payment(_)) => break,
            other => panic!("expected Mox to resume parent payment, got {other:?}"),
        }
    }
    completed_mana_action(state)
}

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring replay cases"
)]
fn submit_kci_action(
    state: &mut GameState,
    source: deckmaste_engine::ObjectId,
    artifact: deckmaste_engine::ObjectId,
) -> deckmaste_engine::ManaActionId {
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(state);
    fulfill(
        state,
        child.outstanding[0].id,
        FulfillmentWitness::Objects(vec![artifact]),
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(state);
    completed_mana_action(state)
}

#[test]
fn declining_unsubmitted_mana_child_retains_only_its_barred_cost() {
    let put_self_on_bottom = Cost(
        vec![CostComponent::do_action(CoreAction::Move(
            Reference::Reg(deckmaste_core::RefId(0)),
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            Arc::from([]),
            Some(Zone::Battlefield),
        ))]
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
