mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

const AVACYN: &str = "Prevent all damage that would be dealt to another target creature this turn by sources of the color of your choice.";
const SPIKESHELL: &str = "If that opponent's speed is greater than each other player's speed, reduce that opponent's speed by 1.";
const ADVISOR: &str = "Your maximum hand size is increased by two.";
const NOCTIS: &str = "You may cast artifact spells from your graveyard by paying 3 life in addition to paying their other costs.";

fn contains_construction(reading: &Reading, name: &str) -> bool {
    let mut found = false;
    reading
        .visit(&mut |node| found |= node.construction() == name)
        .unwrap();
    found
}

fn word(owner: &str, form: WordForm, frame: Option<usize>, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(
            lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap_or_else(|| panic!("missing lexical declaration {owner}"))
                .clone(),
        ),
        frame,
        countability,
    }
}
fn invariant(owner: &str) -> Word {
    word(owner, WordForm::Invariant, None, None)
}
fn noun(owner: &str, form: WordForm, count: bool) -> Reading {
    Reading::Noun {
        form: 0,
        head: word(owner, form, None, Some(count)),
    }
}
fn acc(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}
fn determined(owner: &str, head: Reading) -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant(owner),
        head: Box::new(head),
    }
}
fn possessed(owner: &str, head: Reading) -> Reading {
    Reading::PossessiveNounPhrase {
        form: 0,
        possessor: invariant(owner),
        head: Box::new(head),
    }
}
fn pp(owner: &str, head: Reading) -> Reading {
    Reading::PrepositionPhrase {
        form: 0,
        head: invariant(owner),
        complement: Box::new(acc(head)),
    }
}
fn numeral(value: i32, notation: Numeral) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}
fn measure(value: i32, notation: Numeral) -> Reading {
    if notation == Numeral::Cardinal {
        Reading::CardinalExtentMeasure {
            form: 0,
            value: Box::new(Reading::Cardinal {
                form: 0,
                head: numeral(value, notation),
            }),
        }
    } else {
        Reading::ScalarExtentMeasure {
            form: 0,
            value: Box::new(Reading::ScalarMeasurePhrase {
                form: 0,
                head: Box::new(Reading::UngroupedScalarNumeral {
                    form: 0,
                    head: numeral(value, notation),
                }),
            }),
        }
    }
}
fn laws(expected: &Reading, text: &str) {
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    let actual = readings(text, expected.category());
    let actual = actual
        .get(expected)
        .unwrap_or_else(|| panic!("missing authored Reading of {text}: {expected:?}"));
    let mut nodes = vec![];
    let mut actual_nodes = vec![];
    expected
        .visit(&mut |node| nodes.push(node.clone()))
        .unwrap();
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, nodes);
    let mut words = vec![];
    let mut actual_words = vec![];
    expected
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    actual
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, words);
}
fn avacyn_passive() -> Reading {
    let creature = determined(
        "vocab:Determinative/Another",
        Reading::TargetedNominal {
            form: 0,
            marker: invariant("vocab:TargetingMarker/Target"),
            head: Box::new(noun("lexeme:type/creature", WordForm::Singular, true)),
        },
    );
    Reading::NominalAdjunctPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(Reading::SelectedGapPrepositionPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word("core-verb:Deal", WordForm::PastParticiple, Some(5), None),
            marker: invariant("vocab:Preposition/To"),
            complement: Box::new(acc(creature)),
        }),
        modifier: Box::new(Reading::NominalAdjunctPhrase {
            form: 0,
            determiner: invariant("vocab:SingularDemonstrative/This"),
            head: Box::new(noun("lexeme:CommonNoun/Turn", WordForm::Singular, true)),
        }),
    }
}
fn avacyn_by() -> Reading {
    let choice = possessed(
        "vocab:PossessiveDeterminerPronoun/Your",
        noun("lexeme:CommonNoun/Choice", WordForm::Singular, true),
    );
    let color = determined(
        "vocab:DefiniteMarker/The",
        Reading::PostmodifiedNominal {
            form: 0,
            head: Box::new(noun("lexeme:CommonNoun/Color", WordForm::Singular, true)),
            modifier: Box::new(pp("vocab:Preposition/Of", choice)),
        },
    );
    pp(
        "vocab:Preposition/By",
        Reading::BarePlural {
            form: 0,
            head: Box::new(Reading::PostmodifiedNominal {
                form: 0,
                head: Box::new(noun("lexeme:CommonNoun/Source", WordForm::Plural, true)),
                modifier: Box::new(pp("vocab:Preposition/Of", color)),
            }),
        },
    )
}

