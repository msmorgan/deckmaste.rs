use crate::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TokenKind {
    Word,
    Integer,
    OracleSymbol,
    SymbolSequence,
    PowerToughness,
    /// A collapsed full-name self-reference. Minted by [`collapse_full_names`]
    /// when the source spells out the face's own full name, so a name that
    /// lexes to several tokens — including internal commas — is one atomic
    /// token the structural splitter never divides.
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

/// Collapses every occurrence of the face's full name into a single
/// [`TokenKind::FullSelfReference`] token, taking over what the retired `~~`
/// sigil did. A full name spells to several tokens — often with an internal
/// comma (`Aang, A Lot to Learn`) — so collapsing it keeps the structural
/// splitter from dividing a self-reference at that comma. A possessive full
/// name (`… Akros's power`) collapses the name and leaves its trailing `'s` as
/// its own word token, so the possessive self-reference rule sees the same
/// `[FullSelfReference, 's]` shape it always has.
///
/// Matching is case-sensitive and by whole-token spelling, so a shorter or
/// differently cased phrase never collapses. A full name is unique to the card
/// that bears it, so this collapse needs no ambiguity resolution; nicknames,
/// which do, are recognized in the chart instead.
pub(crate) fn collapse_full_names(
    source: &str,
    tokens: Vec<Token>,
    full_name: &[String],
) -> Vec<Token> {
    if full_name.is_empty() || tokens.len() < full_name.len() {
        return tokens;
    }
    let mut collapsed = Vec::with_capacity(tokens.len());
    let mut index = 0;
    while index < tokens.len() {
        if let Some((consumed, possessive_split)) =
            match_full_name(source, &tokens[index..], full_name)
        {
            let first = tokens[index];
            let last = tokens[index + consumed - 1];
            let name_end = possessive_split.unwrap_or(last.span.end);
            collapsed.push(token(
                TokenKind::FullSelfReference,
                first.span.start,
                name_end,
            ));
            if let Some(split) = possessive_split {
                collapsed.push(token(TokenKind::Word, split, last.span.end));
            }
            index += consumed;
        } else {
            collapsed.push(tokens[index]);
            index += 1;
        }
    }
    collapsed
}

/// Attempts to match `full_name` at the head of `tokens`. Returns the number of
/// tokens consumed and, for a possessive match, the byte offset where the name
/// ends and its trailing `'s` begins.
fn match_full_name(
    source: &str,
    tokens: &[Token],
    full_name: &[String],
) -> Option<(usize, Option<usize>)> {
    let slice = tokens.get(..full_name.len())?;
    let (last_spelling, leading) = full_name.split_last()?;
    for (token, spelling) in slice.iter().zip(leading) {
        if token.span.text(source)? != spelling {
            return None;
        }
    }
    let last = slice.last()?;
    let last_text = last.span.text(source)?;
    if last_text == last_spelling {
        Some((full_name.len(), None))
    } else if last_text
        .strip_suffix("'s")
        .is_some_and(|stem| stem == last_spelling)
    {
        Some((full_name.len(), Some(last.span.end - "'s".len())))
    } else {
        None
    }
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
    use super::collapse_full_names;
    use super::lex;
    use crate::Span;

    fn spellings<'source>(source: &'source str, tokens: &[super::Token]) -> Vec<&'source str> {
        tokens
            .iter()
            .map(|token| token.span.text(source).expect("token span is in source"))
            .collect()
    }

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
    fn full_name_collapse_makes_a_multi_token_name_one_atomic_token() {
        // A comma name collapses to one FullSelfReference token spanning the
        // whole name, so the later trigger comma is the only top-level comma
        // the structural splitter sees.
        let source = "When Aang, A Lot to Learn dies, draw a card.";
        let full_name = ["Aang", ",", "A", "Lot", "to", "Learn"].map(str::to_owned);
        let collapsed = collapse_full_names(source, lex(source).tokens, &full_name);

        assert_eq!(
            spellings(source, &collapsed),
            [
                "When",
                "Aang, A Lot to Learn",
                "dies",
                ",",
                "draw",
                "a",
                "card",
                "."
            ]
        );
        assert_eq!(collapsed[1].kind, TokenKind::FullSelfReference);
    }

    #[test]
    fn full_name_collapse_splits_a_possessive_into_name_and_apostrophe_s() {
        let source = "\"Whenever this creature attacks, draw a card.\" Return Akros's card.";
        let full_name = ["Akros"].map(str::to_owned);
        let collapsed = collapse_full_names(source, lex(source).tokens, &full_name);

        // The quoted clause is untouched; the possessive name splits its 's off.
        assert_eq!(
            spellings(source, &collapsed),
            [
                "\"", "Whenever", "this", "creature", "attacks", ",", "draw", "a", "card", ".",
                "\"", "Return", "Akros", "'s", "card", ".",
            ]
        );
        let name = collapsed
            .iter()
            .position(|token| token.kind == TokenKind::FullSelfReference)
            .expect("the possessive name collapses");
        assert_eq!(collapsed[name].span.text(source), Some("Akros"));
        assert_eq!(collapsed[name + 1].span.text(source), Some("'s"));
    }

    #[test]
    fn modal_bullets_and_brims_punctuation_are_structural_tokens() {
        assert_eq!(
            kinds("Choose one —\n• Draw a card.\n\"Brims\" Barone"),
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
