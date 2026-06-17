//! The mock cards in `plugins/testing` parsed through the macro-aware reader,
//! on top of the builtin prelude. A mock exists ONLY for a combo no real card
//! carries (see that plugin's cards/README.md); engine tests take everything
//! else from `plugins/canon` (covered by tests/canon.rs).

use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::plugin::Plugin;

fn testing_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing")
}

#[test]
fn testing_mocks_are_valid() {
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
        validation.valid >= 3,
        "only {} items checked",
        validation.valid
    );

    let testing = Plugin::load_with_sibling_prelude(testing_path()).unwrap();
    assert!(testing.card("Trample Deathtouch Creature").is_ok());
    assert!(testing.card("Trample granter").is_ok());
    assert!(testing.card("Animate enchantments").is_ok());
}
