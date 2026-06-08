//! The `card` command: show a card as parsed from a plugin, with its macro
//! references expanded. xtask owns the CLI; the loading lives in
//! `deckmaste_cards`.

use std::path::PathBuf;

use clap::Args;
use deckmaste_cards::plugin::Plugin;

#[derive(Debug, Args)]
pub struct CardArgs {
    plugin_dir: PathBuf,
    card_name: String,
}

/// Parse one card (its builtin sibling prelude in scope) and print its
/// expansion.
///
/// # Errors
/// If the plugin fails to load or the card is missing or invalid.
pub fn run(args: CardArgs) -> anyhow::Result<()> {
    let CardArgs {
        plugin_dir,
        card_name,
    } = args;
    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)?;
    let card = plugin.card(&card_name)?;

    println!("{} expands to:\n", plugin.card_path(&card_name).display());
    println!("{card:#?}");

    Ok(())
}
