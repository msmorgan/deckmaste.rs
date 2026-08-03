//! `quantity` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::Quantity {
    type Target = deckmaste_core::Quantity;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Range(f0, f1) => deckmaste_core::Quantity::Range(f0.lower(), f1.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
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
    fn lowers_quantity_range() {
        assert_matches!(
            deckmaste_authoring::Quantity::Range(None, None).lower(),
            deckmaste_core::Quantity::Range(None, None)
        );
    }

    #[test]
    fn lowers_quantity_expanded() {
        assert_matches!(
            deckmaste_authoring::Quantity::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_quantity())
            })
            .lower(),
            deckmaste_core::Quantity::Range(None, None)
        );
    }
}
