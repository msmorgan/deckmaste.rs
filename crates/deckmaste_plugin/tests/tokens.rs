//! Integration tests for the builtin token files: Treasure, Clue, Food, Gold,
//! Blood, and Vibranium.
//!
//! Each token is loaded via `Plugin::token(name)` (macro-aware reader) and
//! compared to the expected Rust value. A validate-level assertion confirms
//! that `validate_plugin` reports zero failures when tokens are included.

use std::path::Path;
use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::DefId;
use deckmaste_core::Instruction;
use deckmaste_core::Kind;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Param;
use deckmaste_core::Provenance;
use deckmaste_core::Reference;
use deckmaste_core::Region;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::Type;
use deckmaste_lowering::Lower;
use deckmaste_plugin::plugin::Plugin;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

/// `Mana([Generic(2)])` cost component.
fn mana_2() -> CostComponent {
    CostComponent::Mana(ManaCost::from(Arc::<[ManaSymbol]>::from(vec![
        ManaSymbol::Simple(SimpleManaSymbol::Generic(2)),
    ])))
}

/// `SacrificeThis` — a `CostComponent` macro invocation whose BODY is
/// `Do(Sacrifice(This))`. `lower` erases invocation provenance (spec §12),
/// so the loaded token carries the body directly, not a remembered
/// `Expanded` wrapper.
fn sacrifice_this() -> CostComponent {
    CostComponent::do_action(Action::Sacrifice(
        Reference::Reg(deckmaste_core::RefId(1)),
        Reference::Reg(deckmaste_core::RefId(0)),
    ))
}

fn mana_ability(ability: ActivatedAbility) -> Ability {
    Ability::Activated(Arc::new(ability))
}

fn artifact_subtype(name: &str) -> Subtype {
    Subtype {
        name: name.into(),
        types: vec![Type::Artifact].into(),
        confers: vec![].into(),
    }
}

fn ability_region(effect: Instruction) -> Region {
    let provenances = [
        (Kind::Entity, Provenance::Source),
        (Kind::Entity, Provenance::Controller),
        (Kind::Number, Provenance::AnnouncedX),
    ];
    let params = provenances
        .into_iter()
        .enumerate()
        .map(|(index, (kind, provenance))| Param {
            def: DefId(u32::try_from(index).unwrap()),
            kind,
            provenance,
        })
        .collect();
    Region::new(params, effect.into())
}

fn lower_activated(
    cost: deckmaste_semantics::Cost,
    effect: deckmaste_semantics::OneShotEffect,
) -> ActivatedAbility {
    deckmaste_semantics::ActivatedAbility {
        ability_word: None,
        cost,
        from: None,
        window: None,
        condition: None,
        limits: [].into(),
        effect,
    }
    .lower()
}

fn lower_activated_effect(effect: deckmaste_semantics::OneShotEffect) -> Region {
    lower_activated(deckmaste_semantics::Cost([].into()), effect).effect
}

