//! Loads the builtin plugin's real data files: subtype meta-macros,
//! the meta-produced instance definitions, and the basic land cards whose
//! type lines reference the result.

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Duration;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Property;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::Type;
use deckmaste_core::ron::options as ron_options;

fn builtin_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
}

fn builtin() -> Plugin {
    Plugin::load(builtin_path()).unwrap()
}

/// What a basic land type's declaration expands to: the subtype plus its
/// intrinsic mana ability ([CR#305.6]), conferred as data.
fn basic_land_subtype(name: &str, color: Color) -> Subtype {
    Subtype {
        name: name.into(),
        types: vec![Type::Land],
        confers: vec![Property::Ability(Arc::new(Ability::activated(
            ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![CostComponent::Tap].into(),
                condition: None,
                limits: vec![],
                effect: OneShotEffect::Act(Action::By(
                    Reference::You,
                    PlayerAction::AddMana(
                        Count::Literal(1),
                        ManaSpec::Specific(ColorOrColorless::Color(color)).into(),
                    ),
                )),
            },
        )))],
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
        permanent: true,
        confers: vec![Property::Ability(Arc::new(Ability::r#static(
            deckmaste_core::StaticEffect::Deontic(deckmaste_core::Deontic::May(
                deckmaste_core::DeonticAction::Play {
                    what: deckmaste_core::Predicate::Ref(Reference::This),
                    by: deckmaste_core::Predicate::Any,
                    from: None,
                },
            )),
        )))],
    }
}

fn basic_land(name: &str) -> Card {
    Card::Normal(CardFace {
        name: name.to_owned(),
        mana_cost: ManaCost::default(),
        supertypes: vec![Supertype::Basic],
        types: vec![land_type()],
        subtypes: vec![basic_land_subtype(name, basic_color(name))],
        ..Default::default()
    })
}

