#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::catalog::CatalogKind;
    use crate::catalog::Catalogs;
    use crate::chart::Grammar;
    use crate::forest::ForestSymbol;
    use crate::syntax::Ability;
    use crate::syntax::AbilityKind;
    use crate::syntax::AdjectiveComplement;
    use crate::syntax::AdjectivePhrase;
    use crate::syntax::ComparisonMarker;
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
    fn turn_structure_nominals_render_without_source() {
        // Sub-shape 1: named steps/phases as ordinary nominals. `draw step`
        // needs the new `draw` count-noun sense; the rest already parse.
        // Causal pair on whose-step: `your upkeep` vs `an opponent's upkeep`.
        for source in [
            "your draw step",
            "the draw step",
            "your upkeep",
            "an opponent's upkeep",
            "your combat phase",
            "each of your turns",
            "each of them",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_fragment(parsed.noun_phrase().expect("noun-phrase root")),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn draw_step_head_is_a_noun_modifier_not_a_reshaped_verb() {
        // `draw step` is a stacked noun-modifier compound: `step` head with a
        // `draw` count-noun modifier — the same shape the negative-armor
        // `combat damage` uses. Guards that adding the `draw` noun sense did not
        // reshape it into anything verb-flavored.
        let parsed = parse("your draw step");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal for the draw step");
        };
        assert!(
            matches!(
                nominal.modifiers.as_slice(),
                [NominalModifier::Noun(NounInstance::Singular(Noun::Word(
                    Vocab::Draw
                )))]
            ),
            "{nominal:#?}"
        );
        assert!(matches!(
            &nominal.head,
            NounInstance::Singular(Noun::Word(word)) if word.spelling() == "step"
        ));
    }

    #[test]
    fn combat_damage_negative_armor_keeps_combat_as_a_noun_modifier() {
        // Negative armor: `combat` in noun-modifier position must stay a noun
        // modifier of `damage`, unaffected by turn-structure work.
        let parsed = parse("combat damage");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal for combat damage");
        };
        assert!(
            matches!(
                nominal.modifiers.as_slice(),
                [NominalModifier::Noun(NounInstance::Singular(Noun::Word(
                    Vocab::Combat
                )))]
            ),
            "{nominal:#?}"
        );
        assert!(matches!(
            &nominal.head,
            NounInstance::Mass(Noun::Word(Vocab::Damage))
        ));
    }

    #[test]
    fn distributive_and_counted_partitives_keep_distinct_heads() {
        use crate::syntax::PartitiveHead;
        // Causal pair: the distributive `each of X` records `PartitiveHead::Each`
        // while the counted `one of X` keeps `PartitiveHead::Quantity`.
        let each = parse("each of your turns");
        assert!(
            matches!(
                each.noun_phrase(),
                Some(NounPhrase::Partitive(crate::syntax::PartitiveNounPhrase {
                    head: PartitiveHead::Each,
                    ..
                }))
            ),
            "{:#?}",
            each.noun_phrase()
        );
        assert_eq!(
            render_fragment(each.noun_phrase().unwrap()),
            "each of your turns"
        );

        let one = parse("one of them");
        assert!(matches!(
            one.noun_phrase(),
            Some(NounPhrase::Partitive(crate::syntax::PartitiveNounPhrase {
                head: PartitiveHead::Quantity(crate::syntax::Quantity::Exact(number)),
                ..
            })) if number.value == 1
        ));
    }

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
            "one or two target creatures",
            "one of them",
            "the top three cards",
            "X cards",
            "mana value X",
            "a d20",
            "that many cards",
            "that much damage",
            "fewer than three counters",
            "more than one artifact",
            "more than one of the same mana symbol in its mana cost",
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
            "power greater than its base power",
            "mana value less than or equal to the number of lands you control",
            "a color other than black",
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
            ("an emblem", "emblem"),
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
    fn postpositive_comparative_adjectives_parse_without_noun_recovery() {
        for (source, expected_head, expected_marker) in [
            (
                "power greater than its base power",
                Vocab::Greater,
                ComparisonMarker::Than,
            ),
            (
                "mana value less than or equal to the number of lands you control",
                Vocab::Less,
                ComparisonMarker::ThanOrEqualTo,
            ),
            (
                "a color other than black",
                Vocab::Other,
                ComparisonMarker::Than,
            ),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a nominal phrase for {source:?}");
            };
            let [NominalComplement::Adjective(AdjectivePhrase { head, complements })] =
                nominal.complements.as_slice()
            else {
                panic!("expected one postpositive adjective for {source:?}: {nominal:#?}");
            };
            assert_eq!(head, &Adjective::Word(expected_head), "{source}");
            assert!(
                matches!(
                    complements.as_slice(),
                    [AdjectiveComplement::Comparison(comparison)]
                        if comparison.marker == expected_marker
                ),
                "{source}: {complements:#?}"
            );
        }
    }

    #[test]
    fn self_reference_possessive_is_one_determiner() {
        let parsed = parse("~'s power");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a possessive nominal");
        };
        assert!(
            matches!(
                &nominal.determiner,
                Some(Determiner::Possessive(Possessor::NounPhrase(possessor)))
                    if matches!(possessor.as_ref(), NounPhrase::ThisCard(_))
            ),
            "{nominal:#?}"
        );
        assert_eq!(
            render_fragment(parsed.noun_phrase().unwrap()),
            "Test Card's power"
        );
    }

    #[test]
    fn self_reference_possessive_can_stand_as_an_elliptical_noun_phrase() {
        let parsed = parse("~'s");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert!(matches!(
            parsed.noun_phrase(),
            Some(NounPhrase::Possessive(Possessor::NounPhrase(possessor)))
                if matches!(possessor.as_ref(), NounPhrase::ThisCard(_))
        ));
        assert_eq!(
            render_fragment(parsed.noun_phrase().unwrap()),
            "Test Card's"
        );
    }

    #[test]
    fn comparison_inside_preposition_stays_with_its_object() {
        let parsed = parse("target creature card with mana value less than or equal to ~'s power");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let Some(NounPhrase::Nominal(card)) = parsed.noun_phrase() else {
            panic!("expected a card nominal");
        };
        let [NominalComplement::Prepositional(with)] = card.complements.as_slice() else {
            panic!("comparison escaped its prepositional object: {card:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(object) = with.object.as_ref() else {
            panic!("with object should be a noun phrase");
        };
        let NounPhrase::Nominal(value) = object.as_ref() else {
            panic!("with object should be nominal");
        };
        assert!(
            matches!(
                value.complements.as_slice(),
                [NominalComplement::Adjective(_)]
            ),
            "{value:#?}"
        );
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
            Some(Determiner::Quantity(crate::syntax::Quantity::OrComparison(
                number,
                crate::syntax::ComparativeWord::More
            )))
                if number.value == 1
        ));
        assert!(matches!(at_least.head, NounInstance::Plural(_)));

        let either = parse("one or two target creatures");
        let Some(NounPhrase::Nominal(either)) = either.noun_phrase() else {
            panic!("expected an either-quantity nominal");
        };
        assert!(matches!(
            either.determiner,
            Some(Determiner::Target(Some(crate::syntax::Quantity::Or(one, two))))
                if one.value == 1 && two.value == 2
        ));
        assert!(matches!(either.head, NounInstance::Plural(_)));

        let definite_quantity = parse("the top three cards");
        let Some(NounPhrase::Nominal(definite_quantity)) = definite_quantity.noun_phrase() else {
            panic!("expected a definite quantity nominal");
        };
        assert!(matches!(
            definite_quantity.modifiers.as_slice(),
            [
                NominalModifier::Adjective(_),
                NominalModifier::Quantity(crate::syntax::Quantity::Exact(three)),
            ] if three.value == 3
        ));

        let variable_quantity = parse("X cards");
        let Some(NounPhrase::Nominal(variable_quantity)) = variable_quantity.noun_phrase() else {
            panic!("expected a variable quantity nominal");
        };
        assert!(
            matches!(
                variable_quantity.determiner,
                Some(Determiner::Quantity(crate::syntax::Quantity::X))
            ),
            "{variable_quantity:#?}"
        );

        let value = parse("mana value X");
        let Some(NounPhrase::Nominal(value)) = value.noun_phrase() else {
            panic!("expected a quantified value nominal");
        };
        assert!(matches!(
            value.complements.as_slice(),
            [NominalComplement::Quantity(crate::syntax::Quantity::X)]
        ));

        let die = parse("a d20");
        let Some(NounPhrase::Nominal(die)) = die.noun_phrase() else {
            panic!("expected a die nominal");
        };
        assert!(matches!(
            die.head,
            NounInstance::Singular(Noun::Die(number)) if number.value == 20
        ));

        let partitive = parse("one of them");
        assert!(matches!(
            partitive.noun_phrase(),
            Some(NounPhrase::Partitive(crate::syntax::PartitiveNounPhrase {
                head: crate::syntax::PartitiveHead::Quantity(crate::syntax::Quantity::Exact(one)),
                ..
            })) if one.value == 1
        ));

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

        for (source, more) in [
            ("more than one artifact", true),
            ("fewer than three counters", false),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a bounded quantity nominal for {source:?}");
            };
            assert!(
                matches!(
                    &nominal.determiner,
                    Some(Determiner::Quantity(crate::syntax::Quantity::MoreThan(_))) if more
                ) || matches!(
                    &nominal.determiner,
                    Some(Determiner::Quantity(crate::syntax::Quantity::FewerThan(_))) if !more
                ),
                "{source}: {nominal:#?}"
            );
        }

        let partitive = parse("more than one of the same mana symbol in its mana cost");
        assert!(matches!(
            partitive.noun_phrase(),
            Some(NounPhrase::Partitive(crate::syntax::PartitiveNounPhrase {
                head: crate::syntax::PartitiveHead::Quantity(crate::syntax::Quantity::MoreThan(one)),
                ..
            })) if one.value == 1
        ));
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

    #[test]
    fn characteristic_postmodifier_bounds_parse_as_structural_quantities() {
        use crate::syntax::ComparativeWord;
        use crate::syntax::Quantity;
        // Causal pair: `or less` (ceiling) and its `or greater` (floor) mirror,
        // both as a `with <characteristic> N or <word>` postmodifier.
        for (source, value, word) in [
            ("creatures with power 2 or less", 2, ComparativeWord::Less),
            (
                "creatures with power 4 or greater",
                4,
                ComparativeWord::Greater,
            ),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_fragment(parsed.noun_phrase().expect("noun-phrase root")),
                source,
                "{source}"
            );
            let bound = characteristic_bound(&parsed);
            assert!(
                matches!(bound, Quantity::OrComparison(number, seen)
                    if number.value == value && seen == word),
                "{source}: {bound:?}"
            );
        }
    }

    #[test]
    fn comparative_quantity_determiners_mirror_the_postmodifier() {
        use crate::syntax::ComparativeWord;
        use crate::syntax::Quantity;
        // Causal pair: the same bound in determiner position — `two or fewer`
        // ceiling against its `three or more` floor mirror.
        for (source, value, word) in [
            ("two or fewer other lands", 2, ComparativeWord::Fewer),
            ("three or more lands", 3, ComparativeWord::More),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_fragment(parsed.noun_phrase().expect("noun-phrase root")),
                source,
                "{source}"
            );
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a quantified nominal for {source:?}");
            };
            assert!(
                matches!(
                    &nominal.determiner,
                    Some(Determiner::Quantity(Quantity::OrComparison(number, seen)))
                        if number.value == value && *seen == word
                ),
                "{source}: {nominal:#?}"
            );
        }
    }

    #[test]
    fn plain_value_and_number_coordination_are_not_comparative_bounds() {
        use crate::syntax::Quantity;
        // Negative armor 1: a bare `with power 2` stays an exact value — no
        // comparative word, so no `or`-bound production fires.
        let plain = parse("creatures with power 2");
        assert_eq!(
            render_fragment(plain.noun_phrase().expect("noun-phrase root")),
            "creatures with power 2"
        );
        assert!(
            matches!(characteristic_bound(&plain), Quantity::Exact(number) if number.value == 2),
            "plain characteristic must stay exact: {:?}",
            characteristic_bound(&plain)
        );

        // Negative armor 2: an unrelated number `or` coordination is the `Or`
        // quantity, never hijacked into a comparative bound.
        let either = parse("one or two target creatures");
        let Some(NounPhrase::Nominal(either)) = either.noun_phrase() else {
            panic!("expected an either-quantity nominal");
        };
        assert!(
            matches!(
                either.determiner,
                Some(Determiner::Target(Some(Quantity::Or(one, two))))
                    if one.value == 1 && two.value == 2
            ),
            "{either:#?}"
        );
    }

    /// Extracts the quantity bounding the characteristic inside a
    /// `<noun> with <characteristic> …` postmodifier.
    fn characteristic_bound(parsed: &ParsedNonterminal) -> crate::syntax::Quantity {
        let Some(NounPhrase::Nominal(outer)) = parsed.noun_phrase() else {
            panic!("expected an outer nominal");
        };
        let [NominalComplement::Prepositional(with)] = outer.complements.as_slice() else {
            panic!("expected a single `with` complement: {outer:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(object) = with.object.as_ref() else {
            panic!("`with` object should be a noun phrase");
        };
        let NounPhrase::Nominal(characteristic) = object.as_ref() else {
            panic!("`with` object should be nominal");
        };
        let [NominalComplement::Quantity(quantity)] = characteristic.complements.as_slice() else {
            panic!("expected a single quantity complement: {characteristic:#?}");
        };
        *quantity
    }

    fn parse(source: &str) -> ParsedNonterminal {
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::Supertype, ["Legendary"])
    }

    fn render_fragment(noun_phrase: &NounPhrase) -> String {
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    flavor_header: None,
                    sentences: vec![Sentence {
                        initial_uppercase: true,
                        body: SentenceBody::Independent(IndependentClause::Imperative(
                            Predicate::Transitive(TransitivePredicate {
                                head: PredicateHead {
                                    auxiliaries: vec![],
                                    first_auxiliary_contracted_with_subject: false,
                                    preverb_modifiers: vec![],
                                    verb: VerbInstance {
                                        verb: Verb::Word(Vocab::Draw),
                                        slot: VerbSlot::Imperative,
                                    },
                                },
                                pre_object_elements: vec![],
                                object: PredicateObject::NounPhrase(noun_phrase.clone()),
                                elements: vec![],
                            }),
                        )),
                        ending: SentenceEnding::Period,
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
