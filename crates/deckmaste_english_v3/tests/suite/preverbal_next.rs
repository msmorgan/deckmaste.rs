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
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

use crate::common;

fn witness(card: &str, text: &str, verb: &str) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut conditions = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ClauseComplementPreposition {
                    head, complement, ..
                } = node
                    && common::lexicon().realize(&head.value).unwrap() == "When"
                {
                    let Reading::FiniteClause {
                        subject, predicate, ..
                    } = complement.as_ref()
                    else {
                        panic!("the trigger condition must be a finite Clause");
                    };
                    assert_eq!(subject.realize(common::lexicon()).unwrap(), "you");
                    let mut adjuncts = Vec::new();
                    predicate
                        .visit(&mut |node| {
                            if let Reading::CentralAdverbPredicate { head, .. } = node {
                                let Reading::CentralAdverbHead {
                                    category,
                                    form,
                                    head,
                                    modifier,
                                } = head.as_ref()
                                else {
                                    panic!(
                                        "the central Adjunct must modify the selected verb head"
                                    );
                                };
                                let LexicalReading::Word(modifier) = &modifier.value else {
                                    panic!("the Adjunct must retain its Adverb Lexeme");
                                };
                                let LexicalReading::Word(head) = &head.value else {
                                    panic!("the head must retain its Verb Lexeme");
                                };
                                adjuncts.push((
                                    *category,
                                    *form,
                                    modifier.lexeme.as_str().to_owned(),
                                    head.lexeme.as_str().to_owned(),
                                    head.features.finiteness,
                                ));
                            }
                        })
                        .unwrap();
                    conditions.push(adjuncts);
                }
            })
            .unwrap();
        assert_eq!(
            conditions,
            vec![vec![(
                Category::CentralAdverbSelectedHead,
                0,
                "vocab:Adverb/Next".to_owned(),
                verb.to_owned(),
                Some(Finiteness::Finite),
            )]],
            "{card}"
        );
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn galvanic_iteration() {
    witness(
        "Galvanic Iteration",
        "When you next cast an instant or sorcery spell this turn, copy that spell.",
        "lexeme:keyword_action/cast",
    );
}

#[test]
fn the_clone_saga() {
    witness(
        "The Clone Saga",
        "When you next cast a creature spell this turn, copy it, except the copy isn't legendary.",
        "lexeme:keyword_action/cast",
    );
}

#[test]
fn all_out_assault() {
    witness(
        "All-Out Assault",
        "When you next attack this turn, untap each creature you control.",
        "core-verb:Attack",
    );
}

#[test]
fn dual_strike() {
    witness(
        "Dual Strike",
        "When you next cast an instant or sorcery spell with mana value 4 or less this turn, copy that spell. You may choose new targets for the copy.",
        "lexeme:keyword_action/cast",
    );
}

#[test]
fn attributive_next_and_existing_central_adjuncts_keep_their_readings() {
    let lexicon = deckmaste_lexical::Lexicon::new(
        common::lexicon()
            .lexemes()
            .values()
            .filter(|lexeme| lexeme.id.as_str() != "vocab:Adverb/Next")
            .cloned(),
    )
    .unwrap();
    for (text, category) in [
        (
            "At the beginning of your next upkeep, draw a card.",
            Category::Sentence,
        ),
        ("the next spell you cast this turn", Category::NounPhrase),
        ("the next end step", Category::NounPhrase),
        (
            "The next time you cast a spell this turn, copy it.",
            Category::Sentence,
        ),
        ("You also draw a card.", Category::Sentence),
        ("It still has flying.", Category::Sentence),
    ] {
        let before = common::readings_with_lexicon(&lexicon, text, category);
        assert!(!before.is_empty(), "{text}");
        assert_eq!(common::readings(text, category), before, "{text}");
    }
}

#[test]
fn next_has_only_the_preverbal_licence() {
    for text in [
        "Next you cast a spell.",
        "You cast a spell next.",
        "You may next cast a spell.",
        "It is next a land.",
        "Next cast a spell.",
    ] {
        assert!(
            common::readings(text, Category::Sentence).is_empty(),
            "{text}"
        );
    }
}

fn word(owner: &str, form: WordForm, features: FeatureBundle, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

#[test]
fn independently_assembled_preverbal_adjunct_roundtrips() {
    // The trigger-condition constituent attested on Ride the Avalanche.
    let object = common::readings("a spell", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    let value = Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            form: 0,
            category: Category::NominativePhrase,
            head: Box::new(Reading::NominativePronoun {
                form: 0,
                category: Category::NounPhrase,
                head: word(
                    "vocab:SubjectPronoun/You",
                    WordForm::Invariant,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(Person::Second),
                        case: Some(Case::Nominative),
                        ..Default::default()
                    },
                    None,
                ),
            }),
        }),
        predicate: Box::new(Reading::CentralAdverbPredicate {
            form: 0,
            head: Box::new(Reading::CentralAdverbHead {
                form: 0,
                category: Category::CentralAdverbSelectedHead,
                head: word(
                    "lexeme:keyword_action/cast",
                    WordForm::Present,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(Person::Second),
                        tense: Some(Tense::Present),
                        finiteness: Some(Finiteness::Finite),
                        ..Default::default()
                    },
                    Some(0),
                ),
                modifier: word(
                    "vocab:Adverb/Next",
                    WordForm::Invariant,
                    FeatureBundle::default(),
                    None,
                ),
            }),
            complements: vec![FrameValue::Argument(Box::new(object))],
        }),
    };
    value.admit(common::lexicon()).unwrap();
    let text = value.realize(common::lexicon()).unwrap();
    assert_eq!(text, "you next cast a spell");
    assert!(common::readings(&text, Category::FiniteClause).contains(&value));
}
