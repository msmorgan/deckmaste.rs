use serde::Deserialize;
use serde::Serialize;

use crate::Color;
use crate::Expand;
use crate::ManaCost;
use crate::Subtype;
use crate::Supertype;
use crate::Type;
use crate::ability::Ability;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub struct CardFace {
    pub name: String,

    #[serde(default, skip_serializing_if = "ManaCost::is_empty")]
    pub mana_cost: ManaCost,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub color_indicator: Vec<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<Supertype>,

    pub types: Vec<Type>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<Subtype>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<Ability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<StatValue>,
}

/// How a two-faced card's faces are arranged — the Idris `FaceLayout`. The
/// layouts share ONE card shape (two full faces, [CR#712.8] — each face has
/// its own characteristics); what differs per layout — transforming, casting
/// either face, casting halves, the adventure exile state, flipping — is the
/// engine's job, not the grammar's. Single-faced "layouts" (saga, class,
/// leveler, …) are NOT here: their mechanics ride subtypes and abilities on a
/// `Normal` card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum FaceLayout {
    /// A transforming (nonmodal) double-faced card ([CR#712.2]).
    Transforming,
    /// A modal double-faced card — play either face ([CR#712.3]).
    ModalDfc,
    /// A split card — two halves on one face ([CR#709.1]).
    Split,
    /// An adventurer card — a permanent face with an Adventure spell face
    /// ([CR#715.1]).
    Adventure,
    /// A Kamigawa-style flip card — one card, flipped half ([CR#710.1]).
    Flip,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub enum Card {
    Normal(CardFace),
    /// A TWO-faced card: `front` (the primary/default face) and `back`,
    /// arranged per `layout` — the Idris `TwoFaced`. Each face is a full
    /// [`CardFace`] with its own characteristics ([CR#712.8]).
    TwoFaced {
        layout: FaceLayout,
        front: CardFace,
        back: CardFace,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum StatValue {
    // Power or toughness set by a characteristic-defining ability.
    // Any power or toughness containing * is essentially reminder text.
    DefinedByAbility,

    // Loyalty set to X from casting cost.
    Variable,

    #[serde(untagged)]
    Number(crate::Int),
}
