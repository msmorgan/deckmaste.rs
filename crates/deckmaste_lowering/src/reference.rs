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
