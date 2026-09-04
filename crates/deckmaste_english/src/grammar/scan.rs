use super::Adjective;
use super::AdjectiveComparisonState;
use super::Auxiliary;
use super::AuxiliaryInflection;
use super::AuxiliaryInstance;
use super::BareNominalAdjunct;
use super::CatalogSlot;
use super::CatalogValue;
use super::CoordinationDomain;
use super::CopulaInflection;
use super::Demonstrative;
use super::Determiner;
use super::EnglishGrammar;
use super::EnglishLexicalSlot;
use super::ExistentialForm;
use super::ExistentialKey;
use super::Features;
use super::FrequencyBound;
use super::FrequencyCount;
use super::FrequencyPhrase;
use super::IndefiniteArticle;
use super::InitialSound;
use super::LexicalMatch;
use super::LexicalSlot;
use super::LiteralKey;
use super::MeaningKey;
use super::Noun;
use super::NounForm;
use super::NounInstance;
use super::NounUsage;
use super::Number;
use super::NumberLiteral;
use super::Numeral;
use super::OracleSymbol;
use super::ParseCost;
use super::Person;
use super::PersonNumber;
use super::PowerToughness;
use super::Preposition;
use super::Pronoun;
use super::PronounCase;
use super::PronounInstance;
use super::Punctuation;
use super::SUBJECT_AUXILIARY_FORMS;
use super::SUBJECT_AUXILIARY_SURFACES;
use super::SetExceptionState;
use super::Span;
use super::SubjectAuxiliaryKey;
use super::ThisCardForm;
use super::Token;
use super::TokenKind;
use super::Verb;
use super::VerbAnalysis;
use super::VerbInstance;
use super::VerbSlot;
use super::Vocab;
use super::Vocabulary;
use super::WordMatch;
use super::parse_notation;
use super::surface_initial_sound;
use crate::word::AdjectiveComplementKind;

const NUMERAL_NOTATIONS: [Numeral; 5] = [
    Numeral::Cardinal,
    Numeral::Ordinal,
    Numeral::Arabic(false),
    Numeral::Arabic(true),
    Numeral::Roman,
];

