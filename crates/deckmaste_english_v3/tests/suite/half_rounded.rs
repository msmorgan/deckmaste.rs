use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{LexicalReading, Lexicon, SurfaceCase, WordForm};

use crate::common;

fn witness(card: &str, text: &str, host: &[&str], anchor_text: &str, direction: &str) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    let hosts: BTreeSet<_> = host
        .iter()
        .flat_map(|text| common::noun_phrase_readings(text))
        .collect();
    assert!(!hosts.is_empty(), "{host:?}");
    let mut observed_fractions = BTreeSet::new();
    for value in values {
        let mut fractions = Vec::new();
        let mut supplements = Vec::new();
        value
            .visit(&mut |node| match node {
                Reading::PredeterminerNounPhrase { modifier, head, .. } => {
                    let LexicalReading::Word(word) = &modifier.value else {
                        panic!("the fraction must retain its Noun Lexeme");
                    };
                    if word.lexeme.as_str() == "lexeme:Fraction/Half" {
                        observed_fractions.insert(node.clone());
                        fractions.push((modifier.clone(), (**head).clone()));
                    }
                }
                Reading::ParticipialSupplementation {
                    anchor, supplement, ..
                } => {
                    if let Reading::SelectedFramePassive {
                        head, complements, ..
                    } = &**supplement
                    {
                        let LexicalReading::Word(word) = &head.value else {
                            panic!("the supplement must retain its Verb Lexeme");
                        };
                        if word.lexeme.as_str() == "core-verb:Round" {
                            assert_eq!(word.form, WordForm::PastParticiple);
                            let [FrameValue::Marker(particle)] = complements.as_slice() else {
                                panic!("round selects one particle");
                            };
                            let LexicalReading::Word(particle) = &particle.value else {
                                panic!("the particle must retain its Lexeme");
                            };
                            assert_eq!(
                                particle.lexeme.as_str(),
                                format!("vocab:Adverb/{direction}")
                            );
                            supplements.push((
                                anchor.category(),
                                anchor.realize(common::lexicon()).unwrap(),
                            ));
                        }
                    }
                }
                _ => {}
            })
            .unwrap();
        let [(modifier, _head)] = fractions.as_slice() else {
            panic!("{card}: expected one fractional Predeterminer Modifier");
        };
        assert_eq!(modifier.countability, Some(true));
        let [(category, anchor)] = supplements.as_slice() else {
            panic!("{card}: expected one participial Supplement");
        };
        assert!([Category::ImperativeClause, Category::FiniteClause].contains(category));
        assert_eq!(anchor, anchor_text);
        value.admit(common::lexicon()).unwrap();
    }
    // HR-R1 retains the genitive-possessor bracketing as well. Assert the
    // intended constituency by value without requiring it to be unique.
    for head in hosts {
        assert!(observed_fractions.contains(&fraction_over(head)), "{card}");
    }
}

#[test]
fn grievous_wound() {
    witness(
        "Grievous Wound",
        "Whenever enchanted player is dealt damage, they lose half their life, rounded up.",
        &["their life"],
        "they lose half their life",
        "Up",
    );
}

#[test]
fn cut_your_losses() {
    witness(
        "Cut Your Losses",
        "Target player mills half their library, rounded down.",
        &["their library"],
        "Target player mills half their library",
        "Down",
    );
}

#[test]
fn endless_ranks_of_the_dead() {
    witness(
        "Endless Ranks of the Dead",
        "At the beginning of your upkeep, create X 2/2 black Zombie creature tokens, where X is half the number of Zombies you control, rounded down.",
        &["the number of Zombies you control"],
        "X is half the number of Zombies you control",
        "Down",
    );
}

#[test]
fn the_endstone() {
    witness(
        "The Endstone",
        "At the beginning of your end step, your life total becomes half your starting life total, rounded up.",
        &["your starting life total"],
        "your life total becomes half your starting life total",
        "Up",
    );
}

#[test]
fn heartless_hidetsugu() {
    witness(
        "Heartless Hidetsugu",
        "{T}: Heartless Hidetsugu deals damage to each player equal to half that player's life total, rounded down.",
        &["that player's life total"],
        "Heartless Hidetsugu deals damage to each player equal to half that player's life total",
        "Down",
    );
}

#[test]
fn malignus() {
    witness(
        "Malignus",
        "Malignus's power and toughness are each equal to half the highest life total among your opponents, rounded up.",
        &["the highest life total among your opponents"],
        "Malignus's power and toughness are each equal to half the highest life total among your opponents",
        "Up",
    );
}

