use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::ActivatedManaProfile;
use deckmaste_core::Binder;
use deckmaste_core::Cmp;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Destination;
use deckmaste_core::EventFilter;
use deckmaste_core::KeywordAbility;
use deckmaste_core::ManaAbility;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaRider;
use deckmaste_core::ManaSpec;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Replacement;
use deckmaste_core::Stat;
use deckmaste_core::StatValue;
use deckmaste_core::StatePredicate;
use deckmaste_core::StaticEffect;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::EngineIncident;
use deckmaste_engine::FulfillmentWitness;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::IouId;
use deckmaste_engine::IouKind;
use deckmaste_engine::ManaCoverage;
use deckmaste_engine::ManaPayment;
use deckmaste_engine::ManaPip;
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectSource;
use deckmaste_engine::ObservationBarrier;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentDeclined;
use deckmaste_engine::PaymentProgress;
use deckmaste_engine::PaymentStage;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::PriorityRound;
use deckmaste_engine::ReversalBarrier;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_lowering::Lower;
use deckmaste_plugin::plugin::Plugin;

fn activated_card(cost: Vec<CostComponent>) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Payment fixture".into(),
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
    }))
}

fn activation_fixture(
    cost: Vec<CostComponent>,
) -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    activation_fixture_with_extras(cost, Vec::new())
}

fn activation_fixture_with_extras(
    cost: Vec<CostComponent>,
    extras: Vec<Arc<Card>>,
) -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    activation_fixture_with_card(activated_card(cost), extras)
}

fn activation_fixture_with_card(
    source_card: Arc<Card>,
    extras: Vec<Arc<Card>>,
) -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    let payer = PlayerId(0);
    let source_name = card_name(&source_card).to_owned();
    let mut deck = vec![source_card];
    deck.extend(extras);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
        seed: 7,
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
        .find(|&object| card_name(state.def(object)) == source_name)
        .expect("the fixture source was drawn");
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

fn crew_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let plugin = Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"))
        .expect("the builtin plugin loads");
    let semantic: deckmaste_semantics::KeywordAbility =
        plugin.macros.read_str("Crew(3)").expect("Crew expands");
    let KeywordAbility::Composite { abilities, .. } = semantic.lower() else {
        panic!("Crew lowers to a composite keyword")
    };
    let crew = abilities
        .into_iter()
        .find(|ability| ability.as_activated().is_some())
        .expect("Crew confers one activated ability");
    let vehicle = Arc::new(Card::Normal(CardFace {
        name: "Crew Vehicle".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![crew],
        ..CardFace::default()
    }));
    let (mut state, payer, source) = activation_fixture_with_card(
        vehicle,
        vec![
            vanilla_creature("One", 1),
            vanilla_creature("Two", 2),
            vanilla_creature("Three", 3),
            vanilla_creature("Four", 4),
        ],
    );
    let one = put_named_card_on_battlefield(&mut state, payer, "One");
    let two = put_named_card_on_battlefield(&mut state, payer, "Two");
    let three = put_named_card_on_battlefield(&mut state, payer, "Three");
    let four = put_named_card_on_battlefield(&mut state, payer, "Four");
    (state, payer, source, one, two, three, four)
}

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
                    Reference::You,
                    Count::Literal(1),
                    ManaSpec::AnyColor.into(),
                ))
                .into(),
            }),
            profile: ActivatedManaProfile::Always,
        })],
        ..CardFace::default()
    }))
}

fn card_name(card: &Card) -> &str {
    match card {
        Card::Normal(face) => &face.name,
        Card::TwoFaced { front, .. } => &front.name,
    }
}

fn hand_card(state: &GameState, player: PlayerId, name: &str) -> deckmaste_engine::ObjectId {
    state.zones.hands[player.index()]
        .iter()
        .copied()
        .find(|&object| card_name(state.def(object)) == name)
        .unwrap_or_else(|| panic!("{name} should be in the fixture hand"))
}

fn put_named_card_on_battlefield(
    state: &mut GameState,
    player: PlayerId,
    name: &str,
) -> deckmaste_engine::ObjectId {
    let object = hand_card(state, player, name);
    state.zones.hands[player.index()].retain(|&candidate| candidate != object);
    state.objects.obj_mut(object).zone = Some(Zone::Battlefield);
    state.objects.obj_mut(object).summoning_sick = false;
    state.zones.battlefield.push(object);
    object
}

fn vanilla_creature(name: &str, power: i32) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: name.into(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(power)),
        toughness: Some(StatValue::Number(power)),
        ..CardFace::default()
    }))
}

fn discard_two_cost() -> CostComponent {
    let CoreAction::Composite { name, body } =
        CoreAction::discard(Reference::You, deckmaste_core::Count::Literal(2), false)
    else {
        unreachable!()
    };
    let OneShotEffect::With(with) = body.as_ref() else { unreachable!() };
    CostComponent::ChooseAndPay {
        binder: Arc::new(with.binder.clone()),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Composite {
                name,
                body: Arc::clone(&with.body),
            })]
            .into(),
        ),
    }
}

fn announce_to_payment(state: &mut GameState, source: deckmaste_engine::ObjectId) {
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: source,
            ability: 0,
        }))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Payment(_)) => return,
            other => panic!("unexpected stop while announcing payment fixture: {other:?}"),
        }
    }
}

fn payment_prompt(state: &GameState) -> deckmaste_engine::PaymentPrompt {
    match state.pending.as_ref() {
        Some(PendingDecision::Payment(prompt)) => prompt.clone(),
        other => panic!("expected payment prompt, got {other:?}"),
    }
}

