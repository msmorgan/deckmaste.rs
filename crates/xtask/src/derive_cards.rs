//! Reproduce the gitignored Oracle snapshot from MTGJSON's atomic-card dump.

use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeriveCardsArgs {
    /// MTGJSON atomic-card input.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    atomic: PathBuf,

    /// Flat, one-face-per-line Oracle snapshot.
    #[arg(long, default_value = "data/derived/cards.jsonl")]
    output: PathBuf,
}

pub fn run(args: &DeriveCardsArgs) -> anyhow::Result<()> {
    let atomic = std::fs::read(&args.atomic)
        .with_context(|| format!("reading {}", args.atomic.display()))?;
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let output = File::create(&args.output)
        .with_context(|| format!("creating {}", args.output.display()))?;
    let count = deckmaste_migrations::oracle_snapshot::write(&atomic, BufWriter::new(output))?;
    println!("wrote {count} faces to {}", args.output.display());
    Ok(())
}
