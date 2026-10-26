//! `cargo xtask` — workspace automation. xtask owns all of the workspace's CLI
//! parsing; each command's logic lives in the `xtask` library so integration
//! tests can drive it. Run via the `cargo xtask` alias (see
//! `.cargo/config.toml`).

use clap::Parser;
use clap::Subcommand;
use xtask::authoring::ScaffoldIdentityArgs;
use xtask::card::CardArgs;
use xtask::catalogs::CatalogArgs;
use xtask::cite::CiteArgs;
use xtask::derive_cards::DeriveCardsArgs;
use xtask::english::EnglishArgs;
use xtask::expansions::ExpansionsArgs;
use xtask::extract::ExtractArgs;
use xtask::fidelity::FidelityArgs;
use xtask::generate::GenerateArgs;
use xtask::graduate::GraduateArgs;
use xtask::idris_check::IdrisCheckArgs;
use xtask::lean::LeanArgs;
use xtask::lexical::LexicalArgs;
use xtask::map::MapArgs;
use xtask::resolve::ResolveArgs;
use xtask::scryfall_snapshot::ScryfallSnapshotArgs;
use xtask::stubs::StubsArgs;
use xtask::validate::ValidateArgs;

/// Workspace automation for cards, English census, generation, validation,
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
    /// Derive the flat Oracle snapshot from Scryfall Oracle Cards JSONL.
    DeriveCards(DeriveCardsArgs),
    /// Validate every finished card in a plugin (defaults to plugins/builtin).
    Validate(ValidateArgs),
    /// Show a card as parsed from a plugin, with its macros expanded.
    Card(CardArgs),
    /// Print the fully expanded body of every `plugins_v2` macro declaration,
    /// one file each, for `diff -r` between two trees (defaults to
    /// `plugins_v2/builtin` into `target/expansions`).
    Expansions(ExpansionsArgs),
    /// The render-back fidelity gate: diff every finished card's rendered
    /// English against the oracle snapshot (strong form — any unwaivered
    /// difference fails; `--waivers` lists the waiver inventory).
    Fidelity(FidelityArgs),
    /// Generate a plugin's cards (stubs -> extract -> resolve -> graduate).
    Generate(GenerateArgs),
    /// Generate a plugin's keyword/subtype macro stubs.
    Stubs(StubsArgs),
    /// Extract cards/*.ron.todo from the pinned Scryfall snapshot.
    Extract(ExtractArgs),
    /// Resolve and validate the cached Scryfall Oracle Cards bulk snapshot.
    ScryfallSnapshot(ScryfallSnapshotArgs),
    /// Rewrite resolvable Unparsed abilities in a plugin's .ron.todo cards.
    Resolve(ResolveArgs),
    /// Graduate every `cards/*.ron.todo` in a plugin that now parses cleanly.
    Graduate(GraduateArgs),
    /// Check / bless / diff / list / show CR citations.
    Cite(CiteArgs),
    /// Stratify the corpus and run the English census.
    English(EnglishArgs),
    /// Analyze supported raw Oracle text against the independent lexicon.
    Lexical(LexicalArgs),
    /// The anaphora-soundness gate: re-emit each expanded card as a raw
    /// Idris `Core.idr` expression and typecheck it with `idris2 --check`.
    /// One card name = single-card mode; omitted = batch-check the plugin.
    IdrisCheck(IdrisCheckArgs),
    /// Build the native Lean workbench with warnings treated as errors.
    Lean(LeanArgs),
    /// On-demand "bearings" dumps of current code shape (`enums`/`idris`).
    Map(MapArgs),
    /// Write one identity-macro scaffold per reachable variant that needs
    /// one and has no def yet (spec §5); never overwrites an existing def.
    ScaffoldIdentity(ScaffoldIdentityArgs),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Cmd::Catalogs(args) => xtask::catalogs::run(&args),
        Cmd::DeriveCards(args) => xtask::derive_cards::run(&args),
        Cmd::Validate(args) => xtask::validate::run(args),
        Cmd::Card(args) => xtask::card::run(args),
        Cmd::Expansions(args) => xtask::expansions::run(&args),
        Cmd::Fidelity(args) => xtask::fidelity::run(args),
        Cmd::Generate(args) => xtask::generate::run(&args),
        Cmd::Stubs(args) => xtask::stubs::run(&args),
        Cmd::Extract(args) => xtask::extract::run(&args),
        Cmd::ScryfallSnapshot(args) => xtask::scryfall_snapshot::run(&args),
        Cmd::Resolve(args) => xtask::resolve::run(&args),
        Cmd::Graduate(args) => xtask::graduate::run(&args),
        Cmd::Cite(args) => xtask::cite::dispatch(&args),
        Cmd::English(args) => {
            let mut stdout = std::io::stdout().lock();
            xtask::english::run(&args, &mut stdout)
        }
        Cmd::Lexical(args) => {
            let mut stdout = std::io::stdout().lock();
            xtask::lexical::run(&args, &mut stdout)
        }
        Cmd::IdrisCheck(args) => xtask::idris_check::run(&args),
        Cmd::Lean(args) => xtask::lean::run(&args),
        Cmd::Map(args) => xtask::map::run(&args),
        Cmd::ScaffoldIdentity(args) => xtask::authoring::run(&args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogs_command_family_requires_a_subcommand() {
        for command in ["generate", "check", "text"] {
            let cli = Cli::try_parse_from(["cargo xtask", "catalogs", command]).unwrap();
            assert!(matches!(cli.command, Cmd::Catalogs(_)));
        }
        assert!(Cli::try_parse_from(["cargo xtask", "catalogs"]).is_err());
    }

    #[test]
    fn lean_command_family_requires_a_subcommand() {
        let cli = Cli::try_parse_from(["cargo xtask", "lean", "check"]).unwrap();
        assert!(matches!(cli.command, Cmd::Lean(_)));
        for command in ["definitions", "macros"] {
            assert!(Cli::try_parse_from(["cargo xtask", "lean", command]).is_err());
        }
        assert!(
            Cli::try_parse_from(["cargo xtask", "lean", "check", "plugins_v2/testing"]).is_err()
        );
        assert!(Cli::try_parse_from(["cargo xtask", "lean"]).is_err());
        assert!(Cli::try_parse_from(["cargo xtask", "lean-check"]).is_err());
        assert!(Cli::try_parse_from(["cargo xtask", "definition-check"]).is_err());
    }

    #[test]
    fn derive_cards_subcommand_parses_with_defaults() {
        let cli = Cli::try_parse_from(["cargo xtask", "derive-cards"]).unwrap();
        assert!(matches!(cli.command, Cmd::DeriveCards(_)));
    }

    #[test]
    fn catalogs_generate_help_names_canonical_card_face_input() {
        let help = Cli::try_parse_from(["cargo xtask", "catalogs", "generate", "--help"])
            .unwrap_err()
            .to_string();
        assert!(help.contains("Vintage-playable card face names"));
        assert!(!help.contains("observed keyword variants"));
    }
}
