use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::{Expansion, Filter, Reference};

/// A measurable characteristic of an object, read by `Count::StatOf`
/// ([CR#109.3,208,209,210]). The open part (mana value, loyalty, defense) is
/// finite; new printed stats are rare and get a variant when one arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Stat {
    /// [CR#208].
    Power,
    /// [CR#208].
    Toughness,
    /// [CR#202.3].
    ManaValue,
    /// [CR#209] (planeswalker).
    Loyalty,
    /// [CR#210] (battle).
    Defense,
}

/// A scalar magnitude an effect computes at resolution: an amount, never an
/// object (objects are `Reference`s, [CR#107.1,107.3]).
///
/// Core's grammar is strict: a literal is written `Literal(3)`, a plain
/// tagged variant like every other. A bare numeral (`3`) at a `Count`
/// position is *reader sugar* the macro layer splices into `Literal(3)`
/// before core ever sees it — exactly like a bare declared subtype name. So
/// `Count` is a plain derived enum, and full macro interception applies
/// at and under its positions.
///
/// `Deserialize` is derived (the macro reader synthesizes the `Expanded`
/// stream); `Serialize` is **manual** so `Expanded` writes the invocation
/// back rather than the literal struct — the other variants mirror the derive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Count {
    /// The value chosen for {X} as the spell or ability was put on the
    /// stack ([CR#107.3]).
    X,
    /// How many objects match a filter ([CR#107.3], "for each"). Boxed to
    /// break the `Filter` → `Stat` → `Count` → `Filter` size cycle.
    CountOf(Box<Filter>),
    /// A referenced object's stat ([CR#107.3], "equal to its power").
    StatOf(Reference, Stat),
    /// Magnitude anaphora: "that much" / "that many" — the amount fixed by
    /// an earlier instruction ([CR#107.3]).
    ThatMuch,
    /// A bare integer literal. Written `Literal(3)` in core grammar; a bare
    /// `3` is macro-layer reader sugar for it.
    Literal(crate::Uint),
    /// A remembered `Count` macro invocation.
    Expanded(Expansion<Count>),
}

impl Serialize for Count {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON. Mirrors the
        // shapes the derive produced, plus the `Expanded` invocation arm.
        match self {
            Count::X => serializer.serialize_unit_variant("Count", 0, "X"),
            Count::CountOf(f) => serializer.serialize_newtype_variant("Count", 1, "CountOf", f),
            Count::StatOf(r, s) => {
                serializer.serialize_newtype_variant("Count", 2, "StatOf", &(r, s))
            }
            Count::ThatMuch => serializer.serialize_unit_variant("Count", 3, "ThatMuch"),
            Count::Literal(n) => serializer.serialize_newtype_variant("Count", 4, "Literal", n),
            // The invocation, not the struct.
            Count::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::Reference;

    fn read(source: &str) -> Count { crate::ron::options().from_str(source).unwrap() }

    /// In core grammar a literal is tagged: `Literal(3)`. A bare `3` is not
    /// core grammar — it's macro-layer sugar, tested in the cards crate.
    #[test]
    fn literal_reads_tagged() {
        assert_eq!(read("Literal(3)"), Count::Literal(3));
    }

    #[test]
    fn constructors_read_named() {
        assert_eq!(read("X"), Count::X);
        assert_eq!(read("ThatMuch"), Count::ThatMuch);
        assert_eq!(
            read("StatOf(This, Power)"),
            Count::StatOf(Reference::This, Stat::Power),
        );
    }

    /// The 2-field tuple variant pins the manual `Serialize` — serialize → read
    /// is identity (the highest-risk arm).
    #[test]
    fn stat_of_round_trips() {
        let value = Count::StatOf(Reference::This, Stat::Power);
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(read(&written), value);
    }
}
