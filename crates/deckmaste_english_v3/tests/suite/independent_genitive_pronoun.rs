use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm,
};

use crate::common;

fn independent(member: &str, person: Person, number: Number) -> Reading {
    Reading::FusedDeterminerHeadNounPhrase {
        form: 1,
        dependent: None,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: deckmaste_lexical::LexemeId::new(&format!("vocab:SubjectPronoun/{member}")),
                form: WordForm::IndependentGenitive,
                features: FeatureBundle {
                    case: Some(Case::Genitive),
                    person: Some(person),
                    number: Some(number),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
    }
}

#[test]
fn existing_personal_pronouns_have_one_independent_genitive_head() {
    // The paradigm is the owner ruling; yours is attested on Mesa Lynx.
    // Other cells are independently constructed, not invented Oracle effects.
    for (member, text, person, number) in [
        ("I", "mine", Person::First, Number::Singular),
        ("You", "yours", Person::Second, Number::Singular),
        ("He", "his", Person::Third, Number::Singular),
        ("She", "hers", Person::Third, Number::Singular),
        ("They", "theirs", Person::Third, Number::Plural),
    ] {
        let expected = independent(member, person, number);
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::from([expected])
        );
    }
    let absent = independent("It", Person::Third, Number::Singular);
    assert!(absent.admit(common::lexicon()).is_err());
    assert!(common::readings("its", Category::NounPhrase).is_empty());
    assert!(
        !common::lexicon()
            .lexemes()
            .contains_key("vocab:SubjectPronoun/We")
    );
}

#[test]
fn independent_genitive_is_the_complement_of_than() {
    let yours = independent("You", Person::Second, Number::Singular);
    let values = common::readings("turns other than yours", Category::NounPhrase);
    assert!(!values.is_empty());
    for value in values {
        let mut complements = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ComparativeAdjectivePhrase {
                    governor,
                    complement,
                    ..
                } = node
                    && let Reading::ComparativeGovernorHead { marker, .. } = &**governor
                    && common::lexicon().realize(&marker.value).unwrap() == "than"
                {
                    let Reading::NominalComparativeComplement { value, .. } = &**complement else {
                        panic!("than must have a nominal Complement");
                    };
                    complements.push((**value).clone());
                }
            })
            .unwrap();
        assert_eq!(
            complements,
            vec![Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(yours.clone()),
            }]
        );
    }
}

#[test]
fn witnesses_preserve_their_controls_independent_ambiguity() {
    for (card, text, control, category) in [
        (
            "Mesa Lynx",
            "During turns other than yours, this creature gets +0/+2.",
            "During your turn, this creature gets +0/+2.",
            Category::Document,
        ),
        (
            "Geyser Drake",
            "During turns other than yours, spells you cast cost {1} less to cast.",
            "During your turn, spells you cast cost {1} less to cast.",
            Category::Document,
        ),
        (
            "Midnight Mangler",
            "During turns other than yours, this Vehicle is an artifact creature.",
            "During your turn, this Vehicle is an artifact creature.",
            Category::Document,
        ),
        (
            "Glory of Warfare",
            "During turns other than yours, creatures you control get +0/+2.",
            "During turns other than your turn, creatures you control get +0/+2.",
            Category::Sentence,
        ),
    ] {
        let values = common::readings_on_card(card, text, category);
        let controls = common::readings_on_card(card, control, category);
        assert!(!controls.is_empty(), "{card}");
        assert_eq!(values.len(), controls.len(), "{card}");
        for value in values {
            let mut heads = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::FusedDeterminerHeadNounPhrase { form: 1, .. } = node {
                        heads.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                heads,
                vec![independent("You", Person::Second, Number::Singular)]
            );
        }
    }
}

#[test]
fn dependent_forms_keep_their_nominal_and_do_not_become_independent_heads() {
    for text in ["your", "their"] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
    let object_her = Reading::AccusativePronoun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:ObjectPronoun/Her".into(),
                form: WordForm::Invariant,
                features: FeatureBundle {
                    case: Some(Case::Accusative),
                    person: Some(Person::Third),
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
    };
    assert_eq!(
        common::readings("her", Category::NounPhrase),
        BTreeSet::from([object_her])
    );
    for text in [
        "your devotion",
        "his controller",
        "its power",
        "their turn",
        "turns other than your turn",
        "creatures other than this creature",
    ] {
        let values = common::readings(text, Category::NounPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut independent_heads = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::FusedDeterminerHeadNounPhrase { form: 1, .. } = node {
                        independent_heads.push(node.clone());
                    }
                })
                .unwrap();
            assert!(independent_heads.is_empty(), "{text}");
        }
    }
    assert!(common::readings("yours", Category::AdjectivePhrase).is_empty());
}

#[test]
fn independent_head_reuses_existing_number_neutral_agreement() {
    // Construct agreement sites from the attested Mesa/Glory predicates.
    // These are admission checks, not invented Oracle sentence witnesses.
    for text in ["gets +0/+2", "get +0/+2"] {
        let predicates = common::readings(text, Category::FinitePredicate);
        assert!(!predicates.is_empty());
        let mut third_person_sites = Vec::new();
        for predicate in predicates {
            let mut persons = Vec::new();
            predicate
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(value) = &word.value
                        && let Some(person) = value.features.person
                    {
                        persons.push(person);
                    }
                })
                .unwrap();
            let expected_agreement = persons == vec![Person::Third];
            let clause = Reading::FiniteClause {
                form: 0,
                subject: Box::new(Reading::CasePhrase {
                    category: Category::NominativePhrase,
                    form: 0,
                    head: Box::new(independent("You", Person::Second, Number::Singular)),
                }),
                predicate: Box::new(predicate),
            };
            assert_eq!(clause.admit(common::lexicon()).is_ok(), expected_agreement);
            if expected_agreement {
                third_person_sites.push(clause);
            }
        }
        assert!(!third_person_sites.is_empty(), "{text}");
    }
}
