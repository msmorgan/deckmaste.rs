use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

fn witness(card: &str, text: &str, restriction: &str) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut adjuncts = 0;
        value
            .visit(&mut |node| {
                let modifier = match node {
                    Reading::PrepositionPredicate { modifier, .. } => Some(modifier),
                    Reading::ClausalPreposition { dependent, .. } => Some(dependent),
                    _ => None,
                };
                if let Some(modifier) = modifier
                    && modifier.realize(common::lexicon()).unwrap() == restriction
                {
                    let Reading::FocusedPhrase { head, .. } = modifier.as_ref() else {
                        panic!("expected a focused Adjunct: {modifier:?}");
                    };
                    let Reading::PredicativeComplementPreposition { complement, .. } =
                        head.as_ref()
                    else {
                        panic!("expected nominal predicative as: {head:?}");
                    };
                    assert_eq!(complement.category(), Category::PredicativeComplement);
                    adjuncts += 1;
                }
                if let Reading::PostmodifiedNominal { modifier, .. } = node {
                    assert_ne!(modifier.realize(common::lexicon()).unwrap(), restriction);
                }
            })
            .unwrap();
        assert_eq!(adjuncts, 1, "{card}");
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn experimental_synthesizer() {
    witness(
        "Experimental Synthesizer",
        "When this artifact enters or leaves the battlefield, exile the top card of your library. Until end of turn, you may play that card.\n{2}{R}, Sacrifice this artifact: Create a 2/2 white Samurai creature token with vigilance. Activate only as a sorcery.",
        "only as a sorcery",
    );
}

#[test]
fn lions_eye_diamond() {
    witness(
        "Lion's Eye Diamond",
        "Discard your hand, Sacrifice this artifact: Add three mana of any one color. Activate only as an instant.",
        "only as an instant",
    );
}

#[test]
fn sun_blessed_guardian() {
    witness(
        "Sun-Blessed Guardian",
        "{5}{R/P}: Transform this creature. Activate only as a sorcery.",
        "only as a sorcery",
    );
}

#[test]
fn intrepid_tenderfoot() {
    witness(
        "Intrepid Tenderfoot",
        "{3}: Put a +1/+1 counter on this creature. Activate only as a sorcery.",
        "only as a sorcery",
    );
}

#[test]
fn cat_burglar() {
    witness(
        "Cat Burglar",
        "{2}{B}, {T}: Target player discards a card. Activate only as a sorcery.",
        "only as a sorcery",
    );
}

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

#[test]
fn independently_constructed_focused_predicative_roundtrips() {
    // Experimental Synthesizer's attested restriction, independently constructed.
    let mut article = invariant("vocab:Article/Indefinite");
    let LexicalReading::Word(value) = &mut article.value else { unreachable!() };
    value.features.number = Some(Number::Singular);
    let mut noun = invariant("lexeme:type/sorcery");
    let LexicalReading::Word(value) = &mut noun.value else { unreachable!() };
    value.form = WordForm::Singular;
    value.features.number = Some(Number::Singular);
    noun.countability = Some(true);
    let expected = Reading::FocusedPhrase {
        category: Category::PrepositionPhrase,
        form: 0,
        modifier: invariant("vocab:FocusAdverb/Only"),
        head: Box::new(Reading::PredicativeComplementPreposition {
            form: 0,
            head: invariant("vocab:Preposition/As"),
            complement: Box::new(Reading::NominalComplement {
                form: 0,
                phrase: Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(Reading::IndefiniteNounPhrase {
                        form: 0,
                        determiner: article,
                        head: Box::new(Reading::NominalProjection {
                            form: 0,
                            head: Box::new(Reading::NominalCoreProjection {
                                form: 0,
                                head: Box::new(Reading::Noun {
                                    form: 0,
                                    head: noun,
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        }),
    };
    expected.admit(common::lexicon()).unwrap();
    let text = "only as a sorcery";
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert_eq!(
        common::readings(text, Category::PrepositionPhrase),
        BTreeSet::from([expected])
    );
}

#[test]
fn selected_and_comparative_controls_gain_no_as_adjunct() {
    for (card, text) in [
        (
            "Counterbore",
            "Counter target spell. Search its controller's graveyard, hand, and library for all cards with the same name as that spell and exile them. Then that player shuffles.",
        ),
        (
            "Aven Shrine",
            "Whenever a player casts a spell, that player gains X life, where X is the number of cards in all graveyards with the same name as that spell.",
        ),
        (
            "Clone",
            "You may have this creature enter as a copy of any creature on the battlefield.",
        ),
    ] {
        let values = common::readings_on_card(card, text, Category::Document);
        assert!(
            !values.is_empty(),
            "{card}: the selected/comparative control must be read"
        );
        for value in values {
            value
                .visit(&mut |node| {
                    let modifier = match node {
                        Reading::PrepositionPredicate { modifier, .. } => modifier,
                        Reading::ClausalPreposition { dependent, .. } => dependent,
                        _ => return,
                    };
                    assert!(
                        !modifier
                            .realize(common::lexicon())
                            .unwrap()
                            .starts_with("as "),
                        "{card}: {node:?}"
                    );
                })
                .unwrap();
        }
    }
}
