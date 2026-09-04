use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::ActivatedManaProfile;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::ChooseSpec;
use deckmaste_core::Color;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::EventFilter;
use deckmaste_core::Instruction;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaModeClass;
use deckmaste_core::ManaSpec;
use deckmaste_core::May;
use deckmaste_core::Modal;
use deckmaste_core::Mode;
use deckmaste_core::Predicate;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::Selection;
use deckmaste_core::SpellAbility;
use deckmaste_core::StaticSpec;
use deckmaste_core::TargetSpec;
use deckmaste_core::Token;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Type;
use deckmaste_core::UseLimit;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::FulfillmentWitness;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentPrompt;
use deckmaste_engine::PaymentStage;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::PriorityRound;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;

fn activated_ability(cost: Cost, effect: Instruction) -> ActivatedAbility {
    ActivatedAbility {
        ability_word: None,
        targets: [].into(),
        cost,
        from: None,
        window: None,
        condition: None,
        limits: Arc::from([]),
        effect: deckmaste_core::Region::new(
            deckmaste_core::announced_region_params(0),
            effect.into(),
        ),
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
    payment_fixture_with_source_ability(Ability::Activated(Arc::new(activated_ability(
        source_cost,
        Instruction::Act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Green.into()).into(),
        )),
    ))))
}

fn payment_fixture_with_source_ability(
    source_ability: Ability,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    payment_fixture_with_source_ability_and_extras(source_ability, Vec::new())
}

fn payment_fixture_with_source_ability_and_extras(
    source_ability: Ability,
    extras: Vec<Arc<Card>>,
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
            Instruction::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let mana_source = Arc::new(Card::Normal(CardFace {
        name: "Green source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![source_ability],
        ..CardFace::default()
    }));
    let mut deck = vec![parent, mana_source];
    deck.extend(extras);
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
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
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, mana_source)
}

fn nested_optional_mana_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let optional_source = Ability::Activated(Arc::new(activated_ability(
        Cost(vec![CostComponent::Tap].into()),
        Instruction::Sequentially(
            vec![
                Instruction::Act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
                Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: Some(Arc::new(Instruction::Act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )))),
                    if_not: None,
                }),
            ]
            .into(),
        ),
    )));
    let helper_card = Arc::new(Card::Normal(CardFace {
        name: "Nested optional helper".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(vec![CostComponent::Tap].into()),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
        )))],
        ..CardFace::default()
    }));
    let (mut state, payer, parent, optional_source) =
        payment_fixture_with_source_ability_and_extras(optional_source, vec![helper_card]);
    let helper = put_in_play(&mut state, payer, "Nested optional helper");
    (state, payer, parent, optional_source, helper)
}

fn self_spending_optional_mana_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let source_ability = Ability::Activated(Arc::new(activated_ability(
        Cost(vec![CostComponent::Tap].into()),
        Instruction::Sequentially(
            vec![
                Instruction::Act(CoreAction::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
                Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: None,
                    if_not: None,
                }),
            ]
            .into(),
        ),
    )));
    payment_fixture_with_source_ability_and_extras(source_ability, Vec::new())
}

fn cost_replacement_nested_mana_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let outer = Ability::Activated(Arc::new(activated_ability(
        Cost(
            vec![CostComponent::do_action(CoreAction::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(deckmaste_core::RefId(0)),
            ))]
            .into(),
        ),
        Instruction::Sequentially(
            vec![
                Instruction::Act(CoreAction::Shuffle(Selection::LibraryOf(Reference::Reg(
                    deckmaste_core::RefId(1),
                )))),
                Instruction::Act(CoreAction::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
            ]
            .into(),
        ),
    )));
    let replacement = Arc::new(Card::Normal(CardFace {
        name: "Cost replacement optional".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
            Replacement::Instead {
                would: EventFilter::ZoneChange {
                    what: Predicate::Any,
                    from: Some(Zone::Battlefield),
                    to: Some(Zone::Graveyard),
                    cause: None,
                },
                instead: Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: None,
                    if_not: None,
                }),
            },
        )))],
        ..CardFace::default()
    }));
    let helper = Arc::new(Card::Normal(CardFace {
        name: "Ownership helper".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(vec![CostComponent::Tap].into()),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
        )))],
        ..CardFace::default()
    }));
    let (mut state, payer, parent, outer) =
        payment_fixture_with_source_ability_and_extras(outer, vec![replacement, helper]);
    put_in_play(&mut state, payer, "Cost replacement optional");
    let helper = put_in_play(&mut state, payer, "Ownership helper");
    (state, payer, parent, outer, helper)
}

fn created_nested_optional_mana_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let token_ability = Ability::Activated(Arc::new(activated_ability(
        Cost(vec![CostComponent::Tap].into()),
        Instruction::Act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Green.into()).into(),
        )),
    )));
    let token = Token {
        name: Some("Nested created helper".into()),
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: vec![token_ability].into(),
        power: None,
        toughness: None,
    };
    let source_ability = Ability::Activated(Arc::new(activated_ability(
        Cost(vec![CostComponent::Tap].into()),
        Instruction::Sequentially(
            vec![
                Instruction::Act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
                Instruction::Act(CoreAction::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: token.into(),
                    riders: Arc::from([]),
                }),
                Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: Some(Arc::new(Instruction::Act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )))),
                    if_not: None,
                }),
            ]
            .into(),
        ),
    )));
    payment_fixture_with_source_ability(source_ability)
}

