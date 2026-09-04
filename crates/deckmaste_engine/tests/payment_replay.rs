use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::AggregateOp;
use deckmaste_core::Anchor;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::ChooseSpec;
use deckmaste_core::Color;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Countable;
use deckmaste_core::Destination;
use deckmaste_core::EventFilter;
use deckmaste_core::Instruction;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::May;
use deckmaste_core::Modal;
use deckmaste_core::Mode;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::Predicate;
use deckmaste_core::Projection;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Replacement;
use deckmaste_core::StatValue;
use deckmaste_core::StatePredicate;
use deckmaste_core::StaticSpec;
use deckmaste_core::Supertype;
use deckmaste_core::Token;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::EngineIncident;
use deckmaste_engine::FulfillmentWitness;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::IouKind;
use deckmaste_engine::ManaCoverage;
use deckmaste_engine::ManaPayment;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentPrompt;
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
            effect: Instruction::Sequentially(Arc::from([])).into(),
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
        .find(|&object| state.def(object).primary_face().name.as_ref() == "Replay fixture")
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
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![action],
    }));
    (state, payer, source)
}

fn run_to_payment(state: &mut GameState) -> PaymentPrompt {
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => return prompt,
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
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost,
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::act(CoreAction::AddMana(
                recipient,
                Count::Literal(1),
                ManaSpec::Specific(color.into()).into(),
            ))
            .into(),
        }))],
        ..CardFace::default()
    }))
}

fn suspending_barred_mana_source() -> Arc<Card> {
    let choose = Instruction::Modal(Modal {
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
                effect: Instruction::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: Instruction::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
        ]
        .into(),
    });
    Arc::new(Card::Normal(CardFace {
        name: "Suspending barred mana source".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(Arc::from([])),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::Sequentially(
                vec![
                    Instruction::act(CoreAction::Move(
                        Reference::Reg(deckmaste_core::RefId(0)),
                        Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                        Arc::from([]),
                        Some(Zone::Battlefield),
                    )),
                    choose,
                    Instruction::act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )),
                ]
                .into(),
            )
            .into(),
        }))],
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
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
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
            effect: Instruction::Sequentially(
                vec![
                    Instruction::act(CoreAction::Create {
                        agent: Reference::Reg(deckmaste_core::RefId(1)),
                        count: Count::Literal(1),
                        token: token.into(),
                        riders: Arc::from([]),
                    }),
                    Instruction::act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )),
                ]
                .into(),
            )
            .into(),
        }))],
        ..CardFace::default()
    }))
}

fn token_creating_source_with_a_token_mana_ability() -> Arc<Card> {
    // Re-spelled from `Instruction::With`/`Binder::ChooseOne`: the choice is
    // its own instruction writing register 3 (0 source, 1 controller,
    // 2 announced X), and the mana add follows it.
    let token_ability = Ability::Activated(Arc::new(ActivatedAbility {
        ability_word: None,
        targets: [].into(),
        cost: Cost(vec![CostComponent::Tap].into()),
        from: None,
        window: None,
        condition: None,
        limits: Arc::from([]),
        effect: Instruction::Sequentially(
            vec![
                Instruction::Choose(deckmaste_core::Choose {
                    dest: deckmaste_core::DefId(3),
                    by: Reference::Reg(deckmaste_core::RefId(1)),
                    quantity: Quantity::one(),
                    filter: Arc::new(deckmaste_core::Region::candidate(Predicate::And(
                        vec![
                            Predicate::r#type(Type::Artifact),
                            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                        ]
                        .into(),
                    ))),
                }),
                Instruction::act(CoreAction::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::Specific(Color::Green.into()).into(),
                )),
            ]
            .into(),
        )
        .into(),
    }));
    let token = Token {
        name: Some("Replay mana token".into()),
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: vec![token_ability].into(),
        power: None,
        toughness: None,
    };
    Arc::new(Card::Normal(CardFace {
        name: "Created-source producer".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
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
            effect: Instruction::Sequentially(
                vec![
                    Instruction::act(CoreAction::Create {
                        agent: Reference::Reg(deckmaste_core::RefId(1)),
                        count: Count::Literal(1),
                        token: token.into(),
                        riders: Arc::from([]),
                    }),
                    Instruction::act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )),
                ]
                .into(),
            )
            .into(),
        }))],
        ..CardFace::default()
    }))
}

fn fulfillment_created_mana_source_replacements() -> (Arc<Card>, Arc<Card>) {
    let token_ability = Ability::Activated(Arc::new(ActivatedAbility {
        ability_word: None,
        targets: [].into(),
        cost: Cost(vec![CostComponent::Tap].into()),
        from: None,
        window: None,
        condition: None,
        limits: Arc::from([]),
        effect: Instruction::act(CoreAction::AddMana(
            Reference::Reg(deckmaste_core::RefId(1)),
            Count::Literal(1),
            ManaSpec::Specific(Color::Green.into()).into(),
        ))
        .into(),
    }));
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: token.into(),
                    riders: Arc::from([]),
                }),
                Instruction::act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
            ]
            .into(),
        ),
    };
    let creator = Arc::new(Card::Normal(CardFace {
        name: "Fulfillment source creator".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
        instead: Instruction::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(Instruction::act(CoreAction::Pay(Cost(
                vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
            )))),
            if_did: None,
            if_not: None,
        }),
    };
    let optional = Arc::new(Card::Normal(CardFace {
        name: "Fulfillment source consumer".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
            optional_replacement,
        )))],
        ..CardFace::default()
    }));
    (creator, optional)
}

