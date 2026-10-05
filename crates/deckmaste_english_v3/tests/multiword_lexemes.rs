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
        .map(|value| {
            let value = value.unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), text);
            value
        })
        .collect()
}
fn noun_word(owner: &str, number: Number, casing: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            variant: 0,
            capitalization: casing,
        }),
        frame: None,
        countability: Some(true),
    }
}

#[test]
fn independent_multiword_nominals_own_one_lexical_leaf_in_both_roundtrip_directions() {
    for (owner, singular, plural) in [
        ("lexeme:CommonNoun/ManaValue", "mana value", "mana values"),
        ("lexeme:CommonNoun/ManaCost", "mana cost", "mana costs"),
    ] {
        for (text, number) in [(singular, Number::Singular), (plural, Number::Plural)] {
            for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
                let text = if casing == SurfaceCase::Initial {
                    format!("M{}", &text[1..])
                } else {
                    text.to_owned()
                };
                let leaf = noun_word(owner, number, casing);
                let expected = Reading::Noun {
                    form: 0,
                    head: leaf.clone(),
                };
                assert_eq!(
                    expected.realize(&LEXICON).unwrap().as_bytes(),
                    text.as_bytes()
                );
                let summary = expected.admit(&LEXICON).unwrap();
                let parsed = readings(&text, Category::Nominal);
                assert!(parsed.contains(&expected), "{text}");
                assert_eq!(
                    parsed.get(&expected).unwrap().admit(&LEXICON).unwrap(),
                    summary
                );
                let mut words = vec![];
                parsed
                    .get(&expected)
                    .unwrap()
                    .visit_words(&mut |word| words.push(word.clone()))
                    .unwrap();
                assert_eq!(words, [leaf]);
                assert_eq!(LEXICON.realize(&words[0].value).unwrap(), text);
            }
        }
    }
}

#[test]
fn grammar_consumes_noun_number_and_countability_without_decomposing_the_lexeme() {
    for owner in ["lexeme:CommonNoun/ManaValue", "lexeme:CommonNoun/ManaCost"] {
        let head = Reading::Noun {
            form: 0,
            head: noun_word(owner, Number::Plural, SurfaceCase::Declared),
        };
        let expected = Reading::BarePlural {
            form: 0,
            head: Box::new(head),
        };
        let text = expected.realize(&LEXICON).unwrap();
        assert!(readings(&text, Category::NounPhrase).contains(&expected));
        let mut wrong_mass = noun_word(owner, Number::Singular, SurfaceCase::Declared);
        wrong_mass.countability = Some(false);
        assert!(
            Reading::Noun {
                form: 0,
                head: wrong_mass
            }
            .admit(&LEXICON)
            .is_err()
        );
        let mut crossed = noun_word(owner, Number::Plural, SurfaceCase::Declared);
        let LexicalReading::Word(value) = &mut crossed.value else { unreachable!() };
        value.features.number = Some(Number::Singular);
        assert!(
            Reading::Noun {
                form: 0,
                head: crossed
            }
            .admit(&LEXICON)
            .is_err()
        );
    }
}

fn invariant(owner: &str, casing: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: casing,
        }),
        frame: None,
        countability: None,
    }
}
fn orientation(owner: &str, casing: SurfaceCase) -> Reading {
    Reading::IntransitivePreposition {
        form: 0,
        head: invariant(owner, casing),
    }
}

#[test]
fn independent_orientation_pps_and_coordination_preserve_whole_lexical_words() {
    for (owner, spelling) in [
        ("vocab:Preposition/FaceDown", "face down"),
        ("vocab:Preposition/FaceUp", "face up"),
    ] {
        for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
            let spelling = if casing == SurfaceCase::Initial {
                format!("F{}", &spelling[1..])
            } else {
                spelling.into()
            };
            let expected = orientation(owner, casing);
            assert_eq!(expected.realize(&LEXICON).unwrap(), spelling);
            assert!(readings(&spelling, Category::PrepositionPhrase).contains(&expected));
            let mut leaves = vec![];
            expected
                .visit_words(&mut |word| leaves.push(word.clone()))
                .unwrap();
            assert_eq!(leaves, [invariant(owner, casing)]);
            assert_eq!(LEXICON.realize(&leaves[0].value).unwrap(), spelling);
        }
    }
    let expected = Reading::PrepositionPhraseCoordination {
        form: 0,
        left: Box::new(orientation(
            "vocab:Preposition/FaceUp",
            SurfaceCase::Declared,
        )),
        coordinator: invariant("vocab:Coordinator/Or", SurfaceCase::Declared),
        right: Box::new(orientation(
            "vocab:Preposition/FaceDown",
            SurfaceCase::Declared,
        )),
    };
    assert_eq!(expected.realize(&LEXICON).unwrap(), "face up or face down");
    assert!(readings("face up or face down", Category::PrepositionPhrase).contains(&expected));
    let mut leaves = vec![];
    expected
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        [
            invariant("vocab:Preposition/FaceUp", SurfaceCase::Declared),
            invariant("vocab:Coordinator/Or", SurfaceCase::Declared),
            invariant("vocab:Preposition/FaceDown", SurfaceCase::Declared)
        ]
    );
}

#[test]
fn orientation_predication_uses_the_existing_selected_locative_frame() {
    let predicate = Reading::FiniteLocative {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "core-verb:Be".into(),
                form: WordForm::Present,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    case: None,
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: Some(2),
            countability: None,
        },
        complement: Box::new(Reading::LocativeComplement {
            form: 0,
            phrase: Box::new(orientation(
                "vocab:Preposition/FaceDown",
                SurfaceCase::Declared,
            )),
        }),
    };
    assert_eq!(predicate.realize(&LEXICON).unwrap(), "is face down");
    assert!(readings("is face down", Category::FinitePredicate).contains(&predicate));
    let clause = Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::NominativePhrase {
            form: 0,
            head: Box::new(Reading::NominativePronoun {
                form: 0,
                head: Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "vocab:SubjectPronoun/It".into(),
                        form: WordForm::Invariant,
                        features: FeatureBundle {
                            number: Some(Number::Singular),
                            person: Some(Person::Third),
                            case: Some(Case::Nominative),
                            ..Default::default()
                        },
                        variant: 0,
                        capitalization: SurfaceCase::Declared,
                    }),
                    frame: None,
                    countability: None,
                },
            }),
        }),
        predicate: Box::new(predicate),
    };
    assert_eq!(clause.realize(&LEXICON).unwrap(), "it is face down");
    assert!(readings("it is face down", Category::FiniteClause).contains(&clause));
}
