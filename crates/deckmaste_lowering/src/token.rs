//! `token` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::TokenSpec {
    type Target = deckmaste_core::TokenSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Token(f0) => deckmaste_core::TokenSpec::Token(f0.lower()),
            Self::Named(f0) => deckmaste_core::TokenSpec::Named(f0.lower()),
            Self::Copy(f0) => deckmaste_core::TokenSpec::Copy(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::TokenName {
    type Target = deckmaste_core::TokenName;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TokenName(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::PredefinedToken {
    type Target = deckmaste_core::PredefinedToken;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Treasure => deckmaste_core::PredefinedToken::Treasure,
            Self::Food => deckmaste_core::PredefinedToken::Food,
            Self::Gold => deckmaste_core::PredefinedToken::Gold,
            Self::Clue => deckmaste_core::PredefinedToken::Clue,
            Self::Blood => deckmaste_core::PredefinedToken::Blood,
            Self::Vibranium => deckmaste_core::PredefinedToken::Vibranium,
        }
    }
}

impl Lower for deckmaste_authoring::Token {
    type Target = deckmaste_core::Token;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Token {
            name: self.name.lower(),
            color_indicator: self.color_indicator.lower(),
            supertypes: self.supertypes.lower(),
            types: self.types.lower(),
            subtypes: self.subtypes.lower(),
            abilities: self.abilities.lower(),
            power: self.power.lower(),
            toughness: self.toughness.lower(),
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
    fn lowers_token_spec_token() {
        assert_matches!(
            deckmaste_authoring::TokenSpec::Token(std::sync::Arc::new(minimal_token())).lower(),
            deckmaste_core::TokenSpec::Token(_)
        );
    }

    #[test]
    fn lowers_token_spec_named() {
        assert_matches!(
            deckmaste_authoring::TokenSpec::Named(minimal_token_name()).lower(),
            deckmaste_core::TokenSpec::Named(deckmaste_core::TokenName(_))
        );
    }

    #[test]
    fn lowers_token_spec_copy() {
        assert_matches!(
            deckmaste_authoring::TokenSpec::Copy(std::sync::Arc::new(minimal_copy_spec())).lower(),
            deckmaste_core::TokenSpec::Copy(_)
        );
    }

    #[test]
    fn lowers_token_name() {
        assert_matches!(
            deckmaste_authoring::TokenName("X".into()).lower(),
            deckmaste_core::TokenName(_)
        );
    }

    #[test]
    fn lowers_predefined_token_treasure() {
        assert_matches!(
            deckmaste_authoring::PredefinedToken::Treasure.lower(),
            deckmaste_core::PredefinedToken::Treasure
        );
    }

    #[test]
    fn lowers_predefined_token_food() {
        assert_matches!(
            deckmaste_authoring::PredefinedToken::Food.lower(),
            deckmaste_core::PredefinedToken::Food
        );
    }

    #[test]
    fn lowers_predefined_token_gold() {
        assert_matches!(
            deckmaste_authoring::PredefinedToken::Gold.lower(),
            deckmaste_core::PredefinedToken::Gold
        );
    }

    #[test]
    fn lowers_predefined_token_clue() {
        assert_matches!(
            deckmaste_authoring::PredefinedToken::Clue.lower(),
            deckmaste_core::PredefinedToken::Clue
        );
    }

    #[test]
    fn lowers_predefined_token_blood() {
        assert_matches!(
            deckmaste_authoring::PredefinedToken::Blood.lower(),
            deckmaste_core::PredefinedToken::Blood
        );
    }

    #[test]
    fn lowers_predefined_token_vibranium() {
        assert_matches!(
            deckmaste_authoring::PredefinedToken::Vibranium.lower(),
            deckmaste_core::PredefinedToken::Vibranium
        );
    }

    #[test]
    fn lowers_token() {
        assert_matches!(
            deckmaste_authoring::Token {
                name: None,
                color_indicator: [].into(),
                supertypes: [].into(),
                types: [].into(),
                subtypes: [].into(),
                abilities: [].into(),
                power: None,
                toughness: None
            }
            .lower(),
            deckmaste_core::Token {
                name: None,
                color_indicator: _,
                supertypes: _,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None
            }
        );
    }
}
