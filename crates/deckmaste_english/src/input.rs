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
