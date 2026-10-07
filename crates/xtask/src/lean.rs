//! `cargo xtask lean` — the Lean workbench's gates over `plugins_v2` data.
//!
//! Two subcommands:
//!  - `lean check [PLUGIN]…` — the card soundness gate ([`crate::lean_check`]).
//!  - `lean definitions [PLUGIN]` — the registry definition gate
//!    ([`crate::definition_check`]).
//!
//! `facts` stays a top-level command, because it also writes the Idris twin.

use clap::Args;
use clap::Subcommand;

use crate::definition_check::DefinitionCheckArgs;
use crate::lean_check::LeanCheckArgs;

#[derive(Debug, Args)]
pub struct LeanArgs {
    #[command(subcommand)]
    command: LeanCmd,
}

#[derive(Debug, Subcommand)]
enum LeanCmd {
    /// The card soundness gate: re-emit every `plugins_v2` card as a Lean
    /// term and prove `Card.check` empty by `decide`, ratcheted per plugin.
    /// No plugin named = every plugin under `plugins_v2/` that has cards.
    Check(LeanCheckArgs),
    /// The registry definition gate: re-emit every `plugins_v2` Registry
    /// Definition (counters, subtypes, designations) as a Lean term and prove
    /// `Definition.check` empty by `decide`, naming every refused one.
    /// Defaults to `plugins_v2/builtin`.
    Definitions(DefinitionCheckArgs),
}

pub fn run(args: &LeanArgs) -> anyhow::Result<()> {
    match &args.command {
        LeanCmd::Check(args) => crate::lean_check::run(args),
        LeanCmd::Definitions(args) => crate::definition_check::run(args),
    }
}
