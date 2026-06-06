use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};

static POOL: LazyLock<Mutex<HashSet<&'static str>>> = LazyLock::new(Default::default);

/// The dumbest possible string interner.
fn intern(s: &str) -> &'static str {
    let mut pool = POOL.lock().unwrap();
    pool.get(s).cloned().unwrap_or_else(|| {
        let interned = Box::leak(s.into());
        pool.insert(interned);
        interned
    })
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
pub struct Ident(&'static str);

impl Ident {
    pub fn new(s: &str) -> Self { Self(intern(s)) }
    pub fn as_str(&self) -> &'static str { self.0 }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self { Self::new(s) }
}

impl std::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target { self.as_str() }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str { self.as_str() }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.as_str().fmt(f) }
}

impl std::borrow::Borrow<str> for Ident {
    fn borrow(&self) -> &str { self.as_str() }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool { self.as_str() == other }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
}

impl<'de> Deserialize<'de> for Ident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Ident;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Ident::new(v))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
