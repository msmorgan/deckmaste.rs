//! Minimal models of the MTGJSON atomic card data: only the fields the
//! migrations use, with `DataStr` instead of closed enums for the fields
//! MTGJSON adds variants to (layouts, colors), so data updates can't break
//! deserialization.
//!
//! The model borrows from the underlying file bytes (see
//! [`atomic_cards_bytes`]); strings only allocate when their JSON contains
//! escape sequences. Consumers clone out what they keep.

use std::collections::HashMap;

use serde::Deserialize;

use crate::DataRoot;
use crate::DataStr;

#[derive(Debug, Clone, Deserialize)]
pub struct AtomicCards<'a> {
    /// Cards grouped by full name; one entry per face.
    #[serde(borrow)]
    pub data: HashMap<DataStr<'a>, Vec<AtomicCard<'a>>>,
}

impl<'a> AtomicCards<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCard<'a> {
    /// The full card name; faces of multi-face cards share it.
    #[serde(borrow)]
    pub name: DataStr<'a>,
    /// The name of this face, for multi-face cards.
    #[serde(borrow, default)]
    pub face_name: Option<DataStr<'a>>,
    /// Face ordering marker (`"a"`, `"b"`, …) for multi-face cards.
    #[serde(borrow, default)]
    pub side: Option<DataStr<'a>>,
    /// Symbols like "{2}{W/U}{X}".
    #[serde(borrow, default)]
    pub mana_cost: Option<DataStr<'a>>,
    #[serde(default)]
    pub mana_value: Option<f64>,
    /// The printed type line. `type` is the upstream JSON field name.
    #[serde(borrow, default, rename = "type")]
    pub type_line: Option<DataStr<'a>>,
    /// Single-letter color codes ("W", "U", ...).
    #[serde(borrow, default, deserialize_with = "crate::null_to_default")]
    pub color_indicator: Vec<DataStr<'a>>,
    #[serde(borrow, default, deserialize_with = "crate::null_to_default")]
    pub colors: Vec<DataStr<'a>>,
    #[serde(borrow, default, deserialize_with = "crate::null_to_default")]
    pub color_identity: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub types: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub supertypes: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub subtypes: Vec<DataStr<'a>>,
    /// MTGJSON's broad keyword annotations. Catalog generation further checks
    /// these against the Comprehensive Rules before accepting any value.
    #[serde(borrow, default, deserialize_with = "crate::null_to_default")]
    pub keywords: Vec<DataStr<'a>>,
    /// Oracle rules text, one line per ability.
    #[serde(borrow, default)]
    pub text: Option<DataStr<'a>>,
    #[serde(borrow, default)]
    pub power: Option<DataStr<'a>>,
    #[serde(borrow, default)]
    pub toughness: Option<DataStr<'a>>,
    #[serde(borrow, default)]
    pub loyalty: Option<DataStr<'a>>,
    #[serde(borrow, default)]
    pub defense: Option<DataStr<'a>>,
    /// `snake_case` layout name, e.g. "normal", "`modal_dfc`".
    #[serde(borrow)]
    pub layout: DataStr<'a>,
    #[serde(borrow)]
    pub legalities: Legalities<'a>,
}

impl AtomicCard<'_> {
    #[must_use]
    pub fn vintage_playable(&self) -> bool {
        matches!(
            self.legalities.vintage.as_deref(),
            Some("Legal" | "Restricted")
        )
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Legalities<'a> {
    #[serde(borrow, default)]
    pub vintage: Option<DataStr<'a>>,
}

/// Reads the atomic cards file; parse with [`AtomicCards::parse`],
/// which borrows from the returned bytes.
pub fn atomic_cards_bytes() -> anyhow::Result<Vec<u8>> {
    DataRoot::workspace_default().read("mtgjson/AtomicCards.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card_with_legalities(legalities: &str, layout: &str) -> AtomicCard<'static> {
        let json = Box::leak(
            format!(
                r#"{{
            "name":"Test", "layout":"{layout}",
            "types":[], "supertypes":[], "subtypes":[],
            "legalities":{legalities}
        }}"#
            )
            .into_boxed_str(),
        );
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn vintage_playable_accepts_only_legal_and_restricted() {
        for (value, expected) in [
            (r#"{"vintage":"Legal"}"#, true),
            (r#"{"vintage":"Restricted"}"#, true),
            (r#"{"vintage":"Banned"}"#, false),
            (r#"{"vintage":"Not Legal"}"#, false),
            (r#"{"vintage":null}"#, false),
            (r#"{}"#, false),
        ] {
            assert_eq!(
                card_with_legalities(value, "normal").vintage_playable(),
                expected
            );
        }
    }

    #[test]
    fn vintage_playable_is_independent_of_layout() {
        assert!(
            card_with_legalities(r#"{"vintage":"Legal"}"#, "reversible_card").vintage_playable()
        );
    }

    #[test]
    fn deserializes_the_fields_we_use() {
        use std::borrow::Cow;

        let json = r#"{
            "colorIdentity": ["W"],
            "colors": ["W"],
            "faceName": "Front",
            "foreignData": [{"language": "German"}],
            "layout": "some_future_layout",
            "legalities": {"commander": "Legal", "vintage": "Legal"},
            "keywords": ["Flying"],
            "manaCost": "{1}{W}",
            "manaValue": 2.0,
            "name": "Front // Back",
            "defense": "4",
            "subtypes": ["Time Lord"],
            "supertypes": ["Legendary"],
            "text": "Flying\nProtection from \"quotes\"",
            "type": "Legendary Battle — Siege",
            "types": ["Battle"]
        }"#;
        let card: AtomicCard = serde_json::from_str(json).unwrap();
        assert_eq!(card.face_name.as_deref(), Some("Front"));
        assert_eq!(card.mana_value, Some(2.0));
        assert_eq!(card.type_line.as_deref(), Some("Legendary Battle — Siege"));
        assert_eq!(card.layout.as_str(), "some_future_layout");
        assert_eq!(card.legalities.vintage.as_deref(), Some("Legal"));
        assert_eq!(card.defense.as_deref(), Some("4"));
        assert_eq!(card.keywords[0].as_str(), "Flying");
        assert_eq!(card.power, None);

        // Escape-free strings borrow from the input; escaped ones allocate.
        assert!(matches!(card.name, DataStr(Cow::Borrowed("Front // Back"))));
        assert!(matches!(
            card.subtypes[0],
            DataStr(Cow::Borrowed("Time Lord"))
        ));
        assert!(matches!(card.text, Some(DataStr(Cow::Owned(_)))));
        assert_eq!(
            card.text.as_deref(),
            Some("Flying\nProtection from \"quotes\"")
        );
    }
}
