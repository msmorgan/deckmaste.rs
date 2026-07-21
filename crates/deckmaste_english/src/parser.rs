use crate::Ability;
use crate::AbilityKind;
use crate::ActivatedAbility;
use crate::Auxiliary;
use crate::AuxiliaryInflection;
use crate::AuxiliaryKind;
use crate::AuxiliaryNegation;
use crate::Capitalization;
use crate::CatalogKind;
use crate::Catalogs;
use crate::Clause;
use crate::ColorWord;
use crate::CommaSeparatedClause;
use crate::ConditionalClause;
use crate::ConditionalPosition;
use crate::CoordinatedClause;
use crate::CoordinatedPredicate;
use crate::Cost;
use crate::Determiner;
use crate::DeterminerKind;
use crate::Diagnostic;
use crate::DiagnosticKind;
use crate::EmbeddedRules;
use crate::EmbeddedRulesFrame;
use crate::KeywordAbility;
use crate::KeywordAbilityList;
use crate::KeywordArgumentSeparator;
use crate::KeywordListSeparator;
use crate::LoyaltyAbility;
use crate::LoyaltyCost;
use crate::LoyaltyCostSign;
use crate::LoyaltyCostValue;
use crate::ModalAbility;
use crate::ModalFrame;
use crate::ModalHeaderSuffix;
use crate::ModalPreambleSeparator;
use crate::Mode;
use crate::ModifiedNounPhrase;
use crate::NounPhrase;
use crate::NumberSpelling;
use crate::OracleText;
use crate::Paragraph;
use crate::PartOfSpeech;
use crate::Phrase;
use crate::PowerToughness;
use crate::Predicate;
use crate::PredicateConjunction;
use crate::PreverbWord;
use crate::QuantityPhrase;
use crate::ReminderText;
use crate::ScalarSign;
use crate::ScalarValue;
use crate::Sentence;
use crate::SentenceTerminal;
use crate::SentenceTerminalKind;
use crate::SentenceTerminalSuffix;
use crate::SignedScalar;
use crate::SimpleClause;
use crate::Span;
use crate::Subordinator;
use crate::ThisCardForm;
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
            if let Some(header) = modal_header(self.source, span)
                && lines
                    .get(line + 1)
                    .is_some_and(|next| mode_body(self.source, *next).is_some())
            {
                let (ability_word, body) = split_ability_word(self.source, span, self.catalogs);
                let frame = self.modal_frame(body, header.framed_span);
                let header_paragraph = self.paragraph(header.content_span);
                let mut modes = Vec::new();
                let mut end = span.end;
                line += 1;
                while let Some(mode_span) = lines.get(line).copied() {
                    let Some(body_span) = mode_body(self.source, mode_span) else {
                        break;
                    };
                    end = mode_span.end;
                    modes.push(Mode {
                        span: mode_span,
                        body: self.paragraph(body_span),
                    });
                    line += 1;
                }
                let ability_span = Span::new(span.start, end);
                abilities.push(Ability {
                    span: ability_span,
                    ability_word: ability_word.map(|word| self.phrase(word, Vec::new())),
                    reminder_text: reminder_text(self.source, ability_span),
                    kind: AbilityKind::Modal(ModalAbility {
                        frame,
                        header: header_paragraph,
                        header_suffix: header.suffix,
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
                    components: split_top_level(self.source, cost_span, ", ")
                        .into_iter()
                        .map(|component| self.phrase(component, Vec::new()))
                        .collect(),
                });
            }
        }
        if body.start < header.start {
            let before_header = &self.source[body.start..header.start];
            let (preamble_end, separator) = if before_header.ends_with(", ") {
                (header.start - 2, ModalPreambleSeparator::CommaSpace)
            } else if before_header.ends_with(' ') {
                (header.start - 1, ModalPreambleSeparator::Space)
            } else {
                (header.start, ModalPreambleSeparator::None)
            };
            let preamble = trim_span(self.source, Span::new(body.start, preamble_end));
            if !preamble.is_empty() {
                return ModalFrame::Preamble {
                    body: self.paragraph(preamble),
                    separator,
                };
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
        } else if let Some(mut keyword) = self
            .catalogs
            .and_then(|catalogs| keyword_ability_list(self.source, body, catalogs))
        {
            for item in &mut keyword.abilities {
                let argument_span = keyword_argument(
                    self.source,
                    Span::new(item.span.start + item.name.len(), item.span.end),
                );
                if let Some(argument_span) = argument_span
                    && self.looks_like_rules(argument_span)
                {
                    let embedded = EmbeddedRules {
                        span: argument_span,
                        frame: EmbeddedRulesFrame::Bare,
                        ability: Box::new(self.embedded_ability(argument_span)),
                    };
                    item.argument = Some(self.phrase(argument_span, vec![embedded]));
                }
            }
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
                    components: split_top_level(self.source, cost_span, ", ")
                        .into_iter()
                        .map(|component| self.phrase(component, Vec::new()))
                        .collect(),
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
            ability_word: ability_word.map(|word| self.phrase(word, Vec::new())),
            reminder_text: reminder_text(self.source, span),
            kind,
        }
    }

    fn paragraph(&self, span: Span) -> Paragraph {
        let reminders = reminder_text(self.source, span);
        let sentences = sentence_spans(self.source, span)
            .into_iter()
            .filter_map(|(sentence_span, content, terminal)| {
                let content = strip_leading_reminder_text(self.source, content, &reminders);
                (!content.is_empty()).then(|| Sentence {
                    span: Span::new(content.start, sentence_span.end),
                    terminal,
                    clause: self.clause(content),
                })
            })
            .collect();
        Paragraph { span, sentences }
    }

    fn clause(&self, span: Span) -> Clause {
        const PREFIXES: [(&str, Subordinator); 6] = [
            ("as long as ", Subordinator::AsLongAs),
            ("because ", Subordinator::Because),
            ("unless ", Subordinator::Unless),
            ("until ", Subordinator::Until),
            ("when ", Subordinator::When),
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
                        subordinator_capitalization: capitalization(self.text(subordinator_span)),
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

        if let Some(comma) = find_top_level(self.source, span, ", ") {
            let first = trim_span(self.source, Span::new(span.start, comma));
            let last_word = self.text(first).split_whitespace().next_back();
            if last_word.is_some_and(|word| auxiliary_parts(word).is_some()) {
                let second = trim_span(self.source, Span::new(comma + 2, span.end));
                return Clause::CommaSeparated(CommaSeparatedClause {
                    span,
                    first: self.simple_clause(first),
                    second: self.simple_clause(second),
                });
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
                    subordinator_capitalization: Capitalization::Lowercase,
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
                    && token_is_top_level(self.source, span, token.span.start)
            })
            .collect();
        let Some(predicate_match) = predicate_word(self.source, span, &words, self.catalogs) else {
            return SimpleClause {
                span,
                unparsed: Some(self.phrase(span, self.quoted_rules(span))),
                subject: None,
                predicate: None,
                coordinated_predicates: Vec::new(),
                coordinated_clauses: Vec::new(),
            };
        };
        let predicate_start = predicate_match
            .auxiliary
            .map_or(predicate_match.verb.start, |auxiliary| auxiliary.span.start);
        let subject = trim_span(self.source, Span::new(span.start, predicate_start));
        let mut current_match = predicate_match;
        let mut current_start = predicate_start;
        let mut coordinated_predicates = Vec::new();
        let mut predicate = None;
        let mut pending_coordination = None;
        let clause_coordination = coordinated_clause(
            self.source,
            Span::new(predicate_match.verb.end, span.end),
            &self.tokens,
            self.catalogs,
        );
        let predicate_bound =
            clause_coordination.map_or(span.end, |coordination| coordination.left_end);

        loop {
            let coordination = coordinated_predicate(
                self.source,
                Span::new(current_match.verb.end, predicate_bound),
                &self.tokens,
                self.catalogs,
            );
            let predicate_end =
                coordination.map_or(predicate_bound, |coordination| coordination.left_end);
            let predicate_span = trim_span(self.source, Span::new(current_start, predicate_end));
            let parsed = self.predicate(current_match, predicate_span);
            if let Some((conjunction, conjunction_span, has_comma)) = pending_coordination.take() {
                coordinated_predicates.push(CoordinatedPredicate {
                    conjunction,
                    conjunction_span,
                    has_comma,
                    predicate: parsed,
                });
            } else {
                predicate = Some(parsed);
            }

            let Some(coordination) = coordination else {
                break;
            };

            current_match = coordination.predicate_match;
            current_start = current_match
                .auxiliary
                .map_or(current_match.verb.start, |auxiliary| auxiliary.span.start);
            pending_coordination = Some((
                coordination.conjunction,
                coordination.conjunction_span,
                coordination.has_comma,
            ));
        }

        SimpleClause {
            span,
            unparsed: None,
            subject: (!subject.is_empty()).then(|| self.phrase(subject, Vec::new())),
            predicate,
            coordinated_predicates,
            coordinated_clauses: clause_coordination
                .map(|coordination| {
                    vec![CoordinatedClause {
                        conjunction: coordination.conjunction,
                        conjunction_span: coordination.conjunction_span,
                        has_comma: coordination.has_comma,
                        clause: Box::new(self.simple_clause(coordination.remainder)),
                    }]
                })
                .unwrap_or_default(),
        }
    }

    fn predicate(&self, predicate_match: PredicateMatch, span: Span) -> Predicate {
        let complement = trim_span(self.source, Span::new(predicate_match.verb.end, span.end));
        let preverb_words = self
            .tokens
            .iter()
            .filter(|token| {
                matches!(token.kind, TokenKind::Word)
                    && predicate_match
                        .auxiliary
                        .is_some_and(|auxiliary| token.span.start >= auxiliary.span.end)
                    && token.span.end <= predicate_match.verb.start
            })
            .filter_map(
                |token| match self.text(token.span).to_ascii_lowercase().as_str() {
                    "not" => Some(PreverbWord::Not),
                    "also" => Some(PreverbWord::Also),
                    _ => None,
                },
            )
            .collect::<Vec<_>>();
        let negated = predicate_match
            .auxiliary
            .is_some_and(|auxiliary| auxiliary.negation != AuxiliaryNegation::None)
            || preverb_words.contains(&PreverbWord::Not);
        Predicate {
            span,
            auxiliary: predicate_match.auxiliary,
            preverb_words,
            verb: structured_verb_text(self.text(predicate_match.verb).to_owned(), self.catalogs),
            verb_kind: predicate_match.verb_kind,
            complement: (!complement.is_empty())
                .then(|| self.complement_phrase(complement, self.quoted_rules(complement))),
            negated,
        }
    }

    fn quoted_rules(&self, span: Span) -> Vec<EmbeddedRules> {
        let quotes = self
            .tokens
            .iter()
            .filter(|token| {
                token.span.start >= span.start
                    && token.span.end <= span.end
                    && matches!(token.kind, TokenKind::Punctuation('"'))
            })
            .collect::<Vec<_>>();
        let mut embedded = quotes
            .chunks_exact(2)
            .filter_map(|pair| {
                let body = trim_span(self.source, Span::new(pair[0].span.end, pair[1].span.start));
                self.looks_like_rules(body).then(|| EmbeddedRules {
                    span: Span::new(pair[0].span.start, pair[1].span.end),
                    frame: EmbeddedRulesFrame::DoubleQuoted { closed: true },
                    ability: Box::new(self.embedded_ability(body)),
                })
            })
            .collect::<Vec<_>>();
        if let Some(open) = quotes.chunks_exact(2).remainder().first() {
            let body = trim_span(self.source, Span::new(open.span.end, span.end));
            if self.looks_like_rules(body) {
                embedded.push(EmbeddedRules {
                    span: Span::new(open.span.start, span.end),
                    frame: EmbeddedRulesFrame::DoubleQuoted { closed: false },
                    ability: Box::new(self.embedded_ability(body)),
                });
            }
        }
        embedded
    }

    fn looks_like_rules(&self, span: Span) -> bool {
        if span.is_empty() {
            return false;
        }
        if find_top_level(self.source, span, ":").is_some()
            || trigger_start(span, self.text(span)).is_some()
            || self
                .catalogs
                .is_some_and(|catalogs| keyword_ability_list(self.source, span, catalogs).is_some())
        {
            return true;
        }
        sentence_spans(self.source, span)
            .into_iter()
            .any(|(_, content, terminal)| {
                let words = self
                    .tokens
                    .iter()
                    .filter(|token| {
                        token.span.start >= content.start
                            && token.span.end <= content.end
                            && matches!(token.kind, TokenKind::Word)
                    })
                    .collect::<Vec<_>>();
                predicate_word(self.source, content, &words, self.catalogs).is_some_and(
                    |predicate| {
                        predicate.auxiliary.is_some()
                            || terminal.is_some()
                            || self.source[predicate.verb.end..content.end]
                                .chars()
                                .any(|character| character.is_alphanumeric() || character == '{')
                    },
                )
            })
    }

    fn embedded_ability(&self, span: Span) -> Ability {
        Parser {
            source: self.source,
            catalogs: self.catalogs,
            tokens: self.tokens.clone(),
            diagnostics: Vec::new(),
        }
        .ability(span)
    }

    fn phrase(&self, span: Span, embedded_rules: Vec<EmbeddedRules>) -> Phrase {
        structured_phrase(self.source, span, embedded_rules, self.catalogs)
    }

    fn complement_phrase(&self, span: Span, embedded_rules: Vec<EmbeddedRules>) -> Phrase {
        match self.phrase(span, embedded_rules) {
            Phrase::UnknownPhrase(text) => {
                if let Some(canonical) = self
                    .catalogs
                    .and_then(|catalogs| catalogs.keyword_action_form(&text))
                {
                    Phrase::CatalogTerm {
                        text,
                        canonical: canonical.to_owned(),
                        kind: CatalogKind::KeywordAction,
                    }
                } else {
                    Phrase::UnknownPhrase(text)
                }
            }
            phrase => phrase,
        }
    }

    fn text(&self, span: Span) -> &str {
        span.text(self.source).unwrap_or_default()
    }
}

fn capitalization(text: &str) -> Capitalization {
    if text.chars().next().is_some_and(char::is_uppercase) {
        Capitalization::Capitalized
    } else {
        Capitalization::Lowercase
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
        if ch == '~' {
            let end = if source[index + width..].starts_with('~') {
                index + 2 * width
            } else {
                index + width
            };
            tokens.push(Token {
                kind: if end == index + width {
                    TokenKind::SelfReference
                } else {
                    TokenKind::FullSelfReference
                },
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
    let mut preceding_separator = None;

    while start < span.end {
        let remaining = trim_span(source, Span::new(start, span.end));
        let text = remaining.text(source)?;
        let name = catalogs.keyword_ability_prefix(text)?;
        let name_span = Span::new(remaining.start, remaining.start + name.len());
        let dash_argument = source[name_span.end..span.end]
            .trim_start()
            .starts_with('—');
        let next = (!dash_argument)
            .then(|| next_keyword_ability(source, name_span.end, span.end, catalogs))
            .flatten();
        let end = next.map_or(span.end, |(delimiter, _, _)| delimiter);
        let item_span = trim_span(source, Span::new(remaining.start, end));
        let argument_span = keyword_argument(source, Span::new(name_span.end, item_span.end));
        let argument_separator = argument_span
            .map(|argument| keyword_argument_separator(&source[name_span.end..argument.start]));
        let argument = argument_span
            .map(|argument| structured_phrase(source, argument, Vec::new(), Some(catalogs)));
        abilities.push(KeywordAbility {
            span: item_span,
            preceding_separator,
            name: name.to_owned(),
            printed_name: structured_phrase(source, name_span, Vec::new(), Some(catalogs)),
            argument_separator,
            argument,
        });

        let Some((_, next_start, separator)) = next else {
            break;
        };
        preceding_separator = Some(separator);
        start = next_start;
    }

    (!abilities.is_empty()).then_some(KeywordAbilityList { span, abilities })
}

fn next_keyword_ability(
    source: &str,
    start: usize,
    end: usize,
    catalogs: &Catalogs,
) -> Option<(usize, usize, KeywordListSeparator)> {
    let mut state = Nesting::default();
    for (relative, ch) in source[start..end].char_indices() {
        let delimiter = start + relative;
        if state.is_top_level() && matches!(ch, ',' | ';') {
            let next = trim_span(source, Span::new(delimiter + ch.len_utf8(), end));
            if catalogs
                .keyword_ability_prefix(next.text(source)?)
                .is_some()
            {
                let separator = match ch {
                    ',' => KeywordListSeparator::Comma,
                    ';' => KeywordListSeparator::Semicolon,
                    _ => return None,
                };
                return Some((delimiter, next.start, separator));
            }
        }
        state.observe(ch);
    }
    None
}

fn keyword_argument_separator(text: &str) -> KeywordArgumentSeparator {
    if text.contains('—') {
        if text.chars().any(char::is_whitespace) {
            KeywordArgumentSeparator::SpacedEmDash
        } else {
            KeywordArgumentSeparator::EmDash
        }
    } else {
        KeywordArgumentSeparator::Space
    }
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
    let reminders = reminder_text(source, argument);
    if let Some(start) = trailing_reminder_start(source, argument, &reminders) {
        argument = trim_span(source, Span::new(argument.start, start));
    }
    (!argument.is_empty()).then_some(argument)
}

fn trailing_reminder_start(
    source: &str,
    argument: Span,
    reminders: &[ReminderText],
) -> Option<usize> {
    let first = reminders.first()?;
    let mut cursor = first.span.end;
    for reminder in &reminders[1..] {
        if !source[cursor..reminder.span.start].trim().is_empty() {
            return None;
        }
        cursor = reminder.span.end;
    }
    source[cursor..argument.end]
        .trim()
        .is_empty()
        .then_some(first.span.start)
}

fn reminder_text(source: &str, span: Span) -> Vec<ReminderText> {
    let mut reminders = Vec::new();
    let mut state = Nesting::default();
    let mut start = None;

    for (relative, ch) in source[span.start..span.end].char_indices() {
        let index = span.start + relative;
        if ch == '(' && state.is_top_level() {
            start = Some(index);
        }
        state.observe(ch);
        if ch == ')'
            && state.parentheses == 0
            && let Some(start) = start.take()
        {
            let span = Span::new(start, index + ch.len_utf8());
            reminders.push(ReminderText { span });
        }
    }

    reminders
}

fn strip_leading_reminder_text(source: &str, mut span: Span, reminders: &[ReminderText]) -> Span {
    loop {
        span = trim_span(source, span);
        let Some(reminder) = reminders
            .iter()
            .find(|reminder| reminder.span.start == span.start)
        else {
            return span;
        };
        span = Span::new(reminder.span.end, span.end);
    }
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

fn loyalty_parts(source: &str, span: Span) -> Option<(LoyaltyCost, Span)> {
    let text = span.text(source)?;
    if !text.starts_with('[') {
        return None;
    }
    let delimiter = text.find("]: ")?;
    let cost_end = span.start + delimiter + 1;
    let effect_start = span.start + delimiter + 3;
    let cost_span = Span::new(span.start, cost_end);
    let printed_cost = &text[1..delimiter];
    let (sign, printed_value) = if let Some(value) = printed_cost.strip_prefix('+') {
        (LoyaltyCostSign::Plus, value)
    } else if let Some(value) = printed_cost.strip_prefix('−') {
        (LoyaltyCostSign::Minus, value)
    } else if let Some(value) = printed_cost.strip_prefix('-') {
        (LoyaltyCostSign::Minus, value)
    } else {
        (LoyaltyCostSign::None, printed_cost)
    };
    let value = if printed_value == "X" {
        LoyaltyCostValue::X
    } else {
        LoyaltyCostValue::Number(printed_value.parse().ok()?)
    };
    Some((
        LoyaltyCost {
            span: cost_span,
            sign,
            value,
        },
        trim_span(source, Span::new(effect_start, span.end)),
    ))
}

struct ParsedModalHeader {
    framed_span: Span,
    content_span: Span,
    suffix: ModalHeaderSuffix,
}

fn modal_header(source: &str, line: Span) -> Option<ParsedModalHeader> {
    let line = trim_span(source, line);
    let line_text = line.text(source)?;
    if line_text.contains(" chooses ") && line_text.ends_with(" —") {
        return Some(ParsedModalHeader {
            framed_span: line,
            content_span: trim_span(source, Span::new(line.start, line.end - " —".len())),
            suffix: ModalHeaderSuffix::SpacedEmDash,
        });
    }
    ["Choose ", "choose "].into_iter().find_map(|opening| {
        let start = find_top_level(source, line, opening)?;
        let framed_span = trim_span(source, Span::new(start, line.end));
        let text = framed_span.text(source)?;
        if text.ends_with(" —") {
            Some(ParsedModalHeader {
                framed_span,
                content_span: trim_span(
                    source,
                    Span::new(framed_span.start, framed_span.end - " —".len()),
                ),
                suffix: ModalHeaderSuffix::SpacedEmDash,
            })
        } else {
            Some(ParsedModalHeader {
                framed_span,
                content_span: framed_span,
                suffix: ModalHeaderSuffix::None,
            })
        }
    })
}

fn mode_body(source: &str, span: Span) -> Option<Span> {
    let text = span.text(source)?;
    let rest = text.strip_prefix('•')?;
    let body_start = span.end - rest.len();
    let body = trim_span(source, Span::new(body_start, span.end));
    (!body.is_empty()).then_some(body)
}

fn sentence_spans(source: &str, span: Span) -> Vec<(Span, Span, Option<SentenceTerminal>)> {
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
            let terminal_span = Span::new(index - 1, index);
            if !content.is_empty()
                && let Some(kind) = terminal_span
                    .text(source)
                    .and_then(|text| text.chars().next())
                    .and_then(sentence_terminal_kind)
            {
                sentences.push((
                    Span::new(content.start, end),
                    content,
                    Some(SentenceTerminal {
                        span: terminal_span,
                        kind,
                        repetitions: 1,
                        suffix: SentenceTerminalSuffix::DoubleQuote,
                    }),
                ));
            }
            start = end;
        }
        if state.is_top_level()
            && matches!(ch, '.' | '!' | '?')
            && !(ch == '.'
                && (period_is_abbreviation(source, index, span)
                    || period_is_spaced_ellipsis(source, index, span)
                    || period_is_unspaced_ellipsis(source, index, span)
                    || period_joins_word_characters(source, index, span)))
        {
            let terminal_end = index + ch.len_utf8();
            let repetitions = if ch == '.' {
                u8::try_from(
                    source[terminal_end..span.end]
                        .chars()
                        .take_while(|next| *next == '.')
                        .count()
                        .saturating_add(1),
                )
                .unwrap_or(u8::MAX)
            } else {
                1
            };
            let (end, suffix) = if source[terminal_end..span.end].starts_with('\'') {
                (
                    terminal_end + '\''.len_utf8(),
                    SentenceTerminalSuffix::SingleQuote,
                )
            } else if source[terminal_end..span.end].starts_with('"') {
                (
                    terminal_end + '"'.len_utf8(),
                    SentenceTerminalSuffix::DoubleQuote,
                )
            } else {
                (terminal_end, SentenceTerminalSuffix::None)
            };
            let content = trim_span(source, Span::new(start, index));
            if !content.is_empty()
                && let Some(kind) = sentence_terminal_kind(ch)
            {
                sentences.push((
                    Span::new(content.start, end),
                    content,
                    Some(SentenceTerminal {
                        span: Span::new(index, terminal_end),
                        kind,
                        repetitions,
                        suffix,
                    }),
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

fn sentence_terminal_kind(terminal: char) -> Option<SentenceTerminalKind> {
    match terminal {
        '.' => Some(SentenceTerminalKind::Period),
        '!' => Some(SentenceTerminalKind::ExclamationMark),
        '?' => Some(SentenceTerminalKind::QuestionMark),
        _ => None,
    }
}

fn quote_ends_sentence(source: &str, after_quote: usize, paragraph_end: usize) -> bool {
    let rest = source[after_quote..paragraph_end].trim_start();
    rest.is_empty()
        || rest
            .chars()
            .next()
            .is_some_and(|character| character == '~' || character.is_uppercase())
}

fn period_is_abbreviation(source: &str, index: usize, span: Span) -> bool {
    let previous = source[span.start..index].chars().next_back();
    if !previous.is_some_and(char::is_uppercase) {
        return false;
    }
    let next = source[index + 1..span.end].chars().next();
    if next.is_some_and(char::is_uppercase) {
        return true;
    }
    let token_start = source[span.start..index]
        .rfind(char::is_whitespace)
        .map_or(span.start, |relative| span.start + relative + 1);
    source[token_start..index].contains('.')
}

fn period_is_spaced_ellipsis(source: &str, index: usize, span: Span) -> bool {
    source[span.start..span.end]
        .match_indices(". . .")
        .any(|(relative, _)| {
            let start = span.start + relative;
            index
                .checked_sub(start)
                .is_some_and(|offset| matches!(offset, 0 | 2 | 4))
        })
}

fn period_is_unspaced_ellipsis(source: &str, index: usize, span: Span) -> bool {
    source[span.start..span.end]
        .match_indices("...")
        .any(|(relative, _)| {
            let start = span.start + relative;
            index
                .checked_sub(start)
                .is_some_and(|offset| matches!(offset, 0..=2))
        })
}

fn period_joins_word_characters(source: &str, index: usize, span: Span) -> bool {
    source[span.start..index]
        .chars()
        .next_back()
        .is_some_and(char::is_alphanumeric)
        && source[index + 1..span.end]
            .chars()
            .next()
            .is_some_and(char::is_alphanumeric)
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

fn structured_phrase(
    source: &str,
    span: Span,
    embedded_rules: Vec<EmbeddedRules>,
    catalogs: Option<&Catalogs>,
) -> Phrase {
    let reminders = reminder_text(source, span);
    let mut text = String::new();
    let mut cursor = span.start;
    for reminder in reminders {
        if reminder.span.start >= cursor {
            text.push_str(&source[cursor..reminder.span.start]);
            cursor = reminder.span.end;
        }
    }
    text.push_str(&source[cursor..span.end]);
    let text = text.trim().to_owned();
    if embedded_rules.is_empty() {
        structured_text(text, catalogs)
    } else {
        Phrase::EmbeddedRulesPhrase {
            text,
            embedded_rules,
        }
    }
}

fn structured_text(text: String, catalogs: Option<&Catalogs>) -> Phrase {
    if let Some((kind, canonical)) = catalogs.and_then(|catalogs| catalogs.exact_term(&text)) {
        Phrase::CatalogTerm {
            text,
            canonical: canonical.to_owned(),
            kind,
        }
    } else if text == "~" {
        Phrase::ThisCard {
            text,
            form: ThisCardForm::AbbreviatedName,
        }
    } else if text == "~~" {
        Phrase::ThisCard {
            text,
            form: ThisCardForm::FullName,
        }
    } else if let Some(symbol) = oracle_symbol(&text) {
        Phrase::OracleSymbol {
            symbol: symbol.to_owned(),
            text,
        }
    } else if let Some(symbols) = oracle_symbol_sequence(&text) {
        Phrase::SymbolSequence { text, symbols }
    } else if let Some((power, toughness)) = power_toughness(&text) {
        Phrase::PowerToughness(Box::new(PowerToughness {
            text,
            power,
            toughness,
        }))
    } else if let Some((value, spelling)) = number_literal(&text) {
        Phrase::NumberLiteral {
            text,
            value,
            spelling,
        }
    } else if let Some(color) = color_word(&text) {
        Phrase::ColorWord { text, color }
    } else if let Some((quantity, unit)) = quantity_parts(&text) {
        Phrase::QuantityPhrase(Box::new(QuantityPhrase {
            text,
            quantity: Box::new(structured_text(quantity, catalogs)),
            unit: Box::new(structured_text(unit, catalogs)),
        }))
    } else if let Some((determiner, head)) = determined_noun(&text) {
        Phrase::NounPhrase(Box::new(NounPhrase {
            text,
            determiner,
            head: Box::new(structured_text(head, catalogs)),
        }))
    } else if let Some((modifier, head)) = modified_noun(&text) {
        Phrase::ModifiedNounPhrase(Box::new(ModifiedNounPhrase {
            text,
            modifier: Box::new(structured_text(modifier, catalogs)),
            head: Box::new(structured_text(head, catalogs)),
        }))
    } else if let Some(lemma) = pronoun_lemma(&text) {
        Phrase::Lexeme {
            text,
            lemma: lemma.to_owned(),
            part_of_speech: PartOfSpeech::Pronoun,
        }
    } else if let Some((lemma, part_of_speech)) = lexical_entry(&text) {
        Phrase::Lexeme {
            text,
            lemma: lemma.to_owned(),
            part_of_speech,
        }
    } else {
        Phrase::UnknownPhrase(text)
    }
}

fn pronoun_lemma(text: &str) -> Option<&'static str> {
    match text.to_ascii_lowercase().as_str() {
        "he" | "him" => Some("he"),
        "it" => Some("it"),
        "she" | "her" => Some("she"),
        "this" => Some("this"),
        "that" => Some("that"),
        "these" => Some("these"),
        "those" => Some("those"),
        "they" | "them" => Some("they"),
        "we" | "us" => Some("we"),
        "you" => Some("you"),
        _ => None,
    }
}

fn structured_verb_text(text: String, catalogs: Option<&Catalogs>) -> Phrase {
    if let Some(canonical) = catalogs.and_then(|catalogs| catalogs.keyword_action_form(&text)) {
        return Phrase::CatalogTerm {
            text,
            canonical: canonical.to_owned(),
            kind: CatalogKind::KeywordAction,
        };
    }
    match structured_text(text, catalogs) {
        Phrase::UnknownPhrase(text) => {
            if let Some(verb) = ordinary_verb(&text) {
                Phrase::Lexeme {
                    text,
                    lemma: verb.lemma.to_owned(),
                    part_of_speech: PartOfSpeech::Verb,
                }
            } else {
                Phrase::UnknownPhrase(text)
            }
        }
        Phrase::Lexeme {
            text,
            lemma,
            part_of_speech,
        } => {
            if let Some(verb) = ordinary_verb(&text) {
                Phrase::Lexeme {
                    text,
                    lemma: verb.lemma.to_owned(),
                    part_of_speech: PartOfSpeech::Verb,
                }
            } else {
                Phrase::Lexeme {
                    text,
                    lemma,
                    part_of_speech,
                }
            }
        }
        phrase => phrase,
    }
}

fn oracle_symbol(text: &str) -> Option<&str> {
    let symbol = text.strip_prefix('{')?.strip_suffix('}')?;
    (!symbol.is_empty() && !symbol.contains(['{', '}'])).then_some(symbol)
}

fn oracle_symbol_sequence(text: &str) -> Option<Vec<String>> {
    let mut symbols = Vec::new();
    let mut rest = text;
    while let Some(after_open) = rest.strip_prefix('{') {
        let close = after_open.find('}')?;
        let symbol = &after_open[..close];
        if symbol.is_empty() || symbol.contains(['{', '}']) {
            return None;
        }
        symbols.push(symbol.to_owned());
        rest = &after_open[close + 1..];
    }
    (rest.is_empty() && symbols.len() > 1).then_some(symbols)
}

fn power_toughness(text: &str) -> Option<(SignedScalar, SignedScalar)> {
    let (power, toughness) = text.split_once('/')?;
    Some((signed_scalar(power)?, signed_scalar(toughness)?))
}

fn signed_scalar(text: &str) -> Option<SignedScalar> {
    let (sign, value) = if let Some(value) = text.strip_prefix('+') {
        (ScalarSign::Plus, value)
    } else if let Some(value) = text.strip_prefix('-').or_else(|| text.strip_prefix('−')) {
        (ScalarSign::Minus, value)
    } else {
        (ScalarSign::None, text)
    };
    let value = match value {
        "X" => ScalarValue::X,
        "*" => ScalarValue::Star,
        value => ScalarValue::Integer(value.parse().ok()?),
    };
    Some(SignedScalar { sign, value })
}

fn number_literal(text: &str) -> Option<(ScalarValue, NumberSpelling)> {
    if text == "X" {
        return Some((
            ScalarValue::X,
            NumberSpelling::EnglishWord(Capitalization::Capitalized),
        ));
    }
    if let Ok(value) = text.parse() {
        return Some((ScalarValue::Integer(value), NumberSpelling::Digits));
    }
    let value = match text.to_ascii_lowercase().as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "thirteen" => 13,
        "fourteen" => 14,
        "fifteen" => 15,
        "sixteen" => 16,
        "seventeen" => 17,
        "eighteen" => 18,
        "nineteen" => 19,
        "twenty" => 20,
        "fifty" => 50,
        _ => return None,
    };
    Some((
        ScalarValue::Integer(value),
        NumberSpelling::EnglishWord(capitalization(text)),
    ))
}

fn quantity_parts(text: &str) -> Option<(String, String)> {
    let (quantity, unit) = text.split_once(' ')?;
    (!unit.is_empty() && !unit.contains(char::is_whitespace) && number_literal(quantity).is_some())
        .then(|| (quantity.to_owned(), unit.to_owned()))
}

fn determined_noun(text: &str) -> Option<(Determiner, String)> {
    let (article, head) = text.split_once(' ')?;
    if head.is_empty() || head.contains(char::is_whitespace) {
        return None;
    }
    let kind = match article.to_ascii_lowercase().as_str() {
        "a" | "an" => DeterminerKind::IndefiniteArticle,
        "the" => DeterminerKind::DefiniteArticle,
        "this" | "that" => DeterminerKind::Demonstrative,
        "each" => DeterminerKind::Distributive,
        "my" | "your" | "his" | "her" | "its" | "our" | "their" => DeterminerKind::Possessive,
        "target" => DeterminerKind::Targeting,
        "all" => DeterminerKind::Universal,
        _ => return None,
    };
    Some((
        Determiner {
            text: article.to_owned(),
            kind,
        },
        head.to_owned(),
    ))
}

fn modified_noun(text: &str) -> Option<(String, String)> {
    let (modifier, head) = text.split_once(' ')?;
    if head.is_empty() || head.contains(char::is_whitespace) {
        return None;
    }
    (color_word(modifier).is_some()
        || lexical_entry(modifier)
            .is_some_and(|(_, part_of_speech)| part_of_speech == PartOfSpeech::Adjective))
    .then(|| (modifier.to_owned(), head.to_owned()))
}

fn color_word(text: &str) -> Option<ColorWord> {
    match text.to_ascii_lowercase().as_str() {
        "white" => Some(ColorWord::White),
        "blue" => Some(ColorWord::Blue),
        "black" => Some(ColorWord::Black),
        "red" => Some(ColorWord::Red),
        "green" => Some(ColorWord::Green),
        _ => None,
    }
}

fn lexical_entry(text: &str) -> Option<(&'static str, PartOfSpeech)> {
    let lower = text.to_ascii_lowercase();
    let entry = match lower.as_str() {
        "ability" | "abilities" => ("ability", PartOfSpeech::Noun),
        "artifact" | "artifacts" => ("artifact", PartOfSpeech::Noun),
        "battlefield" => ("battlefield", PartOfSpeech::Noun),
        "card" | "cards" => ("card", PartOfSpeech::Noun),
        "color" | "colors" => ("color", PartOfSpeech::Noun),
        "commander" | "commanders" => ("commander", PartOfSpeech::Noun),
        "combat" => ("combat", PartOfSpeech::Noun),
        "controller" | "controllers" => ("controller", PartOfSpeech::Noun),
        "counter" | "counters" => ("counter", PartOfSpeech::Noun),
        "copy" | "copies" => ("copy", PartOfSpeech::Noun),
        "creature" | "creatures" => ("creature", PartOfSpeech::Noun),
        "damage" => ("damage", PartOfSpeech::Noun),
        "day" => ("day", PartOfSpeech::Noun),
        "deck" | "decks" => ("deck", PartOfSpeech::Noun),
        "enchantment" | "enchantments" => ("enchantment", PartOfSpeech::Noun),
        "effect" | "effects" => ("effect", PartOfSpeech::Noun),
        "evidence" => ("evidence", PartOfSpeech::Noun),
        "game" | "games" => ("game", PartOfSpeech::Noun),
        "graveyard" | "graveyards" => ("graveyard", PartOfSpeech::Noun),
        "hand" | "hands" => ("hand", PartOfSpeech::Noun),
        "initiative" => ("initiative", PartOfSpeech::Noun),
        "land" | "lands" => ("land", PartOfSpeech::Noun),
        "life" => ("life", PartOfSpeech::Noun),
        "library" | "libraries" => ("library", PartOfSpeech::Noun),
        "mana" => ("mana", PartOfSpeech::Noun),
        "mode" | "modes" => ("mode", PartOfSpeech::Noun),
        "monarch" => ("monarch", PartOfSpeech::Noun),
        "night" => ("night", PartOfSpeech::Noun),
        "number" | "numbers" => ("number", PartOfSpeech::Noun),
        "opponent" | "opponents" => ("opponent", PartOfSpeech::Noun),
        "owner" | "owners" => ("owner", PartOfSpeech::Noun),
        "pile" | "piles" => ("pile", PartOfSpeech::Noun),
        "permanent" | "permanents" => ("permanent", PartOfSpeech::Noun),
        "planeswalker" | "planeswalkers" => ("planeswalker", PartOfSpeech::Noun),
        "player" | "players" => ("player", PartOfSpeech::Noun),
        "power" => ("power", PartOfSpeech::Noun),
        "process" | "processes" => ("process", PartOfSpeech::Noun),
        "rest" => ("rest", PartOfSpeech::Noun),
        "result" | "results" => ("result", PartOfSpeech::Noun),
        "source" | "sources" => ("source", PartOfSpeech::Noun),
        "spell" | "spells" => ("spell", PartOfSpeech::Noun),
        "time" | "times" => ("time", PartOfSpeech::Noun),
        "token" | "tokens" => ("token", PartOfSpeech::Noun),
        "toughness" => ("toughness", PartOfSpeech::Noun),
        "turn" | "turns" => ("turn", PartOfSpeech::Noun),
        "type" | "types" => ("type", PartOfSpeech::Noun),
        "way" | "ways" => ("way", PartOfSpeech::Noun),
        "chaos" => ("chaos", PartOfSpeech::Noun),
        "coin" | "coins" => ("coin", PartOfSpeech::Noun),
        "foe" | "foes" => ("foe", PartOfSpeech::Noun),
        "friend" | "friends" => ("friend", PartOfSpeech::Noun),
        "team" | "teams" => ("team", PartOfSpeech::Noun),
        "able" => ("able", PartOfSpeech::Adjective),
        "attacking" | "attacked" => ("attack", PartOfSpeech::Adjective),
        "bargained" => ("bargain", PartOfSpeech::Adjective),
        "blocked" | "blocking" => ("block", PartOfSpeech::Adjective),
        "chosen" => ("choose", PartOfSpeech::Adjective),
        "collected" => ("collect", PartOfSpeech::Adjective),
        "colorless" => ("colorless", PartOfSpeech::Adjective),
        "copied" => ("copy", PartOfSpeech::Adjective),
        "dealt" => ("deal", PartOfSpeech::Adjective),
        "defending" => ("defend", PartOfSpeech::Adjective),
        "enchanted" => ("enchant", PartOfSpeech::Adjective),
        "equipped" => ("equip", PartOfSpeech::Adjective),
        "even" => ("even", PartOfSpeech::Adjective),
        "foretold" => ("foretell", PartOfSpeech::Adjective),
        "greater" => ("great", PartOfSpeech::Adjective),
        "kicked" => ("kick", PartOfSpeech::Adjective),
        "monstrous" => ("monstrous", PartOfSpeech::Adjective),
        "modified" => ("modify", PartOfSpeech::Adjective),
        "monocolored" => ("monocolored", PartOfSpeech::Adjective),
        "nonbasic" => ("nonbasic", PartOfSpeech::Adjective),
        "noncreature" => ("noncreature", PartOfSpeech::Adjective),
        "odd" => ("odd", PartOfSpeech::Adjective),
        "poisoned" => ("poison", PartOfSpeech::Adjective),
        "prevented" => ("prevent", PartOfSpeech::Adjective),
        "promised" => ("promise", PartOfSpeech::Adjective),
        "renowned" => ("renowned", PartOfSpeech::Adjective),
        "same" => ("same", PartOfSpeech::Adjective),
        "saddled" => ("saddle", PartOfSpeech::Adjective),
        "suspended" => ("suspend", PartOfSpeech::Adjective),
        "tapped" => ("tap", PartOfSpeech::Adjective),
        "untapped" => ("untap", PartOfSpeech::Adjective),
        "alone" => ("alone", PartOfSpeech::Adverb),
        "again" => ("again", PartOfSpeech::Adverb),
        "instead" => ("instead", PartOfSpeech::Adverb),
        "only" => ("only", PartOfSpeech::Adverb),
        "so" => ("so", PartOfSpeech::Adverb),
        "then" => ("then", PartOfSpeech::Adverb),
        "twice" => ("twice", PartOfSpeech::Adverb),
        "and" => ("and", PartOfSpeech::Conjunction),
        "or" => ("or", PartOfSpeech::Conjunction),
        "the" => ("the", PartOfSpeech::Determiner),
        "at" => ("at", PartOfSpeech::Preposition),
        "for" => ("for", PartOfSpeech::Preposition),
        "there" => ("there", PartOfSpeech::Pronoun),
        _ => {
            if let Some(verb) = ordinary_verb(text) {
                return Some((verb.lemma, PartOfSpeech::Verb));
            }
            let (kind, _, _) = auxiliary_parts(text)?;
            let lemma = match kind {
                AuxiliaryKind::Can => "can",
                AuxiliaryKind::Could => "could",
                AuxiliaryKind::Do => "do",
                AuxiliaryKind::May => "may",
                AuxiliaryKind::Might => "might",
                AuxiliaryKind::Must => "must",
                AuxiliaryKind::Shall => "shall",
                AuxiliaryKind::Should => "should",
                AuxiliaryKind::Will => "will",
                AuxiliaryKind::Would => "would",
            };
            return Some((lemma, PartOfSpeech::Verb));
        }
    };
    Some(entry)
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

fn token_is_top_level(source: &str, span: Span, token_start: usize) -> bool {
    let mut state = Nesting::default();
    for character in source[span.start..token_start].chars() {
        state.observe(character);
    }
    state.is_top_level()
}

#[derive(Clone, Copy)]
struct PredicateMatch {
    verb: Span,
    auxiliary: Option<Auxiliary>,
    verb_kind: VerbKind,
}

#[derive(Clone, Copy)]
struct PredicateCoordination {
    conjunction: PredicateConjunction,
    conjunction_span: Span,
    has_comma: bool,
    left_end: usize,
    predicate_match: PredicateMatch,
}

#[derive(Clone, Copy)]
struct ClauseCoordination {
    conjunction: PredicateConjunction,
    conjunction_span: Span,
    has_comma: bool,
    left_end: usize,
    remainder: Span,
}

fn coordinated_clause(
    source: &str,
    span: Span,
    tokens: &[Token],
    catalogs: Option<&Catalogs>,
) -> Option<ClauseCoordination> {
    let text = span.text(source)?;
    let mut state = Nesting::default();
    for (relative, ch) in text.char_indices() {
        if state.is_top_level() {
            let (word, conjunction, remainder_offset) = if text[relative..].starts_with(", then ") {
                ("then", PredicateConjunction::Then, 7)
            } else if text[relative..].starts_with(", and ") {
                ("and", PredicateConjunction::And, 6)
            } else if text[relative..].starts_with(", or ") {
                ("or", PredicateConjunction::Or, 5)
            } else {
                state.observe(ch);
                continue;
            };
            let left_end = span.start + relative;
            let conjunction_start = span.start
                + relative
                + text[relative..relative + remainder_offset]
                    .find(word)
                    .expect("connector pattern contains its word");
            let conjunction_span = Span::new(conjunction_start, conjunction_start + word.len());
            let remainder = trim_span(
                source,
                Span::new(span.start + relative + remainder_offset, span.end),
            );
            let words = tokens
                .iter()
                .filter(|token| {
                    token.span.start >= remainder.start
                        && token.span.end <= remainder.end
                        && matches!(token.kind, TokenKind::Word)
                })
                .collect::<Vec<_>>();
            let first = words
                .first()
                .and_then(|token| token.span.text(source))
                .unwrap_or_default();
            let plausible_subject = matches!(
                first.to_ascii_lowercase().as_str(),
                "each" | "he" | "it" | "she" | "that" | "the" | "they" | "this" | "you"
            );
            if plausible_subject
                && let Some(predicate) = predicate_word(source, remainder, &words, catalogs)
            {
                let predicate_start = predicate
                    .auxiliary
                    .map_or(predicate.verb.start, |auxiliary| auxiliary.span.start);
                let next_comma = find_top_level(source, remainder, ", ").unwrap_or(remainder.end);
                if predicate_start > remainder.start && predicate.verb.start < next_comma {
                    return Some(ClauseCoordination {
                        conjunction,
                        conjunction_span,
                        has_comma: true,
                        left_end,
                        remainder,
                    });
                }
            }
        }
        state.observe(ch);
    }
    None
}

fn coordinated_predicate(
    source: &str,
    span: Span,
    tokens: &[Token],
    catalogs: Option<&Catalogs>,
) -> Option<PredicateCoordination> {
    let text = span.text(source)?;
    let mut state = Nesting::default();
    for (relative, ch) in text.char_indices() {
        if state.is_top_level() {
            let (word, conjunction, remainder_offset) = if text[relative..].starts_with(", then ") {
                ("then", PredicateConjunction::Then, 7)
            } else if text[relative..].starts_with(", and ") {
                ("and", PredicateConjunction::And, 6)
            } else if text[relative..].starts_with(", or ") {
                ("or", PredicateConjunction::Or, 5)
            } else if text[relative..].starts_with(" then ") {
                ("then", PredicateConjunction::Then, 6)
            } else if text[relative..].starts_with(" and ") {
                ("and", PredicateConjunction::And, 5)
            } else if text[relative..].starts_with(" or ") {
                ("or", PredicateConjunction::Or, 4)
            } else {
                state.observe(ch);
                continue;
            };
            let left_end = span.start + relative;
            let has_comma = text[relative..].starts_with(',');
            let conjunction_start = span.start
                + relative
                + text[relative..relative + remainder_offset]
                    .find(word)
                    .expect("connector pattern contains its word");
            let conjunction_span = Span::new(conjunction_start, conjunction_start + word.len());
            let remainder = trim_span(
                source,
                Span::new(span.start + relative + remainder_offset, span.end),
            );
            let words = tokens
                .iter()
                .filter(|token| {
                    token.span.start >= remainder.start
                        && token.span.end <= remainder.end
                        && matches!(token.kind, TokenKind::Word)
                })
                .collect::<Vec<_>>();
            if let Some(predicate_match) = predicate_word(source, remainder, &words, catalogs) {
                let predicate_start = predicate_match
                    .auxiliary
                    .map_or(predicate_match.verb.start, |auxiliary| auxiliary.span.start);
                if predicate_start == remainder.start {
                    return Some(PredicateCoordination {
                        conjunction,
                        conjunction_span,
                        has_comma,
                        left_end,
                        predicate_match,
                    });
                }
            }
        }
        state.observe(ch);
    }
    None
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
    if let Some(action) = catalogs.and_then(|catalogs| {
        catalogs.keyword_action_prefix(clause.text(source).unwrap_or_default())
    }) {
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
        if let Some(auxiliary) = auxiliary(text, token.span) {
            if auxiliary.kind == AuxiliaryKind::Do
                && source[token.span.end..clause.end]
                    .trim_start()
                    .starts_with('~')
            {
                return Some(PredicateMatch {
                    verb: token.span,
                    auxiliary: None,
                    verb_kind: VerbKind::Ordinary,
                });
            }
            let verb = words
                .iter()
                .skip(index + 1)
                .find(|next| !matches!(next.span.text(source), Some("not" | "also")))
                .copied()
                .unwrap_or(token);
            if let Some(length) = catalogs.and_then(|catalogs| {
                keyword_action_verb_length(catalogs, &source[verb.span.start..clause.end], true)
            }) {
                return Some(PredicateMatch {
                    verb: Span::new(verb.span.start, verb.span.start + length),
                    auxiliary: (verb.span != token.span).then_some(auxiliary),
                    verb_kind: VerbKind::KeywordAction,
                });
            }
            return Some(PredicateMatch {
                verb: verb.span,
                auxiliary: (verb.span != token.span).then_some(auxiliary),
                verb_kind: VerbKind::Ordinary,
            });
        }
        if let Some(length) = catalogs.and_then(|catalogs| {
            keyword_action_verb_length(catalogs, &source[token.span.start..clause.end], false)
        }) {
            return Some(PredicateMatch {
                verb: Span::new(token.span.start, token.span.start + length),
                auxiliary: None,
                verb_kind: VerbKind::KeywordAction,
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

fn keyword_action_verb_length(
    catalogs: &Catalogs,
    text: &str,
    include_base_form: bool,
) -> Option<usize> {
    include_base_form
        .then(|| catalogs.keyword_action_prefix(text).map(str::len))
        .flatten()
        .or_else(|| {
            catalogs
                .inflected_keyword_action_prefix(text)
                .map(|(length, _)| length)
        })
}

fn is_auxiliary(word: &str) -> bool {
    auxiliary_parts(word).is_some()
}

fn auxiliary(word: &str, span: Span) -> Option<Auxiliary> {
    let (kind, inflection, negation) = auxiliary_parts(word)?;
    Some(Auxiliary {
        span,
        kind,
        inflection,
        negation,
        capitalization: capitalization(word),
    })
}

fn auxiliary_parts(word: &str) -> Option<(AuxiliaryKind, AuxiliaryInflection, AuxiliaryNegation)> {
    use AuxiliaryInflection::Base;
    use AuxiliaryInflection::ThirdPersonSingular;
    use AuxiliaryKind::Can;
    use AuxiliaryKind::Could;
    use AuxiliaryKind::Do;
    use AuxiliaryKind::May;
    use AuxiliaryKind::Might;
    use AuxiliaryKind::Must;
    use AuxiliaryKind::Shall;
    use AuxiliaryKind::Should;
    use AuxiliaryKind::Will;
    use AuxiliaryKind::Would;
    use AuxiliaryNegation::Contracted;
    use AuxiliaryNegation::Fused;
    use AuxiliaryNegation::None as NoNegation;

    Some(match word.to_ascii_lowercase().as_str() {
        "can" => (Can, Base, NoNegation),
        "can't" => (Can, Base, Contracted),
        "cannot" => (Can, Base, Fused),
        "could" => (Could, Base, NoNegation),
        "do" => (Do, Base, NoNegation),
        "don't" => (Do, Base, Contracted),
        "does" => (Do, ThirdPersonSingular, NoNegation),
        "doesn't" => (Do, ThirdPersonSingular, Contracted),
        "may" => (May, Base, NoNegation),
        "might" => (Might, Base, NoNegation),
        "must" => (Must, Base, NoNegation),
        "shall" => (Shall, Base, NoNegation),
        "should" => (Should, Base, NoNegation),
        "will" => (Will, Base, NoNegation),
        "won't" => (Will, Base, Contracted),
        "would" => (Would, Base, NoNegation),
        _ => return Option::None,
    })
}

#[derive(Clone, Copy)]
struct OrdinaryVerb {
    lemma: &'static str,
    imperative: bool,
    finite: bool,
}

const fn verb(lemma: &'static str, imperative: bool, finite: bool) -> OrdinaryVerb {
    OrdinaryVerb {
        lemma,
        imperative,
        finite,
    }
}

fn ordinary_verb(word: &str) -> Option<OrdinaryVerb> {
    let word = word.to_ascii_lowercase();
    Some(match word.as_str() {
        "add" => verb("add", true, false),
        "adds" => verb("add", false, true),
        "apply" => verb("apply", true, false),
        "applies" => verb("apply", false, true),
        "are" | "is" | "was" | "were" | "it's" => verb("be", false, true),
        "ask" => verb("ask", true, false),
        "attack" => verb("attack", false, false),
        "attacks" => verb("attack", false, true),
        "be" => verb("be", false, false),
        "become" => verb("become", false, false),
        "becomes" => verb("become", false, true),
        "begin" => verb("begin", true, false),
        "begins" => verb("begin", false, true),
        "block" => verb("block", false, false),
        "blocks" => verb("block", false, true),
        "choose" => verb("choose", true, false),
        "chooses" => verb("choose", false, true),
        "cause" => verb("cause", true, false),
        "causes" => verb("cause", false, true),
        "change" => verb("change", true, false),
        "changes" => verb("change", false, true),
        "control" => verb("control", false, false),
        "controls" => verb("control", false, true),
        "cost" | "costs" => verb("cost", false, true),
        "copy" => verb("copy", true, false),
        "copies" => verb("copy", false, true),
        "count" => verb("count", false, false),
        "counts" => verb("count", false, true),
        "deal" => verb("deal", true, false),
        "deals" => verb("deal", false, true),
        "die" => verb("die", false, false),
        "dies" => verb("die", false, true),
        "do" => verb("do", false, false),
        "draw" => verb("draw", true, false),
        "draws" => verb("draw", false, true),
        "draft" => verb("draft", true, false),
        "drafts" => verb("draft", false, true),
        "enter" => verb("enter", false, false),
        "enters" => verb("enter", false, true),
        "flip" => verb("flip", true, false),
        "flips" => verb("flip", false, true),
        "gain" => verb("gain", true, false),
        "gains" => verb("gain", false, true),
        "get" | "gets" => verb("get", false, true),
        "has" | "have" => verb("have", false, true),
        "leave" => verb("leave", false, false),
        "leaves" => verb("leave", false, true),
        "look" => verb("look", true, false),
        "lose" => verb("lose", true, false),
        "loses" => verb("lose", false, true),
        "move" => verb("move", true, false),
        "moves" => verb("move", false, true),
        "own" => verb("own", false, false),
        "owns" => verb("own", false, true),
        "pay" => verb("pay", true, false),
        "pays" => verb("pay", false, true),
        "phase" => verb("phase", true, false),
        "phases" => verb("phase", false, true),
        "prevent" => verb("prevent", true, false),
        "prevents" => verb("prevent", false, true),
        "produce" => verb("produce", true, false),
        "produces" => verb("produce", false, true),
        "put" => verb("put", true, false),
        "puts" => verb("put", false, true),
        "reduce" => verb("reduce", true, false),
        "reduces" => verb("reduce", false, true),
        "remove" => verb("remove", true, false),
        "removes" => verb("remove", false, true),
        "repeat" => verb("repeat", true, false),
        "repeats" => verb("repeat", false, true),
        "reselect" => verb("reselect", true, false),
        "reselects" => verb("reselect", false, true),
        "return" => verb("return", true, false),
        "returns" => verb("return", false, true),
        "roll" => verb("roll", true, false),
        "rolls" => verb("roll", false, true),
        "search" => verb("search", true, false),
        "searches" => verb("search", false, true),
        "skip" => verb("skip", true, false),
        "skips" => verb("skip", false, true),
        "spend" => verb("spend", true, false),
        "spends" => verb("spend", false, true),
        "top" => verb("top", true, false),
        "tops" => verb("top", false, true),
        "target" => verb("target", false, false),
        "targets" => verb("target", false, true),
        "win" => verb("win", false, false),
        "wins" => verb("win", false, true),
        "yell" => verb("yell", true, false),
        _ => return None,
    })
}

fn is_finite_verb(word: &str) -> bool {
    ordinary_verb(word).is_some_and(|verb| verb.finite)
}

fn is_imperative(word: &str) -> bool {
    ordinary_verb(word).is_some_and(|verb| verb.imperative)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CatalogKind;

    fn text(source: &str, span: Span) -> &str {
        span.text(source).unwrap()
    }

    fn phrase_text<'phrase>(_: &str, phrase: &'phrase Phrase) -> &'phrase str {
        phrase.text()
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
                .map(|phrase| phrase_text(source, phrase))
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
        assert_eq!(
            phrase_text(source, ability.ability_word.as_ref().unwrap()),
            "Landfall"
        );
        let AbilityKind::Triggered(trigger) = &ability.kind else {
            panic!("expected triggered ability")
        };
        assert_eq!(trigger.introducer, TriggerWord::Whenever);
        assert_eq!(
            text(source, trigger.event.span),
            "a land enters under your control"
        );
        assert_eq!(
            phrase_text(source, trigger.event.subject.as_ref().unwrap()),
            "a land"
        );
        assert_eq!(
            phrase_text(source, &trigger.event.predicate.as_ref().unwrap().verb),
            "enters"
        );
        assert_eq!(text(source, trigger.effect.span), "draw a card.");
    }

    #[test]
    fn frequent_closed_class_phrases_have_structured_leaves() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);

        let phrase = |text: &str| structured_text(text.to_owned(), Some(&catalogs));
        let verb = |text: &str| structured_verb_text(text.to_owned(), Some(&catalogs));

        for (printed, lemma) in [
            ("enters", "enter"),
            ("deals", "deal"),
            ("gets", "get"),
            ("put", "put"),
            ("is", "be"),
        ] {
            assert_eq!(
                verb(printed),
                Phrase::Lexeme {
                    text: printed.to_owned(),
                    lemma: lemma.to_owned(),
                    part_of_speech: PartOfSpeech::Verb,
                }
            );
        }
        assert_eq!(
            phrase("you"),
            Phrase::Lexeme {
                text: "you".to_owned(),
                lemma: "you".to_owned(),
                part_of_speech: PartOfSpeech::Pronoun,
            }
        );
        assert_eq!(
            phrase("It"),
            Phrase::Lexeme {
                text: "It".to_owned(),
                lemma: "it".to_owned(),
                part_of_speech: PartOfSpeech::Pronoun,
            }
        );
        assert_eq!(
            phrase("black"),
            Phrase::ColorWord {
                text: "black".to_owned(),
                color: ColorWord::Black,
            }
        );
        assert!(matches!(
            phrase("black creature"),
            Phrase::ModifiedNounPhrase(phrase)
                if matches!(
                    phrase.modifier.as_ref(),
                    Phrase::ColorWord {
                        color: ColorWord::Black,
                        ..
                    }
                )
        ));
        assert!(matches!(
            phrase("colorless"),
            Phrase::Lexeme {
                part_of_speech: PartOfSpeech::Adjective,
                ..
            }
        ));
        assert_eq!(
            phrase("~"),
            Phrase::ThisCard {
                text: "~".to_owned(),
                form: ThisCardForm::AbbreviatedName,
            }
        );
        assert_eq!(
            phrase("{T}"),
            Phrase::OracleSymbol {
                text: "{T}".to_owned(),
                symbol: "T".to_owned(),
            }
        );
        assert!(matches!(
            phrase("this creature"),
            Phrase::NounPhrase(phrase)
                if matches!(
                    phrase.as_ref(),
                    NounPhrase {
                        determiner: Determiner {
                            kind: DeterminerKind::Demonstrative,
                            ..
                        },
                        head,
                        ..
                    } if matches!(head.as_ref(), Phrase::CatalogTerm { kind: CatalogKind::CardType, .. })
                )
        ));
        assert!(matches!(
            phrase("a card"),
            Phrase::NounPhrase(phrase)
                if matches!(
                    phrase.as_ref(),
                    NounPhrase {
                        determiner: Determiner {
                            kind: DeterminerKind::IndefiniteArticle,
                            ..
                        },
                        head,
                        ..
                    } if matches!(head.as_ref(), Phrase::Lexeme { part_of_speech: PartOfSpeech::Noun, .. })
                )
        ));
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
    fn modal_punctuation_belongs_to_the_modal_production() {
        let source = "Choose one —\n• Draw two cards.\n• Destroy target artifact or enchantment.";
        let ast = parse(source);
        assert_eq!(ast.abilities.len(), 1);
        let AbilityKind::Modal(modal) = &ast.abilities[0].kind else {
            panic!("expected modal ability")
        };
        assert_eq!(modal.frame, ModalFrame::Unframed);
        assert_eq!(modal.header_suffix, ModalHeaderSuffix::SpacedEmDash);
        assert_eq!(text(source, modal.header.span), "Choose one");
        assert_eq!(modal.modes.len(), 2);
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
                .map(|phrase| phrase_text(activated_source, phrase))
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
        assert_eq!(text(triggered_source, modal.header.span), "choose one");
        assert_eq!(modal.header_suffix, ModalHeaderSuffix::SpacedEmDash);
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
        assert_eq!(text(source, loyalty.cost.span), "[−X]");
        assert_eq!(loyalty.cost.sign, LoyaltyCostSign::Minus);
        assert_eq!(loyalty.cost.value, LoyaltyCostValue::X);
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
        assert_eq!(
            phrase_text(source, clause.subject.as_ref().unwrap()),
            "Target creature"
        );
        assert_eq!(text(source, predicate.auxiliary.unwrap().span), "can't");
        assert_eq!(phrase_text(source, &predicate.verb), "block");
        assert!(predicate.negated);
    }

    #[test]
    fn shared_subject_predicates_are_coordinated() {
        let source = "Other Goblin creatures you control get +1/+1 and have haste.";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        let predicate = clause.predicate.as_ref().unwrap();
        assert_eq!(
            phrase_text(source, clause.subject.as_ref().unwrap()),
            "Other Goblin creatures you control"
        );
        assert_eq!(phrase_text(source, &predicate.verb), "get");
        assert_eq!(
            predicate.complement,
            Some(Phrase::PowerToughness(Box::new(PowerToughness {
                text: "+1/+1".to_owned(),
                power: SignedScalar {
                    sign: ScalarSign::Plus,
                    value: ScalarValue::Integer(1),
                },
                toughness: SignedScalar {
                    sign: ScalarSign::Plus,
                    value: ScalarValue::Integer(1),
                },
            })))
        );
        assert_eq!(
            phrase_text(source, predicate.complement.as_ref().unwrap()),
            "+1/+1"
        );

        assert_eq!(clause.coordinated_predicates.len(), 1);
        let coordinated = &clause.coordinated_predicates[0];
        assert_eq!(coordinated.conjunction, PredicateConjunction::And);
        assert_eq!(text(source, coordinated.conjunction_span), "and");
        assert_eq!(phrase_text(source, &coordinated.predicate.verb), "have");
        assert_eq!(
            phrase_text(source, coordinated.predicate.complement.as_ref().unwrap()),
            "haste"
        );
    }

    #[test]
    fn exact_catalog_terms_are_recognized_outside_ability_position() {
        let source = "Other Goblin creatures you control get +1/+1 and have haste.";
        let catalogs = Catalogs::new(
            ["Haste"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        )
        .with_catalog(CatalogKind::CreatureType, ["Goblin"])
        .with_catalog(CatalogKind::CardType, ["Creature"]);
        let ast = parse_with_catalogs(source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };

        assert_eq!(
            clause.coordinated_predicates[0].predicate.complement,
            Some(Phrase::CatalogTerm {
                text: "haste".to_owned(),
                canonical: "Haste".to_owned(),
                kind: CatalogKind::KeywordAbility,
            })
        );
        assert_eq!(
            clause.subject,
            Some(Phrase::UnknownPhrase(
                "Other Goblin creatures you control".to_owned()
            ))
        );
    }

    #[test]
    fn inflected_keyword_actions_use_grammatical_context() {
        let source = "Target player mills three cards.";
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Counter", "Mill"],
            std::iter::empty::<&str>(),
        );
        let ast = parse_with_catalogs(source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };

        assert!(matches!(
            clause.predicate.as_ref().map(|predicate| &predicate.verb),
            Some(Phrase::CatalogTerm {
                canonical,
                kind: CatalogKind::KeywordAction,
                ..
            }) if canonical == "Mill"
        ));
        assert!(matches!(
            structured_text("counters".to_owned(), Some(&catalogs)),
            Phrase::Lexeme {
                part_of_speech: PartOfSpeech::Noun,
                ..
            }
        ));

        let passive = parse_with_catalogs(
            "Target creature can't be regenerated.",
            &Catalogs::new(
                std::iter::empty::<&str>(),
                ["Regenerate"],
                std::iter::empty::<&str>(),
            ),
        );
        let AbilityKind::Paragraph(paragraph) = &passive.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        assert!(matches!(
            clause
                .predicate
                .as_ref()
                .and_then(|predicate| predicate.complement.as_ref()),
            Some(Phrase::CatalogTerm {
                canonical,
                kind: CatalogKind::KeywordAction,
                ..
            }) if canonical == "Regenerate"
        ));
    }

    #[test]
    fn conjunction_inside_a_complement_is_not_a_coordinated_predicate() {
        let source = "Destroy target artifact and enchantment.";
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Destroy"],
            std::iter::empty::<&str>(),
        );
        let ast = parse_with_catalogs(source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        let predicate = clause.predicate.as_ref().unwrap();
        assert_eq!(
            phrase_text(source, predicate.complement.as_ref().unwrap()),
            "target artifact and enchantment"
        );
        assert!(clause.coordinated_predicates.is_empty());
    }

    #[test]
    fn then_chains_form_sequential_predicates() {
        let source = "Each player discards a card, then loses 1 life, then removes a counter, then gets a poison counter.";
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            ["Discard"],
            std::iter::empty::<&str>(),
        );
        let ast = parse_with_catalogs(source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };

        assert_eq!(
            phrase_text(source, &clause.predicate.as_ref().unwrap().verb),
            "discards"
        );
        assert_eq!(clause.coordinated_predicates.len(), 3);
        assert!(
            clause
                .coordinated_predicates
                .iter()
                .all(|predicate| predicate.conjunction == PredicateConjunction::Then)
        );
        assert_eq!(
            phrase_text(source, &clause.coordinated_predicates[0].predicate.verb),
            "loses"
        );
        assert_eq!(
            phrase_text(source, &clause.coordinated_predicates[2].predicate.verb),
            "gets"
        );
    }

    #[test]
    fn coordination_preserves_a_new_clause_subject() {
        let source = "It becomes a Vehicle, and it gains crew 2.";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };

        assert_eq!(clause.coordinated_clauses.len(), 1);
        let coordinated = &clause.coordinated_clauses[0];
        assert_eq!(coordinated.conjunction, PredicateConjunction::And);
        assert_eq!(
            phrase_text(source, coordinated.clause.subject.as_ref().unwrap()),
            "it"
        );
        assert_eq!(
            phrase_text(source, &coordinated.clause.predicate.as_ref().unwrap().verb),
            "gains"
        );
    }

    #[test]
    fn quoted_granted_rules_are_nested_as_an_ability() {
        let source = "Target creature gains \"Whenever this creature attacks, draw a card.\"";
        let ast = parse(source);
        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!()
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        let complement = clause
            .predicate
            .as_ref()
            .unwrap()
            .complement
            .as_ref()
            .unwrap();
        let embedded = complement
            .embedded_rules()
            .first()
            .expect("quoted rules should remain structured");

        assert!(matches!(embedded.ability.kind, AbilityKind::Triggered(_)));
    }

    #[test]
    fn keyword_argument_can_contain_an_embedded_activated_ability() {
        let source = "Power-up — {W}{U}{B}{R}{G}: Put a +1/+1 counter on this creature.";
        let catalogs = Catalogs::new(
            ["Power-up"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        );
        let ast = parse_with_catalogs(source, &catalogs);
        let AbilityKind::Keyword(keywords) = &ast.abilities[0].kind else { panic!() };
        let argument = keywords.abilities[0].argument.as_ref().unwrap();
        let rules = argument
            .embedded_rules()
            .first()
            .expect("keyword argument should contain nested rules");

        assert!(matches!(rules.ability.kind, AbilityKind::Activated(_)));
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
        let source = "{T}: ~ adds {G}.\n~~ draws 2 cards.";
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
                .any(|token| token.kind == TokenKind::FullSelfReference)
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
            phrase_text(keyword_source, &keyword_list.abilities[1].printed_name),
            "first strike"
        );
        assert_eq!(
            phrase_text(
                keyword_source,
                keyword_list.abilities[2].argument.as_ref().unwrap()
            ),
            "from red"
        );

        let action_source = "Manifest dread 2.";
        let action = parse_with_catalogs(action_source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &action.abilities[0].kind else {
            panic!("expected an ordinary spell paragraph")
        };
        let Clause::Simple(clause) = &paragraph.sentences[0].clause else { panic!() };
        let predicate = clause.predicate.as_ref().unwrap();
        assert_eq!(
            phrase_text(action_source, &predicate.verb),
            "Manifest dread"
        );
        assert_eq!(predicate.verb_kind, VerbKind::KeywordAction);

        let ability_word_source = "Void — Whenever ~ attacks, draw a card.";
        let ability_word = parse_with_catalogs(ability_word_source, &catalogs);
        assert_eq!(
            phrase_text(
                ability_word_source,
                ability_word.abilities[0].ability_word.as_ref().unwrap()
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

    #[test]
    fn reminder_text_is_preserved_whole_and_excluded_from_rules_syntax() {
        let catalogs = Catalogs::new(["Flying"], ["Scry"], std::iter::empty::<&str>());

        let flying_source =
            "Flying (This creature can't be blocked except by creatures with flying or reach.)";
        let flying = parse_with_catalogs(flying_source, &catalogs);
        assert_eq!(flying.abilities[0].reminder_text.len(), 1);
        assert_eq!(
            text(flying_source, flying.abilities[0].reminder_text[0].span),
            "(This creature can't be blocked except by creatures with flying or reach.)"
        );
        let AbilityKind::Keyword(keywords) = &flying.abilities[0].kind else {
            panic!()
        };
        assert!(keywords.abilities[0].argument.is_none());

        let scry_source =
            "Scry 1. (Look at the top card of your library. You may put that card on the bottom.)";
        let scry = parse_with_catalogs(scry_source, &catalogs);
        let AbilityKind::Paragraph(paragraph) = &scry.abilities[0].kind else {
            panic!()
        };
        assert_eq!(paragraph.sentences.len(), 1);
        assert_eq!(text(scry_source, paragraph.sentences[0].span), "Scry 1.");

        let quoted_source = "Create a token named \"(Definitely Rules Text)\". (A reminder.)";
        let quoted = parse(quoted_source);
        assert_eq!(quoted.abilities[0].reminder_text.len(), 1);
        assert_eq!(
            text(quoted_source, quoted.abilities[0].reminder_text[0].span),
            "(A reminder.)"
        );
    }
}
