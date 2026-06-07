use deckmaste_core::{StepOrPhase, Uint};

use crate::player::PlayerId;

/// Where the turn is (CR 500). `current` starts as a pre-game placeholder;
/// the first `BeginStep(Untap)` begins turn 1.
#[derive(Debug, Clone)]
pub struct TurnState {
    pub active_player: PlayerId,
    /// 0 before the game starts; `BeginStep(Untap)` increments.
    pub turn_number: Uint,
    pub current: StepOrPhase,
    /// `Some` while a priority round is open (CR 117).
    pub priority: Option<PriorityRound>,
}

/// An open priority round (CR 117): who holds priority and how many players
/// have passed in succession (CR 117.4 ends the round at all-pass).
#[derive(Debug, Clone)]
pub struct PriorityRound {
    pub holder: PlayerId,
    pub consecutive_passes: Uint,
}

/// The next step in the skeleton's turn order, or `None` past Cleanup (the
/// caller begins the next turn). No attackers are ever declared in the
/// skeleton, so `DeclareAttackers` is followed by `EndOfCombat` (CR 508.8 skips
/// `DeclareBlockers` and `CombatDamage`).
#[must_use]
pub fn successor(step: StepOrPhase) -> Option<StepOrPhase> {
    use StepOrPhase::{
        BeginningOfCombat, Cleanup, CombatDamage, DeclareAttackers, DeclareBlockers, Draw,
        EndOfCombat, EndStep, PostcombatMain, PrecombatMain, Untap, Upkeep,
    };
    Some(match step {
        Untap => Upkeep,
        Upkeep => Draw,
        Draw => PrecombatMain,
        PrecombatMain => BeginningOfCombat,
        BeginningOfCombat => DeclareAttackers,
        // CR 508.8: no attackers (ever, in the skeleton) skips blocks/damage.
        DeclareAttackers | CombatDamage => EndOfCombat,
        DeclareBlockers => CombatDamage,
        EndOfCombat => PostcombatMain,
        PostcombatMain => EndStep,
        EndStep => Cleanup,
        Cleanup => return None,
    })
}

#[cfg(test)]
mod tests {
    use StepOrPhase::{
        BeginningOfCombat, Cleanup, DeclareAttackers, EndOfCombat, EndStep, PrecombatMain, Untap,
        Upkeep,
    };

    use super::*;

    #[test]
    fn successor_walks_the_turn_and_skips_combat() {
        assert_eq!(successor(Untap), Some(Upkeep));
        assert_eq!(successor(PrecombatMain), Some(BeginningOfCombat));
        // CR 508.8: no attackers in the skeleton.
        assert_eq!(successor(DeclareAttackers), Some(EndOfCombat));
        assert_eq!(successor(EndStep), Some(Cleanup));
        assert_eq!(successor(Cleanup), None);
    }
}
