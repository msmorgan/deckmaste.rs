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

/// The notation used to format an integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Numeral {
    /// Lowercase English cardinal words.
    Cardinal,
    /// Lowercase English ordinal words.
    Ordinal,
    /// Signed decimal digits; the flag enables comma grouping.
    Arabic(bool),
    /// Uppercase Roman numerals with Latin fallbacks and signs.
    Roman,
}

impl Numeral {
    /// Formats `value` using this notation.
    #[must_use]
    pub fn format(self, value: i32) -> String {
        match self {
            Self::Cardinal => format_cardinal(value),
            Self::Ordinal => format_ordinal(value),
            Self::Arabic(commas) => format_arabic(value, commas),
            Self::Roman => format_roman(value),
        }
    }
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

fn format_ordinal(value: i32) -> String {
    if value == 0 {
        return "never".to_owned();
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
    use super::*;

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
    fn positive_ordinals_transform_the_final_word() {
        assert_formats(
            Numeral::Ordinal,
            &[
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
    fn nonpositive_ordinals_are_sequence_relative() {
        assert_formats(
            Numeral::Ordinal,
            &[
                (0, "never"),
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
