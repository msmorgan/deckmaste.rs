use super::Nonterminal;
use super::ParsedNonterminal;
use super::VerbDependent;
use super::clause::finish_simple_clause;
use super::parse_nonterminal;
use super::parse_symbol_sequence;
use crate::Numeral;
use crate::Span;
use crate::catalog::CatalogSlot;
use crate::catalog::CatalogValue;
use crate::catalog::Catalogs;
use crate::chart::ChartStats;
use crate::forest::ForestStats;
use crate::forest::ParseCost;
use crate::surface::Punctuation;
use crate::surface::Token;
use crate::surface::TokenKind;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::ActivatedAbility;
use crate::syntax::ClassLevelAbility;
use crate::syntax::Clause;
use crate::syntax::Cost;
use crate::syntax::DependentClause;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordAbilityList;
use crate::syntax::KeywordArgumentSeparator;
use crate::syntax::KeywordListSeparator;
use crate::syntax::LoyaltyAbility;
use crate::syntax::LoyaltyCost;
use crate::syntax::LoyaltyCostSign;
use crate::syntax::LoyaltyCostValue;
use crate::syntax::ModalAbility;
use crate::syntax::ModalFrame;
use crate::syntax::ModalHeaderSuffix;
use crate::syntax::Mode;
use crate::syntax::NumberLiteral;
use crate::syntax::OracleSymbol;
use crate::syntax::OracleText;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::QuotedAbility;
use crate::syntax::RecoveredText;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;
use crate::syntax::SentenceEnding;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::syntax::TriggerEvent;
use crate::syntax::TriggerWord;
use crate::syntax::TriggeredAbility;
use crate::word::ColorWord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AbilityDiagnosticKind {
    OrphanMode,
    EmptyActivationEffect,
    NoCompleteParse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AbilityDiagnostic {
    pub(crate) kind: AbilityDiagnosticKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AbilitySelection {
    pub(crate) span: Span,
    pub(crate) rule: Option<usize>,
    pub(crate) tied_alternatives: Vec<usize>,
    pub(crate) cost: ParseCost,
    pub(crate) chart_stats: ChartStats,
    pub(crate) forest_stats: ForestStats,
}

#[derive(Debug, Default)]
pub(crate) struct AbilityParse {
    pub(crate) ast: OracleText,
    pub(crate) diagnostics: Vec<AbilityDiagnostic>,
    pub(crate) selections: Vec<AbilitySelection>,
}

pub(crate) fn parse_oracle_text(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
) -> AbilityParse {
    Parser::new(source, catalogs).parse(tokens)
}

struct Parser<'source, 'catalogs> {
    source: &'source str,
    catalogs: &'catalogs Catalogs,
    diagnostics: Vec<AbilityDiagnostic>,
    selections: Vec<AbilitySelection>,
}

impl<'source, 'catalogs> Parser<'source, 'catalogs> {
    fn new(source: &'source str, catalogs: &'catalogs Catalogs) -> Self {
        Self {
            source,
            catalogs,
            diagnostics: Vec::new(),
            selections: Vec::new(),
        }
    }

    fn parse(mut self, tokens: &[Token]) -> AbilityParse {
        let lines = split_top_level_lines(tokens);
        let mut abilities = Vec::new();
        let mut line = 0;
        while line < lines.len() {
            let current = lines[line];
            if current.is_empty() {
                line += 1;
                continue;
            }
            if starts_with_bullet(current) {
                self.diagnostics.push(AbilityDiagnostic {
                    kind: AbilityDiagnosticKind::OrphanMode,
                    span: tokens_span(current),
                });
                abilities.push(self.parse_ability(strip_bullet(current)));
                line += 1;
                continue;
            }

            let mut mode_end = line + 1;
            while mode_end < lines.len() && starts_with_bullet(lines[mode_end]) {
                mode_end += 1;
            }
            if mode_end > line + 1 {
                let modes = lines[line + 1..mode_end]
                    .iter()
                    .map(|mode| strip_bullet(mode))
                    .collect::<Vec<_>>();
                abilities.push(self.parse_modal(current, &modes));
                line = mode_end;
            } else {
                abilities.push(self.parse_ability(current));
                line += 1;
            }
        }

        AbilityParse {
            ast: OracleText { abilities },
            diagnostics: self.diagnostics,
            selections: self.selections,
        }
    }

    fn parse_ability(&mut self, tokens: &[Token]) -> Ability {
        let (ability_word, body) = self
            .ability_word_prefix(tokens)
            .map_or((None, tokens), |(word, body)| (Some(word), body));
        let kind = self.parse_ability_kind(body);
        Ability { ability_word, kind }
    }

    fn parse_ability_kind(&mut self, tokens: &[Token]) -> AbilityKind {
        if let Some(keywords) = self.parse_keyword_list(tokens) {
            return AbilityKind::Keyword(keywords);
        }
        if let Some((cost, effect)) = self.loyalty_frame(tokens) {
            return AbilityKind::Loyalty(LoyaltyAbility {
                cost,
                effect: self.parse_paragraph(effect),
            });
        }
        if let Some((introducer, event, intervening_condition, effect)) = self.trigger_frame(tokens)
        {
            return AbilityKind::Triggered(TriggeredAbility {
                introducer,
                event,
                intervening_condition,
                effect: self.parse_paragraph(effect),
            });
        }
        if let Some(colon) = find_top_level_punctuation(tokens, Punctuation::Colon) {
            let cost = self.parse_cost(&tokens[..colon]);
            let effect_tokens = &tokens[colon + 1..];
            if let Some(level) = self.class_level(effect_tokens) {
                return AbilityKind::ClassLevel(ClassLevelAbility { cost, level });
            }
            if effect_tokens.is_empty() {
                self.diagnostics.push(AbilityDiagnostic {
                    kind: AbilityDiagnosticKind::EmptyActivationEffect,
                    span: tokens
                        .get(colon)
                        .map_or_else(|| tokens_span(tokens), |token| token.span),
                });
            }
            return AbilityKind::Activated(ActivatedAbility {
                cost,
                effect: self.parse_paragraph(effect_tokens),
                effect_initial_uppercase: self.tokens_start_uppercase(effect_tokens),
            });
        }
        AbilityKind::Paragraph(self.parse_paragraph(tokens))
    }

    fn class_level(&self, tokens: &[Token]) -> Option<NumberLiteral> {
        let [level, number] = tokens else {
            return None;
        };
        if !self.token_text(level).eq_ignore_ascii_case("level")
            || number.kind != TokenKind::Integer
        {
            return None;
        }
        Some(NumberLiteral {
            value: Numeral::Arabic(false).parse(self.token_text(number)).ok()?,
            numeral: Numeral::Arabic(false),
        })
    }

    fn parse_modal(&mut self, header: &[Token], modes: &[&[Token]]) -> Ability {
        let (ability_word, header) = self
            .ability_word_prefix(header)
            .map_or((None, header), |(word, body)| (Some(word), body));
        let (header, header_suffix) = if matches!(
            header.last().map(|token| token.kind),
            Some(TokenKind::Punctuation(Punctuation::EmDash))
        ) {
            (&header[..header.len() - 1], ModalHeaderSuffix::SpacedEmDash)
        } else {
            (header, ModalHeaderSuffix::None)
        };

        let (frame, header) = if let Some((cost, effect)) = self.loyalty_frame(header) {
            (ModalFrame::Loyalty(cost), effect)
        } else if let Some((introducer, event, intervening_condition, effect)) =
            self.trigger_frame(header)
        {
            (
                ModalFrame::Triggered {
                    introducer,
                    event,
                    intervening_condition,
                },
                effect,
            )
        } else if let Some(colon) = find_top_level_punctuation(header, Punctuation::Colon) {
            (
                ModalFrame::Activated(self.parse_cost(&header[..colon])),
                &header[colon + 1..],
            )
        } else {
            (ModalFrame::Unframed, header)
        };

        Ability {
            ability_word,
            kind: AbilityKind::Modal(ModalAbility {
                frame,
                header: self.parse_paragraph(header),
                header_suffix,
                modes: modes
                    .iter()
                    .map(|mode| Mode {
                        body: self.parse_paragraph(mode),
                    })
                    .collect(),
            }),
        }
    }

    fn ability_word_prefix<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> Option<(crate::catalog::CatalogAtom, &'tokens [Token])> {
        let first = tokens.first()?;
        let suffix = self.source.get(first.span.start..)?;
        self.catalogs
            .matches(suffix, CatalogSlot::AbilityWord)
            .into_iter()
            .filter_map(|catalog_match| {
                let CatalogValue::Atom(atom) = catalog_match.value else {
                    return None;
                };
                let end = first.span.start.checked_add(catalog_match.length)?;
                let matched_end = token_boundary(tokens, end)?;
                let dash = tokens.get(matched_end)?;
                (dash.kind == TokenKind::Punctuation(Punctuation::EmDash)).then_some((
                    catalog_match.length,
                    atom,
                    &tokens[matched_end + 1..],
                ))
            })
            .max_by_key(|(length, _, _)| *length)
            .map(|(_, atom, body)| (atom, body))
    }

    fn loyalty_frame<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> Option<(LoyaltyCost, &'tokens [Token])> {
        if tokens.first()?.kind != TokenKind::Punctuation(Punctuation::OpenBracket) {
            return None;
        }
        let close = tokens
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::CloseBracket))?;
        if tokens.get(close + 1)?.kind != TokenKind::Punctuation(Punctuation::Colon) {
            return None;
        }
        let content = self
            .source
            .get(tokens.first()?.span.end..tokens.get(close)?.span.start)?
            .trim();
        let (sign, value) = if let Some(value) = content.strip_prefix('+') {
            (LoyaltyCostSign::Plus, value)
        } else if let Some(value) = content
            .strip_prefix('-')
            .or_else(|| content.strip_prefix('−'))
        {
            (LoyaltyCostSign::Minus, value)
        } else {
            (LoyaltyCostSign::None, content)
        };
        let value = if value == "X" {
            LoyaltyCostValue::X
        } else {
            LoyaltyCostValue::Number(value.parse().ok()?)
        };
        Some((LoyaltyCost { sign, value }, &tokens[close + 2..]))
    }

    fn trigger_frame<'tokens>(
        &mut self,
        tokens: &'tokens [Token],
    ) -> Option<(
        TriggerWord,
        TriggerEvent,
        Option<DependentClause>,
        &'tokens [Token],
    )> {
        let introducer = match self.token_text(tokens.first()?) {
            text if text.eq_ignore_ascii_case("when") => TriggerWord::When,
            text if text.eq_ignore_ascii_case("whenever") => TriggerWord::Whenever,
            text if text.eq_ignore_ascii_case("at") => TriggerWord::At,
            _ => return None,
        };
        let comma = find_top_level_punctuation(tokens, Punctuation::Comma)?;
        let event_tokens = tokens.get(1..comma)?;
        let event = if introducer == TriggerWord::At {
            let event = self.parse_exact(event_tokens, Nonterminal::NounPhrase)?;
            TriggerEvent::Temporal(event.noun_phrase()?.clone())
        } else {
            let event = self.parse_exact(event_tokens, Nonterminal::SimpleClause)?;
            TriggerEvent::Clause(finish_simple_clause(event.simple_clause()?.clone())?)
        };
        let mut effect = tokens.get(comma + 1..)?;
        let intervening_condition = if effect
            .first()
            .is_some_and(|token| self.token_text(token).eq_ignore_ascii_case("if"))
        {
            let condition_comma = find_top_level_punctuation(effect, Punctuation::Comma)?;
            let condition =
                self.parse_exact(effect.get(1..condition_comma)?, Nonterminal::Clause)?;
            let Clause::Independent(condition) = condition.clause()?.clone() else {
                return None;
            };
            effect = effect.get(condition_comma + 1..)?;
            Some(DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Finite(Box::new(condition)),
            ))
        } else {
            None
        };
        Some((introducer, event, intervening_condition, effect))
    }

    fn parse_cost(&mut self, tokens: &[Token]) -> Cost {
        let components = split_top_level(tokens, &[Punctuation::Comma])
            .into_iter()
            .filter(|component| !component.is_empty())
            .map(|component| self.parse_cost_component(component))
            .collect();
        Cost::Components(components)
    }

    fn parse_cost_component(&mut self, tokens: &[Token]) -> Phrase {
        if tokens.len() == 1 {
            match tokens[0].kind {
                TokenKind::OracleSymbol => {
                    if let Some(symbol) = OracleSymbol::new(self.token_text(&tokens[0])) {
                        return Phrase::OracleSymbol(symbol);
                    }
                }
                TokenKind::SymbolSequence => {
                    if let Some(symbols) = parse_symbol_sequence(self.token_text(&tokens[0])) {
                        return Phrase::SymbolSequence(symbols);
                    }
                }
                _ => {}
            }
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::Clause)
            && let Some(clause) = parsed.clause()
        {
            return Phrase::Clause(Box::new(clause.clone()));
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::NounPhrase)
            && let Some(noun_phrase) = parsed.noun_phrase()
        {
            return Phrase::NounPhrase(Box::new(noun_phrase.clone()));
        }
        self.recovered_phrase(tokens)
    }

    fn parse_paragraph(&mut self, tokens: &[Token]) -> Paragraph {
        Paragraph {
            sentences: split_sentences(self.source, tokens)
                .into_iter()
                .filter(|sentence| !sentence.is_empty())
                .map(|sentence| self.parse_sentence(sentence))
                .collect(),
        }
    }

    fn parse_sentence(&mut self, tokens: &[Token]) -> Sentence {
        let initial_uppercase = self.tokens_start_uppercase(tokens);
        if let Some(sentence) = self.parse_quoted_sentence(tokens) {
            return sentence;
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::Sentence)
            && let Some(sentence) = parsed.sentence()
        {
            let mut sentence = sentence.clone();
            sentence.initial_uppercase = initial_uppercase;
            return sentence;
        }

        let (body, ending) = peel_sentence_ending(tokens);
        self.diagnostics.push(AbilityDiagnostic {
            kind: AbilityDiagnosticKind::NoCompleteParse,
            span: tokens_span(tokens),
        });
        Sentence {
            initial_uppercase,
            body: SentenceBody::Recovered(RecoveredText::new(self.tokens_text(body), tokens.len())),
            ending,
        }
    }

    fn parse_quoted_sentence(&mut self, tokens: &[Token]) -> Option<Sentence> {
        let open = tokens
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote))?;
        let close = tokens[open + 1..]
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote))
            .map(|relative| open + relative + 1)?;
        let (trailing, ending) = peel_sentence_ending(&tokens[close + 1..]);
        if !trailing.is_empty() {
            return None;
        }

        let quoted_tokens = &tokens[open + 1..close];
        let initial_uppercase = self.tokens_start_uppercase(quoted_tokens);
        let quoted = self.parse_ability(quoted_tokens);
        let mut prefix = &tokens[..open];
        let preposition = prefix.last().and_then(|token| {
            self.token_text(token)
                .eq_ignore_ascii_case("with")
                .then_some(Preposition::With)
        });
        if preposition.is_some() {
            prefix = &prefix[..prefix.len() - 1];
        }
        let parsed = self.parse_exact(prefix, Nonterminal::SimpleClause)?;
        let mut clause = parsed.simple_clause()?.clone();
        let dependent = if let Some(preposition) = preposition {
            VerbDependent::Prepositional(PrepositionalPhrase {
                preposition,
                object: Box::new(Phrase::QuotedAbility(Box::new(QuotedAbility {
                    ability: Box::new(quoted),
                    initial_uppercase,
                    closed: true,
                }))),
            })
        } else {
            VerbDependent::PredicateComplement(Phrase::QuotedAbility(Box::new(QuotedAbility {
                ability: Box::new(quoted),
                initial_uppercase,
                closed: true,
            })))
        };
        clause.predicate.dependents.push(dependent);
        Some(Sentence {
            initial_uppercase: self.tokens_start_uppercase(tokens),
            body: SentenceBody::Independent(finish_simple_clause(clause)?),
            ending,
        })
    }

    fn parse_keyword_list(&mut self, tokens: &[Token]) -> Option<KeywordAbilityList> {
        if tokens.is_empty() {
            return None;
        }
        let chunks = split_keyword_items(tokens);
        let has_list_separator = chunks.len() > 1;
        let mut abilities = Vec::with_capacity(chunks.len());
        for (preceding_separator, chunk) in chunks {
            let first = chunk.first()?;
            let suffix = self.source.get(first.span.start..)?;
            let (atom, matched_end) = self
                .catalogs
                .matches(suffix, CatalogSlot::AbilityItem)
                .into_iter()
                .filter_map(|catalog_match| {
                    let CatalogValue::Atom(atom) = catalog_match.value else {
                        return None;
                    };
                    let byte_end = first.span.start.checked_add(catalog_match.length)?;
                    let token_end = token_boundary(chunk, byte_end)?;
                    Some((catalog_match.length, atom, token_end))
                })
                .max_by_key(|(length, _, _)| *length)
                .map(|(_, atom, end)| (atom, end))?;
            let argument_tokens = &chunk[matched_end..];
            if !argument_tokens.is_empty()
                && !has_list_separator
                && !keyword_argument_is_plausible(&atom, argument_tokens, self.source)
            {
                return None;
            }
            let (argument_separator, argument) = if argument_tokens.is_empty() {
                (None, None)
            } else if argument_tokens[0].kind == TokenKind::Punctuation(Punctuation::EmDash) {
                let ability_end = chunk.get(matched_end.checked_sub(1)?)?.span.end;
                let separator = if ability_end < argument_tokens[0].span.start
                    || argument_tokens
                        .get(1)
                        .is_some_and(|next| argument_tokens[0].span.end < next.span.start)
                {
                    KeywordArgumentSeparator::SpacedEmDash
                } else {
                    KeywordArgumentSeparator::EmDash
                };
                let embedded = self.parse_ability(&argument_tokens[1..]);
                (
                    Some(separator),
                    Some(Phrase::EmbeddedAbility(Box::new(embedded))),
                )
            } else {
                (
                    Some(KeywordArgumentSeparator::Space),
                    Some(self.parse_keyword_argument(argument_tokens)),
                )
            };
            abilities.push(KeywordAbility {
                preceding_separator,
                ability: atom,
                argument_separator,
                argument,
            });
        }
        (!abilities.is_empty()).then_some(KeywordAbilityList { abilities })
    }

    fn parse_keyword_argument(&mut self, tokens: &[Token]) -> Phrase {
        let text = self.tokens_text(tokens);
        if text.starts_with('{') && text.len() > 2 && text.ends_with('}') {
            return Phrase::Cost(Cost::SymbolList(text.to_owned()));
        }
        if tokens.len() == 2
            && self.token_text(&tokens[0]).eq_ignore_ascii_case("from")
            && let Some(color) = color_word(self.token_text(&tokens[1]))
        {
            return Phrase::PrepositionalPhrase(Box::new(PrepositionalPhrase {
                preposition: Preposition::From,
                object: Box::new(Phrase::ColorWord(color)),
            }));
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::PrepositionalPhrase)
            && let Some(preposition) = parsed.prepositional_phrase()
        {
            return Phrase::PrepositionalPhrase(Box::new(preposition.clone()));
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::Quantity)
            && let Some(quantity) = parsed.quantity()
        {
            return Phrase::Quantity(*quantity);
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::NounPhrase)
            && let Some(noun_phrase) = parsed.noun_phrase()
        {
            return Phrase::NounPhrase(Box::new(noun_phrase.clone()));
        }
        self.recovered_phrase(tokens)
    }

    fn parse_exact(
        &mut self,
        tokens: &[Token],
        nonterminal: Nonterminal,
    ) -> Option<ParsedNonterminal> {
        if tokens.is_empty() {
            return None;
        }
        let span = tokens_span(tokens);
        let parsed = parse_nonterminal(span.text(self.source)?, self.catalogs, nonterminal).ok()?;
        self.selections.push(AbilitySelection {
            span,
            rule: parsed.root_rule(),
            tied_alternatives: parsed.root_tied_alternatives().to_vec(),
            cost: parsed.cost(),
            chart_stats: parsed.chart_stats(),
            forest_stats: parsed.forest_stats(),
        });
        Some(parsed)
    }

    fn recovered_phrase(&mut self, tokens: &[Token]) -> Phrase {
        self.diagnostics.push(AbilityDiagnostic {
            kind: AbilityDiagnosticKind::NoCompleteParse,
            span: tokens_span(tokens),
        });
        Phrase::Recovered(RecoveredText::new(self.tokens_text(tokens), tokens.len()))
    }

    fn token_text(&self, token: &Token) -> &str {
        token.span.text(self.source).unwrap_or_default()
    }

    fn tokens_text(&self, tokens: &[Token]) -> &str {
        tokens_span(tokens).text(self.source).unwrap_or_default()
    }

    fn tokens_start_uppercase(&self, tokens: &[Token]) -> bool {
        tokens
            .first()
            .and_then(|token| self.token_text(token).chars().next())
            .is_some_and(char::is_uppercase)
    }
}

