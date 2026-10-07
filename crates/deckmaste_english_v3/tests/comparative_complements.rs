mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

#[test]
fn equality_complements_belong_to_the_adjective_inside_the_object() {
    // Soul's Fire; Mirkwood Elk's second sentence.
    for (text, object, comparison, complement) in [
        (
            "Target creature you control deals damage equal to its power to any target.",
            "damage equal to its power",
            "equal to its power",
            "its power",
        ),
        (
            "You gain life equal to that card's power.",
            "life equal to that card's power",
            "equal to that card's power",
            "that card's power",
        ),
    ] {
        assert_constituents(
            text,
            Category::Document,
            &[
                (Category::NounPhrase, object),
                (Category::AdjectivePhrase, comparison),
                (Category::NounPhrase, complement),
            ],
        );
        for reading in readings(text, Category::Document) {
            reading
                .visit(&mut |node| {
                    if node.category() == Category::PrepositionPhrase {
                        assert_ne!(node.realize(lexicon()).unwrap(), format!("to {complement}"));
                    }
                })
                .unwrap();
        }
    }
    assert_eq!(
        readings("equal to its power", Category::AdjectivePhrase).len(),
        1
    );
}

#[test]
fn equality_is_predicative_and_survives_a_postposed_recipient_order() {
    // Freedom Fighter Recruit; Massive Raid.
    assert_constituents(
        "Freedom Fighter Recruit's power is equal to the number of creatures you control.",
        Category::Document,
        &[(
            Category::AdjectivePhrase,
            "equal to the number of creatures you control",
        )],
    );
    assert_constituents(
        "Massive Raid deals damage to any target equal to the number of creatures you control.",
        Category::Document,
        &[(
            Category::ComparativeAdjectivePhrase,
            "equal to the number of creatures you control",
        )],
    );
}

#[test]
fn an_independent_nominal_comparison_preserves_structure_and_words() {
    use deckmaste_english_v3::grammar::Word;
    use deckmaste_lexical::Case;
    use deckmaste_lexical::FeatureBundle;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::LexicalValue;
    use deckmaste_lexical::Number;
    use deckmaste_lexical::Person;
    use deckmaste_lexical::SurfaceCase;
    use deckmaste_lexical::WordForm;
    let word = |id: &str, form, features| Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    };
    let mut head = word(
        "vocab:ScalarDegree/Equal",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    head.frame = Some(0);
    let marker = word(
        "vocab:Preposition/To",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let possessor = word(
        "vocab:PossessiveDeterminerPronoun/Its",
        WordForm::Invariant,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            case: Some(Case::Genitive),
            ..FeatureBundle::default()
        },
    );
    let mut power = word(
        "lexeme:CommonNoun/Power",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        },
    );
    power.countability = Some(true);
    let value = Reading::ComparativeAdjective {
        form: 0,
        phrase: Box::new(Reading::ComparativeAdjectivePhrase {
            form: 0,
            governor: Box::new(Reading::ComparativeGovernorHead {
                form: 0,
                head: head.clone(),
                marker: marker.clone(),
            }),
            complement: Box::new(Reading::NominalComparativeComplement {
                form: 0,
                value: Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(Reading::PossessiveNounPhrase {
                        form: 0,
                        possessor: possessor.clone(),
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: power.clone(),
                        }),
                    }),
                }),
            }),
        }),
    };
    // Soul's Fire's comparative constituent, constructed independently.
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), "equal to its power");
    assert_eq!(
        readings("equal to its power", Category::AdjectivePhrase),
        std::collections::BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        [head.clone(), marker.clone(), possessor, power.clone()]
    );

    // Sandbender Scavengers / Gurgling Anointer: one bare Complement is
    // shared by two governors, each retaining its own selected marker.
    let mut less = word(
        "vocab:Adjective/Less",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    less.frame = Some(0);
    let than = word(
        "vocab:Preposition/Than",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let or = word(
        "vocab:Coordinator/Or",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let this = word(
        "vocab:SingularDemonstrative/This",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let mut creature = word(
        "lexeme:type/creature",
        WordForm::Singular,
        FeatureBundle {
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        },
    );
    creature.countability = Some(true);
    let genitive = word(
        "vocab:Genitive/Default",
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let shared = Reading::ComparativeAdjective {
        form: 0,
        phrase: Box::new(Reading::ComparativeAdjectivePhrase {
            form: 0,
            governor: Box::new(Reading::Coordination {
                category: Category::ComparativeGovernorHead,
                form: 0,
                left: Box::new(Reading::ComparativeGovernorHead {
                    form: 0,
                    head: less.clone(),
                    marker: than.clone(),
                }),
                coordinator: or.clone(),
                right: Box::new(Reading::ComparativeGovernorHead {
                    form: 0,
                    head: head.clone(),
                    marker: marker.clone(),
                }),
            }),
            complement: Box::new(Reading::NominalComparativeComplement {
                form: 0,
                value: Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(Reading::GenitiveNounPhrase {
                        form: 0,
                        possessor: Box::new(Reading::DeterminedNounPhrase {
                            form: 0,
                            determiner: this.clone(),
                            head: Box::new(Reading::Noun {
                                form: 0,
                                head: creature.clone(),
                            }),
                        }),
                        marker: genitive.clone(),
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: power.clone(),
                        }),
                    }),
                }),
            }),
        }),
    };
    shared.admit(lexicon()).unwrap();
    let text = "less than or equal to this creature's power";
    assert_eq!(shared.realize(lexicon()).unwrap(), text);
    assert_eq!(
        readings(text, Category::AdjectivePhrase),
        std::collections::BTreeSet::from([shared.clone()])
    );
    let mut leaves = Vec::new();
    shared
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        [
            less, than, or, head, marker, this, creature, genitive, power
        ]
    );
}

