use crate::Ability;
use crate::AbilityKind;
use crate::ActivatedAbility;
use crate::Catalogs;
use crate::Clause;
use crate::ConditionalClause;
use crate::ConditionalPosition;
use crate::Cost;
use crate::Diagnostic;
use crate::DiagnosticKind;
use crate::KeywordAbility;
use crate::KeywordAbilityList;
use crate::LoyaltyAbility;
use crate::ModalAbility;
use crate::ModalFrame;
use crate::Mode;
use crate::OracleText;
use crate::Paragraph;
use crate::Predicate;
use crate::Sentence;
use crate::SimpleClause;
use crate::Span;
use crate::Subordinator;
use crate::Token;
use crate::TokenKind;
use crate::TriggerWord;
use crate::TriggeredAbility;
use crate::VerbKind;

/// Parses one face's Oracle text without performing rules-semantic lowering.
#[must_use]
pub fn parse(source: &str) -> OracleText {
    let (tokens, diagnostics) = lex(source);
    Parser {
        source,
        catalogs: None,
        tokens,
        diagnostics,
    }
    .oracle_text()
}

/// Parses one face's Oracle text using the supplied Scryfall English catalogs
/// to recognize keyword abilities, keyword actions, and ability words.
#[must_use]
pub fn parse_with_catalogs(source: &str, catalogs: &Catalogs) -> OracleText {
    let (tokens, diagnostics) = lex(source);
    Parser {
        source,
        catalogs: Some(catalogs),
        tokens,
        diagnostics,
    }
    .oracle_text()
}

struct Parser<'source, 'catalog> {
    source: &'source str,
    catalogs: Option<&'catalog Catalogs>,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl Parser<'_, '_> {
    fn oracle_text(mut self) -> OracleText {
        let lines = line_spans(self.source);
        let mut abilities = Vec::new();
        let mut line = 0;
        while line < lines.len() {
            let span = lines[line];
            if let Some(header_span) = modal_header_span(self.source, span)
                && lines
                    .get(line + 1)
                    .is_some_and(|next| mode_body(self.source, *next).is_some())
            {
                let (ability_word, body) = split_ability_word(self.source, span, self.catalogs);
                let frame = self.modal_frame(body, header_span);
                let header = self.paragraph(header_span);
                let mut modes = Vec::new();
                let mut end = span.end;
                line += 1;
                while let Some(mode_span) = lines.get(line).copied() {
                    let Some((bullet, body_span)) = mode_body(self.source, mode_span) else {
                        break;
                    };
                    end = mode_span.end;
                    modes.push(Mode {
                        span: mode_span,
                        bullet,
                        body: self.paragraph(body_span),
                    });
                    line += 1;
                }
                abilities.push(Ability {
                    span: Span::new(span.start, end),
                    ability_word,
                    kind: AbilityKind::Modal(ModalAbility {
                        frame,
                        header,
                        modes,
                    }),
                });
                continue;
            }
            abilities.push(self.ability(span));
            line += 1;
        }
        OracleText {
            span: Span::new(0, self.source.len()),
            tokens: self.tokens,
            abilities,
            diagnostics: self.diagnostics,
        }
    }

    fn modal_frame(&self, body: Span, header: Span) -> ModalFrame {
        if let Some((cost, effect)) = loyalty_parts(self.source, body)
            && effect == header
        {
            return ModalFrame::Loyalty(cost);
        }
        if let Some((introducer, introducer_span, rest)) = trigger_start(body, self.text(body))
            && let Some(comma) = find_top_level(self.source, rest, ", ")
        {
            let event = trim_span(self.source, Span::new(rest.start, comma));
            let effect = trim_span(self.source, Span::new(comma + 2, rest.end));
            if effect == header {
                return ModalFrame::Triggered {
                    introducer,
                    introducer_span,
                    event: self.simple_clause(event),
                };
            }
        }
        if let Some(colon) = find_top_level(self.source, body, ":") {
            let cost_span = trim_span(self.source, Span::new(body.start, colon));
            let effect_span = trim_span(self.source, Span::new(colon + 1, body.end));
            if effect_span == header {
                return ModalFrame::Activated(Cost {
                    span: cost_span,
                    components: split_top_level(self.source, cost_span, ", "),
                });
            }
        }
        ModalFrame::Unframed
    }

