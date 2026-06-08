use serde::{Deserialize, Serialize};

use crate::continuous::Duration;
use crate::{Count, Effect, Event, Filter, StepOrPhase};

/// A replacement effect: the CR's closed template list ([CR#614]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Replacement {
    /// "If [event] would happen, [effect] instead" ([CR#614.1a]).
    Instead { would: Event, instead: Effect },
    /// Skip a step or phase ([CR#614.1b]).
    Skip { what: StepOrPhase },
    /// As the object enters, apply an effect ([CR#614.1c,614.12]).
    AsEnters { effect: Effect },
    /// Redirect an event from one object to another ([CR#614.9]).
    Redirect { from: Filter, to: Filter },
}

/// A prevention effect ([CR#615]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Prevention {
    /// Prevent the next N damage from `from` to `to` ([CR#615.7]).
    PreventNext {
        n: Count,
        from: Filter,
        to: Filter,
        duration: Duration,
    },
    /// Prevent the next instance of damage ([CR#615.8]).
    PreventNextInstance { from: Filter, to: Filter },
    /// Prevent all damage from `from` to `to` for a duration ([CR#615.10]).
    PreventAll {
        from: Filter,
        to: Filter,
        duration: Duration,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_reads_flat() {
        let parsed: Replacement = crate::ron::options()
            .from_str("Skip(what: Upkeep)")
            .unwrap();
        assert_eq!(
            parsed,
            Replacement::Skip {
                what: StepOrPhase::Upkeep
            },
        );
    }
}
