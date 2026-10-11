use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{LexicalReading, Number, Numeral, Person, SurfaceCase, WordForm};

use crate::common;

fn word(owner: &str, form: WordForm) -> Word {
    let value = common::lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == owner
                && value.form == form
                && (form != WordForm::Irrealis
                    || (value.features.number == Some(Number::Singular)
                        && value.features.person == Some(Person::Third)))
        })
        .unwrap()
        .clone();
    let lexeme = &common::lexicon().lexemes()[&value.lexeme];
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn scalar() -> Reading {
    Reading::UngroupedScalarNumeral {
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
    }
}

fn measured() -> Reading {
    let mut head = word("vocab:ScalarDegree/Greater", WordForm::Invariant);
    head.frame = common::lexicon().lexemes()["vocab:ScalarDegree/Greater"]
        .properties
        .frames
        .iter()
        .position(|frame| frame.kind == "Predicate" && frame.items.is_empty());
    Reading::MeasuredScalarLocation {
        category: Category::PredicativeComplement,
        form: 0,
        modifier: Box::new(scalar()),
        head,
    }
}

fn irrealis_scalar() -> Reading {
    let mut head = word("core-verb:Be", WordForm::Irrealis);
    head.frame = common::lexicon().lexemes()["core-verb:Be"]
        .properties
        .frames
        .iter()
        .position(|frame| frame.kind == "ScalarLocation");
    Reading::IrrealisClause {
        form: 0,
        subject: Box::new(Reading::CasePhrase {
            category: Category::NominativePhrase,
            form: 0,
            head: Box::new(Reading::PossessiveNounPhrase {
                form: 0,
                possessor: word("vocab:PossessiveDeterminerPronoun/Its", WordForm::Invariant),
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word("lexeme:CommonNoun/Power", WordForm::Singular),
                }),
            }),
        }),
        predicate: Box::new(Reading::ScalarLocationPredicate {
            category: Category::IrrealisPredicate,
            form: 0,
            head,
            complements: vec![FrameValue::Argument(Box::new(
                Reading::ScalarMeasureValue {
                    category: Category::ScalarValue,
                    form: 0,
                    value: Box::new(scalar()),
                },
            ))],
        }),
    }
}

#[test]
fn pilot_witnesses_use_the_measure_as_comparative_modifier() {
    // Hotshot Mechanic; Deathless Pilot, Dynamite Diver, Cloudspire Captain;
    // Prodigy's Prototype's quoted token ability.
    for text in [
        "This creature crews Vehicles as though its power were 2 greater.",
        "This creature saddles Mounts and crews Vehicles as though its power were 2 greater.",
        "Whenever one or more Vehicles you control attack, create a 1/1 colorless Pilot creature token with \"This token crews Vehicles as though its power were 2 greater.\"",
    ] {
        common::assert_constituents(
            text,
            Category::Sentence,
            &[
                (Category::IrrealisClause, "its power were 2 greater"),
                (Category::IrrealisPredicate, "were 2 greater"),
                (Category::PredicativeComplement, "2 greater"),
                (Category::MeasurePhrase, "2"),
            ],
        );
        for reading in common::readings(text, Category::Sentence) {
            let mut modifiers = Vec::new();
            reading
                .visit(&mut |node| {
                    if let Reading::MeasuredScalarLocation {
                        category: Category::PredicativeComplement,
                        ..
                    } = node
                    {
                        modifiers.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(modifiers, vec![measured()]);
        }
    }
}

#[test]
fn independent_values_preserve_both_roundtrip_laws() {
    // The Pilot line's attested constituents; the numeric clause localizes IS1.
    for (value, text, category) in [
        (
            measured(),
            "2 greater",
            Category::PredicativeComplement,
        ),
        (
            irrealis_scalar(),
            "its power were 2",
            Category::IrrealisClause,
        ),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, category),
            BTreeSet::from([value.clone()])
        );
        let mut words = Vec::new();
        value
            .visit_words(&mut |word| words.push(common::lexicon().realize(&word.value).unwrap()))
            .unwrap();
        assert_eq!(words.join(" "), text);
    }
}

#[test]
fn measured_comparative_requires_the_declared_licence_and_empty_frame() {
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "vocab:ScalarDegree/Greater")
        .unwrap()
        .properties
        .features
        .insert("MeasureModifierUse".into(), "No".into());
    let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert!(measured().admit(&lexicon).is_err());
    assert!(
        common::readings_with_lexicon(&lexicon, "2 greater", Category::PredicativeComplement)
            .is_empty()
    );
    let mut invalid = measured();
    let Reading::MeasuredScalarLocation { head, .. } = &mut invalid else {
        unreachable!()
    };
    head.frame = Some(0); // The declared frame selecting than, with no Complement.
    assert!(invalid.admit(common::lexicon()).is_err());
    assert!(invalid.realize(common::lexicon()).is_err());
}

#[test]
fn irrealis_scalar_retains_subject_and_form_restrictions() {
    for text in ["that creature were 2", "its power is 2", "they were 2"] {
        assert!(
            common::readings(text, Category::IrrealisClause).is_empty(),
            "{text}"
        );
    }
    let mut invalid = irrealis_scalar();
    let Reading::IrrealisClause { predicate, .. } = &mut invalid else {
        unreachable!()
    };
    let Reading::ScalarLocationPredicate { head, .. } = predicate.as_mut() else {
        unreachable!()
    };
    head.frame = Some(0);
    assert!(invalid.admit(common::lexicon()).is_err());
}

#[test]
fn explicit_comparison_and_bounded_postpositive_keep_their_roles() {
    common::assert_constituents(
        "Its power is greater than 2.",
        Category::Sentence,
        &[
            (Category::ComparativeAdjectivePhrase, "greater than 2"),
            (Category::ComparativeComplement, "2"),
        ],
    );
    common::assert_constituents(
        "Destroy target creature with power 4 or greater.",
        Category::Sentence,
        &[(Category::ScalarValue, "4 or greater")],
    );
    // IS3: a numeral cannot be followed by a stray adjective in the copula's
    // nominal value frame; the measured AdjP has its own adjectival route.
    assert!(common::readings("2 greater", Category::MeasurePhrase).is_empty());
    common::assert_constituents(
        "Its power is 2 greater.",
        Category::Sentence,
        &[(Category::PredicativeComplement, "2 greater")],
    );
    for reading in common::readings("Its power is 2 greater.", Category::Sentence) {
        let mut complements = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::MeasuredScalarLocation {
                    category: Category::PredicativeComplement,
                    ..
                } = node
                {
                    complements.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(complements, vec![measured()]);
    }
}
