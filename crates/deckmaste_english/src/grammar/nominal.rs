#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::catalog::CatalogKind;
    use crate::catalog::Catalogs;
    use crate::chart::Grammar;
    use crate::forest::ForestSymbol;
    use crate::syntax::Ability;
    use crate::syntax::AbilityKind;
    use crate::syntax::AdjectivePhrase;
    use crate::syntax::Determiner;
    use crate::syntax::IndependentClause;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::Possessor;
    use crate::syntax::Predicate;
    use crate::syntax::PredicateHead;
    use crate::syntax::PredicateObject;
    use crate::syntax::Sentence;
    use crate::syntax::SentenceBody;
    use crate::syntax::SentenceEnding;
    use crate::syntax::TransitivePredicate;
    use crate::word::Adjective;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::NounUsage;
    use crate::word::Pronoun;
    use crate::word::PronounCase;
    use crate::word::Verb;
    use crate::word::VerbInstance;
    use crate::word::VerbSlot;
    use crate::word::Vocab;

    #[test]
    fn nominal_fixtures_parse_structurally_and_render_without_source() {
        for source in [
            "a card",
            "an hour",
            "the target creature",
            "target creature",
            "up to one target creature",
            "up to three target creatures",
            "one or more creatures",
            "that many cards",
            "that much damage",
            "any target",
            "no cards",
            "each other",
            "each other creature",
            "a random order",
            "black creature",
            "legendary Goblin creature",
            "artifact creature card",
            "creature you control",
            "cards in your graveyard",
            "cards among all permanents",
            "cards from among them",
            "cards from anywhere",
            "combat during your turn",
            "combat before your turn",
            "maximum hand size",
            "the amount of mana",
            "your party",
        ] {
            let parsed = parse(source);
            assert_eq!(
                render_fragment(parsed.noun_phrase().expect("noun-phrase root")),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn regular_table_nouns_fill_nominal_head_slots() {
        for (source, expected) in [
            ("maximum hand size", "size"),
            ("the amount of mana", "amount"),
            ("your party", "party"),
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a nominal phrase for {source:?}");
            };
            assert!(matches!(
                &nominal.head,
                NounInstance::Singular(Noun::Word(word)) if word.spelling() == expected
            ));
        }
    }

    #[test]
    fn determiners_quantities_and_reciprocal_pronouns_keep_distinct_meanings() {
        let target = parse("up to three target creatures");
        let Some(NounPhrase::Nominal(target)) = target.noun_phrase() else {
            panic!("expected a nominal target phrase");
        };
        assert!(matches!(
            target.determiner,
            Some(Determiner::Target(Some(crate::syntax::Quantity::UpTo(number))))
                if number.value == 3
        ));
        assert!(matches!(target.head, NounInstance::Plural(_)));

        let at_least = parse("one or more creatures");
        let Some(NounPhrase::Nominal(at_least)) = at_least.noun_phrase() else {
            panic!("expected an at-least quantified nominal");
        };
        assert!(matches!(
            at_least.determiner,
            Some(Determiner::Quantity(crate::syntax::Quantity::AtLeast(number)))
                if number.value == 1
        ));
        assert!(matches!(at_least.head, NounInstance::Plural(_)));

        let any = parse("any target");
        let Some(NounPhrase::Nominal(any)) = any.noun_phrase() else {
            panic!("expected an any-determined nominal");
        };
        assert_eq!(any.determiner, Some(Determiner::Any));
        assert!(matches!(
            any.head,
            NounInstance::Singular(Noun::Word(Vocab::Target))
        ));

        let no = parse("no cards");
        let Some(NounPhrase::Nominal(no)) = no.noun_phrase() else {
            panic!("expected a no-determined nominal");
        };
        assert_eq!(no.determiner, Some(Determiner::No));
        assert!(matches!(no.head, NounInstance::Plural(_)));

        let much = parse("that much damage");
        let Some(NounPhrase::Nominal(much)) = much.noun_phrase() else {
            panic!("expected a quantified nominal");
        };
        assert!(matches!(
            much.determiner,
            Some(Determiner::Quantity(crate::syntax::Quantity::ThatMuch))
        ));
        assert!(matches!(
            much.head,
            NounInstance::Mass(Noun::Word(Vocab::Damage))
        ));

        let reciprocal = parse("each other");
        assert_eq!(
            reciprocal.noun_phrase(),
            Some(&NounPhrase::Pronoun {
                pronoun: Pronoun::EachOther,
                case: PronounCase::Object,
            })
        );
    }

    #[test]
    fn each_other_analysis_coexists_with_each_plus_other_modifier() {
        let parsed = parse("each other creature");
        assert!(parsed.chart.forest.nodes().any(|node| {
            node.key.symbol == ForestSymbol::Nonterminal(Nonterminal::NounPhrase)
                && (node.key.start, node.key.end) == (0, 2)
        }));

        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("full phrase should select a nominal");
        };
        assert_eq!(nominal.determiner, Some(Determiner::Each));
        assert!(matches!(
            nominal.modifiers.as_slice(),
            [NominalModifier::Adjective(AdjectivePhrase {
                head: Adjective::Word(Vocab::Other),
                ..
            })]
        ));
    }

    #[test]
    fn catalogs_and_relative_complements_preserve_their_grammar_slots() {
        let parsed = parse("legendary Goblin creature");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected catalog nominal");
        };
        assert!(matches!(
            nominal.modifiers.as_slice(),
            [
                NominalModifier::Adjective(AdjectivePhrase {
                    head: Adjective::Catalog(legendary),
                    ..
                }),
                NominalModifier::Noun(NounInstance::Singular(Noun::Catalog(goblin))),
            ] if legendary.kind == CatalogKind::Supertype
                && goblin.kind == CatalogKind::CreatureType
        ));

        let controlled = parse("creature you control");
        let Some(NounPhrase::Nominal(controlled)) = controlled.noun_phrase() else {
            panic!("expected relative-clause nominal");
        };
        assert!(matches!(
            controlled.complements.as_slice(),
            [NominalComplement::Relative(_)]
        ));

        let graveyard = parse("cards in your graveyard");
        let Some(NounPhrase::Nominal(graveyard)) = graveyard.noun_phrase() else {
            panic!("expected prepositional nominal");
        };
        let [NominalComplement::Prepositional(preposition)] = graveyard.complements.as_slice()
        else {
            panic!("expected one prepositional complement");
        };
        let crate::syntax::Phrase::NounPhrase(object) = preposition.object.as_ref() else {
            panic!("preposition object should be a noun phrase");
        };
        let NounPhrase::Nominal(object) = object.as_ref() else {
            panic!("preposition object should be nominal");
        };
        assert_eq!(
            object.determiner,
            Some(Determiner::Possessive(Possessor::Pronoun(Pronoun::You)))
        );
    }

    #[test]
    fn prepositions_can_take_nominal_nested_and_adverbial_objects() {
        let among = parse("cards among all permanents");
        let Some(NounPhrase::Nominal(among)) = among.noun_phrase() else {
            panic!("expected an among nominal");
        };
        assert!(matches!(
            among.complements.as_slice(),
            [NominalComplement::Prepositional(preposition)]
                if preposition.preposition == crate::syntax::Preposition::Among
                    && matches!(
                        preposition.object.as_ref(),
                        crate::syntax::Phrase::NounPhrase(_)
                    )
        ));

        let nested = parse("cards from among them");
        let Some(NounPhrase::Nominal(nested)) = nested.noun_phrase() else {
            panic!("expected a nested-preposition nominal");
        };
        assert!(matches!(
            nested.complements.as_slice(),
            [NominalComplement::Prepositional(outer)]
                if outer.preposition == crate::syntax::Preposition::From
                    && matches!(
                        outer.object.as_ref(),
                        crate::syntax::Phrase::PrepositionalPhrase(inner)
                            if inner.preposition == crate::syntax::Preposition::Among
                    )
        ));

        let anywhere = parse("cards from anywhere");
        let Some(NounPhrase::Nominal(anywhere)) = anywhere.noun_phrase() else {
            panic!("expected an adverb-object nominal");
        };
        assert!(matches!(
            anywhere.complements.as_slice(),
            [NominalComplement::Prepositional(preposition)]
                if matches!(
                    preposition.object.as_ref(),
                    crate::syntax::Phrase::Adverb(adverb)
                        if adverb.spelling() == "anywhere"
                )
        ));

        for (source, expected) in [
            (
                "combat during your turn",
                crate::syntax::Preposition::During,
            ),
            (
                "combat before your turn",
                crate::syntax::Preposition::Before,
            ),
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a temporal nominal");
            };
            assert!(matches!(
                nominal.complements.as_slice(),
                [NominalComplement::Prepositional(preposition)]
                    if preposition.preposition == expected
            ));
        }
    }

    #[test]
    fn target_candidates_are_requested_by_slot_instead_of_chosen_by_the_lexer() {
        let catalogs = fixture_catalogs();
        let surface = crate::surface::lex("target");
        let grammar = EnglishGrammar::new("target", &catalogs, Nonterminal::NounPhrase);

        for slot in [
            EnglishLexicalSlot::DeterminerTarget,
            EnglishLexicalSlot::Adjective,
            EnglishLexicalSlot::Noun(NounUsage::Count),
            EnglishLexicalSlot::Verb(VerbSlot::Imperative),
        ] {
            assert!(
                !grammar.scan(slot, &surface.tokens, 0).is_empty(),
                "{slot:?}"
            );
        }
    }

    fn parse(source: &str) -> ParsedNonterminal {
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature"])
            .with_catalog(CatalogKind::Supertype, ["Legendary"])
    }

    fn render_fragment(noun_phrase: &NounPhrase) -> String {
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    sentences: vec![Sentence {
                        body: SentenceBody::Independent(IndependentClause::Imperative(
                            Predicate::Transitive(TransitivePredicate {
                                head: PredicateHead {
                                    auxiliaries: vec![],
                                    preverb_modifiers: vec![],
                                    verb: VerbInstance {
                                        verb: Verb::Word(Vocab::Draw),
                                        slot: VerbSlot::Imperative,
                                    },
                                },
                                object: PredicateObject::NounPhrase(noun_phrase.clone()),
                                elements: vec![],
                            }),
                        )),
                        ending: SentenceEnding::Period(1),
                    }],
                }),
            }],
        };
        ast.render("Test Card", false)
            .expect("parsed noun phrase must render")
            .strip_prefix("Draw ")
            .and_then(|rendered| rendered.strip_suffix('.'))
            .expect("fixture wrapper is stable")
            .to_owned()
    }
}
