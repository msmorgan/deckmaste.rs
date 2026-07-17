#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

pub fn offset_at(text: &str, position: Position) -> Option<usize> {
    let line = text.split_inclusive('\n').nth(position.line as usize)?;
    let line_start = line.as_ptr() as usize - text.as_ptr() as usize;
    let mut utf16 = 0_u32;
    for (offset, ch) in line.char_indices() {
        if utf16 == position.character {
            return Some(line_start + offset);
        }
        utf16 += u32::try_from(ch.len_utf16()).expect("a char is at most two UTF-16 units");
        if utf16 > position.character {
            return None;
        }
    }
    (utf16 == position.character).then_some(line_start + line.len())
}

pub fn position_at(text: &str, offset: usize) -> Position {
    let before = &text[..offset];
    let line =
        u32::try_from(before.bytes().filter(|byte| *byte == b'\n').count()).unwrap_or(u32::MAX);
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    let character =
        u32::try_from(text[line_start..offset].encode_utf16().count()).unwrap_or(u32::MAX);
    Position { line, character }
}

pub fn word_at(text: &str, position: Position) -> Option<&str> {
    let offset = offset_at(text, position)?;
    let mut start = offset;
    while start > 0 {
        let ch = text[..start].chars().next_back()?;
        if !is_ident_continue(ch) {
            break;
        }
        start -= ch.len_utf8();
    }
    let mut end = offset;
    while end < text.len() {
        let ch = text[end..].chars().next()?;
        if !is_ident_continue(ch) {
            break;
        }
        end += ch.len_utf8();
    }
    (start < end).then_some(&text[start..end])
}

pub fn identifiers(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let mut found = Vec::new();
    let mut chars = text.char_indices().peekable();
    let mut in_string = false;
    while let Some((offset, ch)) = chars.next() {
        if in_string {
            if ch == '\\' {
                chars.next();
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
            continue;
        }
        if ch == '/' && chars.peek().is_some_and(|(_, next)| *next == '/') {
            chars.next();
            while chars.next().is_some_and(|(_, next)| next != '\n') {}
            continue;
        }
        if is_ident_start(ch) {
            let mut end = offset + ch.len_utf8();
            while let Some(&(next_offset, next)) = chars.peek() {
                if !is_ident_continue(next) {
                    break;
                }
                chars.next();
                end = next_offset + next.len_utf8();
            }
            found.push((offset, &text[offset..end]));
        }
    }
    found.into_iter()
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}
fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

/// First `field: "…"` string value, ignoring strings and comments.
pub fn string_field<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    for (offset, token) in identifiers(text) {
        if token != field {
            continue;
        }
        let tail = text.get(offset + token.len()..)?;
        let quote = tail.find('"')?;
        let rest = &tail[quote + 1..];
        let end = rest.find('"')?;
        return Some(&rest[..end]);
    }
    None
}

/// First `field: [ … ]` bracketed value (inclusive of the brackets).
pub fn bracket_field<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    for (offset, token) in identifiers(text) {
        if token != field {
            continue;
        }
        let tail = text.get(offset + token.len()..)?;
        let open = tail.find('[')?;
        let mut depth = 0_usize;
        for (i, ch) in tail[open..].char_indices() {
            match ch {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&tail[open..=open + i]);
                    }
                }
                _ => {}
            }
        }
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_word_using_utf16_position() {
        let text = "// 🃏\nAnyTarget";
        assert_eq!(
            word_at(
                text,
                Position {
                    line: 1,
                    character: 3
                }
            ),
            Some("AnyTarget")
        );
    }

    #[test]
    fn identifiers_skip_comments_and_strings() {
        let names: Vec<_> = identifiers("// Nope\nReal(\"AlsoNope\")")
            .map(|(_, name)| name)
            .collect();
        assert_eq!(names, ["Real"]);
    }
}
