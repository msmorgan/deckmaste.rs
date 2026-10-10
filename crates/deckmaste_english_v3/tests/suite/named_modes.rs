use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::LexicalReading;

#[test]
fn whole_faces_have_flavor_word_labels_on_modes_and_parsed_paragraph_bodies() {
    for (card, text) in [
        (
            "Megaton's Fate",
            "Choose one —\n• Disarm — Destroy target artifact. Create four Treasure tokens.\n• Detonate — Megaton's Fate deals 8 damage to each creature. Each player gets four rad counters.",
        ),
        (
            "Dawnbringer Cleric",
            "When this creature enters, choose one —\n• Cure Wounds — You gain 2 life.\n• Dispel Magic — Destroy target enchantment.\n• Gentle Repose — Exile target card from a graveyard.",
        ),
        (
            "Cult of Skaro",
            "Whenever Cult of Skaro attacks, choose one at random —\n• Thay — Put a +1/+1 counter on each artifact creature you control.\n• Caan — Draw two cards.\n• Sec — Create a 3/3 black Dalek artifact creature token with menace.\n• Jast — Each opponent loses 4 life.",
        ),
        (
            "Special Move",
            "Choose two —\n• Jump Kick — Destroy target artifact.\n• Dash Attack — Put two +1/+1 counters on target attacking or blocking creature you control.\n• Foot Toss — Target creature you control deals damage equal to its power to any other target. Then sacrifice it.",
        ),
        (
            "Battle Menu",
            "Choose one —\n• Attack — Create a 2/2 white Knight creature token.\n• Ability — Target creature gets +0/+4 until end of turn.\n• Magic — Destroy target creature with power 4 or greater.\n• Item — You gain 4 life.",
        ),
    ] {
        let expected: Vec<_> = text
            .lines()
            .filter_map(|line| {
                line.strip_prefix("• ")
                    .and_then(|line| line.split_once(" — "))
            })
            .collect();
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        let mut observed = Vec::new();
        for reading in readings {
            let mut actual = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::Mode { body, .. } = node {
                        let Reading::FlavorWordHead { head, body, .. } = body.as_ref() else {
                            panic!("a named Mode must have a Flavor Word head")
                        };
                        let LexicalReading::FlavorWord { label } = &head.value else {
                            panic!("a mode name must be an open Flavor Word label")
                        };
                        let Reading::OrdinaryAbility { body, .. } = body.as_ref() else {
                            panic!("the labelled body must retain its ordinary Paragraph")
                        };
                        assert_eq!(body.category(), Category::Paragraph);
                        actual.push((label.clone(), body.realize(common::lexicon()).unwrap()));
                    }
                    if let Reading::ModalItem { header, .. } = node {
                        assert_eq!(header.category(), Category::Clause);
                    }
                })
                .unwrap();
            assert_eq!(
                actual
                    .iter()
                    .map(|(label, _)| label.as_str())
                    .collect::<Vec<_>>(),
                expected.iter().map(|(label, _)| *label).collect::<Vec<_>>()
            );
            observed.push(actual);
        }
        // Preserve the plain Mode's paragraph-scope alternatives, including a
        // final sentence outside the modal item, while requiring the full bodies.
        assert!(
            observed.contains(
                &expected
                    .iter()
                    .map(|(label, body)| { ((*label).to_owned(), (*body).to_owned()) })
                    .collect::<Vec<_>>()
            )
        );
    }
}

#[test]
fn plain_modal_controls_keep_unlabelled_paragraph_bodies() {
    // Naturalize's effects under the ticket's modal control; Dawnbringer Cleric.
    for text in [
        "Choose one —\n• Destroy target artifact.\n• Destroy target enchantment.",
        "When this creature enters, choose one —\n• You gain 2 life.\n• Destroy target enchantment.",
    ] {
        let readings = common::readings(text, Category::Document);
        assert!(!readings.is_empty());
        for reading in readings {
            let mut bodies = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::Mode { body, .. } = node {
                        bodies.push(body.category());
                    }
                })
                .unwrap();
            assert_eq!(bodies, vec![Category::Paragraph, Category::Paragraph]);
        }
    }
}

#[test]
fn a_flavor_word_head_outside_a_mode_remains_an_ordinary_ability() {
    // Dawnbringer Cleric's Cure Wounds body, as the ticket's ordinary-head control.
    let text = "Cure Wounds — You gain 2 life.";
    let readings = common::readings(text, Category::Document);
    assert!(!readings.is_empty());
    for reading in readings {
        let Reading::Document { first, .. } = reading else {
            panic!("expected Document")
        };
        let Reading::FlavorWordHead { head, body, .. } = first.as_ref() else {
            panic!("expected ordinary Flavor Word head")
        };
        assert_eq!(
            head.value,
            LexicalReading::FlavorWord {
                label: "Cure Wounds".into()
            }
        );
        assert_eq!(body.realize(common::lexicon()).unwrap(), "You gain 2 life.");
    }
}

#[test]
fn bullets_and_a_modal_header_are_not_standalone_flavor_abilities() {
    for text in [
        "• Cure Wounds — You gain 2 life.",
        "• You gain 2 life.",
        "Choose one —",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn an_independently_constructed_named_mode_roundtrips_with_its_exact_label() {
    // Dawnbringer Cleric: construct the label, wrapper and Mode independently
    // of the complete named-mode source, retaining a parsed Paragraph fixture.
    let paragraph = common::readings("You gain 2 life.", Category::Paragraph)
        .into_iter()
        .next()
        .unwrap();
    let body = Reading::FlavorWordHead {
        form: 0,
        head: Word {
            value: LexicalReading::FlavorWord {
                label: "Cure Wounds".into(),
            },
            frame: None,
            countability: None,
        },
        body: Box::new(Reading::OrdinaryAbility {
            form: 0,
            body: Box::new(paragraph),
        }),
    };
    let expected = Reading::Mode {
        form: 1,
        body: Box::new(body.clone()),
    };
    expected.admit(common::lexicon()).unwrap();
    let text = expected.realize(common::lexicon()).unwrap();
    assert_eq!(text, "• Cure Wounds — You gain 2 life.");
    assert_eq!(common::readings(&text, Category::Mode), [expected].into());
    assert!(
        Reading::Mode {
            form: 0,
            body: Box::new(body)
        }
        .admit(common::lexicon())
        .is_err()
    );
    let invalid = LexicalReading::FlavorWord {
        label: "• Cure Wounds".into(),
    };
    assert!(common::lexicon().realize(&invalid).is_err());
}
