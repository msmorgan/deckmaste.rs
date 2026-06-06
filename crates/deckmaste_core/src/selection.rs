use serde::{Deserialize, Serialize};

use crate::{Filter, Quantity, Reference};

/// A Filter lifted into a resolution-time choice context: who picks, when,
/// how many (CR 608.2d). Verb object slots take exactly one `Selection`.
///
/// Targeting is NOT here — it lives in [`crate::TargetSpec`], the announce
/// list — because a target has legality recheck and retargeting rules the
/// other choice forms lack (CR 115). References lift into `Selection`
/// (`This`, `Target(0)`, …) so a bound object can stand where a choice would.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Selection {
    /// Every matching object, one at a time — distributive "each", no
    /// targeting, evaluated when the instruction applies (CR 608.2d).
    Each(Filter),
    /// All matching objects as one set — the shape continuous-effect
    /// scopes and set-wide instructions consume.
    All(Filter),
    /// One untargeted choice, made at resolution: not announced, not
    /// rechecked the way targets are (CR 608.2d vs 601.2c).
    Choose(Filter),
    /// A quantity of untargeted choices made at resolution (CR 608.2d).
    ChooseN(Quantity, Filter),
    /// A random selection of a quantity of matching objects (CR 701.x).
    Random(Quantity, Filter),

    // References, flattened: a bound object can stand where a choice would.
    // Names mirror `Reference`'s variants exactly; `From<Reference>` below
    // is the compiler-enforced sync point.
    /// See [`Reference::This`].
    This,
    /// See [`Reference::You`].
    You,
    /// See [`Reference::Target`].
    Target(usize),
    /// See [`Reference::ThatObject`].
    ThatObject,
    /// See [`Reference::ThatPlayer`].
    ThatPlayer,
    /// See [`Reference::Bound`].
    Bound(crate::Ident),
    /// See [`Reference::Linked`].
    Linked(crate::Ident),
    /// See [`Reference::ControllerOf`].
    ControllerOf(Box<Reference>),
    /// See [`Reference::OwnerOf`].
    OwnerOf(Box<Reference>),
    /// See [`Reference::EnchantedObject`].
    EnchantedObject,
    /// See [`Reference::EquippedCreature`].
    EquippedCreature,
    /// See [`Reference::AttachedTo`].
    AttachedTo(Box<Reference>),
}

/// Lifts a bound `Reference` into a `Selection`. The match is EXHAUSTIVE on
/// purpose: it is the sync point that fails to compile if `Reference` grows
/// a variant `Selection` hasn't mirrored.
impl From<Reference> for Selection {
    fn from(reference: Reference) -> Self {
        match reference {
            Reference::This => Selection::This,
            Reference::You => Selection::You,
            Reference::Target(n) => Selection::Target(n),
            Reference::ThatObject => Selection::ThatObject,
            Reference::ThatPlayer => Selection::ThatPlayer,
            Reference::Bound(role) => Selection::Bound(role),
            Reference::Linked(key) => Selection::Linked(key),
            Reference::ControllerOf(r) => Selection::ControllerOf(r),
            Reference::OwnerOf(r) => Selection::OwnerOf(r),
            Reference::EnchantedObject => Selection::EnchantedObject,
            Reference::EquippedCreature => Selection::EquippedCreature,
            Reference::AttachedTo(r) => Selection::AttachedTo(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, ObjectKind, Type};

    fn read(source: &str) -> Selection { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn quantifiers_wrap_filters() {
        assert_eq!(
            read("Each(Kind(Player))"),
            Selection::Each(Filter::Kind(ObjectKind::Player)),
        );
        assert_eq!(
            read("Choose(Type(Creature))"),
            Selection::Choose(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            ))),
        );
    }

    /// References lift into Selection flat — `This`, `Target(0)`, … read
    /// directly, no `That(...)` wrapper (the old shape, now dead).
    #[test]
    fn references_lift_flat() {
        assert_eq!(read("This"), Selection::This);
        assert_eq!(read("Target(0)"), Selection::Target(0));
        assert_eq!(
            read("ControllerOf(This)"),
            Selection::ControllerOf(Box::new(Reference::This)),
        );
    }

    /// `From<Reference>` mirrors every reference variant; spot-check the
    /// payload-carrying arms (the exhaustive match is the real guard).
    #[test]
    fn from_reference_mirrors() {
        assert_eq!(Selection::from(Reference::This), Selection::This);
        assert_eq!(Selection::from(Reference::Target(2)), Selection::Target(2));
        assert_eq!(
            Selection::from(Reference::AttachedTo(Box::new(Reference::This))),
            Selection::AttachedTo(Box::new(Reference::This)),
        );
    }
}
