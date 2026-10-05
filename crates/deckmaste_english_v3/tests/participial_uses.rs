mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn word(owner: &str) -> Word {
    Word {
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
fn noun(owner: &str) -> Reading {
    let mut head = word(owner);
    head.countability = Some(true);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = WordForm::Singular;
        value.features.number = Some(Number::Singular);
    }
    Reading::Noun { form: 0, head }
}
fn verb(owner: &str, form: WordForm, frame: usize) -> Word {
    let mut head = word(owner);
    head.frame = Some(frame);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = form;
        value.features = if form == WordForm::GerundParticiple {
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..Default::default()
            }
        } else if form == WordForm::Plain {
            FeatureBundle::default()
        } else {
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                ..Default::default()
            }
        };
    }
    head
}
fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    let actual: BTreeSet<_> = grammar
        .readings(&forest)
        .map(|value| value.unwrap())
        .collect();
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = vec![];
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = vec![];
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
    let mut expected_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    let mut actual_nodes = Vec::new();
    actual
        .first()
        .unwrap()
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
}
#[test]
fn authentic_attacking_and_blocking_modifiers_retain_verbal_identity() {
    // Armed Response and Silent Assassin, respectively.
    for (owner, text) in [
        ("core-verb:Attack", "target attacking creature"),
        ("core-verb:Block", "target blocking creature"),
    ] {
        exact(
            text,
            Category::NounPhrase,
            Reading::TargetNounPhrase {
                form: 0,
                marker: word("vocab:TargetingMarker/Target"),
                head: Box::new(Reading::ParticipialPremodifier {
                    form: 0,
                    modifier: Box::new(Reading::VerbalPremodifier {
                        form: 0,
                        head: verb(owner, WordForm::GerundParticiple, 0),
                    }),
                    head: Box::new(noun("lexeme:type/creature")),
                }),
            },
        );
    }
}
#[test]
fn authentic_tapped_and_untapped_modifiers_retain_adjective_identity() {
    // Assassinate/Royal Assassin; Traitor's Roar, respectively.
    for (owner, text) in [
        ("tap", "target tapped creature"),
        ("untap", "target untapped creature"),
    ] {
        exact(
            text,
            Category::NounPhrase,
            Reading::TargetNounPhrase {
                form: 0,
                marker: word("vocab:TargetingMarker/Target"),
                head: Box::new(Reading::PremodifiedNominal {
                    form: 0,
                    modifier: Box::new(Reading::Adjective {
                        form: 0,
                        head: word(&format!("lexeme:keyword_action/{owner}/adjective")),
                    }),
                    head: Box::new(noun("lexeme:type/creature")),
                }),
            },
        );
    }
}
#[test]
fn idyllic_beachfront_enters_with_an_adjectival_depictive() {
    let mut this = word("vocab:SingularDemonstrative/This");
    if let LexicalReading::Word(value) = &mut this.value {
        value.capitalization = SurfaceCase::Initial;
    }
    exact(
        "This land enters tapped.",
        Category::Sentence,
        Reading::Sentence {
            form: 0,
            clause: Box::new(Reading::Declarative {
                form: 0,
                clause: Box::new(Reading::FiniteClause {
                    form: 0,
                    subject: Box::new(Reading::CasePhrase {
                        form: 0,
                        category: Category::NominativePhrase,
                        head: Box::new(Reading::DeterminedNounPhrase {
                            form: 0,
                            determiner: this,
                            head: Box::new(noun("lexeme:type/land")),
                        }),
                    }),
                    predicate: Box::new(Reading::DepictivePredicate {
                        form: 0,
                        category: Category::FinitePredicate,
                        head: verb("core-verb:Enter", WordForm::Present, 0),
                        modifier: Box::new(Reading::AdjectivalDepictive {
                            form: 0,
                            phrase: Box::new(Reading::Adjective {
                                form: 0,
                                head: word("lexeme:keyword_action/tap/adjective"),
                            }),
                        }),
                    }),
                }),
            }),
        },
    );
}
#[test]
fn righteous_blow_keeps_coordinated_verbal_modifiers_under_the_nominal() {
    exact(
        "target attacking or blocking creature",
        Category::NounPhrase,
        Reading::TargetNounPhrase {
            form: 0,
            marker: word("vocab:TargetingMarker/Target"),
            head: Box::new(Reading::ParticipialPremodifier {
                form: 0,
                modifier: Box::new(Reading::Coordination {
                    form: 0,
                    category: Category::VerbalPremodifier,
                    left: Box::new(Reading::VerbalPremodifier {
                        form: 0,
                        head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
                    }),
                    coordinator: word("vocab:Coordinator/Or"),
                    right: Box::new(Reading::VerbalPremodifier {
                        form: 0,
                        head: verb("core-verb:Block", WordForm::GerundParticiple, 0),
                    }),
                }),
                head: Box::new(noun("lexeme:type/creature")),
            }),
        },
    );
}

