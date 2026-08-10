use std::collections::VecDeque;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::ManaCost;
use deckmaste_core::OneShotEffect;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::IouId;
use deckmaste_engine::IouKind;
use deckmaste_engine::ManaCoverage;
use deckmaste_engine::ManaPayment;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentStage;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::PriorityRound;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;

fn activated_card(cost: Vec<CostComponent>) -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Payment fixture".into(),
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
    }))
}

fn activation_fixture(
    cost: Vec<CostComponent>,
) -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    let payer = PlayerId(0);
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![activated_card(cost)],
            },
            PlayerConfig { deck: vec![] },
        ],
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
    let source = state.zones.hands[payer.index()][0];
    state.zones.hands[payer.index()].clear();
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