    fn ability(&mut self, span: Span) -> Ability {
        let (ability_word, body) = split_ability_word(self.source, span, self.catalogs);
        let text = self.text(body);
        let kind = if let Some((cost, effect)) = loyalty_parts(self.source, body) {
            AbilityKind::Loyalty(LoyaltyAbility {
                cost,
                effect: self.paragraph(effect),
            })
        } else if let Some(keyword) = self
            .catalogs
            .and_then(|catalogs| keyword_ability_list(self.source, body, catalogs))
        {
            AbilityKind::Keyword(keyword)
        } else if let Some((introducer, introducer_span, rest)) = trigger_start(body, text) {
            if let Some(comma) = find_top_level(self.source, rest, ", ") {
                let event = trim_span(self.source, Span::new(rest.start, comma));
                let effect = trim_span(self.source, Span::new(comma + 2, rest.end));
                AbilityKind::Triggered(TriggeredAbility {
                    introducer,
                    introducer_span,
                    event: self.simple_clause(event),
                    effect: self.paragraph(effect),
                })
            } else {
                self.diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::MissingTriggerSeparator,
                    span: body,
                });
                AbilityKind::Paragraph(self.paragraph(body))
            }
        } else if let Some(colon) = find_top_level(self.source, body, ":") {
            let cost_span = trim_span(self.source, Span::new(body.start, colon));
            let effect_span = trim_span(self.source, Span::new(colon + 1, body.end));
            if cost_span.is_empty() {
                self.diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::EmptyActivationCost,
                    span: Span::new(colon, colon + 1),
                });
            }
            if effect_span.is_empty() {
                self.diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::EmptyActivationEffect,
                    span: Span::new(colon, colon + 1),
                });
            }
            AbilityKind::Activated(ActivatedAbility {
                cost: Cost {
                    span: cost_span,
                    components: split_top_level(self.source, cost_span, ", "),
                },
                effect: self.paragraph(effect_span),
            })
        } else {
            if text.starts_with('•') {
                self.diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::OrphanMode,
                    span: body,
                });
            }
            AbilityKind::Paragraph(self.paragraph(body))
        };
        Ability {
            span,
            ability_word,
            kind,
        }
    }

    fn paragraph(&self, span: Span) -> Paragraph {
        let sentences = sentence_spans(self.source, span)
            .into_iter()
            .map(|(sentence_span, content, terminal)| Sentence {
                span: sentence_span,
                terminal,
                clause: self.clause(content),
            })
            .collect();
        Paragraph { span, sentences }
    }

    fn clause(&self, span: Span) -> Clause {
        const PREFIXES: [(&str, Subordinator); 5] = [
            ("as long as ", Subordinator::AsLongAs),
            ("because ", Subordinator::Because),
            ("unless ", Subordinator::Unless),
            ("until ", Subordinator::Until),
            ("if ", Subordinator::If),
        ];
        const INFIXES: [(&str, Subordinator); 4] = [
            (" as long as ", Subordinator::AsLongAs),
            (" unless ", Subordinator::Unless),
            (" until ", Subordinator::Until),
            (" if ", Subordinator::If),
        ];
        for (prefix, subordinator) in PREFIXES {
            if self
                .text(span)
                .get(..prefix.len())
                .is_some_and(|opening| opening.eq_ignore_ascii_case(prefix))
            {
                let subordinator_span = Span::new(span.start, span.start + prefix.trim_end().len());
                let rest = Span::new(span.start + prefix.len(), span.end);
                if let Some(comma) = find_top_level(self.source, rest, ", ") {
                    return Clause::Conditional(ConditionalClause {
                        span,
                        subordinator,
                        subordinator_span,
                        position: ConditionalPosition::BeforeConsequence,
                        condition: self
                            .simple_clause(trim_span(self.source, Span::new(rest.start, comma))),
                        consequence: self
                            .simple_clause(trim_span(self.source, Span::new(comma + 2, rest.end))),
                    });
                }
            }
        }

        for (infix, subordinator) in INFIXES {
            if let Some(start) = find_top_level(self.source, span, infix) {
                let consequence = trim_span(self.source, Span::new(span.start, start));
                let condition_start = start + infix.len();
                let condition = trim_span(self.source, Span::new(condition_start, span.end));
                return Clause::Conditional(ConditionalClause {
                    span,
                    subordinator,
                    subordinator_span: Span::new(start + 1, condition_start - 1),
                    position: ConditionalPosition::AfterConsequence,
                    condition: self.simple_clause(condition),
                    consequence: self.simple_clause(consequence),
                });
            }
        }
        Clause::Simple(self.simple_clause(span))
    }

    fn simple_clause(&self, span: Span) -> SimpleClause {
        let words: Vec<&Token> = self
            .tokens
            .iter()
            .filter(|token| {
                token.span.start >= span.start
                    && token.span.end <= span.end
                    && matches!(token.kind, TokenKind::Word)
            })
            .collect();
        let Some(predicate_match) = predicate_word(self.source, span, &words, self.catalogs) else {
            return SimpleClause {
                span,
                subject: None,
                predicate: None,
            };
        };
        let predicate_start = predicate_match
            .auxiliary
            .map_or(predicate_match.verb.start, |span| span.start);
        let subject = trim_span(self.source, Span::new(span.start, predicate_start));
        let complement = trim_span(self.source, Span::new(predicate_match.verb.end, span.end));
        let negated = predicate_match
            .auxiliary
            .is_some_and(|auxiliary| is_negative(self.text(auxiliary)))
            || words.iter().any(|token| {
                token.span.start >= predicate_start
                    && token.span.start < predicate_match.verb.start
                    && self.text(token.span).eq_ignore_ascii_case("not")
            });
        SimpleClause {
            span,
            subject: (!subject.is_empty()).then_some(subject),
            predicate: Some(Predicate {
                span: Span::new(predicate_start, span.end),
                auxiliary: predicate_match.auxiliary,
                verb: predicate_match.verb,
                verb_kind: predicate_match.verb_kind,
                complement: (!complement.is_empty()).then_some(complement),
                negated,
            }),
        }
    }

    fn text(&self, span: Span) -> &str {
        span.text(self.source).unwrap_or_default()
    }
}

