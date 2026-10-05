use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str) -> Reading {
    Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Singular,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(true),
        },
    }
}

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|value| {
            let value = value.unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), text);
            value
        })
        .collect()
}

fn exact(text: &str, expected: Reading) {
    expected
        .admit(&LEXICON)
        .unwrap_or_else(|error| panic!("{text}: {error:?}"));
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let actual = readings(text, Category::KeywordPhrase);
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

fn keyword(name: &str) -> Word {
    invariant(
        &format!("lexeme:keyword_ability/{name}"),
        SurfaceCase::Initial,
    )
}
fn number(value: i32) -> Word {
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
fn scalar(value: i32) -> Reading {
    Reading::SmallUnsignedScalar {
        form: 0,
        head: number(value),
    }
}
fn cost(first: Reading, rest: Vec<Reading>) -> Reading {
    Reading::CostSymbols {
        form: 0,
        first: Box::new(first),
        rest,
    }
}
fn named(name: &str) -> Reading {
    Reading::NamedCostSymbol {
        form: 0,
        symbol: invariant(
            &format!("vocab:FixedCostSymbol/{name}"),
            SurfaceCase::Declared,
        ),
    }
}
fn marked(marker: &str, quality: Reading) -> Reading {
    Reading::KeywordQualityPreposition {
        form: 0,
        head: invariant(
            &format!("vocab:Preposition/{marker}"),
            SurfaceCase::Declared,
        ),
        quality: Box::new(quality),
    }
}
#[test]
fn authentic_quality_and_subject_keywords_keep_declared_payloads() {
    // Vulshok Refugee: "Protection from red".
    exact(
        "Protection from red",
        Reading::QualityKeyword {
            form: 0,
            head: keyword("protection"),
            quality: Box::new(marked(
                "From",
                Reading::AdjectivalKeywordQuality {
                    form: 0,
                    phrase: Box::new(Reading::Adjective {
                        form: 0,
                        head: invariant("vocab:ColorWord/Red", SurfaceCase::Declared),
                    }),
                },
            )),
        },
    );
    // Myr Enforcer: "Affinity for artifacts" (followed by reminder text).
    let mut artifacts = noun("lexeme:type/artifact");
    if let Reading::Noun { head, .. } = &mut artifacts {
        if let LexicalReading::Word(value) = &mut head.value {
            value.form = WordForm::Plural;
            value.features.number = Some(Number::Plural);
        }
    }
    let nominal = Reading::QualityKeyword {
        form: 0,
        head: keyword("affinity"),
        quality: Box::new(marked(
            "For",
            Reading::NominalKeywordQuality {
                form: 0,
                phrase: Box::new(artifacts.clone()),
            },
        )),
    };
    let noun_phrase = Reading::QualityKeyword {
        form: 0,
        head: keyword("affinity"),
        quality: Box::new(marked(
            "For",
            Reading::NounPhraseKeywordQuality {
                form: 0,
                phrase: Box::new(Reading::BarePlural {
                    form: 0,
                    head: Box::new(artifacts),
                }),
            },
        )),
    };
    let expected = BTreeSet::from([nominal, noun_phrase]);
    assert_eq!(
        readings("Affinity for artifacts", Category::KeywordPhrase),
        expected
    );
    for value in expected {
        value.admit(&LEXICON).unwrap();
        assert_eq!(value.realize(&LEXICON).unwrap(), "Affinity for artifacts");
    }
    // Evil Presence: "Enchant land".
    exact(
        "Enchant land",
        Reading::SubjectKeyword {
            form: 0,
            head: keyword("enchant"),
            subject: Box::new(noun("lexeme:type/land")),
        },
    );
}
#[test]
fn authentic_amount_cost_quality_cost_and_size_payloads_keep_separators() {
    // Rift Bolt: "Suspend 1—{R}" (followed by reminder text).
    exact(
        "Suspend 1—{R}",
        Reading::AmountCostKeyword {
            form: 0,
            head: keyword("suspend"),
            amount: Box::new(Reading::UngroupedScalarNumeral {
                form: 0,
                head: number(1),
            }),
            separator: Box::new(Reading::DashKeywordSeparator { form: 0 }),
            cost: Box::new(cost(named("Red"), vec![])),
        },
    );
    // Rust Goliath: "Prototype {3}{G}{G} — 3/5" (followed by reminder text).
    exact(
        "Prototype {3}{G}{G} — 3/5",
        Reading::CostPowerToughnessKeyword {
            form: 0,
            head: keyword("prototype"),
            cost: Box::new(cost(
                Reading::NumericCostSymbol {
                    form: 0,
                    number: number(3),
                },
                vec![named("Green"), named("Green")],
            )),
            separator: Box::new(Reading::SpacedDashKeywordSeparator { form: 0 }),
            size: Box::new(Reading::SlashPair {
                form: 0,
                left: Box::new(Reading::UnsignedScalar {
                    form: 0,
                    value: Box::new(scalar(3)),
                }),
                right: Box::new(Reading::UnsignedScalar {
                    form: 0,
                    value: Box::new(scalar(5)),
                }),
            }),
        },
    );
    // Kodama's Might: "Splice onto Arcane {G}" (followed by reminder text).
    let mut arcane = noun("lexeme:spell_subtype/arcane");
    if let Reading::Noun { head, .. } = &mut arcane {
        if let LexicalReading::Word(value) = &mut head.value {
            value.capitalization = SurfaceCase::Declared;
        }
    }
    exact(
        "Splice onto Arcane {G}",
        Reading::QualityCostKeyword {
            form: 0,
            head: keyword("splice"),
            quality: Box::new(marked(
                "Onto",
                Reading::NominalKeywordQuality {
                    form: 0,
                    phrase: Box::new(arcane),
                },
            )),
            separator: Box::new(Reading::SpaceKeywordSeparator { form: 0 }),
            cost: Box::new(cost(named("Green"), vec![])),
        },
    );
}
#[test]
fn keyword_payload_contracts_reject_wrong_markers_numbers_and_separators() {
    for text in [
        "Protection for red",
        "Affinity for artifact",
        "Enchant",
        "Suspend",
        "Suspend 1 {R}",
        "Suspend 1 — {R}",
        "Prototype",
        "Splice onto Arcane",
    ] {
        assert!(readings(text, Category::KeywordPhrase).is_empty(), "{text}");
    }
}

#[test]
fn authentic_boast_payload_preserves_activated_ability_structure() {
    // Goldmaw Champion: "Boast — {1}{W}: Tap target creature." (followed by reminder text).
    let mut action = invariant("lexeme:keyword_action/tap", SurfaceCase::Initial);
    action.frame = Some(0);
    if let LexicalReading::Word(value) = &mut action.value {
        value.form = WordForm::Plain;
    }
    let body = Reading::Paragraph {
        form: 0,
        first: Box::new(Reading::SentenceItem {
            form: 0,
            sentence: Box::new(Reading::Sentence {
                form: 0,
                clause: Box::new(Reading::Imperative {
                    form: 0,
                    predicate: Box::new(Reading::BarePredicate {
                        form: 0,
                        head: Box::new(Reading::TransitivePredicate {
                            category: Category::SecondaryVerbPhrase,
                            form: 0,
                            head: action,
                            object: Box::new(Reading::CasePhrase {
                                category: Category::AccusativePhrase,
                                form: 0,
                                head: Box::new(Reading::TargetNounPhrase {
                                    form: 0,
                                    marker: invariant(
                                        "vocab:TargetingMarker/Target",
                                        SurfaceCase::Declared,
                                    ),
                                    head: Box::new(noun("lexeme:type/creature")),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        }),
        rest: vec![],
    };
    exact(
        "Boast — {1}{W}: Tap target creature.",
        Reading::ClausalKeyword {
            form: 0,
            head: keyword("boast"),
            separator: Box::new(Reading::SpacedDashKeywordSeparator { form: 0 }),
            body: Box::new(Reading::AbilityKeywordPayload {
                form: 0,
                body: Box::new(Reading::ActivatedAbility {
                    form: 0,
                    cost: Box::new(Reading::Cost {
                        form: 0,
                        first: Box::new(Reading::SymbolCost {
                            form: 0,
                            symbols: Box::new(cost(
                                Reading::NumericCostSymbol {
                                    form: 0,
                                    number: number(1),
                                },
                                vec![named("White")],
                            )),
                        }),
                        rest: vec![],
                    }),
                    body: Box::new(body),
                }),
            }),
        },
    );
}
