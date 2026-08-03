//! `temporal` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::Timing {
    type Target = deckmaste_core::Timing;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::InstantSpeed => deckmaste_core::Timing::InstantSpeed,
            Self::SorcerySpeed => deckmaste_core::Timing::SorcerySpeed,
            Self::DuringTurn(f0) => deckmaste_core::Timing::DuringTurn(f0.lower()),
            Self::DuringStep(f0, f1) => deckmaste_core::Timing::DuringStep(f0.lower(), f1.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Lookback {
    type Target = deckmaste_core::Lookback;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ThisTurn => deckmaste_core::Lookback::ThisTurn,
            Self::ThisGame => deckmaste_core::Lookback::ThisGame,
            Self::LastTurn => deckmaste_core::Lookback::LastTurn,
            Self::ThisCombat => deckmaste_core::Lookback::ThisCombat,
            Self::ThisStep => deckmaste_core::Lookback::ThisStep,
            Self::SinceYour(f0) => deckmaste_core::Lookback::SinceYour(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::TurnMarker {
    type Target = deckmaste_core::TurnMarker;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::EndOfTurn => deckmaste_core::TurnMarker::EndOfTurn,
            Self::EndOfCombat => deckmaste_core::TurnMarker::EndOfCombat,
            Self::YourNextTurn => deckmaste_core::TurnMarker::YourNextTurn,
        }
    }
}

impl Lower for deckmaste_authoring::LockPoint {
    type Target = deckmaste_core::LockPoint;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Announce => deckmaste_core::LockPoint::Announce,
            Self::StackPlacement => deckmaste_core::LockPoint::StackPlacement,
            Self::TotalCost => deckmaste_core::LockPoint::TotalCost,
            Self::Payment => deckmaste_core::LockPoint::Payment,
            Self::EffectBegin => deckmaste_core::LockPoint::EffectBegin,
            Self::CopyCreation => deckmaste_core::LockPoint::CopyCreation,
            Self::Declaration => deckmaste_core::LockPoint::Declaration,
            Self::PreGame => deckmaste_core::LockPoint::PreGame,
            Self::Resolution => deckmaste_core::LockPoint::Resolution,
            Self::Never => deckmaste_core::LockPoint::Never,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_timing_instant_speed() {
        assert_matches!(
            deckmaste_authoring::Timing::InstantSpeed.lower(),
            deckmaste_core::Timing::InstantSpeed
        );
    }

    #[test]
    fn lowers_timing_sorcery_speed() {
        assert_matches!(
            deckmaste_authoring::Timing::SorcerySpeed.lower(),
            deckmaste_core::Timing::SorcerySpeed
        );
    }

    #[test]
    fn lowers_timing_during_turn() {
        assert_matches!(
            deckmaste_authoring::Timing::DuringTurn(minimal_whose_turn()).lower(),
            deckmaste_core::Timing::DuringTurn(deckmaste_core::WhoseTurn::Your)
        );
    }

    #[test]
    fn lowers_timing_during_step() {
        assert_matches!(
            deckmaste_authoring::Timing::DuringStep(minimal_phase_step(), minimal_whose_turn())
                .lower(),
            deckmaste_core::Timing::DuringStep(
                deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap),
                deckmaste_core::WhoseTurn::Your
            )
        );
    }

    #[test]
    fn lowers_lookback_this_turn() {
        assert_matches!(
            deckmaste_authoring::Lookback::ThisTurn.lower(),
            deckmaste_core::Lookback::ThisTurn
        );
    }

    #[test]
    fn lowers_lookback_this_game() {
        assert_matches!(
            deckmaste_authoring::Lookback::ThisGame.lower(),
            deckmaste_core::Lookback::ThisGame
        );
    }

    #[test]
    fn lowers_lookback_last_turn() {
        assert_matches!(
            deckmaste_authoring::Lookback::LastTurn.lower(),
            deckmaste_core::Lookback::LastTurn
        );
    }

    #[test]
    fn lowers_lookback_this_combat() {
        assert_matches!(
            deckmaste_authoring::Lookback::ThisCombat.lower(),
            deckmaste_core::Lookback::ThisCombat
        );
    }

    #[test]
    fn lowers_lookback_this_step() {
        assert_matches!(
            deckmaste_authoring::Lookback::ThisStep.lower(),
            deckmaste_core::Lookback::ThisStep
        );
    }

    #[test]
    fn lowers_lookback_since_your() {
        assert_matches!(
            deckmaste_authoring::Lookback::SinceYour(minimal_phase_step()).lower(),
            deckmaste_core::Lookback::SinceYour(deckmaste_core::PhaseStep::Beginning(
                deckmaste_core::BeginningStep::Untap
            ))
        );
    }

    #[test]
    fn lowers_turn_marker_end_of_turn() {
        assert_matches!(
            deckmaste_authoring::TurnMarker::EndOfTurn.lower(),
            deckmaste_core::TurnMarker::EndOfTurn
        );
    }

    #[test]
    fn lowers_turn_marker_end_of_combat() {
        assert_matches!(
            deckmaste_authoring::TurnMarker::EndOfCombat.lower(),
            deckmaste_core::TurnMarker::EndOfCombat
        );
    }

    #[test]
    fn lowers_turn_marker_your_next_turn() {
        assert_matches!(
            deckmaste_authoring::TurnMarker::YourNextTurn.lower(),
            deckmaste_core::TurnMarker::YourNextTurn
        );
    }

    #[test]
    fn lowers_lock_point_announce() {
        assert_matches!(
            deckmaste_authoring::LockPoint::Announce.lower(),
            deckmaste_core::LockPoint::Announce
        );
    }

    #[test]
    fn lowers_lock_point_stack_placement() {
        assert_matches!(
            deckmaste_authoring::LockPoint::StackPlacement.lower(),
            deckmaste_core::LockPoint::StackPlacement
        );
    }

    #[test]
    fn lowers_lock_point_total_cost() {
        assert_matches!(
            deckmaste_authoring::LockPoint::TotalCost.lower(),
            deckmaste_core::LockPoint::TotalCost
        );
    }

    #[test]
    fn lowers_lock_point_payment() {
        assert_matches!(
            deckmaste_authoring::LockPoint::Payment.lower(),
            deckmaste_core::LockPoint::Payment
        );
    }

    #[test]
    fn lowers_lock_point_effect_begin() {
        assert_matches!(
            deckmaste_authoring::LockPoint::EffectBegin.lower(),
            deckmaste_core::LockPoint::EffectBegin
        );
    }

    #[test]
    fn lowers_lock_point_copy_creation() {
        assert_matches!(
            deckmaste_authoring::LockPoint::CopyCreation.lower(),
            deckmaste_core::LockPoint::CopyCreation
        );
    }

    #[test]
    fn lowers_lock_point_declaration() {
        assert_matches!(
            deckmaste_authoring::LockPoint::Declaration.lower(),
            deckmaste_core::LockPoint::Declaration
        );
    }

    #[test]
    fn lowers_lock_point_pre_game() {
        assert_matches!(
            deckmaste_authoring::LockPoint::PreGame.lower(),
            deckmaste_core::LockPoint::PreGame
        );
    }

    #[test]
    fn lowers_lock_point_resolution() {
        assert_matches!(
            deckmaste_authoring::LockPoint::Resolution.lower(),
            deckmaste_core::LockPoint::Resolution
        );
    }

    #[test]
    fn lowers_lock_point_never() {
        assert_matches!(
            deckmaste_authoring::LockPoint::Never.lower(),
            deckmaste_core::LockPoint::Never
        );
    }
}
