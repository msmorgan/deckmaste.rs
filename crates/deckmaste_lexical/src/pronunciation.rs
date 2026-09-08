use crate::Numeral;
use crate::NumeralCodec;
use crate::Onset;

/// Normalization recipe for declared spellings. Overrides take precedence;
/// orthography is the final fallback, never the grammatical admission rule.
pub(crate) fn default_onset(surface: &str) -> Option<Onset> {
    let word = surface
        .split(|c: char| c.is_whitespace() || c == '-')
        .next()?;
    if word.is_empty() {
        return None;
    }
    let digits: String = word
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',')
        .collect();
    if !digits.is_empty() {
        return digits.replace(',', "").parse().ok().and_then(numeral_onset);
    }
    let letters: String = word.chars().take_while(char::is_ascii_alphabetic).collect();
    if !letters.is_empty() && letters.chars().all(|c| c.is_ascii_uppercase()) {
        return Some(
            if matches!(
                letters.as_bytes()[0],
                b'A' | b'E' | b'F' | b'H' | b'I' | b'L' | b'M' | b'N' | b'O' | b'R' | b'S' | b'X'
            ) {
                Onset::Vowel
            } else {
                Onset::Consonant
            },
        );
    }
    let lower = word.to_ascii_lowercase();
    if ["heir", "honest", "honor", "hour"]
        .iter()
        .any(|p| lower.starts_with(p))
    {
        return Some(Onset::Vowel);
    }
    if lower.starts_with("eu")
        || lower == "one"
        || lower.starts_with("once")
        || [
            "unit",
            "unite",
            "unity",
            "unicorn",
            "uniform",
            "unique",
            "union",
            "universe",
            "universal",
            "university",
            "use",
            "user",
            "usual",
            "utensil",
            "utility",
            "utopia",
        ]
        .iter()
        .any(|p| lower.starts_with(p))
    {
        return Some(Onset::Consonant);
    }
    match lower.as_bytes().first().copied() {
        Some(b'a' | b'e' | b'i' | b'o' | b'u') => Some(Onset::Vowel),
        Some(c) if c.is_ascii_alphabetic() => Some(Onset::Consonant),
        _ => None,
    }
}

pub(crate) fn numeral_onset(value: i32) -> Option<Onset> {
    default_onset(&Numeral::Cardinal.format(value))
}
