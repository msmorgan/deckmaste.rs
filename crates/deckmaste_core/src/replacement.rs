use serde::Deserialize;
use serde::Serialize;

use crate::Count;
use crate::EventFilter;
use crate::Instruction;
use crate::PhaseStep;
use crate::Predicate;
use crate::continuous::Duration;

/// A replacement effect: the CR's closed template list ([CR#614]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Replacement {
    /// "If [event] would happen, [effect] instead" — replace ([CR#614.1a]).
    Instead {
        would: EventFilter,
        instead: Instruction,
    },
    /// Skip a step or phase — omit ([CR#614.1b]).
    Skip { what: PhaseStep },
    /// "If [event] would happen, [event] and [effect]" — augment, all-at-once
    /// ([CR#614.1c]). `AsEnters` is a prelude macro over this.
    Also {
        would: EventFilter,
        also: Instruction,
    },
}

/// A prevention effect ([CR#615]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Prevention {
    /// Prevent the next N damage from `from` to `to` ([CR#615.7]).
    /// `duration: None` = the carrier's implicit duration — a static's
    /// prevention lasts while it functions ([CR#611.3]); one-shots state
    /// theirs ([CR#615.3]).
    PreventNext {
        n: Count,
        from: Predicate,
        to: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        duration: Option<Duration>,
    },
    /// Prevent the next instance of damage ([CR#615.8]).
    PreventNextInstance { from: Predicate, to: Predicate },
    /// Prevent all damage from `from` to `to` ([CR#615.1,615.3] —
    /// Fog-style shields; no dedicated prevent-all rule). `duration: None`
    /// = the carrier's implicit duration (statics: while it functions,
    /// [CR#611.3] — protection's [CR#702.16e] clause rides this).
    PreventAll {
        from: Predicate,
        to: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        duration: Option<Duration>,
    },
}
