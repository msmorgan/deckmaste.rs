use serde::Deserialize;

use crate::DataRoot;
use crate::DataStr;

/// Replaces typographic quotation marks with their ASCII equivalents.
#[must_use]
pub fn normalize_quotes(text: &str) -> String {
    text.replace(['‘', '’'], "'").replace(['“', '”'], "\"")
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keywords<'a> {
    #[serde(borrow)]
    pub keyword_abilities: Vec<DataStr<'a>>,
}

impl<'a> Keywords<'a> {
    /// Parses the `AcademyRuins` keyword-list snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when `bytes` is not valid snapshot JSON.
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

/// Reads the keyword lists file; parse with
/// [`Keywords::parse`], which borrows from the returned bytes.
///
/// # Errors
///
/// Returns an error when the configured keyword-list snapshot cannot be read.
pub fn keywords_bytes() -> anyhow::Result<Vec<u8>> {
    DataRoot::workspace_default().read("rules/keywords.json")
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
