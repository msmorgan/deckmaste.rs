//! `cargo xtask english` — inspect the English AST and audit unresolved
//! phrases against the local Oracle-text snapshot.

use clap::Args;
use clap::Subcommand;

use self::inspect::InspectArgs;
use self::recovery::RecoveryArgs;
use self::unknown_phrases::UnknownPhrasesArgs;

mod data;
mod inspect;
mod recovery;
mod unknown_phrases;

#[derive(Debug, Args)]
pub struct EnglishArgs {
    #[command(subcommand)]
    command: EnglishCommand,
}

#[derive(Debug, Subcommand)]
enum EnglishCommand {
    /// Parse and display one card or card face's Oracle text.
    Inspect(InspectArgs),
    /// Rank unresolved phrase leaves across the Oracle-text snapshot.
    Unknown(UnknownPhrasesArgs),
    /// Report structural recovery and licensed lexical opacity by source token.
    Recovery(RecoveryArgs),
}

pub fn run(args: EnglishArgs) -> anyhow::Result<()> {
    match args.command {
        EnglishCommand::Inspect(args) => inspect::run(&args),
        EnglishCommand::Unknown(args) => unknown_phrases::run(&args),
        EnglishCommand::Recovery(args) => recovery::run(&args),
    }
}
