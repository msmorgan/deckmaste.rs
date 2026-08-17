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
use xtask::english_v2::EnglishV2Args;
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
    /// Derive the flat Oracle snapshot from MTGJSON's atomic-card data.
    DeriveCards(DeriveCardsArgs),
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
    /// Inspect the declaration-driven English-v2 grammar.
    #[command(name = "english_v2")]
    EnglishV2(EnglishV2Args),
    /// Frame-layer tooling: compiled-frame dumps, `template:` upkeep, and
    /// the gates and sweeps that read the frame set against real cards.
    Macro(MacroArgs),
    /// The anaphora-soundness gate: re-emit each expanded card as a raw
    /// Idris `Core.idr` expression and typecheck it with `idris2 --check`.
    /// One card name = single-card mode; omitted = batch-check the plugin.
    IdrisCheck(IdrisCheckArgs),
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
        Cmd::Fidelity(args) => xtask::fidelity::run(args),
        Cmd::Generate(args) => xtask::generate::run(&args),
        Cmd::Stubs(args) => xtask::stubs::run(&args),
        Cmd::Extract(args) => xtask::extract::run(&args),
        Cmd::Resolve(args) => xtask::resolve::run(&args),
        Cmd::Graduate(args) => xtask::graduate::run(&args),
        Cmd::Cite(args) => xtask::cite::dispatch(&args),
        Cmd::English(args) => xtask::english::run(args),
        Cmd::EnglishV2(args) => xtask::english_v2::run(&args),
        Cmd::Macro(args) => xtask::macros::run(args),
        Cmd::IdrisCheck(args) => xtask::idris_check::run(&args),
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
    fn derive_cards_subcommand_parses_with_defaults() {
        let cli = Cli::try_parse_from(["cargo xtask", "derive-cards"]).unwrap();
        assert!(matches!(cli.command, Cmd::DeriveCards(_)));
    }

    #[test]
    fn english_v2_commands_require_and_accept_a_subcommand() {
        for command in ["expand", "parse", "roundtrip", "report"] {
            let cli = Cli::try_parse_from(["cargo xtask", "english_v2", command]).unwrap();
            assert!(matches!(cli.command, Cmd::EnglishV2(_)));
        }
        assert!(Cli::try_parse_from(["cargo xtask", "english_v2"]).is_err());
    }

    #[test]
    fn english_v2_report_accepts_only_json() {
        let cli = Cli::try_parse_from(["cargo xtask", "english_v2", "report", "--json"])
            .expect("report accepts JSON output");
        assert!(matches!(cli.command, Cmd::EnglishV2(_)));

        for flag in [
            "--data",
            "--catalogs",
            "--lock",
            "--bless",
            "--require",
            "--require-complete",
            "--require-clean",
            "--trace",
        ] {
            let args = if matches!(flag, "--data" | "--catalogs" | "--lock") {
                vec!["cargo xtask", "english_v2", "report", flag, "fixtures"]
            } else {
                vec!["cargo xtask", "english_v2", "report", flag]
            };
            assert!(Cli::try_parse_from(args).is_err(), "report accepted {flag}");
        }
    }

    #[test]
    fn english_v2_parse_accepts_its_flags_and_expand_rejects_them() {
        let cli = Cli::try_parse_from([
            "cargo xtask",
            "english_v2",
            "parse",
            "--data",
            "fixtures/atomic-cards.json",
            "--catalogs",
            "fixtures/catalogs",
            "--json",
            "--require-complete",
            "--lock",
            "fixtures/coverage.lock",
            "--bless",
        ])
        .unwrap();
        assert!(matches!(cli.command, Cmd::EnglishV2(_)));

        for flag in [
            "--data",
            "--catalogs",
            "--json",
            "--require-complete",
            "--lock",
            "--bless",
        ] {
            let args = if matches!(flag, "--data" | "--catalogs") {
                vec!["cargo xtask", "english_v2", "expand", flag, "fixtures"]
            } else {
                vec!["cargo xtask", "english_v2", "expand", flag]
            };
            assert!(Cli::try_parse_from(args).is_err(), "expand accepted {flag}");
        }
    }

    #[test]
    fn english_v2_roundtrip_accepts_only_its_corpus_and_gate_flags() {
        let cli = Cli::try_parse_from([
            "cargo xtask",
            "english_v2",
            "roundtrip",
            "--data",
            "fixtures/atomic-cards.json",
            "--catalogs",
            "fixtures/catalogs",
            "--json",
            "--require-clean",
        ])
        .unwrap();
        assert!(matches!(cli.command, Cmd::EnglishV2(_)));

        assert!(
            Cli::try_parse_from([
                "cargo xtask",
                "english_v2",
                "roundtrip",
                "--require-complete",
            ])
            .is_err()
        );
        assert!(
            Cli::try_parse_from(["cargo xtask", "english_v2", "parse", "--require-clean",])
                .is_err()
        );
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
