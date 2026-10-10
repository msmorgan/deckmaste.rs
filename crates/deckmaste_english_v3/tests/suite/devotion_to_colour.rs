use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm,
};

use crate::common::{assert_constituents, lexicon, readings, readings_on_card};

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

fn colour(owner: &str) -> Reading {
    Reading::Adjective {
        form: 0,
        head: word(owner),
    }
}

fn selected(complement: Reading) -> Reading {
    let mut head = word("lexeme:CommonNoun/Devotion");
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = WordForm::Singular;
    value.features.number = Some(Number::Singular);
    head.frame = Some(2);
    head.countability = Some(false);
    Reading::PrepositionComplementNominal {
        form: 2,
        head,
        marker: word("vocab:Preposition/To"),
        complement: Box::new(complement),
    }
}

fn possessive(head: Reading) -> Reading {
    let mut possessor = word("vocab:PossessiveDeterminerPronoun/Your");
    let LexicalReading::Word(value) = &mut possessor.value else { unreachable!() };
    value.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Genitive),
        ..Default::default()
    };
    Reading::PossessiveNounPhrase {
        form: 0,
        possessor,
        head: Box::new(head),
    }
}

#[test]
fn independently_constructed_colour_complements_roundtrip() {
    // Erebos and Pharika supply these constituents. The colour Coordination
    // belongs to the selected Complement, below the possessive Noun Phrase.
    let black = colour("vocab:ColorWord/Black");
    let coordinated = Reading::Coordination {
        form: 0,
        category: Category::AdjectivePhrase,
        left: Box::new(black.clone()),
        coordinator: word("vocab:Coordinator/And"),
        right: Box::new(colour("vocab:ColorWord/Green")),
    };
    for (text, complement) in [
        ("your devotion to black", black),
        ("your devotion to black and green", coordinated),
    ] {
        let expected = possessive(selected(complement));
        expected.admit(lexicon()).unwrap();
        assert_eq!(expected.realize(lexicon()).unwrap(), text);
        assert_eq!(
            readings(text, Category::NounPhrase),
            BTreeSet::from([expected])
        );
    }
}

#[test]
fn witnesses_select_the_colour_complement_on_devotion() {
    for (card, text, owner) in [
        (
            "Nylea's Disciple",
            "When this creature enters, you gain life equal to your devotion to green.",
            "vocab:ColorWord/Green",
        ),
        (
            "Tymaret, Chosen from Death",
            "Tymaret's toughness is equal to your devotion to black.",
            "vocab:ColorWord/Black",
        ),
        (
            "Gray Merchant of Asphodel",
            "When this creature enters, each opponent loses X life, where X is your devotion to black. You gain life equal to the life lost this way.",
            "vocab:ColorWord/Black",
        ),
        (
            "Master of Waves",
            "When this creature enters, create a number of 1/0 blue Elemental creature tokens equal to your devotion to blue.",
            "vocab:ColorWord/Blue",
        ),
    ] {
        let values = readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut complements = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::PrepositionComplementNominal { .. } = node {
                        complements.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(complements, vec![selected(colour(owner))], "{card}");
        }
    }
}

#[test]
fn devotion_retains_scalar_denotation_in_subject_and_value_roles() {
    assert_constituents(
        "Your devotion to black is 5.",
        Category::Sentence,
        &[
            (Category::NominativePhrase, "Your devotion to black"),
            (Category::ScalarValue, "5"),
        ],
    );
    // Gray Merchant's variable-defining Supplement.
    assert_constituents(
        "X is your devotion to black",
        Category::FiniteClause,
        &[(Category::NounPhrase, "your devotion to black")],
    );
}

#[test]
fn selected_noun_phrase_complement_retires_devotion_modifier_attachment() {
    // Nykthos, Nyx Lotus, and Altar of the Pantheon.
    for text in [
        "your devotion to that color",
        "your devotion to each color and each combination of colors",
    ] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty(), "{text}");
        let mut selected_complements = BTreeSet::new();
        for value in values {
            value
                .visit(&mut |node| {
                    if let Reading::PrepositionComplementNominal { complement, .. } = node {
                        selected_complements.insert(complement.realize(lexicon()).unwrap());
                    }
                    if let Reading::PostmodifiedNominal { head, modifier, .. } = node {
                        assert!(
                            head.realize(lexicon()).unwrap() != "devotion"
                                || !modifier.realize(lexicon()).unwrap().starts_with("to "),
                            "a selected noun Complement must not be a Modifier",
                        );
                    }
                })
                .unwrap();
        }
        let expected = text.strip_prefix("your devotion to ").unwrap();
        assert!(selected_complements.contains(expected), "{text}");
    }
}

#[test]
fn colour_complement_use_does_not_create_nominal_or_elided_noun_readings() {
    for text in ["black", "black and green"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for text in [
        "your devotion of black",
        "your devotion to legendary",
        "your devotion to black and legendary",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    // Deathlace retains an adjectival predicative Complement.
    let text = "Target spell or permanent becomes black.";
    let values = readings(text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut adjectives = BTreeSet::new();
        value
            .visit(&mut |node| {
                if let Reading::AdjectivalComplement { phrase, .. } = node {
                    adjectives.insert(phrase.realize(lexicon()).unwrap());
                }
            })
            .unwrap();
        assert_eq!(adjectives, BTreeSet::from(["black".to_owned()]));
    }
}

#[test]
fn protection_keeps_its_existing_adjectival_quality() {
    let mut head = word("lexeme:keyword_ability/protection");
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
    let expected = Reading::QualityKeyword {
        form: 0,
        head,
        quality: Box::new(Reading::KeywordQualityPreposition {
            form: 0,
            head: word("vocab:Preposition/From"),
            quality: Box::new(Reading::AdjectivalKeywordQuality {
                form: 0,
                phrase: Box::new(colour("vocab:ColorWord/Black")),
            }),
        }),
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(
        readings("Protection from black", Category::KeywordPhrase),
        BTreeSet::from([expected])
    );
}