impl EnglishGrammar<'_, '_> {
    pub(super) fn recognized_numerals(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<(usize, NumberLiteral)> {
        let Some(first) = tokens.get(start) else {
            return Vec::new();
        };
        if !matches!(first.kind, TokenKind::Word | TokenKind::Integer) {
            return Vec::new();
        }

        let mut matches = Vec::new();
        for end in (start + 1)..=tokens.len() {
            let last = &tokens[end - 1];
            if !matches!(
                last.kind,
                TokenKind::Word | TokenKind::Integer | TokenKind::Punctuation(Punctuation::Comma)
            ) {
                break;
            }
            let span = Span::new(first.span.start, last.span.end);
            let Some(surface) = span.text(self.source) else {
                break;
            };
            for notation in NUMERAL_NOTATIONS {
                if let Some(value) =
                    parse_notation(notation, surface, Self::is_sentence_initial(tokens, start))
                {
                    matches.push((
                        end,
                        NumberLiteral {
                            value,
                            numeral: notation,
                        },
                    ));
                }
            }
        }
        matches
    }

    fn token_is_in_recognized_numeral(&self, tokens: &[Token], index: usize) -> bool {
        (0..=index).any(|start| {
            self.recognized_numerals(tokens, start)
                .into_iter()
                .any(|(end, _)| index < end)
        })
    }

    pub(super) fn scan_verb(
        &self,
        tokens: &[Token],
        start: usize,
        verb_slot: VerbSlot,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let mut matches = self.word_matches(tokens, start, LexicalSlot::Verb(verb_slot));
        matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Verb(verb_slot)));
        let follows_conjunction = start.checked_sub(1).is_some_and(|previous| {
            self.token_text(tokens, previous)
                .is_some_and(|surface| surface.eq_ignore_ascii_case("and"))
        });
        if follows_conjunction {
            for item in &mut matches {
                if matches!(
                    &item.meaning,
                    MeaningKey::Verb(VerbAnalysis {
                        instance: VerbInstance {
                            verb: Verb::Word(Vocab::Target),
                            ..
                        },
                        ..
                    })
                ) {
                    // After `and`, rules text commonly starts a second noun
                    // phrase with the `target` determiner. Keep the imperative
                    // verb reading as a fallback, but make it lose whenever
                    // the coordinated-object reading also completes.
                    item.local_cost.reading_dispreference += 1;
                }
            }
        }
        // A multi-word keyword action spells one rules-defined action; the shorter
        // verb readings that start at the same token — the vocabulary verb, or the
        // single-word keyword action whose spelling the phrase's head coincides with
        // — are decompositions of it and must lose **by cost**, not by scan order.
        // Vocabulary and catalog matches are concatenated with no cross-source dedup
        // or cost, so an equal-cost tie here would fall to interning order
        // (`alternative_index`/`NodeId`, invariant-audit §A.1).
        //
        // The test is span length, not identity: a verb match covering more than one
        // token can only be a multi-word catalog action — `word_matches` is
        // single-token by construction and `CatalogSlot::Verb` reaches only
        // `action_matches`.
        let longest = matches.iter().map(|item| item.end).max().unwrap_or(start);
        if longest > start + 1 {
            for item in &mut matches {
                if item.end < longest {
                    item.local_cost.precedence += 1;
                }
            }
        }
        matches
    }

    pub(super) fn scan_clause_lexical(
        &self,
        slot: EnglishLexicalSlot,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        match slot {
            EnglishLexicalSlot::PossessiveNoun => self.scan_possessive_noun(tokens, start),
            EnglishLexicalSlot::OracleSymbol => self
                .magic_match(tokens, start, TokenKind::OracleSymbol)
                .and_then(|(surface, end)| {
                    OracleSymbol::new(surface).map(|symbol| LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::OracleSymbol(symbol),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::SymbolSequence => self
                .magic_match(tokens, start, TokenKind::SymbolSequence)
                .and_then(|(surface, end)| {
                    parse_symbol_sequence(surface).map(|symbols| LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::SymbolSequence(symbols),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::PowerToughness => self
                .magic_match(tokens, start, TokenKind::PowerToughness)
                .and_then(|(surface, end)| {
                    parse_power_toughness(surface).map(|power_toughness| LexicalMatch {
                        end,
                        features: Features::PowerToughness {
                            initial_sound: power_toughness.initial_sound(),
                        },
                        meaning: MeaningKey::PowerToughness(power_toughness),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Punctuation(expected) => tokens
                .get(start)
                .filter(|token| token.kind == TokenKind::Punctuation(expected))
                .map(|_| LexicalMatch {
                    end: start + 1,
                    features: Features::None,
                    meaning: MeaningKey::Punctuation(expected),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Subordinator => self.scan_subordinators(tokens, start),
            EnglishLexicalSlot::RatherThan => self
                .spelling_match(
                    tokens,
                    start,
                    crate::syntax::Subordinator::RatherThan.spelling(),
                )
                .map(|end| LexicalMatch {
                    end,
                    features: Features::None,
                    meaning: MeaningKey::Subordinator(crate::syntax::Subordinator::RatherThan),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Conjunction => self.scan_conjunction(tokens, start, false),
            EnglishLexicalSlot::NounPhraseConjunction => self.scan_conjunction(tokens, start, true),
            slot @ EnglishLexicalSlot::Plus => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Plus))
                .into_iter()
                .collect(),
            slot @ EnglishLexicalSlot::Except => self
                .literal_token_match(tokens, start, slot)
                .map(|end| literal_match(end, LiteralKey::Except))
                .into_iter()
                .collect(),
            _ => Vec::new(),
        }
    }

    pub(super) fn has_known_word(&self, tokens: &[Token], start: usize) -> bool {
        if tokens
            .get(start)
            .is_none_or(|token| token.kind != TokenKind::Word)
        {
            return true;
        }
        let vocabulary_slots = [
            LexicalSlot::Noun(NounUsage::Either),
            LexicalSlot::Adjective,
            LexicalSlot::Adverb,
            LexicalSlot::Pronoun(PronounCase::Subject),
            LexicalSlot::Pronoun(PronounCase::Object),
            LexicalSlot::Auxiliary,
        ];
        vocabulary_slots
            .into_iter()
            .any(|slot| !self.word_matches(tokens, start, slot).is_empty())
            || crate::word::VERB_SLOTS.into_iter().any(|slot| {
                !self
                    .word_matches(tokens, start, LexicalSlot::Verb(slot))
                    .is_empty()
            })
            || !self.scan_determiner(tokens, start).is_empty()
            || !self.scan_preposition(tokens, start).is_empty()
            || !self.scan_conjunction(tokens, start, false).is_empty()
            || self.subordinator_at(tokens, start).is_some()
            || self.token_is_in_recognized_numeral(tokens, start)
            || EnglishLexicalSlot::LITERAL_SLOTS
                .iter()
                .copied()
                .any(|slot| {
                    slot.literal_surfaces()
                        .iter()
                        .enumerate()
                        .any(|(index, literal)| {
                            slot.reserves_literal_for_opacity(index)
                                && self.one_token_match(tokens, start, literal).is_some()
                        })
                })
            || crate::constructions::GROUPS
                .iter()
                .flat_map(|group| group.constructions)
                .flat_map(|construction| construction.forms)
                .flat_map(|form| form.atoms)
                .any(|atom| {
                    let deckmaste_construction_compiler::runtime::AtomData::Literal(literal) = atom
                    else {
                        return false;
                    };
                    literal.len() == 1
                        && literal.as_bytes()[0].is_ascii_uppercase()
                        && self.one_token_match(tokens, start, literal).is_some()
                })
            || !self
                .catalog_matches(tokens, start, CatalogSlot::Noun(NounUsage::Either))
                .is_empty()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::Adjective)
                .is_empty()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::AbilityItem)
                .is_empty()
            || crate::word::VERB_SLOTS.into_iter().any(|slot| {
                !self
                    .catalog_matches(tokens, start, CatalogSlot::Verb(slot))
                    .is_empty()
            })
    }

    pub(super) fn scan_subordinators(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        self.subordinator_at(tokens, start)
            .map(|(end, subordinator)| LexicalMatch {
                end,
                features: Features::Subordinator(subordinator),
                meaning: MeaningKey::Subordinator(subordinator),
                local_cost: ParseCost::default(),
            })
            .into_iter()
            .collect()
    }

    pub(super) fn subordinator_at(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Option<(usize, crate::syntax::Subordinator)> {
        crate::syntax::Subordinator::FORMS
            .iter()
            .filter(|(subordinator, _)| {
                !matches!(
                    subordinator,
                    crate::syntax::Subordinator::When
                        | crate::syntax::Subordinator::Because
                        | crate::syntax::Subordinator::RatherThan
                )
            })
            .find_map(|(subordinator, spelling)| {
                self.spelling_match(tokens, start, spelling)
                    .map(|end| (end, *subordinator))
            })
    }

    pub(super) fn scan_existential(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        ExistentialForm::FORMS
            .iter()
            .filter_map(|(form, spelling)| {
                self.spelling_match(tokens, start, spelling)
                    .map(|end| LexicalMatch {
                        end,
                        features: Features::Existential {
                            number: form.number(),
                        },
                        meaning: MeaningKey::Existential(ExistentialKey {
                            verb_slot: form.verb_slot(),
                        }),
                        local_cost: ParseCost::default(),
                    })
            })
            .collect()
    }

    pub(super) fn scan_subject_auxiliary(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        SUBJECT_AUXILIARY_FORMS
            .iter()
            .flat_map(|(surface_index, subject, auxiliaries)| {
                auxiliaries.iter().filter_map(move |auxiliary| {
                    let surface = SUBJECT_AUXILIARY_SURFACES[*surface_index];
                    self.one_token_match(tokens, start, surface).map(|end| {
                        let PersonNumber { person, number } = subject.agreement();
                        let auxiliary = AuxiliaryInstance {
                            auxiliary: *auxiliary,
                            inflection: AuxiliaryInflection::Present { person, number },
                            contracted_negation: crate::features::Contraction::Full,
                        };
                        LexicalMatch {
                            end,
                            features: Features::SubjectAuxiliary {
                                subject: *subject,
                                agreement: PersonNumber { person, number },
                                auxiliary: auxiliary.into(),
                            },
                            meaning: MeaningKey::SubjectAuxiliary(SubjectAuxiliaryKey {
                                subject: *subject,
                                auxiliary: auxiliary.into(),
                            }),
                            local_cost: ParseCost {
                                precedence: u32::from(
                                    auxiliary.auxiliary == Auxiliary::Have
                                        && surface.ends_with("'s"),
                                ),
                                ..ParseCost::default()
                            },
                        }
                    })
                })
            })
            .collect()
    }

    pub(super) fn scan_possessive_noun(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens
            .get(start)
            .filter(|token| token.kind == TokenKind::Word)
        else {
            return Vec::new();
        };
        let Some(surface) = token.span.text(self.source) else {
            return Vec::new();
        };
        // One possessive, two spellings. `owner's` is a single word token because
        // the tokenizer glues an apostrophe into a word when a letter follows it;
        // the plural `owners'` ends the word there and leaves the genitive marker
        // as its own bare apostrophe token (`word_end` in `surface.rs`). The
        // s-apostrophe arm therefore consumes two tokens and takes the whole
        // surface as the stem.
        let (stem, end, plural_only) = match surface.strip_suffix("'s") {
            Some(stem) => (stem, start + 1, false),
            None if self.bare_genitive_apostrophe(tokens, start) => (surface, start + 2, true),
            None => return Vec::new(),
        };
        // Without this filter a singular noun whose lemma ends in `s` would match
        // the bare-apostrophe arm and then render back as `…'s`, breaking
        // round-trip. The bare marker is licensed only by a plural `NounInstance`.
        let word_filter = |word: &WordMatch| {
            !plural_only
                || matches!(
                    word,
                    WordMatch::Noun(noun)
                        if matches!(noun.kind(), crate::word::NounInstanceKind::Plural(_))
                )
        };
        let mut matches = Vocabulary::new()
            .matches(stem, LexicalSlot::Noun(NounUsage::Either))
            .into_iter()
            .filter(word_filter)
            .flat_map(|word| lexical_word_matches(word, end))
            .collect::<Vec<_>>();
        matches.extend(
            self.catalogs
                .matches(stem, CatalogSlot::Noun(NounUsage::Either))
                .into_iter()
                .filter(|catalog_match| catalog_match.length == stem.len())
                .flat_map(|catalog_match| match catalog_match.value {
                    CatalogValue::Word(word) if word_filter(&word) => {
                        lexical_word_matches(word, end)
                    }
                    CatalogValue::Word(_) | CatalogValue::Atom(_) => Vec::new(),
                }),
        );
        // A possessive vocabulary noun renders its stem lowercase (`fang's`), so
        // reading a capitalized nickname's possessive (`Fang's`) as one corrupts
        // the printed name. Dispreference such readings when the nickname collides
        // here so the case-preserving
        // [`PossessiveThisCard`](EnglishLexicalSlot::PossessiveThisCard)
        // self-reference wins; this is the possessive counterpart of the same
        // penalty in `word_matches` and `catalog_matches`.
        if self.nickname_lowercasing_collision(tokens, start) {
            let dispreference = ParseCost {
                reading_dispreference: 3,
                ..ParseCost::default()
            };
            for candidate in &mut matches {
                candidate.local_cost += dispreference;
            }
        }
        matches
    }

    /// Whether the token after the word at `start` is the bare genitive
    /// apostrophe of an s-plural possessive (`owners'`). The apostrophe must be
    /// a punctuation token butted directly against the word — the tokenizer
    /// never emits it as part of the word, and no space intervenes in the
    /// canonical templates — and the word must end in `s`, which is the only
    /// spelling the bare marker attaches to.
    pub(super) fn bare_genitive_apostrophe(&self, tokens: &[Token], start: usize) -> bool {
        let Some(word) = tokens.get(start) else {
            return false;
        };
        let Some(surface) = word.span.text(self.source) else {
            return false;
        };
        if !surface.ends_with('s') {
            return false;
        }
        tokens.get(start + 1).is_some_and(|marker| {
            marker.kind == TokenKind::Punctuation(Punctuation::Apostrophe)
                && marker.span.start == word.span.end
        })
    }

    pub(super) fn magic_match<'grammar>(
        &'grammar self,
        tokens: &[Token],
        start: usize,
        kind: TokenKind,
    ) -> Option<(&'grammar str, usize)> {
        let token = tokens.get(start).filter(|token| token.kind == kind)?;
        Some((token.span.text(self.source)?, start + 1))
    }

    pub(super) fn scan_conjunction(
        &self,
        tokens: &[Token],
        start: usize,
        allow_plus: bool,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        use crate::features::Conjunction;

        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(conjunction) = Conjunction::from_spelling(surface) else {
            return Vec::new();
        };
        if conjunction == Conjunction::Plus && !allow_plus {
            return Vec::new();
        }
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Conjunction(conjunction),
            meaning: MeaningKey::Conjunction(conjunction),
            local_cost: ParseCost::default(),
        }]
    }

    pub(super) fn scan_determiner(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(determiner) = Determiner::from_spelling(surface) else {
            return Vec::new();
        };
        // `The` heading a multi-word nickname (e.g. `The Beast`) also parses as
        // the lowercase `the` determiner leading an opaque noun; that reading
        // lowercases the name's first word. Dispreference it so the nickname,
        // which reproduces the capitalized `The`, wins.
        let local_cost = if matches!(determiner.kind(), crate::syntax::DeterminerKind::The)
            && self.nickname_lowercasing_collision(tokens, start)
        {
            ParseCost {
                reading_dispreference: 3,
                ..ParseCost::default()
            }
        } else {
            ParseCost::default()
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Determiner {
                cardinality: determiner.noun_cardinality(),
                // The scanned word itself, not `determiner` (which no longer
                // carries it — see `DeterminerKind::Indefinite`): this feature
                // exists only to gate the parse against the following
                // material's initial sound in `article_accepts`.
                article: IndefiniteArticle::from_spelling(surface),
                demonstrative_this: matches!(
                    determiner.kind(),
                    crate::syntax::DeterminerKind::Demonstrative(
                        crate::syntax::Demonstrative::This
                    )
                ),
                set_exception_host: matches!(
                    determiner.kind(),
                    crate::syntax::DeterminerKind::All | crate::syntax::DeterminerKind::Each
                ),
            },
            meaning: MeaningKey::Determiner(determiner),
            local_cost,
        }]
    }

    pub(super) fn scan_demonstrative(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(demonstrative) = Demonstrative::from_spelling(surface) else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::NounPhrase {
                agreement: Some(PersonNumber {
                    person: Person::Third,
                    number: demonstrative.number(),
                }),
                coordination_domain: Some(CoordinationDomain::Entity),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: super::NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            },
            meaning: MeaningKey::Determiner(
                crate::constructions::determiner::build_determiner_closed(
                    crate::syntax::ClosedDeterminer::Demonstrative(demonstrative),
                )
                .expect("every demonstrative is a closed determiner identity"),
            ),
            local_cost: ParseCost::default(),
        }]
    }

    /// Scans a quoted ability (`"…"`) as one lexical unit. Matches only when a
    /// double quote opens at `start` and a later double quote closes it with a
    /// non-empty interior; the match spans both delimiters and carries the
    /// interior's source span, reparsed to an
    /// [`Ability`](crate::syntax::Ability) at lowering. Earley prediction
    /// restricts the scan to positions where a grant/coordination rule
    /// expects an object, so the interior tokens never invite a scan of
    /// their own.
    pub(super) fn scan_quoted_ability(
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        if tokens.get(start).map(|token| token.kind)
            != Some(TokenKind::Punctuation(Punctuation::DoubleQuote))
        {
            return Vec::new();
        }
        let Some(close) = tokens[start + 1..]
            .iter()
            .position(|token| token.kind == TokenKind::Punctuation(Punctuation::DoubleQuote))
            .map(|offset| start + 1 + offset)
        else {
            return Vec::new();
        };
        // The interior must hold at least one token — an empty `""` is never a
        // quoted ability.
        if close == start + 1 {
            return Vec::new();
        }
        let interior_start = tokens[start + 1].span.start;
        let interior_end = tokens[close - 1].span.end;
        vec![LexicalMatch {
            end: close + 1,
            features: Features::None,
            meaning: MeaningKey::QuotedAbility(Span::new(interior_start, interior_end)),
            local_cost: ParseCost::default(),
        }]
    }

    pub(super) fn scan_frequency(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let (bound, count_start) =
            if let Some(end) = self.words_match(tokens, start, &["no", "more", "than"]) {
                (FrequencyBound::NoMoreThan, end)
            } else if let Some(end) = self.words_match(tokens, start, &["more", "than"]) {
                (FrequencyBound::MoreThan, end)
            } else {
                return Vec::new();
            };
        if let Some(end) = self.one_token_match(tokens, count_start, "once") {
            return vec![frequency_match(
                end,
                FrequencyPhrase {
                    bound,
                    count: FrequencyCount::Once,
                },
            )];
        }
        if let Some(end) = self.one_token_match(tokens, count_start, "twice") {
            return vec![frequency_match(
                end,
                FrequencyPhrase {
                    bound,
                    count: FrequencyCount::Twice,
                },
            )];
        }
        let Some(surface) = self.token_text(tokens, count_start) else {
            return Vec::new();
        };
        let Some(end) = self.one_token_match(tokens, count_start + 1, "times") else {
            return Vec::new();
        };
        let sentence_initial = Self::is_sentence_initial(tokens, count_start);
        [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            parse_notation(notation, surface, sentence_initial).map(|value| {
                frequency_match(
                    end,
                    FrequencyPhrase {
                        bound,
                        count: FrequencyCount::Times(NumberLiteral {
                            value,
                            numeral: notation,
                        }),
                    },
                )
            })
        })
        .collect()
    }

    pub(super) fn scan_preposition(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(preposition) = Preposition::from_spelling(surface) else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Preposition(preposition),
            meaning: MeaningKey::Preposition(preposition),
            local_cost: ParseCost::default(),
        }]
    }
}

pub(super) fn lexical_word_matches(
    word: WordMatch,
    end: usize,
) -> Vec<LexicalMatch<Features, MeaningKey>> {
    let single = |features, meaning| {
        vec![LexicalMatch {
            end,
            features,
            meaning,
            local_cost: ParseCost::default(),
        }]
    };
    match word {
        WordMatch::Noun(noun) => {
            let form = noun_form(&noun);
            let Some(initial_sound) = noun_initial_sound(&noun) else {
                return Vec::new();
            };
            let adjunct = noun_adjunct_kind(&noun);
            let coordination_domain = noun_coordination_domain(&noun);
            vec![LexicalMatch {
                end,
                features: Features::Noun {
                    identity: noun_coordination_head(&noun),
                    coordination_domain,
                    form,
                    initial_sound,
                    adjunct,
                    opaque: false,
                    recipient_passive_theme: matches!(noun.noun(), Noun::Word(Vocab::Damage)),
                },
                // `other` is scanned as a count noun only so the anaphoric
                // fused head `the other`/`the others` (the sibling of `the
                // rest`) can head a nominal. Everywhere else `other` is an
                // attributive adjective (`each other creature`, `all other
                // permanents`); dispreference the noun reading so the adjective
                // reading — equal in every structural cost — always wins in
                // modifier position, and the noun reading surfaces only as the
                // fused head, where no adjective reading completes.
                //
                // `nearest` is likewise a fused superlative head (`choose the
                // nearest`) or an attributive adjective (`the nearest
                // opponent`).
                //
                // `one` is the same shape against the number-literal reading:
                // it is scanned as a count noun so the anaphoric fused head
                // (`a different one of those creatures`, `each one`) can head a
                // nominal, but dispreferenced so a competing `NumberLiteral`
                // quantity reading (`one card`, `one or more counters`) always
                // wins wherever both complete.
                // Productive agent nouns are dispreferred against explicit
                // lexical nouns. Thus technical words such as `player`,
                // `controller`, and `owner` keep their lexical identity, while
                // an unclaimed surface such as `voter` is verb-backed.
                //
                // Singular `target` is also a genuine noun (`any target`), but
                // its bare reading must lose when the rules determiner can head
                // the same complete selection (`target enchanted creature`).
                // Plural `targets` is unambiguous and pays no cost.
                local_cost: ParseCost {
                    reading_dispreference: u32::from(is_fused_head_noun(&noun))
                        + u32::from(is_derived_agent_noun(&noun))
                        + u32::from(matches!(
                            noun.kind(),
                            crate::word::NounInstanceKind::Singular(Noun::Word(Vocab::Target))
                        )),
                    ..ParseCost::default()
                },
                meaning: MeaningKey::Noun(noun),
            }]
        }
        WordMatch::Verb(verb) => verb
            .verb
            .predicate_frames()
            .iter()
            .copied()
            .map(|frame| LexicalMatch {
                end,
                features: Features::Verb {
                    slot: verb.slot,
                    frame,
                    head_is_copular: verb.verb == Verb::Word(Vocab::Be),
                    // `control` and `own` take objects in the rules sense
                    // [CR#109.1]. A mass noun such as damage cannot fill that
                    // object gap [CR#120.1]. Keep this lexical fact on the
                    // grammar item so attachment can reject an impossible
                    // antecedent before lowering erases the competing parse.
                    object_gap_requires_rules_object: matches!(
                        verb.verb,
                        Verb::Word(Vocab::Control | Vocab::Own)
                    ),
                },
                meaning: MeaningKey::Verb(VerbAnalysis {
                    instance: verb.clone(),
                    frame,
                }),
                local_cost: ParseCost::default(),
            })
            .collect(),
        WordMatch::Adjective(adjective) => {
            let Some(features) = adjective_features(&adjective, false) else {
                return Vec::new();
            };
            let target_modifier = matches!(&adjective, Adjective::Word(Vocab::Target));
            let mut matches = single(features, MeaningKey::Adjective(adjective));
            if target_modifier {
                // Keep the adjective reading for `the target creature`, where
                // the leading article rules out a second determiner, but let
                // an overt repeated `target` start its own selected noun phrase
                // whenever both readings complete.
                matches[0].local_cost.reading_dispreference += 1;
            }
            matches
        }
        WordMatch::Adverb(adverb) | WordMatch::SentenceAdverbial(adverb) => {
            single(Features::None, MeaningKey::Adverb(adverb))
        }
        WordMatch::Pronoun(pronoun) => single(
            noun_phrase_features(pronoun.pronoun, Some(pronoun.case)),
            MeaningKey::Pronoun(pronoun),
        ),
        WordMatch::Auxiliary(auxiliary) => vec![LexicalMatch {
            end,
            features: Features::auxiliary(auxiliary),
            meaning: MeaningKey::Auxiliary(auxiliary.into()),
            local_cost: ParseCost {
                // `were`/`weren't` are ambiguous between indicative
                // Past{Third,Plural} and the subjunctive: prefer the
                // indicative reading whenever both are available (e.g. `they
                // were untapped`), leaving subjunctive as the only surviving
                // reading where no indicative subject agrees (`it were`).
                reading_dispreference: u32::from(matches!(
                    auxiliary.inflection,
                    crate::word::AuxiliaryInflection::PastSubjunctive
                )),
                ..ParseCost::default()
            },
        }],
    }
}

fn noun_coordination_head(noun: &NounInstance) -> Option<crate::catalog::CatalogAtom> {
    match noun.noun() {
        Noun::Catalog(atom) => Some(atom.clone()),
        _ => None,
    }
}

fn noun_coordination_domain(noun: &NounInstance) -> Option<CoordinationDomain> {
    match noun.noun() {
        Noun::Catalog(atom) => match atom.kind {
            crate::catalog::CatalogKind::ArtifactType
            | crate::catalog::CatalogKind::BattleType
            | crate::catalog::CatalogKind::CreatureType
            | crate::catalog::CatalogKind::EnchantmentType
            | crate::catalog::CatalogKind::LandType
            | crate::catalog::CatalogKind::PlaneswalkerType
            | crate::catalog::CatalogKind::SpellType
            | crate::catalog::CatalogKind::Supertype
            | crate::catalog::CatalogKind::CardType => Some(CoordinationDomain::Entity),
            crate::catalog::CatalogKind::KeywordAbility => Some(CoordinationDomain::NonEntity),
            crate::catalog::CatalogKind::KeywordAction
            | crate::catalog::CatalogKind::AbilityWord
            | crate::catalog::CatalogKind::RulesBundle
            | crate::catalog::CatalogKind::FlavorWord => None,
        },
        Noun::Word(
            Vocab::Card
            | Vocab::Opponent
            | Vocab::Permanent
            | Vocab::Player
            | Vocab::Spell
            | Vocab::Token
            | Vocab::Ability,
        ) => Some(CoordinationDomain::Entity),
        Noun::Word(Vocab::Counter) => Some(CoordinationDomain::NonEntity),
        Noun::Word(Vocab::One) => Some(CoordinationDomain::SelectionContinuation),
        Noun::Word(Vocab::Damage) => Some(CoordinationDomain::Damage),
        Noun::Word(Vocab::Power) => Some(CoordinationDomain::Power),
        Noun::Word(Vocab::Toughness) => Some(CoordinationDomain::Toughness),
        Noun::Word(_) | Noun::Agentive(_) | Noun::Gerund(_) | Noun::Die(_) | Noun::Opaque(_) => {
            None
        }
    }
}

pub(super) fn frequency_match(
    end: usize,
    frequency: FrequencyPhrase,
) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::None,
        meaning: MeaningKey::Frequency(frequency),
        local_cost: ParseCost::default(),
    }
}

