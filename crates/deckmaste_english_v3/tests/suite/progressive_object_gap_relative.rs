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

fn selected_frame(kind: &str, relation: Relation, category: &str) -> Frame {
    Frame {
        kind: kind.into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation,
            category: category.into(),
        })],
    }
}

fn progressive_frame() -> Frame {
    selected_frame("Auxiliary", Relation::Complement, "ParticipialPredicate")
}

fn transitive_frame() -> Frame {
    selected_frame("Predicate", Relation::Object, "NounPhrase")
}

fn contains(reading: &Reading, predicate: impl Fn(&Reading) -> bool) -> bool {
    let mut found = false;
    reading.visit(&mut |node| found |= predicate(node)).unwrap();
    found
}

fn progressive_gap(reading: &Reading, participle: &str) -> bool {
    let Reading::AuxiliaryObjectGap {
        head, complement, ..
    } = reading
    else {
        return false;
    };
    let Reading::GerundParticipleObjectGap { head: verb, .. } = complement.as_ref() else {
        return false;
    };
    assert_eq!(frame(head), &progressive_frame());
    assert_eq!(lexical(head).features.finiteness, Some(Finiteness::Finite));
    assert_eq!(lexical(verb).form, WordForm::GerundParticiple);
    assert_eq!(frame(verb), &transitive_frame());
    assert_eq!(lexical(verb).lexeme.as_str(), participle);
    true
}

fn has_relative_modifier(reading: &Reading, antecedents: &[&str], participle: &str) -> bool {
    contains(reading, |node| {
        let Reading::ObjectRelativeNominal { head, relative, .. } = node else {
            return false;
        };
        if !antecedents.contains(&head.realize(common::lexicon()).unwrap().as_str()) {
            return false;
        }
        let Reading::ZeroObjectRelativeClause { predicate, .. } = relative.as_ref() else {
            return false;
        };
        progressive_gap(predicate, participle)
    })
}

fn assert_relative_modifier(reading: &Reading, antecedents: &[&str], participle: &str) {
    assert!(has_relative_modifier(reading, antecedents, participle));
    assert!(!contains(reading, |node| matches!(
        node,
        Reading::AdverbialGapRelativeClause { .. }
    )));
    reading.admit(common::lexicon()).unwrap();
}