fn submit_and_run_to_payment(state: &mut GameState, command: PaymentCommand) {
    state.submit_decision(Decision::Payment(command)).unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Payment(_)) => return,
            other => panic!("unexpected stop while completing a fulfillment: {other:?}"),
        }
    }
}

#[test]
fn tap_cost_opens_paying_without_mutating_committed_image() {
    let (mut state, _, source) = activation_fixture(vec![CostComponent::Tap]);
    announce_to_payment(&mut state, source);

    let prompt = payment_prompt(&state);
    assert_eq!(prompt.stage, PaymentStage::Paying);
    assert_eq!(prompt.outstanding.len(), 1);
    assert!(matches!(prompt.outstanding[0].kind, IouKind::Tap));
    assert!(state.announcing.is_some());
    assert!(state.committed().announcing.is_none());
    assert!(!state.objects.obj(source).tapped);
    assert!(!state.committed().objects.obj(source).tapped);
}

#[test]
fn tap_cost_requires_fulfill_then_submit() {
    let (mut state, _, source) = activation_fixture(vec![CostComponent::Tap]);
    announce_to_payment(&mut state, source);
    let tap = payment_prompt(&state).outstanding[0].id;

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        },
    );
    assert!(state.objects.obj(source).tapped);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
    assert!(!state.committed().objects.obj(source).tapped);

    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    assert!(state.committed().objects.obj(source).tapped);
}

#[test]
fn fulfillment_stays_in_flight_until_its_finish_sentinel() {
    let (mut state, _, source) = activation_fixture(vec![CostComponent::Tap]);
    announce_to_payment(&mut state, source);
    let tap = payment_prompt(&state).outstanding[0].id;

    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(
        state.payment_progress(),
        Some(PaymentProgress::Fulfilling {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        })
    );
    assert!(state.pending.is_none());
    assert!(!state.objects.obj(source).tapped);

    assert!(matches!(state.step(), StepOutcome::Progress(_)));
    assert!(state.objects.obj(source).tapped);
    assert!(matches!(
        state.payment_progress(),
        Some(PaymentProgress::Fulfilling { iou, .. }) if iou == tap
    ));

    assert!(matches!(state.step(), StepOutcome::Progress(_)));
    assert_eq!(state.payment_progress(), Some(PaymentProgress::Idle));
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

#[test]
fn deferred_library_iou_waits_for_the_ordinary_tier() {
    let library_move = CostComponent::do_action(CoreAction::Move(
        Reference::This,
        Destination::Zone(Zone::Exile),
        Arc::from([]),
        Some(Zone::Library),
    ));
    let (mut state, _, source) = activation_fixture(vec![CostComponent::Tap, library_move]);
    announce_to_payment(&mut state, source);
    let prompt = payment_prompt(&state);
    let tap = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .unwrap()
        .id;
    let library = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Act(_)))
        .unwrap()
        .id;
    assert_eq!(prompt.fulfillable, vec![tap]);

    let before = payment_prompt(&state);
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: library,
                witness: FulfillmentWitness::Bound,
            }))
            .is_err()
    );
    assert_eq!(payment_prompt(&state), before);

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        },
    );
    assert_eq!(payment_prompt(&state).fulfillable, vec![library]);
}

#[test]
fn discard_set_validates_before_any_card_moves() {
    let first_card = Arc::new(Card::Normal(CardFace {
        name: "First discard".into(),
        ..CardFace::default()
    }));
    let second_card = Arc::new(Card::Normal(CardFace {
        name: "Second discard".into(),
        ..CardFace::default()
    }));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![discard_two_cost()], vec![first_card, second_card]);
    let first = hand_card(&state, payer, "First discard");
    let second = hand_card(&state, payer, "Second discard");
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;

    let before = state.zones.hands[payer.index()].clone();
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Objects(vec![first, first]),
            }))
            .is_err()
    );
    assert_eq!(state.zones.hands[payer.index()], before);
    assert_eq!(payment_prompt(&state).outstanding.len(), 1);

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Objects(vec![first, second]),
        },
    );
    assert!(state.zones.hands[payer.index()].is_empty());
    assert_eq!(state.zones.graveyards[payer.index()].len(), 2);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

fn choose_one_cost(filter: Predicate, actions: Vec<CoreAction>) -> CostComponent {
    CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::ChooseOne {
            filter,
            by: Reference::You,
        }),
        body: Cost(
            actions
                .into_iter()
                .map(CostComponent::do_action)
                .collect::<Vec<_>>()
                .into(),
        ),
    }
}

#[test]
fn choose_and_pay_preflights_the_entire_bound_body() {
    let filter = Predicate::And(
        vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    let bound = Reference::That(deckmaste_core::Sort::Card);
    let cost = choose_one_cost(
        filter,
        vec![CoreAction::Tap(bound.clone()), CoreAction::Tap(bound)],
    );
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![cost],
        vec![vanilla_creature("Atomic cost subject", 1)],
    );
    let subject = put_named_card_on_battlefield(&mut state, payer, "Atomic cost subject");
    announce_to_payment(&mut state, source);
    let before = payment_prompt(&state);

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: before.outstanding[0].id,
                witness: FulfillmentWitness::Objects(vec![subject]),
            }))
            .is_err(),
        "the second tap makes the selected body atomically unpayable",
    );
    assert_eq!(payment_prompt(&state), before);
    assert!(!state.objects.obj(subject).tapped);
}

