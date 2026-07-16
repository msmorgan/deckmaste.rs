//! Integration tests for the builtin token files: Treasure, Clue, Food, Gold,
//! Blood, and Vibranium.
//!
//! Each token is loaded via `Plugin::token(name)` (macro-aware reader) and
//! compared to the expected Rust value. A validate-level assertion confirms
//! that `validate_plugin` reports zero failures when tokens are included.

use std::path::Path;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Ability;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Expansion;
use deckmaste_core::ExpansionArgs;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::ManaSymbol;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::Type;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

/// `Mana([Generic(2)])` cost component.
fn mana_2() -> CostComponent {
    CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
        SimpleManaSymbol::Generic(2),
    )]))
}

/// `SacrificeThis` — a remembered `CostComponent` macro invocation whose body
/// expanded to `Do(Sacrifice(This))`.
fn sacrifice_this() -> CostComponent {
    CostComponent::Expanded(Expansion {
        name: "SacrificeThis".into(),
        args: ExpansionArgs::none(),
        template: None,
        value: Box::new(CostComponent::do_(PlayerAction::Sacrifice(Reference::This))),
    })
}

fn artifact_subtype(name: &str) -> Subtype {
    Subtype {
        name: name.into(),
        types: vec![Type::Artifact],
        confers: vec![],
    }
}

// [CR#111.10a]
#[test]
fn treasure_token_parses() {
    let token = builtin().token("Treasure").unwrap();
    assert_eq!(
        token,
        Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![artifact_subtype("Treasure")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![CostComponent::Tap, sacrifice_this()].into(),
                condition: None,
                limits: vec![],
                effect: OneShotEffect::act_by_you(PlayerAction::AddMana(
                    Count::Literal(1),
                    ManaSpec::AnyColor.into()
                )),
            })],
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10f]
#[test]
fn clue_token_parses() {
    let token = builtin().token("Clue").unwrap();
    assert_eq!(
        token,
        Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![artifact_subtype("Clue")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![mana_2(), sacrifice_this()].into(),
                condition: None,
                limits: vec![],
                effect: builtin().macros.read_str("Draw(1)").unwrap(),
            })],
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10b]
#[test]
fn food_token_parses() {
    let token = builtin().token("Food").unwrap();
    assert_eq!(
        token,
        Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![artifact_subtype("Food")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![mana_2(), CostComponent::Tap, sacrifice_this()].into(),
                condition: None,
                limits: vec![],
                effect: OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(3))),
            })],
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10c]
#[test]
fn gold_token_parses() {
    let token = builtin().token("Gold").unwrap();
    assert_eq!(
        token,
        Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![artifact_subtype("Gold")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![sacrifice_this()].into(),
                condition: None,
                limits: vec![],
                effect: OneShotEffect::act_by_you(PlayerAction::AddMana(
                    Count::Literal(1),
                    ManaSpec::AnyColor.into()
                )),
            })],
            power: None,
            toughness: None,
        }
    );
}

// [CR#111.10g]
#[test]
fn blood_token_parses() {
    let mana_1 = CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
        SimpleManaSymbol::Generic(1),
    )]));
    // Read through the builtin macro set so the remembered `Expanded`
    // wrapper (the `DiscardCards(1)` cost macro) matches exactly.
    let discard_one: CostComponent = builtin().macros.read_str("DiscardCards(1)").unwrap();
    let token = builtin().token("Blood").unwrap();
    assert_eq!(
        token,
        Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![artifact_subtype("Blood")],
            abilities: vec![Ability::Activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![mana_1, CostComponent::Tap, discard_one, sacrifice_this()].into(),
                condition: None,
                limits: vec![],
                effect: builtin().macros.read_str("Draw(1)").unwrap(),
            })],
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
    use deckmaste_core::ObjectKind;
    use deckmaste_core::Predicate;
    use deckmaste_core::StaticEffect;

    // Indestructible expands from the `Keyword(Indestructible)` macro — a
    // `Composite` keyword carrying the event-side can't-happen, under the
    // macro-provenance `Expanded` wrapper.
    let indestructible = Ability::Keyword(KeywordAbility::Expanded(Expansion {
        name: "Indestructible".into(),
        args: ExpansionArgs::none(),
        template: Some("indestructible".into()),
        value: Box::new(KeywordAbility::Composite {
            name: "Indestructible".into(),
            abilities: vec![Ability::Static(StaticEffect::CantHappen(EventFilter::Act(
                deckmaste_core::KeywordActionPattern::Destroy(Predicate::Ref(Reference::This)),
            )))],
        }),
    }));
    // "{T}: Add {C}. This mana can't be spent to cast a nonartifact spell." The
    // SpendOnly rider admits everything EXCEPT a nonartifact spell.
    let restricted_mana = OneShotEffect::act_by_you(PlayerAction::AddMana(
        Count::Literal(1),
        ManaProduction::WithRiders {
            mana: ManaSpec::Specific(ColorOrColorless::Colorless),
            riders: vec![ManaRider::SpendOnly(Predicate::Not(Box::new(
                Predicate::And(vec![
                    Predicate::Kind(ObjectKind::Spell),
                    Predicate::Not(Box::new(Predicate::type_(Type::Artifact))),
                ]),
            )))],
        },
    ));
    let token = builtin().token("Vibranium").unwrap();
    assert_eq!(
        token,
        Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![artifact_subtype("Vibranium")],
            abilities: vec![
                indestructible,
                Ability::Activated(ActivatedAbility {
                    ability_word: None,
                    from: None,
                    window: None,
                    cost: vec![CostComponent::Tap].into(),
                    condition: None,
                    limits: vec![],
                    effect: restricted_mana,
                }),
            ],
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
    let validation = deckmaste_cards::validate::validate_plugin(&builtin).unwrap();
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
