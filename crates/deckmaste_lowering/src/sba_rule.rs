//! `sba_rule` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::SbaRule {
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

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_sba_rule() {
        assert_matches!(
            deckmaste_semantics::SbaRule {
                scope: minimal_predicate(),
                when: minimal_condition(),
                then: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::SbaRule {
                scope: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                when: deckmaste_core::Condition::Compare(
                    deckmaste_core::Count::X,
                    deckmaste_core::Cmp::Eq,
                    deckmaste_core::Count::X
                ),
                then: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }
}