fn token_creating_then_choosing_barred_mana_source() -> Arc<Card> {
    let token = Token {
        name: Some("Same-record choice token".into()),
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: Arc::from([]),
        power: None,
        toughness: None,
    };
    Arc::new(Card::Normal(CardFace {
        name: "Same-record choice source".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
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
            // Re-spelled from `Instruction::With`/`Binder::ChooseOne`:
            // the choice is its own instruction writing register 3.
            effect: Instruction::Sequentially(
                vec![
                    Instruction::act(CoreAction::Create {
                        agent: Reference::Reg(deckmaste_core::RefId(1)),
                        count: Count::Literal(1),
                        token: token.into(),
                        riders: Arc::from([]),
                    }),
                    Instruction::Choose(deckmaste_core::Choose {
                        dest: deckmaste_core::DefId(3),
                        by: Reference::Reg(deckmaste_core::RefId(1)),
                        quantity: Quantity::one(),
                        filter: Arc::new(deckmaste_core::Region::candidate(Predicate::And(
                            vec![
                                Predicate::Characteristic(CharacteristicPredicate::Named(
                                    "Same-record choice token".into(),
                                )),
                                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                            ]
                            .into(),
                        ))),
                    }),
                    Instruction::act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )),
                ]
                .into(),
            )
            .into(),
        }))],
        ..CardFace::default()
    }))
}

fn token_revealing_reversible_mana_source() -> Arc<Card> {
    let token = Token {
        name: Some("Observed transient token".into()),
        color_indicator: Arc::from([]),
        supertypes: Arc::from([]),
        types: vec![Type::Artifact.def()].into(),
        subtypes: Arc::from([]),
        abilities: Arc::from([]),
        power: None,
        toughness: None,
    };
    Arc::new(Card::Normal(CardFace {
        name: "Transient token source".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(vec![CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            // Re-spelled from `Instruction::With`/`Binder::TheRef`: the
            // pinned read is a `Let` writing register 3, and the reveal
            // reads that register instead of `That(Token)`.
            effect: Instruction::Sequentially(
                vec![
                    Instruction::act(CoreAction::Create {
                        agent: Reference::Reg(deckmaste_core::RefId(1)),
                        count: Count::Literal(1),
                        token: token.into(),
                        riders: Arc::from([]),
                    }),
                    Instruction::Let(deckmaste_core::Let {
                        dest: deckmaste_core::DefId(3),
                        expr: deckmaste_core::Expr::Object(Reference::Single(Arc::new(
                            deckmaste_core::Selection::SelectAll(Arc::new(
                                deckmaste_core::Region::candidate(Predicate::Characteristic(
                                    CharacteristicPredicate::Named(
                                        "Observed transient token".into(),
                                    ),
                                )),
                            )),
                        ))),
                    }),
                    Instruction::act(CoreAction::Reveal {
                        what: Reference::Reg(deckmaste_core::RefId(3)),
                        to: None,
                    }),
                    Instruction::act(CoreAction::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaSpec::Specific(Color::Green.into()).into(),
                    )),
                ]
                .into(),
            )
            .into(),
        }))],
        ..CardFace::default()
    }))
}

