use serde::{Deserialize, Serialize};

use crate::Ident;

type ParamValue = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct KeywordAbility {
    pub keyword: Ident,
    pub param_values: Vec<ParamValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Ability {
    Static,
    Activated,
    Triggered,
    Keyword(KeywordAbility),
}
