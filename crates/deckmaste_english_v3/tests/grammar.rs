mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

#[test]
fn authored_vocabulary_composes_across_the_grammar() {
    for text in [
        "Draw cards.",
        "You draw cards.",
        "Each creature attacks.",
        "Those creatures attack.",
        "Draw cards from them.",
        "If you draw cards, draw cards.",
        "Draw cards. If you do, draw cards.",
        "If you do, draw cards.",
        "Creatures that attack draw cards.",
        "Creatures you control attack.",
        "Creatures that you control attack.",
        "Creatures you draw and discard attack.",
        "You and they draw cards.",
        "You may draw cards.",
        "Cards are drawn.",
        "Cards have been drawn.",
        "Target creature attacks.",
        "Target white creatures attack.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
}

#[test]
fn agreement_case_frames_and_local_ellipsis_are_admission_constraints() {
    for text in [
        "Each creatures attack.",
        "Those creature attacks.",
        "Creatures attacks.",
        "You draws cards.",
        "Them draw cards.",
        "Draw they.",
        "Draw cards from they.",
        "Creatures that attacks draw cards.",
        "Creatures you controls attack.",
        "Draw.",
        "You may draws cards.",
        "White target creatures attack.",
        ".",
        "If you do, .",
        "Draw cards and .",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid Reading for {text:?}"
        );
    }
}

fn word(id: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability: None,
        frame: None,
    }
}

#[test]
fn independent_named_nominal_retains_selected_frame_and_catalog_identity() {
    let mut noun = word(
        "lexeme:type/artifact",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        },
    );
    noun.countability = Some(true);
    let mut named = word(
        "lexeme:Verb/Name",
        WordForm::PastParticiple,
        FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..FeatureBundle::default()
        },
    );
    named.frame = Some(1);
    let name = word(
        "catalog:card-names.txt/Powerstone Shard",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let complement = Reading::CatalogName {
        form: 0,
        head: name.clone(),
    };
    assert_eq!(
        readings("Powerstone Shard", Category::Name),
        BTreeSet::from([complement.clone()])
    );
    let predicate = Reading::PassiveNamePredicate {
        form: 0,
        head: named.clone(),
        complement: Box::new(complement),
    };
    assert_eq!(
        readings("named Powerstone Shard", Category::NamePredicate),
        BTreeSet::from([predicate.clone()])
    );
    let value = Reading::NamedNominal {
        form: 0,
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun.clone(),
        }),
        modifier: Box::new(predicate),
    };
    assert_eq!(
        value.realize(lexicon()).unwrap(),
        "artifact named Powerstone Shard"
    );
    assert_eq!(
        readings("artifact named Powerstone Shard", Category::Nominal),
        BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(leaves, [noun, named.clone(), name]);
    named.frame = Some(0);
    assert!(
        Reading::PassiveNamePredicate {
            form: 0,
            head: named,
            complement: Box::new(Reading::CatalogName {
                form: 0,
                head: word(
                    "catalog:card-names.txt/Powerstone Shard",
                    WordForm::Invariant,
                    FeatureBundle::default()
                ),
            }),
        }
        .admit(lexicon())
        .is_err()
    );
    for text in [
        "named artifact",
        "named powerstone shard",
        "named NoSuchCard",
        "naming Powerstone Shard",
        "names Powerstone Shard",
        "spent Powerstone Shard",
        "named Powerstone Shard Powerstone Shard",
    ] {
        assert!(
            readings(text, Category::NamePredicate).is_empty(),
            "{text:?}"
        );
    }
}

#[test]
fn independent_participial_premodifier_preserves_verb_form_and_distribution() {
    let mut noun = word(
        "lexeme:CommonNoun/Player",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        },
    );
    noun.countability = Some(true);
    let mut modifier = word(
        "lexeme:Verb/Defend",
        WordForm::GerundParticiple,
        FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..FeatureBundle::default()
        },
    );
    modifier.frame = Some(0);
    let value = Reading::ParticipialPremodifier {
        form: 0,
        modifier: Box::new(Reading::VerbalPremodifier {
            form: 0,
            head: modifier.clone(),
        }),
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun.clone(),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "defending player");
    assert_eq!(
        readings("defending player", Category::Nominal),
        BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(leaves, [modifier.clone(), noun.clone()]);
    modifier.frame = Some(1);
    assert!(
        Reading::ParticipialPremodifier {
            form: 0,
            modifier: Box::new(Reading::VerbalPremodifier {
                form: 0,
                head: modifier
            }),
            head: Box::new(Reading::Noun {
                form: 0,
                head: noun
            }),
        }
        .admit(lexicon())
        .is_err()
    );
    for text in [
        "defend player",
        "defends player",
        "defended player",
        "spent player",
        "defending target player",
    ] {
        assert!(readings(text, Category::Nominal).is_empty(), "{text:?}");
    }
    assert!(readings("defending", Category::AdjectivePhrase).is_empty());
    assert!(!readings("target defending player", Category::NounPhrase).is_empty());
    assert!(!readings("Defending players attack.", Category::Document).is_empty());
}

