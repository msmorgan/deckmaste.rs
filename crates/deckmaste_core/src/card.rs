use serde::{Deserialize, Serialize};

use crate::ability::Ability;
use crate::{Color, ManaCost, Subtype, Supertype, Type};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Card {
    Normal(CardFace),
    ModalDfc(CardFace, CardFace),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum StatValue {
    // Power or toughness set by a characteristic-defining ability.
    // Any power or toughness containing * is essentially reminder text.
    DefinedByAbility,

    // Loyalty set to X from casting cost.
    Variable,

    #[serde(untagged)]
    Number(crate::Int),
}
