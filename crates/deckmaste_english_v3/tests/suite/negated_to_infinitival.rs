use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::WordForm;

use crate::common;

fn owner(word: &Word) -> &str {
    let LexicalReading::Word(value) = &word.value else {
        panic!("expected a lexical Word Form");
    };
    value.lexeme.as_str()
}

fn attachments(reading: &Reading) -> BTreeSet<String> {
    let mut hosts = BTreeSet::new();
    reading
        .visit(&mut |node| {
            if let Reading::PrepositionPredicate { head, modifier, .. } = node
                && modifier.realize(common::lexicon()).unwrap() == "during your untap step"
            {
                hosts.insert(head.realize(common::lexicon()).unwrap().replace("not ", ""));
            }
        })
        .unwrap();
    hosts
}

#[test]
fn witnesses_keep_secondary_negation_in_the_selected_infinitival() {
    for (card, object) in [
        ("Rust Tick", "this creature"),
        ("Amber Prison", "this artifact"),
        ("Ice Floe", "this land"),
        ("Rubinia Soulsinger", "Rubinia Soulsinger"),
    ] {
        let text = format!("You may choose not to untap {object} during your untap step.");
        let positive = text.replace("not ", "");
        let readings = common::readings_on_card(card, &text, Category::Sentence);
        assert!(!readings.is_empty(), "{card}");
        let mut negative_attachments = BTreeSet::new();
        for reading in readings {
            let mut selected = false;
            reading
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        head, complements, ..
                    } = node
                        && owner(head) == "core-verb:Choose"
                    {
                        for complement in complements {
                            if let FrameValue::Argument(value) = complement
                                && let Reading::ToInfinitive {
                                    category,
                                    negator: Some(negator),
                                    marker,
                                    predicate,
                                    ..
                                } = value.as_ref()
                            {
                                assert_eq!(*category, Category::InfinitiveComplement);
                                let Reading::SecondaryNegator { head, .. } = negator.as_ref()
                                else {
                                    panic!("the infinitival must retain its Secondary Negator");
                                };
                                assert_eq!(owner(head), "vocab:PredicateNegator/Not");
                                assert_eq!(owner(marker), "vocab:InfinitivalMarker/To");
                                assert_eq!(predicate.category(), Category::BarePredicate);
                                selected = true;
                            }
                        }
                    }
                    if let Reading::CentralAdverbHead { modifier, .. }
                    | Reading::Adverb { head: modifier, .. } = node
                    {
                        assert_ne!(owner(modifier), "vocab:PredicateNegator/Not");
                    }
                })
                .unwrap();
            assert!(
                selected,
                "{card}: negation must be inside choose's Complement"
            );
            negative_attachments.insert(attachments(&reading));
        }
        let positive_attachments = common::readings_on_card(card, &positive, Category::Sentence)
            .iter()
            .map(attachments)
            .collect();
        assert_eq!(negative_attachments, positive_attachments, "{card}");
    }
}

#[test]
fn controls_preserve_their_roles_and_roundtrip() {
    for text in [
        "You may untap this creature during your untap step.",
        "This creature doesn't untap during your untap step.",
        "You may choose new targets for the copy.",
    ] {
        let readings = common::readings(text, Category::Sentence);
        assert!(!readings.is_empty(), "{text}");
        for reading in readings {
            reading
                .visit(&mut |node| {
                    if let Reading::ToInfinitive { negator, .. } = node {
                        assert_eq!(*negator, None);
                    }
                })
                .unwrap();
        }
    }
}

#[test]
fn secondary_licence_does_not_supply_matrix_negation_or_the_to_not_alternant() {
    for text in [
        "not choose to untap this creature",
        "choose to not untap this creature",
        "choose not not to untap this creature",
        "choose not untap this creature",
    ] {
        assert!(
            common::readings(text, Category::BarePredicate).is_empty(),
            "{text}"
        );
    }
    for category in [Category::InfinitiveComplement, Category::PurposeInfinitival] {
        assert!(common::readings("to not untap this creature", category).is_empty());
    }
}

#[test]
fn negation_requires_the_declared_feature() {
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id.as_str() == "vocab:PredicateNegator/Not")
        .unwrap()
        .properties
        .features
        .remove("SecondaryNegation");
    let lexicon = Lexicon::new(lexemes).unwrap();
    assert!(
        common::readings_with_lexicon(
            &lexicon,
            "choose not to untap this creature",
            Category::BarePredicate,
        )
        .is_empty()
    );
    assert!(
        !common::readings_with_lexicon(
            &lexicon,
            "choose to untap this creature",
            Category::BarePredicate,
        )
        .is_empty()
    );
}

fn word(owner: &str, form: WordForm) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| value.lexeme.as_str() == owner && value.form == form)
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_constructed_negated_complement_preserves_both_laws() {
    // The attested predicate inside Rust Tick's modal Complement.
    let mut creature = word("lexeme:type/creature", WordForm::Singular);
    creature.countability = Some(true);
    let mut untap = word("lexeme:keyword_action/untap", WordForm::Plain);
    untap.frame = Some(1);
    let infinitival = Reading::ToInfinitive {
        category: Category::InfinitiveComplement,
        form: 0,
        negator: Some(Box::new(Reading::SecondaryNegator {
            form: 0,
            head: word("vocab:PredicateNegator/Not", WordForm::Invariant),
        })),
        marker: word("vocab:InfinitivalMarker/To", WordForm::Invariant),
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: untap,
                complements: vec![FrameValue::Argument(Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(Reading::DeterminedNounPhrase {
                        form: 0,
                        determiner: word("vocab:SingularDemonstrative/This", WordForm::Invariant),
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: creature,
                        }),
                    }),
                }))],
            }),
        }),
    };
    let mut choose = word("core-verb:Choose", WordForm::Plain);
    choose.frame = Some(1);
    let predicate = Reading::BarePredicate {
        form: 0,
        head: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: choose,
            complements: vec![FrameValue::Argument(Box::new(infinitival.clone()))],
        }),
    };
    for (value, text) in [
        (infinitival, "not to untap this creature"),
        (predicate, "choose not to untap this creature"),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let actual = common::readings(text, value.category());
        assert!(actual.contains(&value));
        let mut expected_words = Vec::new();
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        let mut actual_words = Vec::new();
        actual
            .get(&value)
            .unwrap()
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);
    }
}
