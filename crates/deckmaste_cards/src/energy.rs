//! Spelled-number ⇄ `{E}`-glyph-run normalization for energy costs/effects.
//!
//! The energy symbol `{E}` ([CR#107.14] — to pay `{E}` a player removes one
//! energy counter) prints as a run of glyphs up to five ("Pay {E}{E}"), then
//! spells the number and a single glyph past five ("Pay six {E}", "fifty {E}"),
//! because a run of fifty glyphs is unreadable. The `PayEnergy`/`GainEnergy`
//! macros match a run of glyphs (`${0*\{E\}}`), so a spelled-number form must
//! first fold to a run before it can graduate. This module owns that fold —
//! shared by extract/parse (rewriting the oracle line before it becomes an
//! `Unparsed`) and by `fidelity::normalize` (collapsing both the rendered and
//! oracle lines before the diff) — and its inverse, spelling a count past five
//! back out so a graduated card renders like the printed card.

use std::sync::LazyLock;

use regex::Regex;

/// The single-word English number names, paired with their value. The corpus's
/// energy costs only ever use single words (one, six, eight, fifty); compound
/// numbers ("twenty-one") never appear on an energy card, so the table stops at
/// the single-word range. Serves both directions: name → count (the fold) and
/// count → name (the render threshold).
const SPELLED: &[(u32, &str)] = &[
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (30, "thirty"),
    (40, "forty"),
    (50, "fifty"),
    (60, "sixty"),
    (70, "seventy"),
    (80, "eighty"),
    (90, "ninety"),
];

/// Matches a spelled number immediately followed by a single `{E}` — and
/// nothing else, so "six cards" / "eight damage" / "one or more {E}" are left
/// alone (only a spelled number *directly* before `{E}` is an energy count).
/// The `\b` word boundaries keep "eight" out of "eighteen"; the alternatives
/// run longest-first as belt-and-suspenders on top of that.
static SPELLED_ENERGY: LazyLock<Regex> = LazyLock::new(|| {
    let mut words: Vec<&str> = SPELLED.iter().map(|&(_, w)| w).collect();
    words.sort_by_key(|w| std::cmp::Reverse(w.len()));
    let alt = words.join("|");
    Regex::new(&format!(r"(?i)\b({alt})\b\s+\{{E\}}")).expect("valid spelled-energy regex")
});

/// Folds every `<spelled-number> {E}` into that many `{E}` glyphs so the
/// glyph-run matcher can read it: `"Pay six {E}"` → `"Pay {E}{E}{E}{E}{E}{E}"`.
/// Text without a spelled-number-`{E}` pair is returned unchanged.
#[must_use]
pub fn normalize_spelled_energy(text: &str) -> String {
    SPELLED_ENERGY
        .replace_all(text, |caps: &regex::Captures| {
            let word = &caps[1];
            let n = SPELLED
                .iter()
                .find(|&&(_, w)| w.eq_ignore_ascii_case(word))
                .map_or(0, |&(n, _)| n as usize);
            "{E}".repeat(n)
        })
        .into_owned()
}

/// The single-word English name for `n` (`50` → `"fifty"`), or the bare digits
/// when `n` has no single-word name (never a >5 energy count in the corpus,
/// which tops out at fifty). Feeds the render threshold that spells counts past
/// five.
#[must_use]
pub fn spell_number(n: usize) -> String {
    SPELLED
        .iter()
        .find(|&&(v, _)| v as usize == n)
        .map_or_else(|| n.to_string(), |&(_, w)| w.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_spelled_number_before_single_glyph() {
        assert_eq!(
            normalize_spelled_energy("Pay six {E}"),
            "Pay {E}{E}{E}{E}{E}{E}"
        );
        assert_eq!(normalize_spelled_energy("you get one {E}"), "you get {E}");
        // The >5 outlier that motivates the whole exercise.
        assert_eq!(
            normalize_spelled_energy("Pay fifty {E}: Draw seven cards."),
            format!("Pay {}: Draw seven cards.", "{E}".repeat(50))
        );
    }

    #[test]
    fn leaves_non_energy_counts_alone() {
        // A spelled number that isn't followed by `{E}`.
        assert_eq!(
            normalize_spelled_energy("Look at the top six cards"),
            "Look at the top six cards"
        );
        assert_eq!(
            normalize_spelled_energy("~ deals eight damage"),
            "~ deals eight damage"
        );
        // A word boundary keeps "eight" out of "eighteen" (and there is no
        // single glyph after, anyway).
        assert_eq!(
            normalize_spelled_energy("target with eighteen life"),
            "target with eighteen life"
        );
        // The dynamic "one or more {E}" shape is untouched: the number isn't
        // *directly* before the glyph.
        assert_eq!(
            normalize_spelled_energy("If you would get one or more {E}"),
            "If you would get one or more {E}"
        );
    }

    #[test]
    fn spell_number_matches_oracle_words() {
        assert_eq!(spell_number(6), "six");
        assert_eq!(spell_number(8), "eight");
        assert_eq!(spell_number(50), "fifty");
        // Unnamed count falls back to digits rather than panicking.
        assert_eq!(spell_number(37), "37");
    }
}