/// builtin is the prelude every other plugin depends on, so this guards it
/// under plain `cargo test`; wizards is the explicit
/// `cargo xtask validate plugins/wizards`.
#[test]
fn builtin_cards_are_valid() {
    let validation = deckmaste_cards::validate::validate_plugin(&builtin_path()).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    // The handwritten builtin cards: 5 basics + 3 tokens at the time of writing.
    // Floor, not exact, so adding cards or tokens doesn't break the test.
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
        let card = plugin.card(name).unwrap();
        assert_eq!(card, basic_land(name));

        // Every subtype the card references must be declared, under a type
        // the declaration allows.
        let Card::Normal(face) = &card else {
            panic!("{name} should be single-faced");
        };
        for subtype in &face.subtypes {
            let declared = plugin
                .subtypes
                .get(&subtype.name)
                .unwrap_or_else(|| panic!("{name} references undeclared {}", subtype.name));
            for parent in &subtype.types {
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
        let expanded: Subtype = plugin.macros.read_str(name).unwrap();
        assert_eq!(Some(&expanded), plugin.subtypes.get(name));
    }
}

#[test]
fn subtypes_round_trip_plainly() {
    let forest = Subtype {
        name: "Forest".into(),
        types: vec![Type::Land],
        confers: vec![],
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
        types: vec![Type::Land],
        confers: vec![],
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

    // Regenerate(This): the self form. A macro invocation is REMEMBERED as
    // `Expanded` (the bidirectional form — it renders back to "Regenerate(This)"
    // via the template; the typed param is what restores that round-trip), with
    // the expansion in `value`.
    let effect: OneShotEffect = plugin.macros.read_str("Regenerate(This)").unwrap();
    let OneShotEffect::Expanded(ref ex) = effect else {
        panic!("a macro invocation is remembered as Expanded, got {effect:?}");
    };
    assert_eq!(ex.name.as_str(), "Regenerate");
    // The subject is bound by an enclosing `With(TheRef(Param(0)))` as the
    // singular `That` (the shield freezes it at creation); `CreateReplacement`
    // no longer carries an authored `subject:` field.
    let OneShotEffect::With(deckmaste_core::With { binder, body }) = (*ex.value).clone() else {
        panic!(
            "Regenerate(This) must expand to With(TheRef, CreateReplacement), got {:?}",
            ex.value
        );
    };
    assert_eq!(
        binder,
        deckmaste_core::Binder::TheRef(Reference::This),
        "the regenerated permanent is bound by With(TheRef(This))"
    );
    let OneShotEffect::Act(Action::CreateReplacement {
        replacement,
        duration,
        one_shot,
    }) = body.as_ref().clone()
    else {
        panic!("the With body is a CreateReplacement, got {body:?}");
    };
    assert!(one_shot, "a regeneration shield is one-shot [CR#614.3]");
    assert_eq!(
        duration,
        Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
    );
    // The watched event is a destruction `Instead`; the heal+tap body taps and
    // removes damage from the same reference.
    let Replacement::Instead { instead, .. } = replacement.as_ref().clone() else {
        panic!("regeneration is an Instead replacement");
    };
    let OneShotEffect::Sequentially(body) = instead else {
        panic!("the regen body is a Sequentially (remove damage, then tap)");
    };
    assert_eq!(body.len(), 2, "remove all damage, then tap [CR#701.19a]");

    // Regenerate(It): the announced-target anaphor parses too — the param is
    // a Reference, so `It` fits exactly where `This` did (as the `TheRef`
    // subject the `With` binds).
    let tgt: OneShotEffect = plugin.macros.read_str("Regenerate(It)").unwrap();
    let OneShotEffect::Expanded(tex) = tgt else {
        panic!("Regenerate(It) is remembered as Expanded");
    };
    assert!(
        matches!(
            *tex.value,
            OneShotEffect::With(deckmaste_core::With {
                binder: deckmaste_core::Binder::TheRef(Reference::It),
                ..
            })
        ),
        "Regenerate(It) expands with the anaphor subject bound by With, got {:?}",
        tex.value
    );
}

/// Ticket core-quantity-range: the named `Quantity` forms are builtin macros
/// over the single `Range` primitive, and each round-trips BYTE-IDENTICALLY at
/// the RON surface — the headline guarantee that existing cards don't churn.
#[test]
fn named_quantity_macros_round_trip_byte_identical() {
    use deckmaste_core::Quantity;
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
        let written = ron_options().to_string(&parsed).unwrap();
        assert_eq!(written, surface, "surface RON changed for {surface}");
    }
}

/// The macros expand to the right `Range` shape under the remembered
/// invocation — `Exactly` fills both bounds, `AtMost`/`AtLeast` one, `Between`
/// both distinct, `AnyNumber` neither.
#[test]
fn named_quantity_macros_expand_to_range() {
    use deckmaste_core::Count;
    use deckmaste_core::Quantity;
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

    // Unless — the English order over MustPay ([CR#118.12a]): actor
    // defaults to You, the cost splices flat.
    let unless: OneShotEffect = plugin
        .macros
        .read_str("Unless(effect: Draw(1), unless: [Mana([Generic(2)])])")
        .unwrap();
    let OneShotEffect::Expanded(exp) = unless else {
        panic!("expected a remembered Unless expansion, got {unless:?}");
    };
    assert_eq!(exp.name.as_str(), "Unless");
    let OneShotEffect::MustPay(m) = exp.value.as_ref() else {
        panic!("Unless must expand to MustPay, got {:?}", exp.value);
    };
    assert_eq!(m.actor, Reference::You, "the payer defaults to You");
    // `Draw(1)` is now the `Draw` macro (like `Mill`), so the unpaid branch is
    // its remembered `Expanded` wrapping the `Composite(Draw(You, 1), …)`.
    let OneShotEffect::Expanded(draw_exp) = m.or_else.as_ref() else {
        panic!(
            "or_else should be the remembered Draw expansion, got {:?}",
            m.or_else
        );
    };
    assert_eq!(draw_exp.name.as_str(), "Draw");
    // `Draw(1)` is a slice-family `Batch(1, Act(Composite(name: Draw, …)))`.
    let OneShotEffect::Batch(_, draw_inner) = draw_exp.value.as_ref() else {
        panic!("Draw expands to a Batch, got {:?}", draw_exp.value);
    };
    assert!(
        matches!(
            draw_inner.as_ref(),
            OneShotEffect::Act(Action::Composite { name, .. }) if name.as_str() == "Draw"
        ),
        "or_else carries the unpaid Draw batch, got {draw_inner:?}"
    );

    // Exile — the render name over the pure zone move ([CR#701.13]).
    let exile: OneShotEffect = plugin.macros.read_str("Exile(This)").unwrap();
    let OneShotEffect::Expanded(exp) = exile else {
        panic!("expected a remembered Exile expansion");
    };
    assert!(
        matches!(
            exp.value.as_ref(),
            OneShotEffect::Act(Action::Move(Reference::This, _, _, _))
        ),
        "Exile(This) is Move(This, Exile), got {:?}",
        exp.value
    );

    // DestroyNoRegen — destroy + the ForThisEvent-scoped
    // Cant(Regenerate) rider ([CR#701.19c]).
    let dnr: OneShotEffect = plugin.macros.read_str("DestroyNoRegen(This)").unwrap();
    let OneShotEffect::Expanded(exp) = dnr else {
        panic!("expected a remembered DestroyNoRegen expansion");
    };
    let OneShotEffect::Sequentially(parts) = exp.value.as_ref() else {
        panic!("DestroyNoRegen is a Sequentially, got {:?}", exp.value);
    };
    assert!(
        matches!(
            &parts[0],
            OneShotEffect::Act(Action::Composite { name, body })
                if name.as_str() == "Destroy"
                    && matches!(
                        body.as_ref(),
                        OneShotEffect::Act(Action::Move(Reference::This, _, _, _))
                    )
        ),
        "DestroyNoRegen's first part destroys This, got {:?}",
        parts[0]
    );
    assert!(
        matches!(&parts[1], OneShotEffect::Until(Duration::ForThisEvent, statics) if statics.len() == 1),
        "the rider is a ForThisEvent-scoped static, got {:?}",
        parts[1]
    );

    // PreventNext / PreventAll — render names over the Prevention class
    // ([CR#615.7,615.1]), one-shot shields until end of turn.
    let next: OneShotEffect = plugin
        .macros
        .read_str("PreventNext(n: 3, to: Creature)")
        .unwrap();
    let OneShotEffect::Expanded(exp) = next else {
        panic!("expected a remembered PreventNext expansion");
    };
    assert!(
        matches!(
            exp.value.as_ref(),
            OneShotEffect::Continuously(c) if c.duration == Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
        ),
        "PreventNext is an until-end-of-turn shield, got {:?}",
        exp.value
    );
    let all: OneShotEffect = plugin.macros.read_str("PreventAll(to: Ref(You))").unwrap();
    assert!(matches!(all, OneShotEffect::Expanded(_)));

    // Multikicker — the repeatable kicker variant ([CR#702.33c]): the same
    // Kicker-tagged CostOption with repeatable: true.
    let multi: deckmaste_core::KeywordAbility = plugin
        .macros
        .read_str("Multikicker([Mana([Generic(1)])])")
        .unwrap();
    let deckmaste_core::KeywordAbility::Expanded(exp) = multi else {
        panic!("expected a remembered Multikicker expansion");
    };
    let deckmaste_core::KeywordAbility::Composite { abilities, .. } = exp.value.as_ref() else {
        panic!("Multikicker lands on a Composite");
    };
    let Ability::Static(s) = &abilities[0] else {
        panic!("Multikicker's row is a Static");
    };
    let deckmaste_core::StaticEffect::CostOption(oc) = s.as_ref() else {
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
    let chapter: Ability = plugin
        .macros
        .read_str("Chapter(n: [1], effect: Draw(1))")
        .unwrap();
    let Ability::Expanded(exp) = chapter else {
        panic!("expected a remembered Chapter expansion");
    };
    let Ability::Triggered(t) = exp.value.as_ref() else {
        panic!("a chapter is a plain removable triggered ability ([CR#714.2,714.2d])");
    };
    assert!(
        matches!(&t.event, deckmaste_core::EventFilter::OneOrMore(_)),
        "one occurrence per placement batch ([CR#603.3b])"
    );
    let Some(deckmaste_core::Condition::Crossed { thresholds, .. }) = &t.condition else {
        panic!("the [CR#714.2b] was-less-than/became-at-least gate");
    };
    assert_eq!(
        thresholds,
        &vec![deckmaste_core::Count::Literal(1)],
        "a single chapter is a one-element threshold list"
    );
}

/// A chapter RANGE `{rN1}, {rN2}—[Effect]` ([CR#714.2c]) is ONE `Chapter`
/// atom with a multi-number `n`, expanding to a single removable triggered
/// ability whose `Crossed` gate lists every chapter number.
#[test]
fn chapter_range_expands_to_plural_thresholds() {
    let plugin = builtin();
    let chapter: Ability = plugin
        .macros
        .read_str("Chapter(n: [2, 3], effect: Draw(1))")
        .unwrap();
    let Ability::Expanded(exp) = chapter else {
        panic!("expected a remembered Chapter expansion");
    };
    let Ability::Triggered(t) = exp.value.as_ref() else {
        panic!("a chapter is a plain removable triggered ability ([CR#714.2,714.2d])");
    };
    let Some(deckmaste_core::Condition::Crossed { thresholds, .. }) = &t.condition else {
        panic!("the [CR#714.2b] crossing gate");
    };
    assert_eq!(
        thresholds,
        &vec![
            deckmaste_core::Count::Literal(2),
            deckmaste_core::Count::Literal(3)
        ],
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
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Timing;
    use deckmaste_core::UseLimit;

    let plugin = builtin();

    let plus: Ability = plugin
        .macros
        .read_str("LoyaltyPlus(n: 1, effect: Draw(1))")
        .unwrap();
    let Ability::Expanded(exp) = plus else {
        panic!("expected a remembered LoyaltyPlus expansion");
    };
    let Ability::Activated(a) = exp.value.as_ref() else {
        panic!("a loyalty ability is an Activated ability ([CR#606.3])");
    };
    assert_eq!(
        a.window,
        Some(Timing::SorcerySpeed),
        "loyalty abilities activate only as a sorcery ([CR#606.3])"
    );
    assert_eq!(
        a.limits,
        vec![UseLimit::LoyaltyOncePerTurn],
        "the shared per-permanent limit ([CR#606.3,306.5d]), not a plain OncePerTurn"
    );
    let Cost(components) = &a.cost;
    assert!(
        components.iter().any(|c| matches!(
            c,
            CostComponent::Do(action)
                if matches!(
                    action.as_ref(),
                    deckmaste_core::Action::By(_, PlayerAction::PutCounters(
                        deckmaste_core::Reference::This,
                        counter,
                        _,
                    )) if *counter == CounterRef::from("LoyaltyCounter")
                )
        )),
        "LoyaltyPlus pays with a PutCounters(This, LoyaltyCounter, N) verb, got {components:?}"
    );

    let minus: Ability = plugin
        .macros
        .read_str("LoyaltyMinus(n: 3, effect: Draw(1))")
        .unwrap();
    let Ability::Expanded(exp) = minus else {
        panic!("expected a remembered LoyaltyMinus expansion");
    };
    let Ability::Activated(a) = exp.value.as_ref() else {
        panic!("a loyalty ability is an Activated ability ([CR#606.3])");
    };
    assert_eq!(a.window, Some(Timing::SorcerySpeed));
    assert_eq!(a.limits, vec![UseLimit::LoyaltyOncePerTurn]);
    let Cost(components) = &a.cost;
    assert!(
        components.iter().any(|c| matches!(
            c,
            CostComponent::Do(action)
                if matches!(
                    action.as_ref(),
                    deckmaste_core::Action::By(_, PlayerAction::RemoveCounters(
                        deckmaste_core::Reference::This,
                        counter,
                        _,
                    )) if *counter == CounterRef::from("LoyaltyCounter")
                )
        )),
        "LoyaltyMinus pays with a RemoveCounters(This, LoyaltyCounter, N) verb, got {components:?}"
    );
}

/// Amass [subtype] N ([CR#701.47a]) — the canonical *composite* keyword action:
/// it decomposes into core primitives, never a new engine verb. Expanding
/// `Amass("Orc", 1)` (Orcish Bowmasters' "amass Orcs 1") must yield the four
/// CR sentences as data: (1) the guard-token `Create`, (2) the `ChooseOne`
/// bind, (3) the `PutCounters` growth, (4) the "becomes a [subtype]" continuous
/// add.
#[test]
fn amass_decomposes_into_core_primitives() {
    use deckmaste_core::Binder;
    use deckmaste_core::CollectionOp;
    use deckmaste_core::Condition;
    use deckmaste_core::Continuously;
    use deckmaste_core::CounterRef;
    use deckmaste_core::Modification;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::TokenSpec;

    let plugin = builtin();
    let effect: OneShotEffect = plugin.macros.read_str("Amass(\"Orc\", 1)").unwrap();
    // The invocation is remembered (so it can render back through the template).
    let OneShotEffect::Expanded(exp) = effect else {
        panic!("expected a remembered Amass expansion");
    };
    assert_eq!(exp.name.as_str(), "Amass");
    let OneShotEffect::Sequentially(steps) = exp.value.as_ref() else {
        panic!("Amass is a Sequentially, got {:?}", exp.value);
    };
    assert_eq!(
        steps.len(),
        2,
        "guard-token step, then the choose+grow step"
    );

    // Step 1: "If you don't control an Army creature, create a 0/0 black
    // [subtype] Army creature token."
    let OneShotEffect::If(guard) = &steps[0] else {
        panic!("step 1 is an If, got {:?}", steps[0]);
    };
    assert!(
        matches!(&guard.condition, Condition::Not(inner) if matches!(inner.as_ref(), Condition::Exists(_))),
        "the guard is `Not(Exists(Army creature you control))`, got {:?}",
        guard.condition,
    );
    let OneShotEffect::Act(Action::By(
        Reference::You,
        PlayerAction::Create(_, TokenSpec::Token(tok), _),
    )) = guard.then.as_ref()
    else {
        panic!("the guard creates a token, got {:?}", guard.then);
    };
    assert_eq!(tok.color_indicator, vec![Color::Black], "0/0 BLACK token");
    // The token's `Creature` type carries the combat-capability confers
    // ([CR#508.1a,509.1a]) — the `Creature` cardtype macro expands to the full
    // conferring `TypeDef`, not the empty-confer `Type::Creature.def()`. Read
    // the SAME expansion the token's authored `types: [Creature]` produced.
    let creature_type: deckmaste_core::TypeDef = plugin.macros.read_str("Creature").unwrap();
    assert_eq!(tok.types, vec![creature_type]);
    let names: Vec<&str> = tok.subtypes.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, vec!["Orc", "Army"], "the amassed subtype PLUS Army");
    assert_eq!(tok.power, Some(deckmaste_core::StatValue::Number(0)));
    assert_eq!(tok.toughness, Some(deckmaste_core::StatValue::Number(0)));

    // Step 2: "Choose an Army creature you control," then grow + becomes.
    let OneShotEffect::With(with) = &steps[1] else {
        panic!("step 2 is a With, got {:?}", steps[1]);
    };
    assert!(
        matches!(&with.binder, Binder::ChooseOne { .. }),
        "the Army is CHOSEN (bound as That), got {:?}",
        with.binder,
    );
    let OneShotEffect::Sequentially(body) = with.body.as_ref() else {
        panic!("the With body is a Sequentially, got {:?}", with.body);
    };
    assert_eq!(body.len(), 2, "put counters, then the becomes-subtype If");

    // Step 3: "Put N +1/+1 counters on that creature."
    assert_eq!(
        body[0],
        OneShotEffect::Act(Action::By(
            Reference::You,
            PlayerAction::PutCounters(
                Reference::It,
                CounterRef::from("P1P1Counter"),
                Count::Literal(1),
            ),
        )),
        "N +1/+1 counters on the chosen Army",
    );

    // Step 4: "If it isn't a [subtype], it becomes a [subtype] in addition to
    // its other types." — a one-shot-created continuous subtype-add.
    let OneShotEffect::If(becomes) = &body[1] else {
        panic!("step 4 is an If, got {:?}", body[1]);
    };
    assert!(
        matches!(&becomes.condition, Condition::Not(inner) if matches!(inner.as_ref(), Condition::Matches(Reference::It, _))),
        "guarded on `Not(Matches(It, Orc))`, got {:?}",
        becomes.condition,
    );
    let OneShotEffect::Continuously(Continuously { effect, duration }) = becomes.then.as_ref()
    else {
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
    let StaticEffect::Modify(Reference::It, Modification::Subtypes(CollectionOp::Add(added))) =
        effect.as_ref()
    else {
        panic!("it ADDS the subtype to the chosen Army, got {effect:?}");
    };
    assert_eq!(added.as_str(), "Orc", "becomes an Orc in addition");
}
