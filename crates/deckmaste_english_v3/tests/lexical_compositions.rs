mod common;

use std::collections::BTreeSet;

use common::{lexicon, readings};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm,
};

fn word(owner: &str, form: WordForm, features: FeatureBundle, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability,
        frame: None,
    }
}

fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    assert_eq!(readings(text, category), BTreeSet::from([expected]));
}

#[test]
fn combat_damage_keeps_its_noun_premodifier_and_mass_head() {
    // Fell Flagship's triggered ability.
    assert!(
        !readings(
            "Whenever this Vehicle deals combat damage to a player, that player discards a card.",
            Category::Document,
        )
        .is_empty()
    );
    let features = FeatureBundle {
        number: Some(Number::Singular),
        ..Default::default()
    };
    exact(
        "combat damage",
        Category::Nominal,
        Reading::NounPremodifiedNominal {
            form: 0,
            modifier: Box::new(Reading::NounPremodifier {
                form: 0,
                head: word(
                    "lexeme:turn_part/combat",
                    WordForm::Singular,
                    features.clone(),
                    Some(true),
                ),
            }),
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(
                    "lexeme:CommonNoun/Damage",
                    WordForm::Singular,
                    features,
                    Some(false),
                ),
            }),
        },
    );
    assert!(readings("combats damage", Category::Nominal).is_empty());
}

