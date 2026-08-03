use std::sync::Arc;

use macro_ron::Expand;
use serde::Deserialize;
use serde::Serialize;

use crate::Ability;
use crate::Color;
use crate::ManaCost;
use crate::StatValue;
use crate::Subtype;
use crate::Supertype;
use crate::TypeDef;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub struct CardFace {
    pub name: Arc<str>,

    #[serde(default, skip_serializing_if = "ManaCost::is_empty")]
    pub mana_cost: ManaCost,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub color_indicator: Vec<Color>,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub supertypes: Vec<Supertype>,

    pub types: Vec<TypeDef>,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub subtypes: Vec<Subtype>,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
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
// `TwoFaced` (two full `CardFace`s) is inherently larger than `Normal` (one).
// Boxing a face would push `Box::new` into every construction/read site and
// complicate the RON derive for no runtime gain — a `Card` is a rarely-copied
// authoring value, not a hot enum.
#[expect(
    clippy::large_enum_variant,
    reason = "leaf card model; boxing a face buys nothing"
)]
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
