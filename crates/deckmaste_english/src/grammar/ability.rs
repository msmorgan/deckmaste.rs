use super::Nonterminal;
use super::ParsedNonterminal;
use super::VerbDependent;
use super::clause::finish_simple_clause;
use super::parse_nonterminal_with_self_reference;
use super::parse_symbol_sequence;
use crate::Numeral;
use crate::Span;
use crate::catalog::CatalogAtom;
use crate::catalog::CatalogSlot;
use crate::catalog::CatalogValue;
use crate::catalog::Catalogs;
use crate::chart::ChartStats;
use crate::forest::ForestStats;
use crate::forest::ParseCost;
use crate::identity::SelfReference;
use crate::surface::Punctuation;
use crate::surface::Token;
use crate::surface::TokenKind;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::ActivatedAbility;
use crate::syntax::ChapterAbility;
use crate::syntax::ChoiceInstruction;
use crate::syntax::ChoiceTrigger;
use crate::syntax::ClassLevelAbility;
use crate::syntax::Clause;
use crate::syntax::Cost;
use crate::syntax::DependentClause;
use crate::syntax::FlavorHeader;
use crate::syntax::IndependentClause;
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
use crate::syntax::ModeHeading;
use crate::syntax::NumberLiteral;
use crate::syntax::OracleSymbol;
use crate::syntax::OracleText;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::Predicate;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::QuotedAbility;
use crate::syntax::RecoveredText;
use crate::syntax::RollRange;
use crate::syntax::RollRowAbility;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::syntax::TriggerEvent;
use crate::syntax::TriggerWord;
use crate::syntax::TriggeredAbility;
use crate::word::ColorWord;
use crate::word::Verb;
use crate::word::VerbSlot;
use crate::word::Vocab;

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
    self_reference: &SelfReference,
) -> AbilityParse {
    Parser::new(source, catalogs, self_reference).parse(tokens)
}

struct Parser<'source, 'catalogs, 'sr> {
    source: &'source str,
    catalogs: &'catalogs Catalogs,
    self_reference: &'sr SelfReference,
    diagnostics: Vec<AbilityDiagnostic>,
    selections: Vec<AbilitySelection>,
}

