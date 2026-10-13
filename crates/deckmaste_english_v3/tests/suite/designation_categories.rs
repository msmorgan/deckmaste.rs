use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, SurfaceCase, WordForm};

use crate::common;

fn witness(card: &str, text: &str, owner: &str, expected: &[(Category, &str)]) {
    let values = common::readings_on_card(card, text, Category::Sentence);
    assert!(!values.is_empty(), "{card}: {text}");
    common::assert_constituents_on_card(card, text, Category::Sentence, expected);
    for reading in values {
        let mut owners = Vec::new();
        reading
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value {
                    owners.push(value.lexeme);
                }
            })
            .unwrap();
        assert!(owners.contains(&owner.into()), "{card}: {owners:?}");
    }
}

#[test]
fn commodore_guff_has_an_ordinary_commander_noun() {
    witness(
        "Commodore Guff",
        "Commodore Guff can be your commander.",
        "lexeme:designation/commander",
        &[(Category::NounPhrase, "your commander")],
    );
}

#[test]
fn flamekin_herald_uses_the_existing_noun_as_a_premodifier() {
    witness(
        "Flamekin Herald",
        "Commander spells you cast have cascade.",
        "lexeme:designation/commander",
        &[(Category::NounPremodifier, "Commander")],
    );
}

#[test]
fn blazing_firesinger_enters_with_an_adjective() {
    witness(
        "Blazing Firesinger",
        "This creature enters prepared.",
        "lexeme:designation/prepared",
        &[(Category::AdjectivePhrase, "prepared")],
    );
}

#[test]
fn colossus_of_akros_has_an_adjectival_predicate() {
    witness(
        "Colossus of Akros",
        "As long as this creature is monstrous, it has trample and can attack as though it didn't have defender.",
        "lexeme:designation/monstrous",
        &[(Category::AdjectivePhrase, "monstrous")],
    );
}

#[test]
fn court_of_ire_keeps_the_determiner_separate_from_the_noun() {
    witness(
        "Court of Ire",
        "When this enchantment enters, you become the monarch.",
        "lexeme:designation/monarch",
        &[
            (Category::NounPhrase, "the monarch"),
            (Category::Nominal, "monarch"),
        ],
    );
}

#[test]
fn goaded_premodification_keeps_the_verb_owner() {
    // Attested on Baeloth Barrityl, Entertainer.
    let text =
        "Whenever a goaded attacking or blocking creature dies, you create a Treasure token.";
    witness(
        "Baeloth Barrityl, Entertainer",
        text,
        "lexeme:keyword_action/goad",
        &[(Category::VerbalPremodifier, "goaded")],
    );
}

fn word(owner: &str, form: WordForm, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability,
    }
}

#[test]
fn independently_constructed_nouns_and_adjectives_roundtrip_with_their_owners() {
    for (owner, form, surface) in [
        (
            "lexeme:designation/commander",
            WordForm::Singular,
            "commander",
        ),
        (
            "lexeme:designation/commander",
            WordForm::Plural,
            "commanders",
        ),
        ("lexeme:designation/monarch", WordForm::Singular, "monarch"),
        (
            "lexeme:designation/initiative",
            WordForm::Singular,
            "initiative",
        ),
        (
            "lexeme:designation/citysBlessing",
            WordForm::Singular,
            "city's blessing",
        ),
    ] {
        let expected = Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word(owner, form, Some(true)),
                }),
            }),
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), surface);
        assert_eq!(
            common::readings(surface, Category::Nominal),
            BTreeSet::from([expected])
        );
    }
    for surface in ["prepared", "saddled", "monstrous", "renowned", "solved"] {
        let owner = format!("lexeme:designation/{surface}");
        let expected = Reading::Adjective {
            form: 0,
            head: word(&owner, WordForm::Invariant, None),
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), surface);
        assert_eq!(
            common::readings(surface, Category::AdjectivePhrase),
            BTreeSet::from([expected])
        );
        assert_eq!(
            common::lexicon().lexemes()[owner.as_str()].category,
            deckmaste_lexical::Category::Adjective
        );
    }
    for surface in ["goaded", "suspected", "harnessed"] {
        assert!(
            !common::lexicon()
                .lexemes()
                .contains_key(format!("lexeme:designation/{surface}").as_str())
        );
    }
    let blessing = &common::lexicon().lexemes()["lexeme:designation/citysBlessing"];
    assert_eq!(
        blessing.surface_structure,
        deckmaste_lexical::SurfaceStructure::Multiword
    );
    assert_eq!(
        blessing
            .forms
            .iter()
            .map(|slot| slot.form)
            .collect::<Vec<_>>(),
        [WordForm::Singular]
    );
}
