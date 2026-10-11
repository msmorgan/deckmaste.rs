use crate::common;
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{LexicalReading, LexicalValue, SurfaceCase, WordForm};

fn postposed(text: &str, card: Option<&str>, owner: &str, object: &str) {
    let values = match card {
        Some(card) => common::readings_on_card(card, text, Category::Sentence),
        None => common::readings(text, Category::Sentence),
    };
    assert!(!values.is_empty(), "missing Reading of {text:?}");
    let mut complete_object = false;
    for value in values {
        value.admit(common::lexicon()).unwrap();
        let mut selected = false;
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_str() == owner
                {
                    selected = true;
                    let frame = &common::lexicon().lexemes()[&word.lexeme].properties.frames
                        [head.frame.unwrap()];
                    let mut observed = false;
                    for (slot, argument) in frame.items.iter().zip(complements) {
                        if let FrameValue::PostposedObject { argument, .. } = argument {
                            assert!(matches!(slot, deckmaste_lexical::FrameItem::Argument(slot)
                            if slot.relation == deckmaste_lexical::Relation::Object));
                            observed = true;
                            complete_object |=
                                argument.realize(common::lexicon()).unwrap() == object;
                        }
                    }
                    assert!(
                        observed,
                        "the selected frame must retain its postposed Object"
                    );
                }
            })
            .unwrap();
        assert!(selected, "missing selected Predicate");
    }
    assert!(complete_object, "missing complete Object {object:?}");
}

#[test]
fn return_witnesses_keep_the_object_in_its_declared_slot() {
    for (text, object) in [
        (
            "Return to the battlefield all permanent cards in your graveyard that were put there from the battlefield this turn.",
            "all permanent cards in your graveyard that were put there from the battlefield this turn",
        ),
        (
            "Return to your hand all cards in your graveyard that you cycled or discarded this turn.",
            "all cards in your graveyard that you cycled or discarded this turn",
        ),
        (
            "Return to their owners' hands all creatures with toughness less than or equal to the number of Islands you control.",
            "all creatures with toughness less than or equal to the number of Islands you control",
        ),
        (
            "When this creature enters, return to your hand all cards in your graveyard that were put there from the battlefield this turn.",
            "all cards in your graveyard that were put there from the battlefield this turn",
        ),
        (
            "Return to its owner's hand each creature with power greater than the number of cards in its controller's hand.",
            "each creature with power greater than the number of cards in its controller's hand",
        ),
    ] {
        postposed(text, None, "core-verb:Return", object);
    }
    postposed(
        "Whenever Moira deals combat damage to a player, return to the battlefield target creature card in your graveyard that was put there from the battlefield this turn.",
        Some("Moira, Urborg Haunt"),
        "core-verb:Return",
        "target creature card in your graveyard that was put there from the battlefield this turn",
    );
}

#[test]
fn the_alternation_applies_to_other_declared_frame_shapes() {
    postposed(
        "Put onto the battlefield under your control all creature cards in all graveyards that were put there from the battlefield this turn.",
        None,
        "core-verb:Put",
        "all creature cards in all graveyards that were put there from the battlefield this turn",
    );
    postposed(
        "Shuffle into your library all creature cards in your graveyard.",
        None,
        "lexeme:keyword_action/shuffle",
        "all creature cards in your graveyard",
    );
}

#[test]
fn default_order_retains_ordinary_frame_values() {
    for text in [
        "Return target creature card from your graveyard to the battlefield.",
        "Return this card from your graveyard to your hand.",
        "Return all creature cards in your graveyard to your hand.",
        "Put all creature cards in all graveyards that were put there from the battlefield this turn onto the battlefield under your control.",
    ] {
        let values = common::readings(text, Category::Sentence);
        assert!(!values.is_empty());
        for value in values {
            value
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate { complements, .. } = node {
                        assert!(
                            complements
                                .iter()
                                .all(|value| !matches!(value, FrameValue::PostposedObject { .. }))
                        );
                    }
                })
                .unwrap();
        }
    }
}