fn modal_payment_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    modal_payment_fixture_with_extras(Vec::new())
}

fn modal_payment_fixture_with_extras(
    extras: Vec<Arc<Card>>,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    modal_payment_fixture_with_ordinary_mode(
        extras,
        Arc::from([]),
        Instruction::Sequentially(Arc::from([])),
        ManaModeClass {
            adds_mana: false,
            targetless: true,
        },
    )
}

fn modal_payment_fixture_with_ordinary_mode(
    extras: Vec<Arc<Card>>,
    ordinary_targets: Arc<[TargetSpec]>,
    ordinary_effect: Instruction,
    ordinary_class: ManaModeClass,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let ordinary_params = deckmaste_core::announced_region_params(ordinary_targets.len());
    let effect = Instruction::Modal(Modal {
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
                effect: deckmaste_core::Region::new(
                    deckmaste_core::announced_region_params(0),
                    Instruction::Act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    ))
                    .into(),
                ),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: ordinary_targets,
                effect: deckmaste_core::Region::new(ordinary_params, ordinary_effect.into()),
                cost: deckmaste_core::Cost::default(),
            },
        ]
        .into(),
    });
    let ability = Ability::Activated(Arc::new(activated_ability(
        Cost(vec![CostComponent::Tap].into()),
        effect,
    )));
    // The mode rows the fixture intends, derived from the ability itself
    // ([CR#605.1a]) rather than authored onto it.
    assert_eq!(
        ability.mana_profile(),
        Some(ActivatedManaProfile::ByAnnouncedMode(
            vec![
                ManaModeClass {
                    adds_mana: true,
                    targetless: true,
                },
                ordinary_class,
            ]
            .into()
        ))
    );
    payment_fixture_with_source_ability_and_extras(ability, extras)
}

fn blanket_activate_lockout_card() -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Split-second-style lockout".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
            DeonticAction::Activate {
                what: Predicate::Any,
                by: Predicate::Any,
                cost: None,
            },
        )))],
        ..CardFace::default()
    }))
}

fn put_in_play(state: &mut GameState, player: PlayerId, name: &str) -> deckmaste_engine::ObjectId {
    let object = state.zones.hands[player.index()]
        .iter()
        .copied()
        .find(|&object| state.def(object).primary_face().name.as_ref() == name)
        .unwrap();
    state.zones.hands[player.index()].retain(|&candidate| candidate != object);
    state.objects.obj_mut(object).zone = Some(Zone::Battlefield);
    state.objects.obj_mut(object).summoning_sick = false;
    state.zones.battlefield.push(object);
    object
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
            Instruction::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let source = Arc::new(Card::Normal(CardFace {
        name: "Triggering source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(vec![CostComponent::Tap].into()),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
        )))],
        ..CardFace::default()
    }));
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Mana watcher".into(),
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::TapForMana {
                what: Predicate::Any,
                by: Predicate::Any,
            },
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::ProducedByEvent.into(),
            ))
            .into(),
        })],
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
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, source)
}

fn nested_resolution_cast_trigger_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let payer = PlayerId(0);
    let parent = Arc::new(Card::Normal(CardFace {
        name: "Nested cast parent".into(),
        abilities: vec![Ability::activated(activated_ability(
            Cost(vec![CostComponent::Mana("{B}".parse().unwrap())].into()),
            Instruction::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let source = Arc::new(Card::Normal(CardFace {
        name: "Nested cast source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(vec![CostComponent::Tap].into()),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
        )))],
        ..CardFace::default()
    }));
    let cast_spell = Arc::new(Card::Normal(CardFace {
        name: "Nested cast spell".into(),
        mana_cost: "{G}".parse().unwrap(),
        types: vec![Type::Instant.def()],
        abilities: vec![Ability::spell(SpellAbility {
            ability_word: None,
            cost: deckmaste_core::Cost::default(),
            targets: [].into(),
            effect: Instruction::Sequentially(Arc::from([])).into(),
        })],
        ..CardFace::default()
    }));
    let cast_ref = Reference::Single(
        Selection::SelectAll(Arc::new(deckmaste_core::Region::candidate(
            Predicate::Characteristic(CharacteristicPredicate::Named("Nested cast spell".into())),
        )))
        .into(),
    );
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Nested cast watcher".into(),
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::ManaAdded {
                what: Predicate::r#type(Type::Land),
                by: Predicate::Any,
            },
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::Sequentially(
                vec![
                    Instruction::May(May {
                        who: Reference::Reg(deckmaste_core::RefId(1)),
                        effect: Arc::new(Instruction::Act(CoreAction::Cast(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            cast_ref,
                            None,
                        ))),
                        if_did: Some(Arc::new(Instruction::Act(CoreAction::ChangeLife(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            LifeOp::Up(Count::Literal(3)),
                        )))),
                        if_not: Some(Arc::new(Instruction::Act(CoreAction::ChangeLife(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            LifeOp::Up(Count::Literal(5)),
                        )))),
                    }),
                    Instruction::Act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Black.into()).into(),
                    )),
                ]
                .into(),
            )
            .into(),
        })],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![parent, source, watcher, cast_spell],
            },
            PlayerConfig { deck: vec![] },
        ],
        seed: 29,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(payer),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let parent = put_in_play(&mut state, payer, "Nested cast parent");
    let source = put_in_play(&mut state, payer, "Nested cast source");
    put_in_play(&mut state, payer, "Nested cast watcher");
    let cast_spell = state.zones.hands[payer.index()]
        .iter()
        .copied()
        .find(|&object| {
            matches!(state.def(object), Card::Normal(face) if face.name.as_ref() == "Nested cast spell")
        })
        .unwrap();
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, source, cast_spell)
}

