use serde::{Deserialize, Serialize};

use crate::{Filter, Reference};

/// A measurable characteristic of an object, read by `Quantity::StatOf`
/// (CR 109.3, 208, 212). The open part (mana value, loyalty, defense) is
/// finite; new printed stats are rare and get a variant when one arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Stat {
    /// CR 208.
    Power,
    /// CR 208.
    Toughness,
    /// CR 202.3.
    ManaValue,
    /// CR 212.4 (planeswalker).
    Loyalty,
    /// CR 212.6 (battle).
    Defense,
}

/// A number an effect computes at resolution: an amount, never an object
/// (objects are `Reference`s, CR 107.1, 107.3).
///
/// `Literal(Uint)` is `#[serde(untagged)]` and listed LAST so a bare
/// integer in RON parses straight to it (the `StatValue::Number`
/// precedent). KNOWN LIMITATION: because the literal arm is untagged, it
/// deserializes through `deserialize_any`, which never reaches the macro
/// layer's `deserialize_enum` interception — so a macro cannot stand at a
/// Quantity position until a card needs one and this is reworked into a
/// manual impl (as `Filter`/`Effect` did).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Quantity {
    /// The value chosen for {X} as the spell or ability was put on the
    /// stack (CR 107.3).
    X,
    /// How many objects match a filter (CR 107.3, "for each"). Boxed to
    /// break the `Filter` → `Stat` → `Quantity` → `Filter` size cycle.
    CountOf(Box<Filter>),
    /// A referenced object's stat (CR 107.3, "equal to its power").
    StatOf(Reference, Stat),
    /// Magnitude anaphora: "that much" / "that many" — the amount fixed by
    /// an earlier instruction (CR 107.3).
    ThatMuch,
    /// A bare integer literal. Untagged and last; see the type doc for the
    /// macro limitation this implies.
    #[serde(untagged)]
    Literal(crate::Uint),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::Reference;

    fn read(source: &str) -> Quantity { crate::ron::options().from_str(source).unwrap() }

    /// A bare integer reads as `Literal` (the untagged arm) — the property
    /// that keeps existing `DealDamage(Target(0), 3)`-shaped RON parsing.
    #[test]
    fn bare_integer_reads_as_literal() {
        assert_eq!(read("3"), Quantity::Literal(3));
    }

    #[test]
    fn constructors_read_named() {
        assert_eq!(read("X"), Quantity::X);
        assert_eq!(read("ThatMuch"), Quantity::ThatMuch);
        assert_eq!(
            read("StatOf(This, Power)"),
            Quantity::StatOf(Reference::This, Stat::Power),
        );
    }
}
