//! The dumbest possible string interner (pronounced “Dipsy”).

use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::RwLock;

use serde::Deserialize;
use serde::Serialize;

static POOL: LazyLock<RwLock<HashSet<&'static str>>> = LazyLock::new(Default::default);

fn intern(s: &str) -> &'static str {
    // Fast path: shared read lock for the overwhelmingly common hit.
    if let Some(&interned) = POOL.read().unwrap().get(s) {
        return interned;
    }
    let mut pool = POOL.write().unwrap();
    // Another thread may have interned `s` between the two locks.
    pool.get(s).copied().unwrap_or_else(|| {
        let interned = Box::leak(s.into());
        pool.insert(interned);
        interned
    })
}

/// An interned string with cheap copy and equality operations.
#[expect(
    clippy::derived_hash_with_manual_eq,
    reason = "pointer-equal implies content-equal, so Eq implies the same hash"
)]
#[derive(Debug, Clone, Copy, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Ident(&'static str);

impl Ident {
    /// Intern `s` and return its canonical identifier.
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(intern(s))
    }

    /// Return the interned string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

// Every constructor goes through `intern`, so pointer equality is equivalent
// to content equality. The derived Hash remains content-based, preserving the
// `Borrow<str>` lookup contract.
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

// Route the empty string through the interner too, preserving the canonical
// pointer invariant.
impl Default for Ident {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl std::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::borrow::Borrow<str> for Ident {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

struct IdentVisitor(&'static str);

impl serde::de::Visitor<'_> for IdentVisitor {
    type Value = Ident;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0)
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Ident::new(value))
    }
}

impl<'de> Deserialize<'de> for Ident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdentVisitor("a string"))
    }
}

/// A serde seed that reads an [`Ident`] from an identifier position.
///
/// Use this with `EnumAccess::variant_seed` for bare tokens such as `Forest`;
/// [`Ident`]'s ordinary [`Deserialize`] implementation reads string values.
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

#[cfg(test)]
mod tests {
    use super::Ident;

    #[test]
    fn equal_text_shares_one_allocation() {
        let first = Ident::new("Dipsy");
        let owned = String::from("Dipsy");
        let second = Ident::new(&owned);

        assert_eq!(first, second);
        assert!(std::ptr::eq(first.as_str(), second.as_str()));
    }

    #[test]
    fn interning_is_thread_safe() {
        let interns: Vec<_> = (0..8)
            .map(|_| std::thread::spawn(|| Ident::new("DumbestPossibleStringInterner")))
            .map(|thread| thread.join().unwrap())
            .collect();

        assert!(
            interns
                .windows(2)
                .all(|pair| std::ptr::eq(pair[0].as_str(), pair[1].as_str()))
        );
    }
}
