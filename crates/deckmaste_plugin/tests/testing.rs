//! The mock cards in `plugins/testing` parsed through the macro-aware reader,
//! on top of the builtin prelude. A mock exists ONLY for a combo no real card
//! carries (see that plugin's cards/README.md); engine tests take everything
//! else from `plugins/canon` (covered by tests/canon.rs).

use std::path::Path;
use std::path::PathBuf;

use deckmaste_lowering::Lower;
use deckmaste_plugin::plugin::Plugin;

fn testing_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing")
}

fn assert_testing_card_name(plugin: &Plugin, name: &str) {
    let card = plugin.card(name).unwrap().core;
    assert_eq!(card.primary_face().characteristics.name.as_ref(), name);
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

/// The loader hands back BOTH projections on a real card: the semantic term the
/// spelling side needs and the engine value the engine needs, from one call.
///
/// The equality below is a TAUTOLOGY inside this crate:
/// `Plugin::card_from_str` builds `core` by lowering `semantic`. What it pins
/// is the pair's shape and that the API returns it — `semantic` is a `Lower`
/// whose `Target` is the type of `core`. The informative comparison, the engine
/// image against an independent core-kinded reader, is
/// `tests/corpus_identity.rs`.
#[test]
fn card_load_yields_both_projections_of_one_parse() {
    let plugin = Plugin::load_with_sibling_prelude(testing_path()).unwrap();
    let loaded = plugin.card("Exalted Creature").expect("testing card loads");
    assert_eq!(
        loaded.semantic.clone().lower(),
        loaded.core,
        "the core half must be exactly the lowering of the semantic half"
    );
}
