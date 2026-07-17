//! Conversions between the crate's internal source positions and `lsp-types`.

use std::path::Path;

use lsp_types::Position as LspPosition;
use lsp_types::Range as LspRange;
use lsp_types::Uri;

use crate::source::Position;

/// `file://` URI for `path`, spaces percent-escaped. `None` if it can't parse.
pub fn to_uri(path: &Path) -> Option<Uri> {
    let escaped = path.to_string_lossy().replace(' ', "%20");
    format!("file://{escaped}").parse::<Uri>().ok()
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
}
