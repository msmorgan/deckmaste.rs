use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Lexicon, SurfaceCase, WordForm,
};

fn unlicensed_lexicon() -> Lexicon {
    Lexicon::new(
        common::lexicon()
            .lexemes()
            .values()
            .cloned()
            .map(|mut lexeme| {
                lexeme.properties.features.remove("SelectedTailLicence");
                lexeme
            }),
    )
    .unwrap()
}

fn word(id: &str, form: WordForm, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

fn fragment(text: &str, category: Category) -> Box<Reading> {
    Box::new(common::readings(text, category).into_iter().next().unwrap())
}

#[test]
fn witnesses_select_manner_inside_the_final_tail() {
    // Glimpse the Cosmos, Raven Familiar, Tezzeret, Evolutionary Leap,
    // and Enshrined Memories: the ticket's complete failing lines.
    for text in [
        "Look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
        "When this creature enters, look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
        "+1: Reveal cards from the top of your library until you reveal an artifact card. Put that card into your hand and the rest on the bottom of your library in a random order.",
        "{G}, Sacrifice a creature: Reveal cards from the top of your library until you reveal a creature card. Put that card into your hand and the rest on the bottom of your library in a random order.",
        "Reveal the top X cards of your library. Put all creature cards revealed this way into your hand and the rest on the bottom of your library in any order.",
    ] {
        let values = common::readings(text, Category::Document);
        assert!(!values.is_empty(), "unread witness: {text}");
        for value in values {
            let mut clusters = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::SelectedComplementClustersPredicate { .. } = node {
                        clusters.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(
                clusters.len(),
                1,
                "one selected verb coordinates the two tails"
            );
            let Reading::SelectedComplementClustersPredicate { tail, .. } = &clusters[0] else {
                unreachable!()
            };
            let Reading::Coordination { left, right, .. } = tail.as_ref() else {
                panic!("expected tail coordination")
            };
            let Reading::ObjectPrepositionTail {
                object, complement, ..
            } = left.as_ref()
            else {
                panic!("expected marked-NP goal")
            };
            assert_eq!(object.category(), Category::AccusativePhrase);
            assert_eq!(complement.realize(common::lexicon()).unwrap(), "your hand");
            let Reading::ObjectLocativeMannerTail {
                object,
                complement,
                manner,
                ..
            } = right.as_ref()
            else {
                panic!("expected final selected Manner Complement")
            };
            assert_eq!(object.realize(common::lexicon()).unwrap(), "the rest");
            assert_eq!(complement.category(), Category::LocativeComplement);
            assert_eq!(
                complement.realize(common::lexicon()).unwrap(),
                "on the bottom of your library"
            );
            assert_eq!(manner.category(), Category::MannerComplement);
            assert_eq!(
                manner.realize(common::lexicon()).unwrap(),
                if text.contains("random order") { "a random order" } else { "any order" }
            );
        }
    }
}

#[test]
fn mixed_goals_preserve_every_np_coordination_reading() {
    // GC5: the old NP-coordination analyses of the shorter shape are retained.
    let text = "Put one of them into your hand and the rest on the bottom of your library.";
    let incumbent = common::readings_with_lexicon(&unlicensed_lexicon(), text, Category::Document);
    let values = common::readings(text, Category::Document);
    assert!(!incumbent.is_empty());
    assert!(incumbent.is_subset(&values));
    let added: Vec<_> = values.difference(&incumbent).collect();
    assert_eq!(added.len(), 1, "GC6 adds exactly the tail-route Reading");
    common::assert_constituents(
        text,
        Category::Document,
        &[
            (
                Category::SelectedComplementTail,
                "one of them into your hand",
            ),
            (
                Category::SelectedComplementTail,
                "the rest on the bottom of your library",
            ),
            (
                Category::LocativeComplement,
                "on the bottom of your library",
            ),
        ],
    );
}

#[test]
fn independently_constructed_mixed_tails_roundtrip() {
    // Independently assemble the frame and Coordination from attested constituents.
    let value = Reading::SelectedComplementClustersPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Put", WordForm::Plain, Some(3)),
        tail: Box::new(Reading::Coordination {
            category: Category::SelectedComplementTail,
            form: 0,
            left: Box::new(Reading::ObjectPrepositionTail {
                form: 0,
                object: fragment("one of them", Category::AccusativePhrase),
                marker: word("vocab:Preposition/Into", WordForm::Invariant, None),
                complement: fragment("your hand", Category::AccusativePhrase),
            }),
            coordinator: word("vocab:Coordinator/And", WordForm::Invariant, None),
            right: Box::new(Reading::ObjectLocativeMannerTail {
                form: 0,
                object: fragment("the rest", Category::AccusativePhrase),
                complement: fragment(
                    "on the bottom of your library",
                    Category::LocativeComplement,
                ),
                marker: word("vocab:Preposition/In", WordForm::Invariant, None),
                manner: fragment("any order", Category::MannerComplement),
            }),
        }),
    };
    value.admit(common::lexicon()).unwrap();
    let text = value.realize(common::lexicon()).unwrap();
    assert_eq!(
        text,
        "put one of them into your hand and the rest on the bottom of your library in any order"
    );
    assert!(common::readings(&text, Category::SecondaryVerbPhrase).contains(&value));
}

#[test]
fn mixed_tails_require_the_declared_licence() {
    let text =
        "Put one of them into your hand and the rest on the bottom of your library in any order.";
    assert!(
        common::readings_with_lexicon(&unlicensed_lexicon(), text, Category::Document).is_empty()
    );
    // Existing parallel marked-NP tails keep their structural concord.
    assert!(
        !common::readings_with_lexicon(
            &unlicensed_lexicon(),
            "Put one of them into your hand and the rest into your graveyard.",
            Category::Document
        )
        .is_empty()
    );
}

#[test]
fn nonfinal_locative_tail_selects_its_manner_complement() {
    let text = "Put one of them on top of your library in any order and the rest on the bottom of your library.";
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty());
    let mut found = false;
    for value in values {
        value.visit(&mut |node| {
            if let Reading::SelectedComplementClustersPredicate { tail, .. } = node {
                let Reading::Coordination { left, right, .. } = tail.as_ref() else {
                    panic!("expected tail coordination")
                };
                let Reading::ObjectLocativeMannerTail { complement, manner, .. } = left.as_ref() else {
                    panic!("expected non-final selected Manner Complement")
                };
                assert_eq!(complement.category(), Category::LocativeComplement);
                assert_eq!(manner.realize(common::lexicon()).unwrap(), "any order");
                assert!(matches!(right.as_ref(), Reading::ObjectLocativeTail { .. }));
                found = true;
            }
        }).unwrap();
    }
    assert!(found);
}

