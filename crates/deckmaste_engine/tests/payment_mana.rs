use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::ActivatedManaProfile;
use deckmaste_core::Binder;
use deckmaste_core::ChooseSpec;
use deckmaste_core::Color;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::EventFilter;
use deckmaste_core::ManaAbility;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaModeClass;
use deckmaste_core::ManaSpec;
use deckmaste_core::Modal;
use deckmaste_core::Mode;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Predicate;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Sort;
use deckmaste_core::StatePredicate;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::FulfillmentWitness;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentPrompt;
use deckmaste_engine::PaymentStage;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::PriorityRound;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;

fn activated_ability(cost: Cost, effect: OneShotEffect) -> ActivatedAbility {
    ActivatedAbility {
        ability_word: None,
        cost,
        from: None,
        window: None,
        condition: None,
        limits: Arc::from([]),
        effect,
    }
}

fn payment_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    payment_fixture_with_source_cost(Cost(vec![CostComponent::Tap].into()))
}

fn payment_fixture_with_source_cost(
    source_cost: Cost,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    payment_fixture_with_source_ability(Ability::Mana(ManaAbility::Activated {
        ability: Arc::new(activated_ability(
            source_cost,
            OneShotEffect::Act(CoreAction::AddMana(
                Reference::You,
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
        )),
        profile: ActivatedManaProfile::Always,
    }))
}

fn payment_fixture_with_source_ability(
    source_ability: Ability,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let payer = PlayerId(0);
    let parent = Arc::new(Card::Normal(CardFace {
        name: "Parent ability".into(),
        abilities: vec![Ability::activated(activated_ability(
            Cost(vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())].into()),
            OneShotEffect::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let mana_source = Arc::new(Card::Normal(CardFace {
        name: "Green source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![source_ability],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![parent, mana_source],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 11,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let parent = put_in_play(&mut state, payer, "Parent ability");
    let mana_source = put_in_play(&mut state, payer, "Green source");
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, mana_source)
}

fn modal_payment_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let effect = OneShotEffect::Modal(Modal {
        choose: ChooseSpec {
            count: Quantity::one(),
            up_to: false,
            repeats: false,
            chooser: Reference::You,
            rider: None,
        },
        modes: vec![
            Mode {
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
                cost: None,
            },
            Mode {
                effect: OneShotEffect::Sequentially(Arc::from([])),
                cost: None,
            },
        ]
        .into(),
    });
    let classes = vec![
        ManaModeClass {
            adds_mana: true,
            targetless: true,
            library_safe: true,
        },
        ManaModeClass {
            adds_mana: false,
            targetless: true,
            library_safe: true,
        },
    ];
    payment_fixture_with_source_ability(Ability::Mana(ManaAbility::Activated {
        ability: Arc::new(activated_ability(
            Cost(vec![CostComponent::Tap].into()),
            effect,
        )),
        profile: ActivatedManaProfile::ByAnnouncedMode(classes.into()),
    }))
}

fn put_in_play(state: &mut GameState, player: PlayerId, name: &str) -> deckmaste_engine::ObjectId {
    let object = state.zones.hands[player.index()]
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == name,
            Card::TwoFaced { front, .. } => front.name.as_ref() == name,
        })
        .unwrap();
    state.zones.hands[player.index()].retain(|&candidate| candidate != object);
    state.objects.obj_mut(object).zone = Some(Zone::Battlefield);
    state.objects.obj_mut(object).summoning_sick = false;
    state.zones.battlefield.push(object);
    object
}

fn kci_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let payer = PlayerId(0);
    let parent = Arc::new(Card::Normal(CardFace {
        name: "Artifact parent".into(),
        types: vec![Type::Artifact.def(), Type::Creature.def()],
        abilities: vec![Ability::activated(activated_ability(
            Cost(vec![CostComponent::Mana("{0}".parse::<ManaCost>().unwrap())].into()),
            OneShotEffect::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let artifact = Predicate::And(
        vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::r#type(Type::Artifact),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    let sacrifice = CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::ChooseOne {
            filter: artifact,
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
    let kci = Arc::new(Card::Normal(CardFace {
        name: "KCI".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(activated_ability(
                Cost(vec![sacrifice].into()),
                OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(2),
                    ManaSpec::Specific(deckmaste_core::ColorOrColorless::Colorless).into(),
                )),
            )),
            profile: ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![parent, kci],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 13,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let parent = put_in_play(&mut state, payer, "Artifact parent");
    let kci = put_in_play(&mut state, payer, "KCI");
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, kci)
}

fn triggered_mana_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let payer = PlayerId(0);
    let parent = Arc::new(Card::Normal(CardFace {
        name: "Triggered parent".into(),
        abilities: vec![Ability::activated(activated_ability(
            Cost(vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())].into()),
            OneShotEffect::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let source = Arc::new(Card::Normal(CardFace {
        name: "Triggering source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(activated_ability(
                Cost(vec![CostComponent::Tap].into()),
                OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
            )),
            profile: ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }));
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Mana watcher".into(),
        abilities: vec![Ability::Mana(ManaAbility::Triggered(Arc::new(
            TriggeredAbility {
                ability_word: None,
                where_x: None,
                from: None,
                event: EventFilter::TapForMana {
                    what: Predicate::Any,
                    by: Predicate::Any,
                },
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::ProducedByEvent.into(),
                )),
            },
        )))],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![parent, source, watcher],
            },
            PlayerConfig { deck: vec![] },
        ],
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
    let parent = put_in_play(&mut state, payer, "Triggered parent");
    let source = put_in_play(&mut state, payer, "Triggering source");
    let _watcher = put_in_play(&mut state, payer, "Mana watcher");
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, source)
}

fn causal_trigger_fixture(
    event: EventFilter,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let payer = PlayerId(0);
    let parent = Arc::new(Card::Normal(CardFace {
        name: "Causal parent".into(),
        abilities: vec![Ability::activated(activated_ability(
            Cost(vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())].into()),
            OneShotEffect::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let source = Arc::new(Card::Normal(CardFace {
        name: "Causal source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Mana(ManaAbility::Activated {
            ability: Arc::new(activated_ability(
                Cost(vec![CostComponent::Tap].into()),
                OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
            )),
            profile: ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }));
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Causal watcher".into(),
        abilities: vec![Ability::Mana(ManaAbility::Triggered(Arc::new(
            TriggeredAbility {
                ability_word: None,
                where_x: None,
                from: None,
                event,
                condition: None,
                limits: Arc::from([]),
                effect: OneShotEffect::Act(CoreAction::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Black.into()).into(),
                )),
            },
        )))],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![parent, source, watcher],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 19,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let parent = put_in_play(&mut state, payer, "Causal parent");
    let source = put_in_play(&mut state, payer, "Causal source");
    let _watcher = put_in_play(&mut state, payer, "Causal watcher");
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, source)
}

fn run_to_payment(state: &mut GameState) -> PaymentPrompt {
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Payment(prompt)) => return prompt,
            other => panic!("expected a payment prompt, got {other:?}"),
        }
    }
}

#[test]
fn tap_only_mana_child_runs_paying_ready_submit_and_promotes_whole_image() {
    let (mut state, payer, parent, mana_source) = payment_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    let parent_prompt = run_to_payment(&mut state);
    assert_eq!(parent_prompt.stage, PaymentStage::PrePayment);
    assert_eq!(parent_prompt.mana_abilities, vec![(mana_source, 0)]);

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: mana_source,
            ability: 0,
        }))
        .unwrap();
    let child_prompt = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 2);
    assert_eq!(child_prompt.stage, PaymentStage::Paying);
    let tap = child_prompt.outstanding[0].id;

    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    let child_ready = run_to_payment(&mut state);
    assert_eq!(child_ready.stage, PaymentStage::Ready);
    assert!(state.objects.obj(mana_source).tapped);

    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let resumed_parent = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 1);
    assert_eq!(resumed_parent.stage, PaymentStage::PrePayment);
    assert!(state.objects.obj(mana_source).tapped);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1
    );
}

