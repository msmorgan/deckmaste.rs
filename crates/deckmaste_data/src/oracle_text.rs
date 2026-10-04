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
    let mut removed = false;

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
                        removed = true;
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
    if !removed {
        return output;
    }
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
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
}
