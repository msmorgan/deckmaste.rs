use serde::Deserialize;

use crate::data::DataStr;

/// Replaces typographic quotation marks with their ASCII equivalents.
pub(crate) fn normalize_quotes(text: &str) -> String {
    text.replace(['‘', '’'], "'").replace(['“', '”'], "\"")
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keywords<'a> {
    #[serde(borrow)]
    pub keyword_abilities: Vec<DataStr<'a>>,
}

impl<'a> Keywords<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

/// Reads the keyword lists file; parse with
/// [`Keywords::parse`], which borrows from the returned bytes.
pub fn keywords_bytes() -> anyhow::Result<Vec<u8>> {
    super::read_data("rules/keywords.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_typographic_quotes() {
        assert_eq!(
            normalize_quotes("The ‘fox’ said “hi.”"),
            "The 'fox' said \"hi.\""
        );
    }
}
