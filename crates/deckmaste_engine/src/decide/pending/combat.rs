use crate::object::ObjectId;
use crate::player::PlayerId;

/// [CR#508.1a,508.1b]: the active player declares attackers, and what each
/// attacks. `legal` is the surfaced candidate attacker set; `legal_targets`
/// is what may be attacked — the defending player's proxy object plus every
/// planeswalker they control ([CR#506.3,508.1b]). `submit_decision`
/// re-validates each declared pair against both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareAttackers {
    pub player: PlayerId,
    pub legal: Vec<ObjectId>,
    pub legal_targets: Vec<ObjectId>,
}

/// [CR#509.1a]: the **defending** player declares blockers. `player` is the
/// defender (the non-active player); `legal` is the surfaced candidate set
/// of legal blockers; `submit_decision` re-validates against it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareBlockers {
    pub player: PlayerId,
    pub legal: Vec<ObjectId>,
}

/// [CR#510.1c]: divide `source`'s combat damage among its `recipients`
/// (free division — any split summing to `source`'s power is legal).
/// Surfaced only for a multi-blocked attacker (≥ 2 recipients); forced
/// cases auto-resolve. `player` is the source's controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignCombatDamage {
    pub player: PlayerId,
    pub source: ObjectId,
    pub recipients: Vec<ObjectId>,
}
