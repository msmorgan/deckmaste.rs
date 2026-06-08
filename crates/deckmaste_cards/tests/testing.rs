//! The fake cards in `plugins/testing` (Vanilla Creature, Instant
//! `DealDamage` `AnyTarget`) parsed through the macro-aware reader, on top of
//! the builtin prelude they depend on.

use std::path::{Path, PathBuf};

use deckmaste_cards::plugin::Plugin;

fn testing_path() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing") }

fn testing() -> Plugin {
    // testing sits on top of builtin: its cards reference builtin's macros
    // (CreatureType, AnyTarget, DealDamage) and subtype declarations.
    Plugin::load_with_sibling_prelude(testing_path()).unwrap()
}

#[test]
fn testing_cards_are_valid() {
    let validation = deckmaste_cards::validate::validate_plugin(&testing_path()).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    assert!(
        validation.valid >= 2,
        "only {} items checked",
        validation.valid
    );

    // Confirm lookup by name works for both fake cards.
    assert!(testing().card("Vanilla Creature").is_ok());
    assert!(testing().card("Instant DealDamage AnyTarget").is_ok());
}
