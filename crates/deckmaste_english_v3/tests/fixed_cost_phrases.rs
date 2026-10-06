mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

#[test]
fn glimmervoid_keeps_the_color_phrase_inside_the_mana_nominal() {
    assert_constituents(
        "{T}: Add one mana of any color.",
        Category::Document,
        &[
            (Category::NounPhrase, "one mana of any color"),
            (Category::Nominal, "mana of any color"),
            (Category::PrepositionPhrase, "of any color"),
            (Category::NounPhrase, "any color"),
        ],
    );
}

#[test]
fn additional_and_chosen_mana_keep_their_internal_modifiers() {
    // Fertile Ground and Utopia Sprawl, respectively.
    for tail in ["any color", "the chosen color"] {
        let text = format!("an additional one mana of {tail}");
        assert_constituents(
            &text,
            Category::NounPhrase,
            &[
                (Category::Nominal, "one mana"),
                (Category::Cardinal, "one"),
                (Category::PrepositionPhrase, &format!("of {tail}")),
                (Category::NounPhrase, tail),
            ],
        );
    }
    // Sol Grail's complete activated ability.
    assert_constituents(
        "{T}: Add one mana of the chosen color.",
        Category::Document,
        &[
            (Category::Nominal, "mana of the chosen color"),
            (Category::VerbalPremodifier, "chosen"),
        ],
    );
}

fn word(owner: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number,
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn noun_word(owner: &str, number: Number) -> Word {
    let mut head = word(
        owner,
        if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
        Some(number),
    );
    head.countability = Some(true);
    head
}

fn noun(owner: &str, number: Number) -> Reading {
    Reading::Noun {
        form: 0,
        head: noun_word(owner, number),
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::Cardinal {
        form: 0,
        head: Word {
            value: LexicalReading::Numeral {
                value,
                notation: Numeral::Cardinal,
                capitalization: SurfaceCase::Declared,
            },
            frame: None,
            countability: None,
        },
    }
}

fn acc(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}

fn color_phrase() -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word("vocab:Determinative/Any", WordForm::Invariant, None),
        head: Box::new(noun("lexeme:CommonNoun/Color", Number::Singular)),
    }
}

fn mana_nominal(number: Number) -> Reading {
    let mut color = color_phrase();
    if number == Number::Plural {
        let Reading::DeterminedNounPhrase { head, .. } = &mut color else {
            unreachable!()
        };
        **head = Reading::CardinalPremodifiedNominal {
            form: 0,
            quantity: Box::new(cardinal(1)),
            head: Box::new(noun("lexeme:CommonNoun/Color", Number::Singular)),
        };
    }
    Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(Reading::Noun {
            form: 0,
            head: {
                let mut head = noun_word("lexeme:CommonNoun/Mana", Number::Singular);
                head.countability = Some(false);
                head
            },
        }),
        modifier: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: word("vocab:Preposition/Of", WordForm::Invariant, None),
            complement: Box::new(acc(color)),
        }),
    }
}

fn assert_independent(text: &str, expected: &Reading, owners: &[&str]) {
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    let observed = readings(text, expected.category());
    assert!(
        observed.contains(expected),
        "independent value missing for {text}: {observed:?}"
    );
    let parsed = observed.get(expected).unwrap();
    assert_eq!(parsed, expected);
    let mut words = Vec::new();
    parsed
        .visit_words(&mut |word| {
            words.push(match &word.value {
                LexicalReading::Word(value) => value.lexeme.to_string(),
                LexicalReading::Numeral { .. } => "numeral".into(),
                LexicalReading::FlavorWord { .. } => panic!("unexpected opaque lexical leaf"),
            });
        })
        .unwrap();
    assert_eq!(words, owners);
    let mut expected_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    let mut parsed_nodes = Vec::new();
    parsed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(parsed_nodes, expected_nodes);
}

