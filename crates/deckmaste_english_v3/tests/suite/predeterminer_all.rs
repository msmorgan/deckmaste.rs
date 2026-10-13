use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, Lexicon, Number, Person, SurfaceCase, WordForm};

use crate::common;

fn word(owner: &str, form: WordForm, countability: Option<bool>) -> Word {
    let mut value = common::lexicon()
        .values()
        .find(|value| value.lexeme.as_str() == owner && value.form == form && value.variant == 0)
        .unwrap()
        .clone();
    value.capitalization = SurfaceCase::Declared;
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability,
    }
}

fn universal(owner: &str, host: Reading) -> Reading {
    Reading::PredeterminerNounPhrase {
        form: 2,
        modifier: word(owner, WordForm::Invariant, None),
        head: Box::new(host),
    }
}

fn determined(owner: &str, determiner: &str, form: WordForm, countable: bool) -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(determiner, WordForm::Invariant, None),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word(owner, form, Some(countable)),
                }),
            }),
        }),
    }
}

#[test]
fn independently_constructed_universal_quantifiers_keep_both_roundtrip_laws() {
    // Tolarian Winds supplies the definite-card host. The owner principle
    // licenses both as another member of the same universal-quantifier class.
    for (owner, determiner, noun, form, countable, text) in [
        (
            "vocab:FloatedQuantifier/All",
            "vocab:DefiniteMarker/The",
            "lexeme:CommonNoun/Card",
            WordForm::Plural,
            true,
            "all the cards",
        ),
        (
            "vocab:FloatedQuantifier/All",
            "vocab:Determinative/Those",
            "lexeme:CommonNoun/Card",
            WordForm::Plural,
            true,
            "all those cards",
        ),
        (
            "vocab:FloatedQuantifier/Both",
            "vocab:DefiniteMarker/The",
            "lexeme:type/creature",
            WordForm::Plural,
            true,
            "both the creatures",
        ),
        (
            "vocab:FloatedQuantifier/All",
            "vocab:DefiniteMarker/The",
            "lexeme:CommonNoun/Damage",
            WordForm::Singular,
            false,
            "all the damage",
        ),
    ] {
        let value = universal(owner, determined(noun, determiner, form, countable));
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        assert_eq!(common::noun_phrase_readings(text), BTreeSet::from([value]));
    }
    let host = Reading::PossessiveNounPhrase {
        form: 0,
        possessor: word(
            "vocab:PossessiveDeterminerPronoun/Your",
            WordForm::Invariant,
            None,
        ),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word("lexeme:type/creature", WordForm::Plural, Some(true)),
                }),
            }),
        }),
    };
    let value = universal("vocab:FloatedQuantifier/All", host);
    value.admit(common::lexicon()).unwrap();
    assert_eq!(
        value.realize(common::lexicon()).unwrap(),
        "all your creatures"
    );
    assert_eq!(
        common::noun_phrase_readings("all your creatures"),
        BTreeSet::from([value])
    );
}

fn witness(card: &str, text: &str, host_surface: &str) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    let control = text.replacen("all ", "", 1);
    let mut expected_hosts = BTreeSet::new();
    for value in common::readings_on_card(card, &control, Category::Document) {
        value
            .visit(&mut |node| {
                if node.category() == Category::NounPhrase
                    && host_surface.starts_with(&node.realize(common::lexicon()).unwrap())
                {
                    expected_hosts.insert(node.clone());
                }
            })
            .unwrap();
    }
    let mut observed_hosts = BTreeSet::new();
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::PredeterminerNounPhrase { modifier, head, .. } = node {
                    modifiers.push((modifier.clone(), (**head).clone()));
                }
            })
            .unwrap();
        let [(modifier, head)] = modifiers.as_slice() else {
            panic!("{card}: expected one external Modifier of a Noun Phrase");
        };
        assert_eq!(
            *modifier,
            word("vocab:FloatedQuantifier/All", WordForm::Invariant, None)
        );
        assert!(
            expected_hosts.contains(head),
            "{card}: altered host Reading"
        );
        observed_hosts.insert(head.clone());
        value.admit(common::lexicon()).unwrap();
    }
    assert_eq!(
        observed_hosts, expected_hosts,
        "{card}: lost host ambiguity"
    );
}

#[test]
fn tolarian_winds() {
    witness(
        "Tolarian Winds",
        "Discard all the cards in your hand, then draw that many cards.",
        "the cards in your hand",
    );
}

#[test]
fn awaken_the_erstwhile() {
    witness(
        "Awaken the Erstwhile",
        "Each player discards all the cards in their hand, then creates that many 2/2 black Zombie creature tokens.",
        "the cards in their hand",
    );
}

#[test]
fn robber_fly() {
    witness(
        "Robber Fly",
        "Whenever this creature becomes blocked, defending player discards all the cards in their hand, then draws that many cards.",
        "the cards in their hand",
    );
}

#[test]
fn endurance() {
    witness(
        "Endurance",
        "When this creature enters, up to one target player puts all the cards from their graveyard on the bottom of their library in a random order.",
        "the cards from their graveyard",
    );
}

#[test]
fn forgotten_creation() {
    witness(
        "Forgotten Creation",
        "At the beginning of your upkeep, you may discard all the cards in your hand.",
        "the cards in your hand",
    );
}

