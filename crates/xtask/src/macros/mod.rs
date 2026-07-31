//! `cargo xtask macro` — frame-layer tooling: dump a compiled frame
//! (`inspect`) and keep a macro's legacy `template:` field honest against
//! its `frames:` (`templates --check`/`--write`, the D10 coexistence
//! contract). Mirrors `crate::english`'s command-wiring shape: one file per
//! subcommand, this module owns only the `clap::Subcommand` dispatch.

use std::path::Path;
use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use self::inspect::InspectArgs;
use self::templates::TemplatesArgs;

mod inspect;
mod templates;

#[derive(Debug, Args)]
pub struct MacroArgs {
    #[command(subcommand)]
    command: MacroCommand,
}

#[derive(Debug, Subcommand)]
enum MacroCommand {
    /// Dump a compiled frame: text, kind, holes with classes and paths,
    /// agreement dependencies, and guards.
    Inspect(InspectArgs),
    /// Check or rewrite a macro's `template:` field against
    /// projection(first frame).
    Templates(TemplatesArgs),
}

pub fn run(args: MacroArgs) -> anyhow::Result<()> {
    match args.command {
        MacroCommand::Inspect(args) => inspect::run(args),
        MacroCommand::Templates(args) => templates::run(args),
    }
}

/// This workspace's `plugins/builtin`, the default every subcommand here
/// falls back to when its plugin-dir argument is omitted — same convention
/// as `xtask::validate`/`xtask::card`.
fn default_plugin_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
}
