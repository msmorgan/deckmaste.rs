//! `textDocument/documentSymbol`: a lightweight outline of an open card file —
//! the card's `name` as the root, one child per top-level ability entry.

use lsp_types::DocumentSymbol;
use lsp_types::SymbolKind;

use crate::convert::to_lsp_range;
use crate::source::identifiers;
use crate::source::position_at;
use crate::source::string_field;

#[expect(
    deprecated,
    reason = "DocumentSymbol::deprecated is a required field with no non-deprecated form"
)]
pub fn document_symbols(text: &str) -> Vec<DocumentSymbol> {
    let Some(name) = string_field(text, "name") else {
        return Vec::new();
    };
    let full = to_lsp_range(position_at(text, 0), position_at(text, text.len()));
    vec![DocumentSymbol {
        name: name.to_owned(),
        detail: None,
        kind: SymbolKind::STRUCT,
        tags: None,
        deprecated: None,
        range: full,
        selection_range: full,
        children: Some(ability_children(text)),
    }]
}

#[expect(
    deprecated,
    reason = "DocumentSymbol::deprecated is a required field with no non-deprecated form"
)]
fn ability_children(text: &str) -> Vec<DocumentSymbol> {
    let Some((start, end)) = abilities_span(text) else {
        return Vec::new();
    };
    let mut children = Vec::new();
    for (relative_start, relative_end) in top_level_elements(&text[start..end]) {
        let element_start = start + relative_start;
        let element_end = start + relative_end;
        let element = text[element_start..element_end].trim();
        if element.is_empty() {
            continue;
        }
        let label = identifiers(element)
            .next()
            .map_or("element", |(_, name)| name);
        let range = to_lsp_range(
            position_at(text, element_start),
            position_at(text, element_end),
        );
        children.push(DocumentSymbol {
            name: label.to_owned(),
            detail: None,
            kind: SymbolKind::FIELD,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        });
    }
    children
}

/// Byte offsets of the content between the `abilities: [` and its matching `]`.
fn abilities_span(text: &str) -> Option<(usize, usize)> {
    for (offset, token) in identifiers(text) {
        if token != "abilities" {
            continue;
        }
        let after = offset + token.len();
        let open = after + text[after..].find('[')?;
        let mut depth = 0_usize;
        for (i, ch) in text[open..].char_indices() {
            match ch {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((open + 1, open + i));
                    }
                }
                _ => {}
            }
        }
        return None;
    }
    None
}

/// Spans of top-level (depth-0) comma-separated elements within `content`,
/// tracking `()`/`[]`/`{}` nesting and skipping string literals.
fn top_level_elements(content: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut depth = 0_i32;
    let mut start = 0;
    let mut in_string = false;
    let mut escaped = false;
    for (i, ch) in content.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                result.push((start, i));
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < content.len() {
        result.push((start, content.len()));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outlines_card_name_and_abilities() {
        let text = "Normal(\n  name: \"Quillspike\",\n  abilities: [\n    Unparsed(\"a\"),\n    Static(Flying),\n  ],\n)\n";
        let symbols = document_symbols(text);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "Quillspike");
        let children = symbols[0].children.as_ref().unwrap();
        let names: Vec<_> = children.iter().map(|child| child.name.as_str()).collect();
        assert_eq!(names, ["Unparsed", "Static"]);
    }
}
