//! Loads the builtin plugin's real data files: subtype meta-macros,
//! the meta-produced instance definitions, and the basic land cards whose
//! type lines reference the result.

use std::path::{Path, PathBuf};

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::ron::options as ron_options;
use deckmaste_core::{
    Ability, Action, ActivatedAbility, Card, CardFace, Color, ColorOrColorless, CostComponent,
    Count, Effect, ManaCost, ManaSpec, PlayerAction, Property, Reference, Subtype, Supertype, Type,
};

fn builtin_path() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin") }

fn builtin() -> Plugin { Plugin::load(builtin_path()).unwrap() }

/// What a basic land type's declaration expands to: the subtype plus its
/// intrinsic mana ability ([CR#305.6]), conferred as data.
fn basic_land_subtype(name: &str, color: Color) -> Subtype {
    Subtype {
        name: name.into(),
        types: vec![Type::Land],
        confers: vec![Property::Ability(Box::new(Ability::Activated(
            ActivatedAbility {
                cost: vec![CostComponent::Tap],
                condition: None,
                limits: vec![],
                targets: vec![],
                effect: Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::AddMana(
                        Count::Literal(1),
                        ManaSpec::Specific(ColorOrColorless::Color(color)),
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