fn causal_trigger_fixture(
    event: EventFilter,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    causal_trigger_fixture_with_effect(
        event,
        Instruction::Act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Green.into()).into(),
        )),
    )
}

fn causal_trigger_fixture_with_effect(
    event: EventFilter,
    source_effect: Instruction,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    causal_trigger_fixture_with_effect_and_limits(event, source_effect, Arc::from([]))
}

fn causal_trigger_fixture_with_effect_and_limits(
    event: EventFilter,
    source_effect: Instruction,
    limits: Arc<[UseLimit]>,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    causal_trigger_fixture_with_effect_limits_and_trigger(
        event,
        source_effect,
        limits,
        Instruction::Act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Black.into()).into(),
        )),
    )
}

fn causal_trigger_fixture_with_effect_limits_and_trigger(
    event: EventFilter,
    source_effect: Instruction,
    limits: Arc<[UseLimit]>,
    trigger_effect: Instruction,
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
            Instruction::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let source = Arc::new(Card::Normal(CardFace {
        name: "Causal source".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(vec![CostComponent::Tap].into()),
            source_effect,
        )))],
        ..CardFace::default()
    }));
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Causal watcher".into(),
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event,
            condition: None,
            limits,
            effect: trigger_effect.into(),
        })],
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
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, source)
}

fn bare_nonmana_mana_added_fixture() -> (GameState, PlayerId, deckmaste_engine::ObjectId) {
    let payer = PlayerId(0);
    // [CR#605.1a] excludes loyalty abilities from the mana-ability
    // classification, so this untargeted mana producer still uses the stack
    // (Chandra, Torch of Defiance's "+1: Add {R}{R}").
    let mut ordinary = activated_ability(
        Cost(vec![CostComponent::Mana("{0}".parse::<ManaCost>().unwrap())].into()),
        Instruction::Act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Green.into()).into(),
        )),
    );
    ordinary.limits = Arc::from([deckmaste_core::UseLimit::LoyaltyOncePerTurn]);
    let source = Arc::new(Card::Normal(CardFace {
        name: "Loyalty mana-adding ability".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::activated(ordinary)],
        ..CardFace::default()
    }));
    let watcher = Arc::new(Card::Normal(CardFace {
        name: "Bare ManaAdded watcher".into(),
        abilities: vec![Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::ManaAdded {
                what: Predicate::r#type(Type::Land),
                by: Predicate::Any,
            },
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Black.into()).into(),
            ))
            .into(),
        })],
        ..CardFace::default()
    }));
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: vec![source, watcher],
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
    let source = put_in_play(&mut state, payer, "Loyalty mana-adding ability");
    put_in_play(&mut state, payer, "Bare ManaAdded watcher");
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: source,
            ability: 0,
        }],
    }));
    (state, payer, source)
}

fn run_to_payment(state: &mut GameState) -> PaymentPrompt {
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => return prompt,
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
fn nested_optional_payment_inside_a_mana_action_replays_with_its_mana_child() {
    let (mut state, payer, parent, optional_source, helper) = nested_optional_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: optional_source,
            ability: 0,
        }))
        .unwrap();
    let outer_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: outer_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    let optional = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 3);
    assert_eq!(optional.stage, PaymentStage::PrePayment);
    assert!(optional.mana_abilities.contains(&(helper, 0)));
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
            ability: 0,
        }))
        .unwrap();
    let helper_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: helper_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let optional = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 3);

    let pip = optional.outstanding[0].id;
    let mana = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = deckmaste_engine::ManaCoverage::empty();
    coverage.insert(pip, deckmaste_engine::ManaPayment::Floating(mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: pip,
            witness: FulfillmentWitness::CoveredMana,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    let records = state.payment_records().unwrap();
    let [outer_record] = records else {
        panic!("the nested action belongs to one enclosing transaction: {records:?}");
    };
    let nested_retained = outer_record
        .children
        .iter()
        .find(|record| {
            matches!(
                record.command,
                deckmaste_engine::ReplayCommand::ManaAbility { .. }
            )
        })
        .expect("the spent nested mana action is retained with its parent")
        .id;
    let retained = outer_record.id;

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&optional_source));
    assert!(state.objects.obj(optional_source).tapped);
    assert!(state.objects.obj(helper).tapped);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1,
    );
    assert!(matches!(
        state.incidents(),
        [deckmaste_engine::EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records == vec![retained, nested_retained]
    ));
}

