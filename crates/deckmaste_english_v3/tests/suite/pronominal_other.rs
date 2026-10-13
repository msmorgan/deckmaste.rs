use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

fn word(owner: &str, form: WordForm, number: Option<Number>, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number,
                ..FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability,
    }
}

fn other_noun() -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(
                    "lexeme:CommonNoun/Other",
                    WordForm::Singular,
                    Some(Number::Singular),
                    Some(true),
                ),
            }),
        }),
    }
}

fn the_other() -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word("vocab:DefiniteMarker/The", WordForm::Invariant, None, None),
        head: Box::new(other_noun()),
    }
}

#[test]
fn ticket_witnesses_have_other_as_the_noun_head_of_a_definite_np() {
    let expected = the_other();
    for (card, text) in [
        (
            "Nagging Thoughts",
            "Look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
        ),
        (
            "Sibsig Appraiser",
            "When this creature enters, look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
        ),
        (
            "Talas Lookout",
            "When this creature dies, look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
        ),
        (
            "Tower Geist",
            "When this creature enters, look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
        ),
        (
            "Write into Being",
            "Manifest one of those cards, then put the other on the top or bottom of your library.",
        ),
    ] {
        let values = common::readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut found = false;
            value.visit(&mut |node| found |= node == &expected).unwrap();
            assert!(
                found,
                "{card}: expected definite NP with the declared noun head"
            );
        }
    }
}

#[test]
fn independent_definite_np_roundtrips_and_preserves_the_lexical_values() {
    let expected = the_other();
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "the other");
    assert_eq!(
        common::noun_phrase_readings("the other"),
        [expected.clone()].into()
    );
    let mut words = Vec::new();
    expected
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    let Reading::DeterminedNounPhrase {
        determiner, head, ..
    } = expected
    else {
        unreachable!()
    };
    let Reading::Noun { head: noun, .. } = crate::common::into_nominal_core(*head) else {
        unreachable!()
    };
    assert_eq!(words, [determiner, noun]);
}

#[test]
fn other_requires_the_selected_determiner_class() {
    // FH7 negatives, including the reciprocal that has no current analysis.
    for text in [
        "each other",
        "any other",
        "no other",
        "an other",
        "some other",
    ] {
        assert!(common::noun_phrase_readings(text).is_empty(), "{text}");
    }
    assert!(common::readings("Those creatures fight each other.", Category::Document).is_empty());
    for owner in [
        "vocab:FloatedQuantifier/Each",
        "vocab:Determinative/Any",
        "vocab:Determinative/No",
        "vocab:Determinative/Some",
    ] {
        let invalid = Reading::DeterminedNounPhrase {
            form: 0,
            determiner: word(owner, WordForm::Invariant, None, None),
            head: Box::new(other_noun()),
        };
        assert!(invalid.admit(common::lexicon()).is_err(), "{owner}");
    }
    let mut article = word(
        "vocab:Article/Indefinite",
        WordForm::Invariant,
        Some(Number::Singular),
        None,
    );
    let LexicalReading::Word(lexical) = &mut article.value else { unreachable!() };
    lexical.variant = 1;
    let invalid = Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: article,
        head: Box::new(other_noun()),
    };
    assert!(invalid.admit(common::lexicon()).is_err());
}

#[test]
fn other_before_an_overt_noun_remains_an_adjective() {
    let mut adjective = word(
        "vocab:AttributiveAdjective/Other",
        WordForm::Invariant,
        None,
        None,
    );
    adjective.frame = Some(0);
    let expected = Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::IntransitiveAdjective {
            form: 0,
            head: adjective,
        }),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word(
                        "lexeme:CommonNoun/Card",
                        WordForm::Singular,
                        Some(Number::Singular),
                        Some(true),
                    ),
                }),
            }),
        }),
    };
    let noun = other_noun();
    for text in ["the other card", "one other card", "any other card"] {
        let values = common::noun_phrase_readings(text);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut found = false;
            value
                .visit(&mut |node| {
                    found |= node == &expected;
                    assert_ne!(node, &noun, "{text}");
                })
                .unwrap();
            assert!(
                found,
                "{text}: expected an adjective premodifying the noun head"
            );
        }
    }
    assert!(common::noun_phrase_readings("the others").is_empty());
    assert!(common::noun_phrase_readings("other card").is_empty());
}

#[test]
fn selection_survives_nominal_coordination_and_postmodification() {
    // The restriction must not be erased by ordinary Nominal composition.
    for text in [
        "any other and card",
        "some other and card",
        "any other you control",
        "an other you control",
        "your other",
        "that player's other",
    ] {
        assert!(common::noun_phrase_readings(text).is_empty(), "{text}");
    }
}
