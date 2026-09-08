use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_english_v3::parse;
use deckmaste_english_v3::slice::grammar::Category;
use deckmaste_english_v3::slice::grammar::Grammar;
use deckmaste_english_v3::slice::grammar::Reading;
use deckmaste_english_v3::slice::grammar::Value;
use deckmaste_english_v3::slice::grammar::Word;
use deckmaste_english_v3::slice::lexicon;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn word(id: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn nominal(id: &str, number: Number) -> Reading {
    let mut head = word(
        id,
        if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
        FeatureBundle {
            number: Some(number),
            ..FeatureBundle::default()
        },
    );
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

fn basic(id: &str, number: Number, determined: bool) -> Reading {
    let nominal = Box::new(nominal(id, number));
    if determined {
        Reading::DeterminedNounPhrase {
            form: 0,
            determiner: word(
                if number == Number::Singular {
                    "determinative:one"
                } else {
                    "determinative:two"
                },
                WordForm::Invariant,
                FeatureBundle {
                    number: Some(number),
                    ..FeatureBundle::default()
                },
            ),
            nominal,
        }
    } else {
        Reading::BarePlural { form: 0, nominal }
    }
}

fn np(head: Reading, modifier: Option<Reading>) -> Reading {
    Reading::NounPhrase {
        form: 0,
        head: Box::new(head),
        modifier: modifier.map(Box::new),
    }
}

fn adjunct() -> Reading {
    let mut head = word(
        "preposition:with",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    head.frame = Some(0);
    Reading::Adjunct {
        form: 0,
        phrase: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head,
            complement: Box::new(basic("noun:counter", Number::Singular, true)),
        }),
    }
}

fn subject(id: &str, number: Number, person: Person) -> Reading {
    Reading::Subject {
        form: 0,
        head: word(
            id,
            WordForm::Invariant,
            FeatureBundle {
                number: Some(number),
                person: Some(person),
                case: Some(Case::Nominative),
                ..FeatureBundle::default()
            },
        ),
    }
}

fn finite_head(id: &str, number: Number, person: Person, tense: Tense) -> Word {
    let mut head = word(
        id,
        if tense == Tense::Present { WordForm::Present } else { WordForm::Preterite },
        FeatureBundle {
            number: Some(number),
            person: Some(person),
            tense: Some(tense),
            finiteness: Some(Finiteness::Finite),
            case: None,
        },
    );
    head.frame = Some(0);
    head
}

fn finite(subject: Reading, head: Word, complement: Reading, modifier: Option<Reading>) -> Reading {
    Reading::FiniteClause {
        form: 0,
        subject: Box::new(subject),
        predicate: Box::new(Reading::FinitePredicate {
            form: 0,
            head,
            complement: Box::new(complement),
            modifier: modifier.map(Box::new),
        }),
    }
}

fn imperative(complement: Reading, modifier: Option<Reading>) -> Reading {
    let mut head = word(
        "verb:cast",
        WordForm::Plain,
        FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..FeatureBundle::default()
        },
    );
    head.frame = Some(0);
    Reading::Imperative {
        form: 0,
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head,
            complement: Box::new(complement),
            modifier: modifier.map(Box::new),
        }),
    }
}

fn sentence(clause: Reading) -> Reading {
    Reading::Sentence {
        form: 0,
        clause: Box::new(clause),
    }
}

fn census(lexicon: &Lexicon, text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &category).unwrap();
    let mut iterator = forest.readings(&grammar);
    assert_eq!(iterator.metrics().builds, 0);
    let mut values = BTreeSet::new();
    for value in iterator.by_ref() {
        let Value::Reading(value) = value.unwrap() else { panic!("non-Reading root") };
        assert_eq!(value.category(), category);
        assert_eq!(value.realize(lexicon).unwrap(), text);
        assert!(values.insert(value));
    }
    assert_eq!(iterator.metrics().internal_failures, 0);
    assert_eq!(iterator.metrics().cyclic_derivations, 0);
    values
}

