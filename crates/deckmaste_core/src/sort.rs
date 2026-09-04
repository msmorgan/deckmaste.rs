//! The SORT of an anaphor or antecedent — the English noun a sorted anaphor
//! carries (`That(Card)` = "that card", `Them(Token)` = "those tokens").
//! Sorts drive the nearest-compatible-antecedent resolution (R1) and the
//! ambiguity gate (R2): an anaphor reaches an antecedent whose sort is
//! compatible (the Idris model's `compat` relation), and the sort
//! is why "exile target **creature** … return that **card**" resolves — the
//! exile clause's product sits in exile, so its sort is
//! [`ReferentSort::Card`] ([CR#400.7] — the new object the effect tracks
//! across the move).
//!
//! A sort names its DOMAIN first and its noun second. [`ReferentSort`] is the
//! Entity domain — one Object or Player, what a [`crate::Reference`] denotes
//! and what an Entity-valued [`crate::Selection`] groups. [`Sort::Amount`] is
//! the value domain and [`Sort::Pile`] the collection domain ([CR#700.3b] — a
//! pile is not an object). [`Sort::register_kind`] is the machine-checked
//! statement of that split: it is the single place a sort says which
//! [`Kind`] of register can answer it, so an amount anaphor can never reach an
//! Entity register and a pile anaphor can never reach an Entity group.

use crate::Kind;
use crate::Type;

/// The English noun of an ENTITY-valued antecedent — one Object or Player
/// ([CR#109.1,102.1]). `OfType(t)` is "that creature"/"that land"/… . Core
/// snapshots use the explicit plain-serde spelling `OfType(Creature)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ReferentSort {
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
}

/// The noun an anaphor carries, tagged by the DOMAIN that noun lives in.
///
/// The three arms are the three register domains a mention can reach, and
/// nothing crosses between them: [`Sort::register_kind`] maps each to the one
/// [`Kind`] that can answer it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Sort {
    /// An ENTITY-valued antecedent — one Object or Player, or a group of
    /// them ([CR#109.1,102.1]). Answered by [`Kind::Entity`]/
    /// [`Kind::Entities`].
    Referent(ReferentSort),
    /// A VALUE antecedent — "that many"/"that much" ([CR#608.2i]). A game
    /// number, never an Entity: answered by [`Kind::Number`] alone.
    Amount,
    /// A COLLECTION antecedent — a labeled pile ([CR#700.3a]). [CR#700.3b]:
    /// each object in a pile is still an individual object and the pile
    /// itself is not an object, so a pile is neither a Referent nor a group
    /// of them: answered by [`Kind::Pile`] alone.
    Pile,
}

impl ReferentSort {
    /// The stable key of this sort's shape — `OfType(_)` collapses to one
    /// key; the Idris model's `compat` relation pairs these keys, with the
    /// same-`t` refinement on `OfType`/`OfType` applied by the consumer.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            ReferentSort::Player => "Player",
            ReferentSort::Card => "Card",
            ReferentSort::Token => "Token",
            ReferentSort::Spell => "Spell",
            ReferentSort::StackObject => "StackObject",
            ReferentSort::Permanent => "Permanent",
            ReferentSort::OfType(_) => "OfType",
        }
    }

    /// The English noun phrase (singular) — what the renderer prints after
    /// "that".
    #[must_use]
    pub fn noun(self) -> &'static str {
        match self {
            ReferentSort::Player => "player",
            ReferentSort::Card => "card",
            ReferentSort::Token => "token",
            ReferentSort::Spell => "spell",
            ReferentSort::StackObject => "spell or ability",
            ReferentSort::Permanent => "permanent",
            ReferentSort::OfType(t) => type_noun(t),
        }
    }

    /// The Entity domain a mention of this sort ranges over
    /// ([CR#109.1,102.1]): `Player` names a player, every other noun names an
    /// object.
    #[must_use]
    pub fn domain(self) -> crate::Domain {
        match self {
            ReferentSort::Player => crate::Domain::Player,
            ReferentSort::Card
            | ReferentSort::Token
            | ReferentSort::Spell
            | ReferentSort::StackObject
            | ReferentSort::Permanent
            | ReferentSort::OfType(_) => crate::Domain::Object,
        }
    }

    /// Whether an antecedent of sort `have` can answer a mention of `self`
    /// — the Idris model's `compat` relation. A widened stack noun reaches a
    /// spell; "that permanent" reaches a token or a typed battlefield object.
    #[must_use]
    pub fn compatible_with(self, have: ReferentSort) -> bool {
        use ReferentSort as R;
        match (self, have) {
            (R::Player, R::Player)
            | (R::Card, R::Card)
            | (R::Token, R::Token)
            | (R::Spell, R::Spell)
            | (R::StackObject, R::Spell | R::StackObject)
            | (R::Permanent, R::Permanent | R::OfType(_) | R::Token) => true,
            (R::OfType(a), R::OfType(b)) => a == b,
            _ => false,
        }
    }
}

