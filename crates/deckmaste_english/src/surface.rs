use crate::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TokenKind {
    Word,
    Integer,
    OracleSymbol,
    SymbolSequence,
    PowerToughness,
    SelfReference,
    FullSelfReference,
    Bullet,
    Punctuation(Punctuation),
    Newline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Punctuation {
    Comma,
    Colon,
    Semicolon,
    Period,
    Exclamation,
    Question,
    Apostrophe,
    DoubleQuote,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Slash,
    Hyphen,
    EnDash,
    EmDash,
    Other(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceDiagnosticKind {
    UnclosedOracleSymbol,
    UnexpectedClosingOracleSymbol,
    UnclosedBracket,
    UnexpectedClosingBracket,
    UnclosedParenthesis,
    UnexpectedClosingParenthesis,
    UnterminatedDoubleQuote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct SurfaceDiagnostic {
    pub(crate) kind: SurfaceDiagnosticKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Surface {
    pub(crate) tokens: Vec<Token>,
    pub(crate) diagnostics: Vec<SurfaceDiagnostic>,
}

pub(crate) fn lex(source: &str) -> Surface {
    let mut surface = Surface::default();
    let mut brackets = Vec::new();
    let mut parentheses = Vec::new();
    let mut double_quote = None;
    let mut index = 0;

    while index < source.len() {
        let ch = next_char(source, index);
        let width = ch.len_utf8();
        if ch == '\n' {
            surface
                .tokens
                .push(token(TokenKind::Newline, index, index + width));
            index += width;
            continue;
        }
        if ch.is_whitespace() {
            index += width;
            continue;
        }
        if ch == '{' {
            index = scan_symbols(source, index, &mut surface);
            continue;
        }
        if let Some(end) = power_toughness_end(source, index) {
            surface
                .tokens
                .push(token(TokenKind::PowerToughness, index, end));
            index = end;
            continue;
        }
        if ch == '~' {
            let doubled = source[index + width..].starts_with('~');
            let end = index + width * if doubled { 2 } else { 1 };
            surface.tokens.push(token(
                if doubled { TokenKind::FullSelfReference } else { TokenKind::SelfReference },
                index,
                end,
            ));
            index = end;
            continue;
        }
        if is_word_start(source, index, ch) {
            let end = word_end(source, index);
            let text = &source[index..end];
            surface.tokens.push(token(
                if text.chars().all(|character| character.is_ascii_digit()) {
                    TokenKind::Integer
                } else {
                    TokenKind::Word
                },
                index,
                end,
            ));
            index = end;
            continue;
        }

        let kind = match ch {
            '•' => TokenKind::Bullet,
            _ => TokenKind::Punctuation(punctuation(ch)),
        };
        surface.tokens.push(token(kind, index, index + width));
        observe_delimiter(
            ch,
            index,
            &mut brackets,
            &mut parentheses,
            &mut double_quote,
            &mut surface.diagnostics,
        );
        index += width;
    }

    for start in brackets {
        surface.diagnostics.push(diagnostic(
            SurfaceDiagnosticKind::UnclosedBracket,
            start,
            start + 1,
        ));
    }
    for start in parentheses {
        surface.diagnostics.push(diagnostic(
            SurfaceDiagnosticKind::UnclosedParenthesis,
            start,
            start + 1,
        ));
    }
    if let Some(start) = double_quote {
        surface.diagnostics.push(diagnostic(
            SurfaceDiagnosticKind::UnterminatedDoubleQuote,
            start,
            start + 1,
        ));
    }
    surface
}

fn scan_symbols(source: &str, start: usize, surface: &mut Surface) -> usize {
    let mut end = start;
    let mut count = 0;
    loop {
        let symbol_start = end;
        let Some(relative_close) = source[symbol_start + 1..].find('}') else {
            surface.diagnostics.push(diagnostic(
                SurfaceDiagnosticKind::UnclosedOracleSymbol,
                symbol_start,
                symbol_start + 1,
            ));
            end = source.len();
            count += 1;
            break;
        };
        end = symbol_start + relative_close + 2;
        count += 1;
        if !source[end..].starts_with('{') {
            break;
        }
    }
    surface.tokens.push(token(
        if count == 1 { TokenKind::OracleSymbol } else { TokenKind::SymbolSequence },
        start,
        end,
    ));
    end
}

fn power_toughness_end(source: &str, start: usize) -> Option<usize> {
    let first_end = scalar_end(source, start)?;
    let slash = source[first_end..].chars().next()?;
    if slash != '/' {
        return None;
    }
    let second_start = first_end + slash.len_utf8();
    let second_end = scalar_end(source, second_start)?;
    source[second_end..]
        .chars()
        .next()
        .is_none_or(|next| !is_scalar_body(next))
        .then_some(second_end)
}

fn scalar_end(source: &str, start: usize) -> Option<usize> {
    let mut index = start;
    if matches!(source[index..].chars().next()?, '+' | '-' | '−') {
        index += next_char(source, index).len_utf8();
    }
    let body_start = index;
    while index < source.len() {
        let ch = next_char(source, index);
        if !is_scalar_body(ch) {
            break;
        }
        index += ch.len_utf8();
    }
    (index > body_start).then_some(index)
}

const fn is_scalar_body(ch: char) -> bool {
    ch.is_ascii_digit() || matches!(ch, 'X' | 'Y' | '*')
}

fn is_word_start(source: &str, index: usize, ch: char) -> bool {
    ch.is_alphanumeric()
        || (is_apostrophe(ch)
            && source[index + ch.len_utf8()..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric))
}

fn word_end(source: &str, start: usize) -> usize {
    let mut index = start;
    while index < source.len() {
        let ch = next_char(source, index);
        if ch.is_alphanumeric() {
            index += ch.len_utf8();
            continue;
        }
        if is_word_connector(ch)
            && source[index + ch.len_utf8()..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric)
        {
            index += ch.len_utf8();
            continue;
        }
        break;
    }
    index
}

const fn is_apostrophe(ch: char) -> bool {
    ch == '\''
}

const fn is_word_connector(ch: char) -> bool {
    is_apostrophe(ch) || matches!(ch, '-' | '/')
}

const fn punctuation(ch: char) -> Punctuation {
    match ch {
        ',' => Punctuation::Comma,
        ':' => Punctuation::Colon,
        ';' => Punctuation::Semicolon,
        '.' => Punctuation::Period,
        '!' => Punctuation::Exclamation,
        '?' => Punctuation::Question,
        '\'' => Punctuation::Apostrophe,
        '"' => Punctuation::DoubleQuote,
        '[' => Punctuation::OpenBracket,
        ']' => Punctuation::CloseBracket,
        '(' => Punctuation::OpenParenthesis,
        ')' => Punctuation::CloseParenthesis,
        '+' => Punctuation::Plus,
        '-' => Punctuation::Hyphen,
        '−' => Punctuation::Minus,
        '/' => Punctuation::Slash,
        '–' => Punctuation::EnDash,
        '—' => Punctuation::EmDash,
        _ => Punctuation::Other(ch),
    }
}

fn observe_delimiter(
    ch: char,
    index: usize,
    brackets: &mut Vec<usize>,
    parentheses: &mut Vec<usize>,
    double_quote: &mut Option<usize>,
    diagnostics: &mut Vec<SurfaceDiagnostic>,
) {
    match ch {
        '[' => brackets.push(index),
        ']' if brackets.pop().is_none() => diagnostics.push(diagnostic(
            SurfaceDiagnosticKind::UnexpectedClosingBracket,
            index,
            index + 1,
        )),
        '(' => parentheses.push(index),
        ')' if parentheses.pop().is_none() => diagnostics.push(diagnostic(
            SurfaceDiagnosticKind::UnexpectedClosingParenthesis,
            index,
            index + 1,
        )),
        '}' => diagnostics.push(diagnostic(
            SurfaceDiagnosticKind::UnexpectedClosingOracleSymbol,
            index,
            index + 1,
        )),
        '"' => {
            if double_quote.is_some() {
                *double_quote = None;
            } else {
                *double_quote = Some(index);
            }
        }
        _ => {}
    }
}

const fn token(kind: TokenKind, start: usize, end: usize) -> Token {
    Token {
        kind,
        span: Span::new(start, end),
    }
}

const fn diagnostic(kind: SurfaceDiagnosticKind, start: usize, end: usize) -> SurfaceDiagnostic {
    SurfaceDiagnostic {
        kind,
        span: Span::new(start, end),
    }
}

fn next_char(source: &str, index: usize) -> char {
    source[index..]
        .chars()
        .next()
        .expect("tokenizer index is on a character boundary")
}

#[cfg(test)]
mod tests {
    use super::Punctuation;
    use super::SurfaceDiagnosticKind;
    use super::TokenKind;
    use super::lex;
    use crate::Span;

    fn kinds(source: &str) -> Vec<TokenKind> {
        lex(source)
            .tokens
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    #[test]
    fn magic_atoms_are_not_split_into_characters() {
        assert_eq!(
            kinds("Other Goblin creatures you control get +1/+1 and have haste."),
            vec![
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::PowerToughness,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::Period),
            ]
        );
        assert_eq!(
            kinds("{W}{U}{B}{R}{G} {C}"),
            vec![TokenKind::SymbolSequence, TokenKind::OracleSymbol]
        );
    }

    #[test]
    fn contractions_possessives_and_lexical_connectors_stay_words() {
        assert_eq!(
            kinds("can't doesn't owner's player's non-Human and/or"),
            vec![TokenKind::Word; 6]
        );
    }

    #[test]
    fn self_references_and_quotes_keep_exact_boundaries() {
        let source =
            "~ attacks. ~~'s power is 3.\n\"Whenever this creature attacks, draw a card.\"";
        let surface = lex(source);

        assert_eq!(
            surface
                .tokens
                .iter()
                .map(|token| token.kind)
                .collect::<Vec<_>>(),
            vec![
                TokenKind::SelfReference,
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::Period),
                TokenKind::FullSelfReference,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Integer,
                TokenKind::Punctuation(Punctuation::Period),
                TokenKind::Newline,
                TokenKind::Punctuation(Punctuation::DoubleQuote),
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::Period),
                TokenKind::Punctuation(Punctuation::DoubleQuote),
            ]
        );
        assert_eq!(
            surface
                .tokens
                .iter()
                .map(|token| token.span.text(source).expect("token span is in source"))
                .collect::<Vec<_>>(),
            vec![
                "~", "attacks", ".", "~~", "'s", "power", "is", "3", ".", "\n", "\"", "Whenever",
                "this", "creature", "attacks", ",", "draw", "a", "card", ".", "\"",
            ]
        );
    }

    #[test]
    fn modal_bullets_and_brims_punctuation_are_structural_tokens() {
        assert_eq!(
            kinds("Choose one —\n• Draw a card.\n~~: Add {C}.\n~~, \"Brims\" Barone"),
            vec![
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::EmDash),
                TokenKind::Newline,
                TokenKind::Bullet,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::Period),
                TokenKind::Newline,
                TokenKind::FullSelfReference,
                TokenKind::Punctuation(Punctuation::Colon),
                TokenKind::Word,
                TokenKind::OracleSymbol,
                TokenKind::Punctuation(Punctuation::Period),
                TokenKind::Newline,
                TokenKind::FullSelfReference,
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Punctuation(Punctuation::DoubleQuote),
                TokenKind::Word,
                TokenKind::Punctuation(Punctuation::DoubleQuote),
                TokenKind::Word,
            ]
        );
    }

    #[test]
    fn malformed_delimiters_report_spans_without_copying_source() {
        assert_eq!(
            lex("{W").diagnostics[0],
            super::SurfaceDiagnostic {
                kind: SurfaceDiagnosticKind::UnclosedOracleSymbol,
                span: Span::new(0, 1),
            }
        );
        assert_eq!(
            lex("]").diagnostics[0],
            super::SurfaceDiagnostic {
                kind: SurfaceDiagnosticKind::UnexpectedClosingBracket,
                span: Span::new(0, 1),
            }
        );
        assert_eq!(
            lex("[x").diagnostics[0],
            super::SurfaceDiagnostic {
                kind: SurfaceDiagnosticKind::UnclosedBracket,
                span: Span::new(0, 1),
            }
        );
        assert_eq!(
            lex("\"open").diagnostics[0],
            super::SurfaceDiagnostic {
                kind: SurfaceDiagnosticKind::UnterminatedDoubleQuote,
                span: Span::new(0, 1),
            }
        );
    }
}
