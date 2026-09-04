//! `stat_value` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::StatValue {
    type Target = deckmaste_core::StatValue;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::DefinedByAbility => deckmaste_core::StatValue::DefinedByAbility,
            Self::Variable => deckmaste_core::StatValue::Variable,
            Self::Number(f0) => deckmaste_core::StatValue::Number(f0.lower()),
            Self::Count(f0) => deckmaste_core::StatValue::Count(f0.lower()),
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
    fn lowers_stat_value_defined_by_ability() {
        assert_matches!(
            deckmaste_semantics::StatValue::DefinedByAbility.lower(),
            deckmaste_core::StatValue::DefinedByAbility
        );
    }

    #[test]
    fn lowers_stat_value_variable() {
        assert_matches!(
            deckmaste_semantics::StatValue::Variable.lower(),
            deckmaste_core::StatValue::Variable
        );
    }

    #[test]
    fn lowers_stat_value_number() {
        assert_matches!(
            deckmaste_semantics::StatValue::Number(0).lower(),
            deckmaste_core::StatValue::Number(0)
        );
    }

    #[test]
    fn lowers_stat_value_count() {
        assert_matches!(
            deckmaste_semantics::StatValue::Count(minimal_count()).lower(),
            deckmaste_core::StatValue::Count(deckmaste_core::Count::Literal(0))
        );
    }
}