#[test]
fn mana_production_facts_include_units_spent_before_the_action_finishes() {
    let (mut state, payer, parent, source) = self_spending_optional_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let source_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: source_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    let optional = run_to_payment(&mut state);
    let pip = optional.outstanding[0].id;
    let produced = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = deckmaste_engine::ManaCoverage::empty();
    coverage.insert(pip, deckmaste_engine::ManaPayment::Floating(produced));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: pip,
            witness: FulfillmentWitness::CoveredMana,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let resumed = run_to_payment(&mut state);

    assert_eq!(resumed.stage, PaymentStage::PrePayment);
    assert!(state.player(payer).mana_pool.is_empty());
    let record = state
        .payment_records()
        .unwrap()
        .iter()
        .find(|record| {
            matches!(
                record.command,
                deckmaste_engine::ReplayCommand::ManaAbility { .. }
            )
        })
        .expect("the completed source action is recorded");
    assert!(record.facts.iter().any(|fact| {
        matches!(
            fact,
            deckmaste_engine::GameEvent::ManaProduced(event)
                if event.produced.iter().any(|unit| unit.id == produced)
        )
    }));
    assert!(record.facts.iter().any(|fact| {
        matches!(
            fact,
            deckmaste_engine::GameEvent::TappedForMana(event)
                if event.produced.iter().any(|unit| unit.id == produced)
        )
    }));
}

#[test]
fn nested_mana_during_an_unsubmitted_action_cost_keeps_its_record_owner() {
    let (mut state, payer, parent, outer, helper) = cost_replacement_nested_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: outer,
            ability: 0,
        }))
        .unwrap();
    let outer_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: outer_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();

    let optional = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 3);
    assert_eq!(optional.stage, PaymentStage::PrePayment);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
            ability: 0,
        }))
        .unwrap();
    let helper_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: helper_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("the nested helper remains a separately reversible mana action");
    };
    assert!(choice.legal.contains(&vec![]));
    let helper_action = choice
        .legal
        .iter()
        .find_map(|set| (set.len() == 1).then(|| set[0]))
        .expect("the helper alone is reversible beneath the barred outer action");
    let mut retained = state.clone();

    retained
        .submit_decision(Decision::ManaReversals(vec![]))
        .unwrap();
    assert!(retained.objects.obj(helper).tapped);
    assert_eq!(
        retained.player(payer).mana_pool.amount(Color::Green.into()),
        2
    );

    state
        .submit_decision(Decision::ManaReversals(vec![helper_action]))
        .unwrap();
    assert!(!state.objects.obj(helper).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
}

#[test]
fn declining_optional_payment_keeps_its_mana_child_separately_reversible() {
    let (mut state, payer, parent, optional_source, helper) = nested_optional_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: optional_source,
            ability: 0,
        }))
        .unwrap();
    let outer_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: outer_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    let optional = run_to_payment(&mut state);
    assert_eq!(state.payment_depth(), 3);
    assert!(optional.mana_abilities.contains(&(helper, 0)));
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
            ability: 0,
        }))
        .unwrap();
    let helper_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: helper_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    run_to_payment(&mut state);
    let records = state.payment_records().unwrap();
    let [outer_record] = records else {
        panic!("the nested action belongs to one enclosing transaction: {records:?}");
    };
    let outer_record_id = outer_record.id;
    assert!(!outer_record.reversal_barriers.is_empty());
    let nested_action = outer_record
        .children
        .iter()
        .find_map(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the optional payment retains its independently activated mana child");

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("decline should expose the nested action despite the outer barrier");
    };
    assert!(choice.legal.contains(&Vec::new()));
    assert!(choice.legal.contains(&vec![nested_action]));

    state
        .submit_decision(Decision::ManaReversals(vec![nested_action]))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.objects.obj(optional_source).tapped);
    assert!(!state.objects.obj(helper).tapped);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        0,
    );
    assert!(matches!(
        state.incidents(),
        [deckmaste_engine::EngineIncident::PaymentDeclined(incident)]
            if incident.crossed_reversal_barrier
                && incident.forced_retained_records == vec![outer_record_id]
    ));
}

#[test]
fn nested_optional_replay_activates_a_mana_source_created_by_the_outer_action() {
    let (mut state, payer, parent, outer) = created_nested_optional_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: outer,
            ability: 0,
        }))
        .unwrap();
    let outer_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: outer_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    let optional = run_to_payment(&mut state);
    let helper = optional
        .mana_abilities
        .iter()
        .find_map(|&(source, ability)| {
            matches!(state.def(source), Card::Normal(face) if face.name.as_ref() == "Nested created helper")
                .then_some((source, ability))
        })
        .expect("the outer mana action creates the optional payment's helper");
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper.0,
            ability: helper.1,
        }))
        .unwrap();
    let helper_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: helper_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let optional = run_to_payment(&mut state);
    let pip = optional.outstanding[0].id;
    let mana = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = deckmaste_engine::ManaCoverage::empty();
    coverage.insert(pip, deckmaste_engine::ManaPayment::Floating(mana));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: pip,
            witness: FulfillmentWitness::CoveredMana,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    let replayed_helper = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| {
            matches!(state.def(object), Card::Normal(face) if face.name.as_ref() == "Nested created helper")
        })
        .expect("replay remints the helper before replaying its nested activation");
    assert!(state.objects.obj(outer).tapped);
    assert!(state.objects.obj(replayed_helper).tapped);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1,
    );
}

