//! The `card` command: show a card as parsed from a plugin, with its macro
//! references expanded. xtask owns the CLI; the loading lives in
//! `deckmaste_cards`.

use std::path::PathBuf;

use clap::Args;
use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Expand;

#[derive(Debug, Args)]
pub struct CardArgs {
    plugin_dir: PathBuf,
    card_name: String,
    /// Keep the `Expanded(Expansion { name, args, .. })` wrapper nodes that
    /// record which macro produced each value. The default output strips
    /// them (`expand_all`), showing the card as the engine evaluates it.
    #[arg(long)]
    show_expansions: bool,
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
        show_expansions,
    } = args;
    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)?;
    let card = plugin.card(&card_name)?;

    println!("{} expands to:\n", plugin.card_path(&card_name).display());
    if show_expansions {
        println!("{card:#?}");
    } else {
        println!("{:#?}", card.expand_all());
    }

    Ok(())
}
