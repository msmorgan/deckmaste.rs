//! Loads the builtin plugin's real data files: subtype macro definitions,
//! the subtype declarations invoking them, and the basic land cards whose
//! type lines reference the result.

use std::path::Path;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::ron::options as ron_options;
use deckmaste_core::{
    Ability, Action, Card, CardFace, CharacteristicFilter, Effect, Filter, ManaCost, ObjectKind,
    Quantity, Selection, SpellAbility, StatValue, StateFilter, Subtype, Supertype, TargetSpec,
    Type, Zone,
};

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn basic_land(name: &str) -> Card {
    Card::Normal(CardFace {
        name: name.to_owned(),
        mana_cost: ManaCost::default(),
        supertypes: vec![Supertype::Basic],
        types: vec![Type::Land],
        subtypes: vec![Subtype {
            name: name.into(),
            types: vec![Type::Land],
        }],
        ..Default::default()
    })
}

#[test]
fn basic_lands_parse_against_the_subtype_macros() {
    let plugin = builtin();
    assert!(
        plugin.macros.get("Subtype", "LandType").is_some(),
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
            Some(&Subtype {
                name: name.into(),
                types: vec![Type::Land],
            })
        );

        // Declared subtypes are nullary macros expanding to themselves.
        let expanded: Subtype = plugin.macros.read_str(name).unwrap();
        assert_eq!(Some(&expanded), plugin.subtypes.get(name));
    }
}

/// The `CreatureType` macro path through real data: `subtypes: [Bear]`
/// resolves the declaration, which invokes `CreatureType("Bear")`.
#[test]
fn grizzly_bears_expand_the_creature_type_macro() {
    let card = builtin().card("Grizzly Bears").unwrap();
    let Card::Normal(face) = card else {
        panic!("Grizzly Bears should be single-faced");
    };
    assert_eq!(face.types, vec![Type::Creature]);
    assert_eq!(
        face.subtypes,
        vec![Subtype {
            name: "Bear".into(),
            types: vec![Type::Creature, Type::Kindred],
        }]
    );
    assert_eq!(face.power, Some(StatValue::Number(2)));
    assert_eq!(face.toughness, Some(StatValue::Number(2)));
}

/// Filter-position interception through real data: `Target(AnyTarget)`
/// expands `AnyTarget` at the Selection's Filter payload.
#[test]
fn lightning_bolt_expands_filter_macros() {
    let card = builtin().card("Lightning Bolt").unwrap();
    let Card::Normal(face) = card else {
        panic!("Lightning Bolt should be single-faced");
    };
    let permanent_of = |t: Type| {
        Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(t)),
        ])
    };
    let any_target = Filter::OneOf(vec![
        permanent_of(Type::Battle),
        permanent_of(Type::Creature),
        permanent_of(Type::Planeswalker),
        Filter::Kind(ObjectKind::Player),
    ]);
    assert_eq!(
        face.abilities,
        vec![Ability::Spell(SpellAbility {
            targets: vec![TargetSpec::Target(any_target)],
            effect: Effect::Act(Action::DealDamage(
                Selection::Target(0),
                Quantity::Literal(3)
            )),
        })]
    );
}

#[test]
fn subtypes_round_trip_plainly() {
    let forest = Subtype {
        name: "Forest".into(),
        types: vec![Type::Land],
    };
    let written = ron_options().to_string(&forest).unwrap();
    let parsed: Subtype = ron_options().from_str(&written).unwrap();
    assert_eq!(parsed, forest);
}
