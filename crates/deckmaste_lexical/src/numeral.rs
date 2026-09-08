const SMALL_CARDINALS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [(u32, &str); 3] = [
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1_000, "thousand"),
];

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

/// Whether a word or digit run can occur within a canonical numeral.
///
/// This only bounds scanning: a component sequence still needs a successful
/// [`Numeral::parse`]. Separators and signs are handled by the token scanner.
pub(crate) fn numeral_component(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    if cardinal_component(text)
        || matches!(
            text,
            "negative"
                | "minus"
                | "last"
                | "to"
                | "infinitum"
                | "negativum"
                | "nulla"
                | "negativus"
                | "first"
                | "second"
                | "third"
                | "fifth"
                | "eighth"
                | "ninth"
                | "twelfth"
        )
        || text.bytes().all(|byte| byte.is_ascii_digit())
        || text
            .bytes()
            .all(|byte| matches!(byte, b'I' | b'V' | b'X' | b'L' | b'C' | b'D' | b'M'))
    {
        return true;
    }
    text.strip_suffix("th").is_some_and(cardinal_component)
        || text
            .strip_suffix("ieth")
            .is_some_and(|stem| TENS.iter().any(|word| word.strip_suffix('y') == Some(stem)))
}

fn cardinal_component(text: &str) -> bool {
    text == "hundred"
        || SMALL_CARDINALS.contains(&text)
        || TENS[2..].contains(&text)
        || SCALES.iter().any(|(_, name)| *name == text)
}

pub use deckmaste_lexical_model::Numeral;

/// An error returned when numeral text is invalid or noncanonical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, serde::Serialize)]
#[error("invalid or noncanonical numeral")]
pub struct ParseNumeralError;

/// Parse and realization behavior for a shared numeral notation value.
pub trait NumeralCodec {
    /// Retains grouping as identity only when it changes the written numeral.
    #[must_use]
    fn canonical_notation(self, value: i32) -> Numeral;

    /// Formats `value` using this notation.
    ///
    /// Roman magnitudes above 3,999 use `infinitum`, with `negativum` appended
    /// for negative values. Parsing those spellings returns `i32::MAX` and
    /// `i32::MIN`, respectively, rather than recovering the original magnitude.
    #[must_use]
    fn format(self, value: i32) -> String;

    /// Formats `value` only when parsing the result recovers that value.
    ///
    /// # Errors
    ///
    /// Returns [`ParseNumeralError`] for Roman values outside -3,999 through
    /// 3,999 other than `i32::MIN` and `i32::MAX`.
    fn try_format(self, value: i32) -> Result<String, ParseNumeralError>;

    /// Parses an integer written using this notation.
    ///
    /// # Errors
    ///
    /// Returns [`ParseNumeralError`] when `input` is invalid or noncanonical.
    fn parse(self, input: &str) -> Result<i32, ParseNumeralError>;
}

impl NumeralCodec for Numeral {
    fn canonical_notation(self, value: i32) -> Numeral {
        match self {
            Self::Arabic(true) if value.unsigned_abs() < 1_000 => Self::Arabic(false),
            notation => notation,
        }
    }

    fn format(self, value: i32) -> String {
        match self {
            Self::Cardinal => format_cardinal(value),
            Self::Ordinal => format_ordinal(value),
            Self::Arabic(commas) => format_arabic(value, commas),
            Self::Roman => format_roman(value),
        }
    }

    fn try_format(self, value: i32) -> Result<String, ParseNumeralError> {
        let surface = self.format(value);
        if self.parse(&surface)? == value {
            Ok(surface)
        } else {
            Err(ParseNumeralError)
        }
    }

    fn parse(self, input: &str) -> Result<i32, ParseNumeralError> {
        match self {
            Self::Arabic(false) => canonical(self, input, input.parse().ok()),
            Self::Arabic(true) => canonical(self, input, input.replace(',', "").parse().ok()),
            Self::Cardinal => canonical(self, input, parse_cardinal_candidate(input)),
            Self::Ordinal => canonical(self, input, parse_ordinal_candidate(input)),
            Self::Roman => canonical(self, input, parse_roman_candidate(input)),
        }
    }
}

