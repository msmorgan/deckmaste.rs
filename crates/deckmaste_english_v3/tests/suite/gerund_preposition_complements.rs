use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::WordForm;

use crate::common;

fn witness(text: &str, pp: &str, gerund: &str) {
    common::assert_constituents(
        text,
        Category::Ability,
        &[
            (Category::PrepositionPhrase, pp),
            (Category::SecondaryVerbPhrase, gerund),
        ],
    );
    for reading in common::ability_readings(text) {
        let mut found = false;
        let mut nominal_attachment = false;
        reading
            .visit(&mut |node| {
                if let Reading::PostmodifiedNominal { modifier, .. } = node {
                    modifier
                        .visit(&mut |child| {
                            if let Reading::GerundComplementPreposition { complement, .. } = child {
                                nominal_attachment |=
                                    complement.realize(common::lexicon()).unwrap() == gerund;
                            }
                        })
                        .unwrap();
                }
                if let Reading::GerundComplementPreposition { complement, .. } = node
                    && complement.realize(common::lexicon()).unwrap() == gerund
                {
                    found = true;
                }
            })
            .unwrap();
        assert!(
            !nominal_attachment,
            "gerund PP inside Object NP: {reading:?}"
        );
        assert!(found, "missing clausal Complement in {reading:?}");
    }
}

#[test]
fn storm_of_memories() {
    witness(
        "You may cast it without paying its mana cost.",
        "without paying its mana cost",
        "paying its mana cost",
    );
}

#[test]
fn spellweaver_volute() {
    witness(
        "You may cast the copy without paying its mana cost.",
        "without paying its mana cost",
        "paying its mana cost",
    );
    assert!(!common::ability_readings("You may cast the copy.").is_empty());
}

#[test]
fn knowledge_exploitation() {
    witness(
        "You may cast that card without paying its mana cost.",
        "without paying its mana cost",
        "paying its mana cost",
    );
}

#[test]
fn syncopate() {
    witness(
        "If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
        "instead of putting it into its owner's graveyard",
        "putting it into its owner's graveyard",
    );
}

#[test]
fn the_five_doctors() {
    witness(
        "If this spell was kicked, put those cards onto the battlefield instead of putting them into your hand.",
        "instead of putting them into your hand",
        "putting them into your hand",
    );
}

fn word(
    owner: &str,
    form: deckmaste_lexical::WordForm,
    frame: Option<usize>,
    countability: Option<bool>,
) -> deckmaste_english_v3::grammar::Word {
    deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == deckmaste_lexical::SurfaceCase::Declared
                })
                .unwrap()
                .clone(),
        ),
        frame,
        countability,
    }
}

fn without_paying() -> Reading {
    Reading::GerundComplementPreposition {
        form: 0,
        head: word("vocab:Preposition/Without", WordForm::Invariant, None, None),
        complement: Box::new(Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word("core-verb:Pay", WordForm::GerundParticiple, Some(0), None),
            complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(Reading::PossessiveNounPhrase {
                        form: 0,
                        possessor: word(
                            "vocab:PossessiveDeterminerPronoun/Its",
                            WordForm::Invariant,
                            None,
                            None,
                        ),
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word(
                                "lexeme:CommonNoun/ManaCost",
                                WordForm::Singular,
                                None,
                                Some(true),
                            ),
                        }),
                    }),
                }),
            )],
        }),
    }
}

#[test]
fn independently_constructed_complement_preserves_both_roundtrip_laws() {
    let expected = without_paying();
    let text = "without paying its mana cost";
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert_eq!(
        common::readings(text, Category::PrepositionPhrase),
        std::collections::BTreeSet::from([expected.clone()])
    );
    let mut words = Vec::new();
    expected
        .visit_words(&mut |value| words.push(value.clone()))
        .unwrap();
    assert_eq!(
        words,
        vec![
            word("vocab:Preposition/Without", WordForm::Invariant, None, None),
            word("core-verb:Pay", WordForm::GerundParticiple, Some(0), None),
            word(
                "vocab:PossessiveDeterminerPronoun/Its",
                WordForm::Invariant,
                None,
                None
            ),
            word(
                "lexeme:CommonNoun/ManaCost",
                WordForm::Singular,
                None,
                Some(true)
            ),
        ]
    );
}

#[test]
fn gerund_complement_requires_declared_licence() {
    for (owner, text) in [
        ("vocab:Preposition/Without", "without paying its mana cost"),
        (
            "vocab:Preposition/Of",
            "instead of putting it into its owner's graveyard",
        ),
    ] {
        let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
        lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id.as_str() == owner)
            .unwrap()
            .properties
            .features
            .remove("GerundClauseComplement");
        let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
        assert!(
            common::readings_with_lexicon(&lexicon, text, Category::PrepositionPhrase).is_empty()
        );
        if owner == "vocab:Preposition/Without" {
            assert!(without_paying().admit(&lexicon).is_err());
        }
    }
    assert!(common::readings("without pay its mana cost", Category::PrepositionPhrase).is_empty());
    assert!(common::readings("without paid its mana cost", Category::PrepositionPhrase).is_empty());
}

#[test]
fn complement_shape_controls_nominal_modifier_permission() {
    assert!(
        common::readings(
            "the copy without paying its mana cost",
            Category::NounPhrase
        )
        .is_empty()
    );
    assert!(!common::readings("creatures without flying", Category::NounPhrase).is_empty());
    assert!(!common::readings("instead of any other type", Category::PrepositionPhrase).is_empty());
}

#[test]
fn coordination_does_not_restore_a_forbidden_nominal_attachment() {
    for text in [
        "the copy without paying its mana cost and without paying its mana cost",
        "the copy either without paying its mana cost or without paying its mana cost",
        "the copy both without paying its mana cost and without paying its mana cost",
        "the copy without paying its mana cost, without paying its mana cost, and without paying its mana cost",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn jazal_goldmane_keeps_its_nominal_complement() {
    let text = "First strike\n{3}{W}{W}: Attacking creatures you control get +X/+X until end of turn, where X is the number of attacking creatures.";
    let readings = common::readings(text, Category::Document);
    assert_eq!(readings.len(), 4);
    for reading in readings {
        reading
            .visit(&mut |node| {
                assert!(
                    !matches!(node, Reading::GerundComplementPreposition { .. }),
                    "{reading:?}"
                );
            })
            .unwrap();
    }
    let predicative = common::readings("of attacking creatures", Category::PredicativeComplement);
    assert!(!predicative.is_empty());
    for reading in predicative {
        reading
            .visit(&mut |node| {
                assert!(
                    !matches!(node, Reading::GerundComplementPreposition { .. }),
                    "{reading:?}"
                );
            })
            .unwrap();
    }
    assert!(
        common::readings("of paying its mana cost", Category::PredicativeComplement).is_empty()
    );
}
