mod common;

use std::collections::BTreeSet;

use common::{lexicon, readings};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm};

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

#[test]
fn plural_bare_genitives_preserve_possessor_number_and_the_apostrophe() {
    // Upheaval.
    assert!(!readings("Return all permanents to their owners' hands.", Category::Document).is_empty());
    let possessor = Reading::PossessiveNounPhrase {
        form: 0,
        possessor: word("vocab:PossessiveDeterminerPronoun/Their", WordForm::Invariant,
            FeatureBundle { number: Some(Number::Plural), person: Some(Person::Third),
                case: Some(deckmaste_lexical::Case::Genitive), ..Default::default() }, None),
        head: Box::new(Reading::Noun {form: 0, head: word("lexeme:CommonNoun/Owner", WordForm::Plural,
            FeatureBundle {number: Some(Number::Plural), ..Default::default()}, Some(true))}),
    };
    let expected = Reading::GenitiveNounPhrase {
        form: 0, possessor: Box::new(possessor),
        marker: word("vocab:Genitive/Sibilant", WordForm::Invariant, FeatureBundle::default(), None),
        head: Box::new(Reading::Noun {form: 0, head: word("lexeme:CommonNoun/Hand", WordForm::Plural,
            FeatureBundle {number: Some(Number::Plural), ..Default::default()}, Some(true))}),
    };
    exact("their owners' hands", Category::NounPhrase, expected.clone());
    let mut wrong = expected.clone();
    let Reading::GenitiveNounPhrase { possessor, .. } = &mut wrong else { unreachable!() };
    let Reading::PossessiveNounPhrase { head, .. } = &mut **possessor else { unreachable!() };
    *head = Box::new(Reading::PostmodifiedNominal {
        form: 0, head: head.clone(),
        modifier: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: word("vocab:Preposition/Of", WordForm::Invariant, FeatureBundle::default(), None),
            complement: Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase, form: 0,
                head: Box::new(Reading::BarePlural {
                    form: 0, head: Box::new(Reading::Noun {
                        form: 0, head: word("lexeme:CommonNoun/Card", WordForm::Plural,
                            FeatureBundle { number: Some(Number::Plural), ..Default::default() }, Some(true)),
                    }),
                }),
            }),
        }),
    });
    // The final noun's eligibility cannot be inherited from an earlier head.
    // The text can separately read with the genitive inside the of phrase.
    assert!(wrong.admit(lexicon()).is_err());
    let mut curved = expected;
    let Reading::GenitiveNounPhrase { marker, .. } = &mut curved else { unreachable!() };
    let LexicalReading::Word(value) = &mut marker.value else { unreachable!() };
    value.variant = 1;
    exact("their owners’ hands", Category::NounPhrase, curved);
    for text in ["their owner's' hands", "their owner' hands", "their owners hands", "their owners's hands", "their Mice' cards"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}
