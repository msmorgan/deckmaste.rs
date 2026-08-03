//! `target_spec` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::TargetSpec {
    type Target = deckmaste_core::TargetSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Target(f0, f1) => deckmaste_core::TargetSpec::Target(f0.lower(), f1.lower()),
            Self::Distinct(f0, f1) => deckmaste_core::TargetSpec::Distinct(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::TargetSpec::Expanded(f0.lower()),
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
            deckmaste_authoring::TargetSpec::Target(minimal_quantity(), minimal_predicate())
                .lower(),
            deckmaste_core::TargetSpec::Target(
                deckmaste_core::Quantity::Range(None, None),
                deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            )
        );
    }

    #[test]
    fn lowers_target_spec_distinct() {
        assert_matches!(
            deckmaste_authoring::TargetSpec::Distinct(
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
            deckmaste_authoring::TargetSpec::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_target_spec())
            })
            .lower(),
            deckmaste_core::TargetSpec::Expanded(_)
        );
    }
}
