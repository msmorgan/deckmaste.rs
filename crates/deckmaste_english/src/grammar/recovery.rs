use super::*;

const MAX_UNKNOWN_PHRASE_WORDS: u32 = 3;

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
        RuleTag::PrepositionalPhraseUnknownObject,
        N::PrepositionalPhrase,
        [
            l(L::Preposition),
            l(L::Unknown(RecoverySlot::PrepositionObject)),
        ],
    );
}

pub(super) fn scan_unknown(
    source: &str,
    tokens: &[Token],
    start: usize,
    slot: RecoverySlot,
) -> Vec<LexicalMatch<Features, MeaningKey>> {
    let Some(first) = tokens.get(start) else {
        return Vec::new();
    };
    if matches!(slot, RecoverySlot::Noun(_)) && first.kind != TokenKind::Word {
        return Vec::new();
    }
    let mut matches = Vec::new();
    let mut nesting = Nesting::default();
    let mut words = 0_u32;

    for (offset, token) in tokens[start..].iter().enumerate() {
        if nesting.is_top_level() && is_boundary(token.kind) {
            break;
        }
        words = words.saturating_add(u32::from(is_word_like(token.kind)));
        if words > MAX_UNKNOWN_PHRASE_WORDS {
            break;
        }
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
            RecoverySlot::NominalModifier | RecoverySlot::PrepositionObject => {
                Features::UnknownPhrase { initial_sound }
            }
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
        RecoverySlot::Noun(_) => {
            matches.truncate(1);
            matches
        }
        RecoverySlot::NominalModifier | RecoverySlot::PrepositionObject => matches,
    }
}

pub(super) fn reduce_recovery(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::NounUnknown => Some(propagate(children.first()?)),
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
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                true,
                NominalMeaning::UnknownModifier {
                    modifier: children.first()?.node,
                    nominal: children.get(1)?.node,
                },
            )
        }
        RuleTag::PrepositionalPhraseUnknownObject => {
            if !matches!(children.first()?.meaning, MeaningKey::Preposition(_)) {
                return None;
            }
            Some((
                Features::PrepositionalPhrase,
                MeaningKey::PrepositionalPhrase(PrepositionalPhraseMeaning::UnknownObject {
                    preposition: children.first()?.node,
                    object: children.get(1)?.node,
                }),
            ))
        }
        _ => None,
    }
}

pub(super) fn lower_recovery(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounUnknown => take(children, 0),
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
    use crate::syntax::Determiner;
    use crate::syntax::IndefiniteArticle;
    use crate::syntax::IndependentClause;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::Predicate;
    use crate::syntax::PredicateObject;
    use crate::syntax::SentenceBody;
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

        let predicate = transitive(parsed.sentence().unwrap());
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &predicate.object else {
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
    }

    #[test]
    fn inflected_unknown_verb_recovers_at_the_sentence_boundary() {
        let catalogs =
            Catalogs::default().with_catalog(crate::catalog::CatalogKind::CardType, ["Creature"]);
        let report = crate::parse_with_catalogs("Target creature frobnitzes a card.", &catalogs);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph fallback");
        };
        assert!(
            matches!(
                &paragraph.sentences[0].body,
                SentenceBody::Unknown(unknown) if unknown.0 == "Target creature frobnitzes a card"
            ),
            "{:#?}",
            paragraph.sentences[0].body
        );
    }

    #[test]
    fn morphologically_ambiguous_bare_unknown_verb_recovers_at_the_sentence_boundary() {
        let report = crate::parse("You frobnitz a card.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph fallback");
        };
        assert!(matches!(
            &paragraph.sentences[0].body,
            SentenceBody::Unknown(unknown) if unknown.0 == "You frobnitz a card"
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
        let predicate = transitive(parsed.sentence().unwrap());
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &predicate.object else {
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
    fn phrase_recovery_offers_each_boundary_through_three_words() {
        let source = "blargly a shiny card.";
        let surface = lex(source);
        for slot in [
            RecoverySlot::NominalModifier,
            RecoverySlot::PrepositionObject,
        ] {
            let matches = scan_unknown(source, &surface.tokens, 0, slot);
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
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a transitive clause");
        };
        assert!(matches!(
            predicate.object,
            PredicateObject::PowerToughness(_)
        ));
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

    fn transitive(sentence: &crate::syntax::Sentence) -> &crate::syntax::TransitivePredicate {
        match &sentence.body {
            SentenceBody::Independent(IndependentClause::Transitive(_, predicate))
            | SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
                predicate,
            ))) => predicate,
            other => panic!("expected a transitive clause, got {other:#?}"),
        }
    }
}
