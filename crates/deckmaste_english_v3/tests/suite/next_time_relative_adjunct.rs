use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn witness(card: &str, text: &str) {
    let readings = common::readings_on_card(card, text, Category::Document);
    assert!(!readings.is_empty(), "{card}");
    for reading in readings {
        reading.admit(common::lexicon()).unwrap();
        assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
        let mut adjuncts = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::InitialPreposition { dependent, .. } = node
                    && let Reading::NominalAdjunctPhrase { head, .. } = dependent.as_ref()
                {
                    adjuncts.push(head.as_ref().clone());
                }
            })
            .unwrap();
        assert_eq!(
            adjuncts.len(),
            1,
            "{card}: temporal NP must be the preposed Adjunct"
        );
        let mut relatives = 0;
        adjuncts[0]
            .visit(&mut |node| {
                if let Reading::AdverbialGapRelativeClause { clause, .. } = node {
                    assert_eq!(clause.category(), Category::FiniteClause);
                    relatives += 1;
                }
            })
            .unwrap();
        assert_eq!(relatives, 1, "{card}: reuse the Adjunct-gap relative");
    }
}

#[test]
fn dazzling_reflection() {
    witness(
        "Dazzling Reflection",
        "The next time that creature would deal damage this turn, prevent that damage.",
    );
}

#[test]
fn words_of_waste() {
    witness(
        "Words of Waste",
        "{1}: The next time you would draw a card this turn, each opponent discards a card instead.",
    );
}

#[test]
fn new_way_forward() {
    witness(
        "New Way Forward",
        "The next time a source of your choice would deal damage to you this turn, prevent that damage.",
    );
}

#[test]
fn circle_of_protection_red() {
    witness(
        "Circle of Protection: Red",
        "{1}: The next time a red source of your choice would deal damage to you this turn, prevent that damage.",
    );
}

#[test]
fn words_of_wilding() {
    witness(
        "Words of Wilding",
        "{1}: The next time you would draw a card this turn, create a 2/2 green Bear creature token instead.",
    );
}

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str, countability: bool) -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: owner.into(),
                        form: WordForm::Singular,
                        features: FeatureBundle {
                            number: Some(Number::Singular),
                            ..Default::default()
                        },
                        variant: 0,
                        capitalization: SurfaceCase::Declared,
                    }),
                    frame: None,
                    countability: Some(countability),
                },
            }),
        }),
    }
}

fn laws(text: &str, value: &Reading) {
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, value.category()).contains(value));
}

#[test]
fn independently_constructed_definite_temporal_adjunct_requires_a_dependent() {
    // The attested constituent of Dazzling Reflection and the Words cycle.
    for countability in [false, true] {
        let mut value = Reading::NominalAdjunctPhrase {
            form: 0,
            determiner: invariant("vocab:DefiniteMarker/The", SurfaceCase::Declared),
            head: Box::new(Reading::NominalProjection {
                form: 0,
                head: Box::new(Reading::NominalCoreProjection {
                    form: 0,
                    head: Box::new(Reading::BarrierPremodifiedNominal {
                        form: 0,
                        modifier: Box::new(Reading::Adjective {
                            form: 0,
                            head: invariant(
                                "vocab:AttributiveAdjective/Next",
                                SurfaceCase::Declared,
                            ),
                        }),
                        head: Box::new(noun("lexeme:CommonNoun/Time", countability)),
                    }),
                }),
            }),
        };
        laws("the next time", &value);
        let Reading::NominalAdjunctPhrase { head, .. } = &mut value else {
            unreachable!()
        };
        **head = noun("lexeme:CommonNoun/Time", countability);
        assert!(value.admit(common::lexicon()).is_err());
    }
    assert!(common::readings("the time", Category::NominalAdjunctPhrase).is_empty());
    // Words of Waste: the relative remains inside the temporal NP Adjunct.
    let text = "the next time you would draw a card this turn";
    let relatives = common::readings(text, Category::NominalAdjunctPhrase);
    assert!(!relatives.is_empty());
    for relative in relatives {
        laws(text, &relative);
    }
}

#[test]
fn independently_preposed_this_turn_uses_the_same_construction_and_checks_form_category() {
    // Combine Guildmage / Zameck Guildmage, with the activated cost omitted.
    let text = "This turn, each creature you control enters with an additional +1/+1 counter on it";
    let clauses = common::readings(
        "each creature you control enters with an additional +1/+1 counter on it",
        Category::Clause,
    );
    assert!(!clauses.is_empty());
    for clause in clauses {
        let mut value = Reading::InitialPreposition {
            form: 2,
            dependent: Box::new(Reading::NominalAdjunctPhrase {
                form: 0,
                determiner: invariant("vocab:SingularDemonstrative/This", SurfaceCase::Initial),
                head: Box::new(noun("lexeme:CommonNoun/Turn", true)),
            }),
            clause: Box::new(clause),
        };
        laws(text, &value);
        let Reading::InitialPreposition { form, .. } = &mut value else {
            unreachable!()
        };
        *form = 0; // The PP form cannot accept the NP's child Category.
        assert!(value.admit(common::lexicon()).is_err());
    }
}

#[test]
fn each_time_requires_a_count_nominal() {
    let readings = common::readings("each time", Category::NominalAdjunctPhrase);
    assert_eq!(readings.len(), 1);
    laws("each time", readings.first().unwrap());
}

#[test]
fn preposed_licence_does_not_open_postposed_definite_or_manner_routes() {
    assert!(
        common::readings(
            "Prevent that damage the next time it would be dealt this turn.",
            Category::Sentence,
        )
        .is_empty()
    );
    assert!(common::readings("This way, prevent that damage.", Category::Sentence).is_empty());
    assert!(common::readings("The time, prevent that damage.", Category::Sentence).is_empty());
    // Existing nominal Adjunct distribution remains available.
    for text in ["this turn", "each turn", "this way"] {
        let values = common::readings(text, Category::NominalAdjunctPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            laws(text, &value);
        }
    }
    // The pre-existing temporal relative constituent keeps its complete set.
    let values = common::readings(
        "the first time you cast a spell each turn",
        Category::NounPhrase,
    );
    assert!(!values.is_empty());
    for value in values {
        laws("the first time you cast a spell each turn", &value);
    }
}
