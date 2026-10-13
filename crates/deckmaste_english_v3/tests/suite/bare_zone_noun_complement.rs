use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn word(owner: &str, form: WordForm) -> Word {
    let singular = form == WordForm::Singular;
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number: singular.then_some(Number::Singular),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: singular.then_some(true),
    }
}

fn bare_location(preposition: &str, noun: &str) -> Reading {
    Reading::BareNominalPreposition {
        form: 0,
        head: word(preposition, WordForm::Invariant),
        complement: Box::new(Reading::BarePrepositionNounPhrase {
            form: 0,
            head: Box::new(Reading::NominalProjection {
                form: 0,
                head: Box::new(Reading::NominalCoreProjection {
                    form: 0,
                    head: Box::new(Reading::Noun {
                        form: 0,
                        head: word(noun, WordForm::Singular),
                    }),
                }),
            }),
        }),
    }
}

fn witness(card: &str, text: &str, preposition: &str, noun: &str) -> BTreeSet<Reading> {
    let expected = bare_location(preposition, noun);
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {card}: {text}");
    for value in &values {
        value.admit(common::lexicon()).unwrap();
        let mut locations = Vec::new();
        value
            .visit(&mut |node| match node {
                Reading::BareNominalPreposition { .. } => locations.push(node.clone()),
                Reading::BareNominalPrepositionPassive {
                    marker, complement, ..
                } => {
                    locations.push(Reading::BareNominalPreposition {
                        form: 0,
                        head: marker.clone(),
                        complement: complement.clone(),
                    });
                }
                _ => {}
            })
            .unwrap();
        assert_eq!(
            locations,
            vec![expected.clone()],
            "wrong bare Complement on {card}"
        );
    }
    values
}

#[test]
fn sea_gate_wreckage_has_a_bare_np_without_an_elided_possessor() {
    witness(
        "Sea Gate Wreckage",
        "Activate only if you have no cards in hand.",
        "vocab:Preposition/In",
        "lexeme:CommonNoun/Hand",
    );
}

#[test]
fn norin_plays_the_card_from_a_bare_location() {
    witness(
        "Norin, Swift Survivalist",
        "You may play that card from exile this turn.",
        "vocab:Preposition/From",
        "lexeme:CommonNoun/Exile",
    );
}

#[test]
fn wild_magic_sorcerer_retains_both_source_attachments() {
    let text = "The first spell you cast from exile each turn has cascade.";
    let values = witness(
        "Wild-Magic Sorcerer",
        text,
        "vocab:Preposition/From",
        "lexeme:CommonNoun/Exile",
    );
    assert_eq!(values.len(), 8);
    // Orchestrator ruling R1b, 2026-10-08: retain VP and NP attachment
    // in this attested constituent. The following "each turn" constrains
    // attachment in the complete witness above.
    let fragment = "the first spell you cast from exile";
    common::assert_constituents(
        fragment,
        Category::NounPhrase,
        &[(Category::FiniteObjectGap, "cast from exile")],
    );
    let mut noun_attachments = BTreeSet::new();
    for value in common::readings(fragment, Category::NounPhrase) {
        value
            .visit(&mut |node| {
                if let Reading::PostmodifiedNominal { head, modifier, .. } = node
                    && modifier.realize(common::lexicon()).unwrap() == "from exile"
                {
                    noun_attachments.insert(head.realize(common::lexicon()).unwrap());
                }
            })
            .unwrap();
    }
    assert_eq!(
        noun_attachments,
        BTreeSet::from([
            "first spell you cast".to_owned(),
            "spell you cast".to_owned(),
        ])
    );
}

#[test]
fn umbris_uses_the_noun_exile_in_the_scoped_card_text() {
    witness(
        "Umbris, Fear Manifest",
        "Umbris gets +1/+1 for each card your opponents own in exile.",
        "vocab:Preposition/In",
        "lexeme:CommonNoun/Exile",
    );
}

#[test]
fn ennis_puts_cards_into_a_bare_location() {
    witness(
        "Ennis, Debate Moderator",
        "At the beginning of your end step, if one or more cards were put into exile this turn, put a +1/+1 counter on Ennis.",
        "vocab:Preposition/Into",
        "lexeme:CommonNoun/Exile",
    );
}

#[test]
fn independently_constructed_bare_location_pps_obey_both_roundtrip_laws() {
    // Constituents of the five ticket witnesses, not invented instructions.
    for (text, preposition, noun) in [
        ("in hand", "vocab:Preposition/In", "lexeme:CommonNoun/Hand"),
        (
            "from exile",
            "vocab:Preposition/From",
            "lexeme:CommonNoun/Exile",
        ),
        (
            "in exile",
            "vocab:Preposition/In",
            "lexeme:CommonNoun/Exile",
        ),
        (
            "into exile",
            "vocab:Preposition/Into",
            "lexeme:CommonNoun/Exile",
        ),
    ] {
        let expected = bare_location(preposition, noun);
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::PrepositionPhrase),
            BTreeSet::from([expected])
        );
    }
    let Reading::BareNominalPreposition {
        head: marker,
        complement,
        ..
    } = bare_location("vocab:Preposition/Into", "lexeme:CommonNoun/Exile")
    else {
        unreachable!()
    };
    let mut head = word("core-verb:Put", WordForm::PastParticiple);
    head.frame = Some(3);
    if let LexicalReading::Word(value) = &mut head.value {
        value.features.finiteness = Some(deckmaste_lexical::Finiteness::Nonfinite);
    }
    let expected = Reading::BareNominalPrepositionPassive {
        form: 0,
        head,
        marker,
        complement,
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "put into exile"
    );
    assert_eq!(
        common::readings("put into exile", Category::SecondaryVerbPhrase),
        BTreeSet::from([expected])
    );
}

#[test]
fn bare_location_permission_does_not_escape_to_other_prepositions_or_cases() {
    for text in [
        "on exile",
        "on hand",
        "of exile",
        "of hand",
        "to exile",
        "to hand",
        "from graveyard",
        "in library",
        "in top of your library",
    ] {
        assert!(
            common::readings(text, Category::PrepositionPhrase).is_empty(),
            "{text}"
        );
    }
    for text in [
        "put on exile",
        "put into graveyard",
        "put into top of your library",
    ] {
        assert!(
            common::readings(text, Category::SecondaryVerbPhrase).is_empty(),
            "{text}"
        );
    }
    for text in ["exile", "hand"] {
        for category in [Category::NominativePhrase, Category::AccusativePhrase] {
            assert!(
                common::readings(text, category).is_empty(),
                "{text} at {category:?}"
            );
        }
    }
    let invalid = bare_location("vocab:Preposition/On", "lexeme:CommonNoun/Exile");
    assert!(invalid.admit(common::lexicon()).is_err());
}

#[test]
fn determined_and_spatial_edge_complements_keep_their_existing_routes() {
    for text in [
        "in your hand",
        "from your graveyard",
        "into a graveyard",
        "in their graveyards",
        "on top of your library",
    ] {
        let values = common::readings(text, Category::PrepositionPhrase);
        assert_eq!(values.len(), 1, "{text}");
        for value in values {
            value.admit(common::lexicon()).unwrap();
            let mut bare_locations = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::BareNominalPreposition { .. } = node {
                        bare_locations.push(node.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
            assert_eq!(
                bare_locations,
                if text.starts_with("on top") { vec![text.to_owned()] } else { vec![] }
            );
        }
    }
}