fn lex(source: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    let mut delimiters: Vec<(char, usize)> = Vec::new();
    let mut quote = None;
    let mut index = 0;
    while index < source.len() {
        let ch = source[index..]
            .chars()
            .next()
            .expect("index is on a character boundary");
        let width = ch.len_utf8();
        if ch == '\n' {
            tokens.push(Token {
                kind: TokenKind::Newline,
                span: Span::new(index, index + width),
            });
            index += width;
            continue;
        }
        if ch.is_whitespace() {
            index += width;
            continue;
        }
        if ch == '{' {
            let end = source[index + width..]
                .find('}')
                .map_or(source.len(), |offset| index + width + offset + 1);
            if end == source.len() && !source[index..].ends_with('}') {
                diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::UnclosedDelimiter('{'),
                    span: Span::new(index, index + width),
                });
            }
            tokens.push(Token {
                kind: TokenKind::Symbol,
                span: Span::new(index, end),
            });
            index = end;
            continue;
        }
        if ch.is_alphanumeric() || matches!(ch, '\'' | '’') {
            let start = index;
            index += width;
            while index < source.len() {
                let next = source[index..].chars().next().expect("character boundary");
                if next.is_alphanumeric() || matches!(next, '\'' | '’' | '-') {
                    index += next.len_utf8();
                } else {
                    break;
                }
            }
            let word = &source[start..index];
            tokens.push(Token {
                kind: if word.chars().all(|c| c.is_ascii_digit()) {
                    TokenKind::Number
                } else {
                    TokenKind::Word
                },
                span: Span::new(start, index),
            });
            continue;
        }
        let kind = match ch {
            '~' => TokenKind::SelfReference,
            '•' => TokenKind::Bullet,
            _ => TokenKind::Punctuation(ch),
        };
        tokens.push(Token {
            kind,
            span: Span::new(index, index + width),
        });
        match ch {
            '(' | '[' => delimiters.push((ch, index)),
            ')' | ']' => {
                let expected = if ch == ')' { '(' } else { '[' };
                if delimiters.last().is_some_and(|(open, _)| *open == expected) {
                    delimiters.pop();
                } else {
                    diagnostics.push(Diagnostic {
                        kind: DiagnosticKind::UnexpectedClosingDelimiter(ch),
                        span: Span::new(index, index + width),
                    });
                }
            }
            '"' => {
                if quote.is_some() {
                    quote = None;
                } else {
                    quote = Some(index);
                }
            }
            _ => {}
        }
        index += width;
    }
    for (delimiter, start) in delimiters {
        diagnostics.push(Diagnostic {
            kind: DiagnosticKind::UnclosedDelimiter(delimiter),
            span: Span::new(start, start + delimiter.len_utf8()),
        });
    }
    if let Some(start) = quote {
        diagnostics.push(Diagnostic {
            kind: DiagnosticKind::UnterminatedQuote,
            span: Span::new(start, start + 1),
        });
    }
    (tokens, diagnostics)
}

fn line_spans(source: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut start = 0;
    for line in source.split_inclusive('\n') {
        let end = start + line.trim_end_matches(['\r', '\n']).len();
        let span = trim_span(source, Span::new(start, end));
        if !span.is_empty() {
            spans.push(span);
        }
        start += line.len();
    }
    spans
}

fn trim_span(source: &str, span: Span) -> Span {
    let Some(text) = span.text(source) else { return span };
    let start_trim = text.len() - text.trim_start().len();
    let end_trim = text.len() - text.trim_end().len();
    Span::new(span.start + start_trim, span.end.saturating_sub(end_trim))
}

