use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Effect;
use crate::Expand;
use crate::Modification;
use crate::Phase;
use crate::ability::Ability;
use crate::continuous::Scope;

/// What bearing a subtype or counter confers on objects that have it
/// ([CR#305.6,714.3a,714.3c,714.4,704.5m,122.1]), typed by execution flavor so
/// each piece gets the right semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Property {
    /// Joins ability derivation like printed text: layer-6-strippable,
    /// stack-using if triggered ([CR#305.6,714.3a]). Boxed like
    /// `Modification::GainAbility`; serde keeps the RON flat.
    Ability(Box<Ability>),
    /// A continuous effect ([CR#611]) — the "boost" flavor. Applied at the
    /// `Modification`'s natural layer (`Modification` spans layers 2–7: P/T,
    /// `GainAbility`, colors, types, controller, …), so "any layer is
    /// conferrable". NOT an ability, so it is inherently strip-immune — a
    /// +1/+1 counter still pumps under `LoseAllAbilities`. A +1/+1 counter
    /// confers `Continuous(of: Of(This), changes: [Power(Up(CounterCount(This,
    /// P1P1Counter))), …])` directly, rather than an ability that grants a
    /// boost.
    Continuous {
        of: Scope,
        changes: Vec<Modification>,
    },
    /// A subtype-derived state-based action, checked in the engine's
    /// [CR#704.3] sweep — no stack, no response window. The Saga sacrifice
    /// ([CR#714.4]) and the Aura attachment check ([CR#704.5m]) are the
    /// canonical instances. Parses today; the engine executes it in
    /// stage 3.
    StateBased {
        condition: Box<Condition>,
        effect: Box<Effect>,
    },
    /// Performed as a turn-based action — no stack ([CR#714.3c]). Parses
    /// today; the engine executes it in stage 3.
    TurnBased { at: Phase, effect: Box<Effect> },
}
