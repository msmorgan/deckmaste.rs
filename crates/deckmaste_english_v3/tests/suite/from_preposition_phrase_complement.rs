use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading};

fn nested_source(text: &str, expected_sources: &[&str]) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {text:?}");
    eprintln!("witness {text:?}: {} Readings", values.len());
    let mut observed_sources = std::collections::BTreeSet::new();
    for value in values {
        let mut sources = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::PrepositionComplementPreposition {
                    head, complement, ..
                } = node
                    && common::lexicon().realize(&head.value).unwrap() == "from"
                {
                    sources.push(complement.realize(common::lexicon()).unwrap());
                }
            })
            .unwrap();
        assert_eq!(sources.len(), 1, "one complete source PP in {value:?}");
        assert!(
            expected_sources.contains(&sources[0].as_str()),
            "unexpected source PP in {value:?}"
        );
        observed_sources.insert(sources[0].clone());
        value.admit(common::lexicon()).unwrap();
    }
    assert_eq!(
        observed_sources,
        expected_sources
            .iter()
            .map(|text| (*text).to_owned())
            .collect()
    );
}

#[test]
fn ostrich_horse_and_satyr_wayfinder_keep_the_nested_source() {
    nested_source(
        "You may put a land card from among them into your hand.",
        &["among them"],
    );
}

#[test]
fn animal_magnetism_keeps_the_nested_source() {
    nested_source(
        "An opponent chooses a creature card from among them.",
        &["among them"],
    );
}

#[test]
fn seek_the_wilds_keeps_the_nested_source() {
    nested_source(
        "You may reveal a creature or land card from among them and put it into your hand.",
        &["among them"],
    );
}

#[test]
fn tomakul_scrapsmith_keeps_the_nested_source() {
    // The participial Postmodifier can qualify the selected artifact card or
    // the internal plural cards; both grammatical attachments are retained.
    nested_source(
        "You may put an artifact card from among the cards milled this way into your hand.",
        &["among the cards", "among the cards milled this way"],
    );
}

#[test]
fn midnight_tilling_source_constituent_survives_its_independent_nominal_gap() {
    // The source-free control also fails: permanent lacks noun Premodifier use.
    // Keep this extra witness localized to its source PP; it is not a face gain.
    assert!(common::readings("a permanent card", Category::NounPhrase).is_empty());
    let expected = Reading::PrepositionComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/From"),
        complement: Box::new(among_them()),
    };
    assert_eq!(
        common::readings("from among them", Category::PrepositionPhrase),
        std::collections::BTreeSet::from([expected])
    );
}

#[test]
fn bag_of_devouring_keeps_every_source_in_the_declared_verb_slot() {
    let text =
        "return up to X cards from among cards exiled with this artifact to their owners' hands";
    let values = common::readings(text, Category::SecondaryVerbPhrase);
    assert!(!values.is_empty());
    for value in values {
        let mut sources = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate { complements, .. } = node {
                    for slot in complements {
                        if let deckmaste_english_v3::grammar::FrameValue::Argument(argument) = slot
                            && argument.category() == Category::SourceComplement
                        {
                            sources.push(argument.realize(common::lexicon()).unwrap());
                        }
                    }
                }
            })
            .unwrap();
        assert_eq!(
            sources,
            vec!["from among cards exiled with this artifact"],
            "wrong source attachment in {value:?}"
        );
    }
}

