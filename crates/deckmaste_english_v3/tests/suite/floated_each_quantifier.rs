use std::collections::BTreeSet;

use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, Finiteness, LexicalReading, LexicalValue, Number, Person, SurfaceCase, Tense,
    WordForm,
};

#[test]
fn ticket_witnesses_quantify_the_plural_subject_and_roundtrip() {
    for (card, text, construction) in [
        (
            "Lord of Extinction",
            "Lord of Extinction's power and toughness are each equal to the number of cards in all graveyards.",
            "PostAuxiliaryQuantification",
        ),
        (
            "Fated Clash",
            "Target creature you control and target creature an opponent controls each gain indestructible until end of turn.",
            "QuantifiedClause",
        ),
        (
            "Press the Advantage",
            "Up to two target creatures each get +2/+2 and gain trample until end of turn.",
            "QuantifiedClause",
        ),
        (
            "Skull Rend",
            "Those players each discard two cards at random.",
            "QuantifiedClause",
        ),
        (
            "Join Forces",
            "They each get +2/+2 until end of turn.",
            "QuantifiedClause",
        ),
    ] {
        let values = common::readings_on_card(card, text, Category::Sentence);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut occurrences = 0;
            value
                .visit(&mut |node| {
                    if node.construction() == construction {
                        occurrences += 1;
                    }
                })
                .unwrap();
            assert_eq!(occurrences, 1, "{card}");
        }
    }
}

#[test]
fn preverbal_each_scopes_over_the_coordinated_predicate() {
    common::assert_constituents(
        "Up to two target creatures each get +2/+2 and gain trample until end of turn.",
        Category::Sentence,
        &[
            (Category::NominativePhrase, "Up to two target creatures"),
            (
                Category::FinitePredicate,
                "get +2/+2 and gain trample until end of turn",
            ),
        ],
    );
    common::assert_constituents(
        "Target creature you control and target creature an opponent controls each gain indestructible until end of turn.",
        Category::Sentence,
        &[(
            Category::NominativePhrase,
            "Target creature you control and target creature an opponent controls",
        )],
    );
}

#[test]
fn singular_subjects_and_post_lexical_each_are_rejected() {
    for text in [
        "That player each discards two cards at random.",
        "Target creature each gets +2/+2 until end of turn.",
        "Those players discard each two cards at random.",
        "They get each +2/+2 until end of turn.",
        "Those players each each discard two cards at random.",
        "Lord of Extinction's power is each equal to the number of cards in all graveyards.",
    ] {
        assert!(
            common::readings(text, Category::Sentence).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn post_auxiliary_each_preserves_modal_and_scalar_complements() {
    // Attested constituents of Turtle Tracks and Angry Mob.
    common::assert_constituents_on_card(
        "Turtle Tracks",
        "Any number of target players may each search their library for a basic land card",
        Category::FiniteClause,
        &[(
            Category::FinitePredicate,
            "may each search their library for a basic land card",
        )],
    );
    common::assert_constituents_on_card(
        "Angry Mob",
        "Angry Mob's power and toughness are each 2.",
        Category::Sentence,
        &[(Category::ScalarLocationPredicate, "are each 2")],
    );
    assert!(
        common::readings_on_card(
            "Angry Mob",
            "Angry Mob's power is each 2.",
            Category::Sentence,
        )
        .is_empty()
    );
}

fn word(owner: &str, form: WordForm, features: FeatureBundle, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

#[test]
fn independently_constructed_post_auxiliary_head_retains_its_words_and_frame() {
    // Lord of Extinction's post-auxiliary constituent, constructed without parsing.
    let head = word(
        "core-verb:Be",
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Plural),
            person: Some(Person::Third),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..FeatureBundle::default()
        },
        Some(0),
    );
    let quantifier = word(
        "vocab:FloatedQuantifier/Each",
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
    );
    let value = Reading::QuantifiedAuxiliaryHead {
        form: 0,
        head: head.clone(),
        quantifier: quantifier.clone(),
    };
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), "are each");
    assert!(common::readings("are each", Category::QuantifiedAuxiliaryHead).contains(&value));
    let mut words = Vec::new();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    assert_eq!(words, [head, quantifier]);
    let mut singular = value.clone();
    let Reading::QuantifiedAuxiliaryHead { head, .. } = &mut singular else {
        unreachable!()
    };
    let LexicalReading::Word(lexical) = &mut head.value else { unreachable!() };
    lexical.features.number = Some(Number::Singular);
    assert!(singular.admit(common::lexicon()).is_err());
    let mut unlicensed = value;
    let Reading::QuantifiedAuxiliaryHead { quantifier, .. } = &mut unlicensed else {
        unreachable!()
    };
    *quantifier = word(
        "vocab:FloatedQuantifier/All",
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
    );
    assert!(unlicensed.admit(common::lexicon()).is_err());
}

#[test]
fn each_determiner_and_partitive_readings_are_preserved() {
    assert!(!common::readings("Each player draws a card.", Category::Sentence).is_empty());
    let partitive = common::readings("each of those players", Category::NounPhrase);
    assert!(!partitive.is_empty());
    let mut constructions = BTreeSet::new();
    for value in partitive {
        value
            .visit(&mut |node| {
                constructions.insert(node.construction());
            })
            .unwrap();
    }
    assert!(!constructions.contains("QuantifiedClause"));
}

#[test]
fn relative_subject_each_retains_plural_agreement() {
    // Ecological Appreciation and Uncage the Menagerie.
    let text = "that each have mana value X or less";
    assert!(!common::readings(text, Category::SubjectRelativeClause).is_empty());
    assert!(
        common::readings(
            "that each has mana value X or less",
            Category::SubjectRelativeClause
        )
        .is_empty()
    );
    assert!(
        !common::readings(
            "that each have mana value X",
            Category::SubjectRelativeClause
        )
        .is_empty()
    );
}
