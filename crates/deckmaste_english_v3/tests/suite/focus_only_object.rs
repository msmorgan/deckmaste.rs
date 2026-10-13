use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, FrameItem, LexicalReading, LexicalValue, Number, Relation, SurfaceCase, WordForm,
};

use crate::common::{lexicon, readings};

fn focused_object(text: &str, verb: &str) {
    let control = text.replace("only ", "");
    let mut expected = BTreeSet::new();
    for value in readings(&control, Category::Sentence) {
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_ref() == verb
                {
                    let frame =
                        &lexicon().lexemes()[&word.lexeme].properties.frames[head.frame.unwrap()];
                    for (slot, argument) in frame.items.iter().zip(complements) {
                        if let (FrameItem::Argument(slot), FrameValue::Argument(argument)) =
                            (slot, argument)
                            && slot.relation == Relation::Object
                        {
                            let Reading::CasePhrase { head: np, .. } = argument.as_ref() else {
                                panic!("control Object must carry accusative case");
                            };
                            expected.insert(np.as_ref().clone());
                        }
                    }
                }
            })
            .unwrap();
    }
    assert!(!expected.is_empty(), "missing unfocused control Object");
    let values = readings(text, Category::Sentence);
    assert!(!values.is_empty(), "{text}");
    let mut observed = BTreeSet::new();
    for value in values {
        let mut found = false;
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_ref() == verb
                {
                    let frame =
                        &lexicon().lexemes()[&word.lexeme].properties.frames[head.frame.unwrap()];
                    for (slot, argument) in frame.items.iter().zip(complements) {
                        if let (FrameItem::Argument(slot), FrameValue::Argument(argument)) =
                            (slot, argument)
                            && slot.relation == Relation::Object
                        {
                            let Reading::CasePhrase { head: np, .. } = argument.as_ref() else {
                                panic!("Object must carry accusative case");
                            };
                            let Reading::FocusedPhrase {
                                category,
                                modifier,
                                head: np_head,
                                ..
                            } = np.as_ref()
                            else {
                                continue;
                            };
                            assert_eq!(*category, Category::NounPhrase);
                            assert_eq!(np_head.category(), Category::NounPhrase);
                            let LexicalReading::Word(focus) = &modifier.value else {
                                panic!("Focus Adverb")
                            };
                            assert_eq!(focus.lexeme.as_ref(), "vocab:FocusAdverb/Only");
                            observed.insert(np_head.as_ref().clone());
                            found = true;
                        }
                    }
                }
            })
            .unwrap();
        assert!(found, "every Reading must focus the verb's Object");
    }
    assert_eq!(
        observed, expected,
        "NP focus must preserve the control attachments: {text}"
    );
}

#[test]
fn attested_objects_retain_their_np_heads() {
    focused_object(
        "This creature can block only creatures with flying.",
        "core-verb:Block",
    );
    focused_object(
        "When this creature enters, change the target of target spell that targets only a single creature to this creature.",
        "lexeme:Verb/Target",
    );
    focused_object(
        "Whenever you cast an instant or sorcery spell that targets only this creature, you may pay {2}.",
        "lexeme:Verb/Target",
    );
    focused_object(
        "This effect reduces only the amount of colored mana you pay.",
        "lexeme:Verb/Reduce",
    );
    focused_object(
        "If target spell has only one target and that target is a creature, change that spell's target to another creature.",
        "core-verb:Have",
    );
}

#[test]
fn subject_focus_uses_the_same_np_instance() {
    let values = readings(
        "Only your opponents may activate this ability.",
        Category::Sentence,
    );
    assert!(!values.is_empty());
    for value in values {
        let mut found = false;
        value
            .visit(&mut |node| {
                if let Reading::FiniteClause { subject, .. } = node
                    && let Reading::CasePhrase { head, .. } = subject.as_ref()
                    && let Reading::FocusedPhrase { category, head, .. } = head.as_ref()
                {
                    assert_eq!(*category, Category::NounPhrase);
                    assert_eq!(head.realize(lexicon()).unwrap(), "your opponents");
                    found = true;
                }
            })
            .unwrap();
        assert!(found);
    }
}

#[test]
fn existing_adjunct_focus_does_not_become_np_focus() {
    for text in [
        "Activate only during your turn.",
        "Activate only as a sorcery.",
        "This creature can block only during your turn.",
        "This creature can attack only alone.",
        "Activate only once each turn.",
    ] {
        let values = readings(text, Category::Sentence);
        assert!(!values.is_empty());
        for value in values {
            value
                .visit(&mut |node| {
                    if let Reading::FocusedPhrase { category, .. } = node {
                        assert_ne!(*category, Category::NounPhrase);
                    }
                })
                .unwrap();
        }
    }
    assert!(readings("only only creatures", Category::NounPhrase).is_empty());
}

#[test]
fn independently_constructed_focused_np_roundtrips() {
    // The NP head is attested in Welkin Tern's Object.
    let expected = Reading::FocusedPhrase {
        category: Category::NounPhrase,
        form: 0,
        modifier: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:FocusAdverb/Only".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        head: Box::new(Reading::BareNounPhrase {
            form: 0,
            head: Box::new(Reading::NominalProjection {
                form: 0,
                head: Box::new(Reading::NominalCoreProjection {
                    form: 0,
                    head: Box::new(Reading::Noun {
                        form: 0,
                        head: Word {
                            value: LexicalReading::Word(LexicalValue {
                                lexeme: "lexeme:type/creature".into(),
                                form: WordForm::Plural,
                                features: FeatureBundle {
                                    number: Some(Number::Plural),
                                    ..FeatureBundle::default()
                                },
                                variant: 0,
                                capitalization: SurfaceCase::Declared,
                            }),
                            frame: None,
                            countability: Some(true),
                        },
                    }),
                }),
            }),
        }),
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "only creatures");
    assert_eq!(
        readings("only creatures", Category::NounPhrase),
        BTreeSet::from([expected.clone()])
    );
    let mut double = expected.clone();
    let Reading::FocusedPhrase { head, .. } = &mut double else { unreachable!() };
    **head = expected;
    assert!(double.admit(lexicon()).is_err());
    assert!(double.realize(lexicon()).is_err());
}
