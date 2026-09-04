//! Loads the builtin plugin's real data files: subtype meta-macros,
//! the meta-produced instance definitions, and the basic land cards whose
//! type lines reference the result.

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_card::Characteristics;
use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Duration;
use deckmaste_core::Instruction;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::Property;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::Type;
use deckmaste_core::ron::options as ron_options;
use deckmaste_lowering::Lower;
use deckmaste_plugin::plugin::Plugin;

fn builtin_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
}

fn builtin() -> Plugin {
    Plugin::load(builtin_path()).unwrap()
}

fn lower_spell_effect(effect: deckmaste_semantics::OneShotEffect) -> Instruction {
    let lowered: deckmaste_core::SpellAbility = deckmaste_semantics::SpellAbility {
        ability_word: None,
        effect,
    }
    .lower();
    match lowered.effect.body.as_ref() {
        [single] => single.clone(),
        _ => Instruction::Sequentially(lowered.effect.body.0),
    }
}

/// What a basic land type's declaration expands to: the subtype plus its
/// intrinsic mana ability ([CR#305.6]), conferred as data.
fn basic_land_subtype(name: &str, color: Color) -> Subtype {
    Subtype {
        name: name.into(),
        types: vec![Type::Land].into(),
        confers: vec![Property::Ability(Arc::new(Ability::activated(
            ActivatedAbility {
                ability_word: None,
                targets: [].into(),
                from: None,
                window: None,
                cost: Arc::<[CostComponent]>::from(vec![CostComponent::Tap]).into(),
                condition: None,
                limits: vec![].into(),
                effect: Instruction::act(Action::AddMana(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    ManaSpec::Specific(ColorOrColorless::Color(color)).into(),
                ))
                .into(),
            },
        )))]
        .into(),
    }
}

fn basic_color(name: &str) -> Color {
    match name {
        "Plains" => Color::White,
        "Island" => Color::Blue,
        "Swamp" => Color::Black,
        "Mountain" => Color::Red,
        "Forest" => Color::Green,
        other => panic!("not a basic land type: {other}"),
    }
}

/// The Land card type as the plugin registry expands it: its default-deny
/// land-play marker conferred as data — a `May(Play(what: Ref(This)))` row
/// ([CR#305.9,116.2a,701.18]). `Type::Land.def()` carries EMPTY confers, so the
/// expected card mirrors the registry confer here.
fn land_type() -> deckmaste_core::TypeDef {
    deckmaste_core::TypeDef {
        name: "Land".into(),
        permanent_type: true,
        confers: vec![Property::Ability(Arc::new(Ability::r#static(
            deckmaste_core::StaticSpec::Deontic(deckmaste_core::Deontic::May(
                deckmaste_core::DeonticAction::Play {
                    what: deckmaste_core::Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    by: deckmaste_core::Predicate::Any,
                    from: None,
                },
            )),
        )))]
        .into(),
    }
}

fn basic_land(name: &str) -> Card {
    Card::Normal(CardFace::from(Characteristics {
        name: name.into(),
        mana_cost: ManaCost::default(),
        supertypes: vec![Supertype::Basic],
        types: vec![land_type()],
        subtypes: vec![basic_land_subtype(name, basic_color(name))],
        ..Default::default()
    }))
}

/// builtin is the prelude every other plugin depends on, so this guards it
/// under plain `cargo test`; wizards is the explicit
/// `cargo xtask validate plugins/wizards`.
#[test]
fn builtin_cards_are_valid() {
    let validation = deckmaste_plugin::validate::validate_plugin(&builtin_path()).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    // The handwritten builtin cards: 5 basics + 3 tokens at the time of
    // writing. Floor, not exact, so adding cards or tokens doesn't break
    // the test.
    assert!(
        validation.valid >= 8,
        "only {} items checked",
        validation.valid
    );
}

