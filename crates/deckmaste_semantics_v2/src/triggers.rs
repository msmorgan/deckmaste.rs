//! Game events, durations, and trigger headers. Mirrors
//! `lean/Semantics/Triggers.lean`.

use crate::phrase::Condition;
use crate::phrase::DurationEnd;
use crate::phrase::GameEvent;
use crate::phrase::NounPhrase;
use crate::words::TurnPart;
use macro_ron::Expand;
use macro_ron::SupportsMacros;
use serde::Deserialize;
use serde::Serialize;

/// When an ability may be used or a trigger fires: "any time you could cast a sorcery",
/// "during your turn", "before attackers are declared".
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Timing {
    AsSorcery,
    AsInstant,
    DuringPart {
        part: TurnPart,
        whose: Option<NounPhrase>,
    },
    BeforePart {
        part: TurnPart,
        whose: Option<NounPhrase>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Duration {
    ThisTurn,
    RestOfGame,
    Until { end: DurationEnd },
    ForAsLongAs { condition: Condition },
    UntilEvent { event: GameEvent },
    DuringNextTurnOf { player: NounPhrase },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
pub enum UsageLimit {
    OncePerTurn,
    OncePerGame,
    ActionOncePerTurn,
}

/// "while …": a condition or an event underway.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Concurrent {
    WhileTrue { condition: Condition },
    WhileDoing { event: GameEvent },
}

/// A second trigger header joined to the first with "or".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct JoinedHeader {
    pub event: GameEvent,
    pub alternatives: Vec<GameEvent>,
    pub r#while: Option<Concurrent>,
    pub timing: Option<Timing>,
}
