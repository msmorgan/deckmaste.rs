use crate::common;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;
use std::collections::BTreeSet;

fn word(id: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}
fn place(dependent: bool) -> Reading {
    Reading::FusedDeterminerHeadNounPhrase {
        form: 0,
        head: word("vocab:CompoundDeterminative/Anywhere"),
        dependent: dependent.then(|| word("vocab:Preposition/Else")),
    }
}
fn goal(dependent: bool) -> Reading {
    Reading::LocativeComplement {
        form: 1,
        phrase: Box::new(place(dependent)),
    }
}

#[test]
fn independent_posthead_values_obey_both_roundtrip_laws() {
    for dependent in [false, true] {
        let text = if dependent { "anywhere else" } else { "anywhere" };
        for (value, category) in [
            (place(dependent), Category::NounPhrase),
            (goal(dependent), Category::LocativeComplement),
        ] {
            value.admit(common::lexicon()).unwrap();
            assert_eq!(value.realize(common::lexicon()).unwrap(), text);
            assert_eq!(
                common::readings(text, category),
                BTreeSet::from([value.clone()])
            );
            let mut words = Vec::new();
            value
                .visit_words(&mut |word| words.push(word.clone()))
                .unwrap();
            let mut expected = vec![word("vocab:CompoundDeterminative/Anywhere")];
            if dependent {
                expected.push(word("vocab:Preposition/Else"));
            }
            assert_eq!(words, expected);
        }
    }
}

#[test]
fn selected_put_has_object_then_locative_np() {
    for dependent in [false, true] {
        let text = if dependent { "putting it anywhere else" } else { "putting it anywhere" };
        let readings = common::readings(text, Category::SecondaryVerbPhrase);
        assert!(!readings.is_empty());
        let objects = common::readings("it", Category::AccusativePhrase);
        let expected: BTreeSet<_> = objects
            .into_iter()
            .map(|object| {
                vec![
                    FrameValue::Argument(Box::new(object)),
                    FrameValue::Argument(Box::new(goal(dependent))),
                ]
            })
            .collect();
        let mut observed = BTreeSet::new();
        for reading in readings {
            let mut selected = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        head, complements, ..
                    } = node
                    {
                        let LexicalReading::Word(value) = &head.value else {
                            panic!("put must be a lexical word");
                        };
                        assert_eq!(value.lexeme.as_str(), "core-verb:Put");
                        selected.push(complements.clone());
                        observed.insert(complements.clone());
                    }
                })
                .unwrap();
            assert_eq!(selected.len(), 1);
            assert!(expected.contains(&selected[0]));
            assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
        }
        assert_eq!(observed, expected);
    }
}

#[test]
fn ticket_witnesses_select_the_posthead_np_as_locative_complement() {
    for (card, text) in WITNESSES {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        for reading in readings {
            let mut goals = Vec::new();
            reading
                .visit(&mut |node| {
                    if node == &goal(true) {
                        goals.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(goals, vec![goal(true)], "{card}");
            assert_eq!(reading.realize(common::lexicon()).unwrap(), *text);
        }
    }
}

#[test]
fn optional_dependent_and_locative_use_require_declared_licenses() {
    for text in ["this else", "anywhere there"] {
        assert_eq!(common::readings(text, Category::NounPhrase), BTreeSet::new());
    }
    assert_eq!(common::readings("else", Category::PrepositionPhrase), BTreeSet::new());
    let unlicensed_head = Reading::FusedDeterminerHeadNounPhrase {
        form: 0,
        head: word("vocab:SingularDemonstrative/This"),
        dependent: Some(word("vocab:Preposition/Else")),
    };
    let unlicensed_dependent = Reading::FusedDeterminerHeadNounPhrase {
        form: 0,
        head: word("vocab:CompoundDeterminative/Anywhere"),
        dependent: Some(word("vocab:Preposition/There")),
    };
    for value in [unlicensed_head, unlicensed_dependent] {
        assert!(value.admit(common::lexicon()).is_err());
        assert!(value.realize(common::lexicon()).is_err());
    }
    let bare_this = Reading::FusedDeterminerHeadNounPhrase {
        form: 0,
        head: word("vocab:SingularDemonstrative/This"),
        dependent: None,
    };
    bare_this.admit(common::lexicon()).unwrap();
    let invalid_goal = Reading::LocativeComplement {
        form: 1,
        phrase: Box::new(bare_this),
    };
    assert!(invalid_goal.admit(common::lexicon()).is_err());
}

const WITNESSES: &[(&str, &str)] = &[
    (
        "Gruesome Encore",
        "Put target creature card from an opponent's graveyard onto the battlefield under your control. It gains haste. Exile it at the beginning of the next end step. If that creature would leave the battlefield, exile it instead of putting it anywhere else.",
    ),
    (
        "Nephalia Academy",
        "If a spell or ability an opponent controls causes you to discard a card, you may reveal that card and put it on top of your library instead of putting it anywhere else.",
    ),
    (
        "Storm Herald",
        "When this creature enters, return any number of Aura cards from your graveyard to the battlefield attached to creatures you control. Exile those Auras at the beginning of your next end step. If those Auras would leave the battlefield, exile them instead of putting them anywhere else.",
    ),
    (
        "From the Catacombs",
        "Put target creature card from a graveyard onto the battlefield under your control with a corpse counter on it. You take the initiative. If that creature would leave the battlefield, exile it instead of putting it anywhere else.",
    ),
];
