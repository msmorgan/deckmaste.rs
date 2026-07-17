//! Conversions between the crate's internal source positions and `lsp-types`.

use std::path::Path;

use lsp_types::Position as LspPosition;
use lsp_types::Range as LspRange;
use lsp_types::Uri;

use crate::source::Position;

/// `file://` URI for `path`. `None` if it can't parse.
///
/// Card filenames carry spaces, commas, apostrophes and non-ASCII letters
/// (`Andúril, Flame of the West.ron.todo`), none of which are legal raw in a
/// URI — so percent-encode every byte outside the unreserved set. Escaping only
/// spaces leaves the ~76 non-ASCII cards unparseable, silently dropping them
/// from every result.
pub fn to_uri(path: &Path) -> Option<Uri> {
    format!("file://{}", percent_encode(&path.to_string_lossy()))
        .parse::<Uri>()
        .ok()
}

fn percent_encode(path: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(path.len());
    for &byte in path.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                encoded.push(char::from(byte));
            }
            _ => {
                encoded.push('%');
                encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                encoded.push(char::from(HEX[usize::from(byte & 0x0F)]));
            }
        }
    }
    encoded
}

pub fn to_lsp_range(start: Position, end: Position) -> LspRange {
    LspRange {
        start: LspPosition {
            line: start.line,
            character: start.character,
        },
        end: LspPosition {
            line: end.line,
            character: end.character,
        },
    }
}

pub fn src_position(position: LspPosition) -> Position {
    Position {
        line: position.line,
        character: position.character,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_escaped_file_uri() {
        let uri = to_uri(Path::new("/a b/c.ron")).unwrap();
        assert_eq!(uri.as_str(), "file:///a%20b/c.ron");
    }

    #[test]
    fn encodes_non_ascii_and_punctuated_card_paths() {
        // A real corpus filename: without encoding this never parses, and the
        // card silently vanishes from every LSP result.
        let uri = to_uri(Path::new("/p/Andúril, Flame of the West.ron.todo"))
            .expect("non-ASCII path must still yield a URI");
        assert_eq!(
            uri.as_str(),
            "file:///p/And%C3%BAril%2C%20Flame%20of%20the%20West.ron.todo"
        );
    }
}
