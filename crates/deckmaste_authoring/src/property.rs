use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Expand;
use crate::Modification;
use crate::OneShotEffect;
use crate::PhaseStep;
use crate::Reference;
use crate::ability::Ability;

/// What bearing a subtype or counter confers on objects that have it
/// ([CR#305.6,714.3a,714.3c,714.4,704.5m,122.1]), typed by execution flavor so
/// each piece gets the right semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Property {
    /// Joins ability derivation like printed text: layer-6-strippable,
    /// stack-using if triggered ([CR#305.6,714.3a]). Boxed like
    /// `Modification::GainAbility`; serde keeps the RON flat.
    Ability(Arc<Ability>),
    /// A continuous effect ([CR#611]) — the "boost" flavor. Applied at the
    /// `Modification`'s natural layer (`Modification` spans layers 2–7: P/T,
    /// `GainAbility`, colors, types, controller, …), so "any layer is
    /// conferrable". NOT an ability, so it is inherently strip-immune — a
    /// +1/+1 counter still pumps under `LoseAllAbilities`. A +1/+1 counter
    /// confers `Continuous(This, Several([Power(Up(CounterCount(This,
    /// P1P1Counter))), …]))` directly, rather than an ability that grants a
    /// boost. Positional single-object `Modify` twin ([`Reference`] +
    /// [`Modification`]); plurality, if ever needed, distributes with `Each`.
    Continuous(Reference, Modification),
    /// A subtype-derived state-based action, checked in the engine's
    /// [CR#704.3] sweep — no stack, no response window. The Saga sacrifice
    /// ([CR#714.4]) and the Aura attachment check ([CR#704.5m]) are the
    /// canonical instances. Parses today; the engine executes it in
    /// stage 3.
    StateBased {
        condition: Arc<Condition>,
        effect: Arc<OneShotEffect>,
    },
    /// Executed as a turn-based action — no stack ([CR#714.3c]). Parses
    /// today; the engine executes it in stage 3.
    TurnBased {
        at: PhaseStep,
        effect: Arc<OneShotEffect>,
    },
}

impl Property {
    /// Registry-conferral EMISSION ([CR#305.6,113.12]): the ability an
    /// `Ability`-flavored conferral contributes to its bearer's ability
    /// list, wrapped in [`Ability::Innate`] — a rule of the object, immune
    /// to layer-6 ability removal (a basic land that "loses all abilities"
    /// still taps for its color) and invisible to card-facing ability
    /// queries. THE one emission path: every registry (subtype, counter,
    /// designation) that turns a conferral into a bearer ability routes
    /// through here — no per-registry special case.
    ///
    /// `None` for the other flavors: they aren't ability emission — a
    /// `Continuous` conferral joins the layer system directly with its own
    /// timestamp (keyword counters are layer 6 BY RULE, [CR#613.1f], so
    /// their grants stay removable there), and `StateBased`/`TurnBased`
    /// execute in their own machinery.
    #[must_use]
    pub fn conferred_ability(&self) -> Option<Ability> {
        match self {
            Property::Ability(a) => Some(Ability::Innate(a.clone())),
            Property::Continuous { .. }
            | Property::StateBased { .. }
            | Property::TurnBased { .. } => None,
        }
    }
}
