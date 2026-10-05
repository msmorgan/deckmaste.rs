use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::{Category, Grammar, Reading, Word};
use deckmaste_english_v3::parse;
use deckmaste_lexical::{
    Case, FeatureBundle, Finiteness, LexicalReading, LexicalValue, Lexicon, Number, Person,
    SurfaceCase, Tense, WordForm,
};

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
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|r| {
            let r = r.unwrap();
            assert_eq!(r.realize(&LEXICON).unwrap(), text);
            r
        })
        .collect()
}

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn pronoun(owner: &str, case: Case) -> Word {
    word(
        owner,
        WordForm::Invariant,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            case: Some(case),
            ..Default::default()
        },
    )
}

fn you() -> Reading {
    Reading::NominativePronoun {
        form: 0,
        head: pronoun("vocab:SubjectPronoun/You", Case::Nominative),
    }
}

fn verb(owner: &str, frame: usize, number: Number) -> Word {
    let mut head = word(
        owner,
        WordForm::Present,
        FeatureBundle {
            number: Some(number),
            person: Some(Person::Second),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..Default::default()
        },
    );
    head.frame = Some(frame);
    head
}

fn clause(subject: Reading, predicate: Reading) -> Reading {
    Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::NominativePhrase {
            form: 0,
            head: Box::new(subject),
        }),
        predicate: Box::new(predicate),
    }
}

fn orientation(number: Number) -> Reading {
    Reading::FiniteLocative {
        form: 0,
        head: verb("core-verb:Be", 2, number),
        complement: Box::new(Reading::LocativeComplement {
            form: 0,
            phrase: Box::new(Reading::IntransitivePreposition {
                form: 0,
                head: word(
                    "vocab:Preposition/FaceDown",
                    WordForm::Invariant,
                    FeatureBundle::default(),
                ),
            }),
        }),
    }
}

// The existing grammar admits auxiliary ellipsis without discourse context.
// Its three PP-modified alternatives retain the same subject concord as
// the independently selected locative predication.
fn orientation_clauses(subject: &Reading, number: Number) -> BTreeSet<Reading> {
    let ellipses = [
        Reading::ProgressiveEllipsis {
            form: 0,
            omission: Box::new(Reading::OmittedGerundParticipleActive { form: 0 }),
        },
        Reading::ProgressiveEllipsis {
            form: 0,
            omission: Box::new(Reading::OmittedGerundParticiplePassive { form: 0 }),
        },
        Reading::PassiveEllipsis {
            form: 0,
            omission: Box::new(Reading::OmittedPastParticiplePassive { form: 0 }),
        },
    ];
    let mut expected = BTreeSet::from([clause(subject.clone(), orientation(number))]);
    for complement in ellipses {
        expected.insert(clause(
            subject.clone(),
            Reading::FinitePreposition {
                form: 0,
                head: Box::new(Reading::FiniteParticipialAuxiliary {
                    form: 0,
                    head: verb("core-verb:Be", 1, number),
                    complement: Box::new(complement),
                }),
                modifier: Box::new(Reading::IntransitivePreposition {
                    form: 0,
                    head: word(
                        "vocab:Preposition/FaceDown",
                        WordForm::Invariant,
                        FeatureBundle::default(),
                    ),
                }),
            },
        ));
    }
    expected
}

fn exact(text: &str, category: Category, expected: BTreeSet<Reading>) {
    for value in &expected {
        assert_eq!(value.realize(&LEXICON).unwrap(), text);
        value.admit(&LEXICON).unwrap();
    }
    assert_eq!(readings(text, category), expected);
}

#[test]
fn independent_you_values_and_present_predicates_use_singular_second_person() {
    exact(
        "you",
        Category::NounPhrase,
        BTreeSet::from([
            you(),
            Reading::AccusativePronoun {
                form: 0,
                head: pronoun("vocab:ObjectPronoun/You", Case::Accusative),
            },
        ]),
    );
    exact(
        "you attack",
        Category::FiniteClause,
        BTreeSet::from([clause(
            you(),
            Reading::FiniteIntransitive {
                form: 0,
                head: verb("core-verb:Attack", 0, Number::Singular),
            },
        )]),
    );
    exact(
        "you are face down",
        Category::FiniteClause,
        orientation_clauses(&you(), Number::Singular),
    );
    for text in ["you attacks", "you is face down"] {
        assert!(readings(text, Category::FiniteClause).is_empty(), "{text}");
    }
}

#[test]
fn coordinated_singular_you_subjects_still_require_plural_concord() {
    let subject = Reading::AdditiveNounPhrase {
        form: 0,
        left: Box::new(you()),
        coordinator: word(
            "vocab:Coordinator/And",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
        right: Box::new(you()),
    };
    exact(
        "you and you are face down",
        Category::FiniteClause,
        orientation_clauses(&subject, Number::Plural),
    );
    assert!(readings("you and you is face down", Category::FiniteClause).is_empty());
}

#[test]
fn repeated_second_person_occurrences_do_not_create_plural_lexical_leaves() {
    let mut hand = word(
        "lexeme:CommonNoun/Hand",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..Default::default()
        },
    );
    hand.countability = Some(true);
    let object = Reading::AccusativePhrase {
        form: 0,
        head: Box::new(Reading::PossessiveNounPhrase {
            form: 0,
            possessor: pronoun("vocab:PossessiveDeterminerPronoun/Your", Case::Genitive),
            head: Box::new(Reading::Noun {
                form: 0,
                head: hand,
            }),
        }),
    };
    let expected = clause(
        you(),
        Reading::FiniteTransitive {
            form: 0,
            head: verb("core-verb:Draw", 0, Number::Singular),
            object: Box::new(object),
        },
    );
    exact(
        "you draw your hand",
        Category::FiniteClause,
        BTreeSet::from([expected.clone()]),
    );
    let mut second_person = vec![];
    expected
        .visit_words(&mut |word| {
            if let LexicalReading::Word(value) = &word.value {
                if value.features.person == Some(Person::Second) {
                    second_person.push(value.features.number);
                }
            }
        })
        .unwrap();
    assert_eq!(second_person, vec![Some(Number::Singular); 3]);
}
