use serde::Serialize;

/// One of the five colors of Magic (CR 105.1). Colorless is not a color.
#[derive(Debug, PartialEq, Serialize)]
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