#[test]
fn lexical_gap_words_reach_existing_compositional_hosts() {
    for (text, category) in [
        ("in addition", Category::PrepositionPhrase),
        ("in addition to other types", Category::PrepositionPhrase),
        ("the game", Category::NounPhrase),
        ("games", Category::NounPhrase),
        ("no mana was spent", Category::FiniteClause),
        (
            "Artifacts named Powerstone Shard attack.",
            Category::Document,
        ),
    ] {
        assert!(!readings(text, category).is_empty(), "{text:?}");
    }
    for (text, category) in [
        ("game", Category::NounPhrase),
        ("much game", Category::NounPhrase),
        ("no mana was spended", Category::FiniteClause),
    ] {
        assert!(readings(text, category).is_empty(), "{text:?}");
    }
}

#[test]
fn independent_amount_complement_consumes_its_frame_without_losing_countability() {
    let mut amount = word(
        "lexeme:CommonNoun/Amount",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        },
    );
    amount.countability = Some(true);
    amount.frame = Some(1);
    let marker = word(
        "vocab:Preposition/Of",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let symbol = word(
        "vocab:FixedCostSymbol/Green",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let symbols = Reading::CostSymbols {
        form: 0,
        first: Box::new(Reading::NamedCostSymbol {
            form: 0,
            symbol: symbol.clone(),
        }),
        rest: vec![],
    };
    let value = Reading::SymbolComplementNominal {
        form: 0,
        head: amount.clone(),
        marker: marker.clone(),
        complement: Box::new(symbols.clone()),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "amount of {G}");
    assert_eq!(
        readings("amount of {G}", Category::Nominal),
        BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(leaves, [amount.clone(), marker.clone(), symbol]);
    assert!(
        Reading::Noun {
            form: 0,
            head: amount.clone()
        }
        .admit(lexicon())
        .is_err()
    );
    amount.frame = Some(0);
    let bare = Reading::BareFramedNoun {
        form: 0,
        head: amount.clone(),
    };
    assert_eq!(
        readings("amount", Category::Nominal),
        BTreeSet::from([bare])
    );
    assert!(
        Reading::SymbolComplementNominal {
            form: 0,
            head: amount,
            marker,
            complement: Box::new(symbols),
        }
        .admit(lexicon())
        .is_err()
    );
    for text in [
        "the amount of {G}",
        "amounts of {G}",
        "the amount of {G}{G}",
    ] {
        assert!(!readings(text, Category::NounPhrase).is_empty(), "{text:?}");
    }
    for text in [
        "amount of",
        "amount {G}",
        "amount with {G}",
        "game of {G}",
        "amount of {G}/",
        "amount of {}",
        "amount of {{G}}",
    ] {
        assert!(readings(text, Category::Nominal).is_empty(), "{text:?}");
    }
    assert!(readings("much amount of {G}", Category::NounPhrase).is_empty());
    assert!(!readings("Add the amount of {G}.", Category::Document).is_empty());
}

#[test]
fn independently_constructed_auxiliary_ellipsis_roundtrips_without_discourse_context() {
    let mut auxiliary = word(
        "core-verb:Do",
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..FeatureBundle::default()
        },
    );
    auxiliary.frame = Some(1);
    let value = Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::NominativePronoun {
                form: 0,
                head: word(
                    "vocab:SubjectPronoun/You",
                    WordForm::Invariant,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(Person::Second),
                        case: Some(Case::Nominative),
                        ..FeatureBundle::default()
                    },
                ),
            }),
        }),
        predicate: Box::new(Reading::BareAuxiliaryPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: auxiliary,
            complement: Box::new(Reading::BareEllipsis {
                form: 0,
                omission: Box::new(Reading::OmittedPlain { form: 0 }),
            }),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "you do");
    assert_eq!(
        readings("you do", Category::FiniteClause),
        BTreeSet::from([value.clone()])
    );
    let mut before = vec![];
    value
        .visit_words(&mut |word| before.push(word.clone()))
        .unwrap();
    assert_eq!(before.len(), 2, "ellipsis invents no lexical material");
    let mut after = vec![];
    readings("you do", Category::FiniteClause)
        .get(&value)
        .unwrap()
        .visit_words(&mut |word| after.push(word.clone()))
        .unwrap();
    assert_eq!(before, after);
}

