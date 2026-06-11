use deckmaste_core::BeginningStep;
use deckmaste_core::CombatStep;
use deckmaste_core::EndingStep;
use deckmaste_core::Phase;
use deckmaste_core::Uint;

use crate::player::PlayerId;

/// Where the turn is ([CR#500]). `current` starts as a pre-game placeholder;
/// the first `BeginStep(Beginning(Untap))` begins turn 1.
#[derive(Debug, Clone)]
pub struct TurnState {
    pub active_player: PlayerId,
    /// 0 before the game starts; `BeginStep(Beginning(Untap))` increments.
    pub turn_number: Uint,
    pub current: Phase,
    /// `Some` while a priority round is open ([CR#117]).
    pub priority: Option<PriorityRound>,
}

/// An open priority round ([CR#117]): who holds priority and how many players
/// have passed in succession ([CR#117.4] ends the round at all-pass).
#[derive(Debug, Clone)]
pub struct PriorityRound {
    pub holder: PlayerId,
    pub consecutive_passes: Uint,
}

/// The next phase/step in the turn order ([CR#500]), or `None` past Cleanup
/// (the caller begins the next turn). Walks the `Phase` hierarchy in CR order.
#[must_use]
pub fn successor(step: Phase) -> Option<Phase> {
    use BeginningStep::Draw;
    use BeginningStep::Untap;
    use BeginningStep::Upkeep;
    use CombatStep::BeginningOfCombat;
    use CombatStep::CombatDamage;
    use CombatStep::DeclareAttackers;
    use CombatStep::DeclareBlockers;
    use CombatStep::EndOfCombat;
    use CombatStep::FirstCombatDamage;
    use EndingStep::Cleanup;
    use EndingStep::End;
    use Phase::Beginning;
    use Phase::Combat;
    use Phase::Ending;
    use Phase::PostcombatMain;
    use Phase::PrecombatMain;

    Some(match step {
        // Beginning phase ([CR#501-503]).
        Beginning(Untap) => Beginning(Upkeep),
        Beginning(Upkeep) => Beginning(Draw),
        Beginning(Draw) => PrecombatMain,
        // Precombat main ([CR#505]) → combat ([CR#506-511]).
        PrecombatMain => Combat(BeginningOfCombat),
        Combat(BeginningOfCombat) => Combat(DeclareAttackers),
        Combat(DeclareAttackers) => Combat(DeclareBlockers),
        // [CR#510.4]: the turn structure ALWAYS traverses FirstCombatDamage; it
        // is elided (begun-and-skipped) by `begin_step` when no combat creature
        // has first/double strike, so the structural successor is uniform.
        Combat(DeclareBlockers) => Combat(FirstCombatDamage),
        // [CR#510.4]: the first combat-damage step precedes the regular one (SBAs
        // run between them — a creature killed by first-strike damage deals
        // nothing in the regular step).
        Combat(FirstCombatDamage) => Combat(CombatDamage),
        Combat(CombatDamage) => Combat(EndOfCombat),
        Combat(EndOfCombat) => PostcombatMain,
        // Postcombat main ([CR#505]) → ending ([CR#512-514]).
        PostcombatMain => Ending(End),
        Ending(End) => Ending(Cleanup),
        Ending(Cleanup) => return None,
    })
}

#[cfg(test)]
mod tests {
    use BeginningStep::Draw;
    use BeginningStep::Untap;
    use BeginningStep::Upkeep;
    use CombatStep::BeginningOfCombat;
    use CombatStep::CombatDamage;
    use CombatStep::DeclareAttackers;
    use CombatStep::DeclareBlockers;
    use CombatStep::EndOfCombat;
    use CombatStep::FirstCombatDamage;
    use EndingStep::Cleanup;
    use EndingStep::End;
    use Phase::Beginning;
    use Phase::Combat;
    use Phase::Ending;
    use Phase::PostcombatMain;
    use Phase::PrecombatMain;

    use super::*;

    #[test]
    fn successor_walks_the_turn_in_cr_order() {
        assert_eq!(successor(Beginning(Untap)), Some(Beginning(Upkeep)));
        assert_eq!(successor(Beginning(Upkeep)), Some(Beginning(Draw)));
        assert_eq!(successor(Beginning(Draw)), Some(PrecombatMain));
        assert_eq!(successor(PrecombatMain), Some(Combat(BeginningOfCombat)));
        assert_eq!(
            successor(Combat(BeginningOfCombat)),
            Some(Combat(DeclareAttackers))
        );
        assert_eq!(
            successor(Combat(DeclareAttackers)),
            Some(Combat(DeclareBlockers))
        );
        // [CR#510.4]: the turn structure always traverses FirstCombatDamage
        // (elided in `begin_step` when no first/double strike exists).
        assert_eq!(
            successor(Combat(DeclareBlockers)),
            Some(Combat(FirstCombatDamage))
        );
        assert_eq!(
            successor(Combat(FirstCombatDamage)),
            Some(Combat(CombatDamage))
        );
        assert_eq!(successor(Combat(CombatDamage)), Some(Combat(EndOfCombat)));
        assert_eq!(successor(Combat(EndOfCombat)), Some(PostcombatMain));
        assert_eq!(successor(PostcombatMain), Some(Ending(End)));
        assert_eq!(successor(Ending(End)), Some(Ending(Cleanup)));
        assert_eq!(successor(Ending(Cleanup)), None);
    }
}