#[test]
fn scalar_inequality_and_shared_governors_keep_their_complements() {
    // Sage-Eye Avengers; Spikeshell Harrier's comparative sentence;
    // Ghastly Demise; Massacre Girl, Known Killer's existing numeric case.
    for (text, comparison) in [
        (
            "Whenever this creature attacks, you may return target creature to its owner's hand if its power is less than this creature's power.",
            "less than this creature's power",
        ),
        (
            "If that opponent's speed is greater than each other player's speed, reduce that opponent's speed by 1.",
            "greater than each other player's speed",
        ),
        (
            "Destroy target nonblack creature if its toughness is less than or equal to the number of cards in your graveyard.",
            "less than or equal to the number of cards in your graveyard",
        ),
        (
            "Whenever a creature an opponent controls dies, if its toughness was less than 1, draw a card.",
            "less than 1",
        ),
    ] {
        assert_constituents(
            text,
            Category::Document,
            &[(Category::AdjectivePhrase, comparison)],
        );
    }
    assert_eq!(
        readings("less than this creature's power", Category::AdjectivePhrase).len(),
        1
    );
    assert_eq!(
        readings(
            "less than or equal to this creature's power",
            Category::AdjectivePhrase
        )
        .len(),
        1
    );
}

#[test]
fn non_scalar_inequality_is_postpositive_without_losing_bare_other() {
    // Arcbound Tracker; Worldslayer.
    for (text, object, comparison) in [
        (
            "Whenever you cast a spell other than your first spell each turn, put a +1/+1 counter on this creature.",
            "a spell other than your first spell",
            "other than your first spell",
        ),
        (
            "Whenever equipped creature deals combat damage to a player, destroy all permanents other than this Equipment.",
            "all permanents other than this Equipment",
            "other than this Equipment",
        ),
    ] {
        assert_constituents(
            text,
            Category::Document,
            &[
                (Category::NounPhrase, object),
                (Category::AdjectivePhrase, comparison),
            ],
        );
    }
    // Spikeshell Harrier retains ordinary attributive Other.
    assert!(!readings("each other player's speed", Category::NounPhrase).is_empty());
    assert_eq!(
        readings("other than this Equipment", Category::AdjectivePhrase).len(),
        1
    );
    assert!(readings("other to this Equipment", Category::AdjectivePhrase).is_empty());
    assert!(readings("equal than its power", Category::AdjectivePhrase).is_empty());
    assert!(readings("less to its power", Category::AdjectivePhrase).is_empty());
    assert!(readings("than its power", Category::PrepositionPhrase).is_empty());
}