fn split_top_level_lines(tokens: &[Token]) -> Vec<&[Token]> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        if token.kind == TokenKind::Newline && depth.is_top_level() {
            lines.push(&tokens[start..index]);
            start = index + 1;
        } else {
            depth.observe(token.kind);
        }
    }
    lines.push(&tokens[start..]);
    lines
}

fn split_sentences<'tokens>(source: &str, tokens: &'tokens [Token]) -> Vec<&'tokens [Token]> {
    let mut sentences = Vec::new();
    let mut start = 0;
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        let was_quoted = depth.double_quote;
        depth.observe(token.kind);
        if is_sentence_terminal(token.kind) && depth.is_top_level() {
            sentences.push(&tokens[start..=index]);
            start = index + 1;
            continue;
        }
        if token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote)
            && was_quoted
            && depth.is_top_level()
            && index > start
            && is_sentence_terminal(tokens[index - 1].kind)
            && tokens.get(index + 1).is_some_and(|next| {
                next.kind == TokenKind::Word
                    && next
                        .span
                        .text(source)
                        .and_then(|word| word.chars().next())
                        .is_some_and(char::is_uppercase)
            })
        {
            sentences.push(&tokens[start..=index]);
            start = index + 1;
        }
    }
    if start < tokens.len() {
        sentences.push(&tokens[start..]);
    }
    sentences
}