fn split_ability_word(
    source: &str,
    span: Span,
    catalogs: Option<&Catalogs>,
) -> (Option<Span>, Span) {
    let Some(dash) = find_top_level(source, span, " — ") else {
        return (None, span);
    };
    let label = trim_span(source, Span::new(span.start, dash));
    let body = trim_span(source, Span::new(dash + " — ".len(), span.end));
    let label_text = label.text(source).unwrap_or_default();
    let plausible = catalogs.map_or_else(
        || {
            !label.is_empty()
                && label.len() <= 40
                && !label_text.contains([',', '.', ':', ';', '"'])
                && label_text.chars().next().is_some_and(char::is_uppercase)
                && !body.is_empty()
        },
        |catalogs| catalogs.is_ability_word(label_text) && !body.is_empty(),
    );
    if plausible { (Some(label), body) } else { (None, span) }
}

fn keyword_ability_list(
    source: &str,
    span: Span,
    catalogs: &Catalogs,
) -> Option<KeywordAbilityList> {
    let first_text = span.text(source)?;
    let first_name = catalogs.keyword_ability_prefix(first_text)?;
    let first_remainder = first_text[first_name.len()..].trim_start();
    if !first_remainder.starts_with('—') && has_top_level_terminal(source, span) {
        return None;
    }

    let mut abilities = Vec::new();
    let mut start = span.start;

    while start < span.end {
        let remaining = trim_span(source, Span::new(start, span.end));
        let text = remaining.text(source)?;
        let name = catalogs.keyword_ability_prefix(text)?;
        let name_span = Span::new(remaining.start, remaining.start + name.len());
        let next = next_keyword_ability(source, name_span.end, span.end, catalogs);
        let end = next.map_or(span.end, |(delimiter, _)| delimiter);
        let item_span = trim_span(source, Span::new(remaining.start, end));
        let argument = keyword_argument(source, Span::new(name_span.end, item_span.end));
        abilities.push(KeywordAbility {
            span: item_span,
            name: name.to_owned(),
            printed_name: name_span,
            argument,
        });

        let Some((_, next_start)) = next else { break };
        start = next_start;
    }

    (!abilities.is_empty()).then_some(KeywordAbilityList { span, abilities })
}

fn next_keyword_ability(
    source: &str,
    start: usize,
    end: usize,
    catalogs: &Catalogs,
) -> Option<(usize, usize)> {
    let mut state = Nesting::default();
    for (relative, ch) in source[start..end].char_indices() {
        let delimiter = start + relative;
        if state.is_top_level() && matches!(ch, ',' | ';') {
            let next = trim_span(source, Span::new(delimiter + ch.len_utf8(), end));
            if catalogs
                .keyword_ability_prefix(next.text(source)?)
                .is_some()
            {
                return Some((delimiter, next.start));
            }
        }
        state.observe(ch);
    }
    None
}

fn has_top_level_terminal(source: &str, span: Span) -> bool {
    let mut state = Nesting::default();
    span.text(source).is_some_and(|text| {
        text.chars().any(|ch| {
            let terminal = state.is_top_level() && matches!(ch, '.' | '!' | '?');
            state.observe(ch);
            terminal
        })
    })
}

fn keyword_argument(source: &str, span: Span) -> Option<Span> {
    let mut argument = trim_span(source, span);
    if argument.text(source)?.starts_with('—') {
        argument = trim_span(
            source,
            Span::new(argument.start + '—'.len_utf8(), argument.end),
        );
    }
    (!argument.is_empty()).then_some(argument)
}

fn trigger_start(body: Span, text: &str) -> Option<(TriggerWord, Span, Span)> {
    let (word, prefix) = if text.starts_with("Whenever ") {
        (TriggerWord::Whenever, "Whenever")
    } else if text.starts_with("When ") {
        (TriggerWord::When, "When")
    } else if text.starts_with("At ") {
        (TriggerWord::At, "At")
    } else {
        return None;
    };
    let introducer = Span::new(body.start, body.start + prefix.len());
    let rest = Span::new(introducer.end + 1, body.end);
    Some((word, introducer, rest))
}

fn loyalty_parts(source: &str, span: Span) -> Option<(Span, Span)> {
    let text = span.text(source)?;
    if !text.starts_with('[') {
        return None;
    }
    let delimiter = text.find("]: ")?;
    let cost_end = span.start + delimiter + 1;
    let effect_start = span.start + delimiter + 3;
    Some((
        Span::new(span.start, cost_end),
        trim_span(source, Span::new(effect_start, span.end)),
    ))
}

fn modal_header_span(source: &str, line: Span) -> Option<Span> {
    ["Choose ", "choose "].into_iter().find_map(|opening| {
        let start = find_top_level(source, line, opening)?;
        let header = trim_span(source, Span::new(start, line.end));
        let text = header.text(source)?;
        (text.ends_with('—') || text.contains("choose the same mode more than once."))
            .then_some(header)
    })
}

