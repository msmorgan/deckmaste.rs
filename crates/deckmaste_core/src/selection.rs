use serde::{Deserialize, Serialize};

use crate::{Filter, Reference};

/// A Filter lifted into a choice context: who picks, when, how many —
/// and, for the target quantifiers, what gets *bound* for later
/// `Reference::Target(n)` use (CR 115, 601.2c).
///
/// Not a Filter variant on purpose: filters compose under
/// AllOf/OneOf/Not, quantifiers don't, and bare-Filter positions
/// (protection qualities, event participants) must not admit "target".
/// Quantity-bearing quantifiers (`Targets(n, …)`, `UpToTargets`,
/// `Random`) arrive with the Quantity module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Selection {
    /// One target: announced, rechecked at resolution.
    Target(Filter),
    /// Every matching object, one at a time — distributive "each", no
    /// targeting, evaluated when the instruction applies (CR 608.2d).
    Each(Filter),
    /// All matching objects as one set — the shape continuous-effect
    /// scopes and set-wide instructions consume.
    All(Filter),
    /// One untargeted choice, made at resolution: not announced, not
    /// rechecked the way targets are (CR 608.2d vs 601.2c).
    Choose(Filter),
    /// An already-bound object: references lift into Selection here.
    That(Reference),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, ObjectKind, Type};

    fn read(source: &str) -> Selection {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn quantifiers_wrap_filters() {
        assert_eq!(
            read("Target(Type(Creature))"),
            Selection::Target(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            ))),
        );
        assert_eq!(
            read("Each(Kind(Player))"),
            Selection::Each(Filter::Kind(ObjectKind::Player)),
        );
    }

    /// `Target` in Selection takes a Filter; `Target` in Reference takes
    /// an index. Distinct types, distinct positions — and a bare index is
    /// rejected at the Filter position, so the shared name stays
    /// unambiguous.
    #[test]
    fn references_lift_via_that() {
        assert_eq!(read("That(Target(0))"), Selection::That(Reference::Target(0)));
        assert_eq!(read("That(This)"), Selection::That(Reference::This));
        // Target at a Selection position requires a Filter payload.
        assert!(
            crate::ron::options()
                .from_str::<Selection>("Target(0)")
                .is_err()
        );
    }

    #[test]
    fn selections_round_trip() {
        let source = "Target(AllOf([Kind(Permanent),Type(Creature)]))";
        let parsed = read(source);
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }
}
