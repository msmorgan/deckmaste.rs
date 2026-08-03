//! `decision` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::DeciderSpec {
    type Target = deckmaste_core::DeciderSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Controller => deckmaste_core::DeciderSpec::Controller,
            Self::ActivePlayer => deckmaste_core::DeciderSpec::ActivePlayer,
            Self::DefendingPlayer => deckmaste_core::DeciderSpec::DefendingPlayer,
            Self::Named(f0) => deckmaste_core::DeciderSpec::Named(f0.lower()),
            Self::EachInTurnOrder => deckmaste_core::DeciderSpec::EachInTurnOrder,
            Self::PriorityHolder => deckmaste_core::DeciderSpec::PriorityHolder,
            Self::Rng => deckmaste_core::DeciderSpec::Rng,
        }
    }
}

impl Lower for deckmaste_authoring::Visibility {
    type Target = deckmaste_core::Visibility;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Open => deckmaste_core::Visibility::Open,
            Self::CommittedHidden => deckmaste_core::Visibility::CommittedHidden,
        }
    }
}

impl Lower for deckmaste_authoring::ChosenValueKind {
    type Target = deckmaste_core::ChosenValueKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Color => deckmaste_core::ChosenValueKind::Color,
            Self::CardName => deckmaste_core::ChosenValueKind::CardName,
            Self::Number => deckmaste_core::ChosenValueKind::Number,
        }
    }
}

impl Lower for deckmaste_authoring::NotedKind {
    type Target = deckmaste_core::NotedKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Objects => deckmaste_core::NotedKind::Objects,
            Self::Piles => deckmaste_core::NotedKind::Piles,
        }
    }
}
