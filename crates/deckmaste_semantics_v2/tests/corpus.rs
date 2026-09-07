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

/// Every helper declaration that takes no arguments, invoked by its bare name
/// at its own kind.
///
/// Reading a declaration file only checks that its `body` is well-formed RON;
/// the body is opaque text until something expands it. This is the expansion
/// half: each nullary helper is written where a card would write it and read
/// as the type its kind names, so a body that names a constructor the type
/// does not have — or a macro that did not port — fails here rather than on
/// the first card to write it.
#[test]
fn every_nullary_helper_expands() {
    let builtin = Plugin::load(plugins_root().join("builtin")).expect("the builtin reads");
    let mut expanded = 0;
    let mut untested = 0;
    for ((kind, name), declaration) in &builtin.declarations {
        let nullary = match &declaration.definition.params {
            macro_ron::Params::Positional(types) => types.is_empty(),
            macro_ron::Params::Named(signature) => signature.is_empty(),
        };
        // A BODYLESS declaration — a meta-macro's omitted `body` argument,
        // which several keyword families still are — has nothing to expand.
        if !nullary || declaration.definition.body_head(&builtin.macros).is_none() {
            continue;
        }
        macro_rules! read {
            ($($position:literal => $ty:ty),* $(,)?) => {
                match kind.as_str() {
                    $($position => builtin
                        .macros
                        .read_str::<$ty>(name.as_str())
                        .map(drop)
                        .unwrap_or_else(|error| {
                            panic!("{kind}/{name} ({}): {error}", declaration.path.display())
                        }),)*
                    _ => {
                        untested += 1;
                        continue;
                    }
                }
            };
        }
        read! {
            "NounPhrase" => deckmaste_semantics_v2::phrase::NounPhrase,
            "Predicate" => deckmaste_semantics_v2::phrase::Predicate,
            "Amount" => deckmaste_semantics_v2::phrase::Amount,
            "Quantity" => deckmaste_semantics_v2::phrase::Quantity,
            "ZoneExpr" => deckmaste_semantics_v2::phrase::ZoneExpr,
            "Condition" => deckmaste_semantics_v2::phrase::Condition,
            "GameEvent" => deckmaste_semantics_v2::phrase::GameEvent,
            "Duration" => deckmaste_semantics_v2::triggers::Duration,
            "Instruction" => deckmaste_semantics_v2::abilities::Instruction,
            "StaticSpec" => deckmaste_semantics_v2::abilities::StaticSpec,
            "Cost" => deckmaste_semantics_v2::abilities::Cost,
            "ManaSymbol" => deckmaste_semantics_v2::words::ManaSymbol,
            "ColorTerm" => deckmaste_semantics_v2::phrase::ColorTerm,
        }
        expanded += 1;
    }
    println!("{expanded} nullary declaration(s) expand; {untested} at untested kinds");
    assert!(
        expanded >= 80,
        "only {expanded} expanded; the scan lost the declarations it reads"
    );
}