fn split_top_level<'tokens>(
    tokens: &'tokens [Token],
    separators: &[Punctuation],
) -> Vec<&'tokens [Token]> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        if depth.is_top_level()
            && matches!(token.kind, TokenKind::Punctuation(punctuation) if separators.contains(&punctuation))
        {
            chunks.push(&tokens[start..index]);
            start = index + 1;
        } else {
            depth.observe(token.kind);
        }
    }
    chunks.push(&tokens[start..]);
    chunks
}

fn split_keyword_items(tokens: &[Token]) -> Vec<(Option<KeywordListSeparator>, &[Token])> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let mut preceding = None;
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        let separator = if depth.is_top_level() {
            match token.kind {
                TokenKind::Punctuation(Punctuation::Comma) => Some(KeywordListSeparator::Comma),
                TokenKind::Punctuation(Punctuation::Semicolon) => {
                    Some(KeywordListSeparator::Semicolon)
                }
                _ => None,
            }
        } else {
            None
        };
        if let Some(separator) = separator {
            chunks.push((preceding, &tokens[start..index]));
            preceding = Some(separator);
            start = index + 1;
        } else {
            depth.observe(token.kind);
        }
    }
    chunks.push((preceding, &tokens[start..]));
    chunks
}

fn find_top_level_punctuation(tokens: &[Token], expected: Punctuation) -> Option<usize> {
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        if depth.is_top_level() && token.kind == TokenKind::Punctuation(expected) {
            return Some(index);
        }
        depth.observe(token.kind);
    }
    None
}

