//! CR-rule coverage: scan citations, tier by binding strength, ratchet.

/// Every rule mentioned in `[CR#…]` brackets in `text`.
///
/// Bracket content is comma-separated tokens; a `A..B` token yields both
/// endpoints (ranges in this repo are always adjacent subrules, so the two
/// endpoints are the whole range — and this keeps the scan free of any
/// `cr.json` ordering dependency).
#[must_use]
pub fn extract_bracket_rules(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[CR#") {
        rest = &rest[start + 4..];
        let Some(end) = rest.find(']') else { break };
        let body = &rest[..end];
        rest = &rest[end + 1..];
        for tok in body.split(',') {
            let tok = tok.trim();
            if let Some((a, b)) = tok.split_once("..") {
                push_rule(&mut out, a);
                push_rule(&mut out, b);
            } else {
                push_rule(&mut out, tok);
            }
        }
    }
    out
}

fn push_rule(out: &mut Vec<String>, s: &str) {
    let s = s.trim();
    if !s.is_empty() {
        out.push(s.to_string());
    }
}

/// Every rule in `#[cr("…", "…")]` attributes in `text`.
#[must_use]
pub fn extract_attr_rules(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("#[cr(") {
        rest = &rest[start + 5..];
        let Some(end) = rest.find(')') else { break };
        let body = &rest[..end];
        rest = &rest[end + 1..];
        let mut chars = body.char_indices();
        while let Some((i, c)) = chars.next() {
            if c == '"'
                && let Some(close) = body[i + 1..].find('"')
            {
                push_rule(&mut out, &body[i + 1..i + 1 + close]);
                // advance past the closing quote
                for _ in 0..=close {
                    chars.next();
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_single_and_list() {
        assert_eq!(extract_bracket_rules("foo [CR#714] bar"), vec!["714"]);
        assert_eq!(
            extract_bracket_rules("[CR#701.12a,701.12b]"),
            vec!["701.12a", "701.12b"]
        );
        assert_eq!(
            extract_bracket_rules("x [CR#601.2g,106.4] y"),
            vec!["601.2g", "106.4"]
        );
    }

    #[test]
    fn brackets_range_yields_endpoints() {
        assert_eq!(
            extract_bracket_rules("[CR#601.2a..601.2b]"),
            vec!["601.2a", "601.2b"]
        );
    }

    #[test]
    fn brackets_multiple_and_none() {
        assert_eq!(
            extract_bracket_rules("[CR#100.1] then [CR#200.2]"),
            vec!["100.1", "200.2"]
        );
        assert!(extract_bracket_rules("no citations here").is_empty());
    }

    #[test]
    fn attrs_extract_quoted_rules() {
        assert_eq!(
            extract_attr_rules(r#"#[cr("704.5f", "704.7")]"#),
            vec!["704.5f", "704.7"]
        );
        assert_eq!(extract_attr_rules(r#"#[cr( "305.2" )]"#), vec!["305.2"]);
        assert!(extract_attr_rules("#[test]").is_empty());
    }
}
