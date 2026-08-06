//! `color` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Color {
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

impl Lower for deckmaste_semantics::ColorOrColorless {
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
    use crate::minimal::*;

    #[test]
    fn lowers_color_white() {
        assert_matches!(
            deckmaste_semantics::Color::White.lower(),
            deckmaste_core::Color::White
        );
    }

    #[test]
    fn lowers_color_blue() {
        assert_matches!(
            deckmaste_semantics::Color::Blue.lower(),
            deckmaste_core::Color::Blue
        );
    }

    #[test]
    fn lowers_color_black() {
        assert_matches!(
            deckmaste_semantics::Color::Black.lower(),
            deckmaste_core::Color::Black
        );
    }

    #[test]
    fn lowers_color_red() {
        assert_matches!(
            deckmaste_semantics::Color::Red.lower(),
            deckmaste_core::Color::Red
        );
    }

    #[test]
    fn lowers_color_green() {
        assert_matches!(
            deckmaste_semantics::Color::Green.lower(),
            deckmaste_core::Color::Green
        );
    }

    #[test]
    fn lowers_color_or_colorless_colorless() {
        assert_matches!(
            deckmaste_semantics::ColorOrColorless::Colorless.lower(),
            deckmaste_core::ColorOrColorless::Colorless
        );
    }

    #[test]
    fn lowers_color_or_colorless_color() {
        assert_matches!(
            deckmaste_semantics::ColorOrColorless::Color(minimal_color()).lower(),
            deckmaste_core::ColorOrColorless::Color(deckmaste_core::Color::White)
        );
    }
}
