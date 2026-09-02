//! `target_spec` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::TargetSpec {
    type Target = deckmaste_core::TargetSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Target(f0, f1) => deckmaste_core::TargetSpec::Target(
                f0.lower(),
                std::sync::Arc::new(crate::region::candidate_region(|| f1.lower())),
            ),
            Self::Distinct(f0, f1) => deckmaste_core::TargetSpec::Distinct(f0.lower(), f1.lower()),
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
    fn lowers_target_spec_target() {
        assert_matches!(
            deckmaste_semantics::TargetSpec::Target(minimal_quantity(), minimal_predicate())
                .lower(),
            deckmaste_core::TargetSpec::Target(deckmaste_core::Quantity::Range(None, None), _)
        );
    }

    #[test]
    fn lowers_target_spec_distinct() {
        assert_matches!(
            deckmaste_semantics::TargetSpec::Distinct(
                [].into(),
                std::sync::Arc::new(minimal_target_spec())
            )
            .lower(),
            deckmaste_core::TargetSpec::Distinct(_, _)
        );
    }

    #[test]
    fn lowers_target_spec_expanded() {
        assert_matches!(
            deckmaste_semantics::TargetSpec::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_target_spec())
            })
            .lower(),
            deckmaste_core::TargetSpec::Target(deckmaste_core::Quantity::Range(None, None), _)
        );
    }
}
