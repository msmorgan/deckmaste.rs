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
}
