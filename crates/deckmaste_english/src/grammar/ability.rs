use super::Nonterminal;
use super::ParsedNonterminal;
use super::clause::finish_simple_clause;
use super::parse_nonterminal_with_self_reference;
use super::parse_symbol_sequence;
use crate::Numeral;
use crate::Span;
use crate::catalog::CatalogAtom;
use crate::catalog::CatalogSlot;
use crate::catalog::CatalogValue;
use crate::catalog::Catalogs;
use crate::catalog::is_named_keyword_argument_label;
use crate::chart::ChartStats;
use crate::construction::ConstructionAlternative;
use crate::construction::ConstructionDecision;
use crate::construction::ConstructionId;
use crate::construction::ProductionId;
use crate::construction::SameFamilyDecision;
use crate::features::Conjunction;
use crate::forest::ForestStats;
use crate::forest::ParseCost;
use crate::forest::ParseCostDimension;
use crate::forest::SelectionReason;
use crate::identity::SelfReference;
use crate::surface::Punctuation;
use crate::surface::Token;
use crate::surface::TokenKind;
use crate::surface::collapse_full_names;
use crate::surface::lex;
use crate::syntax::Ability;
use crate::syntax::AbilityHeader;
use crate::syntax::AbilityKind;
use crate::syntax::ActivatedAbility;
use crate::syntax::ChapterAbility;
use crate::syntax::ChoiceInstruction;
use crate::syntax::ClassLevelAbility;
use crate::syntax::Clause;
use crate::syntax::CopularComplement;
use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::DependentClause;
use crate::syntax::FlavorHeader;
use crate::syntax::IndependentClause;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordAbilityList;
use crate::syntax::KeywordArgument;
use crate::syntax::KeywordArgumentSeparator;
use crate::syntax::KeywordCost;
use crate::syntax::KeywordListSeparator;
use crate::syntax::LevelBandAbility;
use crate::syntax::LevelRange;
use crate::syntax::LoyaltyAbility;
use crate::syntax::LoyaltyCost;
use crate::syntax::LoyaltyCostSign;
use crate::syntax::LoyaltyCostValue;
use crate::syntax::ModalAbility;
use crate::syntax::ModalFrame;
use crate::syntax::ModalHeaderSuffix;
use crate::syntax::Mode;
use crate::syntax::ModeHeading;
use crate::syntax::NonEmpty;
use crate::syntax::NumberLiteral;
use crate::syntax::OracleSymbol;
use crate::syntax::OracleText;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::PowerToughness;
use crate::syntax::Predicate;
use crate::syntax::PredicatedArgument;
use crate::syntax::PredicatedQuality;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhraseKind;
use crate::syntax::QuotedAbility;
use crate::syntax::RecoveredText;
use crate::syntax::RollRange;
use crate::syntax::RollRowAbility;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;
use crate::syntax::Separated;
use crate::syntax::SeparatedNonEmpty;
use crate::syntax::StationThresholdAbility;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::syntax::TriggerCondition;
use crate::syntax::TriggerConditionCoordination;
use crate::syntax::TriggerConditionList;
use crate::syntax::TriggerEvent;
use crate::syntax::TriggerHeader;
use crate::syntax::TriggerWord;
use crate::syntax::TriggeredAbility;
use crate::syntax::TriggeredSentence;
use crate::word::ColorWord;
use crate::word::Noun;
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
    pub(crate) constituent_spans: Vec<Span>,
    pub(crate) rule: Option<usize>,
    pub(crate) construction: Option<crate::construction::ConstructionId>,
    pub(crate) constructions: Vec<crate::construction::ConstructionDecision>,
    pub(crate) tied_alternatives: Vec<usize>,
    pub(crate) cost: ParseCost,
    pub(crate) chart_stats: ChartStats,
    pub(crate) forest_stats: ForestStats,
}

#[derive(Debug, Default)]
pub(crate) struct AbilityParse {
    pub(crate) ast: OracleText,
    pub(crate) ability_spans: Vec<Span>,
    pub(crate) diagnostics: Vec<AbilityDiagnostic>,
    pub(crate) selections: Vec<AbilitySelection>,
}

pub(crate) fn parse_oracle_text(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
) -> AbilityParse {
    Parser::new(source, catalogs, self_reference, false).parse(tokens)
}

/// Parses the interior text of a quoted ability (`"…"`) — `source` is the run
/// between the two double quotes — into a [`QuotedAbility`]. A quoted ability
/// may fill any object/coordination slot the grammar already licenses it in;
/// the chart recognizes the `"…"` span as one lexical unit and defers to this
/// helper at lowering, re-lexing the fragment rather than threading tokens
/// through the forest. The fragment is self-contained — the resulting `Ability`
/// owns its content — and re-collapses the face's full-name self-reference
/// exactly as the top-level parse does, so a quoted ability that names the card
/// resolves identically whether it reaches here or the ability-layer
/// `parse_quoted_sentence`. `closed` is always true: the caller only reaches
/// this with both delimiters present.
pub(crate) fn parse_quoted_ability_fragment(
    source: &str,
    catalogs: &Catalogs,
    self_reference: &SelfReference,
) -> QuotedAbility {
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    let mut parser = Parser::new(source, catalogs, self_reference, true);
    let initial_uppercase = parser.tokens_start_uppercase(&tokens);
    QuotedAbility {
        ability: Box::new(parser.parse_ability(&tokens)),
        initial_uppercase,
    }
}

/// One ability-layer category parsed on its own, with the diagnostics the
/// private [`Parser`] accumulated while parsing it.
///
/// The ability layer is total — it recovers rather than failing — so the value
/// is always present and a failure shows up as a
/// [`AbilityDiagnosticKind::NoCompleteParse`] diagnostic beside a
/// `RecoveredText`-bearing node. [`crate::fragment`] surfaces both.
#[derive(Debug)]
pub(crate) struct AbilityFragment<T> {
    pub(crate) value: T,
    pub(crate) diagnostics: Vec<AbilityDiagnostic>,
    pub(crate) constructions: Vec<crate::construction::ConstructionDecision>,
}

/// Parses a bare activation-cost line (`{2}{W}, Sacrifice this artifact`) with
/// no enclosing ability, wrapping the private `Parser::parse_cost`.
///
/// Shaped exactly like [`parse_quoted_ability_fragment`], the crate's existing
/// precedent for exposing an ability-layer method: build a throwaway `Parser`,
/// call the method, and hand back what it accumulated. `Cost`,
/// `KeywordAbilityList` and `Ability` have no chart nonterminal
/// (`Nonterminal::{Cost, KeywordAbility, KeywordAbilityList, Ability}` carry
/// zero rules), so this layer is the only seam for them.
///
/// `tokens` must already be lexed and full-name-collapsed by the caller, the
/// same preparation [`parse_oracle_text`] receives.
///
/// All three fragment seams build their `Parser` with `quoted_fragment: false`:
/// a fragment is rendered by [`crate::fragment`], never by
/// [`crate::renderer::Renderer::quoted_ability`], so the keyword-line terminal
/// peel that flag gates would strip a period nothing downstream reprints.
pub(crate) fn parse_cost_fragment(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
) -> AbilityFragment<Cost> {
    let mut parser = Parser::new(source, catalogs, self_reference, false);
    let value = parser.parse_cost(tokens);
    AbilityFragment {
        value,
        diagnostics: parser.diagnostics,
        constructions: parser
            .selections
            .into_iter()
            .flat_map(|selection| selection.constructions)
            .collect(),
    }
}

/// Parses a bare keyword line (`Flying`, `Flying, first strike`) with no
/// enclosing ability, wrapping the declaration-backed keyword-line recognizer.
///
/// Unlike the other two ability-layer seams this one is partial: the method
/// returns `None` when the line is not a keyword line at all, and that decline
/// is not a diagnostic — it is the ability layer's ordinary "try the next
/// frame" signal. [`crate::fragment::FragmentReport`] turns a decline into a
/// [`crate::DiagnosticKind::NoCompleteParse`] so an unclean fragment is
/// unclean for one uniform reason.
///
/// See [`parse_cost_fragment`] for the shared wrapper shape and the `tokens`
/// contract.
pub(crate) fn parse_keyword_line_fragment(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
) -> AbilityFragment<Option<KeywordAbilityList>> {
    let mut parser = Parser::new(source, catalogs, self_reference, false);
    let value = parser.parse_keyword_line_construction(tokens);
    AbilityFragment {
        value,
        diagnostics: parser.diagnostics,
        constructions: parser
            .selections
            .into_iter()
            .flat_map(|selection| selection.constructions)
            .collect(),
    }
}

/// Parses one whole ability — the unit a single oracle-text line holds —
/// wrapping the private `Parser::parse_ability`, the same method
/// [`parse_quoted_ability_fragment`] calls for a quoted interior.
///
/// This is the ability-layer entry *below* [`parse_oracle_text`]'s line
/// splitting: it accepts one line's tokens, never a multi-line body, so a
/// modal frame's bulleted modes and a level band's stat rows are out of scope
/// here exactly as they are for a quoted interior.
///
/// See [`parse_cost_fragment`] for the shared wrapper shape and the `tokens`
/// contract.
pub(crate) fn parse_ability_fragment(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
) -> AbilityFragment<Ability> {
    let mut parser = Parser::new(source, catalogs, self_reference, false);
    let value = parser.parse_ability(tokens);
    AbilityFragment {
        value,
        diagnostics: parser.diagnostics,
        constructions: parser
            .selections
            .into_iter()
            .flat_map(|selection| selection.constructions)
            .collect(),
    }
}

pub(crate) fn parse_cost_fragment_with_activation(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
    activation: super::GeneratedActivation,
) -> AbilityFragment<Cost> {
    let mut parser =
        Parser::new_with_activation(source, catalogs, self_reference, false, activation);
    let value = parser.parse_cost(tokens);
    AbilityFragment {
        value,
        diagnostics: parser.diagnostics,
        constructions: parser
            .selections
            .into_iter()
            .flat_map(|selection| selection.constructions)
            .collect(),
    }
}

pub(crate) fn parse_keyword_line_fragment_with_activation(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
    activation: super::GeneratedActivation,
) -> AbilityFragment<Option<KeywordAbilityList>> {
    let mut parser =
        Parser::new_with_activation(source, catalogs, self_reference, false, activation);
    let value = parser.parse_keyword_line_construction(tokens);
    AbilityFragment {
        value,
        diagnostics: parser.diagnostics,
        constructions: parser
            .selections
            .into_iter()
            .flat_map(|selection| selection.constructions)
            .collect(),
    }
}

pub(crate) fn parse_ability_fragment_with_activation(
    source: &str,
    catalogs: &Catalogs,
    tokens: &[Token],
    self_reference: &SelfReference,
    activation: super::GeneratedActivation,
) -> AbilityFragment<Ability> {
    let mut parser =
        Parser::new_with_activation(source, catalogs, self_reference, false, activation);
    let value = parser.parse_ability(tokens);
    AbilityFragment {
        value,
        diagnostics: parser.diagnostics,
        constructions: parser
            .selections
            .into_iter()
            .flat_map(|selection| selection.constructions)
            .collect(),
    }
}

struct Parser<'source, 'catalogs, 'sr> {
    source: &'source str,
    catalogs: &'catalogs Catalogs,
    self_reference: &'sr SelfReference,
    diagnostics: Vec<AbilityDiagnostic>,
    selections: Vec<AbilitySelection>,
    /// Whether `source` is the interior of a quoted ability (`"…"`) rather
    /// than top-level oracle text. Read only by
    /// [`Self::parse_keyword_argument`]'s terminal-peel retry: the renderer
    /// can reprint a keyword line's own stripped closing period only inside
    /// [`crate::renderer::Renderer::quoted_ability`]'s terminal-quote
    /// handling (see the retry's doc comment), so peeling must never fire
    /// outside a quote, where nothing downstream would ever put the period
    /// back.
    quoted_fragment: bool,
    activation: super::GeneratedActivation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbilityFrameCandidate {
    Keyword,
    Loyalty,
    Triggered,
    ClassLevel,
    Activated,
    Chapter,
    RollRow,
}

impl AbilityFrameCandidate {
    const ALL: [Self; 7] = [
        Self::Keyword,
        Self::Loyalty,
        Self::Triggered,
        Self::ClassLevel,
        Self::Activated,
        Self::Chapter,
        Self::RollRow,
    ];

    const fn guard_rank(self) -> u8 {
        match self {
            Self::Keyword => 0,
            Self::Loyalty => 1,
            Self::Triggered => 2,
            Self::ClassLevel => 3,
            Self::Activated => 4,
            Self::Chapter => 5,
            Self::RollRow => 6,
        }
    }
}

struct ParsedAbilityCandidate {
    frame: AbilityFrameCandidate,
    ability: Ability,
    diagnostics: Vec<AbilityDiagnostic>,
    selections: Vec<AbilitySelection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbilityFrameSelectionError {
    EqualGuardRank { rank: u8 },
}

fn select_best_ability_candidate(
    candidates: &[ParsedAbilityCandidate],
) -> Result<Option<usize>, AbilityFrameSelectionError> {
    let Some(best_rank) = candidates
        .iter()
        .map(|candidate| candidate.frame.guard_rank())
        .min()
    else {
        return Ok(None);
    };
    let mut matching = candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| candidate.frame.guard_rank() == best_rank)
        .map(|(index, _)| index);
    let selected = matching
        .next()
        .expect("the minimum rank came from a candidate");
    if matching.next().is_some() {
        Err(AbilityFrameSelectionError::EqualGuardRank { rank: best_rank })
    } else {
        Ok(Some(selected))
    }
}

impl<'source, 'catalogs, 'sr> Parser<'source, 'catalogs, 'sr> {
    fn new(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        self_reference: &'sr SelfReference,
        quoted_fragment: bool,
    ) -> Self {
        Self::new_with_activation(
            source,
            catalogs,
            self_reference,
            quoted_fragment,
            super::GeneratedActivation::Production,
        )
    }

    fn new_with_activation(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        self_reference: &'sr SelfReference,
        quoted_fragment: bool,
        activation: super::GeneratedActivation,
    ) -> Self {
        Self {
            source,
            catalogs,
            self_reference,
            diagnostics: Vec::new(),
            selections: Vec::new(),
            quoted_fragment,
            activation,
        }
    }

    fn parse(mut self, tokens: &[Token]) -> AbilityParse {
        let lines = split_top_level_lines(tokens);
        // [CR#702.184b]: a card printed with the station ability is a station
        // card, and only a station card's `N+ | …` rows are station symbols —
        // a die-roll table prints the same key shape.
        let station_card = lines.iter().any(|line| self.is_station_keyword_line(line));
        let mut abilities = Vec::new();
        let mut ability_spans = Vec::new();
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
                ability_spans.push(tokens_span(current));
                line += 1;
                continue;
            }

            if let Some(range) = self.level_band_header(current).filter(valid_level_range)
                && let Some(stats) = lines
                    .get(line + 1)
                    .and_then(|stat_line| self.bare_power_toughness(stat_line))
            {
                let mut band_end = line + 2;
                while band_end < lines.len()
                    && !lines[band_end].is_empty()
                    && self.level_band_header(lines[band_end]).is_none()
                {
                    band_end += 1;
                }
                let body = lines[line + 2..band_end].to_vec();
                let span = lines_span(&lines[line..band_end]);
                abilities.push(self.parse_level_band(range, stats, &body, span));
                ability_spans.push(span);
                line = band_end;
                continue;
            }

