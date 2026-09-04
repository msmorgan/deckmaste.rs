use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Instruction;
use crate::Modification;
use crate::PhaseStep;
use crate::Reference;
use crate::Region;
use crate::StaticSpec;
use crate::ability::Ability;

/// What bearing a subtype or counter confers on objects that have it
/// ([CR#305.6,714.3a,714.3c,714.4,704.5m,122.1]), typed by execution flavor so
/// each piece gets the right semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
    /// A conferred state-based action ([CR#704]), checked in the engine's
    /// [CR#704.3] sweep — no stack, no response window. THE home for a type's,
    /// subtype's or counter's [CR#704] rule: an SBA is a game action, never an
    /// ability ([CR#704.1,704.1a]), so this flavor confers no ability at all
    /// (see [`Property::conferred_ability`]). The Aura attachment check
    /// ([CR#704.5m]), the Saga sacrifice ([CR#714.4]) and the -1/-1
    /// annihilation ([CR#704.5q]) are the canonical instances.
    StateBased {
        condition: Arc<Condition>,
        effect: Arc<Instruction>,
    },
    /// Executed as a turn-based action — no stack ([CR#714.3c]). Parses
    /// today; the engine executes it in stage 3.
    TurnBased {
        at: PhaseStep,
        effect: Arc<Instruction>,
    },
    /// A static rule of the object, conferred WITHOUT an ability ([CR#113.12]:
    /// an effect that "states a quality of that object" is neither granting an
    /// ability nor setting a characteristic). THE home for a type's or
    /// subtype's deontic rules: the Land play permission ([CR#116.2a,305.9]), the
    /// Creature attack/block permissions and their summoning-sickness `Cant`
    /// pair ([CR#508.1a,509.1a,302.6]), and the Equipment ([CR#301.5]) /
    /// Fortification ([CR#301.6]) legal-host rule — "an Equipment can be
    /// attached to a creature" is a quality of Equipment, not an ability of
    /// one.
    ///
    /// Same payload as [`Ability::Static`], so the RON text of a rule that
    /// moves here is unchanged apart from dropping the `Ability(…)` wrapper.
    /// The engine's static walk reads these off the bearer's DERIVED
    /// types/subtypes (`legal::statics_on`), so they are invisible to
    /// card-facing ability queries and untouched by layer-6 ability removal BY
    /// CONSTRUCTION — no removal-immune ability class is needed.
    Static(Arc<Region<StaticSpec>>),
}

impl Property {
    /// Registry-conferral EMISSION ([CR#305.6]): the ability an
    /// `Ability`-flavored conferral contributes to its bearer's ability list,
    /// UNWRAPPED — a conferred ability occupies the ordinary ability
    /// hierarchy, so it is visible to card-facing queries and removable at
    /// layer 6 like printed text. A basic land type's intrinsic "{T}: Add
    /// [mana]" ([CR#305.6]) and a Saga's intrinsic enters-with-a-lore-counter
    /// replacement ([CR#714.3a]) are both real abilities and behave like it.
    /// THE one emission path: every registry (subtype, counter, designation)
    /// that turns a conferral into a bearer ability routes through here — no
    /// per-registry special case.
    ///
    /// `None` for the other flavors: they aren't ability emission — a
    /// `Continuous` conferral joins the layer system directly with its own
    /// timestamp (keyword counters are layer 6 BY RULE, [CR#613.1f], so
    /// their grants stay removable there), `Static` is an ability-free rule
    /// read by the static walk ([`Property::conferred_static`]), and
    /// `StateBased`/`TurnBased` execute in their own machinery.
    #[must_use]
    pub fn conferred_ability(&self) -> Option<Ability> {
        match self {
            Property::Ability(a) => Some(a.as_ref().clone()),
            Property::Continuous { .. }
            | Property::StateBased { .. }
            | Property::TurnBased { .. }
            | Property::Static(_) => None,
        }
    }

    /// The ability-free static rule this conferral contributes, if any — the
    /// twin of [`conferred_ability`](Self::conferred_ability) for the
    /// [`Property::Static`] flavor. The engine's static walk reads it off the
    /// bearer's derived types/subtypes; it never enters an ability list.
    #[must_use]
    pub fn conferred_static(&self) -> Option<&Arc<Region<StaticSpec>>> {
        match self {
            Property::Static(s) => Some(s),
            Property::Ability(_)
            | Property::Continuous { .. }
            | Property::StateBased { .. }
            | Property::TurnBased { .. } => None,
        }
    }
}
