//! `binder` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Binder {
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
    fn lowers_binder_the_ref() {
        assert_matches!(
            deckmaste_semantics::Binder::TheRef(minimal_reference()).lower(),
            deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_binder_choose_one() {
        assert_matches!(
            deckmaste_semantics::Binder::ChooseOne {
                filter: minimal_predicate(),
                by: minimal_reference()
            }
            .lower(),
            deckmaste_core::Binder::ChooseOne {
                filter: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Reference::This
            }
        );
    }

    #[test]
    fn lowers_binder_produce() {
        assert_matches!(
            deckmaste_semantics::Binder::Produce(std::sync::Arc::new(minimal_action())).lower(),
            deckmaste_core::Binder::Produce(_)
        );
    }

    #[test]
    fn lowers_binder_search_one() {
        assert_matches!(
            deckmaste_semantics::Binder::SearchOne {
                filter: minimal_predicate(),
                by: minimal_reference(),
                whose: minimal_reference(),
                from: [].into(),
                if_none: None
            }
            .lower(),
            deckmaste_core::Binder::SearchOne {
                filter: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Reference::This,
                whose: deckmaste_core::Reference::This,
                from: _,
                if_none: None
            }
        );
    }

    #[test]
    fn lowers_binder_choose() {
        assert_matches!(
            deckmaste_semantics::Binder::Choose {
                quantity: minimal_quantity(),
                filter: minimal_predicate(),
                by: minimal_reference()
            }
            .lower(),
            deckmaste_core::Binder::Choose {
                quantity: deckmaste_core::Quantity::Range(None, None),
                filter: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Reference::This
            }
        );
    }

    #[test]
    fn lowers_binder_existing() {
        assert_matches!(
            deckmaste_semantics::Binder::Existing(minimal_selection()).lower(),
            deckmaste_core::Binder::Existing(deckmaste_core::Selection::SelectAll(
                deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            ))
        );
    }

    #[test]
    fn lowers_binder_search() {
        assert_matches!(
            deckmaste_semantics::Binder::Search {
                quantity: minimal_quantity(),
                filter: minimal_predicate(),
                by: minimal_reference(),
                whose: minimal_reference(),
                from: [].into(),
                if_none: None
            }
            .lower(),
            deckmaste_core::Binder::Search {
                quantity: deckmaste_core::Quantity::Range(None, None),
                filter: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Reference::This,
                whose: deckmaste_core::Reference::This,
                from: _,
                if_none: None
            }
        );
    }

    #[test]
    fn lowers_binder_expanded() {
        assert_matches!(
            deckmaste_semantics::Binder::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_binder())
            })
            .lower(),
            deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This)
        );
    }
}
