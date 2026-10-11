use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, Finiteness, Frame, FrameItem, FrameSlot, LexicalReading, LexicalValue, Number,
    Person, Relation, SurfaceCase, Tense, WordForm,
};

use crate::common;

fn mana_frame() -> Frame {
    Frame {
        kind: "Predicate".into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation: Relation::Complement,
            category: "ManaPhrase".into(),
        })],
    }
}

fn assert_passive_frame(head: &Word) {
    let LexicalReading::Word(word) = &head.value else { panic!("expected a verb") };
    assert_eq!(word.form, WordForm::PastParticiple);
    assert_eq!(
        common::lexicon().lexemes()[&word.lexeme].properties.frames[head.frame.unwrap()],
        mana_frame()
    );
}

#[test]
fn witnesses_keep_mana_subject_and_existing_purpose_attachments() {
    for (text, symbols) in [
        (
            "When this creature enters, sacrifice it unless {U} was spent to cast it.",
            "{U}",
        ),
        ("If {U} was spent to cast this spell, draw a card.", "{U}"),
        (
            "When this creature enters, if {B} was spent to cast it, target player discards a card.",
            "{B}",
        ),
        (
            "When you cast this spell, if {C} wasn't spent to cast it, target opponent may put a creature card from their hand onto the battlefield.",
            "{C}",
        ),
        (
            "If {U}{B} was spent to cast this spell, search your library for a card, put that card into your graveyard, then shuffle.",
            "{U}{B}",
        ),
        ("Draw a card if {G} was spent to cast this spell.", "{G}"),
    ] {
        let readings = common::readings(text, Category::Document);
        assert!(!readings.is_empty(), "{text}");
        // W9-A preserves independently available attachments on the noun control.
        let control = text.replace(symbols, "no mana");
        assert_eq!(
            readings.len(),
            common::readings(&control, Category::Document).len(),
            "{text}"
        );
        for reading in readings {
            let mut subjects = Vec::new();
            let mut purposes = Vec::new();
            let mut heads = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::FiniteClause {
                        subject, predicate, ..
                    } = node
                        && let Reading::ManaSubject { head, .. } = subject.as_ref()
                    {
                        assert_eq!(head.category(), Category::ManaPhrase);
                        assert_eq!(predicate.category(), Category::FiniteManaPassivePredicate);
                        subjects.push(head.realize(common::lexicon()).unwrap());
                    }
                    if let Reading::SelectedVerbHead {
                        category: Category::ManaPassivePredicate,
                        head,
                        ..
                    } = node
                    {
                        assert_passive_frame(head);
                        heads.push(head.clone());
                    }
                    if let Reading::PurposePredicate { modifier, .. } = node {
                        assert_eq!(modifier.category(), Category::PurposeInfinitival);
                        purposes.push(modifier.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
            assert_eq!(subjects, vec![symbols]);
            assert!(!heads.is_empty(), "{text}");
            assert!(
                purposes
                    .iter()
                    .any(|purpose| purpose.starts_with("to cast ")),
                "{text}"
            );
        }
    }
}

#[test]
fn the_simple_conditional_keeps_both_passive_purpose_attachment_heights() {
    let mut hosts = BTreeSet::new();
    for reading in common::readings(
        "If {U} was spent to cast this spell, draw a card.",
        Category::Document,
    ) {
        reading
            .visit(&mut |node| {
                if let Reading::PurposePredicate {
                    category, modifier, ..
                } = node
                {
                    assert_eq!(modifier.category(), Category::PurposeInfinitival);
                    hosts.insert(*category);
                }
            })
            .unwrap();
    }
    assert_eq!(
        hosts,
        BTreeSet::from([
            Category::ManaPassivePredicate,
            Category::FiniteManaPassivePredicate
        ])
    );
}

#[test]
fn passive_licence_belongs_to_the_declared_frame_class() {
    // MP3 explicitly licences all declaring verbs, including unattested passives.
    for text in ["spent", "added", "produced"] {
        let readings = common::readings(text, Category::ManaPassivePredicate);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            let Reading::SelectedVerbHead { head, .. } = reading else {
                panic!("expected the selected head")
            };
            assert_passive_frame(&head);
        }
    }
    assert_eq!(
        common::readings("paid", Category::ManaPassivePredicate),
        BTreeSet::new()
    );
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let spend = lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "lexeme:Verb/Spend")
        .unwrap();
    spend
        .properties
        .frames
        .retain(|frame| frame != &mana_frame());
    let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert_eq!(
        common::readings_with_lexicon(
            &lexicon,
            "If {U} was spent to cast this spell, draw a card.",
            Category::Document
        ),
        BTreeSet::new()
    );
    for text in [
        "If no mana was spent to cast it, draw a card.",
        "Mana from a Treasure was spent to cast it.",
        "You spend mana.",
    ] {
        assert_eq!(
            common::readings_with_lexicon(&lexicon, text, Category::Document),
            common::readings(text, Category::Document)
        );
    }
}

