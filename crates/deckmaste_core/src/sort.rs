//! The SORT of an anaphor or antecedent — the English noun a sorted anaphor
//! carries (`That(Card)` = "that card", `Them(Token)` = "those tokens").
//! Sorts drive the nearest-compatible-antecedent resolution (R1) and the
//! ambiguity gate (R2): an anaphor reaches an antecedent whose sort is
//! compatible (the Idris model's `compat` relation), and the sort
//! is why "exile target **creature** … return that **card**" resolves — the
//! exile clause's product sits in exile, so its sort is [`Sort::Card`]
//! ([CR#400.7] — the new object the effect tracks across the move).

use crate::Type;

/// The English noun of an anaphor/antecedent. `OfType(t)` is "that
/// creature"/"that land"/… . Core snapshots use the explicit plain-serde
/// spelling `That(OfType(Creature))`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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