fn canonical(
    numeral: Numeral,
    input: &str,
    candidate: Option<i32>,
) -> Result<i32, ParseNumeralError> {
    candidate
        .filter(|&value| numeral.format(value) == input)
        .ok_or(ParseNumeralError)
}

fn format_cardinal(value: i32) -> String {
    let words = cardinal_magnitude(value.unsigned_abs());
    if value.is_negative() { format!("negative {words}") } else { words }
}

fn cardinal_magnitude(mut value: u32) -> String {
    if value == 0 {
        return SMALL_CARDINALS[0].to_owned();
    }

    let mut groups = Vec::with_capacity(4);
    for (scale, name) in SCALES {
        let group = value / scale;
        if group != 0 {
            groups.push(format!("{} {name}", cardinal_below_thousand(group)));
            value %= scale;
        }
    }
    if value != 0 {
        groups.push(cardinal_below_thousand(value));
    }
    groups.join(", ")
}

fn cardinal_below_thousand(mut value: u32) -> String {
    debug_assert!((1..1_000).contains(&value));
    let mut parts = Vec::with_capacity(2);
    if value >= 100 {
        parts.push(format!(
            "{} hundred",
            SMALL_CARDINALS[(value / 100) as usize]
        ));
        value %= 100;
    }
    if value != 0 {
        parts.push(cardinal_below_hundred(value));
    }
    parts.join(" ")
}

fn cardinal_below_hundred(value: u32) -> String {
    debug_assert!((1..100).contains(&value));
    if value < 20 {
        return SMALL_CARDINALS[value as usize].to_owned();
    }
    let tens = TENS[(value / 10) as usize];
    let units = value % 10;
    if units == 0 {
        tens.to_owned()
    } else {
        format!("{tens}-{}", SMALL_CARDINALS[units as usize])
    }
}

fn parse_cardinal_below_hundred(input: &str) -> Option<u32> {
    if let Some(value) = SMALL_CARDINALS.iter().position(|&word| word == input) {
        return u32::try_from(value).ok();
    }
    if let Some(tens) = TENS
        .iter()
        .position(|&word| word == input)
        .filter(|&value| value >= 2)
    {
        return u32::try_from(tens).ok()?.checked_mul(10);
    }

    let (tens, units) = input.split_once('-')?;
    let tens = u32::try_from(
        TENS.iter()
            .position(|&word| word == tens)
            .filter(|&value| value >= 2)?,
    )
    .ok()?;
    let units = u32::try_from(
        SMALL_CARDINALS[1..10]
            .iter()
            .position(|&word| word == units)?
            + 1,
    )
    .ok()?;
    tens.checked_mul(10)?.checked_add(units)
}

fn parse_cardinal_below_thousand(input: &str) -> Option<u32> {
    if let Some(value) = parse_cardinal_below_hundred(input) {
        return Some(value);
    }

    let (hundreds, remainder) = input.split_once(" hundred")?;
    let hundreds = u32::try_from(
        SMALL_CARDINALS[1..10]
            .iter()
            .position(|&word| word == hundreds)?
            + 1,
    )
    .ok()?;
    let remainder = if remainder.is_empty() {
        0
    } else {
        parse_cardinal_below_hundred(remainder.strip_prefix(' ')?)?
    };
    hundreds.checked_mul(100)?.checked_add(remainder)
}

fn parse_cardinal_magnitude(input: &str) -> Option<u32> {
    let mut total = 0_u32;
    let mut groups = input.split(", ").peekable();
    while let Some(group) = groups.next() {
        let (coefficient, scale) = SCALES
            .iter()
            .find_map(|&(scale, name)| {
                group
                    .strip_suffix(name)?
                    .strip_suffix(' ')
                    .map(|coefficient| (coefficient, scale))
            })
            .unwrap_or((group, 1));
        if scale == 1 && groups.peek().is_some() {
            return None;
        }
        let coefficient = parse_cardinal_below_thousand(coefficient)?;
        total = total.checked_add(coefficient.checked_mul(scale)?)?;
    }
    Some(total)
}

fn signed_magnitude(magnitude: u32, negative: bool) -> Option<i32> {
    let magnitude = i64::from(magnitude);
    let signed = if negative { -magnitude } else { magnitude };
    signed.try_into().ok()
}

