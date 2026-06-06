//! Minimal models of the MTGJSON atomic card data: only the fields the
//! migrations use, with `DataStr` instead of closed enums for the fields
//! MTGJSON adds variants to (layouts, colors), so data updates can't break
//! deserialization.
//!
//! The model borrows from the underlying file bytes (see
//! [`crate::data::atomic_cards_bytes`]); strings only allocate when their
//! JSON contains escape sequences. Consumers clone out what they keep.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::DataStr;

#[derive(Clone, Debug, Deserialize)]
pub struct AtomicCards<'a> {
    /// Cards grouped by full name; one entry per face.
    #[serde(borrow)]
    pub data: HashMap<DataStr<'a>, Vec<AtomicCard<'a>>>,
}

impl<'a> AtomicCards<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> { serde_json::from_slice(bytes) }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCard<'a> {
    /// The full card name; faces of multi-face cards share it.
    #[serde(borrow)]
    pub name: DataStr<'a>,
    /// The name of this face, for multi-face cards.
    #[serde(borrow, default)]
    pub face_name: Option<DataStr<'a>>,
    /// Symbols like "{2}{W/U}{X}".
    #[serde(borrow, default)]
    pub mana_cost: Option<DataStr<'a>>,
    /// Single-letter color codes ("W", "U", ...).
    #[serde(borrow, default)]
    pub color_indicator: Option<Vec<DataStr<'a>>>,
    #[serde(borrow)]
    pub types: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub supertypes: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub subtypes: Vec<DataStr<'a>>,
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
    /// snake_case layout name, e.g. "normal", "modal_dfc".
    #[serde(borrow)]
    pub layout: DataStr<'a>,
    #[serde(borrow)]
    pub legalities: Legalities<'a>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Legalities<'a> {
    #[serde(borrow, default)]
    pub vintage: Option<DataStr<'a>>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "manaCost": "{1}{W}",
            "name": "Front // Back",
            "defense": "4",
            "subtypes": ["Time Lord"],
            "supertypes": ["Legendary"],
            "text": "Flying\nProtection from \"quotes\"",
            "types": ["Battle"]
        }"#;
        let card: AtomicCard = serde_json::from_str(json).unwrap();
        assert_eq!(card.face_name.as_deref(), Some("Front"));
        assert_eq!(card.layout.as_str(), "some_future_layout");
        assert_eq!(card.legalities.vintage.as_deref(), Some("Legal"));
        assert_eq!(card.defense.as_deref(), Some("4"));
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
