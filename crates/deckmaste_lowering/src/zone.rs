//! `zone` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Zone {
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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_zone_battlefield() {
        assert_lowers_debug(deckmaste_authoring::Zone::Battlefield);
        assert_matches!(
            deckmaste_authoring::Zone::Battlefield.lower(),
            deckmaste_core::Zone::Battlefield
        );
    }

    #[test]
    fn lowers_zone_command() {
        assert_lowers_debug(deckmaste_authoring::Zone::Command);
        assert_matches!(
            deckmaste_authoring::Zone::Command.lower(),
            deckmaste_core::Zone::Command
        );
    }

    #[test]
    fn lowers_zone_exile() {
        assert_lowers_debug(deckmaste_authoring::Zone::Exile);
        assert_matches!(
            deckmaste_authoring::Zone::Exile.lower(),
            deckmaste_core::Zone::Exile
        );
    }

    #[test]
    fn lowers_zone_graveyard() {
        assert_lowers_debug(deckmaste_authoring::Zone::Graveyard);
        assert_matches!(
            deckmaste_authoring::Zone::Graveyard.lower(),
            deckmaste_core::Zone::Graveyard
        );
    }

    #[test]
    fn lowers_zone_hand() {
        assert_lowers_debug(deckmaste_authoring::Zone::Hand);
        assert_matches!(
            deckmaste_authoring::Zone::Hand.lower(),
            deckmaste_core::Zone::Hand
        );
    }

    #[test]
    fn lowers_zone_library() {
        assert_lowers_debug(deckmaste_authoring::Zone::Library);
        assert_matches!(
            deckmaste_authoring::Zone::Library.lower(),
            deckmaste_core::Zone::Library
        );
    }

    #[test]
    fn lowers_zone_stack() {
        assert_lowers_debug(deckmaste_authoring::Zone::Stack);
        assert_matches!(
            deckmaste_authoring::Zone::Stack.lower(),
            deckmaste_core::Zone::Stack
        );
    }
}