#[test]
fn choose_and_pay_preflight_accounts_for_zone_change_remint() {
    let filter = Predicate::And(
        vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    let bound = Reference::That(deckmaste_core::Sort::Card);
    let cost = choose_one_cost(
        filter,
        vec![
            CoreAction::Move(
                bound.clone(),
                Destination::Zone(Zone::Exile),
                Arc::from([]),
                Some(Zone::Battlefield),
            ),
            CoreAction::RemoveCounters(
                bound,
                deckmaste_core::CounterRef::from("ChargeCounter"),
                Count::Literal(1),
            ),
        ],
    );
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![cost],
        vec![vanilla_creature("Reminted cost subject", 1)],
    );
    let subject = put_named_card_on_battlefield(&mut state, payer, "Reminted cost subject");
    state
        .objects
        .obj_mut(subject)
        .counters
        .insert("ChargeCounter".into(), 1);
    announce_to_payment(&mut state, source);
    let before = payment_prompt(&state);

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: before.outstanding[0].id,
                witness: FulfillmentWitness::Objects(vec![subject]),
            }))
            .is_err(),
        "a counter cannot be paid after its carrier remints through a zone change",
    );
    assert_eq!(payment_prompt(&state), before);
    assert!(state.zones.battlefield.contains(&subject));
}

#[test]
fn choose_and_pay_rejects_sacrificing_an_opponents_permanent() {
    let bound = Reference::That(deckmaste_core::Sort::Card);
    let cost = choose_one_cost(
        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
        vec![CoreAction::Sacrifice(Reference::You, bound)],
    );
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![cost],
        vec![vanilla_creature("Opponent-controlled cost subject", 1)],
    );
    let subject =
        put_named_card_on_battlefield(&mut state, payer, "Opponent-controlled cost subject");
    state.objects.obj_mut(subject).controller = PlayerId(1);
    announce_to_payment(&mut state, source);
    let before = payment_prompt(&state);

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: before.outstanding[0].id,
                witness: FulfillmentWitness::Objects(vec![subject]),
            }))
            .is_err(),
        "a payer cannot sacrifice a permanent they do not control",
    );
    assert_eq!(payment_prompt(&state), before);
    assert!(state.zones.battlefield.contains(&subject));
}

fn unsupported_bound_binder_cost(binder: Binder) -> CostComponent {
    CostComponent::ChooseAndPay {
        binder: Arc::new(binder),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Tap(Reference::That(
                deckmaste_core::Sort::Card,
            )))]
            .into(),
        ),
    }
}

#[test]
fn nested_random_reference_cost_binder_fails_closed() {
    let random = deckmaste_core::Selection::Random(deckmaste_core::Quantity::one(), Predicate::Any);
    let cost = unsupported_bound_binder_cost(Binder::TheRef(Reference::Single(Arc::new(random))));
    let (mut state, _, source) = activation_fixture(vec![cost]);
    announce_to_payment(&mut state, source);
    let before = payment_prompt(&state);

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: before.outstanding[0].id,
                witness: FulfillmentWitness::Bound,
            }))
            .is_err(),
        "an unsampled nested Random must reject instead of reaching reference evaluation",
    );
    assert_eq!(payment_prompt(&state), before);
}

#[test]
fn random_producer_subject_cost_binder_fails_closed() {
    let random = Reference::Single(Arc::new(deckmaste_core::Selection::Random(
        deckmaste_core::Quantity::one(),
        Predicate::Any,
    )));
    let cost = unsupported_bound_binder_cost(Binder::Produce(Arc::new(CoreAction::Move(
        random,
        Destination::Zone(Zone::Exile),
        Arc::from([]),
        None,
    ))));
    let (mut state, _, source) = activation_fixture(vec![cost]);
    announce_to_payment(&mut state, source);
    let before = payment_prompt(&state);

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: before.outstanding[0].id,
                witness: FulfillmentWitness::Bound,
            }))
            .is_err(),
        "a raw producer may not carry an unresolved random subject",
    );
    assert_eq!(payment_prompt(&state), before);
}

#[test]
fn tap_total_enumerates_and_accepts_every_live_satisfying_subset() {
    let filter = Predicate::And(
        vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::r#type(Type::Creature),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    let cost = CostComponent::TapTotal {
        stat: Stat::Power,
        cmp: Cmp::AtLeast,
        count: deckmaste_core::Count::Literal(3),
        filter: Arc::new(filter),
    };
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![cost],
        vec![
            vanilla_creature("One", 1),
            vanilla_creature("Two", 2),
            vanilla_creature("Three", 3),
        ],
    );
    let one = put_named_card_on_battlefield(&mut state, payer, "One");
    let two = put_named_card_on_battlefield(&mut state, payer, "Two");
    let three = put_named_card_on_battlefield(&mut state, payer, "Three");
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;

    let subsets = state.legal_tap_total_subsets(iou);
    assert_eq!(subsets.len(), 5);
    for expected in [
        vec![three],
        vec![one, two],
        vec![one, three],
        vec![two, three],
        vec![one, two, three],
    ] {
        assert!(
            subsets.contains(&expected),
            "missing legal subset {expected:?}"
        );
    }

    let before = payment_prompt(&state);
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Objects(vec![one, one, two]),
            }))
            .is_err()
    );
    assert_eq!(payment_prompt(&state), before);
    assert!(!state.objects.obj(one).tapped);

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Objects(vec![one, two]),
        },
    );
    assert!(state.objects.obj(one).tapped);
    assert!(state.objects.obj(two).tapped);
    assert!(!state.objects.obj(three).tapped);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

