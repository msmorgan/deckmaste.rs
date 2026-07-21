/// Replaces whole-word references to a card face with self-reference sigils.
///
/// A legendary face with a comma uses `~` for its abbreviated, pre-comma name
/// and `~~` for its full name. Every other face uses `~` for its full name.
/// Matching is case-sensitive, like printed Oracle self-references.
#[must_use]
pub fn normalize_self_references(text: &str, face_name: &str, is_legendary: bool) -> String {
    let Some((short_name, _)) = is_legendary.then(|| face_name.split_once(',')).flatten() else {
        return replace_whole_word(text, face_name, "~");
    };

    let full_name_replaced = replace_whole_word(text, face_name, "~~");
    replace_whole_word(&full_name_replaced, short_name.trim(), "~")
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

fn replace_whole_word(text: &str, name: &str, replacement: &str) -> String {
    if name.is_empty() || !text.contains(name) {
        return text.to_owned();
    }

    let starts_with_word = name.chars().next().is_some_and(is_word_character);
    let ends_with_word = name.chars().next_back().is_some_and(is_word_character);
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;

    for (start, matched) in text.match_indices(name) {
        let end = start + matched.len();
        let valid_start = !starts_with_word
            || text[..start]
                .chars()
                .next_back()
                .is_none_or(|character| !is_word_character(character));
        let valid_end = !ends_with_word
            || text[end..]
                .chars()
                .next()
                .is_none_or(|character| !is_word_character(character));
        if valid_start && valid_end {
            output.push_str(&text[cursor..start]);
            output.push_str(replacement);
            cursor = end;
        }
    }

    if cursor == 0 {
        text.to_owned()
    } else {
        output.push_str(&text[cursor..]);
        output
    }
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_cards_use_one_tilde_for_their_full_name() {
        assert_eq!(
            normalize_self_references(
                "Exile Long Rest. Return Long Rest to its owner's hand.",
                "Long Rest",
                false,
            ),
            "Exile ~. Return ~ to its owner's hand."
        );
    }

    #[test]
    fn comma_legends_distinguish_abbreviated_and_full_names() {
        assert_eq!(
            normalize_self_references(
                "Aang attacks. Aang, A Lot to Learn's power is 3.",
                "Aang, A Lot to Learn",
                true,
            ),
            "~ attacks. ~~'s power is 3."
        );
    }

    #[test]
    fn short_name_replacement_respects_word_boundaries() {
        assert_eq!(
            normalize_self_references(
                "Gut attacks. The guts spill.",
                "Gut, True Soul Zealot",
                true,
            ),
            "~ attacks. The guts spill."
        );
    }

    #[test]
    fn legendary_names_without_commas_use_one_tilde() {
        assert_eq!(
            normalize_self_references("Aang and Katara attack.", "Aang and Katara", true),
            "~ attack."
        );
    }

    #[test]
    fn names_beginning_with_punctuation_are_replaced() {
        assert_eq!(
            normalize_self_references(
                "\"Brims\" Barone, Midway Mobster enters.",
                "\"Brims\" Barone, Midway Mobster",
                true,
            ),
            "~~ enters."
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
