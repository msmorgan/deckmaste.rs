use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, FrameItem, LexicalReading, LexicalValue, Relation, SurfaceCase, WordForm,
};

use crate::common;

fn is_remove(head: &Word) -> bool {
    let LexicalReading::Word(word) = &head.value else {
        return false;
    };
    word.lexeme == "core-verb:Remove"
}

fn selected_source(text: &str, source: &str) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {text}");
    for value in values {
        let mut sources = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && is_remove(head)
                {
                    let [
                        FrameValue::Argument(object),
                        FrameValue::Argument(complement),
                    ] = complements.as_slice()
                    else {
                        panic!("remove must select its Object and Source Complement");
                    };
                    assert_eq!(object.category(), Category::AccusativePhrase);
                    let Reading::SourceComplement { phrase, .. } = complement.as_ref() else {
                        panic!("the Source must be a selected Complement");
                    };
                    assert_eq!(phrase.realize(common::lexicon()).unwrap(), source);
                    sources.push(complement.clone());
                }
                if let Reading::PostmodifiedNominal { modifier, .. } = node {
                    assert_ne!(modifier.realize(common::lexicon()).unwrap(), source);
                }
                if let Reading::PrepositionPredicate { modifier, .. } = node {
                    assert_ne!(modifier.realize(common::lexicon()).unwrap(), source);
                }
            })
            .unwrap();
        assert!(!sources.is_empty(), "missing selected Source in {text}");
    }
}

fn bare_passive_postmodifier(card: Option<&str>, text: &str) {
    let values =
        common::readings_on_card(card.unwrap_or("Saprazzan Cove"), text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::ParticipialPostmodifiedNominal { modifier, .. } = node {
                    modifier
                        .visit(&mut |predicate| {
                            if let Reading::SelectedFramePassive {
                                head, complements, ..
                            } = predicate
                                && is_remove(head)
                            {
                                assert_eq!(complements, &Vec::<FrameValue>::new());
                                modifiers.push(predicate.clone());
                            }
                        })
                        .unwrap();
                }
            })
            .unwrap();
        assert!(!modifiers.is_empty(), "missing bare passive Postmodifier");
    }
}

#[test]
fn storage_counter_removed_this_way_is_a_bare_passive_postmodifier() {
    bare_passive_postmodifier(
        None,
        "{T}, Remove any number of storage counters from this land: Add {U} for each storage counter removed this way.",
    );
}

#[test]
fn catti_brie_retains_the_bare_passive_in_the_variable_supplement() {
    bare_passive_postmodifier(
        Some("Catti-brie of Mithral Hall"),
        "{1}, Remove all +1/+1 counters from Catti-brie: It deals X damage to target attacking or blocking creature an opponent controls, where X is the number of counters removed this way.",
    );
}

#[test]
fn white_ward_selects_an_object_without_a_source() {
    let values = common::readings("This effect doesn't remove this Aura.", Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut objects = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && is_remove(head)
                {
                    let [FrameValue::Argument(object)] = complements.as_slice() else {
                        panic!("bare remove selects only its Object");
                    };
                    assert_eq!(object.category(), Category::AccusativePhrase);
                    objects.push(object.realize(common::lexicon()).unwrap());
                    let frame = &common::lexicon().lexemes()["core-verb:Remove"]
                        .properties
                        .frames[head.frame.unwrap()];
                    assert_eq!(
                        frame.items,
                        vec![FrameItem::Argument(deckmaste_lexical::FrameSlot {
                            relation: Relation::Object,
                            category: "WholeSourceFreeObject".into()
                        })]
                    );
                }
            })
            .unwrap();
        assert_eq!(objects, vec!["this Aura"]);
    }
}

#[test]
fn spires_and_gustcloak_select_combat_as_source() {
    for text in [
        "{T}: Untap target attacking creature an opponent controls and remove it from combat.",
        "Whenever this creature becomes blocked, you may untap it and remove it from combat.",
        "Untap target attacking creature and remove it from combat. Gain control of that creature until end of turn.",
    ] {
        selected_source(text, "from combat");
    }
}

#[test]
fn an_explicit_source_has_one_complement_attachment() {
    let values = common::readings("Remove a counter from this land.", Category::Document);
    assert_eq!(values.len(), 1, "RS1/RS2 require one Source attachment");
    selected_source("Remove a counter from this land.", "from this land");
    let values = common::readings("Remove it from combat.", Category::Document);
    assert_eq!(values.len(), 1, "RS4 requires one Source attachment");
    selected_source("Remove it from combat.", "from combat");
}

#[test]
fn combat_keeps_its_existing_nominal_distributions() {
    assert_eq!(
        common::readings("combat", Category::NounPhrase),
        BTreeSet::new()
    );
    common::assert_constituents(
        "combat damage",
        Category::Nominal,
        &[
            (Category::NounPremodifier, "combat"),
            (Category::Nominal, "damage"),
        ],
    );
    common::assert_constituents(
        "Draw a card during combat.",
        Category::Document,
        &[
            (Category::PrepositionPhrase, "during combat"),
            (Category::BareTemporalNominal, "combat"),
        ],
    );
}