#[test]
fn tap_total_rejects_a_witness_that_became_stale_after_enumeration() {
    let (mut state, _payer, source, one, two, _three, _four) = crew_fixture();
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;
    assert!(state.legal_tap_total_subsets(iou).contains(&vec![one, two]));

    state.objects.obj_mut(two).tapped = true;
    let before = payment_prompt(&state);
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Objects(vec![one, two]),
            }))
            .is_err()
    );
    assert_eq!(payment_prompt(&state), before);
    assert!(!state.objects.obj(one).tapped);
    assert!(state.objects.obj(two).tapped);
}

#[test]
fn runner_chooses_the_fewest_highest_power_crew_members() {
    let (mut state, _payer, source, one, two, three, four) = crew_fixture();
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;

    assert_eq!(
        state.auto_payment_pending(),
        Some(Decision::Payment(PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Objects(vec![four]),
        }))
    );
    assert!(!state.objects.obj(one).tapped);
    assert!(!state.objects.obj(two).tapped);
    assert!(!state.objects.obj(three).tapped);
}

#[test]
fn runner_selects_a_complete_choose_and_pay_witness() {
    let first_card = Arc::new(Card::Normal(CardFace {
        name: "First automatic discard".into(),
        ..CardFace::default()
    }));
    let second_card = Arc::new(Card::Normal(CardFace {
        name: "Second automatic discard".into(),
        ..CardFace::default()
    }));
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![discard_two_cost()], vec![first_card, second_card]);
    let first = hand_card(&state, payer, "First automatic discard");
    let second = hand_card(&state, payer, "Second automatic discard");
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;

    assert_eq!(
        state.auto_payment_pending(),
        Some(Decision::Payment(PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Objects(vec![first, second]),
        }))
    );
}

#[test]
fn runner_declines_an_insufficient_random_choose_and_pay_cost() {
    let only_card = Arc::new(Card::Normal(CardFace {
        name: "Only automatic random discard".into(),
        ..CardFace::default()
    }));
    let (mut state, _, source) =
        activation_fixture_with_extras(vec![random_discard_two_cost()], vec![only_card]);
    announce_to_payment(&mut state, source);

    assert_eq!(
        state.auto_payment_pending(),
        Some(Decision::Payment(PaymentCommand::DeclinePayment))
    );
}

#[test]
fn runner_does_not_open_an_unfunded_mana_cylix_child() {
    let green: ManaCost = "{G}".parse().unwrap();
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![CostComponent::Mana(green)],
        vec![mana_cylix_fixture()],
    );
    let cylix = put_named_card_on_battlefield(&mut state, payer, "Mana Cylix");
    announce_to_payment(&mut state, source);
    assert_eq!(payment_prompt(&state).mana_abilities, vec![(cylix, 0)]);

    assert_eq!(
        state.auto_payment_pending(),
        Some(Decision::Payment(PaymentCommand::DeclinePayment))
    );
    assert!(!state.objects.obj(cylix).tapped);
}

#[test]
fn runner_does_not_use_restricted_mana_to_fund_a_mana_source() {
    let green: ManaCost = "{G}".parse().unwrap();
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![CostComponent::Mana(green)],
        vec![mana_cylix_fixture()],
    );
    let cylix = put_named_card_on_battlefield(&mut state, payer, "Mana Cylix");
    state.player_mut(payer).mana_pool.add_riders(
        Color::Green.into(),
        1,
        &[ManaRider::SpendOnly(Predicate::r#type(Type::Instant))],
        ManaProvenance::default(),
    );
    announce_to_payment(&mut state, source);
    assert_eq!(payment_prompt(&state).mana_abilities, vec![(cylix, 0)]);

    assert_eq!(
        state.auto_payment_pending(),
        Some(Decision::Payment(PaymentCommand::DeclinePayment))
    );
    assert!(!state.objects.obj(cylix).tapped);
}

#[test]
fn builtin_crew_activates_and_its_creature_type_expires_at_end_of_turn() {
    let (mut state, _payer, vehicle, one, two, three, four) = crew_fixture();
    assert!(!state.layers().get(vehicle).has_type(Type::Creature));
    announce_to_payment(&mut state, vehicle);

    let fulfill = state
        .auto_payment_pending()
        .expect("the runner chooses a Crew witness");
    state.submit_decision(fulfill).unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Payment(prompt)) => {
                assert_eq!(prompt.stage, PaymentStage::Ready);
                break;
            }
            other => panic!("unexpected stop while paying Crew: {other:?}"),
        }
    }
    state
        .submit_decision(
            state
                .auto_payment_pending()
                .expect("the runner submits completed Crew payment"),
        )
        .unwrap();

    let first_priority = loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority(priority)) => break priority,
            other => panic!("unexpected stop before Crew reaches the stack: {other:?}"),
        }
    };
    assert_eq!(first_priority.player, PlayerId(0));
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    let second_priority = loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority(priority)) => break priority,
            other => panic!("unexpected stop before the opponent passes: {other:?}"),
        }
    };
    assert_eq!(second_priority.player, PlayerId(1));
    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
    for _ in 0..100 {
        if state.stack.is_empty() && state.layers().get(vehicle).has_type(Type::Creature) {
            break;
        }
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(PendingDecision::Priority(_)) => break,
            other => panic!("unexpected stop while Crew resolves: {other:?}"),
        }
    }

    assert!(state.objects.obj(four).tapped);
    assert!(!state.objects.obj(one).tapped);
    assert!(!state.objects.obj(two).tapped);
    assert!(!state.objects.obj(three).tapped);
    assert!(state.layers().get(vehicle).has_type(Type::Creature));
    state.expire_end_of_turn();
    assert!(!state.layers().get(vehicle).has_type(Type::Creature));
}

