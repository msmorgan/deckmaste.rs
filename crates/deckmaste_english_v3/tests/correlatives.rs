mod common;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn verb(owner: &str, finite: bool) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: if finite { WordForm::Present } else { WordForm::Plain },
            features: if finite {
                FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    case: None,
                }
            } else {
                FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: Some(0),
        countability: None,
    }
}

fn numeral(value: i32, notation: Numeral) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn np() -> Reading {
    let mut head = word("lexeme:CommonNoun/Card");
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = WordForm::Plural;
        value.features.number = Some(Number::Plural);
    }
    head.countability = Some(true);
    Reading::BarePlural {
        form: 0,
        head: Box::new(Reading::Noun { form: 0, head }),
    }
}

fn predicate(finite: bool, owner: &str) -> Reading {
    if finite {
        Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: verb(owner, true),
            complements: vec![],
        }
    } else {
        Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: verb(owner, false),
            complements: vec![],
        }
    }
}

fn imperative(owner: &str) -> Reading {
    Reading::Imperative {
        form: 0,
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(predicate(false, owner)),
        }),
    }
}

fn mana(name: &str) -> Reading {
    Reading::ManaPhrase {
        form: 0,
        first: Box::new(Reading::NamedManaSymbol {
            form: 0,
            symbol: word(&format!("vocab:FixedCostSymbol/{name}")),
        }),
        rest: vec![],
    }
}

fn keyword(name: &str) -> Reading {
    Reading::BareKeyword {
        form: 0,
        head: word(&format!("lexeme:keyword_ability/{name}")),
    }
}

fn infinitive(owner: &str) -> Reading {
    Reading::ToInfinitive {
        form: 0,
        marker: word("vocab:InfinitivalMarker/To"),
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(predicate(false, owner)),
        }),
    }
}

fn fixtures() -> Vec<(Category, &'static str, Reading, &'static str, Reading)> {
    let mut values = vec![
        (
            Category::Clause,
            "attack",
            imperative("core-verb:Attack"),
            "block",
            imperative("core-verb:Block"),
        ),
        (
            Category::FinitePredicate,
            "attacks",
            predicate(true, "core-verb:Attack"),
            "blocks",
            predicate(true, "core-verb:Block"),
        ),
        (
            Category::SecondaryVerbPhrase,
            "attack",
            predicate(false, "core-verb:Attack"),
            "block",
            predicate(false, "core-verb:Block"),
        ),
        (Category::NounPhrase, "cards", np(), "cards", np()),
        (
            Category::PrepositionPhrase,
            "among cards",
            Reading::PrepositionPhrase {
                form: 0,
                head: word("vocab:Preposition/Among"),
                complement: Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(np()),
                }),
            },
            "among cards",
            Reading::PrepositionPhrase {
                form: 0,
                head: word("vocab:Preposition/Among"),
                complement: Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(np()),
                }),
            },
        ),
        (
            Category::AdverbPhrase,
            "again",
            Reading::Adverb {
                form: 0,
                head: word("vocab:Adverb/Again"),
            },
            "again",
            Reading::Adverb {
                form: 0,
                head: word("vocab:Adverb/Again"),
            },
        ),
        (
            Category::InfinitiveComplement,
            "to attack",
            infinitive("core-verb:Attack"),
            "to block",
            infinitive("core-verb:Block"),
        ),
        (
            Category::FiniteSelectedHead,
            "owns",
            Reading::SelectedVerbHead {
                category: Category::FiniteSelectedHead,
                form: 0,
                head: verb("core-verb:Own", true),
            },
            "controls",
            Reading::SelectedVerbHead {
                category: Category::FiniteSelectedHead,
                form: 0,
                head: verb("core-verb:Control", true),
            },
        ),
        (
            Category::SecondarySelectedHead,
            "own",
            Reading::SelectedVerbHead {
                category: Category::SecondarySelectedHead,
                form: 0,
                head: verb("core-verb:Own", false),
            },
            "control",
            Reading::SelectedVerbHead {
                category: Category::SecondarySelectedHead,
                form: 0,
                head: verb("core-verb:Control", false),
            },
        ),
    ];
    values.extend(notation_fixtures());
    values
}

