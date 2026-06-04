use anyhow::Context;
use mtgjson::AtomicCards;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod data;
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

fn main() -> anyhow::Result<()> {
    let atomic_cards = data::atomic_cards()?;
    let _comp_rules = data::comprehensive_rules()?;

    let legal_cards = atomic_cards
        .data
        .into_values()
        .map(|faces| {
            faces
                .into_iter()
                .filter(
                    |face| match face.legalities.vintage.as_ref().map(String::as_str) {
                        None | Some("Banned") => false,
                        _ => true,
                    },
                )
                .collect::<Vec<_>>()
        })
        .filter(|faces| !faces.is_empty())
        .collect::<Vec<_>>();

    println!("{} cards are legal in Vintage", legal_cards.len());

    // println!("Hello, world!");

    Ok(())
}