#[test]
fn yore_tiller_nephilim_coordinates_adjectival_and_participial_depictives() {
    // Yore-Tiller Nephilim: "... to the battlefield tapped and attacking."
    exact(
        "tapped and attacking",
        Category::DepictivePhrase,
        Reading::Coordination {
            form: 0,
            category: Category::DepictivePhrase,
            left: Box::new(Reading::AdjectivalDepictive {
                form: 0,
                phrase: Box::new(Reading::Adjective {
                    form: 0,
                    head: word("lexeme:keyword_action/tap/adjective"),
                }),
            }),
            coordinator: word("vocab:Coordinator/And"),
            right: Box::new(Reading::ParticipialDepictive {
                form: 0,
                phrase: Box::new(Reading::IntransitivePredicate {
                    form: 0,
                    category: Category::SecondaryVerbPhrase,
                    head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
                }),
            }),
        },
    );
}

#[test]
fn copular_become_does_not_select_a_gerund_participial_predicate() {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze("becomes attacking");
    let forest = parse(&grammar, &LEXICON, &analyzed, &Category::FinitePredicate).unwrap();
    assert_eq!(grammar.readings(&forest).count(), 0);
}

#[test]
fn hero_of_bladehold_auxiliary_frame_is_not_an_intransitive_depictive_host() {
    // Hero of Bladehold's "... tokens that are tapped and attacking."
    // The independently built wrong analysis treats the passive auxiliary
    // frame as an intransitive lexical host for a depictive adjunct.
    let mut are = verb("core-verb:Be", WordForm::Present, 1);
    if let LexicalReading::Word(value) = &mut are.value {
        value.features.number = Some(Number::Plural);
    }
    let modifier = Reading::Coordination {
        form: 0,
        category: Category::DepictivePhrase,
        left: Box::new(Reading::AdjectivalDepictive {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word("lexeme:keyword_action/tap/adjective"),
            }),
        }),
        coordinator: word("vocab:Coordinator/And"),
        right: Box::new(Reading::ParticipialDepictive {
            form: 0,
            phrase: Box::new(Reading::IntransitivePredicate {
                form: 0,
                category: Category::SecondaryVerbPhrase,
                head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
            }),
        }),
    };
    let invalid = Reading::DepictivePredicate {
        form: 0,
        category: Category::FinitePredicate,
        head: are,
        modifier: Box::new(modifier),
    };
    assert!(invalid.admit(&LEXICON).is_err());
}

#[test]
fn grafdiggers_cage_enter_the_battlefield_keeps_an_object_complement() {
    // Grafdigger's Cage: "Creature cards in graveyards and libraries can't
    // enter the battlefield."
    exact(
        "enter the battlefield",
        Category::SecondaryVerbPhrase,
        Reading::TransitivePredicate {
            form: 0,
            category: Category::SecondaryVerbPhrase,
            head: verb("core-verb:Enter", WordForm::Plain, 1),
            object: Box::new(Reading::CasePhrase {
                form: 0,
                category: Category::AccusativePhrase,
                head: Box::new(Reading::DeterminedNounPhrase {
                    form: 0,
                    determiner: word("vocab:DefiniteMarker/The"),
                    head: Box::new(noun("lexeme:CommonNoun/Battlefield")),
                }),
            }),
        },
    );
}

