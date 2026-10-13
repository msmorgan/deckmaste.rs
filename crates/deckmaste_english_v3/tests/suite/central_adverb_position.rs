use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

use crate::common;

fn witness(card: &str, text: &str, owner: &str, position: usize) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut adjuncts = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::CentralAdverbHead { form, modifier, .. } = node {
                    adjuncts.push((*form, modifier.value.clone()));
                }
            })
            .unwrap();
        assert_eq!(adjuncts.len(), 1, "{card}");
        assert_eq!(adjuncts[0].0, position, "{card}");
        let LexicalReading::Word(word) = &adjuncts[0].1 else {
            panic!("the Adjunct must retain its Adverb Lexeme");
        };
        assert_eq!(word.lexeme.as_str(), owner, "{card}");
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn vastwood_animist() {
    witness(
        "Vastwood Animist",
        "It's still a land.",
        "vocab:Adverb/Still",
        1,
    );
}

#[test]
fn thelonite_druid() {
    witness(
        "Thelonite Druid",
        "They're still lands.",
        "vocab:Adverb/Still",
        1,
    );
}

#[test]
fn verdant_touch() {
    witness(
        "Verdant Touch",
        "Target land becomes a 2/2 creature that's still a land.",
        "vocab:Adverb/Still",
        1,
    );
}

#[test]
fn aven_warcraft() {
    witness(
        "Aven Warcraft",
        "Creatures you control also gain protection from the chosen color until end of turn.",
        "vocab:Adverb/Also",
        0,
    );
}

#[test]
fn might_of_the_meek() {
    witness(
        "Might of the Meek",
        "It also gets +1/+0 until end of turn if you control a Mouse.",
        "vocab:Adverb/Also",
        0,
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
fn independently_assembled_central_adjunct_roundtrips() {
    // Vastwood Animist's predicative Complement, assembled with the strong
    // copula.
    let complement = common::readings("a land", Category::PredicativeComplement)
        .into_iter()
        .next()
        .unwrap();
    let value = Reading::CentralAdverbPredicate {
        form: 0,
        head: Box::new(Reading::CentralAdverbHead {
            form: 1,
            category: Category::CentralAdverbSelectedHead,
            head: word(
                "core-verb:Be",
                WordForm::Present,
                FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    ..Default::default()
                },
                Some(0),
            ),
            modifier: word(
                "vocab:Adverb/Still",
                WordForm::Invariant,
                FeatureBundle::default(),
                None,
            ),
        }),
        complements: vec![FrameValue::Argument(Box::new(complement))],
    };
    value.admit(common::lexicon()).unwrap();
    let text = value.realize(common::lexicon()).unwrap();
    assert_eq!(text, "is still a land");
    assert!(common::readings(&text, Category::FinitePredicate).contains(&value));
}

#[test]
fn central_licence_does_not_license_other_positions_or_adverbs() {
    for text in [
        "Still it's a land.",
        "It's a land still.",
        "It's again a land.",
        "It's also land.",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
    let mut lexemes = common::lexicon()
        .lexemes()
        .values()
        .cloned()
        .collect::<Vec<_>>();
    let still = lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id.as_str() == "vocab:Adverb/Still")
        .unwrap();
    still
        .properties
        .features
        .insert("CentralAdjunctLicence".into(), "None".into());
    let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert!(
        common::readings_with_lexicon(&lexicon, "It's still a land.", Category::Document)
            .is_empty()
    );
}

#[test]
fn parnesse_auxiliary_keeps_the_adjunct_outside_the_object() {
    let text = "up to one target opponent may also copy that spell";
    let values =
        common::readings_on_card("Parnesse, the Subtle Brush", text, Category::FiniteClause);
    assert!(!values.is_empty());
    for value in values {
        let mut adjuncts = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::CentralAdverbComplement {
                    category,
                    modifier,
                    predicate,
                    ..
                } = node
                {
                    adjuncts.push((
                        *category,
                        common::lexicon().realize(&modifier.value).unwrap(),
                        predicate.category(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(
            adjuncts,
            vec![(
                Category::BareComplement,
                "also".into(),
                Category::BarePredicate
            )]
        );
    }
}

#[test]
fn dark_dabbling_imperative_keeps_the_central_adjunct_outside_its_object() {
    // The bare imperative inside Dark Dabbling's conditional sentence.
    let text = "also regenerate each other creature you control";
    let values = common::readings_on_card("Dark Dabbling", text, Category::Clause);
    assert!(!values.is_empty());
    for value in values {
        let Reading::CentralAdverbImperative {
            modifier,
            predicate,
            ..
        } = value
        else {
            panic!("expected the licensed central Adjunct on an imperative");
        };
        assert_eq!(common::lexicon().realize(&modifier.value).unwrap(), "also");
        assert_eq!(predicate.category(), Category::BarePredicate);
        assert_eq!(
            predicate.realize(common::lexicon()).unwrap(),
            "regenerate each other creature you control"
        );
    }
}
