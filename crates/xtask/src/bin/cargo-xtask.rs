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
use xtask::english_v3::EnglishV3Args;
use xtask::extract::ExtractArgs;
use xtask::facts::FactsArgs;
use xtask::fidelity::FidelityArgs;
use xtask::gate::GateArgs;
use xtask::generate::GenerateArgs;
use xtask::graduate::GraduateArgs;
use xtask::idris_check::IdrisCheckArgs;
use xtask::lean_check::LeanCheckArgs;
use xtask::lexical::LexicalArgs;
use xtask::macros::MacroArgs;
use xtask::map::MapArgs;
use xtask::resolve::ResolveArgs;
use xtask::scryfall_snapshot::ScryfallSnapshotArgs;
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
    /// Derive the flat Oracle snapshot from Scryfall Oracle Cards JSONL.
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
    /// Print or run the reverse-dependency test gate for changed paths.
    Gate(GateArgs),
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
    /// Inspect parsed Oracle text and audit unresolved English phrases.
    English(EnglishArgs),
    /// Measure all retained v3 readings of supported raw Oracle text.
    #[command(name = "english-v3")]
    EnglishV3(EnglishV3Args),
    /// Analyze supported raw Oracle text against the independent lexicon.
    Lexical(LexicalArgs),
    /// Frame-layer tooling: compiled-frame dumps, `template:` upkeep, and
    /// the gates and sweeps that read the frame set against real cards.
    Macro(MacroArgs),
    /// The anaphora-soundness gate: re-emit each expanded card as a raw
    /// Idris `Core.idr` expression and typecheck it with `idris2 --check`.
    /// One card name = single-card mode; omitted = batch-check the plugin.
    IdrisCheck(IdrisCheckArgs),
    /// The card soundness gate: re-emit every `plugins_v2` card as a Lean
    /// term and prove `Card.check` empty by `decide`, ratcheted per plugin.
    /// No plugin named = every plugin under `plugins_v2/` that has cards.
    #[command(name = "lean-check")]
    LeanCheck(LeanCheckArgs),
    /// On-demand "bearings" dumps of current code shape (`enums`/`idris`).
    Map(MapArgs),
    /// Generate and check the Idris workbench's keyword facts table against
    /// the RON macro stubs that own the label vocabulary.
    Facts(FactsArgs),
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
        Cmd::Gate(args) => xtask::gate::run(&args),
        Cmd::Stubs(args) => xtask::stubs::run(&args),
        Cmd::Extract(args) => xtask::extract::run(&args),
        Cmd::ScryfallSnapshot(args) => xtask::scryfall_snapshot::run(&args),
        Cmd::Resolve(args) => xtask::resolve::run(&args),
        Cmd::Graduate(args) => xtask::graduate::run(&args),
        Cmd::Cite(args) => xtask::cite::dispatch(&args),
        Cmd::English(args) => xtask::english::run(args),
        Cmd::EnglishV3(args) => {
            let mut stdout = std::io::stdout().lock();
            xtask::english_v3::run(&args, &mut stdout)
        }
        Cmd::Lexical(args) => {
            let mut stdout = std::io::stdout().lock();
            xtask::lexical::run(&args, &mut stdout)
        }
        Cmd::Macro(args) => xtask::macros::run(args),
        Cmd::IdrisCheck(args) => xtask::idris_check::run(&args),
        Cmd::LeanCheck(args) => xtask::lean_check::run(&args),
        Cmd::Map(args) => xtask::map::run(&args),
        Cmd::Facts(args) => xtask::facts::run(&args),
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
    fn gate_accepts_its_changed_path_flags_and_acts_only_on_changed() {
        let cli = Cli::try_parse_from([
            "cargo xtask",
            "gate",
            "--changed",
            "--from",
            "default@",
            "--run",
            "--clippy",
        ])
        .expect("gate accepts its changed-path flags");
        assert!(matches!(cli.command, Cmd::Gate(_)));

        let bare = Cli::try_parse_from(["cargo xtask", "gate"]).expect("gate parses bare");
        let Cmd::Gate(args) = bare.command else {
            panic!("`gate` parses as the gate command");
        };
        assert!(xtask::gate::run(&args).is_err());
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
