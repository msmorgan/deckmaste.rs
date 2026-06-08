//! The `migrate` command: build/refine a plugin's data by applying migrations.
//! xtask owns the CLI; the migrations live in `deckmaste_migrations`.
//!
//! Migrations deserialize the ~600 MB `AllPrintings.json`, which is slow under
//! an unoptimized build — the root `Cargo.toml` raises the dev `opt-level` of
//! `deckmaste_migrations` and `serde_json` so this stays fast to *run* without
//! a release build of the whole tree.

use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct MigrateArgs {
    plugin_dir: PathBuf,
    /// Apply a single migration by number instead of all of them in order.
    migration_number: Option<usize>,
}

/// Apply the selected migration(s) to the plugin.
///
/// # Errors
/// If the plugin layout is unusable or a migration fails.
pub fn run(args: MigrateArgs) -> anyhow::Result<()> {
    let MigrateArgs {
        plugin_dir,
        migration_number,
    } = args;
    match migration_number {
        Some(number) => deckmaste_migrations::migrations::apply(&plugin_dir, number),
        None => deckmaste_migrations::migrations::apply_all(&plugin_dir),
    }
}
