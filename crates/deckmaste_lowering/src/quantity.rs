//! `quantity` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Quantity {
    type Target = deckmaste_core::Quantity;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Range(f0, f1) => deckmaste_core::Quantity::Range(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::Quantity::Expanded(f0.lower()),
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
    fn lowers_quantity_range() {
        assert_lowers_debug(deckmaste_authoring::Quantity::Range(None, None));
        assert_matches!(
            deckmaste_authoring::Quantity::Range(None, None).lower(),
            deckmaste_core::Quantity::Range(..)
        );
    }

    #[test]
    fn lowers_quantity_expanded() {
        assert_lowers_debug(deckmaste_authoring::Quantity::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_quantity()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::Quantity::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_quantity())
            })
            .lower(),
            deckmaste_core::Quantity::Expanded(..)
        );
    }
}
