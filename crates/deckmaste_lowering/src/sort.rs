//! `sort` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Sort {
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

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_sort_player() {
        assert_matches!(
            deckmaste_semantics::Sort::Player.lower(),
            deckmaste_core::Sort::Player
        );
    }

    #[test]
    fn lowers_sort_card() {
        assert_matches!(
            deckmaste_semantics::Sort::Card.lower(),
            deckmaste_core::Sort::Card
        );
    }

    #[test]
    fn lowers_sort_token() {
        assert_matches!(
            deckmaste_semantics::Sort::Token.lower(),
            deckmaste_core::Sort::Token
        );
    }

    #[test]
    fn lowers_sort_spell() {
        assert_matches!(
            deckmaste_semantics::Sort::Spell.lower(),
            deckmaste_core::Sort::Spell
        );
    }

    #[test]
    fn lowers_sort_stack_object() {
        assert_matches!(
            deckmaste_semantics::Sort::StackObject.lower(),
            deckmaste_core::Sort::StackObject
        );
    }

    #[test]
    fn lowers_sort_permanent() {
        assert_matches!(
            deckmaste_semantics::Sort::Permanent.lower(),
            deckmaste_core::Sort::Permanent
        );
    }

    #[test]
    fn lowers_sort_of_type() {
        assert_matches!(
            deckmaste_semantics::Sort::OfType(minimal_type()).lower(),
            deckmaste_core::Sort::OfType(deckmaste_core::Type::Artifact)
        );
    }

    #[test]
    fn lowers_sort_amount() {
        assert_matches!(
            deckmaste_semantics::Sort::Amount.lower(),
            deckmaste_core::Sort::Amount
        );
    }

    #[test]
    fn lowers_sort_pile() {
        assert_matches!(
            deckmaste_semantics::Sort::Pile.lower(),
            deckmaste_core::Sort::Pile
        );
    }
}