/// Krark-Clan Ironworks: "Sacrifice an artifact: Add {C}{C}."
///
/// Re-spelled from `CostComponent::ChooseAndPay`/`Binder::ChooseOne`
/// ([CR#601.2b]): the payment-time choice is its own cost instruction writing
/// register 3 (0 source, 1 controller, 2 announced X), and the sacrifice
/// spends that register.
fn krark_clan_ironworks() -> Arc<Card> {
    let chosen = deckmaste_core::DefId(3);
    let sacrifice_artifact = vec![
        CostComponent::Choose(deckmaste_core::Choose {
            dest: chosen,
            by: Reference::Reg(deckmaste_core::RefId(1)),
            quantity: Quantity::one(),
            filter: Arc::new(deckmaste_core::Region::candidate(Predicate::And(
                vec![
                    Predicate::r#type(Type::Artifact),
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                ]
                .into(),
            ))),
        }),
        CostComponent::do_action(CoreAction::Sacrifice(
            Reference::Reg(deckmaste_core::RefId(1)),
            Reference::Reg(chosen.into()),
        )),
    ];
    Arc::new(Card::Normal(CardFace {
        name: "Krark-Clan Ironworks".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(sacrifice_artifact.into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(2),
                ManaSpec::Specific(deckmaste_core::ColorOrColorless::Colorless).into(),
            ))
            .into(),
        }))],
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Reveal {
                    what: Reference::Reg(deckmaste_core::RefId(2)),
                    to: None,
                }),
                Instruction::act(CoreAction::Move(
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
            instead,
        )))],
        ..CardFace::default()
    }))
}

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
    let effect = Instruction::act(CoreAction::AddMana(
        Reference::Reg(deckmaste_core::RefId(1)),
        Count::Literal(1),
        ManaSpec::AmongColorsOf(sole_eligible_legend).into(),
    ));
    Arc::new(Card::Normal(CardFace {
        name: "Mox Amber".into(),
        mana_cost: "{0}".parse().unwrap(),
        supertypes: vec![Supertype::Legendary],
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(vec![CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: effect.into(),
        }))],
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
            targets: [].into(),
            cost: Cost(vec![CostComponent::Mana("{2}{G}".parse::<ManaCost>().unwrap())].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::Sequentially(Arc::from([])).into(),
        })],
        ..CardFace::default()
    }))
}

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
        abilities: vec![Ability::r#static(StaticSpec::Modify(
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

fn mana_cylix_fixture() -> Arc<Card> {
    Arc::new(Card::Normal(CardFace {
        name: "Mana Cylix".into(),
        types: vec![Type::Artifact.def()],
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
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
            effect: Instruction::act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::AnyColor.into(),
            ))
            .into(),
        }))],
        ..CardFace::default()
    }))
}

fn bighorner_rancher_fixture() -> Arc<Card> {
    let controlled_creature = Predicate::And(
        vec![
            Predicate::creature(),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::Reg(deckmaste_core::RefId(1)),
            )))),
        ]
        .into(),
    );
    // Re-spelled from `Reference::It`: a projection's per-element read is a
    // candidate region whose element is register 0.
    let greatest_power = Count::Aggregate(
        AggregateOp::MaxOf,
        Projection {
            of: Countable::Objects(Arc::new(deckmaste_core::Region::candidate(
                controlled_creature,
            ))),
            by: Arc::new(deckmaste_core::Region::candidate(Count::StatOf(
                Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Stat::Power,
            ))),
        },
    );
    Arc::new(Card::Normal(CardFace {
        name: "Bighorner Rancher".into(),
        mana_cost: "{4}{G}".parse().unwrap(),
        types: vec![Type::Creature.def()],
        power: Some(StatValue::Number(2)),
        toughness: Some(StatValue::Number(5)),
        abilities: vec![Ability::Activated(Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(vec![CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                greatest_power,
                ManaSpec::Specific(Color::Green.into()).into(),
            ))
            .into(),
        }))],
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
        .find(|&object| state.def(object).primary_face().name.as_ref() == name)
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
        Some(DecisionPointKind::Payment(prompt)) => prompt,
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
    assert_eq!(
        state.pending,
        Some(DecisionPointKind::Payment(before_prompt))
    );
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
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::Priority(_))
    ));
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records.len() == 1
                && incident.crossed_reversal_barrier
    ));
}

/// Re-spelled from `CostComponent::ChooseAndPay`/`Binder::ChooseOne`
/// ([CR#601.2b]): each payment binder is now a choice instruction writing a
/// register plus the move that spends it, so every step is two IOUs. An
/// activated region declares source(0), controller(1) and announced X(2), so
/// the choices define registers 3 upward.
fn move_chosen(
    dest: deckmaste_core::DefId,
    filter: Predicate,
    destination: Zone,
    from: Zone,
) -> Vec<CostComponent> {
    vec![
        CostComponent::Choose(deckmaste_core::Choose {
            dest,
            by: Reference::Reg(deckmaste_core::RefId(1)),
            quantity: Quantity::one(),
            filter: Arc::new(deckmaste_core::Region::candidate(filter)),
        }),
        CostComponent::do_action(CoreAction::Move(
            Reference::Reg(dest.into()),
            Destination::Zone(destination),
            Arc::from([]),
            Some(from),
        )),
    ]
}