fn assert_family(lexicon: &Lexicon, values: impl IntoIterator<Item = Reading>) -> usize {
    let mut surfaces: BTreeMap<(Category, String), BTreeSet<Reading>> = BTreeMap::new();
    for value in values {
        let text = value.realize(lexicon).unwrap();
        surfaces
            .entry((value.category(), text))
            .or_default()
            .insert(value);
    }
    let count = surfaces.values().map(BTreeSet::len).sum();
    for ((category, text), expected) in surfaces {
        assert_eq!(census(lexicon, &text, category), expected, "{text:?}");
    }
    count
}

#[test]
fn interacting_tense_agreement_complement_and_attachment_readings_are_exact() {
    let lexicon = lexicon().unwrap();
    let complement = basic("noun:spell", Number::Singular, true);
    let mut expected = BTreeSet::new();
    for number in [Number::Singular, Number::Plural] {
        for tense in [Tense::Present, Tense::Past] {
            for noun_attachment in [false, true] {
                expected.insert(finite(
                    subject("pronoun:you", number, Person::Second),
                    finite_head("verb:cast", number, Person::Second, tense),
                    np(complement.clone(), noun_attachment.then(adjunct)),
                    (!noun_attachment).then(adjunct),
                ));
            }
        }
    }
    assert_eq!(
        census(
            &lexicon,
            "you cast one spell with one counter",
            Category::Clause
        ),
        expected
    );
    let imperative_readings = [false, true].map(|noun_attachment| {
        imperative(
            np(complement.clone(), noun_attachment.then(adjunct)),
            (!noun_attachment).then(adjunct),
        )
    });
    assert_eq!(
        census(
            &lexicon,
            "cast one spell with one counter",
            Category::Clause
        ),
        BTreeSet::from(imperative_readings)
    );
    let past_only = finite(
        subject("pronoun:it", Number::Singular, Person::Third),
        finite_head("verb:cast", Number::Singular, Person::Third, Tense::Past),
        np(complement, None),
        None,
    );
    assert_eq!(
        census(&lexicon, "it cast one spell", Category::Clause),
        BTreeSet::from([past_only])
    );
}