#[test]
fn basic_lands_parse_against_the_subtype_macros() {
    let plugin = builtin();
    assert!(
        plugin.macros.get("Macro", "LandType").is_some(),
        "LandType macro missing"
    );

    for name in ["Forest", "Island", "Mountain", "Plains", "Swamp"] {
        let card = plugin.card(name).unwrap().core;
        assert_eq!(card, basic_land(name));

        // Every subtype the card references must be declared, under a type
        // the declaration allows.
        let Card::Normal(face) = &card else {
            panic!("{name} should be single-faced");
        };
        for subtype in &face.characteristics.subtypes {
            let declared = plugin
                .subtypes
                .get(&subtype.name)
                .unwrap_or_else(|| panic!("{name} references undeclared {}", subtype.name));
            for parent in subtype.types.iter() {
                assert!(
                    declared.types.contains(parent),
                    "{} is not a {parent:?} subtype",
                    subtype.name,
                );
            }
        }
    }
}

#[test]
fn declared_subtypes_cover_the_basics() {
    let plugin = builtin();
    for name in ["Forest", "Island", "Mountain", "Plains", "Swamp"] {
        assert_eq!(
            plugin.subtypes.get(name),
            Some(&basic_land_subtype(name, basic_color(name)))
        );

        // Declared subtypes are nullary macros expanding to themselves.
        let expanded: Subtype = plugin
            .macros
            .read_str::<deckmaste_semantics::Subtype>(name)
            .unwrap()
            .lower();
        assert_eq!(Some(&expanded), plugin.subtypes.get(name));
    }
}

#[test]
fn subtypes_round_trip_plainly() {
    let forest = Subtype {
        name: "Forest".into(),
        types: vec![Type::Land].into(),
        confers: vec![].into(),
    };
    let written = ron_options().to_string(&forest).unwrap();
    let parsed: Subtype = ron_options().from_str(&written).unwrap();
    assert_eq!(parsed, forest);
}

/// `confers` is omitted from RON when empty (the skip attr is load-bearing)
/// and round-trips when present.
#[test]
fn subtype_confers_round_trips_and_omits_empty() {
    let plain = Subtype {
        name: "Forest".into(),
        types: vec![Type::Land].into(),
        confers: vec![].into(),
    };
    let written = ron_options().to_string(&plain).unwrap();
    assert!(
        !written.contains("confers"),
        "empty confers omitted: {written}"
    );
    assert_eq!(ron_options().from_str::<Subtype>(&written).unwrap(), plain);
}

/// The `Regenerate` macro types its param as a `Reference` (not `Any`), and the
/// one reference value splices into BOTH slot kinds: bare into the `Reference`
/// slots (`subject`, the heal/tap) and wrapped `Ref(...)` into the event
/// `would`'s `Predicate` slot. Both the self form (`This`) and the bound-target
/// form (`Target(0)`) parse — the corpus's only two regeneration shapes
/// ([CR#701.19]: "regenerate this creature" / "regenerate target creature").
#[test]
fn regenerate_macro_expands_with_typed_reference_param() {
    let plugin = builtin();

    // Regenerate(This): lower the semantic invocation before inspecting the
    // runnable core shape.
    let semantic: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("Regenerate(This)").unwrap();
    let effect = lower_spell_effect(semantic);
    // Core makes the binding explicit: pin the source into a register, then
    // create the shield NAMING that register as its subject ([CR#614.1] — a
    // replacement effect is a shield around whatever it affects).
    let Instruction::Sequentially(steps) = effect else {
        panic!("Regenerate(This) must lower to Let + CreateReplacement, got {effect:?}");
    };
    let [
        Instruction::Let(pin),
        Instruction::Act {
            action:
                Action::CreateReplacement {
                    subject,
                    replacement,
                    duration,
                    one_shot,
                },
            ..
        },
    ] = steps.as_ref()
    else {
        panic!("regeneration must pin its subject before creating the shield: {steps:?}");
    };
    assert!(matches!(
        pin.expr,
        deckmaste_core::Expr::Object(Reference::Reg(deckmaste_core::RefId(0)))
    ));
    // The shield's subject is a DECLARED register read of the pinned
    // definition — the engine never searches its register file for it.
    assert_eq!(
        *subject,
        Reference::Reg(pin.dest.into()),
        "the shield names the register the preceding Let defined"
    );
    assert!(*one_shot, "a regeneration shield is one-shot [CR#614.3]");
    assert_eq!(
        *duration,
        Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
    );
    // The watched event is a destruction `Instead`; the heal+tap body taps and
    // removes damage from the same reference.
    let Replacement::Instead { instead, .. } = replacement.as_ref() else {
        panic!("regeneration is an Instead replacement");
    };
    let Instruction::Sequentially(body) = instead else {
        panic!("the regen body is a Sequentially (remove damage, then tap)");
    };
    assert_eq!(body.len(), 2, "remove all damage, then tap [CR#701.19a]");

    // Regenerate(It): the announced-target anaphor parses too — the param is
    // a Reference, so `It` fits exactly where `This` did (as the `TheRef`
    // subject the `With` binds).
    let _: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("Regenerate(Target(0))").unwrap();
}

