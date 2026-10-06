use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_lexical::{
    Category as Pos, Frame, FrameItem, FrameSlot, Lexeme, LexicalReading, Lexicon, Relation,
    Source, SourceKind, SurfaceCase, WordForm,
};

constructions! {
    pub mod grammar {
        category NounPhrase();
        category Amount();
        category Segment();
        category Predicate(form);
        construction NounPhrase: NounPhrase {
            form [determiner: lexical(Determinative), " ", noun: lexical(Noun)];
        }
        construction Amount: Amount { form [number: lexical(Numeral)]; require number.numeral_kind = Arabic; }
        construction Segment: Segment {
            form [object: NounPhrase, " ", marker: lexical(Preposition), " ", destination: NounPhrase];
            segment Predicate;
        }
        construction Pair: Segment {
            form [left: Segment, " ", coordinator: lexical(Coordinator), " ", right: Segment];
            share_segments left, right;
        }
        construction Host: Predicate {
            form [head: lexical(Verb), " ", tail: Segment];
            require head.form = Plain;
            discharge_segments head, tail;
            export form = head.form;
        }
    }
}

fn source(owner: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "frame-segment-witness".into(),
        owner: owner.into(),
    }
}

fn frame(first: &str, marker: &str, last: &str) -> Frame {
    Frame {
        kind: "Predicate".into(),
        items: vec![
            FrameItem::Argument(FrameSlot {
                relation: Relation::Object,
                category: first.into(),
            }),
            FrameItem::Marker {
                vocabulary: "Preposition".into(),
                member: marker.into(),
            },
            FrameItem::Argument(FrameSlot {
                relation: Relation::Object,
                category: last.into(),
            }),
        ],
    }
}

fn lexicon() -> Lexicon {
    let mut entries = vec![];
    for (category, words) in [
        (Pos::Noun, vec!["card", "battlefield", "rest", "graveyard"]),
        (Pos::Determinative, vec!["that", "the", "your"]),
        (Pos::Coordinator, vec!["and"]),
    ] {
        for word in words {
            entries.push(Lexeme::invariant(word, word, category, source(word)));
        }
    }
    for (word, member) in [("onto", "Onto"), ("into", "Into"), ("to", "To")] {
        let mut lexeme = Lexeme::invariant(word, word, Pos::Preposition, source(word));
        lexeme
            .properties
            .features
            .insert("FrameMarker".into(), format!("Preposition/{member}"));
        entries.push(lexeme);
    }
    let mut put = Lexeme::verb("put", "put", source("put"));
    put.properties.frames = vec![
        frame("NounPhrase", "Into", "NounPhrase"),
        frame("NounPhrase", "Onto", "NounPhrase"),
        frame("Amount", "Onto", "NounPhrase"),
        frame("NounPhrase", "To", "Amount"),
    ];
    entries.push(put);
    // Same surface and individually licensed markers, but different slot relations.
    // Their segment candidates must stay correlated until the shared head discharges them.
    let mut mixed = Lexeme::verb("put-mixed-role", "put", source("put-mixed-role"));
    let onto = frame("NounPhrase", "Onto", "NounPhrase");
    let mut into = frame("NounPhrase", "Into", "NounPhrase");
    let FrameItem::Argument(slot) = &mut into.items[2] else { unreachable!() };
    slot.relation = Relation::Complement;
    mixed.properties.frames = vec![onto, into];
    entries.push(mixed);
    Lexicon::new(entries).unwrap()
}

