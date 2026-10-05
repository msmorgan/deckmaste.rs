mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn numeral(value: i32, notation: Numeral) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        frame: None,
        countability: None,
    }
}
fn scry(capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "lexeme:keyword_action/scry".into(),
            form: WordForm::Plain,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: Some(1),
        countability: None,
    }
}

#[test]
fn independently_constructed_amount_and_imperative_roundtrip() {
    let digit = numeral(3, Numeral::Arabic(false));
    let scalar = Reading::SmallUnsignedScalar {
        form: 0,
        head: digit.clone(),
    };
    let amount = Reading::ScalarAmount {
        form: 0,
        value: Box::new(scalar.clone()),
    };
    assert_eq!(
        readings("3", Category::Amount),
        BTreeSet::from([amount.clone()])
    );
    let head = scry(SurfaceCase::Initial);
    let predicate = Reading::AmountPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: head.clone(),
        amount: Box::new(amount.clone()),
    };
    let bare = Reading::BarePredicate {
        form: 0,
        head: Box::new(predicate.clone()),
    };
    let imperative = Reading::Imperative {
        form: 0,
        predicate: Box::new(bare.clone()),
    };
    let sentence = Reading::Sentence {
        form: 0,
        clause: Box::new(imperative.clone()),
    };
    sentence.admit(lexicon()).unwrap();
    assert_eq!(sentence.realize(lexicon()).unwrap(), "Scry 3.");
    assert_eq!(
        readings("Scry 3.", Category::Sentence),
        BTreeSet::from([sentence.clone()])
    );
    let mut nodes = Vec::new();
    sentence
        .visit(&mut |node| nodes.push(node.clone()))
        .unwrap();
    assert_eq!(
        nodes,
        [
            sentence.clone(),
            imperative,
            bare,
            predicate.clone(),
            amount.clone(),
            scalar
        ]
    );
    let mut words = Vec::new();
    sentence
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    assert_eq!(words, [head.clone(), digit]);
    let mut finite = scry(SurfaceCase::Declared);
    let LexicalReading::Word(verb) = &mut finite.value else { unreachable!() };
    verb.form = WordForm::Present;
    verb.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Third),
        tense: Some(Tense::Present),
        finiteness: Some(Finiteness::Finite),
        ..FeatureBundle::default()
    };
    let finite = Reading::AmountPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: finite,
        amount: Box::new(amount.clone()),
    };
    finite.admit(lexicon()).unwrap();
    assert_eq!(finite.realize(lexicon()).unwrap(), "scries 3");
    assert_eq!(
        readings("scries 3", Category::FinitePredicate),
        BTreeSet::from([finite])
    );
    let mut wrong_frame = head;
    wrong_frame.frame = Some(0);
    assert!(
        Reading::AmountPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: wrong_frame,
            amount: Box::new(amount)
        }
        .admit(lexicon())
        .is_err()
    );
    let negative = Reading::ScalarAmount {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: numeral(-3, Numeral::Arabic(false)),
        }),
    };
    assert!(negative.admit(lexicon()).is_err());
    let cardinal = Reading::ScalarAmount {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: numeral(3, Numeral::Cardinal),
        }),
    };
    assert!(cardinal.admit(lexicon()).is_err());
}

#[test]
fn oracle_numeric_complements_and_agreement() {
    // Reason, Take a Glance, and Sigiled Starfish.
    for text in [
        "Scry 3.",
        "Scry 2.",
        "{T}: Scry 1.",
        "You scry 3.",
        "This creature scries 3.",
        "Connive 2.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
    for text in [
        "You scries 3.",
        "This creature scry 3.",
        "Scry -1.",
        "Scry +1.",
        "Scry 1000.",
        "Scry 3rd.",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid Reading for {text:?}"
        );
    }
    for text in ["0", "1,000", "X"] {
        assert!(
            !readings(text, Category::Amount).is_empty(),
            "no Amount for {text:?}"
        );
    }
    let cardinal = Reading::CardinalAmount {
        form: 0,
        head: numeral(3, Numeral::Cardinal),
    };
    cardinal.admit(lexicon()).unwrap();
    assert_eq!(cardinal.realize(lexicon()).unwrap(), "three");
    assert_eq!(
        readings("three", Category::Amount),
        BTreeSet::from([cardinal])
    );
}
