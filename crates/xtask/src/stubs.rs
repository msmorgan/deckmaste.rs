//! `cargo xtask stubs <plugin>` — generate a plugin's subtype macro stubs
//! (formerly the `_000`-`_003` migrations).

use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct StubsArgs {
    /// The plugin directory to generate stubs into.
    plugin_dir: PathBuf,
}

/// # Errors
/// If the plugin layout is unusable or a generator fails.
pub fn run(args: &StubsArgs) -> anyhow::Result<()> {
    deckmaste_migrations::stubs::generate_stubs(&args.plugin_dir)
}
