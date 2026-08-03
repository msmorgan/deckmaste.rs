//! `conferral_rule` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::ConferralRule {
    type Target = deckmaste_core::ConferralRule;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ConferralRule {
            scope: self.scope.lower(),
            confer: self.confer.lower(),
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
    fn lowers_conferral_rule() {
        assert_lowers(deckmaste_authoring::ConferralRule {
            scope: minimal_predicate(),
            confer: minimal_property(),
        });
    }
}
