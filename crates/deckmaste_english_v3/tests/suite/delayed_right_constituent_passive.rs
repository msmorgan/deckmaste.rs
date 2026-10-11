use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    Frame, FrameItem, FrameSlot, LexicalReading, Relation, SurfaceCase, WordForm,
};

use crate::common;

const WITNESSES: [(&str, &str); 5] = [
    (
        "Prevent all combat damage that would be dealt to and dealt by this creature.",
        "this creature",
    ),
    (
        "Prevent all combat damage that would be dealt to and dealt by enchanted creature.",
        "enchanted creature",
    ),
    (
        "Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
        "that creature",
    ),
    (
        "Until your next turn, prevent all damage that would be dealt to and dealt by target permanent an opponent controls.",
        "target permanent an opponent controls",
    ),
    (
        "Prevent all combat damage that would be dealt to and dealt by creatures you control.",
        "creatures you control",
    ),
];

fn lexical(word: &Word) -> &deckmaste_lexical::LexicalValue {
    let LexicalReading::Word(value) = &word.value else {
        panic!("expected a lexical Word")
    };
    value
}

fn frame(word: &Word) -> &Frame {
    &common::lexicon().lexemes()[&lexical(word).lexeme]
        .properties
        .frames[word.frame.unwrap()]
}

fn object_slot() -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation: Relation::Object,
        category: "NounPhrase".into(),
    })
}

fn recipient_frame() -> Frame {
    Frame {
        kind: "Predicate".into(),
        items: vec![
            object_slot(),
            FrameItem::Marker {
                vocabulary: "Preposition".into(),
                member: "To".into(),
            },
            object_slot(),
        ],
    }
}

fn agent_frame() -> Frame {
    Frame {
        kind: "Predicate".into(),
        items: vec![object_slot()],
    }
}

fn assert_member(member: &Reading, marker_owner: &str, expected_frame: &Frame) {
    let Reading::PassiveSelectedHead { head, marker, .. } = member else {
        panic!("a shared NP requires an incomplete passive member")
    };
    assert_eq!(lexical(head).lexeme.as_str(), "core-verb:Deal");
    assert_eq!(lexical(head).form, WordForm::PastParticiple);
    assert_eq!(lexical(marker).lexeme.as_str(), marker_owner);
    assert_eq!(frame(head), expected_frame);
}

