use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Color;
use deckmaste_core::ManaCost;
use deckmaste_core::StatValue;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::TypeDef;
use serde::Deserialize;
use serde::Serialize;

/// The printed characteristics [CR#109.3] shared by a Card Face and by a
/// rules-defined set of Alternative Characteristics (a flip card's upside-down
/// half [CR#710.1], an adventurer card's inset Adventure characteristics
/// [CR#715.2]).
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Characteristics {
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

/// A Card Face: the [`Characteristics`] printed on something the CR calls a
/// face — nonmeld double-faced front/back [CR#712.1], split's two faces on one
/// side [CR#709.1], and the ordinary (non-alternative) side of a flip or
/// adventurer card — plus whatever else its layout adds, which today is
/// nothing.
///
/// A rules-defined Alternative Characteristics set is not a face, and the
/// types say so: a flip card's upside-down half [CR#710.1] takes
/// [`Characteristics`], never a `CardFace`.
///
/// ```compile_fail
/// use deckmaste_card::Card;
/// use deckmaste_card::CardFace;
/// use deckmaste_card::Characteristics;
///
/// let face = CardFace::from(Characteristics::default());
/// let _ = Card::Flip {
///     normal: face.clone(),
///     alternative: face,
/// };
/// ```
///
/// ```
/// use deckmaste_card::Card;
/// use deckmaste_card::CardFace;
/// use deckmaste_card::Characteristics;
///
/// let _ = Card::Flip {
///     normal: CardFace::from(Characteristics::default()),
///     alternative: Characteristics::default(),
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CardFace {
    pub characteristics: Characteristics,
}

impl From<Characteristics> for CardFace {
    fn from(characteristics: Characteristics) -> Self {
        Self { characteristics }
    }
}

/// How a nonmeld double-faced card's two faces are arranged. Meld's oversized
/// back ([CR#712.4]) is the exception [CR#712.1] states and isn't a
/// [`CardFace`] pair, so it isn't a layout here — `DoubleFaced` never stands
/// in for a meld card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DoubleFacedLayout {
    /// A transforming (nonmodal) double-faced card ([CR#712.2]).
    Transforming,
    /// A modal double-faced card — play either face ([CR#712.3]).
    ModalDfc,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Card {
    Normal(CardFace),
    /// A nonmeld double-faced card: `front` (the primary/default face) and
    /// `back`, both full Card Faces with their own characteristics
    /// ([CR#712.1,712.8]).
    DoubleFaced {
        layout: DoubleFacedLayout,
        front: CardFace,
        back: CardFace,
    },
    /// A split card: two Card Faces on one side ([CR#709.1]).
    Split {
        left: CardFace,
        right: CardFace,
    },
    /// A flip card: one Card Face plus Alternative Characteristics used
    /// according to flipped status, never a second face ([CR#710.1]).
    Flip {
        normal: CardFace,
        alternative: Characteristics,
    },
    /// An adventurer card: normal characteristics plus the Alternative
    /// Characteristics of its Adventure spell — not an Adventure face
    /// ([CR#715.2]).
    Adventurer {
        normal: CardFace,
        adventure: Characteristics,
    },
}

impl Card {
    /// The card's primary Card Face: its one face, or the front/left/normal
    /// side of a multi-part card — the face an object presents off the
    /// battlefield ([CR#712.8a]) and by default on it ([CR#712.8d]).
    #[must_use]
    pub fn primary_face(&self) -> &CardFace {
        match self {
            Card::Normal(face)
            | Card::DoubleFaced { front: face, .. }
            | Card::Split { left: face, .. }
            | Card::Flip { normal: face, .. }
            | Card::Adventurer { normal: face, .. } => face,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn characteristics(name: &str) -> Characteristics {
        Characteristics {
            name: name.into(),
            ..Characteristics::default()
        }
    }

    fn face(name: &str) -> CardFace {
        CardFace::from(characteristics(name))
    }

    /// Delver of Secrets // Insectile Aberration: a nonmeld double-faced card
    /// has front and back Card Faces ([CR#712.1]).
    #[test]
    fn double_faced_witness_has_front_and_back_card_faces() {
        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front: face("Delver of Secrets"),
            back: face("Insectile Aberration"),
        };
        let Card::DoubleFaced { front, back, .. } = &card else {
            panic!("not a DoubleFaced card");
        };
        assert_eq!(front.characteristics.name.as_ref(), "Delver of Secrets");
        assert_eq!(back.characteristics.name.as_ref(), "Insectile Aberration");
        assert_eq!(
            card.primary_face().characteristics.name.as_ref(),
            "Delver of Secrets"
        );
    }

    /// Fire // Ice: a split card has two card faces on one side ([CR#709.1]).
    #[test]
    fn split_witness_has_two_card_faces_on_one_side() {
        let card = Card::Split {
            left: face("Fire"),
            right: face("Ice"),
        };
        let Card::Split { left, right } = &card else {
            panic!("not a Split card");
        };
        assert_eq!(left.characteristics.name.as_ref(), "Fire");
        assert_eq!(right.characteristics.name.as_ref(), "Ice");
        assert_eq!(card.primary_face().characteristics.name.as_ref(), "Fire");
    }

    /// Bushi Tenderfoot // Kenzo the Hardhearted: a flip card has one card
    /// face plus alternative characteristics used according to flipped
    /// status ([CR#710.1]).
    #[test]
    fn flip_witness_has_one_face_and_alternative_characteristics() {
        let card = Card::Flip {
            normal: face("Bushi Tenderfoot"),
            alternative: characteristics("Kenzo the Hardhearted"),
        };
        let Card::Flip {
            normal,
            alternative,
        } = &card
        else {
            panic!("not a Flip card");
        };
        assert_eq!(normal.characteristics.name.as_ref(), "Bushi Tenderfoot");
        assert_eq!(alternative.name.as_ref(), "Kenzo the Hardhearted");
        assert_eq!(
            card.primary_face().characteristics.name.as_ref(),
            "Bushi Tenderfoot"
        );
    }

    /// Merfolk Secretkeeper // Venture Deeper: an adventurer card has normal
    /// characteristics plus alternative Adventure characteristics, not an
    /// Adventure face ([CR#715.2]).
    #[test]
    fn adventurer_witness_has_normal_and_alternative_adventure_characteristics() {
        let card = Card::Adventurer {
            normal: face("Merfolk Secretkeeper"),
            adventure: characteristics("Venture Deeper"),
        };
        let Card::Adventurer { normal, adventure } = &card else {
            panic!("not an Adventurer card");
        };
        assert_eq!(normal.characteristics.name.as_ref(), "Merfolk Secretkeeper");
        assert_eq!(adventure.name.as_ref(), "Venture Deeper");
        assert_eq!(
            card.primary_face().characteristics.name.as_ref(),
            "Merfolk Secretkeeper"
        );
    }

    /// A face adds nothing to its characteristics, so it is serde-transparent:
    /// a card written against the flat pre-split shape still reads, and writes
    /// back the same way.
    #[test]
    fn a_face_is_serde_transparent_over_its_characteristics() {
        let card: Card = deckmaste_core::ron::options()
            .from_str(r#"Normal((name: "Forest", types: []))"#)
            .expect("a flat Normal card reads");
        assert_eq!(card.primary_face().characteristics.name.as_ref(), "Forest");
        assert_eq!(
            deckmaste_core::ron::options()
                .to_string(&card)
                .expect("a card writes"),
            r#"Normal((name:"Forest",types:[]))"#
        );
    }
}