#[test]
fn mana_pips_spend_covered_units_one_at_a_time() {
    let cost: ManaCost = "{1}{G}".parse().unwrap();
    let (mut state, payer, source) = activation_fixture(vec![CostComponent::Mana(cost)]);
    let colorless = state.player_mut(payer).mana_pool.add(
        ColorOrColorless::Colorless,
        1,
        ManaProvenance::default(),
    )[0];
    let green = state.player_mut(payer).mana_pool.add(
        ColorOrColorless::Color(Color::Green),
        1,
        ManaProvenance::default(),
    )[0];
    announce_to_payment(&mut state, source);

    let prompt = payment_prompt(&state);
    assert!(
        prompt.fulfillable.is_empty(),
        "PrePayment must not advertise IOUs that Fulfill will reject"
    );
    let generic_iou = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(ManaPip::Generic)))
        .unwrap()
        .id;
    let green_iou = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(ManaPip::Colored(Color::Green))))
        .unwrap()
        .id;
    let mut coverage = ManaCoverage::empty();
    coverage.insert(generic_iou, ManaPayment::Floating(colorless));
    coverage.insert(green_iou, ManaPayment::Floating(green));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: green_iou,
            witness: FulfillmentWitness::CoveredMana,
        },
    );
    assert!(state.player(payer).mana_pool.get(green).is_none());
    assert!(state.player(payer).mana_pool.get(colorless).is_some());
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Paying);

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: generic_iou,
            witness: FulfillmentWitness::CoveredMana,
        },
    );
    assert!(state.player(payer).mana_pool.is_empty());
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

#[test]
fn automatic_payment_matches_constrained_pips_before_generic_ones() {
    let cost: ManaCost = "{1}{R}".parse().unwrap();
    let (mut state, payer, source) = activation_fixture(vec![CostComponent::Mana(cost)]);
    let red = state.player_mut(payer).mana_pool.add(
        ColorOrColorless::Color(Color::Red),
        1,
        ManaProvenance::default(),
    )[0];
    let green = state.player_mut(payer).mana_pool.add(
        ColorOrColorless::Color(Color::Green),
        1,
        ManaProvenance::default(),
    )[0];
    announce_to_payment(&mut state, source);

    let prompt = payment_prompt(&state);
    let generic = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(ManaPip::Generic)))
        .expect("one generic pip")
        .id;
    let colored = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ManaPip(ManaPip::Colored(Color::Red))))
        .expect("one red pip")
        .id;

    let Decision::Payment(PaymentCommand::BeginPayment(coverage)) = state
        .auto_payment_pending()
        .expect("automatic payment decision")
    else {
        panic!("a fully funded prepayment prompt begins payment")
    };
    assert_eq!(coverage.get(generic), Some(&ManaPayment::Floating(green)));
    assert_eq!(coverage.get(colored), Some(&ManaPayment::Floating(red)));
}

#[test]
fn zero_cost_uses_explicit_empty_coverage_and_commits_only_on_submit() {
    let zero: ManaCost = "{0}".parse().unwrap();
    let (mut state, _, source) = activation_fixture(vec![CostComponent::Mana(zero)]);
    announce_to_payment(&mut state, source);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::PrePayment);

    let before = payment_prompt(&state);
    let mut invalid = ManaCoverage::empty();
    invalid.insert(
        IouId(999),
        ManaPayment::Floating(deckmaste_engine::FloatingManaId(999)),
    );
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(invalid)))
            .is_err()
    );
    assert_eq!(payment_prompt(&state), before);
    assert!(state.committed().announcing.is_none());

    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(
            ManaCoverage::empty(),
        )))
        .unwrap();
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
    assert!(state.committed().announcing.is_none());

    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    assert!(state.pending.is_none());
    assert!(state.committed().announcing.is_some());
}

#[test]
fn announcement_decline_restores_preannouncement_priority() {
    let (mut state, payer, source) = activation_fixture(vec![CostComponent::Tap]);
    announce_to_payment(&mut state, source);
    let subject = payment_prompt(&state).subject;
    assert!(state.announcing.is_some());

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert!(state.announcing.is_none());
    let Some(PendingDecision::Priority(priority)) = state.pending.as_ref() else {
        panic!("declining an announcement should restore its priority prompt");
    };
    assert_eq!(priority.player, payer);
    assert!(priority.legal.contains(&Action::ActivateAbility {
        object: source,
        ability: 0,
    }));
    assert!(!state.objects.obj(source).tapped);
    assert_eq!(
        state.incidents(),
        &[EngineIncident::PaymentDeclined(PaymentDeclined {
            player: payer,
            subject,
            forced_retained_records: Vec::new(),
            crossed_reversal_barrier: false,
            crossed_observation_barrier: false,
        })]
    );
}

#[test]
fn unaffordable_activation_is_still_a_legal_proposal() {
    let green: ManaCost = "{G}".parse().unwrap();
    let (mut state, payer, source) = activation_fixture(vec![CostComponent::Mana(green)]);
    state.pending = None;
    state.agenda = VecDeque::from([WorkItem::OpenPriority]);

    assert!(matches!(state.step(), StepOutcome::Progress(_)));
    let StepOutcome::NeedsDecision(PendingDecision::Priority(priority)) = state.step() else {
        panic!("OpenPriority should surface a priority decision");
    };
    assert_eq!(priority.player, payer);
    assert!(priority.legal.contains(&Action::ActivateAbility {
        object: source,
        ability: 0,
    }));
}

