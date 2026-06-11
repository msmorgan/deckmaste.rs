use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Effect;
use crate::Expand;
use crate::Phase;
use crate::ability::Ability;

/// What bearing a subtype confers on objects that have it
/// ([CR#305.6,714.3a,714.3c,714.4,704.5m]), typed by execution flavor so each
/// piece gets the right semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Property {
    /// Joins ability derivation like printed text: layer-6-strippable,
    /// stack-using if triggered ([CR#305.6,714.3a]). Boxed like
    /// `Modification::GainAbility`; serde keeps the RON flat.
    Ability(Box<Ability>),
    /// A subtype-derived state-based action, checked in the engine's
    /// [CR#704.3] sweep — no stack, no response window. The Saga sacrifice
    /// ([CR#714.4]) and the Aura attachment check ([CR#704.5m]) are the
    /// canonical instances. Parses today; the engine executes it in
    /// stage 3.
    StateBased {
        condition: Condition,
        effect: Effect,
    },
    /// Performed as a turn-based action — no stack ([CR#714.3c]). Parses
    /// today; the engine executes it in stage 3.
    TurnBased { at: Phase, effect: Effect },
}
