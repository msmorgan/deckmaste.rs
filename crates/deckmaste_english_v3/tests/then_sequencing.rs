mod common;

use common::LEXICON;
use common::assert_constituents as assert_expected_constituents;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn assert_constituents(text: &str, category: Category, expected: &[(Category, &str)]) {
    assert_expected_constituents(text, category, expected);
    for value in readings(text, category) {
        let mut links = Vec::new();
        value
            .visit(&mut |node| {
                let (Reading::Coordination { coordinator, .. }
                | Reading::ClauseCoordination { coordinator, .. }
                | Reading::CoordinationSeriesEnd { coordinator, .. }) = node
                else {
                    return;
                };
                if let LexicalReading::Word(word) = &coordinator.value
                    && word.lexeme == deckmaste_lexical::LexemeId::from("vocab:Coordinator/Then")
                {
                    links.push((node.construction(), node.category()));
                }
            })
            .unwrap();
        assert_eq!(
            links.len(),
            1,
            "one sequencing link in each witness Reading"
        );
        assert!(matches!(
            links[0],
            (
                "Coordination",
                Category::SecondaryVerbPhrase | Category::FinitePredicate
            ) | ("ClauseCoordination", Category::Clause)
                | (
                    "CoordinationSeriesEnd",
                    Category::SecondaryPredicateSeries | Category::ClauseSeries
                )
        ));
    }
}

fn word(owner: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number,
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn card_object() -> Reading {
    let mut card = word(
        "lexeme:CommonNoun/Card",
        WordForm::Singular,
        Some(Number::Singular),
    );
    card.countability = Some(true);
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(Reading::IndefiniteNounPhrase {
            form: 0,
            determiner: word(
                "vocab:Article/Indefinite",
                WordForm::Invariant,
                Some(Number::Singular),
            ),
            head: Box::new(Reading::Noun {
                form: 0,
                head: card,
            }),
        }),
    }
}

fn card_predicate(owner: &str, capitalization: SurfaceCase) -> Reading {
    let mut head = word(owner, WordForm::Plain, None);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.capitalization = capitalization;
    head.frame = Some(0);
    Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head,
        complements: vec![FrameValue::Argument(Box::new(card_object()))],
    }
}

#[test]
fn independently_constructed_stitcher_link_preserves_both_roundtrip_laws_and_traversals() {
    let expected = Reading::Coordination {
        category: Category::SecondaryVerbPhrase,
        form: 1,
        left: Box::new(card_predicate("core-verb:Draw", SurfaceCase::Initial)),
        coordinator: word("vocab:Coordinator/Then", WordForm::Invariant, None),
        right: Box::new(card_predicate(
            "lexeme:keyword_action/discard",
            SurfaceCase::Declared,
        )),
    };
    let text = "Draw a card, then discard a card";
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let parsed = readings(text, Category::SecondaryVerbPhrase);
    let actual = parsed
        .get(&expected)
        .expect("independently constructed Reading retained");
    let mut expected_nodes = Vec::new();
    let mut actual_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(expected_nodes, actual_nodes);
    let mut expected_words = Vec::new();
    let mut actual_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    actual
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(expected_words, actual_words);

    let Reading::Coordination {
        category,
        left,
        coordinator,
        right,
        ..
    } = expected
    else {
        unreachable!()
    };
    let unpunctuated = Reading::Coordination {
        category,
        form: 0,
        left,
        coordinator,
        right,
    };
    assert!(unpunctuated.admit(&LEXICON).is_err());
    assert!(unpunctuated.realize(&LEXICON).is_err());
}

#[test]
fn connective_marker_requires_punctuation_and_a_clausal_or_verbal_role() {
    assert!(readings("Draw a card then discard a card.", Category::Document).is_empty());
    assert!(readings("artifact, then creature", Category::NounPhrase).is_empty());
    assert!(readings("red, then blue", Category::AdjectivePhrase).is_empty());
    assert!(!readings("Draw a card and discard a card.", Category::Document).is_empty());
    for value in readings("Draw a card. Then discard a card.", Category::Document) {
        let mut owners = Vec::new();
        value
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value {
                    owners.push(value.lexeme);
                }
            })
            .unwrap();
        assert!(owners.contains(&"vocab:Adverb/Then".into()));
        assert!(!owners.contains(&"vocab:Coordinator/Then".into()));
    }
    assert!(!readings("Draw a card. Then discard a card.", Category::Document).is_empty());
}

#[test]
fn blur_coordinates_complete_imperative_predicates() {
    assert_constituents(
        "Exile target creature you control, then return that card to the battlefield under its owner's control.",
        Category::Document,
        &[
            (
                Category::SecondaryVerbPhrase,
                "Exile target creature you control",
            ),
            (
                Category::SecondaryVerbPhrase,
                "return that card to the battlefield under its owner's control",
            ),
        ],
    );
}

#[test]
fn obsessive_stitcher_coordinates_under_the_activated_ability() {
    assert_constituents(
        "{T}: Draw a card, then discard a card.",
        Category::Document,
        &[
            (Category::SecondaryVerbPhrase, "Draw a card"),
            (Category::SecondaryVerbPhrase, "discard a card"),
        ],
    );
}

#[test]
fn risky_research_coordinates_keyword_and_lexical_predicates() {
    assert_constituents(
        "Surveil 2, then draw two cards.",
        Category::Document,
        &[
            (Category::SecondaryVerbPhrase, "Surveil 2"),
            (Category::SecondaryVerbPhrase, "draw two cards"),
        ],
    );
}

#[test]
fn beneath_the_sands_retains_the_final_link_in_a_flat_serial_list() {
    assert_constituents(
        "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
        Category::Document,
        &[
            (
                Category::SecondaryVerbPhrase,
                "Search your library for a basic land card",
            ),
            (
                Category::SecondaryPredicateSeries,
                "put it onto the battlefield tapped, then shuffle",
            ),
            (
                Category::SecondaryVerbPhrase,
                "put it onto the battlefield tapped",
            ),
            (Category::SecondaryVerbPhrase, "shuffle"),
        ],
    );
}

#[test]
fn erode_shares_one_subject_and_modal_across_three_predicates() {
    assert_constituents(
        "Its controller may search their library for a basic land card, put it onto the battlefield tapped, then shuffle.",
        Category::Document,
        &[
            (
                Category::FinitePredicate,
                "may search their library for a basic land card, put it onto the battlefield tapped, then shuffle",
            ),
            (
                Category::BarePredicate,
                "search their library for a basic land card, put it onto the battlefield tapped, then shuffle",
            ),
            (
                Category::SecondaryPredicateSeries,
                "put it onto the battlefield tapped, then shuffle",
            ),
        ],
    );
}

#[test]
fn vengeful_villagers_keeps_the_second_clauses_own_subject_and_modal() {
    assert_constituents(
        "Tap it, then you may sacrifice an artifact or creature.",
        Category::Document,
        &[
            (Category::Clause, "Tap it"),
            (
                Category::FiniteClause,
                "you may sacrifice an artifact or creature",
            ),
        ],
    );
}
