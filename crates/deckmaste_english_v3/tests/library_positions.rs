mod common;

use common::{assert_constituents, lexicon, readings};
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{FrameItem, Relation};
use deckmaste_lexical::{LexicalReading, SurfaceCase, WordForm};

#[test]
fn look_selects_library_cards_with_internal_position_and_of_modifier() {
    // Commune with Evil.
    let text = "look at the top four cards of your library";
    let values = readings(text, Category::SecondaryVerbPhrase);
    assert!(!values.is_empty(), "no library-position Reading");
    for value in &values {
        let Reading::SelectedPredicate {
            head, complements, ..
        } = value
        else {
            panic!("library position must fill Look's selected frame: {value:?}");
        };
        let [FrameValue::Marker(marker), FrameValue::Argument(object)] = complements.as_slice()
        else {
            panic!("unexpected Look slots: {complements:?}")
        };
        let LexicalReading::Word(marker) = &marker.value else {
            panic!("nonword marker")
        };
        assert_eq!(marker.lexeme.as_str(), "vocab:Preposition/At");
        assert_eq!(object.category(), Category::AccusativePhrase);
        assert_eq!(
            object.realize(lexicon()).unwrap(),
            "the top four cards of your library"
        );
        assert!(head.frame.is_some());
        let LexicalReading::Word(head_value) = &head.value else {
            panic!("nonword head")
        };
        let frame = &lexicon().lexemes()[&head_value.lexeme].properties.frames[head.frame.unwrap()];
        let FrameItem::Argument(slot) = &frame.items[1] else {
            panic!("missing selected slot")
        };
        assert_eq!(slot.relation, Relation::Complement);
    }
    assert_constituents(
        text,
        Category::SecondaryVerbPhrase,
        &[
            (Category::AdjectivePhrase, "top"),
            (Category::Nominal, "four cards"),
            (Category::NounPhrase, "the top four cards of your library"),
            (Category::PrepositionPhrase, "of your library"),
        ],
    );
}

#[test]
fn reveal_and_exile_select_library_position_objects() {
    // Winding Way; Reckless Impulse.
    for (text, object_text) in [
        (
            "reveal the top four cards of your library",
            "the top four cards of your library",
        ),
        (
            "exile the top two cards of your library",
            "the top two cards of your library",
        ),
    ] {
        let values = readings(text, Category::SecondaryVerbPhrase);
        assert!(!values.is_empty());
        for value in values {
            let Reading::SelectedPredicate {
                head, complements, ..
            } = &value
            else {
                panic!("missing selected object: {value:?}");
            };
            let [FrameValue::Argument(object)] = complements.as_slice() else {
                panic!("unexpected object slots: {value:?}");
            };
            assert_eq!(object.category(), Category::AccusativePhrase);
            assert_eq!(object.realize(lexicon()).unwrap(), object_text);
            let LexicalReading::Word(head_value) = &head.value else {
                panic!("nonword head")
            };
            let frame =
                &lexicon().lexemes()[&head_value.lexeme].properties.frames[head.frame.unwrap()];
            let FrameItem::Argument(slot) = &frame.items[0] else {
                panic!("missing object slot")
            };
            assert_eq!(slot.relation, Relation::Object);
        }
    }
}

#[test]
fn put_consumes_its_order_tail_in_every_retained_reading() {
    // Psychic Surgery; Harald, King of Skemfar.
    for text in [
        "put the rest on top of that library in any order",
        "put the rest on the bottom of your library in a random order",
    ] {
        let values = readings(text, Category::SecondaryVerbPhrase);
        assert!(!values.is_empty(), "no order-tail Reading for {text}");
        for value in values {
            let Reading::SelectedPredicate { complements, .. } = &value else {
                panic!("order must be selected by Put: {value:?}");
            };
            let [
                FrameValue::Argument(_),
                FrameValue::Argument(destination),
                FrameValue::Marker(marker),
                FrameValue::Argument(order),
            ] = complements.as_slice()
            else {
                panic!("missing order slots: {value:?}")
            };
            assert_eq!(destination.category(), Category::LocativeComplement);
            let LexicalReading::Word(marker) = &marker.value else {
                panic!("nonword marker")
            };
            assert_eq!(marker.lexeme.as_str(), "vocab:Preposition/In");
            assert_eq!(order.category(), Category::MannerComplement);
            assert!(text.ends_with(&order.realize(lexicon()).unwrap()));
        }
    }
    assert!(
        readings(
            "look at the top two cards of that library in any order",
            Category::SecondaryVerbPhrase,
        )
        .is_empty(),
        "selected manner tail escaped into an unlicensed frame or NP modifier"
    );
}

fn word(owner: &str, form: WordForm, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(
            lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        countability: (form == WordForm::Singular).then_some(true),
        frame,
    }
}

