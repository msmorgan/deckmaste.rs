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
    fn lowers_color_white() {
        assert_lowers_debug(deckmaste_authoring::Color::White);
        assert_matches!(
            deckmaste_authoring::Color::White.lower(),
            deckmaste_core::Color::White
        );
    }

    #[test]
    fn lowers_color_blue() {
        assert_lowers_debug(deckmaste_authoring::Color::Blue);
        assert_matches!(
            deckmaste_authoring::Color::Blue.lower(),
            deckmaste_core::Color::Blue
        );
    }

    #[test]
    fn lowers_color_black() {
        assert_lowers_debug(deckmaste_authoring::Color::Black);
        assert_matches!(
            deckmaste_authoring::Color::Black.lower(),
            deckmaste_core::Color::Black
        );
    }

    #[test]
    fn lowers_color_red() {
        assert_lowers_debug(deckmaste_authoring::Color::Red);
        assert_matches!(
            deckmaste_authoring::Color::Red.lower(),
            deckmaste_core::Color::Red
        );
    }

    #[test]
    fn lowers_color_green() {
        assert_lowers_debug(deckmaste_authoring::Color::Green);
        assert_matches!(
            deckmaste_authoring::Color::Green.lower(),
            deckmaste_core::Color::Green
        );
    }

    #[test]
    fn lowers_color_or_colorless_colorless() {
        assert_lowers_debug(deckmaste_authoring::ColorOrColorless::Colorless);
        assert_matches!(
            deckmaste_authoring::ColorOrColorless::Colorless.lower(),
            deckmaste_core::ColorOrColorless::Colorless
        );
    }

    #[test]
    fn lowers_color_or_colorless_color() {
        assert_lowers_debug(deckmaste_authoring::ColorOrColorless::Color(minimal_color()));
        assert_matches!(
            deckmaste_authoring::ColorOrColorless::Color(minimal_color()).lower(),
            deckmaste_core::ColorOrColorless::Color(..)
        );
    }
}
