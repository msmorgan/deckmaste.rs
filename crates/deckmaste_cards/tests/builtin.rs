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
use deckmaste_core::Selection;
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
/// one reference value splices into BOTH slot kinds: bare into the `Selection`
/// slots (`subject`, the heal/tap) and wrapped `Ref(...)` into the event
/// `would`'s `Filter` slot. Both the self form (`This`) and the bound-target
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
        Selection::this(),
        "subject lands as a bare Selection::Ref"
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

    // Regenerate(Target(0)): the bound-target form parses too — the param is a
    // Reference, so a `Target` fits exactly where `This` did.
    let tgt: Effect = plugin.macros.read_str("Regenerate(Target(0))").unwrap();
    let Effect::Expanded(tex) = tgt else {
        panic!("Regenerate(Target(0)) is remembered as Expanded");
    };
    assert!(
        matches!(
            *tex.value,
            Effect::Act(Action::CreateReplacement {
                subject: Selection::Ref(Reference::Target(0)),
                ..
            })
        ),
        "Regenerate(Target(0)) expands with a Target subject, got {:?}",
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