fn mode_body(source: &str, span: Span) -> Option<(Span, Span)> {
    let text = span.text(source)?;
    let rest = text.strip_prefix('•')?;
    let bullet = Span::new(span.start, span.start + '•'.len_utf8());
    let body_start = span.end - rest.len();
    let body = trim_span(source, Span::new(body_start, span.end));
    (!body.is_empty()).then_some((bullet, body))
}

fn sentence_spans(source: &str, span: Span) -> Vec<(Span, Span, Option<Span>)> {
    if span.is_empty() {
        return Vec::new();
    }
    let mut sentences = Vec::new();
    let mut state = Nesting::default();
    let mut start = span.start;
    for (relative, ch) in source[span.start..span.end].char_indices() {
        let index = span.start + relative;
        // A quoted granted ability conventionally owns its period inside the
        // quotation marks. The closing quote is therefore also the boundary
        // of the surrounding sentence when another sentence follows.
        if ch == '"'
            && state.quoted
            && source[..index]
                .chars()
                .next_back()
                .is_some_and(|previous| matches!(previous, '.' | '!' | '?'))
            && quote_ends_sentence(source, index + ch.len_utf8(), span.end)
        {
            let end = index + ch.len_utf8();
            let content = trim_span(source, Span::new(start, index - 1));
            if !content.is_empty() {
                sentences.push((
                    Span::new(content.start, end),
                    content,
                    Some(Span::new(index - 1, index)),
                ));
            }
            start = end;
        }
        if state.is_top_level() && matches!(ch, '.' | '!' | '?') {
            let end = index + ch.len_utf8();
            let content = trim_span(source, Span::new(start, index));
            if !content.is_empty() {
                sentences.push((
                    Span::new(content.start, end),
                    content,
                    Some(Span::new(index, end)),
                ));
            }
            start = end;
        }
        state.observe(ch);
    }
    let tail = trim_span(source, Span::new(start, span.end));
    if !tail.is_empty() {
        sentences.push((tail, tail, None));
    }
    sentences
}

fn quote_ends_sentence(source: &str, after_quote: usize, paragraph_end: usize) -> bool {
    let rest = source[after_quote..paragraph_end].trim_start();
    rest.is_empty() || rest.chars().next().is_some_and(char::is_uppercase)
}

fn split_top_level(source: &str, span: Span, delimiter: &str) -> Vec<Span> {
    let mut parts = Vec::new();
    let mut start = span.start;
    let mut remaining = Span::new(start, span.end);
    while let Some(index) = find_top_level(source, remaining, delimiter) {
        let part = trim_span(source, Span::new(start, index));
        if !part.is_empty() {
            parts.push(part);
        }
        start = index + delimiter.len();
        remaining = Span::new(start, span.end);
    }
    let tail = trim_span(source, Span::new(start, span.end));
    if !tail.is_empty() {
        parts.push(tail);
    }
    parts
}

fn find_top_level(source: &str, span: Span, needle: &str) -> Option<usize> {
    let text = span.text(source)?;
    let mut state = Nesting::default();
    for (relative, ch) in text.char_indices() {
        if state.is_top_level() && text[relative..].starts_with(needle) {
            return Some(span.start + relative);
        }
        state.observe(ch);
    }
    None
}

#[derive(Default)]
struct Nesting {
    parentheses: usize,
    brackets: usize,
    braces: usize,
    quoted: bool,
}

impl Nesting {
    fn is_top_level(&self) -> bool {
        self.parentheses == 0 && self.brackets == 0 && self.braces == 0 && !self.quoted
    }

    fn observe(&mut self, ch: char) {
        match ch {
            '"' => self.quoted = !self.quoted,
            '(' if !self.quoted => self.parentheses += 1,
            ')' if !self.quoted => self.parentheses = self.parentheses.saturating_sub(1),
            '[' if !self.quoted => self.brackets += 1,
            ']' if !self.quoted => self.brackets = self.brackets.saturating_sub(1),
            '{' if !self.quoted => self.braces += 1,
            '}' if !self.quoted => self.braces = self.braces.saturating_sub(1),
            _ => {}
        }
    }
}

struct PredicateMatch {
    verb: Span,
    auxiliary: Option<Span>,
    verb_kind: VerbKind,
}

