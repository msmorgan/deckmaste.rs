use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn witness(text: &str, pp: &str, clause: &str) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {text:?}");
    eprintln!("{text:?}: {} Readings", values.len());
    for value in &values {
        value.admit(common::lexicon()).unwrap();
        let mut heads = Vec::new();
        value
            .visit(&mut |node| match node {
                Reading::ClauseComplementPreposition {
                    head, complement, ..
                }
                | Reading::IrrealisClauseComplementPreposition {
                    head, complement, ..
                } if common::lexicon().realize(&head.value).unwrap() == "as though" => {
                    assert_eq!(complement.realize(common::lexicon()).unwrap(), clause);
                    assert_eq!(
                        head,
                        &word(
                            "vocab:Preposition/AsThough",
                            WordForm::Invariant,
                            FeatureBundle::default(),
                            None
                        )
                    );
                    heads.push(head.clone());
                }
                _ => {}
            })
            .unwrap();
        assert_eq!(heads.len(), 1, "compound head in {value:?}");
    }
    common::assert_constituents(
        text,
        Category::Document,
        &[
            (Category::PrepositionPhrase, pp),
            (
                if clause.contains("weren't") {
                    Category::IrrealisClause
                } else {
                    Category::FiniteClause
                },
                clause,
            ),
        ],
    );
}

#[test]
fn vedalken_orrery() {
    witness(
        "You may cast spells as though they had flash.",
        "as though they had flash",
        "they had flash",
    );
}

#[test]
fn high_fae_trickster() {
    witness(
        "Flash\nFlying\nYou may cast spells as though they had flash.",
        "as though they had flash",
        "they had flash",
    );
}

#[test]
fn blur_of_heroism() {
    witness(
        "You may cast Hero spells as though they had flash.",
        "as though they had flash",
        "they had flash",
    );
}

#[test]
fn mirror_wall() {
    witness(
        "{W}: This creature can attack this turn as though it didn't have defender.",
        "as though it didn't have defender",
        "it didn't have defender",
    );
}

#[test]
fn ruxa_patient_professor() {
    witness(
        "For each creature you control with no abilities, you may have that creature assign its combat damage as though it weren't blocked.",
        "as though it weren't blocked",
        "it weren't blocked",
    );
}

#[test]
fn quicken_modal_passive() {
    witness(
        "The next sorcery spell you cast this turn can be cast as though it had flash.\nDraw a card.",
        "as though it had flash",
        "it had flash",
    );
}

fn word(id: &str, form: WordForm, features: FeatureBundle, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

fn independent_irrealis() -> Reading {
    Reading::IrrealisClauseComplementPreposition {
        form: 0,
        head: word(
            "vocab:Preposition/AsThough",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        ),
        complement: Box::new(Reading::IrrealisClause {
            form: 0,
            subject: Box::new(Reading::CasePhrase {
                category: Category::NominativePhrase,
                form: 0,
                head: Box::new(Reading::NominativePronoun {
                    form: 0,
                    head: word(
                        "vocab:SubjectPronoun/It",
                        WordForm::Invariant,
                        FeatureBundle {
                            number: Some(Number::Singular),
                            person: Some(Person::Third),
                            case: Some(Case::Nominative),
                            ..Default::default()
                        },
                        None,
                    ),
                }),
            }),
            predicate: Box::new(Reading::SelectedPredicate {
                category: Category::IrrealisPredicate,
                form: 0,
                head: word(
                    "core-verb:BeNegative",
                    WordForm::Irrealis,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(Person::Third),
                        finiteness: Some(Finiteness::Finite),
                        ..Default::default()
                    },
                    Some(0),
                ),
                complements: vec![FrameValue::Argument(Box::new(
                    Reading::AdjectivalComplement {
                        form: 0,
                        phrase: Box::new(Reading::IntransitiveAdjective {
                            form: 0,
                            head: word(
                                "core-verb:Block/adjective",
                                WordForm::Invariant,
                                FeatureBundle::default(),
                                Some(0),
                            ),
                        }),
                    },
                ))],
            }),
        }),
    }
}

#[test]
fn independently_constructed_irrealis_preserves_value_and_leaf_order() {
    // Pride of Lions: the finite content-clause PP.
    let expected = independent_irrealis();
    expected.admit(common::lexicon()).unwrap();
    let text = "as though it weren't blocked";
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    let parsed = common::readings(text, Category::PrepositionPhrase);
    assert!(parsed.contains(&expected));
    let mut expected_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    let mut actual_nodes = Vec::new();
    parsed
        .get(&expected)
        .unwrap()
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
    let mut words = Vec::new();
    expected
        .visit_words(&mut |word| words.push(common::lexicon().realize(&word.value).unwrap()))
        .unwrap();
    assert_eq!(words, ["as though", "it", "weren't", "blocked"]);
}

#[test]
fn declared_irrealis_permission_controls_admission_and_parsing() {
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "vocab:Preposition/AsThough")
        .unwrap()
        .properties
        .features
        .insert("IrrealisClauseComplement".into(), "No".into());
    let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert!(independent_irrealis().admit(&lexicon).is_err());
    assert!(
        common::readings_with_lexicon(
            &lexicon,
            "as though it weren't blocked",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
}

#[test]
fn adjunct_licence_excludes_nominal_predicative_and_preposed_functions() {
    let pp = independent_irrealis();
    let mut noun_head = word(
        "lexeme:type/creature",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..Default::default()
        },
        None,
    );
    noun_head.countability = Some(true);
    let noun = Reading::Noun {
        form: 0,
        head: noun_head,
    };
    noun.admit(common::lexicon()).unwrap();
    let nominal = Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(noun),
        modifier: Box::new(pp.clone()),
    };
    assert!(nominal.admit(common::lexicon()).is_err());
    assert!(
        common::readings(
            "Target creature as though it weren't blocked",
            Category::NounPhrase
        )
        .is_empty()
    );
    let predicative = Reading::PredicativePreposition {
        form: 0,
        phrase: Box::new(pp.clone()),
    };
    assert!(predicative.admit(common::lexicon()).is_err());
    let clause = common::readings("You may cast spells", Category::Clause)
        .into_iter()
        .next()
        .unwrap();
    let preposed = Reading::InitialPreposition {
        form: 0,
        dependent: Box::new(pp),
        clause: Box::new(clause),
    };
    assert!(preposed.admit(common::lexicon()).is_err());
}

#[test]
fn irrealis_is_non_tensed_and_does_not_relax_ordinary_agreement() {
    // The singular PP occurs on Pride of Lions; the plural on Outmaneuver.
    assert!(
        !common::readings("as though it weren't blocked", Category::PrepositionPhrase).is_empty()
    );
    assert!(common::readings("It weren't blocked.", Category::Document).is_empty());
    assert!(common::readings("it weren't blocked", Category::FiniteClause).is_empty());
    assert!(!common::readings("they weren't blocked", Category::FiniteClause).is_empty());
    assert!(common::readings("they weren't blocked", Category::IrrealisClause).is_empty());
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let verb = lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "core-verb:BeNegative")
        .unwrap();
    let slot = verb
        .forms
        .iter_mut()
        .find(|slot| slot.form == WordForm::Irrealis)
        .unwrap();
    assert_eq!(slot.features.tense, None);
    slot.features.tense = Some(deckmaste_lexical::Tense::Past);
    assert!(deckmaste_lexical::Lexicon::new(lexemes).is_err());
}
