//! Every finished (non-todo) builtin card must parse through the
//! macro-aware reader. Run plain `cargo test` and this guards the prelude
//! everything else depends on; wizards is the explicit
//! `cargo xtask validate plugins/wizards`.

use std::path::Path;

#[test]
fn builtin_cards_are_valid() {
    let builtin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin");
    let validation = deckmaste_cards::validate::validate_plugin(&builtin).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    // The hand-written builtin cards: 5 basics + Lightning Bolt +
    // Grizzly Bears + 3 tokens at the time of writing. Floor, not exact,
    // so adding cards or tokens doesn't break the test.
    assert!(
        validation.valid >= 10,
        "only {} items checked",
        validation.valid
    );
}
