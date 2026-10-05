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

fn exact(text: &str, category: Category, expected: BTreeSet<Reading>) {
    for value in &expected {
        assert_eq!(value.realize(&LEXICON).unwrap(), text);
        value.admit(&LEXICON).unwrap();
    }
    assert_eq!(readings(text, category), expected);
}

fn noun(owner: &str) -> Reading {
    let mut head = word(
        owner,
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..Default::default()
        },
    );
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

#[test]
fn authentic_you_and_draw_clause_have_only_singular_second_person_values() {
    // Greta, Sweettooth Scourge: "You draw a card and you lose 1 life."
    // Tinybones, Trinket Thief: "... you draw a card and you lose 1 life."
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
    let card = Reading::AccusativePhrase {
        form: 0,
        head: Box::new(Reading::IndefiniteNounPhrase {
            form: 0,
            determiner: word(
                "vocab:Article/Indefinite",
                WordForm::Invariant,
                FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
            ),
            head: Box::new(noun("lexeme:CommonNoun/Card")),
        }),
    };
    exact(
        "you draw a card",
        Category::FiniteClause,
        BTreeSet::from([clause(
            you(),
            Reading::FiniteTransitive {
                form: 0,
                head: verb("core-verb:Draw", 0, Number::Singular),
                object: Box::new(card),
            },
        )]),
    );
    assert!(readings("you draws a card", Category::FiniteClause).is_empty());
}

#[test]
fn authentic_coordinated_subject_retains_singular_you_leaf() {
    // Bloodroot Apothecary: "When this creature enters, you and target opponent
    // each create a Treasure token." Test the attested subject constituent.
    let target = Reading::TargetNounPhrase {
        form: 0,
        marker: word(
            "vocab:TargetingMarker/Target",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
        head: Box::new(noun("lexeme:CommonNoun/Opponent")),
    };
    let subject = Reading::AdditiveNounPhrase {
        form: 0,
        left: Box::new(you()),
        coordinator: word(
            "vocab:Coordinator/And",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
        right: Box::new(target),
    };
    exact(
        "you and target opponent",
        Category::NominativePhrase,
        BTreeSet::from([Reading::NominativePhrase {
            form: 0,
            head: Box::new(subject),
        }]),
    );
}

#[test]
fn authentic_discard_clause_has_no_plural_second_person_leaves() {
    // Apocalypse: "Exile all permanents. You discard your hand."
    let object = Reading::AccusativePhrase {
        form: 0,
        head: Box::new(Reading::PossessiveNounPhrase {
            form: 0,
            possessor: pronoun("vocab:PossessiveDeterminerPronoun/Your", Case::Genitive),
            head: Box::new(noun("lexeme:CommonNoun/Hand")),
        }),
    };
    let expected = clause(
        you(),
        Reading::FiniteTransitive {
            form: 0,
            head: verb("lexeme:keyword_action/discard", 0, Number::Singular),
            object: Box::new(object),
        },
    );
    exact(
        "you discard your hand",
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
