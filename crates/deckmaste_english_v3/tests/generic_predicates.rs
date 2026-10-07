mod common;

use deckmaste_english_v3::{
    grammar::{Category, FrameValue, GrammarEnvironment, Reading, Word},
    parse,
};
use deckmaste_lexical::{LexicalReading, Lexicon, Numeral, SurfaceCase, WordForm};
use std::{collections::BTreeSet, path::Path, sync::LazyLock};

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});
static ENVIRONMENT: LazyLock<GrammarEnvironment<'static>> =
    LazyLock::new(|| GrammarEnvironment::new(&LEXICON));
fn word(owner: &str, form: WordForm, frame: Option<usize>, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(
            LEXICON
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && (form != WordForm::Present
                            || (value.features.number == Some(deckmaste_lexical::Number::Singular)
                                && value.features.person == Some(deckmaste_lexical::Person::Third)))
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap_or_else(|| panic!("missing lexical value {owner} {form:?}"))
                .clone(),
        ),
        frame,
        countability,
    }
}
fn number() -> Word {
    Word {
        value: LexicalReading::Numeral {
            value: 1,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}
fn noun(owner: &str, form: WordForm) -> Reading {
    Reading::Noun {
        form: 0,
        head: word(owner, form, None, Some(true)),
    }
}
fn accusative(head: Reading) -> Reading {
    Reading::CasePhrase {
        form: 0,
        category: Category::AccusativePhrase,
        head: Box::new(head),
    }
}
fn argument(reading: Reading) -> FrameValue {
    FrameValue::Argument(Box::new(reading))
}
fn predicate(owner: &str, frame: usize, complements: Vec<FrameValue>) -> Reading {
    Reading::SelectedPredicate {
        form: 0,
        category: Category::SecondaryVerbPhrase,
        head: word(owner, WordForm::Plain, Some(frame), None),
        complements,
    }
}
fn readings(text: &str) -> BTreeSet<Reading> {
    readings_at(text, Category::SecondaryVerbPhrase)
}
fn readings_at(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = ENVIRONMENT.grammar();
    let forest = parse(grammar, &LEXICON, &LEXICON.analyze(text), &category).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}
fn laws(text: &str, expected: &Reading) {
    ENVIRONMENT.admit(expected).unwrap();
    assert_eq!(ENVIRONMENT.realize(expected).unwrap(), text);
    assert!(
        readings_at(text, expected.category()).contains(expected),
        "missing independently constructed reading for {text}"
    );
    let mut expected_nodes = vec![];
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    let actual = readings_at(text, expected.category())
        .take(expected)
        .unwrap();
    let mut actual_nodes = vec![];
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
}
#[test]
fn authentic_amass_consumes_declared_frame_without_shape_aliases() {
    // Orcish Bowmasters: "Then amass Orcs 1."
    let orcs = accusative(Reading::BarePlural {
        form: 0,
        head: Box::new(noun("lexeme:creature_subtype/orc", WordForm::Plural)),
    });
    let amount = Reading::ScalarAmount {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: number(),
        }),
    };
    let amass = predicate(
        "lexeme:keyword_action/amass",
        0,
        vec![argument(orcs), argument(amount)],
    );
    common::assert_constituents(
        "amass Orcs 1",
        Category::SecondaryVerbPhrase,
        &[
            (Category::AccusativePhrase, "Orcs"),
            (Category::Amount, "1"),
        ],
    );
    laws("amass Orcs 1", &amass);
    assert!(readings("amass 1 Orcs").is_empty());
}
#[test]
fn selected_with_retains_marker_ownership_and_complement_order() {
    // Reins of the Vinesteed: "... shares a creature type with that creature."
    let nominal = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: word("lexeme:type/creature", WordForm::Singular, None, Some(true)),
        }),
        head: Box::new(noun("lexeme:CommonNoun/Type", WordForm::Singular)),
    };
    let object = accusative(Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: word("vocab:Article/Indefinite", WordForm::Invariant, None, None),
        head: Box::new(nominal),
    });
    let other = accusative(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(
            "vocab:SingularDemonstrative/That",
            WordForm::Invariant,
            None,
            None,
        ),
        head: Box::new(noun("lexeme:type/creature", WordForm::Singular)),
    });
    let expected = predicate(
        "core-verb:Share",
        1,
        vec![
            argument(object),
            FrameValue::Marker(word(
                "vocab:Preposition/With",
                WordForm::Invariant,
                None,
                None,
            )),
            argument(other),
        ],
    );
    laws("share a creature type with that creature", &expected);
    let mut finite = expected.clone();
    let Reading::SelectedPredicate { category, head, .. } = &mut finite else {
        unreachable!()
    };
    *category = Category::FinitePredicate;
    *head = word("core-verb:Share", WordForm::Present, Some(1), None);
    laws("shares a creature type with that creature", &finite);
    let mut wrong = expected.clone();
    let Reading::SelectedPredicate { complements, .. } = &mut wrong else {
        unreachable!()
    };
    complements[1] = FrameValue::Marker(word(
        "vocab:Preposition/To",
        WordForm::Invariant,
        None,
        None,
    ));
    assert!(ENVIRONMENT.admit(&wrong).is_err());
}

