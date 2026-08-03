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
