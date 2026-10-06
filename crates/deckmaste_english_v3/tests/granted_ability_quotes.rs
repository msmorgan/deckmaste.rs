mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn granted_quote(text: &str, inner: &str) {
    let quote = format!("\"{inner}\"");
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::QuotedText, &quote),
            (Category::Document, inner),
            (Category::Ability, inner),
        ],
    );
}

#[test]
fn compulsory_rest_selects_an_activated_ability() {
    granted_quote(
        "Enchanted creature has \"{2}, Sacrifice this creature: You gain 2 life.\"",
        "{2}, Sacrifice this creature: You gain 2 life.",
    );
}

#[test]
fn carrier_thrall_selects_an_activated_ability() {
    granted_quote(
        "It has \"Sacrifice this token: Add {C}.\"",
        "Sacrifice this token: Add {C}.",
    );
}

#[test]
fn rain_of_filth_selects_an_ability_under_a_duration_adjunct() {
    granted_quote(
        "Until end of turn, lands you control gain \"Sacrifice this land: Add {B}.\"",
        "Sacrifice this land: Add {B}.",
    );
}

#[test]
fn heroes_of_the_revel_postmodifies_the_token_with_an_ability() {
    granted_quote(
        "When this creature enters, create a 1/1 red Satyr creature token with \"This token can't block.\"",
        "This token can't block.",
    );
    assert_constituents(
        "When this creature enters, create a 1/1 red Satyr creature token with \"This token can't block.\"",
        Category::Document,
        &[
            (
                Category::Nominal,
                "1/1 red Satyr creature token with \"This token can't block.\"",
            ),
            (
                Category::PrepositionPhrase,
                "with \"This token can't block.\"",
            ),
            (Category::Ability, "This token can't block."),
        ],
    );
}

#[test]
fn web_shooters_coordinates_a_keyword_and_a_triggered_ability() {
    granted_quote(
        "Equipped creature gets +1/+1 and has reach and \"Whenever this creature attacks, tap target creature an opponent controls.\"",
        "Whenever this creature attacks, tap target creature an opponent controls.",
    );
}

#[test]
fn energy_flux_selects_a_triggered_ability_with_plural_agreement() {
    granted_quote(
        "All artifacts have \"At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.\"",
        "At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.",
    );
}

fn word(owner: &str, form: WordForm, casing: SurfaceCase, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(
            lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == casing
                        && (form != WordForm::Present
                            || (value.features.number == Some(deckmaste_lexical::Number::Singular)
                                && value.features.person == Some(deckmaste_lexical::Person::Third)))
                })
                .unwrap()
                .clone(),
        ),
        frame,
        countability: None,
    }
}

#[test]
fn independent_grant_preserves_the_nested_ability_frame_and_traversal() {
    // Verdant Haven's granted ability; also Llanowar Mentor's token ability.
    let quotation = Reading::QuotedText {
        form: 0,
        text: Box::new(Reading::Document {
            form: 0,
            first: Box::new(Reading::ActivatedAbility {
                form: 0,
                cost: Box::new(Reading::Cost {
                    form: 0,
                    first: Box::new(Reading::SymbolCost {
                        form: 0,
                        symbols: Box::new(Reading::CostSymbols {
                            form: 0,
                            first: Box::new(Reading::NamedCostSymbol {
                                form: 0,
                                symbol: word(
                                    "vocab:FixedCostSymbol/Tap",
                                    WordForm::Invariant,
                                    SurfaceCase::Declared,
                                    None,
                                ),
                            }),
                            rest: vec![],
                        }),
                    }),
                    rest: vec![],
                }),
                body: Box::new(Reading::Paragraph {
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
                                        form: 0,
                                        category: Category::SecondaryVerbPhrase,
                                        head: word(
                                            "core-verb:Add",
                                            WordForm::Plain,
                                            SurfaceCase::Initial,
                                            Some(1),
                                        ),
                                        complements: vec![FrameValue::Argument(Box::new(
                                            Reading::ManaPhrase {
                                                form: 0,
                                                first: Box::new(Reading::NamedManaSymbol {
                                                    form: 0,
                                                    symbol: word(
                                                        "vocab:FixedCostSymbol/Green",
                                                        WordForm::Invariant,
                                                        SurfaceCase::Declared,
                                                        None,
                                                    ),
                                                }),
                                                rest: vec![],
                                            },
                                        ))],
                                    }),
                                }),
                            }),
                        }),
                    }),
                    rest: vec![],
                }),
            }),
            rest: vec![],
        }),
    };
    let grant = Reading::SelectedPredicate {
        form: 0,
        category: Category::FinitePredicate,
        head: word(
            "core-verb:Have",
            WordForm::Present,
            SurfaceCase::Declared,
            Some(0),
        ),
        complements: vec![FrameValue::Argument(Box::new(
            Reading::QuotedGrantedAbility {
                form: 0,
                quotation: Box::new(quotation.clone()),
            },
        ))],
    };
    for (text, value) in [
        ("has \"{T}: Add {G}.\"", grant),
        (
            "with \"{T}: Add {G}.\"",
            Reading::QuotedComplementPreposition {
                form: 0,
                head: word(
                    "vocab:Preposition/With",
                    WordForm::Invariant,
                    SurfaceCase::Declared,
                    None,
                ),
                complement: Box::new(Reading::QuotedGrantedAbility {
                    form: 0,
                    quotation: Box::new(quotation),
                }),
            },
        ),
    ] {
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
        let parsed = readings(text, value.category());
        let actual = parsed
            .get(&value)
            .expect("independently authored Reading is retained");
        let mut expected_nodes = vec![];
        let mut actual_nodes = vec![];
        value
            .visit(&mut |node| expected_nodes.push(node.clone()))
            .unwrap();
        actual
            .visit(&mut |node| actual_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(expected_nodes, actual_nodes);
        let mut expected_words = vec![];
        let mut actual_words = vec![];
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        actual
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(expected_words, actual_words);
    }
}

#[test]
fn grant_rejects_a_second_terminal_unpaired_quotes_and_unread_inner_text() {
    for text in [
        "It has \"Sacrifice this token: Add {C}.\".",
        "It has \"Sacrifice this token: Add {C}.",
        "It has “Sacrifice this token: Add {C}.\"",
        "It has \"Sacrifice this token: Add {C}.\" until end of turn",
        "It has \"Sacrifice this token: {C} Add.\"",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
}

#[test]
fn ogre_marauder_preserves_its_fragment_quotation_and_following_duration() {
    let text = "Whenever this creature attacks, it gains \"this creature can't be blocked\" until end of turn unless defending player sacrifices a creature of their choice.";
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::QuotedText, "\"this creature can't be blocked\""),
            (Category::Clause, "this creature can't be blocked"),
        ],
    );
}
