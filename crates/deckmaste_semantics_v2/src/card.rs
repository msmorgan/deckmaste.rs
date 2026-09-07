//! The printed card: its faces and frames. Mirrors
//! `lean/Semantics/Card.lean`.

use crate::abilities::Ability;
use crate::abilities::Characteristics;
use crate::phrase::Amount;
use crate::words::ManaCost;
use crate::words::QualitySort;
use macro_ron::Expand;
use serde::Deserialize;
use serde::Serialize;

/// A printed face: its characteristics and the choices its text announces as it enters (the
/// joint-choice device the face checker reads).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct CardFace {
    pub characteristics: Characteristics,
    #[serde(default)]
    pub choices: Vec<QualitySort>,
}

/// One half of a split card whose halves share a type line: its own name, cost, and text.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct SharedLineHalf {
    pub name: String,
    #[serde(default)]
    pub cost: Option<ManaCost>,
    #[serde(default)]
    pub text: Vec<Ability>,
}

/// [CR#711.2a] a closed band, [CR#711.2b] the open last band.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LevelRange {
    Between { from: u32, to: u32 },
    AtLeast { from: u32 },
}

/// One striation of a leveler's text box: the level symbol's range, the abilities printed in
/// that striation, and its power/toughness box [CR#711.2a,711.2b].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct LevelBand {
    pub range: LevelRange,
    #[serde(default)]
    pub text: Vec<Ability>,
    #[serde(default)]
    pub power: Option<Amount>,
    #[serde(default)]
    pub toughness: Option<Amount>,
}

/// [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct PrototypeFrame {
    #[serde(default)]
    pub cost: Option<ManaCost>,
    #[serde(default)]
    pub power: Option<Amount>,
    #[serde(default)]
    pub toughness: Option<Amount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Card {
    SingleFaced {
        face: CardFace,
    },
    Transforming {
        front: CardFace,
        back: CardFace,
    },
    ModalDfc {
        front: CardFace,
        back: CardFace,
    },
    Split {
        left: CardFace,
        right: CardFace,
    },
    /// The shared line and box, and the two halves' own name, cost, and text.
    SharedLineSplit {
        shared: CardFace,
        left: SharedLineHalf,
        right: SharedLineHalf,
    },
    Adventurer {
        normal: CardFace,
        adventure: CardFace,
    },
    Flip {
        normal: CardFace,
        alternative: CardFace,
    },
    Leveler {
        inner: CardFace,
        bands: Vec<LevelBand>,
    },
    /// [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box.
    Prototype {
        inner: CardFace,
        alternative: PrototypeFrame,
    },
}
