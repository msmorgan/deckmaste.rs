//! `sort` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

const fn referent(sort: deckmaste_core::ReferentSort) -> deckmaste_core::Sort {
    deckmaste_core::Sort::Referent(sort)
}

impl Lower for deckmaste_semantics::Sort {
    type Target = deckmaste_core::Sort;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            // The Entity-valued nouns tag into the referent domain; the
            // value and collection nouns are their own domains
            // ([CR#608.2i,700.3b]).
            Self::Player => referent(deckmaste_core::ReferentSort::Player),
            Self::Card => referent(deckmaste_core::ReferentSort::Card),
            Self::Token => referent(deckmaste_core::ReferentSort::Token),
            Self::Spell => referent(deckmaste_core::ReferentSort::Spell),
            Self::StackObject => referent(deckmaste_core::ReferentSort::StackObject),
            Self::Permanent => referent(deckmaste_core::ReferentSort::Permanent),
            Self::OfType(f0) => referent(deckmaste_core::ReferentSort::OfType(f0.lower())),
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
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::Player)
        );
    }

    #[test]
    fn lowers_sort_card() {
        assert_matches!(
            deckmaste_semantics::Sort::Card.lower(),
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::Card)
        );
    }

    #[test]
    fn lowers_sort_token() {
        assert_matches!(
            deckmaste_semantics::Sort::Token.lower(),
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::Token)
        );
    }

    #[test]
    fn lowers_sort_spell() {
        assert_matches!(
            deckmaste_semantics::Sort::Spell.lower(),
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::Spell)
        );
    }

    #[test]
    fn lowers_sort_stack_object() {
        assert_matches!(
            deckmaste_semantics::Sort::StackObject.lower(),
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::StackObject)
        );
    }

    #[test]
    fn lowers_sort_permanent() {
        assert_matches!(
            deckmaste_semantics::Sort::Permanent.lower(),
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::Permanent)
        );
    }

    #[test]
    fn lowers_sort_of_type() {
        assert_matches!(
            deckmaste_semantics::Sort::OfType(minimal_type()).lower(),
            deckmaste_core::Sort::Referent(deckmaste_core::ReferentSort::OfType(
                deckmaste_core::Type::Artifact
            ))
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
