use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::lexicon;
use crate::common::readings;

fn keyword(name: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: format!("lexeme:keyword_ability/{name}").as_str().into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Initial,
        }),
        frame: None,
        countability: None,
    }
}

fn number(value: i32) -> Reading {
    Reading::NumericCostSymbol {
        form: 0,
        number: Word {
            value: LexicalReading::Numeral {
                value,
                notation: Numeral::Arabic(false),
                capitalization: SurfaceCase::Declared,
            },
            frame: None,
            countability: None,
        },
    }
}

fn symbols(first: Reading, rest: Vec<Reading>) -> Reading {
    Reading::SymbolSequence {
        category: Category::CostSymbols,
        form: 0,
        first: Box::new(first),
        rest,
    }
}

fn exact(text: &str, expected: Reading) {
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    assert_eq!(
        readings(text, Category::KeywordPhrase),
        BTreeSet::from([expected])
    );
}

#[test]
fn independently_constructed_cost_keywords_keep_both_roundtrip_laws() {
    // Isolate the ticket's mana-only dash witness with an independently built
    // Cost.
    exact(
        "Equip—{3}",
        Reading::CostKeyword {
            form: 1,
            head: keyword("equip"),
            cost: Box::new(Reading::Cost {
                form: 0,
                first: Box::new(Reading::SymbolCost {
                    form: 0,
                    symbols: Box::new(symbols(number(3), vec![])),
                }),
                rest: vec![],
            }),
        },
    );
    // Preserve the ticket's space-form Ward control.
    exact(
        "Ward {2}",
        Reading::CostKeyword {
            form: 0,
            head: keyword("ward"),
            cost: Box::new(symbols(number(2), vec![])),
        },
    );
    // Preserve the ticket's space-form Flashback control.
    exact(
        "Flashback {1}{R}",
        Reading::CostKeyword {
            form: 0,
            head: keyword("flashback"),
            cost: Box::new(symbols(
                number(1),
                vec![Reading::NamedCostSymbol {
                    form: 0,
                    symbol: Word {
                        value: LexicalReading::Word(LexicalValue {
                            lexeme: "vocab:FixedCostSymbol/Red".into(),
                            form: WordForm::Invariant,
                            features: FeatureBundle::default(),
                            variant: 0,
                            capitalization: SurfaceCase::Declared,
                        }),
                        frame: None,
                        countability: None,
                    },
                }],
            )),
        },
    );
}

#[test]
fn dash_parameters_retain_every_existing_cost_reading() {
    // Owlin Shieldmage, Flash of Defiance, Sweet Oblivion, Sunscourge Champion,
    // and the ticket's isolated mana-only Equip witness.
    for (text, label, cost) in [
        ("Ward—Pay 3 life.", "ward", "Pay 3 life"),
        (
            "Flashback—{1}{R}, Pay 3 life.",
            "flashback",
            "{1}{R}, Pay 3 life",
        ),
        ("Equip—{3}", "equip", "{3}"),
        (
            "Escape—{3}{U}, Exile four other cards from your graveyard.",
            "escape",
            "{3}{U}, Exile four other cards from your graveyard",
        ),
        (
            "Eternalize—{2}{W}{W}, Discard a card.",
            "eternalize",
            "{2}{W}{W}, Discard a card",
        ),
    ] {
        let expected: BTreeSet<_> = readings(cost, Category::Cost)
            .into_iter()
            .map(|cost| Reading::CostKeyword {
                form: 1,
                head: keyword(label),
                cost: Box::new(cost),
            })
            .collect();
        assert!(!expected.is_empty(), "{text}");
        assert_eq!(readings(text, Category::KeywordPhrase), expected, "{text}");
        assert!(!readings(text, Category::Document).is_empty(), "{text}");
    }
}

