use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::{Expansion, Filter, Quantity};

/// One entry in an ability's announce list (CR 601.2c, 115). A `TargetSpec`
/// is the only place "target" lives: it binds `Reference::Target(n)` for the
/// effect body to read, and is rechecked at resolution (CR 608.2c-d).
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
    /// One target matching the filter (CR 115.1).
    Target(Filter),
    /// Up to a quantity of targets (CR 115.1c, "up to two target …").
    UpToTargets(Quantity, Filter),
    /// Exactly a quantity of targets (CR 115.1b, "two target …").
    Targets(Quantity, Filter),
    /// Any number of targets (CR 115.1d).
    AnyNumberTargets(Filter),
    /// A remembered `TargetSpec` macro invocation.
    Expanded(Expansion<TargetSpec>),
}

impl Serialize for TargetSpec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON.
        match self {
            TargetSpec::Target(f) => {
                serializer.serialize_newtype_variant("TargetSpec", 0, "Target", f)
            }
            TargetSpec::UpToTargets(q, f) => {
                serializer.serialize_newtype_variant("TargetSpec", 1, "UpToTargets", &(q, f))
            }
            TargetSpec::Targets(q, f) => {
                serializer.serialize_newtype_variant("TargetSpec", 2, "Targets", &(q, f))
            }
            TargetSpec::AnyNumberTargets(f) => {
                serializer.serialize_newtype_variant("TargetSpec", 3, "AnyNumberTargets", f)
            }
            // The invocation, not the struct.
            TargetSpec::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, Type};

    fn read(source: &str) -> TargetSpec { crate::ron::options().from_str(source).unwrap() }

    /// The announce-list grammar: `Target(Filter)` and the quantity-bearing
    /// forms, distinct from `Selection`'s resolution choices.
    #[test]
    fn announce_forms_read() {
        assert_eq!(
            read("Target(Type(Creature))"),
            TargetSpec::Target(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            ))),
        );
        assert_eq!(
            read("UpToTargets(Literal(2), Type(Creature))"),
            TargetSpec::UpToTargets(
                crate::Quantity::Literal(2),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ),
        );
    }
}