#[test]
fn unreplayable_dependency_rejects_without_repair_or_mutation() {
    let battlefield = Predicate::State(StatePredicate::InZone(Zone::Battlefield));
    let graveyard = Predicate::State(StatePredicate::InZone(Zone::Graveyard));
    let resource = Arc::new(Card::Normal(CardFace {
        name: "Replay resource".into(),
        ..CardFace::default()
    }));
    let mut cost = move_chosen(
        deckmaste_core::DefId(3),
        battlefield,
        Zone::Graveyard,
        Zone::Battlefield,
    );
    cost.extend(move_chosen(
        deckmaste_core::DefId(4),
        graveyard,
        Zone::Exile,
        Zone::Graveyard,
    ));
    let (mut state, payer, source) = activation_fixture_with_extras(cost, vec![resource]);
    let resource = state.zones.hands[payer.index()][0];
    state.zones.hands[payer.index()].clear();
    state.objects.obj_mut(resource).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(resource);

    let prompt = announce(&mut state, source);
    let first_choice = prompt.outstanding[0].id;
    let first_move = prompt.outstanding[1].id;
    let second_choice = prompt.outstanding[2].id;
    let second_move = prompt.outstanding[3].id;
    fulfill(
        &mut state,
        first_choice,
        FulfillmentWitness::Objects(vec![resource]),
    );
    fulfill(&mut state, first_move, FulfillmentWitness::Bound);
    let graveyard_resource = state.zones.graveyards[payer.index()][0];
    fulfill(
        &mut state,
        second_choice,
        FulfillmentWitness::Objects(vec![graveyard_resource]),
    );
    fulfill(&mut state, second_move, FulfillmentWitness::Bound);
    assert!(state.zones.graveyards[payer.index()].is_empty());
    assert_eq!(state.zones.exile.len(), 1);

    let before_exile = state.zones.exile.clone();
    let before_prompt = state.pending.clone();
    let before_records = state.payment_records().unwrap().to_vec();
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(
                first_move,
            )))
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
            where_x: None,
            targets: [].into(),
            event: EventFilter::LifeLost {
                who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                amount: None,
            },
            from: None,
            condition: None,
            limits: Arc::from([]),
            effect: Instruction::Sequentially(Arc::from([])).into(),
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
fn replay_rebinds_a_retained_card_after_an_earlier_zone_remint_is_omitted() {
    let battlefield = Predicate::State(StatePredicate::InZone(Zone::Battlefield));
    let graveyard = Predicate::State(StatePredicate::InZone(Zone::Graveyard));
    let first_card = Arc::new(Card::Normal(CardFace {
        name: "First replay card".into(),
        ..CardFace::default()
    }));
    let retained_card = Arc::new(Card::Normal(CardFace {
        name: "Retained replay card".into(),
        ..CardFace::default()
    }));
    let mut cost = move_chosen(
        deckmaste_core::DefId(3),
        battlefield.clone(),
        Zone::Graveyard,
        Zone::Battlefield,
    );
    cost.extend(move_chosen(
        deckmaste_core::DefId(4),
        battlefield,
        Zone::Graveyard,
        Zone::Battlefield,
    ));
    cost.extend(move_chosen(
        deckmaste_core::DefId(5),
        graveyard,
        Zone::Exile,
        Zone::Graveyard,
    ));
    let (mut state, payer, source) =
        activation_fixture_with_extras(cost, vec![first_card, retained_card]);
    let first_card = put_named_in_play(&mut state, payer, "First replay card");
    let retained_card = put_named_in_play(&mut state, payer, "Retained replay card");
    let prompt = announce(&mut state, source);
    let first_choice = prompt.outstanding[0].id;
    let first_move = prompt.outstanding[1].id;
    let second_choice = prompt.outstanding[2].id;
    let second_move = prompt.outstanding[3].id;
    let third_choice = prompt.outstanding[4].id;
    let third_move = prompt.outstanding[5].id;

    fulfill(
        &mut state,
        first_choice,
        FulfillmentWitness::Objects(vec![first_card]),
    );
    fulfill(&mut state, first_move, FulfillmentWitness::Bound);
    fulfill(
        &mut state,
        second_choice,
        FulfillmentWitness::Objects(vec![retained_card]),
    );
    fulfill(&mut state, second_move, FulfillmentWitness::Bound);
    let retained_in_graveyard = state.zones.graveyards[payer.index()]
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == "Retained replay card",
            other => other.primary_face().name.as_ref() == "Retained replay card",
        })
        .unwrap();
    fulfill(
        &mut state,
        third_choice,
        FulfillmentWitness::Objects(vec![retained_in_graveyard]),
    );
    fulfill(&mut state, third_move, FulfillmentWitness::Bound);

    state
        .submit_decision(Decision::Payment(PaymentCommand::RescindFulfillment(
            first_move,
        )))
        .unwrap();

    assert!(state.zones.battlefield.contains(&first_card));
    assert_eq!(state.zones.exile.len(), 1);
    let prompt = match state.pending.as_ref() {
        Some(DecisionPointKind::Payment(prompt)) => prompt,
        other => panic!("rescind should return to payment, got {other:?}"),
    };
    assert_eq!(
        prompt.fulfilled,
        vec![
            first_choice,
            second_choice,
            second_move,
            third_choice,
            third_move
        ]
    );
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

    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::Priority(_))
    ));
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
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
        Some(DecisionPointKind::Payment(prompt)) => prompt.clone(),
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

    let DecisionPointKind::Payment(prompt) = state.pending.as_ref().unwrap() else {
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
        Some(DecisionPointKind::Payment(prompt)) => prompt.clone(),
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
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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

    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
    assert!(matches!(state.pending, Some(DecisionPointKind::Payment(_))));
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
        Some(DecisionPointKind::Payment(prompt)) => prompt.clone(),
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
        instead: Instruction::Sequentially(Arc::from([])),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Double replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![
            Ability::r#static(StaticSpec::Replacement(Arc::new(replacement.clone()))),
            Ability::r#static(StaticSpec::Replacement(Arc::new(replacement))),
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseReplacement(_)) => break,
            other => panic!("expected replacement choice, got {other:?}"),
        }
    }

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&source));
    assert!(matches!(
        state.pending,
        Some(DecisionPointKind::Priority(_))
    ));
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
        instead: Instruction::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(Instruction::act(CoreAction::Pay(Cost(
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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

    let DecisionPointKind::Payment(rebuilt) = state.pending.as_ref().unwrap() else {
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
        instead: Instruction::May(May {
            who: Reference::Reg(deckmaste_core::RefId(1)),
            effect: Arc::new(Instruction::act(CoreAction::Pay(Cost(
                vec![CostComponent::Mana("{G}".parse().unwrap())].into(),
            )))),
            if_did: None,
            if_not: None,
        }),
    };
    let shield = Arc::new(Card::Normal(CardFace {
        name: "Optional mana replacement".into(),
        types: vec![Type::Enchantment.def()],
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
                Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::act(CoreAction::Pay(Cost(
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
    let modal = Instruction::Modal(Modal {
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
                effect: Instruction::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: Instruction::Sequentially(Arc::from([])).into(),
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Shuffle(deckmaste_core::Selection::LibraryOf(
                    Reference::Reg(deckmaste_core::RefId(1)),
                ))),
                Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::act(CoreAction::Pay(Cost(
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseModes(_)) => break,
            other => panic!("the fulfillment should suspend after Optional decline, got {other:?}"),
        }
    }
    let helper_action = deckmaste_engine::ManaActionId(0);
    let filter_action = deckmaste_engine::ManaActionId(1);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
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
    let modal = Instruction::Modal(Modal {
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
                effect: Instruction::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: Instruction::Sequentially(Arc::from([])).into(),
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Move(
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseModes(_)) => break,
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: token.into(),
                    riders: Arc::from([]),
                }),
                Instruction::act(CoreAction::Move(
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
                Instruction::act(CoreAction::Move(
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
    let modal = Instruction::Modal(Modal {
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
                effect: Instruction::Sequentially(Arc::from([])).into(),
                cost: deckmaste_core::Cost::default(),
            },
            Mode {
                targets: [].into(),
                effect: Instruction::Sequentially(Arc::from([])).into(),
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
        instead: Instruction::Sequentially(
            vec![
                Instruction::act(CoreAction::Reveal {
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
        abilities: vec![Ability::r#static(StaticSpec::Replacement(Arc::new(
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseModes(_)) => break,
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
fn retained_later_mana_action_rebinds_its_created_source_and_fact_trace() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{3}".parse::<ManaCost>().unwrap())],
        vec![token_creating_source_with_a_token_mana_ability()],
    );
    let producer = put_named_in_play(&mut state, payer, "Created-source producer");
    announce(&mut state, parent);
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: producer,
            ability: 0,
        }))
        .unwrap();
    let producer_payment = run_to_payment(&mut state);
    fulfill(
        &mut state,
        producer_payment.outstanding[0].id,
        FulfillmentWitness::Bound,
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(&mut state);

    let token = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == "Replay mana token",
            _ => false,
        })
        .expect("the retained producer creates its mana-source token");
    state
        .submit_decision(Decision::Payment(PaymentCommand::ActivateManaAbility {
            source: token,
            ability: 0,
        }))
        .unwrap();
    let token_payment = run_to_payment(&mut state);
    fulfill(
        &mut state,
        token_payment.outstanding[0].id,
        FulfillmentWitness::Bound,
    );
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseObjects(_)) => break,
            other => panic!("the created mana source should choose an artifact, got {other:?}"),
        }
    }
    state
        .submit_decision(Decision::Chosen(vec![token]))
        .unwrap();
    run_to_payment(&mut state);
    let token_action = state
        .payment_records()
        .unwrap()
        .last()
        .and_then(|record| match record.command {
            deckmaste_engine::ReplayCommand::ManaAbility { action, .. } => Some(action),
            deckmaste_engine::ReplayCommand::Fulfill { .. } => None,
        })
        .expect("the chosen-object mana action completes");

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("the created token's reversible action should expose a choice");
    };
    assert!(choice.legal.contains(&vec![token_action]));
    assert!(
        choice.legal.contains(&Vec::new()),
        "retaining both actions should replay exactly: {:?}",
        choice.legal,
    );
    state
        .submit_decision(Decision::ManaReversals(Vec::new()))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    let replayed_token = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| match state.def(object) {
            Card::Normal(face) => face.name.as_ref() == "Replay mana token",
            _ => false,
        })
        .expect("the replay remints the retained producer's token");
    assert!(state.objects.obj(replayed_token).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 2);
}

#[test]
fn retained_mana_action_rebinds_a_later_decision_to_its_own_created_object() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{G}".parse::<ManaCost>().unwrap())],
        vec![token_creating_then_choosing_barred_mana_source()],
    );
    let source = put_named_in_play(&mut state, payer, "Same-record choice source");
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
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseObjects(_)) => break,
            other => panic!("the mana action should choose its created token, got {other:?}"),
        }
    }
    let token = state
        .zones
        .battlefield
        .iter()
        .copied()
        .find(|&object| {
            matches!(state.def(object), Card::Normal(face) if face.name.as_ref() == "Same-record choice token")
        })
        .unwrap();
    state
        .submit_decision(Decision::Chosen(vec![token]))
        .unwrap();
    run_to_payment(&mut state);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert!(state.zones.battlefield.iter().any(|&object| {
        matches!(state.def(object), Card::Normal(face) if face.name.as_ref() == "Same-record choice token")
    }));
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
}

#[test]
fn declining_can_reverse_the_creator_of_an_observed_transient_token() {
    let (mut state, payer, parent) = activation_fixture_with_extras(
        vec![CostComponent::Mana("{2}".parse::<ManaCost>().unwrap())],
        vec![token_revealing_reversible_mana_source()],
    );
    let source = put_named_in_play(&mut state, payer, "Transient token source");
    announce(&mut state, parent);
    let action = submit_tap_mana_action(&mut state, source);
    assert!(state.zones.battlefield.iter().any(|&object| {
        matches!(state.def(object), Card::Normal(face) if face.name.as_ref() == "Observed transient token")
    }));

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("the observation-only action remains physically reversible");
    };
    assert!(choice.legal.contains(&vec![action]));
    state
        .submit_decision(Decision::ManaReversals(vec![action]))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(!state.objects.obj(source).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    assert!(!state.zones.battlefield.iter().any(|&object| {
        matches!(state.def(object), Card::Normal(face) if face.name.as_ref() == "Observed transient token")
    }));
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseModes(_)) => break,
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
    // [CR#601.2b]: the payment-time choice writes the register, then the
    // sacrifice spends it — two IOUs where the binder used to be one.
    let choose = choose_iou(&child);
    let child = fulfill(&mut state, choose, FulfillmentWitness::Objects(vec![kci]));
    fulfill(&mut state, act_iou(&child), FulfillmentWitness::Bound);
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

fn mox_kci_payment_fixture(
    under_wheel: bool,
) -> (
    GameState,
    PlayerId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
    deckmaste_engine::ObjectId,
) {
    let payer = PlayerId(0);
    let mut deck = vec![
        colored_legendary_artifact(),
        mox_amber_fixture(),
        krark_clan_ironworks(),
    ];
    if under_wheel {
        deck.push(wheel_of_sun_and_moon());
    }
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
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
    if under_wheel {
        put_named_in_play(&mut state, payer, "Wheel of Sun and Moon");
    }
    let action = Action::ActivateAbility {
        object: legend,
        ability: 0,
    };
    state.turn.priority = Some(PriorityRound {
        holder: payer,
        consecutive_passes: 0,
    });
    state.pending = Some(DecisionPointKind::Priority(Priority {
        player: payer,
        legal: vec![action.clone()],
    }));
    state.submit_decision(Decision::Act(action)).unwrap();
    assert_eq!(
        run_to_payment(&mut state).stage,
        deckmaste_engine::PaymentStage::PrePayment
    );
    (state, payer, legend, mox, kci)
}

/// The outstanding payment-time choice ([CR#601.2b]) — the register writer a
/// re-spelled cost binder becomes.
fn choose_iou(prompt: &PaymentPrompt) -> deckmaste_engine::IouId {
    prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Choose(_)))
        .expect("a payment-time choice IOU")
        .id
}

/// The first outstanding paying action ([CR#601.2h]) — the verb that spends
/// the register a preceding choice wrote.
fn act_iou(prompt: &PaymentPrompt) -> deckmaste_engine::IouId {
    prompt
        .outstanding
        .iter()
        .find(|iou| matches!(iou.kind, IouKind::Act { .. }))
        .expect("a paying-action IOU")
        .id
}

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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaColor(choice)) => {
                let selected = color.expect("a colorless Mox state must not open a choice");
                assert!(choice.options.contains(&selected.into()));
                state
                    .submit_decision(Decision::ManaColor(selected.into()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseObjects(_)) => {
                panic!("Mox chooses a color, never a legendary permanent")
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => break,
            other => panic!("expected Mox to resume parent payment, got {other:?}"),
        }
    }
    completed_mana_action(state)
}

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
    // [CR#601.2b]: the payment-time choice writes the register, then the
    // sacrifice spends it — two IOUs where the binder used to be one.
    let child = fulfill(
        state,
        choose_iou(&child),
        FulfillmentWitness::Objects(vec![artifact]),
    );
    fulfill(state, act_iou(&child), FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    run_to_payment(state);
    completed_mana_action(state)
}

#[test]
fn kci_before_mox_removes_the_only_eligible_color_so_mox_adds_nothing_without_a_choice() {
    let (mut state, payer, legend, mox, kci) = mox_kci_payment_fixture(false);

    submit_kci_action(&mut state, kci, legend);
    assert!(!state.zones.battlefield.contains(&legend));
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Colorless),
        2
    );

    submit_mox_action(&mut state, mox, None);

    assert!(state.objects.obj(mox).tapped);
    assert_eq!(state.player(payer).mana_pool.units().len(), 2);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    assert!(matches!(state.pending, Some(DecisionPointKind::Payment(_))));
}

