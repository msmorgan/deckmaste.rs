use deckmaste_core::{BeginningStep, CombatStep, EndingStep, Phase, Uint};

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
// DeclareBlockers and FirstCombatDamage both land on CombatDamage for different
// reasons (the Stage-3 first-strike skip vs. the future ordinary successor);
// keeping them separate keeps each [CR#510.4] comment attached to its arm.
#[expect(clippy::match_same_arms)]
pub fn successor(step: Phase) -> Option<Phase> {
    use BeginningStep::{Draw, Untap, Upkeep};
    use CombatStep::{
        BeginningOfCombat, CombatDamage, DeclareAttackers, DeclareBlockers, EndOfCombat,
        FirstCombatDamage,
    };
    use EndingStep::{Cleanup, End};
    use Phase::{Beginning, Combat, Ending, PostcombatMain, PrecombatMain};

    Some(match step {
        // Beginning phase ([CR#501-503]).
        Beginning(Untap) => Beginning(Upkeep),
        Beginning(Upkeep) => Beginning(Draw),
        Beginning(Draw) => PrecombatMain,
        // Precombat main ([CR#505]) → combat ([CR#506-511]).
        PrecombatMain => Combat(BeginningOfCombat),
        Combat(BeginningOfCombat) => Combat(DeclareAttackers),
        Combat(DeclareAttackers) => Combat(DeclareBlockers),
        // TODO(stage-4): first/double strike inserts Combat(FirstCombatDamage)
        //   before Combat(CombatDamage) ([CR#510.4]). Stage 3 has no first
        //   strike, so DeclareBlockers goes straight to the single combat-damage
        //   step.
        Combat(DeclareBlockers) => Combat(CombatDamage),
        // Unreachable in Stage 3 (no first strike); when wired, FirstCombatDamage
        // precedes the regular CombatDamage step ([CR#510.4]).
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
    use BeginningStep::{Draw, Untap, Upkeep};
    use CombatStep::{
        BeginningOfCombat, CombatDamage, DeclareAttackers, DeclareBlockers, EndOfCombat,
    };
    use EndingStep::{Cleanup, End};
    use Phase::{Beginning, Combat, Ending, PostcombatMain, PrecombatMain};

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
        // Stage 3 has no first strike: DeclareBlockers → CombatDamage,
        // skipping FirstCombatDamage ([CR#510.4]).
        assert_eq!(
            successor(Combat(DeclareBlockers)),
            Some(Combat(CombatDamage))
        );
        assert_eq!(successor(Combat(CombatDamage)), Some(Combat(EndOfCombat)));
        assert_eq!(successor(Combat(EndOfCombat)), Some(PostcombatMain));
        assert_eq!(successor(PostcombatMain), Some(Ending(End)));
        assert_eq!(successor(Ending(End)), Some(Ending(Cleanup)));
        assert_eq!(successor(Ending(Cleanup)), None);
    }
}