fn random_discard_two_cost() -> CostComponent {
    let CoreAction::Composite { name, body } =
        CoreAction::discard(Reference::You, deckmaste_core::Count::Literal(2), true)
    else {
        unreachable!()
    };
    let OneShotEffect::With(with) = body.as_ref() else { unreachable!() };
    CostComponent::ChooseAndPay {
        binder: Arc::new(with.binder.clone()),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Composite {
                name,
                body: Arc::clone(&with.body),
            })]
            .into(),
        ),
    }
}

#[test]
fn random_cost_waits_for_the_deferred_tier_and_samples_without_a_choice() {
    let first_card = Arc::new(Card::Normal(CardFace {
        name: "First random discard".into(),
        ..CardFace::default()
    }));
    let second_card = Arc::new(Card::Normal(CardFace {
        name: "Second random discard".into(),
        ..CardFace::default()
    }));
    let (mut state, payer, source) = activation_fixture_with_extras(
        vec![CostComponent::Tap, random_discard_two_cost()],
        vec![first_card, second_card],
    );
    announce_to_payment(&mut state, source);
    let prompt = payment_prompt(&state);
    let tap = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .expect("tap IOU")
        .id;
    let random = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ChooseAndPay { .. }))
        .expect("random-discard IOU")
        .id;
    assert_eq!(prompt.fulfillable, vec![tap]);

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        },
    );
    assert_eq!(payment_prompt(&state).fulfillable, vec![random]);
    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: random,
            witness: FulfillmentWitness::Bound,
        },
    );

    assert!(state.zones.hands[payer.index()].is_empty());
    assert_eq!(state.zones.graveyards[payer.index()].len(), 2);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
    let record = state
        .payment_records()
        .expect("active payment records")
        .last()
        .expect("random fulfillment record");
    assert!(
        record
            .observation_barriers
            .contains(&ObservationBarrier::RandomOutcome),
        "the sampled subject set is retained as a replay observation barrier"
    );
    assert!(matches!(
        &record.command,
        deckmaste_engine::ReplayCommand::Fulfill {
            random_outcome: Some(objects),
            ..
        } if objects.objects.len() == 2
    ));
}

#[test]
fn declining_an_omitted_random_cost_preserves_consumed_entropy() {
    let extras = (0..4)
        .map(|index| {
            Arc::new(Card::Normal(CardFace {
                name: format!("Random discard subject {index}").into(),
                ..CardFace::default()
            }))
        })
        .collect();
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![random_discard_two_cost()], extras);
    let before_rng = (state.rng.get_stream(), state.rng.get_word_pos());
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Bound,
        },
    );
    let post_sample_rng = (state.rng.get_stream(), state.rng.get_word_pos());
    assert_ne!(post_sample_rng, before_rng);
    assert_eq!(state.zones.graveyards[payer.index()].len(), 2);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert_eq!(state.zones.hands[payer.index()].len(), 4);
    assert!(state.zones.graveyards[payer.index()].is_empty());
    assert_eq!(
        (state.rng.get_stream(), state.rng.get_word_pos()),
        post_sample_rng,
        "decline may reverse physical effects but must not reuse observed entropy",
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the RNG chronology regression builds and compares both original and reconstructed library states"
)]
fn omitted_random_cost_advances_rng_before_a_retained_shuffle() {
    let move_from_library = CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::SearchOne {
            filter: Predicate::Any,
            by: Reference::You,
            whose: Reference::You,
            from: vec![Zone::Library].into(),
            if_none: None,
        }),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Move(
                Reference::That(deckmaste_core::Sort::Card),
                Destination::Zone(Zone::Exile),
                Arc::from([]),
                Some(Zone::Library),
            ))]
            .into(),
        ),
    };
    let shuffler = Arc::new(Card::Normal(CardFace {
        name: "Payment shuffle replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
            Replacement::Also {
                would: EventFilter::ZoneChange {
                    what: Predicate::Any,
                    from: Some(Zone::Library),
                    to: Some(Zone::Exile),
                    cause: None,
                },
                also: OneShotEffect::Act(CoreAction::Shuffle(
                    deckmaste_core::Selection::LibraryOf(Reference::You),
                )),
            },
        )))],
        ..CardFace::default()
    }));
    let chronology_cards = (0..10).map(|index| {
        Arc::new(Card::Normal(CardFace {
            name: format!("RNG chronology card {index}").into(),
            ..CardFace::default()
        }))
    });
    let payer = PlayerId(0);
    let deck = std::iter::once(activated_card(vec![
        random_discard_two_cost(),
        move_from_library,
    ]))
    .chain(std::iter::once(shuffler))
    .chain(chronology_cards)
    .collect();
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let find_named = |state: &GameState, name: &str| {
        state
            .objects
            .iter()
            .find(|object| {
                matches!(object.source, ObjectSource::Card(_))
                    && card_name(state.def(object.id)) == name
            })
            .expect("fixture card exists")
            .id
    };
    let source = find_named(&state, "Payment fixture");
    let replacement = find_named(&state, "Payment shuffle replacement");
    for object in [source, replacement] {
        state.zones.hands[payer.index()].retain(|&candidate| candidate != object);
        state.zones.libraries[payer.index()].retain(|&candidate| candidate != object);
        state.objects.obj_mut(object).zone = Some(Zone::Battlefield);
        state.objects.obj_mut(object).summoning_sick = false;
        state.zones.battlefield.push(object);
    }
    let chronology_hand: Vec<_> = state.zones.hands[payer.index()]
        .iter()
        .copied()
        .filter(|&object| card_name(state.def(object)).starts_with("RNG chronology card"))
        .collect();
    for object in chronology_hand.into_iter().skip(4) {
        state.zones.hands[payer.index()].retain(|&candidate| candidate != object);
        state.objects.obj_mut(object).zone = Some(Zone::Library);
        state.zones.libraries[payer.index()].push_back(object);
    }
    assert_eq!(state.zones.hands[payer.index()].len(), 4);
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(PendingDecision::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: source,
            ability: 0,
        }],
    }));
    announce_to_payment(&mut state, source);
    let prompt = payment_prompt(&state);
    let random = prompt
        .outstanding
        .iter()
        .find(|iou| {
            matches!(
                &iou.kind,
                IouKind::ChooseAndPay { binder, .. }
                    if matches!(binder.as_ref(), Binder::Existing(deckmaste_core::Selection::Random(..)))
            )
        })
        .unwrap()
        .id;
    let library_iou = prompt
        .outstanding
        .iter()
        .find(|iou| {
            matches!(
                &iou.kind,
                IouKind::ChooseAndPay { binder, .. }
                    if matches!(binder.as_ref(), Binder::SearchOne { .. })
            )
        })
        .unwrap()
        .id;
    let library_object = state.zones.libraries[payer.index()][0];

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: random,
            witness: FulfillmentWitness::Bound,
        },
    );
    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: library_iou,
            witness: FulfillmentWitness::Objects(vec![library_object]),
        },
    );
    let expected_library: Vec<_> = state.zones.libraries[payer.index()]
        .iter()
        .map(|&object| card_name(state.def(object)).to_owned())
        .collect();
    let expected_rng = (state.rng.get_stream(), state.rng.get_word_pos());

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    let replayed_library: Vec<_> = state.zones.libraries[payer.index()]
        .iter()
        .map(|&object| card_name(state.def(object)).to_owned())
        .collect();
    assert_eq!(replayed_library, expected_library);
    assert_eq!(
        (state.rng.get_stream(), state.rng.get_word_pos()),
        expected_rng
    );
}