fn peel_sentence_ending(tokens: &[Token]) -> (&[Token], SentenceEnding) {
    let Some(last) = tokens.last() else {
        return (tokens, SentenceEnding::None);
    };
    let ending = match last.kind {
        TokenKind::Punctuation(Punctuation::Period) => SentenceEnding::Period(1),
        TokenKind::Punctuation(Punctuation::Exclamation) => SentenceEnding::Exclamation(1),
        TokenKind::Punctuation(Punctuation::Question) => SentenceEnding::Question(1),
        _ => return (tokens, SentenceEnding::None),
    };
    (&tokens[..tokens.len() - 1], ending)
}

fn token_boundary(tokens: &[Token], byte_end: usize) -> Option<usize> {
    tokens
        .iter()
        .position(|token| token.span.end == byte_end)
        .map(|index| index + 1)
}

fn tokens_span(tokens: &[Token]) -> Span {
    match (tokens.first(), tokens.last()) {
        (Some(first), Some(last)) => Span::new(first.span.start, last.span.end),
        _ => Span::default(),
    }
}

fn starts_with_bullet(tokens: &[Token]) -> bool {
    tokens
        .first()
        .is_some_and(|token| token.kind == TokenKind::Bullet)
}

fn strip_bullet(tokens: &[Token]) -> &[Token] {
    if starts_with_bullet(tokens) { &tokens[1..] } else { tokens }
}