fn accusative(head: Reading) -> Reading {
    Reading::CasePhrase {
        form: 0,
        category: Category::AccusativePhrase,
        head: Box::new(head),
    }
}
fn pronoun(owner: &str, case: deckmaste_lexical::Case, person: Person) -> Word {
    let mut head = word(owner);
    if let LexicalReading::Word(value) = &mut head.value {
        value.features = FeatureBundle {
            number: Some(Number::Singular),
            person: Some(person),
            case: Some(case),
            ..Default::default()
        };
    }
    head
}
fn this_creature() -> Reading {
    accusative(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word("vocab:SingularDemonstrative/This"),
        head: Box::new(noun("lexeme:type/creature")),
    })
}
fn attacking() -> Reading {
    Reading::IntransitivePredicate {
        form: 0,
        category: Category::SecondaryVerbPhrase,
        head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
    }
}
fn depictive(mixed: bool) -> Reading {
    let right = Reading::ParticipialDepictive {
        form: 0,
        phrase: Box::new(attacking()),
    };
    if !mixed {
        return right;
    }
    Reading::Coordination {
        form: 0,
        category: Category::DepictivePhrase,
        left: Box::new(Reading::AdjectivalDepictive {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word("lexeme:keyword_action/tap/adjective"),
            }),
        }),
        coordinator: word("vocab:Coordinator/And"),
        right: Box::new(right),
    }
}
#[test]
fn authentic_complemented_participial_postmodifiers_preserve_their_objects_and_auxiliary() {
    // Blessed Reversal: "... for each creature attacking you."
    exact(
        "creature attacking you",
        Category::Nominal,
        Reading::ParticipialPostmodifiedNominal {
            form: 0,
            head: Box::new(noun("lexeme:type/creature")),
            modifier: Box::new(Reading::TransitivePredicate {
                form: 0,
                category: Category::SecondaryVerbPhrase,
                head: verb("core-verb:Attack", WordForm::GerundParticiple, 1),
                object: Box::new(accusative(Reading::AccusativePronoun {
                    form: 0,
                    head: pronoun(
                        "vocab:ObjectPronoun/You",
                        deckmaste_lexical::Case::Accusative,
                        Person::Second,
                    ),
                })),
            }),
        },
    );
    // Knight of Dusk: "Destroy target creature blocking this creature."
    exact(
        "target creature blocking this creature",
        Category::NounPhrase,
        Reading::TargetNounPhrase {
            form: 0,
            marker: word("vocab:TargetingMarker/Target"),
            head: Box::new(Reading::ParticipialPostmodifiedNominal {
                form: 0,
                head: Box::new(noun("lexeme:type/creature")),
                modifier: Box::new(Reading::TransitivePredicate {
                    form: 0,
                    category: Category::SecondaryVerbPhrase,
                    head: verb("core-verb:Block", WordForm::GerundParticiple, 1),
                    object: Box::new(this_creature()),
                }),
            }),
        },
    );
    // Apothecary White: "... for each player being attacked."
    let mut attacked = verb("core-verb:Attack", WordForm::PastParticiple, 1);
    if let LexicalReading::Word(value) = &mut attacked.value {
        value.features = FeatureBundle {
            finiteness: Some(Finiteness::Nonfinite),
            ..Default::default()
        };
    }
    exact(
        "player being attacked",
        Category::Nominal,
        Reading::ParticipialPostmodifiedNominal {
            form: 0,
            head: Box::new(noun("lexeme:CommonNoun/Player")),
            modifier: Box::new(Reading::ParticipialAuxiliaryPredicate {
                form: 0,
                category: Category::SecondaryVerbPhrase,
                head: verb("core-verb:Be", WordForm::GerundParticiple, 1),
                complement: Box::new(Reading::OvertComplement {
                    form: 0,
                    category: Category::ParticipialComplement,
                    predicate: Box::new(Reading::PassiveComplement {
                        form: 0,
                        head: Box::new(Reading::PassivePredicate {
                            form: 0,
                            head: attacked,
                        }),
                    }),
                }),
            }),
        },
    );
}
#[test]
fn hero_of_bladehold_mixed_be_complement_preserves_adjective_and_progressive() {
    // Hero of Bladehold: "... creature tokens that are tapped and attacking."
    let mut are = verb("core-verb:Be", WordForm::Present, 1);
    if let LexicalReading::Word(value) = &mut are.value {
        value.features.number = Some(Number::Plural);
    }
    let mut tokens = word("lexeme:CommonNoun/Token");
    tokens.countability = Some(true);
    if let LexicalReading::Word(value) = &mut tokens.value {
        value.form = WordForm::Plural;
        value.features.number = Some(Number::Plural);
    }
    exact(
        "tokens that are tapped and attacking",
        Category::Nominal,
        Reading::SubjectRelativeNominal {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: tokens,
            }),
            relative: Box::new(Reading::SubjectRelativeClause {
                form: 0,
                marker: word("vocab:Subordinator/That"),
                predicate: Box::new(Reading::ParticipialAuxiliaryPredicate {
                    form: 0,
                    category: Category::FinitePredicate,
                    head: are,
                    complement: Box::new(Reading::OvertComplement {
                        form: 0,
                        category: Category::ParticipialComplement,
                        predicate: Box::new(Reading::MixedCopularProgressiveComplement {
                            form: 0,
                            phrase: Box::new(depictive(true)),
                        }),
                    }),
                }),
            }),
        },
    );
}
fn battlefield() -> Reading {
    accusative(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word("vocab:DefiniteMarker/The"),
        head: Box::new(noun("lexeme:CommonNoun/Battlefield")),
    })
}
fn movement(
    owner: &str,
    frame: usize,
    form: WordForm,
    object: Reading,
    marker: &str,
    mixed: bool,
) -> Reading {
    let mut head = verb(owner, form, frame);
    if form == WordForm::PastParticiple {
        if let LexicalReading::Word(value) = &mut head.value {
            value.features = FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..Default::default()
            };
        }
    }
    Reading::ComplementedDepictivePredicate {
        form: 0,
        category: Category::SecondaryVerbPhrase,
        head: Box::new(Reading::SelectedObjectPrepositionPredicate {
            form: 0,
            category: Category::SecondaryDepictiveHost,
            head,
            object: Box::new(object),
            marker: word(marker),
            complement: Box::new(battlefield()),
        }),
        modifier: Box::new(depictive(mixed)),
    }
}
fn exact_alternatives(text: &str, expected: BTreeSet<Reading>) {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(
        &grammar,
        &LEXICON,
        &analyzed,
        &Category::SecondaryVerbPhrase,
    )
    .unwrap();
    let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
    assert_eq!(actual, expected);
    for expected in &expected {
        expected.admit(&LEXICON).unwrap();
        assert_eq!(expected.realize(&LEXICON).unwrap(), text);
        let parsed = actual.get(expected).unwrap();
        let mut nodes = Vec::new();
        let mut parsed_nodes = Vec::new();
        expected.visit(&mut |v| nodes.push(v.clone())).unwrap();
        parsed.visit(&mut |v| parsed_nodes.push(v.clone())).unwrap();
        assert_eq!(parsed_nodes, nodes);
        let mut words = Vec::new();
        let mut parsed_words = Vec::new();
        expected
            .visit_words(&mut |w| words.push(w.clone()))
            .unwrap();
        parsed
            .visit_words(&mut |w| parsed_words.push(w.clone()))
            .unwrap();
        assert_eq!(parsed_words, words);
    }
}
#[test]
fn actual_movement_constituents_keep_selected_destination_before_depictive() {
    // Senu, Keen-Eyed Protector: "... put it onto the battlefield attacking."
    let it = accusative(Reading::AccusativePronoun {
        form: 0,
        head: pronoun(
            "vocab:ObjectPronoun/It",
            deckmaste_lexical::Case::Accusative,
            Person::Third,
        ),
    });
    let expected = [WordForm::Plain, WordForm::PastParticiple]
        .into_iter()
        .flat_map(|form| {
            let intended = movement(
                "core-verb:Put",
                6,
                form,
                it.clone(),
                "vocab:Preposition/Onto",
                false,
            );
            let mut head = verb("core-verb:Put", form, 6);
            if form == WordForm::PastParticiple {
                if let LexicalReading::Word(value) = &mut head.value {
                    value.features = FeatureBundle {
                        finiteness: Some(Finiteness::Nonfinite),
                        ..Default::default()
                    };
                }
            }
            let alternate = Reading::SelectedObjectPrepositionPredicate {
                form: 0,
                category: Category::SecondaryVerbPhrase,
                head,
                object: Box::new(it.clone()),
                marker: word("vocab:Preposition/Onto"),
                complement: Box::new(accusative(Reading::DeterminedNounPhrase {
                    form: 0,
                    determiner: word("vocab:DefiniteMarker/The"),
                    head: Box::new(Reading::ParticipialPostmodifiedNominal {
                        form: 0,
                        head: Box::new(noun("lexeme:CommonNoun/Battlefield")),
                        modifier: Box::new(attacking()),
                    }),
                })),
            };
            [intended, alternate]
        })
        .collect();
    exact_alternatives("put it onto the battlefield attacking", expected);
    // Preeminent Captain: "... put a Soldier creature card from your hand onto
    // the battlefield tapped and attacking."
    let from_hand = Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/From"),
        complement: Box::new(accusative(Reading::PossessiveNounPhrase {
            form: 0,
            possessor: pronoun(
                "vocab:PossessiveDeterminerPronoun/Your",
                deckmaste_lexical::Case::Genitive,
                Person::Second,
            ),
            head: Box::new(noun("lexeme:CommonNoun/Hand")),
        })),
    };
    let premodify = |owner: &str, head: Reading| Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: match noun(owner) {
                Reading::Noun { head, .. } => head,
                _ => unreachable!(),
            },
        }),
        head: Box::new(head),
    };
    let postmodify = |head: Reading| Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(head),
        modifier: Box::new(from_hand.clone()),
    };
    // The source PP can qualify card, creature-card, or Soldier-creature-card.
    let card = noun("lexeme:CommonNoun/Card");
    let nominals = [
        premodify(
            "lexeme:creature_subtype/soldier",
            premodify("lexeme:type/creature", postmodify(card.clone())),
        ),
        premodify(
            "lexeme:creature_subtype/soldier",
            postmodify(premodify("lexeme:type/creature", card.clone())),
        ),
        postmodify(premodify(
            "lexeme:creature_subtype/soldier",
            premodify("lexeme:type/creature", card),
        )),
    ];
    let mut article = word("vocab:Article/Indefinite");
    if let LexicalReading::Word(value) = &mut article.value {
        value.features.number = Some(Number::Singular);
    }
    let expected = nominals
        .into_iter()
        .flat_map(|head| {
            let object = accusative(Reading::IndefiniteNounPhrase {
                form: 0,
                determiner: article.clone(),
                head: Box::new(head),
            });
            [WordForm::Plain, WordForm::PastParticiple]
                .into_iter()
                .map(move |form| {
                    movement(
                        "core-verb:Put",
                        6,
                        form,
                        object.clone(),
                        "vocab:Preposition/Onto",
                        true,
                    )
                })
        })
        .collect();
    exact_alternatives(
        "put a Soldier creature card from your hand onto the battlefield tapped and attacking",
        expected,
    );
    // Yore-Tiller Nephilim: "... return target creature card from your
    // graveyard to the battlefield tapped and attacking."
    let from_graveyard = Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/From"),
        complement: Box::new(accusative(Reading::PossessiveNounPhrase {
            form: 0,
            possessor: pronoun(
                "vocab:PossessiveDeterminerPronoun/Your",
                deckmaste_lexical::Case::Genitive,
                Person::Second,
            ),
            head: Box::new(noun("lexeme:CommonNoun/Graveyard")),
        })),
    };
    let postmodify = |head: Reading| Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(head),
        modifier: Box::new(from_graveyard.clone()),
    };
    let card = noun("lexeme:CommonNoun/Card");
    let nominals = [
        premodify("lexeme:type/creature", postmodify(card.clone())),
        postmodify(premodify("lexeme:type/creature", card)),
    ];
    let expected = nominals
        .into_iter()
        .map(|head| {
            let object = accusative(Reading::TargetNounPhrase {
                form: 0,
                marker: word("vocab:TargetingMarker/Target"),
                head: Box::new(head),
            });
            movement(
                "core-verb:Return",
                1,
                WordForm::Plain,
                object,
                "vocab:Preposition/To",
                true,
            )
        })
        .collect();
    exact_alternatives(
        "return target creature card from your graveyard to the battlefield tapped and attacking",
        expected,
    );
}
#[test]
fn participial_postmodifiers_and_mixed_complements_reject_wrong_form_or_function() {
    // CGEL p. 1523: Oracle English excludes stranded gerund-participial
    // auxiliaries.
    for complement in [
        Reading::ProgressiveEllipsis {
            form: 0,
            omission: Box::new(Reading::OmittedGerundParticiple { form: 0 }),
        },
        Reading::PassiveEllipsis {
            form: 0,
            omission: Box::new(Reading::OmittedPastParticiple { form: 0 }),
        },
    ] {
        let invalid = Reading::ParticipialAuxiliaryPredicate {
            form: 0,
            category: Category::SecondaryVerbPhrase,
            head: verb("core-verb:Be", WordForm::GerundParticiple, 1),
            complement: Box::new(complement),
        };
        assert!(invalid.admit(&LEXICON).is_err());
    }
    let finite = Reading::ParticipialPostmodifiedNominal {
        form: 0,
        head: Box::new(noun("lexeme:type/creature")),
        modifier: Box::new(Reading::IntransitivePredicate {
            form: 0,
            category: Category::FinitePredicate,
            head: verb("core-verb:Attack", WordForm::Present, 0),
        }),
    };
    assert!(finite.admit(&LEXICON).is_err());
    let plain = Reading::ParticipialPostmodifiedNominal {
        form: 0,
        head: Box::new(noun("lexeme:type/creature")),
        modifier: Box::new(Reading::IntransitivePredicate {
            form: 0,
            category: Category::SecondaryVerbPhrase,
            head: verb("core-verb:Attack", WordForm::Plain, 0),
        }),
    };
    assert!(plain.admit(&LEXICON).is_err());
    for phrase in [
        depictive(false),
        Reading::AdjectivalDepictive {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word("lexeme:keyword_action/tap/adjective"),
            }),
        },
    ] {
        assert!(
            Reading::MixedCopularProgressiveComplement {
                form: 0,
                phrase: Box::new(phrase)
            }
            .admit(&LEXICON)
            .is_err()
        );
    }
    // An incompatible selected frame cannot consume the
    // destination-plus-depictive host.
    let wrong_frame = movement(
        "core-verb:Put",
        0,
        WordForm::Plain,
        this_creature(),
        "vocab:Preposition/Onto",
        true,
    );
    assert!(wrong_frame.admit(&LEXICON).is_err());
}
