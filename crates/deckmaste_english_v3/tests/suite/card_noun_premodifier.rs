use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};
use std::collections::BTreeSet;

fn word(owner: &str, number: Number) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: Some(true),
    }
}

fn witness(card: &str, text: &str, count: usize, head_owner: &str) {
    let readings = common::readings_on_card(card, text, Category::Document);
    assert_eq!(readings.len(), count, "{card}");
    for reading in readings {
        let mut heads = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::NounPremodifiedNominal { modifier, head, .. } = node
                    && modifier.realize(common::lexicon()).unwrap() == "card"
                {
                    assert_eq!(
                        **modifier,
                        Reading::NounPremodifier {
                            form: 0,
                            head: word("lexeme:CommonNoun/Card", Number::Singular),
                        }
                    );
                    head.visit_words(&mut |word| {
                        if let LexicalReading::Word(value) = &word.value {
                            heads.push(value.lexeme);
                        }
                    })
                    .unwrap();
                }
            })
            .unwrap();
        assert!(heads.contains(&head_owner.into()), "{reading:?}");
        reading.admit(common::lexicon()).unwrap();
        assert_eq!(reading.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn crop_sigil() {
    witness(
        "Crop Sigil",
        "Activate only if there are four or more card types among cards in your graveyard.",
        11,
        "lexeme:CommonNoun/Type",
    );
}

#[test]
fn peer_past_the_veil() {
    witness(
        "Peer Past the Veil",
        "Then draw X cards, where X is the number of card types among cards in your graveyard.",
        12,
        "lexeme:CommonNoun/Type",
    );
}

#[test]
fn deluge_of_doom() {
    witness(
        "Deluge of Doom",
        "All creatures get -X/-X until end of turn, where X is the number of card types among cards in your graveyard.",
        12,
        "lexeme:CommonNoun/Type",
    );
}

#[test]
fn memoricide() {
    witness(
        "Memoricide",
        "Choose a nonland card name.",
        1,
        "lexeme:CommonNoun/Name",
    );
}

#[test]
fn curse_of_silence() {
    witness(
        "Curse of Silence",
        "As this Aura enters, choose a card name.",
        1,
        "lexeme:CommonNoun/Name",
    );
}

#[test]
fn independently_constructed_card_premodifiers_roundtrip_without_duplicate_routes() {
    // Memoricide and Curse of Silence attest "card name"; Crop Sigil
    // attests "card types"; Stenn, Paranoid Partisan attests "card type".
    for (surface, owner, number) in [
        ("card name", "lexeme:CommonNoun/Name", Number::Singular),
        ("card type", "lexeme:CommonNoun/Type", Number::Singular),
        ("card types", "lexeme:CommonNoun/Type", Number::Plural),
        // Deal Broker: "a card in your card pool".
        ("card pool", "lexeme:CommonNoun/Pool", Number::Singular),
    ] {
        let expected = Reading::NounPremodifiedNominal {
            form: 0,
            modifier: Box::new(Reading::NounPremodifier {
                form: 0,
                head: word("lexeme:CommonNoun/Card", Number::Singular),
            }),
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(owner, number),
            }),
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), surface);
        assert_eq!(
            common::readings(surface, Category::Nominal),
            BTreeSet::from([expected])
        );
    }
}

#[test]
fn open_the_gates_preserves_coordination_without_card_as_a_coordinated_premodifier() {
    let text = "Search your library for a basic land card or Gate card, reveal it, put it into your hand, then shuffle.";
    // The claim-parent census admits 23 Readings of this Open the Gates face.
    let readings = common::readings_on_card("Open the Gates", text, Category::Document);
    assert_eq!(readings.len(), 20);
    for reading in readings {
        reading
            .visit(&mut |node| {
                if let Reading::NounPremodifier { head, .. } = node {
                    let LexicalReading::Word(value) = &head.value else {
                        panic!("noun modifier identity");
                    };
                    assert_ne!(value.lexeme, "lexeme:CommonNoun/Card");
                }
            })
            .unwrap();
    }
}

#[test]
fn attested_card_attributes_are_available_in_choose_instructions() {
    // Blood Oath and Curse of Silence attest these instructions.
    for text in ["Choose a card type.", "Choose a card name."] {
        assert_eq!(common::readings(text, Category::Document).len(), 1);
    }
}
