use serde::{Deserialize, Serialize};

use crate::{Ident, Type};

// Temporary types.
type ParamValue = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Target {
    OneOf(Vec<Target>),
    PermanentOfType(Type),
    Player,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Selector {
    Target(usize),
    GameObject(usize),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Effect {
    DealDamage(Selector, crate::Uint),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Expanded<T> {
    pub params: Vec<ParamValue>,
    pub value: Box<T>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct KeywordAbility {
    pub keyword: Ident,
    pub expanded: Expanded<Ability>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct SpellAbility {
    pub targets: Vec<Target>,
    pub effect: Effect,
}

// The struct-carrying variants read flat in RON — `Spell(targets: ..., ...)`,
// not `Spell((targets: ...))` — via the unwrap_variant_newtypes extension.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Ability {
    Static,
    Activated,
    Triggered,
    Spell(SpellAbility),
    Keyword(KeywordAbility),
}
