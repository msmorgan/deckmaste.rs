use std::collections::BTreeSet;

use crate::common::{LEXICON, readings};
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};

const WITNESSES: [(&str, &str, &str, &str); 5] = [
    (
        "Surreal Memoir",
        "Return an instant card at random from your graveyard to your hand.",
        "an instant card",
        "your hand",
    ),
    (
        "Make a Wish",
        "Return two cards at random from your graveyard to your hand.",
        "two cards",
        "your hand",
    ),
    (
        "Ghoulraiser",
        "When this creature enters, return a Zombie card at random from your graveyard to your hand.",
        "a Zombie card",
        "your hand",
    ),
    (
        "Haunted Fengraf",
        "{3}, {T}, Sacrifice this land: Return a creature card at random from your graveyard to your hand.",
        "a creature card",
        "your hand",
    ),
    (
        "Moldgraf Monstrosity",
        "When this creature dies, exile it, then return two creature cards at random from your graveyard to the battlefield.",
        "two creature cards",
        "the battlefield",
    ),
];

fn owner(word: &Word) -> &str {
    let LexicalReading::Word(value) = &word.value else {
        panic!("expected a declared Word")
    };
    value.lexeme.as_ref()
}

fn witness(index: usize) {
    let (card, text, object_text, goal_text) = WITNESSES[index];
    let values = readings(text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    let head_text = if text.contains("Return") { "Return" } else { "return" };
    let control = format!("{head_text} {object_text} from your graveyard to {goal_text}");
    let expected = readings(&control, Category::SecondaryVerbPhrase);
    let mut observed = BTreeSet::new();
    for value in values {
        let mut found = false;
        value
            .visit(&mut |node| {
                let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                else {
                    return;
                };
                if owner(head) != "core-verb:Return" {
                    return;
                }
                let [
                    FrameValue::Argument(object),
                    FrameValue::Adjunct(manner),
                    FrameValue::Argument(source),
                    FrameValue::Marker(marker),
                    FrameValue::Argument(goal),
                    FrameValue::Optional(None),
                ] = complements.as_slice()
                else {
                    panic!("{card}: the Adjunct must stand outside the selected slots")
                };
                assert_eq!(object.category(), Category::AccusativePhrase);
                assert_eq!(object.realize(&LEXICON).unwrap(), object_text);
                assert_eq!(source.category(), Category::SourceComplement);
                assert_eq!(source.realize(&LEXICON).unwrap(), "from your graveyard");
                assert_eq!(owner(marker), "vocab:Preposition/To");
                assert_eq!(goal.category(), Category::AccusativePhrase);
                assert_eq!(goal.realize(&LEXICON).unwrap(), goal_text);
                let Reading::InterveningPrepositionAdjunct { head: phrase, .. } = manner.as_ref()
                else {
                    panic!("{card}: missing intervening Adjunct")
                };
                let Reading::AdjectiveComplementPreposition {
                    head, complement, ..
                } = phrase.as_ref()
                else {
                    panic!("{card}: missing manner PP")
                };
                assert_eq!(owner(head), "vocab:Preposition/At");
                assert_eq!(owner(complement), "vocab:Adjective/Random");
                let mut stripped = node.clone();
                let Reading::SelectedPredicate { complements, .. } = &mut stripped else {
                    unreachable!()
                };
                complements.remove(1);
                observed.insert(stripped);
                found = true;
            })
            .unwrap();
        assert!(
            found,
            "{card}: every Reading must attach the manner to return"
        );
    }
    assert_eq!(
        observed, expected,
        "{card}: selected slots and control ambiguity survive"
    );
}

#[test]
fn surreal_memoir() {
    witness(0);
}
#[test]
fn make_a_wish() {
    witness(1);
}
#[test]
fn ghoulraiser() {
    witness(2);
}
#[test]
fn haunted_fengraf() {
    witness(3);
}
#[test]
fn moldgraf_monstrosity() {
    witness(4);
}

#[test]
fn existing_adjuncts_keep_their_routes_and_manner_is_not_an_np_modifier() {
    for text in [
        "Return a creature card from your graveyard to your hand.",
        "Discard a card at random.",
        "Target opponent discards a card at random from their hand.",
        "Whenever this creature deals combat damage to a player, that player reveals a card at random from their hand.",
        "Target player reveals two cards at random from their hand, then discards each nonland card revealed this way.",
        "{2}, Discard a card at random: This creature deals 2 damage to any target.",
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            value
                .visit(&mut |node| {
                    if let Reading::InterveningPrepositionAdjunct { .. } = node {
                        panic!("{text}: an existing free Adjunct gained the selected-frame route");
                    }
                })
                .unwrap();
        }
    }
    assert!(readings("a card at random", Category::NounPhrase).is_empty());
    assert!(
        readings(
            "Return at random a creature card from your graveyard to your hand.",
            Category::Document
        )
        .is_empty()
    );
}

#[test]
fn independently_assembled_frame_preserves_both_roundtrip_laws_and_the_licence() {
    let word = |id: &str, form, frame| Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    };
    let constituent =
        |text, category| Box::new(readings(text, category).into_iter().next().unwrap());
    let value = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Return", WordForm::Plain, Some(0)),
        complements: vec![
            FrameValue::Argument(constituent("an instant card", Category::AccusativePhrase)),
            FrameValue::Adjunct(Box::new(Reading::InterveningPrepositionAdjunct {
                form: 0,
                head: Box::new(Reading::AdjectiveComplementPreposition {
                    form: 0,
                    head: word("vocab:Preposition/At", WordForm::Invariant, None),
                    complement: word("vocab:Adjective/Random", WordForm::Invariant, None),
                }),
            })),
            FrameValue::Argument(constituent(
                "from your graveyard",
                Category::SourceComplement,
            )),
            FrameValue::Marker(word("vocab:Preposition/To", WordForm::Invariant, None)),
            FrameValue::Argument(constituent("your hand", Category::AccusativePhrase)),
            FrameValue::Optional(None),
        ],
    };
    value.admit(&LEXICON).unwrap();
    let text = "return an instant card at random from your graveyard to your hand";
    assert_eq!(value.realize(&LEXICON).unwrap(), text);
    assert_eq!(
        readings(text, Category::SecondaryVerbPhrase),
        BTreeSet::from([value.clone()])
    );
    let mut invalid = value.clone();
    let Reading::SelectedPredicate { complements, .. } = &mut invalid else {
        unreachable!()
    };
    complements.swap(0, 1);
    assert!(
        invalid.admit(&LEXICON).is_err(),
        "manner cannot precede the Object"
    );

    let mut lexemes = LEXICON.lexemes().values().cloned().collect::<Vec<_>>();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id.as_ref() == "vocab:Adjective/Random")
        .unwrap()
        .properties
        .features
        .remove("InterveningAdjunctLicence");
    let unlicensed = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert!(
        value.admit(&unlicensed).is_err(),
        "realization must enforce the declared licence"
    );
    assert!(
        crate::common::readings_with_lexicon(&unlicensed, text, Category::SecondaryVerbPhrase)
            .is_empty()
    );
}
