//! `zone` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Zone {
    type Target = deckmaste_core::Zone;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Battlefield => deckmaste_core::Zone::Battlefield,
            Self::Command => deckmaste_core::Zone::Command,
            Self::Exile => deckmaste_core::Zone::Exile,
            Self::Graveyard => deckmaste_core::Zone::Graveyard,
            Self::Hand => deckmaste_core::Zone::Hand,
            Self::Library => deckmaste_core::Zone::Library,
            Self::Stack => deckmaste_core::Zone::Stack,
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
    fn lowers_zone_battlefield() {
        assert_matches!(
            deckmaste_semantics::Zone::Battlefield.lower(),
            deckmaste_core::Zone::Battlefield
        );
    }

    #[test]
    fn lowers_zone_command() {
        assert_matches!(
            deckmaste_semantics::Zone::Command.lower(),
            deckmaste_core::Zone::Command
        );
    }

    #[test]
    fn lowers_zone_exile() {
        assert_matches!(
            deckmaste_semantics::Zone::Exile.lower(),
            deckmaste_core::Zone::Exile
        );
    }

    #[test]
    fn lowers_zone_graveyard() {
        assert_matches!(
            deckmaste_semantics::Zone::Graveyard.lower(),
            deckmaste_core::Zone::Graveyard
        );
    }

    #[test]
    fn lowers_zone_hand() {
        assert_matches!(
            deckmaste_semantics::Zone::Hand.lower(),
            deckmaste_core::Zone::Hand
        );
    }

    #[test]
    fn lowers_zone_library() {
        assert_matches!(
            deckmaste_semantics::Zone::Library.lower(),
            deckmaste_core::Zone::Library
        );
    }

    #[test]
    fn lowers_zone_stack() {
        assert_matches!(
            deckmaste_semantics::Zone::Stack.lower(),
            deckmaste_core::Zone::Stack
        );
    }
}
