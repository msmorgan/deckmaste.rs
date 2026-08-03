//! `sba_rule` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::SbaRule {
    type Target = deckmaste_core::SbaRule;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::SbaRule {
            scope: self.scope.lower(),
            when: self.when.lower(),
            then: self.then.lower(),
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
    fn lowers_sba_rule() {
        assert_lowers(deckmaste_authoring::SbaRule {
            scope: minimal_predicate(),
            when: minimal_condition(),
            then: minimal_one_shot_effect(),
        });
    }
}