#[test]
fn witnesses_keep_two_incomplete_members_and_one_shared_np() {
    // Fog Bank, Gaseous Form, Maze of Ith, Kiora, the Crashing Wave, Statecraft.
    for (text, np) in WITNESSES {
        let control = text.replace("dealt to and dealt by", "dealt to");
        let mut control_complements = BTreeSet::new();
        for value in common::readings(&control, Category::Sentence) {
            value
                .visit(&mut |node| {
                    if let Reading::SelectedGapPrepositionPredicate {
                        head,
                        marker,
                        complement,
                        ..
                    } = node
                        && lexical(head).lexeme == "core-verb:Deal"
                        && lexical(marker).lexeme == "vocab:Preposition/To"
                    {
                        control_complements.insert(complement.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
        }
        assert!(
            control_complements.contains(np),
            "the NP-internal attachment must survive"
        );
        let values = common::readings(text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        let mut shared_complements = BTreeSet::new();
        for value in values {
            let mut shared = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::SharedObjectComplement { head, object, .. } = node {
                        shared.push((head.clone(), object.clone()));
                    }
                })
                .unwrap();
            assert_eq!(shared.len(), 1, "one Shared Complement route");
            let (head, object) = &shared[0];
            assert_eq!(object.category(), Category::AccusativePhrase);
            shared_complements.insert(object.realize(common::lexicon()).unwrap());
            let Reading::Coordination {
                category,
                left,
                right,
                coordinator,
                ..
            } = head.as_ref()
            else {
                panic!("the passive heads must coordinate below the shared NP")
            };
            assert_eq!(*category, Category::SecondarySelectedHead);
            assert_eq!(
                lexical(coordinator).lexeme.as_str(),
                "vocab:Coordinator/And"
            );
            assert_member(left, "vocab:Preposition/To", &recipient_frame());
            assert_member(right, "vocab:Preposition/By", &agent_frame());
        }
        // W9-A preserves independent relative-clause attachment. Kiora and
        // Statecraft also permit the relative outside the shared NP.
        assert_eq!(shared_complements, control_complements, "{text}");
    }
}

fn word(owner: &str, form: WordForm) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: None,
    }
}

fn passive_member(marker: &str, selected_frame: &Frame, form: usize) -> Reading {
    let mut head = word("core-verb:Deal", WordForm::PastParticiple);
    head.frame = Some(
        common::lexicon().lexemes()[&lexical(&head).lexeme]
            .properties
            .frames
            .iter()
            .position(|frame| frame == selected_frame)
            .unwrap(),
    );
    Reading::PassiveSelectedHead {
        form,
        head,
        marker: word(marker, WordForm::Invariant),
    }
}

#[test]
fn independently_constructed_shared_passive_roundtrips() {
    let mut creature = word("lexeme:type/creature", WordForm::Singular);
    creature.countability = Some(true);
    let expected = Reading::SharedObjectComplement {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(Reading::Coordination {
            category: Category::SecondarySelectedHead,
            form: 0,
            left: Box::new(passive_member(
                "vocab:Preposition/To",
                &recipient_frame(),
                0,
            )),
            right: Box::new(passive_member("vocab:Preposition/By", &agent_frame(), 1)),
            coordinator: word("vocab:Coordinator/And", WordForm::Invariant),
        }),
        object: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(Reading::DeterminedNounPhrase {
                form: 0,
                determiner: word("vocab:SingularDemonstrative/This", WordForm::Invariant),
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: creature,
                }),
            }),
        }),
    };
    let text = "dealt to and dealt by this creature"; // Fog Bank constituent.
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert_eq!(
        common::readings(text, Category::SecondaryVerbPhrase),
        BTreeSet::from([expected])
    );
}

#[test]
fn complete_controls_keep_their_existing_routes_and_stranding_stays_unread() {
    let controls = [
        "Prevent all combat damage that would be dealt to this creature and dealt by this creature.",
        "Whenever this creature attacks or blocks a creature, destroy that creature.",
        "Whenever this creature blocks or becomes blocked by a creature, destroy that creature.",
        "Damage is dealt to this creature.",
    ].into_iter().map(str::to_owned).chain(WITNESSES.iter().flat_map(|(text, _)| [
        text.replace("dealt to and dealt by", "dealt to"),
        text.replace("dealt to and dealt by", "dealt by"),
    ]));
    for text in controls {
        let values = common::readings(&text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            value
                .visit(&mut |node| assert!(!matches!(node, Reading::PassiveSelectedHead { .. })))
                .unwrap();
        }
    }
    assert!(common::readings("dealt to", Category::SecondaryVerbPhrase).is_empty());
    assert!(common::readings("dealt to and dealt by", Category::SecondaryVerbPhrase).is_empty());
}

#[test]
fn declared_frames_and_function_licences_exclude_wrong_members() {
    let selected = passive_member("vocab:Preposition/To", &recipient_frame(), 0);
    let Reading::PassiveSelectedHead { head, marker, form } = selected else {
        unreachable!()
    };
    let wrong_marker = Reading::PassiveSelectedHead {
        form,
        head: head.clone(),
        marker: word("vocab:Preposition/During", WordForm::Invariant),
    };
    assert!(wrong_marker.admit(common::lexicon()).is_err());
    let wrong_role = Reading::PassiveSelectedHead {
        form: 1,
        head,
        marker,
    };
    assert!(wrong_role.admit(common::lexicon()).is_err());
    let mut plain = passive_member("vocab:Preposition/By", &agent_frame(), 1);
    let Reading::PassiveSelectedHead { head, .. } = &mut plain else {
        unreachable!()
    };
    let selected_frame = head.frame;
    *head = word("core-verb:Deal", WordForm::Plain);
    head.frame = selected_frame;
    assert!(plain.admit(common::lexicon()).is_err());
}
