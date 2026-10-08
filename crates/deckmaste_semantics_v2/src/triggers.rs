//! Usage limits and trigger headers. Mirrors `lean/Semantics/Triggers.lean`.
//! `Timing` and `Duration` live in [`crate::phrase`], as they do in Lean's
//! phrase block.

use crate::phrase::Amount;
use crate::phrase::GameEvent;
use crate::phrase::Quantity;
use crate::phrase::Timing;
use crate::words::Period;
use macro_ron::Expand;
use macro_ron::SupportsMacros;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CountBound {
    MoreThan { amount: Amount },
    Additional { quantity: Quantity },
}

/// How often something may be done: no more than `times` per `per`
/// [CR#602.5b]. An event's ordinal ("the second spell each turn") is not a
/// usage limit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum UsageLimit {
    Mk { times: CountBound, per: Period },
}

/// A second trigger header joined to the first with "or".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct JoinedHeader {
    pub event: GameEvent,
    pub alternatives: Vec<GameEvent>,
    pub r#while: Option<Timing>,
    pub timing: Option<Timing>,
}