            if station_card && let Some((threshold, body)) = self.station_threshold_frame(current) {
                abilities.push(self.parse_station_threshold(threshold, body, tokens_span(current)));
                ability_spans.push(tokens_span(current));
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
                let span = lines_span(&lines[line..mode_end]);
                abilities.push(self.parse_modal(current, &modes, span));
                ability_spans.push(span);
                line = mode_end;
            } else {
                abilities.push(self.parse_ability(current));
                ability_spans.push(tokens_span(current));
                line += 1;
            }
        }

        AbilityParse {
            ast: OracleText { abilities },
            ability_spans,
            diagnostics: self.diagnostics,
            selections: self.selections,
        }
    }

    fn parse_ability(&mut self, tokens: &[Token]) -> Ability {
        // An ability word wins the one header slot outright; only when none
        // matches does a flavor-word label get a chance to peel.
        let (header, body) = if let Some((word, body)) = self.ability_word_prefix(tokens) {
            (Some(AbilityHeader::AbilityWord(word)), body)
        } else {
            self.flavor_word_prefix(tokens)
                .map_or((None, tokens), |(header, rest)| {
                    (Some(AbilityHeader::Flavor(header)), rest)
                })
        };
        let (ability, decision) = self.parse_ability_kind(body, header);
        self.finish_built_ability_with_decision(tokens_span(tokens), ability, decision)
    }

    fn parse_ability_kind(
        &mut self,
        tokens: &[Token],
        header: Option<AbilityHeader>,
    ) -> (Ability, Option<SameFamilyDecision>) {
        #[cfg_attr(
            not(test),
            allow(
                unused_mut,
                reason = "test activations permute the candidate array in place"
            )
        )]
        let mut frames = AbilityFrameCandidate::ALL;
        #[cfg(test)]
        self.activation.reorder_candidates(&mut frames);
        let mut candidates = frames
            .into_iter()
            .filter_map(|frame| self.probe_ability_candidate(frame, tokens, header.clone()))
            .collect::<Vec<_>>();
        let selected = select_best_ability_candidate(&candidates)
            .expect("ability classifier guard ranks must be unique");
        if let Some(selected) = selected {
            let decision = (candidates.len() > 1).then(|| {
                let selected_frame = candidates[selected].frame;
                let selected_ordinal = crate::constructions::ability::selected_ability_form(
                    &candidates[selected].ability,
                )
                .expect("every checked ability selects exactly one declared form")
                .ordinal;
                let mut alternatives = candidates
                    .iter()
                    .map(|candidate| {
                        (
                            crate::constructions::ability::selected_ability_form(
                                &candidate.ability,
                            )
                            .expect("every checked ability selects exactly one declared form")
                            .ordinal,
                            ParseCost {
                                precedence: u32::from(candidate.frame.guard_rank()),
                                ..ParseCost::default()
                            },
                        )
                    })
                    .collect::<Vec<_>>();
                alternatives.sort_by_key(|(ordinal, _)| *ordinal);
                SameFamilyDecision::ranked(
                    selected_ordinal,
                    ParseCost {
                        precedence: u32::from(selected_frame.guard_rank()),
                        ..ParseCost::default()
                    },
                    SelectionReason::Cost(ParseCostDimension::Precedence),
                    alternatives,
                )
            });
            let mut candidate = candidates.swap_remove(selected);
            self.diagnostics.append(&mut candidate.diagnostics);
            self.selections.append(&mut candidate.selections);
            return (candidate.ability, decision);
        }
        if let Some(colon) = find_top_level_punctuation(tokens, Punctuation::Colon)
            && colon + 1 == tokens.len()
        {
            self.diagnostics.push(AbilityDiagnostic {
                kind: AbilityDiagnosticKind::EmptyActivationEffect,
                span: tokens
                    .get(colon)
                    .map_or_else(|| tokens_span(tokens), |token| token.span),
            });
        }
        let ability = crate::constructions::ability::build_ability_root(
            header,
            AbilityKind::Paragraph(self.parse_paragraph(tokens)),
        )
        .expect("the paragraph fallback satisfies checked ability ingress");
        (ability, None)
    }

    fn probe_ability_candidate(
        &mut self,
        frame: AbilityFrameCandidate,
        tokens: &[Token],
        header: Option<AbilityHeader>,
    ) -> Option<ParsedAbilityCandidate> {
        let diagnostics = self.diagnostics.len();
        let selections = self.selections.len();
        let kind = match frame {
            AbilityFrameCandidate::Keyword => self
                .parse_keyword_line_construction(tokens)
                .map(AbilityKind::Keyword),
            AbilityFrameCandidate::Loyalty => {
                self.loyalty_frame(tokens).and_then(|(cost, effect)| {
                    (!effect.is_empty()).then(|| {
                        AbilityKind::Loyalty(LoyaltyAbility {
                            cost,
                            effect: self.parse_paragraph(effect),
                        })
                    })
                })
            }
            AbilityFrameCandidate::Triggered => self.triggered_ability_frame(tokens).and_then(
                |(conditions, intervening_condition, effect)| {
                    (!effect.is_empty()).then(|| {
                        AbilityKind::Triggered(TriggeredAbility {
                            conditions,
                            intervening_condition,
                            effect: self.parse_paragraph(effect),
                        })
                    })
                },
            ),
            AbilityFrameCandidate::ClassLevel => {
                let colon = find_top_level_punctuation(tokens, Punctuation::Colon)?;
                let level = self.class_level(&tokens[colon + 1..])?;
                Some(AbilityKind::ClassLevel(ClassLevelAbility {
                    cost: self.parse_cost(&tokens[..colon]),
                    level,
                }))
            }
            AbilityFrameCandidate::Activated => {
                let colon = find_top_level_punctuation(tokens, Punctuation::Colon)?;
                let effect = &tokens[colon + 1..];
                if effect.is_empty() || self.class_level(effect).is_some() {
                    None
                } else {
                    Some(AbilityKind::Activated(ActivatedAbility {
                        cost: self.parse_cost(&tokens[..colon]),
                        effect: self.parse_paragraph(effect),
                    }))
                }
            }
            AbilityFrameCandidate::Chapter => {
                self.chapter_frame(tokens).and_then(|(chapters, body)| {
                    (!body.is_empty()).then(|| {
                        AbilityKind::Chapter(ChapterAbility {
                            chapters,
                            body: self.parse_paragraph(body),
                        })
                    })
                })
            }
            AbilityFrameCandidate::RollRow => {
                self.roll_row_frame(tokens).and_then(|(range, body)| {
                    (!body.is_empty()).then(|| {
                        AbilityKind::RollRow(RollRowAbility {
                            range,
                            body: self.parse_paragraph(body),
                        })
                    })
                })
            }
        };
        let candidate_diagnostics = self.diagnostics.split_off(diagnostics);
        let candidate_selections = self.selections.split_off(selections);
        kind.and_then(|kind| crate::constructions::ability::build_ability_root(header, kind).ok())
            .map(|ability| ParsedAbilityCandidate {
                frame,
                ability,
                diagnostics: candidate_diagnostics,
                selections: candidate_selections,
            })
    }

    fn finish_ability(
        &mut self,
        span: Span,
        header: Option<AbilityHeader>,
        kind: AbilityKind,
    ) -> Ability {
        let ability = crate::constructions::ability::build_ability_root(header, kind)
            .expect("the ability classifier satisfies the declaration");
        self.finish_built_ability_with_decision(span, ability, None)
    }

    fn finish_built_ability_with_decision(
        &mut self,
        span: Span,
        ability: Ability,
        decision: Option<SameFamilyDecision>,
    ) -> Ability {
        let ordinal = crate::constructions::ability::selected_ability_form(&ability)
            .expect("the declaration assigns every AbilityKind one form")
            .ordinal;
        self.record_ability_construction_span_with_decision(
            span,
            "ability",
            decision.unwrap_or_else(|| SameFamilyDecision::unique(ordinal)),
        );
        ability
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

    /// Recognizes a leveler band header line — `LEVEL N1-N2` or `LEVEL N3+`
    /// [CR#711.2a,711.2b]. The spelling is matched case-sensitively: the frame
    /// prints `LEVEL` in caps on every attested face, so requiring the exact
    /// spelling lets the renderer be an exact inverse without carrying a casing
    /// field, and keeps prose `Level` (a Class level bar, a rules sentence)
    /// from ever reaching the band frame.
    fn level_band_header(&self, tokens: &[Token]) -> Option<LevelRange> {
        let [head, rest @ ..] = tokens else {
            return None;
        };
        if head.kind != TokenKind::Word || self.token_text(head) != "LEVEL" {
            return None;
        }
        match rest {
            [value, plus]
                if value.kind == TokenKind::Integer
                    && plus.kind == TokenKind::Punctuation(Punctuation::Plus) =>
            {
                Some(LevelRange::AtLeast(self.arabic_literal(value)?))
            }
            [span] if span.kind == TokenKind::Word => {
                let (low, high) = self.token_text(span).split_once('-')?;
                Some(LevelRange::Band {
                    low: arabic_number_literal(low)?,
                    high: arabic_number_literal(high)?,
                })
            }
            _ => None,
        }
    }

    /// A band's bare power/toughness stat line: exactly one
    /// [`TokenKind::PowerToughness`] token and nothing else. The absence of a
    /// terminal period is what separates this from
    /// [`Self::parse_power_toughness_body`]'s tiered-mode `3/2.` sentence, and
    /// the single-element slice pattern enforces it.
    fn bare_power_toughness(&self, tokens: &[Token]) -> Option<PowerToughness> {
        let [stat] = tokens else { return None };
        if stat.kind != TokenKind::PowerToughness {
            return None;
        }
        super::parse_power_toughness(self.token_text(stat))
    }

    /// Assembles one level band. Constructs the `Ability` directly — like
    /// [`Self::parse_modal`] and unlike [`Self::parse_ability`] — because a
    /// band header line carries no ability-word or flavor-word prefix.
    fn parse_level_band(
        &mut self,
        range: LevelRange,
        stats: PowerToughness,
        body: &[&[Token]],
        span: Span,
    ) -> Ability {
        let abilities = body.iter().map(|line| self.parse_ability(line)).collect();
        self.finish_ability(
            span,
            None,
            AbilityKind::LevelBand(LevelBandAbility {
                range,
                stats,
                abilities,
            }),
        )
    }

    /// Whether this line is the bare `Station` keyword ability [CR#702.184a] —
    /// the marker that makes the face a station card [CR#702.184b] and
    /// licenses its `N+ | …` rows as station symbols rather than die-roll
    /// keys. The whole line must be the single catalog-matched keyword:
    /// reminder text is stripped upstream, so `Station (Tap another creature
    /// you control: …)` arrives here as one token, while a sentence merely
    /// naming a card called `… Station` never matches.
    fn is_station_keyword_line(&self, tokens: &[Token]) -> bool {
        let [keyword] = tokens else { return false };
        if keyword.kind != TokenKind::Word {
            return false;
        }
        let Some(text) = self.source.get(keyword.span.start..keyword.span.end) else {
            return false;
        };
        self.catalogs
            .matches(text, CatalogSlot::AbilityItem)
            .into_iter()
            .any(|catalog_match| {
                catalog_match.length == text.len()
                    && matches!(
                        catalog_match.value,
                        CatalogValue::Atom(ref atom) if atom.canonical() == "Station"
                    )
            })
    }

    /// Splits a station threshold row (`8+ | Flying, trample`) into its
    /// charge-counter threshold and the body after the spaced ` | `. The key
    /// shape is closed to a single number and a plus sign [CR#721.2] —
    /// narrower than [`Self::roll_range`], which also admits single values,
    /// inclusive spans, and `or less`. The separator must be exactly ` | ` so
    /// the renderer reproduces it verbatim, the same requirement
    /// [`Self::roll_row_frame`] enforces.
    fn station_threshold_frame<'a>(
        &self,
        tokens: &'a [Token],
    ) -> Option<(NumberLiteral, &'a [Token])> {
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
        let [value, plus] = &tokens[..pipe] else {
            return None;
        };
        if value.kind != TokenKind::Integer
            || plus.kind != TokenKind::Punctuation(Punctuation::Plus)
        {
            return None;
        }
        let threshold = self.arabic_literal(value)?;
        Some((threshold, &tokens[pipe + 1..]))
    }

    /// Assembles one station threshold. The body is parsed as a whole
    /// [`Ability`] — that is what lets a striation carry a keyword list, an
    /// activated ability, or a triggered ability with no new machinery — and
    /// the `Ability` is constructed directly (like [`Self::parse_level_band`]
    /// and [`Self::parse_modal`]) because the row key carries no
    /// ability-word or flavor-word prefix of its own.
    fn parse_station_threshold(
        &mut self,
        threshold: NumberLiteral,
        body: &[Token],
        span: Span,
    ) -> Ability {
        let nested = self.parse_ability(body);
        self.finish_ability(
            span,
            None,
            AbilityKind::StationThreshold(StationThresholdAbility {
                threshold,
                ability: Box::new(nested),
            }),
        )
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

    fn parse_modal(&mut self, header: &[Token], modes: &[&[Token]], span: Span) -> Ability {
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
            self.attempt(|parser| parser.trigger_frame(header))
        {
            (
                ModalFrame::Triggered(TriggerHeader {
                    introducer,
                    event,
                    intervening_condition,
                }),
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

        let modal = ModalAbility {
            frame,
            header: self.parse_choice_header(header),
            header_suffix,
            modes: modes.iter().map(|mode| self.parse_mode(mode)).collect(),
        };
        self.finish_ability(
            span,
            ability_word.map(AbilityHeader::AbilityWord),
            // A modal choice header is never peeled as a flavor word: the six
            // known flavor-word faces and the measured residue population are
            // all single-frame abilities, and a modal header's leading ` — `
            // belongs to the `Choose …` instruction, not a label.
            AbilityKind::Modal(modal),
        )
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
        if !cost
            .components()
            .iter()
            .all(|component| matches!(component, CostComponent::Symbols(_)))
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

    /// Peels a licensed flavor-word header from the front of an ability: a
    /// Scryfall flavor word ([`CatalogKind::FlavorWord`]) named ahead of the
    /// ability's frame and set off by a spaced em dash (`Chaos — Whenever …`,
    /// `Sanctified Rules of Combat — When …`). The label reproduces verbatim as
    /// licensed lexical opacity (a [`FlavorHeader`]); the frame after ` — `
    /// parses by the ordinary machinery, so a trigger or activated cost the
    /// un-peeled label had blocked recovers unchanged.
    ///
    /// Three gates keep the peel off structural em dashes and off headers other
    /// paths already own, mirroring the flavor-header peel's discipline:
    /// - the em dash must be the paragraph-initial *spaced* ` — `, found by
    ///   [`Self::spaced_top_level_em_dash`] (never an unspaced roll range, a
    ///   mid-rules villainous-choice dash, or a nested quoted dash);
    /// - the whole label before it must be a byte-exact flavor-word catalog
    ///   member, so ordinary sentence-initial vocabulary and a capitalized
    ///   non-catalog label are left recovered; and
    /// - the label must not end in inert flavor terminal punctuation (`!`, `?`,
    ///   ellipsis) — those keep the existing paragraph-level
    ///   [`peel_flavor_header`] path so nothing that already peels is rerouted
    ///   here.
    ///
    /// [`CatalogKind::FlavorWord`]: crate::CatalogKind::FlavorWord
    /// [`peel_flavor_header`]: Self::peel_flavor_header
    fn flavor_word_prefix<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> Option<(FlavorHeader, &'tokens [Token])> {
        let dash = self.spaced_top_level_em_dash(tokens)?;
        let label = &tokens[..dash];
        if ends_in_flavor_terminal(label) {
            return None;
        }
        let start = tokens.first()?.span.start;
        let end = tokens[dash - 1].span.end;
        let text = self.source.get(start..end)?;
        if !self.catalogs.is_flavor_word(text) {
            return None;
        }
        Some((FlavorHeader::new(text, dash), &tokens[dash + 1..]))
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
        let comma = find_top_level_punctuation(tokens, Punctuation::Comma)?;
        let condition = self.parse_trigger_condition(tokens.get(..comma)?)?;
        let (intervening_condition, effect) = self.trigger_tail(tokens.get(comma + 1..)?)?;
        Some((
            condition.introducer,
            condition.event,
            intervening_condition,
            effect,
        ))
    }

    /// Parses the ability-initial trigger frame. Mixed-introducer conditions
    /// are tried as one coordinated list before the established single-frame
    /// path; a declined coordinated probe rolls back its chart selections so
    /// ordinary triggers retain exactly their prior provenance.
    fn triggered_ability_frame<'tokens>(
        &mut self,
        tokens: &'tokens [Token],
    ) -> Option<(
        TriggerConditionList,
        Option<DependentClause>,
        &'tokens [Token],
    )> {
        let selection_checkpoint = self.selections.len();
        if let Some(frame) = self.coordinated_trigger_frame(tokens) {
            return Some(frame);
        }
        self.selections.truncate(selection_checkpoint);

        let (introducer, event, intervening_condition, effect) = self.trigger_frame(tokens)?;
        Some((
            TriggerConditionList {
                first: TriggerCondition { introducer, event },
                rest: Vec::new(),
            },
            intervening_condition,
            effect,
        ))
    }

    fn coordinated_trigger_frame<'tokens>(
        &mut self,
        tokens: &'tokens [Token],
    ) -> Option<(
        TriggerConditionList,
        Option<DependentClause>,
        &'tokens [Token],
    )> {
        let comma = find_top_level_punctuation(tokens, Punctuation::Comma)?;
        let conditions = self.parse_trigger_condition_list(tokens.get(..comma)?)?;
        let (intervening_condition, effect) = self.trigger_tail(tokens.get(comma + 1..)?)?;
        Some((conditions, intervening_condition, effect))
    }

    fn parse_trigger_condition_list(&mut self, tokens: &[Token]) -> Option<TriggerConditionList> {
        let mut connectors = Vec::new();
        let mut depth = Nesting::default();
        for (index, token) in tokens.iter().enumerate() {
            if depth.is_top_level() {
                let conjunction = match self.token_text(token) {
                    text if text.eq_ignore_ascii_case("and") => Some(Conjunction::And),
                    text if text.eq_ignore_ascii_case("or") => Some(Conjunction::Or),
                    _ => None,
                };
                let next_introducer = tokens
                    .get(index + 1)
                    .and_then(|next| self.trigger_word(next));
                let scalar_at = next_introducer == Some(TriggerWord::At)
                    && tokens.get(index + 2).is_some_and(|following| {
                        let text = self.token_text(following);
                        text.eq_ignore_ascii_case("least") || text.eq_ignore_ascii_case("most")
                    });
                if let Some(conjunction) = conjunction
                    && next_introducer.is_some()
                    && !scalar_at
                {
                    connectors.push((index, conjunction));
                }
            }
            depth.observe(token.kind);
        }
        let (first_connector, _) = *connectors.first()?;
        let first = self.parse_trigger_condition(tokens.get(..first_connector)?)?;
        let mut rest = Vec::with_capacity(connectors.len());
        for (connector_index, (connector, conjunction)) in connectors.iter().copied().enumerate() {
            let end = connectors
                .get(connector_index + 1)
                .map_or(tokens.len(), |(next, _)| *next);
            let condition = self.parse_trigger_condition(tokens.get(connector + 1..end)?)?;
            rest.push(TriggerConditionCoordination {
                conjunction,
                condition,
            });
        }
        Some(TriggerConditionList { first, rest })
    }

    fn parse_trigger_condition(&mut self, tokens: &[Token]) -> Option<TriggerCondition> {
        let introducer = self.trigger_word(tokens.first()?)?;
        let event_tokens = tokens.get(1..)?;
        let event = if introducer == TriggerWord::At {
            let event = self.accept_exact(event_tokens, Nonterminal::NounPhrase, |parsed| {
                parsed.noun_phrase().cloned()
            })?;
            TriggerEvent::Temporal(event)
        } else if let Some(clause) =
            self.accept_exact(event_tokens, Nonterminal::SimpleClause, |parsed| {
                parsed
                    .simple_clause()
                    .cloned()
                    .and_then(finish_simple_clause)
            })
        {
            TriggerEvent::Clause(clause)
        } else {
            TriggerEvent::Clause(self.clause_event(event_tokens)?)
        };
        Some(TriggerCondition { introducer, event })
    }

    fn trigger_word(&self, token: &Token) -> Option<TriggerWord> {
        TriggerWord::from_spelling(self.token_text(token))
    }

    fn trigger_tail<'tokens>(
        &mut self,
        mut effect: &'tokens [Token],
    ) -> Option<(Option<DependentClause>, &'tokens [Token])> {
        let intervening_condition = if effect
            .first()
            .is_some_and(|token| self.token_text(token).eq_ignore_ascii_case("if"))
        {
            let condition_comma = find_top_level_punctuation(effect, Punctuation::Comma)?;
            let condition = self.accept_exact(
                effect.get(1..condition_comma)?,
                Nonterminal::Clause,
                |parsed| match parsed.clause()? {
                    Clause::Independent(condition) => Some(condition.clone()),
                    Clause::Dependent(_) => None,
                },
            )?;
            effect = effect.get(condition_comma + 1..)?;
            Some(DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Finite(Box::new(condition)),
            ))
        } else {
            None
        };
        Some((intervening_condition, effect))
    }

    /// Parses a trigger event that the simple-clause frame could not: a
    /// coordinated event (`this creature enters or dies`), an existential
    /// (`there are no creatures on the battlefield`), a copular event, or one
    /// carrying a subordinate rider (`… enters while this creature has a -1/-1
    /// counter on it`). Any independent clause is admitted, because a trigger
    /// event is exactly "an independent clause" — the previous restriction to
    /// [`IndependentClause::Coordinated`] discarded 17 of the 18 `Clause`
    /// productions. That restriction's original rationale — keeping a
    /// single-clause event on its more specific simple-clause parse — is still
    /// honored here: this arm is reached only after the simple-clause attempt
    /// above has already declined, so ordering (not the variant filter) is
    /// what protects the simple-clause frame's priority.
    ///
    /// The opaque-copular-complement guard below is load-bearing, not
    /// decorative: a copular reading can swallow a missing keyword-action
    /// verb as a bare opaque noun complement of `is` (`a Faerie is
    /// championed with this creature` — `champion` is absent from the
    /// vocabulary, §1.4) and, once the event fallback stopped requiring
    /// `Coordinated`, that copular reading became reachable here for the
    /// first time. It is a wrong tree that round-trips: the rendered text
    /// matches, but the ability is modeled as an opaque-copula event rather
    /// than staying unparsed pending the keyword-action lexeme round.
    /// Rejecting only that shape (an opaque noun heading the copular
    /// complement) keeps every E3 row on its existing residue while leaving
    /// unrelated opacity — an opaque proper name in the event's subject, as
    /// in Merieke Ri Berit's coordinated event — untouched, since that
    /// opacity is the ordinary, already-licensed self-reference-name path
    /// and not a missing-lexeme camouflage.
    fn clause_event(&mut self, tokens: &[Token]) -> Option<IndependentClause> {
        self.accept_exact(tokens, Nonterminal::Clause, |parsed| {
            match parsed.clause()? {
                Clause::Independent(clause) => {
                    if copular_complement_head_is_opaque(clause) {
                        return None;
                    }
                    Some(clause.clone())
                }
                Clause::Dependent(_) => None,
            }
        })
    }

    fn parse_cost(&mut self, tokens: &[Token]) -> Cost {
        let all_tokens = tokens;
        let (flavor_header, tokens) = self.peel_cost_flavor_header(tokens);
        let mut components = split_top_level(tokens, &[Punctuation::Comma])
            .into_iter()
            .filter(|component| !component.is_empty())
            .map(|component| self.parse_cost_component(component))
            .collect::<Vec<_>>();
        if components.is_empty() {
            components.push(CostComponent::Recovered(self.recovered_text(tokens)));
        }
        let components = NonEmpty::try_from(components)
            .expect("cost recovery makes the component sequence nonempty");
        let cost = crate::constructions::ability::build_cost(flavor_header, components)
            .expect("the cost recognizer satisfies the declaration");
        let form = crate::constructions::ability::selected_cost_form(&cost)
            .expect("the declaration assigns every cost one form")
            .ordinal;
        self.record_ability_construction(all_tokens, "cost", form);
        cost
    }

    fn record_ability_construction(&mut self, tokens: &[Token], id: &'static str, ordinal: u16) {
        self.record_ability_construction_span(
            tokens_span(tokens),
            id,
            SameFamilyDecision::unique(ordinal),
        );
    }

    fn record_ability_construction_span(
        &mut self,
        span: Span,
        id: &'static str,
        decision: SameFamilyDecision,
    ) {
        self.record_ability_construction_span_with_decision(span, id, decision);
    }

    fn record_ability_construction_span_with_decision(
        &mut self,
        span: Span,
        id: &'static str,
        decision: SameFamilyDecision,
    ) {
        let groups = self.activation.backend_groups(
            deckmaste_construction_compiler::runtime::ConstructionBackendData::Ability,
        );
        #[cfg(test)]
        let construction = {
            let mut constructions = groups
                .iter()
                .flat_map(|group| group.constructions)
                .collect::<Vec<_>>();
            self.activation.reorder_candidates(&mut constructions);
            constructions
                .into_iter()
                .find(|construction| construction.id == id)
        };
        #[cfg(not(test))]
        let construction = groups
            .iter()
            .flat_map(|group| group.constructions)
            .find(|construction| construction.id == id);
        let Some(_construction) = construction else {
            return;
        };
        let id = ConstructionId::new(id);
        let family = if self.activation.is_production() {
            super::construction::family_by_id(id)
        } else {
            super::construction::registry_from_groups(self.activation.groups())
                .ok()
                .and_then(|registry| registry.family(id))
        };
        let Some(family) = family else {
            return;
        };
        let cost = decision.cost();
        let construction_decision = decision.finish(span, id, family);
        self.selections.push(AbilitySelection {
            span,
            constituent_spans: Vec::new(),
            rule: None,
            construction: Some(id),
            constructions: vec![construction_decision],
            tied_alternatives: vec![0],
            cost,
            chart_stats: ChartStats::default(),
            forest_stats: ForestStats::default(),
        });
    }

    fn peel_cost_flavor_header<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> (Option<FlavorHeader>, &'tokens [Token]) {
        let Some(dash) = self.spaced_top_level_em_dash(tokens) else {
            return (None, tokens);
        };
        let before_end = tokens[dash - 1].span.end;
        let header_start = tokens[0].span.start;
        let Some(text) = self.source.get(header_start..before_end) else {
            return (None, tokens);
        };
        (Some(FlavorHeader::new(text, dash)), &tokens[dash + 1..])
    }

    fn parse_cost_component(&mut self, tokens: &[Token]) -> CostComponent {
        if tokens.len() == 1 {
            match tokens[0].kind {
                TokenKind::OracleSymbol => {
                    if let Some(symbol) = OracleSymbol::new(self.token_text(&tokens[0])) {
                        return CostComponent::Symbols(vec![symbol]);
                    }
                }
                TokenKind::SymbolSequence => {
                    if let Some(symbols) = parse_symbol_sequence(self.token_text(&tokens[0])) {
                        return CostComponent::Symbols(symbols);
                    }
                }
                _ => {}
            }
        }
        // A cost expressed as a full clause is always independent (imperative or
        // coordinated); the dependent-clause branch never fires on the supported
        // corpus, so narrowing to `IndependentClause` loses nothing and rejects
        // the shape rather than mistyping it.
        if let Some(independent) = self.accept_exact(tokens, Nonterminal::Clause, |parsed| {
            let Clause::Independent(independent) = parsed.clause()? else {
                return None;
            };
            Some(independent.clone())
        }) {
            return CostComponent::Clause(Box::new(independent));
        }
        if let Some(noun_phrase) = self.accept_exact(tokens, Nonterminal::NounPhrase, |parsed| {
            parsed.noun_phrase().cloned()
        }) {
            return CostComponent::Noun(Box::new(noun_phrase));
        }
        if let Some(or_index) = self.find_top_level_or(tokens) {
            let left_tokens = &tokens[..or_index];
            let right_tokens = &tokens[or_index + 1..];
            if !left_tokens.is_empty() && !right_tokens.is_empty() {
                let left = self.parse_cost_component(left_tokens);
                let right = self.parse_cost_component(right_tokens);
                if !matches!(left, CostComponent::Recovered(_))
                    && !matches!(right, CostComponent::Recovered(_))
                {
                    return CostComponent::Alternative(Box::new(left), Box::new(right));
                }
            }
        }
        CostComponent::Recovered(self.recovered_text(tokens))
    }

    fn find_top_level_or(&self, tokens: &[Token]) -> Option<usize> {
        let mut depth = Nesting::default();
        for (index, token) in tokens.iter().enumerate() {
            if depth.is_top_level()
                && token.kind == TokenKind::Word
                && self.token_text(token).eq_ignore_ascii_case("or")
            {
                return Some(index);
            }
            depth.observe(token.kind);
        }
        None
    }

    fn parse_paragraph(&mut self, tokens: &[Token]) -> Paragraph {
        let (flavor_header, body) = self.peel_flavor_header(tokens);
        let mut sentences = split_sentences(self.source, body, self.self_reference.nickname())
            .into_iter()
            .filter(|sentence| !sentence.is_empty())
            .map(|sentence| self.parse_sentence(sentence))
            .collect::<Vec<_>>();
        if sentences.is_empty() {
            sentences.push(self.parse_sentence(body));
        }
        Paragraph {
            flavor_header,
            sentences,
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
    /// - the run before it must be a licensed label: either it ends in inert
    ///   flavor terminal punctuation (`!`, `?`, or an ellipsis) or it is a
    ///   byte-exact flavor-word catalog member (`Aerial Blast — …` at a saga
    ///   chapter body's start). A bare non-catalog word, a lone period, or a
    ///   clause is not a header and stays recovered. The catalog arm mirrors
    ///   the ability-level [`Self::flavor_word_prefix`], which peels the same
    ///   labels one level up when they stand ahead of a trigger or cost frame.
    fn peel_flavor_header<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> (Option<FlavorHeader>, &'tokens [Token]) {
        let Some(dash) = self.spaced_top_level_em_dash(tokens) else {
            return (None, tokens);
        };
        let before_end = tokens[dash - 1].span.end;
        let header_start = tokens[0].span.start;
        let Some(text) = self.source.get(header_start..before_end) else {
            return (None, tokens);
        };
        if !ends_in_flavor_terminal(&tokens[..dash]) && !self.catalogs.is_flavor_word(text) {
            return (None, tokens);
        }
        (Some(FlavorHeader::new(text, dash)), &tokens[dash + 1..])
    }

    fn parse_sentence(&mut self, tokens: &[Token]) -> Sentence {
        if let Some(sentence) = self.attempt(|parser| {
            let sentence = parser.parse_sentence_candidate(tokens)?;
            parser
                .sentence_form_is_admitted(tokens, &sentence)
                .then_some(sentence)
        }) {
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
        Sentence::from_body(SentenceBody::Recovered(RecoveredText::new(
            self.tokens_text(tokens),
            tokens.len(),
        )))
    }

    /// Selects one exact semantic Sentence candidate. The caller owns the one
    /// contextual form-admission boundary, so every present and future branch
    /// reaches it before returning and the outer [`Self::attempt`] can roll
    /// back all candidate provenance when admission declines.
    fn parse_sentence_candidate(&mut self, tokens: &[Token]) -> Option<Sentence> {
        if let Some(sentence) = self.attempt(|parser| parser.parse_quoted_sentence(tokens)) {
            return Some(sentence);
        }
        if let Some(sentence) = self.attempt(|parser| {
            parser.accept_exact(tokens, Nonterminal::Sentence, |parsed| {
                parsed.sentence().cloned()
            })
        }) {
            return Some(sentence);
        }
        if let Some(sentence) = self.attempt(|parser| parser.parse_dash_appositive(tokens)) {
            return Some(sentence);
        }
        if let Some(body) = self.parse_power_toughness_body(tokens) {
            return Some(Sentence::from_body(body));
        }
        if let Some(body) = self.attempt(|parser| parser.parse_triggered_sentence(tokens)) {
            return Some(Sentence::from_body(body));
        }
        None
    }

    fn sentence_form_is_admitted(&self, tokens: &[Token], sentence: &Sentence) -> bool {
        crate::renderer::sentence_form_is_admitted(
            sentence,
            tokens
                .last()
                .is_some_and(|token| token.kind == TokenKind::Punctuation(Punctuation::Period)),
            self.self_reference.name(),
            self.self_reference.is_legendary(),
            self.quoted_fragment,
        )
    }

    /// Parses a sentence whose body is a trigger clause plus its effect.
    /// Reached only as a staged fallback, after the ordinary sentence parse
    /// has declined, so a `When …, …` sentence that already parses through
    /// [`Subordinator::When`] keeps its existing tree untouched. Not widened to
    /// `parse_paragraph` for the effect: a sentence body is one sentence, and a
    /// multi-sentence effect falls through to `Recovered`, unchanged from
    /// today's behavior.
    fn parse_triggered_sentence(&mut self, tokens: &[Token]) -> Option<SentenceBody> {
        let (introducer, event, intervening_condition, effect) = self.trigger_frame(tokens)?;
        // `Nonterminal::Clause` (unlike `Nonterminal::Sentence`) does not itself
        // expect a trailing terminal period, so the period this staged fallback
        // still carries (the ordinary sentence attempt above already declined)
        // must be peeled first, exactly as `parse_power_toughness_body` and the
        // modal-frame builder do before their own `Nonterminal::Clause`/interior
        // parses.
        // A trigger effect is an independent clause, and often a bare imperative
        // (`copy that spell`, `sacrifice this creature`). `Nonterminal::Clause`
        // does not itself consume a terminal period, so peel it before parsing.
        let effect = self.parse_trigger_effect(effect)?;
        Some(SentenceBody::Triggered(Box::new(TriggeredSentence {
            trigger: TriggerHeader {
                introducer,
                event,
                intervening_condition,
            },
            effect,
        })))
    }

    /// Parses a trigger's effect as an independent clause. A `Choose one`
    /// header or a verbless power/toughness body is not an effect, and neither
    /// can lower through the required [`Clause::Independent`] shape.
    fn parse_trigger_effect(&mut self, effect: &[Token]) -> Option<IndependentClause> {
        let peeled = peel_sentence_ending(effect);
        self.accept_exact(peeled, Nonterminal::Clause, |parsed| {
            let Clause::Independent(independent) = parsed.clause()? else {
                return None;
            };
            Some(independent.clone())
        })
    }

    /// Parses a trailing dash-body appositive sentence: a complete clause
    /// matrix, a spaced ` — `, then a top-level `or`-coordinated run of
    /// independent clauses that spell out the choice the matrix tail named
    /// (`… faces a villainous choice — <clause>, or <clause>`). Both licensing
    /// conditions are structural, never lexical: the matrix must reduce to a
    /// complete independent clause, and the dash body must reduce to an
    /// `or`-coordinated independent clause. A flavor header (`<label> —
    /// <sentence>`) fails the first test — a bare label is not a clause — and a
    /// single-clause dash body fails the second, so neither fires this. Built
    /// directly rather than through the chart because the spaced em dash is not
    /// a chart terminal; the construction only composes two existing clause
    /// parses under a [`ComplexClause`] appositive attachment.
    fn parse_dash_appositive(&mut self, tokens: &[Token]) -> Option<Sentence> {
        let dash = self.spaced_top_level_em_dash(tokens)?;
        let matrix_tokens = &tokens[..dash];
        let body_tokens = tokens.get(dash + 1..)?;
        if matrix_tokens.is_empty() || body_tokens.is_empty() {
            return None;
        }
        // The matrix is a complete independent clause; there is no terminal
        // period on this side of the dash, so it parses as a bare clause.
        let matrix = self.accept_exact(matrix_tokens, Nonterminal::Clause, |parsed| {
            let Clause::Independent(matrix) = parsed.clause()? else {
                return None;
            };
            Some(matrix.clone())
        })?;
        // The dash body is parsed as a whole sentence so its terminal period is
        // consumed like any other; it must reduce to an `or`-coordinated
        // independent clause for the appositive to license.
        let body = self.accept_exact(body_tokens, Nonterminal::Sentence, |parsed| {
            let SentenceBody::Independent(body) = &parsed.sentence()?.body else {
                return None;
            };
            Self::coordinates_with_or(body).then(|| body.clone())
        })?;
        crate::constructions::ability::build_dash_appositive_sentence(matrix, body).ok()
    }

    /// Position of the first top-level spaced em dash (` — `), or `None`. The
    /// dash must be spaced exactly ` {dash} ` (excluding unspaced ranges) and
    /// at the top nesting level (outside quotes and brackets), mirroring
    /// the flavor-header and cost-header peels.
    fn spaced_top_level_em_dash(&self, tokens: &[Token]) -> Option<usize> {
        let mut depth = Nesting::default();
        for (index, token) in tokens.iter().enumerate() {
            if depth.is_top_level()
                && token.kind == TokenKind::Punctuation(Punctuation::EmDash)
                && index > 0
                && index + 1 < tokens.len()
            {
                let dash = self.token_text(token);
                let before_end = tokens[index - 1].span.end;
                let after_start = tokens[index + 1].span.start;
                if self.source.get(before_end..after_start) == Some(&format!(" {dash} ")) {
                    return Some(index);
                }
            }
            depth.observe(token.kind);
        }
        None
    }

    /// Parses a verbless power/toughness sentence body (`3/2.`): a single
    /// power/toughness token terminated by a period. It surfaces as a tiered
    /// mode's whole body, where the mode's effect is the base power and
    /// toughness it sets. The terminal period is required — a bare `N/N` with
    /// no period is instead claimed by the level-band frame in
    /// [`Parser::parse`] as that band's stat line [CR#711.2], never reaching
    /// this production — and the renderer re-derives the period like any
    /// other sentence. The terminal period is what keeps a tiered mode's
    /// `3/2.` sentence's territory distinct from the level band's bare stat
    /// line. Nothing else stands in the sentence, so no clause frame ever
    /// competes; this is reached only after the chart declines the bare value.
    fn parse_power_toughness_body(&self, tokens: &[Token]) -> Option<SentenceBody> {
        let body = peel_sentence_ending(tokens);
        if body.len() == tokens.len() {
            // No terminal period was present: not a P/T sentence.
            return None;
        }
        let [stat] = body else {
            return None;
        };
        if stat.kind != TokenKind::PowerToughness {
            return None;
        }
        let value = super::parse_power_toughness(self.token_text(stat))?;
        Some(SentenceBody::PowerToughness(value))
    }

    /// Whether an independent clause is a top-level coordination carrying at
    /// least one `or` conjunction — the dash-body appositive's licensing gate.
    fn coordinates_with_or(clause: &IndependentClause) -> bool {
        let IndependentClause::Coordinated(coordinated) = clause else {
            return false;
        };
        coordinated
            .rest
            .iter()
            .any(|coordination| coordination.conjunction == Some(Conjunction::Or))
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
            sentences: split_sentences(self.source, body, self.self_reference.nickname())
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
        let body = peel_sentence_ending(tokens);
        if let Some(choice) = self.attempt(|parser| parser.parse_choice_instruction(body)) {
            return Sentence::from_body(SentenceBody::Choice(choice));
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
    ) -> (Option<Box<TriggerHeader>>, &'tokens [Token]) {
        let Some(first) = tokens.first() else {
            return (None, tokens);
        };
        let Some(introducer) = TriggerWord::from_spelling(self.token_text(first)) else {
            return (None, tokens);
        };
        let Some(comma) = find_top_level_punctuation(tokens, Punctuation::Comma) else {
            return (None, tokens);
        };
        let Some(event_tokens) = tokens.get(1..comma) else {
            return (None, tokens);
        };
        let event = if introducer == TriggerWord::At {
            let Some(phrase) = self.accept_exact(event_tokens, Nonterminal::NounPhrase, |parsed| {
                parsed.noun_phrase().cloned()
            }) else {
                return (None, tokens);
            };
            TriggerEvent::Temporal(phrase)
        } else {
            let Some(clause) = self.accept_exact(event_tokens, Nonterminal::Clause, |parsed| {
                let Clause::Independent(clause) = parsed.clause()? else {
                    return None;
                };
                Some(clause.clone())
            }) else {
                return (None, tokens);
            };
            TriggerEvent::Clause(clause)
        };
        let Some(rest) = tokens.get(comma + 1..) else {
            return (None, tokens);
        };
        (
            Some(Box::new(TriggerHeader {
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
        self.accept_exact(tokens, Nonterminal::Sentence, |parsed| {
            let SentenceBody::Independent(IndependentClause::Finite(finite)) =
                &parsed.sentence()?.body
            else {
                return None;
            };
            if finite.subject().is_some() {
                return None;
            }
            let crate::syntax::PredicateExpression::Simple(predicate) = finite.predicate() else {
                return None;
            };
            predicate_is_choose(predicate).then(|| predicate.clone())
        })
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
        // This recognizer reuses `self` rather than opening a fresh `Parser`
        // (unlike `parse_quoted_ability_fragment`, the chart-lowering path
        // for a quote in every other grammatical slot), so the interior's
        // `quoted_fragment` bit must be set and restored by hand around the
        // recursive call rather than at construction time.
        let outer_quoted_fragment = std::mem::replace(&mut self.quoted_fragment, true);
        let ability = self.parse_ability(quoted_tokens);
        self.quoted_fragment = outer_quoted_fragment;
        let quoted = QuotedAbility {
            ability: Box::new(ability),
            initial_uppercase,
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
        let sentence = Sentence::from_body(SentenceBody::Independent(clause));
        let id = ConstructionId::new("sentence");
        let production = ProductionId {
            construction: id,
            ordinal: crate::constructions::sentence::terminal_form_ordinal(),
        };
        let cost = ParseCost::default();
        let family = super::construction::family_by_id(id)
            .expect("the active registry contains the generated sentence family");
        self.selections.push(AbilitySelection {
            span: tokens_span(tokens),
            constituent_spans: Vec::new(),
            rule: None,
            construction: Some(id),
            constructions: vec![ConstructionDecision::new(
                tokens_span(tokens),
                production,
                family,
                cost,
                SelectionReason::Unique,
                vec![ConstructionAlternative::new(production, cost, false)],
            )],
            tied_alternatives: vec![0],
            cost,
            chart_stats: ChartStats::default(),
            forest_stats: ForestStats::default(),
        });
        Some(sentence)
    }

    /// Attaches the ability owner's typed quoted `with` postmodifier.
    fn quoted_with_clause(
        &mut self,
        prefix: &[Token],
        quoted: QuotedAbility,
    ) -> Option<IndependentClause> {
        let clause = self.accept_exact(prefix, Nonterminal::SimpleClause, |parsed| {
            parsed.simple_clause().cloned()
        })?;
        let subject = clause.subject;
        let mut predicate = clause.predicate;
        if subject.is_none() && predicate.declaration_verb_slot() == VerbSlot::Infinitive {
            predicate = predicate.declaration_as_imperative()?;
        }
        let predicate = crate::constructions::predicate::build_verb_phrase_ability_postmodifier(
            predicate,
            crate::syntax::AbilityPostmodifier::from_quoted_ability(quoted),
        )
        .ok()?;
        finish_simple_clause(super::SimpleClause {
            subject,
            predicate,
            attachment: clause.attachment,
        })
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
        let clause = if let Some(clause) =
            self.accept_exact(prefix, Nonterminal::SimpleClause, |parsed| {
                parsed.simple_clause().cloned()
            }) {
            clause
        } else {
            // A required-object grant verb (`this creature has`) will not parse without an
            // object of its own. Supply a sentinel ability complement so the
            // prefix parses, then drop it — the quoted ability takes the freed
            // object slot.
            let probe = format!("{} {GRANT_OBJECT_SENTINEL}", self.tokens_text(prefix));
            let clause = parse_nonterminal_with_self_reference(
                &probe,
                self.catalogs,
                Nonterminal::SimpleClause,
                self.self_reference,
            )
            .ok()?
            .simple_clause()?
            .clone();
            let (predicate, sentinel) =
                crate::constructions::predicate::parts_verb_phrase_ability(&clause.predicate)?;
            if sentinel.spelling() != GRANT_OBJECT_SENTINEL {
                return None;
            }
            super::SimpleClause {
                subject: clause.subject,
                predicate,
                attachment: clause.attachment,
            }
        };
        let predicate = crate::constructions::predicate::build_verb_phrase_quoted_ability(
            clause.predicate,
            quoted,
        )
        .ok()?;
        finish_simple_clause(super::SimpleClause {
            subject: clause.subject,
            predicate,
            attachment: clause.attachment,
        })
    }

    fn parse_keyword_line_construction(&mut self, tokens: &[Token]) -> Option<KeywordAbilityList> {
        if tokens.is_empty() {
            return None;
        }
        // The ordinary splitter may accept one or more leading chunks before
        // a later chunk proves this is a single comma-bearing argument. Keep
        // that failed alternative transactional so its chart provenance does
        // not survive beside the whole-line fallback's selected tree.
        let (abilities, trailing) = if let Some(parts) =
            self.attempt(|parser| parser.recognize_keyword_item_parts(tokens))
        {
            parts
        } else {
            // Fallback: a single unsplit line whose sole item is a Stage B
            // restriction-plus-cost whose restriction is itself a comma-
            // coordinated quality list (`Equip Shaman, Warlock, or Wizard {1}`,
            // `Craft with a Dinosaur, a Merfolk, a Pirate, and a Vampire {4}`).
            // The ordinary per-chunk splitter above always fails these lines
            // first — a bare coordinated member (`Warlock`, `a Merfolk`) never
            // independently matches a keyword atom, so the strict "every chunk
            // must match" loop aborts exactly as it always has. Only then do we
            // retry the whole line as one item, letting the noun-phrase grammar
            // (not a second comma-splitter) parse the coordination. This never
            // fires for an ordinary multi-keyword line (`Flying, Ward {2}`)
            // because that line already succeeds in the ordinary path above,
            // nor for a spaced-em-dash designation header (`Solved — …`)
            // because its separator is `SpacedEmDash`, rejected below.
            self.recognize_single_restricted_cost_parts(tokens)?
        };
        let list = crate::constructions::ability::build_keyword_list(abilities, trailing)
            .expect("the keyword-line recognizer satisfies the declaration");
        let form = crate::constructions::ability::selected_keyword_line_form(&list)
            .expect("the declaration assigns every keyword line one form")
            .ordinal;
        self.record_ability_construction(tokens, "keyword_line", form);
        Some(list)
    }

    fn recognize_keyword_item_parts(
        &mut self,
        tokens: &[Token],
    ) -> Option<(
        SeparatedNonEmpty<KeywordAbility, KeywordListSeparator>,
        Option<Paragraph>,
    )> {
        let chunks = self.split_keyword_items(tokens);
        let in_list = chunks.len() > 1;
        let mut abilities = Vec::with_capacity(chunks.len());
        let mut trailing = None;
        for (preceding_separator, chunk) in chunks {
            let (atom, matched_end) = self.longest_ability_item_atom(chunk)?;
            let carries_from = super::keyword_atom_carries_from(&atom);
            // An atom that already spells its own preposition has consumed the
            // one introducing its argument, so what follows is that
            // preposition's complement, never a bare object.
            let bare_object = !super::keyword_atom_carries_preposition(&atom);
            let argument_tokens = &chunk[matched_end..];
            let ability_end = chunk.get(matched_end.checked_sub(1)?)?.span.end;
            let argument = if in_list {
                self.parse_keyword_argument(
                    argument_tokens,
                    ability_end,
                    in_list,
                    carries_from,
                    bare_object,
                )?
            } else {
                let (argument, tail) = self.parse_keyword_argument_with_tail(
                    argument_tokens,
                    ability_end,
                    carries_from,
                    bare_object,
                )?;
                trailing = tail;
                argument
            };
            abilities.push((
                preceding_separator,
                KeywordAbility {
                    ability: atom,
                    argument,
                },
            ));
        }
        let mut abilities = abilities.into_iter();
        let (first_separator, first) = abilities.next()?;
        if first_separator.is_some() {
            return None;
        }
        let rest = abilities
            .map(|(separator, ability)| Some(Separated::new(separator?, ability)))
            .collect::<Option<Vec<_>>>()?;
        Some((SeparatedNonEmpty::new(first, rest), trailing))
    }

    /// The single-item fallback described on
    /// [`Self::parse_keyword_line_construction`]. Requires the whole line
    /// to open `<catalog atom><space>` and its argument to shape as
    /// [`KeywordArgument::RestrictedCost`] — the only shape whose
    /// restriction may itself contain the top-level commas that defeat the
    /// ordinary splitter.
    fn recognize_single_restricted_cost_parts(
        &mut self,
        tokens: &[Token],
    ) -> Option<(
        SeparatedNonEmpty<KeywordAbility, KeywordListSeparator>,
        Option<Paragraph>,
    )> {
        let (atom, matched_end) = self.longest_ability_item_atom(tokens)?;
        let argument_tokens = &tokens[matched_end..];
        let ability_end = tokens.get(matched_end.checked_sub(1)?)?.span.end;
        let (separator, body) = split_keyword_argument_separator(argument_tokens, ability_end);
        if separator != KeywordArgumentSeparator::Space {
            return None;
        }
        let argument = self.parse_restricted_cost(body)?;
        Some((
            SeparatedNonEmpty::new(
                KeywordAbility {
                    ability: atom,
                    argument,
                },
                Vec::new(),
            ),
            None,
        ))
    }

    /// Splitter-time keyword-item boundaries, aware of a tight em-dash cost's
    /// internal commas. A method (not the free function it used to be) so it
    /// can consult the catalog to tell a tight-cost item's argument commas
    /// (never a boundary) from an ordinary keyword-list comma.
    fn split_keyword_items<'tokens>(
        &self,
        tokens: &'tokens [Token],
    ) -> Vec<(Option<KeywordListSeparator>, &'tokens [Token])> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut preceding = None;
        let mut depth = Nesting::default();
        let mut tight_cost: Option<bool> = None;
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
            match separator {
                Some(KeywordListSeparator::Comma) => {
                    let is_tight = *tight_cost
                        .get_or_insert_with(|| self.item_opens_tight_cost(&tokens[start..]));
                    if is_tight {
                        continue;
                    }
                    chunks.push((preceding, &tokens[start..index]));
                    preceding = Some(KeywordListSeparator::Comma);
                    start = index + 1;
                    tight_cost = None;
                }
                Some(KeywordListSeparator::Semicolon) => {
                    chunks.push((preceding, &tokens[start..index]));
                    preceding = Some(KeywordListSeparator::Semicolon);
                    start = index + 1;
                    tight_cost = None;
                }
                None => depth.observe(token.kind),
            }
        }
        chunks.push((preceding, &tokens[start..]));
        chunks
    }

    /// Whether `item` (the tokens from a prospective keyword item's start to
    /// the end of the remaining input) opens `<catalog atom><tight em dash>`
    /// — the shape the comma-boundary exemption is licensed for. Both the
    /// tight/spaced separator classification and the catalog atom match are
    /// surface facts, never a keyword spelling.
    fn item_opens_tight_cost(&self, item: &[Token]) -> bool {
        (|| -> Option<bool> {
            let (_, matched_end) = self.longest_ability_item_atom(item)?;
            let next = item.get(matched_end)?;
            if next.kind != TokenKind::Punctuation(Punctuation::EmDash) {
                return Some(false);
            }
            let prev = item.get(matched_end.checked_sub(1)?)?;
            let tight_before = next.span.start == prev.span.end;
            let tight_after = item
                .get(matched_end + 1)
                .is_some_and(|token| token.span.start == next.span.end);
            Some(tight_before && tight_after)
        })()
        .unwrap_or(false)
    }

    /// The longest `CatalogSlot::AbilityItem` atom matching the start of
    /// `tokens`, and the token index just past it. Shared by the real item
    /// parser ([`Self::recognize_keyword_item_parts`]) and the splitter's
    /// tight-cost lookahead ([`Self::item_opens_tight_cost`]) so both apply
    /// the same `max_by_key(length)` and token-boundary-alignment filter —
    /// the only place the keyword catalog participates. Returns an atom for
    /// the AST but passes no canonical/spelling identity to any shape parser.
    fn longest_ability_item_atom(&self, tokens: &[Token]) -> Option<(CatalogAtom, usize)> {
        let first = tokens.first()?;
        let suffix = self.source.get(first.span.start..)?;
        self.catalogs
            .matches(suffix, CatalogSlot::AbilityItem)
            .into_iter()
            .filter_map(|catalog_match| {
                let CatalogValue::Atom(atom) = catalog_match.value else {
                    return None;
                };
                let byte_end = first.span.start.checked_add(catalog_match.length)?;
                let token_end = token_boundary(tokens, byte_end)?;
                Some((catalog_match.length, atom, token_end))
            })
            .max_by_key(|(length, _, _)| *length)
            .map(|(_, atom, end)| (atom, end))
    }

    /// Like [`Self::parse_keyword_argument`], but licensed only for a
    /// single-keyword list (`!in_list`): if the argument is a tight em-dash
    /// cost whose body contains a top-level sentence terminal with more
    /// tokens after it on the same line, split the tail off as a same-line
    /// trailing paragraph rather than feeding it to `parse_cost`. The split
    /// is committed only when the prefix actually shapes as the structured
    /// cost; otherwise this falls back to the ordinary single path.
    fn parse_keyword_argument_with_tail(
        &mut self,
        argument_tokens: &[Token],
        ability_end: usize,
        carries_from: bool,
        bare_object: bool,
    ) -> Option<(KeywordArgument, Option<Paragraph>)> {
        let (separator, body) = split_keyword_argument_separator(argument_tokens, ability_end);
        if separator == KeywordArgumentSeparator::EmDash
            && let Some(split_index) = find_first_top_level_sentence_terminal(body)
            && split_index + 1 < body.len()
        {
            let cost_part = &body[..=split_index];
            let tail_part = &body[split_index + 1..];
            let candidate = self.parse_tight_keyword_cost(cost_part);
            if matches!(
                candidate,
                KeywordArgument::Costed(KeywordCost::Components { .. })
            ) {
                let tail = self.parse_paragraph(tail_part);
                return Some((candidate, Some(tail)));
            }
        }
        let argument = self.parse_keyword_argument(
            argument_tokens,
            ability_end,
            false,
            carries_from,
            bare_object,
        )?;
        Some((argument, None))
    }

    /// Parses a tight em-dash keyword cost body: peels at most one final
    /// sentence terminal (recorded only as present/absent — see
    /// [`KeywordCost::Components`]'s doc comment), then lowers the remainder
    /// through the existing infallible [`Self::parse_cost`]. Always called on
    /// an `EmDash`-separated body; the separator itself is not stored (see
    /// the same doc comment).
    fn parse_tight_keyword_cost(&mut self, body: &[Token]) -> KeywordArgument {
        let (cost_tokens, terminal) = match body.split_last() {
            Some((last, rest)) if is_sentence_terminal(last.kind) => (rest, true),
            _ => (body, false),
        };
        let cost = self.parse_cost(cost_tokens);
        KeywordArgument::Costed(KeywordCost::Components { cost, terminal })
    }

    /// Parses a keyword ability's argument from the tokens trailing its atom,
    /// consulting only the closed shape vocabulary [`KeywordArgument`] and
    /// nothing about the keyword. Returns `None` to reject the whole
    /// keyword line — a single line whose trailing tokens are neither
    /// empty, a recognized shape, nor a symbol-class run (which a keyword
    /// argument always is) is ordinary rules text and must reparse as such.
    /// `ability_end` is the byte offset just past the keyword atom, read
    /// only for the leading separator's dash spacing.
    fn parse_keyword_argument(
        &mut self,
        argument_tokens: &[Token],
        ability_end: usize,
        in_list: bool,
        carries_from: bool,
        bare_object: bool,
    ) -> Option<KeywordArgument> {
        if argument_tokens.is_empty() {
            return Some(KeywordArgument::Absent);
        }
        // A granted keyword ability's own closing sentence terminal is never
        // part of a symbol argument. Identify the terminal once for both
        // supported cases, including inside a comma list: a lone terminal
        // rejects the keyword line, while a symbol run is peeled before
        // shaping. Other argument shapes retain punctuation they may need to
        // distinguish a sentence cost from a bare label.
        let without_terminal = argument_tokens
            .split_last()
            .and_then(|(terminal, body)| is_sentence_terminal(terminal.kind).then_some(body));
        if without_terminal == Some(&[]) {
            return None;
        }
        let argument_tokens = without_terminal
            .filter(|tokens| {
                let (_, body) = split_keyword_argument_separator(tokens, ability_end);
                symbol_cost(self.tokens_text(body)).is_some()
            })
            .unwrap_or(argument_tokens);
        let (separator, body) = split_keyword_argument_separator(argument_tokens, ability_end);
        if let Some(argument) =
            self.parse_shaped_argument(separator, body, in_list, carries_from, bare_object)
        {
            return Some(argument);
        }
        // A bare noun-phrase quality (`KeywordArgument::Qualified`) has no
        // fixed vocabulary the `symbol_cost` peek above can recognize, so it
        // was never eligible for that upfront peel and the attempt just
        // above still held the terminal. Retry `parse_qualified` alone
        // (never the full `parse_shaped_argument` cascade — see why below)
        // with the terminal removed, but ONLY keep the retry when it makes
        // the *whole* remaining body resolve as a noun phrase, and only
        // inside a quoted ability, where the renderer can actually put the
        // period back.
        //
        // Two failure modes ruled out the broader designs this went
        // through first (round `kwbandsother`):
        //
        // - Peeling whenever the categorical surface allowed it (`Space`, `!in_list`,
        //   `bare_object`), without checking what the stripped body actually parsed as,
        //   silently dropped the closing period on a keyword line followed by ordinary
        //   trailing prose on the same physical line — `Equip {3}. This ability costs
        //   …`, `Kicker {X}. X can't be 0.`, `Splice onto Arcane—Exile four cards from
        //   your graveyard.` — 13 unrelated faces stopped round-tripping. Trying the
        //   retry through the *real* parse and keeping it only on success rules this
        //   out on its own: none of those bodies resolve as one closed shape even with
        //   the final period gone, so the terminal stays put and they fall through to
        //   the unmodified recovery path below exactly as before.
        // - Validating success via the *whole* `parse_shaped_argument` cascade (rather
        //   than `parse_qualified` alone) is still unsound:
        //   `parse_named_keyword_argument` is tried first there, and a synthetic `Gift
        //   a Food.` strips to `a Food`, an exact `NAMED_KEYWORD_ARGUMENT_LABELS`
        //   member — `Named`'s own license is exact-string matching, and "a Food" with
        //   the period is deliberately *not* a licensed spelling of it (see
        //   `named_keyword_argument_label_license_is_exact_and_keyword_independent`).
        //   Narrowing the retry to `parse_qualified` specifically avoids that collision
        //   entirely, rather than special-casing it away.
        // - Even `parse_qualified` alone is unsound outside a quote: a top-level
        //   (unquoted) `AbilityKind::Keyword` never derives its own trailing period
        //   (`keyword_ability_list`'s doc comment) — only `Renderer::quoted_ability`'s
        //   terminal-quote handling reprints one, and only for
        //   `Costed(Symbols)`/`Qualified`. A bare `Gift a Food.` as a whole face's
        //   oracle text would resolve to `Qualified` and then render as `Gift a Food` —
        //   the period silently lost again, just past a different assertion. Gating on
        //   `self.quoted_fragment` (set only by `parse_quoted_ability_
        //   fragment`/`parse_quoted_sentence`, the two paths whose result reaches
        //   `quoted_ability`) ties the peel to the one place that can actually put the
        //   period back.
        //
        // `bare_object` still scopes which keywords are even eligible,
        // matching `parse_qualified`'s own gate (an atom that has already
        // consumed its own preposition, like `Hexproof from`, is excluded).
        // `bands with other legendary creatures.` [CR#702.22b,702.22c] is
        // the corpus's first witness; the retry is general to the shape,
        // not keyed to this keyword.
        if self.quoted_fragment
            && separator == KeywordArgumentSeparator::Space
            && !in_list
            && bare_object
            && let Some(stripped) = without_terminal
            && stripped.len() != argument_tokens.len()
        {
            let (_, stripped_body) = split_keyword_argument_separator(stripped, ability_end);
            if let Some(argument) =
                self.attempt(|parser| parser.parse_qualified(stripped_body, in_list, bare_object))
            {
                return Some(argument);
            }
        }
        // No closed shape matched. On a single keyword line, keep the argument
        // only when it opens the way a keyword argument does — a symbol-class run
        // or a `from`/`for` quality filter — even though the grammar cannot yet
        // structure it; otherwise the line is ordinary rules text. A
        // comma/semicolon list has already committed to keyword items, so a
        // recovered argument stays attached there regardless.
        if !in_list && !opens_like_keyword_argument(body, self.source) {
            return None;
        }
        Some(self.recovered_keyword_argument(body))
    }

    /// Attempts each closed argument shape against the surface of `body`, the
    /// argument tokens after the leading `separator`. Returns `None` when no
    /// shape matches (the caller decides whether to recover or reject).
    fn parse_shaped_argument(
        &mut self,
        separator: KeywordArgumentSeparator,
        body: &[Token],
        in_list: bool,
        carries_from: bool,
        bare_object: bool,
    ) -> Option<KeywordArgument> {
        if let Some(named) = self.parse_named_keyword_argument(separator, body) {
            return Some(named);
        }
        if separator != KeywordArgumentSeparator::Space {
            if body.is_empty() {
                return None;
            }
            // A tight em-dash argument is a structured cost — `cumulative
            // upkeep—Sacrifice a creature` — told from a bare pairing label
            // (`partner—Friends forever`, already returned above) by surface
            // alone: a label carries no sentence-terminal or clause
            // punctuation. A spaced em-dash argument keeps the legacy
            // designation-headed embedded-ability surface.
            if separator == KeywordArgumentSeparator::EmDash {
                return Some(self.parse_tight_keyword_cost(body));
            }
            // Only reachable with `separator == SpacedEmDash`: see
            // `KeywordCost::Sentence`'s doc comment.
            let ability = self.parse_ability(body);
            return Some(KeywordArgument::Costed(KeywordCost::Sentence {
                ability: Box::new(ability),
            }));
        }
        self.parse_space_argument(body, in_list, carries_from, bare_object)
    }

    fn parse_named_keyword_argument(
        &self,
        separator: KeywordArgumentSeparator,
        body: &[Token],
    ) -> Option<KeywordArgument> {
        let label = self.tokens_text(body);
        ((separator == KeywordArgumentSeparator::Space && is_named_keyword_argument_label(label))
            || (separator != KeywordArgumentSeparator::Space
                && em_dash_keyword_label_is_bare(label)))
        .then(|| KeywordArgument::Named {
            separator,
            label: label.to_owned(),
        })
    }

    fn parse_space_argument(
        &mut self,
        body: &[Token],
        in_list: bool,
        carries_from: bool,
        bare_object: bool,
    ) -> Option<KeywordArgument> {
        // An internal em dash pairs a count with a cost (`suspend N—[cost]`) or a
        // cost with power/toughness (`prototype [cost] — [P]/[T]`).
        if let Some(dash) = body
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::EmDash))
        {
            let left = &body[..dash];
            let right = body.get(dash + 1..)?;
            return self
                .parse_counted_cost(left, right)
                .or_else(|| self.parse_statted(left, right))
                .or_else(|| self.parse_restricted_tight_cost(left, &body[dash], right));
        }
        if let Some(counted) = self.parse_counted(body) {
            return Some(counted);
        }
        if let Some(symbols) = symbol_cost(self.tokens_text(body)) {
            return Some(KeywordArgument::Costed(KeywordCost::Symbols(symbols)));
        }
        if let Some(restricted) = self.parse_restricted_cost(body) {
            return Some(restricted);
        }
        if let Some(predicated) = self.parse_predicated(body, in_list, carries_from) {
            return Some(predicated);
        }
        self.parse_qualified(body, in_list, bare_object)
    }

    /// The bare noun-phrase argument [CR#702.5a] gives enchant (`Enchant
    /// [object or player]`) and [CR#702.72a] gives champion (`Champion an
    /// [object]`) — no preposition, no cost, no list.
    ///
    /// Tried last, once every closed shape above has declined, and gated
    /// categorically rather than by keyword name: the atom must not spell its
    /// own preposition (`Partner with`, `Hexproof from` have already consumed
    /// it, so their tail is that preposition's complement — a card name or a
    /// quality — not an object), the keyword must stand alone on its line, and
    /// the tokens must parse *exactly* as one noun phrase.
    fn parse_qualified(
        &mut self,
        body: &[Token],
        in_list: bool,
        bare_object: bool,
    ) -> Option<KeywordArgument> {
        if !bare_object || in_list || body.is_empty() {
            return None;
        }
        self.accept_exact(body, Nonterminal::NounPhrase, |parsed| {
            Some(KeywordArgument::Qualified(Phrase::NounPhrase(Box::new(
                parsed.noun_phrase()?.clone(),
            ))))
        })
    }

    /// Stage B's restriction-plus-final-symbol-cost shape: a quality
    /// restriction (optionally introduced by `onto`/`with`) followed by a
    /// bare mana/symbol cost with no internal em dash — `craft with artifact
    /// {1}{U}`, `equip legendary creature {1}`. The final symbol run is the
    /// categorical surface gate: only tried after the whole-body symbol-cost
    /// and counted-cost attempts above have already declined, so `Equip {3}`
    /// never reaches here.
    fn parse_restricted_cost(&mut self, body: &[Token]) -> Option<KeywordArgument> {
        let (last, rest) = body.split_last()?;
        if !matches!(
            last.kind,
            TokenKind::OracleSymbol | TokenKind::SymbolSequence
        ) {
            return None;
        }
        let symbols = symbol_cost(self.token_text(last))?;
        if rest.is_empty() {
            return None;
        }
        let (preposition, restriction_tokens) = peel_restriction_preposition(rest, self.source);
        if restriction_tokens.is_empty() {
            return None;
        }
        // Reject the general `Quantity + that + …` headless-relative
        // surface (Eye of Ojer Taq's `two that share a card type`) before
        // the whole noun-phrase parse can license an opaque numeral. This
        // gates the *shape*, never the card, the keyword, or the word `two`.
        if self.restriction_opens_quantity_that(restriction_tokens) {
            return None;
        }
        let restriction =
            self.accept_exact(restriction_tokens, Nonterminal::NounPhrase, |parsed| {
                parsed.noun_phrase().cloned()
            })?;
        Some(KeywordArgument::RestrictedCost {
            preposition,
            restriction: Box::new(restriction),
            cost: KeywordCost::Symbols(symbols),
        })
    }

    /// Stage A′'s composition: a restriction joined to a tight em-dash
    /// structured cost by an internal dash — `splice onto Arcane—Exile four
    /// cards from your graveyard.`. Tried only after `parse_counted_cost`
    /// and `parse_statted` have already declined on this same internal
    /// dash, and only when it is tight (no surrounding whitespace) and the
    /// left side carries an *explicit* `onto`/`with` preposition — the
    /// requirement that keeps `Reinforce X—[cost]` (whose left side is a
    /// bare `Quantity::unchecked_x()` with no preposition at all) outside this
    /// shape.
    fn parse_restricted_tight_cost(
        &mut self,
        left: &[Token],
        dash: &Token,
        right: &[Token],
    ) -> Option<KeywordArgument> {
        if right.is_empty() || left.is_empty() {
            return None;
        }
        let prev = left.last()?;
        let tight_before = dash.span.start == prev.span.end;
        let tight_after = right
            .first()
            .is_some_and(|token| token.span.start == dash.span.end);
        if !(tight_before && tight_after) {
            return None;
        }
        let (preposition, restriction_tokens) = peel_restriction_preposition(left, self.source);
        preposition?;
        if restriction_tokens.is_empty() {
            return None;
        }
        let restriction =
            self.accept_exact(restriction_tokens, Nonterminal::NounPhrase, |parsed| {
                parsed.noun_phrase().cloned()
            })?;
        let KeywordArgument::Costed(cost) = self.parse_tight_keyword_cost(right) else {
            return None;
        };
        Some(KeywordArgument::RestrictedCost {
            preposition,
            restriction: Box::new(restriction),
            cost,
        })
    }

    /// Whether `tokens` opens with a `Quantity` immediately followed by
    /// `that` — the headless quantity-plus-relative surface that must stay a
    /// whole-clause recovery rather than force the quantity's numeral into
    /// `Noun::Opaque`.
    fn restriction_opens_quantity_that(&mut self, tokens: &[Token]) -> bool {
        let Some(that_index) = tokens.iter().position(|token| {
            token.kind == TokenKind::Word && self.token_text(token).eq_ignore_ascii_case("that")
        }) else {
            return false;
        };
        if that_index == 0 {
            return false;
        }
        self.parse_exact(&tokens[..that_index], Nonterminal::Quantity)
            .is_some()
    }

    fn parse_counted(&mut self, body: &[Token]) -> Option<KeywordArgument> {
        self.accept_exact(body, Nonterminal::Quantity, |parsed| {
            Some(KeywordArgument::Counted(*parsed.quantity()?))
        })
    }

    fn parse_counted_cost(&self, left: &[Token], right: &[Token]) -> Option<KeywordArgument> {
        let [count] = left else {
            return None;
        };
        if count.kind != TokenKind::Integer {
            return None;
        }
        let count = arabic_number_literal(self.token_text(count))?;
        let symbols = symbol_cost(self.tokens_text(right))?;
        Some(KeywordArgument::CountedCost { count, symbols })
    }

    fn parse_statted(&self, left: &[Token], right: &[Token]) -> Option<KeywordArgument> {
        let symbols = symbol_cost(self.tokens_text(left))?;
        let [stats] = right else {
            return None;
        };
        if stats.kind != TokenKind::PowerToughness {
            return None;
        }
        let stats = super::parse_power_toughness(self.token_text(stats))?;
        Some(KeywordArgument::Statted { symbols, stats })
    }

    fn parse_predicated(
        &mut self,
        body: &[Token],
        in_list: bool,
        carries_from: bool,
    ) -> Option<KeywordArgument> {
        // A `from`/`for` argument is the coordinable quality filter and reads on
        // its own line (`protection from red`, `affinity for artifacts`); any
        // other prepositional or bare quality (`hexproof from blue`, where the
        // keyword atom carries the `from`; a `Champion of Freedom` name fragment a
        // comma split off) reads only inside a keyword list, or on a single line
        // whose matched atom itself carries a final `from` (Stage C, `kwgrant`
        // round) — the closed catalog-surface property `carries_from`, never a
        // keyword-name list.
        let starts_from_for = body
            .first()
            .is_some_and(|token| predicated_preposition(self.token_text(token)).is_some());
        let segments = if starts_from_for {
            split_coordinated_predicates(body, self.source)
        } else if in_list || carries_from {
            vec![body]
        } else {
            return None;
        };
        let mut qualities = Vec::with_capacity(segments.len());
        for segment in segments {
            qualities.push(self.parse_predicated_quality(segment)?);
        }
        (!qualities.is_empty()).then_some(KeywordArgument::Predicated(PredicatedArgument {
            qualities,
        }))
    }

    fn parse_predicated_quality(&mut self, segment: &[Token]) -> Option<PredicatedQuality> {
        // The `from [color]` shorthand does not reach the noun-phrase grammar; read
        // the color directly, as the single-quality path always has.
        if let Some(first) = segment.first()
            && predicated_preposition(self.token_text(first)) == Some(Preposition::From)
            && let [color_token] = &segment[1..]
            && let Some(color) = color_word(self.token_text(color_token))
        {
            return Some(PredicatedQuality {
                preposition: Some(Preposition::From),
                quality: Phrase::ColorWord(color),
            });
        }
        // Any prepositional phrase, carrying its actual preposition.
        if let Some(prepositional) =
            self.accept_exact(segment, Nonterminal::PrepositionalPhrase, |parsed| {
                parsed.prepositional_phrase().cloned()
            })
            && let PrepositionalPhraseKind::Simple(simple) = prepositional.kind()
        {
            return Some(PredicatedQuality {
                preposition: Some(simple.preposition()),
                quality: match simple.object().kind() {
                    crate::syntax::PrepositionalObjectKind::NounPhrase(value) => {
                        Phrase::NounPhrase(value.clone())
                    }
                    crate::syntax::PrepositionalObjectKind::PrepositionalPhrase(value) => {
                        Phrase::PrepositionalPhrase(value.clone())
                    }
                    crate::syntax::PrepositionalObjectKind::GerundClause(value) => {
                        Phrase::Clause(Box::new(crate::syntax::Clause::Dependent(
                            crate::syntax::DependentClause::Gerund(value.as_ref().clone()),
                        )))
                    }
                    crate::syntax::PrepositionalObjectKind::Adverb(value) => Phrase::Adverb(*value),
                },
            });
        }
        // A bare quality with no preposition (`hexproof from blue` → `blue`).
        if let [token] = segment
            && let Some(color) = color_word(self.token_text(token))
        {
            return Some(PredicatedQuality {
                preposition: None,
                quality: Phrase::ColorWord(color),
            });
        }
        // A bare adjective quality (`monocolored`) — `kwgrant` round Stage C.
        // `monocolored` is an adjective (`regular-vocabulary.tsv:526`), not a
        // noun, so it needs its own exact-parse arm between the color and
        // noun-phrase cases or it falls through to the noun-phrase attempt
        // and fails.
        if let Some(adjective) =
            self.accept_exact(segment, Nonterminal::AdjectivePhrase, |parsed| {
                parsed.adjective_phrase().cloned()
            })
        {
            return Some(PredicatedQuality {
                preposition: None,
                quality: Phrase::AdjectivePhrase(Box::new(adjective)),
            });
        }
        self.accept_exact(segment, Nonterminal::NounPhrase, |parsed| {
            Some(PredicatedQuality {
                preposition: None,
                quality: Phrase::NounPhrase(Box::new(parsed.noun_phrase()?.clone())),
            })
        })
    }

    fn recovered_keyword_argument(&mut self, body: &[Token]) -> KeywordArgument {
        self.diagnostics.push(AbilityDiagnostic {
            kind: AbilityDiagnosticKind::NoCompleteParse,
            span: tokens_span(body),
        });
        KeywordArgument::Recovered {
            text: RecoveredText::new(self.tokens_text(body), body.len()),
        }
    }

    fn accept_exact<T>(
        &mut self,
        tokens: &[Token],
        nonterminal: Nonterminal,
        accept: impl FnOnce(&ParsedNonterminal) -> Option<T>,
    ) -> Option<T> {
        let span = tokens_span(tokens);
        let parsed = self.parse_exact(tokens, nonterminal)?;
        let accepted = accept(&parsed)?;
        let quoted_ability_spans = parsed
            .quoted_ability_spans()
            .iter()
            .map(|quoted| Span::new(span.start + quoted.start, span.start + quoted.end))
            .collect::<Vec<_>>();
        self.selections.push(AbilitySelection {
            span,
            constituent_spans: parsed
                .constituent_spans()
                .iter()
                .map(|constituent| {
                    Span::new(span.start + constituent.start, span.start + constituent.end)
                })
                .collect(),
            rule: parsed.root_rule(),
            construction: parsed
                .root_production()
                .map(|production| production.construction),
            constructions: parsed
                .construction_decisions()
                .iter()
                .cloned()
                .map(|decision| decision.offset(span.start))
                .collect(),
            tied_alternatives: parsed.root_tied_alternatives().to_vec(),
            cost: parsed.cost(),
            chart_stats: parsed.chart_stats(),
            forest_stats: parsed.forest_stats(),
        });
        for quoted_ability_span in quoted_ability_spans {
            self.record_quoted_ability_selections(quoted_ability_span);
        }
        Some(accepted)
    }

    /// Parse a chart nonterminal without recording it as accepted provenance.
    /// Used only by shape probes whose success makes an enclosing production
    /// decline; accepted subtrees go through [`Self::accept_exact`] instead.
    fn parse_exact(&self, tokens: &[Token], nonterminal: Nonterminal) -> Option<ParsedNonterminal> {
        if tokens.is_empty() {
            return None;
        }
        let span = tokens_span(tokens);
        super::parse_nonterminal_with_self_reference_and_activation(
            span.text(self.source)?,
            self.catalogs,
            nonterminal,
            self.self_reference,
            self.activation,
        )
        .ok()
    }

    fn record_quoted_ability_selections(&mut self, span: Span) {
        let Some(fragment) = span.text(self.source) else {
            return;
        };
        let surface = lex(fragment);
        let tokens = collapse_full_names(fragment, surface.tokens, self.self_reference.full_name());
        let selections = {
            let mut parser = Parser::new_with_activation(
                fragment,
                self.catalogs,
                self.self_reference,
                true,
                self.activation,
            );
            parser.parse_ability(&tokens);
            parser.selections
        };
        self.selections
            .extend(selections.into_iter().map(|selection| {
                AbilitySelection {
                    span: offset_span(selection.span, span.start),
                    constituent_spans: selection
                        .constituent_spans
                        .into_iter()
                        .map(|constituent| offset_span(constituent, span.start))
                        .collect(),
                    rule: selection.rule,
                    construction: selection.construction,
                    constructions: selection
                        .constructions
                        .into_iter()
                        .map(|decision| decision.offset(span.start))
                        .collect(),
                    tied_alternatives: selection.tied_alternatives,
                    cost: selection.cost,
                    chart_stats: selection.chart_stats,
                    forest_stats: selection.forest_stats,
                }
            }));
    }

    fn attempt<T>(&mut self, parse: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let diagnostics = self.diagnostics.len();
        let selections = self.selections.len();
        let parsed = parse(self);
        if parsed.is_none() {
            self.diagnostics.truncate(diagnostics);
            self.selections.truncate(selections);
        }
        parsed
    }

    #[allow(
        dead_code,
        reason = "diagnostic entry points retain a whole-phrase recovery constructor"
    )]
    fn recovered_phrase(&mut self, tokens: &[Token]) -> Phrase {
        Phrase::Recovered(self.recovered_text(tokens))
    }

    fn recovered_text(&mut self, tokens: &[Token]) -> RecoveredText {
        self.diagnostics.push(AbilityDiagnostic {
            kind: AbilityDiagnosticKind::NoCompleteParse,
            span: tokens_span(tokens),
        });
        RecoveredText::new(self.tokens_text(tokens), tokens.len())
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

fn lines_span(lines: &[&[Token]]) -> Span {
    let first = lines.iter().find_map(|line| line.first());
    let last = lines.iter().rev().find_map(|line| line.last());
    match (first, last) {
        (Some(first), Some(last)) => Span::new(first.span.start, last.span.end),
        _ => Span::default(),
    }
}

fn split_sentences<'tokens>(
    source: &str,
    tokens: &'tokens [Token],
    nickname: Option<&[String]>,
) -> Vec<&'tokens [Token]> {
    let mut sentences = Vec::new();
    let mut start = 0;
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        let was_quoted = depth.double_quote;
        depth.observe(token.kind);
        if is_sentence_terminal(token.kind)
            && depth.is_top_level()
            && !is_interior_nickname_period(source, tokens, index, nickname)
        {
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

/// Whether `index` is a period strictly inside an exact token-run spelling of
/// the face's shortened name. The run remains ordinary surface tokens so the
/// chart can still choose between a self-reference and a colliding lexical
/// reading; this guard only keeps sentence segmentation from destroying that
/// choice before the chart sees it.
fn is_interior_nickname_period(
    source: &str,
    tokens: &[Token],
    index: usize,
    nickname: Option<&[String]>,
) -> bool {
    if tokens.get(index).map(|token| token.kind)
        != Some(TokenKind::Punctuation(Punctuation::Period))
    {
        return false;
    }
    let Some(nickname) = nickname.filter(|nickname| nickname.len() >= 3) else {
        return false;
    };
    tokens
        .windows(nickname.len())
        .enumerate()
        .any(|(start, run)| {
            start < index
                && index < start + nickname.len() - 1
                && run
                    .iter()
                    .zip(nickname)
                    .enumerate()
                    .all(|(offset, (token, spelling))| {
                        let Some(actual) = token.span.text(source) else {
                            return false;
                        };
                        actual == spelling
                            || (offset == nickname.len() - 1
                                && actual.strip_suffix("'s") == Some(spelling))
                    })
        })
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

/// Finds the first top-level sentence terminal in `tokens`, respecting
/// [`Nesting`] so punctuation inside quotes/brackets/parentheses is ignored.
fn find_first_top_level_sentence_terminal(tokens: &[Token]) -> Option<usize> {
    let mut depth = Nesting::default();
    for (index, token) in tokens.iter().enumerate() {
        if depth.is_top_level() && is_sentence_terminal(token.kind) {
            return Some(index);
        }
        depth.observe(token.kind);
    }
    None
}

/// Whether an independent clause is a simple copular predication whose
/// complement is a bare nominal headed by [`Noun::Opaque`] — the shape
/// `clause_event` must reject (see its doc comment).
fn copular_complement_head_is_opaque(clause: &IndependentClause) -> bool {
    let IndependentClause::Finite(finite) = clause else {
        return false;
    };
    let crate::syntax::PredicateExpression::Simple(Predicate::Copular(predicate)) =
        finite.predicate()
    else {
        return false;
    };
    let CopularComplement::NounPhrase(noun_phrase) = &predicate.complement else {
        return false;
    };
    let crate::syntax::NounPhraseKind::Nominal(nominal) = noun_phrase.kind() else {
        return false;
    };
    matches!(nominal.head().noun(), Noun::Opaque(_))
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

fn offset_span(span: Span, offset: usize) -> Span {
    Span::new(offset + span.start, offset + span.end)
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

fn valid_level_range(range: &LevelRange) -> bool {
    match range {
        LevelRange::Band { low, high } => low.value <= high.value,
        LevelRange::AtLeast(_) => true,
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

/// Splits the leading keyword→argument separator from an argument's tokens. A
/// leading em dash is the separator (spaced when whitespace surrounds it) and
/// is consumed; otherwise the separator is an implicit space and the whole run
/// is the body. `ability_end` is the byte offset just past the keyword atom.
fn split_keyword_argument_separator(
    argument_tokens: &[Token],
    ability_end: usize,
) -> (KeywordArgumentSeparator, &[Token]) {
    let Some(first) = argument_tokens.first() else {
        return (KeywordArgumentSeparator::Space, argument_tokens);
    };
    if first.kind != TokenKind::Punctuation(Punctuation::EmDash) {
        return (KeywordArgumentSeparator::Space, argument_tokens);
    }
    let spaced = ability_end < first.span.start
        || argument_tokens
            .get(1)
            .is_some_and(|next| first.span.end < next.span.start);
    let separator = if spaced {
        KeywordArgumentSeparator::SpacedEmDash
    } else {
        KeywordArgumentSeparator::EmDash
    };
    (separator, &argument_tokens[1..])
}

/// Whether an argument opens the way a keyword argument does — with a
/// symbol-class token (an integer, an oracle symbol, a symbol sequence, or a
/// power/toughness) or with a `from`/`for` quality-filter preposition. Such a
/// run stays attached to the keyword and recovers, even when the grammar cannot
/// yet structure it, rather than falling to the paragraph path. Both marks are
/// surface facts about the argument, never about the keyword.
fn opens_like_keyword_argument(tokens: &[Token], source: &str) -> bool {
    let Some(first) = tokens.first() else {
        return false;
    };
    matches!(
        first.kind,
        TokenKind::Integer
            | TokenKind::OracleSymbol
            | TokenKind::SymbolSequence
            | TokenKind::PowerToughness
    ) || predicated_preposition(first.span.text(source).unwrap_or_default()).is_some()
}

/// Whether an em-dash argument is a bare pairing label (`Friends forever`)
/// rather than a sentence cost. A label carries none of the punctuation a
/// rules sentence does; the distinction is drawn from the surface alone, never
/// from the keyword.
fn em_dash_keyword_label_is_bare(text: &str) -> bool {
    !text.is_empty() && !text.contains(['.', '!', '?', ':'])
}

/// Peels exactly one leading `onto` or `with` preposition off a Stage B
/// restriction, or leaves the tokens untouched for any other leading word —
/// widening to other prepositions is explicitly not licensed by the shape.
fn peel_restriction_preposition<'tokens>(
    tokens: &'tokens [Token],
    source: &str,
) -> (Option<Preposition>, &'tokens [Token]) {
    let Some(first) = tokens.first() else {
        return (None, tokens);
    };
    let text = first.span.text(source).unwrap_or_default();
    if text.eq_ignore_ascii_case("onto") {
        (Some(Preposition::Onto), &tokens[1..])
    } else if text.eq_ignore_ascii_case("with") {
        (Some(Preposition::With), &tokens[1..])
    } else {
        (None, tokens)
    }
}

/// The `from`/`for` preposition that opens a predicated quality filter, or
/// `None` for any other leading word.
fn predicated_preposition(surface: &str) -> Option<Preposition> {
    if surface.eq_ignore_ascii_case("from") {
        Some(Preposition::From)
    } else if surface.eq_ignore_ascii_case("for") {
        Some(Preposition::For)
    } else {
        None
    }
}

/// A symbol-sequence cost — a run wrapped in braces (`{2}`, `{X}`,
/// `{5}{G}{G}`) parsed into its structured oracle symbols.
fn symbol_cost(text: &str) -> Option<Vec<OracleSymbol>> {
    (text.starts_with('{') && text.len() > 2 && text.ends_with('}'))
        .then(|| parse_symbol_sequence(text))
        .flatten()
}

/// Splits a coordinated predicated argument (`from red and from white`) at each
/// top-level `and` that a repeated preposition follows [CR#702.16g,702.11f]. An
/// `and` inside a single quality (`activated and triggered abilities`) is not a
/// split point because no preposition follows it.
fn split_coordinated_predicates<'a>(tokens: &'a [Token], source: &str) -> Vec<&'a [Token]> {
    let text_of = |token: &Token| token.span.text(source).unwrap_or_default();
    let mut segments = Vec::new();
    let mut start = 0;
    for index in 0..tokens.len() {
        let is_and = text_of(&tokens[index]).eq_ignore_ascii_case("and");
        let next_is_preposition = tokens
            .get(index + 1)
            .is_some_and(|next| predicated_preposition(text_of(next)).is_some());
        if index > start && is_and && next_is_preposition {
            segments.push(&tokens[start..index]);
            start = index + 1;
        }
    }
    segments.push(&tokens[start..]);
    segments
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
    use crate::features::Conjunction;
    use crate::identity::SelfReference;
    use crate::parse::DiagnosticKind;
    use crate::parse::ParseReport;
    use crate::parse::parse_with_catalogs;
    use crate::parse::parse_with_identity;
    use crate::surface::lex;
    use crate::syntax::*;
    use crate::word::ColorWord;
    use crate::word::Noun;

    #[test]
    fn activated_ability_has_cost_components_and_effect_sentences() {
        let report = parse("{1}{R}, {T}, Sacrifice Nissa: Draw a card. If you do, discard a card.");
        let AbilityKind::Activated(ability) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability");
        };
        assert_eq!(ability.cost.components().len(), 3);
        assert_eq!(ability.effect.sentences.len(), 2);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &ability.effect.sentences[1].body
        else {
            panic!("{:#?}", ability.effect.sentences[1].body);
        };
        assert_eq!(
            complex.attachment().position(),
            AttachmentPosition::BeforeMatrix
        );
        assert!(matches!(
            complex.attachment().payload(),
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(Subordinator::If, _))
        ));
    }

    #[test]
    fn typed_multi_component_cost_carries_each_shape() {
        // A three-part activation cost exercises every non-recovered shape: a
        // multi-symbol run, a single-symbol run, and an imperative cost clause.
        let source = "{1}{R}, {T}, Sacrifice a creature: Draw a card.";
        let report = parse(source);
        let AbilityKind::Activated(ability) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability: {:#?}", report.ast.abilities[0]);
        };
        let [mana, tap, sacrifice] = ability.cost.components() else {
            panic!("expected three cost components: {:#?}", ability.cost);
        };
        assert!(matches!(mana, CostComponent::Symbols(symbols) if symbols.len() == 2));
        assert!(matches!(tap, CostComponent::Symbols(symbols) if symbols.len() == 1));
        assert!(matches!(
            sacrifice,
            CostComponent::Clause(clause)
                if matches!(clause.as_ref(), IndependentClause::Finite(finite)
                    if finite.subject().is_none()
                        && matches!(finite.predicate(), PredicateExpression::Simple(_)))
        ));
        assert_eq!(report.ast.render(FIXTURE_NAME, true).unwrap(), source);
    }

    #[test]
    fn symbol_cost_round_trips_through_structured_symbols() {
        // The keyword symbol cost is now carried as `Vec<OracleSymbol>`, never a
        // raw string; it must reproduce its spelling byte-exactly by
        // concatenation.
        let report = parse("Morph {2}{W}");
        let AbilityKind::Keyword(keywords) = report.ast.abilities[0].kind() else {
            panic!("expected keyword ability");
        };
        let KeywordArgument::Costed(KeywordCost::Symbols(symbols)) = &keywords.first().argument
        else {
            panic!("expected a symbol keyword cost: {:#?}", keywords.first());
        };
        assert_eq!(symbols.len(), 2);
        assert_eq!(
            symbols.iter().map(OracleSymbol::as_str).collect::<String>(),
            "{2}{W}"
        );
        assert_eq!(render(&report), "Morph {2}{W}");
    }

    #[test]
    fn unparsed_cost_component_recovers_at_the_activation_cost_role() {
        // A cost component that matches no closed shape recovers verbatim,
        // mirroring `KeywordArgument::Recovered`. It renders back byte-for-byte
        // and is attributed to the activation-cost role, not the clause role.
        let source = "{T}, Frobnicate a creature: Draw a card.";
        let report = parse(source);
        let AbilityKind::Activated(ability) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability: {:#?}", report.ast.abilities[0]);
        };
        let [tap, frobnicate] = ability.cost.components() else {
            panic!("expected two cost components: {:#?}", ability.cost);
        };
        assert!(matches!(tap, CostComponent::Symbols(_)));
        let CostComponent::Recovered(text) = frobnicate else {
            panic!("expected a recovered cost component: {frobnicate:#?}");
        };
        assert_eq!(text.spelling(), "Frobnicate a creature");
        assert_eq!(report.ast.render(FIXTURE_NAME, true).unwrap(), source);
        assert!(report.ast.recoveries().iter().any(|recovery| {
            recovery.role == RecoveryRole::ActivationCost
                && recovery.text == "Frobnicate a creature"
        }));
    }

    #[test]
    fn class_level_ability_has_a_cost_and_numeric_level() {
        let source = "{1}{R}: Level 2";
        let report = parse(source);
        let AbilityKind::ClassLevel(level) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a class level ability: {:#?}",
                report.ast.abilities[0]
            );
        };
        assert_eq!(level.level.value, 2);
        assert_eq!(level.level.numeral, Numeral::Arabic(false));
        assert!(matches!(
            level.cost.components(),
            [CostComponent::Symbols(symbols)] if symbols.len() == 2
        ));
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
    }

    #[test]
    fn trigger_and_ability_word_are_separate_syntax() {
        let report = parse("Landfall — Whenever a land enters under your control, draw a card.");
        let ability = &report.ast.abilities[0];
        assert_eq!(
            ability.header().and_then(|header| match header {
                AbilityHeader::AbilityWord(word) => Some(word.canonical()),
                AbilityHeader::Flavor(_) => None,
            }),
            Some("Landfall")
        );
        let AbilityKind::Triggered(triggered) = ability.kind() else {
            panic!("expected triggered ability");
        };
        assert_eq!(triggered.conditions.first.introducer, TriggerWord::Whenever);
        assert_eq!(
            render(&report),
            "Landfall — Whenever a land enters under your control, draw a card."
        );
    }

    #[test]
    fn condition_in_intervening_position_is_lifted_out_of_the_effect() {
        let report = parse("Whenever Nissa attacks, if you control another creature, draw a card.");
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!("expected triggered ability");
        };
        assert!(matches!(
            triggered.intervening_condition,
            Some(DependentClause::Subordinate(Subordinator::If, _))
        ));
        assert!(matches!(
            triggered.effect.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Finite(ref finite))
                if finite.subject().is_none()
                    && matches!(
                        finite.predicate(),
                        PredicateExpression::Simple(Predicate::Transitive(_))
                    )
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
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
                    kind: Transitive {
                        object: PredicateObject::NounPhrase(noun_phrase),
                        ..
                    },
                    ..
                }),
            }) if matches!(
                noun_phrase.kind(),
                crate::syntax::NounPhraseKind::Coordinated(coordinated)
                    if matches!(
                        coordinated.first().kind(),
                        crate::syntax::NounPhraseKind::Quantity(quantity)
                            if matches!(
                                quantity.kind(),
                                crate::syntax::QuantityKind::Exact(number) if number.value == 1
                            )
                    ) && matches!(
                        coordinated.rest().as_slice(),
                        [NounPhraseCoordination { phrase, .. }]
                            if matches!(
                                phrase.kind(),
                                crate::syntax::NounPhraseKind::Quantity(quantity)
                                    if quantity.kind() == crate::syntax::QuantityKind::Both
                            )
                    )
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
        let AbilityKind::Modal(modal) = &activated.ast.abilities[0].kind() else {
            panic!("expected activated modal");
        };
        assert!(matches!(modal.frame, ModalFrame::Activated(_)));

        let triggered = parse("Whenever Nissa attacks, choose one —\n• Draw a card.\n• Scry 1.");
        let AbilityKind::Modal(modal) = &triggered.ast.abilities[0].kind() else {
            panic!("expected triggered modal");
        };
        assert!(matches!(
            modal.frame,
            ModalFrame::Triggered(TriggerHeader {
                introducer: TriggerWord::Whenever,
                ..
            })
        ));
    }

    #[test]
    fn quoted_granted_ability_does_not_split_its_sentence_or_colon() {
        let report = parse("Create a token with \"{T}: Add {G}.\" Then draw a card.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("quoted colon must not create an activated frame");
        };
        assert_eq!(paragraph.sentences.len(), 2);
    }

    #[test]
    fn postposed_condition_preserves_surface_order() {
        let report = parse("Draw two cards if you control an artifact.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Complex(ref complex))
                    if complex.attachment().position() == AttachmentPosition::AfterMatrix
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
        let AbilityKind::Loyalty(loyalty) = report.ast.abilities[0].kind() else {
            panic!("expected loyalty ability");
        };
        assert_eq!(loyalty.cost.sign, LoyaltyCostSign::Minus);
        assert_eq!(loyalty.cost.value, LoyaltyCostValue::X);
    }

    #[test]
    fn target_determiner_is_subject_when_a_later_predicate_exists() {
        let report = parse("Target creature can't block this turn.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected nominal subject");
        };
        let (Some(Subject(subject)), PredicateExpression::Simple(Predicate::Deontic(_))) =
            (finite.subject(), finite.predicate())
        else {
            panic!("expected a deontic predicate with a nominal subject");
        };
        let crate::syntax::NounPhraseKind::Nominal(subject) = subject.kind() else {
            panic!("expected nominal subject");
        };
        assert_eq!(subject.determiner(), Some(&crate::determiner::target(None)));
    }

    #[test]
    fn shared_subject_predicates_are_coordinated() {
        let report = parse("Other Goblin creatures you control get +1/+1 and have haste.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        assert!(matches!(
            paragraph.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Finite(ref finite))
                if matches!(finite.predicate(), PredicateExpression::Coordinated(_))
        ));
    }

    #[test]
    fn conjunction_inside_a_complement_is_not_a_coordinated_predicate() {
        let report = parse("Destroy target artifact and enchantment.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Finite(ref finite))
                    if finite.subject().is_none()
                        && matches!(
                            finite.predicate(),
                            PredicateExpression::Simple(Predicate::Transitive(_))
                        )
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected coordinated clause, got {:#?}",
                paragraph.sentences[0].body
            );
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!(
                "expected coordinated predicate expression, got {:#?}",
                finite.predicate()
            );
        };
        assert_eq!(coordination.junctions().len(), 3);
        assert!(coordination.junctions().iter().all(|junction| {
            junction.conjunction == Some(Conjunction::Then) && junction.comma.is_present()
        }));
    }

    #[test]
    fn asyndetic_predicate_chains_preserve_the_missing_conjunction() {
        let report = parse(
            "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
        );
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected coordinated clause, got {:#?}",
                paragraph.sentences[0].body
            );
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!(
                "expected coordinated predicate expression, got {:#?}",
                finite.predicate()
            );
        };
        assert_eq!(coordination.junctions().len(), 2);
        assert_eq!(coordination.junctions()[0].conjunction, None);
        assert_eq!(
            coordination.junctions()[1].conjunction,
            Some(Conjunction::Then)
        );
        assert_eq!(
            render(&report),
            "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle."
        );
    }

    #[test]
    fn coordination_preserves_a_new_clause_subject() {
        let report = parse("It becomes a Vehicle, and it gains crew 2.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected transitive clause");
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!("expected a simple transitive predicate");
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::QuotedAbility(quoted)
                if matches!(quoted.ability.kind(), AbilityKind::Triggered(_))
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
            let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
                panic!("{source}: expected paragraph");
            };
            let SentenceBody::Independent(IndependentClause::Finite(finite)) =
                &paragraph.sentences[0].body
            else {
                panic!(
                    "{source}: expected transitive grant clause, got {:?}",
                    paragraph.sentences[0].body
                );
            };
            let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
            else {
                panic!("{source}: expected a simple transitive grant predicate");
            };
            assert!(
                matches!(&predicate.object, PredicateObject::QuotedAbility(_)),
                "{source}: the quoted ability should be the grant verb's object"
            );
            assert_eq!(render(&report), source);
        }
    }

    #[test]
    fn quoted_ability_object_and_with_postmodifier_keep_distinct_typed_slots() {
        // `gains` takes the quote through its typed predicate-object door.
        // The distinct `with "..."` relation has its own typed adjunct and
        // never masquerades as an ordinary prepositional object.
        let object = parse("Target creature gains \"Flying.\"");
        let AbilityKind::Paragraph(object_paragraph) = &object.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &object_paragraph.sentences[0].body
        else {
            panic!("the grant quote should stay a structured object")
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!("the grant quote should stay on a simple transitive predicate")
        };
        let PredicateObject::QuotedAbility(quoted) = &predicate.object else {
            panic!("the grant quote should occupy the quoted-object slot")
        };
        let _ = quoted;
        assert_eq!(render(&object), "Target creature gains \"Flying.\"");

        let with = parse("Create a Goblin creature token with \"{T}: Add {C}.\"");
        let AbilityKind::Paragraph(with_paragraph) = &with.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &with_paragraph.sentences[0].body
        else {
            panic!(
                "the with-postmodifier clause should remain a structured imperative, got {:?}",
                with_paragraph.sentences[0].body
            )
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!("the with-postmodifier should retain a simple transitive predicate")
        };
        assert!(finite.subject().is_none());
        assert!(predicate.elements().iter().any(|element| matches!(
            element,
            PredicateElement::Adjunct(PredicateAdjunct::AbilityPostmodifier(_))
        )));
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        assert!(
            matches!(
                &paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Finite(finite))
                    if matches!(
                        finite.predicate(),
                        PredicateExpression::Simple(Predicate::Transitive(predicate))
                            if matches!(predicate.object, PredicateObject::QuotedAbility(_))
                    )
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
    fn quoted_name_exposes_existing_clause_recovery_residue() {
        // Negative armor: a quoted string after `named` is a name, not a
        // granted ability. The grant-object slot fires only after a grant verb,
        // so the existing `named` machinery is untouched — no quoted-ability
        // node appears and the sentence round-trips exactly.
        let source = "Create a token named \"A. B\" and draw a card.";
        let report = parse(source);
        let [ability] = report.ast.abilities.as_slice() else {
            panic!("expected one ability: {:#?}", report.ast)
        };
        assert!(
            matches!(
                ability.kind(),
                AbilityKind::Paragraph(Paragraph { sentences, .. })
                    if matches!(sentences.as_slice(), [Sentence {
                    body: SentenceBody::Recovered(_),
                    ..
                }])
            ),
            "existing residue: the quoted-name clause currently recovers instead of \
             lowering a name object: {:#?}",
            report.ast
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn keyword_argument_can_contain_an_embedded_activated_ability() {
        let report = parse("Power-up — {W}{U}{B}{R}{G}: Put a +1/+1 counter on this creature.");
        let AbilityKind::Keyword(keywords) = report.ast.abilities[0].kind() else {
            panic!("expected keyword ability");
        };
        assert!(matches!(
            &keywords.first().argument,
            KeywordArgument::Costed(KeywordCost::Sentence { .. })
        ));
    }

    #[test]
    fn contiguous_symbol_keyword_argument_is_a_cost() {
        let report = parse("Morph {2}{W}");
        let AbilityKind::Keyword(keywords) = report.ast.abilities[0].kind() else {
            panic!("expected keyword ability");
        };
        assert!(matches!(
            &keywords.first().argument,
            KeywordArgument::Costed(KeywordCost::Symbols(symbols))
                if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{2}{W}"
        ));
        assert_eq!(render(&report), "Morph {2}{W}");
    }

    #[test]
    fn punctuation_inside_a_mid_sentence_quote_stays_nested() {
        let report = parse("Create a token named \"A. B\" and draw a card.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
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
        let AbilityKind::Keyword(list) = &keywords.ast.abilities[0].kind() else {
            panic!("expected keyword list");
        };
        assert_eq!(list.len(), 3);
        assert_eq!(list.first().ability.canonical(), "Flying");
        assert_eq!(
            list.get(1).expect("a second keyword").ability.canonical(),
            "First strike"
        );
        assert_eq!(
            list.get(2).expect("a third keyword").ability.canonical(),
            "Protection"
        );

        let action = parse("Manifest dread 2.");
        let AbilityKind::Paragraph(paragraph) = &action.ast.abilities[0].kind() else {
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
                .header()
                .and_then(|header| match header {
                    AbilityHeader::AbilityWord(word) => Some(word.canonical()),
                    AbilityHeader::Flavor(_) => None,
                }),
            Some("Void")
        );
    }

    #[test]
    fn exact_catalog_terms_are_recognized_outside_ability_position() {
        let report = parse("Other Goblin creatures you control get +1/+1 and have haste.");
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected coordination");
        };
        let (Some(Subject(subject)), PredicateExpression::Coordinated(_)) =
            (finite.subject(), finite.predicate())
        else {
            panic!("expected a coordinated predicate with an explicit subject");
        };
        let crate::syntax::NounPhraseKind::Nominal(subject) = subject.kind() else {
            panic!("expected nominal subject");
        };
        assert!(matches!(
            subject.modifiers(),
            [NominalModifier::Adjective { .. }, NominalModifier::Noun { noun, .. }]
                if matches!(
                    noun.kind(),
                    crate::word::NounInstanceKind::Singular(crate::word::Noun::Catalog(goblin))
                        if goblin.kind == CatalogKind::CreatureType
                )
        ));
    }

    #[test]
    fn affinity_accepts_a_for_prepositional_argument() {
        let catalogs = fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Affinity"]);
        let report = parse_with_catalogs("Affinity for artifacts", &catalogs);
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast.abilities[0]);
        };
        assert_eq!(list.len(), 1);
        assert!(matches!(
            list.first(),
            KeywordAbility {
                ability,
                argument: KeywordArgument::Predicated(predicated),
                ..
            } if ability.canonical() == "Affinity"
                && matches!(predicated.qualities.as_slice(), [quality]
                    if quality.preposition == Some(Preposition::For))
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
            report.ast.abilities[0].kind(),
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
        let AbilityKind::Paragraph(aang_static) = &aang.ast.abilities[0].kind() else {
            panic!("expected Aang's first ability to be a paragraph");
        };
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &aang_static.sentences[0].body
        else {
            panic!("{:#?}", aang_static.sentences[0].body);
        };
        assert_eq!(
            complex.attachment().position(),
            AttachmentPosition::AfterMatrix
        );
        let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            Subordinator::AsLongAs,
            SubordinateBody::Finite(condition),
        )) = complex.attachment().payload()
        else {
            panic!("expected a trailing as-long-as attachment: {complex:#?}");
        };
        assert!(matches!(
            condition.as_ref(),
            IndependentClause::Existential(_)
        ));
        assert_eq!(
            aang.ast.render("Aang, A Lot to Learn", true).unwrap(),
            aang_source,
        );

        let keeper_source = "When this creature enters, you become the monarch.\nAt the beginning of your upkeep, if you're the monarch, creatures you control can't be blocked this turn.";
        let keeper = parse_with_catalogs(keeper_source, &catalogs);
        let AbilityKind::Triggered(keeper_upkeep) = &keeper.ast.abilities[1].kind() else {
            panic!("expected Keeper of Keys' second ability to be triggered");
        };
        assert!(matches!(
            keeper_upkeep.conditions.first.event,
            TriggerEvent::Temporal(_)
        ));
        assert!(matches!(
            keeper_upkeep.intervening_condition,
            Some(DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Finite(ref condition),
            )) if matches!(condition.as_ref(), IndependentClause::Finite(finite)
                if matches!(
                    finite.predicate(),
                    PredicateExpression::Simple(Predicate::Copular(_))
                ))
        ));
        assert!(
            matches!(
                keeper_upkeep.effect.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Finite(ref finite))
                    if matches!(
                        finite.predicate(),
                        PredicateExpression::Simple(Predicate::Deontic(deontic))
                            if matches!(
                                deontic.inner(),
                                Some(PredicateExpression::Simple(Predicate::Passive(_)))
                            )
                    )
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
        let AbilityKind::Triggered(justice_trigger) = &justice.ast.abilities[0].kind() else {
            panic!("expected Karmic Justice to be triggered");
        };
        assert!(matches!(
            justice_trigger.conditions.first.event,
            TriggerEvent::Clause(IndependentClause::Finite(ref finite))
                if matches!(
                    finite.predicate(),
                    PredicateExpression::Simple(Predicate::Transitive(_))
                )
        ));
        assert!(
            matches!(
                justice_trigger.effect.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Finite(ref finite))
                    if matches!(
                        finite.predicate(),
                        PredicateExpression::Simple(Predicate::Deontic(deontic))
                            if matches!(
                                deontic.inner(),
                                Some(PredicateExpression::Simple(Predicate::Transitive(_)))
                            )
                    )
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!(
                "expected paragraph ability, got {:#?}",
                report.ast.abilities[0].kind()
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
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
        let AbilityKind::Chapter(chapter) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a single-chapter ability, got {:#?}",
                report.ast.abilities[0].kind()
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
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
            ability.header().and_then(|header| match header {
                AbilityHeader::AbilityWord(word) => Some(word.canonical()),
                AbilityHeader::Flavor(_) => None,
            }),
            Some("Landfall")
        );
        let AbilityKind::Paragraph(paragraph) = ability.kind() else {
            panic!("expected paragraph body under the ability word");
        };
        assert_eq!(paragraph.flavor_header, None);
    }

    fn chapter_values(report: &ParseReport) -> Vec<i32> {
        let AbilityKind::Chapter(chapter) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a chapter ability, got {:#?}",
                report.ast.abilities[0].kind()
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
        let AbilityKind::Chapter(chapter) = report.ast.abilities[0].kind() else {
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
            report.ast.abilities[0].kind(),
            AbilityKind::Paragraph(_)
        ));
    }

    #[test]
    fn a_header_group_that_is_not_a_bare_roman_numeral_is_not_a_chapter() {
        // "III" is Roman, but the "and III" group holds a word, so the header is
        // not a chapter list and the line falls through to the paragraph path.
        let report = parse("I, and III — Draw a card.");
        assert!(
            !matches!(report.ast.abilities[0].kind(), AbilityKind::Chapter(_)),
            "a non-Roman header group must not parse as a chapter: {:#?}",
            report.ast.abilities[0].kind()
        );
    }

    #[test]
    fn modal_choice_header_is_not_eaten_as_a_flavor_header() {
        let report = parse("Choose one —\n• Draw a card.\n• Draw two cards.");
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a modal ability: {:#?}",
                report.ast.abilities[0].kind()
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
                kind: Transitive {
                    object: PredicateObject::NounPhrase(noun_phrase),
                    ..
                },
                ..
            }) if matches!(
                noun_phrase.kind(),
                crate::syntax::NounPhraseKind::Quantity(quantity)
                    if matches!(
                        quantity.kind(),
                        crate::syntax::QuantityKind::Exact(number) if number.value == 1
                    )
            )
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a modal ability: {:#?}",
                report.ast.abilities[0].kind()
            );
        };
        let ModalFrame::Triggered(trigger) = &modal.frame else {
            panic!("expected a triggered modal frame: {:#?}", modal.frame);
        };
        assert_eq!(trigger.introducer, TriggerWord::Whenever);
        assert!(matches!(
            &trigger.event,
            TriggerEvent::Clause(IndependentClause::Finite(finite))
                if matches!(finite.predicate(), PredicateExpression::Coordinated(_))
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!("expected a modal ability");
        };
        assert!(matches!(
            &modal.header.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Finite(finite))
                if finite.subject().is_none()
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!("expected a modal ability");
        };
        assert!(matches!(modal.frame, ModalFrame::Triggered(_)));
        let choice = modal_header_choice(&report);
        assert!(choice.trigger_prefix.is_none());
        assert!(matches!(
            &choice.imperative,
            Predicate::Transitive(TransitivePredicate {
                kind: Transitive {
                    object: PredicateObject::NounPhrase(noun_phrase),
                    ..
                },
                ..
            }) if matches!(
                noun_phrase.kind(),
                crate::syntax::NounPhraseKind::Coordinated(_)
            )
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
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
            matches!(report.ast.abilities[0].kind(), AbilityKind::Triggered(_)),
            "a non-choice When-clause stays a triggered ability: {:#?}",
            report.ast.abilities[0].kind()
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
        let AbilityKind::RollRow(row) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a roll-row ability, got {:#?}",
                report.ast.abilities[0].kind()
            );
        };
        row
    }

    fn level_band(report: &ParseReport) -> &LevelBandAbility {
        let AbilityKind::LevelBand(band) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a level-band ability, got {:#?}",
                report.ast.abilities[0].kind()
            );
        };
        band
    }

    fn station_threshold(report: &ParseReport, index: usize) -> &StationThresholdAbility {
        let AbilityKind::StationThreshold(row) = &report.ast.abilities[index].kind() else {
            panic!(
                "expected a station threshold: {:#?}",
                report.ast.abilities[index]
            );
        };
        row
    }

    #[test]
    fn a_station_threshold_lowers_its_keyword_list_body() {
        let source = "Station\n8+ | Flying, trample";
        let report = parse(source);
        assert_eq!(report.ast.abilities.len(), 2);
        let row = station_threshold(&report, 1);
        assert_eq!(row.threshold, arabic(8));
        let AbilityKind::Keyword(list) = row.ability.kind() else {
            panic!("expected a keyword list body: {:#?}", row.ability.kind());
        };
        assert_eq!(list.len(), 2);
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_station_threshold_lowers_its_activated_body() {
        let source =
            "Station\n12+ | {3}{W}, {T}: Create a Treasure token. Activate only as a sorcery.";
        let report = parse(source);
        let row = station_threshold(&report, 1);
        assert_eq!(row.threshold, arabic(12));
        let AbilityKind::Activated(activated) = row.ability.kind() else {
            panic!("expected an activated body: {:#?}", row.ability.kind());
        };
        let symbol_components = activated
            .cost
            .components()
            .iter()
            .filter(|component| matches!(component, CostComponent::Symbols(_)))
            .count();
        assert_eq!(symbol_components, 2);
        assert!(
            report.ast.recoveries().is_empty(),
            "AST:\n{:#?}",
            report.ast
        );
    }

    #[test]
    fn a_station_threshold_lowers_its_triggered_body() {
        let source = "Station\n10+ | Whenever you attack, draw a card.";
        let report = parse(source);
        let row = station_threshold(&report, 1);
        assert_eq!(row.threshold, arabic(10));
        assert!(matches!(row.ability.kind(), AbilityKind::Triggered(_)));
    }

    #[test]
    fn a_station_threshold_lowers_its_static_body() {
        let source = "Station\n2+ | Other creatures you control get +1/+1.";
        let report = parse(source);
        let row = station_threshold(&report, 1);
        assert_eq!(row.threshold, arabic(2));
        let AbilityKind::Paragraph(paragraph) = row.ability.kind() else {
            panic!("expected a paragraph body: {:#?}", row.ability.kind());
        };
        assert!(matches!(
            paragraph.sentences[0].body,
            SentenceBody::Independent(_)
        ));
    }

    #[test]
    fn two_station_thresholds_stay_separate_line_local_abilities() {
        let source = "Station\n1+ | Whenever an opponent discards a card, they lose 3 life.\n8+ | Flying, deathtouch\nWhenever this permanent attacks, draw a card.";
        let report = parse(source);
        assert_eq!(report.ast.abilities.len(), 4);
        assert!(matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::Keyword(_)
        ));
        assert!(matches!(
            report.ast.abilities[1].kind(),
            AbilityKind::StationThreshold(_)
        ));
        assert!(matches!(
            report.ast.abilities[2].kind(),
            AbilityKind::StationThreshold(_)
        ));
        assert!(matches!(
            report.ast.abilities[3].kind(),
            AbilityKind::Triggered(_)
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_roll_table_plus_row_is_not_a_station_threshold() {
        let source = "Roll a d20.\n1—14 | Draw a card.\n15+ | Draw two cards.";
        let report = parse(source);
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::StationThreshold(_)));
        }
        let AbilityKind::RollRow(first) = &report.ast.abilities[1].kind() else {
            panic!(
                "expected a roll-row ability, got {:#?}",
                report.ast.abilities[1].kind()
            );
        };
        assert_eq!(
            first.range,
            RollRange::Inclusive {
                low: arabic(1),
                high: arabic(14)
            }
        );
        let AbilityKind::RollRow(second) = &report.ast.abilities[2].kind() else {
            panic!(
                "expected a roll-row ability, got {:#?}",
                report.ast.abilities[2].kind()
            );
        };
        assert_eq!(second.range, RollRange::OrMore(arabic(15)));
    }

    #[test]
    fn a_station_card_does_not_claim_a_non_plus_pipe_row() {
        let source = "Station\n20 | Draw a card.";
        let report = parse(source);
        let AbilityKind::RollRow(row) = &report.ast.abilities[1].kind() else {
            panic!(
                "expected a roll-row ability, got {:#?}",
                report.ast.abilities[1].kind()
            );
        };
        assert_eq!(row.range, RollRange::Single(arabic(20)));
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::StationThreshold(_)));
        }
    }

    #[test]
    fn a_plus_pipe_row_without_a_station_keyword_is_not_a_station_threshold() {
        let source = "15+ | Draw a card.";
        let report = parse(source);
        assert!(matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::RollRow(_)
        ));
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::StationThreshold(_)));
        }
    }

    #[test]
    fn a_card_named_station_does_not_license_station_thresholds() {
        let source = "When Infinite Guideline Station enters, draw a card.\n15+ | Draw two cards.";
        let report = parse_with_identity(
            source,
            &fixture_catalogs(),
            "Infinite Guideline Station",
            false,
        );
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::StationThreshold(_)));
        }
    }

    #[test]
    fn an_ordinary_activated_ability_keeps_its_cost_frame() {
        let source = "Station\n{3}{W}, {T}: Create a Treasure token.";
        let report = parse(source);
        let AbilityKind::Activated(activated) = &report.ast.abilities[1].kind() else {
            panic!(
                "expected an activated ability, got {:#?}",
                report.ast.abilities[1].kind()
            );
        };
        let symbol_components = activated
            .cost
            .components()
            .iter()
            .filter(|component| matches!(component, CostComponent::Symbols(_)))
            .count();
        assert_eq!(symbol_components, 2);
    }

    #[test]
    fn hyphen_level_band_lowers_to_a_level_band_ability() {
        let source = "LEVEL 1-3\n4/4";
        let report = parse(source);
        let band = level_band(&report);
        assert_eq!(
            band.range,
            LevelRange::Band {
                low: arabic(1),
                high: arabic(3)
            }
        );
        assert_eq!(
            band.stats,
            PowerToughness {
                power: SignedScalar {
                    sign: ScalarSign::None,
                    value: ScalarValue::Integer(4)
                },
                toughness: SignedScalar {
                    sign: ScalarSign::None,
                    value: ScalarValue::Integer(4)
                },
            }
        );
        assert!(band.abilities.is_empty());
        assert_eq!(render(&report), source);
    }

    #[test]
    fn plus_level_band_lowers_to_a_level_band_ability() {
        let source = "LEVEL 4+\n6/6\nTrample";
        let report = parse(source);
        let band = level_band(&report);
        assert_eq!(band.range, LevelRange::AtLeast(arabic(4)));
        assert_eq!(band.abilities.len(), 1);
        assert!(matches!(band.abilities[0].kind(), AbilityKind::Keyword(_)));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_level_band_binds_every_line_until_the_next_band() {
        let source = "Level up {R}\nLEVEL 4-7\n4/4\nFlying\nLEVEL 8+\n8/8\nFlying, trample\n{R}: This creature gets +1/+0 until end of turn.";
        let report = parse(source);
        assert_eq!(report.ast.abilities.len(), 3);
        assert!(matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::Keyword(_)
        ));
        let AbilityKind::LevelBand(first) = &report.ast.abilities[1].kind() else {
            panic!(
                "expected a level-band ability, got {:#?}",
                report.ast.abilities[1].kind()
            );
        };
        assert_eq!(first.abilities.len(), 1);
        let AbilityKind::LevelBand(second) = &report.ast.abilities[2].kind() else {
            panic!(
                "expected a level-band ability, got {:#?}",
                report.ast.abilities[2].kind()
            );
        };
        assert_eq!(second.abilities.len(), 2);
        assert!(matches!(
            second.abilities[1].kind(),
            AbilityKind::Activated(_)
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_level_header_without_a_stat_line_is_not_a_band() {
        let source = "LEVEL 1-3";
        let report = parse(source);
        assert!(!matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::LevelBand(_)
        ));
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a paragraph ability, got {:#?}",
                report.ast.abilities[0].kind()
            );
        };
        assert!(matches!(
            paragraph.sentences[0].body,
            SentenceBody::Recovered(_)
        ));
    }

    #[test]
    fn a_bare_stat_line_without_a_level_header_is_not_a_band() {
        let source = "Flying\n4/4";
        let report = parse(source);
        assert_eq!(report.ast.abilities.len(), 2);
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::LevelBand(_)));
        }
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[1].kind() else {
            panic!(
                "expected a paragraph ability, got {:#?}",
                report.ast.abilities[1].kind()
            );
        };
        assert!(matches!(
            paragraph.sentences[0].body,
            SentenceBody::Recovered(ref text) if text.spelling() == "4/4"
        ));
    }

    #[test]
    fn a_mixed_case_level_line_is_not_a_band() {
        let source = "Level 1-3\n4/4";
        let report = parse(source);
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::LevelBand(_)));
        }
        assert_eq!(render(&report), source);
    }

    #[test]
    fn a_class_level_bar_is_not_a_level_band() {
        let source = "{1}{R}: Level 2\nCreatures you control have haste.";
        let report = parse(source);
        assert!(matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::ClassLevel(_)
        ));
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::LevelBand(_)));
        }
    }

    #[test]
    fn a_period_terminated_stat_line_is_not_a_band_stat() {
        let source = "LEVEL 1-3\n4/4.";
        let report = parse(source);
        for ability in &report.ast.abilities {
            assert!(!matches!(ability.kind(), AbilityKind::LevelBand(_)));
        }
    }

    #[test]
    fn a_roll_row_is_not_a_level_band() {
        let source = "Roll a d20.\n20 | Draw a card.";
        let report = parse(source);
        assert_eq!(report.ast.abilities.len(), 2);
        let AbilityKind::RollRow(row) = &report.ast.abilities[1].kind() else {
            panic!(
                "expected a roll-row ability, got {:#?}",
                report.ast.abilities[1].kind()
            );
        };
        assert_eq!(row.range, RollRange::Single(arabic(20)));
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
            matches!(report.ast.abilities[0].kind(), AbilityKind::Activated(_)),
            "the instruction line stays an activated ability: {:#?}",
            report.ast.abilities[0].kind()
        );
        let ranges: Vec<RollRange> = report.ast.abilities[1..]
            .iter()
            .map(|ability| {
                let AbilityKind::RollRow(row) = ability.kind() else {
                    panic!("expected a roll-row ability, got {:#?}", ability.kind());
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
            !matches!(report.ast.abilities[0].kind(), AbilityKind::RollRow(_)),
            "an unspaced range with no pipe must not parse as a roll row: {:#?}",
            report.ast.abilities[0].kind()
        );
    }

    #[test]
    fn mid_rules_em_dash_after_a_word_is_not_a_flavor_header() {
        // The em dash follows a word ("choice"), so it is a mid-rules construction
        // (a villainous choice), never a flavor header. Its span stays verbatim.
        let source = "Each opponent faces a villainous choice — You draw a card, or that player discards a card.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
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
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
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

    fn parse_with_activation_for_test(
        source: &str,
        activation: crate::grammar::GeneratedActivation,
    ) -> super::AbilityParse {
        let surface = lex(source);
        let identity = SelfReference::new(FIXTURE_NAME, true);
        let tokens =
            crate::surface::collapse_full_names(source, surface.tokens, identity.full_name());
        super::Parser::new_with_activation(
            source,
            &fixture_catalogs(),
            &identity,
            false,
            activation,
        )
        .parse(&tokens)
    }

    #[test]
    fn multiline_and_nested_ability_roots_are_registration_order_neutral() {
        let activations = [
            crate::grammar::GeneratedActivation::Groups(crate::constructions::ALL_GROUPS),
            crate::grammar::GeneratedActivation::GroupsReversed(crate::constructions::ALL_GROUPS),
            crate::grammar::GeneratedActivation::GroupsFixedShuffle(
                crate::constructions::ALL_GROUPS,
            ),
        ];
        for (source, expected_ordinals) in [
            ("LEVEL 1-3\n4/4\nFlying", vec![9, 4]),
            ("Station\n8+ | Flying", vec![9, 9, 5]),
            ("Choose one —\n• Draw a card.\n• Gain 1 life.", vec![8]),
            (
                "Target creature gains \"Whenever this creature attacks, draw a card.\"",
                vec![6, 10],
            ),
        ] {
            let reports =
                activations.map(|activation| parse_with_activation_for_test(source, activation));
            assert_eq!(reports[0].ast, reports[1].ast, "{source}");
            assert_eq!(reports[0].ast, reports[2].ast, "{source}");
            assert_eq!(reports[0].diagnostics, reports[1].diagnostics, "{source}");
            assert_eq!(reports[0].diagnostics, reports[2].diagnostics, "{source}");
            assert_eq!(reports[0].selections, reports[1].selections, "{source}");
            assert_eq!(reports[0].selections, reports[2].selections, "{source}");
            let ordinals = reports[0]
                .selections
                .iter()
                .flat_map(|selection| &selection.constructions)
                .filter(|decision| decision.selected().as_str() == "ability")
                .map(crate::ConstructionDecision::selected_production_ordinal)
                .collect::<Vec<_>>();
            assert_eq!(ordinals, expected_ordinals, "{source}");
        }

        let source = "Ward—Discard a card: Draw a card.";
        let surface = lex(source);
        let identity = SelfReference::new(FIXTURE_NAME, true);
        let tokens =
            crate::surface::collapse_full_names(source, surface.tokens, identity.full_name());
        let catalogs = shape_catalogs();
        let reports = activations.map(|activation| {
            super::Parser::new_with_activation(source, &catalogs, &identity, false, activation)
                .parse(&tokens)
        });
        assert_eq!(reports[0].ast, reports[1].ast);
        assert_eq!(reports[0].ast, reports[2].ast);
        assert_eq!(reports[0].diagnostics, reports[1].diagnostics);
        assert_eq!(reports[0].diagnostics, reports[2].diagnostics);
        assert_eq!(reports[0].selections, reports[1].selections);
        assert_eq!(reports[0].selections, reports[2].selections);
        let decision = reports[0]
            .selections
            .iter()
            .flat_map(|selection| &selection.constructions)
            .find(|decision| decision.selected().as_str() == "ability")
            .expect("collision records an ability-root decision");
        assert_eq!(decision.selected_production_ordinal(), 9);
        assert_eq!(
            decision.reason(),
            crate::SelectionReason::Cost(crate::ParseCostDimension::Precedence)
        );
        assert_eq!(
            decision
                .alternatives()
                .iter()
                .map(|alternative| (
                    alternative.production_ordinal(),
                    alternative.cost().precedence(),
                ))
                .collect::<Vec<_>>(),
            [(0, 4), (9, 0)]
        );
    }

    #[test]
    fn strive_distributive_target_beyond_ordinal_is_typed_and_order_neutral() {
        // The canonical Strive rider uses one closed distributive NP as the
        // object of `for`, not a general `beyond` preposition or a headless
        // ordinal phrase.
        let source = "Strive — This spell costs {1} more to cast for each target beyond the first.";
        let report = parse(source);
        assert!(report.ast.recoveries().is_empty(), "{source}");
        assert!(report.ast.lexical_opacity().is_empty(), "{source}");
        assert!(
            report
                .ast
                .noun_phrases()
                .iter()
                .any(|phrase| matches!(phrase.kind(), NounPhraseKind::TargetsBeyondFirst))
        );
        assert!(
            report
                .provenance
                .selections
                .iter()
                .flat_map(|selection| &selection.constructions)
                .any(|decision| decision.selected().as_str() == "noun_phrase_targets_beyond_first")
        );
        assert_eq!(render(&report), source);

        let activations = [
            crate::grammar::GeneratedActivation::Groups(crate::constructions::ALL_GROUPS),
            crate::grammar::GeneratedActivation::GroupsReversed(crate::constructions::ALL_GROUPS),
            crate::grammar::GeneratedActivation::GroupsFixedShuffle(
                crate::constructions::ALL_GROUPS,
            ),
        ];
        let reports =
            activations.map(|activation| parse_with_activation_for_test(source, activation));
        assert_eq!(reports[0].ast, reports[1].ast);
        assert_eq!(reports[0].ast, reports[2].ast);
        assert_eq!(reports[0].diagnostics, reports[1].diagnostics);
        assert_eq!(reports[0].diagnostics, reports[2].diagnostics);
        assert_eq!(reports[0].selections, reports[1].selections);
        assert_eq!(reports[0].selections, reports[2].selections);
        for ordered in reports {
            assert!(
                ordered
                    .ast
                    .noun_phrases()
                    .iter()
                    .any(|phrase| matches!(phrase.kind(), NounPhraseKind::TargetsBeyondFirst))
            );
        }
    }

    #[test]
    fn strive_distributive_target_rejects_deceptive_opaque_ordinal_lookalikes() {
        let opaque = parse("This spell costs {1} more to cast for each target beyond first.");
        assert!(
            opaque
                .ast
                .noun_phrases()
                .iter()
                .all(|phrase| !matches!(phrase.kind(), NounPhraseKind::TargetsBeyondFirst))
        );
        assert!(
            opaque
                .provenance
                .selections
                .iter()
                .flat_map(|selection| &selection.constructions)
                .all(|decision| decision.selected().as_str() != "noun_phrase_targets_beyond_first")
        );

        let wrong_ordinal =
            parse("This spell costs {1} more to cast for each target beyond the second.");
        assert!(
            wrong_ordinal
                .ast
                .noun_phrases()
                .iter()
                .all(|phrase| !matches!(phrase.kind(), NounPhraseKind::TargetsBeyondFirst))
        );
        assert!(
            wrong_ordinal
                .provenance
                .selections
                .iter()
                .flat_map(|selection| &selection.constructions)
                .all(|decision| decision.selected().as_str() != "noun_phrase_targets_beyond_first")
        );
        assert!(!wrong_ordinal.ast.recoveries().is_empty());
    }

    #[test]
    fn keyword_cost_and_colon_frame_collision_uses_the_declared_guard_rank() {
        let source = "Ward—Discard a card: Draw a card.";
        let catalogs = shape_catalogs();
        let identity = SelfReference::new(FIXTURE_NAME, true);
        let surface = lex(source);
        let tokens =
            crate::surface::collapse_full_names(source, surface.tokens, identity.full_name());
        let mut parser = super::Parser::new_with_activation(
            source,
            &catalogs,
            &identity,
            false,
            crate::grammar::GeneratedActivation::Production,
        );
        let candidates = super::AbilityFrameCandidate::ALL
            .into_iter()
            .filter(|candidate| {
                parser
                    .probe_ability_candidate(*candidate, &tokens, None)
                    .is_some()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            candidates,
            [
                super::AbilityFrameCandidate::Keyword,
                super::AbilityFrameCandidate::Activated,
            ]
        );

        let ability = parser.parse_ability(&tokens);
        assert!(matches!(ability.kind(), AbilityKind::Keyword(_)));
    }

    #[test]
    fn equal_ability_guard_ranks_are_rejected_explicitly() {
        let candidate = |frame| super::ParsedAbilityCandidate {
            frame,
            ability: crate::constructions::ability::build_ability_root(
                None,
                AbilityKind::Paragraph(Paragraph {
                    flavor_header: None,
                    sentences: vec![Sentence::from_body(SentenceBody::Recovered(
                        RecoveredText::new("test", 1),
                    ))],
                }),
            )
            .expect("the test candidate is a checked ability"),
            diagnostics: Vec::new(),
            selections: Vec::new(),
        };
        let candidates = [
            candidate(super::AbilityFrameCandidate::Keyword),
            candidate(super::AbilityFrameCandidate::Keyword),
        ];
        assert!(matches!(
            super::select_best_ability_candidate(&candidates),
            Err(super::AbilityFrameSelectionError::EqualGuardRank { rank: 0 })
        ));
    }

    /// A catalog exercising every keyword-argument shape. The keyword names are
    /// arbitrary here — the grammar consults no per-keyword facts, so any name
    /// paired with any parsing surface exercises the same shape.
    fn shape_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(
                CatalogKind::KeywordAbility,
                [
                    "Flying",
                    "First strike",
                    "Reach",
                    "Trample",
                    "Fabricate",
                    "Ward",
                    "Escape",
                    "Flashback",
                    "Equip",
                    "Cumulative upkeep",
                    "Recover",
                    "Suspend",
                    "Prototype",
                    "Partner",
                    "Gift",
                    "Champion",
                    "Protection",
                    "Hexproof from",
                    "Affinity",
                    "Rampage",
                    "Craft",
                    "Splice",
                    "Reinforce",
                ],
            )
            .with_catalog(
                CatalogKind::CardType,
                ["Creature", "Artifact", "Instant", "Sorcery"],
            )
            .with_catalog(CatalogKind::SpellType, ["Arcane"])
            .with_catalog(
                CatalogKind::CreatureType,
                ["Dinosaur", "Merfolk", "Pirate", "Vampire"],
            )
            .with_catalog(CatalogKind::LandType, ["Mountain"])
    }

    fn shape_argument(source: &str) -> KeywordArgument {
        let report = parse_with_catalogs(source, &shape_catalogs());
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a keyword ability for {source:?}: {:#?}",
                report.ast
            );
        };
        assert_eq!(list.len(), 1, "expected one keyword for {source:?}");
        assert_eq!(
            report.ast.render("Test Card", false).unwrap(),
            source,
            "shape argument must round-trip"
        );
        list.first().argument.clone()
    }

    #[test]
    fn cumulative_upkeep_is_a_structured_cost_not_a_legacy_embedded_ability() {
        // Existing-user tree gate: this exact surface parsed before Stage A
        // too (no comma in its body), via the legacy embedded-`Ability`
        // `Sentence` carrier. Stage A retypes it to the structured
        // `Components` cost like every other tight-dash cost.
        assert!(matches!(
            shape_argument("Cumulative upkeep—Put a -1/-1 counter on this creature."),
            KeywordArgument::Costed(KeywordCost::Components { terminal: true, .. })
        ));
    }

    #[test]
    fn stage_a_leaves_the_untouched_shapes_unchanged() {
        // `Partner—Friends forever`: a bare em-dash pairing label, still
        // caught by `parse_named_keyword_argument` before the tight-cost
        // branch is ever reached.
        assert!(matches!(
            shape_argument("Partner—Friends forever"),
            KeywordArgument::Named { ref label, .. } if label == "Friends forever"
        ));
        // `Prototype {1}{U}{U} — 2/1`: internal *spaced* em dash, handled by
        // `parse_statted` in `parse_space_argument`, never reaches the
        // tight-cost splitter/shaper at all (its separator is `Space`, not
        // `EmDash`).
        assert!(matches!(
            shape_argument("Prototype {1}{U}{U} — 2/1"),
            KeywordArgument::Statted { .. }
        ));
        // `Equip {3}` and `Ward {2}`: plain space-separated symbol costs.
        assert!(matches!(
            shape_argument("Equip {3}"),
            KeywordArgument::Costed(KeywordCost::Symbols(ref symbols))
                if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{3}"
        ));
        assert!(matches!(
            shape_argument("Ward {2}"),
            KeywordArgument::Costed(KeywordCost::Symbols(ref symbols))
                if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{2}"
        ));
        // `Suspend 4—{U}`: `CountedCost`, an entirely different internal-dash
        // shape (`count—symbols`), parsed in `parse_space_argument` before
        // the keyword-argument-separator tight/spaced split ever applies.
        assert!(matches!(
            shape_argument("Suspend 4—{U}"),
            KeywordArgument::CountedCost { .. }
        ));
    }

    #[test]
    fn quoted_symbol_keyword_argument_is_unaffected_by_stage_a() {
        // The kwterm quoted-symbol case: a space-separated symbol argument
        // quoted inside a granting clause, its own terminal kept outside the
        // quote. No tight em dash is involved, so Stage A cannot touch it;
        // this is the existing `quoted_symbol_keyword_argument_keeps_
        // terminal_outside_typed_cost` surface re-run after Stage A's
        // changes to confirm it is untouched.
        let source = "Target creature gains \"Ward {1}.\"";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
    }

    #[test]
    fn a_spaced_dash_designation_row_is_unchanged_by_the_tight_cost_split() {
        // `Exhaust — {2}{G}{G}: …` is a designation-headed activated ability,
        // never a keyword cost: `SpacedEmDash` still takes the legacy path
        // byte-for-byte and the tight-cost splitter/shaper are never
        // consulted for it, per the corpus-verified 124-row spaced-dash
        // population (only 18 of which were unresolved and targeted here).
        let source = "Exhaust — {2}{G}{G}: Put two +1/+1 counters on this creature.";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert!(
            !matches!(report.ast.abilities[0].kind(), AbilityKind::Keyword(_)),
            "a spaced-dash designation header must not become a keyword ability: {:#?}",
            report.ast
        );
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
    }

    #[test]
    fn a_recovered_tight_cost_component_attributes_to_keyword_argument_not_activation_cost() {
        // Walker fixture: a residue inside a Stage A structured cost must
        // recover at the keyword-argument role, never activation cost —
        // proving `inner = context.or(Some(RecoveryRole::KeywordArgument))`
        // is honored for the new `Components` walker arm exactly as it was
        // for the legacy `Sentence` arm.
        let source = "Recover—Pay half your life, rounded up.";
        let report = parse_with_catalogs(source, &shape_catalogs());
        let recoveries = report.ast.recoveries();
        assert!(
            recoveries
                .iter()
                .any(|recovery| recovery.role == RecoveryRole::KeywordArgument
                    && recovery.text == "rounded up"),
            "expected a keyword-argument recovery of \"rounded up\": {recoveries:?}"
        );
        assert!(
            recoveries
                .iter()
                .all(|recovery| recovery.role != RecoveryRole::ActivationCost),
            "a keyword cost's residue must never attribute to activation cost: {recoveries:?}"
        );
    }

    // Terminal round trips: the supported corpus attests only the `Period`
    // carrier for a tight em-dash keyword cost's own terminal. A corpus-wide
    // grep for a tight-dash keyword cost body ending in `!` or `?` (outside
    // reminder text) found zero hosts — `Exclamation` and `Question` are
    // unattested shapes in this population. Per instruction, this is
    // recorded as a fact rather than backed by a fabricated synthetic test:
    // the enum keeps both carriers for completeness (a keyword cost is not
    // barred from ending a card's last printed sentence in `!`/`?` by any
    // structural rule), but no round-trip test exists for either because no
    // corpus row exercises them.

    #[test]
    fn tight_dash_cost_with_a_comma_boundary_recovers_structurally() {
        // The causal Stage A regression: a tight em-dash cost whose body
        // contains a comma used to be cut at the comma by the keyword-item
        // splitter before the argument was recognized, rejecting the whole
        // line. The comma is now cost-internal punctuation, not an item
        // boundary, once the item is proven to open `<atom><tight em dash>`.
        let source = "Escape—{2}{B}, Exile four other cards from your graveyard.";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast);
        };
        assert_eq!(list.len(), 1, "expected one keyword item");
        let KeywordArgument::Costed(KeywordCost::Components { cost, terminal }) =
            &list.first().argument
        else {
            panic!("expected a structured cost: {:#?}", list.first().argument);
        };
        assert!(*terminal);
        assert!(matches!(
            cost.components(),
            [CostComponent::Symbols(_), CostComponent::Clause(_)]
        ));
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);

        // Genericity: a different synthetic catalog keyword with the same
        // tight cost gets the same shape.
        let synthetic = "Rampage—{2}{B}, Exile four other cards from your graveyard.";
        let synthetic_report = parse_with_catalogs(synthetic, &shape_catalogs());
        assert!(
            synthetic_report.diagnostics.is_empty(),
            "{:?}",
            synthetic_report.diagnostics
        );
        let AbilityKind::Keyword(synthetic_list) = &synthetic_report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", synthetic_report.ast);
        };
        assert!(matches!(
            synthetic_list.first().argument,
            KeywordArgument::Costed(KeywordCost::Components { .. })
        ));

        // List mirror: the comma exemption applies only to the tight-cost
        // item, so a preceding plain keyword is still split at its own comma.
        let listed = "Flying, Ward—{2}, Pay 2 life.";
        let listed_report = parse_with_catalogs(listed, &shape_catalogs());
        assert!(
            listed_report.diagnostics.is_empty(),
            "{:?}",
            listed_report.diagnostics
        );
        let AbilityKind::Keyword(listed_list) = &listed_report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", listed_report.ast);
        };
        assert_eq!(listed_list.len(), 2, "expected two keyword items");
        assert!(matches!(
            listed_list.get(1).expect("a second keyword").argument,
            KeywordArgument::Costed(KeywordCost::Components { .. })
        ));
        assert_eq!(
            listed_report.ast.render("Test Card", false).unwrap(),
            listed
        );

        // Unchanged: a comma inside an ordinary plain list still splits three
        // ways, no tight-cost item involved.
        let plain = "Flying, first strike, trample";
        let plain_report = parse_with_catalogs(plain, &shape_catalogs());
        let AbilityKind::Keyword(plain_list) = &plain_report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", plain_report.ast);
        };
        assert_eq!(plain_list.len(), 3);
    }

    #[test]
    fn light_up_the_night_and_shredders_armor_carry_a_trailing_paragraph() {
        // Light Up the Night: the tight-cost item's cost owns only the
        // material through its own first sentence terminal; the remaining
        // same-line sentence is a trailing paragraph, not swallowed into the
        // cost.
        let source = "Flashback—{3}{R}, Remove X loyalty counters from among \
            planeswalkers you control. If you cast this spell this way, X can't be 0.";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast);
        };
        assert!(matches!(
            list.first().argument,
            KeywordArgument::Costed(KeywordCost::Components { .. })
        ));
        assert!(list.trailing().is_some(), "expected a trailing paragraph");
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);

        // Shredder's Armor: the inverse-shaped non-target witness — a clean
        // cost clause followed by a same-line sentence, with no recovery.
        let armor = "Equip—Sacrifice another nonland permanent. Activate only once each turn.";
        let armor_report = parse_with_catalogs(armor, &shape_catalogs());
        assert!(
            armor_report.diagnostics.is_empty(),
            "{:?}",
            armor_report.diagnostics
        );
        let AbilityKind::Keyword(armor_list) = &armor_report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", armor_report.ast);
        };
        assert!(matches!(
            armor_list.first().argument,
            KeywordArgument::Costed(KeywordCost::Components { .. })
        ));
        assert!(armor_list.trailing().is_some());
        assert_eq!(armor_report.ast.render("Test Card", false).unwrap(), armor);
    }

    #[test]
    fn restricted_cost_shapes_parse_and_round_trip() {
        for source in [
            "Splice onto Arcane {W}",
            "Craft with artifact {1}{U}",
            "Equip legendary creature {1}",
            "Craft with one or more {5}",
        ] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            assert!(
                report.diagnostics.is_empty(),
                "{source:?}: {:?}",
                report.diagnostics
            );
            let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
                panic!(
                    "expected a keyword ability for {source:?}: {:#?}",
                    report.ast
                );
            };
            assert_eq!(list.len(), 1);
            assert!(
                matches!(
                    list.first(),
                    KeywordAbility {
                        argument: KeywordArgument::RestrictedCost { .. },
                        ..
                    }
                ),
                "expected a single RestrictedCost item for {source:?}: {list:#?}"
            );
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }

        // Exact preposition/restriction/cost shapes.
        let onto = shape_argument("Splice onto Arcane {W}");
        assert!(matches!(
            onto,
            KeywordArgument::RestrictedCost {
                preposition: Some(Preposition::Onto),
                cost: KeywordCost::Symbols(ref symbols),
                ..
            } if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{W}"
        ));
        let with = shape_argument("Craft with artifact {1}{U}");
        assert!(matches!(
            with,
            KeywordArgument::RestrictedCost {
                preposition: Some(Preposition::With),
                ..
            }
        ));
        let bare = shape_argument("Equip legendary creature {1}");
        assert!(matches!(
            bare,
            KeywordArgument::RestrictedCost {
                preposition: None,
                ..
            }
        ));
        // The headless `NounPhraseKind::Quantity` restriction retains its
        // numeral structure rather than forcing an opaque noun.
        let headless = shape_argument("Craft with one or more {5}");
        assert!(matches!(
            headless,
            KeywordArgument::RestrictedCost {
                restriction,
                ..
            } if matches!(
                restriction.kind(),
                crate::syntax::NounPhraseKind::Quantity(quantity)
                    if matches!(quantity.kind(), crate::syntax::QuantityKind::OrComparison(..))
            )
        ));
    }

    #[test]
    fn restricted_cost_coordination_positives_round_trip() {
        for source in [
            "Craft with instant or sorcery {2}{U}",
            "Equip Shaman, Warlock, or Wizard {1}",
            "Craft with a Dinosaur, a Merfolk, a Pirate, and a Vampire {4}",
            "Craft with instant and sorcery cards {3}{U}",
        ] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            assert!(
                report.diagnostics.is_empty(),
                "{source:?}: {:?}",
                report.diagnostics
            );
            assert!(matches!(
                report.ast.abilities[0].kind(),
                AbilityKind::Keyword(_)
            ));
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }

        let source = "Craft with a Dinosaur, a Merfolk, a Pirate, and a Vampire {4}";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(
            report
                .provenance()
                .selections()
                .iter()
                .all(|selection| { selection.span().text(source) != Some("with a Dinosaur") }),
            "a failed comma-split alternative leaked into provenance: {:#?}",
            report.provenance()
        );
    }

    #[test]
    fn the_enigma_jewel_restriction_recovers_with_exactly_one_opaque_noun() {
        let source = "Craft with four or more nonlands with activated abilities {8}{U}";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        let opacity = report.ast.lexical_opacity();
        assert_eq!(
            opacity
                .iter()
                .filter(|reference| reference.text == "nonlands")
                .count(),
            1,
            "{opacity:?}"
        );
    }

    #[test]
    fn eye_of_ojer_taq_stays_a_whole_clause_recovery() {
        // The general `Quantity + that + …` headless-relative surface gate:
        // `two` must never become `Noun::Opaque` to license this shape. This
        // is the negative twin of `Craft with one or more {5}` above — both
        // are headless quantities, but only the comparison form is a
        // structurally complete noun phrase.
        let source = "Craft with two that share a card type {6}";
        let report = parse_with_catalogs(source, &shape_catalogs());
        assert!(
            !matches!(report.ast.abilities[0].kind(), AbilityKind::Keyword(_)),
            "must remain a whole-clause recovery, not a keyword ability: {:#?}",
            report.ast
        );
        assert!(!report.ast.recoveries().is_empty());
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
    }

    #[test]
    fn splice_onto_arcane_composes_restriction_with_a_tight_structured_cost() {
        for source in [
            "Splice onto Arcane—Exile four cards from your graveyard.",
            "Splice onto Arcane—Tap an untapped white creature you control.",
            "Splice onto Arcane—An opponent gains 5 life.",
            "Splice onto Arcane—Sacrifice two Mountains.",
            "Splice onto Arcane—Return a blue creature you control to its owner's hand.",
        ] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            assert!(
                report.diagnostics.is_empty(),
                "{source:?}: {:?}",
                report.diagnostics
            );
            assert_eq!(report.ast.recoveries(), Vec::new(), "{source:?}");
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }
        let argument = shape_argument("Splice onto Arcane—Exile four cards from your graveyard.");
        assert!(matches!(
            argument,
            KeywordArgument::RestrictedCost {
                preposition: Some(Preposition::Onto),
                cost: KeywordCost::Components { terminal: true, .. },
                ..
            }
        ));
    }

    #[test]
    fn reinforce_x_stays_outside_restricted_cost() {
        // Negative control: `Reinforce X—[cost]`'s left side is a bare
        // `Quantity::unchecked_x()` with no preposition at all, so
        // `parse_restricted_tight_cost`'s explicit-preposition requirement
        // rejects it — it must not become `RestrictedCost` even though the
        // shape (internal tight dash, symbol-run cost) otherwise resembles
        // Stage A′'s composition.
        for source in ["Reinforce X—{X}{W}{W}", "Reinforce X—{X}{G}{G}"] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            assert!(
                !matches!(report.ast.abilities[0].kind(), AbilityKind::Keyword(_))
                    || !matches!(
                        report.ast.abilities[0].kind(),
                        AbilityKind::Keyword(list)
                            if matches!(
                                &list.first().argument,
                                KeywordArgument::RestrictedCost { .. }
                            )
                    ),
                "must not become RestrictedCost: {:#?}",
                report.ast
            );
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }
    }

    #[test]
    fn restricted_cost_shape_leaves_untouched_surfaces_alone() {
        // `Equip {3}`: the whole-body symbol-cost arm still wins before the
        // restriction shape is ever tried.
        assert!(matches!(
            shape_argument("Equip {3}"),
            KeywordArgument::Costed(KeywordCost::Symbols(_))
        ));
        // Partner-with-name keeps its existing shape; it does not end in a
        // bare symbol run so the restriction shape is never tried.
        assert!(matches!(
            shape_argument("Partner—Friends forever"),
            KeywordArgument::Named { .. }
        ));
        // A non-`onto`/`with` leading preposition is not licensed: widening
        // to other prepositions is explicitly out of scope. `from` opens the
        // existing predicated-argument path instead (and fails it, given the
        // trailing symbol run), so this recovers rather than becoming
        // `RestrictedCost` — the point under test.
        assert!(
            !matches!(
                shape_argument("Craft from artifact {1}{U}"),
                KeywordArgument::RestrictedCost { .. }
            ),
            "an unlicensed leading preposition must not become a RestrictedCost"
        );
    }

    #[test]
    fn keyword_argument_shapes_parse_and_round_trip() {
        // No argument.
        assert!(matches!(shape_argument("Flying"), KeywordArgument::Absent));
        // Counted.
        assert!(matches!(
            shape_argument("Fabricate 2"),
            KeywordArgument::Counted(_)
        ));
        // Costed, symbol-sequence surface.
        assert!(matches!(
            shape_argument("Ward {2}"),
            KeywordArgument::Costed(KeywordCost::Symbols(ref symbols))
                if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{2}"
        ));
        // CountedCost.
        assert!(matches!(
            shape_argument("Suspend 4—{1}{U}"),
            KeywordArgument::CountedCost { .. }
        ));
        // Predicated, single quality carrying its actual preposition.
        assert!(matches!(
            shape_argument("Protection from red"),
            KeywordArgument::Predicated(ref predicated)
                if matches!(predicated.qualities.as_slice(), [quality]
                    if quality.preposition == Some(Preposition::From))
        ));
        // Predicated, coordinated.
        assert!(matches!(
            shape_argument("Protection from red and from white"),
            KeywordArgument::Predicated(ref predicated) if predicated.qualities.len() == 2
        ));
        // Statted (the deliberate seventh shape).
        assert!(matches!(
            shape_argument("Prototype {2}{G}{G} — 3/3"),
            KeywordArgument::Statted { .. }
        ));
        // Named — an em-dash pairing label, told from a sentence cost by surface.
        assert!(matches!(
            shape_argument("Partner—Friends forever"),
            KeywordArgument::Named { ref label, .. } if label == "Friends forever"
        ));
    }

    #[test]
    fn named_keyword_argument_labels_parse_and_round_trip() {
        for label in [
            "a Food",
            "a card",
            "a tapped Fish",
            "an extra turn",
            "a Treasure",
            "an Octopus",
        ] {
            let source = format!("Gift {label}");
            let report = parse_with_catalogs(&source, &shape_catalogs());
            let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
                panic!(
                    "expected {source:?} to be a keyword ability, got {:#?}",
                    report.ast.abilities[0].kind()
                );
            };
            assert_eq!(list.len(), 1);
            assert!(matches!(
                list.first(),
                KeywordAbility {
                    ability,
                    argument: KeywordArgument::Named {
                        separator: KeywordArgumentSeparator::Space,
                        label: parsed_label,
                    },
                    ..
                } if ability.canonical() == "Gift" && parsed_label == label
            ));
            assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }
    }

    #[test]
    fn named_keyword_argument_label_license_is_exact_and_keyword_independent() {
        assert!(matches!(
            shape_argument("Partner a Food"),
            KeywordArgument::Named {
                separator: KeywordArgumentSeparator::Space,
                ref label,
            } if label == "a Food"
        ));

        // The license this test guards is the *label* one: a bare noun phrase
        // is never a `Named` argument, because only an exact catalog member is.
        // Whether the line is a keyword ability at all is a separate question —
        // `Champion a Faerie` is one [CR#702.72a], and since round `enchant` it
        // parses as a `Qualified` noun phrase rather than recovering.
        for source in [
            "Champion a Faerie",
            "Gift a creature",
            "Gift each color",
            "Gift the Trolls",
            "Gift a Food.",
            "Gift a food",
        ] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            if let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() {
                assert!(
                    !matches!(list.first().argument, KeywordArgument::Named { .. }),
                    "{source:?} must not license a named label: {:#?}",
                    report.ast
                );
            }
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }

        let full_name = "Gift the Trolls";
        let full_name_clause = "Gift the Trolls deals 3 damage to any target.";
        let report = parse_with_identity(full_name_clause, &shape_catalogs(), full_name, false);
        assert!(matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::Paragraph(_)
        ));
        assert_eq!(
            report.ast.render(full_name, false).unwrap(),
            full_name_clause
        );

        let foretell = "Whenever you foretell a card, draw a card.";
        let report = parse_with_catalogs(foretell, &shape_catalogs());
        assert!(matches!(
            report.ast.abilities[0].kind(),
            AbilityKind::Triggered(_)
        ));
        assert_eq!(report.ast.render("Test Card", false).unwrap(), foretell);
    }

    #[test]
    fn quoted_symbol_keyword_argument_keeps_terminal_outside_typed_cost() {
        let source = "Target creature gains \"Ward {1}.\"";
        let report = parse_with_catalogs(source, &shape_catalogs());
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected a paragraph: {:#?}", report.ast);
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected a transitive grant clause: {:#?}",
                paragraph.sentences[0]
            );
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!("expected a simple transitive grant predicate");
        };
        let PredicateObject::QuotedAbility(quoted) = &predicate.object else {
            panic!("expected a quoted ability object: {:#?}", predicate.object);
        };
        let AbilityKind::Keyword(list) = quoted.ability.kind() else {
            panic!("expected a keyword ability: {:#?}", quoted.ability);
        };
        assert_eq!(list.len(), 1);
        assert!(matches!(
            list.first(),
            KeywordAbility {
                argument: KeywordArgument::Costed(KeywordCost::Symbols(symbols)),
                ..
            } if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{1}"
        ));
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
    }

    #[test]
    fn lone_terminal_is_rejected_as_a_keyword_argument_even_in_a_list() {
        let source = ".";
        let surface = crate::surface::lex(source);
        let catalogs = shape_catalogs();
        let self_reference = crate::identity::SelfReference::default();
        let mut parser = super::Parser::new(source, &catalogs, &self_reference, false);
        assert!(
            parser
                .parse_keyword_argument(&surface.tokens, 0, true, false, false)
                .is_none()
        );
    }

    #[test]
    fn symbol_keyword_argument_without_terminal_is_unchanged() {
        assert!(matches!(
            shape_argument("Ward {1}"),
            KeywordArgument::Costed(KeywordCost::Symbols(ref symbols))
                if symbols.iter().map(OracleSymbol::as_str).collect::<String>() == "{1}"
        ));
    }

    #[test]
    fn costed_sentence_surface_carries_a_structured_cost() {
        // A tight em-dash cost whose body is a sentence, not a bare label, is
        // the structured `Components` cost, not the legacy embedded-ability
        // `Sentence` (reserved for the `SpacedEmDash` surface).
        assert!(matches!(
            shape_argument("Ward—Sacrifice a creature."),
            KeywordArgument::Costed(KeywordCost::Components { terminal: true, .. })
        ));
    }

    #[test]
    fn bare_quality_is_a_predicated_argument_only_inside_a_list() {
        // Inside a list a bare quality (the atom carries the preposition) parses.
        let report = parse_with_catalogs("Reach, hexproof from blue", &shape_catalogs());
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast);
        };
        assert!(matches!(
            &list.get(1).expect("a second keyword").argument,
            KeywordArgument::Predicated(predicated)
                if matches!(predicated.qualities.as_slice(), [quality]
                    if quality.preposition.is_none())
        ));
        assert_eq!(
            report.ast.render("Test Card", false).unwrap(),
            "Reach, hexproof from blue"
        );
    }

    #[test]
    fn a_shape_outside_the_vocabulary_recovers_at_the_keyword_argument_role() {
        // A cost with a trailing sentence is no closed shape; it opens with a
        // symbol, so it stays a keyword argument and recovers rather than being
        // forced into Costed.
        let report =
            parse_with_catalogs("Ward {3}. This ability costs {1} less.", &shape_catalogs());
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast);
        };
        assert!(matches!(
            list.first().argument,
            KeywordArgument::Recovered { .. }
        ));
        assert_eq!(
            report.ast.render("Test Card", false).unwrap(),
            "Ward {3}. This ability costs {1} less."
        );
    }

    #[test]
    fn a_bare_noun_argument_is_a_qualified_keyword_argument() {
        // A keyword atom followed by a bare noun phrase is the shape [CR#702.5a]
        // gives enchant and [CR#702.72a] gives champion, so English syntax
        // admits it. That protection's own argument is `from [quality]`
        // [CR#702.16a] — making `Protection creature` meaningless as a Magic
        // rule — is a semantic fact this layer deliberately does not decide;
        // see the `english-clauses-are-structural` decision. What the grammar
        // must still refuse is an atom that spells its own preposition, whose
        // tail is that preposition's complement rather than an object.
        let report = parse_with_catalogs("Protection creature", &shape_catalogs());
        let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast);
        };
        assert!(matches!(
            list.first().argument,
            KeywordArgument::Qualified(Phrase::NounPhrase(_))
        ));
        assert_eq!(
            report.ast.render("Test Card", false).unwrap(),
            "Protection creature"
        );
    }

    #[test]
    fn an_atom_carrying_its_own_preposition_takes_no_bare_object() {
        // `Partner with Proud Mentor` names a card; the atom has already
        // consumed `with`, so its tail is that preposition's complement and
        // must not be re-read as an object noun phrase.
        for source in ["Partner with Proud Mentor", "Hexproof from black"] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            if let AbilityKind::Keyword(list) = report.ast.abilities[0].kind() {
                assert!(
                    !matches!(
                        list.first().argument,
                        KeywordArgument::Qualified(Phrase::NounPhrase(_))
                    ),
                    "{source:?} must not take a bare object: {:#?}",
                    report.ast
                );
            }
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }
    }

    #[test]
    fn comma_and_semicolon_keyword_lists_round_trip() {
        for source in [
            "Flying, first strike, protection from red",
            "Flying, protection from red and from white",
            // A comma-split proper-name fragment can look like another
            // keyword plus a predicated `of` argument.  Silvar, Devourer of
            // the Free supplies the corpus witness (`Trynn, Champion of
            // Freedom`), and source-free rendering must preserve that shape.
            "Flying, Champion of Freedom",
            "Trample; rampage 1",
        ] {
            let report = parse_with_catalogs(source, &shape_catalogs());
            assert!(
                matches!(report.ast.abilities[0].kind(), AbilityKind::Keyword(_)),
                "expected a keyword list for {source:?}: {:#?}",
                report.ast
            );
            assert_eq!(report.ast.render("Test Card", false).unwrap(), source);
        }
    }

    fn sentence_independent(sentence: &Sentence) -> &IndependentClause {
        let SentenceBody::Independent(clause) = &sentence.body else {
            panic!("expected independent sentence, got {:?}", sentence.body);
        };
        clause
    }

    fn is_imperative_clause(clause: &IndependentClause) -> bool {
        matches!(
            clause,
            IndependentClause::Finite(finite) if finite.subject().is_none()
        )
    }

    fn is_deontic_clause(clause: &IndependentClause) -> bool {
        matches!(
            clause,
            IndependentClause::Finite(finite)
                if matches!(
                    finite.predicate(),
                    PredicateExpression::Simple(Predicate::Deontic(_))
                )
        )
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
        let IndependentClause::Finite(finite) = clause else {
            panic!("expected finite lexical predicate, got {clause:?}");
        };
        let PredicateExpression::Simple(predicate) = finite.predicate() else {
            panic!(
                "expected simple lexical predicate, got {:?}",
                finite.predicate()
            );
        };
        match predicate {
            Predicate::Transitive(predicate) => predicate.head(),
            Predicate::Intransitive(predicate) => predicate.head(),
            Predicate::Passive(predicate) => predicate.head(),
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
            let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
                panic!(
                    "expected a triggered ability: {:#?}",
                    report.ast.abilities[0].kind()
                );
            };
            assert!(match &triggered.conditions.first.event {
                TriggerEvent::Clause(IndependentClause::Finite(finite)) => {
                    matches!(finite.predicate(), PredicateExpression::Coordinated(_))
                }
                TriggerEvent::Clause(IndependentClause::Coordinated(_)) => true,
                _ => false,
            });
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
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a triggered ability: {:#?}",
                report.ast.abilities[0].kind()
            );
        };
        assert!(matches!(
            triggered.conditions.first.event,
            TriggerEvent::Clause(IndependentClause::Finite(ref finite))
                if matches!(finite.predicate(), PredicateExpression::Coordinated(_))
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
    fn interior_nickname_periods_reach_the_chart_as_one_sentence() {
        let source = "Attach it to U.S.Agent.";
        let report =
            parse_with_identity(source, &fixture_catalogs(), "U.S.Agent, John Walker", true);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected a paragraph: {:#?}", report.ast.abilities[0]);
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!(
                "period-bearing nickname was split: {:#?}",
                paragraph.sentences
            );
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) = &sentence.body else {
            panic!(
                "nickname did not reach a finite clause: {:#?}",
                sentence.body
            );
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!(
                "nickname did not reach a transitive predicate: {:#?}",
                finite.predicate()
            );
        };
        assert!(finite.subject().is_none());
        assert!(
            matches!(
                predicate.elements.as_slice(),
                [PredicateElement::Adjunct(PredicateAdjunct::Prepositional(value))]
                    if matches!(
                        value.kind(),
                        PrepositionalPhraseKind::Simple(simple)
                            if matches!(
                                simple.object().kind(),
                                crate::syntax::PrepositionalObjectKind::NounPhrase(noun_phrase)
                                    if matches!(
                                        noun_phrase.kind(),
                                        crate::syntax::NounPhraseKind::ThisCard(
                                            ThisCardForm::AbbreviatedName
                                        )
                                    )
                            )
                    )
            ),
            "nickname did not reach self-reference disambiguation: {:#?}",
            sentence.body
        );
    }

    #[test]
    fn possessive_period_bearing_nickname_reaches_the_chart_intact() {
        let source = "Ms. Marvel's base power is equal to the number of cards in your hand.";
        let report =
            parse_with_identity(source, &fixture_catalogs(), "Ms. Marvel, Kamala Khan", true);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected a paragraph: {:#?}", report.ast.abilities[0]);
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!(
                "possessive period-bearing nickname was split: {:#?}",
                paragraph.sentences
            );
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) = &sentence.body else {
            panic!(
                "possessive nickname did not reach a finite clause: {:#?}",
                sentence.body
            );
        };
        let (Some(Subject(noun_phrase)), PredicateExpression::Simple(Predicate::Intransitive(_))) =
            (finite.subject(), finite.predicate())
        else {
            panic!("possessive nickname did not reach an intransitive predication: {finite:#?}");
        };
        assert!(
            matches!(
                noun_phrase.kind(),
                crate::syntax::NounPhraseKind::Nominal(nominal)
                    if matches!(
                        nominal.determiner(),
                        Some(determiner)
                            if matches!(
                                determiner.kind(),
                                crate::syntax::DeterminerKind::Possessive(possessor)
                                    if matches!(
                                        possessor,
                                        crate::syntax::Possessor::NounPhrase(possessor)
                                            if matches!(
                                                possessor.kind(),
                                                crate::syntax::NounPhraseKind::ThisCard(
                                                    ThisCardForm::AbbreviatedName
                                                )
                                            )
                                    )
                            )
                    )
            ),
            "possessive nickname did not reach self-reference disambiguation: {:#?}",
            sentence.body
        );
    }

    #[test]
    fn nickname_final_period_remains_a_sentence_terminal() {
        let source = "Destroy Agent X. Draw a card.";
        let surface = lex(source);
        let self_reference = SelfReference::new("Agent X., Hero", true);
        let sentences = super::split_sentences(source, &surface.tokens, self_reference.nickname());
        assert_eq!(
            sentences
                .iter()
                .map(|sentence| super::tokens_span(sentence).text(source).unwrap())
                .collect::<Vec<_>>(),
            ["Destroy Agent X.", "Draw a card."]
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
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a modal ability: {:#?}",
                report.ast.abilities[0].kind()
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
            heading.cost.components(),
            [CostComponent::Symbols(symbols)] if symbols.len() == 1
        ));
        assert!(second.heading.is_some());
        assert_eq!(render(&report), source);
    }

    #[test]
    fn trigger_after_an_activation_cost_parses() {
        let source = "{T}: Draw a card. When you do, target creature can't block this turn.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Activated(activated) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = activated.effect.sentences.as_slice() else {
            panic!("expected two effect sentences: {:#?}", activated.effect);
        };
        assert!(
            matches!(&second.body, SentenceBody::Triggered(triggered) if triggered.trigger.introducer == TriggerWord::When),
            "{:#?}",
            second.body
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn reflexive_when_you_do_trigger_parses_mid_paragraph() {
        let source = "You may exert this creature as it attacks. \
             When you do, target creature can't block this turn.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = paragraph.sentences.as_slice() else {
            panic!("expected two sentences: {paragraph:#?}");
        };
        let SentenceBody::Triggered(triggered) = &second.body else {
            panic!("expected a triggered sentence body: {:#?}", second.body);
        };
        assert_eq!(triggered.trigger.introducer, TriggerWord::When);
        assert!(triggered.trigger.intervening_condition.is_none());
        assert_eq!(render(&report), source);
    }

    #[test]
    fn trigger_after_a_loyalty_header_parses() {
        let source = "[+1]: Draw a card. When you do, target creature can't block this turn.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Loyalty(loyalty) = report.ast.abilities[0].kind() else {
            panic!("expected loyalty ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = loyalty.effect.sentences.as_slice() else {
            panic!("expected two effect sentences: {:#?}", loyalty.effect);
        };
        assert!(
            matches!(&second.body, SentenceBody::Triggered(triggered) if triggered.trigger.introducer == TriggerWord::When),
            "{:#?}",
            second.body
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn bare_imperative_trigger_effect_parses() {
        // A bare-imperative span now falls through an unlowerable nominal root
        // to the lowerable imperative root under the `Clause` goal itself.
        let source = "You may exert this creature as it attacks. When you do, copy that spell.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = paragraph.sentences.as_slice() else {
            panic!("expected two sentences: {paragraph:#?}");
        };
        let SentenceBody::Triggered(triggered) = &second.body else {
            panic!("expected a triggered sentence body: {:#?}", second.body);
        };
        assert!(
            is_imperative_clause(&triggered.effect),
            "{:#?}",
            triggered.effect
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn deontic_trigger_effect_still_parses_through_the_clause_attempt() {
        // A finite deontic effect (Ahn-Crop Crasher's shape) already lowers
        // under the original best root, so root-lowering fallback must not
        // reorder it. Selection-stat invariance is pinned at the grammar
        // boundary by `successful_single_root_selections_are_unchanged`.
        let source = "You may exert this creature as it attacks. \
             When you do, target creature can't block this turn.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = paragraph.sentences.as_slice() else {
            panic!("expected two sentences: {paragraph:#?}");
        };
        let SentenceBody::Triggered(triggered) = &second.body else {
            panic!("expected a triggered sentence body: {:#?}", second.body);
        };
        assert!(
            is_deontic_clause(&triggered.effect),
            "{:#?}",
            triggered.effect
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn trigger_effect_rejects_a_modal_choice_header() {
        // A `Choose one —` sentence following a trigger comma must not become
        // a `TriggeredSentence` effect: `parse_trigger_effect` only accepts
        // `SentenceBody::Independent`, and (upstream of it) the reflexive
        // `When you do,` here is captured by `ChoiceInstruction`'s own
        // `trigger_prefix`, never migrating to `SentenceBody::Triggered`.
        let source = "Draw a card. When you do, choose one —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!("expected a modal ability: {:#?}", report.ast.abilities[0]);
        };
        assert!(
            modal
                .header
                .sentences
                .iter()
                .all(|sentence| !matches!(sentence.body, SentenceBody::Triggered(_))),
            "a modal header must never be captured as a trigger effect: {:#?}",
            modal.header
        );
    }

    #[test]
    fn trigger_effect_does_not_swallow_a_following_sentence() {
        // `parse_trigger_effect` parses one sentence's worth of tokens (its
        // caller peels one sentence at a time); a second, independent
        // sentence after the triggered one must remain its own sentence, not
        // be absorbed into the trigger's effect.
        let source = "You may exert this creature as it attacks. \
             When you do, draw a card. Then discard a card.";
        let report = parse(source);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!("expected paragraph ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second, third] = paragraph.sentences.as_slice() else {
            panic!("expected three sentences: {paragraph:#?}");
        };
        let SentenceBody::Triggered(triggered) = &second.body else {
            panic!(
                "expected the second sentence to be triggered: {:#?}",
                second.body
            );
        };
        assert!(
            is_imperative_clause(&triggered.effect),
            "{:#?}",
            triggered.effect
        );
        assert!(
            !matches!(third.body, SentenceBody::Triggered(_)),
            "the third sentence must not be folded into the trigger's effect: {:#?}",
            third.body
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn imperative_activation_cost_is_unchanged() {
        // Barl's Cage's shape now parses directly under the cost component's
        // single `Clause` attempt; no `Sentence` retry is required.
        let source = "{3}, Sacrifice a creature: Draw a card.";
        let report = parse(source);
        let AbilityKind::Activated(ability) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, sacrifice] = ability.cost.components() else {
            panic!("expected two cost components: {:#?}", ability.cost);
        };
        assert!(matches!(
            sacrifice,
            CostComponent::Clause(clause) if is_imperative_clause(clause)
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn triggered_sentence_with_imperative_effect_round_trips() {
        // Render-back equality, terminal period included, for two of §5's
        // bare-imperative shapes.
        for source in [
            "You may exert this creature as it attacks. \
             When you do, return target creature card from your graveyard to the battlefield.",
            "You may exert this creature as it attacks. \
             When you do, tap target creature an opponent controls.",
        ] {
            let report = parse(source);
            assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
            let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
                panic!("expected paragraph ability: {:#?}", report.ast.abilities[0]);
            };
            let [_, second] = paragraph.sentences.as_slice() else {
                panic!("expected two sentences: {paragraph:#?}");
            };
            let SentenceBody::Triggered(triggered) = &second.body else {
                panic!("expected a triggered sentence body: {:#?}", second.body);
            };
            assert!(
                is_imperative_clause(&triggered.effect),
                "{:#?}",
                triggered.effect
            );
            assert_eq!(render(&report), source);
        }
    }

    #[test]
    fn ability_initial_trigger_still_wins_the_ability_frame() {
        // The ability-initial trigger must still be absorbed by
        // `parse_ability_kind`'s own `trigger_frame` call, never reach the new
        // per-sentence fallback as a `Paragraph` sentence.
        let source = "Whenever you cast an instant or sorcery spell, this creature deals 2 damage to each opponent.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert!(
            matches!(report.ast.abilities[0].kind(), AbilityKind::Triggered(_)),
            "expected AbilityKind::Triggered, not a paragraph sentence: {:#?}",
            report.ast.abilities[0].kind()
        );
    }

    #[test]
    fn modal_choice_trigger_prefix_is_not_stolen_by_the_sentence_fallback() {
        // Same shape as `choice_instruction_carries_a_reflexive_second_trigger_prefix`,
        // re-asserted here under the new fallback's name: the reflexive `When you
        // do,` heading a modal header sentence must stay `ChoiceInstruction`'s own
        // `trigger_prefix`/`TriggerHeader`, never migrate to `SentenceBody::Triggered`.
        let source = "Draw a card. When you do, choose one —\n• Draw a card.\n• Draw two cards.";
        let report = parse(source);
        let AbilityKind::Modal(modal) = report.ast.abilities[0].kind() else {
            panic!("expected a modal ability: {:#?}", report.ast.abilities[0]);
        };
        let choice = modal_header_choice(&report);
        let prefix = choice
            .trigger_prefix
            .as_ref()
            .expect("a reflexive trigger prefix is carried by ChoiceInstruction");
        assert_eq!(prefix.introducer, TriggerWord::When);
        // No header sentence became a `SentenceBody::Triggered`.
        assert!(
            modal
                .header
                .sentences
                .iter()
                .all(|sentence| !matches!(sentence.body, SentenceBody::Triggered(_)))
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn paragraph_initial_trigger_with_a_coordinated_event_still_recovers() {
        // Merieke Ri Berit witness, corrected by this round's Stage 0 finding
        // (§1.2/§6.0): this trigger's blocker was never the coordinated event
        // (`leaves the battlefield or becomes untapped`, which the event
        // parse already handles) — it was the bare-imperative effect
        // (`destroy that creature`) failing to lower under the `Clause`
        // goal. It is measured residue (`midtrigger-positional-stayed.txt`),
        // not the `midtrigger-initial.txt` must-not-move set, so this round's
        // the lowerable-root retry now lowers it: the trigger moves from
        // `Recovered` to `Triggered` with an `Imperative` effect, and the
        // coordinated event is carried unchanged.
        let source = "{T}: Gain control of target creature for as long as you control Merieke Ri Berit. \
             When Merieke Ri Berit leaves the battlefield or becomes untapped, destroy that creature.";
        let report = parse(source);
        let AbilityKind::Activated(activated) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = activated.effect.sentences.as_slice() else {
            panic!("expected two effect sentences: {:#?}", activated.effect);
        };
        let SentenceBody::Triggered(triggered) = &second.body else {
            panic!(
                "expected the non-initial coordinated-event trigger to now move to Triggered: {:#?}",
                second.body
            );
        };
        assert!(
            matches!(
                triggered.trigger.event,
                TriggerEvent::Clause(IndependentClause::Finite(ref finite))
                    if matches!(finite.predicate(), PredicateExpression::Coordinated(_))
            ),
            "{:#?}",
            triggered.trigger.event
        );
        assert!(
            is_imperative_clause(&triggered.effect),
            "{:#?}",
            triggered.effect
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn triggered_sentence_round_trips() {
        // One of each of S1/S3/S6, rendered back byte-exact.
        for source in [
            "You may exert this creature as it attacks. \
             When you do, target creature can't block this turn.",
            "[+1]: Draw a card. When you do, target creature can't block this turn.",
            "{T}: Gain control of target creature for as long as you control Merieke Ri Berit. \
             At the beginning of the next end step, target creature can't block this turn.",
        ] {
            let report = parse(source);
            assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
            assert_eq!(render(&report), source);
        }
    }

    #[test]
    fn existential_trigger_event_parses() {
        // Drop of Honey's shape (§1.1/§6.0 C): the event is
        // `IndependentClause::Existential`, previously discarded by
        // `coordinated_event`'s filter.
        let source = "When there are no creatures on the battlefield, sacrifice this creature.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a triggered ability: {:#?}",
                report.ast.abilities[0]
            );
        };
        assert!(
            matches!(
                triggered.conditions.first.event,
                TriggerEvent::Clause(IndependentClause::Existential(_))
            ),
            "{:#?}",
            triggered.conditions.first.event
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn trigger_event_with_a_while_rider_parses() {
        // Bristlebane Battler's shape (§1.1/§6.0 B): the event is a `Complex`
        // clause carrying the `while` rider as an attachment INSIDE the event,
        // not migrated onto the effect.
        let source = "Whenever another creature you control enters while this creature has \
             a -1/-1 counter on it, draw a card.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a triggered ability: {:#?}",
                report.ast.abilities[0]
            );
        };
        let TriggerEvent::Clause(IndependentClause::Complex(complex)) =
            &triggered.conditions.first.event
        else {
            panic!(
                "expected a Complex clause event: {:#?}",
                triggered.conditions.first.event
            );
        };
        assert!(matches!(
            complex.attachment().payload(),
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(Subordinator::While, _))
        ));
        // The effect is the ordinary bare imperative, not the while rider.
        assert!(matches!(
            triggered.effect.sentences[0].body,
            SentenceBody::Independent(IndependentClause::Finite(ref finite))
                if finite.subject().is_none()
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn coordinated_trigger_event_is_unchanged() {
        // Merieke Ri Berit's shared subject scopes over the coordinated event
        // predicates rather than being buried in a first clause conjunct.
        let source = "{T}: Gain control of target creature for as long as you control Merieke Ri Berit. \
             When Merieke Ri Berit leaves the battlefield or becomes untapped, destroy that creature.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Activated(activated) = report.ast.abilities[0].kind() else {
            panic!("expected activated ability: {:#?}", report.ast.abilities[0]);
        };
        let [_, second] = activated.effect.sentences.as_slice() else {
            panic!("expected two effect sentences: {:#?}", activated.effect);
        };
        let SentenceBody::Triggered(triggered) = &second.body else {
            panic!("expected Triggered: {:#?}", second.body);
        };
        assert!(matches!(
            triggered.trigger.event,
            TriggerEvent::Clause(IndependentClause::Finite(ref finite))
                if matches!(finite.predicate(), PredicateExpression::Coordinated(_))
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn dependent_clause_is_not_a_trigger_event() {
        // A bare subordinate span between the introducer and the comma is not
        // an event: `clause_event`'s `Clause::Dependent(_)` arm keeps
        // rejecting it, and the whole ability recovers rather than admitting a
        // stray subordinate clause as a trigger.
        let source = "Whenever while this creature has a -1/-1 counter on it, draw a card.";
        let report = parse(source);
        assert!(
            !report.diagnostics.is_empty(),
            "a dependent-clause event must not parse cleanly"
        );
        assert!(
            !report
                .ast
                .abilities
                .iter()
                .any(|ability| matches!(ability.kind(), AbilityKind::Triggered(_))),
            "no ability should have become Triggered: {:#?}",
            report.ast.abilities
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn mixed_introducer_trigger_conditions_form_one_ability() {
        // The Shrine/Tombstone Stairwell shape is one triggered ability with
        // two trigger conditions [CR#603.1b], each retaining its own
        // introducer and event. In particular, the second condition must not be
        // swallowed into the first event merely because that wrong tree could
        // still reproduce the source text.
        let source = "At the beginning of your upkeep and whenever you cast a black spell, sacrifice a Goblin.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let [ability] = report.ast.abilities.as_slice() else {
            panic!("expected one ability: {:#?}", report.ast.abilities);
        };
        let AbilityKind::Triggered(triggered) = ability.kind() else {
            panic!("expected one triggered ability: {ability:#?}");
        };
        assert!(matches!(
            triggered.conditions.first,
            TriggerCondition {
                introducer: TriggerWord::At,
                event: TriggerEvent::Temporal(_),
            }
        ));
        assert!(matches!(
            triggered.conditions.rest.as_slice(),
            [TriggerConditionCoordination {
                conjunction: Conjunction::And,
                condition: TriggerCondition {
                    introducer: TriggerWord::Whenever,
                    event: TriggerEvent::Clause(_),
                },
            }]
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn mixed_introducer_conditions_preserve_or_coordination() {
        let source = "When Nissa attacks or when Nissa blocks, draw a card.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!("expected triggered ability: {:#?}", report.ast.abilities[0]);
        };
        assert!(matches!(
            triggered.conditions.rest.as_slice(),
            [TriggerConditionCoordination {
                conjunction: Conjunction::Or,
                condition: TriggerCondition {
                    introducer: TriggerWord::When,
                    ..
                },
            }]
        ));
        assert_eq!(render(&report), source);
    }

    #[test]
    fn coordinated_condition_probe_preserves_single_trigger_selections() {
        // The mixed-condition attempt must be selection-neutral for the
        // thousands of ordinary ability-initial triggers. These are exactly
        // the two successful chart spans from the original single-frame path:
        // one event and one effect, with no partial coordinated probe leaked.
        let source = "Whenever Nissa attacks, draw a card.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!("expected triggered ability: {:#?}", report.ast.abilities[0]);
        };
        assert!(triggered.conditions.rest.is_empty());
        let selection_surfaces = report
            .provenance()
            .selections()
            .iter()
            .map(|selection| selection.span().text(source).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            selection_surfaces,
            [
                "Nissa attacks",
                "draw a card.",
                "Whenever Nissa attacks, draw a card."
            ]
        );
        assert_eq!(render(&report), source);
    }

    #[test]
    fn coordinated_subject_with_at_least_is_not_a_condition_list() {
        let source = "Whenever Nissa and at least one other creature attack, draw a card.";
        let report = parse(source);
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        let AbilityKind::Triggered(triggered) = report.ast.abilities[0].kind() else {
            panic!("expected triggered ability: {:#?}", report.ast.abilities[0]);
        };
        assert!(triggered.conditions.rest.is_empty());
        assert_eq!(render(&report), source);
    }

    // `kwgrant` round, Stage C: the atom itself carries `from`
    // (`Hexproof from black`) — the keyword-line half of the round, gated by
    // `keyword_atom_carries_from`, never a keyword-name list.

    fn hexproof_from_catalogs() -> Catalogs {
        fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Hexproof", "Hexproof from"])
    }

    #[test]
    fn keyword_grant_atom_carried_bare_color_quality() {
        let source = "Hexproof from black";
        let report = parse_with_identity(source, &hexproof_from_catalogs(), FIXTURE_NAME, true);
        assert!(report.ast.recoveries().is_empty(), "{:#?}", report.ast);
        let AbilityKind::Keyword(keywords) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast.abilities[0]);
        };
        assert_eq!(keywords.len(), 1, "expected exactly one keyword ability");
        let ability = keywords.first();
        assert_eq!(ability.ability.canonical(), "Hexproof from");
        let KeywordArgument::Predicated(argument) = &ability.argument else {
            panic!("expected a predicated argument: {:#?}", ability.argument);
        };
        let [quality] = argument.qualities.as_slice() else {
            panic!("expected exactly one quality: {:#?}", argument.qualities);
        };
        assert_eq!(quality.preposition, None);
        assert!(matches!(
            quality.quality,
            Phrase::ColorWord(ColorWord::Black)
        ));
        assert_eq!(report.ast.render(FIXTURE_NAME, true).unwrap(), source);
    }

    #[test]
    fn keyword_grant_atom_carried_bare_adjective_quality() {
        // `monocolored` is an adjective, not a noun — the required
        // `AdjectivePhrase` fallback arm in `parse_predicated_quality`.
        let source = "Hexproof from monocolored";
        let report = parse_with_identity(source, &hexproof_from_catalogs(), FIXTURE_NAME, true);
        assert!(report.ast.recoveries().is_empty(), "{:#?}", report.ast);
        let AbilityKind::Keyword(keywords) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability: {:#?}", report.ast.abilities[0]);
        };
        let KeywordArgument::Predicated(argument) = &keywords.first().argument else {
            panic!(
                "expected a predicated argument: {:#?}",
                keywords.first().argument
            );
        };
        let [quality] = argument.qualities.as_slice() else {
            panic!("expected exactly one quality: {:#?}", argument.qualities);
        };
        assert_eq!(quality.preposition, None);
        assert!(matches!(quality.quality, Phrase::AdjectivePhrase(_)));
        assert_eq!(report.ast.render(FIXTURE_NAME, true).unwrap(), source);
    }

    #[test]
    fn keyword_grant_shorter_atom_never_beats_atom_carried_from() {
        // The canonical atom must be `Hexproof from`, never the shorter
        // `Hexproof`, whenever both are in the catalog and the line carries
        // a `from`-eligible tail.
        let source = "Hexproof from black";
        let report = parse_with_identity(source, &hexproof_from_catalogs(), FIXTURE_NAME, true);
        let AbilityKind::Keyword(keywords) = report.ast.abilities[0].kind() else {
            panic!("expected a keyword ability");
        };
        assert_eq!(keywords.first().ability.canonical(), "Hexproof from");
    }

    #[test]
    fn keyword_grant_bare_quality_without_atom_carried_from_is_recovered() {
        // Negative gate: a plain `Hexproof` atom (no final `from`) must not
        // permit a bare single-line quality — `Hexproof black` recovers,
        // it does not structure.
        let catalogs = fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Hexproof"]);
        let report = parse_with_identity("Hexproof black", &catalogs, FIXTURE_NAME, true);
        assert!(
            !report.ast.recoveries().is_empty(),
            "a bare quality on a non-`from`-carrying atom must not structure: {:#?}",
            report.ast
        );
    }

    #[test]
    fn keyword_grant_atom_carried_repeated_quality_in_grant_position() {
        let source = "This creature gains hexproof from blue and from black.";
        let report = parse_with_identity(source, &hexproof_from_catalogs(), FIXTURE_NAME, true);
        assert!(report.ast.recoveries().is_empty(), "{:#?}", report.ast);
        assert_eq!(report.ast.render(FIXTURE_NAME, true).unwrap(), source);
        let AbilityKind::Paragraph(paragraph) = report.ast.abilities[0].kind() else {
            panic!(
                "expected a paragraph ability: {:#?}",
                report.ast.abilities[0]
            );
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected a transitive clause");
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!("expected a simple transitive predicate");
        };
        let PredicateObject::NounPhrase(noun_phrase) = &predicate.object else {
            panic!("expected a nominal object: {:?}", predicate.object);
        };
        let crate::syntax::NounPhraseKind::Nominal(nominal) = noun_phrase.kind() else {
            panic!("expected a nominal object: {:?}", predicate.object);
        };
        let crate::word::NounInstanceKind::Mass(Noun::Catalog(atom)) = nominal.head().kind() else {
            panic!("expected a catalog noun head: {:?}", nominal.head());
        };
        assert_eq!(atom.canonical(), "Hexproof from");
        let [NominalComplement::KeywordArgument(KeywordArgument::Predicated(argument))] =
            nominal.complements()
        else {
            panic!(
                "expected exactly one predicated keyword argument complement: {:?}",
                nominal.complements()
            );
        };
        let [first, second] = argument.qualities.as_slice() else {
            panic!("expected exactly two qualities: {:?}", argument.qualities);
        };
        assert_eq!(first.preposition, None);
        assert!(matches!(first.quality, Phrase::ColorWord(ColorWord::Blue)));
        assert_eq!(second.preposition, Some(Preposition::From));
        assert!(matches!(
            second.quality,
            Phrase::ColorWord(ColorWord::Black)
        ));
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
                    "Level up",
                    "Trample",
                    "Station",
                    "Deathtouch",
                ],
            )
            .with_catalog(
                CatalogKind::KeywordAction,
                ["Scry", "Manifest dread", "Fight", "Destroy", "Discard"],
            )
            .with_catalog(CatalogKind::AbilityWord, ["Landfall", "Void", "Strive"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(CatalogKind::ArtifactType, ["Treasure", "Vehicle"])
            .with_catalog(
                CatalogKind::CardType,
                ["Creature", "Land", "Artifact", "Enchantment"],
            )
    }
}
