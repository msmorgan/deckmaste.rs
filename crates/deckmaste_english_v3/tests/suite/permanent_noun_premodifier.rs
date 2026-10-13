use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading};

fn witness(text: &str, expected_heads: &[&str]) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {text:?}");
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::BarrierNounPremodifiedNominal { modifier, head, .. } = node
                    && modifier.realize(common::lexicon()).unwrap() == "permanent"
                {
                    let head_text = head.realize(common::lexicon()).unwrap();
                    assert!(expected_heads.contains(&head_text.as_str()), "{head:?}");
                    assert_eq!(
                        node.realize(common::lexicon()).unwrap(),
                        format!("permanent {head_text}")
                    );
                    let mut words = Vec::new();
                    modifier
                        .visit_words(&mut |word| words.push(word.clone()))
                        .unwrap();
                    modifiers.push(words);
                }
            })
            .unwrap();
        let mut expected = Vec::new();
        common::readings("permanent", Category::NounPremodifier)
            .first()
            .unwrap()
            .visit_words(&mut |word| expected.push(word.clone()))
            .unwrap();
        assert_eq!(modifiers, vec![expected]);
        value.admit(common::lexicon()).unwrap();
    }
}

#[test]
fn cosmic_rebirth() {
    witness(
        "Choose target permanent card in your graveyard.",
        &["card", "card in your graveyard"],
    );
}

#[test]
fn summon_esper_maduin() {
    witness(
        "If it's a permanent card, put it into your hand.",
        &["card"],
    );
}

#[test]
fn primevals_glorious_rebirth() {
    witness(
        "Return all legendary permanent cards from your graveyard to the battlefield.",
        &["cards"],
    );
}

#[test]
fn divine_gambit() {
    witness(
        "That player may put a permanent card from their hand onto the battlefield.",
        &["card", "card from their hand"],
    );
}

#[test]
fn skyshroud_poacher() {
    witness(
        "Search your library for an Elf permanent card, put it onto the battlefield, then shuffle.",
        &["card"],
    );
}
