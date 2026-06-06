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
    std::fs::read(&path).with_context(|| format!(r#"Failed to read file: "{}""#, path.display()))
}

/// Deserializes an explicit JSON `null` as the type's default. The upstream
/// data writes `"examples": null` rather than omitting the key, so flattened
/// `Vec` fields need this on top of `#[serde(default)]`.
pub(crate) fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// A string borrowed from the source bytes when its JSON representation is
/// escape-free, owned otherwise.
///
/// `Cow<str>` behind `Option`/`Vec`/map keys always deserializes owned
/// (serde's `#[serde(borrow)]` only rewires top-level `Cow` fields), so this
/// wrapper carries the borrowing visitor everywhere it appears.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataStr<'a>(Cow<'a, str>);

impl<'b: 'a, 'a> From<&'b str> for DataStr<'a> {
    fn from(s: &'b str) -> Self { DataStr(Cow::Borrowed(s)) }
}

impl DataStr<'_> {
    pub fn as_str(&self) -> &str { &self.0 }
}

impl Deref for DataStr<'_> {
    type Target = str;

    fn deref(&self) -> &str { &self.0 }
}

// Lets maps keyed by DataStr be queried with plain &str.
impl std::borrow::Borrow<str> for DataStr<'_> {
    fn borrow(&self) -> &str { &self.0 }
}

impl fmt::Display for DataStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}

impl<'de: 'a, 'a> Deserialize<'de> for DataStr<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DataStrVisitor<'a>(std::marker::PhantomData<&'a ()>);
        impl<'de: 'a, 'a> serde::de::Visitor<'de> for DataStrVisitor<'a> {
            type Value = DataStr<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                Ok(DataStr(Cow::Owned(v.to_owned())))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(DataStr(Cow::Borrowed(v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(DataStr(Cow::Owned(v)))
            }
        }

        deserializer.deserialize_str(DataStrVisitor(std::marker::PhantomData))
    }
}
