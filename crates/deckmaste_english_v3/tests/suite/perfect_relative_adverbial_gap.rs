use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, Finiteness, LexicalReading, LexicalValue, Number, Person, SurfaceCase, Tense,
    WordForm,
};

fn owner(word: &Word, expected: &str) -> bool {
    let LexicalReading::Word(value) = &word.value else {
        return false;
    };
    value.lexeme.as_str() == expected
}

fn assert_temporal_perfect(reading: &Reading) {
    let mut relative = 0;
    let mut perfect = 0;
    let mut wrong = 0;
    reading
        .visit(&mut |node| match node {
            Reading::AdverbialGapRelativeClause { .. } => relative += 1,
            Reading::PerfectAuxiliaryPredicate { .. } => perfect += 1,
            Reading::FiniteObjectGap { head, .. } if owner(head, "core-verb:Have") => wrong += 1,
            Reading::PassivePredicate { head, .. } if owner(head, "core-verb:Resolve") => {
                wrong += 1;
            }
            _ => {}
        })
        .unwrap();
    assert_eq!((relative, perfect, wrong), (1, 1, 0));
    reading.admit(common::lexicon()).unwrap();
}

#[test]
fn resolved_witnesses_use_temporal_gaps_and_perfect_auxiliaries() {
    let witnesses = [
        (
            "Saint Traft and Rem Karolus",
            "Whenever Saint Traft and Rem Karolus becomes tapped, create a 1/1 red Human creature token if this is the first time this ability has resolved this turn.",
            24,
        ),
        (
            "Belladonna Took",
            "Whenever a token you control enters, you gain 1 life if this is the first time this ability has resolved this turn.",
            24,
        ),
        (
            "Nissa, Leyline Tamer",
            "Then if this is the first time this ability has resolved this turn, reveal cards from the top of your library until you reveal a creature card.",
            72,
        ),
        (
            "Teething Wurmlet",
            "If this is the first time this ability has resolved this turn, put a +1/+1 counter on this creature.",
            24,
        ),
        (
            "Bronze Cudgels",
            "Until end of turn, equipped creature gets +X/+0, where X is the number of times this ability has resolved this turn.",
            3,
        ),
    ];
    for (card, text, count) in witnesses {
        let readings = common::readings_on_card(card, text, Category::Sentence);
        assert_eq!(readings.len(), count, "{card}");
        for reading in &readings {
            assert_temporal_perfect(reading);
        }
    }
}

#[test]
fn frequency_and_lexical_have_controls_keep_their_analyses() {
    common::assert_constituents_on_card(
        "Saint Traft and Rem Karolus",
        "If it's the second time, create a 1/1 blue Spirit creature token with flying.",
        Category::Sentence,
        &[(Category::NounPhrase, "the second time")],
    );
    common::assert_constituents_on_card(
        "Aurelia, the Warleader",
        "Whenever Aurelia attacks for the first time each turn, untap all creatures you control.",
        Category::Sentence,
        &[(Category::NounPhrase, "the first time")],
    );
    let readings = common::readings_on_card(
        "Felisa, Fang of Silverquill",
        "Whenever a nontoken creature you control dies, if it had counters on it, create X tapped 2/1 white and black Inkling creature tokens with flying, where X is the number of counters it had on it.",
        Category::Sentence,
    );
    assert!(!readings.is_empty());
    for reading in readings {
        let mut lexical_have = false;
        reading
            .visit(&mut |node| {
                if let Reading::SelectedPredicate { head, .. } = node {
                    lexical_have |= owner(head, "core-verb:Have");
                }
            })
            .unwrap();
        assert!(
            lexical_have,
            "the condition uses lexical possession, not a perfect auxiliary"
        );
    }
}