#[test]
fn marked_np_tail_does_not_select_manner() {
    assert!(common::readings(
        "Put one into your hand in any order and the rest on the bottom of your library.",
        Category::Document,
    ).is_empty());
}

#[test]
fn attested_nonfinal_manner_tails_select_complete_frames() {
    for (card, text) in [
        ("Munda, Ambush Leader", "Rally — Whenever Munda or another Ally you control enters, you may look at the top four cards of your library. If you do, reveal any number of Ally cards from among them, then put those cards on top of your library in any order and the rest on the bottom in any order."),
        ("Ransack", "Look at the top five cards of target player's library. Put any number of them on the bottom of that library in any order and the rest on top of the library in any order."),
    ] {
        let values = common::readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "unread witness: {card}");
        for value in values {
            let mut found = false;
            value.visit(&mut |node| {
                if let Reading::SelectedComplementClustersPredicate { tail, .. } = node {
                    let Reading::Coordination { left, right, .. } = tail.as_ref() else {
                        panic!("expected tail coordination")
                    };
                    for coordinate in [left, right] {
                        let Reading::ObjectLocativeMannerTail { complement, manner, .. } = coordinate.as_ref() else {
                            panic!("each Locative tail selects its Manner Complement")
                        };
                        assert_eq!(complement.category(), Category::LocativeComplement);
                        assert_eq!(manner.realize(common::lexicon()).unwrap(), "any order");
                    }
                    found = true;
                }
            }).unwrap();
            assert!(found);
        }
    }
}
