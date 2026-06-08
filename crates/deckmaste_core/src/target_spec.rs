use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::{Expansion, Filter, Quantity};

/// One entry in an ability's announce list ([CR#601.2c,115]). A `TargetSpec`
/// is the only place "target" lives: it binds `Reference::Target(n)` for the
/// effect body to read, and is rechecked at resolution ([CR#608.2c..608.2d]).
///
/// Separated from [`crate::Selection`] so that resolution-time choices
/// (`Each`, `Choose`, …) and announce-time targets never share a position —
/// targeting has legality recheck and retargeting rules that the other
/// choice forms don't.
///
/// `Deserialize` is derived (the macro reader synthesizes the `Expanded`
/// stream); `Serialize` is **manual** so `Expanded` writes the invocation back
/// rather than the literal struct — the other variants mirror the derive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TargetSpec {
    /// A quantity of targets matching the filter ([CR#115.1,115.6,601.2c]).
    /// Use `Quantity::Exactly(Count::Literal(1))` for a single target,
    /// `Quantity::AtMost(n)` for "up to N", and `Quantity::AnyNumber` for
    /// "any number of targets".
    Target(Quantity, Filter),
    /// A remembered `TargetSpec` macro invocation.
    Expanded(Expansion<TargetSpec>),
}

impl Serialize for TargetSpec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON.
        match self {
            TargetSpec::Target(q, f) => {
                serializer.serialize_newtype_variant("TargetSpec", 0, "Target", &(q, f))
            }
            // The invocation, not the struct.
            TargetSpec::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, Count, Quantity, Type};

    fn read(source: &str) -> TargetSpec { crate::ron::options().from_str(source).unwrap() }

    fn creature_filter() -> Filter {
        Filter::Characteristic(CharacteristicFilter::Type(Type::Creature))
    }

    /// The announce-list grammar: `Target(Quantity, Filter)`, distinct from
    /// `Selection`'s resolution choices.
    #[test]
    fn announce_forms_read() {
        assert_eq!(
            read("Target(Exactly(Literal(1)), Type(Creature))"),
            TargetSpec::Target(Quantity::Exactly(Count::Literal(1)), creature_filter(),),
        );
        assert_eq!(
            read("Target(AtMost(Literal(2)), Type(Creature))"),
            TargetSpec::Target(Quantity::AtMost(Count::Literal(2)), creature_filter(),),
        );
        assert_eq!(
            read("Target(AnyNumber, Type(Creature))"),
            TargetSpec::Target(Quantity::AnyNumber, creature_filter()),
        );
    }

    /// Pins the manual `Serialize` — serialize → read is identity.
    #[test]
    fn target_round_trips() {
        for value in [
            TargetSpec::Target(Quantity::Exactly(Count::Literal(1)), creature_filter()),
            TargetSpec::Target(Quantity::AnyNumber, creature_filter()),
            TargetSpec::Target(Quantity::AtMost(Count::Literal(2)), creature_filter()),
        ] {
            let written = crate::ron::options().to_string(&value).unwrap();
            assert_eq!(read(&written), value, "round-trip failed for {value:?}");
        }
    }
}
