mod common;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

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

fn invariant(owner: &str) -> Word {
    word(owner, WordForm::Invariant, FeatureBundle::default())
}

fn noun(owner: &str) -> Reading {
    let mut head = word(
        owner,
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..Default::default()
        },
    );
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

fn accusative(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}

fn during_untap_step() -> Reading {
    Reading::PrepositionPhrase {
        form: 0,
        head: invariant("vocab:Preposition/During"),
        complement: Box::new(accusative(Reading::GenitiveNounPhrase {
            form: 0,
            possessor: Box::new(Reading::DeterminedNounPhrase {
                form: 0,
                determiner: invariant("vocab:FloatedQuantifier/Each"),
                head: Box::new(Reading::PremodifiedNominal {
                    form: 0,
                    modifier: Box::new(Reading::IntransitiveAdjective {
                        form: 0,
                        head: {
                            let mut head = invariant("vocab:AttributiveAdjective/Other");
                            head.frame = Some(0);
                            head
                        },
                    }),
                    head: Box::new(noun("lexeme:CommonNoun/Player")),
                }),
            }),
            marker: invariant("vocab:Genitive/Default"),
            head: Box::new(Reading::PremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::Adjective {
                    form: 0,
                    head: invariant("vocab:AttributiveAdjective/Untap"),
                }),
                head: Box::new(noun("lexeme:CommonNoun/Step")),
            }),
        })),
    }
}

fn control_gap() -> Reading {
    let mut head = word(
        "core-verb:Control",
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..Default::default()
        },
    );
    head.frame = Some(0);
    Reading::FiniteObjectGap { form: 0, head }
}

fn relative(predicate: Reading) -> Reading {
    Reading::ZeroObjectRelativeClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::NominativePronoun {
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
                ),
            }),
        }),
        predicate: Box::new(predicate),
    }
}

fn laws(text: &str, value: &Reading) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, value.category());
    let actual = parsed
        .get(value)
        .expect("missing independently constructed Reading");
    let mut nodes = Vec::new();
    let mut actual_nodes = Vec::new();
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, nodes);
    let mut words = Vec::new();
    let mut actual_words = Vec::new();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    actual
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, words);
}

fn seedborn_muse(relative: Reading) -> Reading {
    use deckmaste_english_v3::grammar::FrameValue;

    let mut permanent = word(
        "lexeme:CommonNoun/Permanent",
        WordForm::Plural,
        FeatureBundle {
            number: Some(Number::Plural),
            ..Default::default()
        },
    );
    permanent.countability = Some(true);
    let object = accusative(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:FloatedQuantifier/All"),
        head: Box::new(Reading::ObjectRelativeNominal {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: permanent,
            }),
            relative: Box::new(relative),
        }),
    });
    let mut untap = word(
        "lexeme:keyword_action/untap",
        WordForm::Plain,
        FeatureBundle::default(),
    );
    untap.frame = Some(1);
    let LexicalReading::Word(value) = &mut untap.value else { unreachable!() };
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
                                    category: Category::SecondaryVerbPhrase,
                                    form: 0,
                                    head: untap,
                                    complements: vec![FrameValue::Argument(Box::new(object))],
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
fn independently_constructed_seedborn_muse_relative_retains_internal_pp() {
    // Seedborn Muse: "Untap all permanents you control during each other
    // player's untap step." Reading 2 attaches the PP to control.
    let value = relative(Reading::PrepositionPredicate {
        category: Category::FiniteObjectGap,
        form: 0,
        head: Box::new(control_gap()),
        modifier: Box::new(during_untap_step()),
    });
    laws("you control during each other player's untap step", &value);
    laws(
        "Untap all permanents you control during each other player's untap step.",
        &seedborn_muse(value.clone()),
    );
    let mut mismatched = value.clone();
    let Reading::ZeroObjectRelativeClause { predicate, .. } = &mut mismatched else {
        unreachable!()
    };
    let Reading::PrepositionPredicate { head, .. } = predicate.as_mut() else {
        unreachable!()
    };
    let Reading::FiniteObjectGap { head, .. } = head.as_mut() else {
        unreachable!()
    };
    let LexicalReading::Word(head) = &mut head.value else { unreachable!() };
    head.features.person = Some(Person::Third);
    head.features.number = Some(Number::Plural);
    // The predicate is independently valid; only its agreement with you fails.
    let Reading::ZeroObjectRelativeClause { predicate, .. } = &mismatched else {
        unreachable!()
    };
    predicate.admit(lexicon()).unwrap();
    assert!(mismatched.admit(lexicon()).is_err());
}

#[test]
fn object_gap_pp_adjuncts_preserve_declared_adverbial_use() {
    // Seedborn Muse's PP stays licensed for both finite and bare gap hosts.
    for head in [
        control_gap(),
        Reading::BareObjectGap {
            form: 0,
            head: Word {
                frame: Some(0),
                ..word(
                    "core-verb:Control",
                    WordForm::Plain,
                    FeatureBundle::default(),
                )
            },
        },
    ] {
        let mut value = Reading::PrepositionPredicate {
            category: head.category(),
            form: 0,
            head: Box::new(head),
            modifier: Box::new(during_untap_step()),
        };
        laws("control during each other player's untap step", &value);
        // Prismatic Strands' nominal PP "of the color" is not a free Adjunct.
        let Reading::PrepositionPredicate { modifier, .. } = &mut value else {
            unreachable!()
        };
        **modifier = Reading::PrepositionPhrase {
            form: 0,
            head: invariant("vocab:Preposition/Of"),
            complement: Box::new(accusative(Reading::DeterminedNounPhrase {
                form: 0,
                determiner: invariant("vocab:DefiniteMarker/The"),
                head: Box::new(noun("lexeme:CommonNoun/Color")),
            })),
        };
        modifier.admit(lexicon()).unwrap();
        assert!(value.admit(lexicon()).is_err());
    }
}