#[test]
fn legacy_role_labels_remain_unsupported_instead_of_becoming_phrase_categories() {
    let legacy = &LEXICON.lexemes()["core-verb:Enter"].properties.frames[3];
    assert!(ENVIRONMENT.unsupported_frames().iter().any(|unsupported| {
        &unsupported.frame == legacy
            && unsupported
                .reason
                .contains("unsupported slot category Object")
    }));
    // Grafdigger's Cage's exact one-Reading contract is also asserted in participial_uses.
    assert_eq!(readings("enter the battlefield").len(), 1);
}

fn negative_copula() -> Reading {
    let head = Word {
        value: LexicalReading::Word(deckmaste_lexical::LexicalValue {
            lexeme: "core-verb:BeNegative".into(),
            form: WordForm::Present,
            features: deckmaste_lexical::FeatureBundle {
                number: Some(deckmaste_lexical::Number::Plural),
                person: Some(deckmaste_lexical::Person::Third),
                tense: Some(deckmaste_lexical::Tense::Present),
                finiteness: Some(deckmaste_lexical::Finiteness::Finite),
                case: None,
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: Some(0),
        countability: None,
    };
    Reading::SelectedPredicate {
        form: 0,
        category: Category::FinitePredicate,
        head,
        complements: vec![argument(Reading::AdjectivalComplement {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word(
                    "vocab:PredicativeAdjective/Legendary",
                    WordForm::Invariant,
                    None,
                    None,
                ),
            }),
        })],
    }
}
fn urzas_blast(relative_inside_premodifier: bool) -> Reading {
    let modifier = Box::new(Reading::NounPremodifier {
        form: 0,
        head: word("lexeme:type/land/non", WordForm::Singular, None, Some(true)),
    });
    let relative = Box::new(Reading::SubjectRelativeClause {
        form: 0,
        marker: word("vocab:Subordinator/That", WordForm::Invariant, None, None),
        predicate: Box::new(negative_copula()),
    });
    let permanent = Box::new(noun("lexeme:CommonNoun/Permanent", WordForm::Plural));
    let nominal = if relative_inside_premodifier {
        Reading::NounPremodifiedNominal {
            form: 0,
            modifier,
            head: Box::new(Reading::SubjectRelativeNominal {
                form: 0,
                head: permanent,
                relative,
            }),
        }
    } else {
        Reading::SubjectRelativeNominal {
            form: 0,
            head: Box::new(Reading::NounPremodifiedNominal {
                form: 0,
                modifier,
                head: permanent,
            }),
            relative,
        }
    };
    let mut exile = word(
        "lexeme:keyword_action/exile",
        WordForm::Plain,
        Some(0),
        None,
    );
    let LexicalReading::Word(value) = &mut exile.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
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
                        clause: Box::new(Reading::Imperative {
                            form: 0,
                            predicate: Box::new(Reading::BarePredicate {
                                form: 0,
                                head: Box::new(Reading::SelectedPredicate {
                                    form: 0,
                                    category: Category::SecondaryVerbPhrase,
                                    head: exile,
                                    complements: vec![argument(accusative(
                                        Reading::DeterminedNounPhrase {
                                            form: 0,
                                            determiner: word(
                                                "vocab:FloatedQuantifier/All",
                                                WordForm::Invariant,
                                                None,
                                                None,
                                            ),
                                            head: Box::new(nominal),
                                        },
                                    ))],
                                }),
                            }),
                        }),
                    }),
                }),
                rest: vec![],
            }),
        }),
        rest: vec![],
    }
}
#[test]
fn urzas_ruinous_blast_keeps_the_adjective_inside_the_relative_clause() {
    let text = "Exile all nonland permanents that aren't legendary.";
    let expected = BTreeSet::from([urzas_blast(false), urzas_blast(true)]);
    for reading in &expected {
        laws(text, reading);
    }
    assert_eq!(readings_at(text, Category::Document), expected);
    common::assert_constituents(
        text,
        Category::Document,
        &[
            (
                Category::AccusativePhrase,
                "all nonland permanents that aren't legendary",
            ),
            (Category::FinitePredicate, "aren't legendary"),
        ],
    );
    let expected_predicates = [
        (
            deckmaste_lexical::Number::Singular,
            deckmaste_lexical::Person::Second,
        ),
        (
            deckmaste_lexical::Number::Plural,
            deckmaste_lexical::Person::First,
        ),
        (
            deckmaste_lexical::Number::Plural,
            deckmaste_lexical::Person::Second,
        ),
        (
            deckmaste_lexical::Number::Plural,
            deckmaste_lexical::Person::Third,
        ),
    ]
    .into_iter()
    .map(|(number, person)| {
        let mut reading = negative_copula();
        let Reading::SelectedPredicate { head, .. } = &mut reading else {
            unreachable!()
        };
        let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
        value.features.number = Some(number);
        value.features.person = Some(person);
        reading
    })
    .collect();
    assert_eq!(
        readings_at("aren't legendary", Category::FinitePredicate),
        expected_predicates
    );
}