fn word(lexicon: &Lexicon, id: &str, form: WordForm, frame: Option<usize>) -> grammar::Word {
    grammar::Word {
        value: LexicalReading::Word(
            lexicon
                .values()
                .find(|value| {
                    value.lexeme == id
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame,
        countability: None,
    }
}

fn np(lexicon: &Lexicon, determiner: &str, noun: &str) -> grammar::Reading {
    grammar::Reading::NounPhrase {
        form: 0,
        determiner: word(lexicon, determiner, WordForm::Invariant, None),
        noun: word(lexicon, noun, WordForm::Invariant, None),
    }
}

fn segment(
    lexicon: &Lexicon,
    determiner: &str,
    noun: &str,
    marker: &str,
    destination: &str,
) -> grammar::Reading {
    grammar::Reading::Segment {
        form: 0,
        object: Box::new(np(lexicon, determiner, noun)),
        marker: word(lexicon, marker, WordForm::Invariant, None),
        destination: Box::new(np(
            lexicon,
            if destination == "battlefield" { "the" } else { "your" },
            destination,
        )),
    }
}

fn expected(lexicon: &Lexicon) -> grammar::Reading {
    grammar::Reading::Host {
        form: 0,
        head: word(lexicon, "put", WordForm::Plain, Some(1)),
        tail: Box::new(grammar::Reading::Pair {
            form: 0,
            left: Box::new(segment(lexicon, "that", "card", "onto", "battlefield")),
            coordinator: word(lexicon, "and", WordForm::Invariant, None),
            right: Box::new(segment(lexicon, "the", "rest", "into", "graveyard")),
        }),
    }
}

#[test]
fn independent_segment_discharge_preserves_correlated_frames_and_roundtrip_laws() {
    // The destination-sharing constituent of Animal Magnetism.
    let text = "put that card onto the battlefield and the rest into your graveyard";
    let lexicon = lexicon();
    let environment = grammar::GrammarEnvironment::new(&lexicon);
    let expected = expected(&lexicon);
    assert_eq!(environment.realize(&expected).unwrap(), text);
    let forest = environment
        .parse(text, grammar::Category::Predicate)
        .unwrap();
    let values: BTreeSet<_> = environment
        .grammar()
        .readings(&forest)
        .map(Result::unwrap)
        .collect();
    assert_eq!(values, BTreeSet::from([expected.clone()]));
    let mut nodes = vec![];
    let mut words = vec![];
    environment
        .visit(&expected, &mut |node| nodes.push(node.clone()))
        .unwrap();
    environment
        .visit_words(&expected, &mut |word| words.push(word.clone()))
        .unwrap();
    let parsed = values.first().unwrap();
    let mut parsed_nodes = vec![];
    let mut parsed_words = vec![];
    environment
        .visit(parsed, &mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    environment
        .visit_words(parsed, &mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(nodes, parsed_nodes);
    assert_eq!(words, parsed_words);
    assert!(forest.metrics().items < 500);
    assert!(forest.metrics().families < 500);
}

#[test]
fn wrong_arity_order_marker_and_undischarged_segments_never_become_readings() {
    let lexicon = lexicon();
    let environment = grammar::GrammarEnvironment::new(&lexicon);
    let valid = expected(&lexicon);
    for frame in [0, 2, 3] {
        let mut invalid = valid.clone();
        let grammar::Reading::Host { head, .. } = &mut invalid else { unreachable!() };
        head.frame = Some(frame);
        assert!(environment.admit(&invalid).is_err());
        assert!(environment.realize(&invalid).is_err());
    }
    let mut mismatched_relations = valid.clone();
    let grammar::Reading::Host { head, .. } = &mut mismatched_relations else {
        unreachable!()
    };
    *head = word(&lexicon, "put-mixed-role", WordForm::Plain, Some(0));
    assert!(environment.admit(&mismatched_relations).is_err());
    let mut invalid = valid.clone();
    let grammar::Reading::Host { tail, .. } = &mut invalid else { unreachable!() };
    let grammar::Reading::Pair { right, .. } = tail.as_mut() else {
        unreachable!()
    };
    let grammar::Reading::Segment { marker, .. } = right.as_mut() else {
        unreachable!()
    };
    *marker = word(&lexicon, "to", WordForm::Invariant, None);
    assert!(environment.admit(&invalid).is_err());
    for text in [
        "put that card onto the battlefield and the rest to your graveyard",
        "put that card the battlefield onto and the rest into your graveyard",
        "put that card onto and the rest into your graveyard",
        "put that card onto the battlefield and into your graveyard",
        "put that card onto the battlefield and the rest into",
    ] {
        let forest = environment
            .parse(text, grammar::Category::Predicate)
            .unwrap();
        assert_eq!(environment.grammar().readings(&forest).count(), 0, "{text}");
    }
    for text in [
        "that card onto the battlefield",
        "that card onto the battlefield and the rest into your graveyard",
    ] {
        let standalone = segment(&lexicon, "that", "card", "onto", "battlefield");
        assert!(environment.admit(&standalone).is_err());
        assert!(environment.realize(&standalone).is_err());
        let forest = environment.parse(text, grammar::Category::Segment).unwrap();
        assert_eq!(environment.grammar().readings(&forest).count(), 0);
    }
}