// [CR#111.10a]
#[test]
fn treasure_token_parses() {
    let token = builtin().token("Treasure").unwrap().core;
    assert_eq!(
        token,
        Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![artifact_subtype("Treasure")].into(),
            abilities: vec![mana_ability(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                from: None,
                window: None,
                cost: Arc::<[CostComponent]>::from(vec![CostComponent::Tap, sacrifice_this()])
                    .into(),
                condition: None,
                limits: vec![].into(),
                effect: ability_region(Instruction::act(Action::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::AnyColor.into()
                ))),
            })]
            .into(),
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10f]
#[test]
fn clue_token_parses() {
    let token = builtin().token("Clue").unwrap().core;
    assert_eq!(
        token,
        Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![artifact_subtype("Clue")].into(),
            abilities: vec![Ability::activated(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                from: None,
                window: None,
                cost: Arc::<[CostComponent]>::from(vec![mana_2(), sacrifice_this()]).into(),
                condition: None,
                limits: vec![].into(),
                effect: lower_activated_effect(
                    builtin()
                        .macros
                        .read_str::<deckmaste_semantics::OneShotEffect>("Draw(1)")
                        .unwrap(),
                ),
            })]
            .into(),
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10b]
#[test]
fn food_token_parses() {
    let token = builtin().token("Food").unwrap().core;
    assert_eq!(
        token,
        Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![artifact_subtype("Food")].into(),
            abilities: vec![Ability::activated(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                from: None,
                window: None,
                cost: Arc::<[CostComponent]>::from(vec![
                    mana_2(),
                    CostComponent::Tap,
                    sacrifice_this()
                ])
                .into(),
                condition: None,
                limits: vec![].into(),
                effect: Region::new(
                    ability_region(Instruction::act(Action::ChangeLife(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        LifeOp::Up(Count::Literal(3)),
                    )))
                    .params,
                    deckmaste_core::Block(
                        vec![
                            Instruction::Let(deckmaste_core::Let {
                                dest: DefId(3),
                                expr: deckmaste_core::Expr::Number(Count::Literal(3)),
                            }),
                            Instruction::act(Action::ChangeLife(
                                Reference::Reg(deckmaste_core::RefId(1)),
                                LifeOp::Up(Count::Reg(deckmaste_core::RefId(3))),
                            )),
                        ]
                        .into()
                    ),
                ),
            })]
            .into(),
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10c]
#[test]
fn gold_token_parses() {
    let token = builtin().token("Gold").unwrap().core;
    assert_eq!(
        token,
        Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![artifact_subtype("Gold")].into(),
            abilities: vec![mana_ability(ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                from: None,
                window: None,
                cost: Arc::<[CostComponent]>::from(vec![sacrifice_this()]).into(),
                condition: None,
                limits: vec![].into(),
                effect: ability_region(Instruction::act(Action::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::AnyColor.into()
                ))),
            })]
            .into(),
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10g]
#[test]
fn blood_token_parses() {
    // Read the whole ability payload through the builtin macro set. Cost
    // products and the Draw result share one region's destination sequence,
    // so they must be lowered together rather than as isolated subterms.
    let plugin = builtin();
    let cost = plugin
        .macros
        .read_str::<deckmaste_semantics::Cost>(
            "[Mana([Generic(1)]), Tap, DiscardCards(1), SacrificeThis]",
        )
        .unwrap();
    let effect = plugin
        .macros
        .read_str::<deckmaste_semantics::OneShotEffect>("Draw(1)")
        .unwrap();
    let expected_ability = lower_activated(cost, effect);
    let token = plugin.token("Blood").unwrap().core;
    assert_eq!(
        token,
        Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![artifact_subtype("Blood")].into(),
            abilities: vec![Ability::activated(expected_ability)].into(),
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10w]
#[test]
fn vibranium_token_parses() {
    use deckmaste_core::ColorOrColorless;
    use deckmaste_core::EventFilter;
    use deckmaste_core::KeywordAbility;
    use deckmaste_core::ManaProduction;
    use deckmaste_core::ManaRider;
    use deckmaste_core::ObjectClass;
    use deckmaste_core::Predicate;
    use deckmaste_core::StaticSpec;

    // Indestructible expands from the `Keyword(Indestructible)` macro — a
    // `Composite` keyword carrying the event-side can't-happen. `lower`
    // erases invocation provenance (spec §12): the loaded token carries the
    // macro's BODY, not a remembered `Expanded` wrapper (here or on the
    // nested `Destroy(Ref(This))` filter, which expands to the `Act` master
    // form).
    let indestructible = Ability::Keyword(KeywordAbility::Composite {
        name: "Indestructible".into(),
        abilities: vec![Ability::r#static(StaticSpec::CantHappen(
            EventFilter::Act {
                verb: deckmaste_core::VerbName::from("Destroy"),
                who: Predicate::Any,
                on: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                cause: None,
            },
        ))],
    });
    // "{T}: Add {C}. This mana can't be spent to cast a nonartifact spell." The
    // SpendOnly rider admits everything EXCEPT a nonartifact spell.
    let restricted_mana = Instruction::act(Action::AddMana(
        Reference::Reg(deckmaste_core::RefId(1)),
        Count::Literal(1),
        ManaProduction::WithRiders {
            mana: ManaSpec::Specific(ColorOrColorless::Colorless),
            riders: vec![ManaRider::SpendOnly(Predicate::Not(Arc::new(
                Predicate::And(
                    vec![
                        Predicate::Class(ObjectClass::Spell),
                        Predicate::Not(Arc::new(Predicate::r#type(Type::Artifact))),
                    ]
                    .into(),
                ),
            )))]
            .into(),
        },
    ));
    let token = builtin().token("Vibranium").unwrap().core;
    assert_eq!(
        token,
        Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![artifact_subtype("Vibranium")].into(),
            abilities: vec![
                indestructible,
                mana_ability(ActivatedAbility {
                    ability_word: None,
                    targets: [].into(),
                    from: None,
                    window: None,
                    cost: Arc::<[CostComponent]>::from(vec![CostComponent::Tap]).into(),
                    condition: None,
                    limits: vec![].into(),
                    effect: ability_region(restricted_mana),
                }),
            ]
            .into(),
            power: None,
            toughness: None,
        }
    );
}

/// `validate_plugin` on the builtin directory must report zero parse failures
/// and zero lint failures with the token files included.
#[test]
fn validate_builtin_with_tokens_has_no_failures() {
    let builtin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin");
    let validation = deckmaste_plugin::validate::validate_plugin(&builtin).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(
        validation.failures.is_empty(),
        "{} parse failure(s)",
        validation.failures.len()
    );
    assert!(
        validation.lint_failures.is_empty(),
        "{} lint failure(s)",
        validation.lint_failures.len()
    );
    // 5 cards + 6 tokens = 11 minimum.
    assert!(
        validation.valid >= 11,
        "only {} items validated",
        validation.valid
    );
}
