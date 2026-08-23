use macro_ron::v2::Onset;

const ROMAN_PARTS: [(u32, &str); 13] = [
    (1_000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

/// Opaque identity data for the card face whose Oracle text is being parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseContext<'a> {
    card_name: &'a str,
    abbreviated_card_name: &'a str,
    card_name_onset: Onset,
}

impl<'a> ParseContext<'a> {
    /// Creates context for one nonempty face name.
    ///
    /// `is_legendary` comes from the face's authoritative type metadata. It is
    /// the only license for a distinct shortened self-reference. The onset is
    /// a separately normalized realization fact supplied by the caller; this
    /// constructor never infers either fact by inspecting the name. The full
    /// name remains opaque and valid regardless of punctuation or leading byte.
    #[must_use]
    pub fn new(card_name: &'a str, is_legendary: bool, card_name_onset: Onset) -> Option<Self> {
        (!card_name.is_empty()).then(|| Self {
            card_name,
            abbreviated_card_name: legendary_short_name(card_name, is_legendary)
                .unwrap_or(card_name),
            card_name_onset,
        })
    }

    #[must_use]
    pub const fn card_name(self) -> &'a str {
        self.card_name
    }

    #[must_use]
    pub const fn abbreviated_card_name(self) -> &'a str {
        self.abbreviated_card_name
    }

    pub(crate) const fn card_name_onset(self) -> Onset {
        self.card_name_onset
    }

    pub(crate) const fn abbreviated_card_name_onset(self) -> Onset {
        self.card_name_onset
    }
}

fn legendary_short_name(name: &str, is_legendary: bool) -> Option<&str> {
    if !is_legendary {
        return None;
    }
    let candidate = if let Some((pre_comma, _)) = name.split_once(',') {
        pre_comma.trim()
    } else if name.starts_with("The ") {
        return None;
    } else if let Some(without_numeral) = strip_trailing_roman_numeral(name) {
        without_numeral
    } else if let Some(prefix) = prefix_before_epithet(name) {
        prefix
    } else {
        name.split(' ').next().unwrap_or(name)
    };
    (candidate != name && !candidate.is_empty()).then_some(candidate)
}

fn strip_trailing_roman_numeral(name: &str) -> Option<&str> {
    let (prefix, last_word) = name.rsplit_once(' ')?;
    is_canonical_roman_numeral(last_word).then(|| prefix.trim_end())
}

fn is_canonical_roman_numeral(input: &str) -> bool {
    if input.is_empty() || input.len() > "MMMDCCCLXXXVIII".len() {
        return false;
    }
    let mut symbols = input;
    let mut magnitude = 0_u32;
    for (amount, part) in ROMAN_PARTS {
        while let Some(remainder) = symbols.strip_prefix(part) {
            let Some(next_magnitude) = magnitude.checked_add(amount) else {
                return false;
            };
            magnitude = next_magnitude;
            symbols = remainder;
        }
    }
    symbols.is_empty() && (1..=3_999).contains(&magnitude) && roman_magnitude(magnitude) == input
}

fn roman_magnitude(mut value: u32) -> String {
    let mut numeral = String::new();
    for (amount, symbols) in ROMAN_PARTS {
        while value >= amount {
            numeral.push_str(symbols);
            value -= amount;
        }
    }
    numeral
}

fn prefix_before_epithet(name: &str) -> Option<&str> {
    [" the ", " of "]
        .into_iter()
        .filter_map(|separator| name.find(separator))
        .min()
        .map(|index| &name[..index])
}