fn is_sentence_terminal(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Punctuation(
            Punctuation::Period | Punctuation::Exclamation | Punctuation::Question
        )
    )
}

fn keyword_argument_is_plausible(
    ability: &crate::catalog::CatalogAtom,
    argument: &[Token],
    source: &str,
) -> bool {
    if argument.first().is_some_and(|token| {
        matches!(
            token.kind,
            TokenKind::Integer
                | TokenKind::OracleSymbol
                | TokenKind::SymbolSequence
                | TokenKind::PowerToughness
                | TokenKind::Punctuation(Punctuation::EmDash)
        )
    }) {
        return true;
    }
    let Some(first) = argument.first().and_then(|token| token.span.text(source)) else {
        return false;
    };
    (ability.canonical().eq_ignore_ascii_case("protection") && first.eq_ignore_ascii_case("from"))
        || (ability.canonical().eq_ignore_ascii_case("affinity")
            && first.eq_ignore_ascii_case("for"))
}

fn color_word(surface: &str) -> Option<ColorWord> {
    [
        ColorWord::White,
        ColorWord::Blue,
        ColorWord::Black,
        ColorWord::Red,
        ColorWord::Green,
    ]
    .into_iter()
    .find(|color| color.spelling().eq_ignore_ascii_case(surface))
}

#[derive(Default)]
struct Nesting {
    brackets: usize,
    parentheses: usize,
    double_quote: bool,
}

impl Nesting {
    const fn is_top_level(&self) -> bool {
        self.brackets == 0 && self.parentheses == 0 && !self.double_quote
    }

    fn observe(&mut self, kind: TokenKind) {
        match kind {
            TokenKind::Punctuation(Punctuation::OpenBracket) => self.brackets += 1,
            TokenKind::Punctuation(Punctuation::CloseBracket) => {
                self.brackets = self.brackets.saturating_sub(1);
            }
            TokenKind::Punctuation(Punctuation::OpenParenthesis) => self.parentheses += 1,
            TokenKind::Punctuation(Punctuation::CloseParenthesis) => {
                self.parentheses = self.parentheses.saturating_sub(1);
            }
            TokenKind::Punctuation(Punctuation::DoubleQuote) => {
                self.double_quote = !self.double_quote;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Numeral;
    use crate::catalog::CatalogKind;
    use crate::catalog::Catalogs;
    use crate::parse::DiagnosticKind;
    use crate::parse::ParseReport;
    use crate::parse::parse_with_catalogs;
    use crate::syntax::*;

    #[test]
    fn activated_ability_has_cost_components_and_effect_sentences() {
        let report = parse("{1}{R}, {T}, Sacrifice ~: Draw a card. If you do, discard a card.");
        let AbilityKind::Activated(ability) = &report.ast.abilities[0].kind else {
            panic!("expected activated ability");
        };
        assert!(matches!(
            ability.cost,
            Cost::Components(ref components) if components.len() == 3
        ));
        assert_eq!(ability.effect.sentences.len(), 2);
        assert!(
            matches!(
                ability.effect.sentences[1].body,
                SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                    ref attachments,
                    ..
                })) if matches!(
                    attachments.as_slice(),
                    [ClauseAttachment {
                        position: AttachmentPosition::BeforeMatrix,
                        kind: ClauseAttachmentKind::Dependent(
                            DependentClause::Subordinate(Subordinator::If, _),
                        ),
                        ..
                    }]
                )
            ),
            "{:#?}",
            ability.effect.sentences[1].body
        );
    }