#[test]
fn independent_mana_values_preserve_number_attachment_and_adds_existing_frame() {
    // Glimmervoid and Implements of Sacrifice, respectively.
    for (count, number, text) in [
        (1, Number::Singular, "add one mana of any color"),
        (2, Number::Plural, "two mana of any one color"),
    ] {
        let phrase = Reading::CardinalMeasuredNounPhrase {
            form: 0,
            quantity: Box::new(cardinal(count)),
            head: Box::new(mana_nominal(number)),
        };
        let (expected, owners) = if count == 1 {
            let mut head = word("core-verb:Add", WordForm::Plain, None);
            head.frame = Some(0);
            (
                Reading::SelectedPredicate {
                    category: Category::SecondaryVerbPhrase,
                    form: 0,
                    head,
                    complements: vec![FrameValue::Argument(Box::new(acc(phrase)))],
                },
                vec![
                    "core-verb:Add",
                    "numeral",
                    "lexeme:CommonNoun/Mana",
                    "vocab:Preposition/Of",
                    "vocab:Determinative/Any",
                    "lexeme:CommonNoun/Color",
                ],
            )
        } else {
            (
                phrase,
                vec![
                    "numeral",
                    "lexeme:CommonNoun/Mana",
                    "vocab:Preposition/Of",
                    "vocab:Determinative/Any",
                    "numeral",
                    "lexeme:CommonNoun/Color",
                ],
            )
        };
        assert_independent(text, &expected, &owners);
    }
}

#[test]
fn independent_cost_opener_preserves_predication_and_infinitival_selection() {
    let spell = Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(
            "vocab:SingularDemonstrative/This",
            WordForm::Invariant,
            None,
        ),
        head: Box::new(noun("lexeme:CommonNoun/Spell", Number::Singular)),
    };
    let mut verb = word("lexeme:keyword_action/cast", WordForm::Plain, None);
    verb.frame = Some(0);
    let infinitive = Reading::ToInfinitive {
        form: 0,
        marker: word("vocab:InfinitivalMarker/To", WordForm::Invariant, None),
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: verb,
                complements: vec![FrameValue::Argument(Box::new(acc(spell)))],
            }),
        }),
    };
    let mut cost = noun_word("lexeme:CommonNoun/Cost", Number::Singular);
    cost.frame = Some(1);
    let nominal = Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::Adjective {
            form: 0,
            head: word(
                "vocab:AttributiveAdjective/Additional",
                WordForm::Invariant,
                None,
            ),
        }),
        head: Box::new(Reading::InfinitiveComplementNominal {
            form: 0,
            head: cost,
            complement: Box::new(infinitive),
        }),
    };
    let mut article = word(
        "vocab:Article/Indefinite",
        WordForm::Invariant,
        Some(Number::Singular),
    );
    let LexicalReading::Word(value) = &mut article.value else { unreachable!() };
    value.variant = 1;
    let expected = Reading::PredicativeComplementPreposition {
        form: 0,
        head: word("vocab:Preposition/As", WordForm::Invariant, None),
        complement: Box::new(Reading::NominalComplement {
            form: 0,
            phrase: Box::new(acc(Reading::IndefiniteNounPhrase {
                form: 0,
                determiner: article,
                head: Box::new(nominal),
            })),
        }),
    };
    assert_independent(
        "as an additional cost to cast this spell",
        &expected,
        &[
            "vocab:Preposition/As",
            "vocab:Article/Indefinite",
            "vocab:AttributiveAdjective/Additional",
            "lexeme:CommonNoun/Cost",
            "vocab:InfinitivalMarker/To",
            "lexeme:keyword_action/cast",
            "vocab:SingularDemonstrative/This",
            "lexeme:CommonNoun/Spell",
        ],
    );
    let Reading::PredicativeComplementPreposition {
        form, complement, ..
    } = expected
    else {
        unreachable!()
    };
    let wrong = Reading::PredicativeComplementPreposition {
        form,
        complement,
        head: word("vocab:Preposition/Of", WordForm::Invariant, None),
    };
    assert!(wrong.admit(lexicon()).is_err());
    assert!(wrong.realize(lexicon()).is_err());
}