fn predicate_word(
    source: &str,
    clause: Span,
    words: &[&Token],
    catalogs: Option<&Catalogs>,
) -> Option<PredicateMatch> {
    let first = *words.first()?;
    let first_text = first.span.text(source)?.to_ascii_lowercase();
    let later_predicate = words.iter().skip(1).find(|token| {
        let word = token.span.text(source).unwrap_or_default();
        is_finite_verb(word) || is_auxiliary(word)
    });
    if let Some(action) =
        catalogs.and_then(|catalogs| catalogs.keyword_action_prefix(clause.text(source)?))
    {
        return Some(PredicateMatch {
            verb: Span::new(clause.start, clause.start + action.len()),
            auxiliary: None,
            verb_kind: VerbKind::KeywordAction,
        });
    }
    if is_imperative(&first_text) && later_predicate.is_none() {
        return Some(PredicateMatch {
            verb: first.span,
            auxiliary: None,
            verb_kind: VerbKind::Ordinary,
        });
    }
    for (index, token) in words.iter().enumerate() {
        let text = token.span.text(source).unwrap_or_default();
        if is_auxiliary(text) {
            let verb = words
                .iter()
                .skip(index + 1)
                .find(|next| !matches!(next.span.text(source), Some("not" | "also")))
                .copied()
                .unwrap_or(token);
            return Some(PredicateMatch {
                verb: verb.span,
                auxiliary: (verb.span != token.span).then_some(token.span),
                verb_kind: VerbKind::Ordinary,
            });
        }
        if is_finite_verb(text) {
            return Some(PredicateMatch {
                verb: token.span,
                auxiliary: None,
                verb_kind: VerbKind::Ordinary,
            });
        }
    }
    is_imperative(&first_text).then_some(PredicateMatch {
        verb: first.span,
        auxiliary: None,
        verb_kind: VerbKind::Ordinary,
    })
}

fn is_auxiliary(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "can"
            | "can't"
            | "cannot"
            | "could"
            | "does"
            | "doesn't"
            | "do"
            | "don't"
            | "may"
            | "might"
            | "must"
            | "shall"
            | "should"
            | "will"
            | "won't"
            | "would"
    )
}

fn is_negative(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "can't" | "cannot" | "doesn't" | "don't" | "won't"
    )
}

fn is_finite_verb(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "adds"
            | "are"
            | "attacks"
            | "becomes"
            | "blocks"
            | "casts"
            | "chooses"
            | "controls"
            | "costs"
            | "counts"
            | "creates"
            | "deals"
            | "destroys"
            | "dies"
            | "discards"
            | "draws"
            | "enters"
            | "exiles"
            | "gains"
            | "gets"
            | "has"
            | "have"
            | "is"
            | "leaves"
            | "loses"
            | "owns"
            | "pays"
            | "plays"
            | "prevents"
            | "puts"
            | "returns"
            | "reveals"
            | "sacrifices"
            | "searches"
            | "taps"
            | "targets"
            | "untaps"
            | "was"
            | "were"
    )
}

