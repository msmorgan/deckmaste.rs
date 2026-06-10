use serde::{Deserialize, Serialize};

use crate::Expand;

/// One of the five colors of Magic ([CR#105.1]). Colorless is not a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    /// Maps the single-letter codes used by mana symbols and data sources
    /// like MTGJSON.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        Some(match code {
            "W" => Color::White,
            "U" => Color::Blue,
            "B" => Color::Black,
            "R" => Color::Red,
            "G" => Color::Green,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum ColorOrColorless {
    Colorless,
    #[serde(untagged)]
    Color(Color),
}

impl ColorOrColorless {
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        Some(match code {
            "C" => ColorOrColorless::Colorless,
            code => ColorOrColorless::Color(Color::from_code(code)?),
        })
    }

    #[must_use]
    pub fn color(&self) -> Option<Color> {
        match *self {
            Self::Colorless => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl From<Color> for ColorOrColorless {
    fn from(color: Color) -> Self { ColorOrColorless::Color(color) }
}