#[test]
fn standalone_mana_activation_uses_root_payment_and_resolves_stacklessly() {
    let (mut state, payer, _parent, source) = payment_fixture();
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: source,
            ability: 0,
        }],
    }));

    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: source,
            ability: 0,
        }))
        .unwrap();
    let paying = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 1);
    assert_eq!(paying.stage, PaymentStage::Paying);
    assert!(!state.objects.obj(source).tapped);

    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: paying.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    for _ in 0..40 {
        if state.payment_depth() == 0 && matches!(state.pending, Some(PendingDecision::Priority(_)))
        {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    assert_eq!(state.payment_depth(), 0);
    assert!(state.objects.obj(source).tapped);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1
    );
    assert!(state.stack.is_empty());
}

#[test]
fn modal_mana_child_rejects_a_nonmana_announced_mode_without_mutation() {
    let (mut state, _payer, parent, source) = modal_payment_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    let _ = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    for _ in 0..20 {
        if matches!(state.pending, Some(PendingDecision::ChooseModes(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }
    assert!(matches!(
        state.pending,
        Some(PendingDecision::ChooseModes(_))
    ));

    assert!(state.submit_decision(Decision::Modes(vec![1])).is_err());
    assert!(matches!(
        state.pending,
        Some(PendingDecision::ChooseModes(_))
    ));
    assert!(
        state
            .announcing
            .as_ref()
            .expect("the rejected announcement stays live")
            .chosen_modes
            .is_empty()
    );

    state.submit_decision(Decision::Modes(vec![0])).unwrap();
}

#[test]
fn in_flight_mana_ability_is_not_offered_recursively() {
    let zero: ManaCost = "{0}".parse().unwrap();
    let (mut state, _, parent, mana_source) =
        payment_fixture_with_source_cost(Cost(vec![CostComponent::Mana(zero)].into()));
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    let parent_prompt = run_to_payment(&mut state);
    assert_eq!(parent_prompt.mana_abilities, vec![(mana_source, 0)]);

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: mana_source,
            ability: 0,
        }))
        .unwrap();
    let child_prompt = run_to_payment(&mut state);
    assert_eq!(child_prompt.stage, PaymentStage::PrePayment);
    assert!(
        !child_prompt.mana_abilities.contains(&(mana_source, 0)),
        "the exact submitted-but-unfinished action cannot activate itself"
    );
}

