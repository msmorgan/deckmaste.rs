use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::WordForm;

use crate::common::assert_constituents;
use crate::common::lexicon;
use crate::common::readings;
use crate::scalar_property_values::assert_laws;
use crate::scalar_property_values::word;

fn witness(text: &str, nominal: &str, participle: &str, owner: &str) {
    assert_constituents(
        text,
        Category::Ability,
        &[
            (Category::Nominal, nominal),
            (Category::VerbalPremodifier, participle),
        ],
    );
    for value in readings(text, Category::Ability) {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::VerbalPremodifier { head, .. } = node {
                    modifiers.push(head.clone());
                }
            })
            .unwrap();
        assert_eq!(modifiers.len(), 1, "{value:?}");
        for head in modifiers {
            let LexicalReading::Word(value) = head.value else {
                panic!("verb word required")
            };
            assert_eq!(value.lexeme.as_str(), owner);
            assert_eq!(value.form, WordForm::PastParticiple);
            assert_eq!(value.features.finiteness, Some(Finiteness::Nonfinite));
            assert_eq!(value.features.tense, None);
        }
    }
}

#[test]
fn ugins_labyrinth_returns_the_exiled_card() {
    witness(
        "Return the exiled card to its owner's hand.",
        "exiled card",
        "exiled",
        "lexeme:keyword_action/exile",
    );
}

#[test]
fn florian_plays_the_exiled_card() {
    witness(
        "You may play the exiled card this turn.",
        "exiled card",
        "exiled",
        "lexeme:keyword_action/exile",
    );
}

#[test]
fn isochron_scepter_copies_the_exiled_card() {
    witness(
        "You may copy the exiled card.",
        "exiled card",
        "exiled",
        "lexeme:keyword_action/exile",
    );
}

#[test]
fn erratic_explosion_puts_the_revealed_cards_on_the_bottom() {
    witness(
        "Put the revealed cards on the bottom of your library in any order.",
        "revealed cards",
        "revealed",
        "lexeme:keyword_action/reveal",
    );
}

#[test]
fn the_princess_takes_flight_returns_the_exiled_card() {
    witness(
        "Return the exiled card to the battlefield under its owner's control.",
        "exiled card",
        "exiled",
        "lexeme:keyword_action/exile",
    );
}

#[test]
fn independently_built_attested_modifiers_preserve_passive_form_and_both_laws() {
    // Ugin's Labyrinth; Erratic Explosion; Strionic Resonator;
    // Malcolm, Alluring Scoundrel; Seek Thrills, respectively.
    for (owner, frame, surface, noun, plural) in [
        (
            "lexeme:keyword_action/exile",
            0,
            "exiled card",
            "Card",
            false,
        ),
        (
            "lexeme:keyword_action/reveal",
            0,
            "revealed cards",
            "Card",
            true,
        ),
        (
            "core-verb:Trigger",
            1,
            "triggered ability",
            "Ability",
            false,
        ),
        (
            "lexeme:keyword_action/discard",
            0,
            "discarded card",
            "Card",
            false,
        ),
        (
            "lexeme:keyword_action/mill",
            0,
            "milled cards",
            "Card",
            true,
        ),
    ] {
        let mut verb = word(owner, WordForm::PastParticiple);
        verb.frame = Some(frame);
        if let LexicalReading::Word(value) = &mut verb.value {
            value.features.finiteness = Some(Finiteness::Nonfinite);
        }
        let modifier = Reading::VerbalPremodifier {
            category: Category::VerbalPremodifier,
            form: 0,
            head: verb,
        };
        let value = Reading::ParticipialPremodifier {
            form: 0,
            modifier: Box::new(modifier),
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(
                    &format!("lexeme:CommonNoun/{noun}"),
                    if plural { WordForm::Plural } else { WordForm::Singular },
                ),
            }),
        };
        assert_laws(&value, surface, Category::Nominal);
        assert_eq!(
            readings(surface, Category::Nominal),
            std::collections::BTreeSet::from([value])
        );
    }
    // Trigger's intransitive frame has no passive patient to modify.
    let mut head = word("core-verb:Trigger", WordForm::PastParticiple);
    head.frame = Some(0);
    if let LexicalReading::Word(value) = &mut head.value {
        value.features.finiteness = Some(Finiteness::Nonfinite);
    }
    assert!(
        Reading::VerbalPremodifier { category: Category::VerbalPremodifier, form: 0, head }
            .admit(lexicon())
            .is_err()
    );
}

#[test]
fn lithoform_engine_coordinates_activated_and_triggered() {
    assert_constituents(
        "Copy target activated or triggered ability you control.",
        Category::Ability,
        &[
            (Category::VerbalPremodifier, "activated or triggered"),
            (Category::VerbalPremodifier, "activated"),
            (Category::VerbalPremodifier, "triggered"),
        ],
    );
}
