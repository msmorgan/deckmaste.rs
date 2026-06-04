use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use anyhow::Context;
use mtgjson::AtomicCards;
use serde::de::DeserializeOwned;

pub mod academyruins;
mod scryfall;

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let path = path.as_ref();
    let file = File::open(path).with_context(|| format!("Failed to open file: {path:?}"))?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to deserialize AtomicCards from: {path:?}"))?;
    Ok(data)
}

pub fn atomic_cards() -> anyhow::Result<AtomicCards> {
    load_json(data_dir().join("mtgjson/AtomicCards.json"))
}

pub fn all_printings() -> anyhow::Result<mtgjson::AllPrintings> {
    load_json(data_dir().join("mtgjson/AllPrintings.json"))
}

pub fn comprehensive_rules() -> anyhow::Result<academyruins::RulesMap> {
    load_json(data_dir().join("rules/cr.json"))
}

pub fn keywords() -> anyhow::Result<academyruins::Keywords> {
    load_json(data_dir().join("rules/keywords.json"))
}

pub fn card_types() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/card-types.json")).map(|catalog| catalog.data)
}

pub fn artifact_types() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/artifact-types.json")).map(|catalog| catalog.data)
}

pub fn battle_types() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/battle-types.json")).map(|catalog| catalog.data)
}

pub fn enchantment_types() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/enchantment-types.json")).map(|catalog| catalog.data)
}

pub fn land_types() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/land-types.json")).map(|catalog| catalog.data)
}

pub fn planeswalker_types() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/planeswalker-types.json")).map(|catalog| catalog.data)
}

pub fn keyword_abilities() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/keyword-abilities.json")).map(|catalog| catalog.data)
}

pub fn keyword_actions() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/keyword-actions.json")).map(|catalog| catalog.data)
}

pub fn ability_words() -> anyhow::Result<Vec<String>> {
    load_json::<scryfall::Catalog>(data_dir().join("catalogs/ability-words.json")).map(|catalog| catalog.data)
}