pub(super) fn literal_match(end: usize, literal: LiteralKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::None,
        meaning: MeaningKey::Literal(literal),
        local_cost: ParseCost::default(),
    }
}

pub(super) fn pronoun_match(
    end: usize,
    pronoun: Pronoun,
    case: PronounCase,
) -> LexicalMatch<Features, MeaningKey> {
    let instance = PronounInstance { pronoun, case };
    LexicalMatch {
        end,
        features: noun_phrase_features(pronoun, Some(case)),
        meaning: MeaningKey::Pronoun(instance),
        local_cost: ParseCost::default(),
    }
}

/// A self-reference resolves to one card, but a joint `and` face (`Aang and
/// Katara`) names two creatures, and its text agrees plurally (`When Aang and
/// Katara enter, …`). The recognized name carries no number, so both
/// third-person agreements are offered; the verb's own inflection selects one
/// (`… enters` singular, `… enter` plural). The lowered [`ThisCardForm`] is
/// identical either way — the number lives on the verb — so an
/// agreement-neutral verb packs to one AST.
pub(super) fn this_card_matches(
    end: usize,
    form: ThisCardForm,
) -> Vec<LexicalMatch<Features, MeaningKey>> {
    [Number::Singular, Number::Plural]
        .into_iter()
        .map(|number| LexicalMatch {
            end,
            features: Features::NounPhrase {
                agreement: Some(PersonNumber {
                    person: Person::Third,
                    number,
                }),
                coordination_domain: Some(CoordinationDomain::Entity),
                pronoun_case: None,
                adjunct: None,
                set_exception: SetExceptionState::Ineligible,
                coordination: super::NounPhraseCoordinationState::None,
                recipient_passive_theme: false,
                rules_object_followup: false,
            },
            meaning: MeaningKey::ThisCard(form),
            local_cost: this_card_cost(form),
        })
        .collect()
}

