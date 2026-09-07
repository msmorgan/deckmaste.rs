//! The whole `plugins_v2` corpus through the reader, with no Lean involved.
//!
//! `cargo xtask lean-check` proves the cards; it is the only thing that read
//! the corpus, and the Rust gate did not. A dialect change that broke a card
//! therefore surfaced only in an xtask run — twice, in the first landing of
//! `plugins-v2-dialect`. This test is the Rust half: every declaration body
//! in `plugins_v2/builtin/macros/<family>/`, every card and token and rules
//! row in `plugins_v2/canon` and `plugins_v2/testing`, read through the same
//! `MacroSet` a real load uses. Zero refusals, and the counts printed so a
//! silently emptied corpus is visible.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_semantics_v2::reader::Plugin;

fn plugins_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2")
}

/// `plugins_v2/builtin` — every family's declaration bodies.
#[test]
fn every_builtin_declaration_reads() {
    let root = plugins_root().join("builtin");
    let builtin = Plugin::load(&root).expect("the builtin declarations read");
    let families: std::collections::BTreeSet<&str> = builtin
        .declarations
        .keys()
        .map(|(kind, _)| kind.as_str())
        .collect();
    println!(
        "plugins_v2/builtin: {} declaration(s) across {} kind(s)",
        builtin.declarations.len(),
        families.len()
    );
    assert!(
        builtin.declarations.len() >= 400,
        "only {} declarations read; the scan lost the corpus it reads",
        builtin.declarations.len()
    );
}

/// Every card, token and rules row of the two card plugins, over `builtin`.
#[test]
fn every_card_reads() {
    let root = plugins_root();
    let builtin = Plugin::load(root.join("builtin")).expect("the prelude reads");
    let mut total = 0;
    for name in ["canon", "testing"] {
        let plugin = Plugin::load_with_prelude(&builtin, root.join(name))
            .unwrap_or_else(|error| panic!("plugins_v2/{name} reads: {error}"));
        println!(
            "plugins_v2/{name}: {} card(s), {} token(s), {} sba / {} conferral / {} damage row(s)",
            plugin.cards.len(),
            plugin.tokens.len(),
            plugin.rules.sba.len(),
            plugin.rules.conferral.len(),
            plugin.rules.damage_result.len(),
        );
        total += plugin.cards.len();
    }
    assert!(
        total >= 120,
        "only {total} cards read; the scan lost the corpus it reads"
    );
}
