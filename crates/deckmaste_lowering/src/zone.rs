//! `zone` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Zone {
    type Target = deckmaste_core::Zone;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Battlefield => deckmaste_core::Zone::Battlefield,
            Self::Command => deckmaste_core::Zone::Command,
            Self::Exile => deckmaste_core::Zone::Exile,
            Self::Graveyard => deckmaste_core::Zone::Graveyard,
            Self::Hand => deckmaste_core::Zone::Hand,
            Self::Library => deckmaste_core::Zone::Library,
            Self::Stack => deckmaste_core::Zone::Stack,
        }
    }
}
