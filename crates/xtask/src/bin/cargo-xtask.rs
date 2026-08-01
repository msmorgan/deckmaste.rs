//! `cargo xtask` — workspace automation. xtask owns all of the workspace's CLI
//! parsing; each command's logic lives in the `xtask` library so integration
//! tests can drive it. Run via the `cargo xtask` alias (see
//! `.cargo/config.toml`).

use clap::Parser;
use clap::Subcommand;
use xtask::card::CardArgs;
use xtask::catalogs::CatalogArgs;
use xtask::cite::CiteArgs;
use xtask::english::EnglishArgs;
use xtask::extract::ExtractArgs;
use xtask::fidelity::FidelityArgs;
use xtask::generate::GenerateArgs;
use xtask::graduate::GraduateArgs;
use xtask::idris_check::IdrisCheckArgs;
use xtask::macros::MacroArgs;
use xtask::map::MapArgs;
use xtask::resolve::ResolveArgs;
use xtask::stubs::StubsArgs;
use xtask::validate::ValidateArgs;

/// Workspace automation for cards, English parsing, generation, validation,
/// and CR-citation tasks.
#[derive(Debug, Parser)]
#[command(name = "cargo xtask", bin_name = "cargo xtask")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Derive the catalogs we consume from the CR and Vintage card data.
    Catalogs(CatalogArgs),
    /// Validate every finished card in a plugin (defaults to plugins/builtin).
    Validate(ValidateArgs),
    /// Show a card as parsed from a plugin, with its macros expanded.
    Card(CardArgs),
    /// The render-back fidelity gate: diff every finished card's rendered
    /// English against the oracle snapshot (strong form — any unwaivered
    /// difference fails; `--waivers` lists the waiver inventory).
    Fidelity(FidelityArgs),
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
    /// Inspect parsed Oracle text and audit unresolved English phrases.
    English(EnglishArgs),
    /// Frame-layer tooling: compiled-frame dumps, `template:` upkeep, and
    /// the gates and sweeps that read the frame set against real cards.
    Macro(MacroArgs),
    /// The anaphora-soundness gate: re-emit each expanded card as a raw
    /// Idris `Core.idr` expression and typecheck it with `idris2 --check`.
    /// One card name = single-card mode; omitted = batch-check the plugin.
    IdrisCheck(IdrisCheckArgs),
    /// On-demand "bearings" dumps of current code shape (`enums`/`idris`).
    Map(MapArgs),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Cmd::Catalogs(args) => xtask::catalogs::run(&args),
        Cmd::Validate(args) => xtask::validate::run(args),
        Cmd::Card(args) => xtask::card::run(args),
        Cmd::Fidelity(args) => xtask::fidelity::run(args),
        Cmd::Generate(args) => xtask::generate::run(&args),
        Cmd::Stubs(args) => xtask::stubs::run(&args),
        Cmd::Extract(args) => xtask::extract::run(&args),
        Cmd::Resolve(args) => xtask::resolve::run(&args),
        Cmd::Graduate(args) => xtask::graduate::run(&args),
        Cmd::Cite(args) => xtask::cite::dispatch(&args),
        Cmd::English(args) => xtask::english::run(args),
        Cmd::Macro(args) => xtask::macros::run(args),
        Cmd::IdrisCheck(args) => xtask::idris_check::run(&args),
        Cmd::Map(args) => xtask::map::run(&args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogs_subcommand_parses_with_defaults() {
        let cli = Cli::try_parse_from(["cargo xtask", "catalogs"]).unwrap();
        assert!(matches!(cli.command, Cmd::Catalogs(_)));
    }
}
