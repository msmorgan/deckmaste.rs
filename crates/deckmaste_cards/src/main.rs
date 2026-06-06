use std::path::PathBuf;

use clap::Parser;
use deckmaste_cards::plugin::Plugin;

/// Shows a card as parsed from a plugin, with its macro references expanded.
#[derive(Debug, Parser)]
struct Args {
    pub plugin_dir: PathBuf,
    pub card_name: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let plugin = Plugin::load(&args.plugin_dir)?;
    let card = plugin.card(&args.card_name)?;

    println!(
        "{} expands to:\n",
        plugin.card_path(&args.card_name).display()
    );
    println!("{card:#?}");

    Ok(())
}
