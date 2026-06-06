use serde::{Deserialize, Serialize};

use crate::{Ident, Selection};

// Temporary types.
type ParamValue = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Effect {
    DealDamage(Selection, crate::Uint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Expanded<T> {
    pub params: Vec<ParamValue>,
    pub value: Box<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeywordAbility {
    pub keyword: Ident,
    pub expanded: Expanded<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SpellAbility {
    pub targets: Vec<Selection>,
    pub effect: Effect,
}

// The struct-carrying variants read flat in RON — `Spell(targets: ..., ...)`,
// not `Spell((targets: ...))` — via the unwrap_variant_newtypes extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Ability {
    Static,
    Activated,
    Triggered,
    Spell(SpellAbility),
    Keyword(KeywordAbility),
}