#[test]
fn activated_costs_and_ability_words_keep_their_structure() {
    // The ticket's activated-ability and Landfall controls retain every
    // constituent Reading.
    let text = "{2}, Pay 2 life: Draw a card.";
    let actual: BTreeSet<_> = readings(text, Category::Ability)
        .into_iter()
        .map(|reading| {
            let Reading::ActivatedAbility { cost, .. } = reading else {
                panic!("wrong activated-ability structure");
            };
            *cost
        })
        .collect();
    assert_eq!(actual, readings("{2}, Pay 2 life", Category::Cost));
    let text = "Landfall — Whenever a land you control enters, you gain 1 life.";
    let expected: BTreeSet<_> = readings(
        "Whenever a land you control enters, you gain 1 life.",
        Category::Ability,
    )
    .into_iter()
    .flat_map(|body| {
        [
            Reading::AbilityWordHead {
                form: 0,
                head: Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "catalog:ability-words.txt/Landfall".into(),
                        form: WordForm::Invariant,
                        features: FeatureBundle::default(),
                        variant: 0,
                        capitalization: SurfaceCase::Declared,
                    }),
                    frame: None,
                    countability: None,
                },
                body: Box::new(body.clone()),
            },
            Reading::FlavorWordHead {
                form: 0,
                head: Word {
                    value: LexicalReading::FlavorWord {
                        label: "Landfall".into(),
                    },
                    frame: None,
                    countability: None,
                },
                body: Box::new(body),
            },
        ]
    })
    .collect();
    assert_eq!(readings(text, Category::Ability), expected);
}

#[test]
fn dash_costs_require_a_cost_parameter_and_selected_terminal() {
    for text in [
        "Haste—{3}",
        "Ward—Pay 3 life",
        "Ward Pay 3 life.",
        "Ward—Pay 3 life..",
    ] {
        assert!(readings(text, Category::KeywordPhrase).is_empty(), "{text}");
    }
    let invalid = Reading::CostKeyword {
        form: 1,
        head: keyword("haste"),
        cost: Box::new(Reading::Cost {
            form: 0,
            first: Box::new(Reading::SymbolCost {
                form: 0,
                symbols: Box::new(symbols(number(3), vec![])),
            }),
            rest: vec![],
        }),
    };
    assert!(invalid.admit(lexicon()).is_err());
    assert!(invalid.realize(lexicon()).is_err());
}

// Synthetic construction-compiler fixture, not an Oracle witness.
constructions! {
    mod final_component {
        feature Kind { Empty, Symbols, Action } default Empty;
        category Item(Kind);
        category Tail(Kind);
        category List();
        separator terminal(Kind, Kind) {
            (Symbols, Empty) => "", (Action, Empty) => ".",
            (_, Symbols) => "", (_, Action) => ".",
        }
        construction Symbols: Item { form ["s"]; export Kind = Symbols; }
        construction Action: Item { form ["a"]; export Kind = Action; }
        construction Tail: Tail {
            form [" ", item: Item];
            export Kind = item.Kind;
        }
        construction List: List {
            form [first: Item, rest: repeat(Tail, ""), terminal(first.Kind, last(rest).Kind)];
        }
    }
}

#[test]
fn final_component_projection_preserves_empty_and_nonempty_repeat_roundtrips() {
    use final_component::Reading as R;
    let symbol = R::Symbols { form: 0 };
    let action = R::Action { form: 0 };
    let grammar = final_component::Grammar::default();
    for (text, first, rest) in [
        ("s", symbol.clone(), vec![]),
        ("a.", action.clone(), vec![]),
        ("s a.", symbol.clone(), vec![action.clone()]),
        ("a s", action.clone(), vec![symbol.clone()]),
        (
            "s a s",
            symbol.clone(),
            vec![action.clone(), symbol.clone()],
        ),
        ("a s a.", action.clone(), vec![symbol, action]),
    ] {
        let expected = R::List {
            form: 0,
            first: Box::new(first),
            rest: rest
                .into_iter()
                .map(|item| R::Tail {
                    form: 0,
                    item: Box::new(item),
                })
                .collect(),
        };
        expected.admit_with(&grammar, lexicon()).unwrap();
        assert_eq!(expected.realize_with(&grammar, lexicon()).unwrap(), text);
        let forest = parse(
            &grammar,
            lexicon(),
            &lexicon().analyze(text),
            &final_component::Category::List,
        )
        .unwrap();
        let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(actual, BTreeSet::from([expected]));
    }
}