fn is_imperative(word: &str) -> bool {
    matches!(
        word,
        "add"
            | "attach"
            | "choose"
            | "counter"
            | "create"
            | "deal"
            | "destroy"
            | "discard"
            | "draw"
            | "exchange"
            | "exile"
            | "gain"
            | "investigate"
            | "look"
            | "lose"
            | "mill"
            | "pay"
            | "prevent"
            | "put"
            | "return"
            | "reveal"
            | "sacrifice"
            | "scry"
            | "search"
            | "shuffle"
            | "surveil"
            | "tap"
            | "untap"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(source: &str, span: Span) -> &str {
        span.text(source).unwrap()
    }

    #[test]
    fn activated_ability_has_cost_components_and_effect_sentences() {
        let source = "{1}{R}, {T}, Sacrifice ~: Draw a card. If you do, discard a card.";
        let ast = parse(source);
        assert!(ast.diagnostics.is_empty());
        let AbilityKind::Activated(ability) = &ast.abilities[0].kind else {
            panic!("expected activated ability")
        };
        assert_eq!(
            ability
                .cost
                .components
                .iter()
                .map(|span| text(source, *span))
                .collect::<Vec<_>>(),
            ["{1}{R}", "{T}", "Sacrifice ~"]
        );
        assert_eq!(ability.effect.sentences.len(), 2);
        let Clause::Conditional(conditional) = &ability.effect.sentences[1].clause else {
            panic!("expected conditional clause")
        };
        assert_eq!(conditional.subordinator, Subordinator::If);
        assert_eq!(text(source, conditional.condition.span), "you do");
        assert_eq!(text(source, conditional.consequence.span), "discard a card");
    }

    #[test]
    fn trigger_and_ability_word_are_separate_syntax() {
        let source = "Landfall — Whenever a land enters under your control, draw a card.";
        let ast = parse(source);
        let ability = &ast.abilities[0];
        assert_eq!(text(source, ability.ability_word.unwrap()), "Landfall");
        let AbilityKind::Triggered(trigger) = &ability.kind else {
            panic!("expected triggered ability")
        };
        assert_eq!(trigger.introducer, TriggerWord::Whenever);
        assert_eq!(
            text(source, trigger.event.span),
            "a land enters under your control"
        );
        assert_eq!(text(source, trigger.event.subject.unwrap()), "a land");
        assert_eq!(
            text(source, trigger.event.predicate.as_ref().unwrap().verb),
            "enters"
        );
        assert_eq!(text(source, trigger.effect.span), "draw a card.");
    }

    #[test]
    fn lowercased_trigger_effect_can_still_open_with_a_condition() {
        let source = "Whenever ~ attacks, if you control another creature, draw a card.";
        let ast = parse(source);
        let AbilityKind::Triggered(trigger) = &ast.abilities[0].kind else { panic!() };
        let Clause::Conditional(conditional) = &trigger.effect.sentences[0].clause else {
            panic!("expected conditional trigger effect")
        };
        assert_eq!(conditional.subordinator, Subordinator::If);
        assert_eq!(
            text(source, conditional.condition.span),
            "you control another creature"
        );
        assert_eq!(text(source, conditional.consequence.span), "draw a card");
    }

    #[test]
    fn modal_lines_form_one_ability_without_losing_bullets() {
        let source = "Choose one —\n• Draw two cards.\n• Destroy target artifact or enchantment.";
        let ast = parse(source);
        assert_eq!(ast.abilities.len(), 1);
        let AbilityKind::Modal(modal) = &ast.abilities[0].kind else {
            panic!("expected modal ability")
        };
        assert_eq!(modal.frame, ModalFrame::Unframed);
        assert_eq!(modal.modes.len(), 2);
        assert_eq!(text(source, modal.modes[0].bullet), "•");
        assert_eq!(text(source, modal.modes[0].body.span), "Draw two cards.");
        assert_eq!(
            text(source, modal.modes[1].body.span),
            "Destroy target artifact or enchantment."
        );
    }

    #[test]
    fn activated_and_triggered_modal_headers_keep_their_outer_frames() {
        let activated_source = "{2}, {T}: Choose one —\n• Draw a card.\n• Create a Treasure token.";
        let activated = parse(activated_source);
        let AbilityKind::Modal(modal) = &activated.abilities[0].kind else {
            panic!("expected activated modal")
        };
        let ModalFrame::Activated(cost) = &modal.frame else {
            panic!("expected activation frame")
        };
        assert_eq!(
            cost.components
                .iter()
                .map(|span| text(activated_source, *span))
                .collect::<Vec<_>>(),
            ["{2}", "{T}"]
        );

        let triggered_source = "Whenever ~ attacks, choose one —\n• Draw a card.\n• Scry 1.";
        let triggered = parse(triggered_source);
        let AbilityKind::Modal(modal) = &triggered.abilities[0].kind else {
            panic!("expected triggered modal")
        };
        let ModalFrame::Triggered {
            introducer, event, ..
        } = &modal.frame
        else {
            panic!("expected trigger frame")
        };
        assert_eq!(*introducer, TriggerWord::Whenever);
        assert_eq!(text(triggered_source, event.span), "~ attacks");
        assert_eq!(text(triggered_source, modal.header.span), "choose one —");
    }

    #[test]
    fn quoted_granted_ability_does_not_split_its_sentence_or_colon() {
        let source = "Create a token with \"{T}: Add {G}.\" Then draw a card.";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!("quoted colon must not create an activated frame")
        };
        assert_eq!(paragraph.sentences.len(), 2);
        assert_eq!(
            text(source, paragraph.sentences[0].span),
            "Create a token with \"{T}: Add {G}.\""
        );
    }

    #[test]
    fn postposed_condition_preserves_surface_order() {
        let source = "Draw two cards if you control an artifact.";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Conditional(conditional) = &paragraph.sentences[0].clause else {
            panic!()
        };
        assert_eq!(conditional.position, ConditionalPosition::AfterConsequence);
        assert_eq!(text(source, conditional.consequence.span), "Draw two cards");
        assert_eq!(
            text(source, conditional.condition.span),
            "you control an artifact"
        );
    }

    #[test]
    fn loyalty_cost_is_not_mistaken_for_an_activation_cost() {
        let source = "[−X]: Exile each nonland permanent with mana value X or less.";
        let ast = parse(source);
        let AbilityKind::Loyalty(loyalty) = &ast.abilities[0].kind else {
            panic!("expected loyalty ability")
        };
        assert_eq!(text(source, loyalty.cost), "[−X]");
        assert_eq!(
            text(source, loyalty.effect.span),
            "Exile each nonland permanent with mana value X or less."
        );
    }

    #[test]
    fn target_determiner_is_subject_when_a_later_predicate_exists() {
        let source = "Target creature can't block this turn.";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        let predicate = clause.predicate.as_ref().unwrap();
        assert_eq!(text(source, clause.subject.unwrap()), "Target creature");
        assert_eq!(text(source, predicate.auxiliary.unwrap()), "can't");
        assert_eq!(text(source, predicate.verb), "block");
        assert!(predicate.negated);
    }

    #[test]
    fn punctuation_inside_a_mid_sentence_quote_stays_nested() {
        let source = "Create a token named \"A. B\" and draw a card.";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        assert_eq!(paragraph.sentences.len(), 1);
        assert_eq!(text(source, paragraph.sentences[0].span), source);
    }

    #[test]
    fn malformed_delimiters_and_orphan_modes_are_diagnostics_not_parse_failures() {
        let source = "• Draw a card (then discard a card.";
        let ast = parse(source);
        assert_eq!(ast.abilities.len(), 1);
        assert!(
            ast.diagnostics
                .iter()
                .any(|diagnostic| diagnostic.kind == DiagnosticKind::OrphanMode)
        );
        assert!(
            ast.diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.kind == DiagnosticKind::UnclosedDelimiter('(') })
        );

        let empty_effect = parse("{T}:");
        assert!(
            empty_effect
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.kind == DiagnosticKind::EmptyActivationEffect })
        );
    }

    #[test]
    fn lexer_keeps_symbols_self_references_and_newlines() {
        let source = "{T}: ~ adds {G}.\nDraw 2 cards.";
        let ast = parse(source);
        assert!(
            ast.tokens
                .iter()
                .any(|token| token.kind == TokenKind::Symbol)
        );
        assert!(
            ast.tokens
                .iter()
                .any(|token| token.kind == TokenKind::SelfReference)
        );
        assert!(
            ast.tokens
                .iter()
                .any(|token| token.kind == TokenKind::Newline)
        );
        assert!(
            ast.tokens
                .iter()
                .any(|token| token.kind == TokenKind::Number)
        );
    }

    #[test]
    fn scryfall_catalogs_recognize_keyword_lists_actions_and_ability_words() {
        let catalogs = Catalogs::new(
            [
                "Flying",
                "First strike",
                "Protection",
                "Max speed",
                "Forecast",
                "Power-up",
            ],
            ["Scry", "Manifest dread", "Fight"],
            ["Landfall", "Void"],
        );

        let keyword_source = "Flying, first strike, protection from red";
        let keywords = parse_with_catalogs(keyword_source, &catalogs);
        let AbilityKind::Keyword(keyword_list) = &keywords.abilities[0].kind else {
            panic!("expected a keyword-ability list")
        };
        assert_eq!(keyword_list.abilities.len(), 3);
        assert_eq!(keyword_list.abilities[0].name, "Flying");
        assert_eq!(keyword_list.abilities[1].name, "First strike");
        assert_eq!(
            text(keyword_source, keyword_list.abilities[1].printed_name),
            "first strike"
        );
        assert_eq!(
            text(keyword_source, keyword_list.abilities[2].argument.unwrap()),
            "from red"
        );

        let action_source = "Manifest dread 2.";
        let action = parse_with_catalogs(action_source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &action.abilities[0].kind else {
            panic!("expected an ordinary spell paragraph")
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        let predicate = clause.predicate.as_ref().unwrap();
        assert_eq!(text(action_source, predicate.verb), "Manifest dread");
        assert_eq!(predicate.verb_kind, VerbKind::KeywordAction);

        let ability_word_source = "Void — Whenever ~ attacks, draw a card.";
        let ability_word = parse_with_catalogs(ability_word_source, &catalogs);
        assert_eq!(
            text(
                ability_word_source,
                ability_word.abilities[0].ability_word.unwrap()
            ),
            "Void"
        );

        let max_speed_source = "Max speed — Draw an additional card.";
        let max_speed = parse_with_catalogs(max_speed_source, &catalogs);
        assert!(max_speed.abilities[0].ability_word.is_none());
        assert!(matches!(
            max_speed.abilities[0].kind,
            AbilityKind::Keyword(_)
        ));

        let forecast_source = "Forecast — {1}{U}, Reveal this card: Draw a card.";
        let forecast = parse_with_catalogs(forecast_source, &catalogs);
        assert!(matches!(
            forecast.abilities[0].kind,
            AbilityKind::Keyword(_)
        ));

        let static_source = "Power-up abilities you activate cost {1} less to activate.";
        let static_ability = parse_with_catalogs(static_source, &catalogs);
        assert!(matches!(
            static_ability.abilities[0].kind,
            AbilityKind::Paragraph(_)
        ));

        let fight_source = "Fight target creature you don't control.";
        let fight = parse_with_catalogs(fight_source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &fight.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        assert_eq!(
            clause.predicate.as_ref().unwrap().verb_kind,
            VerbKind::KeywordAction
        );
    }
}