#[test]
fn avacyn_internalised_complement_belongs_to_the_bare_passive() {
    let head = avacyn_passive();
    let modifier = avacyn_by();
    let intended = Reading::InternalisedComplementPredicate {
        form: 0,
        head: Box::new(head.clone()),
        modifier: Box::new(modifier.clone()),
    };
    let contrast = Reading::PrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(head),
        modifier: Box::new(modifier),
    };
    let text = "dealt to another target creature this turn by sources of the color of your choice";
    laws(&intended, text);
    laws(&contrast, text);
    assert_ne!(intended, contrast);
    let duplicate = Reading::InternalisedComplementPredicate {
        form: 0,
        head: Box::new(intended.clone()),
        modifier: Box::new(avacyn_by()),
    };
    assert!(duplicate.admit(lexicon()).is_err());
    let active = Reading::InternalisedComplementPredicate {
        form: 0,
        head: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word("core-verb:Prevent", WordForm::Plain, Some(0), None),
            complements: vec![FrameValue::Argument(Box::new(acc(Reading::BareMass {
                form: 0,
                head: Box::new(noun("lexeme:CommonNoun/Damage", WordForm::Singular, false)),
            })))],
        }),
        modifier: Box::new(avacyn_by()),
    };
    assert!(active.admit(lexicon()).is_err());

    assert_constituents(
        AVACYN,
        Category::Sentence,
        &[
            (
                Category::SecondaryVerbPhrase,
                "dealt to another target creature this turn by sources of the color of your choice",
            ),
            (
                Category::PrepositionPhrase,
                "by sources of the color of your choice",
            ),
        ],
    );
    assert!(
        readings(AVACYN, Category::Sentence)
            .iter()
            .any(|reading| contains_construction(reading, "InternalisedComplementPredicate"))
    );
}

#[test]
fn spikeshell_active_extent_is_selected_by_its_lexical_frame() {
    // The complete consequent is an attested VP, not a raw By suffix.
    let text = SPIKESHELL
        .strip_prefix("If that opponent's speed is greater than each other player's speed, ")
        .unwrap()
        .strip_suffix('.')
        .unwrap();
    let object = acc(Reading::GenitiveNounPhrase {
        form: 0,
        possessor: Box::new(determined(
            "vocab:SingularDemonstrative/That",
            noun("lexeme:CommonNoun/Opponent", WordForm::Singular, true),
        )),
        marker: invariant("vocab:Genitive/Default"),
        head: Box::new(noun("lexeme:CommonNoun/Speed", WordForm::Singular, false)),
    });
    let intended = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("lexeme:Verb/Reduce", WordForm::Plain, Some(1), None),
        complements: vec![
            FrameValue::Argument(Box::new(object)),
            FrameValue::Argument(Box::new(Reading::ScalarExtentComplement {
                form: 0,
                marker: invariant("vocab:Preposition/By"),
                complement: Box::new(measure(1, Numeral::Arabic(false))),
            })),
        ],
    };
    laws(&intended, text);
    let Reading::SelectedPredicate { complements, .. } = &intended else {
        unreachable!()
    };
    let FrameValue::Argument(object) = &complements[0] else { unreachable!() };
    let contrast = Reading::PrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word("lexeme:Verb/Reduce", WordForm::Plain, Some(0), None),
            complements: vec![FrameValue::Argument(object.clone())],
        }),
        modifier: Box::new(Reading::ScalarExtentComplement {
            form: 0,
            marker: invariant("vocab:Preposition/By"),
            complement: Box::new(measure(1, Numeral::Arabic(false))),
        }),
    };
    assert!(contrast.admit(lexicon()).is_err());
    // An otherwise valid frame may not take this extent slot or a different
    // marker.
    let mut wrong_frame = intended.clone();
    if let Reading::SelectedPredicate { head, .. } = &mut wrong_frame {
        head.frame = Some(0);
    }
    assert!(wrong_frame.admit(lexicon()).is_err());
    let mut wrong_marker = intended.clone();
    let Reading::SelectedPredicate { complements, .. } = &mut wrong_marker else {
        unreachable!()
    };
    let FrameValue::Argument(value) = &mut complements[1] else { unreachable!() };
    let Reading::ScalarExtentComplement { marker, .. } = value.as_mut() else {
        unreachable!()
    };
    *marker = invariant("vocab:Preposition/With");
    assert!(wrong_marker.admit(lexicon()).is_err());
    assert_constituents(
        "reduce that opponent's speed by 1",
        Category::SecondaryVerbPhrase,
        &[
            (Category::AccusativePhrase, "that opponent's speed"),
            (Category::ScalarMeasurePhrase, "1"),
        ],
    );
}

