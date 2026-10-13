use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn word(owner: &str, form: WordForm, number: Option<Number>, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number,
                ..Default::default()
            },
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: number.map(|_| true),
    }
}
fn keyword(name: &str, capitalization: SurfaceCase) -> Word {
    word(
        &format!("lexeme:keyword_ability/{name}"),
        WordForm::Invariant,
        None,
        capitalization,
    )
}
fn bare(name: &str) -> Reading {
    Reading::BareKeyword {
        form: 0,
        head: keyword(name, SurfaceCase::Declared),
    }
}
fn noun(name: &str, number: Number) -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(
                    &format!("lexeme:CommonNoun/{name}"),
                    if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
                    Some(number),
                    SurfaceCase::Declared,
                ),
            }),
        }),
    }
}
fn modifier(name: &str) -> Reading {
    Reading::NounPremodifier {
        form: 1,
        head: keyword(name, SurfaceCase::Declared),
    }
}
fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert_eq!(common::readings(text, category), BTreeSet::from([expected]));
}
fn contains(reading: &Reading, expected: &Reading) -> bool {
    let mut found = false;
    reading
        .visit(&mut |node| found |= node == expected)
        .unwrap();
    found
}
fn witness(card: &str, text: &str, expected: &Reading) {
    let readings = common::readings_on_card(card, text, Category::Document);
    assert!(!readings.is_empty(), "{card}");
    for reading in readings {
        assert!(
            contains(&reading, expected),
            "{card}: missing selected role"
        );
        reading.admit(common::lexicon()).unwrap();
        assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn bare_keywords_fill_the_granted_ability_complement() {
    for (card, text, name) in [
        (
            "Epochrasite",
            "When this creature dies, exile it with three time counters on it and it gains suspend.",
            "suspend",
        ),
        (
            "Compleat Devotion",
            "Target creature you control gets +2/+2 until end of turn. If that creature has toxic, draw a card.",
            "toxic",
        ),
        (
            "Flashback",
            "Target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.",
            "flashback",
        ),
        (
            "Young Deathclaws",
            "Each creature card in your graveyard has scavenge. The scavenge cost is equal to its mana cost.",
            "scavenge",
        ),
    ] {
        witness(
            card,
            text,
            &Reading::KeywordGrantedAbility {
                form: 0,
                keyword: Box::new(bare(name)),
            },
        );
    }
}

#[test]
fn bare_keyword_is_the_complement_of_with() {
    let expected = Reading::KeywordComplementPreposition {
        category: Category::PrepositionPhrase,
        form: 0,
        head: word(
            "vocab:Preposition/With",
            WordForm::Invariant,
            None,
            SurfaceCase::Declared,
        ),
        complement: Box::new(bare("flashback")),
    };
    witness(
        "Rootcoil Creeper",
        "{G}{U}, {T}, Exile this creature: Return target card with flashback you own from exile to your hand.",
        &expected,
    );
    exact("with flashback", Category::PrepositionPhrase, expected);
}

#[test]
fn keyword_premodifiers_keep_keyword_identity_in_costs_and_abilities() {
    for (card, text, name) in [
        (
            "Flashback",
            "Target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.",
            "flashback",
        ),
        (
            "Murasa Sproutling",
            "When this creature enters, if it was kicked, return target card with a kicker ability from your graveyard to your hand.",
            "kicker",
        ),
        (
            "Young Deathclaws",
            "Each creature card in your graveyard has scavenge. The scavenge cost is equal to its mana cost.",
            "scavenge",
        ),
    ] {
        witness(card, text, &modifier(name));
    }
    let mut cost_head = word(
        "lexeme:CommonNoun/Cost",
        WordForm::Singular,
        Some(Number::Singular),
        SurfaceCase::Declared,
    );
    cost_head.frame = Some(0);
    for (surface, name, head) in [
        (
            "flashback cost",
            "flashback",
            Reading::NominalProjection {
                form: 0,
                head: Box::new(Reading::NominalCoreProjection {
                    form: 0,
                    head: Box::new(Reading::BareFramedNoun {
                        form: 0,
                        head: cost_head,
                    }),
                }),
            },
        ),
        ("morph ability", "morph", noun("Ability", Number::Singular)),
        ("equip abilities", "equip", noun("Ability", Number::Plural)),
    ] {
        exact(
            surface,
            Category::Nominal,
            Reading::NominalProjection {
                form: 0,
                head: Box::new(Reading::NominalCoreProjection {
                    form: 0,
                    head: Box::new(Reading::BarrierNounPremodifiedNominal {
                        form: 0,
                        modifier: Box::new(modifier(name)),
                        head: Box::new(head),
                    }),
                }),
            },
        );
    }
}

#[test]
fn independently_constructed_bare_mentions_roundtrip() {
    for name in ["flashback", "toxic", "suspend"] {
        exact(name, Category::KeywordPhrase, bare(name));
        exact(
            name,
            Category::GrantedAbility,
            Reading::KeywordGrantedAbility {
                form: 0,
                keyword: Box::new(bare(name)),
            },
        );
    }
    // KM4 explicitly permits a standalone bare parameter-taking Keyword Line.
    exact(
        "Suspend",
        Category::KeywordPhrase,
        Reading::BareKeyword {
            form: 0,
            head: keyword("suspend", SurfaceCase::Initial),
        },
    );
}

fn numeral(value: i32) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation: deckmaste_lexical::Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}
fn amount(value: i32) -> Reading {
    Reading::UngroupedScalarNumeral {
        form: 0,
        modifier: None,
        head: numeral(value),
    }
}
fn numeric(value: i32) -> Reading {
    Reading::NumericCostSymbol {
        form: 0,
        number: numeral(value),
    }
}
fn symbol(name: &str) -> Reading {
    Reading::NamedCostSymbol {
        form: 0,
        symbol: word(
            &format!("vocab:FixedCostSymbol/{name}"),
            WordForm::Invariant,
            None,
            SurfaceCase::Declared,
        ),
    }
}
fn cost(first: Reading, rest: Vec<Reading>) -> Reading {
    Reading::SymbolSequence {
        category: Category::CostSymbols,
        form: 0,
        first: Box::new(first),
        rest,
    }
}

#[test]
fn parameterized_keywords_keep_their_complete_payload_reading() {
    for (surface, name, payload) in [
        (
            "Flashback {2}{R}",
            "flashback",
            cost(numeric(2), vec![symbol("Red")]),
        ),
        ("Cycling {2}", "cycling", cost(numeric(2), vec![])),
    ] {
        exact(
            surface,
            Category::KeywordPhrase,
            Reading::CostKeyword {
                form: 0,
                head: keyword(name, SurfaceCase::Initial),
                cost: Box::new(payload),
            },
        );
    }
    exact(
        "Suspend 4—{1}{U}",
        Category::KeywordPhrase,
        Reading::AmountCostKeyword {
            form: 0,
            head: keyword("suspend", SurfaceCase::Initial),
            amount: Box::new(amount(4)),
            cost: Box::new(cost(numeric(1), vec![symbol("Blue")])),
        },
    );
    exact(
        "Toxic 1",
        Category::KeywordPhrase,
        Reading::AmountKeyword {
            form: 0,
            head: keyword("toxic", SurfaceCase::Initial),
            amount: Box::new(amount(1)),
        },
    );
    let text = "Target creature gains flashback {2} until end of turn.";
    let readings = common::readings(text, Category::Document);
    let expected = Reading::KeywordGrantedAbility {
        form: 0,
        keyword: Box::new(Reading::CostKeyword {
            form: 0,
            head: keyword("flashback", SurfaceCase::Declared),
            cost: Box::new(cost(numeric(2), vec![])),
        }),
    };
    assert!(!readings.is_empty());
    for reading in readings {
        assert!(contains(&reading, &expected));
        assert!(!contains(&reading, &bare("flashback")));
    }
}

#[test]
fn keyword_premodification_requires_the_head_permission() {
    // A bound suffix cannot stand alone as a keyword-name Premodifier.
    // In particular, typecycling's suffix must not add a third analysis
    // beside the cycling keyword and the Cycle gerund-participle (KM3/KM5).
    let suffix = Reading::NounPremodifier {
        form: 1,
        head: word(
            "lexeme:keyword_ability/typecycling/bound-suffix",
            WordForm::Invariant,
            None,
            SurfaceCase::Declared,
        ),
    };
    assert!(suffix.admit(common::lexicon()).is_err());
    let bad = Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::BarrierNounPremodifiedNominal {
                form: 0,
                modifier: Box::new(modifier("flashback")),
                head: Box::new(Reading::NominalProjection {
                    form: 0,
                    head: Box::new(Reading::NominalCoreProjection {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word(
                                "lexeme:type/creature",
                                WordForm::Singular,
                                Some(Number::Singular),
                                SurfaceCase::Declared,
                            ),
                        }),
                    }),
                }),
            }),
        }),
    };
    assert!(bad.admit(common::lexicon()).is_err());
    assert!(common::readings("flashback creature", Category::Nominal).is_empty());
    for text in ["Boast", "Exhaust", "Enchant", "Prototype"] {
        assert!(
            common::readings(text, Category::KeywordPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn cycling_keeps_the_gerund_and_keyword_premodifier_readings() {
    let text =
        "You may pay {0} rather than pay the cycling cost of the first card you cycle each turn.";
    let readings = common::readings_on_card("Gavi, Nest Warden", text, Category::Document);
    assert!(
        readings
            .iter()
            .any(|reading| contains(reading, &modifier("cycling")))
    );
    assert!(readings.iter().any(|reading| {
        let mut gerund = false;
        reading
            .visit(&mut |node| {
                if let Reading::VerbalPremodifier { .. } = node {
                    let mut cycle = false;
                    node.visit_words(&mut |word| {
                        if let LexicalReading::Word(value) = &word.value {
                            cycle |= value.lexeme == "core-verb:Cycle"
                                && value.form == WordForm::GerundParticiple;
                        }
                    })
                    .unwrap();
                    gerund |= cycle;
                }
            })
            .unwrap();
        gerund
    }));
}