fn parse_cardinal_candidate(input: &str) -> Option<i32> {
    let (magnitude, negative) = input
        .strip_prefix("negative ")
        .map_or((input, false), |magnitude| (magnitude, true));
    signed_magnitude(parse_cardinal_magnitude(magnitude)?, negative)
}

fn cardinalize_ordinal(input: &str) -> Option<String> {
    let last_start = input.rfind([' ', '-']).map_or(0, |delimiter| delimiter + 1);
    let (prefix, last) = input.split_at(last_start);
    let cardinal = match last {
        "first" => "one".to_owned(),
        "second" => "two".to_owned(),
        "third" => "three".to_owned(),
        "fifth" => "five".to_owned(),
        "eighth" => "eight".to_owned(),
        "ninth" => "nine".to_owned(),
        "twelfth" => "twelve".to_owned(),
        _ => {
            let (stem, ending) = last
                .strip_suffix("ieth")
                .map(|stem| (stem, "y"))
                .or_else(|| last.strip_suffix("th").map(|stem| (stem, "")))?;
            if stem.is_empty() {
                return None;
            }
            format!("{stem}{ending}")
        }
    };
    Some(format!("{prefix}{cardinal}"))
}

fn parse_ordinal_candidate(input: &str) -> Option<i32> {
    if input == "last" {
        Some(-1)
    } else {
        let (ordinal, negative) = input
            .strip_suffix(" to last")
            .map_or((input, false), |ordinal| (ordinal, true));
        let cardinal = cardinalize_ordinal(ordinal)?;
        signed_magnitude(parse_cardinal_magnitude(&cardinal)?, negative)
    }
}

fn parse_roman_candidate(input: &str) -> Option<i32> {
    match input {
        "infinitum" => return Some(i32::MAX),
        "infinitum negativum" => return Some(i32::MIN),
        "nulla" => return Some(0),
        _ => {}
    }

    let (mut symbols, negative) = input
        .strip_suffix(" negativus")
        .map_or((input, false), |symbols| (symbols, true));
    let mut magnitude = 0_u32;
    for (amount, part) in ROMAN_PARTS {
        while let Some(remainder) = symbols.strip_prefix(part) {
            magnitude = magnitude.checked_add(amount)?;
            symbols = remainder;
        }
    }
    if !symbols.is_empty() || !(1..=3_999).contains(&magnitude) {
        return None;
    }
    signed_magnitude(magnitude, negative)
}

fn format_ordinal(value: i32) -> String {
    if value == 0 {
        return "zeroth".to_owned();
    }
    if value == -1 {
        return "last".to_owned();
    }
    let words = ordinal_magnitude(value.unsigned_abs());
    if value.is_negative() { format!("{words} to last") } else { words }
}

fn ordinal_magnitude(value: u32) -> String {
    debug_assert!(value != 0);
    let cardinal = cardinal_magnitude(value);
    let last_start = cardinal
        .rfind([' ', '-'])
        .map_or(0, |delimiter| delimiter + 1);
    let (prefix, last) = cardinal.split_at(last_start);
    format!("{prefix}{}", ordinal_word(last))
}

fn ordinal_word(word: &str) -> String {
    match word {
        "one" => "first".to_owned(),
        "two" => "second".to_owned(),
        "three" => "third".to_owned(),
        "five" => "fifth".to_owned(),
        "eight" => "eighth".to_owned(),
        "nine" => "ninth".to_owned(),
        "twelve" => "twelfth".to_owned(),
        _ => word
            .strip_suffix('y')
            .map_or_else(|| format!("{word}th"), |stem| format!("{stem}ieth")),
    }
}

fn format_arabic(value: i32, commas: bool) -> String {
    if !commas {
        return value.to_string();
    }

    let digits = value.unsigned_abs().to_string();
    let separator_count = (digits.len() - 1) / 3;
    let sign_len = usize::from(value.is_negative());
    let mut formatted = String::with_capacity(digits.len() + separator_count + sign_len);
    if value.is_negative() {
        formatted.push('-');
    }
    let first_group_len = match digits.len() % 3 {
        0 => 3,
        remainder => remainder,
    };
    formatted.push_str(&digits[..first_group_len]);
    let mut group_start = first_group_len;
    while group_start < digits.len() {
        formatted.push(',');
        formatted.push_str(&digits[group_start..group_start + 3]);
        group_start += 3;
    }
    formatted
}

