mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn structure(reading: &Reading) -> Vec<(Category, &'static str, Vec<LexicalReading>)> {
    let mut nodes = Vec::new();
    reading
        .visit(&mut |node| {
            let mut words = Vec::new();
            node.visit_words(&mut |word| {
                let mut value = word.value.clone();
                if let LexicalReading::Word(value) = &mut value
                    && value.lexeme == "vocab:Preposition/AsLongAs"
                {
                    value.lexeme = "vocab:Preposition/If".into();
                }
                words.push(value);
            })
            .unwrap();
            nodes.push((node.category(), node.construction(), words));
        })
        .unwrap();
    nodes
}

#[test]
fn conditional_witnesses_retain_the_if_twins_readings_and_structure() {
    for (card, text) in [
        (
            "Aeronaut Tinkerer",
            "This creature has flying as long as you control an artifact.",
        ),
        (
            "Kargan Dragonrider",
            "As long as you control a Dragon, this creature has flying.",
        ),
        (
            "Arisen Gorgon",
            "This creature has deathtouch as long as you control a Liliana planeswalker.",
        ),
        (
            "Pristine Angel",
            "As long as this creature is untapped, it has protection from artifacts and from each color.",
        ),
        (
            "Manor Gargoyle",
            "This creature has indestructible as long as it has defender.",
        ),
    ] {
        let twin = text.replace("As long as", "If").replace("as long as", "if");
        let expected = readings(&twin, Category::Document);
        assert!(!expected.is_empty(), "if twin: {card}");
        let actual = readings(text, Category::Document);
        assert_eq!(actual.len(), expected.len(), "Reading count: {card}");
        assert_eq!(
            actual.iter().map(structure).collect::<BTreeSet<_>>(),
            expected.iter().map(structure).collect::<BTreeSet<_>>(),
            "Reading structure: {card}"
        );
    }
}

fn only(text: &str, category: Category) -> Reading {
    let values = readings(text, category);
    assert_eq!(values.len(), 1, "{text}");
    values.into_iter().next().unwrap()
}

fn invariant(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(
            lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == WordForm::Invariant
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: None,
    }
}

fn document(clause: Reading) -> Reading {
    Reading::Document {
        form: 0,
        first: Box::new(Reading::OrdinaryAbility {
            form: 0,
            body: Box::new(Reading::Paragraph {
                form: 0,
                first: Box::new(Reading::SentenceItem {
                    form: 0,
                    sentence: Box::new(Reading::Sentence {
                        form: 0,
                        clause: Box::new(clause),
                    }),
                }),
                rest: vec![],
            }),
        }),
        rest: vec![],
    }
}

#[test]
fn aeronaut_tinkerer_has_exactly_the_two_conditional_readings() {
    let finite = only("This creature has flying", Category::FiniteClause);
    let conditional = only(
        "as long as you control an artifact",
        Category::PrepositionPhrase,
    );
    let Reading::FiniteClause {
        subject, predicate, ..
    } = finite.clone()
    else {
        panic!()
    };
    let expected = BTreeSet::from([
        document(Reading::ClausalPreposition {
            form: 0,
            clause: Box::new(Reading::Declarative {
                form: 0,
                clause: Box::new(finite),
            }),
            dependent: Box::new(conditional.clone()),
        }),
        document(Reading::Declarative {
            form: 0,
            clause: Box::new(Reading::FiniteClause {
                form: 0,
                subject,
                predicate: Box::new(Reading::PrepositionPredicate {
                    category: Category::FinitePredicate,
                    form: 0,
                    head: predicate,
                    modifier: Box::new(conditional),
                }),
            }),
        }),
    ]);
    assert_eq!(
        readings(
            "This creature has flying as long as you control an artifact.",
            Category::Document
        ),
        expected
    );
}