#[test]
fn ticket_witnesses_link_the_progressive_object_gap_to_the_antecedent() {
    for (card, text, antecedent, participle) in [
        (
            "Wall of Corpses",
            "{B}, Sacrifice this creature: Destroy target creature this creature is blocking.",
            &["creature"][..],
            "core-verb:Block",
        ),
        (
            "Tinder Wall",
            "It deals 2 damage to target creature it's blocking.",
            &["creature"][..],
            "core-verb:Block",
        ),
        (
            "Scorch Spitter",
            "Whenever this creature attacks, it deals 1 damage to the player or planeswalker it's attacking.",
            &["player or planeswalker", "planeswalker"][..],
            "core-verb:Attack",
        ),
        (
            "Astral Confrontation",
            "This spell costs {1} less to cast for each opponent you're attacking.",
            &["opponent"][..],
            "core-verb:Attack",
        ),
        (
            "Wall of Shadows",
            "Prevent all damage that would be dealt to this creature by creatures it's blocking.",
            &["creatures"][..],
            "core-verb:Block",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Document);
        assert!(!readings.is_empty(), "{card}");
        assert!(
            readings
                .iter()
                .any(|reading| has_relative_modifier(reading, antecedent, participle)),
            "{card}: the intended antecedent retains its relative Modifier"
        );
        // Other admitted nominal attachment scopes retain the same Object-gap
        // role.
        for reading in &readings {
            assert!(
                contains(reading, |node| {
                    let Reading::ObjectRelativeNominal { head, relative, .. } = node else {
                        return false;
                    };
                    assert_eq!(head.category(), Category::Nominal);
                    let Reading::ZeroObjectRelativeClause { predicate, .. } = relative.as_ref()
                    else {
                        return false;
                    };
                    progressive_gap(predicate, participle)
                }),
                "{card}: every Reading retains a progressive Object gap on its nominal antecedent"
            );
            reading.admit(common::lexicon()).unwrap();
        }
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

fn independent_relative(second_person: bool, contracted: bool) -> Reading {
    let person = if second_person { Person::Second } else { Person::Third };
    Reading::ZeroObjectRelativeClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::NominativePronoun {
                category: Category::NounPhrase,
                form: 0,
                head: word(
                    if second_person {
                        "vocab:SubjectPronoun/You"
                    } else {
                        "vocab:SubjectPronoun/It"
                    },
                    WordForm::Invariant,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(person),
                        case: Some(Case::Nominative),
                        ..Default::default()
                    },
                    None,
                ),
            }),
        }),
        predicate: Box::new(Reading::AuxiliaryObjectGap {
            form: 2,
            head: word(
                if contracted { "core-verb:BeContracted" } else { "core-verb:Be" },
                WordForm::Present,
                FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(person),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    ..Default::default()
                },
                Some(progressive_frame()),
            ),
            complement: Box::new(Reading::GerundParticipleObjectGap {
                form: 0,
                head: word(
                    "core-verb:Attack",
                    WordForm::GerundParticiple,
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
fn independent_strong_and_clitic_relatives_obey_both_roundtrip_laws() {
    // Astral Confrontation and Scorch Spitter relative constituents; PG1–PG3
    // license the corresponding strong forms through the same Construction.
    for (second_person, contracted, text) in [
        (false, false, "it is attacking"),
        (false, true, "it's attacking"),
        (true, false, "you are attacking"),
        (true, true, "you're attacking"),
    ] {
        let value = independent_relative(second_person, contracted);
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let parsed = common::readings(text, Category::ObjectRelativeClause);
        assert!(
            parsed.contains(&value),
            "independently constructed relative retained"
        );

        let mut wrong = value.clone();
        let Reading::ZeroObjectRelativeClause { predicate, .. } = &mut wrong else {
            unreachable!()
        };
        let Reading::AuxiliaryObjectGap { complement, .. } = predicate.as_mut() else {
            unreachable!()
        };
        **complement = Reading::PastParticipleObjectGap {
            form: 0,
            head: word(
                "core-verb:Attack",
                WordForm::PastParticiple,
                FeatureBundle {
                    finiteness: Some(Finiteness::Nonfinite),
                    ..Default::default()
                },
                Some(transitive_frame()),
            ),
        };
        assert!(
            wrong.admit(common::lexicon()).is_err(),
            "past participle cannot fill progressive Object gap"
        );
        assert!(wrong.realize(common::lexicon()).is_err());
    }
    // The 're Clitic requires a pronoun which is itself the Subject.
    let mut wrong = independent_relative(true, true);
    let Reading::ZeroObjectRelativeClause {
        subject, predicate, ..
    } = &mut wrong
    else {
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
fn strong_phrase_subjects_and_that_relatives_share_the_object_gap_route() {
    for text in [
        "the player it's attacking",
        "the player it is attacking",
        "the player that creature is attacking",
    ] {
        let readings = common::readings(text, Category::NounPhrase);
        assert_eq!(readings.len(), 1, "PG4: one Object-gap relative analysis");
        for reading in &readings {
            assert_relative_modifier(reading, &["player"], "core-verb:Attack");
        }
    }
    // PG1 licenses relative that with the same progressive Object gap.
    let readings = common::readings(
        "that this creature is blocking",
        Category::ObjectRelativeClause,
    );
    assert!(!readings.is_empty());
    for reading in &readings {
        let Reading::ObjectRelativeClause { predicate, .. } = reading else {
            panic!("expected marked Object relative");
        };
        assert!(progressive_gap(predicate, "core-verb:Block"));
    }
}

#[test]
fn subject_gaps_passives_and_ungapped_progressives_keep_their_roles() {
    for (text, subject_relative, passive) in [
        (
            "Destroy target creature that is blocking this creature.",
            true,
            false,
        ),
        (
            "Destroy target creature blocking this creature.",
            false,
            false,
        ),
        ("This creature is blocking a creature.", false, false),
        ("that is blocked", true, true),
    ] {
        let category = if text == "that is blocked" {
            Category::SubjectRelativeClause
        } else {
            Category::Document
        };
        let readings = common::readings(text, category);
        // PG4 explicitly requires the Subject-gap control to retain one
        // analysis.
        if subject_relative && !passive {
            assert_eq!(readings.len(), 1);
        }
        assert!(!readings.is_empty());
        for reading in &readings {
            assert_eq!(
                contains(reading, |node| matches!(
                    node,
                    Reading::SubjectRelativeClause { .. }
                )),
                subject_relative
            );
            if passive {
                let Reading::SubjectRelativeClause { predicate, .. } = reading else {
                    panic!("expected Subject relative");
                };
                let verbal = contains(predicate, |node| {
                    let Reading::ParticipialAuxiliaryPredicate { head, .. } = node else {
                        return false;
                    };
                    frame(head) == &progressive_frame()
                }) && contains(predicate, |node| {
                    let Reading::PassivePredicate { head, .. } = node else {
                        return false;
                    };
                    lexical(head).lexeme.as_str() == "core-verb:Block"
                        && lexical(head).form == WordForm::PastParticiple
                        && frame(head) == &transitive_frame()
                });
                let adjectival = contains(predicate, |node| {
                    let Reading::SelectedPredicate { head, .. } = node else {
                        return false;
                    };
                    frame(head)
                        == &selected_frame(
                            "Predicate",
                            Relation::Complement,
                            "PredicativeComplement",
                        )
                }) && contains(predicate, |node| {
                    let Reading::IntransitiveAdjective { head, .. } = node else {
                        return false;
                    };
                    lexical(head).lexeme.as_str() == "core-verb:Block/adjective"
                        && lexical(head).form == WordForm::Invariant
                });
                assert!(
                    verbal || adjectival,
                    "blocked retains its declared verbal or adjectival Subject-relative analysis"
                );
            } else {
                assert!(!contains(reading, |node| matches!(
                    node,
                    Reading::PassivePredicate { .. }
                )));
            }
            assert!(!contains(reading, |node| matches!(
                node,
                Reading::GerundParticipleObjectGap { .. }
            )));
        }
    }
    for text in [
        "Destroy target creature this creature blocked.",
        "Destroy target creature this creature blocks.",
        "It deals 1 damage to the player it attacked.",
        "Destroy target creature you've attacked.",
    ] {
        let readings = common::readings(text, Category::Document);
        assert!(!readings.is_empty());
        for reading in &readings {
            assert!(contains(reading, |node| matches!(
                node,
                Reading::ObjectRelativeNominal { .. }
            )));
            assert!(!contains(reading, |node| matches!(
                node,
                Reading::GerundParticipleObjectGap { .. }
            )));
        }
    }
    for text in [
        "the player it is attack",
        "the player it is attacked",
        "the player it is attacking you",
        "the player it is attacking with",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}
