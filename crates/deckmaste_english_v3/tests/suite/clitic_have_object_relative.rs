use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

use crate::common;

fn lexical(word: &Word) -> &LexicalValue {
    let LexicalReading::Word(value) = &word.value else {
        panic!("expected a declared Word");
    };
    value
}

fn frame(word: &Word) -> &Frame {
    &common::lexicon().lexemes()[&lexical(word).lexeme]
        .properties
        .frames[word.frame.expect("selected Verb Frame")]
}

fn transitive_frame() -> Frame {
    Frame {
        kind: "Predicate".into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation: Relation::Object,
            category: "NounPhrase".into(),
        })],
    }
}

fn perfect_frame() -> Frame {
    Frame {
        kind: "Auxiliary".into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation: Relation::Complement,
            category: "PastParticiplePredicate".into(),
        })],
    }
}

fn contains(reading: &Reading, predicate: impl Fn(&Reading) -> bool) -> bool {
    let mut found = false;
    reading.visit(&mut |node| found |= predicate(node)).unwrap();
    found
}

fn assert_perfect_relative(reading: &Reading, subject_owner: &str, participle_owner: &str) {
    assert!(contains(reading, |node| {
        let Reading::ZeroObjectRelativeClause {
            subject, predicate, ..
        } = node
        else {
            return false;
        };
        let subject_ok = contains(subject, |node| {
            let Reading::NominativePronoun { head, .. } = node else { return false };
            lexical(head).lexeme.as_str() == subject_owner
        });
        subject_ok
            && contains(predicate, |node| {
                let Reading::AuxiliaryObjectGap {
                    head, complement, ..
                } = node
                else {
                    return false;
                };
                assert_eq!(lexical(head).lexeme.as_str(), "core-verb:HaveContracted");
                assert_eq!(lexical(head).features.finiteness, Some(Finiteness::Finite));
                assert_eq!(frame(head), &perfect_frame());
                // The existing Clitic Host policy licenses this Subject
                // pronoun; exact realization checks its bound
                // separator as well.
                assert_eq!(
                    common::lexicon().lexemes()[&lexical(head).lexeme]
                        .properties
                        .features["Clitic"],
                    "Yes"
                );
                contains(complement, |node| {
                    let Reading::PastParticipleObjectGap { head, .. } = node else {
                        return false;
                    };
                    lexical(head).lexeme.as_str() == participle_owner
                        && lexical(head).form == WordForm::PastParticiple
                        && frame(head) == &transitive_frame()
                })
            })
    }));
    assert!(!contains(reading, |node| matches!(
        node,
        Reading::AdverbialGapRelativeClause { .. }
    )));
    reading.admit(common::lexicon()).unwrap();
}

