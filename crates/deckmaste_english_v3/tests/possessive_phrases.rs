mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn word(id: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle {
                number,
                ..FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability: None,
        frame: None,
    }
}
fn assert_laws(value: Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert!(
        parsed.contains(&value),
        "independent value missing for {text:?}"
    );
    let reparsed = parsed.get(&value).unwrap();
    let mut nodes = Vec::new();
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    let mut parsed_nodes = Vec::new();
    reparsed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(nodes, parsed_nodes);
    let mut words = Vec::new();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    let mut parsed_words = Vec::new();
    reparsed
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(words, parsed_words);
}

fn possessed(noun: &str, possessor_number: Number) -> Reading {
    let mut head = word(
        &format!("lexeme:CommonNoun/{noun}"),
        WordForm::Singular,
        Some(Number::Singular),
    );
    head.countability = Some(true);
    let mut possessor = word(
        "vocab:PossessiveDeterminerPronoun/Your",
        WordForm::Invariant,
        Some(possessor_number),
    );
    let LexicalReading::Word(value) = &mut possessor.value else { unreachable!() };
    value.features.person = Some(deckmaste_lexical::Person::Second);
    value.features.case = Some(Case::Genitive);
    Reading::PossessiveNounPhrase {
        form: 0,
        possessor,
        head: Box::new(Reading::Noun { form: 0, head }),
    }
}

#[test]
fn possessed_bases_preserve_singular_second_person_before_coordination() {
    // Apocalypse: "You discard your hand."
    // Skyfisher Spider: "... each creature card in your graveyard."
    for (noun, text) in [("Hand", "your hand"), ("Graveyard", "your graveyard")] {
        let singular = possessed(noun, Number::Singular);
        let plural = possessed(noun, Number::Plural);
        assert_laws(singular.clone(), text, Category::NounPhrase);
        assert!(plural.admit(lexicon()).is_err());
        assert!(plural.realize(lexicon()).is_err());
        assert_eq!(
            readings(text, Category::NounPhrase),
            BTreeSet::from([singular])
        );
    }
    // Mind Extraction: "Target player reveals their hand ..."
    // Next of Kin: "... from your hand or from the command zone ..."
    for text in ["their hand", "the command zone"] {
        assert!(
            !readings(text, Category::NounPhrase).is_empty(),
            "no base NP for {text:?}"
        );
    }
    for text in ["yours hand", "they hand", "your", "your the hand"] {
        assert!(
            readings(text, Category::NounPhrase).is_empty(),
            "invalid possessed NP for {text:?}"
        );
    }
}

#[test]
fn authentic_possessed_preposition_bases_then_their_coordination() {
    // Next of Kin; verify each base before the full coordinated constituent.
    for text in ["from your hand", "from the command zone"] {
        assert!(
            !readings(text, Category::PrepositionPhrase).is_empty(),
            "no PP base for {text:?}"
        );
    }
    assert!(
        !readings(
            "from your hand or from the command zone",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
    for text in [
        // Phyrexian Dragon Engine: "... you may discard your hand."
        "you may discard your hand",
        // Apocalypse: "Exile all permanents. You discard your hand."
        "You discard your hand.",
    ] {
        assert!(
            !readings(
                text,
                if text.ends_with('.') { Category::Document } else { Category::Clause }
            )
            .is_empty(),
            "no possessed NP composition for {text:?}"
        );
    }
    assert!(readings("You discards your hand.", Category::Document).is_empty());
}