#[test]
fn fraction_requires_a_declared_definite_host_and_has_no_bare_dependent_route() {
    // Blood Tribute's fraction without a Supplement.
    let values = common::readings("That player loses half their life.", Category::Sentence);
    assert!(!values.is_empty());
    let values = common::noun_phrase_readings("half that player's life total");
    assert!(!values.is_empty());
    // HR-R1 (orchestrator ruling, 2026-10-10): both genitive scopes remain.
    for head in common::noun_phrase_readings("that player's life total") {
        assert!(values.contains(&fraction_over(head)));
    }
    for value in values {
        assert_eq!(
            value.realize(common::lexicon()).unwrap(),
            "half that player's life total"
        );
        value.admit(common::lexicon()).unwrap();
    }
    for text in [
        "half X damage",
        "half X life",
        "half X cards",
        "half of their library",
        "half their life, rounded up",
    ] {
        assert!(common::noun_phrase_readings(text).is_empty(), "{text}");
    }
}

#[test]
fn lexical_licence_controls_supplement_admission_and_keeps_the_multiplier_controls() {
    let without_rounding = Lexicon::new(common::lexicon().lexemes().values().cloned().map(
        |mut lexeme| {
            lexeme
                .properties
                .features
                .remove("ParticipialSupplementUse");
            lexeme
        },
    ))
    .unwrap();
    let text = "Target player mills half their library, rounded down.";
    assert!(common::readings_with_lexicon(&without_rounding, text, Category::Sentence).is_empty());
    for text in [
        "It deals double that damage instead.",
        "Copy that spell twice.",
        "Put twice that many +1/+1 counters on it.",
        "Prevent double that damage.",
    ] {
        let expected = common::readings_with_lexicon(&without_rounding, text, Category::Sentence);
        assert!(!expected.is_empty(), "{text}");
        assert_eq!(
            common::readings(text, Category::Sentence),
            expected,
            "{text}"
        );
    }
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

fn fraction_over(head: Reading) -> Reading {
    Reading::PredeterminerNounPhrase {
        form: 1,
        modifier: word("lexeme:Fraction/Half", WordForm::Singular, Some(true)),
        head: Box::new(head),
    }
}

#[test]
fn independently_constructed_fraction_preserves_both_roundtrip_laws() {
    // Infernal Contract's Object host, built without parsing it.
    let value = Reading::PredeterminerNounPhrase {
        form: 1,
        modifier: word("lexeme:Fraction/Half", WordForm::Singular, Some(true)),
        head: Box::new(Reading::PossessiveNounPhrase {
            form: 0,
            possessor: word(
                "vocab:PossessiveDeterminerPronoun/Your",
                WordForm::Invariant,
                None,
            ),
            head: Box::new(Reading::Noun {
                form: 0,
                head: word("lexeme:CommonNoun/Life", WordForm::Singular, Some(false)),
            }),
        }),
    };
    value.admit(common::lexicon()).unwrap();
    let text = value.realize(common::lexicon()).unwrap();
    assert_eq!(text, "half your life");
    assert_eq!(common::noun_phrase_readings(&text), BTreeSet::from([value]));
}

#[test]
fn independently_composed_supplement_preserves_anchor_and_particle() {
    let lexicon = common::lexicon();
    let mut verb = word("core-verb:Round", WordForm::PastParticiple, None);
    verb.frame = Some(
        lexicon.lexemes()[&deckmaste_lexical::LexemeId::new("core-verb:Round")]
            .properties
            .frames
            .iter()
            .position(|frame| {
                frame.kind == "Passive"
                    && frame.items
                        == vec![deckmaste_lexical::FrameItem::Marker {
                            vocabulary: "Adverb".into(),
                            member: "Up".into(),
                        }]
            })
            .unwrap(),
    );
    let value = Reading::ParticipialSupplementation {
        category: Category::FiniteClause,
        form: 0,
        anchor: Box::new(
            common::readings("You lose half your life", Category::FiniteClause)
                .into_iter()
                .next()
                .unwrap(),
        ),
        supplement: Box::new(Reading::SelectedFramePassive {
            form: 0,
            head: verb,
            complements: vec![FrameValue::Marker(word(
                "vocab:Adverb/Up",
                WordForm::Invariant,
                None,
            ))],
        }),
    };
    value.admit(lexicon).unwrap();
    let text = value.realize(lexicon).unwrap();
    assert_eq!(text, "You lose half your life, rounded up");
    assert_eq!(
        common::readings(&text, Category::FiniteClause),
        BTreeSet::from([value])
    );
}

#[test]
fn lurking_evil_rounding_supplement_anchors_on_the_action_cost() {
    // Lurking Evil, before the colon of its activated ability.
    let values = common::readings("Pay half your life, rounded up", Category::Cost);
    assert!(!values.is_empty());
    for value in values {
        let mut anchors = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ParticipialSupplementation {
                    category,
                    anchor,
                    supplement,
                    ..
                } = node
                {
                    anchors.push((
                        *category,
                        anchor.category(),
                        anchor.realize(common::lexicon()).unwrap(),
                        supplement.realize(common::lexicon()).unwrap(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(
            anchors,
            vec![(
                Category::CostComponent,
                Category::CostComponent,
                "Pay half your life".into(),
                "rounded up".into()
            )]
        );
        value.admit(common::lexicon()).unwrap();
    }
}
