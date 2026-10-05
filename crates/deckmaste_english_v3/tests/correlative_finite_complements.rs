mod common;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::*;

fn lexical(
    owner: &str,
    form: WordForm,
    features: FeatureBundle,
    frame: Option<usize>,
    countability: Option<bool>,
) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability,
    }
}

fn clause(verb: &str) -> Reading {
    Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::BarePlural {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: lexical(
                        "lexeme:type/creature",
                        WordForm::Plural,
                        FeatureBundle {
                            number: Some(Number::Plural),
                            ..Default::default()
                        },
                        None,
                        Some(true),
                    ),
                }),
            }),
        }),
        predicate: Box::new(Reading::IntransitivePredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: lexical(
                verb,
                WordForm::Present,
                FeatureBundle {
                    number: Some(Number::Plural),
                    person: Some(Person::Third),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    case: None,
                },
                Some(0),
                None,
            ),
        }),
    }
}
fn marker(owner: &str) -> Word {
    lexical(
        owner,
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
        None,
    )
}

#[test]
fn independent_either_finite_clauses_keep_each_selected_finite_constituent() {
    let left = clause("core-verb:Attack");
    let right = clause("core-verb:Block");
    assert!(readings("creatures attack", Category::FiniteClause).contains(&left));
    assert!(readings("creatures block", Category::FiniteClause).contains(&right));
    let expected = Reading::EitherCoordination {
        category: Category::CoordinatedFiniteClause,
        form: 0,
        marker: marker("vocab:Determinative/Either"),
        left: Box::new(left.clone()),
        coordinator: marker("vocab:Coordinator/Or"),
        right: Box::new(right.clone()),
    };
    assert_eq!(
        expected.realize(&LEXICON).unwrap(),
        "either creatures attack or creatures block"
    );
    assert!(
        readings(
            "either creatures attack or creatures block",
            Category::CoordinatedFiniteClause
        )
        .contains(&expected)
    );
    let pp = Reading::CoordinatedClauseComplementPreposition {
        form: 0,
        head: marker("vocab:Preposition/If"),
        complement: Box::new(expected),
    };
    assert_eq!(
        pp.realize(&LEXICON).unwrap(),
        "if either creatures attack or creatures block"
    );
    assert!(
        readings(
            "if either creatures attack or creatures block",
            Category::PrepositionPhrase
        )
        .contains(&pp)
    );
    let serial = Reading::EitherSerialCoordination {
        category: Category::CoordinatedFiniteClause,
        form: 0,
        marker: marker("vocab:Determinative/Either"),
        left: Box::new(left),
        rest: Box::new(Reading::CorrelativeSeriesEnd {
            category: Category::CorrelativeFiniteClauseSeries,
            form: 0,
            left: Box::new(right),
            coordinator: marker("vocab:Coordinator/Or"),
            right: Box::new(clause("core-verb:Enter")),
        }),
    };
    assert_eq!(
        serial.realize(&LEXICON).unwrap(),
        "either creatures attack, creatures block, or creatures enter"
    );
    assert!(
        readings(
            "either creatures attack, creatures block, or creatures enter",
            Category::CoordinatedFiniteClause
        )
        .contains(&serial)
    );
}

#[test]
fn selected_either_complements_require_finite_conjuncts_and_oxford_pairing() {
    for text in [
        "if either you draw cards or they discard cards",
        "if either you draw cards, they discard cards, or creatures attack",
    ] {
        assert!(
            !readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
    for text in [
        "if both you draw cards and they discard cards",
        "if neither you draw cards nor they discard cards",
        "if either draw cards or they discard cards",
        "if either you draw cards nor they discard cards",
        "if either you draw cards, they discard cards or creatures attack",
    ] {
        assert!(
            readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
}
