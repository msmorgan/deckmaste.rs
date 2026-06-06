use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Deserializer};

pub mod academyruins;
pub mod mtgjson;
pub mod scryfall;

fn data_dir() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data") }

fn read_data(relative: &str) -> anyhow::Result<Vec<u8>> {
    let path = data_dir().join(relative);
    std::fs::read(&path).with_context(|| format!("Failed to read file: {path:?}"))
}

/// Reads the atomic cards file; parse with [`mtgjson::AtomicCards::parse`],
/// which borrows from the returned bytes.
pub fn atomic_cards_bytes() -> anyhow::Result<Vec<u8>> { read_data("mtgjson/AtomicCards.json") }

/// Reads the comprehensive rules file; parse with
/// [`academyruins::RulesMap::parse`], which borrows from the returned bytes.
pub fn comprehensive_rules_bytes() -> anyhow::Result<Vec<u8>> { read_data("rules/cr.json") }

/// Reads the keyword lists file; parse with
/// [`academyruins::Keywords::parse`], which borrows from the returned bytes.
pub fn keywords_bytes() -> anyhow::Result<Vec<u8>> { read_data("rules/keywords.json") }

/// Reads a scryfall catalog file (e.g. "creature-types"); parse with
/// [`scryfall::Catalog::parse`], which borrows from the returned bytes.
pub fn catalog_bytes(name: &str) -> anyhow::Result<Vec<u8>> {
    read_data(&format!("catalogs/{name}.json"))
}

/// A string borrowed from the source bytes when its JSON representation is
/// escape-free, owned otherwise.
///
/// `Cow<str>` behind `Option`/`Vec`/map keys always deserializes owned
/// (serde's `#[serde(borrow)]` only rewires top-level `Cow` fields), so this
/// wrapper carries the borrowing visitor everywhere it appears.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Str<'a>(Cow<'a, str>);

impl<'b: 'a, 'a> From<&'b str> for Str<'a> {
    fn from(s: &'b str) -> Self { Str(Cow::Borrowed(s)) }
}

impl Str<'_> {
    pub fn as_str(&self) -> &str { &self.0 }
}

impl Deref for Str<'_> {
    type Target = str;

    fn deref(&self) -> &str { &self.0 }
}

// Lets maps keyed by Str be queried with plain &str.
impl std::borrow::Borrow<str> for Str<'_> {
    fn borrow(&self) -> &str { &self.0 }
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
