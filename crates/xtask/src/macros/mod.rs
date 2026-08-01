//! `cargo xtask macro` — frame-layer tooling: dump a compiled frame
//! (`inspect`), keep a macro's legacy `template:` field honest against its
//! `frames:` (`templates --check`/`--write`, the D10 coexistence contract),
//! and read the frame set against real cards (`pilot`, `census`,
//! `residuals`). Mirrors `crate::english`'s command-wiring shape: one file
//! per subcommand, this module owns only the `clap::Subcommand` dispatch.

use std::path::Path;
use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use self::census::CensusArgs;
use self::inspect::InspectArgs;
use self::pilot::PilotArgs;
use self::residuals::ResidualArgs;
use self::templates::TemplatesArgs;

mod census;
mod inspect;
mod pilot;
mod residuals;
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
    /// The pilot gate battery: G3 shadow parity (+ `fidelity`) and G4 ground
    /// truth, over real canon cards.
    Pilot(PilotArgs),
    /// The D10 ratchet's reading: framed defs / true macro definitions,
    /// per-gate pilot status, and the corpus-wide excepted-template count.
    Census(CensusArgs),
    /// Classify every canon ability line against the whole lexicon and rank
    /// the residual shapes the matcher still cannot recover.
    Residuals(ResidualArgs),
}

pub fn run(args: MacroArgs) -> anyhow::Result<()> {
    match args.command {
        MacroCommand::Inspect(args) => inspect::run(args),
        MacroCommand::Templates(args) => templates::run(args),
        MacroCommand::Pilot(args) => pilot::run(args),
        MacroCommand::Census(args) => census::run(args),
        MacroCommand::Residuals(args) => residuals::run(args),
    }
}

/// This workspace's `plugins/builtin`, the default every subcommand here
/// falls back to when its plugin-dir argument is omitted — same convention
/// as `xtask::validate`/`xtask::card`.
fn default_plugin_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
}
