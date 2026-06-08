use std::fmt;

use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Event, Expansion, Filter, IdentSeed, Quantity, Reference};

/// A numeric comparison ([CR#107.3]). The named forms keep RON readable —
/// `AtLeast` rather than `>=`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Cmp {
    Eq,
    AtLeast,
    AtMost,
    Greater,
    Less,
}

/// The turn-history window a `Happened` condition looks back over
/// ([CR#603.10]). Just `ThisTurn` for now; `ThisCombat` etc. accrete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Window {
    /// Since the start of the current turn (morbid, raid, "was kicked").
    ThisTurn,
}

/// A truth-valued test the engine evaluates ([CR#603.4] intervening-if,
/// [CR#118.12a] "unless", ability words). Ability words (`Threshold`,
/// `Delirium`, `Morbid`) are declared `Condition` macros — hence the manual
/// serde, so unknown names at Condition positions fall through to the macro
/// layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Condition {
    /// Compare two quantities ([CR#107.3]).
    Compare(Quantity, Cmp, Quantity),
    /// At least one object matches ([CR#107.3], "if you control a …").
    Exists(Filter),
    /// A referenced object matches a filter ([CR#107.3], "if it is a …").
    Is(Reference, Filter),
    /// An event happened within a window (morbid/raid, [CR#603.10]).
    Happened { event: Event, within: Window },
    /// All sub-conditions hold.
    AllOf(Vec<Condition>),
    /// At least one sub-condition holds.
    OneOf(Vec<Condition>),
    /// The sub-condition does not hold.
    Not(Box<Condition>),
    /// A remembered `Condition` macro invocation (ability words: `Threshold`,
    /// `Delirium`, `Morbid`). Serialized as the invocation, not the struct.
    Expanded(Expansion<Condition>),
}

/// Every name a Condition position accepts. Must stay in sync with
/// `visit_enum` (the drift-guard test catches missing arms).
const VARIANTS: &[&str] = &[
    "Compare", "Exists", "Is", "Happened", "AllOf", "OneOf", "Not", "Expanded",
];

/// `Compare`, deserialized via its own tuple shape.
struct Triple<A, B, C>(A, B, C);

struct TripleVisitor<A, B, C>(std::marker::PhantomData<(A, B, C)>);

impl<'de, A, B, C> Visitor<'de> for TripleVisitor<A, B, C>
where
    A: Deserialize<'de>,
    B: Deserialize<'de>,
    C: Deserialize<'de>,
{
    type Value = Triple<A, B, C>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("a 3-element tuple") }

    fn visit_seq<S: de::SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let a = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let b = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let c = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        Ok(Triple(a, b, c))
    }
}

/// `Happened`, deserialized as its own struct (newtype-variant delegation).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct Happened {
    event: Event,
    within: Window,
}

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ConditionVisitor;

        impl<'de> Visitor<'de> for ConditionVisitor {
            type Value = Condition;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a condition")
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Condition, A::Error> {
                let (ident, v) = data.variant_seed(IdentSeed)?;
                // Adding a form? Update VARIANTS above to match.
                Ok(match ident.as_str() {
                    "Compare" => {
                        let Triple(a, cmp, b) = v.tuple_variant(
                            3,
                            TripleVisitor::<Quantity, Cmp, Quantity>(std::marker::PhantomData),
                        )?;
                        Condition::Compare(a, cmp, b)
                    }
                    "Exists" => Condition::Exists(v.newtype_variant()?),
                    "Is" => {
                        let (r, f) = v.tuple_variant(2, crate::de_util::Pair::new())?;
                        Condition::Is(r, f)
                    }
                    "Happened" => {
                        let h: Happened = v.newtype_variant()?;
                        Condition::Happened {
                            event: h.event,
                            within: h.within,
                        }
                    }
                    "AllOf" => Condition::AllOf(v.newtype_variant()?),
                    "OneOf" => Condition::OneOf(v.newtype_variant()?),
                    "Not" => Condition::Not(v.newtype_variant()?),
                    "Expanded" => Condition::Expanded(v.newtype_variant()?),
                    _ => return Err(de::Error::unknown_variant(&ident, VARIANTS)),
                })
            }
        }

        deserializer.deserialize_enum("Condition", VARIANTS, ConditionVisitor)
    }
}

impl Serialize for Condition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Condition::Compare(a, cmp, b) => {
                serializer.serialize_newtype_variant("Condition", 0, "Compare", &(a, cmp, b))
            }
            Condition::Exists(f) => {
                serializer.serialize_newtype_variant("Condition", 1, "Exists", f)
            }
            Condition::Is(r, f) => {
                serializer.serialize_newtype_variant("Condition", 2, "Is", &(r, f))
            }
            Condition::Happened { event, within } => {
                let h = Happened {
                    event: event.clone(),
                    within: *within,
                };
                serializer.serialize_newtype_variant("Condition", 3, "Happened", &h)
            }
            Condition::AllOf(cs) => {
                serializer.serialize_newtype_variant("Condition", 4, "AllOf", cs)
            }
            Condition::OneOf(cs) => {
                serializer.serialize_newtype_variant("Condition", 5, "OneOf", cs)
            }
            Condition::Not(c) => serializer.serialize_newtype_variant("Condition", 6, "Not", c),
            // The invocation, not the struct: `Expansion`'s Serialize emits it.
            Condition::Expanded(e) => e.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str) -> Condition { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn compare_reads() {
        assert_eq!(
            read("Compare(X, AtLeast, Literal(3))"),
            Condition::Compare(Quantity::X, Cmp::AtLeast, Quantity::Literal(3)),
        );
    }

    #[test]
    fn happened_reads() {
        assert_eq!(
            read(r#"Happened(event: Performed(verb: "Sacrifice"), within: ThisTurn)"#),
            Condition::Happened {
                event: Event::Performed {
                    verb: "Sacrifice".into(),
                    by: Filter::Any,
                    on: Filter::Any,
                },
                within: Window::ThisTurn,
            },
        );
    }

    #[test]
    fn variants_list_matches_visit_enum() {
        for &name in VARIANTS {
            if let Err(error) = crate::ron::options().from_str::<Condition>(name) {
                let message = error.to_string();
                assert!(
                    !message.contains("Unexpected variant") && !message.contains("unknown variant"),
                    "VARIANTS entry `{name}` is not handled in visit_enum: {message}"
                );
            }
        }
    }
}
