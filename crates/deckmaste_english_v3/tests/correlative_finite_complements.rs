use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::{Category, Grammar, Reading, Word};
use deckmaste_english_v3::parse;
use deckmaste_lexical::*;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &input, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|reading| {
            let reading = reading.unwrap();
            assert_eq!(reading.realize(&LEXICON).unwrap(), text);
            reading
        })
        .collect()
}

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
        subject: Box::new(Reading::NominativePhrase {
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
        predicate: Box::new(Reading::FiniteIntransitive {
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
    let expected = Reading::EitherCoordinatedFiniteClause {
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
    let serial = Reading::SerialEitherCoordinatedFiniteClause {
        form: 0,
        marker: marker("vocab:Determinative/Either"),
        left: Box::new(left),
        rest: Box::new(Reading::CorrelativeFiniteClauseSeriesEnd {
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
