use std::collections::BTreeSet;

use crate::common::{lexicon, readings, readings_with_lexicon};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Lexicon, Number, SurfaceCase, WordForm,
};

fn noun_word(owner: &str) -> Word {
    Word {
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
        countability: Some(true),
    }
}

fn bare_complement() -> Reading {
    Reading::PrepositionComplementNominal {
        form: 1,
        head: noun_word("lexeme:CommonNoun/Kind"),
        marker: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:Preposition/Of".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
        complement: Box::new(Reading::Noun {
            form: 0,
            head: noun_word("lexeme:CommonNoun/Counter"),
        }),
    }
}

fn check_taxonomic_roles(value: &Reading) {
    let mut found = false;
    value
        .visit(&mut |node| {
            if let Reading::PrepositionComplementNominal { complement, .. } = node
                && complement.category() == Category::Nominal
            {
                // The noun head selects the bare Nominal, rather than an NP or a verb.
                assert_eq!(node, &bare_complement());
                found = true;
            }
        })
        .unwrap();
    assert!(found, "missing taxonomic Complement");
}

#[test]
fn independently_constructed_taxonomic_complement_obeys_both_roundtrip_laws() {
    // Contractual Safeguard: "Choose a kind of counter on a creature you control."
    let expected = bare_complement();
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "kind of counter");
    assert_eq!(
        readings("kind of counter", Category::Nominal),
        BTreeSet::from([expected])
    );
}

#[test]
fn postmodifiers_keep_the_ordinary_nominal_attachment_shapes() {
    // Ferrafor, Gilder Bairn, Contractual Safeguard, and the for-each-kind witnesses.
    for (taxonomic, ordinary) in [
        (
            "each kind of counter on target creature",
            "each counter on target creature",
        ),
        (
            "each kind of counter on target permanent",
            "each counter on target permanent",
        ),
        (
            "a kind of counter on a creature you control",
            "a counter on a creature you control",
        ),
        (
            "each kind of counter among permanents you control",
            "each counter among permanents you control",
        ),
    ] {
        let taxonomic = readings(taxonomic, Category::NounPhrase);
        assert!(!taxonomic.is_empty());
        let shapes = |values: &BTreeSet<Reading>, taxonomic: bool| {
            values
                .iter()
                .map(|value| {
                    let mut shape = Vec::new();
                    value
                        .visit(&mut |node| {
                            // Erasing the selecting taxonomic head leaves the ordinary noun's structure.
                            if !(taxonomic && node.construction() == "PrepositionComplementNominal")
                            {
                                shape.push(node.construction());
                            }
                        })
                        .unwrap();
                    shape
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            shapes(&taxonomic, true),
            shapes(&readings(ordinary, Category::NounPhrase), false)
        );
        for value in taxonomic {
            check_taxonomic_roles(&value);
        }
    }
}

#[test]
fn ticket_witnesses_use_the_licensed_nominal_complement() {
    for (text, category) in [
        (
            "{T}: Double the number of each kind of counter on target creature.",
            Category::Document,
        ),
        (
            "Choose a kind of counter on a creature you control.",
            Category::Sentence,
        ),
        (
            "Whenever you attack, double the number of each kind of counter on each Spider and legendary creature you control.",
            Category::Sentence,
        ),
        (
            "Double the number of each kind of counter on target permanent.",
            Category::Sentence,
        ),
        (
            "Double the number of each kind of counter on target artifact, creature, or land.",
            Category::Sentence,
        ),
    ] {
        let values = readings(text, category);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            check_taxonomic_roles(&value);
        }
    }
}

#[test]
fn bare_count_complement_requires_the_declared_head_licence() {
    let mut lexemes: Vec<_> = lexicon().lexemes().values().cloned().collect();
    for lexeme in &mut lexemes {
        lexeme.properties.features.remove("TaxonomicComplementUse");
    }
    let unlicensed = Lexicon::new(lexemes).unwrap();
    assert!(readings_with_lexicon(&unlicensed, "kind of counter", Category::Nominal).is_empty());
    assert!(readings("type of counter", Category::Nominal).is_empty());
    assert!(readings("counter", Category::NounPhrase).is_empty());
    assert!(readings("of counter", Category::PrepositionPhrase).is_empty());
}

#[test]
fn existing_controls_retain_their_ordinary_np_complements() {
    for text in [
        "Put a counter of that kind on each other creature you control.",
        "Put a counter of each kind on it.",
        "Remove a counter of each kind from it.",
        "Double the number of counters on target creature.",
        "Double the number of +1/+1 counters on each creature you control.",
    ] {
        let values = readings(text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            value
                .visit(&mut |node| {
                    if let Reading::PrepositionComplementNominal { complement, .. } = node {
                        assert_eq!(complement.category(), Category::AccusativePhrase);
                    }
                })
                .unwrap();
        }
    }
}
