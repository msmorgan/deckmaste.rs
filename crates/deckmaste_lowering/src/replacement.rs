//! `replacement` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Replacement {
    type Target = deckmaste_core::Replacement;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Instead { would, instead } => deckmaste_core::Replacement::Instead {
                would: would.lower(),
                instead: instead.lower(),
            },
            Self::Skip { what } => deckmaste_core::Replacement::Skip { what: what.lower() },
            Self::Also { would, also } => deckmaste_core::Replacement::Also {
                would: would.lower(),
                also: also.lower(),
            },
            Self::Expanded(f0) => deckmaste_core::Replacement::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Prevention {
    type Target = deckmaste_core::Prevention;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::PreventNext {
                n,
                from,
                to,
                duration,
            } => deckmaste_core::Prevention::PreventNext {
                n: n.lower(),
                from: from.lower(),
                to: to.lower(),
                duration: duration.lower(),
            },
            Self::PreventNextInstance { from, to } => {
                deckmaste_core::Prevention::PreventNextInstance {
                    from: from.lower(),
                    to: to.lower(),
                }
            }
            Self::PreventAll { from, to, duration } => deckmaste_core::Prevention::PreventAll {
                from: from.lower(),
                to: to.lower(),
                duration: duration.lower(),
            },
        }
    }
}