pub(super) fn possessive_this_card_match(
    end: usize,
    form: ThisCardForm,
) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::PossessiveThisCard {
            agreement: PersonNumber {
                person: Person::Third,
                number: Number::Singular,
            },
        },
        meaning: MeaningKey::ThisCard(form),
        local_cost: this_card_cost(form),
    }
}

/// The tiebreak cost of reading tokens as a self-reference. A
/// non-self-reference reading pays nothing here and so wins any tie on the
/// structural fields; among self-references the fuller
/// [`ThisCardForm::FullName`] outranks a derived nickname.
pub(super) fn this_card_cost(form: ThisCardForm) -> ParseCost {
    ParseCost {
        reading_dispreference: match form {
            ThisCardForm::FullName => 1,
            ThisCardForm::AbbreviatedName => 2,
        },
        ..ParseCost::default()
    }
}

pub(super) fn noun_phrase_features(pronoun: Pronoun, case: Option<PronounCase>) -> Features {
    let agreement = match pronoun {
        Pronoun::You => Some(PersonNumber {
            person: Person::Second,
            number: Number::Singular,
        }),
        Pronoun::It(_) => Some(PersonNumber {
            person: Person::Third,
            number: Number::Singular,
        }),
        Pronoun::They => Some(PersonNumber {
            person: Person::Third,
            number: Number::Plural,
        }),
        Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute => None,
    };
    Features::NounPhrase {
        agreement,
        coordination_domain: Some(CoordinationDomain::Entity),
        pronoun_case: case,
        adjunct: None,
        set_exception: SetExceptionState::Ineligible,
        coordination: super::NounPhraseCoordinationState::None,
        recipient_passive_theme: false,
        rules_object_followup: false,
    }
}

