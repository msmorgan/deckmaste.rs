mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn causative(text: &str, category: Category, object: &str, predicates: &[&str]) {
    let values = readings(text, category);
    assert!(!values.is_empty(), "missing causative Reading of {text}");
    let mut observed = BTreeSet::new();
    for value in values {
        let mut found = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme == "core-verb:Have"
                {
                    let [FrameValue::Argument(np), FrameValue::Argument(clause)] =
                        complements.as_slice()
                    else {
                        panic!("Have must select its Object and infinitival: {node:?}");
                    };
                    assert_eq!(np.category(), Category::AccusativePhrase);
                    assert_eq!(np.realize(lexicon()).unwrap(), object);
                    assert_eq!(clause.category(), Category::BarePredicate);
                    observed.insert(clause.realize(lexicon()).unwrap());
                    let frame =
                        &lexicon().lexemes()[&word.lexeme].properties.frames[head.frame.unwrap()];
                    assert_eq!(
                        frame.items,
                        vec![
                            FrameItem::Argument(deckmaste_lexical::FrameSlot {
                                relation: Relation::Object,
                                category: "NounPhrase".into(),
                            }),
                            FrameItem::Argument(deckmaste_lexical::FrameSlot {
                                relation: Relation::Complement,
                                category: "BarePredicate".into(),
                            }),
                        ]
                    );
                    found.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(found.len(), 1, "missing or duplicate causative in {text}");
    }
    assert_eq!(
        observed,
        predicates.iter().map(ToString::to_string).collect()
    );
}

#[test]
fn quill_slinger_boggart_selects_object_and_bare_infinitival_under_may() {
    causative(
        "Whenever a player casts a Kithkin spell, you may have target player lose 1 life.",
        Category::Document,
        "target player",
        &["lose 1 life"],
    );
}

#[test]
fn rage_forger_keeps_the_damage_predicate_inside_the_infinitival() {
    causative(
        "Whenever a creature you control with a +1/+1 counter on it attacks, you may have that creature deal 1 damage to target player or planeswalker.",
        Category::Document,
        "that creature",
        &["deal 1 damage to target player or planeswalker"],
    );
}

#[test]
fn joraga_bard_selects_plural_object_and_infinitival() {
    causative(
        "Whenever this creature or another Ally you control enters, you may have Ally creatures you control gain vigilance until end of turn.",
        Category::Document,
        "Ally creatures you control",
        // The duration may scope Gain or the matrix predicate. Both keep the
        // plural NP as Have's Object and Gain as its catenative Complement.
        &["gain vigilance", "gain vigilance until end of turn"],
    );
}

#[test]
fn extractor_demon_selects_the_player_and_mill_infinitival() {
    causative(
        "Whenever another creature leaves the battlefield, you may have target player mill two cards.",
        Category::Document,
        "target player",
        &["mill two cards"],
    );
}

#[test]
fn ebon_dragon_selects_the_opponent_as_object() {
    causative(
        "When this creature enters, you may have target opponent discard a card.",
        Category::Document,
        "target opponent",
        &["discard a card"],
    );
}

#[test]
fn wandering_troubadour_finite_had_selects_object_and_infinitival() {
    causative(
        "At the beginning of your end step, if you had a land enter the battlefield under your control this turn, venture into the dungeon.",
        Category::Document,
        "a land",
        &[
            "enter the battlefield",
            "enter the battlefield under your control",
            "enter the battlefield under your control this turn",
        ],
    );
}

#[test]
fn lava_blister_finite_has_selects_the_named_damage_source_as_object() {
    causative(
        "Destroy target nonbasic land unless its controller has Lava Blister deal 6 damage to them.",
        Category::Document,
        "Lava Blister",
        &["deal 6 damage to them"],
    );
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

fn noun(owner: &str) -> Reading {
    let mut head = word(
        owner,
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..Default::default()
        },
        None,
    );
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

fn accusative(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}

#[test]
fn independent_ebon_dragon_constituent_preserves_both_roundtrip_laws() {
    let object = accusative(Reading::TargetNounPhrase {
        form: 0,
        marker: word(
            "vocab:TargetingMarker/Target",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        ),
        head: Box::new(noun("lexeme:CommonNoun/Opponent")),
    });
    let predicate = Reading::BarePredicate {
        form: 0,
        head: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word(
                "lexeme:keyword_action/discard",
                WordForm::Plain,
                FeatureBundle::default(),
                Some(0),
            ),
            complements: vec![FrameValue::Argument(Box::new(accusative(
                Reading::IndefiniteNounPhrase {
                    form: 0,
                    determiner: word(
                        "vocab:Article/Indefinite",
                        WordForm::Invariant,
                        FeatureBundle {
                            number: Some(Number::Singular),
                            ..Default::default()
                        },
                        None,
                    ),
                    head: Box::new(noun("lexeme:CommonNoun/Card")),
                },
            )))],
        }),
    };
    // The attested Ebon Dragon constituent supplies the same surface for the
    // plain and present forms; finite admission retains agreement alternatives.
    for (category, form, features) in [
        (
            Category::SecondaryVerbPhrase,
            WordForm::Plain,
            FeatureBundle::default(),
        ),
        (
            Category::FinitePredicate,
            WordForm::Present,
            FeatureBundle {
                number: Some(Number::Plural),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                ..Default::default()
            },
        ),
    ] {
        let value = Reading::SelectedPredicate {
            category,
            form: 0,
            head: word("core-verb:Have", form, features, Some(2)),
            complements: vec![
                FrameValue::Argument(Box::new(object.clone())),
                FrameValue::Argument(Box::new(predicate.clone())),
            ],
        };
        value.admit(lexicon()).unwrap();
        let text = "have target opponent discard a card";
        assert_eq!(
            value.realize(lexicon()).unwrap().as_bytes(),
            text.as_bytes()
        );
        let parsed = readings(text, category);
        assert!(parsed.contains(&value));
        if category == Category::SecondaryVerbPhrase {
            assert_eq!(parsed, BTreeSet::from([value.clone()]));
        }
        let actual = parsed.get(&value).unwrap();
        let mut expected_nodes = Vec::new();
        let mut actual_nodes = Vec::new();
        value
            .visit(&mut |node| expected_nodes.push(node.clone()))
            .unwrap();
        actual
            .visit(&mut |node| actual_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(actual_nodes, expected_nodes);
        let mut expected_words = Vec::new();
        let mut actual_words = Vec::new();
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        actual
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);
        let mut wrong = value;
        let Reading::SelectedPredicate { complements, .. } = &mut wrong else {
            unreachable!()
        };
        complements.swap(0, 1);
        assert!(wrong.admit(lexicon()).is_err());
        assert!(wrong.realize(lexicon()).is_err());
    }
}

