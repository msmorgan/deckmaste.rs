use super::*;
use crate::syntax::VerbDependent;

pub(super) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Expected::Nonterminal as n;
    use Nonterminal as N;

    for form in [NounForm::Singular, NounForm::Plural, NounForm::Mass] {
        builder.add(
            RuleTag::NounUnknown,
            N::Noun,
            [l(L::Unknown(RecoverySlot::Noun(form)))],
        );
    }
    builder.add(
        RuleTag::NominalUnknownModifier,
        N::Nominal,
        [l(L::Unknown(RecoverySlot::NominalModifier)), n(N::Nominal)],
    );
    builder.add(
        RuleTag::VerbPhraseUnknownDependent,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::Unknown(RecoverySlot::VerbDependent))],
    );
    builder.add(
        RuleTag::PrepositionalPhraseUnknownObject,
        N::PrepositionalPhrase,
        [
            l(L::Preposition),
            l(L::Unknown(RecoverySlot::PrepositionObject)),
        ],
    );
}

pub(super) fn add_unknown_verb_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Nonterminal as N;

    for slot in crate::word::VERB_SLOTS {
        builder.add(
            RuleTag::VerbUnknown,
            N::Verb,
            [l(L::Unknown(RecoverySlot::Verb(slot)))],
        );
    }
}

pub(super) fn scan_unknown(
    source: &str,
    tokens: &[Token],
    start: usize,
    slot: RecoverySlot,
    allow_bare_verb: bool,
) -> Vec<LexicalMatch<Features, MeaningKey>> {
    let Some(first) = tokens.get(start) else {
        return Vec::new();
    };
    if matches!(slot, RecoverySlot::Noun(_) | RecoverySlot::Verb(_))
        && first.kind != TokenKind::Word
    {
        return Vec::new();
    }
    if let RecoverySlot::Verb(verb_slot) = slot {
        let Some(surface) = first.span.text(source) else {
            return Vec::new();
        };
        if !unknown_verb_surface_accepts(surface, verb_slot, allow_bare_verb) {
            return Vec::new();
        }
    }
    let mut matches = Vec::new();
    let mut nesting = Nesting::default();
    let mut words = 0_u32;

    for (offset, token) in tokens[start..].iter().enumerate() {
        if nesting.is_top_level() && is_boundary(token.kind) {
            break;
        }
        words = words.saturating_add(u32::from(is_word_like(token.kind)));
        nesting.observe(token.kind);
        let end = start + offset + 1;
        if words == 0 || !nesting.is_top_level() && end != tokens.len() {
            continue;
        }
        let span = Span::new(first.span.start, token.span.end);
        let initial_sound = span
            .text(source)
            .map_or(InitialSound::Consonant, surface_initial_sound);
        let features = match slot {
            RecoverySlot::Noun(form) => Features::Noun {
                form,
                initial_sound,
            },
            RecoverySlot::Verb(verb) => Features::Verb(verb),
            RecoverySlot::NominalModifier
            | RecoverySlot::VerbDependent
            | RecoverySlot::PrepositionObject => Features::UnknownPhrase { initial_sound },
        };
        matches.push(LexicalMatch {
            end,
            features,
            meaning: MeaningKey::Unknown(UnknownKey { slot, span }),
            local_cost: ParseCost {
                unknown_words: words,
                recoveries: 1,
                ..ParseCost::default()
            },
        });
    }
    match slot {
        RecoverySlot::Noun(_) | RecoverySlot::Verb(_) => {
            matches.truncate(1);
            matches
        }
        RecoverySlot::NominalModifier
        | RecoverySlot::VerbDependent
        | RecoverySlot::PrepositionObject => matches,
    }
}

pub(super) fn reduce_recovery(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
    shape: u64,
) -> Option<Reduced> {
    match tag {
        RuleTag::NounUnknown | RuleTag::VerbUnknown => Some(propagate(children.first()?)),
        RuleTag::NominalUnknownModifier => {
            let Features::UnknownPhrase { initial_sound } = children.first()?.features else {
                return None;
            };
            let Features::Nominal {
                leading_recovery, ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *leading_recovery {
                return None;
            }
            nominal_with_prefix(children.get(1)?, *initial_sound, true, shape)
        }
        RuleTag::VerbPhraseUnknownDependent => {
            let Features::VerbPhrase {
                form,
                has_direct_object,
                trailing_recovery,
            } = children.first()?.features
            else {
                return None;
            };
            if *trailing_recovery {
                return None;
            }
            Some((
                Features::VerbPhrase {
                    form: *form,
                    has_direct_object: *has_direct_object,
                    trailing_recovery: true,
                },
                MeaningKey::VerbPhrase { form: *form, shape },
            ))
        }
        RuleTag::PrepositionalPhraseUnknownObject => {
            let MeaningKey::Preposition(preposition) = children.first()?.meaning else {
                return None;
            };
            Some((
                Features::PrepositionalPhrase,
                MeaningKey::PrepositionalPhrase {
                    preposition: *preposition,
                    shape,
                },
            ))
        }
        _ => None,
    }
}

pub(super) fn lower_recovery(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounUnknown | RuleTag::VerbUnknown => take(children, 0),
        RuleTag::NominalUnknownModifier => {
            let Lowered::Unknown(unknown) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Unknown(unknown));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::VerbPhraseUnknownDependent => {
            let Lowered::VerbPhrase(mut verb) = take(children, 0)? else {
                return None;
            };
            let Lowered::Unknown(unknown) = take(children, 1)? else {
                return None;
            };
            verb.dependents.push(VerbDependent::Unknown(unknown));
            Some(Lowered::VerbPhrase(verb))
        }
        RuleTag::PrepositionalPhraseUnknownObject => {
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::Unknown(object) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase {
                preposition,
                object: Box::new(Phrase::UnknownPhrase(object)),
            }))
        }
        _ => None,
    }
}