/// Ticket core-quantity-range: the named `Quantity` forms are builtin macros
/// over the single `Range` primitive, and each round-trips BYTE-IDENTICALLY at
/// the RON surface — the headline guarantee that existing cards don't churn.
#[test]
fn named_quantity_macros_round_trip_byte_identical() {
    use deckmaste_semantics::Quantity;
    let plugin = builtin();
    for surface in [
        "Exactly(1)",
        "Exactly(2)",
        "AtMost(2)",
        "AtLeast(3)",
        "Between(1,3)",
        "AnyNumber",
    ] {
        let parsed: Quantity = plugin
            .macros
            .read_str(surface)
            .unwrap_or_else(|e| panic!("parsing {surface}: {e}"));
        let written = deckmaste_semantics::ron::options()
            .to_string(&parsed)
            .unwrap();
        assert_eq!(written, surface, "surface RON changed for {surface}");
    }
}

/// The macros expand to the right `Range` shape under the remembered
/// invocation — `Exactly` fills both bounds, `AtMost`/`AtLeast` one, `Between`
/// both distinct, `AnyNumber` neither.
#[test]
fn named_quantity_macros_expand_to_range() {
    use deckmaste_semantics::Count;
    use deckmaste_semantics::Quantity;
    let plugin = builtin();
    let q = |s: &str| -> Quantity { plugin.macros.read_str(s).unwrap() };
    assert_eq!(
        q("Exactly(2)").bounds(),
        (Some(&Count::Literal(2)), Some(&Count::Literal(2)))
    );
    assert_eq!(q("AtLeast(3)").bounds(), (Some(&Count::Literal(3)), None));
    assert_eq!(q("AtMost(2)").bounds(), (None, Some(&Count::Literal(2))));
    assert_eq!(
        q("Between(1,3)").bounds(),
        (Some(&Count::Literal(1)), Some(&Count::Literal(3)))
    );
    assert_eq!(q("AnyNumber").bounds(), (None, None));
    assert!(q("Exactly(1)").is_one());
}

/// The open-vocabulary REGISTRIES load as plugin data with their dependent
/// index columns: counter rows carry `scope` ([CR#122.1,122.1f]),
/// designation rows their `Stored` scope ([CR#725.1] Monarch on players,
/// [CR#903.3] Commander on cards), and every `KeywordAbility`-kind macro
/// derives a `KeywordDecl` whose `ParamShape` mirrors its typed parameter
/// signature ([CR#702] keyword one-liners).
#[test]
fn registries_load_with_their_index_columns() {
    use deckmaste_core::CounterScope;
    use deckmaste_core::DesignationDef;
    use deckmaste_core::DesignationScope;
    use deckmaste_core::ParamShape;

    let plugin = builtin();

    // Counter scope column: poison is player-borne, +1/+1 object-borne.
    assert_eq!(
        plugin.counters[&deckmaste_core::Ident::from("Poison")].scope,
        CounterScope::Player
    );
    assert_eq!(
        plugin.counters[&deckmaste_core::Ident::from("P1P1Counter")].scope,
        CounterScope::Object
    );

    // Designation rows: Monarch player-scoped; Commander (outside the
    // curated engine table) object-scoped.
    let scope_of =
        |name: &str| match &plugin.designations[&deckmaste_core::Ident::from(name)].definition {
            DesignationDef::Stored { scope, .. } => *scope,
            other => panic!("{name} should be Stored, got {other:?}"),
        };
    assert_eq!(scope_of("Monarch"), DesignationScope::Player);
    assert_eq!(scope_of("Commander"), DesignationScope::Object);

    // Keyword shapes derive from the macros' typed params.
    let shape_of = |name: &str| plugin.keywords[&deckmaste_core::Ident::from(name)].shape;
    assert_eq!(shape_of("Flying"), ParamShape::None);
    // Ward's signature is { cost: Cost, where_x: Default(Count, X) } — the
    // optional ward-{X} definition rides the params ([CR#702.21b]), so the
    // derived shape is CountedCost.
    assert_eq!(shape_of("Ward"), ParamShape::CountedCost);
    assert_eq!(shape_of("Crew"), ParamShape::Counted);
    assert_eq!(shape_of("Hexproof"), ParamShape::Predicated);
    assert_eq!(shape_of("Reinforce"), ParamShape::CountedCost);
}