#[test]
fn sun_droplet_remove_retains_object_and_marked_source_roundtrip() {
    let frames = &LEXICON.lexemes()["core-verb:Remove"].properties.frames;
    assert_eq!(frames.len(), 1);
    assert!(
        ENVIRONMENT
            .unsupported_frames()
            .iter()
            .all(|entry| !frames.contains(&entry.frame))
    );
    // Sun Droplet: "...you may remove a charge counter from this artifact."
    let object = accusative(Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: word("vocab:Article/Indefinite", WordForm::Invariant, None, None),
        head: Box::new(noun(
            "lexeme:counter_kind/chargeCounter/compound-noun",
            WordForm::Singular,
        )),
    });
    let source = accusative(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(
            "vocab:SingularDemonstrative/This",
            WordForm::Invariant,
            None,
            None,
        ),
        head: Box::new(noun("lexeme:type/artifact", WordForm::Singular)),
    });
    let expected = predicate(
        "core-verb:Remove",
        0,
        vec![
            argument(object),
            FrameValue::Marked {
                marker: word("vocab:Preposition/From", WordForm::Invariant, None, None),
                argument: Box::new(source),
            },
        ],
    );
    laws("remove a charge counter from this artifact", &expected);
    assert_eq!(
        readings("remove a charge counter from this artifact"),
        BTreeSet::from([expected.clone()])
    );
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let actual = readings("remove a charge counter from this artifact")
        .pop_first()
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);

    let mut wrong = expected;
    let Reading::SelectedPredicate { complements, .. } = &mut wrong else {
        unreachable!()
    };
    let FrameValue::Marked { marker, .. } = &mut complements[1] else {
        unreachable!()
    };
    *marker = word("vocab:Preposition/To", WordForm::Invariant, None, None);
    assert!(ENVIRONMENT.admit(&wrong).is_err());
}
