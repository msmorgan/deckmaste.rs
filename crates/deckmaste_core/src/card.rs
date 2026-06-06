use serde::{Deserialize, Serialize};

use crate::ability::Ability;
use crate::{Color, Ident, ManaCost};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct CardFace {
    name: String,

    #[serde(skip_serializing_if = "ManaCost::is_empty")]
    mana_cost: ManaCost,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    color_indicator: Vec<Color>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    supertypes: Vec<Ident>,

    types: Vec<Ident>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    subtypes: Vec<Ident>,

    abilities: Vec<Ability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    toughness: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    loyalty: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    defense: Option<StatValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum Card {
    Normal(CardFace),
    ModalDfc(CardFace, CardFace),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum StatValue {
    DefinedByAbility, // Power or toughness set by a characteristic-defining ability.
    Variable, // Loyalty set to X when cast.
    #[serde(untagged)]
    Number(crate::Int),
}
