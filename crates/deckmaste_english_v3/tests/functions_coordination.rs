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

fn coordinator() -> Word {
    lexical(
        "vocab:Coordinator/And",
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
        None,
    )
}
fn bare_gap(owner: &str) -> Reading {
    Reading::BareObjectGap {
        form: 0,
        head: lexical(
            owner,
            WordForm::Plain,
            FeatureBundle::default(),
            Some(0),
            None,
        ),
    }
}
fn finite_gap(owner: &str) -> Reading {
    Reading::FiniteObjectGap {
        form: 0,
        head: lexical(
            owner,
            WordForm::Present,
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                case: None,
            },
            Some(0),
            None,
        ),
    }
}

#[test]
fn independent_object_gaps_keep_frames_and_concord_before_coordination() {
    let bare = bare_gap("core-verb:Control");
    assert_eq!(bare.realize(&LEXICON).unwrap(), "control");
    assert_eq!(
        readings("control", Category::BareObjectGap),
        BTreeSet::from([bare.clone()])
    );
    let finite = finite_gap("core-verb:Control");
    assert_eq!(finite.realize(&LEXICON).unwrap(), "controls");
    assert_eq!(
        readings("controls", Category::FiniteObjectGap),
        BTreeSet::from([finite])
    );
    let binary = Reading::BareObjectGapCoordination {
        form: 0,
        left: Box::new(bare),
        coordinator: coordinator(),
        right: Box::new(bare_gap("core-verb:Own")),
    };
    assert_eq!(binary.realize(&LEXICON).unwrap(), "control and own");
    assert!(readings("control and own", Category::BareObjectGap).contains(&binary));
    let serial = Reading::SerialFiniteObjectGap {
        form: 0,
        left: Box::new(finite_gap("core-verb:Control")),
        rest: Box::new(Reading::FiniteObjectGapSeriesEnd {
            form: 0,
            left: Box::new(finite_gap("core-verb:Own")),
            coordinator: coordinator(),
            right: Box::new(finite_gap("core-verb:Draw")),
        }),
    };
    assert_eq!(
        serial.realize(&LEXICON).unwrap(),
        "controls, owns, and draws"
    );
    assert!(readings("controls, owns, and draws", Category::FiniteObjectGap).contains(&serial));
    for text in [
        "controls, owns and draws",
        "controls and own",
        "controls nor owns",
    ] {
        assert!(
            readings(text, Category::FiniteObjectGap).is_empty(),
            "{text}"
        );
    }
    for text in [
        "control, own and draw",
        "control nor own",
        "controls and own",
    ] {
        assert!(readings(text, Category::BareObjectGap).is_empty(), "{text}");
    }
    assert!(!readings("control, own, and draw", Category::BareObjectGap).is_empty());
}

#[test]
fn selected_finite_clause_complements_coordinate_without_imperative_contamination() {
    for text in [
        "if you draw cards",
        "if you draw cards and they discard cards",
        "if you draw cards, they discard cards, and creatures attack",
    ] {
        assert!(
            !readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
    for text in [
        "if draw cards and they discard cards",
        "if you draw cards, they discard cards and creatures attack",
    ] {
        assert!(
            readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
    assert!(
        readings(
            "draw cards and they discard cards",
            Category::CoordinatedFiniteClause
        )
        .is_empty()
    );
    assert!(
        readings(
            "you draw cards and discard cards",
            Category::CoordinatedFiniteClause
        )
        .is_empty()
    );
}

#[test]
fn unlike_predicative_categories_share_a_function_without_erasing_their_categories() {
    let adjective = Reading::AdjectivalComplement {
        form: 0,
        phrase: Box::new(Reading::Adjective {
            form: 0,
            head: lexical(
                "vocab:ColorWord/Red",
                WordForm::Invariant,
                FeatureBundle::default(),
                None,
                None,
            ),
        }),
    };
    let nominal = Reading::NominalComplement {
        form: 0,
        phrase: Box::new(Reading::AccusativePhrase {
            form: 0,
            head: Box::new(Reading::IndefiniteNounPhrase {
                form: 0,
                determiner: lexical(
                    "vocab:Article/Indefinite",
                    WordForm::Invariant,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        ..Default::default()
                    },
                    None,
                    None,
                ),
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: lexical(
                        "lexeme:type/creature",
                        WordForm::Singular,
                        FeatureBundle {
                            number: Some(Number::Singular),
                            ..Default::default()
                        },
                        None,
                        Some(true),
                    ),
                }),
            }),
        }),
    };
    assert!(readings("red", Category::PredicativeComplement).contains(&adjective));
    assert!(readings("a creature", Category::PredicativeComplement).contains(&nominal));
    let expected = Reading::UnlikePredicativeCoordination {
        form: 0,
        left: Box::new(adjective),
        coordinator: coordinator(),
        right: Box::new(nominal),
    };
    assert_eq!(expected.realize(&LEXICON).unwrap(), "red and a creature");
    assert!(readings("red and a creature", Category::PredicativeComplement).contains(&expected));
    for text in ["red, white, and a creature", "a creature and red"] {
        assert!(
            !readings(text, Category::PredicativeComplement).is_empty(),
            "{text}"
        );
    }
    for text in ["red, white and a creature", "red nor a creature"] {
        assert!(
            readings(text, Category::PredicativeComplement).is_empty(),
            "{text}"
        );
    }
}