impl<'source, 'catalogs, 'sr> Parser<'source, 'catalogs, 'sr> {
    fn new(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        self_reference: &'sr SelfReference,
    ) -> Self {
        Self {
            source,
            catalogs,
            self_reference,
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
        if let Some((chapters, effect)) = self.chapter_frame(tokens) {
            return AbilityKind::Chapter(ChapterAbility {
                chapters,
                body: self.parse_paragraph(effect),
            });
        }
        if let Some((range, body)) = self.roll_row_frame(tokens) {
            return AbilityKind::RollRow(RollRowAbility {
                range,
                body: self.parse_paragraph(body),
            });
        }
        AbilityKind::Paragraph(self.parse_paragraph(tokens))
    }

    /// Splits a die-roll result-table row (`20 | …`, `2—9 | …`, `15+ | …`,
    /// `9 or less | …`) into its face-value [`RollRange`] and the body after
    /// the spaced ` | ` separator. The pipe lexes as
    /// [`Punctuation::Other`]`('|')`, and the separator must be exactly ` |
    /// ` so the renderer reproduces it verbatim. The whole range prefix is
    /// structural, so nothing recovers at the row key; the body parses as
    /// an ordinary paragraph. Dispatched after the chapter frame and before
    /// the flavor-header fallback, so a row whose body opens with a flavor
    /// header (`1 | Trapped! — …`) keeps that header on the body paragraph
    /// rather than swallowing the `1 | ` key into it.
    fn roll_row_frame<'a>(&self, tokens: &'a [Token]) -> Option<(RollRange, &'a [Token])> {
        let pipe = tokens
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::Other('|')))?;
        if pipe == 0 || pipe + 1 >= tokens.len() {
            return None;
        }
        let before_end = tokens[pipe - 1].span.end;
        let after_start = tokens[pipe + 1].span.start;
        if self.source.get(before_end..after_start) != Some(" | ") {
            return None;
        }
        let range = self.roll_range(&tokens[..pipe])?;
        Some((range, &tokens[pipe + 1..]))
    }

    /// Parses a die-roll row's face-value key from the tokens before the ` | `.
    /// Every surface shape the supported corpus prints is carried structurally:
    /// a single face, an inclusive em-dash or hyphen span, an at-least `+`
    /// threshold, or an `or less` at-most threshold. Anything else is not a row
    /// key and the line falls through to the paragraph path.
    fn roll_range(&self, tokens: &[Token]) -> Option<RollRange> {
        match tokens {
            [single] if single.kind == TokenKind::Integer => {
                Some(RollRange::Single(self.arabic_literal(single)?))
            }
            // The normalized corpus prints every inclusive range with an
            // unspaced en dash; an em dash is tolerated too so an un-normalized
            // surface still parses as a row (it renders back as an en dash).
            [low, dash, high]
                if low.kind == TokenKind::Integer
                    && matches!(
                        dash.kind,
                        TokenKind::Punctuation(Punctuation::EnDash | Punctuation::EmDash)
                    )
                    && high.kind == TokenKind::Integer =>
            {
                Some(RollRange::Inclusive {
                    low: self.arabic_literal(low)?,
                    high: self.arabic_literal(high)?,
                })
            }
            [value, plus]
                if value.kind == TokenKind::Integer
                    && plus.kind == TokenKind::Punctuation(Punctuation::Plus) =>
            {
                Some(RollRange::OrMore(self.arabic_literal(value)?))
            }
            [value, or, less]
                if value.kind == TokenKind::Integer
                    && or.kind == TokenKind::Word
                    && less.kind == TokenKind::Word
                    && self.token_text(or).eq_ignore_ascii_case("or")
                    && self.token_text(less).eq_ignore_ascii_case("less") =>
            {
                Some(RollRange::OrLess(self.arabic_literal(value)?))
            }
            // An ASCII-hyphen inclusive span (`1-9`) is a single word token
            // because the hyphen is a word connector; split it on the hyphen.
            // Normalization rewrites this surface to an en dash upstream, so
            // this arm only fires for un-normalized input; either way the row
            // renders back with an en dash.
            [word] if word.kind == TokenKind::Word => {
                let (low, high) = self.token_text(word).split_once('-')?;
                Some(RollRange::Inclusive {
                    low: arabic_number_literal(low)?,
                    high: arabic_number_literal(high)?,
                })
            }
            _ => None,
        }
    }

    fn arabic_literal(&self, token: &Token) -> Option<NumberLiteral> {
        arabic_number_literal(self.token_text(token))
    }

    /// Splits a saga chapter header (`I — …`, `I, II — …`) into its list of
    /// chapter numbers and the effect body after the spaced em dash. The header
    /// is a comma-separated list of Roman numerals; every distinction is
    /// carried by the returned [`NumberLiteral`]s, so nothing recovers.
    fn chapter_frame<'a>(&self, tokens: &'a [Token]) -> Option<(Vec<NumberLiteral>, &'a [Token])> {
        let em_dash = tokens
            .iter()
            .position(|token| matches!(token.kind, TokenKind::Punctuation(Punctuation::EmDash)))?;
        if em_dash == 0 || em_dash + 1 >= tokens.len() {
            return None;
        }
        let chapters = self.chapter_numbers(&tokens[..em_dash])?;
        Some((chapters, &tokens[em_dash + 1..]))
    }

    /// Parses a chapter header's comma-separated Roman-numeral list. Each
    /// comma-delimited group must be exactly one canonical Roman numeral;
    /// anything else (a bare word, a multi-token group, an empty group from a
    /// trailing comma) makes this not a chapter header.
    fn chapter_numbers(&self, header: &[Token]) -> Option<Vec<NumberLiteral>> {
        if header.is_empty() {
            return None;
        }
        split_top_level(header, &[Punctuation::Comma])
            .into_iter()
            .map(|group| {
                let [token] = group else {
                    return None;
                };
                let value = Numeral::Roman.parse(self.token_text(token)).ok()?;
                Some(NumberLiteral {
                    value,
                    numeral: Numeral::Roman,
                })
            })
            .collect()
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
        } else if let Some((chapters, effect)) = self.chapter_frame(header) {
            (ModalFrame::Chapter(chapters), effect)
        } else if let Some(atom) = self.keyword_frame(header) {
            (ModalFrame::Keyword(atom), &header[header.len()..])
        } else {
            (ModalFrame::Unframed, header)
        };

        Ability {
            ability_word,
            kind: AbilityKind::Modal(ModalAbility {
                frame,
                header: self.parse_choice_header(header),
                header_suffix,
                modes: modes.iter().map(|mode| self.parse_mode(mode)).collect(),
            }),
        }
    }

    /// Recognizes a bare keyword-ability header (`Tiered`) that stands in for a
    /// `Choose …` instruction. The atom must consume the whole header with no
    /// argument; anything trailing is an ordinary header, not a keyword frame.
    fn keyword_frame(&self, header: &[Token]) -> Option<CatalogAtom> {
        let first = header.first()?;
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
                let token_end = token_boundary(header, byte_end)?;
                Some((catalog_match.length, atom, token_end))
            })
            .max_by_key(|(length, _, _)| *length)
            .map(|(_, atom, end)| (atom, end))?;
        (matched_end == header.len()).then_some(atom)
    }

    /// Parses one bulleted mode. A [`Tiered`](ModalFrame::Keyword) mode carries
    /// a `<name> — <cost> — ` heading before its body; every other mode is
    /// a bare body paragraph. The heading is only peeled when the name is
    /// followed by a spaced em dash, a cost, and a second spaced em dash,
    /// so an ordinary mode whose body merely contains an em dash is left
    /// intact.
    fn parse_mode(&mut self, tokens: &[Token]) -> Mode {
        if let Some((heading, body)) = self.mode_heading(tokens) {
            Mode {
                heading: Some(heading),
                body: self.parse_paragraph(body),
            }
        } else {
            Mode {
                heading: None,
                body: self.parse_paragraph(tokens),
            }
        }
    }

    /// Peels a tiered mode's `<name> — <cost> — ` heading: an opaque name run,
    /// a spaced em dash, a mana cost, and a second spaced em dash. Returns
    /// the heading and the remaining body tokens, or `None` when the shape
    /// does not match.
    fn mode_heading<'tokens>(
        &mut self,
        tokens: &'tokens [Token],
    ) -> Option<(ModeHeading, &'tokens [Token])> {
        let label_end = Self::spaced_em_dash(tokens, 0)?;
        if label_end == 0 {
            return None;
        }
        let cost_start = label_end + 1;
        let cost_end = Self::spaced_em_dash(tokens, cost_start)?;
        let cost_tokens = tokens.get(cost_start..cost_end)?;
        let body = tokens.get(cost_end + 1..)?;
        if cost_tokens.is_empty() || body.is_empty() {
            return None;
        }
        let cost = self.parse_cost(cost_tokens);
        if !matches!(&cost, Cost::Components(components) if components
            .iter()
            .all(|phrase| matches!(phrase, Phrase::OracleSymbol(_) | Phrase::SymbolSequence(_))))
        {
            return None;
        }
        let label_tokens = tokens.get(..label_end)?;
        let label = FlavorHeader::new(self.tokens_text(label_tokens), label_tokens.len());
        Some((ModeHeading { label, cost }, body))
    }

    /// Position of a spaced em dash (` — `) at or after `from`, or `None`. The
    /// em dash must have surrounding space on both sides so an unspaced em
    /// dash inside a name (`Cross-Slash`) never splits a heading.
    fn spaced_em_dash(tokens: &[Token], from: usize) -> Option<usize> {
        (from..tokens.len()).find(|&index| {
            let token = &tokens[index];
            token.kind == TokenKind::Punctuation(Punctuation::EmDash)
                && index > 0
                && tokens[index - 1].span.end < token.span.start
                && tokens
                    .get(index + 1)
                    .is_some_and(|next| token.span.end < next.span.start)
        })
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
        } else if let Some(clause) = self
            .parse_exact(event_tokens, Nonterminal::SimpleClause)
            .and_then(|event| event.simple_clause().cloned())
            .and_then(finish_simple_clause)
        {
            TriggerEvent::Clause(clause)
        } else {
            // A coordinated event (`Ashcoat enters or attacks`, `this creature
            // enters or the creature it haunts dies`) reduces only through the general
            // clause nonterminal; the simple-clause frame parse above rejects
            // the conjunction. Admit exactly the coordinated shape here so a
            // single-clause event keeps its existing simple-clause parse.
            TriggerEvent::Clause(self.coordinated_event(event_tokens)?)
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

    /// Parses a coordinated trigger event (`this creature enters or dies`,
    /// `Ashcoat enters or attacks`, `this creature enters or the creature it
    /// haunts dies`) as a single [`IndependentClause::Coordinated`]. Restricted
    /// to the coordinated shape so a single-clause event is never re-parsed
    /// here — it keeps its more specific simple-clause frame parse.
    fn coordinated_event(&mut self, tokens: &[Token]) -> Option<IndependentClause> {
        let parsed = self.parse_exact(tokens, Nonterminal::Clause)?;
        match parsed.clause()? {
            Clause::Independent(clause @ IndependentClause::Coordinated(_)) => Some(clause.clone()),
            _ => None,
        }
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
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::Sentence)
            && let Some(sentence) = parsed.sentence()
            && let SentenceBody::Independent(independent) = &sentence.body
        {
            return Phrase::Clause(Box::new(Clause::Independent(independent.clone())));
        }
        if let Some(parsed) = self.parse_exact(tokens, Nonterminal::NounPhrase)
            && let Some(noun_phrase) = parsed.noun_phrase()
        {
            return Phrase::NounPhrase(Box::new(noun_phrase.clone()));
        }
        self.recovered_phrase(tokens)
    }

    fn parse_paragraph(&mut self, tokens: &[Token]) -> Paragraph {
        let (flavor_header, body) = self.peel_flavor_header(tokens);
        Paragraph {
            flavor_header,
            sentences: split_sentences(self.source, body)
                .into_iter()
                .filter(|sentence| !sentence.is_empty())
                .map(|sentence| self.parse_sentence(sentence))
                .collect(),
        }
    }

    /// Peels a licensed flavor junk-before-dash header from the front of a
    /// paragraph. A flavor header is an arbitrary token run terminated by a
    /// spaced em dash (` — `) in header position. Structural em-dash headers —
    /// ability words and saga chapter headers — are consumed by
    /// [`Self::ability_word_prefix`] and [`Self::chapter_frame`] before a
    /// paragraph is parsed, so the peel only fires as a staged fallback (never
    /// competing with those exact parses).
    ///
    /// Two guards keep the peel off the mid-rules em dashes that punctuate real
    /// text (`… faces a villainous choice — You draw a card`), mode labels
    /// (`Run and Hide — Prevent …`), and die-roll ranges (`2—9`):
    /// - the em dash must be *spaced* (` — `), excluding unspaced ranges; and
    /// - the run before it must end in inert flavor terminal punctuation (`!`,
    ///   `?`, or an ellipsis), never a word or a lone period.
    fn peel_flavor_header<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> (Option<FlavorHeader>, &'tokens [Token]) {
        let mut depth = Nesting::default();
        for (index, token) in tokens.iter().enumerate() {
            if depth.is_top_level() && token.kind == TokenKind::Punctuation(Punctuation::EmDash) {
                // Only the first top-level em dash can begin a header body; if it
                // does not qualify, the run is not a flavor header.
                if index == 0 || index + 1 >= tokens.len() {
                    return (None, tokens);
                }
                let dash = self.token_text(token);
                let before_end = tokens[index - 1].span.end;
                let after_start = tokens[index + 1].span.start;
                let separator = self.source.get(before_end..after_start);
                if separator != Some(&format!(" {dash} ")) {
                    return (None, tokens);
                }
                if !ends_in_flavor_terminal(&tokens[..index]) {
                    return (None, tokens);
                }
                let header_start = tokens[0].span.start;
                let Some(text) = self.source.get(header_start..before_end) else {
                    return (None, tokens);
                };
                let header = FlavorHeader::new(text, index);
                return (Some(header), &tokens[index + 1..]);
            }
            depth.observe(token.kind);
        }
        (None, tokens)
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

        self.diagnostics.push(AbilityDiagnostic {
            kind: AbilityDiagnosticKind::NoCompleteParse,
            span: tokens_span(tokens),
        });
        // A recovered span is reproduced verbatim, INCLUDING any terminal
        // period, so its punctuation round-trips without derivation. The
        // renderer therefore never appends a period to a recovered sentence.
        // (`source_tokens` still counts the whole span, so the recovery census
        // is unchanged.)
        Sentence {
            initial_uppercase,
            body: SentenceBody::Recovered(RecoveredText::new(
                self.tokens_text(tokens),
                tokens.len(),
            )),
        }
    }

    /// Parses a modal ability's header like [`Self::parse_paragraph`], but each
    /// sentence is first offered to the choice-instruction production. This is
    /// scoped to modal headers so the `Choose one` instruction and its optional
    /// trigger prefix are recognized structurally there without letting the
    /// trigger-prefix grammar over-claim ordinary `When …, draw a card.`
    /// abilities elsewhere.
    fn parse_choice_header(&mut self, tokens: &[Token]) -> Paragraph {
        let (flavor_header, body) = self.peel_flavor_header(tokens);
        Paragraph {
            flavor_header,
            sentences: split_sentences(self.source, body)
                .into_iter()
                .filter(|sentence| !sentence.is_empty())
                .map(|sentence| self.parse_choice_sentence(sentence))
                .collect(),
        }
    }

    /// Parses one modal-header sentence, trying the choice instruction first
    /// and falling back to the ordinary sentence parse (so follow-up
    /// sentences such as `Each mode must target a different player.` are
    /// unaffected).
    fn parse_choice_sentence(&mut self, tokens: &[Token]) -> Sentence {
        let initial_uppercase = self.tokens_start_uppercase(tokens);
        let body = peel_sentence_ending(tokens);
        if let Some(choice) = self.parse_choice_instruction(body) {
            return Sentence {
                initial_uppercase,
                body: SentenceBody::Choice(choice),
            };
        }
        self.parse_sentence(tokens)
    }

    /// Parses a `[When …,] choose <quantity> [at random]` choice instruction.
    /// Returns `None` (so the caller recovers the sentence unchanged) unless
    /// the core is a `choose` imperative. Every surface distinction is
    /// carried structurally: the trigger prefix as a chart-parsed trigger
    /// clause, the quantity as the imperative's object, and `at random` as
    /// a flag.
    fn parse_choice_instruction(&mut self, tokens: &[Token]) -> Option<ChoiceInstruction> {
        let (trigger_prefix, rest) = self.peel_choice_trigger(tokens);
        let (core, at_random) = self.peel_at_random(rest);
        let imperative = self.parse_choice_core(core)?;
        Some(ChoiceInstruction {
            trigger_prefix,
            imperative,
            at_random,
        })
    }

    /// Peels an optional leading trigger clause (`When …,`, `Whenever …,`,
    /// `At …,`) from a choice instruction, parsing its event with the chart.
    /// This fires for a reflexive second trigger (`When you do, …`) heading a
    /// non-initial header sentence; an ability-initial trigger is absorbed by
    /// the outer frame first. The chart admits coordinated events either way.
    /// Returns the original slice unchanged when there is no parseable trigger
    /// prefix.
    fn peel_choice_trigger<'tokens>(
        &mut self,
        tokens: &'tokens [Token],
    ) -> (Option<Box<ChoiceTrigger>>, &'tokens [Token]) {
        let Some(first) = tokens.first() else {
            return (None, tokens);
        };
        let introducer = match self.token_text(first) {
            text if text.eq_ignore_ascii_case("when") => TriggerWord::When,
            text if text.eq_ignore_ascii_case("whenever") => TriggerWord::Whenever,
            text if text.eq_ignore_ascii_case("at") => TriggerWord::At,
            _ => return (None, tokens),
        };
        let Some(comma) = find_top_level_punctuation(tokens, Punctuation::Comma) else {
            return (None, tokens);
        };
        let Some(event_tokens) = tokens.get(1..comma) else {
            return (None, tokens);
        };
        let event = if introducer == TriggerWord::At {
            let Some(parsed) = self.parse_exact(event_tokens, Nonterminal::NounPhrase) else {
                return (None, tokens);
            };
            let Some(phrase) = parsed.noun_phrase() else {
                return (None, tokens);
            };
            TriggerEvent::Temporal(phrase.clone())
        } else {
            let Some(parsed) = self.parse_exact(event_tokens, Nonterminal::Clause) else {
                return (None, tokens);
            };
            let Some(Clause::Independent(clause)) = parsed.clause() else {
                return (None, tokens);
            };
            TriggerEvent::Clause(clause.clone())
        };
        let Some(rest) = tokens.get(comma + 1..) else {
            return (None, tokens);
        };
        (
            Some(Box::new(ChoiceTrigger {
                introducer,
                event,
                intervening_condition: None,
            })),
            rest,
        )
    }

    /// Peels a trailing `at random` adverbial from a choice instruction's core.
    fn peel_at_random<'tokens>(&self, tokens: &'tokens [Token]) -> (&'tokens [Token], bool) {
        let [.., at, random] = tokens else {
            return (tokens, false);
        };
        if at.kind == TokenKind::Word
            && random.kind == TokenKind::Word
            && self.token_text(at).eq_ignore_ascii_case("at")
            && self.token_text(random).eq_ignore_ascii_case("random")
        {
            (&tokens[..tokens.len() - 2], true)
        } else {
            (tokens, false)
        }
    }

    /// Parses a choice instruction's core, requiring a `choose` imperative. Any
    /// other imperative (a follow-up `Create …`) or clause returns `None`, so
    /// the trigger-prefix grammar cannot over-claim a non-choice sentence.
    fn parse_choice_core(&mut self, tokens: &[Token]) -> Option<Predicate> {
        let parsed = self.parse_exact(tokens, Nonterminal::Sentence)?;
        let sentence = parsed.sentence()?;
        let SentenceBody::Independent(IndependentClause::Imperative(predicate)) = &sentence.body
        else {
            return None;
        };
        predicate_is_choose(predicate).then(|| predicate.clone())
    }

    /// A quoted ability (`"..."`) may fill any grammatical slot the oracle
    /// licenses it in, not just a `with` postmodifier: the direct object of a
    /// grant verb (`this creature has "..."`, `this creature gains "..."`)
    /// shares the slot too. The quoted text parses recursively as an
    /// ability, and its interior parse failures recover at the
    /// embedded-rules role (the `syntax` visitor tags them) without
    /// poisoning this outer clause — recovery there is a reclassification,
    /// not a whole-clause loss. The quote must occupy the tail (only a
    /// sentence ending may follow the closing quote); anything after it is
    /// a different construction and this production declines.
    fn parse_quoted_sentence(&mut self, tokens: &[Token]) -> Option<Sentence> {
        let open = tokens
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote))?;
        let close = tokens[open + 1..]
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote))
            .map(|relative| open + relative + 1)?;
        let trailing = peel_sentence_ending(&tokens[close + 1..]);
        if !trailing.is_empty() {
            return None;
        }

        let quoted_tokens = &tokens[open + 1..close];
        let initial_uppercase = self.tokens_start_uppercase(quoted_tokens);
        let quoted = QuotedAbility {
            ability: Box::new(self.parse_ability(quoted_tokens)),
            initial_uppercase,
            closed: true,
        };
        let prefix = &tokens[..open];
        let clause = if prefix
            .last()
            .is_some_and(|token| self.token_text(token).eq_ignore_ascii_case("with"))
        {
            self.quoted_with_clause(&prefix[..prefix.len() - 1], quoted)?
        } else {
            self.quoted_grant_object_clause(prefix, quoted)?
        };
        Some(Sentence {
            initial_uppercase: self.tokens_start_uppercase(tokens),
            body: SentenceBody::Independent(clause),
        })
    }

    /// Attaches a quoted ability as the object of a `with` postmodifier on the
    /// clause the prefix spells (`create a ... token with "..."`). A
    /// subjectless imperative prefix, parsed in isolation, resolves its
    /// base-form verb to the infinitive slot; coerce it to the imperative a
    /// standalone effect clause is, so `finish_simple_clause` accepts the
    /// subjectless clause.
    fn quoted_with_clause(
        &mut self,
        prefix: &[Token],
        quoted: QuotedAbility,
    ) -> Option<IndependentClause> {
        let mut clause = self
            .parse_exact(prefix, Nonterminal::SimpleClause)?
            .simple_clause()?
            .clone();
        if clause.subject.is_none() && clause.predicate.verb.slot == VerbSlot::Infinitive {
            clause.predicate.verb.slot = VerbSlot::Imperative;
        }
        clause
            .predicate
            .dependents
            .push(VerbDependent::Prepositional(PrepositionalPhrase {
                preposition: Preposition::With,
                object: Box::new(Phrase::QuotedAbility(Box::new(quoted))),
            }));
        finish_simple_clause(clause)
    }

    /// Attaches a quoted ability as the direct object of a grant verb
    /// (`this creature has/have/gains/gain/loses/lose "..."`). Only a grant
    /// verb licenses a quoted object here, so a quoted string in any other
    /// tail position (`... named "A. B"`) is not this slot and this
    /// production declines, leaving that construction to whatever owns it.
    /// Optional-object grant verbs (`gains`) parse the prefix as a complete
    /// clause directly; the required-object `has`/`have` prefix has no
    /// object of its own, so a sentinel ability complement lets it parse
    /// and is then dropped so the quoted ability takes the freed object
    /// slot.
    fn quoted_grant_object_clause(
        &mut self,
        prefix: &[Token],
        quoted: QuotedAbility,
    ) -> Option<IndependentClause> {
        if !prefix
            .last()
            .is_some_and(|token| is_grant_verb(self.token_text(token)))
        {
            return None;
        }
        let mut clause = if let Some(parsed) = self.parse_exact(prefix, Nonterminal::SimpleClause) {
            parsed.simple_clause()?.clone()
        } else {
            // A required-object grant verb (`this creature has`) will not parse without an
            // object of its own. Supply a sentinel ability complement so the
            // prefix parses, then drop it — the quoted ability takes the freed
            // object slot.
            let probe = format!("{} {GRANT_OBJECT_SENTINEL}", self.tokens_text(prefix));
            let mut clause = parse_nonterminal_with_self_reference(
                &probe,
                self.catalogs,
                Nonterminal::SimpleClause,
                self.self_reference,
            )
            .ok()?
            .simple_clause()?
            .clone();
            clause.predicate.dependents.pop();
            clause
        };
        clause
            .predicate
            .dependents
            .push(VerbDependent::PredicateComplement(Phrase::QuotedAbility(
                Box::new(quoted),
            )));
        finish_simple_clause(clause)
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
            // A lone sentence-terminal (a granted keyword ability's own closing
            // period, e.g. inside `"Cascade, cascade."`) is never a keyword
            // argument. Reject it even inside a comma list, so the ability falls
            // through to the paragraph path and keeps its terminal, rather than
            // rendering a spurious space before the period.
            if !argument_tokens.is_empty()
                && (matches!(argument_tokens, [token] if is_sentence_terminal(token.kind))
                    || (!has_list_separator
                        && !keyword_argument_is_plausible(&atom, argument_tokens, self.source)))
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
        let parsed = parse_nonterminal_with_self_reference(
            span.text(self.source)?,
            self.catalogs,
            nonterminal,
            self.self_reference,
        )
        .ok()?;
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

/// Strips a trailing sentence-terminating period token, returning the body
/// tokens. The period itself is not recorded: the renderer re-derives it from
/// the sentence's structure. `!`/`?` are absorbed by keyword spellings,
/// self-references, or flavor headers upstream and, where they survive into a
/// recovered span, stay verbatim in its text.
fn peel_sentence_ending(tokens: &[Token]) -> &[Token] {
    match tokens.last() {
        Some(last) if last.kind == TokenKind::Punctuation(Punctuation::Period) => {
            &tokens[..tokens.len() - 1]
        }
        _ => tokens,
    }
}

fn token_boundary(tokens: &[Token], byte_end: usize) -> Option<usize> {
    tokens
        .iter()
        .position(|token| token.span.end == byte_end)
        .map(|index| index + 1)
}

/// Whether an imperative predicate's verb is `choose`, the head of a modal
/// choice instruction. Identity is checked against [`Vocab::Choose`], never a
/// surface spelling.
fn predicate_is_choose(predicate: &Predicate) -> bool {
    let head = match predicate {
        Predicate::Transitive(predicate) => &predicate.head,
        Predicate::Intransitive(predicate) => &predicate.head,
        _ => return false,
    };
    matches!(head.verb.verb, Verb::Word(Vocab::Choose))
}

fn tokens_span(tokens: &[Token]) -> Span {
    match (tokens.first(), tokens.last()) {
        (Some(first), Some(last)) => Span::new(first.span.start, last.span.end),
        _ => Span::default(),
    }
}

/// Parses a plain decimal roll-range bound (`9`, `20`) into a structural
/// [`NumberLiteral`]. Returns `None` for anything that is not a canonical
/// unsigned decimal, so a malformed prefix is not read as a row key.
fn arabic_number_literal(text: &str) -> Option<NumberLiteral> {
    Some(NumberLiteral {
        value: Numeral::Arabic(false).parse(text).ok()?,
        numeral: Numeral::Arabic(false),
    })
}

fn starts_with_bullet(tokens: &[Token]) -> bool {
    tokens
        .first()
        .is_some_and(|token| token.kind == TokenKind::Bullet)
}

fn strip_bullet(tokens: &[Token]) -> &[Token] {
    if starts_with_bullet(tokens) { &tokens[1..] } else { tokens }
}

/// The verbs that grant an ability as their direct object, so a quoted ability
/// may fill that object slot after them. Matched on surface form because both
/// inflections of each lemma appear in the corpus (`has`/`have`,
/// `gains`/`gain`, `loses`/`lose`).
fn is_grant_verb(surface: &str) -> bool {
    ["has", "have", "gains", "gain", "loses", "lose"]
        .iter()
        .any(|verb| surface.eq_ignore_ascii_case(verb))
}

/// A base-form ability keyword used only to satisfy a required-object grant
/// verb (`this creature has`) so its prefix parses; it is dropped before the
/// quoted ability takes the object slot, so it never reaches the AST.
const GRANT_OBJECT_SENTINEL: &str = "flying";

/// A flavor header ends in inert terminal junk — `!`, `?`, or an ellipsis
/// (`...`) — never a word or a single sentence-final period. This is what
/// distinguishes a real header (`Exterminate! — …`) from a mid-rules em dash or
/// a mode label, whose run before the dash ends in a word.
fn ends_in_flavor_terminal(header: &[Token]) -> bool {
    match header.last().map(|token| token.kind) {
        Some(TokenKind::Punctuation(Punctuation::Exclamation | Punctuation::Question)) => true,
        Some(TokenKind::Punctuation(Punctuation::Period)) => matches!(
            header
                .len()
                .checked_sub(2)
                .and_then(|index| header.get(index))
                .map(|token| token.kind),
            Some(TokenKind::Punctuation(Punctuation::Period))
        ),
        _ => false,
    }
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
    use crate::parse::parse_with_identity;
    use crate::syntax::*;

    #[test]
    fn activated_ability_has_cost_components_and_effect_sentences() {
        let report = parse("{1}{R}, {T}, Sacrifice Nissa: Draw a card. If you do, discard a card.");
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
        let report = parse("Whenever Nissa attacks, if you control another creature, draw a card.");
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
            "Whenever Nissa attacks, if you control another creature, draw a card."
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
            SentenceBody::Choice(ChoiceInstruction {
                trigger_prefix: None,
                at_random: false,
                imperative: Predicate::Transitive(TransitivePredicate {
                    object: PredicateObject::NounPhrase(NounPhrase::Coordinated(
                        CoordinatedNounPhrase {
                            first,
                            rest,
                        }
                    )),
                    ..
                }),
            }) if matches!(
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

        let triggered = parse("Whenever Nissa attacks, choose one —\n• Draw a card.\n• Scry 1.");
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
    fn comparative_characteristic_postmodifier_gates_the_whole_sentence() {
        // The bound is what lets each sentence parse: the postmodifier gates
        // the surrounding predicate. Causal pair — a ceiling that gates a
        // deontic subject, and its floor mirror gating an imperative object.
        for source in [
            "Target creature with power 2 or less can't be blocked this turn.",
            "Destroy target creature with power 4 or greater.",
        ] {
            let report = parse(source);
            assert!(
                report.diagnostics.is_empty(),
                "{source}: {:?}",
                report.diagnostics
            );
            assert_eq!(render(&report), source);
        }
    }

    #[test]
    fn unrelated_object_or_coordination_is_not_a_comparative_bound() {
        // Negative armor: a plain `X or Y` object coordination must keep
        // parsing structurally without being drawn into the `N or <word>`
        // quantity production.
        let source = "Destroy target artifact or enchantment.";
        let report = parse(source);
        assert!(
            report.diagnostics.is_empty(),
            "{source}: {:?}",
            report.diagnostics
        );
        assert_eq!(render(&report), source);
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
    fn quoted_ability_fills_the_grant_verb_object_for_has_and_have() {
        // Causal pair: the singular `has` and plural `have` inflections of the
        // grant verb both take the quoted ability as their direct object,
        // through the same generalized slot the optional-object `gains` uses —
        // not a `has`/`have`-only special case. (`has`/`have` require an object,
        // so the prefix cannot parse alone; the slot supplies it.)
        for source in [
            "It has \"Sacrifice this token: Add {C}.\"",
            "They have \"Sacrifice this token: Add {C}.\"",
        ] {
            let report = parse(source);
            let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
                panic!("{source}: expected paragraph");
            };
            let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
                &paragraph.sentences[0].body
            else {
                panic!(
                    "{source}: expected transitive grant clause, got {:?}",
                    paragraph.sentences[0].body
                );
            };
            assert!(
                matches!(&predicate.object, PredicateObject::QuotedAbility(_)),
                "{source}: the quoted ability should be the grant verb's object"
            );
            assert_eq!(render(&report), source);
        }
    }

    #[test]
    fn quoted_ability_object_and_with_postmodifier_share_one_slot() {
        // Causal pair across slot kinds: `gains` takes the quoted ability as a
        // grant-verb object, `with` as a postmodifier on a created token. Both
        // are the same generalized quoted-ability slot and both round-trip.
        let object = parse("Target creature gains \"Flying.\"");
        let AbilityKind::Paragraph(object_paragraph) = &object.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert!(matches!(
            &object_paragraph.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Transitive(_, predicate))
                if matches!(predicate.object, PredicateObject::QuotedAbility(_))
        ));
        assert_eq!(render(&object), "Target creature gains \"Flying.\"");

        let with = parse("Create a Goblin creature token with \"{T}: Add {C}.\"");
        let AbilityKind::Paragraph(with_paragraph) = &with.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                &with_paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(_)))
            ),
            "the with-postmodifier clause should be a parsed imperative, got {:?}",
            with_paragraph.sentences[0].body
        );
        assert_eq!(
            render(&with),
            "Create a Goblin creature token with \"{T}: Add {C}.\""
        );
    }

    #[test]
    fn quoted_ability_interior_failure_recovers_at_embedded_rules_only() {
        // A quoted ability whose interior does not parse must still let the
        // outer grant clause parse: the interior failure recovers at the
        // embedded-rules role, never poisons the outer clause with a clause
        // recovery, and the whole span round-trips verbatim.
        let source = "It has \"Glarf the wug quux.\"";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                &paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Transitive(_, predicate))
                    if matches!(predicate.object, PredicateObject::QuotedAbility(_))
            ),
            "outer grant clause must parse despite the interior failure, got {:?}",
            paragraph.sentences[0].body
        );
        let recoveries = report.ast.recoveries();
        assert!(
            recoveries
                .iter()
                .any(|recovery| recovery.role == RecoveryRole::EmbeddedRules),
            "the interior failure should recover at the embedded-rules role: {recoveries:?}"
        );
        assert!(
            recoveries
                .iter()
                .all(|recovery| recovery.role != RecoveryRole::Clause),
            "the interior failure must not surface as an outer clause recovery: {recoveries:?}"
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_quoted_name_is_not_admitted_as_a_grant_object() {
        // Negative armor: a quoted string after `named` is a name, not a
        // granted ability. The grant-object slot fires only after a grant verb,
        // so the existing `named` machinery is untouched — no quoted-ability
        // node appears and the sentence round-trips exactly.
        let source = "Create a token named \"A. B\" and draw a card.";
        let report = parse(source);
        assert!(
            !format!("{:#?}", report.ast).contains("QuotedAbility"),
            "a quoted name must not be lowered to a quoted ability"
        );
        assert_eq!(render(&report), source);
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

        let ability_word = parse("Void — Whenever Nissa attacks, draw a card.");
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
            [NominalModifier::Adjective { .. }, NominalModifier::Noun { noun: crate::word::NounInstance::Singular(crate::word::Noun::Catalog(goblin)), .. }]
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
            "{1}{R}, {T}, Sacrifice Nissa: Draw a card. If you do, discard a card.",
            "Landfall — Whenever a land enters under your control, draw a card.",
            "Whenever Nissa attacks, if you control another creature, draw a card.",
            "Choose one —\n• Draw two cards.\n• Destroy target artifact or enchantment.",
            "{2}, {T}: Choose one —\n• Draw a card.\n• Create a Treasure token.",
            "Whenever Nissa attacks, choose one —\n• Draw a card.\n• Scry 1.",
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
            "Void — Whenever Nissa attacks, draw a card.",
        ] {
            let ast = parse(source).into_ast();
            assert_eq!(
                ast.render(FIXTURE_NAME, true).expect("AST should render"),
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
        let aang = parse_with_identity(aang_source, &catalogs, "Aang, A Lot to Learn", true);
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
                SentenceBody::Independent(IndependentClause::Deontic(
                    _,
                    _,
                    Some(Predicate::Passive(_)),
                ))
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
                    Some(Predicate::Transitive(_)),
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

    #[test]
    fn flavor_header_at_ability_start_is_licensed_opacity() {
        let source = "Zorbo Rampage! — Draw a card.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!(
                "expected paragraph ability, got {:#?}",
                report.ast.abilities[0].kind
            );
        };
        let header = paragraph.flavor_header.as_ref().expect("flavor header");
        assert_eq!(header.text(), "Zorbo Rampage!");
        assert_eq!(header.source_tokens(), 3);
        assert!(
            matches!(&paragraph.sentences[0].body, SentenceBody::Independent(_)),
            "body should parse structurally, got {:#?}",
            paragraph.sentences[0].body
        );
        assert!(
            report.ast.recoveries().is_empty(),
            "flavor header leaves no clause recovery: {:#?}",
            report.ast.recoveries()
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn ellipsis_and_question_flavor_header_round_trips() {
        let source = "Would You Believe...? — Draw a card.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph ability");
        };
        assert_eq!(
            paragraph.flavor_header.as_ref().map(FlavorHeader::text),
            Some("Would You Believe...?")
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn internal_periods_flavor_header_round_trips() {
        let source = "I. AM. LOUD! — Draw a card.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph ability");
        };
        assert_eq!(
            paragraph.flavor_header.as_ref().map(FlavorHeader::text),
            Some("I. AM. LOUD!")
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn flavor_header_stacks_inside_a_single_chapter_body() {
        let source = "I — Stampede! — Draw a card.";
        let report = parse(source);
        let AbilityKind::Chapter(chapter) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a single-chapter ability, got {:#?}",
                report.ast.abilities[0].kind
            );
        };
        assert_eq!(chapter.chapters.len(), 1);
        assert_eq!(
            chapter.body.flavor_header.as_ref().map(FlavorHeader::text),
            Some("Stampede!")
        );
        assert!(matches!(
            &chapter.body.sentences[0].body,
            SentenceBody::Independent(_)
        ));
        // The chapter body's flavor header is licensed opacity, not recovery.
        assert!(report.ast.lexical_opacity().iter().any(|opaque| {
            opaque.kind == LexicalOpacityKind::FlavorHeader && opaque.text == "Stampede!"
        }));
        // The stacked flavor header round-trips inline on the single chapter line.
        assert_eq!(render(&report), source);
    }

    #[test]
    fn flavor_header_inside_a_bulleted_mode_body_round_trips() {
        let source = "I —\n• Stampede! — Draw a card.";
        let report = parse(source);
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!("expected a modal ability");
        };
        assert_eq!(
            modal.modes[0]
                .body
                .flavor_header
                .as_ref()
                .map(FlavorHeader::text),
            Some("Stampede!")
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn ability_word_header_is_not_read_as_a_flavor_header() {
        let report = parse("Landfall — Draw a card.");
        let ability = &report.ast.abilities[0];
        assert_eq!(
            ability
                .ability_word
                .as_ref()
                .map(crate::catalog::CatalogAtom::canonical),
            Some("Landfall")
        );
        let AbilityKind::Paragraph(paragraph) = &ability.kind else {
            panic!("expected paragraph body under the ability word");
        };
        assert_eq!(paragraph.flavor_header, None);
    }

    fn chapter_values(report: &ParseReport) -> Vec<i32> {
        let AbilityKind::Chapter(chapter) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a chapter ability, got {:#?}",
                report.ast.abilities[0].kind
            );
        };
        assert!(
            chapter
                .chapters
                .iter()
                .all(|number| number.numeral == Numeral::Roman),
            "chapter numbers carry the Roman notation structurally"
        );
        chapter.chapters.iter().map(|number| number.value).collect()
    }

    #[test]
    fn single_chapter_saga_header_lowers_to_a_chapter_ability() {
        let report = parse("I — Draw a card.");
        assert_eq!(chapter_values(&report), vec![1]);
        let AbilityKind::Chapter(chapter) = &report.ast.abilities[0].kind else {
            unreachable!();
        };
        assert_eq!(chapter.body.flavor_header, None);
        // Chapter numbers are structural: nothing recovers at the modal header.
        assert!(
            report
                .ast
                .recoveries()
                .iter()
                .all(|recovery| recovery.role != RecoveryRole::ModalHeader),
            "a chapter header carries no modal-header recovery: {:?}",
            report.ast.recoveries()
        );
    }

    #[test]
    fn combined_chapter_header_carries_every_listed_chapter() {
        assert_eq!(chapter_values(&parse("I, II — Draw a card.")), vec![1, 2]);
        assert_eq!(chapter_values(&parse("II, III — Draw a card.")), vec![2, 3]);
        assert_eq!(
            chapter_values(&parse("I, II, III — Draw a card.")),
            vec![1, 2, 3]
        );
        assert_eq!(
            chapter_values(&parse("I, II, III, IV — Draw a card.")),
            vec![1, 2, 3, 4]
        );
        // A combined header carries no modal-header recovery either.
        let report = parse("I, II — Draw a card.");
        assert!(
            report
                .ast
                .recoveries()
                .iter()
                .all(|recovery| recovery.role != RecoveryRole::ModalHeader),
            "a combined chapter header carries no modal-header recovery: {:?}",
            report.ast.recoveries()
        );
    }

    #[test]
    fn a_roman_numeral_without_a_header_em_dash_is_not_a_chapter() {
        // A leading Roman-numeral-like token with no spaced-em-dash split stays an
        // ordinary paragraph; nothing becomes a chapter mid-sentence.
        let report = parse("Exile target creature.");
        assert!(matches!(
            &report.ast.abilities[0].kind,
            AbilityKind::Paragraph(_)
        ));
    }

    #[test]
    fn a_header_group_that_is_not_a_bare_roman_numeral_is_not_a_chapter() {
        // "III" is Roman, but the "and III" group holds a word, so the header is
        // not a chapter list and the line falls through to the paragraph path.
        let report = parse("I, and III — Draw a card.");
        assert!(
            !matches!(&report.ast.abilities[0].kind, AbilityKind::Chapter(_)),
            "a non-Roman header group must not parse as a chapter: {:#?}",
            report.ast.abilities[0].kind
        );
    }

    #[test]
    fn modal_choice_header_is_not_eaten_as_a_flavor_header() {
        let report = parse("Choose one —\n• Draw a card.\n• Draw two cards.");
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!("expected a modal ability");
        };
        assert_eq!(modal.modes.len(), 2);
        assert_eq!(modal.header.flavor_header, None);
        assert!(
            matches!(
                &modal.header.sentences[0].body,
                SentenceBody::Choice(ChoiceInstruction {
                    trigger_prefix: None,
                    at_random: false,
                    ..
                })
            ),
            "the choice instruction parses structurally, not opaque: {:#?}",
            modal.header.sentences[0].body
        );
    }

    fn modal_header_choice(report: &ParseReport) -> &ChoiceInstruction {
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a modal ability: {:#?}",
                report.ast.abilities[0].kind
            );
        };
        let choice = modal
            .header
            .sentences
            .iter()
            .find_map(|sentence| match &sentence.body {
                SentenceBody::Choice(choice) => Some(choice),
                _ => None,
            });
        choice.unwrap_or_else(|| panic!("expected a choice instruction in the header: {modal:#?}"))
    }

    #[test]
    fn choice_instruction_carries_an_at_random_adverbial_structurally() {
        let source = "Choose one at random —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let choice = modal_header_choice(&report);
        assert!(choice.trigger_prefix.is_none());
        assert!(
            choice.at_random,
            "the at-random adverbial is carried: {choice:#?}"
        );
        // The quantity stays the structural imperative object, never a spelling.
        assert!(matches!(
            &choice.imperative,
            Predicate::Transitive(TransitivePredicate {
                object: PredicateObject::NounPhrase(NounPhrase::Quantity(Quantity::Exact(number))),
                ..
            }) if number.value == 1
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn modal_frame_absorbs_a_coordinated_trigger_event() {
        // A coordinated event heading a modal ability is absorbed by the outer
        // `ModalFrame::Triggered`, exactly as a single-clause event is: the
        // frame's trigger parse admits the conjunction through the general
        // clause nonterminal, so the choice header stays a bare `choose one`.
        let source =
            "Whenever Nissa enters or attacks, choose one —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a modal ability: {:#?}",
                report.ast.abilities[0].kind
            );
        };
        let ModalFrame::Triggered {
            introducer, event, ..
        } = &modal.frame
        else {
            panic!("expected a triggered modal frame: {:#?}", modal.frame);
        };
        assert_eq!(*introducer, TriggerWord::Whenever);
        assert!(matches!(
            event,
            TriggerEvent::Clause(IndependentClause::Coordinated(_))
        ));
        let choice = modal_header_choice(&report);
        assert!(choice.trigger_prefix.is_none());
        assert!(!choice.at_random);
        assert_eq!(render(&report), source);
    }

    #[test]
    fn choice_instruction_carries_a_reflexive_second_trigger_prefix() {
        // A `When you do, …` reflexive trigger is a non-initial header sentence,
        // so it cannot be an outer frame: it is the choice's own trigger prefix.
        let source = "Draw a card. When you do, choose one —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!("expected a modal ability");
        };
        assert!(matches!(
            &modal.header.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Imperative(_))
        ));
        let choice = modal_header_choice(&report);
        let prefix = choice
            .trigger_prefix
            .as_ref()
            .expect("a reflexive trigger prefix is carried");
        assert_eq!(prefix.introducer, TriggerWord::When);
        assert_eq!(render(&report), source);
    }

    #[test]
    fn choice_instruction_carries_an_or_both_quantity() {
        let source = "When Nissa enters, choose one or both —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        // The trigger is a simple event, so the outer frame absorbs it and the
        // choice header is the bare `choose one or both`.
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!("expected a modal ability");
        };
        assert!(matches!(modal.frame, ModalFrame::Triggered { .. }));
        let choice = modal_header_choice(&report);
        assert!(choice.trigger_prefix.is_none());
        assert!(matches!(
            &choice.imperative,
            Predicate::Transitive(TransitivePredicate {
                object: PredicateObject::NounPhrase(NounPhrase::Coordinated(_)),
                ..
            })
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn combined_chapter_choice_header_round_trips() {
        // Life of Toshiro Umezawa's shape: a saga chapter heading a choice, plus
        // an at-random adverbial (Summon: Magus Sisters). Both distinctions are
        // structural and render as an exact inverse.
        let source = "I, II — Choose one at random —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!("expected a modal ability");
        };
        let ModalFrame::Chapter(chapters) = &modal.frame else {
            panic!("expected a chapter frame: {:#?}", modal.frame);
        };
        assert_eq!(chapters.len(), 2);
        let choice = modal_header_choice(&report);
        assert!(choice.at_random);
        assert_eq!(render(&report), source);
    }

    #[test]
    fn plain_choose_one_header_still_round_trips_identically() {
        // Regression armor: the motivating plain modal is unchanged on the wire.
        let source = "Choose one —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let choice = modal_header_choice(&report);
        assert!(choice.trigger_prefix.is_none());
        assert!(!choice.at_random);
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_plain_when_trigger_ability_is_not_over_claimed_as_a_choice() {
        // The trigger-prefix grammar must not hijack an ordinary triggered
        // ability whose effect is not a choice.
        let source = "When Nissa enters, draw a card.";
        let report = parse(source);
        assert!(
            matches!(&report.ast.abilities[0].kind, AbilityKind::Triggered(_)),
            "a non-choice When-clause stays a triggered ability: {:#?}",
            report.ast.abilities[0].kind
        );
        assert_eq!(render(&report), source);
    }

    fn arabic(value: i32) -> NumberLiteral {
        NumberLiteral {
            value,
            numeral: Numeral::Arabic(false),
        }
    }

    fn roll_row(report: &ParseReport) -> &RollRowAbility {
        let AbilityKind::RollRow(row) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a roll-row ability, got {:#?}",
                report.ast.abilities[0].kind
            );
        };
        row
    }

    #[test]
    fn single_value_roll_row_lowers_to_a_roll_row_ability() {
        let source = "20 | Search your library for a card.";
        let report = parse(source);
        let row = roll_row(&report);
        assert_eq!(row.range, RollRange::Single(arabic(20)));
        assert_eq!(row.body.flavor_header, None);
        assert!(matches!(
            &row.body.sentences[0].body,
            SentenceBody::Independent(_)
        ));
        // The range key is structural: nothing recovers at the row prefix.
        assert!(
            report.ast.recoveries().is_empty(),
            "a roll-row key carries no recovery: {:?}",
            report.ast.recoveries()
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn unspaced_em_dash_range_row_lowers_to_a_roll_row_ability() {
        // The unspaced dash between the bounds is structure (an inclusive
        // range), never a flavor header. A raw em-dash surface is tolerated and
        // renders back with the canonical en dash.
        let source = "2—9 | Create five tokens.";
        let report = parse(source);
        let row = roll_row(&report);
        assert_eq!(
            row.range,
            RollRange::Inclusive {
                low: arabic(2),
                high: arabic(9),
            }
        );
        assert_eq!(row.body.flavor_header, None);
        assert!(matches!(
            &row.body.sentences[0].body,
            SentenceBody::Independent(_)
        ));
        assert_eq!(render(&report), "2–9 | Create five tokens.");
    }

    #[test]
    fn unspaced_en_dash_range_row_round_trips() {
        // The canonical normalized surface: an unspaced en dash between bounds.
        let source = "2–9 | Create five tokens.";
        let report = parse(source);
        assert_eq!(
            roll_row(&report).range,
            RollRange::Inclusive {
                low: arabic(2),
                high: arabic(9),
            }
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn roll_row_body_flavor_header_stacks_inside_the_row() {
        // The interim behavior peeled `1 | Trapped!` as one flavor header; now
        // `1 |` is the structural key and only `Trapped!` stays a flavor header
        // on the body paragraph, whose sentence parses.
        let source = "1 | Trapped! — You lose 3 life.";
        let report = parse(source);
        let row = roll_row(&report);
        assert_eq!(row.range, RollRange::Single(arabic(1)));
        assert_eq!(
            row.body.flavor_header.as_ref().map(FlavorHeader::text),
            Some("Trapped!")
        );
        assert!(matches!(
            &row.body.sentences[0].body,
            SentenceBody::Independent(_)
        ));
        // The stacked flavor header is licensed opacity, not recovery.
        assert!(report.ast.recoveries().is_empty());
        assert!(report.ast.lexical_opacity().iter().any(|opaque| {
            opaque.kind == LexicalOpacityKind::FlavorHeader && opaque.text == "Trapped!"
        }));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn at_least_and_at_most_threshold_rows_round_trip() {
        // Mirror thresholds the range now admits: `N+` and `N or less`.
        let more = parse("15+ | Draw a card.");
        assert_eq!(roll_row(&more).range, RollRange::OrMore(arabic(15)));
        assert_eq!(render(&more), "15+ | Draw a card.");

        let less = parse("9 or less | Draw a card.");
        assert_eq!(roll_row(&less).range, RollRange::OrLess(arabic(9)));
        assert_eq!(render(&less), "9 or less | Draw a card.");
    }

    #[test]
    fn ascii_hyphen_range_row_normalizes_to_an_en_dash_on_render() {
        // A raw hyphen range lexes as one word token; it lowers to the inclusive
        // shape and renders back with the canonical en dash (the input boundary
        // normalizes the hyphen away, so the parser tolerates it defensively).
        let source = "1-9 | Draw a card.";
        let report = parse(source);
        assert_eq!(
            roll_row(&report).range,
            RollRange::Inclusive {
                low: arabic(1),
                high: arabic(9),
            }
        );
        assert_eq!(render(&report), "1–9 | Draw a card.");
    }

    #[test]
    fn a_multi_row_die_roll_table_parses_into_row_abilities() {
        let source = concat!(
            "{2}, {T}: Roll a d20.\n",
            "1 | Trapped! — You lose 3 life.\n",
            "2–9 | Create five tokens.\n",
            "10–19 | Draw two cards.\n",
            "20 | Draw four cards."
        );
        let report = parse(source);
        assert_eq!(report.ast.abilities.len(), 5);
        assert!(
            matches!(&report.ast.abilities[0].kind, AbilityKind::Activated(_)),
            "the instruction line stays an activated ability: {:#?}",
            report.ast.abilities[0].kind
        );
        let ranges: Vec<RollRange> = report.ast.abilities[1..]
            .iter()
            .map(|ability| {
                let AbilityKind::RollRow(row) = &ability.kind else {
                    panic!("expected a roll-row ability, got {:#?}", ability.kind);
                };
                row.range
            })
            .collect();
        assert_eq!(
            ranges,
            vec![
                RollRange::Single(arabic(1)),
                RollRange::Inclusive {
                    low: arabic(2),
                    high: arabic(9),
                },
                RollRange::Inclusive {
                    low: arabic(10),
                    high: arabic(19),
                },
                RollRange::Single(arabic(20)),
            ]
        );
        // Every row's face key is structure: the whole table has no recovery.
        assert!(
            report.ast.recoveries().is_empty(),
            "die-roll table rows carry no recovery: {:?}",
            report.ast.recoveries()
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn an_unspaced_range_without_a_pipe_is_not_a_roll_row() {
        // No ` | ` separator, so the leading `2—9` is not a row key; the line
        // must not become a roll row (it stays an ordinary paragraph).
        let report = parse("2—9 creatures attack.");
        assert!(
            !matches!(&report.ast.abilities[0].kind, AbilityKind::RollRow(_)),
            "an unspaced range with no pipe must not parse as a roll row: {:#?}",
            report.ast.abilities[0].kind
        );
    }

    #[test]
    fn mid_rules_em_dash_after_a_word_is_not_a_flavor_header() {
        // The em dash follows a word ("choice"), so it is a mid-rules construction
        // (a villainous choice), never a flavor header. Its span stays verbatim.
        let source = "Each opponent faces a villainous choice — You draw a card, or that player discards a card.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph ability");
        };
        assert_eq!(paragraph.flavor_header, None);
        assert_eq!(render(&report), source);
    }

    #[test]
    fn mode_label_before_a_spaced_em_dash_is_not_a_flavor_header() {
        // A word-terminated mode label ("Run and Hide") is not inert junk, so it
        // is not peeled; the mode body round-trips verbatim as before.
        let source = "Run and Hide — Prevent all combat damage this turn.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph ability");
        };
        assert_eq!(paragraph.flavor_header, None);
        assert_eq!(render(&report), source);
    }

    /// A legendary identity (nickname `Nissa`, full name `Nissa Revane`) so the
    /// self-referencing fixtures below recognize their own name; fixtures that
    /// never name the face are unaffected by the identity.
    const FIXTURE_NAME: &str = "Nissa Revane";

    fn parse(source: &str) -> ParseReport {
        parse_with_identity(source, &fixture_catalogs(), FIXTURE_NAME, true)
    }

    fn render(report: &ParseReport) -> String {
        report
            .ast
            .render(FIXTURE_NAME, true)
            .expect("AST should render")
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

    #[test]
    fn activation_cost_imperatives_parse_structurally() {
        for source in [
            "Sacrifice this creature: Draw a card.",
            "Sacrifice this artifact: Draw a card.",
            "Discard a card: Draw a card.",
            "Sacrifice this land: Add {C}.",
            "Sacrifice a creature: Draw a card.",
            "Sacrifice another creature: Scry 1.",
            "Sacrifice this enchantment: Draw a card.",
            "Discard this card: Draw two cards.",
            "Sacrifice an artifact: Draw a card.",
        ] {
            let report = parse(source);
            assert!(
                report.diagnostics.is_empty(),
                "failed on '{source}': {:?}",
                report.diagnostics
            );
        }
    }

    #[test]
    fn coordinated_trigger_events_parse_and_round_trip() {
        for source in [
            "When this creature enters or dies, draw a card.",
            "Whenever this creature enters or attacks, draw a card.",
            "When this creature enters or the creature it haunts dies, draw a card.",
        ] {
            let report = parse(source);
            assert!(
                report.diagnostics.is_empty(),
                "failed on '{source}': {:?}",
                report.diagnostics
            );
            let AbilityKind::Triggered(triggered) = &report.ast.abilities[0].kind else {
                panic!(
                    "expected a triggered ability: {:#?}",
                    report.ast.abilities[0].kind
                );
            };
            assert!(matches!(
                triggered.event,
                TriggerEvent::Clause(IndependentClause::Coordinated(_))
            ));
            assert_eq!(render(&report), source);
        }
    }

    #[test]
    fn shortened_name_coordinated_event_parses_clean() {
        // With real nickname recognition, a coordinated event headed by the
        // face's shortened name parses as an AbbreviatedName self-reference plus
        // a coordinated trigger — no longer residue.
        let source = "Whenever Ashcoat attacks or blocks, draw a card.";
        let report = parse_with_identity(
            source,
            &fixture_catalogs(),
            "Ashcoat of the Shadow Swarm",
            true,
        );
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Triggered(triggered) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a triggered ability: {:#?}",
                report.ast.abilities[0].kind
            );
        };
        assert!(matches!(
            triggered.event,
            TriggerEvent::Clause(IndependentClause::Coordinated(_))
        ));
        assert_eq!(
            report
                .ast
                .render("Ashcoat of the Shadow Swarm", true)
                .unwrap(),
            source
        );
    }

    #[test]
    fn self_reference_takes_verb_selected_agreement() {
        // The self-reference offers both third-person agreements; the verb's own
        // inflection selects one (a joint `and` face reads as plural).
        let plural = parse("When Nissa enter, draw a card.");
        assert!(plural.diagnostics.is_empty(), "{:?}", plural.diagnostics);
        assert_eq!(render(&plural), "When Nissa enter, draw a card.");

        let singular = parse("When Nissa enters, draw a card.");
        assert!(
            singular.diagnostics.is_empty(),
            "{:?}",
            singular.diagnostics
        );
        assert_eq!(render(&singular), "When Nissa enters, draw a card.");
    }

    #[test]
    fn tiered_keyword_header_and_mode_headings_parse() {
        let source = "Tiered\n• Cross-Slash — {0} — Destroy target creature.\n\
             • Blade Beam — {1} — Destroy target creature.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Modal(modal) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a modal ability: {:#?}",
                report.ast.abilities[0].kind
            );
        };
        let ModalFrame::Keyword(atom) = &modal.frame else {
            panic!("expected a keyword modal frame: {:#?}", modal.frame);
        };
        assert_eq!(atom.spelling(), "Tiered");
        assert!(modal.header.sentences.is_empty());
        let [first, second] = modal.modes.as_slice() else {
            panic!("expected two modes: {:#?}", modal.modes);
        };
        let heading = first
            .heading
            .as_ref()
            .expect("first mode carries a heading");
        assert_eq!(heading.label.text(), "Cross-Slash");
        assert!(matches!(
            &heading.cost,
            Cost::Components(components)
                if matches!(components.as_slice(), [Phrase::OracleSymbol(_)])
        ));
        assert!(second.heading.is_some());
        assert_eq!(render(&report), source);
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
                    "Tiered",
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
