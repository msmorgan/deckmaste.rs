mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::*;

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

fn numeral(value: i32) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation: Numeral::Cardinal,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn arabic(value: i32) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::Cardinal {
        form: 0,
        head: numeral(value),
    }
}

fn mana(owner: &str) -> Reading {
    Reading::ManaPhrase {
        form: 0,
        first: Box::new(Reading::NamedManaSymbol {
            form: 0,
            symbol: word(owner),
        }),
        rest: vec![],
    }
}

fn keyword(owner: &str) -> Reading {
    Reading::BareKeyword {
        form: 0,
        head: word(owner),
    }
}

#[test]
fn independent_noncoordinated_units_survive() {
    for (text, category, value) in [
        (
            "2",
            Category::MeasurePhrase,
            Reading::UngroupedScalarNumeral {
                form: 0,
                head: arabic(2),
            },
        ),
        (
            "\"flying\"",
            Category::QuotedText,
            Reading::QuotedKeyword {
                form: 0,
                keyword: Box::new(keyword("lexeme:keyword_ability/flying")),
            },
        ),
        ("one", Category::Cardinal, cardinal(1)),
        (
            "{W}",
            Category::ManaPhrase,
            mana("vocab:FixedCostSymbol/White"),
        ),
        (
            "flying",
            Category::KeywordPhrase,
            keyword("lexeme:keyword_ability/flying"),
        ),
        (
            "one",
            Category::Amount,
            Reading::CardinalAmount {
                form: 0,
                head: numeral(1),
            },
        ),
    ] {
        assert_eq!(value.realize(&LEXICON).unwrap(), text);
        assert_eq!(readings(text, category), BTreeSet::from([value]));
    }
}

#[test]
fn independently_constructed_binary_and_oxford_series_retain_every_child() {
    let binary = Reading::Coordination {
        category: Category::ManaPhrase,
        form: 0,
        left: Box::new(mana("vocab:FixedCostSymbol/White")),
        coordinator: word("vocab:Coordinator/Or"),
        right: Box::new(mana("vocab:FixedCostSymbol/Blue")),
    };
    assert_eq!(binary.realize(&LEXICON).unwrap(), "{W} or {U}");
    assert!(readings("{W} or {U}", Category::ManaPhrase).contains(&binary));
    let series = Reading::SerialCoordination {
        category: Category::ManaPhrase,
        form: 0,
        left: Box::new(mana("vocab:FixedCostSymbol/White")),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::ManaPhraseSeries,
            form: 0,
            left: Box::new(mana("vocab:FixedCostSymbol/Blue")),
            coordinator: word("vocab:Coordinator/Or"),
            right: Box::new(mana("vocab:FixedCostSymbol/Black")),
        }),
    };
    assert_eq!(series.realize(&LEXICON).unwrap(), "{W}, {U}, or {B}");
    assert!(readings("{W}, {U}, or {B}", Category::ManaPhrase).contains(&series));
    let counted = Reading::Coordination {
        category: Category::Cardinal,
        form: 0,
        left: Box::new(cardinal(1)),
        coordinator: word("vocab:Coordinator/Or"),
        right: Box::new(cardinal(2)),
    };
    assert!(readings("one or two", Category::Cardinal).contains(&counted));
    let grant = Reading::Coordination {
        category: Category::KeywordPhrase,
        form: 0,
        left: Box::new(keyword("lexeme:keyword_ability/flying")),
        coordinator: word("vocab:Coordinator/And"),
        right: Box::new(keyword("lexeme:keyword_ability/haste")),
    };
    assert!(readings("flying and haste", Category::KeywordPhrase).contains(&grant));
}

#[test]
fn all_notation_families_compose_and_reject_non_oxford_serials() {
    for (base, binary, serial, invalid, category) in [
        (
            "{W}",
            "{W} or {U}",
            "{W}, {U}, or {B}",
            "{W}, {U} or {B}",
            Category::ManaPhrase,
        ),
        (
            "one",
            "one or two",
            "one, two, or three",
            "one, two or three",
            Category::Cardinal,
        ),
        (
            "one",
            "one or two",
            "one, two, or three",
            "one, two or three",
            Category::Amount,
        ),
        (
            "2",
            "2 or 3",
            "2, 3, or 4",
            "2, 3 or 4",
            Category::MeasurePhrase,
        ),
        (
            "flying",
            "flying and haste",
            "flying, haste, and vigilance",
            "flying, haste and vigilance",
            Category::KeywordPhrase,
        ),
        (
            "\"flying\"",
            "\"flying\" and \"haste\"",
            "\"flying\", \"haste\", and \"vigilance\"",
            "\"flying\", \"haste\" and \"vigilance\"",
            Category::QuotedText,
        ),
    ] {
        for text in [base, binary, serial] {
            assert!(!readings(text, category).is_empty(), "{text}");
        }
        assert!(readings(invalid, category).is_empty(), "{invalid}");
    }
    assert!(readings("2 or +1/+1", Category::MeasurePhrase).is_empty());
    // Keyword lines are comma-separated ability units, not Oxford lists.
    assert!(!readings("Flying, vigilance", Category::Ability).is_empty());
    assert!(!readings("{W}{U}", Category::ManaPhrase).is_empty());
}

#[test]
fn cardinal_coordination_agrees_with_the_whole_range() {
    for text in [
        "one or two creatures",
        "two or one creatures",
        "one and one creatures",
        "one or one creature",
        "one, two, or three creatures",
    ] {
        assert!(!readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for text in [
        "one or two creature",
        "two or one creature",
        "one and one creature",
        "one or one creatures",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn correlative_markers_keep_determinative_identity_and_nor_is_distinct() {
    for (owner, expected) in [
        ("vocab:FloatedQuantifier/Both", "Both"),
        ("vocab:Determinative/Either", "Either"),
        ("vocab:Determinative/Neither", "Neither"),
    ] {
        let entry = &LEXICON.lexemes()[owner];
        assert_eq!(entry.category, deckmaste_lexical::Category::Determinative);
        assert_eq!(entry.properties.features["CorrelativeKind"], expected);
    }
    let nor = &LEXICON.lexemes()["vocab:Coordinator/Nor"];
    assert_eq!(nor.category, deckmaste_lexical::Category::Coordinator);
    assert_eq!(nor.properties.features["CoordinationKind"], "Alternative");
    assert_eq!(nor.properties.features["CorrelativeCoordinator"], "Nor");
    assert_eq!(nor.properties.features["NoncorrelativeCoordination"], "No");
    let inclusive = &LEXICON.lexemes()["vocab:Coordinator/AndOr"];
    assert_eq!(
        inclusive.properties.features["NoncorrelativeCoordination"],
        "Yes"
    );
    assert_eq!(
        inclusive.properties.features["CoordinationKind"],
        "Alternative"
    );
    assert!(
        !inclusive
            .properties
            .features
            .contains_key("CorrelativeCoordinator")
    );
}

#[test]
fn bare_nor_needs_negative_or_correlative_licensing() {
    for (text, category) in [
        ("{W} nor {U}", Category::ManaPhrase),
        ("one nor two", Category::Cardinal),
        ("one nor two", Category::Amount),
        ("2 nor 3", Category::MeasurePhrase),
        ("flying nor haste", Category::KeywordPhrase),
        ("\"flying\" nor \"haste\"", Category::QuotedText),
    ] {
        assert!(readings(text, category).is_empty(), "{text}");
    }
    for (text, category) in [
        ("flying and/or reach", Category::KeywordPhrase),
        ("one and/or two", Category::Cardinal),
    ] {
        assert!(!readings(text, category).is_empty(), "{text}");
    }
}
