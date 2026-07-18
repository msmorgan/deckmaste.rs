use deckmaste_core::Uint;

use crate::player::PlayerId;

/// [CR#106.1b]: a resolving `AddMana` whose production is a choice ("any
/// color" offers the five colors per [CR#105.4]; "{W} or {U}" offers its
/// printed set) — `player` picks one of `options`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseManaColor {
    pub player: PlayerId,
    pub options: Vec<deckmaste_core::ColorOrColorless>,
    pub amount: Uint,
    pub riders: Vec<deckmaste_core::ManaRider>,
}

/// [CR#106.1b]: a resolving `AddMana` whose production is a choice among
/// multi-symbol runs (the filterland cycle "{W}{W}, {W}{U}, or {U}{U}") —
/// `player` picks one of `options` (each a run of mana), and the chosen
/// run's whole sequence lands. The answer ([`Decision::ManaMode`]) is the
/// chosen option index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseManaMode {
    pub player: PlayerId,
    pub options: Vec<Vec<deckmaste_core::ColorOrColorless>>,
    pub amount: Uint,
    pub riders: Vec<deckmaste_core::ManaRider>,
}