#[test]
fn independent_position_pp_retains_noun_complement_and_both_roundtrip_laws() {
    // Psychic Surgery's destination constituent.
    let library = Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(
            "vocab:SingularDemonstrative/That",
            WordForm::Invariant,
            None,
        ),
        head: Box::new(Reading::Noun {
            form: 0,
            head: word("lexeme:CommonNoun/Library", WordForm::Singular, None),
        }),
    };
    let expected = Reading::BareNominalPreposition {
        form: 0,
        head: word("vocab:Preposition/On", WordForm::Invariant, None),
        complement: Box::new(Reading::BarePrepositionNounPhrase {
            form: 0,
            head: Box::new(Reading::PrepositionComplementNominal {
                form: 0,
                head: word("lexeme:CommonNoun/Top", WordForm::Singular, Some(1)),
                marker: word("vocab:Preposition/Of", WordForm::Invariant, None),
                complement: Box::new(Reading::CasePhrase {
                    form: 0,
                    category: Category::AccusativePhrase,
                    head: Box::new(library),
                }),
            }),
        }),
    };
    let text = "on top of that library";
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    let actual = readings(text, Category::PrepositionPhrase);
    assert_eq!(actual, std::collections::BTreeSet::from([expected.clone()]));
    let mut expected_nodes = Vec::new();
    let mut actual_nodes = Vec::new();
    let mut expected_words = Vec::new();
    let mut actual_words = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    actual
        .first()
        .unwrap()
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
    assert_eq!(actual_words, expected_words);
    for text in [
        "Exile top of that library.",
        "Top of that library draws cards.",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "bare count NP escaped its PP"
        );
    }
}

#[test]
fn put_selects_a_position_destination_instead_of_a_free_adjunct() {
    // Totally Lost and Hide.
    for (text, destination) in [
        (
            "put target nonland permanent on top of its owner's library",
            "on top of its owner's library",
        ),
        (
            "put target artifact or enchantment on the bottom of its owner's library",
            "on the bottom of its owner's library",
        ),
    ] {
        let values = readings(text, Category::SecondaryVerbPhrase);
        assert!(!values.is_empty(), "missing destination for {text}");
        for value in &values {
            let Reading::SelectedPredicate { complements, .. } = value else {
                panic!("destination must fill a selected slot: {value:?}");
            };
            assert!(
                complements.iter().any(|slot| match slot {
                    FrameValue::Argument(child) =>
                        child.category() == Category::LocativeComplement
                            && child.realize(lexicon()).unwrap() == destination,
                    _ => false,
                }),
                "missing selected destination: {value:?}"
            );
        }
    }
}

#[test]
fn selected_destination_preserves_passive_and_object_gap_readings() {
    // Fathom Mage; Melira's Keepers.
    for text in [
        "Whenever a +1/+1 counter is put on this creature, you may draw a card.",
        "This creature can't have counters put on it.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "lost passive for {text}"
        );
    }
}

#[test]
fn stand_together_retains_coordinated_object_and_locative_complements() {
    let text = "Put two +1/+1 counters on target creature and two +1/+1 counters on another target creature.";
    let values = readings(text, Category::Document);
    assert_eq!(values.len(), 6);
    let mut clusters = Vec::new();
    for value in &values {
        value
            .visit(&mut |node| {
                if matches!(node, Reading::SelectedComplementClustersPredicate { .. }) {
                    clusters.push(node.clone());
                }
            })
            .unwrap();
    }
    assert_eq!(
        clusters.len(),
        1,
        "Stand Together lost its complement-cluster Reading"
    );
    let Reading::SelectedComplementClustersPredicate { head, tail, .. } = &clusters[0] else {
        unreachable!()
    };
    assert_eq!(head.frame, Some(1));
    assert_eq!(tail.category(), Category::SelectedComplementTail);
    let Reading::Coordination { left, right, .. } = tail.as_ref() else {
        panic!("missing parallel complement clusters: {tail:?}");
    };
    for (coordinate, destination) in [
        (left, "on target creature"),
        (right, "on another target creature"),
    ] {
        let Reading::ObjectLocativeTail {
            object, complement, ..
        } = coordinate.as_ref()
        else {
            panic!("missing Object + Locative Complement coordinate: {coordinate:?}");
        };
        assert_eq!(object.category(), Category::AccusativePhrase);
        assert_eq!(object.realize(lexicon()).unwrap(), "two +1/+1 counters");
        assert_eq!(complement.category(), Category::LocativeComplement);
        assert_eq!(complement.realize(lexicon()).unwrap(), destination);
    }
    assert_constituents(
        text,
        Category::Document,
        &[
            (
                Category::SelectedComplementTail,
                "two +1/+1 counters on target creature",
            ),
            (
                Category::SelectedComplementTail,
                "two +1/+1 counters on another target creature",
            ),
            (Category::LocativeComplement, "on target creature"),
            (Category::LocativeComplement, "on another target creature"),
        ],
    );
}
