//! Loads the builtin plugin's real data files: subtype meta-macros,
//! the meta-produced instance definitions, and the basic land cards whose
//! type lines reference the result.

use std::path::Path;
use std::path::PathBuf;

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
use deckmaste_core::Effect;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSpec;
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
        confers: vec![Property::Ability(Box::new(Ability::Activated(
            ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: vec![CostComponent::Tap].into(),
                condition: None,
                limits: vec![],
                effect: Effect::Act(Action::By(
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

fn basic_land(name: &str) -> Card {
    Card::Normal(CardFace {
        name: name.to_owned(),
        mana_cost: ManaCost::default(),
        supertypes: vec![Supertype::Basic],
        types: vec![Type::Land],
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
    let effect: Effect = plugin.macros.read_str("Regenerate(This)").unwrap();
    let Effect::Expanded(ref ex) = effect else {
        panic!("a macro invocation is remembered as Expanded, got {effect:?}");
    };
    assert_eq!(ex.name.as_str(), "Regenerate");
    let Effect::Act(Action::CreateReplacement {
        replacement,
        subject,
        duration,
        one_shot,
    }) = (*ex.value).clone()
    else {
        panic!(
            "Regenerate(This) must expand to CreateReplacement, got {:?}",
            ex.value
        );
    };
    assert_eq!(
        subject,
        Reference::This,
        "subject lands as a bare reference"
    );
    assert!(one_shot, "a regeneration shield is one-shot [CR#614.3]");
    assert_eq!(
        duration,
        Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
    );
    // The watched event is a destruction `Instead`; the heal+tap body taps and
    // removes damage from the same reference.
    let Replacement::Instead { instead, .. } = *replacement else {
        panic!("regeneration is an Instead replacement");
    };
    let Effect::Sequence(body) = instead else {
        panic!("the regen body is a Sequence (remove damage, then tap)");
    };
    assert_eq!(body.len(), 2, "remove all damage, then tap [CR#701.19a]");

    // Regenerate(It): the announced-target anaphor parses too — the param is
    // a Reference, so `It` fits exactly where `This` did.
    let tgt: Effect = plugin.macros.read_str("Regenerate(It)").unwrap();
    let Effect::Expanded(tex) = tgt else {
        panic!("Regenerate(It) is remembered as Expanded");
    };
    assert!(
        matches!(
            *tex.value,
            Effect::Act(Action::CreateReplacement {
                subject: Reference::It,
                ..
            })
        ),
        "Regenerate(It) expands with the anaphor subject, got {:?}",
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
    let unless: Effect = plugin
        .macros
        .read_str("Unless(effect: Draw(1), unless: [Mana([Generic(2)])])")
        .unwrap();
    let Effect::Expanded(exp) = unless else {
        panic!("expected a remembered Unless expansion, got {unless:?}");
    };
    assert_eq!(exp.name.as_str(), "Unless");
    let Effect::MustPay(m) = exp.value.as_ref() else {
        panic!("Unless must expand to MustPay, got {:?}", exp.value);
    };
    assert_eq!(m.actor, Reference::You, "the payer defaults to You");
    assert!(
        matches!(
            m.or_else.as_ref(),
            Effect::Act(Action::By(_, PlayerAction::Draw(_)))
        ),
        "or_else carries the unpaid branch"
    );

    // Exile — the render name over the pure zone move ([CR#701.13]).
    let exile: Effect = plugin.macros.read_str("Exile(This)").unwrap();
    let Effect::Expanded(exp) = exile else {
        panic!("expected a remembered Exile expansion");
    };
    assert!(
        matches!(
            exp.value.as_ref(),
            Effect::Act(Action::Move(Reference::This, _, _))
        ),
        "Exile(This) is Move(This, Exile), got {:?}",
        exp.value
    );

    // DestroyNoRegen — destroy + the ForThisEvent-scoped
    // Cant(Regenerate) rider ([CR#701.19c]).
    let dnr: Effect = plugin.macros.read_str("DestroyNoRegen(This)").unwrap();
    let Effect::Expanded(exp) = dnr else {
        panic!("expected a remembered DestroyNoRegen expansion");
    };
    let Effect::Sequence(parts) = exp.value.as_ref() else {
        panic!("DestroyNoRegen is a Sequence, got {:?}", exp.value);
    };
    assert!(matches!(
        parts[0],
        Effect::Act(Action::Destroy(Reference::This))
    ));
    assert!(
        matches!(&parts[1], Effect::Until(Duration::ForThisEvent, statics) if statics.len() == 1),
        "the rider is a ForThisEvent-scoped static, got {:?}",
        parts[1]
    );

    // PreventNext / PreventAll — render names over the Prevention class
    // ([CR#615.7,615.1]), one-shot shields until end of turn.
    let next: Effect = plugin
        .macros
        .read_str("PreventNext(n: 3, to: Creature)")
        .unwrap();
    let Effect::Expanded(exp) = next else {
        panic!("expected a remembered PreventNext expansion");
    };
    assert!(
        matches!(
            exp.value.as_ref(),
            Effect::Continuously(c) if c.duration == Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
        ),
        "PreventNext is an until-end-of-turn shield, got {:?}",
        exp.value
    );
    let all: Effect = plugin.macros.read_str("PreventAll(to: Ref(You))").unwrap();
    assert!(matches!(all, Effect::Expanded(_)));

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
    let deckmaste_core::StaticEffect::CostOption(oc) = s else {
        panic!("Multikicker declares a CostOption");
    };
    assert!(oc.repeatable, "multikicker is the repeatable row");
    assert_eq!(
        oc.tag,
        deckmaste_core::CostTag::from("Kicker"),
        "a multikicker cost IS a kicker cost ([CR#702.33c])"
    );

    // Chapter — sagas as data ([CR#714.2b]): OneOrMore + Crossed.
    let chapter: Ability = plugin
        .macros
        .read_str("Chapter(n: 2, effect: Draw(1))")
        .unwrap();
    let Ability::Expanded(exp) = chapter else {
        panic!("expected a remembered Chapter expansion");
    };
    let Ability::Innate(inner) = exp.value.as_ref() else {
        panic!("a chapter is the rule of the chapter symbol (Innate)");
    };
    let Ability::Triggered(t) = inner.as_ref() else {
        panic!("a chapter is a triggered ability ([CR#714.2])");
    };
    assert!(
        matches!(&t.event, deckmaste_core::EventFilter::OneOrMore(_)),
        "one occurrence per placement batch ([CR#603.3b])"
    );
    assert!(
        matches!(
            &t.condition,
            Some(deckmaste_core::Condition::Crossed { .. })
        ),
        "the [CR#714.2b] was-less-than/became-at-least gate"
    );
}