/// The first-macro-wave idiom set expands to its blessed core bodies —
/// each macro's acceptance floor: the invocation reads, the expansion is
/// the CR-family node, and the spelling stays remembered for render.
#[test]
fn wave_macros_expand_to_their_blessed_bodies() {
    let plugin = builtin();

    // Unless — the English order over the collapsed `May(Pay(cost))` MustPay
    // shape ([CR#118.12a]): who defaults to You, the cost splices flat.
    let semantic: deckmaste_semantics::OneShotEffect = plugin
        .macros
        .read_str("Unless(effect: Draw(1), unless: [Mana([Generic(2)])])")
        .unwrap();
    let unless = lower_spell_effect(semantic);
    let Instruction::May(m) = &unless else {
        panic!("Unless must lower to May, got {unless:?}");
    };
    assert_eq!(
        m.who,
        Reference::Reg(deckmaste_core::RefId(1)),
        "the payer defaults to You"
    );
    assert!(
        m.if_did.is_none(),
        "no positive branch — this is the punisher shape"
    );
    let Some(if_not) = m.if_not.as_ref() else {
        panic!("expected if_not, got None");
    };
    // The literal-one `Draw(1)` expansion is one draw instruction followed by
    // the explicit pinned amount consumed by later discourse reads. Its
    // spelling may retain the semantically neutral `Batch(1, ...)` wrapper —
    // the instruction level a count-referring replacement bites ([CR#121.2a]),
    // whose elements are the individual card draws ([CR#121.2]).
    // It is not a `Composite`: drawing is [CR#121], not a keyword action
    // ([CR#701]), and [CR#121.5] makes it irreducible, so there is no body.
    let draw = match if_not.as_ref() {
        Instruction::Batch(Count::Literal(1), inner) => inner.as_ref(),
        other => other,
    };
    let Instruction::Sequentially(draw) = draw else {
        panic!("Draw(1) lowers to draw + pinned amount, got {if_not:?}");
    };
    assert!(
        matches!(
            draw.as_ref(),
            [
                Instruction::Act {
                    dest: Some(_),
                    action: Action::DrawCard(who),
                },
                Instruction::Let(deckmaste_core::Let {
                    expr: deckmaste_core::Expr::Number(Count::Literal(1)),
                    ..
                }),
            ] if *who == Reference::Reg(deckmaste_core::RefId(1))
        ),
        "or_else carries the unpaid Draw instructions, got {draw:?}"
    );

    // Exile — the render name over the pure zone move ([CR#701.13]).
    let semantic: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("Exile(This)").unwrap();
    let exile = lower_spell_effect(semantic);
    assert!(
        matches!(
            exile,
            Instruction::Act {
                action: Action::Move(Reference::Reg(deckmaste_core::RefId(0)), _, _, _),
                ..
            }
        ),
        "Exile(This) lowers to Move(This, Exile)"
    );

    // DestroyNoRegen — destroy + the ForThisEvent-scoped
    // Cant(Regenerate) rider ([CR#701.19c]).
    let semantic: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("DestroyNoRegen(This)").unwrap();
    let dnr = lower_spell_effect(semantic);
    let Instruction::Sequentially(parts) = &dnr else {
        panic!("DestroyNoRegen lowers to Sequentially, got {dnr:?}");
    };
    assert!(
        matches!(
            &parts[0],
            Instruction::Act { action: Action::Composite { name, body }, .. }
                if name.as_str() == "Destroy"
                    && matches!(
                        body.as_ref(),
                        Instruction::Act { action: Action::Move(Reference::Reg(deckmaste_core::RefId(0)), _, _, _), .. }
                    )
        ),
        "DestroyNoRegen's first part destroys This, got {:?}",
        parts[0]
    );
    assert!(
        matches!(&parts[1], Instruction::Until(Duration::ForThisEvent, statics) if statics.len() == 1),
        "the rider is a ForThisEvent-scoped static, got {:?}",
        parts[1]
    );

    // PreventNext / PreventAll — render names over the Prevention class
    // ([CR#615.7,615.1]), one-shot shields until end of turn.
    let semantic: deckmaste_semantics::OneShotEffect = plugin
        .macros
        .read_str("PreventNext(n: 3, to: Creature)")
        .unwrap();
    let next = lower_spell_effect(semantic);
    assert!(
        matches!(
            next,
            Instruction::Continuously(c) if c.duration == Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
        ),
        "PreventNext lowers to an until-end-of-turn shield"
    );
    let all: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("PreventAll(to: Ref(You))").unwrap();
    assert!(matches!(
        all,
        deckmaste_semantics::OneShotEffect::Expanded(_)
    ));

    // Multikicker — the repeatable kicker variant ([CR#702.33c]): the same
    // Kicker-tagged CostOption with repeatable: true.
    let semantic: deckmaste_semantics::KeywordAbility = plugin
        .macros
        .read_str("Multikicker([Mana([Generic(1)])])")
        .unwrap();
    let multi: deckmaste_core::KeywordAbility = semantic.lower();
    let deckmaste_core::KeywordAbility::Composite { abilities, .. } = &multi else {
        panic!("Multikicker lands on a Composite");
    };
    let Ability::Static(s) = &abilities[0] else {
        panic!("Multikicker's row is a Static");
    };
    let deckmaste_core::StaticSpec::CostOption(oc) = &s.body else {
        panic!("Multikicker declares a CostOption");
    };
    assert!(oc.repeatable, "multikicker is the repeatable row");
    assert_eq!(
        oc.tag,
        deckmaste_core::CostTag::from("Kicker"),
        "a multikicker cost IS a kicker cost ([CR#702.33c])"
    );

    // Chapter — sagas as data ([CR#714.2b]): OneOrMore + Crossed, a plain
    // REMOVABLE triggered ability (NOT Innate: [CR#714.2d] contemplates a
    // Saga that has lost its chapter abilities, so they cannot be innate).
    let semantic: deckmaste_semantics::Ability = plugin
        .macros
        .read_str("Chapter(n: [1], effect: Draw(1))")
        .unwrap();
    let chapter: Ability = semantic.lower();
    let Ability::Triggered(t) = &chapter else {
        panic!("a chapter is a plain removable triggered ability ([CR#714.2,714.2d])");
    };
    assert!(
        matches!(&t.event, deckmaste_core::EventFilter::OneOrMore(_)),
        "one occurrence per placement batch ([CR#603.2c])"
    );
    let Some(deckmaste_core::Condition::Crossed { thresholds, .. }) = &t.condition else {
        panic!("the [CR#714.2b] was-less-than/became-at-least gate");
    };
    assert_eq!(
        thresholds,
        &Arc::<[deckmaste_core::Count]>::from([deckmaste_core::Count::Literal(1)]),
        "a single chapter is a one-element threshold list"
    );
}