impl Sort {
    /// The register shape that can answer a mention of this sort — the one
    /// place the value/collection/Entity split is stated, so no consumer has
    /// to re-derive it. An `Amount` reaches a number and nothing else; a
    /// `Pile` reaches a pile and nothing else ([CR#700.3b]).
    #[must_use]
    pub fn register_kind(self) -> Kind {
        match self {
            // Cardinality (one vs many) is the mention's, not the sort's, so
            // an Entity sort answers to either Entity register shape; the
            // caller supplies the cardinality.
            Sort::Referent(_) => Kind::Entity,
            Sort::Amount => Kind::Number,
            Sort::Pile => Kind::Pile,
        }
    }

    /// Whether a register of shape `have` can answer a mention of this sort.
    #[must_use]
    pub fn answered_by(self, have: Kind) -> bool {
        match self {
            Sort::Referent(_) => matches!(have, Kind::Entity | Kind::Entities),
            Sort::Amount | Sort::Pile => have == self.register_kind(),
        }
    }

    /// Whether an antecedent of sort `have` can answer a mention of `self`.
    /// Domains never cross: an amount mention never reaches an Entity or a
    /// pile antecedent, and a pile mention never reaches an Entity group.
    #[must_use]
    pub fn compatible_with(self, have: Sort) -> bool {
        match (self, have) {
            (Sort::Referent(want), Sort::Referent(had)) => want.compatible_with(had),
            (Sort::Amount, Sort::Amount) | (Sort::Pile, Sort::Pile) => true,
            _ => false,
        }
    }

    /// The stable key of this sort's shape.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Sort::Referent(r) => r.key(),
            Sort::Amount => "Amount",
            Sort::Pile => "Pile",
        }
    }

    /// The English noun phrase (singular) — what the renderer prints after
    /// "that".
    #[must_use]
    pub fn noun(self) -> &'static str {
        match self {
            Sort::Referent(r) => r.noun(),
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

#[cfg(test)]
mod tests {
    use super::ReferentSort;
    use super::Sort;
    use crate::Kind;

    /// Every sort names exactly one register domain, and the value and
    /// collection domains are reachable by nothing else ([CR#700.3b]).
    #[test]
    fn each_sort_names_one_register_domain() {
        assert_eq!(
            Sort::Referent(ReferentSort::Card).register_kind(),
            Kind::Entity
        );
        assert_eq!(Sort::Amount.register_kind(), Kind::Number);
        assert_eq!(Sort::Pile.register_kind(), Kind::Pile);

        assert!(Sort::Referent(ReferentSort::Card).answered_by(Kind::Entities));
        assert!(!Sort::Referent(ReferentSort::Card).answered_by(Kind::Pile));
        assert!(!Sort::Referent(ReferentSort::Card).answered_by(Kind::Number));
        assert!(!Sort::Amount.answered_by(Kind::Entity));
        assert!(!Sort::Pile.answered_by(Kind::Entities));
    }

    /// A mention never reaches an antecedent of another domain.
    #[test]
    fn domains_do_not_cross() {
        for entity in [
            ReferentSort::Player,
            ReferentSort::Card,
            ReferentSort::Permanent,
        ] {
            assert!(!Sort::Amount.compatible_with(Sort::Referent(entity)));
            assert!(!Sort::Pile.compatible_with(Sort::Referent(entity)));
            assert!(!Sort::Referent(entity).compatible_with(Sort::Amount));
            assert!(!Sort::Referent(entity).compatible_with(Sort::Pile));
        }
        assert!(!Sort::Amount.compatible_with(Sort::Pile));
        assert!(!Sort::Pile.compatible_with(Sort::Amount));
        assert!(Sort::Amount.compatible_with(Sort::Amount));
        assert!(Sort::Pile.compatible_with(Sort::Pile));
    }

    /// The Entity-domain `compat` relation is unchanged by the split.
    #[test]
    fn referent_compat_is_preserved() {
        assert!(ReferentSort::StackObject.compatible_with(ReferentSort::Spell));
        assert!(!ReferentSort::Spell.compatible_with(ReferentSort::StackObject));
        assert!(ReferentSort::Permanent.compatible_with(ReferentSort::Token));
        assert!(
            ReferentSort::Permanent.compatible_with(ReferentSort::OfType(crate::Type::Creature))
        );
        assert!(!ReferentSort::Card.compatible_with(ReferentSort::Token));
        assert!(
            ReferentSort::OfType(crate::Type::Creature)
                .compatible_with(ReferentSort::OfType(crate::Type::Creature))
        );
        assert!(
            !ReferentSort::OfType(crate::Type::Creature)
                .compatible_with(ReferentSort::OfType(crate::Type::Land))
        );
    }

    /// A player noun ranges over the Player domain; every other Entity noun
    /// over the Object domain ([CR#109.1,102.1]).
    #[test]
    fn referent_sorts_name_their_entity_domain() {
        assert_eq!(ReferentSort::Player.domain(), crate::Domain::Player);
        assert_eq!(ReferentSort::Card.domain(), crate::Domain::Object);
        assert_eq!(ReferentSort::StackObject.domain(), crate::Domain::Object);
    }
}
