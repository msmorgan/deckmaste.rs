//! The SORT of an anaphor or antecedent — the English noun a sorted anaphor
//! carries (`That(Card)` = "that card", `Them(Token)` = "those tokens").
//! Sorts drive the nearest-compatible-antecedent resolution (R1) and the
//! ambiguity gate (R2): an anaphor reaches an antecedent whose sort is
//! compatible (the Idris model's `compat` relation), and the sort
//! is why "exile target **creature** … return that **card**" resolves — the
//! exile clause's product sits in exile, so its sort is [`Sort::Card`]
//! ([CR#400.7] — the new object the effect tracks across the move).

use serde::Deserialize;
use serde::Serialize;
use serde::de;

use crate::Expand;
use crate::Type;

/// The English noun of an anaphor/antecedent. `OfType(t)` is "that
/// creature"/"that land"/… — it reads bare in RON (`That(Creature)`), the
/// named sorts read by name (`That(Card)`).
///
/// Serde is hand-written (not `SupportsMacros`): a bare card-type name in a
/// `Sort` position reads as `OfType(t)` and writes back bare, so the wire
/// spells `That(Creature)`, never `That(OfType(Creature))` (both are
/// accepted on read).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Expand)]
pub enum Sort {
    /// A player ([CR#102.1]).
    Player,
    /// A non-battlefield object — a card in a graveyard, exile, a hand, a
    /// library ([CR#108.2]). Tokens aren't cards ([CR#108.2b]); "that card"
    /// never reaches a token antecedent.
    Card,
    /// A created token ([CR#111.1]).
    Token,
    /// A spell on the stack ([CR#112.1]).
    Spell,
    /// A spell or ability on the stack ([CR#405.1]) — the widened
    /// stack-object noun (`StackObject` reaches `Spell` antecedents).
    StackObject,
    /// A battlefield object of unknown type ([CR#110.1]).
    Permanent,
    /// A battlefield object of this card type — "that creature", "that
    /// land" ([CR#110.1,205.2a]).
    OfType(Type),
    /// A value antecedent — "that many"/"that much" ([CR#608.2i]).
    Amount,
    /// A labeled pile group ([CR#700.3a] — dividing cards into piles).
    Pile,
}

impl Sort {
    /// The stable key of this sort's shape — `OfType(_)` collapses to
    /// one key; the Idris model's `compat` relation pairs these keys, with the
    /// same-`t` refinement on `OfType`/`OfType` applied by the consumer.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Sort::Player => "Player",
            Sort::Card => "Card",
            Sort::Token => "Token",
            Sort::Spell => "Spell",
            Sort::StackObject => "StackObject",
            Sort::Permanent => "Permanent",
            Sort::OfType(_) => "OfType",
            Sort::Amount => "Amount",
            Sort::Pile => "Pile",
        }
    }

    /// The English noun phrase (singular) — what the renderer prints after
    /// "that".
    #[must_use]
    pub fn noun(self) -> &'static str {
        match self {
            Sort::Player => "player",
            Sort::Card => "card",
            Sort::Token => "token",
            Sort::Spell => "spell",
            Sort::StackObject => "spell or ability",
            Sort::Permanent => "permanent",
            Sort::OfType(t) => type_noun(t),
            Sort::Amount => "amount",
            Sort::Pile => "pile",
        }
    }
}

/// The lowercase English noun of a card type ([CR#205.2a]).
fn type_noun(t: Type) -> &'static str {
    match t {
        Type::Artifact => "artifact",
        Type::Battle => "battle",
        Type::Creature => "creature",
        Type::Dungeon => "dungeon",
        Type::Enchantment => "enchantment",
        Type::Instant => "instant",
        Type::Kindred => "kindred",
        Type::Land => "land",
        Type::Planeswalker => "planeswalker",
        Type::Sorcery => "sorcery",
    }
}

/// The capitalized wire name of a card type — the bare `OfType` spelling.
fn type_name(t: Type) -> &'static str {
    match t {
        Type::Artifact => "Artifact",
        Type::Battle => "Battle",
        Type::Creature => "Creature",
        Type::Dungeon => "Dungeon",
        Type::Enchantment => "Enchantment",
        Type::Instant => "Instant",
        Type::Kindred => "Kindred",
        Type::Land => "Land",
        Type::Planeswalker => "Planeswalker",
        Type::Sorcery => "Sorcery",
    }
}