#[test]
fn standalone_mana_activation_uses_root_payment_and_resolves_stacklessly() {
    let (mut state, payer, _parent, source) = payment_fixture();
    state.pending = Some(DecisionPointKind::Priority(Priority {
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
        if state.payment_depth() == 0
            && matches!(state.pending, Some(DecisionPointKind::Priority(_)))
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
        if matches!(state.pending, Some(DecisionPointKind::ChooseModes(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::ChooseModes(_))
    ));

    assert!(state.submit_decision(Decision::Modes(vec![1])).is_err());
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::ChooseModes(_))
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
fn modal_mana_profile_routes_an_ordinary_mode_through_the_stack() {
    let (mut state, payer, _parent, source) = modal_payment_fixture();
    state.pending = Some(DecisionPointKind::Priority(Priority {
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
    for _ in 0..20 {
        if matches!(state.pending, Some(DecisionPointKind::ChooseModes(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    state.submit_decision(Decision::Modes(vec![1])).unwrap();
    let prompt = run_to_payment(&mut state);
    assert_eq!(prompt.stage, PaymentStage::Paying);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
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
    assert!(state.objects.obj(source).tapped);
    assert!(state.player(payer).mana_pool.is_empty());
}

#[test]
fn mixed_modal_mana_profile_rechecks_blanket_lockout_after_modes() {
    let (mut state, payer, _parent, source) =
        modal_payment_fixture_with_extras(vec![blanket_activate_lockout_card()]);
    let lockout = put_in_play(&mut state, PlayerId(0), "Split-second-style lockout");
    state.pending = None;
    state.agenda.clear();
    state.agenda.push_front(WorkItem::OpenPriority);
    assert!(matches!(state.step(), StepOutcome::Progress(_)));
    let DecisionPointKind::Priority(prompt) = state.pending.as_ref().unwrap() else {
        panic!("OpenPriority should expose the mixed modal activation")
    };
    let activation = Action::ActivateAbility {
        object: source,
        ability: 0,
    };
    assert!(
        prompt.legal.contains(&activation),
        "a qualifying mana completion keeps the mixed ability offer legal"
    );

    state.submit_decision(Decision::Act(activation)).unwrap();
    for _ in 0..20 {
        if matches!(state.pending, Some(DecisionPointKind::ChooseModes(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    assert!(state.submit_decision(Decision::Modes(vec![1])).is_err());
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::ChooseModes(_))
    ));
    assert!(
        state
            .announcing
            .as_ref()
            .expect("the rejected announcement remains live")
            .chosen_modes
            .is_empty()
    );

    state.submit_decision(Decision::Modes(vec![0])).unwrap();
    let prompt = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    for _ in 0..40 {
        if state.payment_depth() == 0
            && matches!(state.pending, Some(DecisionPointKind::Priority(_)))
        {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    assert!(state.objects.obj(source).tapped);
    assert!(state.stack.is_empty());
    assert!(state.zones.battlefield.contains(&lockout));
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1
    );
}

#[test]
fn mixed_modal_mana_profile_rejects_an_unsatisfiable_ordinary_mode() {
    let missing_targets: Arc<[TargetSpec]> = vec![TargetSpec::Target(
        Quantity::one(),
        Arc::new(deckmaste_core::Region::candidate(
            Predicate::Characteristic(CharacteristicPredicate::Named("Missing target".into())),
        )),
    )]
    .into();
    let missing_target = Instruction::Sequentially(Arc::from([]));
    let (mut state, _payer, _parent, source) = modal_payment_fixture_with_ordinary_mode(
        Vec::new(),
        missing_targets,
        missing_target,
        ManaModeClass {
            adds_mana: false,
            targetless: false,
        },
    );
    state.pending = None;
    state.agenda.clear();
    state.agenda.push_front(WorkItem::OpenPriority);
    assert!(matches!(state.step(), StepOutcome::Progress(_)));
    let DecisionPointKind::Priority(priority) = state.pending.as_ref().unwrap() else {
        panic!("OpenPriority should expose the mixed modal activation")
    };
    assert!(priority.legal.contains(&Action::ActivateAbility {
        object: source,
        ability: 0,
    }));
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: source,
            ability: 0,
        }))
        .unwrap();
    for _ in 0..20 {
        if matches!(state.pending, Some(DecisionPointKind::ChooseModes(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    assert!(state.submit_decision(Decision::Modes(vec![1])).is_err());
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::ChooseModes(_))
    ));
    assert!(
        state
            .announcing
            .as_ref()
            .expect("the rejected mode leaves the announcement retryable")
            .chosen_modes
            .is_empty()
    );

    state.submit_decision(Decision::Modes(vec![0])).unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Paying);
}

#[test]
fn modal_mana_profile_routes_a_qualifying_mode_stacklessly() {
    let (mut state, payer, _parent, source) = modal_payment_fixture();
    state.pending = Some(DecisionPointKind::Priority(Priority {
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
    for _ in 0..20 {
        if matches!(state.pending, Some(DecisionPointKind::ChooseModes(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    state.submit_decision(Decision::Modes(vec![0])).unwrap();
    let prompt = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: prompt.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    for _ in 0..40 {
        if state.payment_depth() == 0
            && matches!(state.pending, Some(DecisionPointKind::Priority(_)))
        {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }

    assert!(state.stack.is_empty());
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1
    );
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
fn triggered_mana_resolution_cast_owns_a_nested_announcement_frame() {
    let (mut state, payer, parent, source, spell) = nested_resolution_cast_trigger_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source,
            ability: 0,
        }))
        .unwrap();
    let source_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: source_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut state).stage, PaymentStage::Ready);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    for _ in 0..80 {
        if matches!(state.pending, Some(DecisionPointKind::YesNo(_))) {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }
    state.submit_decision(Decision::Answer(true)).unwrap();
    let cast_payment = run_to_payment(&mut state);

    assert_eq!(cast_payment.stage, PaymentStage::PrePayment);
    assert_eq!(state.payment_depth(), 3);

    let mut submitted = state.clone();
    let pip = cast_payment.outstanding[0].id;
    let green = submitted
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .find(|unit| unit.kind == Color::Green.into())
        .unwrap()
        .id;
    let mut coverage = deckmaste_engine::ManaCoverage::empty();
    coverage.insert(pip, deckmaste_engine::ManaPayment::Floating(green));
    submitted
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    submitted
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: pip,
            witness: FulfillmentWitness::CoveredMana,
        }))
        .unwrap();
    assert_eq!(run_to_payment(&mut submitted).stage, PaymentStage::Ready);
    submitted
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    let resumed = run_to_payment(&mut submitted);
    assert_eq!(resumed.stage, PaymentStage::PrePayment);
    assert_eq!(submitted.payment_depth(), 1);
    assert!(submitted.stack.iter().any(|entry| entry.id == spell));
    assert_eq!(submitted.player(payer).life, 23);
    assert_eq!(
        submitted
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        0
    );
    assert_eq!(
        submitted
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Black)),
        1
    );

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let resumed = run_to_payment(&mut state);
    assert_eq!(resumed.stage, PaymentStage::PrePayment);
    assert_eq!(state.payment_depth(), 1);
    assert!(state.zones.hands[payer.index()].contains(&spell));
    assert!(state.stack.is_empty());
    assert_eq!(state.player(payer).life, 25);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1
    );
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Black)),
        1
    );
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

#[test]
fn activation_triggered_mana_waits_for_the_source_mana_effect() {
    let event = EventFilter::ManaAbilityActivated {
        what: Predicate::r#type(Type::Land),
        by: Predicate::Any,
    };
    let (mut state, _payer, parent, source) = causal_trigger_fixture(event);
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

    let mut additions = Vec::new();
    for _ in 0..40 {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::ManaAdded(
                event,
            )))) => additions.push(event.mana),
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => break,
            other => panic!("unexpected outcome while resolving mana action: {other:?}"),
        }
    }
    assert_eq!(
        additions,
        vec![Color::Green.into(), Color::Black.into()],
        "the source mana ability finishes before its activation trigger resolves"
    );
}

#[test]
fn mana_added_triggers_wait_for_all_effects_of_the_source_mana_ability() {
    let event = EventFilter::ManaAdded {
        what: Predicate::r#type(Type::Land),
        by: Predicate::Any,
    };
    let source_effect = Instruction::Sequentially(
        vec![
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Blue.into()).into(),
            )),
        ]
        .into(),
    );
    let (mut state, _payer, parent, source) =
        causal_trigger_fixture_with_effect(event, source_effect);
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

    let mut additions = Vec::new();
    for _ in 0..60 {
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Single(GameEvent::ManaAdded(
                event,
            )))) => additions.push(event.mana),
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => break,
            other => panic!("unexpected outcome while resolving mana action: {other:?}"),
        }
    }
    assert_eq!(
        additions,
        vec![
            Color::Green.into(),
            Color::Blue.into(),
            Color::Black.into(),
            Color::Black.into(),
        ],
        "ManaAdded triggers wait until every effect of the causing mana ability finishes"
    );
}