pub(super) const fn copula_agreement(
    auxiliary: super::AuxiliaryFeatures,
) -> Option<CopulaInflection> {
    if !matches!(auxiliary.auxiliary, Auxiliary::Be) {
        return None;
    }
    match auxiliary.inflection {
        AuxiliaryInflection::Present { person, number }
        | AuxiliaryInflection::Past { person, number } => {
            Some(CopulaInflection::Indicative(PersonNumber {
                person,
                number,
            }))
        }
        AuxiliaryInflection::PastSubjunctive => Some(CopulaInflection::PastSubjunctive),
        AuxiliaryInflection::Base
        | AuxiliaryInflection::PresentParticiple
        | AuxiliaryInflection::PastParticiple => None,
    }
}

pub(super) fn noun_form(noun: &NounInstance) -> NounForm {
    match noun.kind() {
        crate::word::NounInstanceKind::Singular(_) => NounForm::Singular,
        crate::word::NounInstanceKind::Plural(_) => NounForm::Plural,
        crate::word::NounInstanceKind::Mass(_) => NounForm::Mass,
    }
}

/// Whether this noun is a dual-reading lexeme scanned as a noun purely to
/// license an anaphoric fused head (`the other`/`the others`, `the nearest`, `a
/// different one of those creatures`, `each one`). Its noun reading is
/// dispreferenced so the competing reading — the attributive adjective for
/// `other`/`nearest`, the `NumberLiteral` quantity for `one` — wins wherever
/// both complete; see [`lexical_word_matches`].
pub(super) fn is_fused_head_noun(noun: &NounInstance) -> bool {
    matches!(
        noun.noun(),
        Noun::Word(Vocab::Nearest | Vocab::One | Vocab::Other)
    )
}

