//! Minimal models of the MTGJSON atomic card data: only the fields the
//! migrations use, with `Str` instead of closed enums for the fields
//! MTGJSON adds variants to (layouts, colors), so data updates can't break
//! deserialization.
//!
//! The model borrows from the underlying file bytes (see
//! [`crate::data::atomic_cards_bytes`]); strings only allocate when their
//! JSON contains escape sequences. Consumers clone out what they keep.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

use serde::{Deserialize, Deserializer};

/// A string borrowed from the source bytes when its JSON representation is
/// escape-free, owned otherwise.
///
/// `Cow<str>` behind `Option`/`Vec`/map keys always deserializes owned
/// (serde's `#[serde(borrow)]` only rewires top-level `Cow` fields), so this
/// wrapper carries the borrowing visitor everywhere it appears.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Str<'a>(Cow<'a, str>);

impl Str<'_> {
    pub fn as_str(&self) -> &str { &self.0 }
}

impl Deref for Str<'_> {
    type Target = str;

    fn deref(&self) -> &str { &self.0 }
}

impl fmt::Display for Str<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}

impl<'de: 'a, 'a> Deserialize<'de> for Str<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StrVisitor<'a>(std::marker::PhantomData<&'a ()>);
        impl<'de: 'a, 'a> serde::de::Visitor<'de> for StrVisitor<'a> {
            type Value = Str<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Str(Cow::Owned(v.to_owned())))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(Str(Cow::Borrowed(v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> { Ok(Str(Cow::Owned(v))) }
        }

        deserializer.deserialize_str(StrVisitor(std::marker::PhantomData))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AtomicCards<'a> {
    /// Cards grouped by full name; one entry per face.
    #[serde(borrow)]
    pub data: HashMap<Str<'a>, Vec<AtomicCard<'a>>>,
}

impl<'a> AtomicCards<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> { serde_json::from_slice(bytes) }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCard<'a> {
    /// The full card name; faces of multi-face cards share it.
    #[serde(borrow)]
    pub name: Str<'a>,
    /// The name of this face, for multi-face cards.
    #[serde(borrow, default)]
    pub face_name: Option<Str<'a>>,
    /// Symbols like "{2}{W/U}{X}".
    #[serde(borrow, default)]
    pub mana_cost: Option<Str<'a>>,
    /// Single-letter color codes ("W", "U", ...).
    #[serde(borrow, default)]
    pub color_indicator: Option<Vec<Str<'a>>>,
    #[serde(borrow)]
    pub types: Vec<Str<'a>>,
    #[serde(borrow)]
    pub supertypes: Vec<Str<'a>>,
    #[serde(borrow)]
    pub subtypes: Vec<Str<'a>>,
    /// Oracle rules text, one line per ability.
    #[serde(borrow, default)]
    pub text: Option<Str<'a>>,
    #[serde(borrow, default)]
    pub power: Option<Str<'a>>,
    #[serde(borrow, default)]
    pub toughness: Option<Str<'a>>,
    #[serde(borrow, default)]
    pub loyalty: Option<Str<'a>>,
    #[serde(borrow, default)]
    pub defense: Option<Str<'a>>,
    /// snake_case layout name, e.g. "normal", "modal_dfc".
    #[serde(borrow)]
    pub layout: Str<'a>,
    #[serde(borrow)]
    pub legalities: Legalities<'a>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Legalities<'a> {
    #[serde(borrow, default)]
    pub vintage: Option<Str<'a>>,
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
        assert!(matches!(card.name, Str(Cow::Borrowed("Front // Back"))));
        assert!(matches!(card.subtypes[0], Str(Cow::Borrowed("Time Lord"))));
        assert!(matches!(card.text, Some(Str(Cow::Owned(_)))));
        assert_eq!(
            card.text.as_deref(),
            Some("Flying\nProtection from \"quotes\"")
        );
    }
}