#[test]
fn random_cost_rejects_an_insufficient_subject_set_without_advancing_rng() {
    let only_card = Arc::new(Card::Normal(CardFace {
        name: "Only random discard".into(),
        ..CardFace::default()
    }));
    let (mut state, _, source) =
        activation_fixture_with_extras(vec![random_discard_two_cost()], vec![only_card]);
    announce_to_payment(&mut state, source);
    let prompt = payment_prompt(&state);
    let iou = prompt.outstanding[0].id;

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Bound,
            }))
            .is_err()
    );
    assert_eq!(payment_prompt(&state), prompt);
    assert!(state.payment_records().is_some_and(<[_]>::is_empty));
}

#[test]
fn search_cost_validates_and_runs_an_explicit_complete_witness() {
    let sought = Arc::new(Card::Normal(CardFace {
        name: "Sought card".into(),
        ..CardFace::default()
    }));
    let search = CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::SearchOne {
            filter: Predicate::Any,
            by: Reference::You,
            whose: Reference::You,
            from: Arc::from([Zone::Hand]),
            if_none: None,
        }),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Move(
                Reference::That(deckmaste_core::Sort::Card),
                Destination::Zone(Zone::Exile),
                Arc::from([]),
                Some(Zone::Hand),
            ))]
            .into(),
        ),
    };
    let (mut state, payer, source) = activation_fixture_with_extras(vec![search], vec![sought]);
    let sought = hand_card(&state, payer, "Sought card");
    announce_to_payment(&mut state, source);
    let prompt = payment_prompt(&state);
    let iou = prompt.outstanding[0].id;

    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou,
                witness: FulfillmentWitness::Objects(vec![source]),
            }))
            .is_err(),
        "a battlefield object is outside the searched Hand domain"
    );
    assert_eq!(payment_prompt(&state), prompt);

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Objects(vec![sought]),
        },
    );
    assert!(!state.zones.hands[payer.index()].contains(&sought));
    assert_eq!(state.zones.exile.len(), 1);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

#[test]
fn plural_library_search_cost_is_deferred_and_requires_the_complete_set() {
    let first = Arc::new(Card::Normal(CardFace {
        name: "First library subject".into(),
        ..CardFace::default()
    }));
    let second = Arc::new(Card::Normal(CardFace {
        name: "Second library subject".into(),
        ..CardFace::default()
    }));
    let search = CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::Search {
            quantity: deckmaste_core::Quantity::Range(
                Some(deckmaste_core::Count::Literal(2)),
                Some(deckmaste_core::Count::Literal(2)),
            ),
            filter: Predicate::Any,
            by: Reference::You,
            whose: Reference::You,
            from: Arc::from([Zone::Library]),
            if_none: None,
        }),
        body: Cost(Arc::from([])),
    };
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![CostComponent::Tap, search], vec![first, second]);
    let first = hand_card(&state, payer, "First library subject");
    let second = hand_card(&state, payer, "Second library subject");
    for object in [first, second] {
        state.zones.hands[payer.index()].retain(|&id| id != object);
        state.objects.obj_mut(object).zone = Some(Zone::Library);
        state.zones.libraries[payer.index()].push_back(object);
    }
    announce_to_payment(&mut state, source);
    let prompt = payment_prompt(&state);
    let tap = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Tap))
        .expect("tap IOU")
        .id;
    let search = prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::ChooseAndPay { .. }))
        .expect("search IOU")
        .id;
    assert_eq!(prompt.fulfillable, vec![tap]);
    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: tap,
            witness: FulfillmentWitness::Bound,
        },
    );

    let before = payment_prompt(&state);
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                iou: search,
                witness: FulfillmentWitness::Objects(vec![first]),
            }))
            .is_err()
    );
    assert_eq!(payment_prompt(&state), before);
    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou: search,
            witness: FulfillmentWitness::Objects(vec![first, second]),
        },
    );
    assert_eq!(state.zones.libraries[payer.index()].len(), 2);
    assert!(state.zones.exile.is_empty());
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

