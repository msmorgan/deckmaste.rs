use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn selected(text: &str, passive: bool, direction: &str) {
    selected_in_category(text, Category::Document, passive, direction);
}

fn selected_in_category(text: &str, category: Category, passive: bool, direction: &str) {
    let values = common::readings(text, category);
    assert_eq!(values.len(), 1, "{text}");
    for value in values {
        let mut selected = 0;
        value
            .visit(&mut |node| {
                let (head, complements) = match node {
                    Reading::SelectedFramePassive { head, complements, .. } => {
                        assert!(passive);
                        (head, complements)
                    }
                    Reading::SelectedPredicate { head, complements, .. }
                        if matches!(&head.value, LexicalReading::Word(word) if word.lexeme.as_ref() == "core-verb:Turn") =>
                    {
                        assert!(!passive);
                        (head, complements)
                    }
                    _ => return,
                };
                assert!(matches!(&head.value, LexicalReading::Word(word) if word.lexeme.as_ref() == "core-verb:Turn"));
                let offset = usize::from(direction == "down");
                assert_eq!(head.frame, Some(if passive { 2 } else { 0 } + offset));
                let markers = if passive {
                    complements.as_slice()
                } else {
                    let [FrameValue::Argument(object), rest @ ..] = complements.as_slice() else {
                        panic!("missing selected Object: {node:?}");
                    };
                    assert_eq!(object.category(), Category::AccusativePhrase);
                    rest
                };
                let [FrameValue::Marker(marker)] = markers else {
                    panic!("expected one invariant frame marker: {node:?}");
                };
                assert_eq!(common::lexicon().realize(&marker.value).unwrap(), format!("face {direction}"));
                let LexicalReading::Word(marker) = &marker.value else {
                    panic!("expected lexical frame marker: {node:?}");
                };
                assert_eq!(marker.lexeme.as_ref(), if direction == "down" { "vocab:Preposition/FaceDown" } else { "vocab:Preposition/FaceUp" });
                assert_eq!(marker.form, WordForm::Invariant);
                selected += 1;
            })
            .unwrap();
        assert_eq!(selected, 1, "{text}");
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn liege_of_the_axe() {
    selected(
        "When this creature is turned face up, untap it.",
        true,
        "up",
    );
}

#[test]
fn shaleskin_plower() {
    selected(
        "When this creature is turned face up, destroy target land.",
        true,
        "up",
    );
}

#[test]
fn daru_mender() {
    selected(
        "When this creature is turned face up, regenerate target creature.",
        true,
        "up",
    );
}

#[test]
fn aven_farseer() {
    selected(
        "Whenever a permanent is turned face up, put a +1/+1 counter on this creature.",
        true,
        "up",
    );
}

#[test]
fn zimone_mystery_unraveler() {
    selected(
        "Otherwise, you may turn a permanent you control face up.",
        false,
        "up",
    );
}

#[test]
fn active_directions_keep_selected_objects() {
    // Etrata, Deadly Fugitive's granted ability; Wall of Deceit's activation.
    selected("Turn this creature face up.", false, "up");
    selected("Turn this creature face down.", false, "down");
}

#[test]
fn vesuvan_shapeshifter_passive_face_down() {
    // Its "until this creature is turned face down" constituent.
    selected_in_category(
        "is turned face down",
        Category::FinitePredicate,
        true,
        "down",
    );
}

#[test]
fn unattested_intransitive_frame_is_retired() {
    // Pyxis has a postposed Object: "turns face up all cards ...".
    // Its surface match does not license an intransitive Predicate.
    assert!(common::readings("turns face up", Category::FinitePredicate).is_empty());
}

fn word(id: &str, form: WordForm, frame: Option<usize>, countability: Option<bool>) -> Word {
    let value = common::lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_ref() == id
                && value.form == form
                && value.capitalization == SurfaceCase::Declared
        })
        .unwrap()
        .clone();
    Word {
        value: LexicalReading::Word(value),
        frame,
        countability,
    }
}

fn independent(value: &Reading, text: &str) {
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    let values = common::readings(text, Category::SecondaryVerbPhrase);
    assert_eq!(values, BTreeSet::from([value.clone()]));
    let mut constructed_words = vec![];
    value
        .visit_words(&mut |word| constructed_words.push(word.clone()))
        .unwrap();
    let mut parsed_words = vec![];
    values
        .first()
        .unwrap()
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(constructed_words, parsed_words);
}

#[test]
fn independently_constructed_active_and_passive_frames_roundtrip() {
    // Wall of Deceit's Predicate and Liege of the Axe's passive constituent.
    let creature = Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(
            "vocab:SingularDemonstrative/This",
            WordForm::Invariant,
            None,
            None,
        ),
        head: Box::new(Reading::Noun {
            form: 0,
            head: word("lexeme:type/creature", WordForm::Singular, None, Some(true)),
        }),
    };
    independent(
        &Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word("core-verb:Turn", WordForm::Plain, Some(1), None),
            complements: vec![
                FrameValue::Argument(Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(creature),
                })),
                FrameValue::Marker(word(
                    "vocab:Preposition/FaceDown",
                    WordForm::Invariant,
                    None,
                    None,
                )),
            ],
        },
        "turn this creature face down",
    );
    let passive = Reading::SelectedFramePassive {
        form: 0,
        head: word("core-verb:Turn", WordForm::PastParticiple, Some(2), None),
        complements: vec![FrameValue::Marker(word(
            "vocab:Preposition/FaceUp",
            WordForm::Invariant,
            None,
            None,
        ))],
    };
    independent(&passive, "turned face up");
    let Reading::SelectedFramePassive { complements, .. } = passive else {
        unreachable!()
    };
    assert!(
        Reading::SelectedFramePassive {
            form: 0,
            head: word("core-verb:Turn", WordForm::Plain, Some(2), None),
            complements,
        }
        .admit(common::lexicon())
        .is_err()
    );
}

#[test]
fn invariant_frame_markers_reject_plural_faces() {
    for text in [
        "turn that creature faces up",
        "turned faces up",
        "turn this creature faces down",
        "turned faces down",
    ] {
        assert!(
            common::readings(text, Category::SecondaryVerbPhrase).is_empty(),
            "{text}"
        );
    }
}
