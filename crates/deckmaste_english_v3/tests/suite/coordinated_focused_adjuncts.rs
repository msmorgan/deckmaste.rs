use deckmaste_english_v3::grammar::{Category, Reading};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};

use crate::common::{lexicon, readings, readings_on_card};

fn assert_focused_coordination(text: &str, card: Option<&str>) {
    let values = match card {
        Some(card) => readings_on_card(card, text, Category::Sentence),
        None => readings(text, Category::Sentence),
    };
    assert!(!values.is_empty(), "{text}");
    for value in values {
        let mut attached = false;
        value
            .visit(&mut |node| {
                if let Reading::PrepositionPredicate { modifier, .. } = node
                    && modifier.category() == Category::FocusedAdjunctCoordination
                {
                    attached = true;
                }
                if let Reading::Coordination {
                    category,
                    left,
                    right,
                    ..
                } = node
                    && *category == Category::FocusedAdjunctCoordination
                {
                    assert_eq!(left.category(), Category::FocusedAdjunctMember);
                    assert_eq!(right.category(), Category::FocusedAdjunctMember);
                    for member in [left, right] {
                        let (Reading::FocusedAdjunctMember { head: focused, .. }
                        | Reading::NominalAdjunctPredicate { head: focused, .. }) = member.as_ref()
                        else {
                            panic!("coordinate has no focused phrase");
                        };
                        assert!(matches!(focused.as_ref(), Reading::FocusedPhrase { .. }));
                    }
                }
                assert!(!matches!(node, Reading::ClauseCoordination { .. }));
                if let Reading::FocusedPhrase { head, .. } = node {
                    assert!(!matches!(head.as_ref(), Reading::Coordination { .. }));
                }
            })
            .unwrap();
        assert!(
            attached,
            "coordination must function as the clause's Adjunct"
        );
    }
}

#[test]
fn thunderhead_gunner_coordinates_pp_and_frequency() {
    assert_focused_coordination("Activate only as a sorcery and only once each turn.", None);
}

#[test]
fn instill_energy_coordinates_temporal_pp_and_frequency() {
    assert_focused_coordination(
        "Activate only during your turn and only once each turn.",
        None,
    );
}

#[test]
fn temple_of_cyclical_time_coordinates_conditional_and_predicative_pps() {
    assert_focused_coordination(
        "Activate only if it has no time counters on it and only as a sorcery.",
        None,
    );
}

#[test]
fn security_detail_coordinates_conditional_pp_and_frequency() {
    assert_focused_coordination(
        "Activate only if you control no creatures and only once each turn.",
        None,
    );
}

#[test]
fn hakim_keeps_the_member_focus_route() {
    assert_focused_coordination(
        "Activate only during your upkeep and only if Hakim isn't enchanted.",
        Some("Hakim, Loreweaver"),
    );
}

#[test]
fn standalone_controls_keep_their_readings() {
    for text in [
        "Activate only as a sorcery.",
        "Activate only once each turn.",
        "Activate only if you control no creatures.",
        "Activate during your turn and during your upkeep.",
        "Any player may activate this ability.",
        "Only your opponents may activate this ability.",
    ] {
        let values = readings(text, Category::Sentence);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            value
                .visit(&mut |node| {
                    assert_ne!(node.category(), Category::FocusedAdjunctCoordination);
                })
                .unwrap();
        }
    }
}

#[test]
fn clause_plus_focused_phrase_requires_its_own_route() {
    // CF3: a complete Clause is not a focused Adjunct member.
    for text in [
        "Any player may activate this ability but only as a sorcery.",
        "Any player may activate this ability but only during their draw step.",
        "Only your opponents may activate this ability and only as a sorcery.",
    ] {
        assert!(readings(text, Category::Sentence).is_empty(), "{text}");
    }
}

fn only_word() -> deckmaste_english_v3::grammar::Word {
    deckmaste_english_v3::grammar::Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "vocab:FocusAdverb/Only".into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_assembled_thunderhead_adjunct_roundtrips() {
    let left = readings("only as a sorcery", Category::PrepositionPhrase)
        .into_iter()
        .next()
        .unwrap();
    let right = readings("only once", Category::AdverbPhrase)
        .into_iter()
        .next()
        .unwrap();
    let temporal = readings("each turn", Category::NominalAdjunctPhrase)
        .into_iter()
        .next()
        .unwrap();
    let source = readings(
        "during your turn and during your upkeep",
        Category::PrepositionPhrase,
    )
    .into_iter()
    .next()
    .unwrap();
    let Reading::Coordination { coordinator, .. } = source else {
        panic!("coordinator fixture")
    };
    let expected = Reading::Coordination {
        category: Category::FocusedAdjunctCoordination,
        form: 0,
        left: Box::new(Reading::FocusedAdjunctMember {
            form: 0,
            head: Box::new(left),
        }),
        coordinator,
        right: Box::new(Reading::NominalAdjunctPredicate {
            category: Category::FocusedAdjunctMember,
            form: 0,
            head: Box::new(right),
            modifier: Box::new(temporal),
        }),
    };
    expected.admit(lexicon()).unwrap();
    let text = expected.realize(lexicon()).unwrap();
    assert_eq!(text, "only as a sorcery and only once each turn");
    assert_eq!(
        readings(&text, Category::FocusedAdjunctCoordination),
        [expected].into_iter().collect(),
    );
}

#[test]
fn focus_cannot_enclose_a_coordination_with_a_focused_member() {
    let left = readings("during your turn", Category::PrepositionPhrase)
        .into_iter()
        .next()
        .unwrap();
    let right = readings(
        "only if you control no creatures",
        Category::PrepositionPhrase,
    )
    .into_iter()
    .next()
    .unwrap();
    let source = readings(
        "during your turn and during your upkeep",
        Category::PrepositionPhrase,
    )
    .into_iter()
    .next()
    .unwrap();
    let Reading::Coordination { coordinator, .. } = source else {
        panic!("coordinator fixture")
    };
    let head = Reading::Coordination {
        category: Category::PrepositionPhrase,
        form: 0,
        left: Box::new(left),
        coordinator,
        right: Box::new(right),
    };
    head.admit(lexicon()).unwrap();
    let wrong = Reading::FocusedPhrase {
        category: Category::PrepositionPhrase,
        form: 0,
        modifier: only_word(),
        head: Box::new(head),
    };
    assert!(wrong.admit(lexicon()).is_err());
    assert!(wrong.realize(lexicon()).is_err());
}
