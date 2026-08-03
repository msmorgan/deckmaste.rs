//! `color` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Color {
    type Target = deckmaste_core::Color;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::White => deckmaste_core::Color::White,
            Self::Blue => deckmaste_core::Color::Blue,
            Self::Black => deckmaste_core::Color::Black,
            Self::Red => deckmaste_core::Color::Red,
            Self::Green => deckmaste_core::Color::Green,
        }
    }
}

impl Lower for deckmaste_authoring::ColorOrColorless {
    type Target = deckmaste_core::ColorOrColorless;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Colorless => deckmaste_core::ColorOrColorless::Colorless,
            Self::Color(f0) => deckmaste_core::ColorOrColorless::Color(f0.lower()),
        }
    }
}
