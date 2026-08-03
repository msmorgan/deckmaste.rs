//! `damage_result_rule` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::DamageResultRule {
    type Target = deckmaste_core::DamageResultRule;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::DamageResultRule {
            recipient: self.recipient.lower(),
            remove: self.remove.lower(),
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
    fn lowers_damage_result_rule() {
        assert_matches!(
            deckmaste_authoring::DamageResultRule {
                recipient: minimal_predicate(),
                remove: minimal_counter_ref()
            }
            .lower(),
            deckmaste_core::DamageResultRule {
                recipient: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                remove: deckmaste_core::CounterRef(_)
            }
        );
    }
}