/// Every accepted variant name — the named sorts, the explicit `OfType`
/// spelling, and every bare card-type name. The macro-aware reader consults
/// this list to decide whether an identifier is a real variant, so the bare
/// type names must be present or they would be mistaken for macro
/// invocations.
const VARIANTS: &[&str] = &[
    "Player",
    "Card",
    "Token",
    "Spell",
    "StackObject",
    "Permanent",
    "OfType",
    "Amount",
    "Pile",
    // Bare card-type names — `That(Creature)` ≡ `That(OfType(Creature))`.
    "Artifact",
    "Battle",
    "Creature",
    "Dungeon",
    "Enchantment",
    "Instant",
    "Kindred",
    "Land",
    "Planeswalker",
    "Sorcery",
];

/// The bare card type named `name`, if any.
fn type_by_name(name: &str) -> Option<Type> {
    Some(match name {
        "Artifact" => Type::Artifact,
        "Battle" => Type::Battle,
        "Creature" => Type::Creature,
        "Dungeon" => Type::Dungeon,
        "Enchantment" => Type::Enchantment,
        "Instant" => Type::Instant,
        "Kindred" => Type::Kindred,
        "Land" => Type::Land,
        "Planeswalker" => Type::Planeswalker,
        "Sorcery" => Type::Sorcery,
        _ => return None,
    })
}

impl Serialize for Sort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (index, name) = match self {
            Sort::Player => (0, "Player"),
            Sort::Card => (1, "Card"),
            Sort::Token => (2, "Token"),
            Sort::Spell => (3, "Spell"),
            Sort::StackObject => (4, "StackObject"),
            Sort::Permanent => (5, "Permanent"),
            // `OfType(t)` writes back as the bare type name.
            Sort::OfType(t) => (6, type_name(*t)),
            Sort::Amount => (7, "Amount"),
            Sort::Pile => (8, "Pile"),
        };
        serializer.serialize_unit_variant("Sort", index, name)
    }
}

impl<'de> Deserialize<'de> for Sort {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Sort, D::Error> {
        struct SortVisitor;

        impl<'de> de::Visitor<'de> for SortVisitor {
            type Value = Sort;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a sort name (Player/Card/Token/Spell/StackObject/Permanent/Amount/Pile, a bare card type, or OfType(t))")
            }

            fn visit_enum<A: de::EnumAccess<'de>>(self, access: A) -> Result<Sort, A::Error> {
                let (name, variant): (String, A::Variant) = access.variant()?;
                let sort = match name.as_str() {
                    "Player" => Sort::Player,
                    "Card" => Sort::Card,
                    "Token" => Sort::Token,
                    "Spell" => Sort::Spell,
                    "StackObject" => Sort::StackObject,
                    "Permanent" => Sort::Permanent,
                    "Amount" => Sort::Amount,
                    "Pile" => Sort::Pile,
                    "OfType" => {
                        return de::VariantAccess::newtype_variant::<Type>(variant)
                            .map(Sort::OfType);
                    }
                    other => match type_by_name(other) {
                        Some(t) => Sort::OfType(t),
                        None => {
                            return Err(de::Error::unknown_variant(other, VARIANTS));
                        }
                    },
                };
                de::VariantAccess::unit_variant(variant)?;
                Ok(sort)
            }
        }

        deserializer.deserialize_enum("Sort", VARIANTS, SortVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str) -> Sort {
        crate::ron::options().from_str(source).unwrap()
    }

    fn write(sort: Sort) -> String {
        crate::ron::options().to_string(&sort).unwrap()
    }

    /// Named sorts read by name and round-trip.
    #[test]
    fn named_sorts_round_trip() {
        for (source, value) in [
            ("Player", Sort::Player),
            ("Card", Sort::Card),
            ("Token", Sort::Token),
            ("Spell", Sort::Spell),
            ("StackObject", Sort::StackObject),
            ("Permanent", Sort::Permanent),
            ("Amount", Sort::Amount),
            ("Pile", Sort::Pile),
        ] {
            assert_eq!(read(source), value);
            assert_eq!(write(value), source);
        }
    }

    /// A bare card-type name reads as `OfType(t)` and writes back bare —
    /// the wire spells `That(Creature)`.
    #[test]
    fn bare_card_types_read_as_of_type() {
        assert_eq!(read("Creature"), Sort::OfType(Type::Creature));
        assert_eq!(read("Land"), Sort::OfType(Type::Land));
        assert_eq!(write(Sort::OfType(Type::Creature)), "Creature");
        // The explicit spelling is accepted on read too.
        assert_eq!(read("OfType(Creature)"), Sort::OfType(Type::Creature));
    }

    /// An unknown name is an error, not a silent wildcard.
    #[test]
    fn unknown_sort_errors() {
        let error = crate::ron::options().from_str::<Sort>("Bogus").unwrap_err();
        let msg = error.to_string();
        assert!(msg.contains("Bogus"), "unexpected error: {msg}");
        assert!(msg.contains("Sort"), "unexpected error: {msg}");
    }
}
