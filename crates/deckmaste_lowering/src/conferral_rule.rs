//! `conferral_rule` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

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
    use crate::minimal::*;

    #[test]
    fn lowers_conferral_rule() {
        assert_matches!(
            deckmaste_authoring::ConferralRule {
                scope: minimal_predicate(),
                confer: minimal_property()
            }
            .lower(),
            deckmaste_core::ConferralRule {
                scope: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                confer: deckmaste_core::Property::Ability(_)
            }
        );
    }
}