#[test]
fn existing_routes_keep_their_complete_reading_sets() {
    let without_universal_predeterminers = Lexicon::new(
        common::lexicon()
            .lexemes()
            .values()
            .cloned()
            .map(|mut lexeme| {
                if lexeme
                    .properties
                    .features
                    .get("PredeterminerUse")
                    .is_some_and(|value| value == "UniversalQuantifier")
                {
                    lexeme.properties.features.remove("PredeterminerUse");
                }
                lexeme
            }),
    )
    .unwrap();
    for text in [
        "all cards in your hand",
        "the cards in your hand",
        "all other creatures",
        "all three cards",
        "all of the cards",
        "both creatures",
        "both of the cards",
        "twice that many cards",
        "half the cards",
    ] {
        let expected = common::readings_with_lexicon(
            &without_universal_predeterminers,
            text,
            Category::NounPhrase,
        );
        assert!(!expected.is_empty(), "{text}");
        assert_eq!(common::noun_phrase_readings(text), expected, "{text}");
    }
    let text = "Each player discards the cards in their hand, then draws that many cards.";
    assert_eq!(
        common::readings(text, Category::Document),
        common::readings_with_lexicon(&without_universal_predeterminers, text, Category::Document)
    );
}

#[test]
fn number_definiteness_and_stacking_constraints_are_bidirectional() {
    for text in [
        "all the card",
        "both the card",
        "both the damage",
        "all a card",
        "both a creature",
        "twice all the cards",
        "all twice the cards",
        "half all the cards",
        "all half the cards",
        "all all the cards",
        "both both the cards",
    ] {
        assert!(common::noun_phrase_readings(text).is_empty(), "{text}");
    }
    for (owner, host) in [
        (
            "vocab:FloatedQuantifier/All",
            determined(
                "lexeme:CommonNoun/Card",
                "vocab:DefiniteMarker/The",
                WordForm::Singular,
                true,
            ),
        ),
        (
            "vocab:FloatedQuantifier/Both",
            determined(
                "lexeme:CommonNoun/Damage",
                "vocab:DefiniteMarker/The",
                WordForm::Singular,
                false,
            ),
        ),
        (
            "vocab:FloatedQuantifier/All",
            Reading::BareNounPhrase {
                form: 0,
                head: Box::new(Reading::NominalProjection {
                    form: 0,
                    head: Box::new(Reading::NominalCoreProjection {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word("lexeme:CommonNoun/Card", WordForm::Plural, Some(true)),
                        }),
                    }),
                }),
            },
        ),
        (
            "vocab:FloatedQuantifier/All",
            Reading::PredeterminerNounPhrase {
                form: 1,
                modifier: word("lexeme:Fraction/Half", WordForm::Singular, Some(true)),
                head: Box::new(determined(
                    "lexeme:CommonNoun/Card",
                    "vocab:DefiniteMarker/The",
                    WordForm::Plural,
                    true,
                )),
            },
        ),
    ] {
        let invalid = universal(owner, host);
        assert!(invalid.admit(common::lexicon()).is_err());
        assert!(invalid.realize(common::lexicon()).is_err());
    }
}

#[test]
fn the_host_exports_plural_subject_agreement() {
    // Reuse Night of Souls' Betrayal's real finite predicate to test the
    // independently constructed Subject's agreement, without minting an effect.
    let mut predicates = BTreeSet::new();
    for value in common::readings("All creatures get -1/-1.", Category::Document) {
        value
            .visit(&mut |node| {
                if let Reading::FiniteClause { predicate, .. } = node {
                    predicates.insert((**predicate).clone());
                }
            })
            .unwrap();
    }
    assert!(!predicates.is_empty());
    for owner in [
        "vocab:FloatedQuantifier/All",
        "vocab:FloatedQuantifier/Both",
    ] {
        let mut subject = universal(
            owner,
            determined(
                "lexeme:type/creature",
                "vocab:DefiniteMarker/The",
                WordForm::Plural,
                true,
            ),
        );
        let Reading::PredeterminerNounPhrase { modifier, .. } = &mut subject else {
            unreachable!()
        };
        let LexicalReading::Word(value) = &mut modifier.value else { unreachable!() };
        value.capitalization = SurfaceCase::Initial;
        for predicate in &predicates {
            let clause = Reading::FiniteClause {
                form: 0,
                subject: Box::new(Reading::CasePhrase {
                    form: 0,
                    category: Category::NominativePhrase,
                    head: Box::new(subject.clone()),
                }),
                predicate: Box::new(predicate.clone()),
            };
            clause.admit(common::lexicon()).unwrap();
            let mut singular = predicate.clone();
            let Reading::SelectedPredicate { head, .. } = &mut singular else {
                panic!("expected the selected get predicate")
            };
            let LexicalReading::Word(original) = &head.value else { unreachable!() };
            let replacement = common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme == original.lexeme
                        && value.form == WordForm::Present
                        && value.features.number == Some(Number::Singular)
                        && value.features.person == Some(Person::Third)
                })
                .unwrap()
                .clone();
            head.value = LexicalReading::Word(replacement);
            let Reading::FiniteClause { subject, .. } = clause else { unreachable!() };
            assert!(
                Reading::FiniteClause {
                    form: 0,
                    subject,
                    predicate: Box::new(singular)
                }
                .admit(common::lexicon())
                .is_err()
            );
        }
    }
}
