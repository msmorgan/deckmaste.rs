//! `cargo xtask extract <plugin>` — (re)generate `cards/*.ron.todo` from
//! mtgjson. Thin wrapper over [`deckmaste_migrations::extract::extract_cards`].

use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct ExtractArgs {
    /// The plugin directory to extract into (e.g. plugins/wizards).
    plugin_dir: PathBuf,
}

/// Extract every supported card from mtgjson into `cards/*.ron.todo`.
///
/// # Errors
/// If source data is unreadable or a card fails to render.
#[allow(clippy::needless_pass_by_value)]
pub fn run(args: ExtractArgs) -> anyhow::Result<()> {
    deckmaste_migrations::extract::extract_cards(&args.plugin_dir)
}
