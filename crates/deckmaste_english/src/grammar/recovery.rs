use super::*;

pub(super) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Nonterminal as N;

    for form in [NounForm::Singular, NounForm::Plural, NounForm::Mass] {
        builder.add(
            RuleTag::NounUnknown,
            N::Noun,
            [l(L::Unknown(RecoverySlot::Noun(form)))],
        );
    }
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
    if first.kind != TokenKind::Word {
        return Vec::new();
    }
    let RecoverySlot::Noun(form) = slot;
    let span = first.span;
    let initial_sound = span
        .text(source)
        .map_or(InitialSound::Consonant, surface_initial_sound);
    vec![LexicalMatch {
        end: start + 1,
        features: Features::Noun {
            form,
            initial_sound,
        },
        meaning: MeaningKey::Unknown(UnknownKey { slot, span }),
        local_cost: ParseCost {
            unknown_words: 1,
            recoveries: 1,
            ..ParseCost::default()
        },
    }]
}

pub(super) fn reduce_recovery(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::NounUnknown => Some(propagate(children.first()?)),
        _ => None,
    }
}

pub(super) fn lower_recovery(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounUnknown => take(children, 0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
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
    fn unknown_nominal_words_recover_only_as_nouns() {
        let parsed = parse("Draw a shiny strange card.");
        let predicate = transitive(parsed.sentence().unwrap());
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &predicate.object else {
            panic!("expected one nominal direct object");
        };
        assert!(
            matches!(
                object.modifiers.as_slice(),
                [
                    NominalModifier::Noun(NounInstance::Singular(Noun::Unknown(shiny))),
                    NominalModifier::Noun(NounInstance::Singular(Noun::Unknown(strange))),
                ] if shiny.0 == "shiny" && strange.0 == "strange"
            ),
            "{:#?}",
            object.modifiers
        );
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
