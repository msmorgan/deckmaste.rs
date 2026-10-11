use deckmaste_english_v3::grammar::Category;

use crate::common::assert_constituents;
use crate::common::readings;

fn activation_witness() {
    assert_constituents(
        "Activate only once each turn.",
        Category::Document,
        &[
            (Category::AdverbPhrase, "only once"),
            (Category::AdverbPhrase, "once"),
            (Category::NominalAdjunctPhrase, "each turn"),
        ],
    );
}

#[test]
fn kozileks_translator_frequency_restriction() {
    activation_witness();
}

#[test]
fn jeskai_devotee_frequency_restriction() {
    activation_witness();
}

#[test]
fn basking_rootwalla_frequency_restriction() {
    activation_witness();
}

#[test]
fn twinblade_slasher_frequency_restriction() {
    activation_witness();
}

#[test]
fn ondu_spiritdancer_frequency_restriction() {
    assert_constituents(
        "Do this only once each turn.",
        Category::Document,
        &[
            (Category::AdverbPhrase, "only once"),
            (Category::AdverbPhrase, "once"),
            (Category::NominalAdjunctPhrase, "each turn"),
            (Category::AccusativePhrase, "this"),
        ],
    );
}

#[test]
fn bounding_frequency_retains_adverb_category() {
    // Attested restrictions: Duggan, Private Detective and Nadu, Winged Wisdom.
    for text in [
        "Activate only once.",
        "This ability triggers only twice each turn.",
    ] {
        assert!(!readings(text, Category::Document).is_empty(), "{text}");
    }
    for text in ["once", "twice"] {
        assert_eq!(readings(text, Category::AdverbPhrase).len(), 1);
        assert!(readings(text, Category::NounPhrase).is_empty());
        assert!(readings(text, Category::FrequencyPhrase).is_empty());
    }
}

