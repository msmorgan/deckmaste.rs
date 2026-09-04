//! `counter` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::CounterRef {
    type Target = deckmaste_core::CounterRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CounterRef(self.0.lower())
    }
}

impl Lower for deckmaste_semantics::CounterSpec {
    type Target = deckmaste_core::CounterSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Named(f0, f1) => deckmaste_core::CounterSpec::Named(f0.lower(), f1.lower()),
            Self::AllKinds => deckmaste_core::CounterSpec::AllKinds,
        }
    }
}

impl Lower for deckmaste_semantics::CounterScope {
    type Target = deckmaste_core::CounterScope;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object => deckmaste_core::CounterScope::Object,
            Self::Player => deckmaste_core::CounterScope::Player,
        }
    }
}

impl Lower for deckmaste_semantics::Counter {
    type Target = deckmaste_core::Counter;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Counter {
            name: self.name.lower(),
            scope: self.scope.lower(),
            confers: self.confers.lower(),
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
    fn lowers_counter_ref() {
        assert_matches!(
            deckmaste_semantics::CounterRef("X".into()).lower(),
            deckmaste_core::CounterRef(_)
        );
    }

    #[test]
    fn lowers_counter_spec_named() {
        assert_matches!(
            deckmaste_semantics::CounterSpec::Named(minimal_counter_ref(), minimal_count()).lower(),
            deckmaste_core::CounterSpec::Named(
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_counter_spec_all_kinds() {
        assert_matches!(
            deckmaste_semantics::CounterSpec::AllKinds.lower(),
            deckmaste_core::CounterSpec::AllKinds
        );
    }

    #[test]
    fn lowers_counter_scope_object() {
        assert_matches!(
            deckmaste_semantics::CounterScope::Object.lower(),
            deckmaste_core::CounterScope::Object
        );
    }

    #[test]
    fn lowers_counter_scope_player() {
        assert_matches!(
            deckmaste_semantics::CounterScope::Player.lower(),
            deckmaste_core::CounterScope::Player
        );
    }

    #[test]
    fn lowers_counter() {
        assert_matches!(
            deckmaste_semantics::Counter {
                name: "X".into(),
                scope: minimal_counter_scope(),
                confers: Vec::new()
            }
            .lower(),
            deckmaste_core::Counter {
                name: _,
                scope: deckmaste_core::CounterScope::Object,
                confers: _
            }
        );
    }
}
