/// Normalizes typographic quotation marks before tokenization.
///
/// The semantic grammar and renderer use one ASCII representation for quotes
/// and apostrophes. Round-trip callers apply this function to expected text as
/// well as parser input.
#[must_use]
pub fn normalize_typographic_quotes(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '’' => '\'',
            '“' | '”' => '"',
            _ => character,
        })
        .collect()
}

/// Normalizes a die-roll result-row range separator to a single en dash.
///
/// A roll-table row is printed as `RANGE | body`. Oracle text draws an
/// inclusive range's bounds with two different glyphs — an unspaced em dash
/// (`1—9 |`, the overwhelming majority) and, on one card, an unspaced ASCII
/// hyphen (`1-9 |`). Neither distinction is language: the en dash (`–`,
/// U+2013) is the typographically correct range glyph, so both surfaces are
/// rewritten to it at the input boundary. The grammar and renderer then use
/// that one glyph, and round-trip callers apply this to expected text as well.
///
/// The rewrite is scoped tightly to the row-key position — a line that begins
/// with `<digits><dash><digits> |` — so a hyphen anywhere else (a mid-line
/// range, a hyphenated word) is untouched. Both raw glyphs collapse to the en
/// dash; a line already using an en dash, or one that is not a roll row, is
/// returned unchanged.
#[must_use]
pub fn normalize_roll_row_dashes(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            output.push('\n');
        }
        output.push_str(&normalize_roll_row_line(line));
    }
    output
}

/// Rewrites the range separator of a single line when it is a roll-row key
/// (`^<digits><dash><digits> |`, where `<dash>` is an em dash or an ASCII
/// hyphen), leaving every other line untouched.
fn normalize_roll_row_line(line: &str) -> String {
    let low_end = line
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len());
    if low_end == 0 {
        return line.to_owned();
    }
    let Some(dash) = line[low_end..].chars().next() else {
        return line.to_owned();
    };
    if dash != '-' && dash != '\u{2014}' {
        return line.to_owned();
    }
    let high_start = low_end + dash.len_utf8();
    let high_len = line[high_start..]
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len() - high_start);
    if high_len == 0 || !line[high_start + high_len..].starts_with(" |") {
        return line.to_owned();
    }
    let mut rewritten = String::with_capacity(line.len());
    rewritten.push_str(&line[..low_end]);
    rewritten.push('\u{2013}');
    rewritten.push_str(&line[high_start..]);
    rewritten
}

/// Uppercases a sentence-initial letter that Oracle printed lowercase by
/// data-entry mistake, not by any rule of the language.
///
/// A lowercase ASCII letter is rewritten to uppercase when it is preceded, on
/// the same line, by a `.` or a `:` followed by exactly one space — the two
/// punctuation marks that open a new sentence in Oracle text (a period
/// between sentences, a colon after an activation cost). Nothing else is
/// touched: a mid-word or decimal period, a letter already uppercase, and any
/// other position are all left alone.
///
/// This is typography, not language: two faces in the supported corpus break
/// Oracle's own sentence-initial capitalization rule, and both are data
/// defects rather than a real casing distinction to model —
///
/// - Sphinx Summoner: the snapshot reads `...put it into your hand. then
///   shuffle...` where the live card has a comma, not a period (a stale
///   snapshot datum).
/// - Necratog: the snapshot reads `...of your graveyard: this creature gets
///   +2/+2...` where Oracle capitalizes after the cost colon (a live-oracle
///   typo; normalizing the position is inert on errata).
///
/// Rather than store a casing bit on every sentence to reproduce two data
/// defects, the boundary rewrites the position and the round-trip gate
/// compares in the normalized domain. This is the same move the 2026-07-23
/// round made for `RollRangeDash` via [`normalize_roll_row_dashes`].
#[must_use]
pub fn normalize_sentence_case(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            output.push('\n');
        }
        output.push_str(&normalize_sentence_case_line(line));
    }
    output
}

/// Rewrites every sentence-initial lowercase letter on a single line, leaving
/// every other character untouched. Works over `char`s, not bytes, so a
/// multi-byte character elsewhere on the line is reproduced intact.
fn normalize_sentence_case_line(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut output = String::with_capacity(line.len());
    for (index, &character) in chars.iter().enumerate() {
        // A `. ` or `: ` immediately behind this character is a sentence
        // boundary; two spaces would put a space, not the punctuation, at
        // `index - 2`, which is exactly "followed by exactly one space".
        let boundary =
            index >= 2 && chars[index - 1] == ' ' && matches!(chars[index - 2], '.' | ':');
        if boundary && character.is_ascii_lowercase() {
            output.push(character.to_ascii_uppercase());
        } else {
            output.push(character);
        }
    }
    output
}

