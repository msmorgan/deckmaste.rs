use crate::common;
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Numeral, Relation, SurfaceCase, WordForm,
};

fn assert_count_complement(header: &Reading, surface: &str) {
    let mut counts = Vec::new();
    header
        .visit(&mut |node| {
            if let Reading::SelectedPredicate {
                head, complements, ..
            } = node
            {
                let LexicalReading::Word(value) = &head.value else { return };
                if value.lexeme.as_str() != "core-verb:Choose" {
                    return;
                }
                let frame = &common::lexicon().lexemes()[&value.lexeme].properties.frames
                    [head.frame.unwrap()];
                for (slot, complement) in frame.items.iter().zip(complements) {
                    if let deckmaste_lexical::FrameItem::Argument(slot) = slot
                        && slot.relation == Relation::Complement
                        && slot.category == "QuantitativeDeterminer"
                    {
                        let FrameValue::Argument(count) = complement else {
                            panic!("count must fill an Argument")
                        };
                        assert_eq!(count.category(), Category::QuantitativeDeterminer);
                        counts.push(count.realize(common::lexicon()).unwrap());
                    }
                }
            }
        })
        .unwrap();
    assert_eq!(counts, [surface]);
}

#[test]
fn modal_headers_keep_the_count_complement_and_each_mode_body() {
    // Decoy Ploy; Collective Brutality's modal Ability; Sawblade Slinger.
    for (text, quantity) in [
        (
            "Choose one or both —\n• Return target Villain card from your graveyard to your hand.\n• Return target Hero card from your graveyard to your hand.",
            "one or both",
        ),
        (
            "Choose one or more —\n• Target opponent reveals their hand. You choose an instant or sorcery card from it. That player discards that card.\n• Target creature gets -2/-2 until end of turn.\n• Target opponent loses 2 life and you gain 2 life.",
            "one or more",
        ),
        (
            "When this creature enters, choose up to one —\n• Destroy target artifact an opponent controls.\n• This creature fights target Zombie an opponent controls.",
            "up to one",
        ),
    ] {
        let values = common::readings(text, Category::Document);
        assert!(!values.is_empty(), "{text}");
        let expected: Vec<_> = text
            .lines()
            .filter_map(|line| line.strip_prefix("• "))
            .collect();
        let mut complete_modes = false;
        for value in values {
            let mut bodies = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::ModalItem { header, modes, .. } = node {
                        assert_eq!(header.category(), Category::Clause);
                        assert_count_complement(header, quantity);
                        assert_eq!(modes.category(), Category::Modes);
                    }
                    if let Reading::Mode { body, .. } = node {
                        assert_eq!(body.category(), Category::Paragraph);
                        bodies.push(body.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
            complete_modes |= bodies == expected;
        }
        assert!(
            complete_modes,
            "each complete bullet must be a Mode: {text}"
        );
    }
}

#[test]
fn count_shapes_are_unique_and_do_not_compete_with_nominal_objects() {
    // Header constituents and the ticket's pinned sentence controls (MH4).
    for quantity in [
        "one or both",
        "one or more",
        "two or more",
        "up to one",
        "up to two",
        "up to three",
        "one",
        "two",
        "three",
        "one or two",
    ] {
        let text = format!("Choose {quantity}");
        let values = common::readings(&text, Category::Clause);
        assert_eq!(values.len(), 1, "one count Complement analysis: {text}");
        assert_count_complement(values.first().unwrap(), quantity);
    }
    // The retained any-number control has an NP Object headed by number.
    for text in [
        "Choose any number.",
        "Choose one or more creatures.",
        "Choose up to one creature.",
        "Choose one of them.",
    ] {
        let values = common::readings(text, Category::Document);
        assert_eq!(values.len(), 1, "unchanged NP Object analysis: {text}");
        let mut objects = Vec::new();
        values
            .first()
            .unwrap()
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                {
                    let LexicalReading::Word(value) = &head.value else { return };
                    if value.lexeme.as_str() == "core-verb:Choose" {
                        let frame = &common::lexicon().lexemes()[&value.lexeme].properties.frames
                            [head.frame.unwrap()];
                        let [deckmaste_lexical::FrameItem::Argument(slot)] = frame.items.as_slice()
                        else {
                            panic!("expected Object frame")
                        };
                        assert_eq!(slot.relation, Relation::Object);
                        let [FrameValue::Argument(object)] = complements.as_slice() else {
                            panic!("expected NP Object")
                        };
                        assert_eq!(object.category(), Category::AccusativePhrase);
                        let Reading::CasePhrase { head, .. } = object.as_ref() else {
                            panic!("expected the Object's accusative projection")
                        };
                        assert_eq!(head.category(), Category::NounPhrase);
                        objects.push(object.realize(common::lexicon()).unwrap());
                    }
                }
            })
            .unwrap();
        assert_eq!(
            objects,
            [text
                .strip_prefix("Choose ")
                .unwrap()
                .strip_suffix('.')
                .unwrap()]
        );
    }
    // MH3 and MH5: no general fused cardinal or standalone Determinative.
    for text in [
        "Choose one that hasn't been chosen.",
        "Choose both.",
        "You may choose both instead.",
        "Put one into your hand.",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
}

fn invariant_word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_constructed_coordinated_count_roundtrips_in_its_selected_frame() {
    // Decoy Ploy's count and bare header clause, constructed independently.
    let count = Reading::CardinalDeterminativeCoordination {
        form: 0,
        left: Box::new(Reading::Cardinal {
            form: 0,
            modifier: None,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 1,
                    notation: Numeral::Cardinal,
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        coordinator: invariant_word("vocab:Coordinator/Or"),
        right: invariant_word("vocab:FloatedQuantifier/Both"),
    };
    let lexeme = &common::lexicon().lexemes()["core-verb:Choose"];
    let frame = lexeme.properties.frames.iter().position(|frame| {
        matches!(frame.items.as_slice(), [deckmaste_lexical::FrameItem::Argument(slot)] if slot.relation == Relation::Complement && slot.category == "QuantitativeDeterminer")
    }).unwrap();
    let header = Reading::Imperative {
        form: 0,
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "core-verb:Choose".into(),
                        form: WordForm::Plain,
                        features: FeatureBundle::default(),
                        variant: 0,
                        capitalization: SurfaceCase::Initial,
                    }),
                    frame: Some(frame),
                    countability: None,
                },
                complements: vec![FrameValue::Argument(Box::new(count.clone()))],
            }),
        }),
    };
    for (value, category, surface) in [
        (count, Category::QuantitativeDeterminer, "one or both"),
        (header, Category::Clause, "Choose one or both"),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), surface);
        assert_eq!(common::readings(surface, category), [value].into());
    }
}
