//! The noncanon plugin parses and validates on top of the builtin prelude.

use std::path::Path;
use std::path::PathBuf;

fn noncanon_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/noncanon")
}

#[test]
fn noncanon_plugin_is_valid() {
    let validation = deckmaste_cards::validate::validate_plugin(&noncanon_path()).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    // At least the wave-0 graduates: Shock + Llanowar Elves.
    assert!(
        validation.valid >= 2,
        "only {} items checked",
        validation.valid
    );
}