fn notation_fixtures() -> Vec<(Category, &'static str, Reading, &'static str, Reading)> {
    vec![
        (
            Category::ManaPhrase,
            "{W}",
            mana("White"),
            "{U}",
            mana("Blue"),
        ),
        (
            Category::Cardinal,
            "one",
            Reading::Cardinal {
                form: 0,
                head: numeral(1, Numeral::Cardinal),
            },
            "two",
            Reading::Cardinal {
                form: 0,
                head: numeral(2, Numeral::Cardinal),
            },
        ),
        (
            Category::Amount,
            "1",
            Reading::ScalarAmount {
                form: 0,
                value: Box::new(Reading::SmallUnsignedScalar {
                    form: 0,
                    head: numeral(1, Numeral::Arabic(false)),
                }),
            },
            "2",
            Reading::ScalarAmount {
                form: 0,
                value: Box::new(Reading::SmallUnsignedScalar {
                    form: 0,
                    head: numeral(2, Numeral::Arabic(false)),
                }),
            },
        ),
        (
            Category::MeasurePhrase,
            "1",
            Reading::UngroupedScalarNumeral {
                form: 0,
                head: numeral(1, Numeral::Arabic(false)),
            },
            "2",
            Reading::UngroupedScalarNumeral {
                form: 0,
                head: numeral(2, Numeral::Arabic(false)),
            },
        ),
        (
            Category::KeywordPhrase,
            "flying",
            keyword("flying"),
            "haste",
            keyword("haste"),
        ),
        (
            Category::QuotedText,
            "\"flying\"",
            Reading::QuotedKeyword {
                form: 0,
                keyword: Box::new(keyword("flying")),
            },
            "\"haste\"",
            Reading::QuotedKeyword {
                form: 0,
                keyword: Box::new(keyword("haste")),
            },
        ),
    ]
}

fn marker(kind: &str) -> Word {
    word(match kind {
        "Both" => "vocab:FloatedQuantifier/Both",
        "Either" => "vocab:Determinative/Either",
        "Neither" => "vocab:Determinative/Neither",
        _ => unreachable!(),
    })
}

fn coordinator(kind: &str) -> Word {
    word(match kind {
        "Both" => "vocab:Coordinator/And",
        "Either" => "vocab:Coordinator/Or",
        "Neither" => "vocab:Coordinator/Nor",
        _ => unreachable!(),
    })
}

fn binary(category: Category, kind: &str, left: Reading, right: Reading) -> Reading {
    let marker = marker(kind);
    let coordinator = coordinator(kind);
    let left = Box::new(left);
    let right = Box::new(right);
    macro_rules! pair {
        ($variant:ident) => {
            Reading::$variant {
                category,
                form: 0,
                marker,
                left,
                coordinator,
                right,
            }
        };
    }
    match kind {
        "Both" => pair!(BothCoordination),
        "Either" => pair!(EitherCoordination),
        "Neither" => pair!(NeitherCoordination),
        _ => unreachable!("unsupported correlative placement"),
    }
}

fn serial(
    category: Category,
    kind: &str,
    first: Reading,
    second: Reading,
    third: Reading,
) -> Reading {
    let marker = marker(kind);
    let coordinator = coordinator(kind);
    let left = Box::new(second);
    let right = Box::new(third);
    let series = match category {
        Category::Clause => Category::CorrelativeClauseSeries,
        Category::FinitePredicate => Category::CorrelativeFinitePredicateSeries,
        Category::SecondaryVerbPhrase => Category::CorrelativeSecondaryVerbPhraseSeries,
        Category::NounPhrase => Category::CorrelativeNounPhraseSeries,
        Category::PrepositionPhrase => Category::CorrelativePrepositionPhraseSeries,
        Category::AdverbPhrase => Category::CorrelativeAdverbPhraseSeries,
        Category::ManaPhrase => Category::CorrelativeManaPhraseSeries,
        Category::Cardinal => Category::CorrelativeCardinalSeries,
        Category::Amount => Category::CorrelativeAmountSeries,
        Category::MeasurePhrase => Category::CorrelativeMeasurePhraseSeries,
        Category::KeywordPhrase => Category::CorrelativeKeywordPhraseSeries,
        Category::QuotedText => Category::CorrelativeQuotedTextSeries,
        Category::InfinitiveComplement => Category::CorrelativeInfinitiveComplementSeries,
        Category::FiniteSelectedHead => Category::CorrelativeFiniteSelectedHeadSeries,
        Category::SecondarySelectedHead => Category::CorrelativeSecondarySelectedHeadSeries,
        _ => unreachable!(),
    };
    let rest = Box::new(Reading::CorrelativeSeriesEnd {
        category: series,
        form: 0,
        left,
        coordinator,
        right,
    });
    let left = Box::new(first);
    macro_rules! value {
        ($variant:ident) => {
            Reading::$variant {
                category,
                form: 0,
                marker,
                left,
                rest,
            }
        };
    }
    match kind {
        "Either" => value!(EitherSerialCoordination),
        "Neither" => value!(NeitherSerialCoordination),
        _ => unreachable!(),
    }
}