#[test]
fn spend_selects_the_mana_complement_without_an_object_projection() {
    let readings = common::readings("spend {U}", Category::SecondaryVerbPhrase);
    assert!(!readings.is_empty());
    for reading in readings {
        let Reading::SelectedPredicate {
            head, complements, ..
        } = reading
        else {
            panic!("expected the selected predicate")
        };
        let LexicalReading::Word(word) = &head.value else { panic!("expected a verb") };
        assert_eq!(
            common::lexicon().lexemes()[&word.lexeme].properties.frames[head.frame.unwrap()],
            mana_frame()
        );
        let [FrameValue::Argument(complement)] = complements.as_slice() else {
            panic!("expected the Mana Complement")
        };
        assert_eq!(complement.category(), Category::ManaPhrase);
        assert_eq!(complement.realize(common::lexicon()).unwrap(), "{U}");
    }
    assert!(!common::readings("You spend {U}.", Category::Document).is_empty());
}

#[test]
fn agreement_and_symbol_boundaries_remain_governed() {
    for text in [
        "{U}{B} were spent.",
        "{U} was paid.",
        "{X} was spent.",
        "{2} was spent.",
        "{S} was spent.",
        "{T} was spent.",
    ] {
        assert_eq!(
            common::readings(text, Category::Document),
            BTreeSet::new(),
            "{text}"
        );
    }
    for text in ["{U}", "{U}{B}", "{C}", "{X}", "{2}"] {
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::new(),
            "{text}"
        );
    }
}

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_constructed_passive_preserves_both_roundtrip_laws() {
    // The subordinate clause of Ribbons of Night, built without parsing.
    let subject = Reading::ManaSubject {
        form: 0,
        head: Box::new(Reading::SymbolSequence {
            category: Category::ManaPhrase,
            form: 0,
            first: Box::new(Reading::NamedManaSymbol {
                form: 0,
                symbol: word(
                    "vocab:FixedCostSymbol/Blue",
                    WordForm::Invariant,
                    FeatureBundle::default(),
                ),
            }),
            rest: vec![],
        }),
    };
    let mut spent = word(
        "lexeme:Verb/Spend",
        WordForm::PastParticiple,
        FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..Default::default()
        },
    );
    spent.frame = Some(
        common::lexicon().lexemes()["lexeme:Verb/Spend"]
            .properties
            .frames
            .iter()
            .position(|frame| frame == &mana_frame())
            .unwrap(),
    );
    let mut was = word(
        "core-verb:Be",
        WordForm::Preterite,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(Tense::Past),
            finiteness: Some(Finiteness::Finite),
            case: None,
        },
    );
    was.frame = Some(
        common::lexicon().lexemes()["core-verb:Be"]
            .properties
            .frames
            .iter()
            .position(|frame| {
                frame
                    == &Frame {
                        kind: "Auxiliary".into(),
                        items: vec![FrameItem::Argument(FrameSlot {
                            relation: Relation::Complement,
                            category: "ParticipialPredicate".into(),
                        })],
                    }
            })
            .unwrap(),
    );
    let expected = Reading::FiniteClause {
        form: 2,
        subject: Box::new(subject),
        predicate: Box::new(Reading::ParticipialAuxiliaryPredicate {
            category: Category::FiniteManaPassivePredicate,
            form: 0,
            head: was,
            complement: Box::new(Reading::OvertComplement {
                category: Category::ManaPassiveComplement,
                form: 0,
                predicate: Box::new(Reading::SelectedVerbHead {
                    category: Category::ManaPassivePredicate,
                    form: 0,
                    head: spent,
                }),
            }),
        }),
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "{U} was spent"
    );
    assert_eq!(
        common::readings("{U} was spent", Category::FiniteClause),
        BTreeSet::from([expected])
    );
}
