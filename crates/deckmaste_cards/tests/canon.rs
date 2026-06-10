//! The canon cards (real cards — pipeline output plus marked hand-finished
//! entries, see docs/card-data.md) parsed through the macro-aware reader, on
//! top of the builtin prelude they depend on. Run by plain `cargo test`;
//! wizards is the explicit `cargo xtask validate plugins/wizards`.

use std::path::{Path, PathBuf};

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::{
    Ability, Action, Card, Count, Effect, Reference, Selection, SpellAbility, StatValue, Subtype,
    TargetSpec, Type,
};

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
    // The canon slice: 25 cards at the time of writing, growing per
    // docs/card-data.md.
    assert!(
        validation.valid >= 25,
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

/// Target-position interception through real data: the bare `AnyTarget`
/// invocation expands at the ability's `TargetSpec` announce slot.
#[test]
fn lightning_bolt_expands_target_macros() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Lightning Bolt").unwrap() else {
        panic!("Lightning Bolt should be single-faced");
    };
    // The card's `targets` field is macro-aware: loading it expands the bare
    // `AnyTarget` exactly as reading the macro directly does — interior filter
    // expansions (Battle/Creature/…) and all. Comparing against a fresh read
    // keeps this robust to macro refactors instead of pinning the nested,
    // provenance-bearing expansion by hand.
    let any_target: TargetSpec = plugin.macros.read_str("AnyTarget").unwrap();
    assert_eq!(
        face.abilities,
        vec![Ability::Spell(SpellAbility {
            targets: vec![any_target],
            effect: Effect::Act(Action::DealDamage(
                Selection::Ref(Reference::Target(0)),
                Count::Literal(3)
            )),
        })]
    );
}
