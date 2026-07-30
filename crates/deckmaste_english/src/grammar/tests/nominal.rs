#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::catalog::CatalogKind;
    use crate::catalog::Catalogs;
    use crate::catalog::RulesNominal;
    use crate::chart::Grammar;
    use crate::forest::ForestSymbol;
    use crate::identity::SelfReference;
    use crate::syntax::Ability;
    use crate::syntax::AbilityKind;
    use crate::syntax::AdjectiveComplement;
    use crate::syntax::AdjectivePhrase;
    use crate::syntax::ComparisonMarker;
    use crate::syntax::Determiner;
    use crate::syntax::IndefiniteArticle;
    use crate::syntax::IndependentClause;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalModifier;
    use crate::syntax::NominalPhrase;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::Polarity;
    use crate::syntax::Possessor;
    use crate::syntax::Predicate;
    use crate::syntax::PredicateHead;
    use crate::syntax::PredicateObject;
    use crate::syntax::Sentence;
    use crate::syntax::SentenceBody;
    use crate::syntax::SetExceptionMarker;
    use crate::syntax::TransitivePredicate;
    use crate::word::Adjective;
    use crate::word::ColorWord;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::NounUsage;
    use crate::word::Pronoun;
    use crate::word::PronounCase;
    use crate::word::Tense;
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
                [NominalModifier::Noun {
                    noun: NounInstance::Singular(Noun::Word(Vocab::Draw)),
                    ..
                }]
            ),
            "{nominal:#?}"
        );
        assert!(matches!(
            &nominal.head,
            NounInstance::Singular(Noun::Word(word)) if word.spelling() == "step"
        ));
    }

    #[test]
    fn combat_step_names_are_step_nominals() {
        for source in [
            "the declare attackers step",
            "the declare blockers step",
            "your declare attackers step",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_fragment(parsed.noun_phrase().expect(source)),
                source,
                "{source}"
            );
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a nominal for {source}");
            };
            assert!(
                matches!(
                    nominal.modifiers.as_slice(),
                    [NominalModifier::CombatStepName { participants }]
                        if matches!(
                            participants,
                            NounInstance::Plural(Noun::Agentive(Verb::Word(
                                Vocab::Attack | Vocab::Block
                            )))
                        )
                ),
                "{source}: {nominal:#?}"
            );
            assert!(matches!(
                &nominal.head,
                NounInstance::Singular(Noun::Word(word)) if word.spelling() == "step"
            ));
        }
    }

    #[test]
    fn combat_step_name_requires_all_three_words() {
        for source in [
            "the declare",
            "declare",
            "declare attackers",
            "declare creatures step",
        ] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase).is_err(),
                "{source} must not parse as a combat-step nominal"
            );
        }
        // `the attack step` is an ordinary noun-modifier compound (unrelated
        // to `declare attackers`/`declare blockers`) — it parses, but never
        // through the `CombatStepName` shape.
        let parsed = parse("the attack step");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal for `the attack step`");
        };
        assert!(
            !matches!(
                nominal.modifiers.as_slice(),
                [NominalModifier::CombatStepName { .. }]
            ),
            "{nominal:#?}"
        );
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
                [NominalModifier::Noun {
                    noun: NounInstance::Singular(Noun::Word(Vocab::Combat)),
                    ..
                }]
            ),
            "{nominal:#?}"
        );
        assert!(matches!(
            &nominal.head,
            NounInstance::Mass(Noun::Word(Vocab::Damage))
        ));
    }

    #[test]
    fn productive_negation_shares_the_positive_outer_shape_up_to_polarity() {
        // Owner's invariant: `nonland card` and `land card` are the same outer
        // modifier node, differing only in the polarity flag — negation is never
        // a mechanism or category change. One causal pair per base category the
        // corpus needs: supertype, card type, hyphenated subtype, color,
        // participle.
        for (positive, negative) in [
            ("basic land", "nonbasic land"), // supertype catalog adjective
            ("land card", "nonland card"),   // card-type attributive noun
            ("Human creature", "non-Human creature"), // hyphenated subtype noun
            ("black creature", "nonblack creature"), // color adjective
            ("attacking creature", "nonattacking creature"), // participle
        ] {
            let positive_modifier = first_modifier(&parse(positive));
            let negative_modifier = first_modifier(&parse(negative));
            assert!(
                matches!(polarity_of(&positive_modifier), Some(Polarity::Positive)),
                "{positive}: {positive_modifier:#?}"
            );
            assert!(
                matches!(polarity_of(&negative_modifier), Some(Polarity::Negative)),
                "{negative}: {negative_modifier:#?}"
            );
            assert_eq!(
                depolarize(&negative_modifier),
                positive_modifier,
                "{negative} must equal {positive} up to the polarity flag",
            );
        }
    }

    #[test]
    fn negation_scan_accepts_either_spelling_for_the_same_base() {
        // The hyphenation glyph is no longer recorded structurally: solid
        // `nonland` and hyphenated `non-land` spellings of the same base
        // resolve to the identical negated modifier, as do solid `nonHuman`
        // and hyphenated `non-Human` — only the renderer decides which
        // spelling to emit, from the base's capitalization.
        for (solid, hyphenated) in [
            ("nonland permanent", "non-land permanent"),
            ("nonHuman creature", "non-Human creature"),
        ] {
            let solid_modifier = first_modifier(&parse(solid));
            let hyphenated_modifier = first_modifier(&parse(hyphenated));
            assert_eq!(
                solid_modifier, hyphenated_modifier,
                "{solid} vs {hyphenated}"
            );
            assert!(matches!(
                polarity_of(&solid_modifier),
                Some(Polarity::Negative)
            ));
        }
    }

    #[test]
    fn negation_safety_net_leaves_narrative_words_intact() {
        // Only a residue that resolves fires the morphology; `none` and
        // `nonetheless` strip to non-words and must not decompose, so those
        // spellings survive unchanged.
        let catalogs = fixture_catalogs();
        for word in ["none", "nonetheless"] {
            let surface = crate::surface::lex(word);
            let grammar = EnglishGrammar::new(word, &catalogs, Nonterminal::NounPhrase);
            assert!(
                grammar
                    .scan(EnglishLexicalSlot::NegatedModifier, &surface.tokens, 0)
                    .is_empty(),
                "{word} must not decompose as a negation"
            );
        }
        // Control: a resolving residue does fire.
        let surface = crate::surface::lex("nonland");
        let grammar = EnglishGrammar::new("nonland", &catalogs, Nonterminal::NounPhrase);
        assert!(
            !grammar
                .scan(EnglishLexicalSlot::NegatedModifier, &surface.tokens, 0)
                .is_empty(),
            "nonland must decompose as a negation"
        );
    }

    #[test]
    fn negation_round_trips_solid_and_hyphenated_byte_exactly() {
        for source in [
            "nonland permanent",
            "non-Human creature",
            // `outlaw` is a lowercase rules-bundle noun that still hyphenates —
            // its `non-` glyph derives from the bundle category, not
            // capitalization (Shoot the Sheriff: "Destroy target non-outlaw
            // creature.").
            "target non-outlaw creature",
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
    fn rules_bundle_words_round_trip_in_nominal_slots() {
        // The four rules-bundle shorthands reach the AST through the ordinary
        // catalog adjective/noun paths and render back byte-exactly: `historic`
        // and `modified` as attributive adjectives, `party` as a count noun,
        // and `outlaw` as both a head noun and an attributive adjective.
        for source in [
            "a historic card",
            "a modified creature",
            "your party",
            "an outlaw",
            "outlaw creatures",
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
    fn coordinated_modifiers_round_trip_byte_exactly() {
        // One witness-shaped round-trip per coordinated-modifier sub-shape. The
        // causal control is the single-modifier baseline `a white creature`,
        // which already parses; adding a conjunct must keep it byte-exact.
        for source in [
            "a white creature",                     // control: single modifier
            "a white and blue creature",            // color pair (Ashiok)
            "a black and green Goblin creature",    // color pair + noun stack (Amzu)
            "a white and/or blue creature",         // and/or pair (Amphibious Kavu)
            "a white, blue, and black creature",    // Oxford color triple
            "a white, blue, or black creature",     // Oxford `or` color triple
            "an artifact, creature, and land card", // type-noun Oxford list (Warp World)
            "a creature and land card",             // two-way type-noun modifiers
            "a nonwhite and nonblue creature",      // coordinated negated conjuncts
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
    fn coordinated_head_lists_round_trip_byte_exactly() {
        // Head-list coordination: comma/Oxford extensions of the existing binary
        // noun-phrase coordination under a shared determiner. The causal control
        // is the two-way `target artifact or land`, which already parsed; the
        // Oxford comma extends it to three heads (Acidic Slime).
        for source in [
            "target artifact or land",             // control: two-way head coordination
            "target artifact, creature, or land",  // Oxford `or` head list (Acidic Slime)
            "all artifacts, creatures, and lands", // Oxford `and` head list
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
    fn target_coordination_carries_one_shared_determiner() {
        let source = "target artifact or land";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected nominal coordination under one determiner: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(coordinated.determiner, Determiner::Target(None));
        assert!(coordinated.first.determiner.is_none());
        let [land] = coordinated.rest.as_slice() else {
            panic!("expected one coordinated head: {coordinated:#?}");
        };
        assert_eq!(
            land.conjunction,
            Some(crate::syntax::NounPhraseConjunction::Or)
        );
        assert!(land.phrase.determiner.is_none());
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn target_scopes_over_coordinated_modified_nominals() {
        let source = "target enchanted creature or enchantment creature you control";
        let catalogs =
            fixture_catalogs().with_catalog(CatalogKind::CardType, ["Creature", "Enchantment"]);
        let parsed = parse_nonterminal(source, &catalogs, Nonterminal::NounPhrase)
            .expect("modified target alternatives must parse");
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected modified nominals under one target determiner: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(coordinated.determiner, Determiner::Target(None));
        assert_eq!(coordinated.rest.len(), 1);
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn among_oxford_list_stays_inside_the_second_coordinated_selection() {
        let source = concat!(
            "a +1/+1 counter and a counter from among flying, first strike, ",
            "lifelink, or vigilance on it"
        );
        let catalogs = fixture_catalogs().with_catalog(
            CatalogKind::KeywordAbility,
            ["Flying", "First Strike", "Lifelink", "Vigilance"],
        );
        let parsed = parse_nonterminal(source, &catalogs, Nonterminal::NounPhrase)
            .expect("the coordinated counter selection must parse");
        let Some(NounPhrase::Coordinated(outer)) = parsed.noun_phrase() else {
            panic!(
                "expected two coordinated counter selections: {:#?}",
                parsed.noun_phrase()
            );
        };
        let [second] = outer.rest.as_slice() else {
            panic!("expected one outer `and` member: {outer:#?}");
        };
        assert_eq!(
            second.conjunction,
            Some(crate::syntax::NounPhraseConjunction::And)
        );
        let NounPhrase::Nominal(second) = &second.phrase else {
            panic!("expected a nominal second counter: {second:#?}");
        };
        let [
            NominalComplement::Prepositional(from),
            NominalComplement::Prepositional(on),
        ] = second.complements.as_slice()
        else {
            panic!("expected `from among ...` followed by `on it`: {second:#?}");
        };
        assert_eq!(from.head().preposition, crate::syntax::Preposition::From);
        assert_eq!(on.head().preposition, crate::syntax::Preposition::On);
        let crate::syntax::Phrase::PrepositionalPhrase(among) = from.head().object.as_ref() else {
            panic!("expected nested `among` PP: {from:#?}");
        };
        assert_eq!(among.head().preposition, crate::syntax::Preposition::Among);
        let crate::syntax::Phrase::NounPhrase(options) = among.head().object.as_ref() else {
            panic!("expected an `among` option list: {among:#?}");
        };
        let NounPhrase::Coordinated(options) = options.as_ref() else {
            panic!("expected coordinated keyword options: {options:#?}");
        };
        assert!(matches!(
            options.rest.as_slice(),
            [
                crate::syntax::NounPhraseCoordination {
                    conjunction: None,
                    comma: true,
                    ..
                },
                crate::syntax::NounPhraseCoordination {
                    conjunction: None,
                    comma: true,
                    ..
                },
                crate::syntax::NounPhraseCoordination {
                    conjunction: Some(crate::syntax::NounPhraseConjunction::Or),
                    comma: true,
                    ..
                }
            ]
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn general_determiners_scope_over_modified_nominals() {
        for (source, expected_determiner) in [
            ("another target creature or artifact", Determiner::Another),
            (
                "up to one other target creature or spell",
                Determiner::Quantity(crate::syntax::Quantity::UpTo(
                    crate::syntax::QuantityValue::Literal(crate::syntax::NumberLiteral {
                        value: 1,
                        numeral: crate::numeral::Numeral::Cardinal,
                    }),
                )),
            ),
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
                panic!(
                    "expected nominal coordination under one determiner: {:#?}",
                    parsed.noun_phrase()
                );
            };
            assert_eq!(coordinated.determiner, expected_determiner, "{source}");
            assert!(coordinated.first.determiner.is_none(), "{source}");
            assert!(
                coordinated
                    .rest
                    .iter()
                    .all(|member| member.phrase.determiner.is_none()),
                "{source}"
            );
            assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
        }
    }

    #[test]
    fn trailing_relative_scopes_over_the_completed_shared_group() {
        let source = "another target creature or artifact you control";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected one shared-determiner group: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(coordinated.determiner, Determiner::Another);
        assert!(coordinated.first.complements.is_empty());
        assert!(
            coordinated
                .rest
                .iter()
                .all(|member| member.phrase.complements.is_empty()),
            "the relative must not remain on the final member: {coordinated:#?}"
        );
        assert!(matches!(
            coordinated.complements.as_slice(),
            [NominalComplement::Relative(_)]
        ));
        assert!(
            parsed
                .constituent_spans()
                .iter()
                .any(|span| span.text(source) == Some("target creature or artifact"))
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn modified_options_share_the_determiner_when_no_common_head_parse_exists() {
        for (source, determiner) in [
            (
                "target spell, activated ability, or triggered ability",
                Determiner::Target(None),
            ),
            (
                "target instant spell, sorcery spell, or triggered ability",
                Determiner::Target(None),
            ),
            ("each supertype, card type, and subtype", Determiner::Each),
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
                panic!(
                    "expected one shared-determiner group: {:#?}",
                    parsed.noun_phrase()
                );
            };
            assert_eq!(coordinated.determiner, determiner);
            assert_eq!(coordinated.rest.len(), 2);
            assert!(coordinated.complements.is_empty());
            assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
        }
    }

    #[test]
    fn parallel_relatives_remain_on_their_own_group_members() {
        let source = "target permanent you control or suspended card you own";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!("expected one target group: {:#?}", parsed.noun_phrase());
        };
        assert!(matches!(
            coordinated.first.complements.as_slice(),
            [NominalComplement::Relative(_)]
        ));
        assert!(matches!(
            coordinated.rest.as_slice(),
            [crate::syntax::NominalPhraseCoordination {
                phrase: NominalPhrase { complements, .. },
                ..
            }] if matches!(complements.as_slice(), [NominalComplement::Relative(_)])
        ));
        assert!(coordinated.complements.is_empty());
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn one_pp_bearing_closing_member_stays_inside_the_shared_group() {
        let source = "the type and amount of mana";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!("expected one definite group: {:#?}", parsed.noun_phrase());
        };
        assert_eq!(coordinated.determiner, Determiner::The);
        assert!(matches!(
            coordinated.rest.as_slice(),
            [crate::syntax::NominalPhraseCoordination {
                phrase: NominalPhrase { complements, .. },
                ..
            }] if matches!(complements.as_slice(), [NominalComplement::Prepositional(_)])
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn restrictive_with_pp_stays_on_the_closing_shared_member() {
        let source = "target artifact, enchantment, or creature with flying";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!("expected one target group: {:#?}", parsed.noun_phrase());
        };
        assert!(coordinated.complements.is_empty());
        assert!(matches!(
            coordinated.rest.as_slice(),
            [
                crate::syntax::NominalPhraseCoordination { .. },
                crate::syntax::NominalPhraseCoordination {
                    phrase: NominalPhrase { complements, .. },
                    ..
                }
            ] if matches!(
                complements.as_slice(),
                [NominalComplement::Prepositional(crate::syntax::PrepositionalPhrase::Simple(crate::syntax::SimplePrepositionalPhrase {
                    preposition: crate::syntax::Preposition::With,
                    ..
                }))]
            )
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn oxford_nominal_list_carries_one_shared_determiner() {
        let source = "target Goblin, Human, Kraken, Leviathan, Octopus, Serpent, or artifact";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected an Oxford nominal list under one determiner: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(coordinated.determiner, Determiner::Target(None));
        assert!(coordinated.first.determiner.is_none());
        assert_eq!(coordinated.rest.len(), 6);
        assert!(
            coordinated
                .rest
                .iter()
                .all(|member| member.phrase.determiner.is_none())
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn adversarial_oxford_group_has_an_inner_coordination_constituent() {
        let source = "an Elf, Orc, or Equipment";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected the article to scope over one nominal coordination: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(
            coordinated.determiner,
            Determiner::Indefinite(IndefiniteArticle::An)
        );
        let [orc, equipment] = coordinated.rest.as_slice() else {
            panic!("expected three coordinated nominals: {coordinated:#?}");
        };
        assert!(orc.phrase.determiner.is_none());
        assert!(equipment.phrase.determiner.is_none());
        assert!(
            parsed
                .constituent_spans()
                .iter()
                .any(|span| { span.text(source) == Some("Elf, Orc, or Equipment") }),
            "the completed determinerless coordination needs its own span: {:?}",
            parsed.constituent_spans()
        );
        assert!(
            !parsed
                .constituent_spans()
                .iter()
                .any(|span| span.text(source) == Some("Elf, Orc")),
            "the open list prefix is not a constituent: {:?}",
            parsed.constituent_spans()
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn adjacent_shared_determiner_groups_do_not_flatten() {
        let source = "target artifact, creature, or planeswalker and target land or battle";
        let parsed = parse(source);
        let Some(NounPhrase::Coordinated(outer)) = parsed.noun_phrase() else {
            panic!(
                "expected coordination between two target groups: {:#?}",
                parsed.noun_phrase()
            );
        };
        let NounPhrase::CoordinatedNominal(left) = outer.first.as_ref() else {
            panic!("expected the Oxford group on the left: {outer:#?}");
        };
        assert_eq!(left.determiner, Determiner::Target(None));
        assert_eq!(left.rest.len(), 2);
        let [right] = outer.rest.as_slice() else {
            panic!("expected exactly one outer conjunct: {outer:#?}");
        };
        assert_eq!(
            right.conjunction,
            Some(crate::syntax::NounPhraseConjunction::And)
        );
        let NounPhrase::CoordinatedNominal(right) = &right.phrase else {
            panic!("the repeated determiner must begin a second group: {right:#?}");
        };
        assert_eq!(right.determiner, Determiner::Target(None));
        assert_eq!(right.rest.len(), 1);
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn bare_plural_coordinates_as_a_complete_noun_phrase() {
        let source = "a creature and artifacts";
        let parsed = parse(source);
        assert!(
            matches!(parsed.noun_phrase(), Some(NounPhrase::Coordinated(_))),
            "a bare plural needs no shared determiner: {:#?}",
            parsed.noun_phrase()
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn attached_role_exception_is_typed_in_the_lowering_rewrite() {
        let first = parse("this creature")
            .noun_phrase()
            .expect("fixture noun phrase")
            .clone();
        for (source, shared) in [
            ("tapped creature", true),
            ("equipped creature", false),
            ("enchanted creature", false),
        ] {
            let next = parse(source)
                .noun_phrase()
                .expect("fixture noun phrase")
                .clone();
            let lowered = super::super::lowering::push_noun_phrase_coordination(
                first.clone(),
                crate::syntax::NounPhraseCoordination {
                    conjunction: Some(crate::syntax::NounPhraseConjunction::Or),
                    comma: false,
                    phrase: next,
                },
            );
            assert_eq!(
                matches!(lowered, NounPhrase::CoordinatedNominal(_)),
                shared,
                "{source}: {lowered:#?}"
            );
        }
    }

    #[test]
    fn shared_determiner_coordination_stays_inside_its_preposition() {
        let source = "two counters on up to one target creature or artifact";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(counters)) = parsed.noun_phrase() else {
            panic!(
                "the coordination must not attach to the whole counter phrase: {:#?}",
                parsed.noun_phrase()
            );
        };
        let [NominalComplement::Prepositional(recipient)] = counters.complements.as_slice() else {
            panic!("expected one recipient preposition: {counters:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(recipient) = recipient.head().object.as_ref() else {
            panic!("expected a noun-phrase recipient: {recipient:#?}");
        };
        let NounPhrase::CoordinatedNominal(recipient) = recipient.as_ref() else {
            panic!("expected shared-determiner recipient heads: {recipient:#?}");
        };
        assert!(
            matches!(
                recipient.determiner,
                Determiner::Target(Some(crate::syntax::Quantity::UpTo(_)))
            ),
            "expected the quantity and target marker on the shared group: {recipient:#?}"
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn later_determiner_begins_a_nested_shared_group() {
        let source = "this creature or another creature or artifact";
        let parsed = parse(source);
        let Some(NounPhrase::Coordinated(outer)) = parsed.noun_phrase() else {
            panic!(
                "expected an outer coordination: {:#?}",
                parsed.noun_phrase()
            );
        };
        let [group] = outer.rest.as_slice() else {
            panic!("the later determiner must begin one grouped member: {outer:#?}");
        };
        let NounPhrase::CoordinatedNominal(group) = &group.phrase else {
            panic!("expected a nested shared-determiner group: {group:#?}");
        };
        assert_eq!(group.determiner, Determiner::Another);
        assert_eq!(group.rest.len(), 1);
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn shared_determiner_oxford_list_stays_inside_its_preposition() {
        let source = "two damage to each artifact, creature, and land";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(damage)) = parsed.noun_phrase() else {
            panic!(
                "the list must not attach to the whole damage phrase: {:#?}",
                parsed.noun_phrase()
            );
        };
        let [NominalComplement::Prepositional(recipient)] = damage.complements.as_slice() else {
            panic!("expected one recipient preposition: {damage:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(recipient) = recipient.head().object.as_ref() else {
            panic!("expected a noun-phrase recipient: {recipient:#?}");
        };
        let NounPhrase::CoordinatedNominal(recipient) = recipient.as_ref() else {
            panic!("expected shared Oxford recipient heads: {recipient:#?}");
        };
        assert_eq!(recipient.determiner, Determiner::Each);
        assert_eq!(recipient.rest.len(), 2);
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn number_constrained_determiner_scopes_over_plural_members() {
        for source in [
            "two artifacts and creatures",
            "all artifacts and creatures",
            "all artifacts and Equipment",
            "all creatures and Spacecraft",
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
                panic!(
                    "expected invariant nominal coordination under all: {:#?}",
                    parsed.noun_phrase()
                );
            };
            assert_eq!(coordinated.rest.len(), 1);
            assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
        }
    }

    #[test]
    fn unconstrained_determiner_does_not_claim_a_complete_bare_plural() {
        let source = "the creature and artifacts";
        let parsed = parse(source);
        assert!(
            matches!(parsed.noun_phrase(), Some(NounPhrase::Coordinated(_))),
            "number-neutral `the` does not prove shared scope: {:#?}",
            parsed.noun_phrase()
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn plural_shared_scope_does_not_reopen_a_relative_closed_member() {
        let first = parse("all creatures you control")
            .noun_phrase()
            .expect("fixture noun phrase")
            .clone();
        let next = parse("colors")
            .noun_phrase()
            .expect("fixture noun phrase")
            .clone();
        let lowered = super::super::lowering::push_noun_phrase_coordination(
            first,
            crate::syntax::NounPhraseCoordination {
                conjunction: Some(crate::syntax::NounPhraseConjunction::Or),
                comma: false,
                phrase: next,
            },
        );
        assert!(matches!(lowered, NounPhrase::Coordinated(_)));
    }

    #[test]
    fn repeated_target_determiners_coordinate_complete_noun_phrases() {
        let source = "target artifact and target land";
        let parsed = parse(source);
        let Some(NounPhrase::Coordinated(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected coordination of two complete noun phrases: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert!(matches!(
            coordinated.first.as_ref(),
            NounPhrase::Nominal(first)
                if first.determiner == Some(Determiner::Target(None))
        ));
        assert!(matches!(
            coordinated.rest.as_slice(),
            [crate::syntax::NounPhraseCoordination {
                conjunction: Some(crate::syntax::NounPhraseConjunction::And),
                phrase: NounPhrase::Nominal(next),
                ..
            }] if next.determiner == Some(Determiner::Target(None))
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn coordinated_type_modifiers_do_not_become_shared_target_heads() {
        let source = "target artifact or land card";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected one card nominal: {:#?}", parsed.noun_phrase());
        };
        assert_eq!(nominal.determiner, Some(Determiner::Target(None)));
        assert!(matches!(
            nominal.modifiers.as_slice(),
            [NominalModifier::Coordinated(_)]
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn coordinated_type_modifier_lists_keep_their_common_head() {
        for source in [
            "each artifact and/or creature card",
            "a basic artifact, creature, or land card",
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!(
                    "expected one card nominal for {source}: {:#?}",
                    parsed.noun_phrase()
                );
            };
            assert!(matches!(
                nominal.head,
                NounInstance::Singular(Noun::Word(Vocab::Card))
            ));
            assert!(
                nominal
                    .modifiers
                    .iter()
                    .any(|modifier| matches!(modifier, NominalModifier::Coordinated(_))),
                "expected a coordinated modifier under the common head: {nominal:#?}"
            );
            assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
        }
    }

    #[test]
    fn coordinated_rules_types_keep_a_following_common_head() {
        let source = "an instant or sorcery spell";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected one spell nominal: {:#?}", parsed.noun_phrase());
        };
        assert!(matches!(
            nominal.modifiers.as_slice(),
            [NominalModifier::Coordinated(_)]
        ));
        assert!(matches!(
            nominal.head,
            NounInstance::Singular(Noun::Word(Vocab::Spell))
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn keyword_ability_options_keep_their_counter_common_head() {
        let source = "a first strike, vigilance, or lifelink counter";
        let catalogs = fixture_catalogs().with_catalog(
            CatalogKind::KeywordAbility,
            ["First strike", "Vigilance", "Lifelink"],
        );
        let parsed = parse_nonterminal(source, &catalogs, Nonterminal::NounPhrase)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        let Some(NounPhrase::Nominal(counter)) = parsed.noun_phrase() else {
            panic!(
                "expected one common-head counter: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert!(matches!(
            counter.modifiers.as_slice(),
            [NominalModifier::Coordinated(_)]
        ));
        assert!(matches!(
            counter.head,
            NounInstance::Singular(Noun::Word(Vocab::Counter))
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn participial_modifier_coordination_keeps_its_common_head() {
        let source = "target attacking or blocking creature";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected one creature nominal: {:#?}", parsed.noun_phrase());
        };
        assert_eq!(nominal.determiner, Some(Determiner::Target(None)));
        assert!(matches!(
            nominal.modifiers.as_slice(),
            [NominalModifier::Coordinated(_)]
        ));
        assert!(matches!(
            &nominal.head,
            NounInstance::Singular(Noun::Catalog(atom)) if atom.canonical() == "Creature"
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn productive_agent_nouns_remain_shared_determiner_options() {
        let source = "target attacker or blocker";
        let parsed = parse(source);
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected productive agent nouns in one target group: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(coordinated.determiner, Determiner::Target(None));
        assert!(matches!(
            coordinated.first.head,
            NounInstance::Singular(Noun::Agentive(_))
        ));
        assert!(matches!(
            coordinated.rest.as_slice(),
            [crate::syntax::NominalPhraseCoordination {
                phrase: NominalPhrase {
                    head: NounInstance::Singular(Noun::Agentive(_)),
                    ..
                },
                ..
            }]
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn shared_options_do_not_swallow_the_outer_hosts_following_preposition() {
        let source = "their choice of the top or bottom of their library";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(choice)) = parsed.noun_phrase() else {
            panic!("expected one choice nominal: {:#?}", parsed.noun_phrase());
        };
        let [
            NominalComplement::Prepositional(options),
            NominalComplement::Prepositional(library),
        ] = choice.complements.as_slice()
        else {
            panic!("both `of` phrases must remain on choice: {choice:#?}");
        };
        assert_eq!(options.head().preposition, crate::syntax::Preposition::Of);
        assert_eq!(library.head().preposition, crate::syntax::Preposition::Of);
        let crate::syntax::Phrase::NounPhrase(options) = options.head().object.as_ref() else {
            panic!("expected noun-phrase options: {options:#?}");
        };
        assert!(matches!(
            options.as_ref(),
            NounPhrase::CoordinatedNominal(coordinated)
                if coordinated.determiner == Determiner::The
        ));
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn shared_determiner_does_not_reopen_a_pp_closed_nominal() {
        let source = "the power to target player or planeswalker";
        let parsed = parse(source);
        let Some(NounPhrase::Nominal(power)) = parsed.noun_phrase() else {
            panic!(
                "the outer determiner must stay on power: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(power.determiner, Some(Determiner::The));
        let [NominalComplement::Prepositional(recipient)] = power.complements.as_slice() else {
            panic!("power must retain its recipient PP: {power:#?}");
        };
        assert_eq!(recipient.head().preposition, crate::syntax::Preposition::To);
        let crate::syntax::Phrase::NounPhrase(recipient) = recipient.head().object.as_ref() else {
            panic!("expected a noun-phrase recipient: {recipient:#?}");
        };
        let NounPhrase::CoordinatedNominal(recipient) = recipient.as_ref() else {
            panic!("target must scope over the recipient coordination: {recipient:#?}");
        };
        assert_eq!(recipient.determiner, Determiner::Target(None));
        assert_eq!(recipient.rest.len(), 1);
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn noun_phrase_set_exceptions_preserve_scope_marker_and_comma() {
        for (source, expected_marker, comma, included_coordination, excluded_coordination) in [
            (
                "all creatures except artifacts",
                SetExceptionMarker::Bare,
                false,
                false,
                false,
            ),
            (
                "all creatures except for artifacts, creatures, and lands",
                SetExceptionMarker::For,
                false,
                false,
                true,
            ),
            (
                "each creature except Goblins",
                SetExceptionMarker::Bare,
                false,
                false,
                false,
            ),
            (
                "all creatures and lands except for Goblins",
                SetExceptionMarker::For,
                false,
                true,
                false,
            ),
            (
                "all creatures, except for Goblins",
                SetExceptionMarker::For,
                true,
                false,
                false,
            ),
        ] {
            let parsed = parse(source);
            let Some(NounPhrase::SetException(exception)) = parsed.noun_phrase() else {
                panic!("expected a set-exception noun phrase for {source:?}");
            };
            assert_eq!(exception.marker, expected_marker);
            assert_eq!(exception.comma, comma);
            assert_eq!(
                matches!(
                    exception.included.as_ref(),
                    NounPhrase::Coordinated(_) | NounPhrase::CoordinatedNominal(_)
                ),
                included_coordination
            );
            assert_eq!(
                matches!(exception.excluded.as_ref(), NounPhrase::Coordinated(_)),
                excluded_coordination
            );
            assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
        }
    }

    #[test]
    fn noun_phrase_set_exception_requires_a_set_denoting_host() {
        for source in [
            "creatures except artifacts",
            "a card except the first one",
            "all creatures except artifacts except lands",
        ] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase).is_err(),
                "inadmissible set-exception host parsed: {source:?}"
            );
        }
    }

    #[test]
    fn oxford_head_list_records_comma_and_final_conjunction_per_member() {
        // The interior members carry a comma with no conjunction; the final
        // member carries the closing conjunction with its comma — the exact
        // surface the renderer replays.
        let parsed = parse("target artifact, creature, or land");
        let Some(NounPhrase::CoordinatedNominal(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected nominal coordination under one determiner: {:#?}",
                parsed.noun_phrase()
            );
        };
        assert_eq!(coordinated.determiner, Determiner::Target(None));
        let [interior, final_member] = coordinated.rest.as_slice() else {
            panic!("expected two continuations, got {:#?}", coordinated.rest);
        };
        assert_eq!(interior.conjunction, None);
        assert!(interior.comma);
        assert_eq!(
            final_member.conjunction,
            Some(crate::syntax::NounPhraseConjunction::Or)
        );
        assert!(final_member.comma);
    }

    #[test]
    fn coordinated_modifier_is_one_outer_node_with_conjuncts() {
        // The owner's invariant applied to coordination: `white and blue Goblin`
        // is ONE `Coordinated` modifier slot carrying the color conjuncts, with
        // `Goblin` following as a separate ordinary noun modifier — not a
        // per-mechanism list type and not a flattened run of sibling modifiers.
        let parsed = parse("a white and blue Goblin creature");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal");
        };
        let [
            NominalModifier::Coordinated(coordinated),
            NominalModifier::Noun { .. },
        ] = nominal.modifiers.as_slice()
        else {
            panic!("expected [Coordinated, Noun], got {:#?}", nominal.modifiers);
        };
        assert!(matches!(
            coordinated.first.as_ref(),
            NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase: AdjectivePhrase {
                    head: Adjective::Color(ColorWord::White),
                    ..
                },
            }
        ));
        let [member] = coordinated.rest.as_slice() else {
            panic!("expected one continuation");
        };
        assert_eq!(
            member.conjunction,
            Some(crate::syntax::PredicateConjunction::And)
        );
        assert!(!member.comma);
        assert!(matches!(
            &member.modifier,
            NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase: AdjectivePhrase {
                    head: Adjective::Color(ColorWord::Blue),
                    ..
                },
            }
        ));
    }

    #[test]
    fn coordinated_modifier_preserves_per_conjunct_polarity() {
        // Coordination of a negated conjunct keeps each conjunct's own polarity
        // (ticket invariant: polarity composes). `nonwhite and nonblue` carries
        // two independently negated color conjuncts.
        let parsed = parse("a nonwhite and nonblue creature");
        let modifier = first_modifier(&parsed);
        let NominalModifier::Coordinated(coordinated) = &modifier else {
            panic!("expected a coordinated modifier, got {modifier:#?}");
        };
        assert_eq!(
            polarity_of(&coordinated.first),
            Some(Polarity::Negative),
            "first conjunct is negated"
        );
        assert_eq!(
            polarity_of(&coordinated.rest[0].modifier),
            Some(Polarity::Negative),
            "second conjunct is negated"
        );
    }

    #[test]
    fn and_or_is_a_structured_conjunction_not_an_opaque_noun() {
        // `and/or` in modifier position lands on the shared conjunction value,
        // not on an opaque noun: the coordinated slot's continuation records
        // `AndOr`, and the group carries exactly the two color conjuncts.
        let parsed = parse("a white and/or blue creature");
        let modifier = first_modifier(&parsed);
        let NominalModifier::Coordinated(coordinated) = &modifier else {
            panic!("expected a coordinated modifier, got {modifier:#?}");
        };
        assert_eq!(
            coordinated.rest[0].conjunction,
            Some(crate::syntax::PredicateConjunction::AndOr)
        );
        assert_eq!(coordinated.rest.len(), 1);
    }

    #[test]
    fn coordinated_indefinite_article_follows_the_first_conjunct() {
        // The a/an of the whole phrase is fixed by the first conjunct's onset:
        // vowel-initial `artifact` takes `an`, and a leading `non-` conjunct
        // takes `a` (consonant onset) regardless of the negated base.
        for source in [
            "an artifact and creature card",
            "a nonartifact and creature card",
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
    fn rules_bundle_words_carry_the_bundle_category() {
        // The attributive adjective and the head noun both land on
        // `RulesBundle` catalog atoms, so the render side can derive the `non-`
        // glyph by category.
        let modified_parse = parse("a modified creature");
        let Some(NounPhrase::Nominal(modified)) = modified_parse.noun_phrase() else {
            panic!("expected a nominal for a modified creature");
        };
        assert!(matches!(
            modified.modifiers.first(),
            Some(NominalModifier::Adjective { phrase, .. })
                if matches!(&phrase.head, Adjective::Catalog(atom) if atom.is_rules_bundle())
        ));

        let outlaw_parse = parse("an outlaw");
        let Some(NounPhrase::Nominal(outlaw)) = outlaw_parse.noun_phrase() else {
            panic!("expected a nominal for an outlaw");
        };
        assert!(matches!(
            &outlaw.head,
            NounInstance::Singular(Noun::Catalog(atom)) if atom.is_rules_bundle()
        ));
    }

    #[test]
    fn negation_render_derives_the_hyphen_from_base_capitalization() {
        // The render side derives the `non-` glyph from whether the rendered
        // base is capitalized, never from a stored flag: the lowercase color
        // adjective base `black` stays solid (`nonblack`), while the
        // capitalized catalog subtype noun base `Human` gets the hyphen
        // (`non-Human`).
        let solid_modifier = NominalModifier::Adjective {
            polarity: Polarity::Negative,
            phrase: AdjectivePhrase {
                degree: None,
                head: Adjective::Color(ColorWord::Black),
                complements: vec![],
            },
        };
        let solid_phrase = NounPhrase::Nominal(NominalPhrase {
            determiner: None,
            modifiers: vec![solid_modifier],
            head: NounInstance::Singular(Noun::Word(Vocab::Card)),
            complements: vec![],
        });
        assert_eq!(render_fragment(&solid_phrase), "nonblack card");

        let parsed = parse("Human creature");
        let Some(NounPhrase::Nominal(human_creature)) = parsed.noun_phrase() else {
            panic!("expected a nominal for Human creature");
        };
        let hyphenated_modifiers = human_creature
            .modifiers
            .iter()
            .cloned()
            .map(|modifier| match modifier {
                NominalModifier::Noun { noun, .. } => NominalModifier::Noun {
                    polarity: Polarity::Negative,
                    noun,
                },
                other => other,
            })
            .collect();
        let hyphenated_phrase = NounPhrase::Nominal(NominalPhrase {
            modifiers: hyphenated_modifiers,
            ..human_creature.clone()
        });
        assert_eq!(render_fragment(&hyphenated_phrase), "non-Human creature");
    }

    #[test]
    fn negation_render_consults_lexical_metadata_when_capitalization_gives_no_signal() {
        // Two lowercase nouns, opposite outcomes: the rules-bundle noun
        // `outlaw` hyphenates by category (`non-outlaw`, Shoot the Sheriff),
        // while an ordinary lowercase vocabulary noun like `combat` is not a
        // bundle and stays solid (`noncombat`) — proving the category drives
        // the hyphen, not a stray default.
        for (noun, expected) in [
            (
                NounInstance::Singular(Noun::Catalog(RulesNominal::Outlaw.atom())),
                "non-outlaw card",
            ),
            (
                NounInstance::Mass(Noun::Word(Vocab::Combat)),
                "noncombat card",
            ),
        ] {
            let modifier = NominalModifier::Noun {
                polarity: Polarity::Negative,
                noun,
            };
            let noun_phrase = NounPhrase::Nominal(NominalPhrase {
                determiner: None,
                modifiers: vec![modifier],
                head: NounInstance::Singular(Noun::Word(Vocab::Card)),
                complements: vec![],
            });
            assert_eq!(render_fragment(&noun_phrase), expected);
        }
    }

    /// The first nominal modifier of a parsed noun phrase, cloned for
    /// polarity-shape comparisons.
    fn first_modifier(parsed: &ParsedNonterminal) -> NominalModifier {
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!(
                "expected a nominal noun phrase: {:#?}",
                parsed.noun_phrase()
            );
        };
        nominal
            .modifiers
            .first()
            .cloned()
            .unwrap_or_else(|| panic!("expected at least one modifier: {nominal:#?}"))
    }

    fn polarity_of(modifier: &NominalModifier) -> Option<Polarity> {
        match modifier {
            NominalModifier::Adjective { polarity, .. }
            | NominalModifier::Noun { polarity, .. } => Some(*polarity),
            NominalModifier::Quantity(_)
            | NominalModifier::PowerToughness(_)
            | NominalModifier::CombatStepName { .. }
            | NominalModifier::Coordinated(_) => None,
        }
    }

    /// Rewrites a modifier's polarity to `Positive`, isolating the "same outer
    /// shape up to the flag" comparison.
    fn depolarize(modifier: &NominalModifier) -> NominalModifier {
        match modifier.clone() {
            NominalModifier::Adjective { phrase, .. } => NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase,
            },
            NominalModifier::Noun { noun, .. } => NominalModifier::Noun {
                polarity: Polarity::Positive,
                noun,
            },
            other => other,
        }
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
    fn devotion_value_nominal_round_trips_with_its_color_argument() {
        // Family: the rules-defined `devotion` value nominal [CR#700.5] with its
        // mandatory `to <color>` argument. A single color and a two-color pair
        // ride the dedicated production; the generic `a/each/that color` shapes
        // ride the ordinary count-noun + prepositional path.
        for source in [
            "your devotion to green",
            "your devotion to black",
            "your devotion to white and black",
            "your devotion to red and green",
            "your devotion to a color",
            "your devotion to each color",
            "your devotion to that color",
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
    fn devotion_carries_its_color_argument_structurally() {
        use crate::syntax::DevotionColors;
        use crate::word::ColorWord;
        let single = parse("your devotion to green");
        let Some(NounPhrase::Nominal(nominal)) = single.noun_phrase() else {
            panic!("expected a devotion nominal");
        };
        assert!(matches!(
            &nominal.head,
            NounInstance::Singular(Noun::Catalog(atom)) if atom.canonical() == "devotion"
        ));
        assert!(matches!(
            nominal.complements.as_slice(),
            [NominalComplement::Devotion(DevotionColors::Color(
                ColorWord::Green
            ))]
        ));

        let pair = parse("your devotion to white and black");
        let Some(NounPhrase::Nominal(pair)) = pair.noun_phrase() else {
            panic!("expected a devotion pair nominal");
        };
        assert!(matches!(
            pair.complements.as_slice(),
            [NominalComplement::Devotion(DevotionColors::Pair(
                ColorWord::White,
                ColorWord::Black
            ))]
        ));

        // `devotion counter` keeps devotion as a plain count-noun modifier.
        let counter = parse("a devotion counter");
        let Some(NounPhrase::Nominal(counter)) = counter.noun_phrase() else {
            panic!("expected a devotion counter nominal");
        };
        assert!(matches!(
            counter.modifiers.as_slice(),
            [NominalModifier::Noun {
                noun: NounInstance::Singular(Noun::Catalog(atom)),
                ..
            }] if atom.canonical() == "devotion"
        ));
    }

    #[test]
    fn superlative_property_nominals_round_trip_with_a_group_complement() {
        // Family: `the greatest/highest/lowest/least <property> among/of
        // <group>`. Superlatives are plain adjectives modifying an existing
        // property nominal (power/toughness/mana value/life total), with the
        // `among`/`of` group as an ordinary prepositional complement.
        for source in [
            "the greatest power among creatures you control",
            "the greatest mana value among permanents you control",
            "the least toughness among creatures you control",
            "the highest life total among all players",
            "the lowest life total among your opponents",
            "the greatest mana value of a commander you own",
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
    fn regular_table_nouns_fill_nominal_head_slots() {
        for (source, expected) in [
            ("maximum hand size", "size"),
            ("the amount of mana", "amount"),
            ("your upkeep", "upkeep"),
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
            let [
                NominalComplement::Adjective(AdjectivePhrase {
                    head,
                    complements,
                    degree: None,
                }),
            ] = nominal.complements.as_slice()
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

    // a_known_literal_lexeme_is_never_opacifiable folded into
    // grammar::mod::litaudit_tests::every_reserved_literal_surface_is_a_known_word
    // (round litaudit), which covers every opacity-reserved literal surface.

    #[test]
    fn self_reference_possessive_is_one_determiner() {
        let parsed = parse_self("Nissa's power");
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
            render_fragment_as(parsed.noun_phrase().unwrap(), "Nissa Revane", true),
            "Nissa's power"
        );
    }

    #[test]
    fn plural_genitive_is_one_possessive_determiner() {
        let parsed = parse("their owners' hands");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(
            render_fragment(parsed.noun_phrase().expect("noun-phrase root")),
            "their owners' hands"
        );
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal for their owners' hands");
        };
        assert!(
            matches!(
                &nominal.head,
                NounInstance::Plural(Noun::Word(word)) if word.spelling() == "hand"
            ),
            "{nominal:#?}"
        );
        let Some(Determiner::Possessive(Possessor::NounPhrase(possessor))) = &nominal.determiner
        else {
            panic!(
                "expected a possessive determiner: {:#?}",
                nominal.determiner
            );
        };
        let NounPhrase::Nominal(possessor_nominal) = possessor.as_ref() else {
            panic!("expected a nominal possessor: {possessor:#?}");
        };
        assert!(
            matches!(
                &possessor_nominal.determiner,
                Some(Determiner::Possessive(Possessor::Pronoun(Pronoun::They)))
            ),
            "{possessor_nominal:#?}"
        );
        assert!(
            matches!(
                &possessor_nominal.head,
                NounInstance::Plural(Noun::Word(word)) if word.spelling() == "owner"
            ),
            "{possessor_nominal:#?}"
        );
        assert!(possessor_nominal.modifiers.is_empty());
        assert!(possessor_nominal.complements.is_empty());
    }

    #[test]
    fn a_bare_plural_noun_is_never_a_possessive() {
        for source in ["opponents", "creatures you control"] {
            let parsed = parse(source);
            let Some(noun_phrase) = parsed.noun_phrase() else {
                panic!("expected a noun phrase for {source:?}");
            };
            assert!(
                !contains_noun_phrase_possessive(noun_phrase),
                "{source:?} must not contain a Possessive(NounPhrase) determiner: {noun_phrase:#?}"
            );
        }
    }

    fn contains_noun_phrase_possessive(noun_phrase: &NounPhrase) -> bool {
        match noun_phrase {
            NounPhrase::Nominal(nominal) => matches!(
                &nominal.determiner,
                Some(Determiner::Possessive(Possessor::NounPhrase(_)))
            ),
            NounPhrase::Possessive(Possessor::NounPhrase(_)) => true,
            _ => false,
        }
    }

    #[test]
    fn except_is_not_an_opaque_noun_head() {
        // Whelming Wave: before Edit C, `except` could be swallowed as an
        // opaque noun head with `hands` demoted to a known-noun modifier,
        // completing a wrong parse. The set-exception production now parses
        // it structurally, but the literal must still never become opaque.
        let source = "Return all creatures to their owners' hands except for Krakens, Leviathans, Octopuses, and Serpents.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
        let debug = format!("{ast:#?}");
        assert!(
            !debug.contains("OpaqueLexeme(\n    \"except\"") && !debug.contains("\"except\","),
            "except must never lower as an OpaqueLexeme: {debug}"
        );
        assert!(
            !debug.contains("Opaque"),
            "no Opaque node may appear at all for this sentence: {debug}"
        );
    }

    #[test]
    fn not_is_not_an_opaque_noun_head() {
        // Consuming Tide: `not` could be swallowed as an opaque plural noun
        // head with `nonland`/`permanents` demoted to known-noun modifiers.
        let source = "Return all nonland permanents not chosen this way to their owners' hands.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
        let debug = format!("{ast:#?}");
        assert!(
            !debug.contains("Opaque"),
            "no Opaque node may appear at all for this sentence: {debug}"
        );
    }

    #[test]
    fn a_singular_noun_ending_in_s_keeps_the_s_genitive() {
        // `chaos` (Vocab::Chaos) is a Mass noun whose lemma already ends in
        // `s`. The plural-only filter must refuse the bare-apostrophe arm for
        // it, since `chaos` never yields a Plural WordMatch: `chaos'` must not
        // parse as a possessive noun phrase at all.
        let result = parse_nonterminal(
            "chaos' effects",
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
        );
        let refused_possessive = match result {
            Err(_) => true,
            Ok(parsed) => {
                !contains_noun_phrase_possessive(parsed.noun_phrase().expect("noun-phrase root"))
            }
        };
        assert!(
            refused_possessive,
            "a singular/mass noun ending in s must not be licensed by the bare-apostrophe arm"
        );
    }

    #[test]
    fn premodified_possessive_scopes_modifier_inside_possessor() {
        // Causal pair: `that creature's toughness` (bare possessor, already
        // correct) versus this premodified possessor, which round `opqposs`
        // adds a production for.
        let source = "the sacrificed creature's power";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let Some(NounPhrase::Nominal(outer)) = parsed.noun_phrase() else {
            panic!("expected a nominal phrase for {source:?}");
        };
        assert!(
            matches!(&outer.head, NounInstance::Singular(Noun::Word(word)) if word.spelling() == "power"),
            "{outer:#?}"
        );
        assert!(outer.modifiers.is_empty(), "{outer:#?}");
        let Some(Determiner::Possessive(Possessor::NounPhrase(possessor))) = &outer.determiner
        else {
            panic!("expected a possessive determiner: {:#?}", outer.determiner);
        };
        let NounPhrase::Nominal(possessor) = possessor.as_ref() else {
            panic!("expected a nominal possessor: {possessor:#?}");
        };
        assert!(
            matches!(&possessor.determiner, Some(Determiner::The)),
            "{possessor:#?}"
        );
        let [
            NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase,
            },
        ] = possessor.modifiers.as_slice()
        else {
            panic!("expected exactly one positive adjective modifier: {possessor:#?}");
        };
        assert!(
            matches!(
                &phrase.head,
                Adjective::Participle(Tense::Past, Verb::Word(vocab)) if vocab.spelling() == "sacrifice"
            ),
            "{phrase:#?}"
        );
        assert!(
            matches!(
                &possessor.head,
                NounInstance::Singular(Noun::Catalog(atom)) if atom.canonical() == "Creature"
            ),
            "{possessor:#?}"
        );
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn adjective_and_participle_possessors_share_one_spine() {
        for (source, expected_head) in [
            ("the next turn's upkeep", "next"),
            ("enchanted creature's controller", "enchant"),
            ("each other player's speed", "other"),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let Some(NounPhrase::Nominal(outer)) = parsed.noun_phrase() else {
                panic!("expected a nominal phrase for {source:?}");
            };
            assert!(outer.modifiers.is_empty(), "{source}: {outer:#?}");
            let Some(Determiner::Possessive(Possessor::NounPhrase(possessor))) = &outer.determiner
            else {
                panic!(
                    "{source}: expected a possessive determiner: {:#?}",
                    outer.determiner
                );
            };
            let NounPhrase::Nominal(possessor) = possessor.as_ref() else {
                panic!("{source}: expected a nominal possessor: {possessor:#?}");
            };
            assert_eq!(possessor.modifiers.len(), 1, "{source}: {possessor:#?}");
            let modifier_spelling = match &possessor.modifiers[0] {
                NominalModifier::Adjective {
                    polarity: Polarity::Positive,
                    phrase,
                } => match &phrase.head {
                    Adjective::Word(vocab) | Adjective::Participle(_, Verb::Word(vocab)) => {
                        vocab.spelling()
                    }
                    other => panic!("{source}: unexpected adjective head {other:?}"),
                },
                other => panic!("{source}: expected an adjective modifier, got {other:?}"),
            };
            assert_eq!(modifier_spelling, expected_head, "{source}");
            assert_eq!(
                render_fragment(parsed.noun_phrase().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn premodified_possessors_enforce_indefinite_article_agreement() {
        for source in ["an exiled card's owner", "a sacrificed creature's power"] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_fragment(parsed.noun_phrase().unwrap()),
                source,
                "{source}"
            );
        }
        for source in ["a exiled card's owner", "an sacrificed creature's power"] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase).is_err(),
                "{source:?} must be rejected on indefinite-article agreement"
            );
        }
    }

    #[test]
    fn premodified_plural_genitive_uses_a_bare_apostrophe() {
        let source = "the sacrificed creatures' controllers";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let Some(NounPhrase::Nominal(outer)) = parsed.noun_phrase() else {
            panic!("expected a nominal phrase for {source:?}");
        };
        let Some(Determiner::Possessive(Possessor::NounPhrase(possessor))) = &outer.determiner
        else {
            panic!("expected a possessive determiner: {:#?}", outer.determiner);
        };
        let NounPhrase::Nominal(possessor) = possessor.as_ref() else {
            panic!("expected a nominal possessor: {possessor:#?}");
        };
        assert!(
            matches!(
                &possessor.head,
                NounInstance::Plural(Noun::Catalog(atom)) if atom.canonical() == "Creature"
            ),
            "{possessor:#?}"
        );
        let debug = format!("{parsed:#?}");
        assert!(!debug.contains("Opaque"), "{debug}");
        assert_eq!(render_fragment(parsed.noun_phrase().unwrap()), source);
    }

    #[test]
    fn a_possessive_modifier_cannot_precede_a_determined_child() {
        assert!(
            parse_nonterminal(
                "sacrificed the creature's power",
                &fixture_catalogs(),
                Nonterminal::NounPhrase,
            )
            .is_err(),
            "a possessive-adjective prepend must not attach after a determiner already \
             completed the possessor"
        );
    }

    #[test]
    fn self_reference_possessive_can_stand_as_an_elliptical_noun_phrase() {
        let parsed = parse_self("Nissa's");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert!(matches!(
            parsed.noun_phrase(),
            Some(NounPhrase::Possessive(Possessor::NounPhrase(possessor)))
                if matches!(possessor.as_ref(), NounPhrase::ThisCard(_))
        ));
        assert_eq!(
            render_fragment_as(parsed.noun_phrase().unwrap(), "Nissa Revane", true),
            "Nissa's"
        );
    }

    #[test]
    fn comparison_inside_preposition_stays_with_its_object() {
        let parsed =
            parse_self("target creature card with mana value less than or equal to Nissa's power");
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let Some(NounPhrase::Nominal(card)) = parsed.noun_phrase() else {
            panic!("expected a card nominal");
        };
        let [NominalComplement::Prepositional(with)] = card.complements.as_slice() else {
            panic!("comparison escaped its prepositional object: {card:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(object) = with.head().object.as_ref() else {
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
            Some(Determiner::Target(Some(crate::syntax::Quantity::UpTo(
                crate::syntax::QuantityValue::Literal(number)
            ))))
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
                crate::syntax::QuantityValue::Literal(number),
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
                NominalModifier::Adjective { .. },
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
                head: crate::syntax::PartitiveHead::Quantity(crate::syntax::Quantity::MoreThan(
                    crate::syntax::QuantityValue::Literal(one)
                )),
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
            [NominalModifier::Adjective {
                phrase: AdjectivePhrase {
                    head: Adjective::Word(Vocab::Other),
                    ..
                },
                ..
            }]
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
                NominalModifier::Adjective {
                    phrase: AdjectivePhrase {
                        head: Adjective::Catalog(legendary),
                        ..
                    },
                    ..
                },
                NominalModifier::Noun {
                    noun: NounInstance::Singular(Noun::Catalog(goblin)),
                    ..
                },
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
        let crate::syntax::Phrase::NounPhrase(object) = preposition.head().object.as_ref() else {
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
                if preposition.head().preposition == crate::syntax::Preposition::Among
                    && matches!(
                        preposition.head().object.as_ref(),
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
                if outer.head().preposition == crate::syntax::Preposition::From
                    && matches!(
                        outer.head().object.as_ref(),
                        crate::syntax::Phrase::PrepositionalPhrase(inner)
                            if inner.head().preposition == crate::syntax::Preposition::Among
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
                    preposition.head().object.as_ref(),
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
                        if preposition.head().preposition == expected
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
                matches!(bound, Quantity::OrComparison(crate::syntax::QuantityValue::Literal(number), seen)
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
                    Some(Determiner::Quantity(Quantity::OrComparison(
                        crate::syntax::QuantityValue::Literal(number),
                        seen
                    )))
                        if number.value == value && *seen == word
                ),
                "{source}: {nominal:#?}"
            );
        }
    }

    #[test]
    fn variable_x_bounds_are_the_variable_not_roman_ten() {
        use crate::syntax::Quantity;
        use crate::syntax::QuantityValue;
        // Causal pairs: the variable `X` against the literal that renders
        // identically (`up to ten`) and against an ordinary literal bound.
        // `X` is a placeholder for a number to be determined ([CR#107.3]);
        // `Numeral::Roman` merely happens to spell ten `X`.
        for (source, expected) in [
            (
                "up to X target creatures",
                Quantity::UpTo(QuantityValue::Variable),
            ),
            (
                "up to ten target creatures",
                Quantity::UpTo(QuantityValue::Literal(crate::syntax::NumberLiteral {
                    value: 10,
                    numeral: crate::numeral::Numeral::Cardinal,
                })),
            ),
            (
                "up to three target creatures",
                Quantity::UpTo(QuantityValue::Literal(crate::syntax::NumberLiteral {
                    value: 3,
                    numeral: crate::numeral::Numeral::Cardinal,
                })),
            ),
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
            assert_eq!(
                nominal.determiner,
                Some(Determiner::Target(Some(expected))),
                "{source}: {nominal:#?}"
            );
        }
    }

    #[test]
    fn variable_x_comparative_bounds_are_the_variable() {
        use crate::syntax::ComparativeWord;
        use crate::syntax::Quantity;
        use crate::syntax::QuantityValue;
        // `power X or less` -> OrComparison(Variable, Less); mirrored against
        // an ordinary literal floor and ceiling.
        for (source, value, word) in [
            (
                "creatures with power X or less",
                QuantityValue::Variable,
                ComparativeWord::Less,
            ),
            (
                "creatures with power 2 or less",
                QuantityValue::Literal(crate::syntax::NumberLiteral {
                    value: 2,
                    numeral: crate::numeral::Numeral::Arabic(false),
                }),
                ComparativeWord::Less,
            ),
            (
                "creatures with power 4 or greater",
                QuantityValue::Literal(crate::syntax::NumberLiteral {
                    value: 4,
                    numeral: crate::numeral::Numeral::Arabic(false),
                }),
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
            assert_eq!(
                bound,
                Quantity::OrComparison(value, word),
                "{source}: {bound:?}"
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
        let crate::syntax::Phrase::NounPhrase(object) = with.head().object.as_ref() else {
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

    #[test]
    fn jace_ordinal_coordination_recovers() {
        // Jace Reawakened's During-PP object shape, parsed directly as a
        // noun phrase: a plural head with an Oxford-coordinated run of
        // ordinal adjectives, not a `CoordinatedNounPhrase`/`Opaque` misparse.
        let parsed = parse("your first, second, or third turns of the game");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal, got {:#?}", parsed.noun_phrase());
        };
        assert_eq!(
            nominal.determiner,
            Some(Determiner::Possessive(Possessor::Pronoun(Pronoun::You)))
        );
        let [NominalModifier::Coordinated(coordinated)] = nominal.modifiers.as_slice() else {
            panic!(
                "expected a single coordinated modifier, got {:#?}",
                nominal.modifiers
            );
        };
        assert!(
            matches!(
                coordinated.first.as_ref(),
                NominalModifier::Adjective {
                    polarity: Polarity::Positive,
                    phrase: AdjectivePhrase {
                        head: Adjective::Ordinal(1),
                        ..
                    },
                }
            ),
            "expected first conjunct Ordinal(1), got {:#?}",
            coordinated.first
        );
        let [second, third] = coordinated.rest.as_slice() else {
            panic!("expected two continuations, got {:#?}", coordinated.rest);
        };
        assert!(second.comma, "second conjunct is comma-joined");
        assert_eq!(second.conjunction, None);
        assert!(matches!(
            &second.modifier,
            NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase: AdjectivePhrase {
                    head: Adjective::Ordinal(2),
                    ..
                },
            }
        ));
        assert!(third.comma);
        assert_eq!(
            third.conjunction,
            Some(crate::syntax::PredicateConjunction::Or)
        );
        assert!(matches!(
            &third.modifier,
            NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase: AdjectivePhrase {
                    head: Adjective::Ordinal(3),
                    ..
                },
            }
        ));
        assert_eq!(nominal.head, NounInstance::Plural(Noun::Word(Vocab::Turn)));
        let [NominalComplement::Prepositional(of)] = nominal.complements.as_slice() else {
            panic!(
                "expected a single `of` complement, got {:#?}",
                nominal.complements
            );
        };
        assert_eq!(of.head().preposition, crate::syntax::Preposition::Of);
    }

    #[test]
    fn ordinal_singular_head_recovers() {
        // Starting Town's copular complement: same coordinated-modifier
        // shape, but a singular head noun (`turn`) rather than plural.
        let parsed = parse("your first, second, or third turn of the game");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal, got {:#?}", parsed.noun_phrase());
        };
        assert!(matches!(
            nominal.modifiers.as_slice(),
            [NominalModifier::Coordinated(_)]
        ));
        assert_eq!(
            nominal.head,
            NounInstance::Singular(Noun::Word(Vocab::Turn))
        );
    }

    #[test]
    fn binary_ordinal_recovers() {
        // Lady Octopus / Rose Room Treasurer: a two-way (no Oxford comma)
        // ordinal coordination riding `CoordinatedModifierConjoined`.
        let parsed = parse("your first or second card");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal, got {:#?}", parsed.noun_phrase());
        };
        let [NominalModifier::Coordinated(coordinated)] = nominal.modifiers.as_slice() else {
            panic!(
                "expected a single coordinated modifier, got {:#?}",
                nominal.modifiers
            );
        };
        assert!(matches!(
            coordinated.first.as_ref(),
            NominalModifier::Adjective {
                phrase: AdjectivePhrase {
                    head: Adjective::Ordinal(1),
                    ..
                },
                ..
            }
        ));
        let [only] = coordinated.rest.as_slice() else {
            panic!("expected one continuation, got {:#?}", coordinated.rest);
        };
        assert!(!only.comma, "binary `or` coordination has no comma");
        assert_eq!(
            only.conjunction,
            Some(crate::syntax::PredicateConjunction::Or)
        );
        assert!(matches!(
            &only.modifier,
            NominalModifier::Adjective {
                phrase: AdjectivePhrase {
                    head: Adjective::Ordinal(2),
                    ..
                },
                ..
            }
        ));
        assert_eq!(
            nominal.head,
            NounInstance::Singular(Noun::Word(Vocab::Card))
        );
    }

    #[test]
    fn bare_single_ordinal_attributive() {
        // "your first turn" — a single bare ordinal rides the ordinary
        // `NominalModifier::Adjective` path, not `Coordinated`.
        let parsed = parse("your first turn");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal, got {:#?}", parsed.noun_phrase());
        };
        let [NominalModifier::Adjective { phrase, .. }] = nominal.modifiers.as_slice() else {
            panic!(
                "expected a single adjective modifier, got {:#?}",
                nominal.modifiers
            );
        };
        assert_eq!(phrase.head, Adjective::Ordinal(1));
        assert_eq!(
            nominal.head,
            NounInstance::Singular(Noun::Word(Vocab::Turn))
        );
    }

    #[test]
    fn ordinal_adjective_initial_sound() {
        // Guards the `adjective_initial_sound` fallthrough (renderer.rs):
        // the indefinite article picked for an ordinal-headed nominal must
        // match the ordinal's *spelled* initial sound, not the parsed word's.
        for source in ["an eighth copy", "a first copy"] {
            let parsed = parse(source);
            assert_eq!(
                render_fragment(parsed.noun_phrase().expect("noun-phrase root")),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn ordinal_coordination_round_trips_byte_exactly() {
        for source in [
            "your first, second, or third turns of the game",
            "your first, second, or third turn of the game",
            "your first or second card",
            "your first turn",
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
    fn first_strike_keyword_not_displaced_by_ordinal() {
        // The catalog keyword-ability atom "first strike" must keep winning
        // over an ordinal-adjective + bare-noun misreading of "first"
        // "strike": the completed keyword-ability reading wins structurally
        // (zero opacity), with no dispreference needed on the ordinal edge.
        let catalogs = Catalogs::default().with_catalog(
            CatalogKind::KeywordAbility,
            ["Flying", "First strike", "Protection"],
        );
        let report = crate::parse::parse_with_identity(
            "Flying, first strike, protection from red",
            &catalogs,
            "Test Card",
            false,
        );
        let AbilityKind::Keyword(list) = &report.ast.abilities[0].kind else {
            panic!(
                "expected a keyword list, got {:#?}",
                report.ast.abilities[0].kind
            );
        };
        assert_eq!(list.abilities.len(), 3);
        assert_eq!(list.abilities[1].ability.canonical(), "First strike");
    }

    #[test]
    fn ordinal_adjective_does_not_disturb_numeral_reading() {
        // A quantity/numeral shape that must stay a `NumberLiteral` (not
        // reparsed as an ordinal adjective): the `Exact` quantity determiner
        // on a plural head.
        let parsed = parse("two cards");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal, got {:#?}", parsed.noun_phrase());
        };
        assert!(
            matches!(
                nominal.determiner,
                Some(Determiner::Quantity(crate::syntax::Quantity::Exact(number)))
                    if number.value == 2
            ),
            "expected an Exact(2) quantity determiner, got {:?}",
            nominal.determiner
        );
        assert!(
            nominal.modifiers.iter().all(|modifier| !matches!(
                modifier,
                NominalModifier::Adjective {
                    phrase: AdjectivePhrase {
                        head: Adjective::Ordinal(_),
                        ..
                    },
                    ..
                }
            )),
            "no ordinal adjective modifier should appear: {:#?}",
            nominal.modifiers
        );
    }

    #[test]
    fn ordinal_attributive_does_not_overfire() {
        // A plain adjective + noun run that is not an ordinal run must not
        // spuriously coordinate into `NominalModifier::Coordinated`.
        let parsed = parse("a white creature");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal, got {:#?}", parsed.noun_phrase());
        };
        assert!(
            !nominal
                .modifiers
                .iter()
                .any(|modifier| matches!(modifier, NominalModifier::Coordinated(_))),
            "a single plain adjective must not coordinate: {:#?}",
            nominal.modifiers
        );
    }

    fn parse(source: &str) -> ParsedNonterminal {
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    /// Parses a noun phrase as the legendary face `Nissa Revane` (nickname
    /// `Nissa`), so a `Nissa`/`Nissa's` self-reference is recognized.
    fn parse_self(source: &str) -> ParsedNonterminal {
        parse_nonterminal_with_self_reference(
            source,
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
            &SelfReference::new("Nissa Revane", true),
        )
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(
                CatalogKind::CreatureType,
                [
                    "Goblin",
                    "Human",
                    "Elf",
                    "Kraken",
                    "Leviathan",
                    "Octopus",
                    "Orc",
                    "Serpent",
                ],
            )
            .with_catalog(CatalogKind::ArtifactType, ["Equipment", "Spacecraft"])
            .with_catalog(
                CatalogKind::CardType,
                ["Artifact", "Creature", "Instant", "Land", "Sorcery"],
            )
            .with_catalog(CatalogKind::Supertype, ["Basic", "Legendary"])
    }

    #[test]
    fn indefinite_articles_agree_with_a_power_toughness_modifier() {
        // Causal pairs: the vowel-onset powers the corpus prints (`X`, `8`)
        // against the consonant-onset powers it prints (`1`, `0`, `+1`, `9`).
        // The pair is read aloud — "an ex-ex", "an eight-eight", "a one-one",
        // "a zero-zero", "a plus-one-plus-one" — so the agreement is with the
        // pronunciation, not with the character.
        for source in [
            "an X/X green Goblin creature token",
            "an 8/8 blue Goblin creature token",
            "an X/1 red Goblin creature token",
            "a 1/1 white Goblin creature token",
            "a 0/0 green Goblin creature token",
            "a +1/+1 Goblin creature token",
            "a 9/9 blue Goblin creature token",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let noun_phrase = parsed.noun_phrase().expect("noun-phrase root");
            assert_eq!(render_fragment(noun_phrase), source, "{source}");
            let NounPhrase::Nominal(nominal) = noun_phrase else {
                panic!("{source}: expected a Nominal noun phrase, got {noun_phrase:?}");
            };
            assert!(
                matches!(nominal.determiner, Some(Determiner::Indefinite(_))),
                "{source}: expected an indefinite determiner, got {:?}",
                nominal.determiner
            );
            assert!(
                matches!(
                    nominal.modifiers.first(),
                    Some(NominalModifier::PowerToughness(_))
                ),
                "{source}: expected the first modifier to be PowerToughness, got {:?}",
                nominal.modifiers.first()
            );
        }
    }

    #[test]
    fn a_x_x_and_an_1_1_are_rejected() {
        // The over-fire check: the corpus prints zero `a X/X` and zero
        // `an 1/1` witnesses, and the gate must reject both.
        assert!(
            parse_nonterminal(
                "a X/X green Goblin creature token",
                &fixture_catalogs(),
                Nonterminal::NounPhrase,
            )
            .is_err(),
            "`a X/X ...` must not parse"
        );
        assert!(
            parse_nonterminal(
                "an 1/1 white Goblin creature token",
                &fixture_catalogs(),
                Nonterminal::NounPhrase,
            )
            .is_err(),
            "`an 1/1 ...` must not parse"
        );
        // Zero is read "zero" — a table keyed on "is a digit" dies here.
        assert!(
            parse_nonterminal(
                "an 0/0 green Goblin creature token",
                &fixture_catalogs(),
                Nonterminal::NounPhrase,
            )
            .is_err(),
            "`an 0/0 ...` must not parse"
        );
    }

    fn render_fragment(noun_phrase: &NounPhrase) -> String {
        render_fragment_as(noun_phrase, "Test Card", false)
    }

    fn render_fragment_as(noun_phrase: &NounPhrase, name: &str, is_legendary: bool) -> String {
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    flavor_header: None,
                    sentences: vec![Sentence {
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
                                    distributive_each: false,
                                },
                                kind: crate::syntax::Transitive {
                                    pre_object_elements: vec![],
                                    object: PredicateObject::NounPhrase(noun_phrase.clone()),
                                },
                                elements: vec![],
                            }),
                        )),
                    }],
                }),
            }],
        };
        ast.render(name, is_legendary)
            .expect("parsed noun phrase must render")
            .strip_prefix("Draw ")
            .and_then(|rendered| rendered.strip_suffix('.'))
            .expect("fixture wrapper is stable")
            .to_owned()
    }
}
