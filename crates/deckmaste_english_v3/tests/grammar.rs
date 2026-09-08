use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn lexicon() -> &'static Lexicon {
    static LEXICON: OnceLock<Lexicon> = OnceLock::new();
    LEXICON.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source = deckmaste_lexical_source::load_workspace(&root).unwrap();
        Lexicon::new(source.lexemes).unwrap()
    })
}

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let lexicon = lexicon();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &category).unwrap();
    let mut values = BTreeSet::new();
    for value in grammar.readings(&forest) {
        let value = value.unwrap();
        assert_eq!(value.realize(lexicon).unwrap(), text);
        assert!(values.insert(value), "duplicate Reading for {text:?}");
    }
    values
}

#[test]
fn authored_vocabulary_composes_across_the_grammar() {
    for text in [
        "Draw cards.",
        "You draw cards.",
        "Each creature attacks.",
        "These creatures attack.",
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
        "These creature attacks.",
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
        subject: Box::new(Reading::NominativePhrase {
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
        predicate: Box::new(Reading::FiniteBareAuxiliary {
            form: 0,
            head: auxiliary,
            complement: Box::new(Reading::BareEllipsis {
                form: 0,
                omission: Box::new(Reading::OmittedPlainActive { form: 0 }),
            }),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "you do");
    assert!(readings("you do", Category::FiniteClause).contains(&value));
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
        "Creatures attack by 2 plus 2.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
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
    assert_eq!(grouped.realize(lexicon()).unwrap(), "2");
    assert_eq!(ungrouped.realize(lexicon()).unwrap(), "2");
    assert!(alternatives.contains(&grouped));
    assert!(alternatives.contains(&ungrouped));
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
    let value = Reading::EqualityComplement {
        form: 0,
        head: head.clone(),
        marker: word(
            "vocab:Preposition/To",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
        measure: Box::new(Reading::GroupedScalarNumeral {
            form: 0,
            head: numeral(2, Numeral::Arabic(true)),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "equal to 2");
    assert!(readings("equal to 2", Category::EqualityComplement).contains(&value));
    assert!(
        Reading::Adjective { form: 0, head }
            .admit(lexicon())
            .is_err()
    );
    assert!(readings("equal", Category::AdjectivePhrase).is_empty());
    assert!(readings("equal than 2", Category::EqualityComplement).is_empty());
    assert!(!readings("greater than 2", Category::OrderingComplement).is_empty());
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
        "Creatures have \"Draw cards.\".",
        "Creatures have “{T}: Draw cards.”.",
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
            finiteness: Some(Finiteness::Nonfinite),
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
                        head: Box::new(Reading::NonfiniteIntransitive { form: 0, head }),
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