#[test]
fn master_thief_has_exactly_the_three_duration_readings() {
    let duration = Reading::AdverbComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/For"),
        complement: Box::new(Reading::EquativeAdverb {
            form: 0,
            governor: invariant("vocab:Adverb/EquativeAs"),
            head: invariant("vocab:Adverb/Long"),
            marker: invariant("vocab:Preposition/As"),
            complement: Box::new(only("you control this creature", Category::FiniteClause)),
        }),
    };
    duration.admit(lexicon()).unwrap();
    assert_eq!(
        readings(
            "for as long as you control this creature",
            Category::PrepositionPhrase
        ),
        BTreeSet::from([duration.clone()])
    );
    let triggered_clause = only(
        "When this creature enters, gain control of target artifact",
        Category::Clause,
    );
    let Reading::InitialPreposition {
        dependent: trigger,
        clause: imperative,
        ..
    } = triggered_clause.clone()
    else {
        panic!()
    };
    let Reading::Imperative {
        predicate: bare, ..
    } = *imperative.clone()
    else {
        panic!()
    };
    let Reading::BarePredicate {
        head: predicate, ..
    } = *bare
    else {
        panic!()
    };
    let expected = BTreeSet::from([
        document(Reading::ClausalPreposition {
            form: 0,
            clause: Box::new(triggered_clause),
            dependent: Box::new(duration.clone()),
        }),
        document(Reading::InitialPreposition {
            form: 0,
            dependent: trigger.clone(),
            clause: Box::new(Reading::ClausalPreposition {
                form: 0,
                clause: imperative,
                dependent: Box::new(duration.clone()),
            }),
        }),
        document(Reading::InitialPreposition {
            form: 0,
            dependent: trigger,
            clause: Box::new(Reading::Imperative {
                form: 0,
                predicate: Box::new(Reading::BarePredicate {
                    form: 0,
                    head: Box::new(Reading::PrepositionPredicate {
                        category: Category::SecondaryVerbPhrase,
                        form: 0,
                        head: predicate,
                        modifier: Box::new(duration),
                    }),
                }),
            }),
        }),
    ]);
    assert_eq!(
        readings(
            "When this creature enters, gain control of target artifact for as long as you control this creature.",
            Category::Document
        ),
        expected
    );
}

#[test]
fn independently_constructed_conditional_pp_obeys_both_roundtrip_laws() {
    use deckmaste_english_v3::grammar::FrameValue;
    use deckmaste_english_v3::grammar::Word;
    use deckmaste_lexical::Number;
    use deckmaste_lexical::Person;
    use deckmaste_lexical::SurfaceCase;
    use deckmaste_lexical::WordForm;
    let word = |owner: &str, form: WordForm, frame, countability| Word {
        value: LexicalReading::Word(
            lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                        && (owner != "vocab:Article/Indefinite" || value.variant == 1)
                        && (owner != "core-verb:Control"
                            || (value.features.number == Some(Number::Singular)
                                && value.features.person == Some(Person::Second)))
                })
                .unwrap_or_else(|| panic!("missing {owner} {form:?}"))
                .clone(),
        ),
        frame,
        countability,
    };
    // Aeronaut Tinkerer's conditional constituent.
    let complement = Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::NominativePronoun {
                form: 0,
                head: word("vocab:SubjectPronoun/You", WordForm::Invariant, None, None),
            }),
        }),
        predicate: Box::new(Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: word("core-verb:Control", WordForm::Present, Some(0), None),
            complements: vec![FrameValue::Argument(Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(Reading::IndefiniteNounPhrase {
                    form: 0,
                    determiner: word("vocab:Article/Indefinite", WordForm::Invariant, None, None),
                    head: Box::new(Reading::Noun {
                        form: 0,
                        head: word("lexeme:type/artifact", WordForm::Singular, None, Some(true)),
                    }),
                }),
            }))],
        }),
    };
    for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
        let mut head = word(
            "vocab:Preposition/AsLongAs",
            WordForm::Invariant,
            None,
            None,
        );
        let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
        value.capitalization = casing;
        let value = Reading::ClauseComplementPreposition {
            form: 0,
            head,
            complement: Box::new(complement.clone()),
        };
        value.admit(lexicon()).unwrap();
        let text = if casing == SurfaceCase::Initial {
            "As long as you control an artifact"
        } else {
            "as long as you control an artifact"
        };
        assert_eq!(
            value.realize(lexicon()).unwrap().as_bytes(),
            text.as_bytes()
        );
        let actual = readings(text, Category::PrepositionPhrase);
        assert_eq!(actual, BTreeSet::from([value.clone()]));
        let mut words = Vec::new();
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        let mut parsed_words = Vec::new();
        actual
            .first()
            .unwrap()
            .visit_words(&mut |word| parsed_words.push(word.clone()))
            .unwrap();
        assert_eq!(parsed_words, words);
        assert_eq!(structure(actual.first().unwrap()), structure(&value));
    }
}

#[test]
fn conditional_head_excludes_that_and_nonfinite_complements() {
    for text in [
        "as long as that you control an artifact",
        "as long as control an artifact",
        "as long as controlling an artifact",
    ] {
        assert!(
            readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn duration_comparison_requires_its_expanded_complement_marker() {
    assert!(
        readings(
            "for as long to you control this creature",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
    assert!(
        readings(
            "for as long as that you control this creature",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
}
