//! `counter` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::CounterRef {
    type Target = deckmaste_core::CounterRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CounterRef(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::CounterSpec {
    type Target = deckmaste_core::CounterSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Named(f0, f1) => deckmaste_core::CounterSpec::Named(f0.lower(), f1.lower()),
            Self::AllKinds => deckmaste_core::CounterSpec::AllKinds,
        }
    }
}

impl Lower for deckmaste_authoring::CounterScope {
    type Target = deckmaste_core::CounterScope;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object => deckmaste_core::CounterScope::Object,
            Self::Player => deckmaste_core::CounterScope::Player,
        }
    }
}

impl Lower for deckmaste_authoring::Counter {
    type Target = deckmaste_core::Counter;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Counter {
            name: self.name.lower(),
            scope: self.scope.lower(),
            confers: self.confers.lower(),
        }
    }
}
