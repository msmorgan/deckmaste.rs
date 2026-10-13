use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, Lexicon, SurfaceCase, WordForm};

use crate::common;

fn witness(card: &str, text: &str, owner: &str, host: &str) {
    let values = common::readings_on_card(card, text, Category::Sentence);
    assert!(!values.is_empty(), "{card}");
    let intended_hosts = common::noun_phrase_readings(host);
    assert!(!intended_hosts.is_empty(), "{host}");
    let host_start = text.find(host).unwrap();
    let multiplier_start = text[..host_start].trim_end().rfind(' ').unwrap() + 1;
    let control = format!("{}{}", &text[..multiplier_start], &text[host_start..]);
    let tail = &control[multiplier_start..];
    let mut hosts = BTreeSet::new();
    for control_value in common::readings_on_card(card, &control, Category::Sentence) {
        control_value
            .visit(&mut |node| {
                if node.category() == Category::NounPhrase
                    && tail.starts_with(&node.realize(common::lexicon()).unwrap())
                {
                    hosts.insert(node.clone());
                }
            })
            .unwrap();
    }
    assert!(!hosts.is_empty(), "{card}: no original host Readings");
    let mut observed_hosts = BTreeSet::new();
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::PredeterminerNounPhrase { modifier, head, .. } = node {
                    modifiers.push((modifier.clone(), (**head).clone()));
                }
            })
            .unwrap();
        let [(modifier, head)] = modifiers.as_slice() else {
            panic!("{card}: expected one external Modifier of an NP");
        };
        let LexicalReading::Word(word) = &modifier.value else {
            panic!("the multiplier must retain its Lexeme");
        };
        assert_eq!(word.lexeme.as_str(), owner, "{card}");
        assert!(hosts.contains(head), "{card}: altered host Reading");
        observed_hosts.insert(head.clone());
        value.admit(common::lexicon()).unwrap();
    }
    assert!(
        intended_hosts.is_subset(&observed_hosts),
        "{card}: lost the intended host"
    );
    assert_eq!(
        observed_hosts, hosts,
        "{card}: changed the original host Reading set"
    );
}

#[test]
fn boon_reflection() {
    witness(
        "Boon Reflection",
        "If you would gain life, you gain twice that much life instead.",
        "vocab:FrequencyAdverb/Twice",
        "that much life",
    );
}

#[test]
fn fire_servant() {
    witness(
        "Fire Servant",
        "If a red instant or sorcery spell you control would deal damage, it deals double that damage instead.",
        "lexeme:Multiplier/Double",
        "that damage",
    );
}

#[test]
fn polliwallop() {
    witness(
        "Polliwallop",
        "Target creature you control deals damage equal to twice its power to target creature you don't control.",
        "vocab:FrequencyAdverb/Twice",
        "its power",
    );
}

#[test]
fn price_of_progress() {
    witness(
        "Price of Progress",
        "Price of Progress deals damage to each player equal to twice the number of nonbasic lands that player controls.",
        "vocab:FrequencyAdverb/Twice",
        "the number of nonbasic lands that player controls",
    );
}

#[test]
fn jovial_evil() {
    witness(
        "Jovial Evil",
        "Jovial Evil deals X damage to target opponent, where X is twice the number of white creatures that player controls.",
        "vocab:FrequencyAdverb/Twice",
        "the number of white creatures that player controls",
    );
}

#[test]
fn city_on_fire_keeps_the_multiplier_noun() {
    witness(
        "City on Fire",
        "If a source you control would deal damage to a permanent or player, it deals triple that damage instead.",
        "lexeme:Multiplier/Triple",
        "that damage",
    );
}

#[test]
fn bruvac_keeps_the_demonstrative_quantity_inside_the_host() {
    witness(
        "Bruvac the Grandiloquent",
        "If an opponent would mill one or more cards, they mill twice that many cards instead.",
        "vocab:FrequencyAdverb/Twice",
        "that many cards",
    );
}

fn word(owner: &str, form: WordForm, countability: Option<bool>) -> Word {
    let mut value = common::lexicon()
        .values()
        .find(|value| value.lexeme.as_str() == owner && value.form == form && value.variant == 0)
        .unwrap()
        .clone();
    value.capitalization = SurfaceCase::Declared;
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability,
    }
}

#[test]
fn independently_constructed_multiplier_preserves_both_roundtrip_laws() {
    // Polliwallop's scalar Complement, constructed independently of a parse.
    let host = Reading::PossessiveNounPhrase {
        form: 0,
        possessor: word(
            "vocab:PossessiveDeterminerPronoun/Its",
            WordForm::Invariant,
            None,
        ),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word("lexeme:CommonNoun/Power", WordForm::Singular, Some(true)),
                }),
            }),
        }),
    };
    let value = Reading::PredeterminerNounPhrase {
        form: 0,
        modifier: word("vocab:FrequencyAdverb/Twice", WordForm::Invariant, None),
        head: Box::new(host),
    };
    value.admit(common::lexicon()).unwrap();
    let text = value.realize(common::lexicon()).unwrap();
    assert_eq!(text, "twice its power");
    assert_eq!(common::noun_phrase_readings(&text), BTreeSet::from([value]));
}

#[test]
fn declared_licence_preserves_the_controls_complete_reading_sets() {
    let lexemes = common::lexicon()
        .lexemes()
        .values()
        .cloned()
        .map(|mut lexeme| {
            lexeme.properties.features.remove("PredeterminerUse");
            lexeme
        });
    let without_multipliers = Lexicon::new(lexemes).unwrap();
    for text in [
        "Copy that spell twice.",
        "Activate only twice each turn.",
        "Double target player's life total.",
        "Target creature gains double strike until end of turn.",
    ] {
        let expected =
            common::readings_with_lexicon(&without_multipliers, text, Category::Sentence);
        assert!(!expected.is_empty(), "{text}");
        assert_eq!(
            common::readings(text, Category::Sentence),
            expected,
            "{text}"
        );
    }
    assert!(
        common::readings_with_lexicon(
            &without_multipliers,
            "twice its power",
            Category::NounPhrase,
        )
        .is_empty()
    );
}

#[test]
fn nondefinite_and_unclassified_hosts_do_not_gain_a_multiplier_route() {
    // MP4 leaves these attested scalar-X forms outside this mechanism;
    // MP5 protects frequency twice from becoming an NP predeterminer here.
    for text in [
        "twice each turn",
        "twice X stun counters",
        "twice X life",
        "twice -X/-X",
    ] {
        assert!(common::noun_phrase_readings(text).is_empty(), "{text}");
    }
}
