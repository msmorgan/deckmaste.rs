use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

#[test]
fn predicate_and_event_terms_fill_the_attested_grammatical_roles() {
    for (card, text, role, surface) in [
        (
            "Artificer's Assistant",
            "Whenever you cast a historic spell, scry 1.",
            Category::AdjectivePhrase,
            "historic",
        ),
        (
            "Jhoira's Familiar",
            "Historic spells you cast cost {1} less to cast.",
            Category::AdjectivePhrase,
            "Historic",
        ),
        (
            "Hellspur Posse Boss",
            "Other outlaws you control have haste.",
            Category::Nominal,
            "outlaws",
        ),
        (
            "Double Down",
            "Whenever you cast an outlaw spell, copy that spell.",
            Category::NounPremodifier,
            "outlaw",
        ),
        (
            "Patrolling Peacemaker",
            "Whenever an opponent commits a crime, proliferate.",
            Category::FinitePredicate,
            "commits a crime",
        ),
        (
            "Intimidation Campaign",
            "Whenever you commit a crime, you may return this enchantment to its owner's hand.",
            Category::FinitePredicate,
            "commit a crime",
        ),
        (
            "Shoot the Sheriff",
            "Destroy target non-outlaw creature.",
            Category::NounPremodifier,
            "non-outlaw",
        ),
        (
            "Nimble Brigand",
            "This creature can't be blocked if you've committed a crime this turn.",
            Category::SecondaryVerbPhrase,
            "committed a crime",
        ),
    ] {
        common::assert_constituents_on_card(card, text, Category::Sentence, &[(role, surface)]);
    }
}

#[test]
fn independently_constructed_outlaw_premodifier_keeps_noun_identity_and_roundtrips() {
    // Double Down: "Whenever you cast an outlaw spell, copy that spell."
    let expected = Reading::NounPremodifier {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "lexeme:predicate/outlaw".into(),
                form: WordForm::Singular,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(true),
        },
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "outlaw");
    assert_eq!(
        common::readings("outlaw", Category::NounPremodifier),
        BTreeSet::from([expected])
    );
}
