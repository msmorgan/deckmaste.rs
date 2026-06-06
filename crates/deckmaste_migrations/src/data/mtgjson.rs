//! Minimal models of the MTGJSON AllPrintings data: only the fields the
//! migrations use, with `DataStr` instead of closed enums for the fields
//! MTGJSON adds variants to (layouts, colors), so data updates can't break
//! deserialization.
//!
//! The model borrows from the underlying file bytes (see
//! [`all_printings_bytes`]); strings only allocate when their
//! JSON contains escape sequences. Consumers clone out what they keep.

use serde::Deserialize;
use serde::de::{IgnoredAny, MapAccess, Visitor};

use super::DataStr;

/// Every printing of every card, grouped by set. The set-code keys of the
/// data map are dropped: consumers iterate, they never look sets up.
#[derive(Clone, Debug)]
pub struct AllPrintings<'a> {
    pub sets: Vec<Set<'a>>,
}

impl<'a> AllPrintings<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> { serde_json::from_slice(bytes) }
}

impl<'de: 'a, 'a> Deserialize<'de> for AllPrintings<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Outer<'a> {
            #[serde(borrow)]
            data: Sets<'a>,
        }
        struct Sets<'a>(Vec<Set<'a>>);
        impl<'de: 'a, 'a> Deserialize<'de> for Sets<'a> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct V<'a>(std::marker::PhantomData<&'a ()>);
                impl<'de: 'a, 'a> Visitor<'de> for V<'a> {
                    type Value = Sets<'a>;

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        f.write_str("a map of sets")
                    }

                    fn visit_map<A: MapAccess<'de>>(
                        self,
                        mut map: A,
                    ) -> Result<Self::Value, A::Error> {
                        let mut sets = Vec::with_capacity(map.size_hint().unwrap_or(0));
                        while let Some((IgnoredAny, set)) = map.next_entry::<IgnoredAny, Set>()? {
                            sets.push(set);
                        }
                        Ok(Sets(sets))
                    }
                }
                deserializer.deserialize_map(V(std::marker::PhantomData))
            }
        }
        Ok(AllPrintings {
            sets: Outer::deserialize(deserializer)?.data.0,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set<'a> {
    /// ISO date ("1993-08-05"); lexical order is chronological order.
    #[serde(borrow)]
    pub release_date: DataStr<'a>,
    #[serde(borrow, default)]
    pub cards: Vec<Card<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card<'a> {
    /// The full card name; faces of multi-face cards share it.
    #[serde(borrow)]
    pub name: DataStr<'a>,
    /// The name of this face, for multi-face cards.
    #[serde(borrow, default)]
    pub face_name: Option<DataStr<'a>>,
    /// Which face of a multi-face card this is ("a", "b", ...).
    #[serde(borrow, default)]
    pub side: Option<DataStr<'a>>,
    /// True when an earlier set already printed this card.
    #[serde(default)]
    pub is_reprint: bool,
    /// True for printings only available in digital clients.
    #[serde(default)]
    pub is_online_only: bool,
    /// True for oversized printings (boxtopper promos and the like), which
    /// are not traditional Magic cards.
    #[serde(default)]
    pub is_oversized: bool,
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
            "side": "a",
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
        let card: Card = serde_json::from_str(json).unwrap();
        assert_eq!(card.face_name.as_deref(), Some("Front"));
        assert_eq!(card.side.as_deref(), Some("a"));
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

    #[test]
    fn parses_the_sets_map_as_a_flat_list() {
        let json = r#"{
            "meta": {"date": "2026-06-01"},
            "data": {
                "LEA": {"releaseDate": "1993-08-05", "cards": []},
                "LEB": {"releaseDate": "1993-10-04", "cards": []}
            }
        }"#;
        let all = AllPrintings::parse(json.as_bytes()).unwrap();
        assert_eq!(all.sets.len(), 2);
        assert_eq!(all.sets[0].release_date.as_str(), "1993-08-05");
    }
}

/// Reads the printings file; parse with [`AllPrintings::parse`],
/// which borrows from the returned bytes.
pub fn all_printings_bytes() -> anyhow::Result<Vec<u8>> {
    super::read_data("mtgjson/AllPrintings.json")
}
