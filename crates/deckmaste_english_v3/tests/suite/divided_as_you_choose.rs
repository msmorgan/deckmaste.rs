use crate::common::{LEXICON, readings, readings_on_card};
use deckmaste_english_v3::grammar::{Category, Reading};

fn owner(word: &deckmaste_english_v3::grammar::Word) -> &str {
    match &word.value {
        deckmaste_lexical::LexicalReading::Word(value) => value.lexeme.as_str(),
        _ => panic!("expected declared word"),
    }
}

const WITNESSES: [(&str, &str); 5] = [
    (
        "Forked Bolt",
        "Forked Bolt deals 2 damage divided as you choose among one or two targets.",
    ),
    (
        "Magma Opus",
        "Magma Opus deals 4 damage divided as you choose among any number of targets.",
    ),
    (
        "Samut, the Tested",
        "−2: Samut deals 2 damage divided as you choose among one or two targets.",
    ),
    (
        "Death to Our Enemies",
        "When you do, it deals 7 damage divided as you choose among one or two targets.",
    ),
    (
        "Boulderfall",
        "Boulderfall deals 5 damage divided as you choose among any number of targets.",
    ),
];

fn witness(index: usize) {
    let (card, text) = WITNESSES[index];
    let values = readings_on_card(card, text, Category::Document);
    assert_eq!(values.len(), 1, "{card}");
    for value in values {
        let mut postmodifiers = 0;
        value
            .visit(&mut |node| {
                if let Reading::ParticipialPostmodifiedNominal { modifier, .. } = node {
                    let Reading::SelectedGapPrepositionPredicate {
                        head,
                        modifier: Some(manner),
                        marker,
                        ..
                    } = modifier.as_ref()
                    else {
                        panic!("{card}: wrong participial analysis")
                    };
                    assert_eq!(owner(head), "core-verb:Divide");
                    assert_eq!(owner(marker), "vocab:Preposition/Among");
                    let Reading::InterveningPrepositionAdjunct { head: phrase, .. } =
                        manner.as_ref()
                    else {
                        panic!("{card}: missing manner")
                    };
                    let Reading::ClauseComplementPreposition {
                        head, complement, ..
                    } = phrase.as_ref()
                    else {
                        panic!("{card}: missing as phrase")
                    };
                    assert_eq!(owner(head), "vocab:Preposition/As");
                    assert!(
                        matches!(complement.as_ref(), Reading::FiniteGappedClause { predicate, .. }
                        if matches!(predicate.as_ref(), Reading::FiniteObjectGap { .. }))
                    );
                    postmodifiers += 1;
                }
            })
            .unwrap();
        assert_eq!(postmodifiers, 1, "{card}");
    }
}

#[test]
fn forked_bolt() {
    witness(0);
}
#[test]
fn magma_opus() {
    witness(1);
}
#[test]
fn samut_the_tested() {
    witness(2);
}
#[test]
fn death_to_our_enemies() {
    witness(3);
}
#[test]
fn boulderfall() {
    witness(4);
}

#[test]
fn among_is_not_a_dependent_of_choose_and_manner_cannot_precede_the_object() {
    let temporal = readings("as this land enters", Category::PrepositionPhrase);
    assert_eq!(temporal.len(), 1);
    for value in temporal {
        let Reading::ClauseComplementPreposition { complement, .. } = value else {
            panic!("expected clause complement")
        };
        assert!(matches!(complement.as_ref(), Reading::FiniteClause { .. }));
    }
    assert!(
        readings(
            "as you choose among one or two targets",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
    assert!(
        readings_on_card(
            "Forked Bolt",
            "Forked Bolt deals as you choose 2 damage to any target.",
            Category::Document
        )
        .is_empty()
    );
}

#[test]
fn independently_assembled_participle_preserves_both_roundtrip_laws() {
    // Independently assemble the Forked Bolt constituent from its lexical
    // head, manner PP and target NP; do not copy its parsed passive tree.
    use deckmaste_english_v3::grammar::Word;
    use deckmaste_lexical::{
        FeatureBundle, Finiteness, LexicalReading, LexicalValue, Number, Numeral, SurfaceCase,
        WordForm,
    };
    let word = |owner: &str, form, frame| Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: if form == WordForm::PastParticiple {
                FeatureBundle {
                    finiteness: Some(Finiteness::Nonfinite),
                    ..Default::default()
                }
            } else {
                FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    };
    let phrase = readings("as you choose", Category::PrepositionPhrase)
        .into_iter()
        .next()
        .unwrap();
    let target = readings("one or two targets", Category::AccusativePhrase)
        .into_iter()
        .next()
        .unwrap();
    let value = Reading::SelectedGapPrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Divide", WordForm::PastParticiple, Some(0)),
        modifier: Some(Box::new(Reading::InterveningPrepositionAdjunct {
            form: 0,
            head: Box::new(phrase),
        })),
        marker: word("vocab:Preposition/Among", WordForm::Invariant, None),
        complement: Box::new(target),
    };
    value.admit(&LEXICON).unwrap();
    let text = "divided as you choose among one or two targets";
    assert_eq!(value.realize(&LEXICON).unwrap(), text);
    assert_eq!(
        readings(text, Category::SecondaryVerbPhrase),
        [value.clone()].into_iter().collect()
    );
    let measured = Reading::MeasuredNounPhrase {
        form: 0,
        quantity: Box::new(Reading::UngroupedScalarNumeral {
            form: 0,
            modifier: None,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 2,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        head: Box::new(Reading::ParticipialPostmodifiedNominal {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: Word {
                    value: LexicalReading::Word(LexicalValue {
                        lexeme: "lexeme:CommonNoun/Damage".into(),
                        form: WordForm::Singular,
                        features: FeatureBundle {
                            number: Some(Number::Singular),
                            ..Default::default()
                        },
                        variant: 0,
                        capitalization: SurfaceCase::Declared,
                    }),
                    frame: None,
                    countability: Some(false),
                },
            }),
            modifier: Box::new(value),
        }),
    };
    measured.admit(&LEXICON).unwrap();
    let text = "2 damage divided as you choose among one or two targets";
    assert_eq!(measured.realize(&LEXICON).unwrap(), text);
    assert_eq!(
        readings(text, Category::NounPhrase),
        [measured].into_iter().collect()
    );
}