fn invariant(id: &str) -> deckmaste_english_v3::grammar::Word {
    deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Word(deckmaste_lexical::LexicalValue {
            lexeme: id.into(),
            form: deckmaste_lexical::WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            variant: 0,
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn among_them() -> Reading {
    let mut them = invariant("vocab:ObjectPronoun/Them");
    let deckmaste_lexical::LexicalReading::Word(word) = &mut them.value else {
        unreachable!()
    };
    word.features = deckmaste_lexical::FeatureBundle {
        number: Some(deckmaste_lexical::Number::Plural),
        person: Some(deckmaste_lexical::Person::Third),
        case: Some(deckmaste_lexical::Case::Accusative),
        ..Default::default()
    };
    Reading::PrepositionPhrase {
        form: 0,
        head: invariant("vocab:Preposition/Among"),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(Reading::AccusativePronoun {
                form: 0,
                head: them,
            }),
        }),
    }
}

#[test]
fn independently_constructed_source_pp_preserves_the_complete_value() {
    let expected = Reading::PrepositionComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/From"),
        complement: Box::new(among_them()),
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "from among them"
    );
    assert_eq!(
        common::readings("from among them", Category::PrepositionPhrase),
        std::collections::BTreeSet::from([expected.clone()]),
    );
    let mut words = Vec::new();
    expected
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    assert_eq!(
        words,
        vec![
            invariant("vocab:Preposition/From"),
            invariant("vocab:Preposition/Among"),
            {
                let Reading::PrepositionPhrase { complement, .. } = among_them() else {
                    unreachable!()
                };
                let Reading::CasePhrase { head, .. } = *complement else { unreachable!() };
                let Reading::AccusativePronoun { head, .. } = *head else { unreachable!() };
                head
            }
        ]
    );
    // Both governing and complement-head permissions participate in admission.
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let among = lexemes
        .iter_mut()
        .find(|word| word.id.as_str() == "vocab:Preposition/Among")
        .unwrap();
    among
        .properties
        .features
        .insert("SourceComplementUse".into(), "No".into());
    let unlicensed = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert!(expected.admit(&unlicensed).is_err());
    assert!(
        common::readings_with_lexicon(&unlicensed, "from among them", Category::PrepositionPhrase)
            .is_empty()
    );
    let invalid = Reading::PrepositionComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/Into"),
        complement: Box::new(among_them()),
    };
    assert!(invalid.admit(common::lexicon()).is_err());
}

#[test]
fn avatar_destiny_keeps_its_coordinated_tails_and_checks_their_object_features() {
    // Avatar Destiny's final instruction, localized to its shared verb.
    let text = "return this card to its owner's hand and up to one creature card milled this way to the battlefield under your control";
    let values = common::readings(text, Category::SecondaryVerbPhrase);
    let mut checked = 0;
    for value in values {
        let Reading::SelectedComplementClustersPredicate { head, tail, .. } = &value else {
            continue;
        };
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let Reading::Coordination { left, .. } = tail.as_ref() else {
            panic!("missing coordinated tail")
        };
        let Reading::ObjectPrepositionTail { object, .. } = left.as_ref() else {
            panic!("missing Object tail")
        };
        assert_eq!(object.realize(common::lexicon()).unwrap(), "this card");
        assert_eq!(head.frame, Some(1));
        let mut invalid = value.clone();
        let Reading::SelectedComplementClustersPredicate { tail, .. } = &mut invalid else {
            unreachable!()
        };
        let Reading::Coordination { left, .. } = tail.as_mut() else { unreachable!() };
        let Reading::ObjectPrepositionTail { object, .. } = left.as_mut() else {
            unreachable!()
        };
        **object = common::readings("this card from them", Category::AccusativePhrase)
            .into_iter()
            .next()
            .unwrap();
        assert!(
            invalid.admit(common::lexicon()).is_err(),
            "segment admission ignored the declared Object feature"
        );
        checked += 1;
    }
    assert!(checked > 0, "no shared Complement-tail Reading");
}

#[test]
fn independently_constructed_return_checks_its_projected_object_feature() {
    use deckmaste_english_v3::grammar::FrameValue;
    use deckmaste_lexical::{LexicalReading, WordForm};
    // Avatar Destiny's first coordinated instruction is also an admitted VP.
    let mut head = invariant("core-verb:Return");
    let LexicalReading::Word(word) = &mut head.value else { unreachable!() };
    word.form = WordForm::Plain;
    head.frame = Some(1);
    let object = common::readings("this card", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    let destination = common::readings("its owner's hand", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    let expected = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head,
        complements: vec![
            FrameValue::Argument(Box::new(object)),
            FrameValue::Marker(invariant("vocab:Preposition/To")),
            FrameValue::Argument(Box::new(destination)),
        ],
    };
    expected.admit(common::lexicon()).unwrap();
    let text = "return this card to its owner's hand";
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, Category::SecondaryVerbPhrase).contains(&expected));
    let mut invalid = expected;
    let Reading::SelectedPredicate { complements, .. } = &mut invalid else {
        unreachable!()
    };
    complements[0] = FrameValue::Argument(Box::new(
        common::readings("this card from them", Category::AccusativePhrase)
            .into_iter()
            .next()
            .unwrap(),
    ));
    assert!(
        invalid.admit(common::lexicon()).is_err(),
        "constructed-value admission ignored the declared Object feature"
    );
}
