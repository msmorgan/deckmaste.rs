use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::Ident;
use crate::continuous::StaticEffect;

/// A counter kind at a REFERENCE position — `HasCounter(P1P1Counter)`,
/// `CounterCount(This, P1P1Counter)`, `PutCounters(~, P1P1Counter, 2)`:
/// spelled as a bare identifier, exactly like [`KeywordRef`](crate::KeywordRef)
/// and `kinds: [Subtype]`. It is a NAME minted by a `Counter`-kind macro
/// (`P1P1Counter`, `M1M1Counter`, …), never a symbolic string like `"+1/+1"`
/// (which isn't even a legal identifier). Nothing expands — it is a leaf name,
/// matched through [`as_str`](Self::as_str). Name validity (the ref resolves to
/// a loaded counter decl) is checked by a resolution pass right after a plugin
/// loads, like a programming language's link step — not inside serde.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterRef(pub Ident);

impl CounterRef {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
}

impl From<&str> for CounterRef {
    fn from(s: &str) -> Self {
        CounterRef(s.into())
    }
}

impl crate::Expand for CounterRef {
    // A leaf: a name, never an expandable value.
    fn expand_all(self) -> Self {
        self
    }
}

impl Serialize for CounterRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A unit variant writes as a bare identifier in RON.
        serializer.serialize_unit_variant("CounterRef", 0, self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for CounterRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `KeywordRef` and `kinds: [Subtype]` read through.
        struct NameVisitor;
        impl<'de> serde::de::Visitor<'de> for NameVisitor {
            type Value = CounterRef;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a counter name (bare identifier)")
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::VariantAccess;
                let (ident, variant) = data.variant_seed(macro_ron::IdentSeed)?;
                variant.unit_variant()?;
                Ok(CounterRef(ident))
            }
        }
        deserializer.deserialize_enum("", &[], NameVisitor)
    }
}

/// A counter-kind declaration ([CR#122], §6): an open `Ident` vocabulary with
/// an optional payload (e.g. a keyword counter's `GainAbility(Flying)`, a stun
/// / shield counter's replacement payload). This is a declaration-file type
/// (like `MacroDef`); where Filters and Actions reference counters they use a
/// bare `Ident`. No loader wiring yet.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CounterDecl {
    pub name: Ident,
    /// The static effect a counter of this kind confers, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<StaticEffect>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `CounterRef` reads and writes as a BARE identifier (`P1P1Counter`),
    /// never a quoted string — the hand-written serde impls, the highest-risk
    /// arm. (A quoted `"P1P1Counter"` must NOT parse.)
    #[test]
    fn counter_ref_round_trips_bare() {
        let value = CounterRef::from("P1P1Counter");
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(written, "P1P1Counter", "writes bare, no quotes");
        let read: CounterRef = crate::ron::options().from_str("P1P1Counter").unwrap();
        assert_eq!(read, value);
        assert!(
            crate::ron::options()
                .from_str::<CounterRef>("\"P1P1Counter\"")
                .is_err(),
            "a quoted string is not a counter ref"
        );
    }
}