#[test]
fn producer_cost_runs_the_producer_then_binds_its_moved_product() {
    let producer = CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::Produce(Arc::new(CoreAction::Move(
            Reference::This,
            Destination::Zone(Zone::Exile),
            Arc::from([]),
            Some(Zone::Battlefield),
        )))),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Move(
                Reference::That(deckmaste_core::Sort::Card),
                Destination::Zone(Zone::Graveyard),
                Arc::from([]),
                Some(Zone::Exile),
            ))]
            .into(),
        ),
    };
    let (mut state, payer, source) = activation_fixture(vec![producer]);
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;

    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Bound,
        },
    );

    assert!(state.zones.exile.is_empty());
    assert_eq!(state.zones.graveyards[payer.index()].len(), 1);
    assert_eq!(payment_prompt(&state).stage, PaymentStage::Ready);
}

#[test]
fn unresolved_choice_cannot_construct_a_runnable_core_act() {
    let malformed = CostComponent::try_do_action(CoreAction::discard(
        Reference::You,
        deckmaste_core::Count::Literal(1),
        false,
    ));
    assert_eq!(
        malformed,
        Err(deckmaste_core::RunnableCostActionError::UnresolvedSubject)
    );
}

fn random_library_exile_two_cost() -> CostComponent {
    let filter = Predicate::And(
        vec![
            Predicate::State(StatePredicate::InZone(Zone::Library)),
            Predicate::Relation(RelationPredicate::Owner(Arc::new(Predicate::Ref(
                Reference::You,
            )))),
        ]
        .into(),
    );
    CostComponent::ChooseAndPay {
        binder: Arc::new(Binder::Existing(deckmaste_core::Selection::Random(
            deckmaste_core::Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
            filter,
        ))),
        body: Cost(
            vec![CostComponent::do_action(CoreAction::Composite {
                name: deckmaste_core::VerbName::from("Discard"),
                body: Arc::new(OneShotEffect::Each(deckmaste_core::Each {
                    binder: Binder::Existing(deckmaste_core::Selection::They),
                    effect: Arc::new(OneShotEffect::Act(CoreAction::Move(
                        Reference::It,
                        Destination::Zone(Zone::Exile),
                        Arc::from([]),
                        Some(Zone::Library),
                    ))),
                })),
            })]
            .into(),
        ),
    }
}

#[test]
fn decline_replays_the_exact_random_subset_and_restores_post_sample_rng() {
    let extras = (0..4)
        .map(|index| {
            Arc::new(Card::Normal(CardFace {
                name: format!("Random library subject {index}").into(),
                ..CardFace::default()
            }))
        })
        .collect();
    let (mut state, payer, source) =
        activation_fixture_with_extras(vec![random_library_exile_two_cost()], extras);
    let subjects = state.zones.hands[payer.index()].clone();
    assert_eq!(subjects.len(), 4);
    for object in subjects {
        state.zones.hands[payer.index()].retain(|&id| id != object);
        state.objects.obj_mut(object).zone = Some(Zone::Library);
        state.zones.libraries[payer.index()].push_back(object);
    }

    let before_rng = (state.rng.get_stream(), state.rng.get_word_pos());
    announce_to_payment(&mut state, source);
    let iou = payment_prompt(&state).outstanding[0].id;
    submit_and_run_to_payment(
        &mut state,
        PaymentCommand::Fulfill {
            iou,
            witness: FulfillmentWitness::Bound,
        },
    );

    let mut first_subset: Vec<_> = state
        .zones
        .exile
        .iter()
        .map(|&object| card_name(state.def(object)).to_owned())
        .collect();
    first_subset.sort();
    assert_eq!(first_subset.len(), 2);
    let post_sample_rng = (state.rng.get_stream(), state.rng.get_word_pos());
    assert_ne!(
        post_sample_rng, before_rng,
        "the original sample consumes RNG"
    );
    let record = state.payment_records().unwrap().last().unwrap();
    assert!(
        record
            .reversal_barriers
            .contains(&ReversalBarrier::MovedFromLibrary)
    );
    assert!(
        record
            .observation_barriers
            .contains(&ObservationBarrier::RandomOutcome)
    );
    let deckmaste_engine::ReplayCommand::Fulfill {
        random_outcome: Some(random_outcome),
        ..
    } = &record.command
    else {
        panic!("random fulfillment must retain its deterministic outcome");
    };
    assert_eq!(random_outcome.objects.len(), 2);
    assert_eq!(
        (
            random_outcome.post_sample_rng.stream,
            random_outcome.post_sample_rng.word_pos,
        ),
        post_sample_rng,
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    let mut replayed_subset: Vec<_> = state
        .zones
        .exile
        .iter()
        .map(|&object| card_name(state.def(object)).to_owned())
        .collect();
    replayed_subset.sort();
    assert_eq!(
        replayed_subset, first_subset,
        "replay must not reroll the subset"
    );
    assert_eq!(
        (state.rng.get_stream(), state.rng.get_word_pos()),
        post_sample_rng,
        "replay must neither resample nor reuse the consumed entropy",
    );
}
