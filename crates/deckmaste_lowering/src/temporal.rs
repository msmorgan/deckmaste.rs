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