fn format_roman(value: i32) -> String {
    let magnitude = value.unsigned_abs();
    let numeral = match magnitude {
        0 => "nulla".to_owned(),
        1..=3_999 => roman_magnitude(magnitude),
        _ => "infinitum".to_owned(),
    };
    if value.is_negative() {
        let adjective = if magnitude <= 3_999 { "negativus" } else { "negativum" };
        format!("{numeral} {adjective}")
    } else {
        numeral
    }
}

fn roman_magnitude(mut value: u32) -> String {
    debug_assert!((1..=3_999).contains(&value));
    let mut numeral = String::new();
    for (amount, symbols) in ROMAN_PARTS {
        while value >= amount {
            numeral.push_str(symbols);
            value -= amount;
        }
    }
    numeral
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn components_preserve_every_notation(
            value in any::<i32>(),
            finite_roman in -3_999i32..=3_999,
        ) {
            for (notation, value) in [
                (Numeral::Cardinal, value),
                (Numeral::Ordinal, value),
                (Numeral::Arabic(false), value),
                (Numeral::Arabic(true), value),
                (Numeral::Roman, finite_roman),
                (Numeral::Roman, i32::MIN),
                (Numeral::Roman, i32::MAX),
            ] {
                let surface = notation.try_format(value).unwrap();
                for component in surface.split(|c: char| !c.is_ascii_alphanumeric()) {
                    if !component.is_empty() {
                        prop_assert!(numeral_component(component), "{} in {}", component, surface);
                    }
                }
            }
        }

        #[test]
        fn arabic_round_trips(value in any::<i32>(), commas in any::<bool>()) {
            let numeral = Numeral::Arabic(commas);
            let formatted = numeral.format(value);
            prop_assert_eq!(numeral.parse(&formatted), Ok(value));
        }

        #[test]
        fn cardinal_round_trips(value in any::<i32>()) {
            let formatted = Numeral::Cardinal.format(value);
            prop_assert_eq!(Numeral::Cardinal.parse(&formatted), Ok(value));
        }

        #[test]
        fn ordinal_round_trips(value in any::<i32>()) {
            let formatted = Numeral::Ordinal.format(value);
            prop_assert_eq!(Numeral::Ordinal.parse(&formatted), Ok(value));
        }

        #[test]
        fn roman_finite_round_trips(value in -3_999i32..=3_999) {
            let formatted = Numeral::Roman.format(value);
            prop_assert_eq!(Numeral::Roman.parse(&formatted), Ok(value));
        }

        #[test]
        fn roman_positive_infinity_normalizes(value in 4_000i32..=i32::MAX) {
            let formatted = Numeral::Roman.format(value);
            prop_assert_eq!(Numeral::Roman.parse(&formatted), Ok(i32::MAX));
        }

        #[test]
        fn roman_negative_infinity_normalizes(value in i32::MIN..=-4_000i32) {
            let formatted = Numeral::Roman.format(value);
            prop_assert_eq!(Numeral::Roman.parse(&formatted), Ok(i32::MIN));
        }

        #[test]
        fn successful_parses_are_stable(
            numeral in prop_oneof![
                Just(Numeral::Cardinal),
                Just(Numeral::Ordinal),
                any::<bool>().prop_map(Numeral::Arabic),
                Just(Numeral::Roman),
            ],
            input in any::<String>(),
        ) {
            if let Ok(value) = numeral.parse(&input) {
                prop_assert_eq!(numeral.format(value), input);
            }
        }
    }

    #[test]
    fn components_cover_small_values_and_scale_boundaries() {
        for value in (0..=1_000).chain([
            -1,
            -2,
            -21,
            -1_000,
            1_000_000,
            1_000_000_000,
            i32::MIN,
            i32::MAX,
        ]) {
            for notation in [
                Numeral::Cardinal,
                Numeral::Ordinal,
                Numeral::Arabic(false),
                Numeral::Arabic(true),
                Numeral::Roman,
            ] {
                if let Ok(surface) = notation.try_format(value) {
                    for component in surface.split(|c: char| !c.is_ascii_alphanumeric()) {
                        if !component.is_empty() {
                            assert!(numeral_component(component), "{component} in {surface}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn components_do_not_license_complete_numerals() {
        assert!(!numeral_component(""));
        assert!(!numeral_component("creature"));
        assert!(!numeral_component("target"));
        for component in ["hundred", "billion", "negativum", "to", "IIII"] {
            assert!(numeral_component(component));
            for notation in [
                Numeral::Cardinal,
                Numeral::Ordinal,
                Numeral::Arabic(false),
                Numeral::Arabic(true),
                Numeral::Roman,
            ] {
                assert_eq!(notation.parse(component), Err(ParseNumeralError));
            }
        }
    }

    #[test]
    fn checked_roman_formatting_rejects_lossy_values() {
        for value in [4_000, -4_000, i32::MAX - 1, i32::MIN + 1] {
            assert_eq!(Numeral::Roman.try_format(value), Err(ParseNumeralError));
        }
        for (value, expected) in [
            (0, "nulla"),
            (3_999, "MMMCMXCIX"),
            (-3_999, "MMMCMXCIX negativus"),
            (i32::MAX, "infinitum"),
            (i32::MIN, "infinitum negativum"),
        ] {
            assert_eq!(Numeral::Roman.try_format(value), Ok(expected.to_owned()));
        }
    }

    #[test]
    fn checked_formatting_preserves_other_notations() {
        for numeral in [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
        ] {
            for value in [i32::MIN, -4_000, -1, 0, 1, 4_000, i32::MAX] {
                assert_eq!(numeral.try_format(value), Ok(numeral.format(value)));
            }
        }
    }

    #[test]
    fn arabic_parsing_rejects_noncanonical_forms() {
        for (numeral, input) in [
            (Numeral::Arabic(true), "1000"),
            (Numeral::Arabic(true), "1,00"),
            (Numeral::Arabic(true), "01"),
            (Numeral::Arabic(true), "+1"),
            (Numeral::Arabic(false), "1,000"),
        ] {
            assert_eq!(numeral.parse(input), Err(ParseNumeralError));
        }
    }

    #[test]
    fn cardinal_parsing_rejects_noncanonical_forms() {
        for input in [
            "eleven hundred",
            "forty and five",
            "one hundred and five",
            "One",
            " one",
            "",
        ] {
            assert_eq!(Numeral::Cardinal.parse(input), Err(ParseNumeralError));
        }
    }

    #[test]
    fn cardinal_parsing_covers_compact_spellings() {
        for (value, input) in [
            (18_017_016, "eighteen million, seventeen thousand, sixteen"),
            (15_014_013, "fifteen million, fourteen thousand, thirteen"),
            (12_011_019, "twelve million, eleven thousand, nineteen"),
        ] {
            assert_eq!(Numeral::Cardinal.format(value), input);
            assert_eq!(Numeral::Cardinal.parse(input), Ok(value));
        }
    }

    #[test]
    fn ordinal_parsing_rejects_noncanonical_forms() {
        for input in [
            "never",
            "zeroth to last",
            "first to last",
            "negative first",
            "twenty-oneth",
        ] {
            assert_eq!(Numeral::Ordinal.parse(input), Err(ParseNumeralError));
        }
    }

    #[test]
    fn roman_parsing_rejects_noncanonical_forms() {
        for input in [
            "IIII",
            "iv",
            "I negative",
            "infinitum negativus",
            "nulla negativus",
        ] {
            assert_eq!(Numeral::Roman.parse(input), Err(ParseNumeralError));
        }
    }

    fn assert_formats(numeral: Numeral, cases: &[(i32, &str)]) {
        for &(value, expected) in cases {
            assert_eq!(
                numeral.format(value),
                expected,
                "formatting {value} as {numeral:?}"
            );
        }
    }

    #[test]
    fn cardinals_cover_the_full_i32_scale() {
        assert_formats(
            Numeral::Cardinal,
            &[
                (0, "zero"),
                (1, "one"),
                (100, "one hundred"),
                (101, "one hundred one"),
                (999, "nine hundred ninety-nine"),
                (1_000, "one thousand"),
                (1_001, "one thousand, one"),
                (1_000_000, "one million"),
                (1_000_000_000, "one billion"),
                (
                    i32::MAX,
                    "two billion, one hundred forty-seven million, four hundred eighty-three thousand, six hundred forty-seven",
                ),
                (
                    i32::MIN,
                    "negative two billion, one hundred forty-seven million, four hundred eighty-three thousand, six hundred forty-eight",
                ),
            ],
        );
    }

    #[test]
    fn cardinals_follow_chicago_word_boundaries() {
        assert_formats(
            Numeral::Cardinal,
            &[
                (11, "eleven"),
                (12, "twelve"),
                (19, "nineteen"),
                (20, "twenty"),
                (21, "twenty-one"),
                (32, "thirty-two"),
                (45, "forty-five"),
                (56, "fifty-six"),
                (67, "sixty-seven"),
                (78, "seventy-eight"),
                (89, "eighty-nine"),
                (99, "ninety-nine"),
                (105, "one hundred five"),
                (-42, "negative forty-two"),
            ],
        );
        for value in [105, 999, 1_105, 1_999_105, i32::MAX, i32::MIN] {
            assert!(!Numeral::Cardinal.format(value).contains(" and "));
        }
    }

    #[test]
    fn nonnegative_ordinals_transform_the_final_word() {
        assert_formats(
            Numeral::Ordinal,
            &[
                (0, "zeroth"),
                (1, "first"),
                (2, "second"),
                (3, "third"),
                (4, "fourth"),
                (5, "fifth"),
                (8, "eighth"),
                (9, "ninth"),
                (12, "twelfth"),
                (20, "twentieth"),
                (21, "twenty-first"),
                (32, "thirty-second"),
                (100, "one hundredth"),
                (101, "one hundred first"),
                (1_000, "one thousandth"),
                (1_001, "one thousand, first"),
                (1_000_000, "one millionth"),
                (1_000_000_000, "one billionth"),
                (
                    i32::MAX,
                    "two billion, one hundred forty-seven million, four hundred eighty-three thousand, six hundred forty-seventh",
                ),
            ],
        );
    }

    #[test]
    fn negative_ordinals_are_sequence_relative() {
        assert_formats(
            Numeral::Ordinal,
            &[
                (-1, "last"),
                (-2, "second to last"),
                (-21, "twenty-first to last"),
                (
                    i32::MIN,
                    "two billion, one hundred forty-seven million, four hundred eighty-three thousand, six hundred forty-eighth to last",
                ),
            ],
        );
    }

    #[test]
    fn arabic_formatting_toggles_comma_grouping() {
        assert_formats(
            Numeral::Arabic(false),
            &[
                (0, "0"),
                (1_234_567, "1234567"),
                (-1_234_567, "-1234567"),
                (i32::MIN, "-2147483648"),
            ],
        );
        assert_formats(
            Numeral::Arabic(true),
            &[
                (0, "0"),
                (999, "999"),
                (1_000, "1,000"),
                (1_234_567, "1,234,567"),
                (-1_234_567, "-1,234,567"),
                (i32::MAX, "2,147,483,647"),
                (i32::MIN, "-2,147,483,648"),
            ],
        );
    }

    #[test]
    fn roman_formatting_uses_standard_subtractive_notation() {
        assert_formats(
            Numeral::Roman,
            &[
                (1, "I"),
                (4, "IV"),
                (9, "IX"),
                (14, "XIV"),
                (40, "XL"),
                (49, "XLIX"),
                (90, "XC"),
                (99, "XCIX"),
                (400, "CD"),
                (944, "CMXLIV"),
                (3_999, "MMMCMXCIX"),
            ],
        );
    }

    #[test]
    fn roman_edge_cases_use_agreeing_latin() {
        assert_formats(
            Numeral::Roman,
            &[
                (0, "nulla"),
                (-1, "I negativus"),
                (-3_999, "MMMCMXCIX negativus"),
                (4_000, "infinitum"),
                (-4_000, "infinitum negativum"),
                (i32::MAX, "infinitum"),
                (i32::MIN, "infinitum negativum"),
            ],
        );
    }
}
