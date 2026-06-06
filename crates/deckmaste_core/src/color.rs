use serde::{Deserialize, Serialize};

/// One of the five colors of Magic (CR 105.1). Colorless is not a color.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum ColorOrColorless {
    Colorless,
    #[serde(untagged)]
    Color(Color),
}

impl ColorOrColorless {
    pub fn from_code(code: &str) -> Option<Self> {
        Some(match code {
            "C" => ColorOrColorless::Colorless,
            code => ColorOrColorless::Color(Color::from_code(code)?),
        })
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            &Self::Colorless => None,
            &Self::Color(color) => Some(color),
        }
    }
}

impl From<Color> for ColorOrColorless {
    fn from(color: Color) -> Self { ColorOrColorless::Color(color) }
}
