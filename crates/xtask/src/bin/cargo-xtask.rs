//! `cargo xtask` — workspace automation. xtask owns all of the workspace's CLI
//! parsing; each command's logic lives in the `xtask` library so integration
//! tests can drive it. Run via the `cargo xtask` alias (see
//! `.cargo/config.toml`).

use clap::{Parser, Subcommand};
use xtask::card::CardArgs;
use xtask::cite::CiteArgs;
use xtask::extract::ExtractArgs;
use xtask::generate::GenerateArgs;
use xtask::graduate::GraduateArgs;
use xtask::resolve::ResolveArgs;
use xtask::stubs::StubsArgs;
use xtask::validate::ValidateArgs;

/// Workspace automation: validate, card, generate, stubs, extract, resolve,
/// graduate, and CR-citation tasks.
#[derive(Debug, Parser)]
#[command(name = "cargo xtask", bin_name = "cargo xtask")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Validate every finished card in a plugin (defaults to plugins/builtin).
    Validate(ValidateArgs),
    /// Show a card as parsed from a plugin, with its macros expanded.
    Card(CardArgs),
    /// Generate a plugin's cards (stubs -> extract -> resolve -> graduate).
    Generate(GenerateArgs),
    /// Generate a plugin's keyword/subtype macro stubs.
    Stubs(StubsArgs),
    /// Extract cards/*.ron.todo from mtgjson.
    Extract(ExtractArgs),
    /// Rewrite resolvable Unparsed abilities in a plugin's .ron.todo cards.
    Resolve(ResolveArgs),
    /// Graduate every `cards/*.ron.todo` in a plugin that now parses cleanly.
    Graduate(GraduateArgs),
    /// Check / bless / diff / list / show CR citations.
    Cite(CiteArgs),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Cmd::Validate(args) => xtask::validate::run(args),
        Cmd::Card(args) => xtask::card::run(args),
        Cmd::Generate(args) => xtask::generate::run(args),
        Cmd::Stubs(args) => xtask::stubs::run(args),
        Cmd::Extract(args) => xtask::extract::run(args),
        Cmd::Resolve(args) => xtask::resolve::run(args),
        Cmd::Graduate(args) => xtask::graduate::run(args),
        Cmd::Cite(args) => xtask::cite::dispatch(args),
    }
}
