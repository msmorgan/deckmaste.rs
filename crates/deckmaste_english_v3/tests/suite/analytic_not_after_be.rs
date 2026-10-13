use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::common;

const WITNESSES: [(&str, &str); 5] = [
    (
        "Force of Negation",
        "If it's not your turn, you may exile a blue card from your hand rather than pay this spell's mana cost.",
    ),
    (
        "Eddymurk Crab",
        "This creature enters tapped if it's not your turn.",
    ),
    (
        "Glademuse",
        "Whenever a player casts a spell, if it's not their turn, that player draws a card.",
    ),
    (
        "Arcades Sabboth",
        "Each untapped creature you control gets +0/+2 as long as it's not attacking.",
    ),
    (
        "Desynchronization",
        "Return each nonland permanent that's not historic to its owner's hand.",
    ),
];

fn synthetic(text: &str) -> String {
    text.replace("it's not", "it isn't")
        .replace("that's not", "that isn't")
        .replace("It's not", "It isn't")
        .replace("They're not", "They aren't")
        .replace("You're not", "You aren't")
}

#[test]
fn witnesses_keep_negation_on_the_auxiliary_and_the_ordinary_complement() {
    for (card, text) in WITNESSES {
        let analytic = common::readings_on_card(card, text, Category::Sentence);
        let controls = common::readings_on_card(card, &synthetic(text), Category::Sentence);
        assert!(!analytic.is_empty(), "{card}");
        assert_eq!(analytic.len(), controls.len(), "{card}");
        let mut control_complements = std::collections::BTreeSet::new();
        let mut control_participles = std::collections::BTreeSet::new();
        for control in &controls {
            control
                .visit(&mut |node| {
                    if let Reading::ParticipialAuxiliaryPredicate {
                        head, complement, ..
                    } = node
                        && let LexicalReading::Word(word) = &head.value
                        && word.lexeme.as_str() == "core-verb:BeNegative"
                    {
                        control_participles.insert(complement.clone());
                    }
                    if let Reading::SelectedPredicate {
                        head, complements, ..
                    } = node
                        && let LexicalReading::Word(word) = &head.value
                        && word.lexeme.as_str() == "core-verb:BeNegative"
                    {
                        control_complements.insert(complements.clone());
                    }
                })
                .unwrap();
        }
        assert!(
            !control_complements.is_empty() || !control_participles.is_empty(),
            "{card}"
        );
        for reading in analytic {
            let mut negated = false;
            reading
                .visit(&mut |node| {
                    if let Reading::NegatedSelectedPredicate { complements, .. } = node {
                        assert!(
                            control_complements.contains(complements),
                            "{card}: ordinary Complement"
                        );
                    }
                    if let Reading::NegatedParticipialPredicate { complement, .. } = node {
                        assert!(
                            control_participles.contains(complement),
                            "{card}: ordinary participial Complement"
                        );
                    }
                    if let Reading::NegatedAuxiliaryHead { head, negator, .. } = node {
                        let LexicalReading::Word(head) = &head.value else {
                            panic!("auxiliary Word Form")
                        };
                        assert_eq!(head.lexeme.as_str(), "core-verb:BeContracted");
                        let Reading::Negator {
                            category,
                            head: marker,
                            ..
                        } = negator.as_ref()
                        else {
                            panic!("Negator")
                        };
                        assert_eq!(*category, Category::PrimaryNegator);
                        let LexicalReading::Word(marker) = &marker.value else {
                            panic!("negator Word Form")
                        };
                        assert_eq!(marker.lexeme.as_str(), "vocab:PredicateNegator/Not");
                        negated = true;
                    }
                })
                .unwrap();
            assert!(negated, "{card}");
        }
    }
}

#[test]
fn analytic_and_synthetic_pairs_keep_equal_readings() {
    for text in [
        "It's not your turn.",
        "It's not a creature.",
        "It's not legendary.",
        "They're not legendary.",
        "You're not the monarch.",
    ] {
        let analytic = common::readings(text, Category::Sentence);
        let control = common::readings(&synthetic(text), Category::Sentence);
        assert!(!analytic.is_empty(), "{text}");
        assert_eq!(analytic.len(), control.len(), "{text}");
    }
    for text in [
        "It isn't your turn.",
        "It's your turn.",
        "It isn't a creature.",
        "They aren't legendary.",
        "You aren't the monarch.",
        "It doesn't untap.",
    ] {
        let controls = common::readings(text, Category::Sentence);
        assert!(!controls.is_empty(), "{text}");
        for reading in controls {
            reading
                .visit(&mut |node| {
                    if let Reading::NegatedSelectedPredicate { .. } = node {
                        panic!("analytic route in control {text}")
                    }
                })
                .unwrap();
        }
    }
}

#[test]
fn licences_and_positive_auxiliary_status_control_admission() {
    for (owner, replacement) in [
        ("vocab:PredicateNegator/Not", None),
        ("core-verb:BeContracted", Some(("AuxiliaryUse", "No"))),
        ("core-verb:BeContracted", Some(("Polarity", "Negative"))),
    ] {
        let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id.as_str() == owner)
            .unwrap();
        if let Some((feature, value)) = replacement {
            lexeme
                .properties
                .features
                .insert(feature.into(), value.into());
        } else {
            // Identical spelling and properties do not confer the named word's identity.
            lexeme.id = "test:unnamed-not".into();
        }
        let lexicon = Lexicon::new(lexemes).unwrap();
        assert!(
            common::readings_with_lexicon(&lexicon, "It's not your turn.", Category::Sentence)
                .is_empty()
        );
        assert!(
            !common::readings_with_lexicon(&lexicon, "It's your turn.", Category::Sentence)
                .is_empty()
        );
    }
    for text in ["It isn't not your turn.", "It's not not your turn."] {
        assert!(
            common::readings(text, Category::Sentence).is_empty(),
            "{text}"
        );
    }
    assert!(!common::readings("It is not your turn.", Category::Sentence).is_empty());
}

#[test]
fn independent_negated_predicate_exports_synthetic_polarity_and_roundtrips() {
    let positive = common::readings("'s your turn", Category::FinitePredicate);
    let Reading::SelectedPredicate {
        head, complements, ..
    } = positive.into_iter().next().unwrap()
    else {
        panic!("selected predicate")
    };
    let marker = common::readings("not", Category::PrimaryNegator)
        .into_iter()
        .next()
        .unwrap();
    let value = Reading::NegatedSelectedPredicate {
        form: 0,
        head: Box::new(Reading::NegatedAuxiliaryHead {
            form: 0,
            head: head.clone(),
            negator: Box::new(marker),
        }),
        complements: complements.clone(),
    };
    let text = "'s not your turn";
    assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, Category::FinitePredicate).contains(&value));
    // An otherwise identical synthetic head provides the Negative export.
    // Compare complete admission summaries, including agreement and polarity.
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id.as_str() == "core-verb:BeContracted")
        .unwrap()
        .properties
        .features
        .insert("Polarity".into(), "Negative".into());
    let synthetic_lexicon = Lexicon::new(lexemes).unwrap();
    let synthetic = Reading::SelectedPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head,
        complements,
    };
    assert_eq!(
        value.admit(common::lexicon()).unwrap(),
        synthetic.admit(&synthetic_lexicon).unwrap()
    );
}