#[test]
fn trusted_advisor_passive_extent_remains_a_selected_complement() {
    let intended = Reading::SelectedExtentPassive {
        form: 0,
        head: word(
            "lexeme:Verb/Increase",
            WordForm::PastParticiple,
            Some(1),
            None,
        ),
        complement: Box::new(Reading::ScalarExtentComplement {
            form: 0,
            marker: invariant("vocab:Preposition/By"),
            complement: Box::new(measure(2, Numeral::Cardinal)),
        }),
    };
    laws(&intended, "increased by two");
    let contrast = Reading::PrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(Reading::PassivePredicate {
            form: 0,
            head: word(
                "lexeme:Verb/Increase",
                WordForm::PastParticiple,
                Some(0),
                None,
            ),
        }),
        modifier: Box::new(Reading::ScalarExtentComplement {
            form: 0,
            marker: invariant("vocab:Preposition/By"),
            complement: Box::new(measure(2, Numeral::Cardinal)),
        }),
    };
    assert!(contrast.admit(lexicon()).is_err());
    let mut wrong_frame = intended.clone();
    if let Reading::SelectedExtentPassive { head, .. } = &mut wrong_frame {
        head.frame = Some(0);
    }
    assert!(wrong_frame.admit(lexicon()).is_err());

    assert_constituents(
        ADVISOR,
        Category::Sentence,
        &[
            (Category::SecondaryVerbPhrase, "increased by two"),
            (Category::ScalarExtentMeasure, "two"),
        ],
    );
}

#[test]
fn noctis_means_preserves_its_complete_gerund_participial_complement() {
    let other_costs = possessed(
        "vocab:PossessiveDeterminerPronoun/Their",
        Reading::PremodifiedNominal {
            form: 0,
            modifier: Box::new(Reading::Adjective {
                form: 0,
                head: invariant("vocab:AttributiveAdjective/Other"),
            }),
            head: Box::new(Reading::BareFramedNoun {
                form: 0,
                head: word(
                    "lexeme:CommonNoun/Cost",
                    WordForm::Plural,
                    Some(0),
                    Some(true),
                ),
            }),
        },
    );
    let paying_costs = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Pay", WordForm::GerundParticiple, Some(0), None),
        complements: vec![FrameValue::Argument(Box::new(acc(other_costs)))],
    };
    let addition = Reading::BareMass {
        form: 0,
        head: Box::new(Reading::PostmodifiedNominal {
            form: 0,
            head: Box::new(noun(
                "lexeme:CommonNoun/Addition",
                WordForm::Singular,
                false,
            )),
            modifier: Box::new(Reading::GerundComplementPreposition {
                form: 0,
                head: invariant("vocab:Preposition/To"),
                complement: Box::new(paying_costs),
            }),
        }),
    };
    let paying_life = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Pay", WordForm::GerundParticiple, Some(0), None),
        complements: vec![FrameValue::Argument(Box::new(acc(
            Reading::MeasuredNounPhrase {
                form: 0,
                quantity: Box::new(Reading::UngroupedScalarNumeral {
                    form: 0,
                    head: numeral(3, Numeral::Arabic(false)),
                }),
                head: word(
                    "lexeme:CommonNoun/Life",
                    WordForm::Singular,
                    None,
                    Some(false),
                ),
            },
        )))],
    };
    let expected = Reading::GerundComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/By"),
        complement: Box::new(Reading::PrepositionPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: Box::new(paying_life),
            modifier: Box::new(pp("vocab:Preposition/In", addition)),
        }),
    };
    laws(
        &expected,
        "by paying 3 life in addition to paying their other costs",
    );

    let cast = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("lexeme:keyword_action/cast", WordForm::Plain, Some(0), None),
        complements: vec![FrameValue::Argument(Box::new(acc(Reading::BarePlural {
            form: 0,
            head: Box::new(Reading::NounPremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::NounPremodifier {
                    form: 0,
                    head: word("lexeme:type/artifact", WordForm::Singular, None, Some(true)),
                }),
                head: Box::new(noun("lexeme:CommonNoun/Spell", WordForm::Plural, true)),
            }),
        })))],
    };
    let graveyard = noun("lexeme:CommonNoun/Graveyard", WordForm::Singular, true);
    let intended = Reading::PrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(Reading::PrepositionPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: Box::new(cast.clone()),
            modifier: Box::new(pp(
                "vocab:Preposition/From",
                possessed("vocab:PossessiveDeterminerPronoun/Your", graveyard.clone()),
            )),
        }),
        modifier: Box::new(expected.clone()),
    };
    let contrast = Reading::PrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: Box::new(cast),
        modifier: Box::new(pp(
            "vocab:Preposition/From",
            possessed(
                "vocab:PossessiveDeterminerPronoun/Your",
                Reading::PostmodifiedNominal {
                    form: 0,
                    head: Box::new(graveyard),
                    modifier: Box::new(expected),
                },
            ),
        )),
    };
    let text = "cast artifact spells from your graveyard by paying 3 life in addition to paying their other costs";
    laws(&intended, text);
    assert!(contrast.admit(lexicon()).is_err());
    assert!(!readings(text, Category::SecondaryVerbPhrase).contains(&contrast));

    assert_constituents(
        NOCTIS,
        Category::Sentence,
        &[
            (
                Category::PrepositionPhrase,
                "by paying 3 life in addition to paying their other costs",
            ),
            (
                Category::SecondaryVerbPhrase,
                "paying 3 life in addition to paying their other costs",
            ),
            (Category::SecondaryVerbPhrase, "paying their other costs"),
        ],
    );
}
