use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;
use serde::Deserializer;

pub mod academyruins;
pub mod scryfall;

#[derive(Debug, Clone)]
pub struct DataRoot(PathBuf);

impl DataRoot {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    #[must_use]
    pub fn workspace_default() -> Self {
        Self(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data"))
    }

    /// Reads one file relative to this data root.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read.
    pub fn read(&self, relative: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
        let path = self.0.join(relative);
        std::fs::read(&path).with_context(|| format!("reading {}", path.display()))
    }
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
    fn from(s: &'b str) -> Self {
        DataStr(Cow::Borrowed(s))
    }
}

impl DataStr<'_> {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for DataStr<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

// Lets maps keyed by DataStr be queried with plain &str.
impl std::borrow::Borrow<str> for DataStr<'_> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DataStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
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