#[test]
fn source_controls_keep_their_selected_complements() {
    for (text, source) in [
        (
            "Remove all counters from target permanent.",
            "from target permanent",
        ),
        ("Remove it from the game.", "from the game"),
    ] {
        selected_source(text, source);
    }
    common::assert_constituents(
        "A counter was removed from this land.",
        Category::Document,
        &[(Category::SourceComplement, "from this land")],
    );
    common::assert_constituents(
        "Return target creature card from your graveyard to your hand.",
        Category::Document,
        &[(Category::SourceComplement, "from your graveyard")],
    );
}

#[test]
fn independently_constructed_bare_passive_roundtrips() {
    let frame = common::lexicon().lexemes()["core-verb:Remove"]
        .properties
        .frames
        .iter()
        .position(|frame| frame.kind == "Passive" && frame.items.is_empty())
        .unwrap();
    let expected = Reading::SelectedFramePassive {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "core-verb:Remove".into(),
                form: WordForm::PastParticiple,
                features: FeatureBundle {
                    finiteness: Some(deckmaste_lexical::Finiteness::Nonfinite),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: Some(frame),
            countability: None,
        },
        complements: vec![],
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "removed");
    assert_eq!(
        common::readings("removed", Category::SecondaryVerbPhrase),
        BTreeSet::from([expected])
    );
}

#[test]
fn source_inside_a_transparent_object_oblique_is_selected_by_the_verb() {
    selected_source(
        "Remove any number of +1/+1 counters from among creatures you control.",
        "from among creatures you control",
    );
}

#[test]
fn a_genitive_possessor_cannot_hide_the_verbs_source() {
    let values = common::readings(
        "Remove a +1/+1 counter from this creature for each card in target opponent's hand.",
        Category::Document,
    );
    assert!(!values.is_empty());
    for value in values {
        let mut selected = false;
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && is_remove(head)
                {
                    let [FrameValue::Argument(_), FrameValue::Argument(source)] =
                        complements.as_slice()
                    else {
                        panic!("a Source in the Object's Possessor cannot license bare remove");
                    };
                    assert_eq!(source.category(), Category::SourceComplement);
                    selected = true;
                }
            })
            .unwrap();
        assert!(selected);
    }
}

#[test]
fn gerund_source_retains_selected_and_matrix_attachments_under_rs12() {
    let values = common::readings(
        "You may cast spells from the top of your library by removing a counter from a creature you control in addition to paying their other costs.",
        Category::Document,
    );
    assert!(!values.is_empty());
    let mut selected = false;
    let mut matrix = false;
    for value in values {
        let mut bare_gerund = false;
        let mut matrix_source = false;
        value
            .visit(&mut |node| {
                if let Reading::GerundComplementPreposition { complement, .. } = node {
                    complement
                        .visit(&mut |predicate| {
                            if let Reading::SelectedPredicate {
                                head, complements, ..
                            } = predicate
                                && is_remove(head)
                            {
                                match complements.as_slice() {
                                    [FrameValue::Argument(_), FrameValue::Argument(source)] => {
                                        assert_eq!(source.category(), Category::SourceComplement);
                                        selected = true;
                                    }
                                    [FrameValue::Argument(_)] => bare_gerund = true,
                                    _ => panic!(
                                        "remove must select its Object, with optional Source"
                                    ),
                                }
                            }
                        })
                        .unwrap();
                }
                if let Reading::PrepositionPredicate { modifier, .. } = node
                    && modifier.realize(common::lexicon()).unwrap() == "from a creature you control"
                {
                    matrix_source = true;
                }
            })
            .unwrap();
        matrix |= bare_gerund && matrix_source;
    }
    assert!(selected, "the gerund retains its selected Source analysis");
    assert!(matrix, "RS12 retains the competing matrix Source Adjunct");
}

#[test]
fn existing_return_frames_keep_their_head_only_object_licence() {
    let values = common::readings(
        "For each color, return up to one target card of that color from your graveyard to your hand. Exile All Suns' Dawn.",
        Category::Document,
    );
    assert!(!values.is_empty());
    let mut retained = false;
    for value in values {
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme == "core-verb:Return"
                    && let Some(FrameValue::Argument(object)) = complements.first()
                    && object.realize(common::lexicon()).unwrap()
                        == "up to one target card of that color from your graveyard"
                {
                    let frame = &common::lexicon().lexemes()["core-verb:Return"]
                        .properties
                        .frames[head.frame.unwrap()];
                    assert_eq!(
                        frame.items.first(),
                        Some(&FrameItem::Argument(deckmaste_lexical::FrameSlot {
                            relation: Relation::Object,
                            category: "SourceFreeObject".into(),
                        }))
                    );
                    retained = true;
                }
            })
            .unwrap();
    }
    assert!(
        retained,
        "Return must retain the Source inside its Object's Oblique"
    );
}