#[test]
fn declining_after_mox_then_kci_restores_the_same_legend_checkpoint() {
    let (mut state, payer, legend, mox, kci) = mox_kci_payment_fixture(false);

    let mox_action = submit_mox_action(&mut state, mox, Some(Color::Green));
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
    let kci_action = submit_kci_action(&mut state, kci, legend);
    assert!(!state.zones.battlefield.contains(&legend));
    assert_eq!(state.player(payer).mana_pool.units().len(), 3);

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("declining should expose both independent mana actions");
    };
    assert!(choice.legal.contains(&vec![mox_action, kci_action]));
    state
        .submit_decision(Decision::ManaReversals(vec![mox_action, kci_action]))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(state.zones.battlefield.contains(&legend));
    assert_eq!(state.objects.obj(legend).zone, Some(Zone::Battlefield));
    assert!(!state.objects.obj(mox).tapped);
    assert_eq!(state.player(payer).mana_pool.units().len(), 0);
}

#[test]
fn wheel_forces_mox_then_kci_sacrifice_and_kci_mana_to_remain() {
    let (mut state, payer, legend, mox, kci) = mox_kci_payment_fixture(true);

    let mox_action = submit_mox_action(&mut state, mox, Some(Color::Green));
    let kci_action = submit_kci_action(&mut state, kci, legend);
    let records = state.payment_records().unwrap();
    let kci_record = records
        .iter()
        .find(|record| {
            matches!(
                record.command,
                deckmaste_engine::ReplayCommand::ManaAbility { action, .. }
                    if action == kci_action
            )
        })
        .unwrap();
    assert!(
        kci_record
            .reversal_barriers
            .contains(&ReversalBarrier::MovedToLibrary)
    );
    let kci_record_id = kci_record.id;

    state
        .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
        .unwrap();
    let DecisionPointKind::ChooseManaReversals(choice) = state.pending.as_ref().unwrap() else {
        panic!("declining should still expose the independent Mox reversal");
    };
    assert!(choice.legal.contains(&vec![mox_action]));
    assert!(!choice.legal.contains(&vec![kci_action]));
    assert!(!choice.legal.contains(&vec![mox_action, kci_action]));
    state
        .submit_decision(Decision::ManaReversals(vec![mox_action]))
        .unwrap();

    assert_eq!(state.payment_depth(), 0);
    assert!(!state.zones.battlefield.contains(&legend));
    assert_eq!(state.zones.libraries[payer.index()].len(), 1);
    assert!(!state.objects.obj(mox).tapped);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 0);
    assert_eq!(
        state
            .player(payer)
            .mana_pool
            .amount(deckmaste_core::ColorOrColorless::Colorless),
        2
    );
    assert!(matches!(
        state.incidents(),
        [EngineIncident::PaymentDeclined(incident)]
            if incident.forced_retained_records == vec![kci_record_id]
    ));
}

