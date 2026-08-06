//! `decision` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::DeciderSpec {
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

impl Lower for deckmaste_semantics::Visibility {
    type Target = deckmaste_core::Visibility;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Open => deckmaste_core::Visibility::Open,
            Self::CommittedHidden => deckmaste_core::Visibility::CommittedHidden,
        }
    }
}

impl Lower for deckmaste_semantics::ChosenValueKind {
    type Target = deckmaste_core::ChosenValueKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Color => deckmaste_core::ChosenValueKind::Color,
            Self::CardName => deckmaste_core::ChosenValueKind::CardName,
            Self::Number => deckmaste_core::ChosenValueKind::Number,
        }
    }
}

impl Lower for deckmaste_semantics::NotedKind {
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
    use crate::minimal::*;

    #[test]
    fn lowers_decider_spec_controller() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::Controller.lower(),
            deckmaste_core::DeciderSpec::Controller
        );
    }

    #[test]
    fn lowers_decider_spec_active_player() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::ActivePlayer.lower(),
            deckmaste_core::DeciderSpec::ActivePlayer
        );
    }

    #[test]
    fn lowers_decider_spec_defending_player() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::DefendingPlayer.lower(),
            deckmaste_core::DeciderSpec::DefendingPlayer
        );
    }

    #[test]
    fn lowers_decider_spec_named() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::Named(minimal_reference()).lower(),
            deckmaste_core::DeciderSpec::Named(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_decider_spec_each_in_turn_order() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::EachInTurnOrder.lower(),
            deckmaste_core::DeciderSpec::EachInTurnOrder
        );
    }

    #[test]
    fn lowers_decider_spec_priority_holder() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::PriorityHolder.lower(),
            deckmaste_core::DeciderSpec::PriorityHolder
        );
    }

    #[test]
    fn lowers_decider_spec_rng() {
        assert_matches!(
            deckmaste_semantics::DeciderSpec::Rng.lower(),
            deckmaste_core::DeciderSpec::Rng
        );
    }

    #[test]
    fn lowers_visibility_open() {
        assert_matches!(
            deckmaste_semantics::Visibility::Open.lower(),
            deckmaste_core::Visibility::Open
        );
    }

    #[test]
    fn lowers_visibility_committed_hidden() {
        assert_matches!(
            deckmaste_semantics::Visibility::CommittedHidden.lower(),
            deckmaste_core::Visibility::CommittedHidden
        );
    }

    #[test]
    fn lowers_chosen_value_kind_color() {
        assert_matches!(
            deckmaste_semantics::ChosenValueKind::Color.lower(),
            deckmaste_core::ChosenValueKind::Color
        );
    }

    #[test]
    fn lowers_chosen_value_kind_card_name() {
        assert_matches!(
            deckmaste_semantics::ChosenValueKind::CardName.lower(),
            deckmaste_core::ChosenValueKind::CardName
        );
    }

    #[test]
    fn lowers_chosen_value_kind_number() {
        assert_matches!(
            deckmaste_semantics::ChosenValueKind::Number.lower(),
            deckmaste_core::ChosenValueKind::Number
        );
    }

    #[test]
    fn lowers_noted_kind_objects() {
        assert_matches!(
            deckmaste_semantics::NotedKind::Objects.lower(),
            deckmaste_core::NotedKind::Objects
        );
    }

    #[test]
    fn lowers_noted_kind_piles() {
        assert_matches!(
            deckmaste_semantics::NotedKind::Piles.lower(),
            deckmaste_core::NotedKind::Piles
        );
    }
}
