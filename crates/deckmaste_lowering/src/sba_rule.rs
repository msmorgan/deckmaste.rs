//! `sba_rule` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::SbaRule {
    type Target = deckmaste_core::SbaRule;
    fn lower(self) -> <Self as Lower>::Target {
        let (params, body) = crate::region::in_region(crate::region::RegionKind::Static, 0, || {
            deckmaste_core::SbaBody {
                scope: self.scope.lower(),
                when: self.when.lower(),
                then: self.then.lower(),
            }
        });
        deckmaste_core::SbaRule {
            region: deckmaste_core::Region::new(params, body),
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

    /// A rules-defined SBA lowers into a closed static region: its scope,
    /// condition, and instruction body all read registers the region declares.
    /// Re-spelled from `lowers_sba_rule` — `SbaRule`'s three fields moved
    /// inside `Region<SbaBody>`.
    #[test]
    fn lowers_sba_rule() {
        let lowered = deckmaste_semantics::SbaRule {
            scope: minimal_predicate(),
            when: minimal_condition(),
            then: minimal_one_shot_effect(),
        }
        .lower();
        assert_matches!(
            lowered.region.body.scope,
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
        );
        assert_matches!(
            lowered.region.body.when,
            deckmaste_core::Condition::Compare(
                deckmaste_core::Count::Literal(0),
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::Literal(0)
            )
        );
        assert!(is_minimal_lowered_effect(&lowered.region.body.then));
        assert_eq!(
            deckmaste_core::validate_sba(&lowered.region),
            Ok(()),
            "the lowered SBA region satisfies the region ABI"
        );
    }
}
