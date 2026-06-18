//! `cargo xtask generate <plugin>` — build a plugin's cards from mtgjson via
//! the full pipeline: stubs -> extract -> resolve -> graduate.
//!
//! Extraction deserializes the ~150 MB `AtomicCards.json`, which is slow under
//! an unoptimized build — the root `Cargo.toml` raises the dev `opt-level` of
//! `deckmaste_migrations` and `serde_json` so this stays fast to *run* without
//! a release build of the whole tree.

use std::path::PathBuf;

use anyhow::Context;
use clap::Args;

use crate::graduate::print_report;

#[derive(Debug, Args)]
pub struct GenerateArgs {
    /// The plugin directory to (re)generate (e.g. plugins/wizards).
    plugin_dir: PathBuf,
    /// Skip keyword/action/ability-word stub generation. Subtype stubs and all
    /// card stages still run. Does not require `data/rules/cr.json`.
    #[arg(long)]
    minimal: bool,
}

/// Run stubs -> extract -> resolve -> graduate on the plugin.
///
/// Stubs run first: declaring the plugin's subtypes (as real `.ron`) lets far
/// more cards graduate, since a card referencing an undeclared subtype can't
/// parse.
///
/// # Errors
/// If any stage fails (unreadable source data, unparsable `.ron.todo`, etc.).
#[allow(clippy::needless_pass_by_value)]
pub fn run(args: GenerateArgs) -> anyhow::Result<()> {
    std::fs::create_dir_all(&args.plugin_dir)
        .with_context(|| format!("creating plugin dir {}", args.plugin_dir.display()))?;
    if args.minimal {
        deckmaste_migrations::stubs::generate_subtype_stubs(&args.plugin_dir)?;
    } else {
        deckmaste_migrations::stubs::generate_stubs(&args.plugin_dir)?;
    }
    deckmaste_migrations::extract::extract_cards(&args.plugin_dir)?;
    deckmaste_migrations::resolve::resolve_cards(&args.plugin_dir)?;
    let report = deckmaste_migrations::graduate::graduate_plugin(&args.plugin_dir)?;
    print_report(&args.plugin_dir, &report);
    Ok(())
}