#[test]
fn triggered_mana_use_limit_is_gated_between_causal_firings() {
    let event = EventFilter::ManaAdded {
        what: Predicate::r#type(Type::Land),
        by: Predicate::Any,
    };
    let source_effect = Instruction::Sequentially(
        vec![
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Green.into()).into(),
            )),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Blue.into()).into(),
            )),
        ]
        .into(),
    );
    let (mut state, payer, parent, source) = causal_trigger_fixture_with_effect_and_limits(
        event,
        source_effect,
        vec![UseLimit::OncePerTurn].into(),
    );
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
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
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Black)),
        1,
        "the second causal firing sees the first one's AbilityUsed fact",
    );
}

#[test]
fn triggered_mana_production_facts_include_units_spent_before_finish() {
    let trigger_effect = Instruction::Sequentially(
        vec![
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(Color::Black.into()).into(),
            )),
            Instruction::May(May {
                who: Reference::Reg(deckmaste_core::RefId(1)),
                effect: Arc::new(Instruction::Act(CoreAction::Pay(Cost(
                    vec![CostComponent::Mana("{B}".parse().unwrap())].into(),
                )))),
                if_did: None,
                if_not: None,
            }),
        ]
        .into(),
    );
    let (mut state, payer, parent, source) = causal_trigger_fixture_with_effect_limits_and_trigger(
        EventFilter::ManaAdded {
            what: Predicate::r#type(Type::Land),
            by: Predicate::Any,
        },
        Instruction::Act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Green.into()).into(),
        )),
        Arc::from([]),
        trigger_effect,
    );
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
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
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    let optional = run_to_payment(&mut state);
    let pip = optional.outstanding[0].id;
    let black = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .find(|unit| unit.kind == Color::Black.into())
        .expect("triggered mana was added")
        .id;
    let mut coverage = deckmaste_engine::ManaCoverage::empty();
    coverage.insert(pip, deckmaste_engine::ManaPayment::Floating(black));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: pip,
            witness: FulfillmentWitness::CoveredMana,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Black)),
        0,
    );
    assert!(state.payment_records().unwrap().iter().any(|record| {
        record.facts.iter().any(|fact| {
            matches!(
                fact,
                GameEvent::ManaProduced(event)
                    if event.produced.iter().any(|unit| unit.id == black)
            )
        })
    }));
}

