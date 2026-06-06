use serde::Deserialize;

use super::Str;

/// A scryfall catalog: a named list of strings. Only the list is modeled.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Catalog<'a> {
    #[serde(borrow)]
    pub data: Vec<Str<'a>>,
}

impl<'a> Catalog<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> { serde_json::from_slice(bytes) }
}
