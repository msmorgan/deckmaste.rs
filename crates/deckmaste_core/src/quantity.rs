use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::{Count, Expansion};

/// A cardinality range over a scalar [`Count`]: how many objects an effect
/// operates on, never a continuous magnitude (amounts use `Count` directly).
///
/// `Deserialize` is derived (the macro reader synthesizes the `Expanded`
/// stream); `Serialize` is **manual** so `Expanded` writes the invocation
/// back rather than the literal struct — the other variants mirror the derive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Quantity {
    /// Exactly `n` objects.
    Exactly(Count),
    /// At least `n` objects.
    AtLeast(Count),
    /// At most `n` objects.
    AtMost(Count),
    /// Between `lo` and `hi` objects (inclusive).
    Between(Count, Count),
    /// Any number of objects ([CR#601.2c]).
    AnyNumber,
    /// A remembered `Quantity` macro invocation.
    Expanded(Expansion<Quantity>),
}

impl Serialize for Quantity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON. Mirrors the
        // shapes the derive produced, plus the `Expanded` invocation arm.
        match self {
            Quantity::Exactly(c) => {
                serializer.serialize_newtype_variant("Quantity", 0, "Exactly", c)
            }
            Quantity::AtLeast(c) => {
                serializer.serialize_newtype_variant("Quantity", 1, "AtLeast", c)
            }
            Quantity::AtMost(c) => serializer.serialize_newtype_variant("Quantity", 2, "AtMost", c),
            Quantity::Between(lo, hi) => {
                serializer.serialize_newtype_variant("Quantity", 3, "Between", &(lo, hi))
            }
            Quantity::AnyNumber => serializer.serialize_unit_variant("Quantity", 4, "AnyNumber"),
            // The invocation, not the struct.
            Quantity::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Count;

    fn read(source: &str) -> Quantity { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn exactly_reads() {
        assert_eq!(
            read("Exactly(Literal(3))"),
            Quantity::Exactly(Count::Literal(3)),
        );
    }

    #[test]
    fn at_most_reads() {
        assert_eq!(
            read("AtMost(Literal(2))"),
            Quantity::AtMost(Count::Literal(2)),
        );
    }

    #[test]
    fn at_least_reads() {
        assert_eq!(read("AtLeast(X)"), Quantity::AtLeast(Count::X),);
    }

    #[test]
    fn between_reads() {
        assert_eq!(
            read("Between(Literal(1), Literal(3))"),
            Quantity::Between(Count::Literal(1), Count::Literal(3)),
        );
    }

    #[test]
    fn any_number_reads() {
        assert_eq!(read("AnyNumber"), Quantity::AnyNumber);
    }

    /// `Between` (2-tuple) and `AnyNumber` (unit) pin the manual `Serialize`:
    /// serialize → read is identity.
    #[test]
    fn variants_round_trip() {
        for value in [
            Quantity::Between(Count::Literal(1), Count::Literal(3)),
            Quantity::AnyNumber,
            Quantity::Exactly(Count::X),
        ] {
            let written = crate::ron::options().to_string(&value).unwrap();
            assert_eq!(read(&written), value, "round-trip failed for {value:?}");
        }
    }
}
