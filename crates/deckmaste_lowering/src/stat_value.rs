//! `stat_value` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::StatValue {
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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_stat_value_defined_by_ability() {
        assert_lowers_debug(deckmaste_authoring::StatValue::DefinedByAbility);
        assert_matches!(
            deckmaste_authoring::StatValue::DefinedByAbility.lower(),
            deckmaste_core::StatValue::DefinedByAbility
        );
    }

    #[test]
    fn lowers_stat_value_variable() {
        assert_lowers_debug(deckmaste_authoring::StatValue::Variable);
        assert_matches!(
            deckmaste_authoring::StatValue::Variable.lower(),
            deckmaste_core::StatValue::Variable
        );
    }

    #[test]
    fn lowers_stat_value_number() {
        assert_lowers_debug(deckmaste_authoring::StatValue::Number(0));
        assert_matches!(
            deckmaste_authoring::StatValue::Number(0).lower(),
            deckmaste_core::StatValue::Number(..)
        );
    }

    #[test]
    fn lowers_stat_value_count() {
        assert_lowers_debug(deckmaste_authoring::StatValue::Count(minimal_count()));
        assert_matches!(
            deckmaste_authoring::StatValue::Count(minimal_count()).lower(),
            deckmaste_core::StatValue::Count(..)
        );
    }
}
