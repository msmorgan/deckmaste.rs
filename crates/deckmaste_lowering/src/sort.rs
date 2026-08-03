//! `sort` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Sort {
    type Target = deckmaste_core::Sort;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Player => deckmaste_core::Sort::Player,
            Self::Card => deckmaste_core::Sort::Card,
            Self::Token => deckmaste_core::Sort::Token,
            Self::Spell => deckmaste_core::Sort::Spell,
            Self::StackObject => deckmaste_core::Sort::StackObject,
            Self::Permanent => deckmaste_core::Sort::Permanent,
            Self::OfType(f0) => deckmaste_core::Sort::OfType(f0.lower()),
            Self::Amount => deckmaste_core::Sort::Amount,
            Self::Pile => deckmaste_core::Sort::Pile,
        }
    }
}
