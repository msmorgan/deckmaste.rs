use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{FrameItem, FrameSlot, LexicalReading, Lexicon, Relation, WordForm};

use crate::common;

fn owner(word: &Word) -> &str {
    let LexicalReading::Word(value) = &word.value else {
        panic!("expected a lexical Word Form");
    };
    value.lexeme.as_str()
}

fn assert_catenative(text: &str, object: &str, infinitival: &str) {
    let readings = common::readings(text, Category::Sentence);
    assert!(!readings.is_empty(), "{text}");
    for reading in readings {
        let mut selected = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && owner(head) == "core-verb:Cause"
                {
                    let [FrameValue::Argument(np), FrameValue::Argument(clause)] =
                        complements.as_slice()
                    else {
                        panic!("Cause must select an Object and Catenative Complement");
                    };
                    assert_eq!(np.category(), Category::AccusativePhrase);
                    assert_eq!(np.realize(common::lexicon()).unwrap(), object);
                    assert_eq!(clause.category(), Category::InfinitiveComplement);
                    assert_eq!(clause.realize(common::lexicon()).unwrap(), infinitival);
                    let frame = &common::lexicon().lexemes()["core-verb:Cause"]
                        .properties
                        .frames[head.frame.unwrap()];
                    assert_eq!(
                        frame.items,
                        vec![
                            FrameItem::Argument(FrameSlot {
                                relation: Relation::Object,
                                category: "NounPhrase".into(),
                            }),
                            FrameItem::Argument(FrameSlot {
                                relation: Relation::Complement,
                                category: "InfinitiveComplement".into(),
                            }),
                        ]
                    );
                    selected.push(node.clone());
                }
                if let Reading::ToInfinitive { category, .. } = node {
                    assert_eq!(*category, Category::InfinitiveComplement);
                }
            })
            .unwrap();
        assert_eq!(selected.len(), 1, "one Cause route per Reading: {text}");
    }
}

#[test]
fn witnesses_select_the_object_and_to_infinitival_in_every_reading() {
    for (text, object, infinitival) in [
        (
            "Creatures entering don't cause abilities to trigger.",
            "abilities",
            "to trigger",
        ),
        (
            "Spells and abilities your opponents control can't cause you to sacrifice permanents.",
            "you",
            "to sacrifice permanents",
        ),
        (
            "When a spell or ability an opponent controls causes you to discard this card, return it to your hand.",
            "you",
            "to discard this card",
        ),
        (
            "A player losing unspent mana causes that player to lose that much life.",
            "that player",
            "to lose that much life",
        ),
        (
            "If a creature dying causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.",
            "a triggered ability of a permanent you control",
            "to trigger",
        ),
    ] {
        assert_catenative(text, object, infinitival);
    }
}

#[test]
fn independent_nominal_ambiguities_are_preserved_under_w9_a() {
    for (text, nominal) in [
        (
            "Spells and abilities your opponents control can't cause you to sacrifice permanents.",
            "Spells and abilities your opponents control",
        ),
        (
            "When a spell or ability an opponent controls causes you to discard this card, return it to your hand.",
            "a spell or ability an opponent controls",
        ),
        (
            "If a creature dying causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.",
            "a triggered ability of a permanent you control",
        ),
    ] {
        let mut embedded = BTreeSet::new();
        for reading in common::readings(text, Category::Sentence) {
            reading
                .visit(&mut |node| {
                    if node.category() == Category::NounPhrase
                        && node.realize(common::lexicon()).unwrap() == nominal
                    {
                        embedded.insert(node.clone());
                    }
                })
                .unwrap();
        }
        assert_eq!(embedded, common::readings(nominal, Category::NounPhrase));
    }
}

#[test]
fn the_frame_licence_preserves_controls_and_requires_the_infinitival() {
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "core-verb:Cause")
        .unwrap()
        .properties
        .frames
        .clear();
    let without_cause = Lexicon::new(lexemes).unwrap();
    for text in [
        "You may have it gain flying until end of turn.",
        "You may have that player discard a card.",
        "That ability triggers an additional time.",
        "Target opponent chooses to sacrifice a creature.",
        "Creatures entering have haste.",
    ] {
        let actual = common::readings(text, Category::Sentence);
        assert!(!actual.is_empty());
        assert_eq!(
            actual,
            common::readings_with_lexicon(&without_cause, text, Category::Sentence)
        );
    }
    assert!(
        common::readings_with_lexicon(
            &without_cause,
            "cause abilities to trigger",
            Category::BarePredicate,
        )
        .is_empty()
    );
    for text in [
        "cause abilities",
        "cause abilities trigger",
        "cause abilities to triggers",
    ] {
        assert!(
            common::readings(text, Category::BarePredicate).is_empty(),
            "{text}"
        );
    }
}

fn word(owner: &str, form: WordForm, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| value.lexeme.as_str() == owner && value.form == form)
                .unwrap()
                .clone(),
        ),
        frame,
        countability: None,
    }
}

#[test]
fn independently_constructed_torpor_orb_constituent_preserves_both_laws() {
    let mut ability = word("lexeme:CommonNoun/Ability", WordForm::Plural, None);
    ability.countability = Some(true);
    let object = Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(Reading::BareNounPhrase {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: ability,
            }),
        }),
    };
    let infinitival = Reading::ToInfinitive {
        category: Category::InfinitiveComplement,
        form: 0,
        negator: None,
        marker: word("vocab:InfinitivalMarker/To", WordForm::Invariant, None),
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: word("core-verb:Trigger", WordForm::Plain, Some(0)),
                complements: vec![],
            }),
        }),
    };
    let value = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Cause", WordForm::Plain, Some(0)),
        complements: vec![
            FrameValue::Argument(Box::new(object)),
            FrameValue::Argument(Box::new(infinitival)),
        ],
    };
    value.admit(common::lexicon()).unwrap();
    let text = "cause abilities to trigger";
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert_eq!(
        common::readings(text, value.category()),
        BTreeSet::from([value.clone()])
    );
    let mut wrong = value;
    let Reading::SelectedPredicate { complements, .. } = &mut wrong else {
        unreachable!()
    };
    let FrameValue::Argument(complement) = &mut complements[1] else {
        unreachable!()
    };
    let Reading::ToInfinitive { category, .. } = complement.as_mut() else {
        unreachable!()
    };
    *category = Category::PurposeInfinitival;
    assert!(wrong.admit(common::lexicon()).is_err());
    assert!(wrong.realize(common::lexicon()).is_err());
}