#[test]
fn counts_and_measures_interact_with_nominals_frames_and_prepositions() {
    for text in [
        "Draw two cards.",
        "Draw thirteen cards.",
        "Draw 1,000 cards.",
        "Draw X cards.",
        "Two creatures attack.",
        "Two target creatures attack.",
        "You gain 3 life.",
        "You gain life equal to 3.",
        "Surveil 2.",
        "Surveil X.",
        "Surveil 2 plus 2.",
        "Creatures with power 3 attack.",
        "Your maximum hand size is increased by two.",
        "You gain X plus 3 life.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
    for reading in readings(
        "Your maximum hand size is increased by two.",
        Category::Document,
    ) {
        let mut extents = vec![];
        reading
            .visit(&mut |node| {
                if let Reading::SelectedExtentPassive { complement, .. } = node {
                    extents.push((
                        node.realize(lexicon()).unwrap(),
                        complement.category(),
                        complement.realize(lexicon()).unwrap(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(
            extents,
            vec![(
                "increased by two".into(),
                Category::ScalarExtentComplement,
                "by two".into()
            )]
        );
    }
    assert!(readings("Creatures attack by 2 plus 2.", Category::Document).is_empty());
    for text in [
        "Draw two card.",
        "Draw 2 cards.",
        "Draw II cards.",
        "Draw X card.",
        "Draw two damage.",
        "You gain three life.",
        "Surveil two.",
        "Surveil II.",
        "Surveil 1000.",
        "You gain life equal than 3.",
        "You gain life greater to 3.",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid Reading for {text:?}"
        );
    }
    assert!(!readings("second", Category::AdjectivePhrase).is_empty());
    assert!(readings("second", Category::Cardinal).is_empty());
    assert!(readings("second", Category::MeasurePhrase).is_empty());
}

fn numeral(value: i32, notation: deckmaste_lexical::Numeral) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        countability: None,
        frame: None,
    }
}

#[test]
fn independent_measure_values_preserve_operator_structure_and_notation() {
    use deckmaste_lexical::Numeral;
    let value = Reading::ArithmeticMeasure {
        form: 0,
        left: Box::new(Reading::GroupedScalarNumeral {
            form: 0,
            head: numeral(1000, Numeral::Arabic(true)),
        }),
        operator: word(
            "vocab:Preposition/Minus",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
        right: Box::new(Reading::ScalarVariable {
            form: 0,
            head: word(
                "vocab:Variable/X",
                WordForm::Invariant,
                FeatureBundle::default(),
            ),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "1,000 minus X");
    assert!(readings("1,000 minus X", Category::MeasurePhrase).contains(&value));
    let wrong_notation = Reading::GroupedScalarNumeral {
        form: 0,
        head: numeral(1000, Numeral::Arabic(false)),
    };
    assert!(wrong_notation.admit(lexicon()).is_err());
    assert!(readings("1000", Category::MeasurePhrase).is_empty());
    let grouped = Reading::GroupedScalarNumeral {
        form: 0,
        head: numeral(2, Numeral::Arabic(true)),
    };
    let ungrouped = Reading::UngroupedScalarNumeral {
        form: 0,
        head: numeral(2, Numeral::Arabic(false)),
    };
    let alternatives = readings("2", Category::MeasurePhrase);
    assert!(grouped.admit(lexicon()).is_err());
    assert_eq!(ungrouped.realize(lexicon()).unwrap(), "2");
    assert_eq!(alternatives, BTreeSet::from([ungrouped]));
}

#[test]
fn small_digits_do_not_multiply_a_predicate_reading() {
    assert_eq!(readings("Gain 1 life.", Category::Document).len(), 1);
}

fn unsigned_scalar(value: i32) -> Reading {
    Reading::SmallUnsignedScalar {
        form: 0,
        head: numeral(value, deckmaste_lexical::Numeral::Arabic(false)),
    }
}

#[test]
fn slash_pairs_preserve_ordered_components_and_explicit_signs() {
    let pair = Reading::SlashPair {
        form: 0,
        left: Box::new(Reading::PositiveScalar {
            form: 0,
            value: Box::new(unsigned_scalar(3)),
        }),
        right: Box::new(Reading::NegativeScalar {
            form: 0,
            value: Box::new(unsigned_scalar(2)),
        }),
    };
    assert_eq!(pair.realize(lexicon()).unwrap(), "+3/-2");
    assert_eq!(
        readings("+3/-2", Category::SlashPair),
        BTreeSet::from([pair.clone()])
    );
    let measure = Reading::SlashMeasure {
        form: 0,
        pair: Box::new(pair),
    };
    assert_eq!(
        readings("+3/-2", Category::MeasurePhrase),
        BTreeSet::from([measure])
    );
    for surface in ["1/1", "X/X", "+3/+3", "+X/-X", "-0/+0", "0/1", "1,000/2"] {
        assert_eq!(
            readings(surface, Category::MeasurePhrase).len(),
            1,
            "{surface}"
        );
    }
    for surface in [
        "1/", "/1", "1/1/1", "+-3/2", "--3/2", "3/++2", "1 /1", "1/ 1", "one/one", "*/*", "1000/2",
    ] {
        assert!(
            readings(surface, Category::MeasurePhrase).is_empty(),
            "{surface}"
        );
    }
    let doubled_sign = Reading::PositiveScalar {
        form: 0,
        value: Box::new(unsigned_scalar(-3)),
    };
    assert!(doubled_sign.admit(lexicon()).is_err());
    assert!(readings("by +3/+3", Category::PrepositionPhrase).is_empty());
}

#[test]
fn slash_pairs_compose_with_nominal_modifiers_and_selected_predicates() {
    for surface in ["X 1/1 tokens", "X X/X tokens", "two +1/-1 creatures"] {
        let values = readings(surface, Category::NounPhrase);
        assert!(!values.is_empty(), "{surface}");
        for value in values {
            let Reading::CountedNounPhrase { quantity, head, .. } = value else {
                panic!("{value:?}")
            };
            assert_eq!(
                quantity.realize(lexicon()).unwrap(),
                surface.split(' ').next().unwrap()
            );
            let Reading::SlashModifiedNominal { modifier, .. } = *head else {
                panic!("{head:?}")
            };
            assert_eq!(
                modifier.realize(lexicon()).unwrap(),
                surface.split(' ').nth(1).unwrap()
            );
        }
    }
    for surface in ["Target creature gets +3/+3.", "Creatures get -1/-1."] {
        assert!(
            !readings(surface, Category::Document).is_empty(),
            "{surface}"
        );
    }
    for surface in [
        "Gain 1/1 life.",
        "Target creature gets 3.",
        "X1/1 tokens",
        "X 1/1/1 tokens",
    ] {
        let category =
            if surface.ends_with('.') { Category::Document } else { Category::NounPhrase };
        assert!(readings(surface, category).is_empty(), "{surface}");
    }
}

#[test]
fn independent_slash_consumers_preserve_count_components_and_leaf_order() {
    let x = word(
        "vocab:Variable/X",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let pair = Reading::SlashPair {
        form: 0,
        left: Box::new(Reading::UnsignedScalar {
            form: 0,
            value: Box::new(unsigned_scalar(1)),
        }),
        right: Box::new(Reading::UnsignedScalar {
            form: 0,
            value: Box::new(unsigned_scalar(1)),
        }),
    };
    let mut tokens = word(
        "lexeme:CommonNoun/Token",
        WordForm::Plural,
        FeatureBundle {
            number: Some(Number::Plural),
            ..FeatureBundle::default()
        },
    );
    tokens.countability = Some(true);
    let value = Reading::CountedNounPhrase {
        form: 0,
        quantity: Box::new(Reading::CardinalDeterminer {
            form: 0,
            value: Box::new(Reading::VariableCount {
                form: 0,
                head: x.clone(),
            }),
        }),
        head: Box::new(Reading::SlashModifiedNominal {
            form: 0,
            modifier: Box::new(pair),
            head: Box::new(Reading::Noun {
                form: 0,
                head: tokens.clone(),
            }),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "X 1/1 tokens");
    assert_eq!(
        readings("X 1/1 tokens", Category::NounPhrase),
        BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        [
            x,
            numeral(1, deckmaste_lexical::Numeral::Arabic(false)),
            numeral(1, deckmaste_lexical::Numeral::Arabic(false)),
            tokens
        ]
    );

    let mut get = word(
        "core-verb:Get",
        WordForm::Plain,
        FeatureBundle {
            finiteness: None,
            ..FeatureBundle::default()
        },
    );
    get.frame = Some(0);
    let value = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: get.clone(),
        complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
            Box::new(Reading::SlashPair {
                form: 0,
                left: Box::new(Reading::PositiveScalar {
                    form: 0,
                    value: Box::new(unsigned_scalar(3)),
                }),
                right: Box::new(Reading::NegativeScalar {
                    form: 0,
                    value: Box::new(unsigned_scalar(2)),
                }),
            }),
        )],
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "get +3/-2");
    assert_eq!(
        readings("get +3/-2", Category::SecondaryVerbPhrase),
        BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        [
            get,
            numeral(3, deckmaste_lexical::Numeral::Arabic(false)),
            numeral(2, deckmaste_lexical::Numeral::Arabic(false))
        ]
    );
}

#[test]
fn a_selected_adjective_frame_requires_its_complement_in_both_directions() {
    use deckmaste_lexical::Numeral;
    let mut head = word(
        "vocab:ScalarDegree/Equal",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    head.frame = Some(0);
    let value = Reading::ComparativeAdjectivePhrase {
        form: 0,
        governor: Box::new(Reading::ComparativeGovernor {
            form: 0,
            head: head.clone(),
            marker: word("vocab:Preposition/To", WordForm::Invariant, FeatureBundle::default()),
        }),
        complement: Box::new(Reading::ScalarComparativeComplement {
            form: 0,
            value: Box::new(Reading::ScalarMeasurePhrase {
                form: 0,
                head: Box::new(Reading::UngroupedScalarNumeral {
                    form: 0,
                    head: numeral(2, Numeral::Arabic(false)),
                }),
            }),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "equal to 2");
    assert!(readings("equal to 2", Category::ComparativeAdjectivePhrase).contains(&value));
    assert!(
        Reading::Adjective { form: 0, head }
            .admit(lexicon())
            .is_err()
    );
    assert!(readings("equal", Category::AdjectivePhrase).is_empty());
    assert!(readings("equal than 2", Category::ComparativeAdjectivePhrase).is_empty());
    assert!(!readings("greater than 2", Category::ComparativeAdjectivePhrase).is_empty());
}

#[test]
fn document_collections_connect_keywords_costs_modes_quotes_and_clauses() {
    for text in [
        "Flying",
        "Flying, vigilance; haste",
        "Ward {2}{U}",
        "Frenzy 2",
        "Flying (Draw cards.)",
        "{T}: Draw cards.",
        "{2}{U}, Discard cards: Draw cards. Draw cards.",
        "Landfall — If you draw cards, draw cards.",
        "Draw cards. (Discard cards.) Draw cards.",
        "Creatures have flying.",
        "Creatures gain ward {2}.",
        "Creatures have \"Draw cards.\"",
        "Creatures have “{T}: Draw cards.”",
        "Choose one —\n• Draw cards.\n• Discard cards.",
        "{T}: Choose one —\n• Draw cards.\n• Discard cards.",
        "Flying\n{T}: Draw cards.\nLandfall — Draw cards.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
    for text in [
        "Ward",
        "Frenzy",
        "Flying 2",
        "Flying {2}",
        "Ward two",
        "Landfall",
        "{two}: Draw cards.",
        "{P}: Draw cards.",
        ": Draw cards.",
        "{T}, : Draw cards.",
        "Flying, ",
        "()",
        "Choose one —\n",
        "Choose one —\n• ",
        "Creatures have “Draw cards.\".",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid Reading for {text:?}"
        );
    }
}

#[test]
fn independent_keyword_collection_preserves_parameters_separators_and_leaf_identity() {
    let head = |identity: &str| word(identity, WordForm::Invariant, FeatureBundle::default());
    let mut flying = head("lexeme:keyword_ability/flying");
    let LexicalReading::Word(value) = &mut flying.value else {
        panic!("expected declared word");
    };
    value.capitalization = SurfaceCase::Initial;
    let ward = head("lexeme:keyword_ability/ward");
    let tap = head("vocab:FixedCostSymbol/Tap");
    let symbols = Reading::CostSymbols {
        form: 0,
        first: Box::new(Reading::NumericCostSymbol {
            form: 0,
            number: numeral(2, deckmaste_lexical::Numeral::Arabic(false)),
        }),
        rest: vec![Reading::NamedCostSymbol {
            form: 0,
            symbol: tap.clone(),
        }],
    };
    let value = Reading::KeywordLine {
        form: 0,
        first: Box::new(Reading::BareKeyword {
            form: 0,
            head: flying.clone(),
        }),
        rest: vec![Reading::KeywordContinuation {
            form: 1,
            keyword: Box::new(Reading::CostKeyword {
                form: 0,
                head: ward.clone(),
                cost: Box::new(symbols.clone()),
            }),
        }],
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "Flying; ward {2}{T}");
    assert_eq!(
        readings("Flying; ward {2}{T}", Category::Ability),
        BTreeSet::from([value.clone()])
    );
    let mut visited = Vec::new();
    value
        .visit_words(&mut |word| visited.push(word.clone()))
        .unwrap();
    assert_eq!(
        visited,
        [
            flying.clone(),
            ward.clone(),
            numeral(2, deckmaste_lexical::Numeral::Arabic(false)),
            tap
        ]
    );
    assert!(
        Reading::BareKeyword {
            form: 0,
            head: ward
        }
        .admit(lexicon())
        .is_err()
    );
    assert!(
        Reading::CostKeyword {
            form: 0,
            head: flying,
            cost: Box::new(symbols)
        }
        .admit(lexicon())
        .is_err()
    );
}

#[test]
fn target_verb_and_targeting_marker_retain_distinct_roles_in_relatives() {
    let text = "Spells that target target creatures attack.";
    let values = readings(text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut roles = Vec::new();
        value
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value
                    && (value.lexeme == "lexeme:Verb/Target"
                        || value.lexeme == "vocab:TargetingMarker/Target")
                {
                    roles.push(value.clone());
                }
            })
            .unwrap();
        assert_eq!(roles.len(), 2);
        assert_eq!(roles[0].lexeme, "lexeme:Verb/Target");
        assert_eq!(roles[0].form, WordForm::Present);
        assert_eq!(roles[0].features.number, Some(Number::Plural));
        assert_eq!(roles[0].features.person, Some(Person::Third));
        assert_eq!(roles[1].lexeme, "vocab:TargetingMarker/Target");
        assert_eq!(roles[1].form, WordForm::Invariant);
        assert_eq!(roles[1].features, FeatureBundle::default());
    }
    assert!(!readings("Target creatures.", Category::Document).is_empty());
    assert!(!readings("Target creatures attack.", Category::Document).is_empty());
    assert!(readings("Spells that targets creatures attack.", Category::Document).is_empty());
}

#[test]
fn nested_document_admission_uses_a_normal_worker_stack() {
    let mut head = word(
        "core-verb:Attack",
        WordForm::Plain,
        FeatureBundle {
            finiteness: None,
            ..FeatureBundle::default()
        },
    );
    head.frame = Some(0);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
    let mut paragraph = Reading::Paragraph {
        form: 0,
        first: Box::new(Reading::SentenceItem {
            form: 0,
            sentence: Box::new(Reading::Sentence {
                form: 0,
                clause: Box::new(Reading::Imperative {
                    form: 0,
                    predicate: Box::new(Reading::BarePredicate {
                        form: 0,
                        head: Box::new(Reading::SelectedPredicate {
                            category: Category::SecondaryVerbPhrase,
                            form: 0,
                            head,
                            complements: vec![],
                        }),
                    }),
                }),
            }),
        }),
        rest: vec![],
    };
    for _ in 0..16 {
        paragraph = Reading::Paragraph {
            form: 0,
            first: Box::new(Reading::ParentheticalItem {
                form: 0,
                parenthetical: Box::new(Reading::Parenthetical {
                    form: 0,
                    body: Box::new(paragraph),
                }),
            }),
            rest: vec![],
        };
    }
    let expected = format!("{}Attack.{}", "(".repeat(16), ")".repeat(16));
    assert_eq!(paragraph.realize(lexicon()).unwrap(), expected);
    assert!(readings(&expected, Category::Paragraph).contains(&paragraph));
}

#[test]
fn type_lines_require_ordered_nonempty_groups_from_declared_vocabulary() {
    for text in [
        "Instant",
        "Artifact Creature — Golem",
        "Basic Snow Land — Island",
        "Legendary Artifact Creature — Human Wizard",
        "Enchantment Creature — Human Soldier",
        "Kindred Instant — Elf",
        "Battle — Siege",
    ] {
        assert!(!readings(text, Category::TypeLine).is_empty(), "{text:?}");
    }
    for text in [
        "",
        "Legendary",
        "Legendary — Elf",
        "Creature Legendary — Elf",
        "Elf — Creature",
        "Creature —",
        "Creature — Legendary",
        "Creature Elf",
        "creature — Elf",
        "Creature - Elf",
        "Creature — NoSuchSubtype",
    ] {
        assert!(readings(text, Category::TypeLine).is_empty(), "{text:?}");
    }
}

#[test]
fn independent_type_line_retains_flat_groups_and_lexical_identity() {
    let catalog_word = |id| word(id, WordForm::Invariant, FeatureBundle::default());
    let legendary = catalog_word("catalog:supertypes.txt/Legendary");
    let artifact = catalog_word("catalog:card-types.txt/Artifact");
    let creature = catalog_word("catalog:card-types.txt/Creature");
    let human = catalog_word("catalog:creature-types.txt/Human");
    let wizard = catalog_word("catalog:creature-types.txt/Wizard");
    let value = Reading::SubtypedLine {
        form: 0,
        supertypes: vec![Reading::SupertypePrefix {
            form: 0,
            head: legendary.clone(),
        }],
        types: Box::new(Reading::CardTypes {
            form: 0,
            head: artifact.clone(),
            rest: vec![Reading::CardTypeContinuation {
                form: 0,
                head: creature.clone(),
            }],
        }),
        subtypes: Box::new(Reading::Subtypes {
            form: 0,
            head: human.clone(),
            rest: vec![Reading::SubtypeContinuation {
                form: 0,
                head: wizard.clone(),
            }],
        }),
    };
    let expected = "Legendary Artifact Creature — Human Wizard";
    assert_eq!(value.realize(lexicon()).unwrap(), expected);
    assert_eq!(
        readings(expected, Category::TypeLine),
        BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        [legendary, artifact.clone(), creature, human, wizard]
    );
    assert!(
        Reading::SupertypePrefix {
            form: 0,
            head: artifact
        }
        .admit(lexicon())
        .is_err()
    );
}

#[test]
fn complemented_adjectives_are_postpositive_in_both_directions() {
    // CGEL Ch. 6 §3.3, pp. 550–553: complements constrain attributive position.
    let mut adjective = word(
        "vocab:ScalarDegree/Equal",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    adjective.frame = Some(0);
    let modifier = Reading::ComparativeAdjective {
        form: 0,
        phrase: Box::new(Reading::ComparativeAdjectivePhrase {
            form: 0,
            governor: Box::new(Reading::ComparativeGovernor {
                form: 0,
                head: adjective,
                marker: word("vocab:Preposition/To", WordForm::Invariant, FeatureBundle::default()),
            }),
            complement: Box::new(Reading::ScalarComparativeComplement {
                form: 0,
                value: Box::new(Reading::ScalarMeasurePhrase {
                    form: 0,
                    head: Box::new(Reading::UngroupedScalarNumeral {
                        form: 0,
                        head: numeral(2, deckmaste_lexical::Numeral::Arabic(false)),
                    }),
                }),
            }),
        }),
    };
    let mut noun = word(
        "lexeme:CommonNoun/Amount",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        },
    );
    noun.frame = Some(0);
    noun.countability = Some(true);
    let head = Reading::BareFramedNoun {
        form: 0,
        head: noun,
    };
    let valid = Reading::PostpositiveNominal {
        form: 0,
        head: Box::new(head.clone()),
        modifier: Box::new(modifier.clone()),
    };
    assert_eq!(valid.realize(lexicon()).unwrap(), "amount equal to 2");
    assert_eq!(
        readings("amount equal to 2", Category::Nominal),
        BTreeSet::from([valid])
    );
    let invalid = Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(modifier),
        head: Box::new(head),
    };
    assert!(invalid.admit(lexicon()).is_err());
    assert!(invalid.realize(lexicon()).is_err());
    assert!(readings("an equal to 2 amount", Category::NounPhrase).is_empty());
    assert!(!readings("an amount equal to 2", Category::NounPhrase).is_empty());
    assert!(!readings("a white creature", Category::NounPhrase).is_empty());
    assert!(readings("a creature white", Category::NounPhrase).is_empty());
}

#[test]
fn integrated_pp_modifiers_stay_inside_the_nominal() {
    // CGEL Ch. 5 §14.2, pp. 444–447; external modifiers have separate
    // licensing.
    let values = readings("the creature on the battlefield", Category::NounPhrase);
    assert_eq!(values.len(), 1);
    let Reading::DeterminedNounPhrase { head, .. } = values.into_iter().next().unwrap() else {
        panic!("determiner must include the restriction")
    };
    assert!(matches!(*head, Reading::PostmodifiedNominal { .. }));
    assert!(!readings("Draw creatures on the battlefield.", Category::Document).is_empty());
}

#[test]
fn copular_location_has_a_selected_complement_reading() {
    // CGEL Ch. 4 §5.2, pp. 257–260: locative complement, not predicative or
    // adjunct.
    for text in [
        "You are on the battlefield.",
        "Cards are in the graveyard.",
        "You aren't on the battlefield.",
        "Be on the battlefield.",
    ] {
        let values = readings(text, Category::Document);
        let mut selected = Vec::new();
        for value in &values {
            value
                .visit(&mut |node| {
                    if matches!(
                        node,
                        Reading::SelectedPredicate {
                            category: Category::FinitePredicate,
                            form: 0,
                            complements, ..
                        } | Reading::SelectedPredicate {
                            category: Category::SecondaryVerbPhrase,
                            form: 0,
                            complements, ..
                        } if matches!(complements.as_slice(), [deckmaste_english_v3::grammar::FrameValue::Argument(child)] if child.category() == Category::LocativeComplement)
                    ) {
                        selected.push(node.clone());
                    }
                })
                .unwrap();
        }
        assert!(
            !selected.is_empty(),
            "missing selected location for {text:?}"
        );
        for mut value in selected {
            let (Reading::SelectedPredicate {
                category: Category::FinitePredicate,
                form: 0,
                head,
                ..
            }
            | Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head,
                ..
            }) = &mut value
            else {
                unreachable!()
            };
            let LexicalReading::Word(v) = &head.value else { unreachable!() };
            let lexeme = &lexicon().lexemes()[&v.lexeme];
            head.frame = Some(
                lexeme
                    .properties
                    .frames
                    .iter()
                    .position(|f| f.items.is_empty())
                    .unwrap_or(0),
            );
            assert!(
                value.admit(lexicon()).is_err(),
                "a non-locative frame cannot license this complement"
            );
        }
    }
    assert!(
        readings("is whenever you draw a card", Category::FinitePredicate)
            .iter()
            .all(|r| !matches!(
                r,
                Reading::SelectedPredicate {
                    category: Category::FinitePredicate,
                    form: 0,
                    complements, ..
                } if matches!(complements.as_slice(), [deckmaste_english_v3::grammar::FrameValue::Argument(child)] if child.category() == Category::LocativeComplement)
            ))
    );
}

#[test]
fn clause_taking_prepositions_keep_their_category_and_complement_selection() {
    // CGEL Ch. 7 §1, p. 600: a clause complement does not turn a P into a
    // subordinator.
    for text in [
        "until you draw cards",
        "before you draw cards",
        "after you draw cards",
        // Psychosis Crawler supplies the clause-taking PP.
        "whenever you draw a card",
        "if you draw cards",
        "when you draw cards",
    ] {
        let values = readings(text, Category::PrepositionPhrase);
        assert!(!values.is_empty(), "missing PP for {text:?}");
        for value in values {
            let Reading::ClauseComplementPreposition { head, .. } = value else {
                panic!("expected a clause-complement PP")
            };
            let LexicalReading::Word(v) = head.value else { unreachable!() };
            assert_eq!(
                lexicon().lexemes()[&v.lexeme].category,
                deckmaste_lexical::Category::Preposition
            );
        }
    }
    assert!(readings("with you draw cards", Category::PrepositionPhrase).is_empty());
    assert!(readings("that you draw cards", Category::PrepositionPhrase).is_empty());
    assert!(readings("whether you draw cards", Category::PrepositionPhrase).is_empty());
    assert!(readings("until draw cards", Category::PrepositionPhrase).is_empty());
    assert!(!readings("Until you draw cards, draw cards.", Category::Document).is_empty());
    assert!(!readings("creatures that attack", Category::Nominal).is_empty());
    assert_eq!(
        lexicon().lexemes()["vocab:Subordinator/If"].category,
        deckmaste_lexical::Category::Subordinator
    );
}

#[test]
fn serial_coordination_composes_with_agreement_case_and_predicate_forms() {
    // CGEL Ch. 15 §1.1, pp. 1275–1278: multiple coordinates form one series.
    for text in [
        "You attack and they attack.",
        "You attack, and they attack.",
        "Draw cards, discard cards, and gain life.",
        "You draw cards, discard cards, and gain life.",
        "You and they attack.",
        "You, they, and creatures attack.",
        "Cards, creatures, artifacts, and lands are legendary.",
        "Draw cards, creatures, and artifacts.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no coordination for {text:?}"
        );
    }
    for text in [
        "You, they, and creatures attacks.",
        "Draw cards, creatures, and they.",
        "You draws cards, discard cards, and gain life.",
        "Draw cards, discard cards, and.",
        "You draw cards, discard cards and gain life.",
        "Draw cards, creatures or artifacts.",
        "You attack, they attack and creatures attack.",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid coordination for {text:?}"
        );
    }
    let values = readings(
        "cards, creatures, artifacts, and lands",
        Category::NounPhrase,
    );
    let noun = |id: &str| {
        let mut head = word(
            id,
            WordForm::Plural,
            FeatureBundle {
                number: Some(Number::Plural),
                ..FeatureBundle::default()
            },
        );
        head.countability = Some(true);
        Reading::Noun { form: 0, head }
    };
    let [cards, creatures, artifacts, lands] = [
        "lexeme:CommonNoun/Card",
        "lexeme:type/creature",
        "lexeme:type/artifact",
        "lexeme:type/land",
    ]
    .map(noun);
    let and = word(
        "vocab:Coordinator/And",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let bare = |head| Reading::BarePlural {
        form: 0,
        head: Box::new(head),
    };
    let phrases = Reading::SerialCoordination {
        category: Category::NounPhrase,
        form: 0,
        left: Box::new(bare(cards.clone())),
        rest: Box::new(Reading::CoordinationSeriesContinuation {
            category: Category::NounPhraseSeries,
            form: 0,
            left: Box::new(bare(creatures.clone())),
            rest: Box::new(Reading::CoordinationSeriesEnd {
                category: Category::NounPhraseSeries,
                form: 0,
                left: Box::new(bare(artifacts.clone())),
                coordinator: and.clone(),
                right: Box::new(bare(lands.clone())),
            }),
        }),
    };
    let nominal = bare(Reading::SerialCoordination {
        category: Category::Nominal,
        form: 0,
        left: Box::new(cards),
        rest: Box::new(Reading::CoordinationSeriesContinuation {
            category: Category::NominalSeries,
            form: 0,
            left: Box::new(creatures),
            rest: Box::new(Reading::CoordinationSeriesEnd {
                category: Category::NominalSeries,
                form: 0,
                left: Box::new(artifacts),
                coordinator: and,
                right: Box::new(lands),
            }),
        }),
    });
    assert_eq!(values, BTreeSet::from([phrases, nominal]));
}

#[test]
fn plain_form_leaves_finiteness_to_the_clause_construction() {
    // CGEL Ch. 3 §§1.8.1–2, pp. 88–90: imperatives are finite, despite plain
    // form.
    let imperatives = readings("Draw cards", Category::Clause);
    assert!(!imperatives.is_empty());
    let declaratives = readings("Cards are drawn", Category::Clause);
    let reference = declaratives
        .into_iter()
        .next()
        .unwrap()
        .admit(lexicon())
        .unwrap();
    for value in imperatives {
        assert_eq!(value.admit(lexicon()).unwrap(), reference);
        value
            .visit_words(&mut |head| {
                if let LexicalReading::Word(v) = &head.value
                    && v.form == WordForm::Plain
                {
                    assert_eq!(v.features.finiteness, None);
                }
            })
            .unwrap();
    }
    assert!(readings("draws cards", Category::BarePredicate).is_empty());
    assert!(readings("drawing cards", Category::BarePredicate).is_empty());
    assert!(!readings("You may draw cards.", Category::Document).is_empty());
    assert!(!readings("Cards are being drawn.", Category::Document).is_empty());
}
