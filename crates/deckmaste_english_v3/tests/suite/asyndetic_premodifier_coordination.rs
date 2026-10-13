use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common::LEXICON;
use crate::common::readings;
use crate::common::readings_on_card;

fn word(owner: &str, noun: bool) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: if noun { WordForm::Singular } else { WordForm::Invariant },
            features: FeatureBundle {
                number: noun.then_some(Number::Singular),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: noun.then_some(true),
    }
}

fn noun_member(owner: &str) -> Reading {
    Reading::NounPremodifierMember {
        form: 0,
        head: Box::new(Reading::NounPremodifier {
            form: 0,
            head: word(owner, true),
        }),
    }
}

fn adjective_member(owner: &str) -> Reading {
    Reading::AdjectivePremodifierMember {
        form: 0,
        head: Box::new(Reading::Adjective {
            form: 0,
            head: word(owner, false),
        }),
    }
}

fn pair(left: Reading, right: Reading) -> Reading {
    Reading::SerialCoordination {
        category: Category::PremodifierCoordination,
        form: 0,
        left: Box::new(left),
        rest: Box::new(right),
    }
}

fn modified(modifier: Reading, head_owner: &str) -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::CoordinatedPremodifiedNominal {
                form: 0,
                modifier: Box::new(modifier),
                head: Box::new(Reading::NominalProjection {
                    form: 0,
                    head: Box::new(Reading::NominalCoreProjection {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word(head_owner, true),
                        }),
                    }),
                }),
            }),
        }),
    }
}

fn witness(card: &str, text: &str, nominal: &str) {
    let documents = readings_on_card(card, text, Category::Document);
    assert!(!documents.is_empty(), "{card}");
    for document in documents {
        let mut shared = Vec::new();
        document
            .visit(&mut |node| {
                if let Reading::CoordinatedPremodifiedNominal { .. } = node {
                    shared.push(node.realize(&LEXICON).unwrap());
                }
            })
            .unwrap();
        assert_eq!(shared.len(), 1, "{card}");
        assert!(shared[0].starts_with(nominal), "{card}: {}", shared[0]);
    }
}

#[test]
fn terror() {
    witness(
        "Terror",
        "Destroy target nonartifact, nonblack creature.",
        "nonartifact, nonblack creature",
    );
}

#[test]
fn duress() {
    witness(
        "Duress",
        "You choose a noncreature, nonland card from it.",
        "noncreature, nonland card",
    );
}

#[test]
fn druidic_satchel() {
    witness(
        "Druidic Satchel",
        "If it's a noncreature, nonland card, you gain 2 life.",
        "noncreature, nonland card",
    );
}

#[test]
fn victim_of_night() {
    witness(
        "Victim of Night",
        "Destroy target non-Vampire, non-Werewolf, non-Zombie creature.",
        "non-Vampire, non-Werewolf, non-Zombie creature",
    );
}

#[test]
fn filter_out() {
    witness(
        "Filter Out",
        "Return all noncreature, nonland permanents to their owners' hands.",
        "noncreature, nonland permanents",
    );
}

#[test]
fn independently_constructed_mixed_members_share_one_head() {
    let expected = modified(
        pair(
            noun_member("lexeme:type/artifact/non"),
            adjective_member("vocab:ColorWord/Black/non"),
        ),
        "lexeme:type/creature",
    );
    expected.admit(&LEXICON).unwrap();
    let text = expected.realize(&LEXICON).unwrap();
    assert_eq!(text, "nonartifact, nonblack creature");
    assert_eq!(
        readings(&text, Category::Nominal),
        BTreeSet::from([expected])
    );
}

#[test]
fn independently_constructed_three_members_have_one_grouping() {
    let expected = modified(
        Reading::CoordinationSeriesContinuation {
            category: Category::PremodifierCoordination,
            form: 0,
            left: Box::new(noun_member("lexeme:creature_subtype/vampire/non")),
            rest: Box::new(pair(
                noun_member("lexeme:creature_subtype/werewolf/non"),
                noun_member("lexeme:creature_subtype/zombie/non"),
            )),
        },
        "lexeme:type/creature",
    );
    expected.admit(&LEXICON).unwrap();
    let text = expected.realize(&LEXICON).unwrap();
    assert_eq!(text, "non-Vampire, non-Werewolf, non-Zombie creature");
    assert_eq!(
        readings(&text, Category::Nominal),
        BTreeSet::from([expected])
    );
}

#[test]
fn existing_syndetic_nominal_reading_is_preserved() {
    let expected = Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Coordination {
                category: Category::NominalCore,
                form: 0,
                left: Box::new(Reading::NominalProjection {
                    form: 0,
                    head: Box::new(Reading::NominalCoreProjection {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word("lexeme:type/artifact/non", true),
                        }),
                    }),
                }),
                coordinator: word("vocab:Coordinator/And", false),
                right: Box::new(Reading::PremodifiedNominal {
                    form: 0,
                    modifier: Box::new(Reading::Adjective {
                        form: 0,
                        head: word("vocab:ColorWord/Black/non", false),
                    }),
                    head: Box::new(Reading::NominalProjection {
                        form: 0,
                        head: Box::new(Reading::NominalCoreProjection {
                            form: 0,
                            head: Box::new(Reading::Noun {
                                form: 0,
                                head: word("lexeme:type/creature", true),
                            }),
                        }),
                    }),
                }),
            }),
        }),
    };
    expected.admit(&LEXICON).unwrap();
    assert_eq!(
        readings("nonartifact and nonblack creature", Category::Nominal),
        BTreeSet::from([expected])
    );
}

#[test]
fn royal_decree_keeps_its_coordinator_ending_nominal_list() {
    let noun = |owner| Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(owner, true),
            }),
        }),
    };
    let coloured = |owner| Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::Adjective {
            form: 0,
            head: word(owner, false),
        }),
        head: Box::new(noun("lexeme:CommonNoun/Permanent")),
    };
    let expected = Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:Article/Indefinite".into(),
                form: WordForm::Invariant,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::SerialCoordination {
                    category: Category::NominalCore,
                    form: 0,
                    left: Box::new(noun("lexeme:land_subtype/swamp")),
                    rest: Box::new(Reading::CoordinationSeriesContinuation {
                        category: Category::NominalSeries,
                        form: 0,
                        left: Box::new(noun("lexeme:land_subtype/mountain")),
                        rest: Box::new(Reading::CoordinationSeriesEnd {
                            category: Category::NominalSeries,
                            form: 0,
                            left: Box::new(coloured("vocab:ColorWord/Black")),
                            coordinator: word("vocab:Coordinator/Or", false),
                            right: Box::new(coloured("vocab:ColorWord/Red")),
                        }),
                    }),
                }),
            }),
        }),
    };
    expected.admit(&LEXICON).unwrap();
    let text = "a Swamp, Mountain, black permanent, or red permanent";
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    assert_eq!(
        readings(text, Category::NounPhrase),
        BTreeSet::from([expected])
    );
}