#[test]
fn mox_then_kci_can_pay_for_the_sacrificed_colored_legends_ability() {
    let (mut state, payer, legend, mox, kci) = mox_kci_payment_fixture(false);

    submit_mox_action(&mut state, mox, Some(Color::Green));
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 1);
    submit_kci_action(&mut state, kci, legend);
    let prompt = match state.pending.as_ref() {
        Some(DecisionPointKind::Payment(prompt)) => prompt.clone(),
        other => panic!("KCI should resume parent payment, got {other:?}"),
    };
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
    let omnath = put_named_in_play(&mut state, payer, "Omnath, Locus of Mana");
    let cylix = put_named_in_play(&mut state, payer, "Mana Cylix");
    let rancher = put_named_in_play(&mut state, payer, "Bighorner Rancher");
    state.player_mut(payer).mana_pool.add(
        Color::Green.into(),
        3,
        deckmaste_engine::ManaProvenance::default(),
    );
    assert_eq!(state.layers().power(omnath), Some(4));
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
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaColor(_)) => {
                state
                    .submit_decision(Decision::ManaColor(Color::Black.into()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => break,
            other => panic!("expected Cylix to resume parent payment, got {other:?}"),
        }
    }
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 2);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Black.into()), 1);
    assert_eq!(state.layers().power(omnath), Some(3));

    submit_tap_mana_action(&mut state, rancher);
    let prompt = match state.pending.as_ref() {
        Some(DecisionPointKind::Payment(prompt)) => prompt.clone(),
        other => panic!("Rancher should resume parent payment, got {other:?}"),
    };
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 5);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Black.into()), 1);
    assert_eq!(state.layers().power(omnath), Some(6));

    let black = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .find(|unit| unit.kind == Color::Black.into())
        .unwrap()
        .id;
    let mut green = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .filter(|unit| unit.kind == Color::Green.into())
        .map(|unit| unit.id);
    let mut incomplete = ManaCoverage::empty();
    let mut mapped_generics = 0;
    for iou in &prompt.outstanding {
        let mana = match iou.kind {
            IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(Color::Black)) => black,
            IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(Color::Green)) => {
                green.next().unwrap()
            }
            IouKind::ManaPip(deckmaste_engine::ManaPip::Generic) if mapped_generics < 4 => {
                mapped_generics += 1;
                green.next().unwrap()
            }
            IouKind::ManaPip(deckmaste_engine::ManaPip::Generic) => continue,
            ref other => panic!("unexpected IOU {other:?}"),
        };
        incomplete.insert(iou.id, ManaPayment::Floating(mana));
    }
    assert_eq!(mapped_generics, 4, "exactly one generic pip is uncovered");
    assert!(
        state
            .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(incomplete,)))
            .is_err()
    );
    assert_eq!(state.player(payer).mana_pool.units().len(), 6);
    assert_eq!(state.layers().power(omnath), Some(6));
}

