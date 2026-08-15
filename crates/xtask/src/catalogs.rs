//! Derive the catalog files consumed by deckmaste from local authoritative
//! sources, without querying Scryfall.

use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use deckmaste_catalogs::legacy::LegacyCatalogKind;
use deckmaste_catalogs::legacy::LegacyCatalogSet;

#[derive(Debug, Args)]
pub struct CatalogArgs {
    /// Comprehensive Rules text snapshot.
    #[arg(long, default_value = "data/rules/cr.txt")]
    cr: PathBuf,

    /// MTGJSON atomic card snapshot used only for observed keyword variants.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    atomic: PathBuf,

    /// Directory for the twelve bare text catalogs.
    #[arg(long, default_value = "data/gen/catalogs")]
    output: PathBuf,
}

pub fn run(args: &CatalogArgs) -> anyhow::Result<()> {
    let cr = std::fs::read_to_string(&args.cr)
        .with_context(|| format!("reading {}", args.cr.display()))?;
    let atomic = std::fs::read(&args.atomic)
        .with_context(|| format!("reading {}", args.atomic.display()))?;

    let catalogs = LegacyCatalogSet::generate(&cr, &atomic)?;
    catalogs.write_to(&args.output)?;

    for kind in LegacyCatalogKind::GENERATED {
        println!(
            "{}: {}",
            kind.filename().trim_end_matches(".txt"),
            catalogs.get(kind).len()
        );
    }
    println!("wrote {}", args.output.display());
    Ok(())
}
