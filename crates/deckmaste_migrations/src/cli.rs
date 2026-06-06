//! The CLI entry point, shared by this crate's bin and `cargo xtask`.
//! Takes full argv (program name included) so both callers parse
//! identically.

use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;

/// Selects the plugin to migrate, and optionally a single migration to
/// apply instead of all of them in order.
#[derive(Debug, Parser)]
struct Args {
    plugin_dir: PathBuf,
    migration_number: Option<usize>,
}

/// Parses full argv (program name included) and runs the migration(s).
///
/// # Errors
/// If the plugin layout is unusable or a migration fails.
pub fn run<I: IntoIterator<Item = OsString>>(args: I) -> anyhow::Result<()> {
    let args = Args::parse_from(args);

    match args.migration_number {
        Some(number) => crate::migrations::apply(&args.plugin_dir, number),
        None => crate::migrations::apply_all(&args.plugin_dir),
    }
}
