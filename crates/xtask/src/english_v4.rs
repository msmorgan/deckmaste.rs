//! `cargo xtask english-v4`: a thin CLI over `deckmaste_english_v4`'s corpus,
//! stratum profiler and census.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use deckmaste_english_v4::census;
use deckmaste_english_v4::corpus::Corpus;
use deckmaste_lexical_source_v4::LexicalEnvironment;

#[derive(Debug, Args)]
pub struct EnglishV4Args {
    #[command(subcommand)]
    command: EnglishV4Command,
}

#[derive(Debug, Subcommand)]
enum EnglishV4Command {
    /// Profile every Corpus Unit into its Complexity Stratum and run the
    /// census: read, known-wrong-only and unread per Stratum.
    Strata(StrataArgs),
}

#[derive(Debug, Args)]
struct StrataArgs {
    /// The Oracle Cards snapshot.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    data: PathBuf,
    /// Only faces of cards with this name (repeatable).
    #[arg(long = "card")]
    cards: Vec<String>,
    /// A v3 census report (`cargo xtask english-v3 --output`) whose unread
    /// faces are accounted for by Stratum.
    #[arg(long)]
    baseline: Option<PathBuf>,
    /// Where to write the full report as JSON.
    #[arg(long, default_value = "target/english-v4/strata.json")]
    output: PathBuf,
    /// Skeletons listed per Stratum in the summary.
    #[arg(long, default_value_t = 15)]
    top: usize,
    /// Fail when a unit at or below the completed Stratum is not read.
    #[arg(long)]
    check: bool,
}

/// The fields of a v3 census report the baseline needs.
#[derive(serde::Deserialize)]
struct V3Report {
    faces: Vec<V3Face>,
}

#[derive(serde::Deserialize)]
struct V3Face {
    id: String,
    census: String,
}

pub fn run(args: &EnglishV4Args, output: &mut dyn Write) -> Result<()> {
    match &args.command {
        EnglishV4Command::Strata(args) => strata(args, output),
    }
}

fn strata(args: &StrataArgs, output: &mut dyn Write) -> Result<()> {
    let env = LexicalEnvironment::load().context("loading the v4 lexical environment")?;
    let reader = BufReader::new(
        File::open(&args.data).with_context(|| format!("opening {}", args.data.display()))?,
    );
    let corpus = Corpus::read(reader, |face| {
        args.cards.is_empty() || args.cards.iter().any(|c| *c == face.card || *c == face.name)
    })?;
    let judgments: Vec<_> =
        deckmaste_english_v4::judgment::load_dir(&deckmaste_english_v4::judgments_dir())?
            .into_iter()
            .filter(|(name, _)| name != "seed.txt")
            .flat_map(|(_, entries)| entries)
            .collect();
    let baseline = args
        .baseline
        .as_ref()
        .map(|path| -> Result<BTreeMap<String, bool>> {
            let file = BufReader::new(
                File::open(path).with_context(|| format!("opening {}", path.display()))?,
            );
            let report: V3Report = serde_json::from_reader(file)
                .with_context(|| format!("reading {}", path.display()))?;
            Ok(report
                .faces
                .into_iter()
                .map(|f| (f.id, f.census != "no"))
                .collect())
        })
        .transpose()?;
    let report = census::run(&corpus, &env, &judgments, baseline.as_ref());
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    serde_json::to_writer(File::create(&args.output)?, &report)?;
    write!(output, "{}", report.summary(args.top))?;
    writeln!(output, "report: {}", args.output.display())?;
    if args.check && !report.regressions.is_empty() {
        anyhow::bail!(
            "{} units at or below the completed stratum are not read",
            report.regressions.len()
        );
    }
    Ok(())
}
