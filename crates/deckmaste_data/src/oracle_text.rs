/// Removes parenthesized reminder text before English parsing.
///
/// Parentheses are lexical trivia in Oracle text for this phase. One ordinary
/// space immediately before a balanced group is removed with it; unmatched
/// opening parentheses are preserved.
///
/// Oracle separates the keywords of a keyword line with semicolons instead of
/// commas when one of them carries reminder text (*Vigilance; horsemanship
/// (…)*). On a line that loses reminder text and holds no sentence (no `.` or
/// `:`), each semicolon is such a separator, and it becomes a comma, the
/// separator the line takes without reminder text.
#[must_use]
pub fn strip_reminder_text(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut group = String::new();
    let mut depth = 0_usize;
    // The output lines a balanced group was removed from.
    let mut stripped = Vec::new();
    let mut line = 0_usize;

    for character in text.chars() {
        if depth == 0 {
            if character == '(' {
                depth = 1;
                group.push(character);
            } else {
                if character == '\n' {
                    line += 1;
                }
                output.push(character);
            }
        } else {
            group.push(character);
            match character {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        stripped.push(line);
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
    if stripped.is_empty() {
        return output;
    }
    output
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            if stripped.contains(&index) && !line.contains(['.', ':']) {
                line.replace(';', ",")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::strip_reminder_text;

    #[test]
    fn reminder_groups_are_removed_without_parsing_their_contents() {
        for (source, expected) in [
            ("Flying (This creature (é) flies.)", "Flying"),
            (
                "Get {E} (an energy counter), then draw.",
                "Get {E}, then draw.",
            ),
            ("Flying\n(Reminder.)\nDraw a card.", "Flying\nDraw a card."),
            ("(Reminder.)", ""),
            ("Draw (reminder) a card.", "Draw a card."),
            ("Choose (perhaps", "Choose (perhaps"),
            ("Nissa Revane attacks.", "Nissa Revane attacks."),
            ("Draw a card.\n\n", "Draw a card.\n\n"),
        ] {
            assert_eq!(strip_reminder_text(source), expected, "{source}");
        }
    }

    #[test]
    fn keyword_separator_semicolons_become_commas_with_the_reminder_text() {
        for (source, expected) in [
            (
                "Flying; trample; rampage 4 (Whenever this creature becomes blocked, \
                 it gets +4/+4 until end of turn for each creature blocking it beyond \
                 the first.)",
                "Flying, trample, rampage 4",
            ),
            // Only the line that lost reminder text is normalized.
            (
                "Flying; banding\nFirst strike; banding (Reminder.)",
                "Flying; banding\nFirst strike, banding",
            ),
            // A semicolon in a sentence is not a keyword separator.
            (
                "Draw a card; then discard. (Reminder.)",
                "Draw a card; then discard.",
            ),
            ("Flying; banding", "Flying; banding"),
        ] {
            assert_eq!(strip_reminder_text(source), expected, "{source}");
        }
    }
}
