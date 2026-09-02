use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::Count;
use crate::Ident;
use crate::Property;

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
                let (ident, variant) = data.variant_seed(crate::IdentSeed)?;
                variant.unit_variant()?;
                Ok(CounterRef(ident))
            }
        }
        deserializer.deserialize_enum("", &[], NameVisitor)
    }
}

/// Which counters a [`MoveCounters`](crate::Action::MoveCounters) operation
/// relocates ([CR#122] — counters move object→object as one operation). The
/// Idris `CounterSpec = Some CounterKind Count | AllKinds`.
///
/// `Named` names a specific kind and how many (Power Conduit / Leech Bonder);
/// `AllKinds` moves every counter of every kind at once (Ozolith / Fate
/// Transfer) — the one case a single-kind remove+put can't reach, because it
/// quantifies over the kinds present.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CounterSpec {
    /// A specific counter kind and count.
    Named(CounterRef, Count),
    /// Every counter, of every kind.
    AllKinds,
}

/// A counter kind's carrier scope ([CR#122.1]): most kinds sit on objects;
/// poison/energy/experience are player-borne ([CR#122.1f] poison). The
/// registry ROW carries this dependent index as data; the counter-scope
/// soundness check (in the Idris model) reads the loaded declaration, not a
/// hardcoded list.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CounterScope {
    /// Object-borne — the [CR#122.1] default (counters sit on objects
    /// unless a player-borne kind says otherwise).
    #[default]
    Object,
    /// Player-borne ([CR#122.1f] poison; energy, experience).
    Player,
}

/// A counter-kind declaration ([CR#122.1]): an identity (`name`, the rusty
/// ident a `CounterRef` resolves to), the carrier `scope` it may sit on
/// (object-borne by default, omitted in RON), plus the bearings it confers
/// on any carrier holding it. Authored as a `Counter`-kind macro (`kinds:
/// [Counter]`, `body: Counter(name: "P1P1Counter", confers: […])`), loaded
/// into the plugin's counter registry. Confers are routed by `Property`
/// flavor — `Continuous` boosts into the layers, `StateBased` SBAs into the
/// 704 sweep.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Counter {
    pub name: Ident,
    #[serde(default, skip_serializing_if = "is_object_scope")]
    pub scope: CounterScope,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub confers: Vec<Property>,
}

/// `skip_serializing_if` for [`Counter::scope`]: the object default is
/// omitted from RON. serde requires the predicate to take `&T`.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde requires the skip_serializing_if predicate to take &T"
)]
fn is_object_scope(scope: &CounterScope) -> bool {
    *scope == CounterScope::Object
}