fn roundtrip(value: &Reading, text: &str, category: Category) {
    assert_eq!(value.realize(&LEXICON).unwrap(), text);
    assert!(
        readings(text, category).contains(value),
        "independent value missing for {text}"
    );
}

#[test]
fn independent_correlative_pairs_preserve_every_supported_phrase_family() {
    for (category, left_text, left, right_text, right) in fixtures() {
        for (kind, link) in [("Both", "and"), ("Either", "or"), ("Neither", "nor")] {
            if category == Category::Clause && kind != "Either" {
                continue;
            }
            let value = binary(category, kind, left.clone(), right.clone());
            let text = format!("{} {left_text} {link} {right_text}", kind.to_lowercase());
            roundtrip(&value, &text, category);
        }
    }
}

#[test]
fn independent_either_and_neither_serials_require_final_comma_and_pair_discharge() {
    for (category, left_text, left, right_text, right) in fixtures() {
        for (kind, link) in [("Either", "or"), ("Neither", "nor")] {
            if category == Category::Clause && kind != "Either" {
                continue;
            }
            let value = serial(category, kind, left.clone(), right.clone(), left.clone());
            let text = format!(
                "{} {left_text}, {right_text}, {link} {left_text}",
                kind.to_lowercase()
            );
            roundtrip(&value, &text, category);
            if kind == "Neither" {
                let text = format!(
                    "{} {left_text}, {right_text} {link} {left_text}",
                    kind.to_lowercase()
                );
                assert_eq!(readings(&text, category).len(), 0, "{text}");
            }
        }
    }
}

#[test]
fn malformed_pairs_bare_nor_and_disallowed_main_clause_prefixes_are_rejected() {
    let invalid = Reading::BothCoordination {
        category: Category::NounPhrase,
        form: 0,
        marker: marker("Both"),
        left: Box::new(np()),
        coordinator: coordinator("Either"),
        right: Box::new(np()),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    let invalid = Reading::NeitherCoordination {
        category: Category::NounPhrase,
        form: 0,
        marker: marker("Either"),
        left: Box::new(np()),
        coordinator: coordinator("Neither"),
        right: Box::new(np()),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    let invalid = Reading::EitherSerialCoordination {
        category: Category::NounPhrase,
        form: 0,
        marker: marker("Either"),
        left: Box::new(np()),
        rest: Box::new(np()),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    for (text, category) in [
        ("cards nor cards", Category::NounPhrase),
        ("attack nor block", Category::SecondaryVerbPhrase),
        ("{W} nor {U}", Category::ManaPhrase),
        ("either {W} nor {U}", Category::ManaPhrase),
        ("neither {W} or {U}", Category::ManaPhrase),
        ("both you attack and they attack", Category::Clause),
        ("neither you attack nor they attack", Category::Clause),
    ] {
        assert_eq!(readings(text, category).len(), 0, "{text}");
    }
}

#[test]
fn correlative_selected_heads_reuse_shared_object_hosts_without_duplicate_atomic_host() {
    let own = Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: verb("core-verb:Own", false),
    };
    let control = Reading::SelectedVerbHead {
        category: Category::SecondarySelectedHead,
        form: 0,
        head: verb("core-verb:Control", false),
    };
    let value = Reading::SharedObjectComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(binary(
            Category::SecondarySelectedHead,
            "Both",
            own,
            control,
        )),
        object: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(np()),
        }),
    };
    roundtrip(
        &value,
        "both own and control cards",
        Category::SecondaryVerbPhrase,
    );
}