#[test]
fn mutated_witnesses_reject_wrong_number_finiteness_and_of_adjuncts() {
    for text in ["any two color", "any one colors", "one manas of any color"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for text in [
        "Of an additional cost to cast this spell, sacrifice a creature.",
        "As an additional cost to casts this spell, sacrifice a creature.",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
    let wrong = Reading::BarePlural {
        form: 0,
        head: Box::new(Reading::CardinalMeasuredNominal {
            form: 0,
            quantity: Box::new(cardinal(2)),
            head: Box::new(Reading::Noun {
                form: 0,
                head: {
                    let mut head = noun_word("lexeme:CommonNoun/Mana", Number::Singular);
                    head.countability = Some(false);
                    head
                },
            }),
        }),
    };
    assert!(wrong.admit(lexicon()).is_err());
    assert!(wrong.realize(lexicon()).is_err());
    assert_eq!(readings("two mana", Category::NounPhrase).len(), 1);
}

#[test]
fn predicative_preposition_adjuncts_require_a_preposed_comma() {
    for text in [
        "Sacrifice a creature as an additional cost to cast this spell.",
        "Sacrifice as an additional cost to cast this spell a creature.",
        "As an additional cost to cast this spell sacrifice a creature.",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
    for text in [
        "At the beginning of your upkeep, sacrifice a creature.",
        "During your turn, sacrifice a creature.",
        "As an additional cost to cast this spell, sacrifice a creature.",
        "You may exert this creature as it attacks.",
    ] {
        assert!(!readings(text, Category::Document).is_empty(), "{text}");
    }
}

#[test]
fn additional_costs_allow_discard_and_alternative_payment_actions() {
    // Magmatic Insight and Lightning Axe.
    for (text, action) in [
        (
            "As an additional cost to cast this spell, discard a land card.",
            "discard a land card",
        ),
        (
            "As an additional cost to cast this spell, discard a card or pay {5}.",
            "pay {5}",
        ),
    ] {
        assert_constituents(
            text,
            Category::Document,
            &[
                (
                    Category::PredicativeComplement,
                    "an additional cost to cast this spell",
                ),
                (Category::InfinitiveComplement, "to cast this spell"),
                (Category::SecondaryVerbPhrase, action),
            ],
        );
    }
}

#[test]
fn altars_reap_keeps_the_cast_infinitive_inside_the_predicative_cost_phrase() {
    assert_constituents(
        "As an additional cost to cast this spell, sacrifice a creature.",
        Category::Document,
        &[
            (
                Category::PrepositionPhrase,
                "As an additional cost to cast this spell",
            ),
            (
                Category::PredicativeComplement,
                "an additional cost to cast this spell",
            ),
            (
                Category::NounPhrase,
                "an additional cost to cast this spell",
            ),
            (Category::Nominal, "cost to cast this spell"),
            (Category::InfinitiveComplement, "to cast this spell"),
            (Category::SecondaryVerbPhrase, "cast this spell"),
        ],
    );
}

#[test]
fn terrarion_keeps_the_combination_phrase_inside_the_mana_nominal() {
    assert_constituents(
        "{2}, {T}, Sacrifice this artifact: Add two mana in any combination of colors.",
        Category::Document,
        &[
            (
                Category::NounPhrase,
                "two mana in any combination of colors",
            ),
            (Category::Nominal, "mana in any combination of colors"),
            (Category::PrepositionPhrase, "in any combination of colors"),
            (Category::NounPhrase, "any combination of colors"),
            (Category::PrepositionPhrase, "of colors"),
        ],
    );
}

#[test]
fn implements_of_sacrifice_keeps_any_one_color_as_a_noun_phrase() {
    assert_constituents(
        "{1}, {T}, Sacrifice this artifact: Add two mana of any one color.",
        Category::Document,
        &[
            (Category::NounPhrase, "two mana of any one color"),
            (Category::Nominal, "mana of any one color"),
            (Category::PrepositionPhrase, "of any one color"),
            (Category::NounPhrase, "any one color"),
            (Category::Cardinal, "one"),
        ],
    );
}
