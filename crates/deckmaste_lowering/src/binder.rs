//! `binder` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Binder {
    type Target = deckmaste_core::Binder;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::TheRef(f0) => deckmaste_core::Binder::TheRef(f0.lower()),
            Self::ChooseOne { filter, by } => deckmaste_core::Binder::ChooseOne {
                filter: filter.lower(),
                by: by.lower(),
            },
            Self::Produce(f0) => deckmaste_core::Binder::Produce(f0.lower()),
            Self::SearchOne {
                filter,
                by,
                whose,
                from,
                if_none,
            } => deckmaste_core::Binder::SearchOne {
                filter: filter.lower(),
                by: by.lower(),
                whose: whose.lower(),
                from: from.lower(),
                if_none: if_none.lower(),
            },
            Self::Choose {
                quantity,
                filter,
                by,
            } => deckmaste_core::Binder::Choose {
                quantity: quantity.lower(),
                filter: filter.lower(),
                by: by.lower(),
            },
            Self::Existing(f0) => deckmaste_core::Binder::Existing(f0.lower()),
            Self::Search {
                quantity,
                filter,
                by,
                whose,
                from,
                if_none,
            } => deckmaste_core::Binder::Search {
                quantity: quantity.lower(),
                filter: filter.lower(),
                by: by.lower(),
                whose: whose.lower(),
                from: from.lower(),
                if_none: if_none.lower(),
            },
            Self::Expanded(f0) => deckmaste_core::Binder::Expanded(f0.lower()),
        }
    }
}