fn invariant(owner: &str) -> deckmaste_english_v3::grammar::Word {
    use deckmaste_lexical::FeatureBundle;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::LexicalValue;
    use deckmaste_lexical::SurfaceCase;
    use deckmaste_lexical::WordForm;
    deckmaste_english_v3::grammar::Word {
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
fn independent_frequency_focus_preserves_both_roundtrip_laws_and_traversal() {
    use deckmaste_english_v3::grammar::Reading;
    use deckmaste_lexical::LexicalReading;

    use crate::common::lexicon;

    for (owner, text) in [
        ("vocab:FrequencyAdverb/Once", "only once"),
        ("vocab:FrequencyAdverb/Twice", "only twice"),
    ] {
        let expected = Reading::FocusedPhrase {
            category: Category::AdverbPhrase,
            form: 0,
            modifier: invariant("vocab:FocusAdverb/Only"),
            head: Box::new(Reading::Adverb {
                form: 0,
                head: invariant(owner),
            }),
        };
        expected.admit(lexicon()).unwrap();
        assert_eq!(expected.realize(lexicon()).unwrap(), text);
        let parsed = readings(text, Category::AdverbPhrase);
        assert_eq!(parsed, [expected.clone()].into_iter().collect());
        let mut expected_nodes = Vec::new();
        expected
            .visit(&mut |node| expected_nodes.push(node.clone()))
            .unwrap();
        let mut actual_nodes = Vec::new();
        parsed
            .first()
            .unwrap()
            .visit(&mut |node| actual_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(actual_nodes, expected_nodes);
        let mut actual_words = Vec::new();
        parsed
            .first()
            .unwrap()
            .visit_words(&mut |word| {
                let LexicalReading::Word(value) = &word.value else { panic!("non-word leaf") };
                actual_words.push(value.lexeme.to_string());
            })
            .unwrap();
        assert_eq!(actual_words, ["vocab:FocusAdverb/Only", owner]);
        let mut wrong = expected.clone();
        let Reading::FocusedPhrase { modifier, .. } = &mut wrong else {
            unreachable!()
        };
        *modifier = invariant(owner);
        assert!(wrong.admit(lexicon()).is_err());
        assert!(wrong.realize(lexicon()).is_err());
        let Reading::FocusedPhrase { modifier, head, .. } = &mut wrong else {
            unreachable!()
        };
        *modifier = invariant("vocab:FocusAdverb/Only");
        **head = Reading::Adverb {
            form: 0,
            head: invariant("vocab:Adverb/Then"),
        };
        assert!(wrong.admit(lexicon()).is_err());
        assert!(wrong.realize(lexicon()).is_err());
    }
}

#[test]
fn frequency_and_distributive_temporal_np_remain_separate_adjuncts() {
    use deckmaste_english_v3::grammar::FrameValue;
    use deckmaste_english_v3::grammar::Reading;

    use crate::common::lexicon;

    for text in [
        "Activate only once each turn.",
        "Do this only once each turn.",
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty());
        for value in values {
            let mut frequencies = 0;
            let mut intervals = 0;
            value
                .visit(&mut |node| {
                    if let Reading::AdverbPredicate { modifier, .. } = node {
                        assert_eq!(modifier.realize(lexicon()).unwrap(), "only once");
                        frequencies += 1;
                    }
                    if let Reading::NominalAdjunctPredicate { modifier, .. } = node {
                        assert_eq!(modifier.realize(lexicon()).unwrap(), "each turn");
                        intervals += 1;
                    }
                    if let Reading::SelectedPredicate { complements, .. } = node {
                        for complement in complements {
                            if let FrameValue::Argument(child) = complement {
                                assert_ne!(child.realize(lexicon()).unwrap(), "each turn");
                            }
                        }
                    }
                })
                .unwrap();
            assert_eq!((frequencies, intervals), (1, 1));
        }
    }
    for text in [
        "once each turn",
        "only once each turn",
        "twice that many",
        "once a creature dies",
        "only only once",
    ] {
        assert!(readings(text, Category::AdverbPhrase).is_empty(), "{text}");
    }
    assert!(readings("Only once each turn, activate.", Category::Document).is_empty());
}

#[test]
fn remorseless_punishment_unfocused_frequency_retains_the_object() {
    assert_constituents(
        "Repeat this process once.",
        Category::Document,
        &[
            (Category::AccusativePhrase, "this process"),
            (Category::AdverbPhrase, "once"),
        ],
    );
}

#[test]
fn independent_fused_determiner_head_and_transitive_do_preserve_lexical_identity() {
    use deckmaste_english_v3::grammar::FrameValue;
    use deckmaste_english_v3::grammar::Reading;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::WordForm;

    use crate::common::lexicon;

    let nominal = Reading::FusedDeterminerHeadNounPhrase {
        dependent: None,
        form: 0,
        head: invariant("vocab:SingularDemonstrative/This"),
    };
    nominal.admit(lexicon()).unwrap();
    assert_eq!(nominal.realize(lexicon()).unwrap(), "this");
    assert_eq!(
        readings("this", Category::NounPhrase),
        [nominal.clone()].into_iter().collect()
    );
    let mut verb = invariant("core-verb:Do");
    let LexicalReading::Word(value) = &mut verb.value else { unreachable!() };
    value.form = WordForm::Plain;
    verb.frame = Some(2);
    let expected = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: verb,
        complements: vec![FrameValue::Argument(Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(nominal),
        }))],
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "do this");
    assert_eq!(
        readings("do this", Category::SecondaryVerbPhrase),
        [expected].into_iter().collect()
    );
    for text in ["every", "a", "only"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn each_player_who_does_has_one_subject_relative_with_auxiliary_ellipsis() {
    use deckmaste_english_v3::grammar::Reading;
    use deckmaste_lexical::Case;
    use deckmaste_lexical::Finiteness;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::Number;
    use deckmaste_lexical::Person;
    use deckmaste_lexical::Tense;
    use deckmaste_lexical::WordForm;

    use crate::common::lexicon;

    let mut marker = invariant("vocab:RelativePronoun/Who");
    let LexicalReading::Word(value) = &mut marker.value else { unreachable!() };
    value.features.number = Some(Number::Singular);
    value.features.person = Some(Person::Third);
    value.features.case = Some(Case::Nominative);
    let mut auxiliary = invariant("core-verb:Do");
    let LexicalReading::Word(value) = &mut auxiliary.value else { unreachable!() };
    value.form = WordForm::Present;
    value.features.number = Some(Number::Singular);
    value.features.person = Some(Person::Third);
    value.features.tense = Some(Tense::Present);
    value.features.finiteness = Some(Finiteness::Finite);
    auxiliary.frame = Some(1);
    let expected = Reading::PronounSubjectRelative {
        form: 0,
        marker,
        predicate: Box::new(Reading::BareAuxiliaryPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: auxiliary,
            complement: Box::new(Reading::BareEllipsis {
                form: 0,
                omission: Box::new(Reading::OmittedPlain { form: 0 }),
            }),
        }),
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "who does");
    assert_eq!(
        readings("who does", Category::SubjectRelativeClause),
        [expected.clone()].into_iter().collect()
    );
    let values = readings("each player who does", Category::NounPhrase);
    assert_eq!(values.len(), 1);
    let mut relatives = Vec::new();
    values
        .first()
        .unwrap()
        .visit(&mut |node| {
            if node.category() == Category::SubjectRelativeClause {
                relatives.push(node.clone());
            }
        })
        .unwrap();
    assert_eq!(relatives, [expected]);
    assert!(readings("who", Category::NominativePhrase).is_empty());
    assert!(readings("who does", Category::ObjectRelativeClause).is_empty());
}