    #[test]
    fn class_level_ability_has_a_cost_and_numeric_level() {
        let source = "{1}{R}: Level 2";
        let report = parse(source);
        let AbilityKind::ClassLevel(level) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a class level ability: {:#?}",
                report.ast.abilities[0]
            );
        };
        assert_eq!(level.level.value, 2);
        assert_eq!(level.level.numeral, Numeral::Arabic(false));
        assert!(matches!(
            level.cost,
            Cost::Components(ref components)
                if matches!(components.as_slice(), [Phrase::SymbolSequence(symbols)] if symbols.len() == 2)
        ));
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
    }

    #[test]
    fn trigger_and_ability_word_are_separate_syntax() {
        let report = parse("Landfall — Whenever a land enters under your control, draw a card.");
        let ability = &report.ast.abilities[0];
        assert_eq!(
            ability
                .ability_word
                .as_ref()
                .map(crate::catalog::CatalogAtom::canonical),
            Some("Landfall")
        );
        let AbilityKind::Triggered(triggered) = &ability.kind else {
            panic!("expected triggered ability");
        };
        assert_eq!(triggered.introducer, TriggerWord::Whenever);
        assert_eq!(
            render(&report),
            "Landfall — Whenever a land enters under your control, draw a card."
        );
    }

    #[test]
    fn condition_in_intervening_position_is_lifted_out_of_the_effect() {
        let report = parse("Whenever ~ attacks, if you control another creature, draw a card.");
        let AbilityKind::Triggered(triggered) = &report.ast.abilities[0].kind else {
            panic!("expected triggered ability");
        };
        assert!(matches!(
            triggered.intervening_condition,
            Some(DependentClause::Subordinate(Subordinator::If, _))
        ));
        assert!(matches!(
            triggered.effect.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(_)))
        ));
        assert_eq!(
            render(&report),
            "Whenever ~ attacks, if you control another creature, draw a card."
        );
    }

    #[test]
    fn modal_punctuation_belongs_to_the_modal_production() {
        let report = parse(
            "Choose one or both —\n• Draw two cards.\n• Destroy target artifact or enchantment.",
        );
        assert_eq!(report.ast.abilities.len(), 1);
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!("expected modal ability");
        };
        assert_eq!(modal.frame, ModalFrame::Unframed);
        assert_eq!(modal.header_suffix, ModalHeaderSuffix::SpacedEmDash);
        assert_eq!(modal.modes.len(), 2);
        assert!(matches!(
            &modal.header.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
                TransitivePredicate {
                    object: PredicateObject::NounPhrase(NounPhrase::Coordinated(
                        CoordinatedNounPhrase {
                            first,
                            rest,
                        }
                    )),
                    ..
                }
            ))) if matches!(
                first.as_ref(),
                NounPhrase::Quantity(Quantity::Exact(number)) if number.value == 1
            ) && matches!(
                rest.as_slice(),
                [NounPhraseCoordination {
                    phrase: NounPhrase::Quantity(Quantity::Both),
                    ..
                }]
            )
        ));
        assert_eq!(
            render(&report),
            "Choose one or both —\n• Draw two cards.\n• Destroy target artifact or enchantment."
        );
    }

    #[test]
    fn activated_and_triggered_modal_headers_keep_their_outer_frames() {
        let activated = parse("{2}, {T}: Choose one —\n• Draw a card.\n• Create a Treasure token.");
        let AbilityKind::Modal(modal) = &activated.ast.abilities[0].kind else {
            panic!("expected activated modal");
        };
        assert!(matches!(modal.frame, ModalFrame::Activated(_)));

        let triggered = parse("Whenever ~ attacks, choose one —\n• Draw a card.\n• Scry 1.");
        let AbilityKind::Modal(modal) = &triggered.ast.abilities[0].kind else {
            panic!("expected triggered modal");
        };
        assert!(matches!(
            modal.frame,
            ModalFrame::Triggered {
                introducer: TriggerWord::Whenever,
                ..
            }
        ));
    }

    #[test]
    fn quoted_granted_ability_does_not_split_its_sentence_or_colon() {
        let report = parse("Create a token with \"{T}: Add {G}.\" Then draw a card.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("quoted colon must not create an activated frame");
        };
        assert_eq!(paragraph.sentences.len(), 2);
    }

    #[test]
    fn postposed_condition_preserves_surface_order() {
        let report = parse("Draw two cards if you control an artifact.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                    ref attachments,
                    ..
                })) if matches!(
                    attachments.as_slice(),
                    [ClauseAttachment {
                        position: AttachmentPosition::AfterMatrix,
                        ..
                    }]
                )
            ),
            "{:#?}",
            paragraph.sentences[0].body
        );
    }

    #[test]
    fn loyalty_cost_is_not_mistaken_for_an_activation_cost() {
        let report = parse("[−X]: Exile each nonland permanent with mana value X or less.");
        let AbilityKind::Loyalty(loyalty) = &report.ast.abilities[0].kind else {
            panic!("expected loyalty ability");
        };
        assert_eq!(loyalty.cost.sign, LoyaltyCostSign::Minus);
        assert_eq!(loyalty.cost.value, LoyaltyCostValue::X);
    }

    #[test]
    fn target_determiner_is_subject_when_a_later_predicate_exists() {
        let report = parse("Target creature can't block this turn.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Deontic(
            Subject(NounPhrase::Nominal(subject)),
            _,
            _,
        )) = &paragraph.sentences[0].body
        else {
            panic!("expected nominal subject");
        };
        assert_eq!(subject.determiner, Some(Determiner::Target(None)));
    }

    #[test]
    fn shared_subject_predicates_are_coordinated() {
        let report = parse("Other Goblin creatures you control get +1/+1 and have haste.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert!(matches!(
            paragraph.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Coordinated(_))
        ));
    }

    #[test]
    fn conjunction_inside_a_complement_is_not_a_coordinated_predicate() {
        let report = parse("Destroy target artifact and enchantment.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(_)))
            ),
            "{:#?}",
            paragraph.sentences[0].body
        );
    }

    #[test]
    fn then_chains_form_sequential_predicates() {
        let report = parse(
            "Each player discards a card, then loses 1 life, then removes a counter, then gets a poison counter.",
        );
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected coordinated clause, got {:#?}",
                paragraph.sentences[0].body
            );
        };
        assert_eq!(coordination.rest.len(), 3);
        assert!(coordination.rest.iter().all(|coordination| {
            coordination.conjunction == Some(PredicateConjunction::Then) && coordination.comma
        }));
    }

    #[test]
    fn asyndetic_predicate_chains_preserve_the_missing_conjunction() {
        let report = parse(
            "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
        );
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected coordinated clause, got {:#?}",
                paragraph.sentences[0].body
            );
        };
        assert_eq!(coordination.rest.len(), 2);
        assert_eq!(coordination.rest[0].conjunction, None);
        assert_eq!(
            coordination.rest[1].conjunction,
            Some(PredicateConjunction::Then)
        );
        assert_eq!(
            render(&report),
            "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle."
        );
    }

    #[test]
    fn coordination_preserves_a_new_clause_subject() {
        let report = parse("It becomes a Vehicle, and it gains crew 2.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected coordinated clause, got {:#?}",
                paragraph.sentences[0].body
            );
        };
        assert!(matches!(
            coordination.rest[0].member,
            CoordinatedClauseMember::Independent(_)
        ));
    }

    #[test]
    fn quoted_granted_rules_are_nested_as_an_ability() {
        let report =
            parse("Target creature gains \"Whenever this creature attacks, draw a card.\"");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected transitive clause");
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::QuotedAbility(quoted)
                if matches!(quoted.ability.kind, AbilityKind::Triggered(_))
        ));
    }

    #[test]
    fn keyword_argument_can_contain_an_embedded_activated_ability() {
        let report = parse("Power-up — {W}{U}{B}{R}{G}: Put a +1/+1 counter on this creature.");
        let AbilityKind::Keyword(keywords) = &report.ast.abilities[0].kind else {
            panic!("expected keyword ability");
        };
        assert!(matches!(
            keywords.abilities[0].argument,
            Some(Phrase::EmbeddedAbility(_))
        ));
    }

    #[test]
    fn contiguous_symbol_keyword_argument_is_a_cost() {
        let report = parse("Morph {2}{W}");
        let AbilityKind::Keyword(keywords) = &report.ast.abilities[0].kind else {
            panic!("expected keyword ability");
        };
        assert!(matches!(
            &keywords.abilities[0].argument,
            Some(Phrase::Cost(Cost::SymbolList(symbols))) if symbols == "{2}{W}"
        ));
        assert_eq!(render(&report), "Morph {2}{W}");
    }

    #[test]
    fn punctuation_inside_a_mid_sentence_quote_stays_nested() {
        let report = parse("Create a token named \"A. B\" and draw a card.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert_eq!(paragraph.sentences.len(), 1);
    }

    #[test]
    fn malformed_delimiters_and_orphan_modes_are_diagnostics_not_parse_failures() {
        let report = parse("• Draw a card (then discard a card.");
        assert_eq!(report.ast.abilities.len(), 1);
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.kind == DiagnosticKind::OrphanMode)
        );
        assert!(report.diagnostics.iter().any(|diagnostic| matches!(
            diagnostic.kind,
            DiagnosticKind::Surface(crate::surface::SurfaceDiagnosticKind::UnclosedParenthesis)
        )));

        let empty = parse("{T}:");
        assert!(
            empty
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.kind == DiagnosticKind::EmptyActivationEffect)
        );
    }

    #[test]
    fn scryfall_catalogs_recognize_keyword_lists_actions_and_ability_words() {
        let keywords = parse("Flying, first strike, protection from red");
        let AbilityKind::Keyword(list) = &keywords.ast.abilities[0].kind else {
            panic!("expected keyword list");
        };
        assert_eq!(list.abilities.len(), 3);
        assert_eq!(list.abilities[0].ability.canonical(), "Flying");
        assert_eq!(list.abilities[1].ability.canonical(), "First strike");
        assert_eq!(list.abilities[2].ability.canonical(), "Protection");

        let action = parse("Manifest dread 2.");
        let AbilityKind::Paragraph(paragraph) = &action.ast.abilities[0].kind else {
            panic!("expected action paragraph");
        };
        let clause = sentence_independent(&paragraph.sentences[0]);
        assert!(matches!(
            predicate_head(clause).verb.verb,
            crate::word::Verb::KeywordAction(_)
        ));

        let ability_word = parse("Void — Whenever ~ attacks, draw a card.");
        assert_eq!(
            ability_word.ast.abilities[0]
                .ability_word
                .as_ref()
                .map(crate::catalog::CatalogAtom::canonical),
            Some("Void")
        );
    }

    #[test]
    fn exact_catalog_terms_are_recognized_outside_ability_position() {
        let report = parse("Other Goblin creatures you control get +1/+1 and have haste.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected coordination");
        };
        let IndependentClause::Transitive(Subject(NounPhrase::Nominal(subject)), _) =
            coordination.first.as_ref()
        else {
            panic!("expected nominal subject");
        };
        assert!(matches!(
            subject.modifiers.as_slice(),
            [NominalModifier::Adjective(_), NominalModifier::Noun(crate::word::NounInstance::Singular(crate::word::Noun::Catalog(goblin)))]
                if goblin.kind == CatalogKind::CreatureType
        ));
    }

    #[test]
    fn affinity_accepts_a_for_prepositional_argument() {
        let catalogs = fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Affinity"]);
        let report = parse_with_catalogs("Affinity for artifacts", &catalogs);
        let AbilityKind::Keyword(list) = &report.ast.abilities[0].kind else {
            panic!("expected a keyword ability: {:#?}", report.ast.abilities[0]);
        };
        assert!(matches!(
            list.abilities.as_slice(),
            [KeywordAbility {
                ability,
                argument: Some(argument),
                ..
            }] if ability.canonical() == "Affinity"
                && matches!(argument, Phrase::PrepositionalPhrase(preposition)
                    if preposition.preposition == Preposition::For)
        ));
        assert_eq!(
            report.ast.render("Test Card", false).unwrap(),
            "Affinity for artifacts"
        );
    }

    #[test]
    fn stripped_input_has_no_reminder_node_to_preserve() {
        let source = crate::strip_reminder_text(
            "Flying (This creature can't be blocked except by creatures with flying or reach.)",
        );
        let report = parse(&source);
        assert_eq!(source, "Flying");
        assert!(matches!(
            report.ast.abilities[0].kind,
            AbilityKind::Keyword(_)
        ));
    }

    #[test]
    fn ability_fixtures_render_without_source_text() {
        for source in [
            "{1}{R}, {T}, Sacrifice ~: Draw a card. If you do, discard a card.",
            "Landfall — Whenever a land enters under your control, draw a card.",
            "Whenever ~ attacks, if you control another creature, draw a card.",
            "Choose one —\n• Draw two cards.\n• Destroy target artifact or enchantment.",
            "{2}, {T}: Choose one —\n• Draw a card.\n• Create a Treasure token.",
            "Whenever ~ attacks, choose one —\n• Draw a card.\n• Scry 1.",
            "Create a token with \"{T}: Add {G}.\" Then draw a card.",
            "Draw two cards if you control an artifact.",
            "[−X]: Exile each nonland permanent with mana value X or less.",
            "Target creature can't block this turn.",
            "Other Goblin creatures you control get +1/+1 and have haste.",
            "Destroy target artifact and enchantment.",
            "Each player discards a card, then loses 1 life, then removes a counter, then gets a poison counter.",
            "It becomes a Vehicle, and it gains crew 2.",
            "Target creature gains \"Whenever this creature attacks, draw a card.\"",
            "Power-up — {W}{U}{B}{R}{G}: Put a +1/+1 counter on this creature.",
            "Create a token named \"A. B\" and draw a card.",
            "Flying, first strike, protection from red",
            "Manifest dread 2.",
            "Void — Whenever ~ attacks, draw a card.",
        ] {
            let ast = parse(source).into_ast();
            assert_eq!(
                ast.render("~", false).expect("AST should render"),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn strict_clause_shapes_cover_the_decision_corpus_fixtures() {
        let catalogs = fixture_catalogs()
            .with_catalog(CatalogKind::KeywordAbility, ["Vigilance"])
            .with_catalog(CatalogKind::SpellType, ["Lesson"]);
        crate::grammar::parse_nonterminal(
            "there's a Lesson card in your graveyard",
            &catalogs,
            crate::grammar::Nonterminal::Clause,
        )
        .expect("the Aang existential condition should parse independently");

        let aang_source = "Aang has vigilance as long as there's a Lesson card in your graveyard.\nWhenever another creature you control dies, put a +1/+1 counter on Aang.";
        let aang_input =
            crate::normalize_self_references(aang_source, "Aang, A Lot to Learn", true);
        let aang = parse_with_catalogs(&aang_input, &catalogs);
        let AbilityKind::Paragraph(aang_static) = &aang.ast.abilities[0].kind else {
            panic!("expected Aang's first ability to be a paragraph");
        };
        assert!(
            matches!(
                &aang_static.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                    attachments,
                    ..
                })) if matches!(
                    attachments.as_slice(),
                    [ClauseAttachment {
                        position: AttachmentPosition::AfterMatrix,
                        kind: ClauseAttachmentKind::Dependent(
                            DependentClause::Subordinate(
                                Subordinator::AsLongAs,
                                SubordinateBody::Finite(condition),
                            ),
                        ),
                        ..
                    }] if matches!(condition.as_ref(), IndependentClause::Existential(_))
                )
            ),
            "{:#?}",
            aang_static.sentences[0].body
        );
        assert_eq!(
            aang.ast.render("Aang, A Lot to Learn", true).unwrap(),
            aang_source,
        );

        let keeper_source = "When this creature enters, you become the monarch.\nAt the beginning of your upkeep, if you're the monarch, creatures you control can't be blocked this turn.";
        let keeper = parse_with_catalogs(keeper_source, &catalogs);
        let AbilityKind::Triggered(keeper_upkeep) = &keeper.ast.abilities[1].kind else {
            panic!("expected Keeper of Keys' second ability to be triggered");
        };
        assert!(matches!(keeper_upkeep.event, TriggerEvent::Temporal(_)));
        assert!(matches!(
            keeper_upkeep.intervening_condition,
            Some(DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Finite(ref condition),
            )) if matches!(condition.as_ref(), IndependentClause::Copular(_, _))
        ));
        assert!(
            matches!(
                keeper_upkeep.effect.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Deontic(_, _, Predicate::Passive(_),))
            ),
            "{:#?}",
            keeper_upkeep.effect.sentences[0].body
        );
        assert_eq!(
            keeper.ast.render("Keeper of Keys", false).unwrap(),
            keeper_source
        );

        let justice_source = "Whenever a spell or ability an opponent controls destroys a noncreature permanent you control, you may destroy target permanent that opponent controls.";
        let justice = parse_with_catalogs(justice_source, &catalogs);
        let AbilityKind::Triggered(justice_trigger) = &justice.ast.abilities[0].kind else {
            panic!("expected Karmic Justice to be triggered");
        };
        assert!(matches!(
            justice_trigger.event,
            TriggerEvent::Clause(IndependentClause::Transitive(_, _))
        ));
        assert!(
            matches!(
                justice_trigger.effect.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Deontic(
                    _,
                    _,
                    Predicate::Transitive(_),
                ))
            ),
            "{:#?}",
            justice_trigger.effect.sentences[0].body
        );
        assert_eq!(
            justice.ast.render("Karmic Justice", false).unwrap(),
            justice_source
        );
    }

    fn parse(source: &str) -> ParseReport {
        parse_with_catalogs(source, &fixture_catalogs())
    }

    fn render(report: &ParseReport) -> String {
        report.ast.render("~", false).expect("AST should render")
    }

    fn sentence_independent(sentence: &Sentence) -> &IndependentClause {
        let SentenceBody::Independent(clause) = &sentence.body else {
            panic!("expected independent sentence, got {:?}", sentence.body);
        };
        clause
    }

    #[allow(
        clippy::match_same_arms,
        reason = "independent predicate variants intentionally share the same head access path"
    )]
    #[allow(
        clippy::unnested_or_patterns,
        reason = "nested pattern variant is equivalent but less readable with this shared head projection"
    )]
    fn predicate_head(clause: &IndependentClause) -> &PredicateHead {
        match clause {
            IndependentClause::Transitive(_, predicate) => &predicate.head,
            IndependentClause::Intransitive(_, predicate) => &predicate.head,
            IndependentClause::Passive(_, predicate) => &predicate.head,
            IndependentClause::Imperative(Predicate::Transitive(predicate)) => &predicate.head,
            IndependentClause::Imperative(Predicate::Intransitive(predicate)) => &predicate.head,
            IndependentClause::Imperative(Predicate::Passive(predicate)) => &predicate.head,
            other => panic!("expected lexical predicate, got {other:?}"),
        }
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(
                CatalogKind::KeywordAbility,
                [
                    "Flying",
                    "First strike",
                    "Protection",
                    "Max speed",
                    "Forecast",
                    "Power-up",
                    "Haste",
                    "Crew",
                    "Morph",
                ],
            )
            .with_catalog(
                CatalogKind::KeywordAction,
                ["Scry", "Manifest dread", "Fight", "Destroy", "Discard"],
            )
            .with_catalog(CatalogKind::AbilityWord, ["Landfall", "Void"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(CatalogKind::ArtifactType, ["Treasure", "Vehicle"])
            .with_catalog(
                CatalogKind::CardType,
                ["Creature", "Land", "Artifact", "Enchantment"],
            )
    }
}
