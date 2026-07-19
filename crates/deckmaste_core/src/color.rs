use crate::SupportsMacros;

/// One of the five colors of Magic ([CR#105.1]). Colorless is not a color.
///
/// `SupportsMacros` (all-unit, no embed/expanded/literal markers, so it behaves
/// exactly like the plain serde derive) because it is the terminal embed
/// payload of [`ColorOrColorless::Color`] — a `#[macro_ron(embed)]` payload
/// must itself be macro-aware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
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

    /// The single-letter mana-symbol code — [`from_code`](Self::from_code)'s
    /// inverse ([CR#105.1,107.4a]).
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Color::White => "W",
            Color::Blue => "U",
            Color::Black => "B",
            Color::Red => "R",
            Color::Green => "G",
        }
    }
}

/// The `#[macro_ron(embed)]` Color variant serializes transparently, so the RON
/// stays flat: `White`, not `Color(White)`. A bare ident that isn't `Colorless`
/// falls through to the embedded [`Color`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ColorOrColorless {
    Colorless,
    #[macro_ron(embed)]
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
    fn from(color: Color) -> Self {
        ColorOrColorless::Color(color)
    }
}
