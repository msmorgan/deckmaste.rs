//! `cargo xtask english` — inspect the English AST and audit unresolved
//! phrases against the local Oracle-text snapshot.

use clap::Args;
use clap::Subcommand;

use self::bracket::BracketArgs;
use self::inspect::InspectArgs;
use self::lint::LintArgs;
use self::performance::PerformanceArgs;
use self::probe::ProbeArgs;
use self::recovery::RecoveryArgs;
use self::roundtrip::RoundtripArgs;
use self::shapes::ShapesArgs;
use self::unknown_phrases::UnknownPhrasesArgs;

mod bracket;
mod data;
mod inspect;
mod lint;
mod performance;
mod probe;
mod recovery;
mod recovery_fingerprints;
mod recovery_worklist;
mod roundtrip;
mod shape;
mod shapes;
mod unknown_phrases;

#[derive(Debug, Args)]
pub struct EnglishArgs {
    #[command(subcommand)]
    command: EnglishCommand,
}

#[derive(Debug, Subcommand)]
enum EnglishCommand {
    /// Display grammatical constituents with angle brackets.
    Bracket(BracketArgs),
    /// Parse and display one card or card face's Oracle text.
    Inspect(InspectArgs),
    /// Rank unresolved phrase leaves across the Oracle-text snapshot.
    Unknown(UnknownPhrasesArgs),
    /// Report recovery, lexical opacity, and packed ambiguity across the
    /// corpus.
    Recovery(RecoveryArgs),
    /// Round-trip supported faces through render and report any mismatches.
    Roundtrip(RoundtripArgs),
    /// Rank rare AST productions across the corpus as misparse candidates.
    Shapes(ShapesArgs),
    /// Report sound structural defects — every finding is proof, not a lead.
    Lint(LintArgs),
    /// Probe the parser directly: free-text parses, ceiling calibration, and
    /// a printed-text law baseline.
    Probe(ProbeArgs),
    /// Audit deterministic single-input parser work before timing corpus runs.
    Performance(PerformanceArgs),
}

pub fn run(args: EnglishArgs) -> anyhow::Result<()> {
    match args.command {
        EnglishCommand::Bracket(args) => bracket::run(&args),
        EnglishCommand::Inspect(args) => inspect::run(&args),
        EnglishCommand::Unknown(args) => unknown_phrases::run(&args),
        EnglishCommand::Recovery(args) => recovery::run(&args),
        EnglishCommand::Roundtrip(args) => roundtrip::run(&args),
        EnglishCommand::Shapes(args) => shapes::run(&args),
        EnglishCommand::Lint(args) => lint::run(&args),
        EnglishCommand::Probe(args) => probe::run(&args),
        EnglishCommand::Performance(args) => performance::run(&args),
    }
}