#[test]
fn bare_nonmana_ability_mana_added_trigger_resolves_immediately() {
    let (mut state, payer, source) = bare_nonmana_mana_added_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: source,
            ability: 0,
        }))
        .unwrap();
    let prompt = run_to_payment(&mut state);
    assert_eq!(prompt.stage, PaymentStage::PrePayment);
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
        if state.stack.len() == 1 {
            break;
        }
        assert!(matches!(state.step(), StepOutcome::Progress(_)));
    }
    assert_eq!(state.stack.len(), 1);
    state.agenda.push_front(WorkItem::OpenPriority);
    for _ in 0..100 {
        if state.stack.is_empty()
            && state
                .player(payer)
                .mana_pool
                .amount(deckmaste_core::ColorOrColorless::Color(Color::Black))
                == 1
            && matches!(state.pending, Some(DecisionPointKind::Priority(_)))
        {
            break;
        }
        match state.step() {
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => {
                state.submit_decision(Decision::Act(Action::Pass)).unwrap();
            }
            StepOutcome::Progress(_) => {}
            other => panic!("unexpected stop while resolving ordinary mana addition: {other:?}"),
        }
    }

    assert!(
        state.stack.is_empty(),
        "ordinary ability never resolved: stack={}, pending={:?}, priority={:?}, agenda={:?}, outcome={:?}",
        state.stack.len(),
        state.pending,
        state.turn.priority,
        state.agenda,
        state.outcome
    );
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Green)),
        1
    );
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Color(Color::Black)),
        1,
        "a triggered mana ability caused by a bare ManaAdded fact resolves before priority"
    );
    assert!(state.pending_triggers.is_empty());
}

// ---- cost-block and pinned-magnitude mana cases ----

/// A cost block's first instruction definition in these fixtures. An
/// announcement's register file opens with source(0) and controller(1), so a
/// payment-time decision writes register 2 ([CR#601.2b]).
const PAID: deckmaste_core::DefId = deckmaste_core::DefId(2);

/// [`PAID`] as the register the verb that spends the payment subject reads.
const PAID_REF: deckmaste_core::RefId = deckmaste_core::RefId(2);

/// The register an effect region pins a magnitude in before the verb that
/// uses it runs ([CR#608.2h]) — the "that much" anaphor as a definition.
const AMOUNT: deckmaste_core::DefId = deckmaste_core::DefId(2);

/// [`AMOUNT`] as a numeric register read.
const AMOUNT_REF: deckmaste_core::RefId = deckmaste_core::RefId(2);

