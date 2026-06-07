use serde::{Deserialize, Serialize};

use crate::ability::Ability;
use crate::{Condition, Effect, StepOrPhase};

/// What bearing a subtype confers on objects that have it (CR 305.6, 714.3,
/// 704.5m), typed by execution flavor so each piece gets the right
/// semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Property {
    /// Joins ability derivation like printed text: layer-6-strippable,
    /// stack-using if triggered (CR 305.6, 714.3a). Boxed like
    /// `Modification::GainAbility`; serde keeps the RON flat.
    Ability(Box<Ability>),
    /// Checked in the CR 704.3 state-based sweep — no stack, no response
    /// window (CR 714.4, 704.5m). Parses today; the engine executes it in
    /// stage 3.
    StateBased {
        condition: Condition,
        effect: Effect,
    },
    /// Performed as a turn-based action — no stack (CR 714.3c). Parses
    /// today; the engine executes it in stage 3.
    TurnBased { at: StepOrPhase, effect: Effect },
}
