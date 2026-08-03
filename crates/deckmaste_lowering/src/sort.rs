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

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_sort_player() {
        assert_lowers_debug(deckmaste_authoring::Sort::Player);
    }

    #[test]
    fn lowers_sort_card() {
        assert_lowers_debug(deckmaste_authoring::Sort::Card);
    }

    #[test]
    fn lowers_sort_token() {
        assert_lowers_debug(deckmaste_authoring::Sort::Token);
    }

    #[test]
    fn lowers_sort_spell() {
        assert_lowers_debug(deckmaste_authoring::Sort::Spell);
    }

    #[test]
    fn lowers_sort_stack_object() {
        assert_lowers_debug(deckmaste_authoring::Sort::StackObject);
    }

    #[test]
    fn lowers_sort_permanent() {
        assert_lowers_debug(deckmaste_authoring::Sort::Permanent);
    }

    #[test]
    fn lowers_sort_of_type() {
        assert_lowers_debug(deckmaste_authoring::Sort::OfType(minimal_type()));
    }

    #[test]
    fn lowers_sort_amount() {
        assert_lowers_debug(deckmaste_authoring::Sort::Amount);
    }

    #[test]
    fn lowers_sort_pile() {
        assert_lowers_debug(deckmaste_authoring::Sort::Pile);
    }
}