fn word(owner: &str, form: WordForm) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: deckmaste_lexical::FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_constructed_postposed_object_obeys_admission_and_roundtrip() {
    let mut head = word("core-verb:Return", WordForm::Plain);
    head.frame = Some(1);
    let object = common::readings(
        "all creature cards in your graveyard",
        Category::AccusativePhrase,
    )
    .into_iter()
    .next()
    .unwrap();
    let destination = common::readings("your hand", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    let expected = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head,
        complements: vec![
            FrameValue::PostposedObject {
                argument: Box::new(object),
                modifier: None,
            },
            FrameValue::Marker(word("vocab:Preposition/To", WordForm::Invariant)),
            FrameValue::Argument(Box::new(destination)),
        ],
    };
    expected.admit(common::lexicon()).unwrap();
    let text = "return to your hand all creature cards in your graveyard";
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, Category::SecondaryVerbPhrase).contains(&expected));
    let mut invalid = expected;
    let Reading::SelectedPredicate { complements, .. } = &mut invalid else {
        unreachable!()
    };
    let FrameValue::PostposedObject { argument, .. } = &mut complements[0] else {
        unreachable!()
    };
    **argument = common::readings("this card from them", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    assert!(
        invalid.admit(common::lexicon()).is_err(),
        "the declared SourceFreeObject requirement must still apply"
    );
}

#[test]
fn a_light_active_object_cannot_postpose() {
    assert!(common::readings("Return to your hand it.", Category::Sentence).is_empty());
}

#[test]
fn agitator_ant_and_melira_passive_fragments_keep_their_readings_by_value() {
    // Passive fragments from Agitator Ant and Melira, the Living Cure.
    // Compose their existing gap predicate and licensed nominal Adjunct;
    // no additional active Predicate with that Adjunct as Object is allowed.
    for (predicate, adjunct) in [
        ("put on it", "this way"),
        ("put into a graveyard", "this turn"),
    ] {
        let heads = common::readings(predicate, Category::SecondaryVerbPhrase);
        let modifiers = common::readings(adjunct, Category::NominalAdjunctPhrase);
        assert!(!heads.is_empty() && !modifiers.is_empty());
        let expected: std::collections::BTreeSet<_> = heads
            .into_iter()
            .flat_map(|head| {
                modifiers
                    .iter()
                    .map(move |modifier| Reading::NominalAdjunctPredicate {
                        category: Category::SecondaryVerbPhrase,
                        form: 0,
                        head: Box::new(head.clone()),
                        modifier: Box::new(modifier.clone()),
                    })
            })
            .collect();
        for value in &expected {
            value.admit(common::lexicon()).unwrap();
        }
        assert_eq!(
            common::readings(
                &format!("{predicate} {adjunct}"),
                Category::SecondaryVerbPhrase
            ),
            expected
        );
    }
}

#[test]
fn tapped_retains_the_existing_depictive_role_before_the_postposed_object() {
    let text = "Return to the battlefield tapped all permanent cards in your graveyard that were put there from the battlefield this turn.";
    postposed(
        text,
        None,
        "core-verb:Return",
        "all permanent cards in your graveyard that were put there from the battlefield this turn",
    );
    let mut observed = false;
    for value in common::readings(text, Category::Sentence) {
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate { complements, .. } = node {
                    for value in complements {
                        if let FrameValue::PostposedObject {
                            modifier: Some(modifier),
                            ..
                        } = value
                        {
                            assert_eq!(modifier.category(), Category::DepictivePhrase);
                            assert_eq!(modifier.realize(common::lexicon()).unwrap(), "tapped");
                            observed = true;
                        }
                    }
                }
            })
            .unwrap();
    }
    assert!(observed, "missing the existing Depictive Adjunct role");
}

#[test]
fn intervening_source_adjunct_keeps_its_role_when_the_object_postposes() {
    let text =
        "Return at random from your graveyard to your hand all creature cards in your graveyard.";
    postposed(
        text,
        None,
        "core-verb:Return",
        "all creature cards in your graveyard",
    );
    for value in common::readings(text, Category::Sentence) {
        let mut observed = false;
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_str() == "core-verb:Return"
                {
                    for value in complements {
                        if let FrameValue::Adjunct(reading) = value {
                            assert_eq!(reading.category(), Category::InterveningPrepositionAdjunct);
                            assert_eq!(reading.realize(common::lexicon()).unwrap(), " at random");
                            observed = true;
                        }
                    }
                }
            })
            .unwrap();
        assert!(observed, "the source Adjunct must retain its declared role");
    }
}
