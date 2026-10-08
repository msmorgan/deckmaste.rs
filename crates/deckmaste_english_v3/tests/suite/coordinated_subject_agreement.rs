use crate::common::{assert_constituent_occurrences, assert_constituents, lexicon, readings};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm,
};

#[test]
fn shared_determiner_nominal_subject_requires_plural_agreement() {
    // Hulking Metamorph's attested clause and the Lord of Extinction
    // agreement control, with floated each handled by its own ticket.
    for (text, count) in [
        (
            "Its power and toughness are equal to this creature's power and toughness.",
            1,
        ),
        (
            "Lord of Extinction's power and toughness are equal to the number of cards in all graveyards.",
            4,
        ),
    ] {
        assert_eq!(readings(text, Category::Sentence).len(), count, "{text}");
        assert_constituent_occurrences(
            text,
            Category::Sentence,
            &[(Category::Nominal, "power and toughness", 0)],
        );
        assert!(readings(&text.replacen(" are ", " is ", 1), Category::Sentence).is_empty());
    }
    assert_eq!(
        readings(
            "Its power is equal to this creature's power.",
            Category::Sentence
        )
        .len(),
        1
    );
}

#[test]
fn singular_determiner_selection_survives_nominal_coordination() {
    // Goblin Chainwhirler: shared each still selects singular coordinates.
    assert_eq!(
        readings(
            "each creature and planeswalker they control",
            Category::NounPhrase
        )
        .len(),
        2
    );
    assert_constituents(
        "each creature and planeswalker they control",
        Category::NounPhrase,
        &[(Category::Nominal, "creature and planeswalker")],
    );
}

#[test]
fn singular_selecting_determiners_fix_agreement_over_coordinated_nominals() {
    // Brightcap Badger and Great Divide Guide, whose complete faces retain
    // both attachment Readings under the owner's distributive agreement ruling.
    for (card, text) in [
        (
            "Brightcap Badger",
            "Each Fungus and Saproling you control has \"{T}: Add {G}.\"\nAt the beginning of your end step, create a 1/1 green Saproling creature token.",
        ),
        (
            "Great Divide Guide",
            "Each land and Ally you control has \"{T}: Add one mana of any color.\"",
        ),
    ] {
        assert_eq!(common_readings_on_card(card, text).len(), 2, "{card}");
        assert!(common_readings_on_card(card, &text.replacen(" has ", " have ", 1)).is_empty());
    }
    // Goblin Chainwhirler's attested constituent; the complete corpus has
    // no occurrence of this constituent followed by have.
    assert!(
        readings(
            "each creature and planeswalker they control have",
            Category::FiniteClause
        )
        .is_empty()
    );
}

fn common_readings_on_card(card: &str, text: &str) -> std::collections::BTreeSet<Reading> {
    crate::common::readings_on_card(card, text, Category::Document)
}

#[test]
fn alternative_coordination_preserves_singular_agreement() {
    // Captain America's Motorcycle's triggered-effect clause.
    let text = "Target creature or Vehicle gets +2/+0 until end of turn.";
    assert_eq!(readings(text, Category::Sentence).len(), 2);
    assert!(readings(&text.replace(" gets ", " get "), Category::Sentence).is_empty());
}

fn word(id: &str, form: WordForm, features: FeatureBundle, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability,
    }
}

#[test]
fn independently_constructed_coordinated_subject_roundtrips_with_singular_coordinates() {
    let noun = |id| Reading::Noun {
        form: 0,
        head: word(
            id,
            WordForm::Singular,
            FeatureBundle {
                number: Some(Number::Singular),
                ..FeatureBundle::default()
            },
            Some(true),
        ),
    };
    let value = Reading::PossessiveNounPhrase {
        form: 0,
        possessor: word(
            "vocab:PossessiveDeterminerPronoun/Its",
            WordForm::Invariant,
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                case: Some(Case::Genitive),
                ..FeatureBundle::default()
            },
            None,
        ),
        head: Box::new(Reading::Coordination {
            form: 0,
            category: Category::Nominal,
            left: Box::new(noun("lexeme:CommonNoun/Power")),
            coordinator: word(
                "vocab:Coordinator/And",
                WordForm::Invariant,
                FeatureBundle::default(),
                None,
            ),
            right: Box::new(noun("lexeme:CommonNoun/Toughness")),
        }),
    };
    let text = "its power and toughness";
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, Category::NounPhrase);
    let reparsed = parsed
        .get(&value)
        .expect("independent subject retained by parsing");
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
