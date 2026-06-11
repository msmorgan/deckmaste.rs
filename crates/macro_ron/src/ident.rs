use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::RwLock;

use serde::Deserialize;
use serde::Serialize;

static POOL: LazyLock<RwLock<HashSet<&'static str>>> = LazyLock::new(Default::default);

/// The dumbest possible string interner.
fn intern(s: &str) -> &'static str {
    // Fast path: shared read lock for the (overwhelmingly common) hit.
    if let Some(&interned) = POOL.read().unwrap().get(s) {
        return interned;
    }
    let mut pool = POOL.write().unwrap();
    // Re-check under the write lock: another thread may have interned `s`
    // between the read and write locks above.
    pool.get(s).copied().unwrap_or_else(|| {
        let interned = Box::leak(s.into());
        pool.insert(interned);
        interned
    })
}

// `Hash` stays content-based while `PartialEq` is pointer-based; the
// `a == b ⟹ hash(a) == hash(b)` contract holds because pointer-equal implies
// content-equal (see the `PartialEq` impl below).
#[expect(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Clone, Copy, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Ident(&'static str);

impl Ident {
    #[must_use]
    pub fn new(s: &str) -> Self { Self(intern(s)) }
    #[must_use]
    pub fn as_str(&self) -> &'static str { self.0 }
}

/// Pointer equality: [`intern`] returns one canonical allocation per distinct
/// string, so comparing addresses is equivalent to comparing contents — every
/// constructor (including [`Default`] below) goes through [`intern`]. The
/// derived [`Hash`] stays content-based so [`Borrow<str>`](std::borrow::Borrow)
/// map lookups keep working.
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.0, other.0) }
}

/// Routes through [`intern`] (a derived impl would not) so the canonical-
/// pointer invariant holds: `Ident::default() == Ident::new("")`.
impl Default for Ident {
    fn default() -> Self { Self::new("") }
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

/// The one visitor behind both entry points; only the expectation differs.
struct IdentVisitor(&'static str);

impl serde::de::Visitor<'_> for IdentVisitor {
    type Value = Ident;
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { f.write_str(self.0) }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> { Ok(Ident::new(v)) }
}

impl<'de> Deserialize<'de> for Ident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdentVisitor("a string"))
    }
}

/// Reads an [`Ident`] from identifier position: a bare `Forest` or `LandType`
/// token, where the string-position [`Deserialize`] impl above would fail.
///
/// In the serde data model an identifier is an enum variant tag, so this seed
/// only works where the deserializer expects one — pass it to
/// `EnumAccess::variant_seed` after driving `Deserializer::deserialize_enum`.
pub struct IdentSeed;

impl<'de> serde::de::DeserializeSeed<'de> for IdentSeed {
    type Value = Ident;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(IdentVisitor("an identifier"))
    }
}
