//! Minimal models of the MTGJSON atomic card data: only the fields the
//! migrations use, with `String` instead of closed enums for the fields
//! MTGJSON adds variants to (layouts, colors), so data updates can't break
//! deserialization.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct AtomicCards {
    /// Cards grouped by full name; one entry per face.
    pub data: HashMap<String, Vec<AtomicCard>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCard {
    /// The full card name; faces of multi-face cards share it.
    pub name: String,
    /// The name of this face, for multi-face cards.
    #[serde(default)]
    pub face_name: Option<String>,
    /// Symbols like "{2}{W/U}{X}".
    #[serde(default)]
    pub mana_cost: Option<String>,
    /// Single-letter color codes ("W", "U", ...).
    #[serde(default)]
    pub color_indicator: Option<Vec<String>>,
    pub types: Vec<String>,
    pub supertypes: Vec<String>,
    pub subtypes: Vec<String>,
    /// Oracle rules text, one line per ability.
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub defense: Option<String>,
    /// snake_case layout name, e.g. "normal", "modal_dfc".
    pub layout: String,
    pub legalities: Legalities,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Legalities {
    #[serde(default)]
    pub vintage: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_the_fields_we_use() {
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
            "text": "Flying",
            "types": ["Battle"]
        }"#;
        let card: AtomicCard = serde_json::from_str(json).unwrap();
        assert_eq!(card.face_name.as_deref(), Some("Front"));
        assert_eq!(card.layout, "some_future_layout");
        assert_eq!(card.legalities.vintage.as_deref(), Some("Legal"));
        assert_eq!(card.defense.as_deref(), Some("4"));
        assert_eq!(card.power, None);
    }
}
