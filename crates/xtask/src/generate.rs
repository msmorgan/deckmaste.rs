//! `cargo xtask generate <plugin>` — build a plugin's cards from mtgjson via
//! the full pipeline: extract -> resolve -> graduate.
//!
//! Extraction deserializes the ~600 MB `AllPrintings.json`, which is slow under
//! an unoptimized build — the root `Cargo.toml` raises the dev `opt-level` of
//! `deckmaste_migrations` and `serde_json` so this stays fast to *run* without
//! a release build of the whole tree.

use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct GenerateArgs {
    /// The plugin directory to (re)generate (e.g. plugins/wizards).
    plugin_dir: PathBuf,
}

/// Run extract -> resolve -> graduate on the plugin.
///
/// # Errors
/// If any stage fails (unreadable source data, unparsable `.ron.todo`, etc.).
#[allow(clippy::needless_pass_by_value)]
pub fn run(args: GenerateArgs) -> anyhow::Result<()> {
    deckmaste_migrations::extract::extract_cards(&args.plugin_dir)?;
    deckmaste_migrations::resolve::resolve_cards(&args.plugin_dir)?;
    // Reuse graduate's reporting (mirror what `xtask::graduate::run` does).
    let report = deckmaste_cards::graduate::graduate_plugin(&args.plugin_dir)?;
    eprintln!(
        "{}: graduated {}, {} still in progress",
        args.plugin_dir.display(),
        report.graduated.len(),
        report.remaining
    );
    Ok(())
}