#[test]
fn ticket_witnesses_have_a_perfect_predicate_with_an_object_gap() {
    for (card, text, subject, participle) in [
        (
            "Aetherflux Reservoir",
            "Whenever you cast a spell, you gain 1 life for each spell you've cast this turn.",
            "vocab:SubjectPronoun/You",
            "lexeme:keyword_action/cast",
        ),
        (
            "Kydele, Chosen of Kruphix",
            "{T}: Add {C} for each card you've drawn this turn.",
            "vocab:SubjectPronoun/You",
            "core-verb:Draw",
        ),
        (
            "Greven, Predator Captain",
            "Greven gets +X/+0, where X is the amount of life you've lost this turn.",
            "vocab:SubjectPronoun/You",
            "core-verb:Lose",
        ),
        (
            "Knowledge Is Power",
            "Creatures you control get +X/+X, where X is the number of cards you've drawn this turn.",
            "vocab:SubjectPronoun/You",
            "core-verb:Draw",
        ),
        (
            "Rug of Smothering",
            "Whenever a player casts a spell, they lose 1 life for each spell they've cast this turn.",
            "vocab:SubjectPronoun/They",
            "lexeme:keyword_action/cast",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        for reading in &readings {
            assert_perfect_relative(reading, subject, participle);
        }
    }
}

#[test]
fn strong_have_retains_lexical_and_perfect_relative_analyses() {
    // Greven's attested NP with the strong auxiliary control (CH3).
    let readings = common::readings("the life you have lost this turn", Category::NounPhrase);
    assert!(readings.iter().any(|reading| contains(reading, |node| {
        let Reading::AuxiliaryObjectGap {
            head, complement, ..
        } = node
        else {
            return false;
        };
        lexical(head).lexeme.as_str() == "core-verb:Have"
            && frame(head) == &perfect_frame()
            && contains(complement, |node| {
                matches!(node,
                Reading::PastParticipleObjectGap { head, .. }
                    if lexical(head).lexeme.as_str() == "core-verb:Lose")
            })
    })));
    assert!(readings.iter().any(|reading| {
        contains(reading, |node| matches!(node, Reading::FiniteObjectGap { head, .. }
            if lexical(head).lexeme.as_str() == "core-verb:Have" && frame(head) == &transitive_frame()))
        && contains(reading, |node| matches!(node, Reading::PassivePredicate { head, .. }
            if lexical(head).lexeme.as_str() == "core-verb:Lose"))
    }));
}

#[test]
fn hollow_one_shares_one_auxiliary_over_coordinated_participle_gaps() {
    let readings = common::readings(
        "each card you've cycled or discarded this turn",
        Category::NounPhrase,
    );
    assert!(!readings.is_empty());
    let shared = |reading: &Reading| {
        contains(reading, |node| {
            let Reading::AuxiliaryObjectGap {
                head, complement, ..
            } = node
            else {
                return false;
            };
            assert_eq!(lexical(head).lexeme.as_str(), "core-verb:HaveContracted");
            let Reading::Coordination {
                category,
                left,
                coordinator,
                right,
                ..
            } = complement.as_ref()
            else {
                return false;
            };
            assert_eq!(*category, Category::PastParticipleObjectGap);
            assert_eq!(lexical(coordinator).lexeme.as_str(), "vocab:Coordinator/Or");
            let (
                Reading::PastParticipleObjectGap { head: left, .. },
                Reading::PastParticipleObjectGap { head: right, .. },
            ) = (left.as_ref(), right.as_ref())
            else {
                return false;
            };
            assert_eq!(lexical(left).lexeme.as_str(), "core-verb:Cycle");
            assert_eq!(
                lexical(right).lexeme.as_str(),
                "lexeme:keyword_action/discard"
            );
            assert_eq!(frame(left), &transitive_frame());
            assert_eq!(frame(right), &transitive_frame());
            true
        })
    };
    // The final verb also has a preterite form. Its finite-coordinate
    // Reading remains grammatical alongside the shared perfect under the
    // retained-Reading contract.
    let finite = |reading: &Reading| {
        contains(reading, |node| {
            let Reading::Coordination {
                category,
                left,
                coordinator,
                right,
                ..
            } = node
            else {
                return false;
            };
            if *category != Category::FiniteObjectGap {
                return false;
            }
            assert_eq!(lexical(coordinator).lexeme.as_str(), "vocab:Coordinator/Or");
            contains(left, |node| {
                matches!(node, Reading::AuxiliaryObjectGap { head, complement, .. }
                if lexical(head).lexeme.as_str() == "core-verb:HaveContracted"
                && matches!(complement.as_ref(), Reading::PastParticipleObjectGap { head, .. }
                    if lexical(head).lexeme.as_str() == "core-verb:Cycle"))
            }) && contains(right, |node| {
                let Reading::FiniteObjectGap { head, .. } = node else { return false };
                assert_eq!(
                    lexical(head).lexeme.as_str(),
                    "lexeme:keyword_action/discard"
                );
                assert_eq!(lexical(head).form, WordForm::Preterite);
                assert_eq!(frame(head), &transitive_frame());
                true
            })
        })
    };
    assert!(
        readings.iter().any(shared),
        "shared auxiliary over both Object gaps"
    );
    assert!(
        readings.iter().any(finite),
        "perfect and preterite Predicate coordination"
    );
    for reading in &readings {
        assert!(shared(reading) || finite(reading));
        assert_perfect_relative(reading, "vocab:SubjectPronoun/You", "core-verb:Cycle");
    }
}

fn word(owner: &str, form: WordForm, features: FeatureBundle, selected: Option<Frame>) -> Word {
    let owner_id = deckmaste_lexical::LexemeId::new(owner);
    let lexeme = &common::lexicon().lexemes()[&owner_id];
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: selected.map(|frame| {
            lexeme
                .properties
                .frames
                .iter()
                .position(|f| *f == frame)
                .unwrap()
        }),
        countability: None,
    }
}