#[test]
fn homographs_multiword_overlap_and_duplicate_derivations_keep_their_identities() {
    let lexicon = lexicon().unwrap();
    assert_eq!(
        census(&lexicon, "cast", Category::Nominal),
        BTreeSet::from([nominal("noun:cast", Number::Singular)])
    );
    assert_eq!(
        census(&lexicon, "one", Category::Nominal),
        BTreeSet::from([nominal("noun:one", Number::Singular)])
    );
    assert_eq!(
        census(&lexicon, "one spell", Category::NounPhrase),
        BTreeSet::from([np(basic("noun:spell", Number::Singular, true), None)])
    );
    let one = lexicon.analyze("one");
    assert!(
        one.matches
            .iter()
            .any(|m| matches!(m.reading, LexicalReading::Numeral { value: 1, .. }))
    );
    let modified = Reading::PremodifiedNominal {
        form: 0,
        modifier: word(
            "adjective:first",
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
        head: match nominal("noun:strike", Number::Singular) {
            Reading::Noun { head, .. } => head,
            _ => unreachable!(),
        },
    };
    assert_eq!(
        census(&lexicon, "first strike", Category::Nominal),
        BTreeSet::from([nominal("noun:first-strike", Number::Singular), modified])
    );
    let grammar = Grammar::default();
    let forest = parse(&grammar, &lexicon, &one, &Category::Nominal).unwrap();
    let mut readings = forest.readings(&grammar);
    assert_eq!(
        readings.by_ref().collect::<Result<Vec<_>, _>>().unwrap(),
        [Value::Reading(nominal("noun:one", Number::Singular))]
    );
    assert_eq!(readings.metrics().derivations, 2);
    assert_eq!(readings.metrics().duplicates, 1);
}

#[test]
fn wrong_agreement_frames_and_jointly_invalid_bundles_are_rejected() {
    let lexicon = lexicon().unwrap();
    // "draw" contains singular and third-person alternatives, but never
    // a singular third-person finite alternative. Marginal unions are unsound.
    let analyses = lexicon.analyze("draw");
    let bundles: Vec<_> = analyses
        .matches
        .iter()
        .filter_map(|m| match &m.reading {
            LexicalReading::Word(w) if w.form == WordForm::Present => Some(&w.features),
            _ => None,
        })
        .collect();
    assert!(bundles.iter().any(|f| f.number == Some(Number::Singular)));
    assert!(bundles.iter().any(|f| f.person == Some(Person::Third)));
    assert!(
        !bundles
            .iter()
            .any(|f| f.number == Some(Number::Singular) && f.person == Some(Person::Third))
    );
    for text in [
        "it draw one spell",
        "they draws one spell",
        "you casts one spell",
        "casts one spell",
        "walk one spell",
        "cast one spells",
        "cast two spell",
        "cast spell",
        "cast",
        "one cast one spell",
    ] {
        assert!(
            census(&lexicon, text, Category::Clause).is_empty(),
            "{text}"
        );
    }
    let invalid = finite(
        subject("pronoun:it", Number::Singular, Person::Third),
        finite_head("verb:draw", Number::Plural, Person::Third, Tense::Present),
        np(basic("noun:spell", Number::Singular, true), None),
        None,
    );
    assert!(invalid.admit(&lexicon).is_err());
    assert!(invalid.realize(&lexicon).is_err());
    let valid = finite(
        subject("pronoun:it", Number::Singular, Person::Third),
        finite_head("verb:draw", Number::Singular, Person::Third, Tense::Present),
        np(basic("noun:spell", Number::Singular, true), None),
        None,
    );
    assert_eq!(
        census(&lexicon, "it draws one spell", Category::Clause),
        BTreeSet::from([valid])
    );
}

#[test]
fn every_declared_finite_bundle_spelling_and_capitalization_roundtrips_independently() {
    let lexicon = lexicon().unwrap();
    let mut expected = vec![];
    // Enumerate declared values without lexical analysis or parser output.
    // This family fixes the Complement and varies every available subject,
    // compatible finite verb, spelling variant and capitalization.
    for subject_value in lexicon
        .values()
        .filter(|w| lexicon.lexemes()[&w.lexeme].category == deckmaste_lexical::Category::Pronoun)
    {
        for verb_value in lexicon
            .values()
            .filter(|w| w.features.finiteness == Some(Finiteness::Finite))
        {
            if verb_value.features.person != subject_value.features.person
                || verb_value.features.number != subject_value.features.number
            {
                continue;
            }
            if lexicon.lexemes()[&verb_value.lexeme].properties.frames[0].kind != "transitive" {
                continue;
            }
            expected.push(finite(
                Reading::Subject {
                    form: 0,
                    head: Word {
                        value: LexicalReading::Word(subject_value.clone()),
                        frame: None,
                        countability: None,
                    },
                },
                Word {
                    value: LexicalReading::Word(verb_value.clone()),
                    frame: Some(0),
                    countability: None,
                },
                np(basic("noun:spell", Number::Singular, true), None),
                None,
            ));
        }
    }
    let count = assert_family(&lexicon, expected);
    assert_eq!(count, 112);
    println!("finite bundle/spelling/capitalization family: {count} values");
}

fn nominal_values(lexicon: &Lexicon) -> Vec<Reading> {
    let mut expected = vec![];
    for value in lexicon
        .values()
        .filter(|w| lexicon.lexemes()[&w.lexeme].category == deckmaste_lexical::Category::Noun)
    {
        let head = Word {
            value: LexicalReading::Word(value.clone()),
            frame: None,
            countability: Some(true),
        };
        expected.push(Reading::Noun {
            form: 0,
            head: head.clone(),
        });
        for modifier in lexicon.values().filter(|w| {
            lexicon.lexemes()[&w.lexeme].category == deckmaste_lexical::Category::Adjective
        }) {
            expected.push(Reading::PremodifiedNominal {
                form: 0,
                modifier: Word {
                    value: LexicalReading::Word(modifier.clone()),
                    frame: None,
                    countability: None,
                },
                head: head.clone(),
            });
        }
    }
    expected
}

#[test]
fn all_nominal_values_and_surface_orders_preserve_lexical_and_structural_traversal() {
    let lexicon = lexicon().unwrap();
    assert_eq!(assert_family(&lexicon, nominal_values(&lexicon)), 72);
    let header = nominal("noun:cast", Number::Singular);
    let object = basic("noun:one", Number::Singular, true);
    let clause = imperative(np(object.clone(), None), Some(adjunct()));
    let sent = sentence(clause.clone());
    let document = Reading::HeadedDocument {
        form: 0,
        header: Box::new(header.clone()),
        sentences: vec![sent.clone()],
    };
    assert_eq!(
        document.realize(&lexicon).unwrap(),
        "cast\ncast one one with one counter."
    );
    assert_eq!(
        census(
            &lexicon,
            "cast\ncast one one with one counter.",
            Category::Document
        )
        .len(),
        2
    );
    let mut nodes = vec![];
    document.visit(&mut |r| nodes.push(r.clone())).unwrap();
    let Reading::Imperative { predicate, .. } = &clause else { unreachable!() };
    let Reading::BarePredicate {
        complement,
        modifier,
        ..
    } = predicate.as_ref()
    else {
        unreachable!()
    };
    let Reading::DeterminedNounPhrase {
        nominal: object_nominal,
        ..
    } = &object
    else {
        unreachable!()
    };
    let modifier = modifier.as_ref().unwrap();
    let Reading::Adjunct { phrase, .. } = modifier.as_ref() else { unreachable!() };
    let Reading::PrepositionPhrase {
        complement: pp_object,
        ..
    } = phrase.as_ref()
    else {
        unreachable!()
    };
    let Reading::DeterminedNounPhrase {
        nominal: pp_nominal,
        ..
    } = pp_object.as_ref()
    else {
        unreachable!()
    };
    assert_eq!(
        nodes,
        vec![
            document.clone(),
            header,
            sent,
            clause.clone(),
            *predicate.clone(),
            *complement.clone(),
            object.clone(),
            *object_nominal.clone(),
            *modifier.clone(),
            *phrase.clone(),
            *pp_object.clone(),
            *pp_nominal.clone()
        ]
    );
    let mut words = vec![];
    document
        .visit_words(&mut |w| words.push(w.clone()))
        .unwrap();
    let expected_words = [
        "noun:cast",
        "verb:cast",
        "determinative:one",
        "noun:one",
        "preposition:with",
        "determinative:one",
        "noun:counter",
    ];
    assert_eq!(words.len(), expected_words.len());
    for (word, id) in words.iter().zip(expected_words) {
        let LexicalReading::Word(value) = &word.value else {
            panic!("unexpected numeral")
        };
        assert_eq!(value.lexeme, id);
        assert_eq!(lexicon.lexemes()[id].source.owner, id);
    }
    let parsed = census(
        &lexicon,
        &document.realize(&lexicon).unwrap(),
        Category::Document,
    );
    assert!(parsed.contains(&document));
    let mut parsed_words = vec![];
    parsed
        .get(&document)
        .unwrap()
        .visit_words(&mut |w| parsed_words.push(w.clone()))
        .unwrap();
    assert_eq!(parsed_words, words);
}

#[test]
fn finite_document_family_checks_both_laws_and_boundary_rejection() {
    let lexicon = lexicon().unwrap();
    let clauses = [Tense::Present, Tense::Past].map(|tense| {
        sentence(finite(
            subject("pronoun:they", Number::Plural, Person::Third),
            finite_head("verb:cast", Number::Plural, Person::Third, tense),
            np(basic("noun:spell", Number::Plural, false), None),
            None,
        ))
    });
    let mut expected = vec![];
    for length in 0..=4 {
        for bits in 0..(1 << length) {
            let sentences: Vec<_> = (0..length)
                .map(|i| clauses[(bits >> i) & 1].clone())
                .collect();
            expected.push(Reading::Document {
                form: 0,
                sentences: sentences.clone(),
            });
            expected.push(Reading::HeadedDocument {
                form: 0,
                header: Box::new(nominal("noun:cast", Number::Singular)),
                sentences,
            });
        }
    }
    assert_eq!(assert_family(&lexicon, expected), 62);
    for text in [
        "they cast spells",
        "they cast spells.\n",
        "they cast spells.they cast spells.",
        "they cast spells.\n\nthey cast spells.",
        "cast they cast spells.",
    ] {
        assert!(
            census(&lexicon, text, Category::Document).is_empty(),
            "{text:?}"
        );
    }
}

#[test]
fn packed_inspection_and_bounded_requests_do_not_enumerate_the_document() {
    let lexicon = lexicon().unwrap();
    let grammar = Grammar::default();
    for length in [4, 16, 64] {
        let text = vec!["they cast spells."; length].join("\n");
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(&text),
            &Category::Document,
        )
        .unwrap();
        let mut iterator = forest.readings(&grammar);
        let mut dump = Vec::new();
        forest.write_packed(&mut dump).unwrap();
        assert!(!dump.is_empty());
        assert_eq!(iterator.metrics().builds, 0);
        let first = iterator.next().unwrap().unwrap();
        let second = iterator.next().unwrap().unwrap();
        assert_ne!(first, second);
        for reading in [first, second] {
            let Value::Reading(reading) = reading else { panic!("non-Reading") };
            assert_eq!(reading.realize(&lexicon).unwrap(), text);
        }
        assert_eq!(iterator.metrics().readings, 2);
        assert_eq!(iterator.metrics().internal_failures, 0);
        assert!(forest.metrics().items < 150 * length);
        assert!(forest.metrics().families < 200 * length);
        println!(
            "slice document {length}: {:?}; {:?}",
            forest.metrics(),
            iterator.metrics()
        );
    }
}

