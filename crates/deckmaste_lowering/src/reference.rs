//! `reference` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Reference {
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
            Self::Expanded(f0) => deckmaste_core::Reference::Expanded(f0.lower()),
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
    fn lowers_reference_this() {
        assert_lowers_debug(deckmaste_authoring::Reference::This);
    }

    #[test]
    fn lowers_reference_single() {
        assert_lowers_debug(deckmaste_authoring::Reference::Single(std::sync::Arc::new(
            minimal_selection(),
        )));
    }

    #[test]
    fn lowers_reference_you() {
        assert_lowers_debug(deckmaste_authoring::Reference::You);
    }

    #[test]
    fn lowers_reference_opponent() {
        assert_lowers_debug(deckmaste_authoring::Reference::Opponent);
    }

    #[test]
    fn lowers_reference_it() {
        assert_lowers_debug(deckmaste_authoring::Reference::It);
    }

    #[test]
    fn lowers_reference_target() {
        assert_lowers_debug(deckmaste_authoring::Reference::Target(0));
    }

    #[test]
    fn lowers_reference_event_object() {
        assert_lowers_debug(deckmaste_authoring::Reference::EventObject);
    }

    #[test]
    fn lowers_reference_event_patient() {
        assert_lowers_debug(deckmaste_authoring::Reference::EventPatient);
    }

    #[test]
    fn lowers_reference_event_actor() {
        assert_lowers_debug(deckmaste_authoring::Reference::EventActor);
    }

    #[test]
    fn lowers_reference_defending_player() {
        assert_lowers_debug(deckmaste_authoring::Reference::DefendingPlayer);
    }

    #[test]
    fn lowers_reference_that() {
        assert_lowers_debug(deckmaste_authoring::Reference::That(minimal_sort()));
    }

    #[test]
    fn lowers_reference_bound() {
        assert_lowers_debug(deckmaste_authoring::Reference::Bound("X".into()));
    }

    #[test]
    fn lowers_reference_linked() {
        assert_lowers_debug(deckmaste_authoring::Reference::Linked("X".into()));
    }

    #[test]
    fn lowers_reference_controller_of() {
        assert_lowers_debug(deckmaste_authoring::Reference::ControllerOf(
            std::sync::Arc::new(minimal_reference()),
        ));
    }

    #[test]
    fn lowers_reference_coalesce() {
        assert_lowers_debug(deckmaste_authoring::Reference::Coalesce([].into()));
    }

    #[test]
    fn lowers_reference_owner_of() {
        assert_lowers_debug(deckmaste_authoring::Reference::OwnerOf(
            std::sync::Arc::new(minimal_reference()),
        ));
    }

    #[test]
    fn lowers_reference_attach_host_of() {
        assert_lowers_debug(deckmaste_authoring::Reference::AttachHostOf(
            std::sync::Arc::new(minimal_reference()),
        ));
    }

    #[test]
    fn lowers_reference_source() {
        assert_lowers_debug(deckmaste_authoring::Reference::Source);
    }

    #[test]
    fn lowers_reference_expanded() {
        assert_lowers_debug(deckmaste_authoring::Reference::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_reference()),
            },
        ));
    }
}