#[test]
fn causative_complement_requires_plain_form_without_to() {
    for text in [
        "have target opponent discards a card",
        "have target opponent to discard a card",
    ] {
        assert!(
            readings(text, Category::SecondaryVerbPhrase).is_empty(),
            "{text}"
        );
    }
    // Participial strings can already read through the possessive or perfect
    // frames. The replacement must add no Reading to either diagnostic.
    let mut lexemes: Vec<_> = lexicon().lexemes().values().cloned().collect();
    let have = lexemes
        .iter_mut()
        .find(|l| l.id == "core-verb:Have")
        .unwrap();
    have.properties.frames[2] = Frame {
        kind: "Predicate".into(),
        items: ["Object", "VerbPhrase"]
            .map(|category| {
                FrameItem::Argument(FrameSlot {
                    relation: Relation::Complement,
                    category: category.into(),
                })
            })
            .into(),
    };
    let baseline = Lexicon::new(lexemes).unwrap();
    for text in [
        "have target opponent discarding a card",
        "have target opponent discarded a card",
    ] {
        assert_eq!(
            readings(text, Category::SecondaryVerbPhrase),
            common::readings_with_lexicon(&baseline, text, Category::SecondaryVerbPhrase),
            "the causative frame must not add a participial Reading of {text}",
        );
    }
}
