use deckmaste_core::Uint;

use crate::object::ObjectId;
use crate::player::PlayerId;

/// [CR#601.2c,115]: choose targets for the in-flight announce. `legal[i]`
/// is the candidate set for `spec[i]`; `submit_decision` re-validates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseTargets {
    pub player: PlayerId,
    pub spec: Vec<deckmaste_core::TargetSpec>,
    pub legal: Vec<Vec<ObjectId>>,
}

/// [CR#707.10c,115.7d]: re-target a COMMITTED stack entry — surface a
/// `ChooseNewTargets` decision whose per-slot legal set is the fresh
/// legal candidates PLUS the current target (leaving a slot unchanged
/// is always allowed, even when the current target is illegal; a
/// CHANGED slot must be legal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseNewTargets {
    pub player: PlayerId,
    pub entry: ObjectId,
    pub spec: Vec<deckmaste_core::TargetSpec>,
    pub legal: Vec<Vec<ObjectId>>,
}

/// [CR#601.2g]: allocate pool mana to the in-flight cost. `subject` is the
/// object being paid for — the spell, or an activated ability's source
/// ([CR#106.6]) — so a `SpendOnly` rider can judge it at validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayMana {
    pub player: PlayerId,
    pub cost: deckmaste_core::ManaCost,
    pub pool: crate::player::ManaPool,
    pub subject: ObjectId,
}

/// Announce-time cost intentions ([CR#601.2b]): the player announces the
/// nonhybrid equivalent of each hybrid symbol ([CR#107.4e]) and, for each
/// Phyrexian symbol, color-or-2-life ([CR#107.4f]). `options[i]` is the
/// legal readings of the i-th choosable symbol of `cost` (cost order); the
/// answer ([`Decision::CostOptions`]) supplies one pick per entry. Kicker
/// and alternative-cost selection will join this kind in a later task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseCostOptions {
    pub player: PlayerId,
    pub cost: deckmaste_core::ManaCost,
    // pre-computed from choosable(&cost); redundancy is intentional so the
    // player's answer can be validated without re-reading the cost.
    pub options: crate::cost_options::ChoosableOptions,
}

/// [CR#601.2b]: announce the value of `{X}` in the in-flight cost. Any value
/// >= 0 is accepted; an unpayable announcement rewinds the cast ([CR#733]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseXValue {
    pub player: PlayerId,
}

/// Choose a modal spell/ability's modes ([CR#700.2a..700.2b]). `options` is
/// how many modes are offered; the answer ([`Decision::Modes`]) is a list
/// of option indices, `min..=max` entries long, distinct unless
/// `repeats` ([CR#700.2d]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseModes {
    pub player: PlayerId,
    pub options: Uint,
    pub min: Uint,
    pub max: Uint,
    pub repeats: bool,
}
