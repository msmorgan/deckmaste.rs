use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, Finiteness, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common::{LEXICON, readings, readings_on_card};

fn word(owner: &str, form: WordForm) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number: (form == WordForm::Singular).then_some(Number::Singular),
                finiteness: (form == WordForm::GerundParticiple).then_some(Finiteness::Nonfinite),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: (form == WordForm::GerundParticiple).then_some(0),
        countability: (form == WordForm::Singular).then_some(true),
    }
}

fn tapped() -> Reading {
    Reading::Adjective {
        form: 0,
        head: word("lexeme:keyword_action/tap/adjective", WordForm::Invariant),
    }
}

fn participle(owner: &str) -> Reading {
    Reading::VerbalPremodifier {
        category: Category::Premodifier,
        form: 0,
        head: word(owner, WordForm::GerundParticiple),
    }
}

fn nominal(modifier: Reading, owner: &str) -> Reading {
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
                            head: word(owner, WordForm::Singular),
                        }),
                    }),
                }),
            }),
        }),
    }
}

fn exact(value: Reading, text: &str) {
    value.admit(&LEXICON).unwrap();
    assert_eq!(value.realize(&LEXICON).unwrap(), text);
    assert_eq!(readings(text, Category::Nominal), BTreeSet::from([value]));
}

#[test]
fn independently_constructed_pair_preserves_categories_and_one_modifier() {
    // Furnace-Blessed Conqueror and Tetsuo Umezawa supply these constituents.
    for (coordinator, verb, owner, text) in [
        (
            "vocab:Coordinator/And",
            "core-verb:Attack",
            "lexeme:CommonNoun/Token",
            "tapped and attacking token",
        ),
        (
            "vocab:Coordinator/Or",
            "core-verb:Block",
            "lexeme:type/creature",
            "tapped or blocking creature",
        ),
    ] {
        exact(
            nominal(
                Reading::Coordination {
                    category: Category::PremodifierCoordination,
                    form: 0,
                    left: Box::new(Reading::AdjectivePremodifierMember {
                        form: 0,
                        head: Box::new(tapped()),
                    }),
                    coordinator: word(coordinator, WordForm::Invariant),
                    right: Box::new(participle(verb)),
                },
                owner,
            ),
            text,
        );
    }
}

#[test]
fn independently_constructed_serial_list_has_one_shared_head() {
    // Sonar Strike's target description.
    exact(
        nominal(
            Reading::SerialCoordination {
                category: Category::PremodifierCoordination,
                form: 0,
                left: Box::new(participle("core-verb:Attack")),
                rest: Box::new(Reading::CoordinationSeriesEnd {
                    category: Category::Premodifier,
                    form: 0,
                    left: Box::new(participle("core-verb:Block")),
                    coordinator: word("vocab:Coordinator/Or", WordForm::Invariant),
                    right: Box::new(Reading::AdjectivePremodifierMember {
                        form: 0,
                        head: Box::new(tapped()),
                    }),
                }),
            },
            "lexeme:type/creature",
        ),
        "attacking, blocking, or tapped creature",
    );
}

fn witness(card: &str, text: &str, modifier_text: &str) {
    let values = readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::CoordinatedPremodifiedNominal { modifier, .. } = node {
                    modifiers.push(modifier.realize(&LEXICON).unwrap());
                }
                if let Reading::VerbalPremodifier { .. } = node {
                    assert_ne!(node.realize(&LEXICON).unwrap(), "tapped");
                }
            })
            .unwrap();
        assert_eq!(modifiers, vec![modifier_text], "{card}");
    }
}

#[test]
fn furnace_blessed_conqueror() {
    witness(
        "Furnace-Blessed Conqueror",
        "Whenever this creature attacks, create a tapped and attacking token that's a copy of it.",
        "tapped and attacking",
    );
}

#[test]
fn pugnacious_pugilist() {
    witness(
        "Pugnacious Pugilist",
        "Whenever this creature attacks, create a tapped and attacking 1/1 red Devil creature token with \"When this token dies, it deals 1 damage to any target.\"",
        "tapped and attacking",
    );
}

#[test]
fn krothuss() {
    witness(
        "Krothuss, Lord of the Deep",
        "Create a tapped and attacking token that's a copy of another target attacking creature.",
        "tapped and attacking",
    );
}

#[test]
fn tetsuo_umezawa() {
    witness(
        "Tetsuo Umezawa",
        "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking creature.",
        "tapped or blocking",
    );
}

#[test]
fn sonar_strike() {
    witness(
        "Sonar Strike",
        "Sonar Strike deals 4 damage to target attacking, blocking, or tapped creature.",
        "attacking, blocking, or tapped",
    );
}
