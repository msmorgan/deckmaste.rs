//! The handwritten canon cards (Lightning Bolt, Grizzly Bears) parsed
//! through the macro-aware reader, on top of the builtin prelude they
//! depend on. Run by plain `cargo test`; wizards is the explicit
//! `cargo xtask validate plugins/wizards`.

use std::path::{Path, PathBuf};

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{
    Ability, Action, Card, CharacteristicFilter, Effect, Filter, ObjectKind, Quantity, Selection,
    SpellAbility, StatValue, StateFilter, Subtype, TargetSpec, Type, Zone,
};
use macro_ron::{Expansion, ExpansionArgs};

fn canon_path() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon") }

fn canon() -> Plugin {
    // canon sits on top of builtin: its cards reference builtin's macros
    // (CreatureType, AnyTarget) and subtype declarations, so it needs the
    // sibling `builtin/` prelude — the same convention validate_plugin uses.
    Plugin::load_with_sibling_prelude(canon_path()).unwrap()
}

#[test]
fn canon_cards_are_valid() {
    let validation = deckmaste_cards::validate::validate_plugin(&canon_path()).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    // The handwritten canon cards: Lightning Bolt + Grizzly Bears at the time of
    // writing.
    assert!(
        validation.valid >= 2,
        "only {} items checked",
        validation.valid
    );
}

/// The `CreatureType` macro path through real data: `subtypes: [Bear]`
/// resolves the declaration, which invokes `CreatureType("Bear")`.
#[test]
fn grizzly_bears_expand_the_creature_type_macro() {
    let card = canon().card("Grizzly Bears").unwrap();
    let Card::Normal(face) = card else {
        panic!("Grizzly Bears should be single-faced");
    };
    assert_eq!(face.types, vec![Type::Creature]);
    assert_eq!(
        face.subtypes,
        vec![Subtype {
            name: "Bear".into(),
            types: vec![Type::Creature, Type::Kindred],
            confers: vec![],
        }]
    );
    assert_eq!(face.power, Some(StatValue::Number(2)));
    assert_eq!(face.toughness, Some(StatValue::Number(2)));
}

/// Filter-position interception through real data: `Target(AnyTarget)`
/// expands `AnyTarget` at the Selection's Filter payload.
#[test]
fn lightning_bolt_expands_filter_macros() {
    let card = canon().card("Lightning Bolt").unwrap();
    let Card::Normal(face) = card else {
        panic!("Lightning Bolt should be single-faced");
    };
    let permanent_of = |t: Type| {
        Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(t)),
        ])
    };
    let any_target_value = Filter::OneOf(vec![
        permanent_of(Type::Battle),
        permanent_of(Type::Creature),
        permanent_of(Type::Planeswalker),
        Filter::Kind(ObjectKind::Player),
    ]);
    // `AnyTarget` is a remembered Filter macro: the invocation survives,
    // wrapping the expanded predicate under `.value`.
    let any_target = Filter::Expanded(Expansion {
        name: "AnyTarget".into(),
        args: ExpansionArgs::none(),
        value: Box::new(any_target_value),
    });
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