fn is_boundary(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Newline
            | TokenKind::Bullet
            | TokenKind::Punctuation(
                Punctuation::Period
                    | Punctuation::Exclamation
                    | Punctuation::Question
                    | Punctuation::Comma
                    | Punctuation::Colon
                    | Punctuation::Semicolon
                    | Punctuation::EmDash,
            )
    )
}

const fn is_word_like(kind: TokenKind) -> bool {
    !matches!(
        kind,
        TokenKind::Punctuation(_) | TokenKind::Newline | TokenKind::Bullet
    )
}

fn unknown_verb_surface_accepts(surface: &str, slot: VerbSlot, allow_ambiguous_bare: bool) -> bool {
    let surface = surface.to_ascii_lowercase();
    match slot {
        VerbSlot::PresentParticiple => surface.ends_with("ing"),
        VerbSlot::PastParticiple | VerbSlot::Past { .. } => surface.ends_with("ed"),
        VerbSlot::Present {
            person: Person::Third,
            number: Number::Singular,
        } => surface.ends_with('s') && !surface.ends_with("ss"),
        VerbSlot::Infinitive
        | VerbSlot::Imperative
        | VerbSlot::Present {
            person: Person::Second,
            ..
        }
        | VerbSlot::Present {
            person: Person::Third,
            number: Number::Plural,
        } => {
            allow_ambiguous_bare
                && !surface.ends_with("ing")
                && !surface.ends_with("ed")
                && !surface.ends_with('s')
        }
    }
}

#[derive(Default)]
struct Nesting {
    brackets: usize,
    parentheses: usize,
    quoted: bool,
}