#[test]
fn defending_player_is_a_bare_np_with_a_verbal_premodifier() {
    // Odious Witch; "the defending player" also occurs on Goblin Firebug.
    assert!(
        !readings(
            "Whenever this creature attacks, defending player loses 1 life and you gain 1 life.",
            Category::Document,
        )
        .is_empty()
    );
    let mut modifier = word(
        "lexeme:Verb/Defend",
        WordForm::GerundParticiple,
        FeatureBundle {
            finiteness: Some(deckmaste_lexical::Finiteness::Nonfinite),
            ..Default::default()
        },
        None,
    );
    modifier.frame = Some(0);
    exact(
        "defending player",
        Category::NounPhrase,
        Reading::BareParticipialStatusNounPhrase {
            form: 0,
            modifier,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(
                    "lexeme:CommonNoun/Player",
                    WordForm::Singular,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        ..Default::default()
                    },
                    Some(true),
                ),
            }),
        },
    );
    for text in ["defend player", "defended player", "attacking player"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn plural_bare_genitives_preserve_possessor_number_and_the_apostrophe() {
    // Upheaval.
    assert!(
        !readings(
            "Return all permanents to their owners' hands.",
            Category::Document
        )
        .is_empty()
    );
    let possessor = Reading::PossessiveNounPhrase {
        form: 0,
        possessor: word(
            "vocab:PossessiveDeterminerPronoun/Their",
            WordForm::Invariant,
            FeatureBundle {
                number: Some(Number::Plural),
                person: Some(Person::Third),
                case: Some(deckmaste_lexical::Case::Genitive),
                ..Default::default()
            },
            None,
        ),
        head: Box::new(Reading::Noun {
            form: 0,
            head: word(
                "lexeme:CommonNoun/Owner",
                WordForm::Plural,
                FeatureBundle {
                    number: Some(Number::Plural),
                    ..Default::default()
                },
                Some(true),
            ),
        }),
    };
    let expected = Reading::GenitiveNounPhrase {
        form: 0,
        possessor: Box::new(possessor),
        marker: word(
            "vocab:Genitive/Sibilant",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        ),
        head: Box::new(Reading::Noun {
            form: 0,
            head: word(
                "lexeme:CommonNoun/Hand",
                WordForm::Plural,
                FeatureBundle {
                    number: Some(Number::Plural),
                    ..Default::default()
                },
                Some(true),
            ),
        }),
    };
    exact(
        "their owners' hands",
        Category::NounPhrase,
        expected.clone(),
    );
    let mut wrong = expected.clone();
    let Reading::GenitiveNounPhrase { possessor, .. } = &mut wrong else {
        unreachable!()
    };
    let Reading::PossessiveNounPhrase { head, .. } = &mut **possessor else {
        unreachable!()
    };
    *head = Box::new(Reading::PostmodifiedNominal {
        form: 0,
        head: head.clone(),
        modifier: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: word(
                "vocab:Preposition/Of",
                WordForm::Invariant,
                FeatureBundle::default(),
                None,
            ),
            complement: Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(Reading::BarePlural {
                    form: 0,
                    head: Box::new(Reading::Noun {
                        form: 0,
                        head: word(
                            "lexeme:CommonNoun/Card",
                            WordForm::Plural,
                            FeatureBundle {
                                number: Some(Number::Plural),
                                ..Default::default()
                            },
                            Some(true),
                        ),
                    }),
                }),
            }),
        }),
    });
    // The final noun's eligibility cannot be inherited from an earlier head.
    // The text can separately read with the genitive inside the of phrase.
    assert!(wrong.admit(lexicon()).is_err());
    let mut curved = expected;
    let Reading::GenitiveNounPhrase { marker, .. } = &mut curved else {
        unreachable!()
    };
    let LexicalReading::Word(value) = &mut marker.value else { unreachable!() };
    value.variant = 1;
    exact("their owners’ hands", Category::NounPhrase, curved);
    for text in [
        "their owner's' hands",
        "their owner' hands",
        "their owners hands",
        "their owners's hands",
        "their Mice' cards",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn attested_hybrid_and_phyrexian_symbols_retain_cost_and_mana_identity() {
    // Vexing Shusher and Immolating Souleater.
    for text in [
        "{R/G}: Target spell can't be countered.",
        "{R/P}: This creature gets +1/+0 until end of turn.",
    ] {
        assert!(!readings(text, Category::Document).is_empty(), "{text}");
    }
    // Symbol constituents attested in supported rules text. Tazri, Beacon of
    // Unity supplies all four monocolored hybrids in one activated cost.
    for (name, text) in [
        ("HybridWhiteBlue", "{W/U}"),
        ("HybridWhiteBlack", "{W/B}"),
        ("HybridBlueBlack", "{U/B}"),
        ("HybridBlueRed", "{U/R}"),
        ("HybridBlackRed", "{B/R}"),
        ("HybridBlackGreen", "{B/G}"),
        ("HybridRedGreen", "{R/G}"),
        ("HybridRedWhite", "{R/W}"),
        ("HybridGreenWhite", "{G/W}"),
        ("HybridGreenBlue", "{G/U}"),
        ("PhyrexianWhite", "{W/P}"),
        ("PhyrexianBlue", "{U/P}"),
        ("PhyrexianBlack", "{B/P}"),
        ("PhyrexianRed", "{R/P}"),
        ("PhyrexianGreen", "{G/P}"),
        ("MonocoloredHybridBlue", "{2/U}"),
        ("MonocoloredHybridBlack", "{2/B}"),
        ("MonocoloredHybridRed", "{2/R}"),
        ("MonocoloredHybridGreen", "{2/G}"),
    ] {
        let symbol = word(
            &format!("vocab:FixedCostSymbol/{name}"),
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
        );
        exact(
            text,
            Category::CostSymbols,
            Reading::CostSymbols {
                form: 0,
                first: Box::new(Reading::NamedCostSymbol {
                    form: 0,
                    symbol: symbol.clone(),
                }),
                rest: vec![],
            },
        );
        exact(
            text,
            Category::ManaPhrase,
            Reading::ManaPhrase {
                form: 0,
                first: Box::new(Reading::NamedManaSymbol { form: 0, symbol }),
                rest: vec![],
            },
        );
    }
    for text in ["{2/W}", "{W/U/P}", "{R/W} {R/P}", "{W//U}"] {
        assert!(readings(text, Category::ManaPhrase).is_empty(), "{text}");
    }
}

#[test]
fn at_random_retains_a_preposition_with_an_adjective_complement() {
    // Spellgorger Barbarian.
    assert!(
        !readings(
            "When this creature enters, discard a card at random.",
            Category::Document
        )
        .is_empty()
    );
    exact(
        "at random",
        Category::PrepositionPhrase,
        Reading::AdjectiveComplementPreposition {
            form: 0,
            head: word(
                "vocab:Preposition/At",
                WordForm::Invariant,
                FeatureBundle::default(),
                None,
            ),
            complement: word(
                "vocab:Adjective/Random",
                WordForm::Invariant,
                FeatureBundle::default(),
                None,
            ),
        },
    );
    for text in ["at black", "of random", "during random"] {
        assert!(
            readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
    assert!(readings("a card at random", Category::NounPhrase).is_empty());
    assert!(readings("at random", Category::LocativeComplement).is_empty());
}

#[test]
fn color_negative_formation_keeps_the_adjective_lexical_hosts() {
    // Doom Blade and constituents from Redcap Melee, Crovax, Ascendant Hero,
    // Battle Frenzy, and Inundate.
    assert!(!readings("Destroy target nonblack creature.", Category::Document).is_empty());
    for (name, text) in [
        ("Black", "nonblack"),
        ("Red", "nonred"),
        ("White", "nonwhite"),
        ("Green", "nongreen"),
        ("Blue", "nonblue"),
    ] {
        exact(
            text,
            Category::AdjectivePhrase,
            Reading::Adjective {
                form: 0,
                head: word(
                    &format!("vocab:ColorWord/{name}/non"),
                    WordForm::Invariant,
                    FeatureBundle::default(),
                    None,
                ),
            },
        );
        assert!(readings(&text.replacen("non", "non-", 1), Category::AdjectivePhrase).is_empty());
    }
}