/// A chapter RANGE `{rN1}, {rN2}—[Effect]` ([CR#714.2c]) is ONE `Chapter`
/// atom with a multi-number `n`, expanding to a single removable triggered
/// ability whose `Crossed` gate lists every chapter number.
#[test]
fn chapter_range_expands_to_plural_thresholds() {
    let plugin = builtin();
    let semantic: deckmaste_semantics::Ability = plugin
        .macros
        .read_str("Chapter(n: [2, 3], effect: Draw(1))")
        .unwrap();
    let chapter: Ability = semantic.lower();
    let Ability::Triggered(t) = &chapter else {
        panic!("a chapter is a plain removable triggered ability ([CR#714.2,714.2d])");
    };
    let Some(deckmaste_core::Condition::Crossed { thresholds, .. }) = &t.condition else {
        panic!("the [CR#714.2b] crossing gate");
    };
    assert_eq!(
        thresholds,
        &Arc::<[deckmaste_core::Count]>::from([
            deckmaste_core::Count::Literal(2),
            deckmaste_core::Count::Literal(3)
        ]),
        "a range lists every chapter number ([CR#714.2c])"
    );
}

/// `LoyaltyPlus`/`LoyaltyMinus` ([CR#606.2,606.3,606.4,306.5d]): both
/// expand to an `Activated` ability with `window: SorcerySpeed` and
/// `limits: [LoyaltyOncePerTurn]` — the shared loyalty gate — wrapping a
/// `PutCounters`/`RemoveCounters(This, LoyaltyCounter, N)` cost.
#[test]
fn loyalty_macros_expand_to_sorcery_speed_shared_once_per_turn() {
    use deckmaste_core::Cost;
    use deckmaste_core::CostComponent;
    use deckmaste_core::CounterRef;
    use deckmaste_core::Timing;
    use deckmaste_core::UseLimit;
    let plugin = builtin();

    let plus: deckmaste_semantics::Ability = plugin
        .macros
        .read_str("LoyaltyPlus(n: 1, effect: Draw(1))")
        .unwrap();
    assert!(
        matches!(&plus, deckmaste_semantics::Ability::Expanded(_)),
        "expected a remembered LoyaltyPlus expansion"
    );
    let plus: Ability = plus.lower();
    let Ability::Activated(a) = plus else {
        panic!("a loyalty ability is an Activated ability ([CR#606.3])");
    };
    assert_eq!(
        a.window,
        Some(Timing::SorcerySpeed),
        "loyalty abilities activate only as a sorcery ([CR#606.3])"
    );
    assert_eq!(
        a.limits,
        vec![UseLimit::LoyaltyOncePerTurn].into(),
        "the shared per-permanent limit ([CR#606.3,306.5d]), not a plain OncePerTurn"
    );
    let Cost(components) = &a.cost;
    assert!(
        components.iter().any(|c| matches!(
            c,
            CostComponent::Act { action, .. }
                if matches!(
                    action.as_action(),
                    deckmaste_core::Action::PutCounters(
                        deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                        counter,
                        _,
                    ) if *counter == CounterRef::from("LoyaltyCounter")
                )
        )),
        "LoyaltyPlus pays with a PutCounters(This, LoyaltyCounter, N) verb, got {components:?}"
    );

    let minus: deckmaste_semantics::Ability = plugin
        .macros
        .read_str("LoyaltyMinus(n: 3, effect: Draw(1))")
        .unwrap();
    assert!(
        matches!(&minus, deckmaste_semantics::Ability::Expanded(_)),
        "expected a remembered LoyaltyMinus expansion"
    );
    let minus: Ability = minus.lower();
    let Ability::Activated(a) = minus else {
        panic!("a loyalty ability is an Activated ability ([CR#606.3])");
    };
    assert_eq!(a.window, Some(Timing::SorcerySpeed));
    assert_eq!(a.limits, vec![UseLimit::LoyaltyOncePerTurn].into());
    let Cost(components) = &a.cost;
    assert!(
        components.iter().any(|c| matches!(
            c,
            CostComponent::Act { action, .. }
                if matches!(
                    action.as_action(),
                    deckmaste_core::Action::RemoveCounters(
                        deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                        counter,
                        _,
                    ) if *counter == CounterRef::from("LoyaltyCounter")
                )
        )),
        "LoyaltyMinus pays with a RemoveCounters(This, LoyaltyCounter, N) verb, got {components:?}"
    );
}

