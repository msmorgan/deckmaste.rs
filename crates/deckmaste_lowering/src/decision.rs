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

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_decider_spec_controller() {
        assert_lowers(deckmaste_authoring::DeciderSpec::Controller);
        assert_matches!(
            deckmaste_authoring::DeciderSpec::Controller.lower(),
            deckmaste_core::DeciderSpec::Controller
        );
    }

    #[test]
    fn lowers_decider_spec_active_player() {
        assert_lowers(deckmaste_authoring::DeciderSpec::ActivePlayer);
        assert_matches!(
            deckmaste_authoring::DeciderSpec::ActivePlayer.lower(),
            deckmaste_core::DeciderSpec::ActivePlayer
        );
    }

    #[test]
    fn lowers_decider_spec_defending_player() {
        assert_lowers(deckmaste_authoring::DeciderSpec::DefendingPlayer);
        assert_matches!(
            deckmaste_authoring::DeciderSpec::DefendingPlayer.lower(),
            deckmaste_core::DeciderSpec::DefendingPlayer
        );
    }

    #[test]
    fn lowers_decider_spec_named() {
        assert_lowers(deckmaste_authoring::DeciderSpec::Named(minimal_reference()));
        assert_matches!(
            deckmaste_authoring::DeciderSpec::Named(minimal_reference()).lower(),
            deckmaste_core::DeciderSpec::Named(..)
        );
    }

    #[test]
    fn lowers_decider_spec_each_in_turn_order() {
        assert_lowers(deckmaste_authoring::DeciderSpec::EachInTurnOrder);
        assert_matches!(
            deckmaste_authoring::DeciderSpec::EachInTurnOrder.lower(),
            deckmaste_core::DeciderSpec::EachInTurnOrder
        );
    }

    #[test]
    fn lowers_decider_spec_priority_holder() {
        assert_lowers(deckmaste_authoring::DeciderSpec::PriorityHolder);
        assert_matches!(
            deckmaste_authoring::DeciderSpec::PriorityHolder.lower(),
            deckmaste_core::DeciderSpec::PriorityHolder
        );
    }

    #[test]
    fn lowers_decider_spec_rng() {
        assert_lowers(deckmaste_authoring::DeciderSpec::Rng);
        assert_matches!(
            deckmaste_authoring::DeciderSpec::Rng.lower(),
            deckmaste_core::DeciderSpec::Rng
        );
    }

    #[test]
    fn lowers_visibility_open() {
        assert_lowers(deckmaste_authoring::Visibility::Open);
        assert_matches!(
            deckmaste_authoring::Visibility::Open.lower(),
            deckmaste_core::Visibility::Open
        );
    }

    #[test]
    fn lowers_visibility_committed_hidden() {
        assert_lowers(deckmaste_authoring::Visibility::CommittedHidden);
        assert_matches!(
            deckmaste_authoring::Visibility::CommittedHidden.lower(),
            deckmaste_core::Visibility::CommittedHidden
        );
    }

    #[test]
    fn lowers_chosen_value_kind_color() {
        assert_lowers(deckmaste_authoring::ChosenValueKind::Color);
        assert_matches!(
            deckmaste_authoring::ChosenValueKind::Color.lower(),
            deckmaste_core::ChosenValueKind::Color
        );
    }

    #[test]
    fn lowers_chosen_value_kind_card_name() {
        assert_lowers(deckmaste_authoring::ChosenValueKind::CardName);
        assert_matches!(
            deckmaste_authoring::ChosenValueKind::CardName.lower(),
            deckmaste_core::ChosenValueKind::CardName
        );
    }

    #[test]
    fn lowers_chosen_value_kind_number() {
        assert_lowers(deckmaste_authoring::ChosenValueKind::Number);
        assert_matches!(
            deckmaste_authoring::ChosenValueKind::Number.lower(),
            deckmaste_core::ChosenValueKind::Number
        );
    }

    #[test]
    fn lowers_noted_kind_objects() {
        assert_lowers(deckmaste_authoring::NotedKind::Objects);
        assert_matches!(
            deckmaste_authoring::NotedKind::Objects.lower(),
            deckmaste_core::NotedKind::Objects
        );
    }

    #[test]
    fn lowers_noted_kind_piles() {
        assert_lowers(deckmaste_authoring::NotedKind::Piles);
        assert_matches!(
            deckmaste_authoring::NotedKind::Piles.lower(),
            deckmaste_core::NotedKind::Piles
        );
    }
}