pub(super) fn is_derived_agent_noun(noun: &NounInstance) -> bool {
    matches!(noun.noun(), Noun::Agentive(_))
}

pub(super) fn noun_adjunct_kind(noun: &NounInstance) -> Option<BareNominalAdjunct> {
    noun.noun().bare_nominal_adjunct()
}

pub(super) fn parse_symbol_sequence(source: &str) -> Option<Vec<OracleSymbol>> {
    let mut rest = source;
    let mut symbols = Vec::new();
    while let Some(close) = rest.find('}') {
        let end = close + 1;
        symbols.push(OracleSymbol::new(rest.get(..end)?)?);
        rest = rest.get(end..)?;
    }
    (rest.is_empty() && !symbols.is_empty()).then_some(symbols)
}

pub(super) fn noun_initial_sound(noun: &NounInstance) -> Option<InitialSound> {
    let noun_identity = noun.noun();
    match noun_identity {
        Noun::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        Noun::Catalog(atom) => Some(surface_initial_sound(atom.canonical())),
        Noun::Die(_) => Some(InitialSound::Consonant),
        Noun::Gerund(_) | Noun::Agentive(_) => Vocabulary::new()
            .render_noun(noun)
            .map(|surface| surface_initial_sound(&surface)),
        Noun::Opaque(opaque) => Some(surface_initial_sound(opaque.spelling())),
    }
}