#[test]
fn temporal_relative_constituents_and_captain_america_are_covered() {
    let phrase = common::readings(
        "the first time you cast a spell each turn",
        Category::NounPhrase,
    );
    assert_eq!(phrase.len(), 16);
    for reading in &phrase {
        let mut relatives = 0;
        reading
            .visit(&mut |node| {
                if let Reading::AdverbialGapRelativeClause { .. } = node {
                    relatives += 1;
                }
            })
            .unwrap();
        assert_eq!(relatives, 1);
    }
    let readings = common::readings_on_card(
        "Captain America, Living Legend",
        "If it's the first time that creature has become tapped this turn, untap it.",
        Category::Sentence,
    );
    assert_eq!(readings.len(), 24);
    for reading in &readings {
        assert_temporal_perfect(reading);
        let mut demonstratives = 0;
        reading
            .visit_words(&mut |word| {
                if owner(word, "vocab:SingularDemonstrative/That") {
                    demonstratives += 1;
                }
            })
            .unwrap();
        assert_eq!(demonstratives, 1);
    }
}

fn word(
    owner: &str,
    form: WordForm,
    features: FeatureBundle,
    frame: Option<usize>,
    countability: Option<bool>,
) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability,
    }
}

fn noun(owner: &str, countability: bool) -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(
                    owner,
                    WordForm::Singular,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        ..Default::default()
                    },
                    None,
                    Some(countability),
                ),
            }),
        }),
    }
}

#[test]
fn independently_constructed_temporal_relative_roundtrips_and_requires_licensed_head() {
    for countability in [false, true] {
        let mut value = Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::ObjectRelativeNominal {
                form: 0,
                head: crate::common::postmodification(noun("lexeme:CommonNoun/Time", countability)),
                relative: Box::new(Reading::AdverbialGapRelativeClause {
                    form: 0,
                    clause: Box::new(Reading::FiniteClause {
                        form: 0,
                        subject: Box::new(Reading::CasePhrase {
                            form: 0,
                            category: Category::NominativePhrase,
                            head: Box::new(Reading::DeterminedNounPhrase {
                                form: 0,
                                determiner: word(
                                    "vocab:SingularDemonstrative/This",
                                    WordForm::Invariant,
                                    FeatureBundle::default(),
                                    None,
                                    None,
                                ),
                                head: Box::new(noun("lexeme:CommonNoun/Ability", true)),
                            }),
                        }),
                        predicate: Box::new(Reading::PerfectAuxiliaryPredicate {
                            form: 0,
                            category: Category::FinitePredicate,
                            head: word(
                                "core-verb:Have",
                                WordForm::Present,
                                FeatureBundle {
                                    number: Some(Number::Singular),
                                    person: Some(Person::Third),
                                    tense: Some(Tense::Present),
                                    finiteness: Some(Finiteness::Finite),
                                    case: None,
                                },
                                Some(4),
                                None,
                            ),
                            complement: Box::new(Reading::OvertComplement {
                                form: 0,
                                category: Category::PerfectComplement,
                                predicate: Box::new(Reading::PerfectComplement {
                                    form: 0,
                                    head: Box::new(Reading::SelectedPredicate {
                                        form: 0,
                                        category: Category::SecondaryVerbPhrase,
                                        head: word(
                                            "core-verb:Resolve",
                                            WordForm::PastParticiple,
                                            FeatureBundle {
                                                finiteness: Some(Finiteness::Nonfinite),
                                                ..Default::default()
                                            },
                                            Some(0),
                                            None,
                                        ),
                                        complements: vec![],
                                    }),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        };
        value.admit(common::lexicon()).unwrap();
        assert_eq!(
            value.realize(common::lexicon()).unwrap(),
            "time this ability has resolved"
        );
        assert!(
            common::readings("time this ability has resolved", Category::Nominal).contains(&value)
        );
        let Reading::NominalProjection { form: 0, head } = &mut value else {
            unreachable!()
        };
        let Reading::ObjectRelativeNominal { head, .. } = head.as_mut() else {
            unreachable!()
        };
        *head = common::postmodification(noun("lexeme:CommonNoun/Ability", true));
        assert!(value.admit(common::lexicon()).is_err());
        assert!(value.realize(common::lexicon()).is_err());
    }
}
