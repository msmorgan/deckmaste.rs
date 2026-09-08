//! Reproduce the gitignored flat Oracle snapshot from Scryfall JSONL.

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeriveCardsArgs {
    /// Scryfall Oracle Cards JSONL input.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    oracle_cards: PathBuf,

    /// Declared catalogs used to derive structured type-line labels.
    #[arg(long, default_value = "data/gen/catalogs")]
    catalogs: PathBuf,

    /// Flat, one-face-per-line Oracle snapshot.
    #[arg(long, default_value = "data/derived/cards.jsonl")]
    output: PathBuf,
}

pub fn run(args: &DeriveCardsArgs) -> anyhow::Result<()> {
    let oracle_cards = File::open(&args.oracle_cards)
        .with_context(|| format!("opening {}", args.oracle_cards.display()))?;
    let catalogs = deckmaste_catalogs::CatalogSet::load(&args.catalogs)
        .with_context(|| format!("loading catalogs from {}", args.catalogs.display()))?;
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let output = File::create(&args.output)
        .with_context(|| format!("creating {}", args.output.display()))?;
    let count = deckmaste_migrations::oracle_snapshot::write(
        BufReader::new(oracle_cards),
        &catalogs,
        BufWriter::new(output),
    )?;
    println!("wrote {count} faces to {}", args.output.display());
    Ok(())
}