/// Removes parenthesized reminder text before English parsing.
///
/// Parentheses are lexical trivia in Oracle text for this phase. One ordinary
/// space immediately before a balanced group is removed with it; unmatched
/// opening parentheses are preserved.
#[must_use]
pub fn strip_reminder_text(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut group = String::new();
    let mut depth = 0_usize;

    for character in text.chars() {
        if depth == 0 {
            if character == '(' {
                depth = 1;
                group.push(character);
            } else {
                output.push(character);
            }
        } else {
            group.push(character);
            match character {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        group.clear();
                        if output.ends_with(' ') {
                            output.pop();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    output.push_str(&group);
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typographic_quotes_normalize_to_ascii_at_the_input_boundary() {
        assert_eq!(
            normalize_typographic_quotes(
                "Other permanents you control have “{T}: Add one mana of any color.”"
            ),
            "Other permanents you control have \"{T}: Add one mana of any color.\""
        );
        assert_eq!(normalize_typographic_quotes("It can’t."), "It can't.");
    }

    #[test]
    fn roll_row_hyphen_ranges_normalize_to_an_en_dash() {
        // Mathise's two hyphen rows are the only hyphen roll rows in the corpus.
        assert_eq!(
            normalize_roll_row_dashes(
                "1-9 | Each player draws a card.\n10-19 | You draw a card.\n20 | Copy that spell."
            ),
            "1\u{2013}9 | Each player draws a card.\n10\u{2013}19 | You draw a card.\n20 | Copy that spell."
        );
    }

    #[test]
    fn roll_row_em_dash_ranges_normalize_to_an_en_dash() {
        assert_eq!(
            normalize_roll_row_dashes("2\u{2014}9 | Create five tokens."),
            "2\u{2013}9 | Create five tokens."
        );
    }

    #[test]
    fn hyphens_outside_the_row_key_position_are_left_alone() {
        // A single-face row has no separator to rewrite.
        assert_eq!(
            normalize_roll_row_dashes("20 | Draw a card."),
            "20 | Draw a card."
        );
        // A mid-line numeric range is not a row key (not line-anchored, no pipe).
        assert_eq!(
            normalize_roll_row_dashes("Whenever you deal 3-4 damage, draw a card."),
            "Whenever you deal 3-4 damage, draw a card."
        );
        // A leading range with no ` |` separator is not a roll row.
        assert_eq!(
            normalize_roll_row_dashes("2-9 creatures attack."),
            "2-9 creatures attack."
        );
    }

    #[test]
    fn sentence_case_witnesses_normalize_after_a_period_or_cost_colon() {
        // Sphinx Summoner's stale snapshot: the live card has a comma, not a
        // period, at this position, so Oracle never actually breaks a
        // sentence here — but the snapshot text does, and must normalize.
        assert_eq!(
            normalize_sentence_case(
                "Exile the top card of your library. Put it into your hand. then shuffle your \
                 library."
            ),
            "Exile the top card of your library. Put it into your hand. Then shuffle your \
             library."
        );
        // Necratog: the live Oracle capitalizes after the activation-cost
        // colon; the snapshot's lowercase `this` is a live-oracle typo.
        assert_eq!(
            normalize_sentence_case(
                "{1}, Sacrifice a creature card from your graveyard: this creature gets +2/+2 \
                 until end of turn."
            ),
            "{1}, Sacrifice a creature card from your graveyard: This creature gets +2/+2 \
             until end of turn."
        );
    }

    #[test]
    fn sentence_case_near_misses_are_left_alone() {
        // Already uppercase: nothing to rewrite.
        assert_eq!(
            normalize_sentence_case("Draw a card. Then discard a card."),
            "Draw a card. Then discard a card."
        );
        // A lowercase letter not preceded by `. ` or `: ` is unchanged.
        assert_eq!(
            normalize_sentence_case("You may then draw a card."),
            "You may then draw a card."
        );
        // A decimal point is not a sentence boundary: the character after it
        // is a digit, never a letter to uppercase.
        assert_eq!(
            normalize_sentence_case("Each player loses 3.5 life."),
            "Each player loses 3.5 life."
        );
        // A mid-word period with no following space is not `. `, so the
        // letter immediately after it is left alone.
        assert_eq!(
            normalize_sentence_case("A card named e.g.this stays as printed."),
            "A card named e.g.this stays as printed."
        );
    }

    #[test]
    fn reminder_text_is_removed_wherever_it_interrupts_rules_text() {
        assert_eq!(
            strip_reminder_text(
                "You get {E} (an energy counter).\n(Reminder only.)\nThen draw a card."
            ),
            "You get {E}.\nThen draw a card."
        );
        assert_eq!(
            strip_reminder_text("Get {E} (an energy counter), then draw."),
            "Get {E}, then draw."
        );
    }

    #[test]
    fn unmatched_parentheses_are_not_discarded() {
        assert_eq!(strip_reminder_text("Choose (perhaps"), "Choose (perhaps");
    }
}