impl Nesting {
    const fn is_top_level(&self) -> bool {
        self.brackets == 0 && self.parentheses == 0 && !self.quoted
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
            TokenKind::Punctuation(Punctuation::DoubleQuote) => self.quoted = !self.quoted,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::scan_unknown;
    use crate::syntax::Ability;
    use crate::syntax::AbilityKind;
    use crate::syntax::Clause;
    use crate::syntax::Determiner;
    use crate::syntax::IndefiniteArticle;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::VerbDependent;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::Vocab;

    #[test]
    fn exact_parse_wins_without_constructing_recovery_edges() {
        let parsed = parse("Draw a card.");
        assert_eq!(parsed.recovery_mode(), RecoveryMode::Exact);
        assert_eq!(parsed.cost(), ParseCost::default());
        assert!(
            !parsed
                .chart
                .forest
                .nodes()
                .any(|node| matches!(node.key.meaning, MeaningKey::Unknown(_)))
        );
        assert!(unknowns(parsed.sentence().unwrap()).is_empty());
    }

    #[test]
    fn unknown_noun_is_recovered_inside_the_object_nominal() {
        let parsed = parse("Draw a blorple.");
        assert_eq!(parsed.recovery_mode(), RecoveryMode::UnknownPhrases);
        assert_eq!(
            parsed.cost(),
            ParseCost {
                unknown_words: 1,
                recoveries: 1,
                ..ParseCost::default()
            }
        );

        let Clause::Simple(clause) = &parsed.sentence().unwrap().clause else {
            panic!("expected a simple clause");
        };
        let [VerbDependent::DirectObject(NounPhrase::Nominal(object))] =
            clause.predicate.dependents.as_slice()
        else {
            panic!("expected one nominal direct object");
        };
        assert_eq!(
            object.determiner,
            Some(Determiner::Indefinite(IndefiniteArticle::A))
        );
        assert!(matches!(
            &object.head,
            NounInstance::Singular(Noun::Unknown(unknown)) if unknown.0 == "blorple"
        ));
        assert!(!parsed.chart.forest.nodes().any(|node| matches!(
            node.key.meaning,
            MeaningKey::Unknown(UnknownKey {
                slot: RecoverySlot::Verb(_),
                ..
            })
        )));
    }

    #[test]
    fn unknown_past_verbs_require_regular_past_morphology() {
        let slot = RecoverySlot::Verb(VerbSlot::Past {
            person: Person::Third,
            number: Number::Singular,
        });
        for (source, expected) in [("frobnitzed", 1), ("card", 0), ("target", 0)] {
            let surface = lex(source);
            assert_eq!(
                scan_unknown(source, &surface.tokens, 0, slot, false).len(),
                expected,
                "{source}",
            );
        }
    }

    #[test]
    fn generic_phrase_recovery_does_not_start_on_known_vocabulary() {
        let source = "each nonland permanent";
        let surface = lex(source);
        let catalogs = Catalogs::default();
        let grammar = EnglishGrammar::with_recovery_profile(
            source,
            &catalogs,
            Nonterminal::VerbPhrase,
            RecoveryProfile::Phrases,
        );
        let matches = crate::chart::Grammar::scan(
            &grammar,
            EnglishLexicalSlot::Unknown(RecoverySlot::VerbDependent),
            &surface.tokens,
            0,
        );

        assert!(matches.is_empty());
    }

    #[test]
    fn inflected_unknown_verb_competes_with_phrase_recovery() {
        let catalogs =
            Catalogs::default().with_catalog(crate::catalog::CatalogKind::CardType, ["Creature"]);
        let parsed = parse_nonterminal(
            "Target creature frobnitzes a card.",
            &catalogs,
            Nonterminal::Sentence,
        )
        .expect("an unknown finite verb should be recoverable");

        assert_eq!(parsed.recovery_mode(), RecoveryMode::UnknownPhrases);
        assert_eq!(parsed.cost().unknown_words, 1);
        assert_eq!(parsed.cost().recoveries, 1);
        let Clause::Simple(clause) = &parsed.sentence().unwrap().clause else {
            panic!("expected a simple clause");
        };
        assert!(matches!(
            clause.subject,
            Some(crate::syntax::Subject::NounPhrase(NounPhrase::Nominal(_)))
        ));
        assert!(matches!(
            &clause.predicate.verb.verb,
            crate::word::Verb::Unknown(unknown) if unknown.0 == "frobnitzes"
        ));
    }

    #[test]
    fn morphologically_ambiguous_bare_unknown_verb_remains_recoverable() {
        let parsed = parse("You frobnitz a card.");
        let Clause::Simple(clause) = &parsed.sentence().unwrap().clause else {
            panic!("expected a simple clause");
        };
        assert!(matches!(
            &clause.predicate.verb.verb,
            crate::word::Verb::Unknown(unknown) if unknown.0 == "frobnitz"
        ));
    }

    #[test]
    fn contiguous_unknown_modifier_beats_multiple_smaller_recoveries() {
        let parsed = parse("Draw a shiny strange card.");
        assert_eq!(
            parsed.cost(),
            ParseCost {
                unknown_words: 2,
                recoveries: 1,
                ..ParseCost::default()
            }
        );
        let Clause::Simple(clause) = &parsed.sentence().unwrap().clause else {
            panic!("expected a simple clause");
        };
        let [VerbDependent::DirectObject(NounPhrase::Nominal(object))] =
            clause.predicate.dependents.as_slice()
        else {
            panic!("expected one nominal direct object");
        };
        assert!(matches!(
            object.modifiers.as_slice(),
            [NominalModifier::Unknown(unknown)] if unknown.0 == "shiny strange"
        ));
        assert!(matches!(
            object.head,
            NounInstance::Singular(Noun::Word(Vocab::Card))
        ));
    }

    #[test]
    fn phrase_recovery_offers_each_contiguous_boundary() {
        let source = "blargly a card.";
        let surface = lex(source);
        for slot in [
            RecoverySlot::NominalModifier,
            RecoverySlot::VerbDependent,
            RecoverySlot::PrepositionObject,
        ] {
            let matches = scan_unknown(source, &surface.tokens, 0, slot, false);
            assert_eq!(
                matches
                    .into_iter()
                    .map(|candidate| candidate.end)
                    .collect::<Vec<_>>(),
                vec![1, 2, 3],
                "slot {slot:?}",
            );
        }
    }

    #[test]
    fn magic_atom_stays_known_instead_of_becoming_a_generic_recovery() {
        let catalogs =
            Catalogs::default().with_catalog(crate::catalog::CatalogKind::CardType, ["Creature"]);
        let parsed = parse_nonterminal(
            "Target creature gets +1/+1.",
            &catalogs,
            Nonterminal::Sentence,
        )
        .unwrap();
        assert_eq!(parsed.recovery_mode(), RecoveryMode::Exact);
        let Clause::Simple(clause) = &parsed.sentence().unwrap().clause else {
            panic!("expected a simple clause");
        };
        assert!(clause.predicate.dependents.iter().any(|dependent| matches!(
            dependent,
            VerbDependent::Statistic(crate::syntax::Phrase::PowerToughness(_))
        )));
    }

    fn parse(source: &str) -> ParsedNonterminal {
        parse_nonterminal(source, &Catalogs::default(), Nonterminal::Sentence)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    fn unknowns(sentence: &crate::syntax::Sentence) -> Vec<String> {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    sentences: vec![sentence.clone()],
                }),
            }],
        }
        .unknown_phrases()
        .into_iter()
        .map(|unknown| unknown.text.to_owned())
        .collect()
    }
}