/// Amass [subtype] N ([CR#701.47a]) — the canonical *composite* keyword action:
/// it decomposes into core primitives, never a new engine verb. Expanding
/// `Amass(Zombie, 1)` (the subtype is a declared `Subtype` param) must yield
/// the four CR sentences as data: (1) the guard-token `Create`, (2) the
/// `ChooseOne` bind, (3) the `PutCounters` growth, (4) the "becomes a
/// [subtype]" continuous add.
#[test]
fn amass_decomposes_into_core_primitives() {
    use deckmaste_core::CollectionOp;
    use deckmaste_core::Condition;
    use deckmaste_core::Continuously;
    use deckmaste_core::CounterRef;
    use deckmaste_core::Modification;
    use deckmaste_core::StaticSpec;
    use deckmaste_core::TokenSpec;

    let plugin = builtin();
    let semantic: deckmaste_semantics::OneShotEffect =
        plugin.macros.read_str("Amass(Zombie, 1)").unwrap();
    let effect = lower_spell_effect(semantic);
    let Instruction::Sequentially(steps) = &effect else {
        panic!("Amass lowers to Sequentially, got {effect:?}");
    };
    assert_eq!(
        steps.len(),
        4,
        "guard-token, explicit choice, growth, then subtype-change steps"
    );

    // Step 1: "If you don't control an Army creature, create a 0/0 black
    // [subtype] Army creature token."
    let Instruction::If(guard) = &steps[0] else {
        panic!("step 1 is an If, got {:?}", steps[0]);
    };
    assert!(
        matches!(&guard.condition, Condition::Not(inner) if matches!(inner.as_ref(), Condition::Exists(_))),
        "the guard is `Not(Exists(Army creature you control))`, got {:?}",
        guard.condition,
    );
    let Instruction::Act {
        action:
            Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                token: TokenSpec::Token(tok),
                ..
            },
        ..
    } = guard.then.as_ref()
    else {
        panic!("the guard creates a token, got {:?}", guard.then);
    };
    assert_eq!(
        tok.color_indicator,
        vec![Color::Black].into(),
        "0/0 BLACK token"
    );
    // The token's `Creature` type carries the combat-capability confers
    // ([CR#508.1a,509.1a]) — the `Creature` cardtype macro expands to the full
    // conferring `TypeDef`, not the empty-confer `Type::Creature.def()`. Read
    // the SAME expansion the token's semantic `types: [Creature]` produced.
    let creature_type: deckmaste_core::TypeDef = plugin
        .macros
        .read_str::<deckmaste_semantics::TypeDef>("Creature")
        .unwrap()
        .lower();
    assert_eq!(tok.types, vec![creature_type].into());
    let names: Arc<[&str]> = tok.subtypes.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(
        names,
        vec!["Zombie", "Army"].into(),
        "the amassed subtype PLUS Army"
    );
    assert_eq!(tok.power, Some(deckmaste_core::StatValue::Number(0)));
    assert_eq!(tok.toughness, Some(deckmaste_core::StatValue::Number(0)));

    // Step 2: "Choose an Army creature you control." Core records the result
    // in the destination register read by both following instructions.
    let Instruction::Choose(choice) = &steps[1] else {
        panic!("step 2 is an explicit Choose, got {:?}", steps[1]);
    };
    let chosen = Reference::Reg(choice.dest.into());

    // Step 3: "Put N +1/+1 counters on that creature."
    assert_eq!(
        steps[2],
        Instruction::Act {
            dest: None,
            action: Action::PutCounters(
                chosen.clone(),
                CounterRef::from("P1P1Counter"),
                Count::Literal(1),
            ),
        },
        "N +1/+1 counters on the explicitly chosen Army",
    );

    // Step 4: "If it isn't a [subtype], it becomes a [subtype] in addition to
    // its other types." — a one-shot-created continuous subtype-add.
    let Instruction::If(becomes) = &steps[3] else {
        panic!("step 4 is an If, got {:?}", steps[3]);
    };
    assert!(
        matches!(&becomes.condition, Condition::Not(inner) if matches!(inner.as_ref(), Condition::Matches(reference, _) if reference == &chosen)),
        "guarded on `Not(Matches(chosen, Zombie))`, got {:?}",
        becomes.condition,
    );
    let Instruction::Continuously(Continuously { effect, duration }) = becomes.then.as_ref() else {
        panic!(
            "the becomes is a one-shot continuous effect, got {:?}",
            becomes.then
        );
    };
    assert_eq!(
        *duration,
        Duration::EndOfGame,
        "no stated duration ([CR#611.2a])"
    );
    let StaticSpec::Modify(reference, Modification::Subtypes(CollectionOp::Add(added))) =
        effect.as_ref()
    else {
        panic!("it ADDS the subtype to the chosen Army, got {effect:?}");
    };
    assert_eq!(reference, &chosen);
    assert_eq!(added.as_str(), "Zombie", "becomes a Zombie in addition");
}

/// The rules tables are semantic containers too (ticket scope): they parse at
/// semantics kinds and reach the engine only through `lower`.
#[test]
fn rules_tables_load_through_the_semantics_grammar() {
    let plugin = Plugin::load(builtin_path()).unwrap();
    assert!(
        !plugin.sba_rules.is_empty(),
        "builtin ships SBA rules; an empty table means the loader silently stopped finding them"
    );
}
