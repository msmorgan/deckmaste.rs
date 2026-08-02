//! The mock cards in `plugins/testing` parsed through the macro-aware reader,
//! on top of the builtin prelude. A mock exists ONLY for a combo no real card
//! carries (see that plugin's cards/README.md); engine tests take everything
//! else from `plugins/canon` (covered by tests/canon.rs).

use std::path::Path;
use std::path::PathBuf;

use deckmaste_core::Card;
use deckmaste_plugin::plugin::Plugin;

fn testing_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing")
}

fn assert_testing_card_name(plugin: &Plugin, name: &str) {
    let card = plugin.card(name).unwrap();
    let face_name = match card {
        Card::Normal(card) => card.name,
        Card::TwoFaced { front, .. } => front.name,
    };
    assert_eq!(face_name.as_ref(), name);
}

#[test]
fn testing_mocks_are_valid() {
    let validation = deckmaste_plugin::validate::validate_plugin(&testing_path()).unwrap();
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
    assert_testing_card_name(&testing, "Trample Deathtouch Creature");
    assert_testing_card_name(&testing, "Trample granter");
    assert_testing_card_name(&testing, "Animate enchantments");
}