/// Krark-Clan Ironworks: a mana ability whose cost sacrifices an artifact the
/// payer controls, activated while the artifact's own ability is announced.
///
/// Re-spelled from the deleted `CostComponent::ChooseAndPay { binder, body }`
/// fixture — `Binder::ChooseOne` is retired, so the choice is its own
/// `CostComponent::Choose` and the sacrifice reads the register it writes
/// ([CR#601.2b]).
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
            Instruction::Sequentially(Arc::from([])),
        ))],
        ..CardFace::default()
    }));
    let artifact = Predicate::And(
        vec![
            Predicate::State(deckmaste_core::StatePredicate::InZone(Zone::Battlefield)),
            Predicate::r#type(Type::Artifact),
            Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(Arc::new(
                Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            ))),
        ]
        .into(),
    );
    let sacrifice = vec![
        CostComponent::Choose(deckmaste_core::Choose {
            dest: PAID,
            by: Reference::Reg(deckmaste_core::RefId(1)),
            quantity: Quantity::one(),
            filter: Arc::new(deckmaste_core::Region::candidate(artifact)),
        }),
        CostComponent::do_action(CoreAction::Sacrifice(
            Reference::Reg(deckmaste_core::RefId(1)),
            Reference::Reg(PAID_REF),
        )),
    ];
    let kci = Arc::new(Card::Normal(CardFace {
        name: "KCI".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(sacrifice.into()),
            Instruction::Act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(2),
                ManaSpec::Specific(deckmaste_core::ColorOrColorless::Colorless).into(),
            )),
        )))],
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
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![Action::ActivateAbility {
            object: parent,
            ability: 0,
        }],
    }));
    (state, payer, parent, kci)
}

/// A mana ability whose resolution pins a magnitude, deals that much damage,
/// then offers an optional mana payment that reads the SAME register back —
/// plus a second mana source whose own resolution pins its own magnitude in
/// the same register of its own activation.
///
/// Re-spelled from the deleted `Count::ThatMuch` fixture: the magnitude
/// anaphor is now a definition written before the damage verb runs
/// ([CR#608.2h]), and the nested source's competing magnitude is what must
/// not reach the outer region.
fn resolution_scope_mana_fixture() -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let outer = Ability::Activated(Arc::new(activated_ability(
        Cost(vec![CostComponent::Tap].into()),
        Instruction::Sequentially(
            vec![
                Instruction::Let(deckmaste_core::Let {
                    dest: AMOUNT,
                    expr: deckmaste_core::Expr::Number(Count::Literal(5)),
                }),
                Instruction::Act(CoreAction::DealDamage(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    Count::Reg(AMOUNT_REF),
                    Reference::Reg(deckmaste_core::RefId(1)),
                )),
                Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::Act(CoreAction::Pay(Cost(
                        vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
                    )))),
                    if_did: Some(Arc::new(Instruction::Act(CoreAction::ChangeLife(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        LifeOp::Up(Count::Reg(AMOUNT_REF)),
                    )))),
                    if_not: None,
                }),
                Instruction::Act(CoreAction::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
            ]
            .into(),
        ),
    )));
    let helper = Arc::new(Card::Normal(CardFace {
        name: "Resolution scope helper".into(),
        types: vec![Type::Land.def()],
        abilities: vec![Ability::Activated(Arc::new(activated_ability(
            Cost(
                vec![CostComponent::do_action(CoreAction::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(Count::Literal(1)),
                ))]
                .into(),
            ),
            Instruction::Sequentially(
                vec![
                    Instruction::Let(deckmaste_core::Let {
                        dest: AMOUNT,
                        expr: deckmaste_core::Expr::Number(Count::Literal(1)),
                    }),
                    Instruction::Act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Reg(AMOUNT_REF),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )),
                ]
                .into(),
            ),
        )))],
        ..CardFace::default()
    }));
    let (mut state, payer, parent, outer) =
        payment_fixture_with_source_ability_and_extras(outer, vec![helper]);
    let helper = put_in_play(&mut state, payer, "Resolution scope helper");
    (state, payer, parent, outer, helper)
}

#[test]
fn nested_mana_resolution_restores_the_containing_that_much_register() {
    let (mut state, payer, parent, outer, helper) = resolution_scope_mana_fixture();
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object: parent,
            ability: 0,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: outer,
            ability: 0,
        }))
        .unwrap();
    let outer_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: outer_cost.outstanding[0].id,
            witness: FulfillmentWitness::Bound,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    run_to_payment(&mut state);
    assert_eq!(
        state.player(payer).life,
        15,
        "the damage verb reads the magnitude the region pinned",
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: helper,
            ability: 0,
        }))
        .unwrap();
    let helper_cost = run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: helper_cost.outstanding[0].id,
            witness: FulfillmentWitness::PayLife,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();

    let optional = run_to_payment(&mut state);
    let pip = optional.outstanding[0].id;
    let unit = state.player(payer).mana_pool.units()[0].id;
    let mut coverage = deckmaste_engine::ManaCoverage::empty();
    coverage.insert(pip, deckmaste_engine::ManaPayment::Floating(unit));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: pip,
            witness: FulfillmentWitness::CoveredMana,
        }))
        .unwrap();
    run_to_payment(&mut state);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    assert_eq!(
        state.player(payer).life,
        19,
        "the helper's own pinned magnitude must not replace the outer region's five",
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
    let choice = kci_prompt.outstanding[0].id;
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: choice,
            witness: FulfillmentWitness::Objects(vec![parent]),
        }))
        .unwrap();
    let sacrifice = run_to_payment(&mut state).outstanding[0].id;
    state
        .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
            iou: sacrifice,
            witness: FulfillmentWitness::Bound,
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