#[test]
fn source_can_be_announced_then_sacrificed_to_kci() {
    let (mut state, payer, parent, kci) = kci_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    let parent_prompt = run_to_payment(&mut state);
    assert_eq!(parent_prompt.stage, PaymentStage::PrePayment);
    assert!(parent_prompt.mana_abilities.contains(&(kci, 0)));

    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: kci,
            ability: 0,
        }))
        .unwrap();
    let kci_prompt = run_to_payment(&mut state);
    assert_eq!(kci_prompt.stage, PaymentStage::Paying);
    let sacrifice = kci_prompt.outstanding[0].id;
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: sacrifice,
            witness: FulfillmentWitness::Objects(vec![parent]),
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    assert!(state.objects.get(parent).is_none());

    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let resumed = run_to_payment(&mut state);
    assert_eq!(resumed.stage, PaymentStage::PrePayment);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Colorless),
        2
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(
            deckmaste_engine::ManaCoverage::empty(),
        )))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    for _ in 0..20 {
        if !state.stack.is_empty() {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }
    assert_eq!(state.stack.len(), 1);
}

#[test]
fn tapped_for_mana_trigger_resolves_before_parent_payment_resumes() {
    let (mut state, payer, parent, source) = triggered_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    let _ = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let child = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: child.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let resumed = run_to_payment(&mut state);

    assert_eq!(resumed.stage, PaymentStage::PrePayment);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        2,
        "the triggered mana ability must finish before parent control resumes"
    );
    assert!(state.pending_triggers.is_empty());
}

#[test]
fn triggered_mana_matches_each_causal_fact_before_parent_continues() {
    let source_is_land = Predicate::r#type(Type::Land);
    let causes = [
        EventFilter::ManaAbilityActivated {
            what: source_is_land.clone(),
            by: Predicate::Any,
        },
        EventFilter::ManaProduced {
            what: source_is_land.clone(),
            by: Predicate::Any,
        },
        EventFilter::ManaAdded {
            what: source_is_land.clone(),
            by: Predicate::Any,
        },
        EventFilter::TapForMana {
            what: source_is_land,
            by: Predicate::Any,
        },
    ];

    for event in causes {
        let (mut state, payer, parent, source) = causal_trigger_fixture(event.clone());
        state
            .submit_decision(Decision::Act(Action::ActivateAbility {
                object: parent,
                ability: 0,
            }))
            .unwrap();
        let _ = run_to_payment(&mut state);
        state
            .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
                source,
                ability: 0,
            }))
            .unwrap();
        let child = run_to_payment(&mut state);
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: child.outstanding[0].id,
                witness: FulfillmentWitness::Bound,
            }))
            .unwrap();
        let _ = run_to_payment(&mut state);
        state
            .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
            .unwrap();
        let resumed = run_to_payment(&mut state);
        assert_eq!(resumed.stage, PaymentStage::PrePayment, "cause {event:?}");
        assert_eq!(
            state
                .player(payer)
                .mana_pool
                .amount(deckmaste_core::ColorOrColorless::Color(Color::Black)),
            1,
            "cause {event:?} must resolve its triggered mana before parent control resumes"
        );
        assert!(state.pending_triggers.is_empty(), "cause {event:?}");
    }
}
