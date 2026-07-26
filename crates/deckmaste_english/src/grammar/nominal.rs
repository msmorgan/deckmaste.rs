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
    use crate::syntax::TransitivePredicate;
    use crate::word::Adjective;
    use crate::word::ColorWord;
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
                        if matches!(participants, NounInstance::Plural(Noun::Word(_)))
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
    fn oxford_head_list_records_comma_and_final_conjunction_per_member() {
        // The interior members carry a comma with no conjunction; the final
        // member carries the closing conjunction with its comma — the exact
        // surface the renderer replays.
        let parsed = parse("target artifact, creature, or land");
        let Some(NounPhrase::Coordinated(coordinated)) = parsed.noun_phrase() else {
            panic!(
                "expected a coordinated noun phrase: {:#?}",
                parsed.noun_phrase()
            );
        };
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
    // grammar::mod::litaudit_tests::every_reserved_literal_is_a_known_word
    // (round litaudit), which covers the full OPACITY_RESERVED_LITERALS table.

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
        // completing a wrong parse. It must now stay honestly recovered.
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
        assert_eq!(of.preposition, crate::syntax::Preposition::Of);
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
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Human"])
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::Supertype, ["Basic", "Legendary"])
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
