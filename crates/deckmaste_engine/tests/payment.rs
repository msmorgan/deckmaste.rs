use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::ActivatedManaProfile;
use deckmaste_core::Cmp;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Destination;
use deckmaste_core::KeywordAbility;
use deckmaste_core::ManaAbility;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaRider;
use deckmaste_core::ManaSpec;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Stat;
use deckmaste_core::StatValue;
use deckmaste_core::StatePredicate;
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
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentDeclined;
use deckmaste_engine::PaymentProgress;
use deckmaste_engine::PaymentStage;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::PriorityRound;
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
                    Reference::Reg(deckmaste_core::RefId(1)),
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
        Reference::Reg(deckmaste_core::RefId(0)),
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
        .find(|iou| matches!(iou.kind, IouKind::Act { .. }))
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

#[allow(
    dead_code,
    reason = "shared fixture retained for neighboring payment cases"
)]
fn choose_one_cost(filter: Predicate, actions: Vec<CoreAction>) -> Vec<CostComponent> {
    // [CR#601.2b]: the choice is its own cost instruction, writing the
    // register the paying verbs read.
    let mut block = vec![CostComponent::Choose(deckmaste_core::Choose {
        dest: deckmaste_core::DefId(2),
        by: Reference::Reg(deckmaste_core::RefId(1)),
        quantity: deckmaste_core::Quantity::one(),
        filter: Arc::new(deckmaste_core::Region::candidate(filter)),
    })];
    block.extend(actions.into_iter().map(CostComponent::do_action));
    block
}

#[test]
fn tap_total_enumerates_and_accepts_every_live_satisfying_subset() {
    let filter = Predicate::And(
        vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::r#type(Type::Creature),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::Reg(deckmaste_core::RefId(1)),
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
    let search = CostComponent::Search(deckmaste_core::Search {
        dest: deckmaste_core::DefId(2),
        by: Reference::Reg(deckmaste_core::RefId(1)),
        whose: Reference::Reg(deckmaste_core::RefId(1)),
        from: Arc::from([Zone::Library]),
        quantity: deckmaste_core::Quantity::Range(
            Some(deckmaste_core::Count::Literal(2)),
            Some(deckmaste_core::Count::Literal(2)),
        ),
        filter: Arc::new(deckmaste_core::Region::candidate(Predicate::Any)),
        if_none: deckmaste_core::Block::default(),
    });
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
        .find(|iou| matches!(iou.kind, IouKind::Search(_)))
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
fn unresolved_choice_cannot_construct_a_runnable_core_act() {
    let malformed = CostComponent::try_do_action(CoreAction::discard(
        Reference::Reg(deckmaste_core::RefId(1)),
        deckmaste_core::Count::Literal(1),
        false,
    ));
    assert_eq!(
        malformed,
        Err(deckmaste_core::RunnableCostActionError::UnresolvedSubject)
    );
}
