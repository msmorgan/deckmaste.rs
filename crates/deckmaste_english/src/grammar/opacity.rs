#[allow(
    clippy::wildcard_imports,
    reason = "module-level import intentionally reuses generated grammar helpers"
)]
use super::*;

pub(super) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Nonterminal as N;

    for form in [NounForm::Singular, NounForm::Plural, NounForm::Mass] {
        builder.add(
            RuleTag::NounOpaque,
            N::Noun,
            [l(L::Opaque(OpacitySlot::Noun(form)))],
        );
    }
}

pub(super) fn scan_opaque(
    source: &str,
    tokens: &[Token],
    start: usize,
    slot: OpacitySlot,
) -> Vec<LexicalMatch<Features, MeaningKey>> {
    let Some(first) = tokens.get(start) else {
        return Vec::new();
    };
    if first.kind != TokenKind::Word {
        return Vec::new();
    }
    let OpacitySlot::Noun(form) = slot;
    let span = first.span;
    let initial_sound = span
        .text(source)
        .map_or(InitialSound::Consonant, surface_initial_sound);
    vec![LexicalMatch {
        end: start + 1,
        features: Features::Noun {
            form,
            initial_sound,
            adjunct: None,
        },
        meaning: MeaningKey::Opaque(OpaqueKey { slot, span }),
        local_cost: ParseCost {
            opaque_words: 1,
            opaque_lexemes: 1,
            ..ParseCost::default()
        },
    }]
}

pub(super) fn reduce_opacity(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::NounOpaque => Some(propagate(children.first()?)),
        _ => None,
    }
}

pub(super) fn lower_opacity(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounOpaque => take(children, 0),
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
    fn exact_parse_wins_without_constructing_opacity_edges() {
        let parsed = parse("Draw a card.");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(parsed.cost(), ParseCost::default());
        assert!(!parsed.chart.forest.nodes().any(|node| {
            node.key
                .lexical_value()
                .is_some_and(|meaning| matches!(meaning, MeaningKey::Opaque(_)))
        }));
        assert!(opaque_nouns(parsed.sentence().unwrap()).is_empty());
    }

    #[test]
    fn opaque_noun_stays_inside_the_object_nominal() {
        let parsed = parse("Draw a blorple.");
        assert_eq!(parsed.opacity_mode(), OpacityMode::OpaqueNouns);
        assert_eq!(
            parsed.cost(),
            ParseCost {
                opaque_words: 1,
                opaque_lexemes: 1,
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
            NounInstance::Singular(Noun::Opaque(opaque)) if opaque.spelling() == "blorple"
        ));
    }

    #[test]
    fn inflected_unknown_verb_uses_structural_sentence_recovery() {
        let catalogs =
            Catalogs::default().with_catalog(crate::catalog::CatalogKind::CardType, ["Creature"]);
        let report = crate::parse_with_catalogs("Target creature frobnitzes a card.", &catalogs);
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph fallback");
        };
        assert!(
            matches!(
                &paragraph.sentences[0].body,
                SentenceBody::Recovered(recovery)
                    if recovery.spelling() == "Target creature frobnitzes a card"
            ),
            "{:#?}",
            paragraph.sentences[0].body
        );
    }

    #[test]
    fn morphologically_ambiguous_unknown_verb_uses_sentence_recovery() {
        let report = crate::parse("You frobnitz a card.");
        let AbilityKind::Paragraph(paragraph) = &report.ast.abilities[0].kind else {
            panic!("expected paragraph fallback");
        };
        assert!(matches!(
            &paragraph.sentences[0].body,
            SentenceBody::Recovered(recovery) if recovery.spelling() == "You frobnitz a card"
        ));
    }

    #[test]
    fn unknown_nominal_words_become_only_opaque_nouns() {
        let parsed = parse("Draw a shiny strange card.");
        let predicate = transitive(parsed.sentence().unwrap());
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &predicate.object else {
            panic!("expected one nominal direct object");
        };
        assert!(
            matches!(
                object.modifiers.as_slice(),
                [
                    NominalModifier::Noun(NounInstance::Singular(Noun::Opaque(shiny))),
                    NominalModifier::Noun(NounInstance::Singular(Noun::Opaque(strange))),
                ] if shiny.spelling() == "shiny" && strange.spelling() == "strange"
            ),
            "{:#?}",
            object.modifiers
        );
    }

    #[test]
    fn magic_atom_stays_known_instead_of_becoming_a_generic_opacity() {
        let catalogs =
            Catalogs::default().with_catalog(crate::catalog::CatalogKind::CardType, ["Creature"]);
        let parsed = parse_nonterminal(
            "Target creature gets +1/+1.",
            &catalogs,
            Nonterminal::Sentence,
        )
        .unwrap();
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
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

    fn opaque_nouns(sentence: &crate::syntax::Sentence) -> Vec<String> {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    sentences: vec![sentence.clone()],
                }),
            }],
        }
        .lexical_opacity()
        .into_iter()
        .map(|opaque| opaque.text.to_owned())
        .collect()
    }

    fn transitive(sentence: &crate::syntax::Sentence) -> &crate::syntax::TransitivePredicate {
        match &sentence.body {
            SentenceBody::Independent(
                IndependentClause::Transitive(_, predicate)
                | IndependentClause::Imperative(Predicate::Transitive(predicate)),
            ) => predicate,
            other => panic!("expected a transitive clause, got {other:#?}"),
        }
    }
}
