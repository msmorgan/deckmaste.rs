//! Workspace automation: a pure dispatcher over the other crates' CLI
//! entry points. clap owns the top level; `validate`/`card`/`migrate`
//! forward their raw args to the entry points the standalone bins use,
//! while `cite` is parsed here.

use std::ffi::OsString;

use clap::{Parser, Subcommand};
use xtask::cite::{self, CiteArgs};

/// Workspace automation: validate, card, migrate, and CR-citation tasks.
#[derive(Debug, Parser)]
#[command(name = "cargo xtask", bin_name = "cargo xtask")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Validate every finished card in a plugin (defaults to plugins/builtin).
    //
    // `disable_help_flag` lets `--help`/`-h` fall through into `rest` so the
    // sub-CLI renders its own help, the same as `cargo card`/`cargo migrate`.
    #[command(disable_help_flag = true)]
    Validate {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        rest: Vec<OsString>,
    },
    /// Show a card as parsed from a plugin, with its macros expanded.
    #[command(disable_help_flag = true)]
    Card {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        rest: Vec<OsString>,
    },
    /// Apply migration(s) to a plugin.
    #[command(disable_help_flag = true)]
    Migrate {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        rest: Vec<OsString>,
    },
    /// Check / bless / diff / list CR citations.
    Cite(CiteArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // The forwarding entry points expect full argv; synthesize the program
    // name each subcommand was invoked as so clap's usage/errors read right.
    let prog = |name: &str| std::iter::once(OsString::from(format!("cargo xtask {name}")));

    match cli.command {
        Cmd::Validate { rest } => deckmaste_cards::cli::validate(prog("validate").chain(rest)),
        Cmd::Card { rest } => deckmaste_cards::cli::card(prog("card").chain(rest)),
        Cmd::Migrate { rest } => deckmaste_migrations::cli::run(prog("migrate").chain(rest)),
        Cmd::Cite(args) => cite::dispatch(args),
    }
}