#[test]
fn rancher_before_cylix_produces_exact_coverage_and_omnath_tracks_each_green_spend() {
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
    let omnath = put_named_in_play(&mut state, payer, "Omnath, Locus of Mana");
    let cylix = put_named_in_play(&mut state, payer, "Mana Cylix");
    let rancher = put_named_in_play(&mut state, payer, "Bighorner Rancher");
    state.player_mut(payer).mana_pool.add(
        Color::Green.into(),
        3,
        deckmaste_engine::ManaProvenance::default(),
    );
    assert_eq!(state.layers().power(omnath), Some(4));
    announce(&mut state, parent);

    submit_tap_mana_action(&mut state, rancher);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 7);
    assert_eq!(state.layers().power(omnath), Some(8));

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
    let spent_green = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .find(|unit| unit.kind == Color::Green.into())
        .unwrap()
        .id;
    let mut child_coverage = ManaCoverage::empty();
    child_coverage.insert(pip, ManaPayment::Floating(spent_green));
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(
            child_coverage,
        )))
        .unwrap();
    fulfill(&mut state, pip, FulfillmentWitness::CoveredMana);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 6);
    assert_eq!(state.layers().power(omnath), Some(7));
    fulfill(&mut state, tap, FulfillmentWitness::Bound);
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    loop {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaColor(_)) => {
                state
                    .submit_decision(Decision::ManaColor(Color::Black.into()))
                    .unwrap();
            }
            StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => break,
            other => panic!("expected Cylix to resume parent payment, got {other:?}"),
        }
    }
    assert_eq!(state.player(payer).mana_pool.amount(Color::Green.into()), 6);
    assert_eq!(state.player(payer).mana_pool.amount(Color::Black.into()), 1);
    assert_eq!(state.layers().power(omnath), Some(7));

    let prompt = match state.pending.as_ref() {
        Some(DecisionPointKind::Payment(prompt)) => prompt.clone(),
        other => panic!("Cylix should resume parent payment, got {other:?}"),
    };
    let green_ids: Vec<_> = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .filter(|unit| unit.kind == Color::Green.into())
        .map(|unit| unit.id)
        .collect();
    let black = state
        .player(payer)
        .mana_pool
        .units()
        .iter()
        .find(|unit| unit.kind == Color::Black.into())
        .unwrap()
        .id;
    let mut green = green_ids.into_iter();
    let mut coverage = ManaCoverage::empty();
    let mut spend_plan = Vec::new();
    for iou in &prompt.outstanding {
        let (mana, spends_green) = match iou.kind {
            IouKind::ManaPip(deckmaste_engine::ManaPip::Colored(Color::Black)) => (black, false),
            IouKind::ManaPip(
                deckmaste_engine::ManaPip::Colored(Color::Green)
                | deckmaste_engine::ManaPip::Generic,
            ) => (green.next().unwrap(), true),
            ref other => panic!("unexpected IOU {other:?}"),
        };
        coverage.insert(iou.id, ManaPayment::Floating(mana));
        spend_plan.push((iou.id, spends_green));
    }
    state
        .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(coverage)))
        .unwrap();

    let mut greens_left = 6;
    for (iou, spends_green) in spend_plan {
        fulfill(&mut state, iou, FulfillmentWitness::CoveredMana);
        if spends_green {
            greens_left -= 1;
        }
        assert_eq!(
            state.player(payer).mana_pool.amount(Color::Green.into()),
            greens_left
        );
        assert_eq!(
            state.layers().power(omnath),
            Some(i32::try_from(greens_left + 1).unwrap())
        );
    }
    state
        .submit_decision(Decision::Payment(PaymentCommand::SubmitPayment))
        .unwrap();
    assert_eq!(state.payment_depth(), 0);
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
    assert!(matches!(state.pending, Some(DecisionPointKind::Payment(_))));
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
