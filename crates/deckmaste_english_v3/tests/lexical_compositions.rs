mod common;

use std::collections::BTreeSet;

use common::{lexicon, readings};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm};

fn word(owner: &str, form: WordForm, features: FeatureBundle, countability: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(), form, features, variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability, frame: None,
    }
}

fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    assert_eq!(readings(text, category), BTreeSet::from([expected]));
}

#[test]
fn combat_damage_keeps_its_noun_premodifier_and_mass_head() {
    // Fell Flagship's triggered ability.
    assert!(!readings(
        "Whenever this Vehicle deals combat damage to a player, that player discards a card.",
        Category::Document,
    ).is_empty());
    let features = FeatureBundle { number: Some(Number::Singular), ..Default::default() };
    exact("combat damage", Category::Nominal, Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: word("lexeme:turn_part/combat", WordForm::Singular, features.clone(), Some(true)),
        }),
        head: Box::new(Reading::Noun {
            form: 0,
            head: word("lexeme:CommonNoun/Damage", WordForm::Singular, features, Some(false)),
        }),
    });
    assert!(readings("combats damage", Category::Nominal).is_empty());
}

#[test]
fn defending_player_is_a_bare_np_with_a_verbal_premodifier() {
    // Odious Witch; "the defending player" also occurs on Goblin Firebug.
    assert!(!readings(
        "Whenever this creature attacks, defending player loses 1 life and you gain 1 life.",
        Category::Document,
    ).is_empty());
    let mut modifier = word("lexeme:Verb/Defend", WordForm::GerundParticiple,
        FeatureBundle { finiteness: Some(deckmaste_lexical::Finiteness::Nonfinite), ..Default::default() }, None);
    modifier.frame = Some(0);
    exact("defending player", Category::NounPhrase, Reading::BareParticipialStatusNounPhrase {
        form: 0, modifier,
        head: Box::new(Reading::Noun { form: 0, head: word("lexeme:CommonNoun/Player", WordForm::Singular,
            FeatureBundle { number: Some(Number::Singular), ..Default::default() }, Some(true)) }),
    });
    for text in ["defend player", "defended player", "attacking player"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}
