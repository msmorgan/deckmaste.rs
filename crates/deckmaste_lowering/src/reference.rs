//! `reference` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Reference {
    type Target = deckmaste_core::Reference;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::This => deckmaste_core::Reference::This,
            Self::Single(f0) => deckmaste_core::Reference::Single(f0.lower()),
            Self::You => deckmaste_core::Reference::You,
            Self::Opponent => deckmaste_core::Reference::Opponent,
            Self::It => deckmaste_core::Reference::It,
            Self::Target(f0) => deckmaste_core::Reference::Target(f0.lower()),
            Self::EventObject => deckmaste_core::Reference::EventObject,
            Self::EventPatient => deckmaste_core::Reference::EventPatient,
            Self::EventActor => deckmaste_core::Reference::EventActor,
            Self::DefendingPlayer => deckmaste_core::Reference::DefendingPlayer,
            Self::That(f0) => deckmaste_core::Reference::That(f0.lower()),
            Self::Bound(f0) => deckmaste_core::Reference::Bound(f0.lower()),
            Self::Linked(f0) => deckmaste_core::Reference::Linked(f0.lower()),
            Self::ControllerOf(f0) => deckmaste_core::Reference::ControllerOf(f0.lower()),
            Self::Coalesce(f0) => deckmaste_core::Reference::Coalesce(f0.lower()),
            Self::OwnerOf(f0) => deckmaste_core::Reference::OwnerOf(f0.lower()),
            Self::AttachHostOf(f0) => deckmaste_core::Reference::AttachHostOf(f0.lower()),
            Self::Source => deckmaste_core::Reference::Source,
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
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
    fn lowers_reference_this() {
        assert_matches!(
            deckmaste_semantics::Reference::This.lower(),
            deckmaste_core::Reference::This
        );
    }

    #[test]
    fn lowers_reference_single() {
        assert_matches!(
            deckmaste_semantics::Reference::Single(std::sync::Arc::new(minimal_selection()))
                .lower(),
            deckmaste_core::Reference::Single(_)
        );
    }

    #[test]
    fn lowers_reference_you() {
        assert_matches!(
            deckmaste_semantics::Reference::You.lower(),
            deckmaste_core::Reference::You
        );
    }

    #[test]
    fn lowers_reference_opponent() {
        assert_matches!(
            deckmaste_semantics::Reference::Opponent.lower(),
            deckmaste_core::Reference::Opponent
        );
    }

    #[test]
    fn lowers_reference_it() {
        assert_matches!(
            deckmaste_semantics::Reference::It.lower(),
            deckmaste_core::Reference::It
        );
    }

    #[test]
    fn lowers_reference_target() {
        assert_matches!(
            deckmaste_semantics::Reference::Target(0).lower(),
            deckmaste_core::Reference::Target(0)
        );
    }

    #[test]
    fn lowers_reference_event_object() {
        assert_matches!(
            deckmaste_semantics::Reference::EventObject.lower(),
            deckmaste_core::Reference::EventObject
        );
    }

    #[test]
    fn lowers_reference_event_patient() {
        assert_matches!(
            deckmaste_semantics::Reference::EventPatient.lower(),
            deckmaste_core::Reference::EventPatient
        );
    }

    #[test]
    fn lowers_reference_event_actor() {
        assert_matches!(
            deckmaste_semantics::Reference::EventActor.lower(),
            deckmaste_core::Reference::EventActor
        );
    }

    #[test]
    fn lowers_reference_defending_player() {
        assert_matches!(
            deckmaste_semantics::Reference::DefendingPlayer.lower(),
            deckmaste_core::Reference::DefendingPlayer
        );
    }

    #[test]
    fn lowers_reference_that() {
        assert_matches!(
            deckmaste_semantics::Reference::That(minimal_sort()).lower(),
            deckmaste_core::Reference::That(deckmaste_core::Sort::Player)
        );
    }

    #[test]
    fn lowers_reference_bound() {
        assert_matches!(
            deckmaste_semantics::Reference::Bound("X".into()).lower(),
            deckmaste_core::Reference::Bound(_)
        );
    }

    #[test]
    fn lowers_reference_linked() {
        assert_matches!(
            deckmaste_semantics::Reference::Linked("X".into()).lower(),
            deckmaste_core::Reference::Linked(_)
        );
    }

    #[test]
    fn lowers_reference_controller_of() {
        assert_matches!(
            deckmaste_semantics::Reference::ControllerOf(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Reference::ControllerOf(_)
        );
    }

    #[test]
    fn lowers_reference_coalesce() {
        assert_matches!(
            deckmaste_semantics::Reference::Coalesce([].into()).lower(),
            deckmaste_core::Reference::Coalesce(_)
        );
    }

    #[test]
    fn lowers_reference_owner_of() {
        assert_matches!(
            deckmaste_semantics::Reference::OwnerOf(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Reference::OwnerOf(_)
        );
    }

    #[test]
    fn lowers_reference_attach_host_of() {
        assert_matches!(
            deckmaste_semantics::Reference::AttachHostOf(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Reference::AttachHostOf(_)
        );
    }

    #[test]
    fn lowers_reference_source() {
        assert_matches!(
            deckmaste_semantics::Reference::Source.lower(),
            deckmaste_core::Reference::Source
        );
    }

    #[test]
    fn lowers_reference_expanded() {
        assert_matches!(
            deckmaste_semantics::Reference::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_reference())
            })
            .lower(),
            deckmaste_core::Reference::This
        );
    }
}
