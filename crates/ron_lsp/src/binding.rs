//! Lexical binder resolution for deckmaste's `It` anaphor.

use serde_json::Value;
use serde_json::json;

use crate::source::Position;
use crate::source::offset_at;
use crate::source::position_at;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn contains(self, offset: usize) -> bool {
        self.start <= offset && offset <= self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind<'a> {
    Ident(&'a str),
    Open,
    Close,
    Comma,
    Colon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Token<'a> {
    kind: TokenKind<'a>,
    span: Span,
}

#[derive(Debug, PartialEq, Eq)]
struct Binding {
    source: Span,
    scope: Span,
    uses: Vec<Span>,
}

pub fn document_highlights(text: &str, position: Position) -> Vec<Value> {
    let Some(offset) = offset_at(text, position) else {
        return Vec::new();
    };
    let bindings = bindings(text);
    let selected = bindings.iter().find(|binding| {
        binding.source.contains(offset) || binding.uses.iter().any(|span| span.contains(offset))
    });
    let Some(binding) = selected else {
        return Vec::new();
    };

    std::iter::once((binding.source, 3))
        .chain(binding.uses.iter().copied().map(|span| (span, 2)))
        .map(|(span, kind)| highlight(text, span, kind))
        .collect()
}

fn highlight(text: &str, span: Span, kind: u8) -> Value {
    let start = position_at(text, span.start);
    let end = position_at(text, span.end);
    json!({
        "range": {
            "start": { "line": start.line, "character": start.character },
            "end": { "line": end.line, "character": end.character }
        },
        "kind": kind
    })
}

fn bindings(text: &str) -> Vec<Binding> {
    let tokens = tokens(text);
    let pairs = paren_pairs(&tokens);
    let mut bindings = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        let TokenKind::Ident(name) = token.kind else {
            continue;
        };
        let Some((field, positional)) = binder_scope(name) else {
            continue;
        };
        if !matches!(
            tokens.get(index + 1).map(|token| token.kind),
            Some(TokenKind::Open)
        ) {
            continue;
        }
        let open = index + 1;
        let Some(close) = pairs[open] else { continue };
        if let Some(scope) = argument_span(&tokens, open, close, field, positional) {
            bindings.push(Binding {
                source: token.span,
                scope,
                uses: Vec::new(),
            });
        }
    }

    for token in &tokens {
        if token.kind != TokenKind::Ident("It") {
            continue;
        }
        let owner = bindings
            .iter()
            .enumerate()
            .filter(|(_, binding)| binding.scope.contains(token.span.start))
            .min_by_key(|(_, binding)| binding.scope.end - binding.scope.start)
            .map(|(index, _)| index);
        if let Some(owner) = owner {
            bindings[owner].uses.push(token.span);
        }
    }
    bindings
}

fn binder_scope(name: &str) -> Option<(Option<&'static str>, usize)> {
    match name {
        "Each" => Some((Some("effect"), 1)),
        "Distribute" | "RevealUntil" => Some((Some("body"), 2)),
        "Where" => Some((None, 0)),
        _ => None,
    }
}

fn argument_span(
    tokens: &[Token<'_>],
    open: usize,
    close: usize,
    field: Option<&str>,
    positional: usize,
) -> Option<Span> {
    let arguments = arguments(tokens, open, close);
    if let Some(field) = field {
        for &(start, end) in &arguments {
            if let (
                Some(Token {
                    kind: TokenKind::Ident(name),
                    ..
                }),
                Some(colon),
            ) = (tokens.get(start), tokens.get(start + 1))
                && *name == field
                && colon.kind == TokenKind::Colon
            {
                return value_span(tokens, start + 2, end);
            }
        }
    }
    let &(start, end) = arguments.get(positional)?;
    let value_start = if tokens
        .get(start + 1)
        .is_some_and(|token| token.kind == TokenKind::Colon)
    {
        start + 2
    } else {
        start
    };
    value_span(tokens, value_start, end)
}

fn value_span(tokens: &[Token<'_>], start: usize, end: usize) -> Option<Span> {
    Some(Span {
        start: tokens.get(start)?.span.start,
        end: tokens.get(end.checked_sub(1)?)?.span.end,
    })
}

fn arguments(tokens: &[Token<'_>], open: usize, close: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut start = open + 1;
    let mut depth = 0;
    for (index, token) in tokens.iter().enumerate().take(close).skip(open + 1) {
        match token.kind {
            TokenKind::Open => depth += 1,
            TokenKind::Close => depth -= 1,
            TokenKind::Comma if depth == 0 => {
                if start < index {
                    result.push((start, index));
                }
                start = index + 1;
            }
            _ => {}
        }
    }
    if start < close {
        result.push((start, close));
    }
    result
}

fn paren_pairs(tokens: &[Token<'_>]) -> Vec<Option<usize>> {
    let mut pairs = vec![None; tokens.len()];
    let mut stack = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::Open => stack.push(index),
            TokenKind::Close => {
                if let Some(open) = stack.pop() {
                    pairs[open] = Some(index);
                    pairs[index] = Some(open);
                }
            }
            _ => {}
        }
    }
    pairs
}

fn tokens(text: &str) -> Vec<Token<'_>> {
    let mut result = Vec::new();
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
        let kind = match ch {
            '(' => Some(TokenKind::Open),
            ')' => Some(TokenKind::Close),
            ',' => Some(TokenKind::Comma),
            ':' => Some(TokenKind::Colon),
            _ => None,
        };
        if let Some(kind) = kind {
            result.push(Token {
                kind,
                span: Span {
                    start: offset,
                    end: offset + ch.len_utf8(),
                },
            });
        } else if ch == '_' || ch.is_alphabetic() {
            let mut end = offset + ch.len_utf8();
            while let Some(&(next_offset, next)) = chars.peek() {
                if next != '_' && !next.is_alphanumeric() {
                    break;
                }
                chars.next();
                end = next_offset + next.len_utf8();
            }
            result.push(Token {
                kind: TokenKind::Ident(&text[offset..end]),
                span: Span { start: offset, end },
            });
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups(text: &str) -> Vec<(String, Vec<String>)> {
        bindings(text)
            .into_iter()
            .map(|binding| {
                (
                    text[binding.source.start..binding.source.end].to_owned(),
                    binding
                        .uses
                        .into_iter()
                        .map(|span| text[span.start..span.end].to_owned())
                        .collect(),
                )
            })
            .collect()
    }

    #[test]
    fn each_binds_only_in_effect() {
        assert_eq!(
            groups("Each(binder: Existing(It), effect: Move(It, Hand))"),
            [("Each".into(), vec!["It".into()])]
        );
    }

    #[test]
    fn nested_each_shadows_outer_it() {
        let found = bindings(
            "Each(X, Several([Move(It, Hand), Each(Y, Move(It, Exile)), Move(It, Graveyard)]))",
        );
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].uses.len(), 2);
        assert_eq!(found[1].uses.len(), 1);
    }

    #[test]
    fn supports_other_it_binders_and_named_fields() {
        let found = groups(
            "Several([Where(Matches(It, X)), Distribute(amount: 3, binder: X, body: Move(It, Hand)), RevealUntil(whose: You, matches: X, body: Move(It, Exile))])",
        );
        assert_eq!(found.len(), 3);
        assert!(found.iter().all(|(_, uses)| uses == &["It"]));
    }
}
