use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
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
        Reading::FiniteIntransitive {
            form: 0,
            head: verb(owner, true),
        }
    } else {
        Reading::SecondaryIntransitive {
            form: 0,
            head: verb(owner, false),
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
                complement: Box::new(Reading::AccusativePhrase {
                    form: 0,
                    head: Box::new(np()),
                }),
            },
            "among cards",
            Reading::PrepositionPhrase {
                form: 0,
                head: word("vocab:Preposition/Among"),
                complement: Box::new(Reading::AccusativePhrase {
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
            Reading::FiniteSelectedObjectHead {
                form: 0,
                head: verb("core-verb:Own", true),
            },
            "controls",
            Reading::FiniteSelectedObjectHead {
                form: 0,
                head: verb("core-verb:Control", true),
            },
        ),
        (
            Category::SecondarySelectedHead,
            "own",
            Reading::SecondarySelectedObjectHead {
                form: 0,
                head: verb("core-verb:Own", false),
            },
            "control",
            Reading::SecondarySelectedObjectHead {
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
                form: 0,
                marker,
                left,
                coordinator,
                right,
            }
        };
    }
    match (category, kind) {
        (Category::Clause, "Either") => pair!(EitherClauseCoordination),
        (Category::FinitePredicate, "Both") => pair!(BothFinitePredicateCoordination),
        (Category::FinitePredicate, "Either") => pair!(EitherFinitePredicateCoordination),
        (Category::FinitePredicate, "Neither") => pair!(NeitherFinitePredicateCoordination),
        (Category::SecondaryVerbPhrase, "Both") => pair!(BothSecondaryVerbPhraseCoordination),
        (Category::SecondaryVerbPhrase, "Either") => {
            pair!(EitherSecondaryVerbPhraseCoordination)
        }
        (Category::SecondaryVerbPhrase, "Neither") => {
            pair!(NeitherSecondaryVerbPhraseCoordination)
        }
        (Category::NounPhrase, "Both") => pair!(BothNounPhraseCoordination),
        (Category::NounPhrase, "Either") => pair!(EitherNounPhraseCoordination),
        (Category::NounPhrase, "Neither") => pair!(NeitherNounPhraseCoordination),
        (Category::PrepositionPhrase, "Both") => pair!(BothPrepositionPhraseCoordination),
        (Category::PrepositionPhrase, "Either") => pair!(EitherPrepositionPhraseCoordination),
        (Category::PrepositionPhrase, "Neither") => pair!(NeitherPrepositionPhraseCoordination),
        (Category::AdverbPhrase, "Both") => pair!(BothAdverbPhraseCoordination),
        (Category::AdverbPhrase, "Either") => pair!(EitherAdverbPhraseCoordination),
        (Category::AdverbPhrase, "Neither") => pair!(NeitherAdverbPhraseCoordination),
        (Category::ManaPhrase, "Both") => pair!(BothManaPhraseCoordination),
        (Category::ManaPhrase, "Either") => pair!(EitherManaPhraseCoordination),
        (Category::ManaPhrase, "Neither") => pair!(NeitherManaPhraseCoordination),
        (Category::Cardinal, "Both") => pair!(BothCardinalCoordination),
        (Category::Cardinal, "Either") => pair!(EitherCardinalCoordination),
        (Category::Cardinal, "Neither") => pair!(NeitherCardinalCoordination),
        (Category::Amount, "Both") => pair!(BothAmountCoordination),
        (Category::Amount, "Either") => pair!(EitherAmountCoordination),
        (Category::Amount, "Neither") => pair!(NeitherAmountCoordination),
        (Category::MeasurePhrase, "Both") => pair!(BothMeasurePhraseCoordination),
        (Category::MeasurePhrase, "Either") => pair!(EitherMeasurePhraseCoordination),
        (Category::MeasurePhrase, "Neither") => pair!(NeitherMeasurePhraseCoordination),
        (Category::KeywordPhrase, "Both") => pair!(BothKeywordPhraseCoordination),
        (Category::KeywordPhrase, "Either") => pair!(EitherKeywordPhraseCoordination),
        (Category::KeywordPhrase, "Neither") => pair!(NeitherKeywordPhraseCoordination),
        (Category::QuotedText, "Both") => pair!(BothQuotedTextCoordination),
        (Category::QuotedText, "Either") => pair!(EitherQuotedTextCoordination),
        (Category::QuotedText, "Neither") => pair!(NeitherQuotedTextCoordination),
        (Category::InfinitiveComplement, "Both") => pair!(BothInfinitiveComplementCoordination),
        (Category::InfinitiveComplement, "Either") => {
            pair!(EitherInfinitiveComplementCoordination)
        }
        (Category::InfinitiveComplement, "Neither") => {
            pair!(NeitherInfinitiveComplementCoordination)
        }
        (Category::FiniteSelectedHead, "Both") => pair!(BothFiniteSelectedHeadCoordination),
        (Category::FiniteSelectedHead, "Either") => pair!(EitherFiniteSelectedHeadCoordination),
        (Category::FiniteSelectedHead, "Neither") => {
            pair!(NeitherFiniteSelectedHeadCoordination)
        }
        (Category::SecondarySelectedHead, "Both") => {
            pair!(BothSecondarySelectedHeadCoordination)
        }
        (Category::SecondarySelectedHead, "Either") => {
            pair!(EitherSecondarySelectedHeadCoordination)
        }
        (Category::SecondarySelectedHead, "Neither") => {
            pair!(NeitherSecondarySelectedHeadCoordination)
        }
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
    macro_rules! tail {
        ($variant:ident) => {
            Reading::$variant {
                form: 0,
                left,
                coordinator,
                right,
            }
        };
    }
    let rest = Box::new(match category {
        Category::Clause => tail!(CorrelativeClauseSeriesEnd),
        Category::FinitePredicate => tail!(CorrelativeFinitePredicateSeriesEnd),
        Category::SecondaryVerbPhrase => tail!(CorrelativeSecondaryVerbPhraseSeriesEnd),
        Category::NounPhrase => tail!(CorrelativeNounPhraseSeriesEnd),
        Category::PrepositionPhrase => tail!(CorrelativePrepositionPhraseSeriesEnd),
        Category::AdverbPhrase => tail!(CorrelativeAdverbPhraseSeriesEnd),
        Category::ManaPhrase => tail!(CorrelativeManaPhraseSeriesEnd),
        Category::Cardinal => tail!(CorrelativeCardinalSeriesEnd),
        Category::Amount => tail!(CorrelativeAmountSeriesEnd),
        Category::MeasurePhrase => tail!(CorrelativeMeasurePhraseSeriesEnd),
        Category::KeywordPhrase => tail!(CorrelativeKeywordPhraseSeriesEnd),
        Category::QuotedText => tail!(CorrelativeQuotedTextSeriesEnd),
        Category::InfinitiveComplement => tail!(CorrelativeInfinitiveComplementSeriesEnd),
        Category::FiniteSelectedHead => tail!(CorrelativeFiniteSelectedHeadSeriesEnd),
        Category::SecondarySelectedHead => tail!(CorrelativeSecondarySelectedHeadSeriesEnd),
        _ => unreachable!(),
    });
    let left = Box::new(first);
    macro_rules! value {
        ($variant:ident) => {
            Reading::$variant {
                form: 0,
                marker,
                left,
                rest,
            }
        };
    }
    match (category, kind) {
        (Category::Clause, "Either") => value!(SerialEitherClauseCoordination),
        (Category::FinitePredicate, "Either") => value!(SerialEitherFinitePredicateCoordination),
        (Category::FinitePredicate, "Neither") => {
            value!(SerialNeitherFinitePredicateCoordination)
        }
        (Category::SecondaryVerbPhrase, "Either") => {
            value!(SerialEitherSecondaryVerbPhraseCoordination)
        }
        (Category::SecondaryVerbPhrase, "Neither") => {
            value!(SerialNeitherSecondaryVerbPhraseCoordination)
        }
        (Category::NounPhrase, "Either") => value!(SerialEitherNounPhraseCoordination),
        (Category::NounPhrase, "Neither") => value!(SerialNeitherNounPhraseCoordination),
        (Category::PrepositionPhrase, "Either") => {
            value!(SerialEitherPrepositionPhraseCoordination)
        }
        (Category::PrepositionPhrase, "Neither") => {
            value!(SerialNeitherPrepositionPhraseCoordination)
        }
        (Category::AdverbPhrase, "Either") => value!(SerialEitherAdverbPhraseCoordination),
        (Category::AdverbPhrase, "Neither") => value!(SerialNeitherAdverbPhraseCoordination),
        (Category::ManaPhrase, "Either") => value!(SerialEitherManaPhraseCoordination),
        (Category::ManaPhrase, "Neither") => value!(SerialNeitherManaPhraseCoordination),
        (Category::Cardinal, "Either") => value!(SerialEitherCardinalCoordination),
        (Category::Cardinal, "Neither") => value!(SerialNeitherCardinalCoordination),
        (Category::Amount, "Either") => value!(SerialEitherAmountCoordination),
        (Category::Amount, "Neither") => value!(SerialNeitherAmountCoordination),
        (Category::MeasurePhrase, "Either") => value!(SerialEitherMeasurePhraseCoordination),
        (Category::MeasurePhrase, "Neither") => value!(SerialNeitherMeasurePhraseCoordination),
        (Category::KeywordPhrase, "Either") => value!(SerialEitherKeywordPhraseCoordination),
        (Category::KeywordPhrase, "Neither") => value!(SerialNeitherKeywordPhraseCoordination),
        (Category::QuotedText, "Either") => value!(SerialEitherQuotedTextCoordination),
        (Category::QuotedText, "Neither") => value!(SerialNeitherQuotedTextCoordination),
        (Category::InfinitiveComplement, "Either") => {
            value!(SerialEitherInfinitiveComplementCoordination)
        }
        (Category::InfinitiveComplement, "Neither") => {
            value!(SerialNeitherInfinitiveComplementCoordination)
        }
        (Category::FiniteSelectedHead, "Either") => {
            value!(SerialEitherFiniteSelectedHeadCoordination)
        }
        (Category::FiniteSelectedHead, "Neither") => {
            value!(SerialNeitherFiniteSelectedHeadCoordination)
        }
        (Category::SecondarySelectedHead, "Either") => {
            value!(SerialEitherSecondarySelectedHeadCoordination)
        }
        (Category::SecondarySelectedHead, "Neither") => {
            value!(SerialNeitherSecondarySelectedHeadCoordination)
        }
        _ => unreachable!(),
    }
}

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &input, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|value| {
            let value = value.unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), text);
            value
        })
        .collect()
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
    let invalid = Reading::BothNounPhraseCoordination {
        form: 0,
        marker: marker("Both"),
        left: Box::new(np()),
        coordinator: coordinator("Either"),
        right: Box::new(np()),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    let invalid = Reading::NeitherNounPhraseCoordination {
        form: 0,
        marker: marker("Either"),
        left: Box::new(np()),
        coordinator: coordinator("Neither"),
        right: Box::new(np()),
    };
    assert!(invalid.admit(&LEXICON).is_err());
    let invalid = Reading::SerialEitherNounPhraseCoordination {
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
    let own = Reading::SecondarySelectedObjectHead {
        form: 0,
        head: verb("core-verb:Own", false),
    };
    let control = Reading::SecondarySelectedObjectHead {
        form: 0,
        head: verb("core-verb:Control", false),
    };
    let value = Reading::SecondarySharedObjectComplement {
        form: 0,
        head: Box::new(binary(
            Category::SecondarySelectedHead,
            "Both",
            own,
            control,
        )),
        object: Box::new(Reading::AccusativePhrase {
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
