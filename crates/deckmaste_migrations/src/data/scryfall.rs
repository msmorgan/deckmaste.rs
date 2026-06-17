use serde::Deserialize;

use crate::data::DataStr;

/// A scryfall catalog: a named list of strings. Only the list is modeled.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Catalog<'a> {
    #[serde(borrow)]
    pub data: Vec<DataStr<'a>>,
}

impl<'a> Catalog<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

/// Reads a scryfall catalog file (e.g. "creature-types"); parse with
/// [`Catalog::parse`], which borrows from the returned bytes.
pub fn catalog_bytes(name: &str) -> anyhow::Result<Vec<u8>> {
    super::read_data(&format!("catalogs/{name}.json"))
}