fn independently_built_relative(contracted: bool) -> Reading {
    Reading::ZeroObjectRelativeClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::NominativePronoun {
                category: Category::NounPhrase,
                form: 0,
                head: word(
                    "vocab:SubjectPronoun/You",
                    WordForm::Invariant,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(Person::Second),
                        case: Some(Case::Nominative),
                        ..Default::default()
                    },
                    None,
                ),
            }),
        }),
        predicate: Box::new(Reading::AuxiliaryObjectGap {
            form: 1,
            head: word(
                if contracted { "core-verb:HaveContracted" } else { "core-verb:Have" },
                WordForm::Present,
                FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Second),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    ..Default::default()
                },
                Some(perfect_frame()),
            ),
            complement: Box::new(Reading::PastParticipleObjectGap {
                form: 0,
                head: word(
                    "core-verb:Draw",
                    WordForm::PastParticiple,
                    FeatureBundle {
                        finiteness: Some(Finiteness::Nonfinite),
                        ..Default::default()
                    },
                    Some(transitive_frame()),
                ),
            }),
        }),
    }
}

#[test]
fn independently_built_perfect_gap_obeys_both_laws_and_declared_licences() {
    // Kydele's relative, built independently of parsing.
    for contracted in [false, true] {
        let value = independently_built_relative(contracted);
        value.admit(common::lexicon()).unwrap();
        let text = if contracted { "you've drawn" } else { "you have drawn" };
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let parsed = common::readings(text, Category::ObjectRelativeClause);
        let actual = parsed
            .get(&value)
            .expect("independently built Reading retained");
        let mut expected_words = vec![];
        let mut actual_words = vec![];
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        actual
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);

        let mut wrong = value.clone();
        let Reading::ZeroObjectRelativeClause { predicate, .. } = &mut wrong else {
            unreachable!()
        };
        let Reading::AuxiliaryObjectGap { complement, .. } = predicate.as_mut() else {
            unreachable!()
        };
        **complement = Reading::BareObjectGap {
            form: 0,
            head: word(
                "core-verb:Draw",
                WordForm::Plain,
                FeatureBundle::default(),
                Some(transitive_frame()),
            ),
        };
        assert!(wrong.admit(common::lexicon()).is_err());
        assert!(wrong.realize(common::lexicon()).is_err());
    }
    // The present clitic requires a pronoun which is itself the Subject.
    let mut wrong = independently_built_relative(true);
    let Reading::ZeroObjectRelativeClause { subject, .. } = &mut wrong else {
        unreachable!()
    };
    let Reading::CasePhrase { head, .. } = subject.as_mut() else { unreachable!() };
    **head = Reading::Coordination {
        category: Category::NounPhrase,
        form: 0,
        left: head.clone(),
        right: head.clone(),
        coordinator: word(
            "vocab:Coordinator/And",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        ),
    };
    subject.admit(common::lexicon()).unwrap();
    let Reading::ZeroObjectRelativeClause { predicate, .. } = &mut wrong else {
        unreachable!()
    };
    let Reading::AuxiliaryObjectGap { head, .. } = predicate.as_mut() else {
        unreachable!()
    };
    let LexicalReading::Word(auxiliary) = &mut head.value else { unreachable!() };
    auxiliary.features.number = Some(Number::Plural);
    predicate.admit(common::lexicon()).unwrap();
    assert!(wrong.admit(common::lexicon()).is_err());
    assert!(wrong.realize(common::lexicon()).is_err());
}

#[test]
fn gap_free_perfects_and_temporal_relatives_keep_their_roles() {
    for text in [
        "Draw a card if you've cast a spell this turn.",
        "You've cast a spell this turn.",
    ] {
        let readings = common::readings(text, Category::Document);
        assert!(!readings.is_empty());
        for reading in &readings {
            assert!(contains(reading, |node| matches!(
                node,
                Reading::PerfectAuxiliaryPredicate { .. }
            )));
            assert!(!contains(reading, |node| matches!(
                node,
                Reading::AuxiliaryObjectGap { .. }
            )));
        }
    }
    let readings = common::readings(
        "the first time you've cast your commander",
        Category::NounPhrase,
    );
    assert!(!readings.is_empty());
    for reading in &readings {
        assert!(contains(reading, |node| matches!(
            node,
            Reading::AdverbialGapRelativeClause { .. }
        )));
        assert!(!contains(reading, |node| matches!(
            node,
            Reading::PastParticipleObjectGap { .. }
        )));
    }
    let readings = common::readings("the second spell you cast this turn", Category::NounPhrase);
    assert!(!readings.is_empty());
    for reading in &readings {
        assert!(contains(
            reading,
            |node| matches!(node, Reading::FiniteObjectGap { head, .. }
            if lexical(head).lexeme.as_str() == "lexeme:keyword_action/cast")
        ));
        assert!(!contains(reading, |node| matches!(
            node,
            Reading::AuxiliaryObjectGap { .. }
        )));
    }
}
