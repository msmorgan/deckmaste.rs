//! `temporal` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

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

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_timing_instant_speed() {
        assert_lowers(deckmaste_authoring::Timing::InstantSpeed);
    }

    #[test]
    fn lowers_timing_sorcery_speed() {
        assert_lowers(deckmaste_authoring::Timing::SorcerySpeed);
    }

    #[test]
    fn lowers_timing_during_turn() {
        assert_lowers(deckmaste_authoring::Timing::DuringTurn(minimal_whose_turn()));
    }

    #[test]
    fn lowers_timing_during_step() {
        assert_lowers(deckmaste_authoring::Timing::DuringStep(
            minimal_phase_step(),
            minimal_whose_turn(),
        ));
    }

    #[test]
    fn lowers_lookback_this_turn() {
        assert_lowers(deckmaste_authoring::Lookback::ThisTurn);
    }

    #[test]
    fn lowers_lookback_this_game() {
        assert_lowers(deckmaste_authoring::Lookback::ThisGame);
    }

    #[test]
    fn lowers_lookback_last_turn() {
        assert_lowers(deckmaste_authoring::Lookback::LastTurn);
    }

    #[test]
    fn lowers_lookback_this_combat() {
        assert_lowers(deckmaste_authoring::Lookback::ThisCombat);
    }

    #[test]
    fn lowers_lookback_this_step() {
        assert_lowers(deckmaste_authoring::Lookback::ThisStep);
    }

    #[test]
    fn lowers_lookback_since_your() {
        assert_lowers(deckmaste_authoring::Lookback::SinceYour(
            minimal_phase_step(),
        ));
    }

    #[test]
    fn lowers_turn_marker_end_of_turn() {
        assert_lowers(deckmaste_authoring::TurnMarker::EndOfTurn);
    }

    #[test]
    fn lowers_turn_marker_end_of_combat() {
        assert_lowers(deckmaste_authoring::TurnMarker::EndOfCombat);
    }

    #[test]
    fn lowers_turn_marker_your_next_turn() {
        assert_lowers(deckmaste_authoring::TurnMarker::YourNextTurn);
    }

    #[test]
    fn lowers_lock_point_announce() {
        assert_lowers(deckmaste_authoring::LockPoint::Announce);
    }

    #[test]
    fn lowers_lock_point_stack_placement() {
        assert_lowers(deckmaste_authoring::LockPoint::StackPlacement);
    }

    #[test]
    fn lowers_lock_point_total_cost() {
        assert_lowers(deckmaste_authoring::LockPoint::TotalCost);
    }

    #[test]
    fn lowers_lock_point_payment() {
        assert_lowers(deckmaste_authoring::LockPoint::Payment);
    }

    #[test]
    fn lowers_lock_point_effect_begin() {
        assert_lowers(deckmaste_authoring::LockPoint::EffectBegin);
    }

    #[test]
    fn lowers_lock_point_copy_creation() {
        assert_lowers(deckmaste_authoring::LockPoint::CopyCreation);
    }

    #[test]
    fn lowers_lock_point_declaration() {
        assert_lowers(deckmaste_authoring::LockPoint::Declaration);
    }

    #[test]
    fn lowers_lock_point_pre_game() {
        assert_lowers(deckmaste_authoring::LockPoint::PreGame);
    }

    #[test]
    fn lowers_lock_point_resolution() {
        assert_lowers(deckmaste_authoring::LockPoint::Resolution);
    }

    #[test]
    fn lowers_lock_point_never() {
        assert_lowers(deckmaste_authoring::LockPoint::Never);
    }
}
