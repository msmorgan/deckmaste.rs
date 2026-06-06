use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

mod data;
mod layout;
mod migrations;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawCharacteristics {
    name: String,
    mana_cost: Option<String>,
    supertypes: Vec<String>,
    types: Vec<String>,
    subtypes: Vec<String>,
    text: String,
}

#[derive(Debug, Parser)]
struct Args {
    pub plugin_dir: PathBuf,
    pub migration_number: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.migration_number {
        Some(number) => migrations::apply(&args.plugin_dir, number)?,
        None => migrations::apply_all(&args.plugin_dir)?,
    }
    //
    // let atomic_cards = data::atomic_cards()?;
    // let _comp_rules = data::comprehensive_rules()?;
    //
    // let legal_cards = atomic_cards
    //     .data
    //     .into_values()
    //     .map(|faces| {
    //         faces
    //             .into_iter()
    //             .filter(
    //                 |face| match face.legalities.vintage.as_ref().map(String::as_str) {
    //                     None | Some("Banned") => false,
    //                     _ => true,
    //                 },
    //             )
    //             .collect::<Vec<_>>()
    //     })
    //     .filter(|faces| !faces.is_empty())
    //     .collect::<Vec<_>>();
    //
    // println!("{} cards are legal in Vintage", legal_cards.len());
    //
    // // println!("Hello, world!");

    Ok(())
}