#[test]
fn recognition_consumes_supplied_alternatives_without_rescanning() {
    let lexicon = lexicon().unwrap();
    let grammar = Grammar::default();
    let mut input = lexicon.analyze("they cast spells");
    input.matches.retain(|m| !matches!(&m.reading, LexicalReading::Word(w) if w.features.tense == Some(Tense::Present)));
    let forest = parse(&grammar, &lexicon, &input, &Category::Clause).unwrap();
    let expected = finite(
        subject("pronoun:they", Number::Plural, Person::Third),
        finite_head("verb:cast", Number::Plural, Person::Third, Tense::Past),
        np(basic("noun:spell", Number::Plural, false), None),
        None,
    );
    assert_eq!(
        grammar
            .readings(&forest)
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        [expected]
    );
    input.matches.retain(
        |m| !matches!(&m.reading, LexicalReading::Word(w) if w.features.tense == Some(Tense::Past)),
    );
    let forest = parse(&grammar, &lexicon, &input, &Category::Clause).unwrap();
    assert!(forest.roots().is_empty());
}

#[test]
fn finite_complement_families_cover_every_nominal_and_both_optional_attachment_sites() {
    let lexicon = lexicon().unwrap();
    let mut basics = vec![];
    for nominal in nominal_values(&lexicon) {
        let (Reading::Noun { head, .. } | Reading::PremodifiedNominal { head, .. }) = &nominal
        else {
            unreachable!()
        };
        let LexicalReading::Word(value) = &head.value else { unreachable!() };
        let number = value.features.number.unwrap();
        if number == Number::Plural {
            basics.push(Reading::BarePlural {
                form: 0,
                nominal: Box::new(nominal.clone()),
            });
        }
        for determiner in lexicon.values().filter(|w| {
            lexicon.lexemes()[&w.lexeme].category == deckmaste_lexical::Category::Determinative
                && w.features.number == Some(number)
        }) {
            basics.push(Reading::DeterminedNounPhrase {
                form: 0,
                determiner: Word {
                    value: LexicalReading::Word(determiner.clone()),
                    frame: None,
                    countability: None,
                },
                nominal: Box::new(nominal.clone()),
            });
        }
    }
    assert_eq!(assert_family(&lexicon, basics.clone()), 180);
    let mut prepositions = vec![];
    for basic in &basics {
        for head in lexicon.values().filter(|w| {
            lexicon.lexemes()[&w.lexeme].category == deckmaste_lexical::Category::Preposition
        }) {
            prepositions.push(Reading::PrepositionPhrase {
                form: 0,
                head: Word {
                    value: LexicalReading::Word(head.clone()),
                    frame: Some(0),
                    countability: None,
                },
                complement: Box::new(basic.clone()),
            });
        }
    }
    assert_eq!(assert_family(&lexicon, prepositions), 360);
    let mut clauses = vec![];
    for basic in basics {
        for nominal_attachment in [false, true] {
            for predicate_attachment in [false, true] {
                clauses.push(imperative(
                    np(basic.clone(), nominal_attachment.then(adjunct)),
                    predicate_attachment.then(adjunct),
                ));
            }
        }
    }
    assert_eq!(assert_family(&lexicon, clauses), 720);
}