pub(super) fn adjective_initial_sound(adjective: &Adjective) -> Option<InitialSound> {
    match adjective {
        Adjective::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        _ => Vocabulary::new()
            .render_adjective(adjective)
            .map(|surface| surface_initial_sound(&surface)),
    }
}

pub(crate) fn adjective_comparison_state(adjective: &Adjective) -> AdjectiveComparisonState {
    match adjective {
        // Comparison capability is vocabulary metadata (`Vocab::comparison`),
        // not a spelling match; the class it carries decides which completions
        // are legal (see `AdjectiveComparisonState`).
        Adjective::Word(word) => word
            .comparison()
            .map_or(AdjectiveComparisonState::NotComparative, |comparison| {
                AdjectiveComparisonState::Pending(comparison.class())
            }),
        _ => AdjectiveComparisonState::NotComparative,
    }
}

pub(crate) fn adjective_features(
    adjective: &Adjective,
    card_orientation: bool,
) -> Option<Features> {
    Some(Features::Adjective {
        initial_sound: adjective_initial_sound(adjective)?,
        comparison: adjective_comparison_state(adjective),
        card_orientation,
        infinitive_complement: adjective.licenses_complement(AdjectiveComplementKind::Infinitive),
        past_participle: matches!(
            adjective,
            Adjective::Participle(crate::word::Tense::Past, _)
        ),
        demonstrative_shared_determiner: !matches!(
            adjective,
            Adjective::Participle(_, Verb::Word(Vocab::Equip | Vocab::Enchant),)
        ),
    })
}

pub(super) fn parse_power_toughness(surface: &str) -> Option<PowerToughness> {
    let (power, toughness) = surface.split_once('/')?;
    Some(PowerToughness {
        power: parse_signed_scalar(power)?,
        toughness: parse_signed_scalar(toughness)?,
    })
}

pub(super) fn parse_signed_scalar(surface: &str) -> Option<crate::syntax::SignedScalar> {
    use crate::syntax::ScalarSign;
    use crate::syntax::ScalarValue;

    let (sign, body) = if let Some(body) = surface.strip_prefix('+') {
        (ScalarSign::Plus, body)
    } else if let Some(body) = surface
        .strip_prefix('-')
        .or_else(|| surface.strip_prefix('−'))
    {
        (ScalarSign::Minus, body)
    } else {
        (ScalarSign::None, surface)
    };
    let value = match body {
        "X" => ScalarValue::X,
        "*" => ScalarValue::Star,
        _ => ScalarValue::Integer(body.parse().ok()?),
    };
    Some(crate::syntax::SignedScalar { sign, value })
}
