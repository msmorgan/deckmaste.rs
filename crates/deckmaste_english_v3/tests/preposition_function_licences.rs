mod common;

use common::{lexicon, readings};
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, Finiteness, LexicalReading, LexicalValue, Number, Person, SurfaceCase,
    Tense, WordForm,
};
use std::collections::BTreeSet;

fn invariant(id: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}
fn noun(id: &str) -> Reading {
    let mut head = invariant(id);
    head.countability = Some(true);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = WordForm::Singular;
    value.features.number = Some(Number::Singular);
    Reading::Noun { form: 0, head }
}
fn determined(marker: &str, head: Reading) -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant(marker),
        head: Box::new(head),
    }
}
fn acc(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}
fn graveyard() -> Reading {
    let mut possessor = invariant("vocab:PossessiveDeterminerPronoun/Your");
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
        head: Box::new(noun("lexeme:CommonNoun/Graveyard")),
    }
}
fn pp(id: &str, head: Reading) -> Reading {
    Reading::PrepositionPhrase {
        form: 0,
        head: invariant(id),
        complement: Box::new(acc(head)),
    }
}
fn exact(text: &str, category: Category, expected: &BTreeSet<Reading>) {
    for value in expected {
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
    }
    assert_eq!(&readings(text, category), expected);
}
#[test]
fn animal_magnetism_has_only_the_two_selected_segment_readings() {
    // Animal Magnetism's final imperative, without its sentence punctuation.
    let object = acc(determined(
        "vocab:SingularDemonstrative/That",
        noun("lexeme:CommonNoun/Card"),
    ));
    let battlefield = acc(determined(
        "vocab:DefiniteMarker/The",
        noun("lexeme:CommonNoun/Battlefield"),
    ));
    let rest = acc(determined(
        "vocab:DefiniteMarker/The",
        noun("lexeme:CommonNoun/Rest"),
    ));
    let tail = Reading::Coordination {
        category: Category::SelectedComplementTail,
        form: 0,
        left: Box::new(Reading::ObjectPrepositionTail {
            form: 0,
            object: Box::new(object),
            marker: invariant("vocab:Preposition/Onto"),
            complement: Box::new(battlefield),
        }),
        coordinator: invariant("vocab:Coordinator/And"),
        right: Box::new(Reading::ObjectPrepositionTail {
            form: 0,
            object: Box::new(rest),
            marker: invariant("vocab:Preposition/Into"),
            complement: Box::new(acc(graveyard())),
        }),
    };
    let expected = [WordForm::Plain, WordForm::PastParticiple].map(|form| {
        let mut head = invariant("core-verb:Put");
        head.frame = Some(5);
        let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
        value.form = form;
        if form == WordForm::PastParticiple {
            value.features.finiteness = Some(Finiteness::Nonfinite);
        }
        Reading::SelectedComplementClustersPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head,
            tail: Box::new(tail.clone()),
        }
    });
    exact(
        "put that card onto the battlefield and the rest into your graveyard",
        Category::SecondaryVerbPhrase,
        &BTreeSet::from(expected),
    );
}
#[test]
fn library_of_leng_fixed_preposition_keeps_its_pp_complement() {
    let expected = Reading::CompoundPrepositionPhrase {
        form: 0,
        head: invariant("vocab:Preposition/Instead"),
        marker: invariant("vocab:Preposition/Of"),
        complement: Box::new(pp("vocab:Preposition/Into", graveyard())),
    };
    exact(
        "instead of into your graveyard",
        Category::PrepositionPhrase,
        &BTreeSet::from([expected]),
    );
}
#[test]
fn hurricane_keyword_pp_remains_a_nominal_modifier() {
    let expected = Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(noun("lexeme:type/creature")),
        modifier: Box::new(Reading::KeywordComplementPreposition {
            form: 0,
            head: invariant("vocab:Preposition/With"),
            complement: Box::new(Reading::BareKeyword {
                form: 0,
                head: invariant("lexeme:keyword_ability/flying"),
            }),
        }),
    };
    exact(
        "creature with flying",
        Category::Nominal,
        &BTreeSet::from([expected]),
    );
}
#[test]
fn raise_dead_keeps_its_selected_source_while_nominal_exclusion_is_deferred() {
    let text = "Return target creature card from your graveyard to your hand.";
    let values = readings(text, Category::Document);
    assert!(!values.is_empty());
    let mut sources = Vec::new();
    for value in values {
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme == "core-verb:Return"
                    && head.frame == Some(0)
                    && let FrameValue::Optional(Some(_)) = &complements[1]
                {
                    sources.push(complements[1].clone());
                }
            })
            .unwrap();
    }
    assert!(
        !sources.is_empty(),
        "Return's declared optional source must survive"
    );
    for source in sources {
        assert_eq!(
            source,
            FrameValue::Optional(Some(Box::new(FrameValue::Marked {
                marker: invariant("vocab:Preposition/From"),
                argument: Box::new(acc(graveyard())),
            })))
        );
    }
}
fn chosen_type() -> Reading {
    let mut head = invariant("core-verb:Choose");
    head.frame = Some(0);
    let LexicalReading::Word(word) = &mut head.value else { unreachable!() };
    word.form = WordForm::PastParticiple;
    word.features.finiteness = Some(Finiteness::Nonfinite);
    determined(
        "vocab:DefiniteMarker/The",
        Reading::ParticipialPremodifier {
            form: 0,
            modifier: Box::new(Reading::VerbalPremodifier { form: 0, head }),
            head: Box::new(noun("lexeme:CommonNoun/Type")),
        },
    )
}

#[test]
fn of_shapes_keep_their_nominal_and_predicative_functions() {
    let phrase = pp("vocab:Preposition/Of", chosen_type());
    exact(
        "of the chosen type",
        Category::PrepositionPhrase,
        &BTreeSet::from([phrase.clone()]),
    );
    let expected = [
        (Number::Singular, Person::Second),
        (Number::Plural, Person::First),
        (Number::Plural, Person::Second),
        (Number::Plural, Person::Third),
    ]
    .map(|(number, person)| {
        let mut head = invariant("core-verb:BeNegative");
        head.frame = Some(0);
        let LexicalReading::Word(word) = &mut head.value else { unreachable!() };
        word.form = WordForm::Present;
        word.features = FeatureBundle {
            number: Some(number),
            person: Some(person),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..Default::default()
        };
        Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head,
            complements: vec![FrameValue::Argument(Box::new(
                Reading::PredicativePreposition {
                    form: 0,
                    phrase: Box::new(phrase.clone()),
                },
            ))],
        }
    });
    exact(
        "aren't of the chosen type",
        Category::FinitePredicate,
        &BTreeSet::from(expected),
    );
    for (text, category) in [
        ("aren't of the chosen type", Category::FinitePredicate),
        ("Spells you cast of the chosen type", Category::NounPhrase),
        ("one of them of their choice", Category::NounPhrase),
    ] {
        assert!(
            !readings(text, category).is_empty(),
            "no Reading for {text:?}"
        );
    }
}
